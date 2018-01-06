use std::cell::RefCell;
use std::rc::Rc;


use gfx_device_gl;


use drawables;
use schema_parser::component;
use schema_parser::component::geometry as component_geometry;
use resource_manager;


type Resources = gfx_device_gl::Resources;


pub fn load_text(
    _resource_manager: Rc<RefCell<resource_manager::ResourceManager>>,
    position: &component_geometry::Point,
    content: &String,
    dimension: f32,
    orientation: &component_geometry::TextOrientation,
    hjustify: component::Justify,
    vjustify: component::Justify
) -> drawables::TextDrawable {
    drawables::TextDrawable {
        position: position.clone(),
        content: content.clone(),
        dimension: dimension,
        orientation: orientation.clone(),
        hjustify: hjustify,
        vjustify: vjustify
    }
}