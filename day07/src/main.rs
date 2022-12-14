use regex::Regex;
use std::iter::Sum;
use std::ops::{Add, Sub};

#[derive(Eq, PartialEq, Clone, Copy, Debug, Ord, PartialOrd)]
struct FileSize(usize);
impl Add for FileSize {
    type Output = FileSize;

    fn add(self, rhs: Self) -> Self::Output {
        FileSize(self.0 + rhs.0)
    }
}

impl Sub for FileSize {
    type Output = FileSize;

    fn sub(self, rhs: Self) -> Self::Output {
        FileSize(self.0 - rhs.0)
    }
}

impl Sum for FileSize {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FileSize(0), |a, b| a + b)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct ID(usize);

#[derive(Debug)]
struct Folder {
    name: String,
    entries: Vec<ID>,
    size: FileSize,
}
#[derive(Debug)]
struct File {
    name: String,
    size: FileSize,
}
#[derive(Debug)]
enum NodeEntry {
    File(File),
    Folder(Folder),
}
#[derive(Debug)]
struct Node {
    id: ID,
    parent: Option<ID>,
    value: NodeEntry,
}

#[derive(Debug)]
struct FileSystem {
    contents: Vec<Node>,
    active_folder: ID,
}

impl FileSystem {
    fn new() -> Self {
        let id = ID(0);
        Self {
            contents: vec![Node {
                id,
                parent: None,
                value: NodeEntry::Folder(Folder {
                    name: "".to_string(),
                    entries: vec![],
                    size: FileSize(0),
                }),
            }],
            active_folder: id,
        }
    }

    fn move_into_folder(&mut self, search_name: &str) {
        if search_name == ".." {
            self.active_folder = self
                .get_parent_folder(self.active_folder)
                .map(|n| n.id)
                .unwrap_or(ID(0));

            return;
        }
        if search_name == "/" {
            self.active_folder = ID(0);
            return;
        }
        let Some(node) = self.contents.get(self.active_folder.0) else { return };
        match &node.value {
            NodeEntry::File { .. } => {
                unreachable!("active folder is a file");
            }
            NodeEntry::Folder(Folder { entries, .. }) => {
                for id in entries {
                    let Some(node) = self.contents.get(id.0) else { continue; };
                    match &node.value {
                        NodeEntry::File { .. } => continue,
                        NodeEntry::Folder(Folder { name, .. }) => {
                            if name == search_name {
                                self.active_folder = id.to_owned();
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_folder(&mut self, name: &str) {
        self.add_node_entry(NodeEntry::Folder(Folder {
            entries: vec![],
            name: name.to_string(),
            size: FileSize(0),
        }));
    }

    fn add_file(&mut self, name: &str, size: FileSize) {
        let id = self.add_node_entry(NodeEntry::File(File {
            size,
            name: name.to_string(),
        }));
        let mut current_id = id;

        while let Some(next_node) = self.get_parent_folder(current_id) {
            current_id = next_node.id;
            if let NodeEntry::Folder(Folder {
                size: folder_size, ..
            }) = &mut next_node.value
            {
                *folder_size = size + *folder_size;
            }
        }
    }

    fn get_parent_folder(&mut self, parent_of: ID) -> Option<&mut Node> {
        let Some(parent_id) = self.contents[parent_of.0].parent else { return None };
        self.contents.get_mut(parent_id.0)
    }

    fn add_node_entry(&mut self, node_entry: NodeEntry) -> ID {
        let new_id = ID(self.contents.len());
        let Some(parent) = self.contents.get_mut(self.active_folder.0) else {
            panic!("current folder is not present")
        };
        let NodeEntry::Folder(Folder{ entries, ..}) = &mut parent.value else {
            panic!("current folder is no folder")
        };

        entries.push(new_id);

        let node = Node {
            id: new_id,
            parent: Some(self.active_folder),
            value: node_entry,
        };
        self.contents.push(node);
        new_id
    }

    fn print_inner(&self, ids: &Vec<ID>, indent: usize) {
        for id in ids {
            if let Some(active_node) = &self.contents.get(id.0) {
                match &active_node.value {
                    NodeEntry::File(File { name, size }) => {
                        println!("{:indent$}{}: {}", "", name, size.0, indent = indent);
                    }
                    NodeEntry::Folder(Folder {
                        name,
                        entries,
                        size,
                    }) => {
                        println!("{:indent$}{}/ ({})", "", name, size.0, indent = indent);
                        self.print_inner(entries, indent + 1)
                    }
                }
            };
        }
    }

    fn print(&self) {
        self.print_inner(&vec![ID(0)], 0);
    }

    fn execute_instructions(&mut self, input: &str) {
        let cd = Regex::new(r"\$ cd (.+)").unwrap();
        let dir = Regex::new(r"dir (.+)").unwrap();
        let file = Regex::new(r"(\d+) (.+)").unwrap();
        input.lines().for_each(|line| {
            if let Some(cd) = cd.captures(line) {
                self.move_into_folder(&cd[1])
            } else if let Some(dir) = dir.captures(line) {
                self.add_folder(&dir[1])
            } else if let Some(file) = file.captures(line) {
                self.add_file(&file[2], FileSize(*&file[1].parse::<usize>().unwrap()))
            }
        });
    }

    fn sum_folders_with_size_less_than(&self, max_size: usize) -> FileSize {
        self.contents
            .iter()
            .flat_map(|n| {
                if let NodeEntry::Folder(Folder { size, .. }) = n.value {
                    Some(size)
                } else {
                    None
                }
            })
            .filter(|size| size.0 <= max_size)
            .sum()
    }

    fn get_smallest_directory_to_delete(
        &self,
        max_size: FileSize,
        needed_size: FileSize,
    ) -> &Folder {
        let NodeEntry::Folder(Folder{size, ..}) = self.contents[0].value else { panic!("root is a folder") };
        let additional_size = needed_size - (max_size - size);
        self.contents
            .iter()
            .flat_map(|n| {
                if let NodeEntry::Folder(ref folder) = n.value {
                    if folder.size > additional_size {
                        Some(folder)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .min_by(|f1, f2| f1.size.cmp(&f2.size))
            .expect("at least one folder")
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut system = FileSystem::new();

    system.execute_instructions(input);

    let sum = system.sum_folders_with_size_less_than(100000);

    println!(
        "The sum of folders with size less or equal to 100k is {}",
        sum.0
    );

    let folder = system.get_smallest_directory_to_delete(FileSize(70000000), FileSize(30000000));

    println!(
        "The smallest folder to delete is {} and will free up {}",
        folder.name, folder.size.0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_gen() {
        let mut system = FileSystem::new();
        system.add_file("foo", FileSize(10));
        system.add_folder("baz");
        system.add_folder("buzz");
        system.move_into_folder("buzz");
        system.add_folder("zubb");
        system.move_into_folder("zubb");
        system.add_file("innerfoo", FileSize(10));
        system.add_file("innerbar", FileSize(20));
        system.move_into_folder("..");
        system.add_file("bar", FileSize(20));
        system.move_into_folder("/");
        system.add_file("bary", FileSize(20));

        system.print()
    }

    #[test]
    fn test_online_input() {
        let mut system = FileSystem::new();

        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        system.execute_instructions(input);
        let size = system.sum_folders_with_size_less_than(100000);
        let folder =
            system.get_smallest_directory_to_delete(FileSize(70000000), FileSize(30000000));

        assert_eq!(size, FileSize(95437));
        assert_eq!(folder.name, "d");

        system.print()
    }
}
