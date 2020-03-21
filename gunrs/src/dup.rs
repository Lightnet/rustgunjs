use serde_json::{Value};
//use rand;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct DupOpt{
  #[allow(dead_code)]
  pub max:i32,
  #[allow(dead_code)]
  pub age:i32
}

pub struct Dup{

  #[allow(dead_code)]
  //s: Vec<Value>,
  pub s: Value, //use serde_json::{Value};
  #[allow(dead_code)]
  pub opt: DupOpt
}

impl Dup{

  #[allow(dead_code)]
  pub fn check(&self){
  println!("check");
  }

  #[allow(dead_code)]
  pub fn track(&self){
  println!("track");
  }

  #[allow(dead_code)]
  pub fn random(&self)-> String{
      //https://www.reddit.com/r/rust/comments/2s9qzh/how_i_can_generate_random_string_in_rust/
      //https://stackoverflow.com/questions/59553586/how-do-i-generate-a-string-of-random-ascii-printable-characters-with-a-specific
      //let rand_string: String = thread_rng()
          //.sample_iter(&Alphanumeric)
          //.take(32)
          //.collect();
      //println!("{}", rand_string);
      //rand_string
      thread_rng().sample_iter(&Alphanumeric)
              .take(32)
              .collect() //return random 32 letters and numbers
  }
}