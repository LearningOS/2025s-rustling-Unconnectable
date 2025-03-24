struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }
}

fn is_left(c: char) -> bool {
    matches!(c, '(' | '[' | '{')
}

fn is_right(c: char) -> bool {
    matches!(c, ')' | ']' | '}')
}

fn is_match(left: char, right: char) -> bool {
    matches!((left, right), ('(', ')') | ('[', ']') | ('{', '}'))
}

fn bracket_match(bracket: &str) -> bool {
    if bracket.len() % 2 != 0 { // 可选优化：长度检查
        return false;
    }

    let mut stack_ = Stack::new();

    for c in bracket.chars() {
        if is_left(c) { // 左括号：压入栈
            stack_.push(c);
        } else if is_right(c) { // 右括号：检查匹配
            if stack_.is_empty() { // 栈为空，无左括号匹配
                return false;
            }
            if let Some(top) = stack_.pop() { // 弹出栈顶
                if !is_match(top, c) { // 检查是否匹配
                    return false;
                }
            }
        }
        // 非括号字符（如数字、字母）直接忽略
    }

    stack_.is_empty() // 栈为空表示所有括号匹配成功
}

#[test]
fn bracket_matching_1() {
    let s = "(2+3){func}[abc]";
    assert_eq!(bracket_match(s), true);
}

#[test]
fn bracket_matching_3() {
    let s = "{{([])}}";
    assert_eq!(bracket_match(s), true);
}

fn main() {
    let s1 = "(2+3){func}[abc]";
    let s3 = "{{([])}}";
    println!("{}: {}", s1, bracket_match(s1)); // true
    println!("{}: {}", s3, bracket_match(s3)); // true
}