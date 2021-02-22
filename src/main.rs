fn main() {
    println!("Hello, world!");
    // 测试打印
    // {} 作为占位符
    println!("hello world! - {}", "first");
    // {:o} 打印8进制
    println!("打印16进制 - 小写 - {:o}", 16);
    // {:x} 打印16进制 - 小写
    println!("hello world! - {:x}", 255);
    // {:X} 打印16进制 - 大写
    println!("打印16进制 - 大写 - {:X}", 255);
    let str1 = "hello world!";
    println!("打印指针 - {:p}", &str1);
    // -- 其他输出打印 格式 详参 https://doc.rust-lang.org/std/fmt/index.html
}

