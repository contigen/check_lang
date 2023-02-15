use whatlang::detect_lang;

fn main() {
    let eng_text = String::from("Have fun with Rust!.");
    let mandarin_text = String::from("是我的名字");
    let hebrew_text = String::from("איך קוראים לי?");
    let mandarin_text_res = detect_lang(&mandarin_text).unwrap();
    let eng_text_res = detect_lang(&eng_text).unwrap();
    let hebrew_text_res = detect_lang(&hebrew_text).unwrap();
    // let info_lang = info.lang();
    println!("what lang:  {hebrew_text_res} {mandarin_text_res} {eng_text_res}");
}
