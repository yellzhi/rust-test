
//   引用
fn main() {
    let mut s = String::from("Hello, world!");
    let len = calculate_len(&s);
    println!("The length of '{}' is {}.", s, len);
    change(&mut s);
    println!("The length of '{}' is {}.", s, len);

    println!("-------slice--------");
    //let test =String::from("fdshbfaju   cvnfjs vfdsa");
    let test ="fdshbfaju   cvnfjs vfdsa";
    let ret = frist_word(&test);
    println!("{}",ret);
}

fn calculate_len(s : &String) ->usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str(" test");
}

// slice
fn frist_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}