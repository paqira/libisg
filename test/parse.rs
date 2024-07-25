use libisg::from_str;

use std::fs;
#[test]
fn example_1() {
    let s = fs::read_to_string("rsc/isg/example.1.isg").unwrap();
    let isg = from_str(&s).unwrap();
    assert_eq!(s, isg.to_string())
}

#[test]
fn example_2() {
    let s = fs::read_to_string("rsc/isg/example.2.isg").unwrap();
    let isg = from_str(&s).unwrap();
    assert_eq!(s, isg.to_string())
}

#[test]
fn example_3() {
    let s = fs::read_to_string("rsc/isg/example.3.isg").unwrap();
    let isg = from_str(&s).unwrap();
    println!("{:?}", isg.data);
    assert_eq!(s, isg.to_string())
}
