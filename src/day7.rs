use std::cell::RefCell;
use std::rc::Rc;
use std::{fs::read_to_string, io::Error, path::Path};
use builder::{Built, Builder};

#[derive(Debug)]
struct ChangeDirectory {
    name: String,
}

impl Built for ChangeDirectory {
    type BuilderType = ChangeDirectoryBuilder;

    fn builder() -> ChangeDirectoryBuilder {
        ChangeDirectoryBuilder::new()
    }
}

#[derive(Debug)]
struct ChangeDirectoryBuilder {
    name: Option<String>,
}

impl ChangeDirectoryBuilder {
    fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
}

impl Builder for ChangeDirectoryBuilder {
    type BuiltType = ChangeDirectory;

    fn new() -> ChangeDirectoryBuilder {
        ChangeDirectoryBuilder { name: None }
    }

    fn build(self) -> Option<ChangeDirectory> {
        if let Some(name) = self.name {
            Some(ChangeDirectory {
                name: name.to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ListDirectory {
    directory_items: Vec<DirectoryItem>,
}

impl Built for ListDirectory {
    type BuilderType = ListDirectoryBuilder;

    fn builder() -> ListDirectoryBuilder {
        ListDirectoryBuilder {
            directory_items: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct ListDirectoryBuilder {
    directory_items: Vec<Option<DirectoryItem>>,
}

impl Builder for ListDirectoryBuilder {
    type BuiltType = ListDirectory;

    fn new() -> ListDirectoryBuilder {
        ListDirectoryBuilder {
            directory_items: Vec::new(),
        }
    }

    fn build(self) -> Option<ListDirectory> {
        if self.directory_items.iter().any(|item| item.is_none()) {
            return None;
        }

        Some(ListDirectory {
            directory_items: self
                .directory_items
                .into_iter()
                .filter_map(|item| item)
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    directory_items: Vec<DirectoryItem>,
    size: Option<u32>,
}

impl Directory {
    fn child_directory(&mut self, name: &str) -> Option<Rc<RefCell<Directory>>> {
        self.directory_items.iter_mut().find_map(|item| match item {
            DirectoryItem::Directory(directory) => {
                if directory.borrow().name == name {
                    Some(Rc::clone(directory))
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct File {
    name: String,
    size: u32,
    extension: Option<String>,
}

#[derive(Debug)]
enum DirectoryItem {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

#[derive(Debug)]
struct FileBuilder {
    name: Option<String>,
    size: Option<u32>,
    extension: Option<String>,
}

impl Built for File {
    type BuilderType = FileBuilder;

    fn builder() -> Self::BuilderType {
        Self::BuilderType::new()
    }
}

impl FileBuilder {
    fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    fn size(&mut self, size: u32) -> &mut Self {
        self.size = Some(size);
        self
    }

    fn extension(&mut self, extension: Option<String>) -> &mut Self {
        self.extension = extension;
        self
    }
}

impl Builder for FileBuilder {
    type BuiltType = File;

    fn new() -> Self {
        FileBuilder {
            name: None,
            size: None,
            extension: None,
        }
    }

    fn build(self) -> Option<Self::BuiltType> {
        if let (Some(name), Some(size)) = (self.name, self.size) {
            Some(File {
                name,
                size,
                extension: self.extension,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum TerminalCommand {
    ChangeDirectory(ChangeDirectory),
    ListDirectory(ListDirectory),
    Invalid(String),
}

#[derive(Debug)]
enum TerminalCommandBuilder {
    ChangeDirectoryBuilder(ChangeDirectoryBuilder),
    ListDirectoryBuilder(ListDirectoryBuilder),
}

fn read_terminal(path: &Path) -> Result<Vec<String>, Error> {
    let lines = read_to_string(path)?;

    let owned_lines = lines.lines().map(|line| line.to_owned()).collect();

    Ok(owned_lines)
}

fn lex_terminal_lines(lines: Vec<String>) -> Vec<TerminalCommand> {
    let mut terminal_commands: Vec<TerminalCommand> = Vec::new();
    let mut current_command_builder: Option<TerminalCommandBuilder> = None;

    for line in lines.iter() {
        let mut tokens_iter = line.split(" ");

        let mut token = match tokens_iter.next() {
            Some(token) => token,
            None => {
                continue;
            }
        };

        if token == "$" {
            if let Some(builder) = current_command_builder {
                match builder {
                    TerminalCommandBuilder::ChangeDirectoryBuilder(builder) => {
                        match builder.build() {
                            Some(command) => {
                                terminal_commands.push(TerminalCommand::ChangeDirectory(command))
                            }
                            None => terminal_commands
                                .push(TerminalCommand::Invalid(format!("{}", line!()).to_string())),
                        }
                    }
                    TerminalCommandBuilder::ListDirectoryBuilder(builder) => {
                        match builder.build() {
                            Some(command) => {
                                terminal_commands.push(TerminalCommand::ListDirectory(command))
                            }
                            None => terminal_commands
                                .push(TerminalCommand::Invalid(format!("{}", line!()).to_string())),
                        }
                    }
                }
            }

            current_command_builder = None;

            token = match tokens_iter.next() {
                Some(token) => token,
                None => {
                    continue;
                }
            };
        }

        match token {
            "ls" => {
                current_command_builder = Some(TerminalCommandBuilder::ListDirectoryBuilder(
                    ListDirectory::builder(),
                ));
            }
            "cd" => {
                current_command_builder = Some(TerminalCommandBuilder::ChangeDirectoryBuilder(
                    ChangeDirectory::builder(),
                ));

                let name = tokens_iter.next();

                match name {
                    None => {
                        current_command_builder = None;
                        terminal_commands
                            .push(TerminalCommand::Invalid(format!("{}", line!()).to_string()));
                    }
                    Some(name) => {
                        if let Some(builder) = current_command_builder {
                            if let TerminalCommandBuilder::ChangeDirectoryBuilder(mut builder) =
                                builder
                            {
                                builder.name(name.to_string());
                                match builder.build() {
                                    Some(command) => {
                                        terminal_commands
                                            .push(TerminalCommand::ChangeDirectory(command));
                                    }
                                    None => {
                                        terminal_commands.push(TerminalCommand::Invalid(
                                            format!("{}", line!()).to_string(),
                                        ));
                                    }
                                }
                            }
                            current_command_builder = None;
                        }
                    }
                }
            }
            "dir" => {
                let name = tokens_iter.next();
                match name {
                    None => {
                        current_command_builder = if let Some(builder) = current_command_builder {
                            Some(
                                if let TerminalCommandBuilder::ListDirectoryBuilder(mut builder) =
                                    builder
                                {
                                    builder.directory_items.push(None);
                                    TerminalCommandBuilder::ListDirectoryBuilder(builder)
                                } else {
                                    builder
                                },
                            )
                        } else {
                            current_command_builder
                        }
                    }
                    Some(name) => {
                        current_command_builder = if let Some(builder) = current_command_builder {
                            Some(
                                if let TerminalCommandBuilder::ListDirectoryBuilder(mut builder) =
                                    builder
                                {
                                    builder.directory_items.push(Some(DirectoryItem::Directory(
                                        Rc::new(RefCell::new(Directory {
                                            name: name.to_string(),
                                            directory_items: Vec::new(),
                                            size: None,
                                        })),
                                    )));
                                    TerminalCommandBuilder::ListDirectoryBuilder(builder)
                                } else {
                                    builder
                                },
                            )
                        } else {
                            current_command_builder
                        }
                    }
                }
            }
            token if token.chars().all(char::is_numeric) => {
                let size = token.parse::<u32>().unwrap();

                let mut file_builder = File::builder();
                file_builder.size(size);

                let name_token = tokens_iter.next();

                match name_token {
                    None => {
                        current_command_builder = None;
                        terminal_commands
                            .push(TerminalCommand::Invalid(format!("{}", line!()).to_string()));
                    }
                    Some(name_token) => {
                        let mut name_tokens = name_token.split(".");

                        if let Some(name) = name_tokens.next() {
                            file_builder.name(name.to_string());
                            let extension = match name_tokens.next() {
                                Some(extension) => Some(extension.to_string()),
                                None => None,
                            };
                            file_builder.extension(extension);
                            let file = file_builder.build();
                            if let Some(file) = file {
                                current_command_builder = if let Some(builder) =
                                    current_command_builder
                                {
                                    Some(
                                        if let TerminalCommandBuilder::ListDirectoryBuilder(
                                            mut builder,
                                        ) = builder
                                        {
                                            builder
                                                .directory_items
                                                .push(Some(DirectoryItem::File(file)));
                                            TerminalCommandBuilder::ListDirectoryBuilder(builder)
                                        } else {
                                            builder
                                        },
                                    )
                                } else {
                                    current_command_builder
                                }
                            }
                        } else {
                            current_command_builder = None;
                            terminal_commands
                                .push(TerminalCommand::Invalid(format!("{}", line!()).to_string()));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(builder) = current_command_builder {
        match builder {
            TerminalCommandBuilder::ChangeDirectoryBuilder(builder) => match builder.build() {
                Some(command) => {
                    terminal_commands.push(TerminalCommand::ChangeDirectory(command));
                }
                None => {
                    terminal_commands
                        .push(TerminalCommand::Invalid(format!("{}", line!()).to_string()));
                }
            },
            TerminalCommandBuilder::ListDirectoryBuilder(builder) => match builder.build() {
                Some(command) => {
                    terminal_commands.push(TerminalCommand::ListDirectory(command));
                }
                None => {
                    terminal_commands
                        .push(TerminalCommand::Invalid(format!("{}", line!()).to_string()));
                }
            },
        }
    }

    terminal_commands
}

fn get_directories_from_commands(commands: &mut Vec<TerminalCommand>) -> Rc<RefCell<Directory>> {
    let root_directory = Rc::new(RefCell::new(Directory {
        name: "/".to_string(),
        directory_items: Vec::new(),
        size: None,
    }));

    let mut directory_stack = Vec::<Rc<RefCell<Directory>>>::new();
    directory_stack.push(Rc::clone(&Rc::clone(&root_directory)));

    for command in commands.iter_mut() {
        match command {
            TerminalCommand::ChangeDirectory(change_directory) => {
                match change_directory.name.as_str() {
                    "/" => {
                        while directory_stack.len() > 1 {
                            directory_stack.pop();
                        }
                    }
                    ".." => {
                        if directory_stack.len() > 1 {
                            directory_stack.pop();
                        }
                    }
                    _ => {
                        let child_directory = directory_stack
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .child_directory(change_directory.name.as_str());
                        match child_directory {
                            Some(directory) => {
                                directory_stack.push(directory);
                            }
                            None => {
                                let new_directory = Rc::new(RefCell::new(Directory {
                                    name: change_directory.name.to_owned(),
                                    directory_items: Vec::new(),
                                    size: None,
                                }));
                                directory_stack.push(new_directory);
                            }
                        }
                    }
                }
            }
            TerminalCommand::ListDirectory(list_directory) => {
                let directory_items =
                    &mut directory_stack.last().unwrap().borrow_mut().directory_items;
                let list_directory_items = &mut list_directory.directory_items;

                while directory_items.len() > 0 {
                    directory_items.pop();
                }
                while list_directory_items.len() > 0 {
                    directory_items.push(list_directory_items.pop().unwrap());
                }
            }
            TerminalCommand::Invalid(_) => {}
        }
    }

    root_directory
}

fn calc_directory_sizes(directories: Rc<RefCell<Directory>>) {
    let size = calc_directory_size(Rc::clone(&directories));
    directories.borrow_mut().size = Some(size);
}

fn calc_directory_size(directories: Rc<RefCell<Directory>>) -> u32 {
    let mut size: u32 = 0;

    for directory_item in directories.borrow().directory_items.iter() {
        size += match directory_item {
            DirectoryItem::Directory(directory) => {
                let dir_size = calc_directory_size(Rc::clone(directory));
                directory.borrow_mut().size = Some(dir_size);
                dir_size
            }
            DirectoryItem::File(file) => file.size,
        }
    }

    size
}

fn get_directories_under_size(
    directories: Rc<RefCell<Directory>>,
    size: u32,
) -> Vec<Rc<RefCell<Directory>>> {
    let mut collected_directories = Vec::<Rc<RefCell<Directory>>>::new();

    if let Some(dir_size) = directories.borrow().size {
        if dir_size <= size {
            collected_directories.push(Rc::clone(&directories));
        }
    }

    for directory in directories.borrow().directory_items.iter() {
        match directory {
            DirectoryItem::Directory(directory) => {
                let mut child_directories = get_directories_under_size(Rc::clone(&directory), size);

                collected_directories.append(&mut child_directories);
            }
            _ => (),
        }
    }

    collected_directories
}

pub fn day7() {
    let lines = read_terminal(Path::new("terminal-output.txt")).unwrap();

    let mut commands = lex_terminal_lines(lines);

    let directories = get_directories_from_commands(&mut commands);

    calc_directory_sizes(Rc::clone(&directories));

    let collected_directories = get_directories_under_size(Rc::clone(&directories), 100_000);

    let sum = collected_directories.iter().fold(0, |mut sum, directory| {
        if let Some(size) = directory.borrow().size {
            sum += size;
        }

        sum
    });

    println!("{sum}");
}
