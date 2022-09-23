use std::collections::HashMap;
use crate::{Topic};

pub struct Course {
    pub modules: HashMap<String, Box<dyn Topic>>
}