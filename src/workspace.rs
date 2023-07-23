pub mod fs;

use std::{
    io::{Read, Write},
    rc::Rc,
    slice::from_ref,
};

pub trait Component: std::fmt::Debug + Clone + for<'a> From<&'a str> + 'static {}

pub trait Workspace {
    type Component: Component;
    type Source: Read + Write;

    fn open(&self, location: &[Self::Component]) -> Self::Source;
    fn create(&self, location: &[Self::Component]) -> Self::Source;
    fn insert_descendants(
        &self,
        descendants: &mut Vec<Rc<[Self::Component]>>,
        location: &[Self::Component],
        skip: usize,
    );

    fn read<T: IntoComponents<Self::Component>>(&self, location: T) -> Rc<[u8]> {
        let mut vec = Vec::new();
        self.open(location.into_components().as_components())
            .read_to_end(&mut vec)
            .unwrap();
        vec.into()
    }

    fn write<T: IntoComponents<Self::Component>>(&self, location: T, bytes: &[u8]) {
        self.create(location.into_components().as_components())
            .write_all(bytes)
            .unwrap();
    }

    fn descendants_from(&self, location: &[Self::Component]) -> Vec<Rc<[Self::Component]>> {
        let mut descendants = Vec::new();
        self.insert_descendants(&mut descendants, location, location.len());
        descendants
    }
}

pub trait IntoComponents<C> {
    type Output: AsComponents<C>;

    fn into_components(self) -> Self::Output;
}

impl<C: Component> IntoComponents<C> for &[&dyn AsComponents<C>] {
    type Output = Rc<[C]>;

    fn into_components(self) -> Self::Output {
        let mut vec = Vec::new();
        for item in self.into_iter() {
            for component in item.as_components().into_iter() {
                vec.push(component.clone());
            }
        }
        vec.into()
    }
}

impl<'a, C: Component> IntoComponents<C> for &'a str {
    type Output = C;

    fn into_components(self) -> Self::Output {
        self.into()
    }
}

impl<C: Component> IntoComponents<C> for C {
    type Output = C;

    fn into_components(self) -> Self::Output {
        self
    }
}

impl<'a, C: Component> IntoComponents<C> for &'a [C] {
    type Output = &'a [C];

    fn into_components(self) -> Self::Output {
        self
    }
}

impl<C: Component> IntoComponents<C> for Rc<[C]> {
    type Output = Rc<[C]>;

    fn into_components(self) -> Self::Output {
        self
    }
}

pub trait AsComponents<C> {
    fn as_components(&self) -> &[C];
}

impl<C: Component> AsComponents<C> for C {
    fn as_components(&self) -> &[C] {
        from_ref(self)
    }
}

impl<'a, C: Component> AsComponents<C> for &'a [C] {
    fn as_components(&self) -> &[C] {
        self
    }
}

impl<C: Component> AsComponents<C> for Rc<[C]> {
    fn as_components(&self) -> &[C] {
        self.as_ref()
    }
}

#[macro_export]
macro_rules! loc {
    ([$($x:expr),*] as $sty:ty) => {
        &[$(&<_ as IntoComponents<$sty>>::into_components($x).as_components() as &dyn AsComponents<$sty>),*] as &[&dyn AsComponents<$sty>]
    };
}
