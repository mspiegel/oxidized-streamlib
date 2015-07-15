extern crate streamlib;
use streamlib::TopK;

#[test]
fn create() {
   let top_k = TopK::new(2);
   assert_eq!(top_k.capacity(), 2);
   assert_eq!(top_k.estimate("foo"), None);
}

#[test]
fn increment() {
   let mut top_k = TopK::new(2);
   top_k.increment("foo");
   top_k.increment("bar");
   assert_eq!(top_k.estimate("foo"), Some(1));
   assert_eq!(top_k.estimate("bar"), Some(1));
   assert_eq!(top_k.error("foo"), Some(0));
   assert_eq!(top_k.error("bar"), Some(0));
   top_k.increment("baz");
   assert_eq!(top_k.estimate_and_error("baz"), Some((2, 1)));
}
