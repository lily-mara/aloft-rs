Aloft-rs
===

This is a sister library to aloft.py. It allows you to interact with winds aloft
data from NOAA. This data is normally available as a giant blob from NOAA
[here](http://aviationweather.gov/products/nws/all) and is obviously quite a
pain to work with. Aloft-rs makes this easier!

Requirements
---

Aloft-rs requires Rust Nightly (`regex!` macro relies on unstable features).

Cargo.toml
---

	[dependencies]
	aloft = "*"

main.rs
---

	extern crate aloft;
	use aloft::winds_aloft_for_station;

	fn main() {
		let winds = winds_aloft_for_station("CVG").unwrap();

		println!("{:?}", winds);
		println!("{:?}", winds.wind_at_altitude(12000).unwrap());
	}

You can also use [rustc-serialize](https://crates.io/crates/rustc-serialize)
to encode/decode the structs to/from JSON for web applications easily

main.rs
---

	extern crate aloft;
	extern crate rustc_serialize;
	use aloft::winds_aloft_for_station;
	use rustc_serialize::json;

	fn main() {
		let winds = winds_aloft_for_station("CVG").unwrap();

		println!("{}", json::encode(&winds).unwrap());
		println!("{}", json::encode(&winds.wind_at_altitude(12000).unwrap()).unwrap());
	}
