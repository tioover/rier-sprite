extern crate rier;
extern crate rier_sprite;

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use rier::texture;
use rier::{Context, Gfx, WindowEvent, Camera2D};
use rier::loader::Resource;
use rier::event::{Notifier, Return};
use rier_sprite::Sprite;


struct Block {
    sprite: Rc<RefCell<Sprite>>,
}

impl Block {
    fn new(gfx: Gfx, notifier: &mut Notifier<WindowEvent>) -> Block {
        let texture = texture::RawImage::load(&PathBuf::from("examples/block.png"))
            .unwrap()
            .process(&gfx)
            .unwrap();
        let sprite = Sprite::new(
            Rc::new(texture),
            texture::Rect { w: 256, h: 256, x: 0, y: 0 },
            (100.0, 100.0));
        let block = Block{ sprite: Rc::new(RefCell::new(sprite)) };
        block.event_register(notifier);
        block
    }

    fn event_register(&self, notifier: &mut Notifier<WindowEvent>) {
        let weak = Rc::downgrade(&self.sprite);
        notifier.register(move |e| {
            match e {
                &WindowEvent::MouseMoved(x, y) => {
                    match weak.upgrade() {
                        None => Return::Dead,
                        Some(sprite) => {
                            let mut sprite = sprite.borrow_mut();
                            sprite.transform.set_position(x as f32, y as f32, 0.0);
                            Return::Next
                        }
                    }
                },
                _ => Return::Next,
            }
        })
    }
}


fn main()
{
    let gfx = Context::create("Sprite", (800, 600)).gfx();
    let renderer = rier::render::Renderer::new(gfx.clone()).unwrap();
    let mut notifier = Notifier::new();
    let mut camera = Camera2D::new(gfx.clone());
    let block = Block::new(gfx.clone(), &mut notifier);
    'main: loop {
        let (_, h) = gfx.display.get_framebuffer_dimensions();
        camera.update();

        for event in gfx.display.poll_events() {
            match event {
                WindowEvent::Closed => break 'main,
                WindowEvent::MouseMoved(x, y) =>
                    notifier.notify(WindowEvent::MouseMoved(x, h as i32 - y)),
                e => notifier.notify(e),
            }
        }

        gfx.frame(|| {
            block.sprite.borrow().render(&renderer, &camera);
        }).unwrap();
    }
}
