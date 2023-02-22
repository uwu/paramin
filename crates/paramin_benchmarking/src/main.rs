use std::time::{Duration, Instant};

const TEST_FILES: [&'static str; 5] = [
	"jquery",
	"lodash",
	"moment",
	"three",
	"typescript",
];

async fn fetch(pkg: &str) -> Result<String, reqwest::Error> {
	let url = String::from("https://unpkg.com/") + pkg;

	reqwest::get(url).await?.text().await
}

struct TestResult {
	pub start_size: u32,
	pub end_size: u32,
	pub time: Duration,
}

#[tokio::main]
async fn main() {
	for test in TEST_FILES {
		let str = fetch(test).await.unwrap();

		let start_time = Instant::now();

		let transformed_size = transform_to_size(&str);

		let result = TestResult {
			start_size: str.len().try_into().unwrap(),
			end_size: transformed_size.try_into().unwrap(),
			time: start_time.elapsed(),
		};

		let startf: f64 = result.start_size.into();
		let endf: f64 = result.end_size.into();

		println!(
			"compressed {} from {:.2}KB to {:.2}KB ({:.1}% reduction) in {}ms",
			test,
			startf / 1_000_f64,
			endf / 1_000_f64,
			100_f64 * (startf - endf) / startf,
			result.time.as_millis()
		);
	}
}

fn transform_to_size(raw: &str) -> usize {
	use swc_core::{
		common::{BytePos, FileName, SourceFile},
		ecma::{
			ast::EsVersion,
			parser::{parse_file_as_program, EsConfig, Syntax},
		},
	};

	let mut parsed = parse_file_as_program(
		&SourceFile::new(
			FileName::Anon,
			false,
			FileName::Anon,
			String::from(raw),
			BytePos(1),
		),
		Syntax::Es(EsConfig::default()),
		EsVersion::EsNext,
		None,
		&mut vec![],
	)
	.unwrap();

	paramin::simple_transform(&mut parsed);

	paramin::measurement::get_length(&parsed)
}
