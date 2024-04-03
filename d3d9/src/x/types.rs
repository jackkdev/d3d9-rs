use std::{
    ffi::{CStr, CString},
    mem::transmute,
};

use bitfield::bitfield;
use d3dx9_sys::d3dx9shader::{
    D3DXCONSTANTTABLE_DESC, D3DXCONSTANT_DESC, D3DXMACRO, D3DXPC_MATRIX_COLUMNS,
    D3DXPC_MATRIX_ROWS, D3DXPC_OBJECT, D3DXPC_SCALAR, D3DXPC_STRUCT, D3DXPC_VECTOR, D3DXPT_BOOL,
    D3DXPT_FLOAT, D3DXPT_INT, D3DXPT_PIXELFRAGMENT, D3DXPT_PIXELSHADER, D3DXPT_SAMPLER,
    D3DXPT_SAMPLER1D, D3DXPT_SAMPLER2D, D3DXPT_SAMPLER3D, D3DXPT_SAMPLERCUBE, D3DXPT_STRING,
    D3DXPT_TEXTURE, D3DXPT_TEXTURE1D, D3DXPT_TEXTURE2D, D3DXPT_TEXTURE3D, D3DXPT_TEXTURECUBE,
    D3DXPT_UNSUPPORTED, D3DXPT_VERTEXFRAGMENT, D3DXPT_VERTEXSHADER, D3DXPT_VOID, D3DXRS_BOOL,
    D3DXRS_FLOAT4, D3DXRS_INT4, D3DXRS_SAMPLER,
};

#[derive(Clone, Debug)]
pub struct Macro {
    pub name: &'static CStr,
    pub definition: &'static CStr,
}

impl Macro {
    pub unsafe fn as_d3dx(&self) -> D3DXMACRO {
        D3DXMACRO {
            Name: self.name.as_ptr(),
            Definition: self.definition.as_ptr(),
        }
    }
}

pub struct ConstantTableDesc {
    pub creator: &'static CStr,
    pub version: u32,
    pub constants: u32,
}

impl From<D3DXCONSTANTTABLE_DESC> for ConstantTableDesc {
    fn from(value: D3DXCONSTANTTABLE_DESC) -> Self {
        Self {
            creator: unsafe { CStr::from_ptr(value.Creator) },
            version: value.Version,
            constants: value.Constants,
        }
    }
}

pub struct ConstantDesc {
    pub name: &'static CStr,
    pub register_set: RegisterSet,
    pub register_index: u32,
    pub register_count: u32,
    pub class: ParameterClass,
    pub kind: ParameterType,
    pub rows: u32,
    pub columns: u32,
    pub elements: u32,
    pub struct_members: u32,
    pub bytes: u32,
    pub default_value: *const (),
}

impl Into<D3DXCONSTANT_DESC> for ConstantDesc {
    fn into(self) -> D3DXCONSTANT_DESC {
        D3DXCONSTANT_DESC {
            Name: self.name.as_ptr(),
            RegisterSet: self.register_set.into(),
            RegisterIndex: self.register_index,
            RegisterCount: self.register_count,
            Class: self.class.into(),
            Type: self.kind.into(),
            Rows: self.rows,
            Columns: self.columns,
            Elements: self.elements,
            StructMembers: self.struct_members,
            Bytes: self.bytes,
            DefaultValue: self.default_value as *const _,
        }
    }
}

impl From<D3DXCONSTANT_DESC> for ConstantDesc {
    fn from(value: D3DXCONSTANT_DESC) -> Self {
        Self {
            name: unsafe { CStr::from_ptr(value.Name) },
            register_set: unsafe { transmute(value.RegisterSet) },
            register_index: value.RegisterIndex,
            register_count: value.RegisterCount,
            class: unsafe { transmute(value.Class) },
            kind: unsafe { transmute(value.Type) },
            rows: value.Rows,
            columns: value.Columns,
            elements: value.Elements,
            struct_members: value.StructMembers,
            bytes: value.Bytes,
            default_value: value.DefaultValue as *const _,
        }
    }
}

#[repr(u32)]
pub enum ParameterClass {
    Scalar = D3DXPC_SCALAR,
    Vector = D3DXPC_VECTOR,
    MatrixRows = D3DXPC_MATRIX_ROWS,
    MatrixColumns = D3DXPC_MATRIX_COLUMNS,
    Object = D3DXPC_OBJECT,
    Struct = D3DXPC_STRUCT,
}

impl Into<u32> for ParameterClass {
    fn into(self) -> u32 {
        self as u32
    }
}

#[repr(u32)]
pub enum ParameterType {
    Void = D3DXPT_VOID,
    Bool = D3DXPT_BOOL,
    Int = D3DXPT_INT,
    Float = D3DXPT_FLOAT,
    String = D3DXPT_STRING,
    Texture = D3DXPT_TEXTURE,
    Texture1D = D3DXPT_TEXTURE1D,
    Texture2D = D3DXPT_TEXTURE2D,
    Texture3D = D3DXPT_TEXTURE3D,
    TextureCube = D3DXPT_TEXTURECUBE,
    Sampler = D3DXPT_SAMPLER,
    Sampler1D = D3DXPT_SAMPLER1D,
    Sampler2D = D3DXPT_SAMPLER2D,
    Sampler3D = D3DXPT_SAMPLER3D,
    SamplerCube = D3DXPT_SAMPLERCUBE,
    PixelShader = D3DXPT_PIXELSHADER,
    VertexShader = D3DXPT_VERTEXSHADER,
    PixelFragment = D3DXPT_PIXELFRAGMENT,
    VertexFragment = D3DXPT_VERTEXFRAGMENT,
    Unsupported = D3DXPT_UNSUPPORTED,
}

impl Into<u32> for ParameterType {
    fn into(self) -> u32 {
        self as u32
    }
}

pub type Handle = *const ();

#[repr(u32)]
pub enum RegisterSet {
    Bool = D3DXRS_BOOL,
    Int4 = D3DXRS_INT4,
    Float4 = D3DXRS_FLOAT4,
    Sampler = D3DXRS_SAMPLER,
}

impl Into<u32> for RegisterSet {
    fn into(self) -> u32 {
        self as u32
    }
}
