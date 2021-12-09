use crate::abssat::reader::Reader;
use crate::abssat::utils::alias::Step;

impl Reader {

    pub fn get_stop_step(&self) -> Step {
        return self.n_vars*2 as Step;
    }

    pub fn get_solution(&self) -> &Vec<bool> {
        return &self.solution;
    }

}