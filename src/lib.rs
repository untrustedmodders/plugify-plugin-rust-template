use plugify::{register_plugin, PlgString};

fn on_plugin_start() {
    println!("ExamplePlugin: on_plugin_start")
}

fn on_plugin_update(_dt: f32) {
    println!("ExamplePlugin: on_plugin_update")
}

fn on_plugin_end() {
    println!("ExamplePlugin: on_plugin_end")
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