use crate::funcs;

pub(crate) fn copy_files() {
    funcs::copy_dir_all("Super Mario Starshine Demo v1.4\\Super Mario Starshine",
    "smg2.d\\files").unwrap();
}