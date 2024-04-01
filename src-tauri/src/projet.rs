use std::collections::HashMap;

use chrono::Datelike;
use comemo::Prehashed;
use typst::{
    diag::FileResult,
    foundations::{Bytes, Datetime},
    model::Document,
    text::{Font, FontBook},
    Library, World,
};
use typst_syntax::{FileId, Source, VirtualPath};

use crate::fonts::{FontSearcher, FontSlot};

/// Object contenant tout les donnée du projet.
///
pub struct Projet {
    /// Fichier représentant le main.
    main_id: FileId,
    /// Fichier du projet.
    /// FileId correspond a un chemin virtuel. cad un chemin relative au root de notre projet (toujour virtuel)
    files: HashMap<FileId, Fichier>,
    /// Lazy loaded font
    fonts: Vec<FontSlot>,
    /// Typst's standard library.
    library: Prehashed<Library>,
    /// Metadata about discovered fonts.
    book: Prehashed<FontBook>,
}

impl Default for Projet {
    fn default() -> Self {
        let mut searcher = FontSearcher::new();
        let library = Library::builder().build();
        searcher.search();
        let mut files: HashMap<FileId, Fichier> = HashMap::new();
        let main_id = FileId::new(None, VirtualPath::new("Foo.typ"));

        files.insert(main_id, Fichier::from_content(main_id, "".as_bytes()));
        Self {
            main_id,
            files,
            library: Prehashed::new(library),
            book: Prehashed::new(searcher.book),
            fonts: searcher.fonts,
        }
    }
}

impl Projet {
    pub fn add_file(&mut self, id: FileId, content: &[u8]) {
        self.files.insert(id, Fichier::from_content(id, content));
    }

    pub fn update_file(&mut self, id: FileId, content: &[u8]) {
        let c = self.files.get_mut(&id).unwrap(); // TODO better error handing
        c.update(content);
    }

    pub fn update_main<T>(&mut self, content: T)
    where
        T: AsRef<[u8]>,
    {
        self.update_file(self.main_id, content.as_ref());
    }
}

impl World for Projet {
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn main(&self) -> Source {
        let main = &self.files[&self.main_id];
        main.source()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        // Doit avoir tout les fichier necesaire dans le projet sinon crash.
        // TODO fair en sorte de rajouter les fichier neccesaire si pas present.
        // important surtout pour les package via @preview.
        let fichier = &self.files[&id];
        Ok(fichier.source())
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        // Doit avoir tout les fichier necesaire dans le projet sinon crash.
        // TODO fair en sorte de rajouter les fichier neccesaire si pas present.
        // important surtout pour les package via @preview.
        let fichier = &self.files[&id];
        Ok(fichier.bytes())
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts[index].get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = chrono::Local::now();

        let naive = match offset {
            None => now.naive_local(),
            Some(o) => now.naive_utc() + chrono::Duration::hours(o),
        };

        Datetime::from_ymd(
            naive.year(),
            naive.month().try_into().ok()?,
            naive.day().try_into().ok()?,
        )
    }
}

#[derive(Default)]
pub struct ProjetCache {
    pub document: Option<Document>,
}

struct Fichier {
    id: FileId,
    data: Bytes,
    _fingerprint: u128, // Pour plutard
}

impl Fichier {
    fn bytes(&self) -> Bytes {
        self.data.clone()
    }
    fn source(&self) -> Source {
        let text = decode_utf8(&self.data).expect("Coulnd decode");
        Source::new(self.id, text.into())
    }

    fn from_content(id: FileId, data: &[u8]) -> Fichier {
        Self {
            id,
            data: data.into(),
            _fingerprint: 0,
        }
    }

    fn update(&mut self, content: &[u8]) {
        self.data = content.into();
    }
}

/// Decode UTF-8 with an optional BOM.
fn decode_utf8(buf: &[u8]) -> FileResult<&str> {
    // Remove UTF-8 BOM.
    Ok(std::str::from_utf8(
        buf.strip_prefix(b"\xef\xbb\xbf").unwrap_or(buf),
    )?)
}
