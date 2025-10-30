//! Code for parsing the emulator components specification file
//!
//! The main part is the `device_tree` node, which consists of two parts:
//! - a list of components,
//! - a `clock_tree` node
//! - TODO: data bus / interrupt lines
//!
//! Each component will be a part of the big Emulator structure, and needs:
//! - info about it's filed name, module path, file path
//! - a name for the generated Proxy for dispatching events through the queue
//! - information about clock inputs
//!
//! The `clock_tree` contains information roughly corresponding to [TI-TRM] 6.5 Clock Management:
//! - info about oscillators (source of pulses)
//! - a list of main clocks (actually not listed)
//! - domains associated with power domains
//! - a tree of power domains, gates and dividers

use crate::components::clock_tree;
use crate::components::parsing::parse_component;
use crate::graph::NodeGraph;
use quote::format_ident;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub const CONFIG_FILE: &str = "cmemu_build_conf.yaml";

//////////////////////////////////////////
//  cmemu_build_conf.yaml schema
//////////////////////////////////////////
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EmuConf {
    pub(crate) device_tree: DeviceTreeConf,
    pub(crate) payload_size_check: usize,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct DeviceTreeConf {
    /// Discrete modules
    pub(crate) components: Vec<ComponentConf>,
    /// An overlay energy subsystem with primitives
    pub(crate) energy: EnergyConf,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentConf {
    pub(crate) field_name: String,
    pub(crate) mod_path: String,
    pub(crate) file_path: String,
    pub(crate) proxy_type_name: String,
    pub(crate) ticked_by: String,
    pub(crate) requires_feature: Option<String>,
}

/// An overlay energy subsystem with primitives
///
/// The energy subsystem represents voltage, power, and clock lines
/// as abstract concepts built from primitives such as switches and dividers,
/// which are controlled by components.
///
/// Additionally, some microcontroller-specific configuration may be attached
/// to facilitate code generation.
#[derive(Deserialize, Debug, Clone)]
// #[serde(deny_unknown_fields)]
pub(crate) struct EnergyConf {
    pub(crate) proxy_type_name: String,
    pub(crate) oscillators: Vec<OscillatorConf>,
    pub(crate) clock_tree: Vec<ClockTreeNodeConf>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct OscillatorConf {
    pub(crate) cycle_in_ps: u64,
    pub(crate) name: String,
    pub(crate) struct_name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClockTreeNodeConf {
    pub(crate) name: String,
    pub(crate) struct_name: String,
    pub(crate) kind: ClockTreeNodeKind,
    pub(crate) config_type: ClockTreeNodeConfigurationType,
    pub(crate) ticked_by: String,
    pub(crate) requires_feature: Option<String>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) enum ClockTreeNodeKind {
    Gate,
    Divider,
    Switch,
}

// The boolean tuple is either (start_run_state, start_sleep_state, start_deep_sleep_state) or (start_state).
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub(crate) enum ClockTreeNodeConfigurationType {
    Global(u32),
    PowerDomainGate(u32),
    Computed(u32),
    Switch(Vec<String>),
}

//////////////////////////////////////////
//  Processing helpers
//////////////////////////////////////////

/// Build metadata contains information parsed both from the config file (`cmemu_build_conf`)
/// and the codebase.
#[derive(Debug)]
pub(crate) struct BuildMetadata {
    pub(crate) components: Vec<ComponentDesc>,
    pub(crate) energy: EnergyConf,
    // TODO: move to confeature
    pub(crate) payload_size_check: usize,

    // Analysis
    pub(crate) clock_graph: ClockTreeGraph,
}

#[derive(Debug)]
pub(crate) enum ClockTreeNode {
    // TODO: bring back Rc? This is not shared anyway, but breaks destruction in match
    Inner(ClockTreeNodeConf),
    Component(ComponentConf),
    Oscillator(OscillatorConf),
}
pub(crate) type ClockTreeGraph = NodeGraph<ClockTreeNode, Rc<str>>;

#[derive(Debug)]
pub(crate) struct ComponentDesc {
    pub(crate) config: ComponentConf,
    pub(crate) is_special: bool,
    pub(crate) imports: Vec<String>,
    pub(crate) handlers: Vec<HandlerDesc>,
}

#[derive(Debug)]
pub(crate) struct HandlerDesc {
    pub(crate) name: String,
    pub(crate) allow_dead_code: bool, // to reapply it to the proxy
    pub(crate) explicit_delay: bool,
    pub(crate) pass_components: bool,
    pub(crate) fields: Vec<(String, String)>, // name & type
    pub(crate) generator: HandlerKind,        // TODO: or make dyn-trait?
}

#[derive(Debug)]
pub(crate) enum HandlerKind {
    Auto,
    ClockTree,
}

impl ClockTreeNode {
    #[must_use]
    pub(crate) fn requires_feature(&self) -> Option<&String> {
        match self {
            ClockTreeNode::Inner(i) => i.requires_feature.as_ref(),
            ClockTreeNode::Component(c) => c.requires_feature.as_ref(),
            ClockTreeNode::Oscillator(_) => None,
        }
    }
    // Qualified path to type
    #[must_use]
    pub(crate) fn type_path(&self) -> &str {
        match self {
            ClockTreeNode::Component(ComponentConf { mod_path, .. }) => mod_path,
            ClockTreeNode::Inner(c) => c.struct_name.as_str(),
            ClockTreeNode::Oscillator(o) => o.struct_name.as_str(),
        }
    }
    // Unqualified struct name
    #[must_use]
    pub(crate) fn struct_name(&self) -> &str {
        match self {
            ClockTreeNode::Component(c) => c.struct_name(),
            ClockTreeNode::Inner(c) => c.struct_name.as_str(),
            ClockTreeNode::Oscillator(o) => o.struct_name.as_str(),
        }
    }
    // Name: unique name in the graph (.name, .field_name)
    #[must_use]
    pub(crate) fn name(&self) -> &str {
        match self {
            ClockTreeNode::Component(c) => c.field_name.as_str(),
            ClockTreeNode::Inner(c) => c.name.as_str(),
            ClockTreeNode::Oscillator(o) => o.name.as_str(),
        }
    }
}

impl ComponentConf {
    #[must_use]
    pub(crate) fn struct_name(&self) -> &str {
        // TODO: intern it in ComponentDesc
        self.mod_path.split("::").last().unwrap()
    }
}

impl ClockTreeNodeConf {
    #[must_use]
    pub(crate) fn type_ident(&self) -> syn::Ident {
        format_ident!("{}", self.struct_name)
    }
}

pub(crate) fn parse_components(emu_conf: EmuConf) -> BuildMetadata {
    let mut components: Vec<ComponentDesc> = emu_conf
        .device_tree
        .components
        .iter()
        .map(parse_component)
        .collect();
    components.push(clock_tree::produce_clock_tree_component_desc(
        &emu_conf.device_tree.energy,
    ));
    let clock_graph = parse_clock_tree_graph(&emu_conf);
    BuildMetadata {
        components,
        energy: emu_conf.device_tree.energy.clone(),
        payload_size_check: emu_conf.payload_size_check,
        clock_graph,
    }
}

fn parse_clock_tree_graph(emu_conf: &EmuConf) -> ClockTreeGraph {
    macro_rules! panic_duplicate {
        ($prev:expr, $new:expr) => {{
            panic!(
                "ClockTree nodes must have unique identifiers: these come from `name` or `field_name` fields.\
                However, these two nodes have the same key: {}\n{:?}\n{:?}\n
            ", $prev.name(), $prev, $new)
        }};
    }
    let mut graph = ClockTreeGraph::new();
    // Add nodes first (make Rcs)

    for osc in &emu_conf.device_tree.energy.oscillators {
        let _ = graph
            .add_node(
                osc.name.clone().into(),
                ClockTreeNode::Oscillator(osc.clone()),
            )
            .is_none_or(|p| panic_duplicate!(p, osc));
    }
    for node in &emu_conf.device_tree.energy.clock_tree {
        let _ = graph
            .add_node(node.name.clone().into(), ClockTreeNode::Inner(node.clone()))
            .is_none_or(|p| panic_duplicate!(p, node));
    }
    for comp in &emu_conf.device_tree.components {
        let _ = graph
            .add_node(
                comp.field_name.clone().into(),
                ClockTreeNode::Component(comp.clone()),
            )
            .is_none_or(|p| panic_duplicate!(p, comp));
    }
    // Add edges when all nodes are registered
    for comp in &emu_conf.device_tree.components {
        graph.link_parent(&*comp.ticked_by, &*comp.field_name);
    }
    for node in &emu_conf.device_tree.energy.clock_tree {
        graph.link_parent(&*node.ticked_by, &*node.name);
        if let ClockTreeNodeConfigurationType::Switch(ref parents) = node.config_type {
            let mut found_parent = false;
            for parent in parents {
                if *parent == node.ticked_by {
                    found_parent = true;
                } else {
                    graph.link_parent(parent.as_str(), &*node.name);
                }
            }
            assert!(
                found_parent,
                "Default parent {} not found in options for {node:?}",
                node.ticked_by
            );
        }
    }
    validate_ticks_graph(
        &graph,
        emu_conf
            .device_tree
            .energy
            .oscillators
            .iter()
            .map(|d| &*d.name),
    );
    graph
}

fn validate_ticks_graph<'a>(clock_tree: &'a ClockTreeGraph, roots: impl Iterator<Item = &'a str>) {
    // Everything is reachable from roots
    let mut visited = HashMap::new();
    for dom_idx in roots {
        clock_tree.dfs_inner(
            &clock_tree.get(dom_idx).unwrap().idx,
            &mut |_| {},
            &mut |_, _| {},
            &mut visited,
        );
    }
    assert_eq!(
        visited.len(),
        clock_tree.len(),
        "The clock tree graph is not consistent, unreachable nodes: {:?}",
        clock_tree
            .indices()
            .filter(|&k| !visited.contains_key(k))
            .collect::<Vec<_>>()
    );

    // Parent counts make sense (this is mostly redundant)
    for n in clock_tree.nodes() {
        let parents = n.parents().count();
        match (&*n, parents) {
            (ClockTreeNode::Oscillator(o), 1..) => {
                panic!("Root oscillator {o:?} cannot have parents!")
            }
            (ClockTreeNode::Component(c), 0 | 2..) => {
                panic!("Component {c:?} should have one parent, has {parents}!")
            }
            (ClockTreeNode::Inner(c), 2..) if !matches!(c.kind, ClockTreeNodeKind::Switch) => {
                panic!("Node {c:?} should have one parent, has {parents}!")
            }
            _ => (),
        }
    }

    // CDL must always be the first (or rework it to call its dumps)
    // TODO: make sure it's first in the whole hf
    assert_eq!(
        &**clock_tree.children("mcu_clk").next().unwrap(),
        "cycle_debug_logger",
        "CDL must be the first ticked component"
    );
}
