use core::fmt;
use std::future::Future;
use serde_json::Value;

pub mod addition;
pub mod movies;

pub enum Ability {
    Addition(Option<Value>),
    SearchMovies(Option<Value>),
}
impl Ability {
    pub fn list() -> Vec<Ability> {
        vec![
            Ability::Addition(None),
            Ability::SearchMovies(None),
        ]
    }
    pub fn list_string() -> Vec<String> {
        Ability::list().iter().map(|a| format!("{}", a)).collect()
    }

    pub fn to_string(&self) -> String {
        match self {
            Ability::Addition(_) => format!("name: {}; arguments: {}; description: {}", 
                "Addition", 
                "{num1:number, num2:number}", 
                "add two numbers"
            ),
            Ability::SearchMovies(_) => format!("name: {}; arguments: {}; description: {}", 
                "SearchMovies", 
                "{movie:string, year:number}", 
                "search for movies"
            ),
        }
    }

    pub fn from_string(ability: &str, args: Option<Value>) -> Ability {
        match ability {
            "Addition" => Ability::Addition(args),
            "SearchMovies" => Ability::SearchMovies(args),
            _ => Ability::Addition(args),
        }
    }
}

impl Ability {
    pub fn run<T, R>(&self, f: impl FnOnce(Option<T>) -> R ) -> R 
        where T: serde::de::DeserializeOwned + Clone
    {

        let args = match self { 
            Ability::Addition(Some(args)) => args, 
            Ability::SearchMovies(Some(args)) => args,
            _ => &Value::Null 
        };
        //convert json to struct
        match serde_json::from_value::<T>(args.clone()) {
            Ok(args) => f(Some(args)),
            Err(e) => f(None)
        }
    }
}

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ability::Addition(_) => write!(f, "{}", &self.to_string()),
            Ability::SearchMovies(_) => write!(f, "{}", &self.to_string()),
        }
    }
}