use cascading_wasm_language::cwl;
extern crate console_error_panic_hook;

// std::panic::set_hook(Box::new(console_error_panic_hook::hook));

cwl! {
   title: "hi";

   .text {
      text: "hello";
   }

   text {
      text: "world";
      link: "http://google.com";
      text {
         background_color: "gray";
      }
   }

}
