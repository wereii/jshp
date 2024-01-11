use std::ffi::OsStr;

#[derive(Debug, Clone)]
pub enum MimeType {
    Jshp,
    Html,
    Css,
    Txt,
    Xml,

    Jpeg,
    Gif,
    Webp,
    Png,
    Svg,
    Icon,

    OctetStream,
    Js,
    Json,
}

impl MimeType {
    pub fn to_content_type_string(&self) -> String {
        match self {
            // TODO change to text/html when js executing is done.
            // Wont print now since the syntax is incorrect.
            Self::Jshp => "text/plain",
            Self::Html => "text/html",
            Self::Css => "text/css",
            Self::Txt => "text/plain",
            Self::Xml => "text/xml",

            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
            Self::Png => "image/png",
            Self::Svg => "image/svg+xml",
            Self::Icon => "image/x-icon",

            Self::OctetStream => "application/octet-stream",
            Self::Js => "application/javascript",
            Self::Json => "application/json",
        }
        .to_string()
    }
}

impl From<&str> for MimeType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "jshp" => Self::Jshp,
            "html" | "htmls" => Self::Html,
            "css" => Self::Css,
            "txt" => Self::Txt,
            "xml" => Self::Xml,

            "gif" => Self::Gif,
            "jpeg" => Self::Jpeg,
            "jpg" => Self::Jpeg,
            "png" => Self::Png,
            "svg" => Self::Svg,
            "webp" => Self::Webp,
            "ico" => Self::Icon,

            "js" => Self::Js,
            "json" => Self::Json,
            _ => Self::OctetStream,
        }
    }
}

impl From<Option<&OsStr>> for MimeType {
    fn from(value: Option<&OsStr>) -> Self {
        match value {
            Some(ext) => {
                let ext = ext.to_str().expect("Should be convertable");
                MimeType::from(ext)
            }
            None => Self::OctetStream,
        }
    }
}

impl From<String> for MimeType {
    fn from(value: String) -> Self {
        MimeType::from(value.as_str())
    }
}
