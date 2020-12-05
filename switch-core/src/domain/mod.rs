use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Reply {
    text: String
}

impl Reply {
    pub fn new(text_: String) -> Reply {
        Reply {
            text: text_
        }
    }
}

