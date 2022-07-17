use anyhow::anyhow;
use tree_sitter::{Language, Node, Parser, TreeCursor};

extern "C" {
    fn tree_sitter_tlox() -> Language;
}

fn main() -> Result<(), anyhow::Error> {
    parse("class Foo {}")?;

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
    Function {},
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

fn get_singular_field<'a>(field_name: &str, cursor: &mut TreeCursor<'a>) -> Option<Node<'a>> {
    let mut children: Vec<_> = cursor
        .node()
        .children_by_field_name(field_name, cursor)
        .collect();
    if children.len() == 1 {
        children.pop()
    } else {
        None
    }
}

fn parse_declaration<'a>(code: &str, cursor: &mut TreeCursor<'a>) -> Result<Stmt, anyhow::Error> {
    match cursor.node().kind() {
        "classDeclaration" => {
            let name = get_singular_field("name", cursor)
                .ok_or_else(|| anyhow!("No `name` in class declaration"))?;
            let name = code[name.byte_range()].to_string();

            let super_class = get_singular_field("super", cursor);
            let super_class = super_class.map(|s| code[s.byte_range()].to_string());

            let methods = cursor
                .node()
                .children_by_field_name("function", cursor)
                .map(parse_function)
                .collect();

            Ok(Stmt::Class {
                name,
                super_class,
                methods,
            })
        }
        kind => todo!("{:?}", kind),
    }
}

fn parse_function<'a>(node: Node<'a>) -> Stmt {
    Stmt::Function {}
}
