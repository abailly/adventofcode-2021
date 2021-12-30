use crate::Addr::*;
use crate::Inst::*;
use crate::Op::*;
use crate::Operand::*;
use crate::AST::*;
use core::i64::MAX;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ALU {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
    input: Vec<u8>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum Addr {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Operand {
    A(Addr),
    V(i64),
    I(usize), // input index
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Inst {
    Inp(Addr, Operand),
    Add(Addr, Operand),
    Mul(Addr, Operand),
    Div(Addr, Operand),
    Mod(Addr, Operand),
    Eql(Addr, Operand),
}

// a tree of operations leading to some result
#[derive(Debug, Clone, PartialEq, Eq)]
enum AST {
    Node(Op, Box<AST>, Box<AST>),
    Leaf(Operand),
}

fn depth(a: &AST) -> usize {
    match a {
        Node(_, l, r) => 1 + depth(l).max(depth(r)),
        _ => 0,
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Ad,
    Mu,
    Di,
    Mo,
    Eq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AbsALU {
    x: AST,
    y: AST,
    z: AST,
    w: AST,
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ad => write!(f, "+"),
            Mu => write!(f, "*"),
            Di => write!(f, "/"),
            Mo => write!(f, "%"),
            Eq => write!(f, "=="),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            A(addr) => write!(f, "{:?}", addr),
            V(n) => write!(f, "{}", n),
            I(i) => write!(f, "[{}]", i),
        }
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node(op, l, r) => write!(f, "({} {} {})", op, l, r),
            Leaf(op) => write!(f, "{}", op),
        }
    }
}

impl Display for AbsALU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x ({}) = {}\ny ({}) = {}\nz ({}) = {}\nw ({}) = {}",
            depth(&self.x),
            self.x,
            depth(&self.y),
            self.y,
            depth(&self.z),
            self.z,
            depth(&self.w),
            self.w
        )
    }
}

fn abs_read(alu: &AbsALU, addr: Addr) -> AST {
    match addr {
        X => alu.x.clone(),
        Y => alu.y.clone(),
        Z => alu.z.clone(),
        W => alu.w.clone(),
    }
}

fn abs_decode(alu: &AbsALU, op: &Operand) -> AST {
    match op {
        A(addr) => abs_read(alu, *addr),
        o => Leaf(*o),
    }
}

fn abs_write(alu: &mut AbsALU, addr: Addr, op: AST) {
    match addr {
        X => {
            alu.x = op;
        }
        Y => {
            alu.y = op;
        }
        Z => {
            alu.z = op;
        }
        W => {
            alu.w = op;
        }
    }
}

fn upper_bound(a: &AST) -> i64 {
    match a {
        Node(Ad, x, y) => upper_bound(x) + upper_bound(y),
        Node(Mu, x, y) => upper_bound(x) * upper_bound(y),
        Node(Di, x, _y) => upper_bound(x),
        Node(Mo, _x, y) => upper_bound(y),
        Node(Eq, _, _) => 1,
        Leaf(I(_)) => 9,
        Leaf(V(x)) => *x,
        _ => MAX,
    }
}

fn lower_bound(a: &AST) -> i64 {
    match a {
        Node(Ad, x, y) => lower_bound(x) + lower_bound(y),
        Node(Mu, x, y) => lower_bound(x) * lower_bound(y),
        Node(Di, _, _) => 0,
        Node(Mo, _, _) => 0,
        Node(Eq, _, _) => 0,
        Leaf(I(_)) => 1,
        Leaf(V(x)) => *x,
        _ => MAX,
    }
}

