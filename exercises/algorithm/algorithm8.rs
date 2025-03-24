/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Clone)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    //TODO
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> where T: Clone {
    pub fn new() -> Self {
        Self {
            //TODO
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
        while self.q1.size() > 1 {
            if let Ok(item) = self.q1.dequeue() {
                self.q2.enqueue(item);
            }
        }

        /*
        原理:两个队列
        push:向q1插入元素
        pop:把q1所有的元素插入到q2 直到只剩下一个 也就是第一个元素
        两种写法
        1. 这个时候q2是满的,q1只有一个元素 先取得q1的元素 然后交换 这个时候q1 就是相当于q1是空的队列,q2存储了除了top的所有元素 然后交换

        但是
        !!!
        !!!
        !!!
        如果先取得q1.dequeue()也就是这个唯一的元素 也就是stack的pop,会存在
        原代码的问题是 self.q1.dequeue() 后立即将结果存到 result,然后再调用 swap,导致 self.q1 的借用未结束的问题

        Ans2:
        也就是下面的写法 先交换然后 dequeue
         */
        std::mem::swap(&mut self.q1, &mut self.q2);
        let result = self.q2.dequeue();
        result
    }
    pub fn is_empty(&self) -> bool {
        //TODO
        self.q1.is_empty()
        //true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
