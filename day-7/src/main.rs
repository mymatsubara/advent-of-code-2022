use std::{fs, time::Instant, cell::RefCell, rc::Rc, fmt::{self, Formatter, Error}};

fn part_one(input: &[String]) -> String {
    let fs = parse_fs(input);
    const MAX_SIZE: usize = 100000;

    let mut solution = 0usize;
    fs.root.borrow().foreach_child(&mut |child| {
        let size = child.size();
        if size < MAX_SIZE { solution += size }
    });

    solution.to_string()
}

fn part_two(input: &[String]) -> String {
    let fs = parse_fs(input);
    let unused_space_required = 40000000;

    let total_size = fs.root.borrow().size();
    let min_size = total_size - unused_space_required;
     
    let mut solution = total_size;
    let mut sizes = Vec::new();
    fs.root.borrow().foreach_child(&mut |child| {
        let size = child.size();
        sizes.push(size);
        if size > min_size && size < solution {
            solution = size;
        }
    });

    solution.to_string()
}

fn parse_fs(input: &[String]) -> Filesystem {
    let mut fs = Filesystem::new();

    for line in input {
        if let Some(command) = Command::from(line) {
            match command {
                Command::Cd(dir_name) => { fs.cd(&dir_name) },
                Command::Ls => ()
            }
        } else {
            match parse_ls_line(line) {
                LsResult::File(file) => fs.new_file(file),
                LsResult::DirName(dir_name) => { fs.new_directory(&dir_name); }
            }
        }
    }

    fs
}

#[derive(Debug)]
struct Filesystem {
    root: DirectoryRef,
    cur_dir: DirectoryRef
}

impl Filesystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Directory::new("/".to_owned(), None)));
        Filesystem {
            cur_dir: root.clone(),
            root,
        }
    }

    fn cd(&mut self, dest: &str) {
        let new_dir = match dest {
            ".." =>  self.cur_dir.borrow().parent.clone(),
            _ => self.cur_dir.borrow().dirs.iter().find(|dir| dir.borrow().name == dest).cloned()
        };

        if let Some(dir) = new_dir {self.cur_dir = dir }
    }

    fn new_file(&self, file: File) {
        self.cur_dir.borrow_mut().files.push(file);
    }

    fn new_directory(&mut self, name: &str) {
        let new_dir = Rc::new(RefCell::new(Directory::new(name.to_owned(), Some(self.cur_dir.clone()))));
        self.cur_dir.borrow_mut().dirs.push(new_dir);
    }
}

enum LsResult {
    File(File),
    DirName(String),
}

fn parse_ls_line(line: &str) -> LsResult {
    let Some((first, name)) = line.split_once(' ') else { 
        panic!("Invalid ls result line: {line}")
    };

    match first {
        "dir" => LsResult::DirName(name.to_owned()),
        size => LsResult::File(File {
            name: name.to_owned(),
            size: size
                .parse::<usize>()
                .expect("File should be preceded by size"),
        }),
    }
}

enum Command {
    Ls,
    Cd(String),
}

impl Command {
    fn from(line: &str) -> Option<Command> {
        if !line.starts_with('$') {
            return None;
        }

        let mut args = line.split_terminator(&[' ', '$'][..]).filter(|str| !str.is_empty());
        let Some(cmd) = args.next() else {
            return None;
        };

        match cmd {
            "ls" => Some(Command::Ls),
            "cd" => args.next().map(|dir| Command::Cd(dir.to_owned())),
            _ => None
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct File {
    name: String,
    size: usize,
}

impl Sizeable for File {
    fn size(&self) -> usize {
        self.size
    }
}

type DirectoryRef = Rc<RefCell<Directory>>;

struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<DirectoryRef>,
    parent: Option<DirectoryRef>
}

impl Sizeable for Directory {
    fn size(&self) -> usize {
        self.files.iter().map(|file| file.size()).sum::<usize>()
            + self.dirs.iter().map(|dir| dir.borrow().size()).sum::<usize>()
    }
}

impl Directory {
    fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Directory {
            name,
            parent,
            files: vec![],
            dirs: vec![],
        }
    }

    fn foreach_child<F: FnMut(&Directory)>(&self, f: &mut F) {
        for child in &self.dirs {
            f(&child.borrow());
            child.borrow().foreach_child(f);
        }
    }
}

impl fmt::Debug for Directory {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.debug_struct("Directory")
            .field("name", &self.name)
            .field("files", &self.files)
            .field("dirs", &self.dirs);

        Ok(())
    }
}

trait Sizeable {
    fn size(&self) -> usize;
}

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input);
        assert_eq!(result, "95437");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "24933642");
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input(false);

    let start_one = Instant::now();
    let result_one = part_one(&input);
    let elapsed_one = start_one.elapsed();

    let start_two = Instant::now();
    let result_two = part_two(&input);
    let elapsed_two = start_two.elapsed();

    println!("Part one result: {result_one} [time: {:.2?}]", elapsed_one);
    println!("Part two result: {result_two} [time: {:.2?}]", elapsed_two);
}

fn parse_input(test: bool) -> Vec<String> {
    let file = if test { "input.test.txt" } else { "input.txt" };

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("'{file}' not found"))
        .lines()
        .map(|line| line.to_owned())
        .collect()
}
