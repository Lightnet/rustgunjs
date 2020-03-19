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