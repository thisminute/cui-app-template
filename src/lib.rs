use cascading_ui::cui;

cui! {
	// declare that all elements of `my-class` have red text
	.myclass {
		color: "red";
	}

	// create an element of my-class that says "hello world"
	myclass {
		text: "hello world";

		// declare that when this element is clicked, its text will become blue
		?click {
			color: "blue";
		}
	}
}
