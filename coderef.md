Read file
```rust
use std::fs::File;
use std::io::prelude::*;

fn main(){
  let mut file = File::open("info.txt").expect("Can't open file!");

  let mut contents = String::new();
  file.read_to_string(&mut contents)
    .expect("Oops! Can not read the file...");

  println!("File contents:\n\n{}",content);
}
```

argument / parameters
```rust
use std::env;

fn main(){
  let args: Vec<String> = env::args().collect();

  //for argument in args.iter(){
    //println!("{}",argument);
  //}

  println!("{}",args[1]);
}
```

write file
```rust
use std::fs::File;
use std::io::prelude::*;

fn main(){
  let mut file = File::open("info.txt")
    .expect("Could not create file!");

    file.write_all(b"Welcome to decode")
      .expect("Could not werite to file, sorry mate.");
}
```

```rust
extern crate serde_json;

use serde_json::{Value};
use serde_json::json;

use std::collections::HashMap;

#[allow(dead_code)]
struct GunA{
  key:HashMap<String, Value>
}

fn main() {
  println!("Hello World! test");
  let mut myarray = GunA{ key: HashMap::new() };

  println!("len {}",myarray.key.len());

  let v = json!("a string");
  myarray.key.insert("test".to_string(),v);

  println!("len {}",myarray.key.len());

  match myarray.key.get("test") {
    Some(value) => println!("Value {}",value),
    None => println!("None found!")
  }

  match myarray.key.get("tests") {
    Some(value) => println!("Value {}",value),
    None => println!("None found!")
  }
  println!("LIST:");
  for (key, value) in myarray.key{
    println!("KEY: VALUE");
    println!("key: {}",key);
    println!("value: {}",value);
  }
}
```


```rust
extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use serde_derive::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::LinkedList;

#[derive(Serialize, Deserialize)]
pub struct Bottle
{
    name: String,
    capacity: u16,
    total_refills: u64
}

impl Bottle
{
    pub fn new(name: &str, capacity: u16, total_refills: u64) -> Self
    {
        Bottle {
            name: String::from(name),
            capacity: capacity,
            total_refills: total_refills
        }
    }

    pub fn get_capacity(&self) -> u16 { self.capacity }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_refills(&self) -> u64 { self.total_refills }
    pub fn refill(&mut self) { self.total_refills += 1; }
}



fn main()
{
    let ret = get_bottles();
}

pub fn get_bottles() -> Result<()>
{
        let data = r#"
        {
            "name": "Klean Kanteen",
            "capacity": 16,
            "total_refills": 10
        }
        "#;

        let arr_data = r#"
        [
            {
                "name": "Klean Kanteen",
                "capacity": 16,
                "total_refills": 10
            },
            {
                "name": "Yeti",
                "capacity": 21,
                "total_refills": 24
            }
        ]
        "#;

        let bottle: Bottle = serde_json::from_str(data)?; // works
        println!("Name: {}, Capacity: {}, Total Refills {}", bottle.get_name(), bottle.get_capacity(), bottle.get_refills());

		let bottles: LinkedList<Bottle> = serde_json::from_str(arr_data)?; // does not work
		for bott in bottles {
			println!("Name: {}, Capacity: {}, Total Refills {}", bott.get_name(), bott.get_capacity(), bott.get_refills());
		}

        Ok(())
}
```