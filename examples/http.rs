extern crate minttp;
#[cfg(feature = "http")]
extern crate http;

#[cfg(feature = "http")]
use http::{Request, Uri};

#[cfg(feature = "http")]
fn main() {
	let uri: Uri = "example.com".parse().unwrap();
	let mut req = Request::builder()
		.uri(uri)
		.method(http::method::GET)
		.header("Test", "Hello World")
		.body([])
		.unwrap();

	let output;
	{
		let response = minttp::request(&mut req).unwrap().try_into().unwrap();
		println!(
			"Status: {}",
			response.status(),
			// response.extensions().get::<String>().unwrap()
		);
		output = String::from_utf8_lossy(response.body()).to_string();
	}

	println!("-------------- High-level flexible request");
	println!("{}", output);
	println!("--------------");
}
#[cfg(not(feature = "http"))]
fn main() { eprintln!("Please run this example with --features \"http\"") }
