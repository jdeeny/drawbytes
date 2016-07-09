#![feature(plugin)]
#![plugin(drawbytes)]

#[test]
fn test_draw_u8() {
    assert_eq!( draw_u8!( ____ ), 0b0000_0000 );
    assert_eq!( draw_u8!( ___X ), 0b0001_0000 );
    assert_eq!( draw_u8!( X__X ), 0b1001_0000 );
    assert_eq!( draw_u8!( XXXXXXXX ), 0xFF );
}
