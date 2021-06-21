use cascading_ui::cui;

// type CssColor = (&'static str, String);

cui! { // 0
	text: "click me";
	?click {
		.a {
			color: "blue";
		}
		a {
			text: "hello world";
		}
	}
}
