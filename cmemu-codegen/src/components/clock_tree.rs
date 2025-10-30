use crate::components;
use crate::components::conf::{
    BuildMetadata, ClockTreeGraph, ClockTreeNode, ClockTreeNodeConf,
    ClockTreeNodeConfigurationType, ClockTreeNodeKind, EnergyConf, HandlerDesc, HandlerKind,
};
use crate::components::conf::{ComponentConf, ComponentDesc};
use itertools::{Itertools, join};

// TODO: just pass it through the parser instead of manually specifying handlers
pub(super) fn produce_clock_tree_component_desc(conf: &EnergyConf) -> ComponentDesc {
    let conf = ComponentConf {
        field_name: "clock_tree".to_owned(),
        mod_path: "crate::component::PowerClockManager".to_owned(),
        file_path: "N/A_should_be_unused".to_owned(),
        proxy_type_name: conf.proxy_type_name.clone(),
        ticked_by: "N/A_mark2".to_owned(),
        requires_feature: None,
    };
    let handlers = [
        ("power_on_reset", ""),
        ("pulse", "Oscillators"),
        ("wakeup_skipping_osc", "Oscillators"),
        ("start_oscillator", "Oscillators"),
        ("stop_oscillator", "Oscillators"),
        ("query_state", "ClockTreeStateQuerent"),
        ("start_sleep", ""),
        ("stop_sleep", ""),
    ];
    let mut handlers: Vec<HandlerDesc> = handlers
        .map(|(name, arg)| HandlerDesc {
            name: name.to_owned(),
            allow_dead_code: false,
            explicit_delay: name == "pulse" || name == "wakeup_skipping_osc",
            pass_components: name == "power_on_reset"
                || name == "pulse"
                || name == "stop_oscillator"
                || name == "start_oscillator"
                || name == "wakeup_skipping_osc",
            fields: if arg.is_empty() {
                vec![]
            } else {
                vec![("arg".to_owned(), arg.to_owned())]
            },
            generator: HandlerKind::ClockTree,
        })
        .into();
    handlers.push(HandlerDesc {
        name: "want_node_state".to_owned(),
        allow_dead_code: false,
        explicit_delay: false,
        pass_components: false,
        fields: vec![
            ("node".to_owned(), "ClockTreeNodes".to_owned()),
            ("power".to_owned(), "PowerMode".to_owned()),
        ],
        generator: HandlerKind::ClockTree,
    });
    handlers.push(HandlerDesc {
        name: "want_switch_parent".to_owned(),
        allow_dead_code: false,
        explicit_delay: false,
        pass_components: false,
        fields: vec![
            ("node".to_owned(), "ClockTreeNodes".to_owned()),
            ("parent".to_owned(), "EnergyEntity".to_owned()),
        ],
        generator: HandlerKind::ClockTree,
    });
    handlers.push(HandlerDesc {
        name: "want_divider_scale".to_owned(),
        allow_dead_code: true, // TODO: implement setting dividers!
        explicit_delay: false,
        pass_components: false,
        fields: vec![
            ("node".to_owned(), "ClockTreeNodes".to_owned()),
            ("divider".to_owned(), "u32".to_owned()),
        ],
        generator: HandlerKind::ClockTree,
    });
    ComponentDesc {
        config: conf,
        is_special: true,
        imports: [
            "use crate::engine::{Context, PowerMode};",
            "use crate::build_data::{ClockTreeNodes, Oscillators, EnergyEntity};",
            "use crate::component::clock_tree::ClockTreeStateQuerent;",
        ]
        .map(str::to_string)
        .to_vec(),
        handlers,
    }
}

impl ClockTreeNodeConf {
    #[must_use]
    fn init_value(&self, graph: &ClockTreeGraph) -> String {
        // FIXME: get rid of some of these
        let initial_value = match self.config_type {
            ClockTreeNodeConfigurationType::Global(i)
            | ClockTreeNodeConfigurationType::PowerDomainGate(i)
            | ClockTreeNodeConfigurationType::Computed(i) => i,
            ClockTreeNodeConfigurationType::Switch(_) => 0, // doesn't matter
        };
        match self.kind {
            ClockTreeNodeKind::Gate => (initial_value != 0).to_string(),
            ClockTreeNodeKind::Divider => initial_value.to_string(),
            ClockTreeNodeKind::Switch => graph[&*self.ticked_by].id(),
        }
    }
    #[must_use]
    pub(crate) fn sc_marker(&self) -> String {
        format!("{}SC", self.struct_name)
    }

    #[must_use]
    pub(crate) fn true_type(&self) -> String {
        format!("{:?}Node<{}>", self.kind, self.sc_marker())
    }

