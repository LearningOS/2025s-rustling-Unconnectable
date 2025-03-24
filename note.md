# rustlings

## match

如何使用`match`

```rust
match value {
    EnumName::Variant1(type1_param1, type1_param2, ...) => {
        // 处理 Variant1 的逻辑
    }
    EnumName::Variant2(type2_param) => {
        // 处理 Variant2 的逻辑
    }
    EnumName::Variant3 { field1: type3_field1, field2: type3_field2, ... } => {
        // 处理 Variant3 的逻辑
    }
    _ => {
        // 默认情况,处理未匹配的变体
    }
}

```

### 作为 var

```rust
pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let string_ = match command {
                Command::Uppercase => {
                    string.to_uppercase()
                }
                Command::Trim => {
                    string.trim().to_string()
                }
                Command::Append(n_times_) => {
                    string.to_strbing() + &"bar".repeat(*n_times_)
                }
            };
            output.push(string_);
        }
        output
    }
```

## `string和&str切片常见函数`

```rust
把`str`转换为`string`
String::from(input.trim()) //这里的trim是&str的函数

```

## 哈希表定义

```rust
let mut var_ = HashMap::new();//自动推断类型
let mut var__ = HashMap::<i32,String>::new();//规定类型
```

检查是否有 value,有则跳过,没有就插入

```rust
let mut scores = HashMap::new();
scores.entry(String::from("Blue")).or_insert(50);
```

直接插入

```rust
let mut hash_ = HashMap::new();
hash_.insert(String::from("Sharon"),999999);
```

根据已经有的 Hashvalue 修改现在的 value,使用传递闭包

```rust
    scores.entry(team_1_name).and_modify(|team|{
        team.goals_scored+= team_1_score;
        team.goals_conceded+= team_2_score;
    }).or_insert(
        Team {
            goals_scored: team_1_score ,
            goals_conceded: team_2_score ,
        },
    );
    scores.entry(team_2_name).and_modify(|team|{
        team.goals_scored += team_2_score;
        team.goals_conceded += team_1_score;
    }).or_insert(
        Team {
            goals_scored: team_2_score,
            goals_conceded: team_1_score,
        },
    );
```

## `Option`

注意这里的 Vec 的成员是`Some(i)`,并且 pop 会返回一个`Option`,因此检查需要 Some(pop),也就是 Some(Some(i)),内层是检查是是否存在,外层检查时候是`i8`

```rust
 fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
```

---

## `Option`和`Result`的转换

### 1. `ok_or`

```rust
let result = some_value.ok_or("Error: value is None"); // Ok(42)

let none_value: Option<i32> = None;
let result = none_value.ok_or("Error: value is None"); // Err("Error: value is None")
```

2. `ok_or_else`使用闭包传递

```rust
let some_value: Option<i32> = Some(42);
let result = some_value.ok_or_else(|| "Error: value is None".to_string()); // Ok(42)

let none_value: Option<i32> = None;
let result = none_value.ok_or_else(|| {
    // 动态生成错误值
    let error_message = format!("Error: value is None at time {}", chrono::Local::now());
    error_message
}); // Err("Error: value is None at time 2023-10-05 12:34:56")

```

Err 的写法

```rust
//1.
let qty = item_quantity.parse::<i32>();
Ok(x * cost_per_item + processing_fee)

//2.
    let qty = item_quantity.parse::<i32>();
    match qty{
        Ok(x) =>{
            Ok(x * cost_per_item + processing_fee)
        }
        Err(E) =>{
            Err(e)
        }
    }

//3.
    let qty = item_quantity.parse::<i32>();
    match qty{
        Ok(x) =>{
            Ok(x * cost_per_item + processing_fee)
        }
        _ =>{
            Err(qty.unwrap_err())
        }
    }

```

一个简单的泛型

```rust

struct Wrapper <T>{
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

### **`trait`** 的基本语法

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) -> Self{
    //fn append_bar(mut self) -> String{ 这里也可以是String因为类型是String
        self.push_str("Bar"); //也可以是 self += "Bar";
        self
    }
}
```

### `trait`对不同的类型实现

```rust
// 定义 trait
pub trait trait_name {
    // 定义一个方法,可以有默认实现
    fn func_name(&self) -> String {
        "default implementation".to_string() // 默认实现
    }
}

// 定义第一个结构体
struct struct_name1;

// 为第一个结构体实现 trait
impl trait_name for struct_name1 {
    // 可以选择覆盖默认实现
    fn func_name(&self) -> String {
        "struct_name1 implementation".to_string() // 覆盖默认实现
    }
}

// 定义第二个结构体
struct struct_name2;

// 为第二个结构体实现 trait
impl trait_name for struct_name2 {
    // 使用默认实现
}

// 测试函数
fn main() {
    let instance1 = struct_name1;
    let instance2 = struct_name2;

    println!("struct_name1: {}", instance1.func_name()); // 输出: struct_name1 implementation
    println!("struct_name2: {}", instance2.func_name()); // 输出: default implementation
}
```

