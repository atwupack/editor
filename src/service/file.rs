use std::path::PathBuf;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::read_dir;
use crate::service::Service;

#[derive(Clone)]
pub struct FileItem {
    index: u32,
    path: PathBuf,
    children_read: bool,
    children: Vec<u32>,
}

impl FileItem {
    fn load_children(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let children = read_dir(self.path.clone()).unwrap();
        for entry in children {
            let entry = entry.unwrap();
            let path= entry.path();
            result.push(path);
        }
        result
    }
}

#[derive(Clone)]
struct FileCache {
    data: HashMap<u32, FileItem>,
    next_index : u32,
}


impl FileCache {
    fn inc_index(&mut self) -> u32 {
        self.next_index+=1;
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