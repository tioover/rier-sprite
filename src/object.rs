use rier::{Id, Transform, Camera2D, texture};
use rier::render::{Frame, DrawError, Renderer};
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
    pub fn new(tex: &texture::Ref, rect: texture::Rect, (width, height): (f32, f32)) -> Sprite {
        Sprite {
            id: Id::new(),
            graphics: Graphics::new(tex, rect, width, height),
            transform: Transform::new(),
        }
    }

    /// Renders this sprite.
    pub fn render(&self,
                  target: &mut Frame,
                  renderer: &Renderer<Graphics>,
                  camera: &Camera2D)
                  -> Result<(), DrawError> {
        self.graphics.render(target, renderer, camera, &self.transform)
    }
}