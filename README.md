# drawbytes
A rust compiler plugin to allow 'drawing' numeric literals. 

# Examples
```
assert_eq!( draw_u8!( XX_X ), 0b1101_0000 );
assert_eq!( draw_u8!( XX_X___X ), 0b1101_0001 );
```
