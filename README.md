# ParaMin

The absolutely insane JS minifier.

You won't find a smaller bundle anywhere else!
(unless you do, in which case, damnnnnnn.)

Implemented with SWC in Rust (:rocket::rocket::rocket:),
to stop it from being *too* horridly inefficient.

> **Warning** |
> *On the open-sourcedness of ParaMin 2023-02-12*
>
> This repository was open sourced very early on because of a request by a friend of mine
> to read the source code for the mangler.
>
> However, almost instantly this proved true to EXACTLY why I keep my
> WIP projects closed-source: *"benchmark for size where"*.
>
> No, its not ready, don't expect it to work.
>
> If you're here to keep an eye on this until its ready *that's awesome*,
> and I'm super glad you're interested in my work,
> but if you are about to come and ask me for size or runtime speed
> metrics of any kind, or for how to use this currently, please don't.
> **There are no metrics, and you cannot currently use this in your pipeline.**
>
> Thanks, Yellowsink.

## Usage hints

ParaMin works best when fed an unminified bundle, so if your build tool has that stage in its pipeline, awesome.

If it doesn't, post-process an unminified build :)

Only ES Modules are officially 100% will-not-break-ever supported *as of now*.

CommonJS and IIFEs should work fine given defaults but no guarantees, especially
if you are using options such as `notopiife` and `unsafeglobals`.

## Why?

Because a lot of minifiers now don't do the best job a lot of the time.

They aren't context-aware enough.

I can't hope to beat hand-optimised output, but I hope to get close!

Based on a few main concepts:
 - Confident minifications
   * Existing, fast minifications that minifiers do
   * ParaMin delegates this to `swc_ecma_minify`, as it already [performs very well](https://github.com/privatenumber/minification-benchmarks).
   * With exceptions, however (our mangler is situationally better for example)

 - Tentative minifications
   * Test all the things!
   * Throw possible minifications at the wall and see what does or doesn't stick
   * *Can* be used to avoid classically confident minifications if they actually might have an adverse effect,
     but unlikely this should ever be needed

 - Smarter situational minifications
   * Use a `function` that turns out bigger than an arrow func but don't make use of `this`? Let's minify!
   * All the minifications that other minifiers are too scared to do, done when they're safe.
   * Come on, really, inliners can be way better than they are. Tentative minifications to the rescue!

 - A thoughtfully chosen order of optimisations
   * Make sure that an optimisation that could benefit a future one comes before it
   * The next-best-thing to testing literally every combination of optimisations

## So, why SWC?

First, simply that I have more experience with SWC.

Secondly, performance. Now I'm not going all rust shill on you here, Babel is very slow.

I have had to use both Babel and SWC together in a web UI before.
SWC could run on every keystroke easily even with WASM fees to pay.
Babel could *not*, and had the benefit of no interop and a JIT.

Also, tentative minifications would be honestly impossible to do with any
reasonable performance in Babel, I need stringification to be EXTREMELY fast.

Finally, I can build on top of the excellent `swc_ecma_minify` crate,
which (see link in list above) is already a very good minifier.

## but uh closure compiler

Yeah, so closure compiler's advanced mode is kinda like this.

Neat. Hopefully mine can beat it >:)

Also, on big libraries, closure compiler is unusably slow (typescript etc.)

If you're interested, here's a more useful page than the landing page:
https://developers.google.com/closure/compiler/docs/api-tutorial3
