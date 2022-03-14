mod utils;

use std::{convert::TryInto};
use wasm_bindgen::prelude::*;

extern crate seal;

use seal::pair::{
    AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, SmithWaterman, Step, Strategy,
};

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Args {
    name: String,
    alignment: String,
    equal: isize,
    align: isize,
    insert: isize,
    del: isize,
}

#[wasm_bindgen]
impl Args {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, alignment: &str, equal: isize, align: isize, insert: isize, del: isize) -> Args {
        Args {
            name: name.to_string(),
            alignment: alignment.to_string(),
            equal: equal,
            align: align,
            insert: insert,
            del: del,
        }
    }
    
    #[wasm_bindgen(getter = name)]
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    #[wasm_bindgen(setter = name)]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    #[wasm_bindgen(getter = alignment)]
    pub fn alignment(&self) -> String {
        self.alignment.to_string()
    }
    #[wasm_bindgen(setter = alignment)]
    pub fn set_alignment(&mut self, alignment: &str) {
        self.alignment = alignment.to_string();
    }

    #[wasm_bindgen(getter = equal)]
    pub fn equal(&self) -> isize {
        self.equal
    }
    #[wasm_bindgen(setter = equal)]
    pub fn set_equal(&mut self, equal: isize) {
        self.equal = equal;
    }

    #[wasm_bindgen(getter = align)]
    pub fn align(&self) -> isize {
        self.align
    }
    #[wasm_bindgen(setter = align)]
    pub fn set_align(&mut self, align: isize) {
        self.align = align;
    }

    #[wasm_bindgen(getter = insert)]
    pub fn insert(&self) -> isize {
        self.insert
    }
    #[wasm_bindgen(setter = insert)]
    pub fn set_insert(&mut self, insert: isize) {
        self.insert = insert;
    }

    #[wasm_bindgen(getter = del)]
    pub fn del(&self) -> isize {
        self.del
    }
    #[wasm_bindgen(setter = del)]
    pub fn set_del(&mut self, del: isize) {
        self.del = del;
    }
}

#[wasm_bindgen]
pub struct AlignmentResult {
    representation: String,
    original_left: String,
    original_right: String,
    aligned_left: String,
    aligned_right: String,
    score: isize
}

#[wasm_bindgen]
impl AlignmentResult {
    #[wasm_bindgen(constructor)]
    pub fn new(representation: &str, original_left: &str, original_right: &str, aligned_left: &str, aligned_right: &str, score: isize) -> AlignmentResult {
        AlignmentResult {
            representation: representation.to_string(),
            original_left: original_left.to_string(),
            original_right: original_right.to_string(),
            aligned_left: aligned_left.to_string(),
            aligned_right: aligned_right.to_string(),
            score: score
        }
    }
    
    #[wasm_bindgen(getter = representation)]
    pub fn representation(&self) -> String {
        self.representation.to_string()
    }
    #[wasm_bindgen(setter = representation)]
    pub fn set_representation(&mut self, representation: &str) {
        self.representation = representation.to_string();
    }

    #[wasm_bindgen(getter = originalLeft)]
    pub fn original_left(&self) -> String {
        self.original_left.to_string()
    }
    #[wasm_bindgen(setter = originalLeft)]
    pub fn set_original_left(&mut self, original_left: &str) {
        self.original_left = original_left.to_string();
    }

    #[wasm_bindgen(getter = originalRight)]
    pub fn original_right(&self) -> String {
        self.original_right.to_string()
    }
    #[wasm_bindgen(setter = originalRight)]
    pub fn set_original_right(&mut self, original_right: &str) {
        self.original_right = original_right.to_string();
    }

    #[wasm_bindgen(getter = alignedLeft)]
    pub fn aligned_left(&self) -> String {
        self.aligned_left.to_string()
    }
    #[wasm_bindgen(setter = alignedLeft)]
    pub fn set_aligned_left(&mut self, aligned_left: &str) {
        self.aligned_left = aligned_left.to_string();
    }

    #[wasm_bindgen(getter = alignedRight)]
    pub fn aligned_right(&self) -> String {
        self.aligned_right.to_string()
    }
    #[wasm_bindgen(setter = alignedRight)]
    pub fn set_aligned_right(&mut self, aligned_right: &str) {
        self.aligned_right = aligned_right.to_string();
    }

    #[wasm_bindgen(getter = score)]
    pub fn score(&self) -> isize {
        self.score
    }
    #[wasm_bindgen(setter = score)]
    pub fn set_score(&mut self, score: isize) {
        self.score = score;
    }
}

#[wasm_bindgen]
pub fn align(left_str: &str, right_str: &str, args: &Args) -> AlignmentResult {
    fn _align<S: Strategy> (left_str: &str, right_str: &str, args: &Args, strategy: S) -> AlignmentResult {
        let seq_left: Vec<char> = left_str.chars().collect();
        let seq_right: Vec<char> = right_str.chars().collect();
    
        let set: AlignmentSet<InMemoryAlignmentMatrix> =
            AlignmentSet::new(seq_left.len(), seq_right.len(), strategy, |x, y| {
                seq_left[x] == seq_right[y]
            })
            .unwrap();
    
        let alignment = match args.alignment.as_str() {
            "global" => set.global_alignment(),
            "local" => set.local_alignment(),
            _ => unreachable!()
        };
        let score = alignment.score();
    
        let mut representation = String::from("");
        let mut aligned_left_str = String::from("");
        let mut aligned_right_str = String::from("");
        for step in alignment.steps() {
            match step {
                Step::Align { x, y } => {
                    aligned_left_str.push(seq_left[x]);
                    aligned_right_str.push(seq_right[y]);
                    if seq_left[x] == seq_right[y] {
                        representation.push('=');
                    } else {
                        representation.push('!');
                    }
                }
                Step::Delete { .. } => representation.push('-'),
                Step::Insert { .. } => representation.push('+'),
            }
        }
    
        return AlignmentResult {
            representation: representation.to_string(),
            score: score.try_into().unwrap(),
            original_left: left_str.to_string(),
            original_right: right_str.to_string(),
            aligned_left: aligned_left_str,
            aligned_right: aligned_right_str
        }
    }

    match args.name.as_str() {
        "smithwaterman" => _align(left_str, right_str, args, SmithWaterman::new(args.equal.try_into().unwrap(), args.align.try_into().unwrap(), args.insert.try_into().unwrap(), args.del.try_into().unwrap())),
        "needlemanwunsch" => _align(left_str, right_str, args, NeedlemanWunsch::new(args.equal.try_into().unwrap(), args.align.try_into().unwrap(), args.insert.try_into().unwrap(), args.del.try_into().unwrap())),
        _ => unreachable!()
    }
}
