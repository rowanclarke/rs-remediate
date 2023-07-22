use std::{
    fs::{read_dir, File},
    path::{self, Path, PathBuf},
    rc::Rc,
    str::FromStr,
};

use pathdiff::diff_paths;

use super::{Component, IntoComponents, Workspace};

#[derive(Debug)]
pub struct LocalWorkspace {
    root: Rc<str>,
}

/*#[derive(Clone)]
pub struct Path(Vec<Rc<str>>);

impl Relative for Path {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn from_str(str: &str) -> Self {
        Self(vec![str.into()])
    }

    fn push(&mut self, other: &Self) {
        self.0.extend_from_slice(other.0.as_slice())
    }
}*/

impl LocalWorkspace {
    pub fn new(root: Rc<str>) -> Self {
        Self { root }
    }

    fn absolute(&self, location: &[<Self as Workspace>::Component]) -> PathBuf {
        let mut path = PathBuf::from_str(self.root.as_ref()).unwrap();
        for component in location.iter() {
            path.push(component.clone().as_ref());
        }
        path
    }

    pub fn relative<P: AsRef<Path>>(&self, path: P) -> Rc<[<Self as Workspace>::Component]> {
        let mut components = Vec::new();
        for component in diff_paths(path, self.root.as_ref())
            .unwrap()
            .components()
            .filter_map(|c| match c {
                path::Component::Normal(s) => s.to_str(),
                _ => None,
            })
        {
            components.push(component.into());
        }
        components.into()
    }
}

/*impl From<Path> for PathBuf {
    fn from(value: Path) -> Self {
        let path = Self::new();
        for component in value.0 {
            path.push(component.as_ref());
        }
        path
    }
}*/

impl Component for Rc<str> {}

impl Workspace for LocalWorkspace {
    type Component = Rc<str>;
    type Source = File;

    fn open(&self, location: &[Self::Component]) -> Self::Source {
        File::open::<PathBuf>(self.absolute(location)).unwrap()
    }

    fn create(&self, location: &[Self::Component]) -> Self::Source {
        File::create::<PathBuf>(self.absolute(location)).unwrap()
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
