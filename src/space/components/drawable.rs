use ecs;
use glium;

use info;

macro_rules! implement_drawable {
    ($struct_name:ident, $func_name:ident) => (
        #[derive(Clone)]
        pub struct $struct_name(
            pub Box<
                ::std::rc::Rc<
                    Fn(&ecs::System,
                       ecs::Entity,
                       &info::Info,
                       &mut glium::framebuffer::MultiOutputFrameBuffer),
                >,
            >
        );

        impl $struct_name {
            pub fn new<
                F: Fn(&ecs::System,
                   ecs::Entity,
                   &info::Info,
                   &mut glium::framebuffer::MultiOutputFrameBuffer)
                    + 'static,
            >(
                f: F,
            ) -> $struct_name {
                $struct_name(Box::new(::std::rc::Rc::new(f)))
            }
        }

        impl ::std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, stringify!($struct_name))
            }
        }

        pub fn $func_name(
            sys: &ecs::System,
            inf: &info::Info,
            context: &mut glium::framebuffer::MultiOutputFrameBuffer,
        ) {
            let _ = sys.run::<$struct_name, _>(|sys, ent| {
                let drawable = sys.borrow::<$struct_name>(ent).unwrap();
                drawable.0(sys, ent, inf, context);
            });
        }

    )
}

implement_drawable!(Drawable, draw);
implement_drawable!(DrawableBackground, draw_bg);
implement_drawable!(DrawableForeground, draw_fg);
