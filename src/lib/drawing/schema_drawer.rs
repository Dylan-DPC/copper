use std::sync::{
    Arc,
    RwLock,
};

use uuid::Uuid;

use super::gfx_machinery::GfxMachinery;

use state::schema::*;
use state::component_libraries::*;
use state::event::{Listener, EventMessage};

use drawing;
use drawing::drawables::loaders::load_rectangle;
use drawing::drawables::Drawable;
use drawing::drawables::schema::{
    ComponentInstanceDrawable,
    WireDrawable
};
use parsing::kicad::schema::WireSegment;
use geometry::*;

pub struct SchemaDrawer {
    _schema: Arc<RwLock<Schema>>,
    view_state: Arc<RwLock<ViewState>>,
    libraries: Arc<RwLock<ComponentLibraries>>,
    gfx_machinery: GfxMachinery,
}

impl SchemaDrawer {
    pub fn new(schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, libraries: Arc<RwLock<ComponentLibraries>>) -> SchemaDrawer {
        SchemaDrawer {
            _schema: schema,
            view_state: view_state,
            libraries: libraries,
            gfx_machinery: GfxMachinery::new(),
        }
    }

    pub fn add_wire(&mut self, wire: WireSegment) {
        let drawable_wire = Box::new(WireDrawable::from_schema(0, &wire));
        self.gfx_machinery.add_drawable(&wire.uuid, drawable_wire);
    }

    pub fn remove_wire(&mut self, wire: WireSegment) {
        self.gfx_machinery.remove_drawable(&wire.uuid);
    }

    pub fn add_rect(&mut self, uuid: &Uuid, rect: &AABB, color: drawing::Color) {
        let drawable_rect = Box::new(load_rectangle(0, None, Some(color), rect));
        self.gfx_machinery.add_drawable(uuid, drawable_rect);
    }

    pub fn update_rect(&mut self, uuid: &Uuid, rect: &AABB, color: drawing::Color) {
        let drawable_rect = Box::new(load_rectangle(0, None, Some(color), rect));
        self.gfx_machinery.remove_drawable(uuid);
        self.gfx_machinery.add_drawable(uuid, drawable_rect);
    }

    pub fn add_drawable(&mut self, uuid: &Uuid, drawable: Box<dyn Drawable>) {
        self.gfx_machinery.add_drawable(uuid, drawable);
    }

    pub fn remove_drawable(&mut self, uuid: &Uuid) {
        self.gfx_machinery.remove_drawable(uuid);
    }

    pub fn update_wire(&mut self, wire: WireSegment) {
        let drawable_wire = Box::new(WireDrawable::from_schema(0, &wire));
        self.gfx_machinery.remove_drawable(&wire.uuid);
        self.gfx_machinery.add_drawable(&wire.uuid, drawable_wire);
    }

    fn get_drawable_mut(&mut self, uuid: &Uuid) -> Option<&mut dyn Drawable> {
        self.gfx_machinery.get_drawable_mut(uuid)
    }
}

impl Listener for SchemaDrawer {
    fn receive(&mut self, msg: &EventMessage) {
        match msg {
            EventMessage::AddComponent(instance) => {
                let component_instance_drawable_instance = {
                    let libraries = self.libraries.write().unwrap();
                    libraries.get_component_by_name(&instance.name).map(|component| {
                        let mut component_instance_drawable_instance = Box::new(
                            ComponentInstanceDrawable::new(
                                0,
                                component
                            )
                        );
                        component_instance_drawable_instance.set_transform(&instance.get_transform().into());
                        component_instance_drawable_instance
                    })
                };
                component_instance_drawable_instance.map(|d| self.gfx_machinery.add_drawable(&instance.uuid,d));
            },
            EventMessage::AddWire(instance) => {
                let drawable_wire = Box::new(WireDrawable::from_schema(
                    0,
                    &instance
                ));
                self.gfx_machinery.add_drawable(&instance.uuid, drawable_wire);
            },
            EventMessage::RemoveWire(instance) => {
                self.gfx_machinery.remove_drawable(&instance.uuid);
            },
            EventMessage::UpdateWire(instance) => {
                let drawable_wire = Box::new(WireDrawable::from_schema(
                    0,
                    &instance
                ));
                self.gfx_machinery.remove_drawable(&instance.uuid);
                self.gfx_machinery.add_drawable(&instance.uuid, drawable_wire);
            },
            EventMessage::DrawSchema => self.gfx_machinery.draw(&self.view_state.read().unwrap()),
            EventMessage::ResizeDrawArea(w, h) => {
                self.gfx_machinery.resize_target(*w, *h);
            },
            EventMessage::UpdateComponent(instance) => {
                self.get_drawable_mut(&instance.uuid).map(|d| d.set_transform(&instance.get_transform()));
            },
            _ => (),
        }
    }
}