use crate::Addr::*;
use crate::Inst::*;
use crate::Operand::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::process;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ALU {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
    input: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Inst {
    Inp(Addr),
    Add(Addr, Operand),
    Mul(Addr, Operand),
    Div(Addr, Operand),
    Mod(Addr, Operand),
    Eql(Addr, Operand),
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
        Inp(addr) => {
            let v = new_alu.input.pop().unwrap();
            write(&mut new_alu, addr, v.into());
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

static program: [Inst; 252] = [
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Inp(W),
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
    Add(Y, V(15)),
    Mul(Y, A(X)),
    Add(Z, A(Y)),
];

fn main() {
    let init = ALU {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
        input: [9; 14].to_vec(),
    };
    let res = compute_result(&program.to_vec(), init);
    println!("min energy: {:?}", res);
}
