#[derive(PartialEq)]
enum Fruit {
	Apple = 1,
	Banana = 2,
	Kiwi,
	Pineapple
}

fn say_it(fruit: Fruit) {
	if fruit == Fruit::Apple {
		println!("Apple");
	} else if fruit == Fruit::Banana {
		println!("Banana");
	} else if fruit == Fruit::Kiwi {
		println!("Kiwi");
	} else if fruit == Fruit::Pineapple {
		println!("Pineapple");
	}
}

fn main() {
    say_it(Fruit::Apple);
    say_it(Fruit::Banana);
    say_it(Fruit::Kiwi);
    say_it(Fruit::Pineapple);
}
