// Will be called when wasm_exports and wasm_memory will be available
function on_init() {
  /// Call rust function with string argument
  wasm_exports.hi_rust_with_string(js_object("plugin initialized!"));
}

register_plugin = function (importObject) {
  // make perform_demo() function available to call from rust
  importObject.env.perform_demo = perform_demo;
}

// register this plugin in miniquad, required to make plugin's functions available from rust
miniquad_add_plugin({ register_plugin, on_init });

function perform_demo() {
  // call rust function with object argument
  wasm_exports.hi_rust_with_struct(js_object(
    {
      "foo": "foo content",
      "bar": "bar content"
    }));

  // call rust function returning a string
  var rust_object = wasm_exports.get_demo_string();

  console.log(consume_js_object(rust_object)); // ensure that this is a string
};