### 当函数需要多种行为,比如`clone,copy,display`,可以传入多个 trait 约束

```rust
pub fn notify<T: Summary + Display>(item: T) {
    println!("Display: {}", item); // 调用 Display 的 fmt 方法
    println!("Summary: {}", item.summarize()); // 调用 Summary 的 summarize 方法
}

pub fn process<T: Clone + ToString>(item: T) {
    let cloned_item = item.clone(); // 调用 Clone 的 clone 方法
    let string_repr = item.to_string(); // 调用 ToString 的 to_string 方法
    println!("Cloned: {:?}", cloned_item);
    println!("String: {}", string_repr);
}
```

### 生命周期

看一个实体概念,这里和引用有关系

```rust
fn longest(x: &str, y: &str) -> &str

let s1 = String::from("short");
let result;
{
    let s2 = String::from("longer");
    result = longest(&s1, &s2);
} // s2 在这里被销毁
println!("{}", result); // result 指向的可能是 s2,但 s2 已无效

```

这里的`s2`被销毁,`result`是悬垂指针,这就是为什么上面那个函数看起来好像没问题但是会编译错误的原因

### 另一个不明显的生命周期的例子

```rust
fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };
    println!("{} by {}", book.title, book.author);
}
```

这里的问题是,&name 和 &title 是对局部变量 name 和 title 的引用,而这些引用的生命周期仅限于 main 函数的当前作用域.结构体需要指定生命周期

### 字符串的相加

只能是`String`+`&str`,不能是``String+Sting`或者`&str+&str`

具体例子实现字符串相加

`String+&str`

```rust
let s1 = String::from("Hello, ");
let s2 = "world!";
let s3 = s1 + s2; // String + &str
```

`String+String`

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // String + &str
let s4 = s1.push_str(&s2); //用push-str
// &s2会把s2变成&str
```

`&str+&str`

```rust
//转换为String然后追加
let str1: &str = "hello";
let str2: &str = "world";
let result = String::from(str1) + str2; // 结果是 String 类型\

//format宏
let str1: &str = "hello";
let str2: &str = "world";
let result = format!("{}{}", str1, str2); // 结果是 "helloworld"
```

### 如何遍历哈希表和数组

单个哈希表:

```rust
//使用k_,val_解包key and value
let hash_it = map.iter();
let (mut x1, mut x2, mut x3) = (0, 0, 0);
for (k_, val_) in map.iter() {
    match val_ {
        Progress::None => {
            x1 += 1;
        }
        Progress::Some => {
            x2 += 1;
        }
        Progress::Complete => {
            x3 += 1;
        }
    }
}
```

遍历一个类型为哈希表的`Vec`

这里是 collection 是 Vec 的切片

```rust
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    //todo!();
    let (mut x1, mut x2, mut x3) = (0, 0, 0);

    for hash_ in collection.iter() {
        for (k_, val_) in hash_.iter() {
            match val_ {
                Progress::None => {
                    x1 += 1;
                }
                Progress::Some => {
                    x2 += 1;
                }
                Progress::Complete => {
                    x3 += 1;
                }
            }
        }
    }
    match value {
        Progress::None => { x1 }
        Progress::Some => { x2 }
        Progress::Complete => { x3 }
    }
}
```

关于宏`marco`

分隔宏之间需要`;`,是的,这很傻逼...

```rust
// 定义一个带参数的宏
macro_rules! say_hello {
    // 匹配无参数的情况
    () => {
        println!("Hello, world!");
    };
    // 匹配带一个参数的情况
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    // 匹配带两个参数的情况
    ($greeting:expr, $name:expr) => {
        println!("{}, {}!", $greeting, $name);
    };
}

fn main() {
    // 使用宏
    say_hello!(); // 输出: Hello, world!
    say_hello!("Alice"); // 输出: Hello, Alice!
    say_hello!("Hi", "Bob"); // 输出: Hi, Bob!
}
```

## `unsafe`

### 前置知识

- 不可变裸指针:`*const T`
- 可变裸指针:`*mut T`

```rust
let x: i32 = 42;
let ptr_1 = &x as *const i32; // 不可变
let ptr_2 = &mut x as *mut i32; // 可变
let ptr_1_num = ptr_1 as usize; //把指针转换为整数表示 也就是0x????????之类的数字
```

