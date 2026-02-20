use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyModel {
    pub my_string: String,
    pub my_i32: i32,
    pub created_at: DateTime<Utc>,
}

impl MyModel{
    pub fn new(mystring:&str, myi32:i32, mydate:DateTime<Utc>)->Self{
        Self{
            my_string:mystring.to_string(),
            my_i32:myi32,
            created_at:mydate,
        }
    }
}