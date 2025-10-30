use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, Index};
use std::rc::Rc;

/// A simple graph implementation with custom indices and easy use for shared data.
///
/// This is a bit similar to `petgraph::GraphMap`, but it doesn't allow associating data,
/// so we would need to write a wrapper.
/// Moreover, out API exposes features from the `HashMap`, such as
/// indexing by `&Q`, where `K: Borrow<Q>`, and that allows us to use `Rc<str>` for indices,
/// while querying with a `&String` or `&str`.
#[derive(Debug)]
pub(crate) struct NodeGraph<N, Idx = Rc<str>>
where
    Idx: Hash + PartialEq + Eq + Clone + Debug,
{
    node_map: HashMap<Idx, N>,
    out_edges: HashMap<Idx, Vec<Idx>>,
    rev_edges: HashMap<Idx, Vec<Idx>>,
}

/// Node reference (holds shared ref to the graph and dereferences to the node value).
/// Operations produce `NodeRef`-s.
#[derive(Debug)]
pub(crate) struct NodeRef<'g, N, Idx = Rc<str>>
where
    Idx: Hash + PartialEq + Eq + Clone + Debug,
{
    pub idx: Idx,
    node: &'g N,
    pub graph: &'g NodeGraph<N, Idx>,
}

impl<N, Idx> Deref for NodeRef<'_, N, Idx>
where
    Idx: Hash + PartialEq + Eq + Clone + Debug,
{
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.node
    }
}

/// The `graph[index]` operator, returns the associated node value (we cannot conjure `NodeRef` here)
impl<N, Idx, QIdx> Index<&QIdx> for NodeGraph<N, Idx>
where
    Idx: Hash + PartialEq + Eq + Clone + Debug,
    QIdx: Hash + Eq + Debug + ?Sized,
    Idx: Borrow<QIdx>,
{
    type Output = N;
    fn index(&self, index: &QIdx) -> &Self::Output {
        &self.node_map[index]
    }
}

