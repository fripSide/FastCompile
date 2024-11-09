#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product, // 乘积
    Sum,     // 和
    Number(u64),
    Paren, // 括号, parenthesis
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub entry: GrammarItem,
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    pub fn new() -> Self {
        ParseNode {
            entry: GrammarItem::Paren,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

fn get_number(c: char, it: &mut std::iter::Peekable<std::str::Chars>) -> u64 {
    let mut num = c.to_digit(10).unwrap() as u64;
    while let Some(&c) = it.peek() {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as u64;
            it.next();
        } else {
            break;
        }
    }
    num
}

/*
expr -> summand + expr | summand
summand -> term * summand | term
term -> NUMBER | ( expr )
 */

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(LexItem::Num(n));
            }
            '+' | '*' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character: {}", c));
            }
        }
    }
    Ok(result)
}

// expr -> summand + expr | summand
fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_summand, next_pos) = parse_summand(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_summand);
            let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        },
        _ => Ok((node_summand, next_pos)),
    }
}

// summand -> term * summand | term
fn parse_summand(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_items, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('*')) => {
           let mut product = ParseNode::new();
            product.entry = GrammarItem::Product;
            product.children.push(node_items);
            let (rhs, i) = parse_summand(tokens, next_pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => Ok((node_items, next_pos)),
    }
}

// term -> NUMBER | ( expr )
fn parse_term(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let c = tokens
        .get(pos)
        .ok_or("unexpected end of input, except number or paren")?;
    match c {
        &LexItem::Num(n) => {
            let mut node = ParseNode::new();
            node.entry = GrammarItem::Number(n);
            Ok((node, pos + 1))
        }
        &LexItem::Paren(c) => match c {
            '(' | '[' | '{' => parse_expr(tokens, pos + 1).and_then(|(node, next_pos)| {
                if let Some(&LexItem::Paren(c2)) = tokens.get(next_pos) {
                    if c2 == match_paren(c) {
                        let mut node_paren = ParseNode::new();
                        node_paren.entry = GrammarItem::Paren;
                        node_paren.children.push(node);
                        Ok((node_paren, next_pos + 1))
                    } else {
                        Err(format!(
                            "Excepted {}  but found: {:?} at pos: {}",
                            match_paren(c2), c2, next_pos))
                    }
                } else {
                    Err(format!("unexpected end of input at {}", next_pos))
                }
            }),
            _ => Err(format!("unexpected paren at {} buf found: {:?}", pos, c)),
        },
        _ => Err(format!("unexpected token: {:?}", c)),
    }
}

fn match_paren(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        // use for log
        '}' => '{',
        ']' => '[',
        ')' => '(',
        _ => panic!("unexpected paren: {}", c),
    }
}

pub fn parse(content: &str) -> Result<ParseNode, String> {
    let tokens = lex(&String::from(content))?;
    // dbg!(tokens);
    parse_expr(&tokens, 0).and_then(|(node, i)| {
        if i == tokens.len() {
            Ok(node)
        } else {
            Err(format!("unexpected end of input, found {:?} at {}", tokens[i], i))
        }
    })
}

pub fn dump(tree: &ParseNode) -> String {
    match tree.entry {
        GrammarItem::Paren => {
            format!("({})", dump(tree.children.get(0).expect("paren should have one child")))
        },
        GrammarItem::Sum => {
            let lhs = dump(tree.children.get(0).expect("sum should have two children"));
            let rhs = dump(tree.children.get(1).expect("sum should have two children"));
            format!("{} + {}", lhs, rhs)
        },
        GrammarItem::Product => {
            let lhs = dump(tree.children.get(0).expect("product should have two children"));
            let rhs = dump(tree.children.get(1).expect("product should have two children"));
            format!("{} * {}", lhs, rhs)
        },
        GrammarItem::Number(n) => {
            format!("{}", n)
        }
    }
}

pub fn eval(node: &ParseNode) -> u64 {
    match node.entry {
        GrammarItem::Paren => eval(node.children.get(0).expect("paren should have one child")),
        GrammarItem::Sum => {
            let lhs = eval(node.children.get(0).expect("sum should have two children"));
            let rhs = eval(node.children.get(1).expect("sum should have two children"));
            lhs + rhs
        },
        GrammarItem::Product => {
            let lhs = eval(node.children.get(0).expect("product should have two children"));
            let rhs = eval(node.children.get(1).expect("product should have two children"));
            lhs * rhs
        },
        GrammarItem::Number(n) => n,
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{eval, parse};

    #[test]
    fn test1() {
        let input = "1 + 2";
        let val = eval(&parse(input).expect("parse failed"));
        assert_eq!(val, 1 + 2);
    }
    
    #[test]
    fn test2() {
        let input = "1 + 2 * 3";
        let val = eval(&parse(input).expect("parse failed"));
        assert_eq!(val, 1 + 2 * 3);
    }
}