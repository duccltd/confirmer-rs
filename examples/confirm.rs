use clicli::module::Module;
use clicli::prompts::confirm::{Action, Confirm};
use clicli::prompts::Actionable;

fn main() -> std::io::Result<()> {
    let module = Module::name("My application");

    let action = Action::new(&format!("Delete user with id: {}", 123), || {
       println!("action ran")
    });

    if Confirm::from_module(module)
        .with_actions(vec![action])
        .interact()? {
        println!("confirmed")
    } else {
        println!("denied")
    }

    Ok(())
}