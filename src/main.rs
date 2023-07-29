use std::io;

fn tokenize(code: Vec<char>) -> Vec<String> {
    let mut pos: usize = 0;
    let mut res: Vec<String> = vec![];

    while pos < code.len() {
        while pos < code.len() && " \n\t".contains(code[pos]) {
            pos += 1;
        }
        if pos >= code.len() {
            break;
        }
        else if code[pos].is_ascii_digit() {
            let mut num: String = String::new();
            while pos < code.len() && code[pos].is_ascii_digit() {
                num.push(code[pos]);
                pos += 1;
            }
            res.push(num);
        }
        else if "+-*/()".contains(code[pos]) {
            res.push(String::from(code[pos]));
            pos += 1;
        }
        else {
            panic!("char: {}", code[pos]);
        }
    }

    res.push(String::from(""));
    res
}

trait AST {
    fn visit(&self) -> i64;
}

struct NumValue {
    value: i64,
}

impl AST for NumValue {
    fn visit(&self) -> i64 {
        return self.value;
    }
}

struct BinaryOp {
    op: String,
    left: Box<dyn AST>,
    right: Box<dyn AST>,
}

impl AST for BinaryOp {
    fn visit(&self) -> i64 {
        if self.op == "+" {
            self.left.visit() + self.right.visit()
        }
        else if self.op == "-" {
            self.left.visit() - self.right.visit()
        }
        else if self.op == "*" {
            self.left.visit() * self.right.visit()
        }
        else if self.op == "/" {
            self.left.visit() / self.right.visit()
        }
        else {
            panic!();
        }
    }
}

struct Parser<'a> {
    tokens: &'a Vec<String>,
    pos: usize,
    token: String,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<String>) -> Self {
        Parser { tokens, pos: 0, token: tokens[0].clone() }
    }

    fn eat(&mut self, expect: &str) -> String {
        if self.token == expect {
            return self.next();
        }
        panic!();
    }

    fn next(&mut self) -> String {
        self.pos += 1;
        self.token = self.tokens[self.pos].clone();
        self.tokens[self.pos - 1].clone()
    }

    fn parse_expr(&mut self) -> Box<dyn AST> {
        let mut res = self.parse_term();
        while self.token == "+" || self.token == "-" {
            let op = self.next();
            let term = self.parse_term();
            res = Box::new(BinaryOp { op: op.clone(), left: res, right: term });
        }
        res
    }

    fn parse_term(&mut self) -> Box<dyn AST> {
        let mut res = self.parse_factor();
        while self.token == "*" || self.token == "/" {
            let op = self.next();
            let factor = self.parse_factor();
            res = Box::new(BinaryOp { op: op.clone(), left: res, right: factor });
        }
        res
    }

    fn parse_factor(&mut self) -> Box<dyn AST> {
        if self.token == "(" {
            self.next();
            let res = self.parse_expr();
            self.eat(")");
            res
        }
        else {
            Box::new(NumValue { value: self.next().parse().unwrap() })
        }
    }
}

fn main() {
    let mut code: String = String::new();

    io::stdin().read_line(&mut code).unwrap();

    let code = code.trim();

    let tokens = tokenize(code.chars().collect());

    let ast = Parser::new(&tokens).parse_expr();

    let res = ast.visit();

    println!("{}", res)
}
