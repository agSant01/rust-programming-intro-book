#[cfg(test)]
mod tests {
    #[test]
    fn conditionals() {
        let i = 20;

        if i < 2 {
            assert!(i < 2);
        } else if i > 2 {
            assert!(i > 2);
        } else {
            assert_eq!(1, 2);
        }
    }

    #[test]
    fn more_conditionals() {
        let my_option = Some(10);

        // if statements can do simple pattern matching
        if let Some(unpacked) = my_option {
            println!("{}", unpacked);
            assert_eq!(unpacked, 10);
        }

        let mut other_option = Some(2);
        // there is also a while let, which does the same thing
        while let Some(unpacked) = other_option {
            other_option = if unpacked > 0 {
                Some(unpacked - 1)
            } else {
                None
            }
        }

        assert_eq!(other_option, None);
    }
    #[test]
    fn loops() {
        let mut i = 42;
        let mut broke = false;

        // basic loop with control statements
        loop {
            i -= 1;
            if i < 2 {
                broke = true;
                break;
            } else {
                if i > 2 {
                    continue;
                }
            }
        }

        assert!(broke);

        //loops can be named for better readability...
        'outer: loop {
            'inner: loop {
                break 'inner; // ... and specifically jump out of
            }
            break 'outer;
        }

        let mut iterations = 0_u32;
        let total_squared = loop {
            iterations += 1;
            if iterations >= 10 {
                break iterations.pow(2);
            }
        };
        assert_eq!(total_squared, 100);

        for i in 0..10 {
            assert!(i >= 0 && i <= 10);
        }

        for v in vec![1, 1, 1, 1].iter() {
            assert_eq!(v, &1);
        }
    }
}
