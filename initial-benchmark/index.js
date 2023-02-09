// @ts-check
import bench from "nanobench";
import { printSync, print } from "@swc/core";
import {
	blankSpan,
	emitArrayExpression,
	emitBinaryExpression,
	emitBlockStatement,
	emitCallExpression,
	emitComputedPropName,
	emitConditionalExpression,
	emitExpressionStatement,
	emitIdentifier,
	emitMemberExpression,
	emitNumericLiteral,
	emitParenthesisExpression,
	emitStringLiteral,
} from "emitkit";

/*

(function test(foo, bar = []) {
	return "ab"[0] === foo[1] ? bar[0] ?? 1 : 4;
})();

*/

const ast = emitCallExpression(
	emitParenthesisExpression({
		type: "FunctionExpression",
		identifier: emitIdentifier("test"),
		params: [
			{ type: "Parameter", span: blankSpan, pat: emitIdentifier("foo") },
			{
				type: "Parameter",
				span: blankSpan,
				pat: {
					type: "AssignmentPattern",
					left: emitIdentifier("bar"),
					right: emitArrayExpression(),
					span: blankSpan,
				},
			},
		],
		async: false,
		generator: false,
		span: blankSpan,
		body: emitBlockStatement({
			type: "ReturnStatement",
			span: blankSpan,
			argument: emitConditionalExpression(
				emitBinaryExpression(
					emitMemberExpression(
						emitStringLiteral("ab"),
						emitComputedPropName(emitNumericLiteral(0))
					),
					emitMemberExpression(
						emitIdentifier("foo"),
						emitComputedPropName(emitNumericLiteral(1))
					),
					"==="
				),

				emitBinaryExpression(
					emitMemberExpression(
						emitIdentifier("bar"),
						emitComputedPropName(emitNumericLiteral(0))
					),
					emitNumericLiteral(1),
					"??"
				),

				emitNumericLiteral(4)
			),
		}),
	})
);

for (const n of [1, 100, 500, 1000, 5000, 10_000, 100_000]) {

	bench(`sync ${n}`, (b) => {
		b.start();

		for (let i = 0; i < n; i++)
			printSync({
				type: "Module",
				span: blankSpan,
				body: [emitExpressionStatement(ast)],
				// @ts-expect-error
				interpreter: null,
			});

		b.end();
	});

	bench(`async ${n}`, async (b) => {
		b.start();

		for (let i = 0; i < n; i++)
			await print({
				type: "Module",
				span: blankSpan,
				body: [emitExpressionStatement(ast)],
				// @ts-expect-error
				interpreter: null,
			});

		b.end();
	});

}
