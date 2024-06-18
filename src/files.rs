use std::{
    fs::{self, read_dir, read_to_string},
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
            } else {
                dirs.push(SiuFileInfo::new(entry.path(), meta));
            }
        }

        file.sort_by_key(|f| f.name.to_lowercase());
        dirs.sort_by_key(|f| f.name.to_lowercase());
        dirs.append(&mut file);

        Ok(Self { path: p, dirs, content: None })
    }

    pub fn read_dir<P: AsRef<Path>>(&mut self, p: P) -> std::io::Result<()> {
        let path = p.as_ref().to_owned();
        let content = fs::read_to_string(&path)?;
        self.content = Some(content);
        self.dirs.clear();
        self.path = path;
        Ok(())
    }

    pub fn print(&self) {
        self.dirs.iter().for_each(|f| println!("{}", f.name));
    }
}
