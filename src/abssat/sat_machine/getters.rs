use crate::abssat::gpath::gpow::GPow;
use crate::abssat::utils::alias::Step;
use crate::abssat::sat_machine::SatMachine;

impl SatMachine {

    pub fn get_current_step(&self) -> Step {
        return self.current_step;
    }

    pub fn get_gpath(&self, index : usize) -> Option<&GPow> {
        return self.timeline[index].as_ref();
    }

    pub fn get_gpath_fusion(&self) -> &GPow {
       return self.timeline[0].as_ref().unwrap()
    }

}