- 裸指针是 Rust 中的不安全抽象,不受 Rust 的所有权和借用规则的保护.
- 编译器不会检查裸指针的有效性,因此使用裸指针可能导致未定义行为(UB),例如:
  - 悬垂指针(指向已释放的内存).
  - 空指针(`null`).
  - 数据竞争(多个指针同时修改同一块内存).
- 使用裸指针时,必须手动确保内存安全.
  rust 默认是 pass by value,
  如果想要修改函数的参数的值,必须通过创建指向该 value 的指针

- **引用**:
  - 只能指向有效的、已初始化的内存.
  - 不可变引用(`&T`)允许多个只读访问.
  - 可变引用(`&mut T`)允许唯一的可变访问(独占访问).
  - 引用会自动解引用(通过 Deref trait),因此可以直接访问其指向的值.
- **裸指针**:
  - 可以指向任意内存地址,包括无效的、未初始化的或空的内存.
  - 裸指针没有自动解引用的功能,必须手动解引用(在 `unsafe` 块中).
  - 裸指针可以绕过 Rust 的借用检查器,因此可以用于实现某些低级操作(如自定义内存管理、与 C 代码交互等).

```rust
unsafe fn modify_by_address(mut address: usize) {
    unsafe {
        //  创建一个指向的 address的raw指针来修改值,如果直接修改,address的值是不会变的
        //为什么raw指针,因为传入的参数就是裸指针的整数地址
        let ptr_ = address as *mut u32;
        *ptr_ = 0xaabbccdd;
    }
}
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        unsafe {
            modify_by_address(&mut t as *mut u32 as usize);
        }
        assert!(t == 0xaabbccdd);
    }
}
```

`modify_by_address(&mut t as *mut u32 as usize);`讲解这里的参数

1. `&mut t `:对 t 的可变引用
2. `as *mut u32`:引用转换为原始指针,类型为 u32
3. `as usize`:将原始指针转换为地址的整数表示

同理:`&data.a as *const u128 as usize`

1. `&data.a `:对 `data.a` 的不可变引用
2. `as *mut u32`:引用转换为原始指针,类型为 `u128`
3. `as usize`:将原始指针转换为地址的整数表示

### 还有一些和`Box`以及裸指针有关的操作

1. `Box::into_raw(boxed)`:把`Box<T>`类型的数据转换为裸指针

```rust
//签名
pub fn into_raw(boxed: Box<T>) -> *mut T

let boxed = Box::new(42); // 创建一个 Box
let ptr = Box::into_raw(boxed); // 将 Box 转换为裸指针

// 此时,`boxed` 的所有权已经转移给 `ptr`,内存不会被释放.
```

1. `Box::into_raw(boxed)`:把裸指针的数据转换为`Box<T>`

```rust
//signature
pub unsafe fn from_raw(ptr: *mut T) -> Box<T>

let boxed = Box::new(42); // 创建一个 Box
let ptr = Box::into_raw(boxed); // 将 Box 转换为裸指针

// SAFETY: `ptr` 是通过 `Box::into_raw` 生成的,因此是有效的.
let boxed_again = unsafe { Box::from_raw(ptr) };
println!("Value: {}", boxed_again); // 输出: Value: 42
```

3. `Box::new(value)`:在堆上分配内存，并将值存储在其中。

```rust
//signature
pub fn new(value: T) -> Box<T>

let boxed = Box::new(42); // 在堆上分配内存并存储值
println!("Boxed value: {}", boxed); // 输出: Boxed value: 42
```

| 函数                   | 功能描述                | 使用场景                       |
| :--------------------- | :---------------------- | :----------------------------- |
| `Box::new(value)`      | 在堆上分配内存并存储值  | 创建 `Box`                     |
| `Box::into_raw(boxed)` | 将 `Box` 转换为裸指针   | 需要直接操作裸指针的场景       |
| `Box::from_raw(ptr)`   | 将裸指针转换回 `Box`    | 恢复 `Box` 的所有权            |
| `Box::leak(boxed)`     | 将 `Box` 转换为静态引用 | 需要延长生命周期的场景         |
| `Box::pin(value)`      | 在堆上分配内存并固定值  | 自引用结构或需要固定内存的场景 |

> 看一个对 box 的处理

```rust
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
        //从一个raw point转为为 Box 类型
        let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
        ret.b = Some(String::from("hello"));
        ret
    }
    let data = Box::new(Foo { a: 1, b: None });
    let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };
```

---

## 算法题目

