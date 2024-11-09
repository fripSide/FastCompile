use std::env;

mod parser;

fn main() {
    // println! is a macro that prints text to the console

    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);
    let terms = parser::parse("1234 + 43* (34 +[2])").unwrap();
    println!("{} = {}", parser::dump(&terms), parser::eval(&terms));
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate quickcheck;
    use rand::Rng;
    use quickcheck::{empty_shrinker, Arbitrary, Gen, quickcheck};
    use self::rand::distributions::{Distribution, Standard};
    use crate::parser::*;

    impl Distribution<GrammarItem> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GrammarItem {
            match rng.gen_range(0,4) {
                0 => GrammarItem::Product,
                1 => GrammarItem::Sum,
                2 => GrammarItem::Paren,
                3 => GrammarItem::Number(rng.gen()),
                _ => unreachable!(),
            }
        }
    }

    fn rec_node<G: Gen>(i: u32, g: &mut G) -> ParseNode {
        let mut node = ParseNode::new();
        if i > 1 {
            loop {
                node.entry = g.gen();
                match node.entry {
                    GrammarItem::Paren => {
                        node.children.push(rec_node(i - 1, g));
                        break;
                    },
                    GrammarItem::Sum | GrammarItem::Product => {
                        node.children.push(rec_node(i - 1, g));
                        node.children.push(rec_node(i - 1, g));
                        break;
                    },
                    GrammarItem::Number(_) => {},
                }
            }
        } else {
            node.entry = GrammarItem::Number(g.gen());
        }
        node
    }

    impl Arbitrary for ParseNode {
        fn arbitrary<G: Gen>(g: &mut G) -> ParseNode {
            rec_node(5, g)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = ParseNode>> {
            empty_shrinker()
        }
    }

    quickcheck! {
        fn prop(xs: ParseNode) -> bool {
            let s = dump(&xs);
            println!("instance {}", s);
            let ys = parse(&s).unwrap();
            let t = dump(&ys);
            
            t == s
        }
    }
}