#[derive(Debug, Clone)]
pub struct MangaChapter {
    pub manga_title: String,
    pub url: String,
    pub chapter_title: String,
    pub num: f32
}

#[derive(Debug, PartialEq, Clone)]
pub struct CSVLine {
    pub url: String,
    pub last_chapter_num: f32
}

#[derive(Debug, Clone)]
pub struct LineChapter {
    pub line: CSVLine,
    pub chapter: MangaChapter
}
