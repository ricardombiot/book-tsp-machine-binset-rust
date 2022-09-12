use crate::tsp::utils::alias::{Color, Km, Step , ActionId, UniqueNodeKey};
use crate::tsp::utils::generator_node_key;
use crate::tsp::utils::generator_ids;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::pathset::components::owners::owners::OwnersByStep;

#[test]
pub fn test_owners_push_and_pop_key_by_step(){
    let n : Color = 4;
    let b_max : Km = 10;
    let bbnnn = generator_node_key::get_max_unique_node_key(n, b_max);

    let mut owners : OwnersByStep = OwnersByStep::new(bbnnn);

    assert!(owners.isempty(0 as Step));


    let action_id : ActionId = generator_ids::get_action_id(n, 1 as Km, 1 as Color);
    let node_root_id = NodeId::new_root(n, b_max, action_id);
    let action_parent_id = action_id;
    let key_expected : UniqueNodeKey = generator_node_key::calc_unique_node_key(n, b_max, 0 as Step, action_id, action_parent_id);
    assert_eq!(node_root_id.key(), key_expected);

    assert!(!owners.have_key(0 as Step ,key_expected));
    assert!(!owners.have(&node_root_id));
    owners.push(&node_root_id);
    //println!("{:?}", owners);
    let not_isempty = !owners.isempty(0 as Step);
    assert!(not_isempty);

    assert!(owners.have_key(0 as Step ,key_expected));
    assert!(owners.have(&node_root_id));
    
    owners.pop(&node_root_id);
    assert!(owners.isempty(0 as Step));
    assert!(!owners.have_key(0 as Step ,key_expected));
    assert!(!owners.have(&node_root_id));

}


