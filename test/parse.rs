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
    assert_eq!(s, isg.to_string())
}

#[test]
fn empty_comment() {
    let s = fs::read_to_string("rsc/isg/empty_comment.isg").unwrap();
    let isg = from_str(&s).unwrap();
    assert_eq!(s, isg.to_string())
}

#[test]
fn minified() {
    let s = fs::read_to_string("rsc/isg/example.1.minify.isg").unwrap();
    let minified = from_str(&s).unwrap();

    let s = fs::read_to_string("rsc/isg/example.1.isg").unwrap();
    let expected = from_str(&s).unwrap();
    assert_eq!(minified, expected)
}

#[test]
fn many_space() {
    let s = fs::read_to_string("rsc/isg/example.1.many_space.isg").unwrap();
    let minified = from_str(&s).unwrap();

    let s = fs::read_to_string("rsc/isg/example.1.isg").unwrap();
    let expected = from_str(&s).unwrap();
    assert_eq!(minified, expected)
}
