use std::any::{Any, TypeId};

// Определяем объекты, которые будут принимать визитера
trait Visitable {
    fn accept<V>(&self, visitor: &mut V)
    where
        V: Visitor + ?Sized;
}

struct A;
impl Visitable for A {
    fn accept<V>(&self, visitor: &mut V)
    where
        V: Visitor + ?Sized,
    {
        visitor.visit_any(self as &dyn Any);
    }
}

struct B;
impl Visitable for B {
    fn accept<V>(&self, visitor: &mut V)
    where
        V: Visitor + ?Sized,
    {
        visitor.visit_any(self as &dyn Any);
    }
}

// Определяем интерфейс визитера
trait Visitor {
    fn visit_any(&mut self, obj: &dyn Any);
}

// Реализация конкретного визитера
struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit_any(&mut self, obj: &dyn Any) {
        if let Some(a) = obj.downcast_ref::<A>() {
            println!("Visiting A");
        } else if let Some(b) = obj.downcast_ref::<B>() {
            println!("Visiting B");
        }
    }
}

fn main() {
    let a = A {};
    let b = B {};

    let mut visitor = ConcreteVisitor {};

    a.accept(&mut visitor);
    b.accept(&mut visitor);
}