    #[must_use]
    fn generate_config(&self, graph: &ClockTreeGraph) -> String {
        match (self.kind, &self.config_type) {
            (ClockTreeNodeKind::Gate | ClockTreeNodeKind::Divider, _) => String::new(),
            (ClockTreeNodeKind::Switch, ClockTreeNodeConfigurationType::Switch(pars)) => format!("
               impl SwitchConf for {struct} {{
                    fn is_valid_parent(p: &Self::IdSpace) -> bool {{ matches!(p, {parents}) }}
                }}
                ",
                struct = self.struct_name,
                parents = pars.iter().map(|p| graph[&**p].id() ).join("|")
            ),
            _ => unreachable!(),
        }
    }
}

impl ClockTreeNode {
    #[must_use]
    fn init_value(&self, graph: &ClockTreeGraph) -> String {
        match self {
            ClockTreeNode::Component(_) => unreachable!(),
            ClockTreeNode::Inner(c) => c.init_value(graph),
            ClockTreeNode::Oscillator(_) => "Default::default(), PowerMode::Off".to_owned(),
        }
    }

    #[must_use]
    pub(crate) fn sc_marker(&self) -> String {
        match self {
            ClockTreeNode::Component(_) => unreachable!("components are top-level"),
            ClockTreeNode::Inner(c) => c.sc_marker(),
            // Temporary, maybe some custom types?
            ClockTreeNode::Oscillator(o) => format!("{}SC", o.struct_name),
        }
    }

    // can be useful for aliasing
    #[must_use]
    pub(crate) fn true_type(&self) -> String {
        match self {
            ClockTreeNode::Component(c) => c.mod_path.clone(),
            ClockTreeNode::Inner(c) => c.true_type(),
            // TODO: Temporary, maybe some custom types?
            ClockTreeNode::Oscillator(o) => format!(
                "OscillatorNode<{}, ConstOsc<{}>>",
                self.sc_marker(),
                o.cycle_in_ps
            ),
        }
    }

    #[must_use]
    fn id(&self) -> String {
        let prefix = match self {
            ClockTreeNode::Inner(_) => {
                "build_data::EnergyEntity::ClockTree(build_data::ClockTreeNodes::"
            }
            ClockTreeNode::Oscillator(_) => {
                "build_data::EnergyEntity::Oscillator(build_data::Oscillators::"
            }
            ClockTreeNode::Component(_) => {
                "build_data::EnergyEntity::Component(build_data::Components::"
            }
        };
        format!("{}{})", prefix, self.struct_name())
    }
}

macro_rules! format_clock_tree_map {
    ($comp_idx:ident $(($($filter:tt)+))?, $elem_fmt_fn:expr) => {
        format_clock_tree_map!($comp_idx $(($($filter)+))?, $elem_fmt_fn, "\n")
    };
    ($comp_idx:ident $(($filter:path))?, $elem_fmt_fn:expr, $sep:expr) => {
        $comp_idx
            .clock_graph
            .values()
            $(.flat_map(|n| if let $filter(n) = n {Some(n)} else {None}))?
            .map($elem_fmt_fn)
            .join($sep)
    };
    ($comp_idx:ident $(($filter:pat))?, $elem_fmt_fn:expr, $sep:expr) => {
        $comp_idx
            .clock_graph
            .nodes()
            $(.filter(|n| matches!(**n, $filter)))?
            .map($elem_fmt_fn)
            .join($sep)
    };
}

