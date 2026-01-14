#![allow(unused)]
use std::io::{self, BufRead, BufWriter, Write};

/// Optimized Scanner for 2024 edition
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
    // 1. Read test cases (if any)
    let t: u8 = scan.next();
    let ans = if t > 2 && t % 2 == 0 { "YES" } else { "NO" };
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Lock stdin/stdout for performance
    let mut scan = Scanner::new(stdin.lock());
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut scan, &mut out);

    // Explicit flush is good practice, though BufWriter handles it on drop
    out.flush().unwrap();
}
