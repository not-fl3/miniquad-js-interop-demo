///! Demo of JS code integration with various datatypes in the API
///! Including strings, vectors and structs
use miniquad::*;
use sapp_jsutils::JsObject;

/// Example on how to receive a string from js
#[no_mangle]
pub extern "C" fn hi_rust_with_string(js_object: JsObject) {
    let mut message = String::new();

    js_object.to_string(&mut message);

    miniquad::debug!("{}", &message);
}

/// Signature of JS function
#[no_mangle]
extern "C" {
    fn perform_demo();
}

/// Example on how to receive a struct from js
/// Original struct:
/// {
///    "foo": "foo content",
///    "bar": "bar content"
/// }
#[no_mangle]
pub extern "C" fn hi_rust_with_struct(js_object: JsObject) {
    let mut data = String::new();

    let foo = js_object.field("foo");
    foo.to_string(&mut data);
    miniquad::debug!("{}", &data);

    let foo = js_object.field("bar");
    foo.to_string(&mut data);
    miniquad::debug!("{}", &data);
}

#[no_mangle]
pub extern "C" fn get_demo_string() -> JsObject {
    JsObject::string("hi from rust!")
}

struct Stage {
    ctx: Context,
}
impl EventHandlerFree for Stage {
    fn key_down_event(&mut self, _: KeyCode, _: KeyMods, _: bool) {
        // call JS function
        unsafe { perform_demo() };
    }

    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.clear(Some((0., 1., 0., 1.)), None, None);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| UserData::free(Stage { ctx }));
}
