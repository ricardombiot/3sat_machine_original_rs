pub type Step = i32;
pub type Index = i32;
pub type NodeId = (Step, Index);
pub type PathNodeId = (NodeId, NodeId);
pub type OptionalNodeId = Option<NodeId>;

use std::collections::HashSet;

pub type SetPathNodesId = HashSet<PathNodeId>;

pub fn create_new_path_id(id : NodeId, parent_id : OptionalNodeId) -> PathNodeId {
    let parent_id = match parent_id {
        None => (-1,0),
        Some(value_parent_id) => value_parent_id
    };

    let path_node_id : PathNodeId = (id, parent_id);
    return path_node_id;
}