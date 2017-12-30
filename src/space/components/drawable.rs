use ecs;
use glium;

use info;

#[derive(Clone)]
pub struct Drawable(
    pub Box<
        ::std::rc::Rc<
            Fn(&ecs::System,
               ecs::Entity,
               &info::Info,
               &mut glium::framebuffer::MultiOutputFrameBuffer),
        >,
    >
);

impl Drawable {
    pub fn new<
        F: Fn(&ecs::System,
           ecs::Entity,
           &info::Info,
           &mut glium::framebuffer::MultiOutputFrameBuffer)
            + 'static,
    >(
        f: F,
    ) -> Drawable {
        Drawable(Box::new(::std::rc::Rc::new(f)))
    }
}

impl ::std::fmt::Debug for Drawable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Drawable")
    }
}

pub fn draw(
    sys: &ecs::System,
    inf: &info::Info,
    context: &mut glium::framebuffer::MultiOutputFrameBuffer,
) {
    let _ = sys.run::<Drawable, _>(|sys, ent| {
        let drawable = sys.borrow::<Drawable>(ent).unwrap();
        drawable.0(sys, ent, inf, context);
    });
}
