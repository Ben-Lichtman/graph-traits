#[cfg(feature = "petgraph")]
mod petgraph;

pub trait GraphBase {
	type NodeID: Copy;
	type EdgeID: Copy;
}

pub trait GraphNodeAddable<N>: GraphBase {
	fn add_node(&mut self, data: N) -> Self::NodeID;
}

pub trait GraphEdgeAddable<E>: GraphBase {
	fn add_edge(&mut self, a: Self::NodeID, b: Self::NodeID, data: E) -> Self::EdgeID;
}

pub trait GraphNodeRemovable<N>: GraphBase {
	fn remove_node(&mut self, id: Self::NodeID) -> N;
}

pub trait GraphEdgeRemovable<E>: GraphBase {
	fn remove_edge(&mut self, id: Self::EdgeID) -> E;
}

pub trait GraphNodeIndexable<N>: GraphBase {
	fn node(&self, id: Self::NodeID) -> &N;
}

pub trait GraphNodeMutIndexable<N>: GraphBase + GraphNodeIndexable<N> {
	fn node_mut(&mut self, id: Self::NodeID) -> &mut N;
}

pub trait GraphEdgeIndexable<E>: GraphBase {
	fn edge(&self, id: Self::EdgeID) -> &E;
}

pub trait GraphEdgeMutIndexable<E>: GraphBase + GraphEdgeIndexable<E> {
	fn edge_mut(&mut self, id: Self::EdgeID) -> &mut E;
}

pub trait GraphEdgeTo: GraphBase {
	fn edge_to(&self, id: Self::EdgeID) -> Self::NodeID;
}

pub trait GraphEdgeFrom: GraphBase + GraphEdgeTo {
	fn edge_from(&self, id: Self::EdgeID) -> Self::NodeID;
}

pub trait GraphEdgeEndpoints: GraphBase + GraphEdgeFrom + GraphEdgeTo {
	fn edge_endpoints(&self, id: Self::EdgeID) -> (Self::NodeID, Self::NodeID) {
		(self.edge_from(id), self.edge_to(id))
	}
}

pub trait GraphEdgesFrom: GraphBase {
	type EdgesFromOutput;

	fn edges_from(&self, id: Self::NodeID) -> Self::EdgesFromOutput;
}
