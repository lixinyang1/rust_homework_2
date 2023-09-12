use regex::Regex;
use std::fs;
use std::path::Path;

pub fn find<P:AsRef<Path>>(root: P , regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches: Vec<String> = Vec::new();
    walk_tree(root.as_ref(),regex,&mut matches)?;
    Ok(matches)
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>
) -> Result<(), Box<dyn std::error::Error>>{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)?{
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                walk_tree(&path,regex,matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s: &std::ffi::OsStr| s.to_str()){
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}