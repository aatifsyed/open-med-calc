use boa_ast::{
    declaration::{Binding, LexicalDeclaration, VarDeclaration, Variable, VariableList},
    expression::{operator::assign::AssignTarget, Identifier},
    visitor::{NodeRef, Visitor},
    Expression,
};
use std::ops::ControlFlow;

#[derive(Debug, Default)]
struct VarVisitor {
    events: Vec<Event>,
}

enum State {
    DeclaredButNotInitialized,
    DeclaredAndInitialized,
    Accessed,
}

#[derive(Debug, Clone)]
enum Event {
    DeclaredAndInitialised(Identifier),
    DeclaredButNotInitialized(Identifier),
    Assigned(Identifier),
    Accessed(Identifier),
}

impl Event {
    fn identifier(&self) -> Identifier {
        match self {
            Event::DeclaredAndInitialised(i)
            | Event::DeclaredButNotInitialized(i)
            | Event::Assigned(i)
            | Event::Accessed(i) => *i,
        }
    }
}

impl VarVisitor {
    fn declared_variable_list<'a>(&mut self, node: &'a VariableList) -> ControlFlow<NodeRef<'a>> {
        for variable in node.as_ref() {
            self.variable_declared(variable)?;
        }
        ControlFlow::Continue(())
    }
    fn variable_declared<'a>(&mut self, node: &'a Variable) -> ControlFlow<NodeRef<'a>> {
        let (binding, init) = (node.binding(), node.init());
        match (binding, init) {
            (Binding::Identifier(i), None) => {
                self.events.push(Event::DeclaredButNotInitialized(*i));
                ControlFlow::Continue(())
            }
            (Binding::Identifier(i), Some(_)) => {
                self.events.push(Event::DeclaredAndInitialised(*i));
                ControlFlow::Continue(())
            }
            _ => ControlFlow::Break(NodeRef::from(node)),
        }
    }
    fn variable_assigned(&mut self, node: &Identifier) {
        self.events.push(Event::Assigned(*node))
    }
    fn variable_accessed(&mut self, node: &Identifier) {
        self.events.push(Event::Accessed(*node))
    }
}

impl<'ast> Visitor<'ast> for VarVisitor {
    type BreakTy = NodeRef<'ast>;
    // note: `fn visit_variable(..)` also visits function parameters, which we don't want
    fn visit_lexical_declaration(
        &mut self,
        node: &'ast LexicalDeclaration,
    ) -> ControlFlow<Self::BreakTy> {
        match node {
            LexicalDeclaration::Const(variable_list) | LexicalDeclaration::Let(variable_list) => {
                self.declared_variable_list(variable_list)?;
                self.visit(variable_list)?;
            }
        };
        ControlFlow::Continue(())
    }
    fn visit_var_declaration(&mut self, node: &'ast VarDeclaration) -> ControlFlow<Self::BreakTy> {
        let VarDeclaration(variable_list) = node;
        self.declared_variable_list(variable_list)?;
        self.visit(variable_list)?;
        ControlFlow::Continue(())
    }
    fn visit_assign_target(&mut self, node: &'ast AssignTarget) -> ControlFlow<Self::BreakTy> {
        match node {
            AssignTarget::Identifier(identifier) => {
                self.variable_assigned(identifier);
                self.visit(identifier)
            }
            AssignTarget::Access(property_access) => {
                ControlFlow::Break(NodeRef::from(property_access))
            }
            AssignTarget::Pattern(pattern) => ControlFlow::Break(NodeRef::from(pattern)),
        }
    }
    fn visit_expression(&mut self, node: &'ast Expression) -> ControlFlow<Self::BreakTy> {
        match node {
            Expression::Identifier(identifier) => {
                self.variable_accessed(identifier);
                self.visit(identifier)
            }
            Expression::Literal(node) => self.visit(node),
            Expression::ArrayLiteral(node) => self.visit(node),
            Expression::ObjectLiteral(node) => self.visit(node),
            Expression::Spread(node) => self.visit(node),
            Expression::Function(node) => self.visit(node),
            Expression::ArrowFunction(node) => self.visit(node),
            Expression::AsyncArrowFunction(node) => self.visit(node),
            Expression::Generator(node) => self.visit(node),
            Expression::AsyncFunction(node) => self.visit(node),
            Expression::AsyncGenerator(node) => self.visit(node),
            Expression::Class(node) => self.visit(&**node),
            Expression::TemplateLiteral(node) => self.visit(node),
            Expression::PropertyAccess(node) => self.visit(node),
            Expression::New(node) => self.visit(node),
            Expression::Call(node) => self.visit(node),
            Expression::SuperCall(node) => self.visit(node),
            Expression::ImportCall(node) => self.visit(node),
            Expression::Optional(node) => self.visit(node),
            Expression::TaggedTemplate(node) => self.visit(node),
            Expression::Assign(node) => self.visit(node),
            Expression::Unary(node) => self.visit(node),
            Expression::Update(node) => self.visit(node),
            Expression::Binary(node) => self.visit(node),
            Expression::BinaryInPrivate(node) => self.visit(node),
            Expression::Conditional(node) => self.visit(node),
            Expression::Await(node) => self.visit(node),
            Expression::Yield(node) => self.visit(node),
            Expression::Parenthesized(node) => self.visit(node),
            Expression::NewTarget | Expression::ImportMeta | Expression::This => {
                ControlFlow::Continue(())
            }
            node => ControlFlow::Break(NodeRef::from(node)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::norm::ParsedJavaScript;

    #[test]
    fn print() {
        let (interner, script, _) =
            ParsedJavaScript::new(include_str!("../js/19-absolute-neutrophil-count-anc.js"))
                .unwrap()
                .into_parts();
        let mut var_checker = VarVisitor::default();
        var_checker.visit(&script);
        for event in var_checker.events {
            let name = interner.resolve_expect(event.identifier().sym());
            match event {
                Event::DeclaredAndInitialised(_) => println!("init   {name}"),
                Event::DeclaredButNotInitialized(_) => println!("declar {name}"),
                Event::Assigned(_) => println!("assign {name}"),
                Event::Accessed(_) => println!("access {name}"),
            }
        }
    }
}
