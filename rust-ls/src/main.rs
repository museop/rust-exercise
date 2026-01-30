use std::{
    env,
    fs::{self},
    os::unix::fs::{MetadataExt, PermissionsExt},
};

use chrono::{DateTime, Local};
use uzers::{get_group_by_gid, get_user_by_uid};

fn main() -> anyhow::Result<()> {
    // parse command line argument
    let path = env::args().nth(1).unwrap_or_else(|| ".".to_string());

    // read directory
    let mut entries: Vec<_> = fs::read_dir(path)?.filter_map(|res| res.ok()).collect();

    // sort by file name
    entries.sort_by_key(|entry| entry.file_name());

    // print entries
    for entry in entries {
        print_direntry(entry)?;
    }
    Ok(())
}

fn print_direntry(entry: fs::DirEntry) -> std::io::Result<()> {
    let metadata = entry.metadata()?;

    // file type and permissions
    let file_type = entry.file_type()?;
    let file_type_char = to_file_type_char(&file_type);
    let mode = to_permission_expression(metadata.permissions().mode());

    // hard link count
    let nlink = metadata.nlink();

    // owner and group
    let uid = metadata.uid();
    let gid = metadata.gid();

    let user_name = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or(uid.to_string());

    let group_name = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or(gid.to_string());

    // file size and modification time
    let file_size = to_human_readable_size(metadata.size());
    let modified = to_readable_datetime(metadata.modified()?);

    // file name (with symlink target if applicable)
    let mut file_name = entry.file_name().to_string_lossy().into_owned();
    if file_type.is_symlink() {
        if let Ok(target) = fs::read_link(entry.path()) {
            file_name = format!("{} -> {}", file_name, target.to_string_lossy())
        }
    }

    // Print formatted output
    println!(
        "{}{} {:>4} {:<8} {:<8} {:>4} {} {}",
        file_type_char, mode, nlink, user_name, group_name, file_size, modified, file_name
    );

    Ok(())
}

fn to_readable_datetime(systime_time: std::time::SystemTime) -> String {
    let datetime: DateTime<Local> = systime_time.into();
    datetime.format("%_m월 %d %H:%M").to_string()
}

fn to_file_type_char(file_type: &fs::FileType) -> char {
    if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else {
        '-'
    }
}

fn to_permission_expression(mode: u32) -> String {
    let chars = ['r', 'w', 'x'];
    let mut result = String::with_capacity(9);
    for i in (0..9).rev() {
        if (mode >> i) & 1 == 1 {
            result.push(chars[2 - (i % 3)]);
        } else {
            result.push('-');
        }
    }
    result
}

fn to_human_readable_size(size: u64) -> String {
    // 1. If size < 1024, return size in bytes (B)
    if size < 1024 {
        return format!("{:>4}B", size);
    }

    // Define units
    let units = ["B", "K", "M", "G", "T", "P"];
    let mut size_f = size as f64;
    let mut unit_index = 0;

    // Devide size by 1024 until it's less than 1024
    while size_f >= 1024.0 && unit_index < units.len() - 1 {
        size_f /= 1024.0;
        unit_index += 1;
    }

    format!("{:>4.1}{}", size_f, units[unit_index])
}
