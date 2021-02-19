use cascading_wasm_language::cwl;
extern crate console_error_panic_hook;

// std::panic::set_hook(Box::new(console_error_panic_hook::hook));

cwl! {
	title: "template";
	text: "root";
	element {
		text: "click me to turn me blue";
		?click {
			background_color: "blue";
		}
	}
}
