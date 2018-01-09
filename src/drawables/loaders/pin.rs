use schema_parser::geometry;
use drawables;
use drawing;
use schema_parser::component;
use schema_parser::component::geometry as component_geometry;


const PIN_RADIUS: f32 = 10.0;

pub fn load_pin(
    position: &geometry::SchemaPoint2D,
    length: f32,
    orientation: &component_geometry::PinOrientation,
    name: Option<String>,
    number: usize,
    number_size: usize,
    name_size: usize
) -> drawables::GroupDrawable {
    // Create a new group drawable
    let mut group = drawables::GroupDrawable::default();

    let circle = super::load_circle(drawing::Color::new(0.61, 0.05, 0.04, 1.0), position, PIN_RADIUS, false);

    let orientation_vec = geometry::SchemaVector2D::new(orientation.unit_vec().x, orientation.unit_vec().y);
    let end_position = position.clone() + (orientation_vec * length);

    let number_pos = end_position + (orientation_vec * -10.0);
    let number_pos = geometry::SchemaPoint2D::new(number_pos.x, number_pos.y + 60.0);

    let number_orientation = match orientation {
        &component_geometry::PinOrientation::Up => component_geometry::TextOrientation::Vertical,
        &component_geometry::PinOrientation::Down => component_geometry::TextOrientation::Vertical,
        &component_geometry::PinOrientation::Right => component_geometry::TextOrientation::Horizontal,
        &component_geometry::PinOrientation::Left => component_geometry::TextOrientation::Horizontal
    };

    let number_hjustify = match orientation {
        &component_geometry::PinOrientation::Up => component::Justify::Right,
        &component_geometry::PinOrientation::Down => component::Justify::Left,
        &component_geometry::PinOrientation::Right => component::Justify::Right,
        &component_geometry::PinOrientation::Left => component::Justify::Left
    };

    let number_text = super::load_text(&number_pos, &format!("{}", number), number_size as f32, &number_orientation, number_hjustify, component::Justify::Center);

    if let Some(name) = name {
        let name_pos = end_position + orientation_vec * 20.0;
        let name_pos = geometry::SchemaPoint2D::new(name_pos.x, name_pos.y + 25.0);
        let name_hjustify = match orientation {
            &component_geometry::PinOrientation::Up => component::Justify::Left,
            &component_geometry::PinOrientation::Down => component::Justify::Right,
            &component_geometry::PinOrientation::Right => component::Justify::Left,
            &component_geometry::PinOrientation::Left => component::Justify::Right
        };
        let name_text = super::load_text(&name_pos, &name, name_size as f32, &number_orientation, name_hjustify, component::Justify::Center);
        group.add(name_text);
    }

    let line = super::load_line(drawing::Color::new(0.61, 0.05, 0.04, 1.0), position, &end_position);

    group.add(line);
    group.add(circle);
    group.add(number_text);

    group
}