# config opts

## global / misc
Affects many part of ParaMin or misc setting.

### `unsafeglobals`
Does not respect global side effects.

Shouldn't do silly things like `console.log()` → `a.b()` but will
break e.g. `window.important_api_dont_change = {}`.

### `noshake`
Does not tree shake.

### `dev`
Intended to be enabled by build tool integrations,
disables many minifications to improve debug experience.

Disables:
 - tree shaking

### `modifyeffects`
Allows ParaMin to change the side effects of your code in limited ways.
Enables:
 - expectations that you aren't doing magic with global prototypes etc
 - remove all console logs 👍

### `notopiife`
By default, ParaMin will *not* inline IIFEs at the top level,
as it is assumed that:
 - they are added by the bundler
 - probably by choice of the user
 - and they are useful due to their behaving
   identically in both expression and statement contexts in JS.
 - they also use objects for their exports which is convenient.
 - early bailout with return (I guess?)

If your target does not require an IIFE,
enabling this will let ParaMin swap a top level IIFE
(without any early returns) into a simple parenthesis expression.

`(()=>{a();return{export1:b}})()`
→ `(a(),{export1:b})`

## mangler
Affects the mangler, the transform that shortens identifiers.
