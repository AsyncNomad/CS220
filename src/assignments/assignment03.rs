//! Assignment 3: Mastering common programming concepts (2/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 6, 7, 8, and 9.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-03.sh` works fine.
//! See `assignment03_grade.rs` and `/scripts/grade-03.sh` for the test script.
use std::collections::{HashMap, HashSet};
/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}
/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    match day {
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
        _ => DayOfWeek::Mon,
    }
}
/// Custom option type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyOption<T> {
    /// Some value of type `T`.
    MySome(T),
    /// No value.
    MyNone,
}
/// Maps the inner value if the given value is `MySome`; returns `MyNone` otherwise.
pub fn my_map<T, U, F: FnOnce(T) -> U>(v: MyOption<T>, f: F) -> MyOption<U> {
    match v {
        MyOption::MySome(v) => MyOption::MySome(f(v)),
        _ => MyOption::MyNone,
    }
}
/// Maps the inner value if the given value is `MySome`, but with a different type of function; returns `MyNone` otherwise.
pub fn my_and_then<T, U, F: FnOnce(T) -> MyOption<U>>(v: MyOption<T>, f: F) -> MyOption<U> {
    match v {
        MyOption::MySome(v) => f(v),
        _ => MyOption::MyNone,
    }
}
/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(values: Vec<isize>) -> Option<isize> {
    let size = values.len();
    let mut newvalues = values;
    newvalues.sort();
    if size == 0 {
        None
    } else {
        let half: usize = size / 2;
        Some(newvalues[half])
    }
}
/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    let mut result = HashMap::new();
    let size = values.len();
    let mut maxvalue: isize = 0;
    let mut max: isize = 2147483647;
    if size == 0 {
        return None;
    }
    for i in values.iter().take(size) {
        let score = result.entry(*i).or_insert(1);
        *score += 1;
    }
    for (key, value) in result {
        if maxvalue < value {
            maxvalue = value;
            max = key;
        } else {
        }
        if maxvalue == value && max >= key {
            maxvalue = value;
            max = key;
        } else {
        }
    }
    Some(max)
}
/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    let output: Vec<char> = input.chars().collect();
    let size = output.len();
    let mut s: String = String::new();
    let mut index: usize = 0;
    for (i, j) in output.iter().enumerate().take(size) {
        let e: u64 = *j as u64;
        if e == 97 || e == 101 || e == 105 || e == 111 || e == 117 {
            index = i;
            break;
        }
    }
    if index == 0 {
        for i in output.iter().take(size).skip(index) {
            let newchar: String = i.to_string();
            s.push_str(&newchar);
        }
        s.push_str("hay");
    } else {
        for i in output.iter().take(size).skip(index) {
            let newchar: String = i.to_string();
            s.push_str(&newchar);
        }
        for i in output.iter().take(index) {
            let newchar: String = i.to_string();
            s.push_str(&newchar);
        }
        s.push_str("ay");
    }
    s
}
/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add <person> to <department>", "Remove <person> from <department>", and "Move <person> from <department> to <department>".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut company: HashMap<String, HashSet<String>> = HashMap::new();
    let mut commandtype = 0;
    for element in commands {
        if let Some((i, e)) = element.split_whitespace().enumerate().next() {
            if e.eq("Add") {
                commandtype = 1;
            } else if e.eq("Remove") {
                commandtype = 2;
            } else {
                commandtype = 3;
            }
        }
        if commandtype == 1 {
            let mut person: String = String::new();
            let mut dep1: String = String::new();
            for (i, e) in element.split_whitespace().enumerate() {
                if i == 1 {
                    person = e.to_string();
                }
                if i == 3 {
                    dep1 = e.to_string();
                }
            }
            let mut name: HashSet<String> = HashSet::new();
            if company.contains_key(&dep1) {
                name = company.get(&dep1).unwrap().to_owned();
                let b = name.insert(person);
                let c = company.insert(dep1.clone(), name);
            } else {
                let b = name.insert(person);
                let c = company.insert(dep1.clone(), name);
            }
        } else if commandtype == 2 {
            let mut person: String = String::new();
            let mut dep1: String = String::new();
            for (i, e) in element.split_whitespace().enumerate() {
                if i == 1 {
                    person = e.to_string();
                }
                if i == 3 {
                    dep1 = e.to_string();
                }
            }
            if company.contains_key(&dep1) {
                let mut name = company.get(&dep1).unwrap().to_owned();
                let b = name.remove(&person);
                if name.is_empty() {
                    let a = company.remove(&dep1);
                } else {
                    let c = company.insert(dep1.clone(), name);
                }
            }
        } else if commandtype == 3 {
            let mut person: String = String::new();
            let mut dep1: String = String::new();
            let mut dep2: String = String::new();
            for (i, e) in element.split_whitespace().enumerate() {
                if i == 1 {
                    person = e.to_string();
                }
                if i == 3 {
                    dep1 = e.to_string();
                }
                if i == 5 {
                    dep2 = e.to_string();
                }
            }
            if company.contains_key(&dep1) {
                let mut name = company.get(&dep1).unwrap().to_owned();
                let b = name.remove(&person);
                if name.is_empty() {
                    let a = company.remove(&dep1);
                } else {
                    let c = company.insert(dep1.clone(), name);
                }
                let mut name2: HashSet<String> = HashSet::new();
                if company.contains_key(&dep2) {
                    name2 = company.get(&dep2).unwrap().to_owned();
                    let b = name2.insert(person);
                    let c = company.insert(dep2.clone(), name2);
                } else {
                    let b = name2.insert(person);
                    let c = company.insert(dep2.clone(), name2);
                }
            }
        }
    }
    company
}
