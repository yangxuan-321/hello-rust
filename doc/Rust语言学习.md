# Rust语言学习

### hello world例子
------
hello.rs

```rust
fn mian(){
    println!("hello world!")
}
```



使用 rsutc hello.rs 编译就会生成 hello 的可执行文件

./hello 执行 就可以打印出  hello world!

---------



### rust的编译过程
-----------------

​                             生成AST    ~~>     HIR     ~~>     MIR     ~~>     LLVM     ~~>     机器码

---------------------



### rust包管理工具、构建工具  
-------

cargo

------



### 使用cargo创建一个工程
-----

cargo new hello-rust



会生成一个工程:

hello-rust
├── Cargo.toml
└── src
    └── main.rs



Cargo.toml就是对工程的一个描述信息(工程相关元数据)

```
[package]
name = "hello-rust"
version = "0.1.0"
authors = ["MODA-Master <yangxuan_321@163.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

使用构建工具运行 

在hello-rust目录下运行

cargo run

就会打印出 hello world，并且生成target文件夹下就有可执行文件[./targer/debug/hello-rust]

cargo build只构建不运行

-----


### 使用cargo创建一个工程
