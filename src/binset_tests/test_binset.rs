use crate::binset::binset::BinSet;
use crate::binset::binset128::BinSet128;
use crate::binset::numeric_set::{TNumericSet,UniqueKeyNumber};

#[test]
fn test_push_have_pop(){
    let mut set = BinSet::new();
   

    assert!(set.isempty());
    assert_eq!(set.have(100), false);
    set.push(100);
    assert_eq!(set.have(100), true);
    set.pop(100);
    assert_eq!(set.have(100), false);
}

#[test]
fn test_subset_key(){
    //Example...
    let subset_key = BinSet::get_subset_key(254 as UniqueKeyNumber);
    let index = BinSet128::get_index_key(254 as UniqueKeyNumber);
    assert_eq!(subset_key, 1 as u64);
    assert_eq!(index, 126 as u8);

    let mut expected_subset_key = 0;
    let mut counter_index = 0;
    for i in 0..5000 {
        let subset_key = BinSet::get_subset_key(i);
        let index = BinSet128::get_index_key(i);
        //println!("Subset:{} Index:{} Item({})", subset_key, index, i);
        assert_eq!(subset_key, expected_subset_key);
        assert_eq!(index, counter_index);
        

        counter_index += 1;
        if counter_index == 128 {
            expected_subset_key += 1;
            counter_index = 0;
        }
    }
}




#[test]
fn test_union(){
    let mut set_a = BinSet::new();
    set_a.push(0 as UniqueKeyNumber);

    let mut set_b = BinSet::new();
    set_b.push(127 as UniqueKeyNumber);

    assert_eq!(set_a.have(0 as UniqueKeyNumber),true);
    assert_eq!(set_a.have(127 as UniqueKeyNumber),false);
    assert_eq!(set_b.have(0 as UniqueKeyNumber),false);
    assert_eq!(set_b.have(127 as UniqueKeyNumber),true);

    set_a.union(&set_b);
    assert_eq!(set_a.have(0 as UniqueKeyNumber),true);
    assert_eq!(set_a.have(127 as UniqueKeyNumber),true);
}

#[test]
fn test_union_odd_with_even(){
    let max = 1_000;
    let mut set_odd = BinSet::new();
    let mut set_even = BinSet::new();
    for i in 0..max {
        if i % 2 != 0{
            set_odd.push(i as UniqueKeyNumber);
        }else{
            set_even.push(i as UniqueKeyNumber);
        } 
    }

    for i in 0..max {
        if i % 2 != 0{
            assert_eq!(set_odd.have(i as UniqueKeyNumber),true);
        }else{
            assert_eq!(set_odd.have(i as UniqueKeyNumber),false);
        } 
    }

    for i in 0..max {
        if i % 2 != 0{
            assert_eq!(set_even.have(i as UniqueKeyNumber),false);
        }else{
            assert_eq!(set_even.have(i as UniqueKeyNumber),true);
        } 
    }

    let mut set_union = set_odd.clone();
    set_union.union(&set_even);
    for i in 0..max {
        assert_eq!(set_union.have(i as UniqueKeyNumber),true);
    }
    assert_eq!(set_union.count(), max);
}


#[test]
fn test_intersection(){
    let mut set_a = BinSet::new();
    set_a.push(0 as UniqueKeyNumber);

    let mut set_b = BinSet::new();
    set_b.push(127 as UniqueKeyNumber);

    assert_eq!(set_a.have(0 as UniqueKeyNumber),true);
    assert_eq!(set_a.have(127 as UniqueKeyNumber),false);
    assert_eq!(set_b.have(0 as UniqueKeyNumber),false);
    assert_eq!(set_b.have(127 as UniqueKeyNumber),true);

    set_a.intersection(&set_b);
    assert_eq!(set_a.have(0 as UniqueKeyNumber),false);
    assert_eq!(set_a.have(127 as UniqueKeyNumber),false);
}


#[test]
fn test_eq(){
    let mut set_a = BinSet::new();
    let mut set_b = BinSet::new();
    set_a.push(0 as UniqueKeyNumber);

    let is_equal = set_a == set_b;
    assert_eq!(is_equal, false);
    set_b.push(0 as UniqueKeyNumber);


    let is_equal = set_a == set_b;
    assert_eq!(is_equal, true);
}

#[test]
fn test_eq_big(){
    let mut set_a = BinSet::new();
    let mut set_b = BinSet::new();
    for i in 0..1000 {
        set_a.push(i as UniqueKeyNumber);
    }
    
    let is_equal = set_a == set_b;
    assert_eq!(is_equal, false);
    for i in 0..1000 {
        set_b.push(i as UniqueKeyNumber);
    }

    let is_equal = set_a == set_b;
    assert_eq!(is_equal, true);
}