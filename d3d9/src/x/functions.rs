use std::{
    ffi::{CString, NulError},
    ptr,
    ptr::NonNull,
    str::Utf8Error,
};

use d3dx9_sys::{
    d3dx9core::ID3DXBuffer,
    d3dx9shader::{D3DXCompileShader, D3DXCompileShaderFromFileA, ID3DXConstantTable, D3DXMACRO},
};

use crate::{
    check_hresult, check_hresult_mut,
    error::WindowsError,
    x::{
        interfaces::{Buffer, ConstantTable},
        types::Macro,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("Windows Error: {from}")]
    Windows {
        #[from]
        from: WindowsError,
    },

    #[error("Nul Error: {from}")]
    Nul {
        #[from]
        from: NulError,
    },
}

pub fn compile_shader<S: Into<String>, F: Into<String>, P: Into<String>>(
    src_data: S,
    defines: Option<&[Macro]>,
    function_name: F,
    profile: P,
) -> Result<(Buffer, ConstantTable), CompileError> {
    let src_data = src_data.into();
    let c_src_data = CString::new(src_data.clone())?;
    let c_function_name = CString::new(function_name.into())?;
    let c_profile = CString::new(profile.into())?;

    let defines_ptr = match defines {
        Some(defines) => {
            let c_defines = Vec::from(defines);
            let mut c_defines = c_defines
                .into_iter()
                .map(|define| unsafe { define.as_d3dx() })
                .collect::<Vec<D3DXMACRO>>();
            c_defines.push(D3DXMACRO {
                Name: ptr::null(),
                Definition: ptr::null(),
            });
            c_defines.as_ptr()
        }
        None => ptr::null(),
    };

    let mut c_shader: *mut ID3DXBuffer = ptr::null_mut();
    let mut c_error_messages: *mut ID3DXBuffer = ptr::null_mut();
    let mut c_constant_table: *mut ID3DXConstantTable = ptr::null_mut();

    unsafe {
        check_hresult_mut!(D3DXCompileShader(
            c_src_data.as_ptr(),
            src_data.len() as u32,
            defines_ptr,
            ptr::null(),
            c_function_name.as_ptr(),
            c_profile.as_ptr(),
            0,
            &mut c_shader as *mut _,
            &mut c_error_messages as *mut _,
            &mut c_constant_table as *mut _,
        ))?;
    }

    Ok((
        Buffer::with_ptr(NonNull::new(c_shader).expect("returned shader buffer is null")),
        ConstantTable::with_ptr(
            NonNull::new(c_constant_table).expect("returned constant table is null"),
        ),
    ))
}