### 实现合并两个链表为一个有序链表

前置知识

```plainttext
function merge(list_a, list_b):
    // 创建一个新的链表来存储合并后的结果
    let merged_list = new LinkedList()

    // 初始化两个指针，分别指向 list_a 和 list_b 的头节点
    let current_a = list_a.start
    let current_b = list_b.start

    // 当两个链表都有节点时，比较并选择较小的节点加入 merged_list
    while current_a is not None and current_b is not None:
        if current_a.val <= current_b.val:
            merged_list.add(current_a.val)
            current_a = current_a.next
        else:
            merged_list.add(current_b.val)
            current_b = current_b.next

    // 如果 list_a 还有剩余节点，将其全部加入 merged_list
    while current_a is not None:
        merged_list.add(current_a.val)
        current_a = current_a.next

    // 如果 list_b 还有剩余节点，将其全部加入 merged_list
    while current_b is not None:
        merged_list.add(current_b.val)
        current_b = current_b.next

    // 返回合并后的链表
    return merged_list
```

### 反转链表

```plaintext
    let reverse_list = new LinkedList()

    let pos_ = List.end

    unsafe{
        while let Some(pos_ptr) = pos_ {
            let node_ = &*a_ptr.as_ptr();
            reverse_list.add(node_.val.clone())
        }
    }

```



### 冒泡排序

```rust
procedure bubbleSort(A : list of sortable items)
    n = length(A)
    for i from 0 to n-1 do
        for j from 0 to n-i-1 do
            if A[j] > A[j+1] then
                swap(A[j], A[j+1])
            end if
        end for
    end for
end procedure

```

### `BST`二搜索树

1. 节点结构
```plaintext
struct Node {
    value: int           // 节点存储的值
    left: Node or null    // 左子节点
    right: Node or null   // 右子节点
}
```
2. 插入操作
```plaintext
function insert(root, value):
    if root is null:
        return new Node(value)  // 如果树为空，创建一个新节点
    if value < root.value:
        root.left = insert(root.left, value)  // 递归插入左子树
    else if value > root.value:
        root.right = insert(root.right, value)  // 递归插入右子树
    return root
```
3. 查找操作
```plaintext

function search(root, value):
    if root is null or root.value == value:
        return root  // 找到目标节点或树为空
    if value < root.value:
        return search(root.left, value)  // 递归查找左子树
    else:
        return search(root.right, value)  // 递归查找右子树
```
4. 删除操作
```plaintext
function delete(root, value):
    if root is null:
        return root  // 树为空，直接返回

    if value < root.value:
        root.left = delete(root.left, value)  // 递归删除左子树
    else if value > root.value:
        root.right = delete(root.right, value)  // 递归删除右子树
    else:
        // 找到要删除的节点
        if root.left is null:
            return root.right  // 只有右子节点，直接返回右子节点
        else if root.right is null:
            return root.left  // 只有左子节点，直接返回左子节点
        else:
            // 有两个子节点，找到右子树的最小值节点
            min_node = find_min(root.right)
            root.value = min_node.value  // 用最小值节点的值替换当前节点
            root.right = delete(root.right, min_node.value)  // 删除右子树的最小值节点
    return root

function find_min(node):
    while node.left is not null:
        node = node.left
    return node
```
5. 遍历操作
中序遍历（左-根-右）
```plaintext
function inorder_traversal(root):
    if root is not null:
        inorder_traversal(root.left)
        print(root.value)
        inorder_traversal(root.right)
```
前序遍历（根-左-右）
```plaintext
function preorder_traversal(root):
    if root is not null:
        print(root.value)
        preorder_traversal(root.left)
        preorder_traversal(root.right)
```
后序遍历（左-右-根）
```plaintext
function postorder_traversal(root):
    if root is not null:
        postorder_traversal(root.left)
        postorder_traversal(root.right)
        print(root.value)
```

```rust
/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if self.root == None {
            self.root = value;
        }
        if value < self.root.value {
            self.root.left = insert(self.left, value);
        } else if value > self.root.value {
            self.root.right = insert(self.right, value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        //查找该元素是否存在
        if self.root == None {
            return false;
        } else if self.root.value == value {
            return true;
        }
        if value < self.root.value {
            search(self.left, value);
        } else if value > self.root.value {
            search(self.right, value);
        }
        //true
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}

```


```rust
impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater | Ordering::Equal => {
                if let Some(right) = &mut self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
        }
    }
}


fn search(&self, value: T) -> bool {
        let mut current = &self.root;  // 从根节点开始
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
            }
        }
        false  // 如果循环结束（current 变为 None），说明没找到
    }
```