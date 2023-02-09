use std::io::{Write, Result};
use swc_core::common::sync::{Lrc, RwLock};

use swc_core::ecma::codegen::Config;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::{
	common::SourceMap,
	ecma::codegen::{Emitter,Node}
};

struct WriteCounter {
	pub written: Lrc<RwLock<usize>>
}

impl Write for WriteCounter {
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}

	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let len = buf.len();

		// useful for debug
		//print!("{}", std::str::from_utf8(buf).unwrap());

		*self.written.write() += len;

		Ok(len)
	}
}

pub fn get_length(node: &impl Node) -> usize {
	let srcmap = Lrc::new(SourceMap::default());
	let written = Lrc::new(RwLock::new(0));
	let counter = WriteCounter { written: written.clone() };

	let mut emitter = Emitter {
		cm: srcmap.clone(),
		comments: None,
		wr: JsWriter::new(srcmap, "\n", counter, None),
		cfg: Config {
			minify: true,
			..Config::default()
		}
	};

	node.emit_with(&mut emitter).unwrap();

	let written = *written.read();
	written
}

pub fn test_operation() -> bool {
	panic!("not implemented lol die");
}

#[test]
fn test_get_length() {

	use swc_core::common::{Span, SyntaxContext, BytePos};
	use swc_core::ecma::ast;

	let node = ast::Str {
		raw: Some("deez nuts".into()),
		value: "deez nuts".into(),
		span: Span {
			lo: BytePos(0),
			hi: BytePos(11),
			ctxt: SyntaxContext::empty()
		}
	};

	let res = get_length(&node);

	assert_eq!(res, 11);
}