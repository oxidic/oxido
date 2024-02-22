use std::{collections::HashMap, path::Path};

use inkwell::{
	builder::Builder, context::Context, module::Linkage, types::BasicMetadataTypeEnum, values::{BasicMetadataValueEnum, PointerValue}, AddressSpace
};

use crate::{
	ast::{Ast, Expression},
	data::DataType,
};

pub fn compile<'ctx>(context: &'ctx Context, ast: Ast) {
	let module = context.create_module("main_module");
	let builder = context.create_builder();
	let mut variables: HashMap<String, PointerValue<'ctx>> = HashMap::new();

	let i32_type = context.i32_type();
	let bool_type = context.bool_type();

	let str_type = context.i8_type().ptr_type(AddressSpace::default());
	let printf_type = i32_type.fn_type(&[BasicMetadataTypeEnum::PointerType(str_type)], true);
	let printf = module.add_function("printf", printf_type, Some(Linkage::External));

	let function_type = i32_type.fn_type(&[], false);
	let function = module.add_function("main", function_type, None);
	let entry_block = context.append_basic_block(function, "entry");

	builder.position_at_end(entry_block);

	for (node, _) in ast {
		match node {
			crate::ast::AstNode::Assignment(name, datatype, expr) => {
				match datatype {
					DataType::Vector(_) => todo!(),
					DataType::Str => todo!(),
					DataType::Int => {
						let alloca = builder.build_alloca(i32_type, &name).unwrap();

						variables.insert(name, alloca);

						builder
							.build_store(
								alloca,
								compile_expression(context, &builder, expr, variables.clone()).into_int_value(),
							)
							.unwrap();
					}
					DataType::Bool => {
						let alloca = builder.build_alloca(bool_type, &name).unwrap();

						variables.insert(name, alloca);

						builder
							.build_store(
								alloca,
								compile_expression(context, &builder, expr, variables.clone()).into_int_value(),
							)
							.unwrap();
					}
				};
			}
			crate::ast::AstNode::ReAssignment(_, _) => todo!(),
			crate::ast::AstNode::VecReAssignment(_, _, _) => todo!(),
			crate::ast::AstNode::If(_, _) => todo!(),
			crate::ast::AstNode::IfElse(_, _, _) => todo!(),
			crate::ast::AstNode::Loop(_) => todo!(),
			crate::ast::AstNode::FunctionCall(name, exprs) => {
				builder.build_call(printf, &exprs.iter().map(|expr| compile_expression(context, &builder, expr.clone(), variables.clone())).collect::<Vec<_>>(), &name).unwrap();
			},
			crate::ast::AstNode::FunctionDeclaration(_, _, _, _) => todo!(),
			crate::ast::AstNode::Break => todo!(),
			crate::ast::AstNode::Return(_) => todo!(),
			crate::ast::AstNode::Exit(_) => todo!(),
		}
	}

	builder.build_return(Some(&i32_type.const_int(0, false))).unwrap();

	module.print_to_stderr();
	module.print_to_file(Path::new("test.ll")).unwrap();
}

pub fn compile_expression<'a>(
	context: &'a Context,
	builder: &'a Builder<'a>,
	expr: Expression,
	variables: HashMap<String, PointerValue<'a>>
) -> BasicMetadataValueEnum<'a> {
	let i32_type = context.i32_type();
	let bool_type = context.bool_type();

	match expr {
		Expression::BinaryOperation(_, _, _) => todo!(),
		Expression::Str(str) => BasicMetadataValueEnum::ArrayValue(context.const_string(str.as_bytes(), false)),
		Expression::Int(n) => BasicMetadataValueEnum::IntValue(i32_type.const_int(n.try_into().unwrap(), false)),
		Expression::Bool(b) => BasicMetadataValueEnum::IntValue(bool_type.const_int(b as u64, false)),
		Expression::FunctionCall(_, _) => todo!(),
		Expression::Identifier(a) => BasicMetadataValueEnum::PointerValue(*variables.get(&a).unwrap()),
		Expression::Vector(_, _) => todo!(),
		Expression::VecIndex(_, _) => todo!(),
	}
}
