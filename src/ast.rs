use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::{map, value},
    multi::fold_many0,
    sequence::{delimited, preceded},
};
use std::fmt;



#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AST {
    Variable(String),
    Conjunction(Box<AST>, Box<AST>),
    Disjunction(Box<AST>, Box<AST>),
    Implication(Box<AST>, Box<AST>),
    BiImplication(Box<AST>, Box<AST>),
    Negation(Box<AST>),
}



impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AST::Variable(name) => write!(f, "{}", name),

            AST::Negation(inner) => {
                match **inner {
                    AST::Variable(_) | AST::Negation(_) => write!(f, "¬{}", inner),
                    _ => write!(f, "¬({})", inner),
                }
            }

            AST::Conjunction(left, right) => {
                write!(f, "({} ∧ {})", left, right)
            }

            AST::Disjunction(left, right) => {
                write!(f, "({} ∨ {})", left, right)
            }

            AST::Implication(left, right) => {
                write!(f, "({} → {})", left, right)
            }

            AST::BiImplication(left, right) => {
                write!(f, "({} ↔ {})", left, right)
            }
        }
    }
}



fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}



fn variable(input: &str) -> IResult<&str, AST> {
    map(ws(alpha1), |s: &str| AST::Variable(s.to_string()))(input)
}



fn parens(input: &str) -> IResult<&str, AST> {
    delimited(ws(tag("(")), expr, ws(tag(")")))(input)
}



fn atom(input: &str) -> IResult<&str, AST> {
    alt((parens, variable))(input)
}



fn negation(input: &str) -> IResult<&str, AST> {
    alt((
        map(
            preceded(ws(alt((tag("!"), tag("¬")))), negation),
            |e| AST::Negation(Box::new(e)),
        ),
        atom,
    ))(input)
}



fn conjunction(input: &str) -> IResult<&str, AST> {
    let (input, head) = negation(input)?;
    fold_many0(
        preceded(
            ws(alt((tag("&"), tag("∧"), tag("^"), tag("and")))),
            negation,
        ),
        move || head.clone(),
        |acc, val| AST::Conjunction(Box::new(acc), Box::new(val)),
    )(input)
}



fn disjunction(input: &str) -> IResult<&str, AST> {
    let (input, head) = conjunction(input)?;
    fold_many0(
        preceded(
            ws(alt((tag("|"), tag("v"), tag("∨"), tag("or")))),
            conjunction,
        ),
        move || head.clone(),
        |acc, val| AST::Disjunction(Box::new(acc), Box::new(val)),
    )(input)
}



fn implication(input: &str) -> IResult<&str, AST> {
    let (input, lhs) = disjunction(input)?;

    let mut rest = preceded(
        ws(alt((tag("->"), tag("→")))), // Remove '⇒' here
        implication,
    );

    match rest(input) {
        Ok((input, rhs)) => Ok((input, AST::Implication(Box::new(lhs), Box::new(rhs)))),
        Err(_) => Ok((input, lhs)),
    }
}



fn biimplication(input: &str) -> IResult<&str, AST> {
    let (input, lhs) = implication(input)?;

    fold_many0(
        preceded(
            ws(alt((tag("<->"), tag("↔"), tag("⇔"), tag("⇒")))), // Add '⇒' here for biimplication
            implication,
        ),
        move || lhs.clone(),
        |acc, val| AST::BiImplication(Box::new(acc), Box::new(val)),  // Use BiImplication here
    )(input)
}




pub fn expr(input: &str) -> IResult<&str, AST> {
    biimplication(input)
}
