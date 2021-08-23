use super::Tick;

#[inline(always)]
pub fn build_frame_backing_file_name<T: Tick>(
    tick: &T,
) -> String {
    format!("{}.qtf", tick.epoch())
}

#[inline(always)]
pub fn build_index_backing_file_name<T: Tick>(
    tick: &T,
) -> String {
    format!("{}.qti", tick.epoch())
}

#[inline(always)]
pub fn build_super_index_backing_file_name<T: Tick>(
    tick: &T,
) -> String {
    format!("{}.qti", tick.epoch())
}
