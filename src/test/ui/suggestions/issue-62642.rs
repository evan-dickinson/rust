fn main() {
	let mut s = "foo";
	let _ = s.as_str();
	let _ = s.as_mut_str();
	let _ = s.as_str("foo");

	let _ = String::as_str(s);
}
