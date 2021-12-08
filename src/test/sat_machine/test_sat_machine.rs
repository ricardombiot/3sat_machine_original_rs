use std::time::Instant;
use crate::abssat::sat_machine::SatMachine;
use crate::abssat::gpath::path_diagram::PathDiagram;

#[test]
fn test_sat_machine(){
    // IMPORTANT:
    // EXECUTE WITH RELEASE MODE
    // cargo test --release -- --nocapture
    let n_vars = 3;
    
    let now = Instant::now();
    let mut machine = SatMachine::new(n_vars);

    let gpath = machine.get_gpath(0).unwrap();
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("sat_machine_simple_init", "test_visual");

    machine.make_step((1,1,1));
    machine.make_step_fusion();
 
    let clauses = 1;
    assert_eq!(machine.get_current_step(), (n_vars*2)+1+clauses+1 );
    let elapsed_time = now.elapsed();
    println!("Running build_gpath_sat_machine() took {} seconds.", elapsed_time.as_secs());

    //let gpath = machine.get_gpath(0);
    let gpath = machine.get_gpath_fusion();
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("sat_machine_simple", "test_visual");
}