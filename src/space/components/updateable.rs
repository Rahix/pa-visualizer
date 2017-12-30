use ecs;
use info;

#[derive(Clone)]
pub struct Updateable(pub Box<::std::rc::Rc<Fn(&mut ecs::System, ecs::Entity, &info::Info)>>);

impl Updateable {
    pub fn new<F: Fn(&mut ecs::System, ecs::Entity, &info::Info) + 'static>(f: F) -> Updateable {
        Updateable(Box::new(::std::rc::Rc::new(f)))
    }
}

impl ::std::fmt::Debug for Updateable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Updateable")
    }
}

pub fn update(sys: &mut ecs::System, inf: &info::Info) {
    let _ = sys.run_mut::<Updateable, _>(|sys, ent| {
        let updateable = sys.get::<Updateable>(ent).unwrap();
        updateable.0(sys, ent, inf);
    });
}
