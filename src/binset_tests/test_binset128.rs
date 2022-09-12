use crate::binset::binset128::BinSet128;
use crate::binset::numeric_set::{TNumericSet,UniqueKeyNumber};

#[test]
fn test_indexs(){
    let key = 0; 
    let index = BinSet128::get_index_key(key);
    assert_eq!(index, 0);

    let key = 127; 
    let index = BinSet128::get_index_key(key);
    assert_eq!(index, 127);

    let key = 128; 
    let index = BinSet128::get_index_key(key);
    assert_eq!(index, 0);

    let key = 129; 
    let index = BinSet128::get_index_key(key);
    assert_eq!(index, 1);

    let key = 128+128; 
    let index = BinSet128::get_index_key(key);
    assert_eq!(index, 0);

    let key = 128*4+10; 
    let index = BinSet128::get_index_key(key);
    assert_eq!(index, 10);

    let key = 2500; 
    // Expected Position: 2500 - trunk(2500/128) * 128
    let index = BinSet128::get_index_key(key);
    assert_eq!(index, 68);

    let max = 10000;
    let mut position_expected = 0;
    for n in 0..max {
        let key = n;     
        let index = BinSet128::get_index_key(key);
        assert_eq!(index, position_expected);

        if position_expected == 127 {
            position_expected = 0;
        }else{
            position_expected += 1;
        }
    }
}

 
#[test]
fn test_push_first(){
    let mut set = BinSet128::new();
    let key = 0 as UniqueKeyNumber;
    set.push(key);

    let content = format!("{}", set);
    assert_eq!(content, "0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    assert!(set.have(key));
    assert_eq!(set.count(), 1);
}

#[test]
fn test_push_last_first(){
    let mut set = BinSet128::new();
    let key = 1 as UniqueKeyNumber;
    set.push(key);

    let content = format!("{}", set);
    assert_eq!(content, "0b01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    assert!(set.have(key));
    assert_eq!(set.count(), 1);
}

#[test]
fn test_push_last(){
    let mut set = BinSet128::new();
    let key = 127 as UniqueKeyNumber;
    set.push(key);

    let content = format!("{}", set);
    assert_eq!(content, "0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
    assert!(set.have(key));
    assert_eq!(set.count(), 1);
}

#[test]
fn test_push_pre_last(){
    let mut set = BinSet128::new();
    let key = 126 as UniqueKeyNumber;
    set.push(key);

    let content = format!("{}", set);
    assert_eq!(content, "0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010");
    assert!(set.have(key));
    assert_eq!(set.count(), 1);
}

#[test]
fn test_union(){
    let mut set_a = BinSet128::new();
    set_a.push(0 as UniqueKeyNumber);

    let mut set_b = BinSet128::new();
    set_b.push(127 as UniqueKeyNumber);

    assert_eq!(set_a.have(0 as UniqueKeyNumber),true);
    assert_eq!(set_a.have(127 as UniqueKeyNumber),false);
    assert_eq!(set_a.count(), 1);
    assert_eq!(set_b.have(0 as UniqueKeyNumber),false);
    assert_eq!(set_b.have(127 as UniqueKeyNumber),true);
    assert_eq!(set_b.count(), 1);

    set_a.union(&set_b);
    assert_eq!(set_a.have(0 as UniqueKeyNumber),true);
    assert_eq!(set_a.have(127 as UniqueKeyNumber),true);
    assert_eq!(set_a.count(), 2);
}

#[test]
fn test_union_odd_with_even(){
    let mut set_odd = BinSet128::new();
    let mut set_even = BinSet128::new();
    for i in 0..128 {
        if i % 2 != 0{
            set_odd.push(i as UniqueKeyNumber);
        }else{
            set_even.push(i as UniqueKeyNumber);
        } 
    }

    for i in 0..128 {
        if i % 2 != 0{
            assert_eq!(set_odd.have(i as UniqueKeyNumber),true);
        }else{
            assert_eq!(set_odd.have(i as UniqueKeyNumber),false);
        } 
    }

    for i in 0..128 {
        if i % 2 != 0{
            assert_eq!(set_even.have(i as UniqueKeyNumber),false);
        }else{
            assert_eq!(set_even.have(i as UniqueKeyNumber),true);
        } 
    }

    let mut set_union = set_odd.clone();
    set_union.union(&set_even);
    for i in 0..128 {
        assert_eq!(set_union.have(i as UniqueKeyNumber),true);
    }
    assert_eq!(set_union.count(), 128);
}

#[test]
fn test_intersection(){
    let mut set_a = BinSet128::new();
    set_a.push(0 as UniqueKeyNumber);

    let mut set_b = BinSet128::new();
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
fn test_intersection_full_to_odd(){
    let mut set_odd = BinSet128::new();
    let mut set_full  = BinSet128::new();
    for i in 0..128 {
        if i % 2 != 0{
            set_odd.push(i as UniqueKeyNumber);
        } 
        set_full.push(i as UniqueKeyNumber);
    }

    for i in 0..128 {
        if i % 2 != 0{
            assert_eq!(set_odd.have(i as UniqueKeyNumber),true);
        }else{
            assert_eq!(set_odd.have(i as UniqueKeyNumber),false);
        } 
        assert_eq!(set_full.have(i as UniqueKeyNumber),true);
    }

    let mut restore_set_odd = set_full.clone();
    restore_set_odd.intersection(&set_odd);
    assert_eq!(restore_set_odd, set_odd);

    let content = format!("{}", restore_set_odd);
    assert_eq!(content, "0b01010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101");
}


#[test]
fn test_intersection_full_to_even(){
    let mut set_even = BinSet128::new();
    let mut set_full  = BinSet128::new();
    for i in 0..128 {
        if i % 2 == 0{
            set_even.push(i as UniqueKeyNumber);
        } 
        set_full.push(i as UniqueKeyNumber);
    }

    for i in 0..128 {
        if i % 2 == 0{
            assert_eq!(set_even.have(i as UniqueKeyNumber),true);
        }else{
            assert_eq!(set_even.have(i as UniqueKeyNumber),false);
        } 
        assert_eq!(set_full.have(i as UniqueKeyNumber),true);
    }

    let mut restore_set_even = set_full.clone();
    restore_set_even.intersection(&set_even);
    assert_eq!(restore_set_even, set_even);

    let content = format!("{}", restore_set_even);
    assert_eq!(content, "0b10101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010");
}

#[test]
fn test_eq(){
    let mut set_a = BinSet128::new();
    let mut set_b = BinSet128::new();
    set_a.push(0 as UniqueKeyNumber);

    let is_equal = set_a == set_b;
    assert_eq!(is_equal, false);
    set_b.push(0 as UniqueKeyNumber);


    let is_equal = set_a == set_b;
    assert_eq!(is_equal, true);
}