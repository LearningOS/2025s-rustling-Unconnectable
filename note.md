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
        // 默认情况，处理未匹配的变体
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

检查是否有 value,有则跳过，没有就插入

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

注意这里的 Vec 的成员是`Some(i)`,并且 pop 会返回一个`Option`,因此检查需要 Some(pop)，也就是 Some(Some(i)),内层是检查是是否存在，外层检查时候是`i8`

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
    // 定义一个方法，可以有默认实现
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

### 当函数需要多种行为，比如`clone,copy,display`，可以传入多个trait约束

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
println!("{}", result); // result 指向的可能是 s2，但 s2 已无效

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

这里的问题是，&name 和 &title 是对局部变量 name 和 title 的引用，而这些引用的生命周期仅限于 main 函数的当前作用域.结构体需要指定生命周期





### 字符串的相加

只能是`String`+`&str`，不能是``String+Sting`或者`&str+&str`

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

