use crate::{
	data::{DataType, Param},
	token::Token,
};
use std::ops::Range;

pub type Ast = Vec<(AstNode, Range<usize>)>;

#[derive(Clone, Debug)]
pub enum AstNode {
	Assignment(String, DataType, Expression),
	ReAssignment(String, Expression),
	VecReAssignment(String, Expression, Expression),
	If(Expression, Ast),
	IfElse(Expression, Ast, Ast),
	Loop(Ast),
	FunctionCall(String, Vec<Expression>),
	FunctionDeclaration(String, Vec<Param>, Option<DataType>, Ast),
	Break,
	Return(Expression),
	Exit(Expression),
}

#[derive(Clone, Debug)]
pub enum Expression {
	BinaryOperation(Box<Expression>, Token, Box<Expression>),
	Str(String),
	Int(i32),
	Bool(bool),
	FunctionCall(String, Vec<Expression>),
	Identifier(String),
	Vector(Vec<Expression>, Option<DataType>),
	VecIndex(String, Box<Expression>),
}

impl Expression {
	pub fn infer_datatype(&self) -> Option<DataType> {
		match self {
			Expression::BinaryOperation(lhs, _, rhs) => {
				let lhs = Self::infer_datatype(lhs);
				let rhs = Self::infer_datatype(rhs);

				if lhs.is_none() || rhs.is_none() {
					return None;
				};

				let lhs = lhs.unwrap();
				let rhs = rhs.unwrap();

				Some(match (lhs, rhs) {
					(DataType::Vector(t), _) => DataType::Vector(t),
					(_, DataType::Vector(t)) => DataType::Vector(t),
					(DataType::Str, _) => DataType::Str,
					(_, DataType::Str) => DataType::Str,
					(DataType::Int, _) => DataType::Int,
					(_, DataType::Int) => DataType::Int,
					(DataType::Bool, _) => DataType::Bool,
				})
			}
			Expression::Str(_) => Some(DataType::Str),
			Expression::Int(_) => Some(DataType::Int),
			Expression::Bool(_) => Some(DataType::Bool),
			Expression::FunctionCall(_, _) => None,
			Expression::Identifier(_) => None,
			Expression::Vector(_, d) => d.clone(),
			Expression::VecIndex(_, _) => None,
		}
	}
}
