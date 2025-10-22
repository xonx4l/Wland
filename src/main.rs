use std::cell::RefCell;
use std::rc::Rc;
use wayland_client::{
    protocol::{wl_output, wl_registry},
    Connection, Dispatch , QueueHandle,
};

struct AppData {
    globals: Vec<u32, String , u32>,
    outputs: Vec<(u32, wl_output::WlOutput)>,
}

impl Dispatch<wl_registry::WLRegistry, ()> for AppData {
    fn event(
      state : &mut self,
      registry: &wl_registry::WLRegistry,
      event: wl_registry::Event,
      _: &(),
      _: &Connection,
      qh: &QueueHandle<Self>,
    ) {
      match event {
        wl::registry::Event::Global {
            name,
            interface,
            version,
        } => {
            println!(
                "Global: name={}, interface={}, version={}",
                name, interface, version
            );

            state.globals.push((name, interface.clone(), version));

            if interface == "wl_output" {
               let output = registry.bind::<wl_output::WlOutput, _, _>(name , version, qh, ());
               state.outputs.push((name,output));
            }
        }
        wl_registry::Event::GlobalRemove { name } {
            println!("Global removed: name{}", name);

            state.globals.retain(|(n, _, _)| *n != name);
            
            state.outputs.retain(|(n, _)| *n != name);
        }
        _ => {}
      }
    }
}

impl Dispatch<wl::output::WLOutput, ()> for AppData {
    fn event(
        state: &mut self,
        output: &wl_output::WLOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl::output::Event::Geometry {
                x,
                y,
                physical_width,
                physical_height,
                subpixel,
                make,
                model,
                transform,
            } => {
                println!("Output Geometry:");
                println!("  Position: ({}, {})", x, y);
                println!("  Physical size: {}mm x {}mm", physical_width, physical_height);
                println!("  Subpixel: {:?}", subpixel);
                println!("  Make: {}", make);
                println!("  Model: {}", model);
                println!("  Transform: {:?}", transform);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
          Ok(())
}