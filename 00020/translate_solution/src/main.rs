fn main() {
    let mut hello = String::from("hello");
    println!("en: {}", hello);    
    translate_mutably(&mut hello);
    println!("fr: {}", hello);
}

fn translate_mutably(phrase: &mut String) {
    let translation = translate_immutably(&phrase);
    phrase.clear();
    phrase.push_str(translation.as_str());
}

fn translate_immutably(phrase: &String) -> String {
    match phrase.as_str() {
        "hello" => String::from("bonjour"),
        "goodbye" => String::from("au revoir"),
        _ => phrase.clone()
    }    
}

// test translating hello to bonjour
#[test]
fn test_translate_hello() {
    let mut hello = String::from("hello");
    translate_mutably(&mut hello);
    assert_eq!("bonjour", hello);

    let hello_en = String::from("hello");
    let hello_fr = translate_immutably(&hello_en);
    assert_eq!("hello", hello_en);
    assert_eq!("bonjour", hello_fr);
}

// test translating goodbye to au revoir
#[test]
fn test_translate_goodbye() {
    let mut goodbye = String::from("goodbye");
    translate_mutably(&mut goodbye);
    assert_eq!("au revoir", goodbye);

    let goodbye_en = String::from("goodbye");
    let goodbye_fr = translate_immutably(&goodbye_en);
    assert_eq!("goodbye", goodbye_en);
    assert_eq!("au revoir", goodbye_fr);
}

// test translating unknown strings
#[test]
fn test_translate_unknown() {
    let mut unknown = String::from("unknown");
    translate_mutably(&mut unknown);
    assert_eq!("unknown", unknown);

    let unknown_en = String::from("unknown");
    let unknown_fr = translate_immutably(&unknown_en);
    assert_eq!("unknown", unknown_en);
    assert_eq!("unknown", unknown_fr);
}