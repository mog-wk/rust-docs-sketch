extern crate Rectangle as Rect;

#[test]
fn rect_add() {
    let r1 = Rect::new(5, 4);
    let r2 = Rect::new(10, 8);
    let r3 = Rect::new(15, 12);
    assert_eq!(r1 + r1, r2);
    assert_ne!(r1 + r1, r3);
    assert_ne!(r1 + r2, r3);
}
