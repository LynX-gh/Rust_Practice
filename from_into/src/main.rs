use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<Number> for i32 {
    fn from(num: Number) -> Self {
        num.value
    }
}

fn add_into<T: Into<i64>>(a: T, b: T) -> i64 {
    a.into() + b.into()
}

fn add_from<T1, T2>(a: T1, b: T2) -> f64
where
    f64: From<T1> + From<T2>,
{
    f64::from(a) + f64::from(b)
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    let int: i32 = num.into();
    println!("Int number is {}", int);

    println!("4 + 7 = {}", add_into(4_i32, 7_i32));
    println!("4 + 7 = {}", add_from(4_u8, 7_u8));

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // Parsing from String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
