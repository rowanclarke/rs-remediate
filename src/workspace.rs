pub mod fs;

use crate::{loc, loc_root, root};
use std::{
    io::{Read, Write},
    rc::Rc,
    slice::from_ref,
};

pub trait Component: std::fmt::Debug + Clone + for<'a> From<&'a str> + 'static {}

pub enum Access {
    Read,
    Write,
}

pub trait Workspace {
    type Component: Component;
    type Source: Read + Write;

    fn get_source(&self, location: &[Self::Component], access: Access) -> Self::Source;
    fn make_component(&self, location: &[Self::Component]);
    fn insert_descendants(
        &self,
        descendants: &mut Vec<Rc<[Self::Component]>>,
        location: &[Self::Component],
        skip: usize,
    );

    fn read<R: Root<Self::Component>, T: IntoComponents<Self::Component>>(
        &self,
        location: T,
    ) -> Rc<[u8]> {
        let mut vec = Vec::new();
        let location = loc_root!(R, [location] as Self::Component).into_components();
        self.get_source(location.as_components(), Access::Read)
            .read_to_end(&mut vec)
            .unwrap();
        vec.into()
    }

    fn write<R: Root<Self::Component>, T: IntoComponents<Self::Component>>(
        &self,
        location: T,
        bytes: &[u8],
    ) {
        let location = loc_root!(R, [location] as Self::Component).into_components();
        let components = location.as_components();
        self.make_component(&components[..components.len() - 1]);
        self.get_source(location.as_components(), Access::Write)
            .write_all(bytes)
            .unwrap();
    }

    fn descendants_from<R: Root<Self::Component>, T: IntoComponents<Self::Component>>(
        &self,
        location: T,
    ) -> Vec<Rc<[Self::Component]>> {
        let location = loc_root!(R, [location] as Self::Component).into_components();
        let components = location.as_components();
        let mut descendants = Vec::new();
        self.insert_descendants(&mut descendants, components, components.len());
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

impl<C: Component> IntoComponents<C> for () {
    type Output = Rc<[C]>;

    fn into_components(self) -> Self::Output {
        Rc::from([])
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

pub trait Root<C: Component> {
    type Output: AsComponents<C>;

    fn get_root() -> Self::Output;
}

root!(pub type WorkspaceRoot = []);

#[macro_export]
macro_rules! loc {
    ([$($x:expr),*] as $sty:ty) => {
        &[$(&<_ as IntoComponents<$sty>>::into_components($x).as_components() as &dyn AsComponents<$sty>),*] as &[&dyn AsComponents<$sty>]
    };
}

#[macro_export]
macro_rules! root {
    ($v:vis type $i:ident = [$($x:expr),*]) => {
        $v struct $i;

        impl<C: Component> Root<C> for $i {
            type Output = Rc<[C]>;

            fn get_root() -> Self::Output {
                loc!([$($x),*] as C).into_components()
            }
        }
    };
    ($v:vis type $i:ident: $j:ident = [$($x:expr),*]) => {
        $v struct $i;

        impl<C: Component> Root<C> for $i {
            type Output = Rc<[C]>;

            fn get_root() -> Self::Output {
                loc_root!($j, [$($x),*] as C).into_components()
            }
        }
    };
}

#[macro_export]
macro_rules! loc_root {
    ($i:ident, [$($x:expr),*] as $sty:ty) => {
        loc!([<_ as AsComponents<$sty>>::as_components(&$i::get_root()), $($x),*] as $sty)
    };
}
