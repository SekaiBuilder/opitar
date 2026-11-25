//! IDX-01-R02: file kind and MIME type, by extension first, then by byte signature, then by a text sniff.

use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Image,
    Audio,
    Video,
    Model,
    Document,
    Font,
    Code,
    Archive,
    Other,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Image => "image",
            Kind::Audio => "audio",
            Kind::Video => "video",
            Kind::Model => "3d",
            Kind::Document => "document",
            Kind::Font => "font",
            Kind::Code => "code",
            Kind::Archive => "archive",
            Kind::Other => "other",
        }
    }
}

/// (extension, kind, mime). The MIME is what the server sends; browsers need it right to play media.
const TABLE: &[(&str, Kind, &str)] = &[
    // images
    ("png", Kind::Image, "image/png"),
    ("jpg", Kind::Image, "image/jpeg"),
    ("jpeg", Kind::Image, "image/jpeg"),
    ("webp", Kind::Image, "image/webp"),
    ("gif", Kind::Image, "image/gif"),
    ("svg", Kind::Image, "image/svg+xml"),
    ("avif", Kind::Image, "image/avif"),
    ("bmp", Kind::Image, "image/bmp"),
    ("ico", Kind::Image, "image/x-icon"),
    ("tif", Kind::Image, "image/tiff"),
    ("tiff", Kind::Image, "image/tiff"),
    ("heic", Kind::Image, "image/heic"),
    ("psd", Kind::Image, "image/vnd.adobe.photoshop"),
    ("exr", Kind::Image, "image/x-exr"),
    ("hdr", Kind::Image, "image/vnd.radiance"),
    ("tga", Kind::Image, "image/x-tga"),
    ("dds", Kind::Image, "image/vnd-ms.dds"),
    ("ktx2", Kind::Image, "image/ktx2"),
    // audio
    ("wav", Kind::Audio, "audio/wav"),
    ("mp3", Kind::Audio, "audio/mpeg"),
    ("ogg", Kind::Audio, "audio/ogg"),
    ("oga", Kind::Audio, "audio/ogg"),
    ("opus", Kind::Audio, "audio/ogg"),
    ("m4a", Kind::Audio, "audio/mp4"),
    ("aac", Kind::Audio, "audio/aac"),
    ("flac", Kind::Audio, "audio/flac"),
    ("aif", Kind::Audio, "audio/aiff"),
    ("aiff", Kind::Audio, "audio/aiff"),
    ("mid", Kind::Audio, "audio/midi"),
    ("midi", Kind::Audio, "audio/midi"),
    // video
    ("mp4", Kind::Video, "video/mp4"),
    ("m4v", Kind::Video, "video/mp4"),
    ("webm", Kind::Video, "video/webm"),
    ("mov", Kind::Video, "video/quicktime"),
    ("mkv", Kind::Video, "video/x-matroska"),
    ("avi", Kind::Video, "video/x-msvideo"),
    ("ogv", Kind::Video, "video/ogg"),
    // 3d
    ("glb", Kind::Model, "model/gltf-binary"),
    ("gltf", Kind::Model, "model/gltf+json"),
    ("obj", Kind::Model, "model/obj"),
    ("stl", Kind::Model, "model/stl"),
    ("fbx", Kind::Model, "application/octet-stream"),
    ("ply", Kind::Model, "application/octet-stream"),
    ("usdz", Kind::Model, "model/vnd.usdz+zip"),
    ("usd", Kind::Model, "application/octet-stream"),
    ("dae", Kind::Model, "model/vnd.collada+xml"),
    ("3ds", Kind::Model, "application/octet-stream"),
    ("blend", Kind::Model, "application/octet-stream"),
    ("vox", Kind::Model, "application/octet-stream"),
    // documents
    ("pdf", Kind::Document, "application/pdf"),
    ("md", Kind::Document, "text/markdown; charset=utf-8"),
    ("txt", Kind::Document, "text/plain; charset=utf-8"),
    ("rtf", Kind::Document, "application/rtf"),
    ("doc", Kind::Document, "application/msword"),
    ("docx", Kind::Document, "application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
    ("xls", Kind::Document, "application/vnd.ms-excel"),
    ("xlsx", Kind::Document, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    ("ppt", Kind::Document, "application/vnd.ms-powerpoint"),
    ("pptx", Kind::Document, "application/vnd.openxmlformats-officedocument.presentationml.presentation"),
    ("key", Kind::Document, "application/octet-stream"),
    ("pages", Kind::Document, "application/octet-stream"),
    ("csv", Kind::Document, "text/csv; charset=utf-8"),
    ("tsv", Kind::Document, "text/tab-separated-values; charset=utf-8"),
    ("epub", Kind::Document, "application/epub+zip"),
    // fonts
    ("ttf", Kind::Font, "font/ttf"),
    ("otf", Kind::Font, "font/otf"),
    ("woff", Kind::Font, "font/woff"),
    ("woff2", Kind::Font, "font/woff2"),
    // code and data
    ("rs", Kind::Code, "text/plain; charset=utf-8"),
    ("js", Kind::Code, "text/plain; charset=utf-8"),
    ("mjs", Kind::Code, "text/plain; charset=utf-8"),
    ("ts", Kind::Code, "text/plain; charset=utf-8"),
    ("tsx", Kind::Code, "text/plain; charset=utf-8"),
    ("jsx", Kind::Code, "text/plain; charset=utf-8"),
    ("py", Kind::Code, "text/plain; charset=utf-8"),
    ("swift", Kind::Code, "text/plain; charset=utf-8"),
    ("kt", Kind::Code, "text/plain; charset=utf-8"),
    ("java", Kind::Code, "text/plain; charset=utf-8"),
    ("c", Kind::Code, "text/plain; charset=utf-8"),
    ("h", Kind::Code, "text/plain; charset=utf-8"),
    ("cpp", Kind::Code, "text/plain; charset=utf-8"),
    ("hpp", Kind::Code, "text/plain; charset=utf-8"),
    ("cs", Kind::Code, "text/plain; charset=utf-8"),
    ("go", Kind::Code, "text/plain; charset=utf-8"),
    ("rb", Kind::Code, "text/plain; charset=utf-8"),
    ("php", Kind::Code, "text/plain; charset=utf-8"),
    ("lua", Kind::Code, "text/plain; charset=utf-8"),
    ("gd", Kind::Code, "text/plain; charset=utf-8"),
    ("glsl", Kind::Code, "text/plain; charset=utf-8"),
    ("wgsl", Kind::Code, "text/plain; charset=utf-8"),
    ("hlsl", Kind::Code, "text/plain; charset=utf-8"),
    ("shader", Kind::Code, "text/plain; charset=utf-8"),
    ("sh", Kind::Code, "text/plain; charset=utf-8"),
    ("html", Kind::Code, "text/plain; charset=utf-8"),
    ("css", Kind::Code, "text/plain; charset=utf-8"),
    ("json", Kind::Code, "text/plain; charset=utf-8"),
    ("toml", Kind::Code, "text/plain; charset=utf-8"),
    ("yaml", Kind::Code, "text/plain; charset=utf-8"),
    ("yml", Kind::Code, "text/plain; charset=utf-8"),
    ("xml", Kind::Code, "text/plain; charset=utf-8"),
    ("sql", Kind::Code, "text/plain; charset=utf-8"),
    ("tscn", Kind::Code, "text/plain; charset=utf-8"),
    ("tres", Kind::Code, "text/plain; charset=utf-8"),
    ("unity", Kind::Code, "text/plain; charset=utf-8"),
    ("prefab", Kind::Code, "text/plain; charset=utf-8"),
    ("mtl", Kind::Code, "text/plain; charset=utf-8"),
    // archives
    ("zip", Kind::Archive, "application/zip"),
    ("tar", Kind::Archive, "application/x-tar"),
    ("gz", Kind::Archive, "application/gzip"),
    ("tgz", Kind::Archive, "application/gzip"),
    ("7z", Kind::Archive, "application/x-7z-compressed"),
    ("rar", Kind::Archive, "application/vnd.rar"),
    ("xz", Kind::Archive, "application/x-xz"),
    ("unitypackage", Kind::Archive, "application/gzip"),
];

pub fn by_extension(ext: &str) -> Option<(Kind, &'static str)> {
    let ext = ext.to_ascii_lowercase();
    TABLE.iter().find(|(e, _, _)| *e == ext).map(|(_, k, m)| (*k, *m))
}

/// Kind and MIME for a file; `head` is the first bytes of the file (up to 8 KB is enough).
pub fn classify(path: &Path, head: &[u8]) -> (Kind, String) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if let Some((kind, mime)) = by_extension(ext) {
        return (kind, mime.to_string());
    }
    if let Some(t) = infer::get(head) {
        let kind = match t.matcher_type() {
            infer::MatcherType::Image => Kind::Image,
            infer::MatcherType::Audio => Kind::Audio,
            infer::MatcherType::Video => Kind::Video,
            infer::MatcherType::Font => Kind::Font,
            infer::MatcherType::Archive => Kind::Archive,
            infer::MatcherType::Doc | infer::MatcherType::Book => Kind::Document,
            _ => Kind::Other,
        };
        return (kind, t.mime_type().to_string());
    }
    if !head.is_empty() && looks_like_text(head) {
        return (Kind::Code, "text/plain; charset=utf-8".into());
    }
    (Kind::Other, "application/octet-stream".into())
}

/// UTF-8 without NUL bytes. A multi-byte char cut at the end of the sample is allowed.
pub fn looks_like_text(head: &[u8]) -> bool {
    if head.contains(&0) {
        return false;
    }
    match std::str::from_utf8(head) {
        Ok(_) => true,
        Err(e) => e.error_len().is_none() && e.valid_up_to() + 4 > head.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_wins_and_is_case_insensitive() {
        assert_eq!(classify(Path::new("a/B.PNG"), b"").0, Kind::Image);
        assert_eq!(classify(Path::new("m.glb"), b"").0, Kind::Model);
        assert_eq!(classify(Path::new("s.flac"), b"").0, Kind::Audio);
        assert_eq!(classify(Path::new("f.woff2"), b"").0, Kind::Font);
    }

    #[test]
    fn signature_then_text_then_other() {
        let png = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0];
        assert_eq!(classify(Path::new("sin_extension"), &png).0, Kind::Image);
        assert_eq!(classify(Path::new("README"), "hello, señor\n".as_bytes()).0, Kind::Code);
        assert_eq!(classify(Path::new("blob"), &[0, 1, 2, 3, 250]).0, Kind::Other);
    }
}
