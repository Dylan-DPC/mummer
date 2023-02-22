use mummer::get_mime_extensions_str;

fn main() {
    print_exts("video/*");
    print_exts("video/x-matroska");
}

fn print_exts(mime_type: &str) {
    println!(
        "Exts for {:?}: {:?}",
        mime_type,
        get_mime_extensions_str(mime_type)
    );
}
