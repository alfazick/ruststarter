
// PLAN

// 1) Demonstrating How Structs Work: A Funny Example

fn simple_struct(){

    struct Cat {
        name: String,
        age: u8,
    }

    let my_cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };

    println!("My cat's name is {} and it is {} years old.", my_cat.name, my_cat.age);

}

// 2. Explaining Impl Block, &self, and &mut self
// Now, let's add methods to our Cat struct to see how the impl block works, 
// including methods with &self and &mut self.

fn struct_impl_reference(){

    struct Cat {
        name: String,
        age: u8,
    }
    
    impl Cat {
        // A method that borrows self immutably
        fn introduce(&self) {
            println!("Meow! My name is {} and I am {} years old.", self.name, self.age);
        }
    
        // A method that borrows self mutably to update age
        fn have_birthday(&mut self) {
            self.age += 1;
            println!("{} is now {} years old. Happy Birthday!", self.name, self.age);
        }
    }
    
    
    let mut my_cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };

    my_cat.introduce();
    my_cat.have_birthday();
    

}


fn creating_and_writing_to_file(){
    use std::fs::File;
    use std::io::Write;

    
    let mut file = File::create("hello.txt").expect("Could not create file");
    writeln!(file, "Hello, file!").expect("Could not write to file");

}

fn executing_os_commands_linux(){

    use std::process::Command;

    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));

}


fn main() {
    simple_struct();
    struct_impl_reference();

    creating_and_writing_to_file();
    executing_os_commands_linux();
    
}

//ALL TOGETHER :) Enjoy coding


