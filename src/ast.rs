use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::{map, value},
    multi::fold_many0,
    sequence::{delimited, preceded},
};



#[derive(Debug, Clone)]
pub enum AST {
    Variable(String),
    Conjunction(Box<AST>, Box<AST>),
    Disjunction(Box<AST>, Box<AST>),
    Implication(Box<AST>, Box<AST>),
    Negation(Box<AST>),
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
    if let Ok((input, rhs)) = preceded(
        ws(alt((tag("->"), tag("→"), tag("⇒")))),
        implication,
    )(input)
    {
        Ok((input, AST::Implication(Box::new(lhs), Box::new(rhs))))
    } else {
        Ok((input, lhs))
    }
}



pub fn expr(input: &str) -> IResult<&str, AST> {
    implication(input)
}