impl<N, Idx> NodeGraph<N, Idx>
where
    Idx: Hash + PartialEq + Eq + Clone + Debug,
{
    /// Create an empty graph implemented as hashmaps, incl. dual adjacency lists.
    pub(crate) fn new() -> Self {
        Self {
            node_map: HashMap::new(),
            out_edges: HashMap::new(),
            rev_edges: HashMap::new(),
        }
    }

    /// Number of nodes in the graph
    pub(crate) fn len(&self) -> usize {
        self.node_map.len()
    }

    /// Adds a node to the graph, returns existing value under that index.
    ///
    /// See [`HashMap::insert`]
    pub(crate) fn add_node(&mut self, idx: Idx, node: N) -> Option<N> {
        self.node_map.insert(idx, node)
    }

    /// Create a directed edge. Both nodes must be present in the graph.
    ///
    /// The input indices are used only for the query, original ones are used to store the links.
    /// (This is important when using weird `QIdx`, see notes in [`HashMap::get_key_value`]).
    pub(crate) fn link_parent<QIdx>(&mut self, parent: &QIdx, child: &QIdx)
    where
        QIdx: Hash + Eq + Debug + ?Sized,
        Idx: Borrow<QIdx>,
    {
        let Some((orig_parent, _)) = self.node_map.get_key_value(parent) else {
            panic!("Parent {parent:?} not found to link with {child:?}");
        };
        let Some((orig_child, _)) = self.node_map.get_key_value(child) else {
            panic!("Child {child:?} not found to link with {parent:?}");
        };
        self.out_edges
            .entry(orig_parent.clone())
            .or_default()
            .push(orig_child.clone());
        self.rev_edges
            .entry(orig_child.clone())
            .or_default()
            .push(orig_parent.clone());
    }

    /// Get a node reference. Note that indexing operator `[]` returns a node value!
    pub(crate) fn get<QIdx>(&self, idx: &QIdx) -> Option<NodeRef<'_, N, Idx>>
    where
        QIdx: Hash + Eq + Debug + ?Sized,
        Idx: Borrow<QIdx>,
    {
        self.node_map.get_key_value(idx).map(|(k, n)| NodeRef {
            idx: k.clone(),
            node: n,
            graph: self,
        })
    }

    /// Iterate over all node indices in the graph
    pub(crate) fn indices(&self) -> impl Iterator<Item = &Idx> {
        self.node_map.keys()
    }

    /// Iterate over all node values in the graph
    pub(crate) fn values(&self) -> impl Iterator<Item = &N> {
        self.node_map.values()
    }

    /// Iterate over all nodes as [`NodeRef`]-s
    pub(crate) fn nodes(&self) -> impl Iterator<Item = NodeRef<'_, N, Idx>> {
        self.node_map.iter().map(|(idx, n)| NodeRef {
            idx: idx.clone(),
            node: n,
            graph: self,
        })
    }

    /// Iterate over children indices (outgoing edges)
    pub(crate) fn children<QIdx>(&self, idx: &QIdx) -> impl Iterator<Item = &Idx>
    where
        QIdx: Hash + Eq + Debug + ?Sized,
        Idx: Borrow<QIdx>,
    {
        self.out_edges.get(idx).into_iter().flatten()
    }

    /// Iterate over parents indices (incoming edges)
    pub(crate) fn parents<QIdx>(&self, idx: &QIdx) -> impl Iterator<Item = &Idx>
    where
        QIdx: Hash + Eq + Debug + ?Sized,
        Idx: Borrow<QIdx>,
    {
        self.rev_edges.get(idx).into_iter().flatten()
    }

    /// Visit nodes in Depth-First-Search fashion
    ///
    /// The search starts from `idx` and all nodes (as references) are visited only once.
    /// The `visit` callback may provide data to be collected after visiting a node.
    /// A `HashMap` with the results is returned.
    #[allow(dead_code)]
    pub(crate) fn dfs<'g, V, QIdx>(
        &'g self,
        idx: &QIdx,
        mut visit: impl FnMut(NodeRef<'g, N, Idx>) -> V,
    ) -> HashMap<Idx, V>
    where
        QIdx: Hash + Eq + Debug + ?Sized,
        Idx: Borrow<QIdx>,
    {
        // Intern the index under a cover reason...
        let idx = self.get(idx).expect("Starting node not in the graph").idx;
        let mut visited = HashMap::new();
        self.dfs_inner(&idx, &mut visit, &mut |_, _| {}, &mut visited);
        visited
    }

    /// Visit nodes in Post-Order Depth-First-Search fashion (children first)
    ///
    /// The search starts from `idx` and all nodes (as references) are visited only once.
    /// A `HashSet` with visited nodes is returned.
    #[allow(dead_code)]
    pub(crate) fn dfs_postorder<'g, QIdx>(
        &'g self,
        idx: &QIdx,
        mut visit: impl FnMut(NodeRef<'g, N, Idx>),
    ) -> HashSet<Idx>
    where
        QIdx: Hash + Eq + Debug + ?Sized,
        Idx: Borrow<QIdx>,
    {
        // Intern the index under a cover reason...
        let idx = self.get(idx).expect("Starting node not in the graph").idx;
        let mut visited = HashMap::new();
        self.dfs_inner(&idx, &mut |_| {}, &mut |n, _| visit(n), &mut visited);
        // I always thought HashSet<K> IS just HashMap<K, ()>, but there is no conversion!
        visited.into_keys().collect()
    }

    /// Iterate over nodes in post-order fashion. Allocates a temporary vec.
    pub(crate) fn nodes_postorder(&self) -> impl Iterator<Item = NodeRef<'_, N, Idx>> {
        let mut order = vec![];
        let mut visited = HashMap::new();
        for i in self.indices() {
            self.dfs_inner(i, &mut |_| {}, &mut |n, _| order.push(n), &mut visited);
        }
        order.into_iter()
    }

    /// Visit nodes in Depth-First-Search fashion
    ///
    /// The search starts from `idx` and all nodes (as references) are visited only once.
    /// The `visit` callback may provide data to be collected after visiting a node.
    /// An accumulator hashmap is passed directly to this function.
    /// Its keys are used to deduplicate visitations.
    pub(crate) fn dfs_inner<'g, V>(
        &'g self,
        idx: &Idx,
        visit_pre: &mut impl FnMut(NodeRef<'g, N, Idx>) -> V,
        visit_post: &mut impl FnMut(NodeRef<'g, N, Idx>, &mut HashMap<Idx, V>),
        acc: &mut HashMap<Idx, V>,
    ) {
        if !acc.contains_key(idx) {
            acc.insert(idx.clone(), visit_pre(self.get(idx).unwrap()));
            for child in self.children(idx) {
                self.dfs_inner(child, visit_pre, visit_post, acc);
            }
            visit_post(self.get(idx).unwrap(), acc);
        }
    }
}

