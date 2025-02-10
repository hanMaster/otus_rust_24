use std::f32::consts::PI;

// Определяем трейт для элементов, которые могут быть посещены
trait Figure {
    fn accept(&self, visitor: &impl Visitor);
}

// Конкретные элементы
struct Circle {
    radius: f32,
}

struct Square {
    side: i32,
}

impl Figure for Circle {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.accept_circle(self);
    }
}

impl Figure for Square {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.accept_square(self);
    }
}

// Трейт для посетителя
trait Visitor {
    fn accept_circle(&self, element: &Circle);
    fn accept_square(&self, element: &Square);
}

// Конкретный посетитель
struct AreaVisitor;

impl Visitor for AreaVisitor {
    fn accept_circle(&self, figure: &Circle) {
        let area = figure.radius * figure.radius * PI;
        println!("Circle area: {area}");
    }

    fn accept_square(&self, figure: &Square) {
        let area = figure.side * figure.side;
        println!("Square area: {area}");
    }
}

fn main() {
    let circle = Circle { radius: 4.0 };
    let square = Square { side: 5 };

    let visitor = AreaVisitor;

    circle.accept(&visitor);
    square.accept(&visitor);
}
