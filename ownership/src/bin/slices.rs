fn main(){
    let two_word=String::from("Hello World");
    let one_word=String::from("Hi");
    let length=first_word_len(&two_word);
    println!("첫단어 길이 length{}",length);
    let slice:&str=&two_word[0..length];
    println!("첫단어{}",slice);
    let length=first_word_len(&one_word);
    println!("전체단어 길이 length{}",length);
    let word_first=first_word(&one_word);
    println!("첫단어(함수){}",word_first);
    // two_word.push_str("")


}


fn first_word_len(s:&String)->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item== b' '{
            return i;
        }
    }
    s.len()
}
fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item== b' '{
            return &s[0..i];
        }
    }
    &s[..]
}