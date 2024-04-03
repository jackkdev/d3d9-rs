use std::{ffi::CString, mem::MaybeUninit, ptr::NonNull};

use d3dx9_sys::d3dx9shader::{ID3DXConstantTable, D3DXCONSTANTTABLE_DESC, D3DXCONSTANT_DESC};
use winapi::um::winnt::VOID;

use crate::{
    check_hresult, check_hresult_mut,
    com::Com,
    error::WindowsResult,
    std::interfaces::Device,
    x::types::{ConstantDesc, ConstantTableDesc, Handle},
};

#[derive(Clone)]
pub struct ConstantTable {
    inner: Com<ID3DXConstantTable>,
}

impl ConstantTable {
    pub fn with_ptr(inner: NonNull<ID3DXConstantTable>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut ID3DXConstantTable {
        self.inner.as_ptr()
    }

    pub fn get_buffer_ptr(&self) -> *const VOID {
        unsafe { self.inner.GetBufferPointer() }
    }

    pub fn get_buffer_size(&self) -> u32 {
        unsafe { self.inner.GetBufferSize() }
    }

    pub fn get_constant(&self, constant: Handle, index: u32) -> Handle {
        unsafe { self.inner.GetConstant(constant, index) }
    }

    pub fn get_constant_by_name<N: Into<String>>(&self, constant: Handle, name: N) -> Handle {
        let name = name.into();
        let name = CString::new(name).expect("failed to convert name into cstring");
        unsafe { self.inner.GetConstantByName(constant, name.as_ptr()) }
    }

    pub fn get_constant_desc<N: Into<String>>(
        &self,
        constant: Handle,
    ) -> WindowsResult<ConstantDesc> {
        unsafe {
            let mut c_constant_desc: D3DXCONSTANT_DESC = MaybeUninit::zeroed().assume_init();
            let mut c_count = 0u32;

            check_hresult_mut!(self.inner.GetConstantDesc(
                constant,
                &mut c_constant_desc as *mut _,
                &mut c_count as *mut _,
            ))?;

            Ok(ConstantDesc::from(c_constant_desc))
        }
    }

    pub fn get_constant_element(&self, constant: Handle, index: u32) -> Handle {
        unsafe { self.inner.GetConstantElement(constant, index) }
    }

    pub fn get_desc(&self) -> WindowsResult<ConstantTableDesc> {
        unsafe {
            let mut c_desc: D3DXCONSTANTTABLE_DESC = MaybeUninit::zeroed().assume_init();

            check_hresult_mut!(self.inner.GetDesc(&mut c_desc as *mut _))?;

            Ok(ConstantTableDesc::from(c_desc))
        }
    }

    pub fn get_sampler_index(&self, handle: Handle) -> u32 {
        unsafe { self.inner.GetSamplerIndex(handle) }
    }

    pub fn set_bool(&self, device: &Device, constant: Handle, value: bool) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetBool(device.as_ptr(), constant, value))?;
        }

        Ok(())
    }

    pub fn set_bool_array(
        &self,
        device: &Device,
        constant: Handle,
        values: &[bool],
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetBoolArray(
                device.as_ptr(),
                constant,
                values.as_ptr(),
                values.len() as u32,
            ))?;
        }

        Ok(())
    }

    pub fn set_defaults(&self, device: &Device) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetDefaults(device.as_ptr(),))?;
        }

        Ok(())
    }

    pub fn set_float(&self, device: &Device, constant: Handle, value: f32) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetFloat(device.as_ptr(), constant, value))?;
        }

        Ok(())
    }

    pub fn set_float_array(
        &self,
        device: &Device,
        constant: Handle,
        values: &[f32],
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetFloatArray(
                device.as_ptr(),
                constant,
                values.as_ptr(),
                values.len() as u32,
            ))?;
        }

        Ok(())
    }

    pub fn set_int(&self, device: &Device, constant: Handle, value: i32) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetInt(device.as_ptr(), constant, value))?;
        }

        Ok(())
    }

    pub fn set_int_array(
        &self,
        device: &Device,
        constant: Handle,
        values: &[i32],
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetIntArray(
                device.as_ptr(),
                constant,
                values.as_ptr(),
                values.len() as u32,
            ))?;
        }

        Ok(())
    }

    pub fn set_matrix(
        &self,
        device: &Device,
        constant: Handle,
        values: &[f32],
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self
                .inner
                .SetMatrix(device.as_ptr(), constant, values.as_ptr()))?;
        }

        Ok(())
    }

    pub fn set_matrix_array(
        &self,
        device: &Device,
        constant: Handle,
        values: &[*const f32],
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetMatrixArray(
                device.as_ptr(),
                constant,
                values.as_ptr(),
                values.len() as u32,
            ))?;
        }

        Ok(())
    }
}
