use std::{
    fs::{create_dir_all, read_dir, File},
    path::{self, Path, PathBuf},
    rc::Rc,
};

use pathdiff::diff_paths;

use super::{Access, Component, Workspace};

#[derive(Debug)]
pub struct LocalWorkspace {
    root: PathBuf,
}

impl LocalWorkspace {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn absolute(&self, location: &[<Self as Workspace>::Component]) -> PathBuf {
        let mut path = self.root.clone();
        for component in location.iter() {
            path.push(component.clone().as_ref());
        }
        path
    }

    pub fn relative<P: AsRef<Path>>(&self, path: P) -> Rc<[<Self as Workspace>::Component]> {
        self.components(diff_paths(path, &self.root).unwrap())
    }

    pub fn components<P: AsRef<Path>>(&self, path: P) -> Rc<[<Self as Workspace>::Component]> {
        let mut components = Vec::new();
        for component in path.as_ref().components().filter_map(|c| match c {
            path::Component::Normal(s) => s.to_str(),
            _ => None,
        }) {
            components.push(component.into())
        }
        components.into()
    }
}

impl Component for Rc<str> {}

impl Workspace for LocalWorkspace {
    type Component = Rc<str>;
    type Source = File;

    fn get_source(&self, location: &[Self::Component], access: Access) -> Self::Source {
        match access {
            Access::Read => File::open::<PathBuf>(self.absolute(location)).unwrap(),
            Access::Write => File::create::<PathBuf>(self.absolute(location)).unwrap(),
        }
    }

    fn make_component(&self, location: &[Self::Component]) {
        create_dir_all(self.absolute(location)).unwrap();
    }

    fn insert_descendants(
        &self,
        descendants: &mut Vec<Rc<[Self::Component]>>,
        location: &[Self::Component],
        skip: usize,
    ) {
        let path: PathBuf = self.absolute(location.as_ref()).into();
        for (entry, is_dir) in read_dir(&path)
            .unwrap()
            .filter_map(Result::ok)
            .map(move |entry| (entry.path(), entry.metadata().unwrap().is_dir()))
        {
            let component: Rc<str> = entry.file_name().unwrap().to_str().unwrap().into();
            let mut vec = Vec::from(location);
            vec.push(component);
            if is_dir {
                self.insert_descendants(descendants, vec.as_ref(), skip);
            } else {
                descendants.push(vec.into_boxed_slice()[skip..].into());
            }
        }
    }
}
