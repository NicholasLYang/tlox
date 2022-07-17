use anyhow::anyhow;
use tree_sitter::{Language, Node, Parser, TreeCursor};

extern "C" {
    fn tree_sitter_tlox() -> Language;
}

fn main() -> Result<(), anyhow::Error> {
    parse("class Foo { fun bar() {} }")?;

    Ok(())
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
struct Program(Vec<Stmt>);

#[derive(Debug)]
enum Stmt {
    Var {
        name: String,
        initializer: Expr,
    },
    Expression(Expr),
    Print(Expr),
    Function {
        name: String,
        body: Vec<Stmt>,
        params: Vec<String>,
    },
    Class {
        name: String,
        super_class: Option<String>,
        methods: Vec<Stmt>,
    },
}

fn parse(code: &str) -> Result<(), anyhow::Error> {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_tlox() };
    parser.set_language(language).unwrap();

    let tree = parser.parse(code, None).unwrap();
    let root_node = tree.root_node();
    let mut cursor = root_node.walk();
    parse_program(code, &mut cursor)?;

    Ok(())
}

fn parse_program(code: &str, cursor: &mut TreeCursor) -> Result<Option<Program>, anyhow::Error> {
    if cursor.node().kind() == "program" {
        let mut declarations = Vec::new();
        let mut is_valid_child = cursor.goto_first_child();

        while is_valid_child {
            if cursor.node().kind() == "declaration" {
                cursor.goto_first_child();
                declarations.push(parse_declaration(code, cursor));
                cursor.goto_parent();
            }
            is_valid_child = cursor.goto_next_sibling();
        }
        println!("{:?}", declarations);
    }

    Ok(None)
}

fn parse_declaration<'a>(code: &str, cursor: &mut TreeCursor<'a>) -> Result<Stmt, anyhow::Error> {
    match cursor.node().kind() {
        "classDeclaration" => {
            let mut is_valid_child = cursor.goto_first_child();
            let mut name = None;
            let mut super_class = None;
            let mut methods = Vec::new();

            while is_valid_child {
                match cursor.field_name() {
                    Some("name") => {
                        name = Some(code[cursor.node().byte_range()].to_string());
                    }
                    Some("super_class") => {
                        super_class = Some(code[cursor.node().byte_range()].to_string());
                    }
                    Some(field) => return Err(anyhow!("Unexpected field `{}`", field)),
                    None if cursor.node().kind() == "function" => {
                        methods.push(parse_function(code, cursor)?);
                    }
                    None => {}
                }

                is_valid_child = cursor.goto_next_sibling();
            }

            Ok(Stmt::Class {
                name: name.ok_or_else(|| anyhow!("Class does not have a name"))?,
                super_class,
                methods,
            })
        }
        "funDeclaration" => {
            let mut is_valid_child = cursor.goto_first_child();

            while is_valid_child {
                match cursor.field_name() {
                    // TODO: Detect potential duplicates
                    Some("function") => return parse_function(code, cursor),
                    Some(field) => return Err(anyhow!("Unexpected field `{}`", field)),
                    None => {}
                }

                is_valid_child = cursor.goto_next_sibling();
            }

            Err(anyhow!("No function declaration found"))
        }
        kind => todo!("{:?}", kind),
    }
}

fn parse_function(code: &str, cursor: &mut TreeCursor) -> Result<Stmt, anyhow::Error> {
    let mut is_valid_child = cursor.goto_first_child();
    let mut name = None;
    let mut body = None;
    let mut params = None;

    while is_valid_child {
        match cursor.field_name() {
            Some("name") => {
                name = Some(code[cursor.node().byte_range()].to_string());
            }
            Some("body") => {
                body = Some(parse_block(cursor)?);
            }
            Some("params") => {
                params = Some(parse_parameters(cursor)?);
            }
            Some(field) => return Err(anyhow!("Unexpected field `{}`", field)),
            None => {}
        }

        is_valid_child = cursor.goto_next_sibling();
    }
    Ok(Stmt::Function {
        name: name.ok_or_else(|| anyhow!("Function does not have name"))?,
        body: body.ok_or_else(|| anyhow!("Function does not have body"))?,
        params: params.unwrap_or_default(),
    })
}

fn parse_block(cursor: &mut TreeCursor) -> Result<Vec<Stmt>, anyhow::Error> {
    Ok(Vec::new())
}

fn parse_parameters(cursor: &mut TreeCursor) -> Result<Vec<String>, anyhow::Error> {
    Ok(Vec::new())
}
