#[inline(always)]
pub fn build_frame_backing_file_name(
    epoch: u64,
) -> String {
    format!("frameset/{}.qtf", epoch)
}

#[inline(always)]
pub fn build_index_backing_file_name(
    epoch: u64,
) -> String {
    format!("frameset/{}.qti", epoch)
}

#[inline(always)]
pub fn build_epoch_index_backing_file_name() -> String {
    format!("epochs.qti")
}
