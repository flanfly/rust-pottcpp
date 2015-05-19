enum NumberOrText {
	Number(i32),
	Text(String)
}

fn print_number_or_text(nt: NumberOrText) {
	match nt {
		NumberOrText::Number(i) => println!("Number: {}",i),
		NumberOrText::Text(t) => println!("Text: {}",t)
	}
}

fn main() {
    let a: NumberOrText = NumberOrText::Number(42);
    let b: NumberOrText = NumberOrText::Text("Hello, World".to_string());

    // Prints "Number: 42"
    print_number_or_text(a);

    // Prints "Text: Hello, World"
    print_number_or_text(b);
}
