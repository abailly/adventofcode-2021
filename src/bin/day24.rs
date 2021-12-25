use crate::Addr::*;
use crate::Inst::*;
use crate::Operand::*;
use crate::AST::*;
use std::collections::HashMap;

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
    In(Box<AST>),
    Leaf(Operand),
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

fn abstract_process(alu: &AbsALU, inst: Inst) -> AbsALU {
    alu.clone()
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
            println!("{:?}", new_alu);
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
    let init = ALU {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
        input: vec![1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    };
    let res = compute_result(&PROGRAM.to_vec(), init);
    println!("min energy: {:?}", res);
}
