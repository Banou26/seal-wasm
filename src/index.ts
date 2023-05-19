  import init, { align, Args } from '../pkg/rust_seal'

  export type Options = {
  alignment: 'global' | 'local'
  equal: number,
  align: number,
  insert: number,
  delete: number
  wasmUrl?: string
}

let wasmPromise: ReturnType<typeof init>

export const initWasm = (wasmUrl?: string) => {
  wasmPromise = init(wasmUrl)
}

export const makeAlign = (name: 'smithwaterman' | 'needlemanwunsch', wasmUrl?: string) => {
  if (wasmUrl && !wasmPromise) {
    wasmPromise = init(wasmUrl)
  }
  return async (leftStr: string, rightStr: string, { alignment, equal, align: _align, insert, delete: del, wasmUrl }: Options) => {
    if (!wasmPromise) {
      wasmPromise = init(wasmUrl)
    }
    await wasmPromise
    const res = align(leftStr, rightStr, new Args(name, alignment, equal, _align, insert, del))
    return {
      representation: res.representation,
      score: res.score,
      originalLeft: res.originalLeft,
      originalRight: res.originalRight,
      alignedLeft: res.alignedLeft,
      alignedRight: res.alignedRight
    }
  }
}

export const swAlign = makeAlign('smithwaterman')

export const nwAlign = makeAlign('needlemanwunsch')

const result = swAlign('The quick brown fox jumps over the lazy dog.', 'The brown dog jumps over the very lazy snail.', { alignment: 'local', equal: 2, align: -1, insert: -1, delete: -1, wasmUrl: '/pkg/rust_seal_bg.wasm' })
console.log('result', result)
