
use super::ui_error::{UIResult, OutOfBounds};
use snafu::OptionExt;
use std::slice::SliceIndex;

// replace vec methods that return Option with ones that return Result and proper Error
pub fn slice_get<T>(index: usize, slice: &[T]) -> UIResult<&<usize as SliceIndex<[T]>>::Output> {
    let elt_ref = slice.get(index).context(OutOfBounds {
        index,
        collection_name: "Slice",
        len: slice.len(),
    })?;

    Ok(elt_ref)
}