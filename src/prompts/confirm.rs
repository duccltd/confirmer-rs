use crate::prompts::Actionable;
use console::Term;
use crate::module::Module;
use std::borrow::Borrow;

pub struct Action {
    name: String,
    function: fn(),
}

impl Action {
    pub fn new(name: &str, function: fn()) -> Action {
        Action {
            name: name.to_string(),
            function,
        }
    }
}

pub struct Confirm {
    prompt: String,
    actions: Vec<Action>,
    module: Module,
}

impl std::default::Default for Confirm {
    fn default() -> Self {
        Confirm::new()
    }
}

impl Confirm {
    pub fn new() -> Confirm {
        Confirm::from_module(
            Module::new()
        )
    }

    pub fn from_module(module: Module) -> Confirm {
        Confirm {
            prompt: "Do you want to perform these actions [y/n]?".to_string(),
            actions: vec![],
            module
        }
    }

    pub fn with_prompt(self, prompt: &str) -> Confirm {
        Confirm {
            prompt: prompt.to_string(),
            ..self
        }
    }

    pub fn with_actions(self, actions: Vec<Action>) -> Confirm {
        Confirm {
            actions,
            ..self
        }
    }
}

impl Actionable<bool> for Confirm {
    fn term_interact(&self, term: &Term) -> std::io::Result<bool> {
        println!("{} will perform the following actions: \n", self.module.name);

        for action in &self.actions {
            println!("{}", action.name)
        }

        println!("\nPlan: {} actions to perform", self.actions.len());

        println!("\n{}", self.prompt);

        Ok(true)
    }

    fn interact(&self) -> std::io::Result<bool> {
        let term = Term::stdout();
        self.term_interact(&term)
    }
}