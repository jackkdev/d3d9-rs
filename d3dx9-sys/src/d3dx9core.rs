use winapi::{
    shared::minwindef::{DWORD, LPVOID},
    um::unknwnbase::{IUnknown, IUnknownVtbl},
    DEFINE_GUID, RIDL,
};

DEFINE_GUID! {IID_ID3DXBuffer,
0x8ba5fb08, 0x5195, 0x40e2, 0xac, 0x58, 0xd, 0x98, 0x9c, 0x3a, 0x1, 0x2}

RIDL! {#[uuid(0x8ba5fb08, 0x5195, 0x40e2, 0xac, 0x58, 0xd, 0x98, 0x9c, 0x3a, 0x1, 0x2)]
interface ID3DXBuffer(ID3DXBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetBufferPointer() -> LPVOID,
    fn GetBufferSize() -> DWORD,
}}

pub type LPD3DXBUFFER = *const ID3DXBuffer;
