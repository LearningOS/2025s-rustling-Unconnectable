// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}



// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let vec_s: Vec<&str> = s.split(",").collect();
        if vec_s.len() == 0 {
            return Err(ParsePersonError::Empty);
        }
        if vec_s.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        let name_str = vec_s[0];
        if name_str.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        let age_str = vec_s[1].trim().parse::<usize>();

        let age_value = match age_str {
            Ok(age_value) => age_value,
            Err(e) => {
                return Err(ParsePersonError::ParseInt(e));
            }
        };
        Ok(Person {
            name: name_str.to_string(),
            age: age_value,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::BadLen));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParsePersonError::ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParsePersonError::ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(
            matches!(
                ",".parse::<Person>(),
                Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
            )
        );
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(
            matches!(
                ",one".parse::<Person>(),
                Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
            )
        );
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(ParsePersonError::BadLen));
    }
}

// from_str.rs
//
// 这与 from_into.rs 类似,但这次我们将实现 `FromStr` 特性并返回错误,而不是回退到默认值.
// 此外,实现了 FromStr 后,你可以在字符串上使用 `parse` 方法来生成实现该特性的类型对象.
// 你可以在 https://doc.rust-lang.org/std/str/trait.FromStr.html 阅读更多相关内容.
//
// 执行 `rustlings hint from_str` 或使用 `hint` watch 子命令获取提示.
// 我还没有完成

// 步骤：
// 1. 如果提供的字符串长度为 0,应返回一个错误
// 2. 将给定的字符串按逗号分隔
// 3. 分隔后应只返回 2 个元素,否则返回一个错误
// 4. 从分隔结果中提取第一个元素作为姓名
// 5. 从分隔结果中提取另一个元素,并将其解析为 `usize` 作为年龄,例如使用 `"4".parse::<usize>()`
// 6. 如果在提取姓名和年龄时出现问题,应返回一个错误
// 如果一切顺利,则返回一个 Person 对象的 Result
//
// 附注：`Box<dyn Error>` 实现了 `From<&'_ str>`.这意味着如果你想返回一个字符串错误消息,
// 可以通过简单地使用 `return Err("我的错误消息".into())` 来实现.
