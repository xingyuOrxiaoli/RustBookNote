
fn main() {
    println!("Slice类型");
    let mut s = String::from("hello world");
    // 字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内
    let hello = &s[0..5];
    let world = &s[6..11];
    let word = first_word(&s);
    println!("first word is: {}", hello);
    println!("second word is: {}", world);
    s.clear(); 
    println!("{}", word);
    // 这清空了字符串，使其等于 ""
    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
    
    first_word2("nihao");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s:&str) {
    println!("{}", s);
}