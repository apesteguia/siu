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

        // Crear un buffer con tamaño máximo especificado
        let mut buffer = vec![0; 2000];

        // Leer el archivo en el buffer
        let bytes_read = file.read(&mut buffer)?;

        // Truncar el buffer para reflejar los bytes realmente leídos
        buffer.truncate(bytes_read);

        // Convertir los bytes leídos a una cadena de texto
        let content_string = String::from_utf8_lossy(&buffer).to_string();

        // Actualizar el contenido del struct
        self.content = Some(content_string);
        self.dirs.clear();
        self.path = path;
        Ok(())
    }

    pub fn print(&self) {
        self.dirs.iter().for_each(|f| println!("{}", f.name));
    }
}
