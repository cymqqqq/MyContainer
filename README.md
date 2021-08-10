# MyContainer
This crate provides various collections.

These are collections where references to entries can be held on to even across insertions. This is safe because these collections only support storing data that's present behind some indirection -- i.e. String, Vec<T>, Box<T>, etc, and they only yield references to the data behind the allocation (&str, &[T], and &T respectively)

