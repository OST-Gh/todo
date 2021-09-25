use std::env;
use std::process;
use std::fs;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use colored::*;

pub struct Config {
    pub workdir: String,
    pub editor: String,
}

impl Config {
    pub fn new () -> Result<Self,String> {
        //Reads EDITOR env variable 
        let editor = match env::var("EDITOR") {
            Ok(e) => e,
            Err(e) => String::from("vim")
        };
        //Reads working directory variable
        let workdir = match env::var("PWD") {
            Ok(w) => w,
            Err(e) => return Err(String::from("Couldn't get working directory")) 
        };

        Ok(Self { editor, workdir})

    }
}
pub struct Todo {
    pub todo: Vec<String>,
}

impl Todo {
    pub fn new () -> Result<Self,String> {

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("TODO")
            .expect("Couldn't open the todofile");

        // Creates a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        // Empty String ready to be filled with TODOs
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();

        // Splits contents of TODO file into a todo vector
        let todo = contents.to_string().lines().map(str::to_string).collect();
        
        // Returns todo
        Ok(Self{todo})
    }


    // Prints every todo
    pub fn list (&self) {
        let mut i = 1;
        
        // This loop will repeat itself for each taks in TODO file
        for task in self.todo.iter() {
            
            // Converts virgin default number into a chad BOLD string
            let number = i.to_string().bold();

            // Saves the symbol of current task
            let symbol = &task[..4];

            // Saves a task without a symbol
            let task = &task[4..];

            // Checks if the current task is completed or not...
            if symbol == "[*] " {
                // DONE
                
                //If the task is completed, then it prints it with a strikethrough 
                println!("{} {}",number, task.strikethrough()); 
            } else if symbol == "[ ] " {
                // NOT DONE

                //If the task is not completed yet, then it will print it as it is
                println!("{} {}",number , task);
            }

                // Increases the i variable by 1
            i = i+1;
        
        }
    }
   
    // Adds a new todo
    pub fn add (&self, element: &str) {
        
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist 
            .append(true) // b) append a line to it
            .open("TODO")
            .expect("Couldn't open the todofile");

        // Appends a new task to the file
        writeln!(todofile, "[ ] {}", element).unwrap();
    }

    // Removes a task
    pub fn remove (&self, element: &str) {
        
        // Converts a string slice into a usize variable
        let element = element.parse::<usize>()
            .expect("Argument must be an integer");

        // Saves the line to be deleted
        let rm = &self.todo[element-1];

        // Creates a new empty string
        let mut newtodo = String::new();

   
        
        for line in self.todo.iter() {
            if line != rm {
                let line = format!("{}\n", line);
                newtodo.push_str(&line[..]);
            }
        }
        
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open("TODO")
            .expect("Couldn't open the todo file");
        
        // Writes contents of a newtodo variable into the TODO file 
        todofile.write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
//        write!(&self.todofile, "{}", newtodo).unwrap();
    }


    pub fn done (&self, element: &str) {
        
        // Converts a string slice into a usize variable
        let element = element.parse::<usize>()
            .expect("Argument must be an integer");

        // Saves the line to be deleted
        let done = &self.todo[element-1];

        // Creates a new empty string
        let mut newtodo = String::new();

   
        
        for line in self.todo.iter() {
            if line.len() > 5 {
                if line != done {
                    let line = format!("{}\n", line);
                    newtodo.push_str(&line[..]);
                } else if line == done {

                    if &line[..4] == "[ ] "{
                        let line = format!("[*] {}\n", &line[4..]);
                        newtodo.push_str(&line[..]);
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        newtodo.push_str(&line[..]);
                    }
        
                }
            }
        }
        
        // Opens the TODO file with a permission to overwrite it
        let mut f = OpenOptions::new()
            .write(true) 
            .open("TODO")
            .expect("Couldn't open the todofile");
        
        // Writes contents of a newtodo variable into the TODO file 
        f.write_all(newtodo.as_bytes()).expect("Error while trying to save the todofile");
//        write!(&self.todofile, "{}", newtodo).unwrap();
    }

    pub fn clear (&self) {
        let mut todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("TODO")
            .expect("Couldn't open the todofile");

        todofile.set_len(0)
            .expect("Couldn't clear the todofile");
    }
}

pub fn help () {
    println!("Usage: todo [OPTION] \"task\"\
    \nSimple todo cli tool\
    \nExample: todo add \"buy apples\"\
    \nCommands:\n
    x
    r
    ");

}
