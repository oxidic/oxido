use std::collections::HashMap;

use inkwell::{context::Context, values::PointerValue};

use crate::{ast::Ast, data::DataType};

pub fn compile<'ctx>(context: &'ctx Context, ast: Ast) {
	let module = context.create_module("main_module");
	let builder = context.create_builder();
	let mut variables: HashMap<String, PointerValue<'ctx>> = HashMap::new();

	let i32_type = context.i32_type();
	let bool_type = context.bool_type();

	let function_type = i32_type.fn_type(&[], false);
	let function = module.add_function("main", function_type, None);
	let entry_block = context.append_basic_block(function, "entry");

	builder.position_at_end(entry_block);

	for (node, _) in ast {
		match node {
			crate::ast::AstNode::Assignment(name, datatype, _) => {
				let llvmtype = match datatype {
					DataType::Vector(_) => todo!(),
					DataType::Str => todo!(),
					DataType::Int => i32_type,
					DataType::Bool => bool_type,
				};
				let alloca = builder.build_alloca(llvmtype, &name);
				variables.insert(name.to_string(), alloca.unwrap());
			}
			crate::ast::AstNode::ReAssignment(_, _) => todo!(),
			crate::ast::AstNode::VecReAssignment(_, _, _) => todo!(),
			crate::ast::AstNode::If(_, _) => todo!(),
			crate::ast::AstNode::IfElse(_, _, _) => todo!(),
			crate::ast::AstNode::Loop(_) => todo!(),
			crate::ast::AstNode::FunctionCall(_, _) => todo!(),
			crate::ast::AstNode::FunctionDeclaration(_, _, _, _) => todo!(),
			crate::ast::AstNode::Break => todo!(),
			crate::ast::AstNode::Return(_) => todo!(),
			crate::ast::AstNode::Exit(_) => todo!(),
		}
	}

	module.print_to_stderr();
}
