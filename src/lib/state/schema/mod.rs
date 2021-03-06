pub mod component;
pub mod component_instance;

pub use self::component::Component;
pub use self::component_instance::ComponentInstance;
pub use viewing::view_state::ViewState;

use std::f32::consts::PI;

use uuid::Uuid;
use state::event::{EventMessage, EventBusHandle};

use geometry::*;
use state::component_libraries::ComponentLibraries;
use parsing::kicad::schema::*;

/// Represents a schema containing all its components and necessary resource references
pub struct Schema {
    components: Vec<ComponentInstance>,
    wires: Vec<WireSegment>,
    event_bus: EventBusHandle,
}

impl Schema {
    /// Creates a new blank schema
    pub fn new(event_bus: EventBusHandle) -> Schema {
        Schema {
            wires: Vec::new(),
            components: Vec::new(),
            event_bus: event_bus,
        }
    }

    /// This function infers the bounding box containing all boundingboxes of the objects contained in the schema
    pub fn get_bounding_box(&self, libraries: &ComponentLibraries) -> AABB {
        let mut aabb = AABB::new(
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 0.0)
        );
        self.components.iter().for_each(|instance| {
            libraries.get_component_by_name(&instance.name).map(|component| {
                let bb = &instance.get_boundingbox(component);
                use ncollide2d::bounding_volume::BoundingVolume;
                aabb.merge(bb);
            });
        });
        aabb
    }

    pub fn get_component_instance(&self, component_uuid: &Uuid) -> &ComponentInstance {
        self.components.iter().find(|c| c.uuid == *component_uuid).unwrap()
    }

    pub fn get_component_instance_mut(&mut self, component_uuid: &Uuid) -> &mut ComponentInstance {
        self.components.iter_mut().find(|c| c.uuid == *component_uuid).unwrap()
    }

    pub fn get_wire_instance(&self, wire_uuid: Uuid) -> &WireSegment {
        self.wires.iter().find(|c| c.uuid == wire_uuid).unwrap()
    }

    pub fn get_wire_instance_mut(&mut self, wire_uuid: Uuid) -> &mut WireSegment {
        self.wires.iter_mut().find(|c| c.uuid == wire_uuid).unwrap()
    }

    pub fn rotate_component(&mut self, component_uuid: &Uuid) {
        let instance = self.get_component_instance_mut(component_uuid);
        instance.rotation *= Matrix4::from_axis_angle(
            &Vector3::z_axis(),
            PI / 2.0
        );
        let instance = instance.clone();
        self.event_bus.send(&EventMessage::UpdateComponent(instance));
    }

    pub fn add_component(&mut self, mut instance: ComponentInstance) -> Uuid {
        instance.uuid = Uuid::new_v4();
        self.components.push(instance.clone());
        self.event_bus.send(&EventMessage::AddComponent(instance.clone()));
        return instance.uuid.clone()
    }

    pub fn add_wire(&mut self, mut instance: WireSegment) {
        instance.uuid = Uuid::new_v4();
        self.wires.push(instance.clone());
        self.event_bus.send(&EventMessage::AddWire(instance));
    }

    pub fn update_wire(&mut self, ws: WireSegment) {
        self.wires.swap_remove(self.wires.iter().enumerate().find(|&e| e.1.uuid == ws.uuid).unwrap().0);
        self.wires.push(ws.clone());
        self.event_bus.send(&EventMessage::UpdateWire(ws));
    }

    pub fn move_component(&mut self, component_uuid: &Uuid, translation: Vector2) {
        let instance = self.get_component_instance_mut(component_uuid);
        // TODO change this to a translation instead of setting the position maybe?
        instance.position = Point2::origin() + translation.clone();
        let instance = instance.clone();
        self.event_bus.send(&EventMessage::UpdateComponent(instance));
    }
}