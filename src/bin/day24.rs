use crate::Addr::*;
use crate::Inst::*;
use crate::Op::*;
use crate::Operand::*;
use crate::AST::*;
use core::i64::MAX;
use core::i64::MIN;
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
        _ => MIN,
    }
}

fn mknode(op: Op, a: &AST, b: &AST) -> AST {
    let res = match op {
        Mu => match (a, b) {
            (Leaf(V(0)), _) => Leaf(V(0)),
            (_, Leaf(V(0))) => Leaf(V(0)),
            (Leaf(V(1)), _) => b.clone(),
            (_, Leaf(V(1))) => a.clone(),

            // associativity
            (_, Node(Mu, y, x)) => {
                // println!("checking associativity");
                let ny = mknode(Mu, a, y);
                let nx = mknode(Mu, a, x);
                if depth(&ny) < depth(&nx) {
                    Node(Mu, Box::new(ny), x.clone())
                } else {
                    Node(Mu, Box::new(nx), y.clone())
                }
            }
            (Node(Mu, _, _), Leaf(V(_))) => mknode(Mu, b, a),
            // distributivity
            (Node(Ad, y, x), Leaf(V(_))) => {
                // println!("checking distributivity");
                let ny = mknode(Mu, b, y);
                let nx = mknode(Mu, b, x);
                Node(Ad, Box::new(ny), Box::new(nx))
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
            (Node(Mu, y, x), z) => {
                if *z == **x {
                    *y.clone()
                } else {
                    let ny = mknode(Di, y, z);
                    let nx = mknode(Di, x, z);
                    if depth(&ny) < depth(&nx) {
                        Node(Mu, Box::new(ny), x.clone())
                    } else {
                        Node(Mu, Box::new(nx), y.clone())
                    }
                }
            }
            _ => Node(Di, Box::new(a.clone()), Box::new(b.clone())),
        },
        Mo => match (a, b) {
            (Leaf(V(x)), Leaf(V(y))) => Leaf(V(x % y)),
            (_, Leaf(V(y))) if upper_bound(a) < *y => a.clone(),
            // (Node(Ad, x, y), Leaf(V(_))) => mknode(Ad, &mknode(Mo, x, b), &mknode(Mo, y, b)),
            // (Node(Mu, x, y), Leaf(V(_))) => mknode(Mu, &mknode(Mo, x, b), &mknode(Mo, y, b)),
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
    };
    res
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

fn eval_ast(ast: &AST, input: &Vec<u8>, resolve: &HashMap<Addr, i64>) -> i64 {
    match ast {
        Node(Ad, x, y) => eval_ast(&x, input, resolve) + eval_ast(&y, input, resolve),
        Node(Mu, x, y) => eval_ast(&x, input, resolve) * eval_ast(&y, input, resolve),
        Node(Di, x, y) => eval_ast(&x, input, resolve) / eval_ast(&y, input, resolve),
        Node(Mo, x, y) => eval_ast(&x, input, resolve) % eval_ast(&y, input, resolve),
        Node(Eq, x, y) => {
            let ex = eval_ast(&x, input, resolve);
            let ey = eval_ast(&y, input, resolve);
            if ex == ey {
                1
            } else {
                0
            }
        }
        Leaf(I(i)) => input[*i].into(),
        Leaf(V(x)) => *x,
        Leaf(A(ad)) => *resolve.get(ad).unwrap(),
    }
}

fn eval(alu: &AbsALU, input: &Vec<u8>, resolve: &HashMap<Addr, i64>) -> ALU {
    let mut res = ALU {
        x: eval_ast(&alu.x, &input, resolve),
        y: eval_ast(&alu.y, &input, resolve),
        z: eval_ast(&alu.z, &input, resolve),
        w: eval_ast(&alu.w, &input, resolve),
        input: input.to_vec(),
    };

    res
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

fn compute_result(prog: &Vec<Inst>, start: &ALU) -> ALU {
    prog.iter()
        .fold(start.clone(), |alu, inst| process(&alu, inst))
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
    Mul(X, V(0)),   // X = 0
    Add(X, A(Z)),   // X = Z
    Mod(X, V(26)),  // X = Z + 26
    Div(Z, V(26)),  // Z = Z / 26
    Add(X, V(-11)), // X = X - 11
    Eql(X, A(W)),   // X = if X == W then 1 else 0
    Eql(X, V(0)),   // X = if X == 0 then 1 else 0
    Mul(Y, V(0)),   // Y = 0
    Add(Y, V(25)),  // Y = 25
    Mul(Y, A(X)),   // Y = Y * X
    Add(Y, V(1)),   // Y = Y + 1
    Mul(Z, A(Y)),   // Z = Z + Y
    Mul(Y, V(0)),   // Y = Y * 0
    Add(Y, A(W)),   // Y = Y + W
    Add(Y, V(15)),  // Y = Y + 15
    Mul(Y, A(X)),   // Y = Y * X
    Add(Z, A(Y)),   // Z = Z + Y
];

/// Given a single equation with a single input and Z as unknowns
/// find all pairs of solutions
fn solve_ast(eq: &AST, tgt: i64) -> Vec<(u8, i64)> {
    let mut res = vec![];
    for i in 1..10 {
        let input = vec![i; 14];
        for z in -100..100 {
            let mut retz = HashMap::new();
            retz.insert(Z, z);
            if eval_ast(eq, &input, &retz) == tgt {
                res.push((i, z));
            }
        }
    }
    res.sort_by(|a, b| b.cmp(a));
    res
}

/// find sequence of inputs that solve the given system of equations
fn solve(eqs: &mut Vec<AST>, goal: i64, res: &mut Vec<u8>) {
    if eqs.is_empty() {
        return;
    }
    if let Some(ast) = eqs.pop() {
        let solutions = solve_ast(&ast, goal);
        println!("solving for {} = {}\n {:?}", ast, goal, solutions);
        for (i, z) in solutions {
            res.insert(0, i);
            println!("exploring solution {} {:?}", z, res);
            solve(eqs, z, res);
            if res.len() == 14 {
                return;
            }
            res.remove(0);
        }
        eqs.push(ast);
    }
}

fn main() {
    let init = AbsALU {
        x: Leaf(A(X)),
        y: Leaf(A(Y)),
        z: Leaf(A(Z)),
        w: Leaf(A(W)),
    };
    let args: Vec<String> = env::args().collect();

    let lb = args[1].parse::<usize>().unwrap();
    let ub = args[2].parse::<usize>().unwrap();
    let mut zs = vec![];
    for i in 0..14 {
        let res = abstract_interpret(&PROGRAM[i * 18..(i + 1) * 18].to_vec(), &init);
        zs.push(res.z.clone());
        println!("z: {}", res.z);
    }

    let mut res = vec![];
    solve(&mut zs, 0, &mut res);

    println!("result: {:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbolic_evaluation_matches_actual_evaluation() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 6, 7, 8, 9];
        let concrete_init = ALU {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            input: data.clone(),
        };
        let sym_init = AbsALU {
            x: Leaf(V(0)),
            y: Leaf(V(0)),
            z: Leaf(V(0)),
            w: Leaf(V(0)),
        };

        for i in 0..252 {
            let mut resolve = HashMap::new();
            resolve.insert(X, 0);
            resolve.insert(Y, 0);
            resolve.insert(Z, 0);
            resolve.insert(W, 0);

            let concrete_eval = compute_result(&PROGRAM[0..i].to_vec(), &concrete_init);
            let sym_eval = abstract_interpret(&PROGRAM[0..i].to_vec(), &sym_init);
            let concrete_sym = eval(&sym_eval, &data.clone(), &resolve);

            assert_eq!(concrete_eval.z, concrete_sym.z);
        }

        let mut z = 0;
        let abs_init = AbsALU {
            x: Leaf(A(X)),
            y: Leaf(A(Y)),
            z: Leaf(A(Z)),
            w: Leaf(A(W)),
        };

        for i in 0..14 {
            let az = abstract_interpret(&PROGRAM[i * 18..(i + 1) * 18].to_vec(), &abs_init);
            println!("ast : {}", az);
            let mut resolve = HashMap::new();
            resolve.insert(Z, z);

            z = eval_ast(&az.z, &data, &resolve);
        }
        assert_eq!(z, compute_result(&PROGRAM.to_vec(), &concrete_init).z);
    }
}
