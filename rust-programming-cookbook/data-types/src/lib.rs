// Rust allows another macro type: derive. It allows to "auto-implement"
// supported traits. Clone, Debug, Copy are typically handy to derive.
#[derive(Clone, Debug, Copy)]
struct MyCustomStruct {
    a: i32,
    b: u32,
    pub c: f32,
}
// A typical Rust struct has an impl block for behavior
impl MyCustomStruct {
    // this is a static function
    // by convention of RUST this is the constructor of a struct
    // pub is the identiier for public
    pub fn new(a: i32, b: u32, c: f32) -> MyCustomStruct {
        // returns a new MyCustomStruct struct instance
        // as well as the ownership of it
        // rust can use the `return <value>;` form or a more clean way of doing it is
        // without the return keyword and no semi-colon
        // MyCustomStruct { a:a, b:b, c:c }
        // also if the names of the variables to add we can simplify to
        MyCustomStruct { a, b, c } // more clean
    }

    // instance function.
    // it is instance because it features a self. The `&` indicates that it just uses the reference of self
    // to read the value form memory but not write to it
    pub fn sum(&self) -> f32 {
        // a return statement with semicolon can be used, but it is a convention not to
        // return self.a as f32 + self.b as f32 + self.c;
        self.a as f32 + self.b as f32 + self.c
    }
}

#[cfg(test)]
mod tests {
    use super::MyCustomStruct;
    use std::mem;

    #[test]
    fn test_custom_struct() {
        assert_eq!(
            mem::size_of::<MyCustomStruct>(),
            mem::size_of::<i32>() + mem::size_of::<u32>() + mem::size_of::<f32>()
        );

        let my_cust_struct = MyCustomStruct::new(1, 2, 3_f32);
        assert_eq!(my_cust_struct.a, 1);
        assert_eq!(my_cust_struct.b, 2);
        assert_eq!(my_cust_struct.c, 3_f32);

        assert_eq!(my_cust_struct.sum(), 6_f32);
        let m2 = my_cust_struct.clone();
        assert_eq!(format!("{:?}", m2), "MyCustomStruct { a: 1, b: 2, c: 3.0 }");
        let mut m3 = my_cust_struct;
        m3.a = 100;

        assert_eq!(m2.a, 1);
        assert_eq!(my_cust_struct.a, 1);
        assert_eq!(m3.a, 100);
    }

    #[test]
    fn basic_math_stuff() {
        assert_eq!(2 + 2, 4);

        assert_eq!(3.14 + 22.86, 26_f32);

        assert_eq!(2_i32.pow(2), 4);

        assert_eq!(4_f32.sqrt(), 2f32);
        assert_eq!(4f32.sqrt(), 2_f32);

        // #[allow(unused_variables)]
        let a: u64 = 32;

        let b: u64 = 64;

        // risky, this could overflow
        assert_eq!(b - a, 32);

        assert_eq!(a.overflowing_sub(b), (18446744073709551584, true));

        let mut c = 100;
        c += 1;
        assert_eq!(c, 101);
    }

    #[test]
    #[should_panic]
    fn attempt_overflow() {
        let a = 10_u32;
        let b = 11u32;

        // this will cause panic since the result is going to be overflow
        // Note: _ means to ignore the result
        let _ = a - b;
    }
}
