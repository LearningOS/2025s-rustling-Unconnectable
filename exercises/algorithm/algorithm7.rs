/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
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
    fn len(&self) -> usize {
        self.size
    }
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        // TODO
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
        //None
    }
    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter {
            stack: Vec::new(),
        };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut {
            stack: Vec::new(),
        };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}
struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn is_left(c: char) -> bool {
    matches!(c, '(' | '[' | '{')
}
fn is_right(c: char) -> bool {
    matches!(c, ')' | ']' | '}')
}
// 判断左右括号是否匹配
fn is_match(left: char, right: char) -> bool {
    matches!((left, right), ('(', ')') | ('[', ']') | ('{', '}'))
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack_ = Stack::new();
    //println!("Processing: {}", bracket);

    for c in bracket.chars() {
        if is_left(c) {
            stack_.push(c);
            //println!("Push '{}', stack: {:?}", c, stack_.data);
        } else if is_right(c) {
            if stack_.is_empty() {
                println!("_!_into R and S is emp{}", c);
                return false;
            }
            if let Some(top) = stack_.pop() {
                //println!("Pop '{}', match with '{}'", top, c);
                if !is_match(top, c) {
                    println!("__!!__Mismatch: '{}' and '{}'", top, c);
                    return false;
                }
            }
        }
        println!("Stack size is {}", stack_.size);
        //然后这里发现是因为他的pop函数没实现...
    }

    let is_empty = stack_.is_empty();
    if !is_empty {
        println!("___!!!___Stack not empty at end: {:?}", stack_.data);
    }
    is_empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
