use crate::abssat::{gpath::gpow::GPow, utils::alias::create_new_path_id};



#[test]
fn test_init_gpow(){

    let mut gpath = GPow::new();

    assert_eq!(gpath.get_map_parent_id().is_none(), true);
    assert_eq!(gpath.get_current_step(), 0);

    let map_node_id = (0,1);
    let path_node_id_expected = create_new_path_id(map_node_id, None);

    assert_eq!(gpath.is(path_node_id_expected), false);
    gpath.do_up(map_node_id);
    assert_eq!(gpath.is(path_node_id_expected), true);

    assert_eq!(gpath.get_map_parent_id().is_none(), false);
    assert_eq!(gpath.get_map_parent_id().unwrap(), map_node_id);
    assert_eq!(gpath.get_current_step(), 1);


    //@test gpath.current_step == Step(0)
}