/*
        2
/   /         \
5   3         4
|   |         |
4   4         6
\  /          |
  1           1
*/
#[test]
pub fn test_operations_owner(){
    let n : Color = 10;
    let b_max : Km = 20;
    let bbnnn = generator_node_key::get_max_unique_node_key(n, b_max);
    let mut owners_set_a : OwnersByStep = OwnersByStep::new(bbnnn);
    let mut owners_set_b : OwnersByStep = OwnersByStep::new(bbnnn);

    let action_id_s0_2 = generator_ids::get_action_id(n, 0 as Km, 2 as Color);

    let action_id_s1_3 = generator_ids::get_action_id(n, 1 as Km, 3 as Color);
    let action_id_s1_4 = generator_ids::get_action_id(n, 1 as Km, 4 as Color);
    let action_id_s1_5 = generator_ids::get_action_id(n, 1 as Km, 5 as Color);

    let action_id_s2_4 = generator_ids::get_action_id(n, 2 as Km, 4 as Color);
    let action_id_s2_6 = generator_ids::get_action_id(n, 2 as Km, 6 as Color);

    let action_id_s3_1 = generator_ids::get_action_id(n,  3 as Km, 1 as Color);


    let node_id_s0_2_2 : NodeId = NodeId::new_root(n, b_max, action_id_s0_2);

    let node_id_s1_3_2 : NodeId = NodeId::new(n, b_max, 1 as Step, action_id_s1_3, action_id_s0_2);
    let node_id_s1_4_2 : NodeId = NodeId::new(n, b_max, 1 as Step, action_id_s1_4, action_id_s0_2);
    let node_id_s1_5_2 : NodeId = NodeId::new(n, b_max, 1 as Step, action_id_s1_5, action_id_s0_2);


    let node_id_s2_4_5 : NodeId = NodeId::new(n, b_max, 2 as Step, action_id_s2_4, action_id_s1_5);
    let node_id_s2_4_3 : NodeId = NodeId::new(n, b_max, 2 as Step, action_id_s2_4, action_id_s1_3);
    let node_id_s2_6_4 : NodeId = NodeId::new(n, b_max, 2 as Step, action_id_s2_6, action_id_s1_4);

    let node_id_s3_1_4 : NodeId = NodeId::new(n, b_max, 3 as Step, action_id_s3_1, action_id_s2_4);
    let node_id_s3_1_6 : NodeId = NodeId::new(n, b_max, 3 as Step, action_id_s3_1, action_id_s2_6);

    // Subset A
    let subset_a = [
        &node_id_s0_2_2, 
        &node_id_s1_3_2, 
        &node_id_s1_5_2,
        &node_id_s2_4_3,
        &node_id_s2_4_5,
        &node_id_s3_1_4
    ];

    owners_set_a.push(&node_id_s0_2_2);
    owners_set_a.push(&node_id_s1_3_2);
    owners_set_a.push(&node_id_s1_5_2);
    owners_set_a.push(&node_id_s2_4_3);
    owners_set_a.push(&node_id_s2_4_5);
    owners_set_a.push(&node_id_s3_1_4);


    assert_eq!(owners_set_a.count(0 as Step), 1);
    assert_eq!(owners_set_a.count(1 as Step), 2);
    assert_eq!(owners_set_a.count(2 as Step), 2);
    assert_eq!(owners_set_a.count(3 as Step), 1);

    for ref_node_id in subset_a.iter() {
        assert!(owners_set_a.have(*ref_node_id));
    }

    assert!(owners_set_a.valid());
    assert_eq!(owners_set_a.max_step(), 3 as Step);
    // Subset B
    let subset_b = [
        &node_id_s0_2_2,
        &node_id_s1_4_2,
        &node_id_s2_6_4,
        &node_id_s3_1_6
    ];

    owners_set_b.push(&node_id_s0_2_2);
    owners_set_b.push(&node_id_s1_4_2);
    owners_set_b.push(&node_id_s2_6_4);
    owners_set_b.push(&node_id_s3_1_6);

    assert_eq!(owners_set_b.count(0 as Step), 1);
    assert_eq!(owners_set_b.count(1 as Step), 1);
    assert_eq!(owners_set_b.count(2 as Step), 1);
    assert_eq!(owners_set_b.count(3 as Step), 1);

    for ref_node_id in subset_b.iter() {
        assert!(owners_set_b.have(*ref_node_id));
    }

    assert_eq!(owners_set_a.max_step(),owners_set_b.max_step());
    assert!(owners_set_b.valid());
    // Union

    let mut owners_set_a_copy = owners_set_a.clone();
    owners_set_a_copy.union(&owners_set_b);
    assert_eq!(owners_set_a_copy.valid(), true);
    

    for ref_node_id in subset_a.iter() {
        assert!(owners_set_a_copy.have(*ref_node_id));
    }
    for ref_node_id in subset_b.iter() {
        assert!(owners_set_a_copy.have(*ref_node_id));
    }

    assert_eq!(owners_set_a_copy.count(0 as Step), 1);
    assert_eq!(owners_set_a_copy.count(1 as Step), 3);
    assert_eq!(owners_set_a_copy.count(2 as Step), 3);
    assert_eq!(owners_set_a_copy.count(3 as Step), 2);


    // Intersect 
    let mut owners_set_a_copy = owners_set_a.clone();
    owners_set_a_copy.intersect(&owners_set_b);
    assert_eq!(owners_set_a_copy.valid(), false);
    // When is UNVALID all it information is consider incoherente, then is remove it.


    /* 
    // The unique common is the root: s0_2
    for ref_node_id in subset_a.iter() {
        let node_id = *ref_node_id;
        if node_id.eq(&node_id_s0_2_2) {
            assert!(owners_set_a_copy.have(*ref_node_id));
        }else{
            assert_eq!(owners_set_a_copy.have(*ref_node_id), false);
        }
    }
    for ref_node_id in subset_b.iter() {
        let node_id = *ref_node_id;
        if node_id.eq(&node_id_s0_2_2) {
            assert!(owners_set_a_copy.have(*ref_node_id));
        }else{
            assert_eq!(owners_set_a_copy.have(*ref_node_id), false);
        }
    }


    assert_eq!(owners_set_a_copy.count(0 as Step), 1);
    assert_eq!(owners_set_a_copy.count(1 as Step), 0);
    assert_eq!(owners_set_a_copy.count(2 as Step), 0);
    assert_eq!(owners_set_a_copy.count(3 as Step), 0);

    // Intersect Quick (Have loop short when is invalid)
    let mut owners_set_a_copy = owners_set_a.clone();
    owners_set_a_copy.intersect_quick(&owners_set_b);
    assert_eq!(owners_set_a_copy.valid(), false);

    assert_eq!(owners_set_a_copy.count(0 as Step), 1);
    assert_eq!(owners_set_a_copy.count(1 as Step), 0);
    assert_eq!(owners_set_a_copy.count(2 as Step), 2);
    assert_eq!(owners_set_a_copy.count(3 as Step), 1);
    */

}




