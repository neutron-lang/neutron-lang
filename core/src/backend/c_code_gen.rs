use crate::types::{bult_in_types::*, parse_nodes::*};
use std::iter::Peekable;

#[derive(Clone)]
pub struct CCodeGen {
    ast: Statement,
    current_node: Statement,
    pub code: String,
}

impl CCodeGen {
    pub fn new(input: Statement) -> Self {
        Self {
            ast: input.clone(),
            current_node: input.clone(),
            code: String::from(""),
        }
    }

    pub fn gen_c_code(&mut self) {
        match &self.current_node {
            Statement::Program { start, body } => self.gen_program(),
            _ => {}
        }
    }

    fn push_code(&mut self, code: &str) {
        self.code.push_str(code);
    }

    fn gen_program(&mut self) {
        self.push_code("#define str char*\n");

        match &self.ast {
            Statement::Program { body, .. } => {
                for state in body.clone().into_iter() {
                    self.current_node = state.clone();

                    match self.current_node {
                        Statement::FuctionDeclaration { .. } => self.gen_function(),
                        _ => {}
                    }

                    dbg!(&self.current_node);
                }
            }
            _ => {}
        }
    }

    fn get_type(&mut self, r#type: Types) -> String {
        match r#type {
            Types::Int => String::from("int"),
            Types::Float => String::from("float"),
            Types::Bool => String::from("bool"),
            Types::Char => String::from("char"),
            Types::Str => String::from("str"),
            Types::Void => String::from("void"),
            Types::Any => String::from("auto"),
        }
    }

    fn gen_args(&mut self, args: Option<Vec<FuncParam>>) -> String {
        let mut arguments = String::new();

        match args {
            Some(args) => {
                let mut index: usize = 0;

                for arg in args.clone() {
                    index += 1;

                    arguments
                        .push_str(format!("{} {}", self.get_type(arg.r#type), arg.name).as_str());

                    if index < args.len() {
                        arguments.push_str(",");
                    }
                }

                arguments
            }
            None => arguments,
        }
    }

    fn gen_function(&mut self) {
        let mut function = String::new();

        match self.current_node.clone() {
            Statement::FuctionDeclaration {
                name,
                r#type,
                params,
                body,
                ..
            } => {
                function.push_str(
                    format!(
                        "{} {}({}){}",
                        self.get_type(r#type.clone()),
                        name,
                        self.gen_args(params),
                        "{}",
                    )
                    .as_str(),
                );
            }
            _ => {}
        }

        self.push_code(function.as_str());
    }
}
