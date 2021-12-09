use crate::abssat::gpath::gpow::GPow;
use crate::abssat::utils::alias::{Step,PathNodeId,SetNodesId};


#[derive(Debug, Clone)]
pub struct Reader {
    gpath : GPow,
    n_vars : i32,
    solution : Vec<bool>,
    current_step : Step,
    last_selected : Option<PathNodeId>,
    last_requires : Option<SetNodesId>,
    is_finished : bool
}


mod constructor;
mod read;
mod getters;