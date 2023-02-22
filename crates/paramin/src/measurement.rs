use std::io::{Result, Write};
use swc_core::common::sync::{Lrc, RwLock};
use swc_core::ecma::codegen::Node;

use crate::codegen::emit_to_writer;

#[derive(Default)]
struct WriteCounter {
	pub written: Lrc<RwLock<usize>>,
}

impl Write for WriteCounter {
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}

	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let len = buf.len();

		*self.written.write() += len;

		Ok(len)
	}
}

pub fn get_length(node: &impl Node) -> usize {
	let counter = WriteCounter::default();
	let write_count = counter.written.clone();

	emit_to_writer(node, counter);

	let write_count = *write_count.read();
	write_count
}

// TODO: this will probably need to become more sophisticated!
pub fn attempt_operation<T: Node, F: Fn(&T) -> T>(node: T, transformer: F) -> T {
	let transformed = transformer(&node);

	if get_length(&node) > get_length(&transformed) {
		transformed
	} else {
		node
	}
}

#[test]
fn test_get_length() {
	use swc_core::ecma::ast;

	let node = ast::Str {
		raw: Some("deez nuts".into()),
		value: "deez nuts".into(),
		span: Default::default(),
	};

	let res = get_length(&node);

	assert_eq!(res, 11);
}

#[test]
fn test_attempt_operation_improvement() {
	use swc_core::ecma::ast;

	let initial_value = "remove this ->!";

	let node = ast::Str {
		raw: Some(initial_value.into()),
		value: initial_value.into(),
		span: Default::default(),
	};

	let res = attempt_operation(node, |n| {
		if n.value.ends_with('!') {
			let mut chars = n.value.chars();
			chars.next_back();
			let trimmed = chars.as_str();
			ast::Str {
				span: n.span,
				raw: Some(trimmed.into()),
				value: trimmed.into(),
			}
		} else {
			n.clone()
		}
	});

	assert_ne!(*initial_value, res.value);
}

#[test]
fn test_attempt_operation_regression() {
	use swc_core::ecma::ast;

	let initial_value = "duplicate this ->!";

	let node = ast::Str {
		raw: Some(initial_value.into()),
		value: initial_value.into(),
		span: Default::default(),
	};

	let res = attempt_operation(node, |n| {
		if n.value.ends_with('!') {
			let extended = String::from(&*n.value) + "!";

			ast::Str {
				span: n.span,
				raw: Some(extended.clone().into()),
				value: extended.into(),
			}
		} else {
			n.clone()
		}
	});

	assert_eq!(*initial_value, res.value);
}
