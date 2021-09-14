use num::Integer;

pub mod math_helper {
    pub fn add_two_digits<T: num::Integer>(a: T, b: T) -> T {
        a + b
    }

    pub fn first_is_greater(a: i32, b: i32) -> bool {
        a > b
    }

    pub fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        }
        else {
            b
        }
    }

    /// tests if argument(i32) is even
    ///
    /// # Example
    ///
    /// let a = 11;
    ///
    /// assert!(is_even(a), true, "x wasn't true!");
    ///
    pub fn is_even(a: i32) -> bool {
	    a % 2 == 0
    }

    pub fn array_sum(arr: &[i32]) -> i32 {
	    let mut sum = 0;

	    for e in arr.iter() {
		    sum += e;
	    }
	sum
    }
}

pub mod string_helper {
    pub fn reverse(s: &str) -> String {
        let mut result = String::new();

        for c in s.chars() {
            result = format!("{}{}", c, result);
        }

    	result
    }
}
