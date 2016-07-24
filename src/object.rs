use std::rc::Rc;
use rier::{Id, Transform, Camera2D};
use rier::texture::{Texture, Rect};
use rier::render::Renderer;
use graphics::Graphics;


/// Sprite game object.
pub struct Sprite {
    /// Object ID.
    pub id: Id,
    /// Rendering data.
    pub graphics: Graphics,
    /// Object transform.
    pub transform: Transform,
}


impl Sprite {
    /// Creates new sprite object.
    pub fn new(tex: Rc<Texture>, rect: Rect, (width, height): (f32, f32)) -> Sprite {
        Sprite {
            id: Id::new(),
            graphics: Graphics::new(tex, rect, width, height),
            transform: Transform::new(),
        }
    }

    /// Renders this sprite.
    pub fn render(&self,
                  renderer: &Renderer<Graphics>,
                  camera: &Camera2D) {
        self.graphics.render(renderer, camera, &self.transform)
    }
}
