use std::io;
use std::process::exit;

fn main() {
    let mut digits: Vec<i32> = vec![];

   	for mut i in 1i32..10 {
   		if digits.len() == 6 {
   			break;
   		}
		print!("I guess: {}\n", i*111111);
		
		let mut input_text = String::new();
    	io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    	let trimmed = input_text.trim();
    	match trimmed.parse::<i32>() {
        	Ok(mut a) => {
       			if a==60 {
        			println!("I guessed it!");
        			exit(0);
        		}
        		let b: i32 = a%10;
        		a-=b;
        		a/=10;
        		for j in 1i32..(a+b+1) {
        			digits.push(i);
        		}
        	},
        	Err(..) => {
        		println!("this was not an integer: {}", trimmed);
        		main();
        	},
    	};
	}

	let mut filler: i32 = 1;
	for i in 1i32..10 {
		if !digits.contains(&i) {
			filler = i;
			break;
		}
	}

	let mut final_result: [i32; 6] = [-1;6];
	for i in 0..6 {
		for f in 0..6 {
			if final_result[f] != -1 {
				continue;
			}
			let mut guess:i32 = filler*111111;
			guess-=(filler-*digits.get(i).unwrap())*(10i32.pow(f as u32));
			println!("I guess: {}", guess);
			let mut input_text = String::new();
    		io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    		let trimmed = input_text.trim();
    		match trimmed.parse::<i32>() {
        		Ok(res) => {
        			if res==10 {
        				final_result[f] = *digits.get(i).unwrap();
        				break;
        			}
        		},
        		Err(..) => {
        			println!("this was not an integer: {}", trimmed);
        			main();
        		},
    		};
    	}
	}
	println!("I guess: {}{}{}{}{}{}", final_result[5], final_result[4], final_result[3], final_result[2], final_result[1], final_result[0]);

	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
    	Ok(res) => {
    		if res==60 {
    			println!("I guessed it!");
    			exit(0);
    		} else {
    			main();
    		}
    	},
    	Err(..) => {
    		println!("this was not an integer: {}", trimmed);
    		main();
    	},
    };
}