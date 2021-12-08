pub mod abssat {
    pub mod gpath;
    pub mod utils;
    pub mod sat_machine;
}

#[cfg(test)]
mod test;


fn main() {
    /*
    println!("Hello, world!");

    let mut owners = Owners49::new();

    let root_id = (-1,0);
    let id0 = (0,0);
    let id1 = (0,1);
    let id0_0_root = (id0,root_id);
    let id0_1_root = (id1,root_id);

    owners.push(id0_1_root);

    println!("{:?}", owners);

    assert_eq!(owners.is(id0_0_root),false);
    assert_eq!(owners.is(id0_1_root),true);


    let id1_1__0_0 = ((1,1),id1);

    assert_eq!(owners.is(id1_1__0_0),false);
    owners.push(id1_1__0_0);
    assert_eq!(owners.is(id1_1__0_0),true);
    

    /*
    let a: i64 = 2;
    let mut inbinary = format!("{:#064b}", a);
    let mut chars = inbinary.chars();
    for _x in 0..49 {
        println!("{:?}",chars.next_back().unwrap());
    } */

    println!("{:?}", owners.to_list_step(0));
    
    //println!("{:?}",chars.next_back().unwrap());

    */
}

/*

#[cfg(test)]
mod tests {
    use crate::Owners49;
    #[test]
    fn it_works() {
        let mut owners = Owners49::new();

        let mut a: i64 = 0;
        assert_eq!(format!("{:#064b}", a),"0b00000000000000000000000000000000000000000000000000000000000000");
        //assert_eq!(2 + 2, 4);

        let root_id = (-1,0);
        let id0 = (0,0);
        let id1 = (0,1);
        let id0_0_root = (id0,root_id);
        let id0_1_root = (id1,root_id);
        let (step, index_write) =  Owners49::bit_selection(id0_0_root);
        assert_eq!(step, 0);
        assert_eq!(index_write, 0);

        let (step, index_write) =  Owners49::bit_selection(id0_1_root);
        assert_eq!(step, 0);
        assert_eq!(index_write, 1);

        owners.push(id0_1_root);

        println!("{:?}", owners);

        assert_eq!(owners.is(id0_0_root),false);
        assert_eq!(owners.is(id0_1_root),true);

        let id1_1__0_0 = ((1,1),id1);

        assert_eq!(owners.is(id1_1__0_0),false);
        owners.push(id1_1__0_0);
        assert_eq!(owners.is(id1_1__0_0),true);


    }
}

*/