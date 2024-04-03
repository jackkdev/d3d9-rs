#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use winapi::{
    shared::{
        d3d9::LPDIRECT3DDEVICE9,
        minwindef::{DWORD, LPCVOID, LPVOID},
    },
    um::{
        unknwnbase::{IUnknown, IUnknownVtbl},
        winnt::{HRESULT, LPCSTR},
    },
    DEFINE_GUID, ENUM, RIDL, STRUCT,
};

use crate::d3dx9core::LPD3DXBUFFER;

pub const D3DXSHADER_DEBUG: u32 = 1 << 0;
pub const D3DXSHADER_SKIPVALIDATION: u32 = 1 << 1;
pub const D3DXSHADER_SKIPOPTIMIZATION: u32 = 1 << 2;
pub const D3DXSHADER_PACKMATRIX_ROWMAJOR: u32 = 1 << 3;
pub const D3DXSHADER_PACKMATRIX_COLUMNMAJOR: u32 = 1 << 4;
pub const D3DXSHADER_PARTIALPRECISION: u32 = 1 << 5;
pub const D3DXSHADER_FORCE_VS_SOFTWARE_NOOPT: u32 = 1 << 6;
pub const D3DXSHADER_FORCE_PS_SOFTWARE_NOOPT: u32 = 1 << 7;
pub const D3DXSHADER_NO_PRESHADER: u32 = 1 << 8;
pub const D3DXSHADER_AVOID_FLOW_CONTROL: u32 = 1 << 9;
pub const D3DXSHADER_PREFER_FLOW_CONTROL: u32 = 1 << 10;
pub const D3DXSHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 1 << 12;
pub const D3DXSHADER_IEEE_STRICTNESS: u32 = 1 << 13;
pub const D3DXSHADER_USE_LEGACY_D3DX9_31_DLL: u32 = 1 << 16;
pub const D3DXSHADER_OPTIMIZATION_LEVEL0: u32 = 1 << 14;
pub const D3DXSHADER_OPTIMIZATION_LEVEL1: u32 = 0;
pub const D3DXSHADER_OPTIMIZATION_LEVEL2: u32 = (1 << 14) | (1 << 15);
pub const D3DXSHADER_OPTIMIZATION_LEVEL3: u32 = 1 << 15;

pub const D3DXCONSTTABLE_LARGEADDRESSAWARE: u32 = 1 << 17;

pub type D3DXHANDLE = *const ();

STRUCT! {struct D3DXMACRO {
    Name: LPCSTR,
    Definition: LPCSTR,
}}

STRUCT! {struct D3DXSEMANTIC {
    Usage: u32,
    UsageIndex: u32,
}}

ENUM! {enum D3DXREGISTER_SET {
    D3DXRS_BOOL,
    D3DXRS_INT4,
    D3DXRS_FLOAT4,
    D3DXRS_SAMPLER,
}}

ENUM! {enum D3DXPARAMETER_CLASS {
    D3DXPC_SCALAR,
    D3DXPC_VECTOR,
    D3DXPC_MATRIX_ROWS,
    D3DXPC_MATRIX_COLUMNS,
    D3DXPC_OBJECT,
    D3DXPC_STRUCT,
}}

ENUM! {enum D3DXPARAMETER_TYPE {
    D3DXPT_VOID,
    D3DXPT_BOOL,
    D3DXPT_INT,
    D3DXPT_FLOAT,
    D3DXPT_STRING,
    D3DXPT_TEXTURE,
    D3DXPT_TEXTURE1D,
    D3DXPT_TEXTURE2D,
    D3DXPT_TEXTURE3D,
    D3DXPT_TEXTURECUBE,
    D3DXPT_SAMPLER,
    D3DXPT_SAMPLER1D,
    D3DXPT_SAMPLER2D,
    D3DXPT_SAMPLER3D,
    D3DXPT_SAMPLERCUBE,
    D3DXPT_PIXELSHADER,
    D3DXPT_VERTEXSHADER,
    D3DXPT_PIXELFRAGMENT,
    D3DXPT_VERTEXFRAGMENT,
    D3DXPT_UNSUPPORTED,
}}

STRUCT! {struct D3DXCONSTANTTABLE_DESC {
    Creator: LPCSTR,
    Version: u32,
    Constants: u32,
}}

