#[derive(Debug)]
pub enum UriPart {
    Hier {
        path: HierPath,
        query: Option<String>,
    },
}

#[derive(Debug)]
pub enum HierPath {
    Net {
        authority: String,
        abs: Option<AbsPath>,
    },
    Abs(AbsPath),
}

#[derive(Debug)]
pub struct AbsPath {
    segments: Vec<String>,
}