pub(super) fn generate_clock_graph(comp_idx: &BuildMetadata) -> String {
    let nodes = format_clock_tree_map!(
        comp_idx(ClockTreeNode::Inner(_) | ClockTreeNode::Oscillator(_)),
        |n| format!(
            "#[subcomponent(pub(super) {})] {}: {}",
            n.sc_marker(),
            n.idx,
            n.struct_name()
        ),
        ","
    );
    let aliases = format_clock_tree_map!(
        comp_idx(ClockTreeNode::Inner(_) | ClockTreeNode::Oscillator(_)),
        |n| format!("pub(super) type {} = {};", n.struct_name(), n.true_type(),)
    );
    let nodes_init = format_clock_tree_map!(
        comp_idx(ClockTreeNode::Inner(_) | ClockTreeNode::Oscillator(_)),
        |n| format!(
            "{}: <{}>::new({}),",
            n.idx,
            n.true_type(),
            n.init_value(n.graph)
        )
    );
    let energy_nodes = format_clock_tree_map!(
        comp_idx(ClockTreeNode::Inner(_) | ClockTreeNode::Oscillator(_)),
        |n| format!(
            "
       impl EnergyNode for {struct} {{
            type Extra = Components;
            type IdSpace = build_data::EnergyEntity;
            const NAME: &\'static str = \"{struct}\";
            fn id() -> Self::IdSpace {{ {id} }}
        }}
        ",
            id = n.id(),
            struct = n.struct_name(),
        )
    );
    let nodes_configs = format_clock_tree_map!(comp_idx(ClockTreeNode::Inner), |n| n
        .generate_config(&comp_idx.clock_graph));
    let mappers = format_clock_tree_map!(
        comp_idx(ClockTreeNode::Inner(_) | ClockTreeNode::Oscillator(_)),
        |n| join(
            [
                generate_mapper(&n.idx, Mappers::Tick, comp_idx),
                generate_mapper(&n.idx, Mappers::Tock, comp_idx),
                generate_mapper(&n.idx, Mappers::MaxSkip, comp_idx),
                generate_mapper(&n.idx, Mappers::Emulate, comp_idx),
                generate_mapper(&n.idx, Mappers::Prepare, comp_idx),
                generate_mapper(&n.idx, Mappers::SetPower, comp_idx),
            ],
            "\n"
        )
    );
    format!(
        "\
        use super::nodes::*;
        use super::oscillators::*;
        use crate::component::Components;
        use crate::engine::*;
        use crate::build_data;

        #[derive(Subcomponent, )]
        #[subcomponent_1to1]
        pub(super) struct Nodes {{
            {nodes}
        }}
        {aliases}
        impl Nodes {{
            pub(super) fn new() -> Self {{
                Nodes {{
                    {nodes_init}
               }}
            }}
        }}
        {energy_nodes}
        {nodes_configs}
        {mappers}
    "
    )
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Mappers {
    Tick,
    Tock,
    MaxSkip,
    Emulate,
    Prepare,
    SetPower,
}

/// For a clock tree node, generate a mapper function iterating over its children.
///
/// For instance, it would do a `tick()` over all children of a gate.
fn generate_mapper(node_id: &str, mapper: Mappers, build_data: &BuildMetadata) -> String {
    let (func, trait_, args) = match mapper {
        Mappers::Tick => ("tick", Some("ClockTreeNode"), ""),
        Mappers::Tock => ("tock", Some("ClockTreeNode"), ""),
        Mappers::MaxSkip => ("max_cycles_to_skip", Some("SkippableClockTreeNode"), ""),
        Mappers::Emulate => (
            "emulate_skipped_cycles",
            Some("SkippableClockTreeNode"),
            "param.0",
        ),
        Mappers::Prepare => ("prepare_to_disable", Some("PowerNode"), "param.0"),
        Mappers::SetPower => ("set_power_state", Some("PowerNode"), "param.0"),
    };
    let (mapper_t, ret_t) = match mapper {
        Mappers::Tick => ("TickMapper", "()"),
        Mappers::Tock => ("TockMapper", "()"),
        Mappers::MaxSkip => ("MaxSkipMapper", "u64"),
        Mappers::Emulate => ("EmulateMapper", "()"),
        Mappers::Prepare => ("PrepareMapper", "PowerMode"),
        Mappers::SetPower => ("SetPowerMapper", "()"),
    };
    let node = &build_data.clock_graph.get(node_id).unwrap();
    let calls = node
        .children()
        .map(|c| generate_call(&c.idx, func, trait_, args, build_data))
        .join(",");
    // NOTE: consider making the iterator lazy (maybe with future gen!)
    format!(
        "
impl RPITITNode<{mapper_t}, {ret_t}> for {node_t} {{
    #[allow(unused)]
    #[allow(clippy::too_many_lines, clippy::let_unit_value, clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn map_children(
        comp: &mut Self::Component,
        ctx: &mut Context,
        components: &mut Self::Extra,
        param: {mapper_t},
    ) -> impl Iterator<Item = {ret_t}>
    {{
        [{calls}].into_iter()
    }}
}}
    ",
        node_t = node.type_path(),
    )
}

/// Generate `<type as trait>::function(&mut proper_self, ctx, parent_id, args...)`
/// for a clock graph node.
fn generate_call(
    node_id: &str,
    function_name: &str,
    trait_: Option<&str>,
    args: &str,
    build_data: &BuildMetadata,
) -> String {
    let node = &build_data.clock_graph[node_id];
    let type_path = node.type_path();
    let self_path = match node {
        ClockTreeNode::Component(ComponentConf { .. }) => "components.as_mut()",
        ClockTreeNode::Inner(ClockTreeNodeConf { .. }) => "comp",
        ClockTreeNode::Oscillator(_) => unimplemented!(),
    };
    let extra_arg = match node {
        ClockTreeNode::Component(ComponentConf { .. }) => "&mut ()",
        ClockTreeNode::Inner(ClockTreeNodeConf { .. }) => "components",
        ClockTreeNode::Oscillator(_) => unimplemented!(),
    };
    let base = if let Some(trait_) = trait_ {
        format!("<{type_path} as {trait_}>::{function_name}({self_path}")
    } else {
        format!("{self_path}.{function_name}(")
    };

    let call = format!("{base}, ctx, <Self as EnergyNode>::id(), {extra_arg}, {args})");
    format!(
        "{}
        {{
        let res = {};
        if *crate::confeature::cm_logs::CLOCK_TREE_TRACES {{
            log::trace!(\"{{:?}}->{node_id}::{function_name}={{res:?}}\", <Self as EnergyNode>::id());
        }}
        res
        }}",
        components::format_cfg_feature(node.requires_feature()),
        call,
    )
}
