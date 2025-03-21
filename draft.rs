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