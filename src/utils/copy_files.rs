use std::fs;
use std::path::Path;
use crate::utils::meta_data_parser::CopyValue;

pub(crate) const COMMON_DIR: &str = "_common";

/// Copy files from the source to the destination.
pub(crate) fn copy_files(work_dir: &str, files: &Vec<CopyValue>) {
    for file in files {
        let (src, dst) = match file {
            CopyValue::Primitive(src) => (src, src),
            CopyValue::Map(map) => map.get_index(0).expect("Failed to get index"),
        };

        let final_src = format!("{work_dir}/../{COMMON_DIR}/{src}");
        let final_dst = format!("{work_dir}/{dst}");
        let parent_dir = Path::new(&final_dst).parent().unwrap();
        fs::create_dir_all(parent_dir).expect("Failed to create directory");
        fs::copy(final_src, final_dst).expect("Failed to copy file");
        println!(" -> Copied {COMMON_DIR}/{src} to {dst}");
    }
}

/// Reverts the copied files by deleting the destination files.
pub(crate) fn delete_copied_files(work_dir: &str, files: &Vec<CopyValue>) {
    for file in files {
        let dst = match file {
            CopyValue::Primitive(dst) => dst,
            CopyValue::Map(map) => map.get_index(0).expect("Failed to get index").1,
        };

        let final_dst = format!("{work_dir}/{dst}");
        fs::remove_file(final_dst).expect("Failed to remove file");
        println!(" -> Removed {dst}");
    }
}
