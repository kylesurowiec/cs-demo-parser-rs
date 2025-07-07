#![allow(dead_code)]

use once_cell::sync::Lazy;

use super::huffman::{Node, build_huffman_tree};
use super::reader::Reader;

#[derive(Clone, Debug)]
pub struct FieldPath {
    pub path: [usize; 7],
    pub last: usize,
    pub done: bool,
}

impl FieldPath {
    pub fn new() -> Self {
        FieldPath {
            path: [-1isize as usize, 0, 0, 0, 0, 0, 0],
            last: 0,
            done: false,
        }
    }

    fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.path[self.last] = 0;
            if self.last > 0 {
                self.last -= 1;
            }
        }
    }

    fn copy_from(&self) -> FieldPath {
        FieldPath {
            path: self.path,
            last: self.last,
            done: self.done,
        }
    }
}

impl Default for FieldPath {
    fn default() -> Self {
        Self::new()
    }
}

struct FieldPathOp {
    weight: i32,
    op: fn(&mut Reader, &mut FieldPath),
}

static FIELD_PATH_TABLE: &[FieldPathOp] = &[
    FieldPathOp {
        weight: 36271,
        op: |_, fp| fp.path[fp.last] += 1,
    },
    FieldPathOp {
        weight: 10334,
        op: |_, fp| fp.path[fp.last] += 2,
    },
    FieldPathOp {
        weight: 1375,
        op: |_, fp| fp.path[fp.last] += 3,
    },
    FieldPathOp {
        weight: 646,
        op: |_, fp| fp.path[fp.last] += 4,
    },
    FieldPathOp {
        weight: 4128,
        op: |r, fp| fp.path[fp.last] += r.read_ubit_var_field_path() + 5,
    },
    FieldPathOp {
        weight: 35,
        op: |_, fp| {
            fp.last += 1;
            fp.path[fp.last] = 0;
        },
    },
    FieldPathOp {
        weight: 3,
        op: |r, fp| {
            fp.last += 1;
            fp.path[fp.last] = r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 521,
        op: |_, fp| {
            fp.path[fp.last] += 1;
            fp.last += 1;
            fp.path[fp.last] = 0;
        },
    },
    FieldPathOp {
        weight: 2942,
        op: |r, fp| {
            fp.path[fp.last] += 1;
            fp.last += 1;
            fp.path[fp.last] = r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 560,
        op: |r, fp| {
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] = 0;
        },
    },
    FieldPathOp {
        weight: 471,
        op: |r, fp| {
            fp.path[fp.last] += r.read_ubit_var_field_path() + 2;
            fp.last += 1;
            fp.path[fp.last] = r.read_ubit_var_field_path() + 1;
        },
    },
    FieldPathOp {
        weight: 10530,
        op: |r, fp| {
            fp.path[fp.last] += r.read_bits(3) as usize + 2;
            fp.last += 1;
            fp.path[fp.last] = r.read_bits(3) as usize + 1;
        },
    },
    FieldPathOp {
        weight: 251,
        op: |r, fp| {
            fp.path[fp.last] += r.read_bits(4) as usize + 2;
            fp.last += 1;
            fp.path[fp.last] = r.read_bits(4) as usize + 1;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.last += 1;
            fp.path[fp.last] = r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] = r.read_bits(5) as usize;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.last += 1;
            fp.path[fp.last] = r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] = r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] = r.read_bits(5) as usize;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += 1;
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += 1;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += 1;
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += 1;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += r.read_ubit_var() as usize + 2;
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += r.read_ubit_var() as usize + 2;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += r.read_ubit_var() as usize + 2;
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            fp.last += 1;
            fp.path[fp.last] += r.read_ubit_var_field_path();
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.path[fp.last] += r.read_ubit_var() as usize + 2;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
            fp.last += 1;
            fp.path[fp.last] += r.read_bits(5) as usize;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            let n = r.read_ubit_var() as usize;
            fp.path[fp.last] += r.read_ubit_var_field_path();
            for _ in 0..n {
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_field_path();
            }
        },
    },
    FieldPathOp {
        weight: 310,
        op: |r, fp| {
            for i in 0..=fp.last {
                if r.read_boolean() {
                    fp.path[i] = (fp.path[i] as i32 + r.read_var_int32() + 1) as usize;
                }
            }
            let count = r.read_ubit_var();
            for _ in 0..count {
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_field_path();
            }
        },
    },
    FieldPathOp {
        weight: 2,
        op: |_, fp| {
            fp.pop(1);
            fp.path[fp.last] += 1;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.pop(1);
            fp.path[fp.last] += r.read_ubit_var_field_path() + 1;
        },
    },
    FieldPathOp {
        weight: 1837,
        op: |_, fp| {
            fp.pop(fp.last);
            fp.path[0] += 1;
        },
    },
    FieldPathOp {
        weight: 149,
        op: |r, fp| {
            fp.pop(fp.last);
            fp.path[0] += r.read_ubit_var_field_path() + 1;
        },
    },
    FieldPathOp {
        weight: 300,
        op: |r, fp| {
            fp.pop(fp.last);
            fp.path[0] += r.read_bits(3) as usize + 1;
        },
    },
    FieldPathOp {
        weight: 634,
        op: |r, fp| {
            fp.pop(fp.last);
            fp.path[0] += r.read_bits(6) as usize + 1;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.pop(r.read_ubit_var_field_path());
            fp.path[fp.last] += 1;
        },
    },
    FieldPathOp {
        weight: 0,
        op: |r, fp| {
            fp.pop(r.read_ubit_var_field_path());
            fp.path[fp.last] = ((fp.path[fp.last] as i32) + r.read_var_int32()) as usize;
        },
    },
    FieldPathOp {
        weight: 1,
        op: |r, fp| {
            fp.pop(r.read_ubit_var_field_path());
            for i in 0..=fp.last {
                if r.read_boolean() {
                    fp.path[i] = ((fp.path[i] as i32) + r.read_var_int32()) as usize;
                }
            }
        },
    },
    FieldPathOp {
        weight: 76,
        op: |r, fp| {
            for i in 0..=fp.last {
                if r.read_boolean() {
                    fp.path[i] = ((fp.path[i] as i32) + r.read_var_int32()) as usize;
                }
            }
        },
    },
    FieldPathOp {
        weight: 271,
        op: |_, fp| {
            if fp.last > 0 {
                fp.path[fp.last - 1] += 1;
            }
        },
    },
    FieldPathOp {
        weight: 99,
        op: |r, fp| {
            for i in 0..=fp.last {
                if r.read_boolean() {
                    let v = r.read_bits(4) as i32 - 7;
                    fp.path[i] = ((fp.path[i] as i32) + v) as usize;
                }
            }
        },
    },
    FieldPathOp {
        weight: 25474,
        op: |_, fp| fp.done = true,
    },
];

static HUFFMAN_TREE: Lazy<Node> = Lazy::new(|| {
    let freqs: Vec<i32> = FIELD_PATH_TABLE.iter().map(|op| op.weight).collect();
    build_huffman_tree(&freqs)
});

pub fn read_field_paths(r: &mut Reader, paths: &mut Vec<FieldPath>) -> usize {
    let mut fp = FieldPath::new();
    let mut node = &*HUFFMAN_TREE;
    let mut i = 0usize;
    while !fp.done {
        let next = if r.read_boolean() {
            node.right()
        } else {
            node.left()
        };
        if next.is_leaf() {
            node = &HUFFMAN_TREE;
            let idx = next.value();
            (FIELD_PATH_TABLE[idx].op)(r, &mut fp);
            if !fp.done {
                if paths.len() <= i {
                    paths.push(fp.copy_from());
                } else {
                    let x = &mut paths[i];
                    x.path = fp.path;
                    x.last = fp.last;
                    x.done = fp.done;
                }
                i += 1;
            }
        } else {
            node = next;
        }
    }
    i
}
