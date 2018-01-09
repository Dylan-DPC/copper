extern crate lyon;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate gfx_glyph;
extern crate glutin;
extern crate euclid;

#[macro_use] extern crate log;
extern crate env_logger;

extern crate schema_parser;


mod drawing;
mod drawables;
mod resource_manager;
mod visual_helpers;
mod library;
mod schema;


// use std::thread;
// use std::time;
use std::env;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;


use gfx::Device;
use gfx::traits::FactoryExt;


const CLEAR_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];


fn main() {
    let _ = env_logger::init();

    // Create a window with an event loop
    let (w, h) = (1800, 1000);
    let mut event_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_dimensions(w, h)
        .with_decorations(true)
        .with_title("Schema Renderer".to_string());
    let api = glutin::Api::OpenGl;
    let version = (3, 2);

    // Create the GL context
    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(api, version))
        .with_multisampling(8);
        // .with_vsync(true);


    info!("Hello world!");

    // Init the draw machinery and infer all handles
    let (
        window,
        mut device,
        mut factory,
        target,
        depth_stencil
    ) = gfx_window_glutin::init::<drawing::ColorFormat, drawing::DepthFormat>(window_builder, context, &event_loop);

    // Create an encoder which is in charge of drawing everything
    let encoder = gfx::Encoder::from(factory.create_command_buffer());

    // Create a resource manager, which will hold fonts and other assets
    let resource_manager = Rc::new(RefCell::new(resource_manager::ResourceManager::new(factory, target, depth_stencil, encoder)));

    // Load library and schema file
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please specify a .lib and a .sch file.");
        ::std::process::exit(1);
    }

    // Create a new Library from a file specified on the commandline
    let library = library::Library::new(&args[1]).unwrap();

    // Create and load a schema form a file specified on the commandline
    let mut schema = schema::Schema::new();

    schema.load(&library, args[2].clone());

    let shader = resource_manager.borrow_mut().factory.link_program(&drawables::loaders::VS_CODE, &drawables::loaders::FS_CODE).unwrap();
    let mut rasterizer = gfx::state::Rasterizer::new_fill();
    rasterizer.samples = Some(gfx::state::MultiSample);
    let program = resource_manager.borrow_mut().factory.create_pipeline_from_program(
        &shader,
        gfx::Primitive::TriangleList,
        rasterizer,
        drawing::pipe::new()
    ).unwrap();

    // Create a new ViewState which holds information about the current perspective, cursor, etc
    let mut view_state = drawing::ViewState::new(w, h);

    let bb = schema.get_bounding_box();
    view_state.update_from_box_pan(bb);

    let mut running = true;

    while running {
        event_loop.poll_events(|ev| {
            // println!("{:?}", ev);
            match ev {
                // The window was closed
                // We break the loop and let it go out of scope, which will close it finally
                glutin::Event::WindowEvent { event,.. } => {
                    // println!("{:?}", event);
                    match event {
                        glutin::WindowEvent::Closed => { running = false; },
                        glutin::WindowEvent::KeyboardInput {
                            input: glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Q),
                                modifiers: glutin::ModifiersState {
                                    ctrl: true,
                                    ..
                                },
                                ..
                            },
                            ..
                        } => { running = false; },
                        glutin::WindowEvent::Resized(w, h) => {
                            println!("Window resized to width={}, height={}", w, h);

                            // We must manually update the inner size of the window
                            window.set_inner_size(w, h);

                            // We also must manually resize the GL context, this 
                            window.resize(w, h);

                            view_state.update_from_resize(w, h);
                            let bb = schema.get_bounding_box();
                            view_state.update_from_box_pan(bb);
                            let target = &mut resource_manager.borrow_mut().target.clone();
                            let depth_stencil = &mut resource_manager.borrow_mut().depth_stencil.clone();
                            gfx_window_glutin::update_views(&window, target, depth_stencil);

                            resource_manager.borrow_mut().target = target.clone();
                            resource_manager.borrow_mut().depth_stencil = depth_stencil.clone();
                        },
                        glutin::WindowEvent::CursorMoved{position, ..} => {
                            view_state.cursor.x = position.0 as f32;
                            view_state.cursor.y = position.1 as f32;
                        },
                        glutin::WindowEvent::MouseWheel{delta, ..} => {
                            if let glutin::MouseScrollDelta::PixelDelta(_x, y) = delta {
                                view_state.update_from_zoom(y);
                            }
                            if let glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
                                view_state.update_from_zoom(y);
                            }
                        },
                        // glium::glutin::WindowEvent::MouseInput{
                        //     state: glium::glutin::ElementState::Pressed,
                        //     button: glium::glutin::MouseButton::Left,
                        //     ..
                        // } => {
                            // let mut c = view_state.cursor.clone();
                            // c.x =  (c.x / view_state.width  as f32) * 2.0 - 1.0;
                            // c.y = -(c.y / view_state.height as f32) * 2.0 - 1.0;

                        //     println!("{:?} => {:?}", c, view_state.current_perspective.inverse().unwrap().transform_point(&c));
                        // },
                        _ => ()
                    }
                },
                _ => ()
            }
            // let m = time::Duration::from_millis(1);
            // thread::sleep(m);
        });

        // Start a new frame
        // Color it uniformly to start off
        let t = resource_manager.borrow().target.clone();
        resource_manager.borrow_mut().encoder.clear(&t, CLEAR_COLOR);

        let start = Instant::now();

        let vbo = Vec::<drawing::Vertex>::new();
        let ibo = Vec::<u32>::new();
        let mut buffers = drawing::Buffers {
            vbo: vbo,
            ibo: ibo
        };

        // Draw the schema
        schema.draw(&mut buffers);

        println!("Length: {}, {}", buffers.vbo.len(), buffers.ibo.len());

        let (vbo, ibo) = resource_manager.borrow_mut().factory.create_vertex_buffer_with_slice(
            &buffers.vbo[..],
            &buffers.ibo[..]
        );

        let buf = resource_manager.borrow_mut().factory.create_constant_buffer(1);

        let bundle = gfx::pso::bundle::Bundle::new(ibo, program.clone(), drawing::pipe::Data { vbuf: vbo, locals: buf, out: resource_manager.borrow().target.clone() });

        let locals = drawing::Locals {
            perspective: view_state.current_perspective.to_row_arrays(),
            color: [0.61, 0.05, 0.04, 1.0],
        };
        resource_manager.borrow_mut().encoder.update_constant_buffer(&bundle.data.locals, &locals);

        bundle.encode(&mut resource_manager.borrow_mut().encoder);

        // Draw the coords and the kicad space coords at the cursor
        let cp = view_state.cursor.clone();
        let mut c = view_state.cursor.clone();
        c.x =  (c.x / view_state.width  as f32) * 2.0 - 1.0;
        c.y = -(c.y / view_state.height as f32) * 2.0 + 1.0;
        let kc = view_state.current_perspective.inverse().unwrap().transform_point3d(&c.to_3d());
        visual_helpers::draw_coords_at_cursor(resource_manager.clone(), cp.x, cp.y, c.x, c.y, kc.x, kc.y);

        // Finish up the current frame
        resource_manager.borrow_mut().encoder.flush(&mut device);

        // This should never fail and if it does we are screwed anyways, so we issue a safe shutdown.
        use glutin::GlContext;
        window.swap_buffers().unwrap();
        device.cleanup();

        let elapsed = start.elapsed();
        // or format as milliseconds:
        println!("Elapsed: {} ms", ((elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64));
    }
}
