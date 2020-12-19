// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_value = Some(String::from("rustlings"));
	// TODO: Make this an if let statement whose value is "Some" type
	let mut value: Option<String>;
	if optional_value.is_some()
	{
		value = optional_value;
        println!("the value of optional value is: {}", value.unwrap()   );
	}
	else
	{
		value = None;
        println!("The optional value doesn't contain anything!");
	}
    let mut optional_values_vec: Vec< Option< i8 >> = Vec::new();
	for x in 1..10
	{
        optional_values_vec.push( Some( x ));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
	let mut v = optional_values_vec.pop();
	while v.is_some()
	{
		println!( "current value: {:?}", v.unwrap() );
		v = optional_values_vec.pop();
    }
}
