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
                    string.to_string() + &"bar".repeat(*n_times_)
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
