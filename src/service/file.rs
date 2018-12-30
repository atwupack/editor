use crate::service::Service;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct FileItem {
    index: u32,
    path: PathBuf,
    children_read: bool,
    children: Vec<u32>,
}

impl FileItem {

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn index(&self) -> u32 {
       self.index
    }

    pub fn name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    pub fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    fn load_children(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let children = read_dir(self.path.clone()).unwrap();
        for entry in children {
            let entry = entry.unwrap();
            let path = entry.path();
            result.push(path);
        }
        result
    }
}

#[derive(Clone)]
struct FileCache {
    data: HashMap<u32, FileItem>,
    next_index: u32,
}

impl FileCache {
    fn inc_index(&mut self) -> u32 {
        self.next_index += 1;
        self.next_index
    }

    fn add_item(&mut self, path: &PathBuf) -> u32 {
        let index = self.inc_index();
        let item = FileItem {
            index,
            path: path.clone(),
            children_read: false,
            children: Vec::new(),
        };
        let _old = self.data.insert(index, item);
        index
    }

    fn get_item(&self, index: u32) -> &FileItem {
        self.data.get(&index).unwrap()
    }

    fn update_item(&mut self, item: FileItem) {
        let _old = self.data.insert(item.index, item);
    }
}

#[derive(Clone)]
pub struct FileService {
    cache: Rc<RefCell<FileCache>>,
}

impl FileService {
    pub fn get_item(&self, index: u32) -> FileItem {
        let cache = self.cache.borrow();
        cache.get_item(index).clone()
    }

    pub fn add_item(&self, path: &PathBuf) -> u32 {
        let mut cache = self.cache.borrow_mut();
        cache.add_item(path)
    }

    pub fn get_children(&self, index: u32) -> Vec<FileItem> {
        let parent = self.get_item(index);
        if !parent.children_read {
            let mut new_children = Vec::new();
            for child_path in parent.load_children() {
                let new_index = self.add_item(&child_path);
                new_children.push(new_index);
            }
            let mut new_parent = parent.clone();
            new_parent.children = new_children;
            new_parent.children_read = true;

            let mut cache = self.cache.borrow_mut();
            cache.update_item(new_parent);
        }
        let parent = self.get_item(index);
        let mut children = Vec::new();
        for child_index in parent.children {
            children.push(self.get_item(child_index));
        }
        children
    }
}

impl Service for FileService {
    fn new() -> FileService {
        let cache = FileCache {
            data: HashMap::new(),
            next_index: 0,
        };

        FileService {
            cache: Rc::new(RefCell::new(cache)),
        }
    }

    fn id() -> &'static str {
        "file-service"
    }

}

#[cfg(test)]
mod tests {
}
