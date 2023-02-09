# ParaMin Initial Benchmark: get_length

Goal: to determine the viability of tentative minifications, and if using Rust is worthwhile.

It will do this by stringifying the following AST:
```yaml
CallExpr
 - args: []
 - body: ParenExpr: FnExpr
	- ident: "test",
	- function: Function
		- async: false
		- generator: false
		- params: [
			Param: BindingIdent: Ident: "foo",
			Param: AssignPat
				- left: BindingIdent: Ident: "bar"
				- right: ArrayLit
					- elems: []
		]
		- body: BlockStmt: ReturnStmt: CondExpr
			- test: BinExpr
				- op: ===
				- left: MemberExpr
					- obj: Lit: Str: "ab"
					- prop: Computed: Lit: 0
				- right: MemberExpr
					- obj: Ident: "foo"
					- prop: Computed: Lit: 1

			- cons: BinExpr
				- op: ??
				- left: MemberExpr
					- obj: Ident: "bar",
					- prop: Computed: Lit: 0
				- right: Lit: 1

			- alt: Lit: 4
```
to produce the following (or an equivalent form thereof):
```js
(function test(foo, bar = []) {
	return "ab"[0] === foo[1] ? bar[0] ?? 1 : 4;
})();
```

Usually picking hundreds of runs is misleading as it can give unfair benefits to JIT code.

Despite this, I will still do this here as this function *is* expected to be run thousands of times
in an invocation and so will benefit from JIT.

I will show results for multiple invocation amounts.

The JS benchmark code lives in this directory, the Rust code under benchmark is
[this code](https://github.com/uwu/paramin/blob/954ebc9/src/measurement.rs#L32-L51),
using [this benchmark](https://github.com/uwu/paramin/blob/master/benches/measurement_benches.rs).

JS data (tab-separated CSV, yes I am the devil):
```csv
async	runs	time	ns per op
false	1	830284	0830284
true	1	2501752	2501752
false	100	13419209	0134192.09
true	100	23697986	0236979.86
false	500	65674554	0131349.108
true	500	90774334	0181548.668
false	1000	123130806	0123130.806
true	1000	189389747	0189389.747
false	5000	575005214	0115001.0428
true	5000	843452804	0168690.5608
false	10000	1019903734	0101990.3734
true	10000	1609945772	0160994.5772
false	100000	10221717741	0102217.17741
true	100000	16433532086	0164335.32086
```

My rust result was:
```
get_length              time:   [2.1703 µs 2.1794 µs 2.1897 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
```

(this is 2179.4ns/op)

This puts rust at (at worst) 46.9x better, and (at best) 1147.9x better.

Conclusion: writing this in Rust is justified, and invoking this function should be fast enough
for this project to work.

For reference, 2179.4ns/op means the function can run 458.8 thousand times per second.
More than enough for our requirements!!