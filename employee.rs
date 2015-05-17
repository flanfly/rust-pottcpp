struct Employee {
	name: String,
	age: u8,
	department: String
}

fn main() {
	let e1 = Employee{name:"Kai Michaelis".to_string(),age:29,department:"Engineering".to_string()};
	/*let Employee{ name: n, ..} = e1;

	println!("{}",n);*/

	if let Employee{ age: 67, ..} = e1 {
		println!("Time to retire!");
	} else {
		println!("You still got {} years to go",67 - e1.age);
	}

	match e1 {
		Employee{ 




}
