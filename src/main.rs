

fn main() {
    //Control Flow
    let number = 3;

    let this_number = 3;

    let another_number = 6;

    //if esle statement
    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    if this_number != 0{
        println!("This number is not zero");
    }
    else{
        println!("This number is zero");
    }

    if another_number % 4 == 0{
        println!("Number is divisible by 4");
    } else if another_number % 3 == 0{
        println!("number is divisible by 3");
    } else if another_number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }

    let condition = true;
    let another_of_this_number = if condition {5} else {6};

    println!("The value of the another_of_this_number is : {another_of_this_number}");

    //Loops

    // loop{
    //    println!("Again");
    // }

    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    
    // label loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut the_number = 3;

    //while loops
    while the_number != 0 {
        println!("{the_number}");
        
        the_number -= 1;
    }

    println!("LIFTOFF!!!!");

    let a = [10,20,30,40,50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}",a[index]);
        
        index += 1;
    }

    // for
    for element in a {
        println!("the value is: {element}");
    }

    //rev
       for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
