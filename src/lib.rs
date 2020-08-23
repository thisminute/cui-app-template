use cascading_wasm_language::cwl;
extern crate console_error_panic_hook;

// std::panic::set_hook(Box::new(console_error_panic_hook::hook));

cwl! {
   // rule in context of page instance
   // sets meta.title to the value
   // appends a title tag to the head element
   title: "template";

   // instance in context of page instance
   // appends an element to the page
   // appends a title tag to the head element
   container {
      // rule in context of element instance
      // sets meta.title to the value
      // appends a title tag to the head element
      text: "click to turn orange";
      ?click {
         background_color: "orange";
      }
   }
   container2 {
      // rule in context of element instance
      // sets meta.title to the value
      // appends a title tag to the head element
      text: "click to turn red";
      ?click {
         background_color: "red";
      }
   }
}
