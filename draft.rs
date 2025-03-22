pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
  let mut list_merged = LinkedList::new();
  let mut pos_a = list_a.start; // 声明为可变变量
  let mut pos_b = list_b.start; // 声明为可变变量

  unsafe {
      while let (Some(a_ptr), Some(b_ptr)) = (pos_a, pos_b) {
          let a_node = &*a_ptr.as_ptr();
          let b_node = &*b_ptr.as_ptr();

          if a_node.val <= b_node.val {
              list_merged.add(a_node.val);
              pos_a = a_node.next; // 更新 pos_a
          } else {
              list_merged.add(b_node.val);
              pos_b = b_node.next; // 更新 pos_b
          }
      }

      // 处理剩余的 a 链表节点
      while let Some(a_ptr) = pos_a {
          let a_node = &*a_ptr.as_ptr();
          list_merged.add(a_node.val);
          pos_a = a_node.next;
      }

      // 处理剩余的 b 链表节点
      while let Some(b_ptr) = pos_b {
          let b_node = &*b_ptr.as_ptr();
          list_merged.add(b_node.val);
          pos_b = b_node.next;
      }
  }

  list_merged
}