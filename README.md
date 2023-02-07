# ParaMin

The absoutely insane JS minifier.

You won't find a smaller bundle anywhere else!
(unless you do, in which case, damnnnnnn.)

Implemented with SWC in Rust (:rocket::rocket::rocket:),
to stop it from being *too* horridly inefficent.

## Why?

Because a lot of minifiers now don't do the best job a lot of the time.

They aren't context-aware enough, and miss minifications that seem obvious
when looking yourself.

I can't hope to beat hand-optimised output, but I hope to get close!

Based on a few main concepts:
 - Arrogant minifications
   * Mangling (foo -> e)
   * Compressing (`function (x, y) {  }` -> `function(x,y){}`)
   * Heuristic minifications (all the neat stuff minifiers do already)
   
 - Tentative minifications
   * Test all the things!
   * Throw possible minifications at the wall and see what does or doesn't stick
   * *Can* be used to avoid arrogant minifications if they actually might have an adverse effect
   * Let's super

 - Smarter situational minifications
   * Use a `function` that turns out bigger than an arrow func but don't make use of `this`? Let's minify!
   * All the minifications that other minifiers are too scared to do, done when they're safe.
   * Come on, really, inliners can be way better than they are. Tentative minifications to the rescue!

 - A thoughtfully chosen order of optimisations
   * Make sure that an optimisation that could benefit a future one comes before it
   * The next-best-thing to testing literally every combination of optimisations

## So, why SWC?

First of all, I have more experience with SWC.

I have built many a transform in it, and am mostly comfortable with its JS API
(though this uses the Rust API!)

Secondly, performance. Now I'm not going all rust on you here, no honestly,
Babel is not fast. It is very slow.

I have had to use both Babel and SWC together in a web ui before.
SWC could run on every keystroke easily even with WASM fees to pay.
Babel could *not*, and had the benefit of no interop and a JIT.

Also, tentative minifications would be honestly impossible to do with any
reasonable performance in Babel, I need stringification to be EXTREMELY fast.

## but uh closure compiler

Yeah so apparently closure compiler's advanced mode is kinda like this.

Neat. Hopefully mine can beat it >:)

If you're interested, here's a more useful page than the landing page:
https://developers.google.com/closure/compiler/docs/api-tutorial3