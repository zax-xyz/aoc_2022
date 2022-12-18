use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::util::read_lines;

struct Dir {
    children: HashMap<String, Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
    size: u32,
    total_size: u32,
}

impl Dir {
    fn new(parent: Option<Rc<RefCell<Dir>>>) -> Dir {
        Dir {
            children: HashMap::new(),
            parent: if let Some(dir) = parent {
                Some(Rc::clone(&dir))
            } else {
                None
            },
            size: 0,
            total_size: 0,
        }
    }

    fn add_subdir(&mut self, name: &str, parent: &Rc<RefCell<Dir>>) {
        if self.children.contains_key(name) {
            return;
        }

        self.children.insert(
            name.to_string(),
            Rc::new(RefCell::new(Dir::new(Some(Rc::clone(&parent))))),
        );
    }

    fn calculate_total_sizes(&mut self) {
        fn helper(tree: &Rc<RefCell<Dir>>) -> u32 {
            tree.borrow_mut().calculate_total_sizes();
            tree.borrow().total_size
        }

        self.total_size = self.size + self.children.values().map(helper).sum::<u32>();
    }

    fn wtf(&self) -> u32 {
        fn helper(tree: &Dir) -> u32 {
            let subtree_size = tree.total_size;
            (if subtree_size <= 100000 {
                subtree_size
            } else {
                0
            }) + tree
                .children
                .values()
                .map(|child| helper(&child.borrow()))
                .sum::<u32>()
        }

        helper(self)
    }

    fn print_tree(&self) {
        println!("- / (size={})", self.total_size);
        fn helper(tree: &Dir, indent: u8) {
            for (name, child_ref) in &tree.children {
                let child = child_ref.borrow();
                println!(
                    "{}- {} (size={})",
                    "  ".repeat(indent.into()),
                    name,
                    child.total_size
                );
                helper(&child, indent + 1);
            }
        }

        helper(self, 1);
    }
}

fn parse_into_tree() -> Rc<RefCell<Dir>> {
    let lines = read_lines("inputs/day07.txt").skip(1);
    let root = Rc::new(RefCell::new(Dir::new(None)));
    let mut cursor = Rc::clone(&root);

    lines.for_each(|line| {
        let mut parts = line.split(" ");
        match parts.next().unwrap() {
            "$" => match parts.next().unwrap() {
                "cd" => match parts.next().unwrap() {
                    ".." => {
                        cursor = Rc::clone(Rc::clone(&cursor).borrow().parent.as_ref().unwrap());
                    }
                    dir_name => {
                        cursor.borrow_mut().add_subdir(dir_name, &cursor);
                        cursor =
                            Rc::clone(Rc::clone(&cursor).borrow().children.get(dir_name).unwrap());
                    }
                },
                "ls" => {}
                _ => panic!("??????????????"),
            },
            "dir" => cursor
                .borrow_mut()
                .add_subdir(parts.next().unwrap(), &cursor),
            size => cursor.borrow_mut().size += str::parse::<u32>(size).unwrap(),
        }
    });

    root
}

pub fn part1() -> u32 {
    let root = parse_into_tree();

    root.borrow_mut().calculate_total_sizes();
    Rc::clone(&root).borrow().wtf()
}

pub fn part2() -> u32 {
    let root = parse_into_tree();
    root.borrow_mut().calculate_total_sizes();

    let free_space = 70000000 - root.borrow().total_size;
    let required = 30000000 - free_space;

    fn helper(tree: &Dir, required: u32, mut lowest: u32) -> u32 {
        if tree.total_size >= required && tree.total_size < lowest {
            lowest = tree.total_size;
        }

        tree.children.values().for_each(|c| {
            lowest = helper(&c.borrow(), required, lowest);
        });

        lowest
    }

    helper(&Rc::clone(&root).borrow(), required, u32::MAX)
}
