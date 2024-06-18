use chrono::{DateTime, Local};
use std::os::unix::fs::PermissionsExt;
use std::{
    fs::{self, read_dir},
    io::Read,
    path::{Path, PathBuf},
};

pub struct SiuFileInfo {
    pub name: String,
    pub path: PathBuf,
    pub meta: Option<fs::Metadata>,
    pub is_file: bool,
}

impl SiuFileInfo {
    pub fn new<P: AsRef<Path>>(path: P, meta: fs::Metadata) -> Self {
        let path = path.as_ref().to_owned();
        let is_file = path.is_file();

        let name = match path.file_name() {
            Some(p) => p.to_string_lossy().to_string(),
            None => String::from("NO DATA"),
        };

        Self {
            path,
            is_file,
            meta: Some(meta),
            name,
        }
    }
}

pub struct SiuDir {
    pub dirs: Vec<SiuFileInfo>,
    pub path: PathBuf,
    pub content: Option<String>,
}

impl SiuDir {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let entries = read_dir(&path)?;
        let p = path.as_ref().to_owned();
        let mut dirs = vec![];
        let mut file = vec![];

        for entry in entries {
            let entry = entry?;
            let meta = entry.metadata()?;
            if meta.is_file() {
                file.push(SiuFileInfo::new(entry.path(), meta));
            } else if meta.is_dir() {
                dirs.push(SiuFileInfo::new(entry.path(), meta));
            }
        }

        file.sort_by_key(|f| f.name.to_lowercase());
        dirs.sort_by_key(|f| f.name.to_lowercase());
        dirs.append(&mut file);

        Ok(Self {
            path: p,
            dirs,
            content: None,
        })
    }

    pub fn read_dir<P: AsRef<Path>>(&mut self, p: P) -> std::io::Result<()> {
        let path = p.as_ref().to_owned();
        let mut file = std::fs::File::open(&path)?;

        let mut buffer = vec![0; 2000];

        let bytes_read = file.read(&mut buffer)?;

        buffer.truncate(bytes_read);

        let content_string = String::from_utf8_lossy(&buffer).to_string();

        self.content = Some(content_string);
        self.dirs.clear();
        self.path = path;
        Ok(())
    }

    pub fn print(&self) {
        self.dirs.iter().for_each(|f| println!("{}", f.name));
    }
}

pub fn format_permissions(permissions: fs::Permissions, is_directory: bool) -> String {
    let mode = permissions.mode();

    let file_type_char = if is_directory { 'd' } else { '-' };

    let owner_read = if mode & 0o400 != 0 { 'r' } else { '-' };
    let owner_write = if mode & 0o200 != 0 { 'w' } else { '-' };
    let owner_execute = if mode & 0o100 != 0 { 'x' } else { '-' };

    let group_read = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_write = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_execute = if mode & 0o010 != 0 { 'x' } else { '-' };

    let other_read = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_write = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_execute = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type_char,
        owner_read,
        owner_write,
        owner_execute,
        group_read,
        group_write,
        group_execute,
        other_read,
        other_write,
        other_execute
    )
}

pub fn format_modified(modified: std::io::Result<std::time::SystemTime>) -> String {
    match modified {
        Ok(sys_time) => {
            let datetime: DateTime<Local> = sys_time.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => "N/A".to_string(),
    }
}
