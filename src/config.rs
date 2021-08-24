#[inline(always)]
pub fn build_frame_backing_file_name(
    epoch: u64,
) -> String {
    format!("{}.qtf", epoch)
}

#[inline(always)]
pub fn build_index_backing_file_name(
    epoch: u64,
) -> String {
    format!("{}.qti", epoch)
}

#[inline(always)]
pub fn build_super_index_backing_file_name(
    epoch: u64,
) -> String {
    format!("{}.qti", epoch)
}