fn mknode(op: Op, a: &AST, b: &AST) -> AST {
    match op {
        Mu => match (a, b) {
            (Leaf(V(0)), _) => Leaf(V(0)),
            (_, Leaf(V(0))) => Leaf(V(0)),
            (Leaf(V(1)), _) => b.clone(),
            (_, Leaf(V(1))) => a.clone(),

            // commutativity
            (Leaf(V(z)), Node(Mu, y, x)) => match **x {
                Leaf(V(t)) => Node(Mu, y.clone(), Box::new(Leaf(V(z * t)))),
                _ => Node(Mu, Box::new(a.clone()), Box::new(b.clone())),
            },
            (Node(Mu, y, x), Leaf(V(z))) => match **x {
                Leaf(V(t)) => Node(Mu, y.clone(), Box::new(Leaf(V(z * t)))),
                _ => Node(Mu, Box::new(a.clone()), Box::new(b.clone())),
            },
            // distributivity
            (Node(Ad, y, x), Leaf(V(_))) => {
                // println!("checking distributivity");
                let ny = mknode(Mu, b, y);
                let nx = mknode(Mu, b, x);
                if depth(&ny) < depth(&nx) {
                    Node(Ad, Box::new(ny), x.clone())
                } else {
                    Node(Ad, Box::new(nx), y.clone())
                }
            }
            (Leaf(V(_)), Node(Ad, _, _)) => mknode(Mu, b, a),
            (Leaf(V(x)), Leaf(V(y))) => Leaf(V(x * y)),
            _ => Node(Mu, Box::new(a.clone()), Box::new(b.clone())),
        },
        Ad => match (a, b) {
            (Leaf(V(0)), _) => b.clone(),
            (_, Leaf(V(0))) => a.clone(),
            (Leaf(V(x)), Leaf(V(y))) => Leaf(V(x + y)),
            // associativity of addition
            (Leaf(V(_)), Node(Ad, y, x)) => {
                // println!("checking associativity");
                let ny = mknode(Ad, a, y);
                let nx = mknode(Ad, a, x);
                if depth(&ny) < depth(&nx) {
                    Node(Ad, Box::new(ny), x.clone())
                } else {
                    Node(Ad, Box::new(nx), y.clone())
                }
            }
            (Node(Ad, _, _), Leaf(V(_))) => mknode(Ad, b, a),
            _ => Node(Ad, Box::new(a.clone()), Box::new(b.clone())),
        },
        Di => match (a, b) {
            (_, Leaf(V(1))) => a.clone(),
            (Leaf(V(x)), Leaf(V(y))) => Leaf(V(x / y)),
            _ => Node(Di, Box::new(a.clone()), Box::new(b.clone())),
        },
        Mo => match (a, b) {
            (Leaf(V(x)), Leaf(V(y))) => Leaf(V(x % y)),
            (_, Leaf(V(y))) if upper_bound(a) < *y => a.clone(),
            (Node(Ad, x, y), Leaf(V(_))) => mknode(Ad, &mknode(Mo, x, b), &mknode(Mo, y, b)),
            (Node(Mu, x, y), Leaf(V(_))) => mknode(Mu, &mknode(Mo, x, b), &mknode(Mo, y, b)),
            _ => Node(Mo, Box::new(a.clone()), Box::new(b.clone())),
        },
        Eq => match (a, b) {
            (Leaf(V(x)), Leaf(V(y))) => {
                if x == y {
                    Leaf(V(1))
                } else {
                    Leaf(V(0))
                }
            }
            _ => {
                if lower_bound(a) > upper_bound(b) || upper_bound(a) < lower_bound(b) {
                    Leaf(V(0))
                } else {
                    Node(Eq, Box::new(a.clone()), Box::new(b.clone()))
                }
            }
        },
    }
}

fn abstract_process(alu: &AbsALU, inst: Inst) -> AbsALU {
    let mut new_alu = alu.clone();
    match inst {
        Inp(addr, opr) => {
            let a = abs_decode(&new_alu, &opr);
            abs_write(&mut new_alu, addr, a);
        }
        Add(addr, opr) => {
            let a = abs_read(&new_alu, addr);
            let b = abs_decode(&new_alu, &opr);
            abs_write(&mut new_alu, addr, mknode(Ad, &a, &b));
        }
        Mul(addr, opr) => {
            let a = abs_read(&new_alu, addr);
            let b = abs_decode(&new_alu, &opr);
            abs_write(&mut new_alu, addr, mknode(Mu, &a, &b));
        }
        Div(addr, opr) => {
            let a = abs_read(&new_alu, addr);
            let b = abs_decode(&new_alu, &opr);
            abs_write(&mut new_alu, addr, mknode(Di, &a, &b));
        }
        Mod(addr, opr) => {
            let a = abs_read(&new_alu, addr);
            let b = abs_decode(&new_alu, &opr);
            abs_write(&mut new_alu, addr, mknode(Mo, &a, &b));
        }
        Eql(addr, opr) => {
            let a = abs_read(&new_alu, addr);
            let b = abs_decode(&new_alu, &opr);
            abs_write(&mut new_alu, addr, mknode(Eq, &a, &b));
        }
    }
    new_alu
}

fn abstract_interpret(prog: &Vec<Inst>, start: &AbsALU) -> AbsALU {
    prog.iter()
        .fold(start.clone(), |alu, inst| abstract_process(&alu, *inst))
}

fn read(alu: &ALU, addr: &Addr) -> i64 {
    match addr {
        X => alu.x,
        Y => alu.y,
        Z => alu.z,
        W => alu.w,
    }
}

