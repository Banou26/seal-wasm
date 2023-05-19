export type Options = {
  alignment: 'global' | 'local'
  equal: number,
  align: number,
  insert: number,
  delete: number
}

const makeAlign = (name: 'smithwaterman' | 'needlemanwunsch') => async (leftStr: string, rightStr: string, { alignment, equal, align: _align, insert, delete: del }: Options) => {
  const { align, Args } = await import('../pkg/rust_seal')
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

export const swAlign = makeAlign('smithwaterman')

export const nwAlign = makeAlign('needlemanwunsch')
