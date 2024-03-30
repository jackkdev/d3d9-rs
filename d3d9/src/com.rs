//! Provides the [`Com<T>`] type which acts a referencing counting wrapper over a Windows COM interface.

use std::{ops::Deref, ptr::NonNull};

use winapi::{um::unknwnbase::IUnknown, Interface};

/// Provides a reference counting clone-able struct over a Windows COM interface.
///
/// Method access is provided via the [`Deref`] trait.
pub struct Com<T: Interface> {
    inner: NonNull<T>,
}

impl<T: Interface> Com<T> {
    /// Returns a new instance with the passed pointer.
    pub fn with_ptr(inner: NonNull<T>) -> Self {
        Self { inner }
    }

    /// Returns a ref to the data which this pointer points to.
    pub fn as_ref(&self) -> &T {
        unsafe { self.inner.as_ref() }
    }

    /// Returns a pointer to the data which this pointer points to.
    pub fn as_ptr(&self) -> *mut T {
        self.inner.as_ptr()
    }

    fn as_unknown(&self) -> *mut IUnknown {
        self.inner.as_ptr() as *mut IUnknown
    }
}

impl<T: Interface> Clone for Com<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.as_unknown()
                .as_ref()
                .expect("IUnknown is null")
                .AddRef()
        };

        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Interface> Drop for Com<T> {
    fn drop(&mut self) {
        unsafe {
            self.as_unknown()
                .as_ref()
                .expect("IUnknown is null")
                .AddRef()
        };
    }
}

impl<T: Interface> Deref for Com<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
