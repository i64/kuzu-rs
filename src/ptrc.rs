use crate::error;
/// Wrapper for a raw pointer, providing validation functionality.
///
/// The `PtrContainer` struct wraps a raw pointer `*mut T` and provides a `validate` method
/// to check if the pointer is null. If the pointer is null, it returns an `Error::FFIGotNull` with an
/// appropriate error message. Otherwise, it returns the original `PtrContainer`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub(crate) struct PtrContainer<T>(pub *mut T)
where
    T: ?Sized,
    Self: CustomDrop;

impl<T> PtrContainer<T>
where
    T: ?Sized,
    Self: CustomDrop,
{
    /// Validates the pointer and creates a `PtrContainer` if it is not null.
    /// Returns an error of type `Error::FFIGotNull` if the pointer is null.
    #[inline]
    pub fn try_new(ptr: *mut T) -> error::Result<Self> {
        match ptr.is_null() {
            true => Err(error::Error::FFIGotNull(std::any::type_name::<Self>())),
            false => Ok(Self(ptr)),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
}

impl<T> Drop for PtrContainer<T>
where
    T: ?Sized,
    Self: CustomDrop,
{
    fn drop(&mut self) {
        self._drop()
    }
}
pub(crate) trait CustomDrop {
    fn _drop(&mut self);
}


macro_rules! drop_ptr_container {
    ($struct_name:ident, $destroyer:ident) => {
        impl CustomDrop for PtrContainer<$crate::ffi::$struct_name> {
            fn _drop(&mut self) {
                if self.is_valid() {
                    unsafe { $crate::ffi::$destroyer(self.0) }
                }
            }
        }
    };
}

drop_ptr_container!(kuzu_connection, kuzu_connection_destroy);
drop_ptr_container!(kuzu_node_val, kuzu_node_val_destroy);
drop_ptr_container!(kuzu_prepared_statement, kuzu_prepared_statement_destroy);
drop_ptr_container!(kuzu_query_result, kuzu_query_result_destroy);
drop_ptr_container!(kuzu_rel_val, kuzu_rel_val_destroy);
drop_ptr_container!(kuzu_value, kuzu_value_destroy);
drop_ptr_container!(kuzu_database, kuzu_database_destroy);
drop_ptr_container!(kuzu_logical_type, kuzu_data_type_destroy);
