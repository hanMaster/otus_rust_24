use std::f32::consts::PI;

// Определяем трейт для элементов, которые могут быть посещены
trait Figure {
    fn area(&self, visitor: &dyn AreaVisitor);
}

// Конкретные элементы
struct Circle {
    radius: f32,
}

struct Square {
    side: i32,
}

impl Figure for Circle {
    fn area(&self, visitor: &dyn AreaVisitor) {
        visitor.area_circle(self);
    }
}

impl Figure for Square {
    fn area(&self, visitor: &dyn AreaVisitor) {
        visitor.area_square(self);
    }
}

// Трейт для посетителя
trait AreaVisitor {
    fn area_circle(&self, element: &Circle);
    fn area_square(&self, element: &Square);
}

// Конкретный посетитель
struct ConcreteVisitor;

impl AreaVisitor for ConcreteVisitor {
    fn area_circle(&self, figure: &Circle) {
        let area = figure.radius * figure.radius * PI;
        println!("Circle area: {area}");
    }

    fn area_square(&self, figure: &Square) {
        let area = figure.side * figure.side;
        println!("Square area: {area}");
    }
}

fn main() {
    let figures: Vec<Box<dyn Figure>> = vec![
        Box::new(Circle { radius: 4.0 }),
        Box::new(Square { side: 5 }),
    ];

    let visitor = ConcreteVisitor;

    for figure in figures {
        figure.area(&visitor);
    }
}
