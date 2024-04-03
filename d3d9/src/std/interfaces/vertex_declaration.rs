use std::ptr::NonNull;

use winapi::shared::d3d9::IDirect3DVertexDeclaration9;

use crate::com::Com;

#[derive(Clone)]
pub struct VertexDeclaration {
    inner: Com<IDirect3DVertexDeclaration9>,
}

impl VertexDeclaration {
    pub fn with_ptr(inner: NonNull<IDirect3DVertexDeclaration9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DVertexDeclaration9 {
        self.inner.as_ptr()
    }
}
