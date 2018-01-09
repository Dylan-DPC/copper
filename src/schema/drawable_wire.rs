use drawables;
use drawing;
use schema_parser::geometry;
use schema_parser::schema_file::{WireSegment, WireType};


pub struct DrawableWire {
    pub start: geometry::SchemaPoint2D,
    pub end: geometry::SchemaPoint2D,
    pub wire: Box<drawables::Drawable>,
}


impl DrawableWire {
    pub fn draw(&self, buffers: &mut drawing::Buffers){
        self.wire.draw(buffers);
    }

    pub fn from_schema(wire: &WireSegment) -> DrawableWire {
        let start = geometry::SchemaPoint2D::new(wire.start.x, -wire.start.y);
        let end = geometry::SchemaPoint2D::new(wire.end.x, -wire.end.y);
        let color = match wire.kind {
            WireType::Wire => drawing::Color::new(0.0, 0.28, 0.0, 1.0),
            WireType::Dotted => drawing::Color::new(0.0, 0.0, 0.48, 1.0),
            _ => drawing::Color::new(0.0, 0.28, 0.0, 1.0)
        };
        DrawableWire {
            start: start.clone(),
            end: end.clone(),
            wire: Box::new(drawables::loaders::load_line(color, &start, &end))
        }
    }
}