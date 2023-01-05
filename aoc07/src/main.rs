use std::collections::HashMap;

struct File {
    name: String,
    size: i32,
}
struct Directory {
    path: Path,
    files: Vec<File>,
    directories: Vec<String>,
}
impl Directory {
    fn new(path: Path) -> Directory {
        Directory {
            path: path,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
    fn size(&self, fs: &FileSystem) -> i32 {
        let fsize: i32 = self.files.iter().map(|f| f.size).sum();
        let dsize: i32 = self
            .directories
            .iter()
            .map(|d| {
                let mut p = self.path.clone();
                p.path.push(d.clone());
                fs.directories[&p.as_string()].size(fs)
            })
            .sum();
        fsize + dsize
    }
}
#[derive(Clone)]
struct Path {
    path: Vec<String>,
}
impl Path {
    fn new() -> Path {
        Path { path: Vec::new() }
    }
    fn as_string(&self) -> String {
        let mut s = String::from("/");
        s.push_str(self.path.join("/").as_str());
        s
    }
}
struct FileSystem {
    cwd: Path,
    directories: HashMap<String, Directory>,
}
impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            cwd: Path { path: Vec::new() },
            directories: HashMap::from([(String::from("/"), Directory::new(Path::new()))]),
        }
    }
    fn cd(&mut self, name: &str) {
        if name == "/" {
            self.cwd.path.clear();
        } else if name == ".." {
            self.cwd.path.pop();
        } else {
            self.cwd.path.push(String::from(name));
        }
    }
    fn add_file(&mut self, name: String, size: i32) {
        self.directories
            .get_mut(&self.cwd.as_string())
            .unwrap()
            .files
            .push(File {
                name: name,
                size: size,
            });
    }
    fn add_directory(&mut self, name: String) {
        let mut path = self.cwd.clone();
        path.path.push(name.clone());
        self.directories
            .insert(path.as_string(), Directory::new(path));
        self.directories
            .get_mut(&self.cwd.as_string())
            .unwrap()
            .directories
            .push(name);
    }
}

fn main() {
    let mut filesystem = FileSystem::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                filesystem.cd(tokens[2]);
            }
        } else {
            if tokens[0] == "dir" {
                filesystem.add_directory(tokens[1].to_string());
            } else {
                filesystem.add_file(tokens[1].to_string(), tokens[0].parse::<i32>().unwrap());
            }
        }
    }
    let mut total_size = 0;
    for dir in filesystem.directories.values() {
        let size = dir.size(&filesystem);
        if size < 100000 {
            total_size += size;
        }
    }
    println!("Part 1: {}", total_size);
}
