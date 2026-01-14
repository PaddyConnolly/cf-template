#![allow(unused)]
use std::io::{self, BufRead, BufWriter, Write};

struct Scanner<R> {
    reader: R,
    tokens: std::collections::VecDeque<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            tokens: std::collections::VecDeque::new(),
        }
    }

    /// Reads the next token (word) and parses it
    fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.tokens.is_empty() {
            let mut line = String::new();
            self.reader
                .read_line(&mut line)
                .expect("Failed to read line");
            for word in line.split_whitespace() {
                self.tokens.push_back(word.to_string());
            }
        }
        self.tokens
            .pop_front()
            .unwrap()
            .parse()
            .ok()
            .expect("Parse error")
    }

    /// Helper for vectors: let v: Vec<i32> = scan.vec(n);
    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

// --- USEFUL UTILITIES ---

/// Modular exponentiation: (base^exp) % m
fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut res = 1;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % m;
        }
        base = (base * base) % m;
        exp /= 2;
    }
    res
}

/// Simple Disjoint Set Union (DSU)
struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }
    fn unite(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}

// --- CORE LOGIC ---

fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, out: &mut BufWriter<W>) {
    let t: usize = scan.next();

    for _ in 0..t {
        let s: String = scan.next();
        let n = s.len();

        if n <= 10 {
            writeln!(out, "{}", s);
        } else {
            let l = s.chars().nth(0).unwrap();
            let r = s.chars().nth(n - 1).unwrap();
            let m = (n - 2).to_string();

            writeln!(out, "{}{}{}", l, m, r);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Lock stdin/stdout for performance
    let mut scan = Scanner::new(stdin.lock());
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut scan, &mut out);

    out.flush().unwrap();
}
