/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T> where T: Default {
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> where T: Default {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        let mut current = self.count;

        while current > 1 {
            let parent = self.parent_idx(current);
            let should_swap = !(self.comparator)(&self.items[parent], &self.items[current]);
            if should_swap {
                //if !(self.comparator)(&self.items[parent], &self.items[current]) {
                //std::mem::swap(&mut self.items[parent], &mut self.items[current]);
                self.items.swap(parent, current);
                current = parent;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        //0

        let left_ = self.left_child_idx(idx);
        if left_ > self.count {
            return idx;
        }
        let right_ = self.right_child_idx(idx);
        if right_ > self.count {
            return idx;
        }
        //let should_swap = !(self.comparator)(&self.items[parent], &self.items[current]);
        //if should_swap{
        if (self.comparator)(&self.items[left_], &self.items[right_]) {
            return left_; // Left has higher priority
        } else {
            return right_; // Right has higher priority
        }
    }
    fn heapify_down(&mut self, idx: usize) {
        let mut current = idx;
        loop {
            let smallest = self.smallest_child_idx(current);
            if smallest == current || smallest >= self.count {
                break;
            }
            if !(self.comparator)(&self.items[current], &self.items[smallest]) {
                self.items.swap(current, smallest);
            
                current = smallest;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T> where T: Default + Ord {
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T> where T: Default {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // 使用 swap 将顶部元素与最后一个元素交换
        self.items.swap(1, self.count);
        // 弹出最后一个元素(原来的顶部元素)
        let item = self.items.pop().unwrap(); // 安全,因为已检查非空
        self.count -= 1;
        if self.count > 0 {
            self.heapify_down(1);
        }
        Some(item)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T> where T: Default + Ord {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T> where T: Default + Ord {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
