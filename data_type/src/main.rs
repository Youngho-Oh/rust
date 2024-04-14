fn main() {
    println!("Hello, world!");
    println!("{}", 10_i8);

    assert_eq!( 10_i8 as u16, 10_u16 );
    assert_eq!( 2525_u16 as i16, 2525_i16 );

    assert_eq!( -1_i16 as i32, -1_i32 );
    assert_eq!( 65535_u16 as i32, 65535_i32 );

    assert_eq!( 1000_i16 as u8, 232_u8 );
    assert_eq!( 65535_u32 as i16, -1_i16 );

    assert_eq!( -1_i8 as u8, 255_u8 );
    assert_eq!( 255_u8 as i8, -1_i8 );

    assert_eq!( 2_u16.pow(4), 16 );
    assert_eq!( (-4_i32).abs(), 4 );
    assert_eq!( 0b101101_u8.count_ones(), 4 );

    // println!("{}", (-4).abs());
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));

    // // let mut i = 1;
    // // loop {
    // //     i *= 10;
    // // }

    // let mut i : i32 = 1;
    // loop {
    //     i = i.checked_mul(10).expect("multiplication oeverflow");
    // }

    // Check method
    assert_eq!( 10_u8.checked_add(20), Some(30) );
    assert_eq!( 100_u8.checked_add(200), None );

    // let x : i8 = 127;
    // let y : i8 = 1;
    // let sum = x.checked_add(y).unwrap();
    
    // wrapping method
    assert_eq!( (-128_i8).checked_div(-1), None);

    assert_eq!( 100_u16.wrapping_mul(200), 20000 );
    assert_eq!( 500_u16.wrapping_mul(500), 53392 );

    assert_eq!( 500_i16.wrapping_mul(500), -12144 );
    assert_eq!( 5_i16.wrapping_shl(17), 10);

    // saturating method
    assert_eq!( 32760_i16.saturating_add(10), 32767 );
    assert_eq!( (-32760_i16).saturating_sub(10), -32768 );

    // overflowing
    assert_eq!( 255_u8.overflowing_sub(2), (253, false) );
    assert_eq!( 255_u8.overflowing_add(2), (1, true) );
}