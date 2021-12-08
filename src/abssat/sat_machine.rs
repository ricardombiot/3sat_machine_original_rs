
use crate::abssat::utils::alias::Step;
use crate::abssat::gpath::gpow::GPow;
pub type OptionalGPow = Option<GPow>;

pub type Timeline = [OptionalGPow;7];

#[derive(Debug, Clone)]
pub struct SatMachine {
    timeline : Timeline,
    current_step : Step,
    n_vars : i32,
    is_valid : bool
}

fn init_timeline() -> Timeline {
    let timeline : Timeline = [None,None,None,None,None,None,None];
    return timeline;
}

mod initialize;
mod getters;
mod literals;
mod step_or;