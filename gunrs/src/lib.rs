use serde_json::{Value};
use serde_json::json;

//use Dup;
mod dup;
mod store;
/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
/*
mod gun {
    mod dup{
        fn check(){}
    }
}
*/

pub struct Gun{
    #[allow(dead_code)]
    dup: dup::Dup,
}

//default setup variable
impl Default for Gun{
    fn default() -> Gun{
        Gun {
            dup: dup::Dup{ 
                //s: vec![],
                s: json!({}), //use serde_json::json;
                opt:  dup::DupOpt{max:1000,age:9000} 
            }
        }
    }
}

impl Gun{

    #[allow(dead_code)]
	pub fn get(&self, _n: &Value) -> Value {
        let mut data = json!({});
        println!("gun get");
        let id = &_n["#"];
        //println!("id :{}",id);
        //data["#"] = serde_json::Value::String(id); //hash id
        //data["#"] = serde_json::from_str(id);
        data["#"] = json!(id); //format json

        data //return json Value
    }

	#[allow(dead_code)]
	pub fn put(&self, _n: &Value){
        println!("gun put");
    }

	#[allow(dead_code)]
	pub fn dup(&self){
		println!("dup");
    }
    
    #[allow(dead_code)]
    pub fn test(&self){
		println!("impl gun test");
	}
}