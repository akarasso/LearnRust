fn main() {
    /*
    ** Signed/Unsigned
    */
    let _unsigned_int: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let _signed_int: i32 = 42;

    /*
    ** Signed/Unsigned
    */
    let _signed_int_dec: i32 = 17;
    let _signed_int_hex: i32 = 0x11;
    let _signed_int_oct: i32 = 0o21;
    let _signed_int_bin: i32 = 0b0001_0001;
    let _signed_int_byte: u8 = b'A';

    println!("Values: {}, {}, {}, {}, {}",
        _signed_int_dec,
        _signed_int_hex,
        _signed_int_oct,
        _signed_int_bin,
        _signed_int_byte,
    );

    let _heart_eyed_cat = '😻';
    println!("Values: {}",
        _heart_eyed_cat,
    );

    /*
    ** Tuple
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    // iS equivalatent to
    println!("The value of y is: {}", tup.1);

    /*
    ** Array
    */
    let a = [1, 2, 3, 4, 5];
    // OR let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b: [u32; 1];
    println!("The value of y is: {}", a[1]);
}
