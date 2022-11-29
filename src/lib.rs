use cascading_ui::cui;

cui! {
	// declare that all elements of `my_class` have red text
	.my_class {
		color: "red";
	}

	// create an element of my_class that says "hello world"
	my_class {
		text: "hello world";

		// declare that when this element is clicked, its text will become blue
		?click {
			color: "blue";
		}
	}
}
