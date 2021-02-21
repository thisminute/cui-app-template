use cascading_wasm_language::cwl;
extern crate console_error_panic_hook;

// std::panic::set_hook(Box::new(console_error_panic_hook::hook));

cwl! {
	title: "template";
	text: "root";
	// .button {
	// 	background_color: "red";
	// 	?click {
	// 		background_color: "blue";
	// 	}
	// }

	button {
		background_color: "red";
		text: "click me to turn me blue";
		link: "https://google.com/";
	}

	button {
		text: "click me also to turn me blue";
		// ?click {
		// 	text: "I've turned blue!";
		// }
	}
}