fn decode(alu: &ALU, op: &Operand) -> i64 {
    match op {
        A(addr) => read(alu, addr),
        V(v) => *v,
        I(i) => alu.input[*i].into(),
    }
}

fn write(alu: &mut ALU, addr: &Addr, val: i64) {
    match addr {
        X => {
            alu.x = val;
        }
        Y => {
            alu.y = val;
        }
        Z => {
            alu.z = val;
        }
        W => {
            alu.w = val;
        }
    }
}

fn process(alu: &ALU, inst: &Inst) -> ALU {
    let mut new_alu = alu.clone();
    match inst {
        Inp(addr, opr) => {
            let b = decode(&new_alu, opr);
            write(&mut new_alu, addr, b);
        }
        Add(addr, opr) => {
            let a = read(&new_alu, addr);
            let b = decode(&new_alu, opr);
            write(&mut new_alu, addr, a + b);
        }
        Mul(addr, opr) => {
            let a = read(&new_alu, addr);
            let b = decode(&new_alu, opr);
            write(&mut new_alu, addr, a * b);
        }
        Div(addr, opr) => {
            let a = read(&new_alu, addr);
            let b = decode(&new_alu, opr);
            write(&mut new_alu, addr, a / b);
        }
        Mod(addr, opr) => {
            let a = read(&new_alu, addr);
            let b = decode(&new_alu, opr);
            write(&mut new_alu, addr, a % b);
        }
        Eql(addr, opr) => {
            let a = read(&new_alu, addr);
            let b = decode(&new_alu, opr);
            write(&mut new_alu, addr, if a == b { 1 } else { 0 });
        }
    }
    new_alu
}

fn compute_result(prog: &Vec<Inst>, start: ALU) -> ALU {
    prog.iter().fold(start, |alu, inst| process(&alu, inst))
}

static PROGRAM: [Inst; 252] = [
    Inp(W, I(0)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(1)),
    Add(X, V(12)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(6)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(1)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(1)),
    Add(X, V(10)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(6)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(2)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(1)),
    Add(X, V(13)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(3)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(3)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(26)),
    Add(X, V(-11)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(11)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(4)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(1)),
    Add(X, V(13)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(9)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(5)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(26)),
    Add(X, V(-1)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(3)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(6)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(1)),
    Add(X, V(10)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(13)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(7)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(1)),
    Add(X, V(11)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(6)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(8)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(26)),
    Add(X, V(0)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(14)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(9)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(1)),
    Add(X, V(10)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(10)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(10)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(26)),
    Add(X, V(-5)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(12)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(11)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(26)),
    Add(X, V(-16)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(10)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(12)),
    Mul(X, V(0)),
    Add(X, A(Z)),
    Mod(X, V(26)),
    Div(Z, V(26)),
    Add(X, V(-7)),
    Eql(X, A(W)),
    Eql(X, V(0)),
    Mul(Y, V(0)),
    Add(Y, V(25)),
    Mul(Y, A(X)),
    Add(Y, V(1)),
    Mul(Z, A(Y)),
    Mul(Y, V(0)),
    Add(Y, A(W)),
    Add(Y, V(11)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
    Inp(W, I(13)),  // W = ?
    Mul(X, V(0)),   // X = X * 0
    Add(X, A(Z)),   // X = X + Z
    Mod(X, V(26)),  // X = X + 26
    Div(Z, V(26)),  // Z = Z / 26
    Add(X, V(-11)), // X = X - 11
    Eql(X, A(W)),   // X = if X == W then 1 else 0
    Eql(X, V(0)),   // X = if X == 0 then 1 else 0
    Mul(Y, V(0)),   // Y = Y * 0
    Add(Y, V(25)),  // Y = Y + 25
    Mul(Y, A(X)),   // Y = Y * X
    Add(Y, V(1)),   // Y = Y + 1
    Mul(Z, A(Y)),   // Z = Z + Y
    Mul(Y, V(0)),   // Y = Y * 0
    Add(Y, A(W)),   // Y = Y + W
    Add(Y, V(15)),  // Y = Y + 15
    Mul(Y, A(X)),   // Y = Y * X
    Add(Z, A(Y)),   // Z = Z + Y
];

fn main() {
    let init = AbsALU {
        x: Leaf(V(0)),
        y: Leaf(V(0)),
        z: Leaf(V(0)),
        w: Leaf(V(0)),
    };
    let args: Vec<String> = env::args().collect();

    let num = args[1].parse::<usize>().unwrap();
    let res = abstract_interpret(&PROGRAM[0..num].to_vec(), &init);
    println!("min energy: {}", res);
}
