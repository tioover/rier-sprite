use std::rc::Rc;
use rier::{Renderer, Transform, Camera2D, Cache, mesh};
use rier::texture::{Texture, Rect};
use rier::render;


type Mesh = mesh::Mesh<Vertex>;


/// 2D vertex type.
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32, u: i32, v: i32) -> Vertex {
        Vertex {
            position: [x, y],
            tex_coords: [u as f32, v as f32],
        }
    }
}


implement_vertex!{ Vertex, position, tex_coords }


impl render::Shader for Graphics {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
        include_str!("sprite.vert")
    }

    fn fragment() -> &'static str {
        include_str!("sprite.frag")
    }
}


/// Sprite graphics component.
pub struct Graphics {
    opacity: f32,
    width: f32,
    height: f32,
    texture: Rc<Texture>,
    texture_rect: Rect,
    mesh_cache: Cache<Mesh>,
}

impl Graphics {
    pub fn new(texture: Rc<Texture>, tex_rect: Rect, width: f32, height: f32)
        -> Graphics
    {
        Graphics {
            texture_rect: tex_rect,
            width: width,
            height: height,
            texture: texture.clone(),
            opacity: 1.0,
            mesh_cache: Cache::new(),
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn get_mesh<'a>(&'a self, renderer: &Renderer<Self>) -> &'a Mesh {
        self.mesh_cache.get(|| {
            // Generate mash.
            let (width, height) = (self.width, self.height);
            let (w, h) = (self.texture_rect.w as i32, self.texture_rect.h as i32);
            let (x, y) = (self.texture_rect.x, self.texture_rect.y);
            let verties = [Vertex::new(  0.0, height, x + 0, y + h),
                           Vertex::new(  0.0,    0.0, x + 0, y + 0),
                           Vertex::new(width,    0.0, x + w, y + 0),
                           Vertex::new(width, height, x + w, y + h),];
            let indices = [0, 1, 2, 3, 2, 0];
            Mesh::with_indices(renderer, &verties, &indices).unwrap()
        })
    }

    /// Renders this sprite.
    pub fn render(&self,
                  renderer: &render::Renderer<Self>,
                  camera: &Camera2D,
                  transform: &Transform) {
        let tex = &*self.texture;

        let uniforms = uniform!
        {
            tex: tex,
            opacity: self.opacity,
            camera: camera,
            transform: transform,
        };

        let mesh = self.get_mesh(renderer);
        renderer.draw(mesh, &uniforms).unwrap();
    }
}
