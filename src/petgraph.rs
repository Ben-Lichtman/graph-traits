use petgraph::{
	data::{Build as PetgraphBuild, DataMap as PetgraphDataMap, DataMapMut as PetgraphDataMapMut},
	visit::{Data as PetgraphData, GraphBase as PetgraphGraphBase},
};

use petgraph::{
	graph::{Graph, IndexType},
	visit::EdgeRef,
	EdgeType,
};

use crate::{
	GraphBase, GraphEdgeAddable, GraphEdgeEndpoints, GraphEdgeFrom, GraphEdgeIndexable,
	GraphEdgeMutIndexable, GraphEdgeRemovable, GraphEdgeTo, GraphEdgesFrom, GraphNodeAddable,
	GraphNodeIndexable, GraphNodeMutIndexable, GraphNodeRemovable,
};

impl<T> GraphBase for T
where
	T: PetgraphGraphBase,
{
	type NodeID = T::NodeId;
	type EdgeID = T::EdgeId;
}

impl<N, T> GraphNodeAddable<N> for T
where
	T: PetgraphBuild + PetgraphData<NodeWeight = N>,
{
	fn add_node(&mut self, data: N) -> Self::NodeID { self.add_node(data) }
}

impl<E, T> GraphEdgeAddable<E> for T
where
	T: PetgraphBuild + PetgraphData<EdgeWeight = E>,
{
	fn add_edge(&mut self, a: Self::NodeID, b: Self::NodeID, data: E) -> Self::EdgeID {
		self.add_edge(a, b, data).unwrap()
	}
}

impl<N, T> GraphNodeIndexable<N> for T
where
	T: PetgraphDataMap + PetgraphData<NodeWeight = N>,
{
	fn node(&self, id: Self::NodeID) -> &N { self.node_weight(id).unwrap() }
}

impl<N, T> GraphNodeMutIndexable<N> for T
where
	T: PetgraphDataMapMut + PetgraphData<NodeWeight = N>,
{
	fn node_mut(&mut self, id: Self::NodeID) -> &mut N { self.node_weight_mut(id).unwrap() }
}

impl<E, T> GraphEdgeIndexable<E> for T
where
	T: PetgraphDataMap + PetgraphData<EdgeWeight = E>,
{
	fn edge(&self, id: Self::EdgeID) -> &E { self.edge_weight(id).unwrap() }
}

impl<E, T> GraphEdgeMutIndexable<E> for T
where
	T: PetgraphDataMapMut + PetgraphData<EdgeWeight = E>,
{
	fn edge_mut(&mut self, id: Self::EdgeID) -> &mut E { self.edge_weight_mut(id).unwrap() }
}

// Petgraph does not have some traits which align with some of the traits in this crate therefore we implement per-struct

impl<N, E, Ty, Ix> GraphNodeRemovable<N> for Graph<N, E, Ty, Ix>
where
	Ty: EdgeType,
	Ix: IndexType,
{
	fn remove_node(&mut self, id: Self::NodeID) -> N { self.remove_node(id).unwrap() }
}

impl<N, E, Ty, Ix> GraphEdgeRemovable<E> for Graph<N, E, Ty, Ix>
where
	Ty: EdgeType,
	Ix: IndexType,
{
	fn remove_edge(&mut self, id: Self::EdgeID) -> E { self.remove_edge(id).unwrap() }
}

impl<N, E, Ty, Ix> GraphEdgeTo for Graph<N, E, Ty, Ix>
where
	Ty: EdgeType,
	Ix: IndexType,
{
	fn edge_to(&self, id: Self::EdgeID) -> Self::NodeID { self.edge_endpoints(id).unwrap().1 }
}

impl<N, E, Ty, Ix> GraphEdgeFrom for Graph<N, E, Ty, Ix>
where
	Ty: EdgeType,
	Ix: IndexType,
{
	fn edge_from(&self, id: Self::EdgeID) -> Self::NodeID { self.edge_endpoints(id).unwrap().0 }
}

impl<N, E, Ty, Ix> GraphEdgeEndpoints for Graph<N, E, Ty, Ix>
where
	Ty: EdgeType,
	Ix: IndexType,
{
	fn edge_endpoints(&self, id: Self::EdgeID) -> (Self::NodeID, Self::NodeID) {
		self.edge_endpoints(id).unwrap()
	}
}

impl<N, E, Ty, Ix> GraphEdgesFrom for Graph<N, E, Ty, Ix>
where
	Ty: EdgeType,
	Ix: IndexType,
{
	type EdgesFromOutput = Vec<Self::EdgeID>;

	fn edges_from(&self, id: Self::NodeID) -> Self::EdgesFromOutput {
		self.edges(id).map(|edge_ref| edge_ref.id()).collect()
	}
}
