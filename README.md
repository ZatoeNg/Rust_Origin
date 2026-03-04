# Rust_Origin
* 这是我第一次学习Rust的仓库。

# 学习进度:P51

# &str 和 String
* let wisdom="xxx", 是 &str（字符串切片），数据在只读段、不可修改、无所有权；
* let mut s = String::from("xxx"),是 String 类型，数据在堆上、可修改、有所有权；

# 瘦指针 和 胖指针
* 瘦指针：仅存内存地址（8 字节 / 64 位），用于大小固定的类型（如 &i32、&Struct）；
* 胖指针：存 “地址 + 元数据(长度 / 虚表)”（16 字节 / 64 位），用于动态大小类型（如 &str、&[T]、trait 对象）；
* &str 是典型的胖指针（地址 + 长度），而String是包含胖指针的结构体（地址 + 长度 + 容量）

# while let Ok(byte_read) = reader.read_line(&mut line)
* while let Ok(..)=....,只有reader成功,才会进入while
* let Ok(....) 表示模式匹配写法

```` rust
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    
    //写法1--错误处理
    let config = Config::new(&args)?; // 一行搞定错误处理

    /*写法2--错误处理
    let config_result = Config::new(&args);
    let config = match config_result {
        Ok(c) => c,
        Err(e) => {
        // 失败：返回错误，终止函数
        return Err(e);
        }
    };
    */
    for file in &config.files {
        grep_file(file, &config);
    }
    
    Ok(())
}
````