STRUCT! {struct D3DXCONSTANT_DESC {
    Name: LPCSTR,

    RegisterSet: D3DXREGISTER_SET,
    RegisterIndex: u32,
    RegisterCount: u32,

    Class: D3DXPARAMETER_CLASS,
    Type: D3DXPARAMETER_TYPE,

    Rows: u32,
    Columns: u32,
    Elements: u32,
    StructMembers: u32,

    Bytes: u32,
    DefaultValue: LPCVOID,
}}

DEFINE_GUID! {IID_ID3DXConstantTable,
0xab3c758f, 0x93e, 0x4356, 0xb7, 0x62, 0x4d, 0xb1, 0x8f, 0x1b, 0x3a, 0x1}

RIDL! {#[uuid(0xab3c758f, 0x93e, 0x4356, 0xb7, 0x62, 0x4d, 0xb1, 0x8f, 0x1b, 0x3a, 0x1)]
interface ID3DXConstantTable(ID3DXConstantTableVtbl): IUnknown(IUnknownVtbl) {
    fn GetBufferPointer() -> LPVOID,
    fn GetBufferSize() -> DWORD,
    fn GetDesc(
        pDesc: *mut D3DXCONSTANTTABLE_DESC,
    ) -> HRESULT,
    fn GetConstantDesc(
        hConstant: D3DXHANDLE,
        pConstantDesc: *const D3DXCONSTANT_DESC,
        pCount: *mut u32,
    ) -> HRESULT,
    fn GetSamplerIndex(
        hConstant: D3DXHANDLE,
    ) -> u32,
    fn GetConstant(
        hConstant: D3DXHANDLE,
        index: u32,
    ) -> D3DXHANDLE,
    fn GetConstantByName(
        hConstant: D3DXHANDLE,
        name: LPCSTR,
    ) -> D3DXHANDLE,
    fn GetConstantElement(
        hConstant: D3DXHANDLE,
        index: u32,
    ) -> D3DXHANDLE,
    fn SetDefaults(
        pDevice: LPDIRECT3DDEVICE9,
    ) -> HRESULT,
    fn SetValue(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        pData: LPCVOID,
        Bytes: u32,
    ) -> HRESULT,
    fn SetBool(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        b: bool,
    ) -> HRESULT,
    fn SetBoolArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        b: *const bool,
        count: u32,
    ) -> HRESULT,
    fn SetInt(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: i32,
    ) -> HRESULT,
    fn SetIntArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const i32,
        count: u32,
    ) -> HRESULT,
    fn SetFloat(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: f32,
    ) -> HRESULT,
    fn SetFloatArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const f32,
        count: u32,
    ) -> HRESULT,
    fn SetVector(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const f32,
    ) -> HRESULT,
    fn SetVectorArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const f32,
        count: u32,
    ) -> HRESULT,
    fn SetMatrix(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const f32,
    ) -> HRESULT,
    fn SetMatrixArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const *const f32,
        count: u32,
    ) -> HRESULT,
    fn SetMatrixPointerArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const *const f32,
        count: u32,
    ) -> HRESULT,
    fn SetMatrixTranspose(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const f32,
    ) -> HRESULT,
    fn SetMatrixTransposeArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const f32,
        count: u32,
    ) -> HRESULT,
    fn SetMatrixTransposePointerArray(
        pDevice: LPDIRECT3DDEVICE9,
        hConstant: D3DXHANDLE,
        n: *const *const f32,
        count: u32,
    ) -> HRESULT,
}}

pub type LPD3DXCONSTANTTABLE = *mut ID3DXConstantTable;

#[link(name = "d3dx9")]
extern "system" {
    pub fn D3DXCompileShaderFromFileA(
        pSrcFile: LPCSTR,
        pDefines: *const D3DXMACRO,
        pInclude: *const (),
        pFunctionName: LPCSTR,
        pProfile: LPCSTR,
        flags: DWORD,
        ppShader: *mut LPD3DXBUFFER,
        ppErrorMsgs: *mut LPD3DXBUFFER,
        ppConstantTable: *mut LPD3DXCONSTANTTABLE,
    ) -> HRESULT;

    pub fn D3DXCompileShader(
        pSrcData: LPCSTR,
        SrcDataLen: u32,
        pDefines: *const D3DXMACRO,
        pInclude: *const (),
        pFunctionName: LPCSTR,
        pProfile: LPCSTR,
        flags: DWORD,
        ppShader: *mut LPD3DXBUFFER,
        ppErrorMsgs: *mut LPD3DXBUFFER,
        ppConstantTable: *mut LPD3DXCONSTANTTABLE,
    ) -> HRESULT;
}
