use cascading_ui::cui;

cui! {
	title: "hello world";

	text: "this is a demo of some implemented features";

	.a {
		color: "blue";
	}
	a {
		text: "clicking me does nothing";
		.b {
			color: "green";
		}
		b {
			?click {
				text: "hi";
			}
			text: "click me to say hi";
		}
	}
	a {
		text: "clicking me makes a new element";
		?click {
			b {
				text: "hello world!";
			}
		}
	}
}
