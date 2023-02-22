#[macro_export]
macro_rules! export_transformer {
	($a:expr) => {
		#[inline(always)]
		pub fn transformer(program: &mut swc_core::ecma::ast::Program) {
			program.visit_mut_with(&mut $a)
		}
	};
}

#[macro_export]
macro_rules! test {
	($a:ty, $b:ident, $c:expr, $d:expr) => {
		swc_core::ecma::transforms::testing::test!(
			Default::default(),
			|_| {
				use swc_core::ecma::visit::as_folder;
				as_folder::<$a>(Default::default())
			},
			$b,
			$c,
			$d
		);
	};
}
