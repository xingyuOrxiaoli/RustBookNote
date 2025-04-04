use std::collections::HashMap;

fn main() {
    println!("第八章 三道练习题");

    println!( "\
    给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）\
    和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）");

    println!( "
    将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加
    “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会
    变成 “apple-hay”）。牢记 UTF-8 编码");

    println!( "
    使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
    例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所
    有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表");

}