/*

#=
        2
/   /         \
5   3         4
|   |         |
4   4         6
\  /          |
  1           1
=#
function test_operations_owner()
    n= Color(10)
    b= Km(20)


    action_id_s0_2 = GeneratorIds.get_action_id(n, Km(0), Color(2))

    action_id_s1_3 = GeneratorIds.get_action_id(n, Km(1), Color(3))
    action_id_s1_4 = GeneratorIds.get_action_id(n, Km(1), Color(4))
    action_id_s1_5 = GeneratorIds.get_action_id(n, Km(1), Color(5))

    action_id_s2_4 = GeneratorIds.get_action_id(n, Km(2), Color(4))
    action_id_s2_6 = GeneratorIds.get_action_id(n, Km(2), Color(6))

    action_id_s3_1 = GeneratorIds.get_action_id(n, Km(3), Color(1))

    node_id_s0_2_2 = NodeIdentity.new(n, Step(0), action_id_s0_2, action_id_s0_2)

    node_id_s1_3_2 = NodeIdentity.new(n, Step(1), action_id_s1_3, action_id_s0_2)
    node_id_s1_4_2 = NodeIdentity.new(n, Step(1),action_id_s1_4, action_id_s0_2)
    node_id_s1_5_2 = NodeIdentity.new(n, Step(1),action_id_s1_5, action_id_s0_2)

    node_id_s2_4_5 = NodeIdentity.new(n, Step(2),action_id_s2_4, action_id_s1_5)
    node_id_s2_4_3 = NodeIdentity.new(n, Step(2),action_id_s2_4, action_id_s1_3)
    node_id_s2_6_4 = NodeIdentity.new(n, Step(2),action_id_s2_6, action_id_s1_4)

    node_id_s3_1_4 = NodeIdentity.new(n, Step(3),action_id_s3_1, action_id_s2_4)
    node_id_s3_1_6 = NodeIdentity.new(n, Step(3),action_id_s3_1, action_id_s2_6)

    bbnn = UniqueNodeKey(b^2*n^2)
    owners_set_a = Owners.new(bbnn)
    owners_set_b = Owners.new(bbnn)

    Owners.push!(owners_set_a, Step(0), node_id_s0_2_2)
    Owners.push!(owners_set_a, Step(1), node_id_s1_3_2)
    Owners.push!(owners_set_a, Step(1), node_id_s1_5_2)
    Owners.push!(owners_set_a, Step(2), node_id_s2_4_3)
    Owners.push!(owners_set_a, Step(2), node_id_s2_4_5)
    Owners.push!(owners_set_a, Step(3), node_id_s3_1_4)

    @test Owners.have(owners_set_a, Step(0), node_id_s0_2_2)
    @test Owners.have(owners_set_a, Step(1), node_id_s1_3_2)
    @test Owners.have(owners_set_a, Step(1), node_id_s1_5_2)
    @test Owners.have(owners_set_a, Step(2), node_id_s2_4_3)
    @test Owners.have(owners_set_a, Step(2), node_id_s2_4_5)
    @test Owners.have(owners_set_a, Step(3), node_id_s3_1_4)

    Owners.push!(owners_set_b, Step(0), node_id_s0_2_2)
    Owners.push!(owners_set_b, Step(1), node_id_s1_4_2)
    Owners.push!(owners_set_b, Step(2), node_id_s2_6_4)
    Owners.push!(owners_set_b, Step(3), node_id_s3_1_6)

    @test Owners.have(owners_set_b, Step(0), node_id_s0_2_2)
    @test Owners.have(owners_set_b, Step(1), node_id_s1_4_2)
    @test Owners.have(owners_set_b, Step(2), node_id_s2_6_4)
    @test Owners.have(owners_set_b, Step(3), node_id_s3_1_6)

    # UNION

    owners_set_a_copy = deepcopy(owners_set_a)
    Owners.union!(owners_set_a_copy, owners_set_b)

    @test Owners.have(owners_set_a_copy, Step(0), node_id_s0_2_2)
    @test Owners.have(owners_set_a_copy, Step(1), node_id_s1_3_2)
    @test Owners.have(owners_set_a_copy, Step(1), node_id_s1_5_2)
    @test Owners.have(owners_set_a_copy, Step(2), node_id_s2_4_3)
    @test Owners.have(owners_set_a_copy, Step(2), node_id_s2_4_5)
    @test Owners.have(owners_set_a_copy, Step(3), node_id_s3_1_6)

    @test Owners.have(owners_set_a_copy, Step(0), node_id_s0_2_2)
    @test Owners.have(owners_set_a_copy, Step(1), node_id_s1_4_2)
    @test Owners.have(owners_set_a_copy, Step(2), node_id_s2_6_4)
    @test Owners.have(owners_set_a_copy, Step(3), node_id_s3_1_6)

    @test Owners.count(owners_set_a_copy, Step(0)) == 1
    @test Owners.count(owners_set_a_copy, Step(1)) == 3
    @test Owners.count(owners_set_a_copy, Step(2)) == 3
    @test Owners.count(owners_set_a_copy, Step(3)) == 2

    # intersect!

    owners_set_a_copy = deepcopy(owners_set_a)
    Owners.intersect!(owners_set_a_copy, owners_set_b)

    @test Owners.have(owners_set_a_copy, Step(0), node_id_s0_2_2) == true
    @test Owners.have(owners_set_a_copy, Step(1), node_id_s1_3_2) == false
    @test Owners.have(owners_set_a_copy, Step(1), node_id_s1_5_2) == false
    @test Owners.have(owners_set_a_copy, Step(2), node_id_s2_4_3) == false
    @test Owners.have(owners_set_a_copy, Step(2), node_id_s2_4_5) == false
    @test Owners.have(owners_set_a_copy, Step(3), node_id_s3_1_6) == false

    @test Owners.have(owners_set_a_copy, Step(0), node_id_s0_2_2) == true
    @test Owners.have(owners_set_a_copy, Step(1), node_id_s1_4_2) == false
    @test Owners.have(owners_set_a_copy, Step(2), node_id_s2_6_4) == false
    @test Owners.have(owners_set_a_copy, Step(3), node_id_s3_1_6) == false

    @test Owners.count(owners_set_a_copy, Step(0)) == 1
    @test Owners.count(owners_set_a_copy, Step(1)) == 0
    @test Owners.count(owners_set_a_copy, Step(2)) == 0
    @test Owners.count(owners_set_a_copy, Step(3)) == 0

    @test owners_set_a_copy.valid == false

    #=
    # diff! a / b

    owners_set_a_copy = deepcopy(owners_set_a)
    Owners.diff!(owners_set_a_copy, owners_set_b)

    @test Owners.have(owners_set_a_copy, Step(0), node_id_s0_2_2) == false
    @test owners_set_a_copy.valid == false

    # diff! b / a

    owners_set_b_copy = deepcopy(owners_set_b)
    Owners.diff!(owners_set_b_copy, owners_set_a)

    @test Owners.have(owners_set_b_copy, Step(0), node_id_s0_2_2) == false
    @test owners_set_b_copy.valid == false
    =#
end
*/