// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            // 使用 is_empty() 更符合 Rust 习惯
            return Person::default();
        } else {
            let vec_: Vec<&str> = s.split(",").collect();
            if vec_.len() != 2 || vec_[0].is_empty() {
                return Person::default();
            } else {
                let name_ = vec_[0];
                let age_ = vec_[1];
                let age_number = match age_.parse::<usize>() {
                    Ok(age_number) => age_number,
                    Err(_) => {
                        return Person::default();
                    }
                };

                Person {
                    name: name_.to_string(),
                    age: age_number,
                }
            }
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}

// From 特性用于值到值的转换.如果 From 针对某个类型被正确实现,那么 Into 特性应该反向生效.
// 你可以在 https://doc.rust-lang.org/std/convert/trait.From.html 阅读更多相关内容.
// 我们实现了 Default 特性,以便在提供的字符串无法转换为 Person 对象时作为备用选项.

// 你的任务是完成这个实现,使得代码行 `let p = Person::from("Mark,20")` 能够编译通过.
// 请注意,你需要将年龄部分解析为 `usize`,可以使用类似 `"4".parse::<usize>()` 的方法.
// 需要适当地处理解析的结果.

//
// 步骤：
// 1. 如果提供的字符串长度为 0,则返回 Person 的默认值.
// 2. 将给定的字符串按逗号分隔.
// 3. 从分隔结果中提取第一个元素作为姓名.
// 4. 如果姓名为空,则返回 Person 的默认值.
// 5. 从分隔结果中提取另一个元素并将其解析为 `usize` 作为年龄.
//    如果解析年龄时出现问题,则返回 Person 的默认值.
//    否则,返回一个使用解析结果实例化的 Person 对象.
