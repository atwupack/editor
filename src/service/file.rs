use crate::service::{Service, ServiceFactory};
use std::collections::HashMap;
use std::fs::{read_dir};
use std::path::{PathBuf, Path};
use dunce::canonicalize;
use crate::app::App;


#[derive(Clone, Debug)]
pub struct FileItem {
    path: PathBuf,
    children: Option<Vec<PathBuf>>,
}

impl FileItem {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        FileItem {
            path: canonicalize(path).unwrap(),
            children: None,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

}

impl AsRef<Path> for FileItem {
    fn as_ref(&self) -> &Path {
        self.path.as_path()
    }
}

fn load_children<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let children = read_dir(path).unwrap();
    for entry in children {
        let entry = entry.unwrap();
        let path = entry.path();
        result.push(path);
    }
    result
}

pub struct FileService {
    cache: HashMap<PathBuf, FileItem>,
}

impl FileService {
    pub fn get_item<P: AsRef<Path>>(&mut self, path: P) -> FileItem {
        let key = canonicalize(&path).unwrap();
        let item = self.cache.remove(&key).unwrap_or_else( || {FileItem::new(&path)});
        let _old = self.cache.insert(item.path(), item.clone());
        item
    }

    fn update_item(&mut self, item: FileItem) {
        let key = item.path.canonicalize().unwrap();
        let _old = self.cache.insert(key, item);
    }

    pub fn get_children<P: AsRef<Path>>(&mut self, parent: P) -> Vec<FileItem> {
        let parent = self.get_item(&parent);
        let child_paths = parent.clone().children.unwrap_or_else(|| {
            let mut new_children = Vec::new();

            for child_path in load_children(&parent) {
                let new_child = self.get_item(&child_path);
                new_children.push(new_child.path());
            }

            let mut new_parent = parent.clone();
            new_parent.children = Some(new_children.clone());

            self.update_item(new_parent);

            new_children
        });

        let mut children = Vec::new();
        for child_path in child_paths {
            children.push(self.get_item(child_path));
        }
        children
    }
}

impl Service for FileService {
    fn new(_sf: &mut ServiceFactory) -> FileService {
        FileService {
            cache: HashMap::new(),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::service::file::{FileService, FileItem};
    use crate::service::Service;
    use std::path::Path;

    fn create_current_dir(fs: &mut FileService) -> FileItem {
        let path = Path::new(".");
        fs.get_item(path)
    }

    #[test]
    fn add_path() {
        let mut fs = FileService::new();

        let item = create_current_dir(&mut fs);

        assert!(item.is_dir());
        let inner_path = Path::new(item.path_str());
        assert!(inner_path.is_absolute());
    }

    #[test]
    fn get_children() {
        let mut fs = FileService::new();

        let item = create_current_dir(&mut fs);

        let children = fs.get_children(&item);
        assert!(children.len() > 0, "Children length {}", children.len());
    }
}
