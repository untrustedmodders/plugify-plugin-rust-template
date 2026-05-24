use plugify::{register_plugin, PlgString};
use std::error::Error;

fn on_plugin_start() -> Result<(), Box<dyn Error>> {
    println!("ExamplePlugin: on_plugin_start");
    Ok(())
}

fn on_plugin_update(_dt: f32) -> Result<(), Box<dyn Error>> {
    println!("ExamplePlugin: on_plugin_update");
    Ok(())
}

fn on_plugin_end() -> Result<(), Box<dyn Error>> {
    println!("ExamplePlugin: on_plugin_end");
    Ok(())
}

register_plugin!(
    start: on_plugin_start,
    update: on_plugin_update,
    end: on_plugin_end
);

#[unsafe(no_mangle)]
pub extern "C" fn make_print(count: i32, message: &PlgString) {
    for _ in 0..count {
        println!("{}", message);
    }
}