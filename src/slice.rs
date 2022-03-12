pub fn slice() {
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];

  println!("from 0 to 5 = {}, from 6 to 11 = {}", hello, world);

  let first_word = first_word(&s);

  println!("first word in s is: {}", first_word);

  let slice = &s[..];

  println!("You can copy with .. so s = {} and slice = {}", s, slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
