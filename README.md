# About
This package is a typescript interface for the [seal rust crate](https://crates.io/crates/seal).

It exports `swAlign` for Smith-Waterman and `nwAlign` for Needleman-Wunsch.

## Example
```ts
const result = swAlign(
  'The quick brown fox jumps over the lazy dog.',
  'The brown dog jumps over the very lazy snail.',
  { alignment: 'local', equal: 2, align: -1, insert: -1, delete: -1 }
)
// result {
//   alignedLeft: "The brown fox jumps over the lazy "
//   alignedRight: "The brown dog jumps over the lazy "
//   originalLeft: "The quick brown fox jumps over the lazy dog."
//   originalRight: "The brown dog jumps over the very lazy snail."
//   representation: "====------======!=!================+++++====="
//   score: 51
// }
```
