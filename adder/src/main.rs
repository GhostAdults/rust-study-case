fn main() {
    let mut sum = 0;
    loop {
        // 提示用户输入数字
        println!("请输入一个数字（按回车键结束输入）：");

        // 读取每一行输入
        let mut input = Default::default();
        std::io::stdin().read_line(&mut input).unwrap();

        // 如果输入为空行，则停止循环
        if input.trim().is_empty() {
            break;
        }
        // 尝试解析输入为数字并累加
        match input.trim().parse::<i32>() {
            Ok(num) => sum += num,
            Err(_) => println!("输入无效，请输入一个有效的数字！"),
        }
    }

    println!("数字之和: {}", sum);
}