impl<N, Idx> NodeRef<'_, N, Idx>
where
    Idx: Hash + PartialEq + Eq + Clone + Debug,
{
    /// Iterate over children node references (outgoing edges)
    pub(crate) fn children(&self) -> impl Iterator<Item = Self> {
        self.graph
            .children(&self.idx)
            .map(|i| self.graph.get(i).unwrap())
    }

    /// Iterate over parent node references (incoming edges)
    pub(crate) fn parents(&self) -> impl Iterator<Item = Self> {
        self.graph
            .parents(&self.idx)
            .map(|i| self.graph.get(i).unwrap())
    }

    /// Get the first incoming edge or panic.
    #[allow(dead_code)]
    pub(crate) fn unwrap_parent(&self) -> Self {
        self.parents()
            .next()
            .expect("The node was asserted to have a parent")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph() -> NodeGraph<u32, &'static str> {
        let mut graph: NodeGraph<u32, &'static str> = NodeGraph::new();
        graph.add_node("zero", 0);
        graph.add_node("one", 1);
        graph.add_node("two", 2);
        graph.link_parent("zero", "one");
        graph.link_parent("one", "two");
        graph.link_parent("zero", "two");
        graph
    }

    #[test]
    fn basic_graph() {
        let graph = make_graph();

        assert_eq!(graph["zero"], 0);
        let zero_ref = graph.get("zero").unwrap();
        assert_eq!(*zero_ref, 0);
        assert_eq!(zero_ref.idx, "zero");
        assert!(graph.get("three").is_none());
        assert_eq!(
            graph.indices().collect::<HashSet<_>>(),
            [&"zero", &"one", &"two"]
                .into_iter()
                .collect::<HashSet<_>>()
        );
        assert_eq!(graph.nodes().count(), 3);
        assert_eq!(graph.children("zero").count(), 2);
        assert_eq!(graph.parents("zero").count(), 0);
        assert_eq!(graph.parents("two").count(), 2);
        assert_eq!(graph.parents("one").next(), Some(&"zero"));
    }

    #[test]
    fn graph_api_with_rc() {
        let mut graph: NodeGraph<u32, Rc<str>> = NodeGraph::new();
        graph.add_node("zero".into(), 0);
        graph.add_node("one".into(), 1);
        graph.add_node("two".into(), 2);
        let zero_string = "zero".to_owned();
        graph.link_parent(&*zero_string, "one");
        graph.link_parent("one", "two");
        graph.link_parent("zero", "two");
        // Keys are Rc, but we query by &str
        assert_eq!(graph["zero"], 0);
        let zero_ref = graph.get("zero").unwrap();
        assert_eq!(Rc::weak_count(&zero_ref.idx), 0);
        // 5: one in the main map, one in ref, one in fwd edges, two in back edges
        let zero_rc_sc = Rc::strong_count(&zero_ref.idx);
        assert_eq!(zero_rc_sc, 5);
        assert_eq!(zero_ref.children().count(), 2);
        assert_eq!(zero_ref.parents().count(), 0);
        // no leak
        assert_eq!(Rc::strong_count(&zero_ref.idx), zero_rc_sc);
        // some nested loops
        for n in graph.nodes() {
            for ch in n.children() {
                for p in n.parents() {
                    for ch2 in p.children() {
                        for p_dir in graph.parents(&ch2.idx) {
                            assert_ne!(*n, *ch);
                            assert_ne!(*p_dir, ch2.idx);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn graph_api_with_enum() {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        enum Enum {
            Zero,
            One,
            Two,
        }

        let mut graph = NodeGraph::new();
        graph.add_node(Enum::Zero, Enum::One);
        graph.add_node(Enum::One, Enum::Two);
        graph.link_parent(&Enum::Zero, &Enum::One);
        assert_eq!(graph.nodes().count(), 2);
        assert_eq!(graph[&graph[&Enum::Zero]], Enum::Two);
    }

    #[test]
    fn dfses() {
        let graph = make_graph();
        let visit_from_one = graph.dfs("one", |_| {});
        assert_eq!(visit_from_one.len(), 2);
        assert!(visit_from_one.contains_key("two"));
        assert!(!visit_from_one.contains_key("zero"));

        let sum_of_parents = graph.dfs("zero", |n| n.parents().map(|c| *c).sum::<u32>());
        assert_eq!(sum_of_parents.len(), 3);
        assert_eq!(sum_of_parents["zero"], 0);
        assert_eq!(sum_of_parents["two"], 1);

        let mut post_order = vec![];
        graph.dfs_postorder("zero", |n| post_order.push(*n));
        assert_eq!(post_order, vec![2, 1, 0]);
        assert_eq!(
            graph.nodes_postorder().map(|n| *n).collect::<Vec<_>>(),
            vec![2, 1, 0]
        );

        // Using the verbose DFS api
        let mut subtree_sum = HashMap::new();
        graph.dfs_inner(
            &"zero",
            &mut |n| *n,
            &mut |n, acc| {
                *acc.get_mut(n.idx).unwrap() += n.children().map(|c| acc[c.idx]).sum::<u32>();
            },
            &mut subtree_sum,
        );
        // two is counted twice
        assert_eq!(subtree_sum["zero"], 5);
    }
}
