//! Assignment 2: Mastering common programming concepts (1/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 3 and 5.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-02.sh` works fine.
//! See `assignment02_grade.rs` and `/scripts/grade-02.sh` for the test script.

use std::ops::Mul;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub fn fahrenheit_to_celsius(degree: f64) -> f64 {
    (degree - FAHRENHEIT_OFFSET) * FAHRENHEIT_SCALE
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub fn capitalize(input: String) -> String {
    let output: Vec<char> = input.chars().collect();
    let mut s: String = String::new();
    for element in output {
        let e: u64 = element as u64;
        if (97..=122).contains(&e) {
            let newchar: String = element.to_string().to_uppercase();
            s.push_str(&newchar);
        } else {
            let newchar: String = element.to_string();
            s.push_str(&newchar);
        }
    }
    s
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub fn sum_array(input: &[u64]) -> u64 {
    let mut result: u64 = 0;
    for element in input {
        result += element;
    }
    result
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub fn up3(n: u64) -> u64 {
    let mut result: u64 = 1;
    loop {
        if result >= n {
            break;
        } else {
            result *= 3;
        }
    }
    result
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence of integer overflow.)
pub fn gcd(lhs: u64, rhs: u64) -> u64 {
    let a: u64;
    let mut l: u64 = lhs;
    let mut r: u64 = rhs;
    if l < r {
        a = l;
        l = r;
        r = a;
    }
    if r == 0 {
        return l;
    }
    loop {
        let result = l % r;
        if result != 0 {
            l = r;
            r = result;
        } else {
            break;
        }
    }
    r
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial coefficients without integer overflow.
pub fn chooses(n: u64) -> Vec<u64> {
    let mut arr: Vec<Vec<u64>> = vec![vec![1; 70]; 70];

    let n: usize = n as usize;
    let mut a = vec![0; n + 1];

    for i in 1..=67 {
        for j in 1..i {
            arr[i][j] = arr[i - 1][j - 1] + arr[i - 1][j];
        }
    }

    a[..(n + 1)].copy_from_slice(&arr[n][..(n + 1)]);

    a
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`.
/// Here, `3` is ignored because it doesn't have a partner.
pub fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    let size: usize = if lhs.len() > rhs.len() {
        rhs.len()
    } else {
        lhs.len()
    };

    let mut zip: Vec<(u64, u64)> = vec![(0, 0); size];

    for i in 0..size {
        zip[i] = (lhs[i], rhs[i]);
    }
    zip
}

/// 2x2 matrix of the following configuration:
///
/// a, b
/// c, d
#[derive(Debug, Clone, Copy)]
struct Mat2 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

/// 2x1 matrix of the following configuration:
///
/// a
/// b
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    a: u64,
    b: u64,
}

impl Mat2 {
    /// Creates an identity matrix.
    fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        Self {
            a: self.a * rhs.a + self.b * rhs.c,
            b: self.a * rhs.b + self.b * rhs.d,
            c: self.c * rhs.a + self.d * rhs.c,
            d: self.c * rhs.b + self.d * rhs.d,
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let result: Vec2 = Vec2 {
            a: self.a * rhs.a + self.b * rhs.b,
            b: self.c * rhs.a + self.d * rhs.b,
        };
        result
    }
}

impl Mat2 {
    /// Calculates the power of matrix.
    fn power(self, power: u64) -> Mat2 {
        let mut result: Mat2 = Mat2 {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        };
        for i in 0..power {
            result = result.mul(self);
        }
        result
    }
}

impl Vec2 {
    /// Gets the upper value of vector.
    fn get_upper(self) -> u64 {
        self.a
    }
}

/// The matrix used for calculating Fibonacci numbers.
const FIBONACCI_MAT: Mat2 = Mat2 {
    a: 1,
    b: 1,
    c: 1,
    d: 0,
};

/// The vector used for calculating Fibonacci numbers.
const FIBONACCI_VEC: Vec2 = Vec2 { a: 1, b: 0 };

/// Calculates the Fibonacci number. (We assume the absence of integer overflow.)
///
/// Consult <https://web.media.mit.edu/~holbrow/post/calculating-fibonacci-numbers-with-matrices-and-linear-algebra/> for matrix computation of Fibonacci numbers.
pub fn fibonacci(n: u64) -> u64 {
    (FIBONACCI_MAT.power(n) * FIBONACCI_VEC).get_upper()
}

/// Writes down the lyrics of "twelve days of christmas".
///
/// Hint: Google the song title for lyrics and look at the test code for the expected result.
pub fn twelve_days_of_christmas_lyrics() -> String {
    const LYRICS: &str = "On the first day of Christmas, my true love sent to me\nA partridge in a pear tree\n\nOn the second day of Christmas, my true love sent to me\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the third day of Christmas, my true love sent to me\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the fourth day of Christmas, my true love sent to me\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the fifth day of Christmas, my true love sent to me\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the sixth day of Christmas, my true love sent to me\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the seventh day of Christmas, my true love sent to me\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the eighth day of Christmas, my true love sent to me\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the ninth day of Christmas, my true love sent to me\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the tenth day of Christmas, my true love sent to me\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the eleventh day of Christmas, my true love sent to me\nI sent eleven pipers piping\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nOn the twelfth day of Christmas, my true love sent to me\nTwelve drummers drumming\nEleven pipers piping\nTen lords a-leaping\nNine ladies dancing\nEight maids a-milking\nSeven swans a-swimming\nSix geese a-laying\nFive gold rings (five golden rings)\nFour calling birds\nThree French hens\nTwo turtledoves\nAnd a partridge in a pear tree\n\nAnd a partridge in a pear tree\n";
    LYRICS.to_string()
}
