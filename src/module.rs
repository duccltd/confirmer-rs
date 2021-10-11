

pub struct Module {
    pub name: String,
}

impl std::default::Default for Module {
    fn default() -> Self {
        Module::default()
    }
}

impl Module {
    pub fn new() -> Module {
        Module {
            name: "Application".to_string()
        }
    }

    pub fn name(name: &str) -> Module {
        Module {
            name: name.to_string(),
        }
    }
}