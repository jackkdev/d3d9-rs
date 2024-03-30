use std::{ffi::CStr, mem::transmute, ptr};

/// Provides wrappers/Rusty types over the `d3d9types.h` file.
use winapi::shared::d3d9types::{
    D3DFMT_CxV8U8, D3DADAPTER_IDENTIFIER9, D3DCOLOR, D3DDEVTYPE_HAL, D3DDEVTYPE_NULLREF,
    D3DDEVTYPE_REF, D3DDEVTYPE_SW, D3DDISPLAYMODE, D3DFMT_A1, D3DFMT_A16B16G16R16, D3DFMT_A1R5G5B5,
    D3DFMT_A2B10G10R10, D3DFMT_A2R10G10B10, D3DFMT_A2W10V10U10, D3DFMT_A4L4, D3DFMT_A4R4G4B4,
    D3DFMT_A8, D3DFMT_A8B8G8R8, D3DFMT_A8L8, D3DFMT_A8P8, D3DFMT_A8R3G3B2, D3DFMT_A8R8G8B8,
    D3DFMT_BINARYBUFFER, D3DFMT_D15S1, D3DFMT_D16, D3DFMT_D16_LOCKABLE, D3DFMT_D24FS8,
    D3DFMT_D24S8, D3DFMT_D24X4S4, D3DFMT_D24X8, D3DFMT_D32, D3DFMT_D32F_LOCKABLE,
    D3DFMT_D32_LOCKABLE, D3DFMT_DXT1, D3DFMT_DXT2, D3DFMT_DXT3, D3DFMT_DXT4, D3DFMT_DXT5,
    D3DFMT_G16R16, D3DFMT_G16R16F, D3DFMT_G32R32F, D3DFMT_G8R8_G8B8, D3DFMT_INDEX16,
    D3DFMT_INDEX32, D3DFMT_L16, D3DFMT_L6V5U5, D3DFMT_L8, D3DFMT_P8, D3DFMT_Q16W16V16U16,
    D3DFMT_Q8W8V8U8, D3DFMT_R16F, D3DFMT_R32F, D3DFMT_R3G3B2, D3DFMT_R5G6B5, D3DFMT_R8G8B8,
    D3DFMT_R8G8_B8G8, D3DFMT_S8_LOCKABLE, D3DFMT_UNKNOWN, D3DFMT_UYVY, D3DFMT_V16U16, D3DFMT_V8U8,
    D3DFMT_VERTEXDATA, D3DFMT_X1R5G5B5, D3DFMT_X4R4G4B4, D3DFMT_X8B8G8R8, D3DFMT_X8L8V8U8,
    D3DFMT_X8R8G8B8, D3DFMT_YUY2, D3DPOOL_DEFAULT, D3DPOOL_MANAGED, D3DPRESENT_PARAMETERS,
    D3DRTYPE_CUBETEXTURE, D3DRTYPE_INDEXBUFFER, D3DRTYPE_SURFACE, D3DRTYPE_TEXTURE,
    D3DRTYPE_VERTEXBUFFER, D3DRTYPE_VOLUME, D3DRTYPE_VOLUMETEXTURE, D3DSWAPEFFECT_COPY,
    D3DSWAPEFFECT_DISCARD, D3DSWAPEFFECT_FLIP, D3DSWAPEFFECT_OVERLAY,
};
use winapi::shared::{d3d9::D3DADAPTER_DEFAULT, d3d9caps::D3DCAPS9, guiddef::GUID, windef::HWND};

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Adapter(u32);

impl Default for Adapter {
    fn default() -> Self {
        Self(D3DADAPTER_DEFAULT)
    }
}

impl Into<u32> for Adapter {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Format {
    Unknown = D3DFMT_UNKNOWN,
    R8G8B8 = D3DFMT_R8G8B8,
    A8R8G8B8 = D3DFMT_A8R8G8B8,
    X8R8G8B8 = D3DFMT_X8R8G8B8,
    R5G6B5 = D3DFMT_R5G6B5,
    X1R5G5B5 = D3DFMT_X1R5G5B5,
    A1R5G5B5 = D3DFMT_A1R5G5B5,
    A4R4G4B4 = D3DFMT_A4R4G4B4,
    R3G3B2 = D3DFMT_R3G3B2,
    A8 = D3DFMT_A8,
    A8R3G3B2 = D3DFMT_A8R3G3B2,
    X4R4G4B4 = D3DFMT_X4R4G4B4,
    A2B10G10R10 = D3DFMT_A2B10G10R10,
    A8B8G8R8 = D3DFMT_A8B8G8R8,
    X8B8G8R8 = D3DFMT_X8B8G8R8,
    G16R16 = D3DFMT_G16R16,
    A2R10G10B10 = D3DFMT_A2R10G10B10,
    A8P8 = D3DFMT_A8P8,
    P8 = D3DFMT_P8,
    L8 = D3DFMT_L8,
    A8L8 = D3DFMT_A8L8,
    A4L4 = D3DFMT_A4L4,
    V8U8 = D3DFMT_V8U8,
    L6V5U5 = D3DFMT_L6V5U5,
    X8L8V8U8 = D3DFMT_X8L8V8U8,
    Q8W8V8U8 = D3DFMT_Q8W8V8U8,
    V16U16 = D3DFMT_V16U16,
    A2W10V10U10 = D3DFMT_A2W10V10U10,
    UYVY = D3DFMT_UYVY,
    R8g8B8g8 = D3DFMT_R8G8_B8G8,
    YUY2 = D3DFMT_YUY2,
    G8r8G8b8 = D3DFMT_G8R8_G8B8,
    DXT1 = D3DFMT_DXT1,
    DXT2 = D3DFMT_DXT2,
    DXT3 = D3DFMT_DXT3,
    DXT4 = D3DFMT_DXT4,
    DXT5 = D3DFMT_DXT5,
    D16Lockable = D3DFMT_D16_LOCKABLE,
    D32 = D3DFMT_D32,
    D15S1 = D3DFMT_D15S1,
    D24S8 = D3DFMT_D24S8,
    D24X8 = D3DFMT_D24X8,
    D24X4S4 = D3DFMT_D24X4S4,
    D16 = D3DFMT_D16,
    D32fLockable = D3DFMT_D32F_LOCKABLE,
    D24FS8 = D3DFMT_D24FS8,
    D32Lockable = D3DFMT_D32_LOCKABLE,
    S8Lockable = D3DFMT_S8_LOCKABLE,
    L16 = D3DFMT_L16,
    VertexData = D3DFMT_VERTEXDATA,
    INDEX16 = D3DFMT_INDEX16,
    INDEX32 = D3DFMT_INDEX32,
    Q16W16V16U16 = D3DFMT_Q16W16V16U16,
    R16F = D3DFMT_R16F,
    G16R16F = D3DFMT_G16R16F,
    A16B16G16R16 = D3DFMT_A16B16G16R16,
    R32F = D3DFMT_R32F,
    G32R32F = D3DFMT_G32R32F,
    CxV8U8 = D3DFMT_CxV8U8,
    A1 = D3DFMT_A1,
    BinaryBuffer = D3DFMT_BINARYBUFFER,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Pool {
    Default = D3DPOOL_DEFAULT,
    Managed = D3DPOOL_MANAGED,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum DeviceType {
    Hal = D3DDEVTYPE_HAL,
    NullRef = D3DDEVTYPE_NULLREF,
    Ref = D3DDEVTYPE_REF,
    Sw = D3DDEVTYPE_SW,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ResourceType {
    Surface = D3DRTYPE_SURFACE,
    Volume = D3DRTYPE_VOLUME,
    Texture = D3DRTYPE_TEXTURE,
    VolumeTexture = D3DRTYPE_VOLUMETEXTURE,
    CubeTexture = D3DRTYPE_CUBETEXTURE,
    VertexBuffer = D3DRTYPE_VERTEXBUFFER,
    IndexBuffer = D3DRTYPE_INDEXBUFFER,
}

#[derive(Clone, Debug)]
pub enum MultiSampleType {
    None,
    NonMaskable,
    Some(u32),
}

impl Into<u32> for MultiSampleType {
    fn into(self) -> u32 {
        match self {
            MultiSampleType::None => 0,
            MultiSampleType::NonMaskable => 1,
            MultiSampleType::Some(n) => n,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum SwapEffect {
    Discard = D3DSWAPEFFECT_DISCARD,
    Flip = D3DSWAPEFFECT_FLIP,
    Copy = D3DSWAPEFFECT_COPY,
    Overlay = D3DSWAPEFFECT_OVERLAY,
}

#[derive(Clone, Debug)]
pub struct PresentationParameters {
    pub back_buffer_width: u32,
    pub back_buffer_height: u32,
    pub back_buffer_format: Format,
    pub back_buffer_count: u32,
    pub multi_sample_type: MultiSampleType,
    pub multi_sample_quality: u32,
    pub swap_effect: SwapEffect,
    pub device_window: HWND,
    pub windowed: bool,
    pub enable_auto_depth_stencil: bool,
    pub auto_depth_stencil_format: Format,
    pub flags: u32,
    pub refresh_rate: u32,
    pub presentation_interval: u32,
}

impl Default for PresentationParameters {
    fn default() -> Self {
        Self {
            back_buffer_width: 0,
            back_buffer_height: 0,
            back_buffer_format: Format::Unknown,
            back_buffer_count: 0,
            multi_sample_type: MultiSampleType::None,
            multi_sample_quality: 0,
            swap_effect: SwapEffect::Discard,
            device_window: ptr::null_mut(),
            windowed: false,
            enable_auto_depth_stencil: false,
            auto_depth_stencil_format: Format::Unknown,
            flags: 0,
            refresh_rate: 0,
            presentation_interval: 0,
        }
    }
}

impl Into<D3DPRESENT_PARAMETERS> for PresentationParameters {
    fn into(self) -> D3DPRESENT_PARAMETERS {
        D3DPRESENT_PARAMETERS {
            BackBufferWidth: self.back_buffer_width,
            BackBufferHeight: self.back_buffer_height,
            BackBufferFormat: self.back_buffer_format as u32,
            BackBufferCount: self.back_buffer_count,
            MultiSampleType: self.multi_sample_type.into(),
            MultiSampleQuality: self.multi_sample_quality,
            SwapEffect: self.swap_effect as u32,
            hDeviceWindow: self.device_window,
            Windowed: self.windowed as i32,
            EnableAutoDepthStencil: self.enable_auto_depth_stencil as i32,
            AutoDepthStencilFormat: self.auto_depth_stencil_format as u32,
            Flags: self.flags,
            FullScreen_RefreshRateInHz: self.refresh_rate,
            PresentationInterval: self.presentation_interval,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub format: Format,
}

impl Into<D3DDISPLAYMODE> for DisplayMode {
    fn into(self) -> D3DDISPLAYMODE {
        D3DDISPLAYMODE {
            Width: self.width,
            Height: self.height,
            RefreshRate: self.refresh_rate,
            Format: self.format as u32,
        }
    }
}

impl From<D3DDISPLAYMODE> for DisplayMode {
    fn from(value: D3DDISPLAYMODE) -> Self {
        Self {
            width: value.Width,
            height: value.Height,
            refresh_rate: value.RefreshRate,
            format: unsafe { transmute(value.Format) },
        }
    }
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            refresh_rate: 0,
            format: Format::Unknown,
        }
    }
}

pub struct AdapterIdentifier {
    pub driver: String,
    pub description: String,
    pub device_name: String,
    pub driver_version: i64,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub device_identifier: GUID,
    pub whql_level: u32,
}

impl From<D3DADAPTER_IDENTIFIER9> for AdapterIdentifier {
    fn from(value: D3DADAPTER_IDENTIFIER9) -> Self {
        let c_driver = unsafe {
            CStr::from_bytes_with_nul(transmute(value.Driver.as_ref()))
                .expect("failed to parse driver name")
        };
        let driver = c_driver.to_str().expect("failed to parse driver name");

        let c_description = unsafe {
            CStr::from_bytes_with_nul(transmute(value.Description.as_ref()))
                .expect("failed to parse description")
        };
        let description = c_description.to_str().expect("failed to parse description");

        let c_device_name = unsafe {
            CStr::from_bytes_with_nul(transmute(value.DeviceName.as_ref()))
                .expect("failed to parse device name")
        };
        let device_name = c_device_name.to_str().expect("failed to parse device name");

        Self {
            driver: driver.to_string(),
            description: description.to_string(),
            device_name: device_name.to_string(),
            driver_version: unsafe { transmute(value.DriverVersion) },
            vendor_id: value.VendorId,
            device_id: value.DeviceId,
            sub_sys_id: value.SubSysId,
            revision: value.Revision,
            device_identifier: value.DeviceIdentifier,
            whql_level: value.WHQLLevel,
        }
    }
}

#[repr(C)]
pub struct Rect {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[repr(transparent)]
pub struct Color(u32);

impl Into<D3DCOLOR> for Color {
    fn into(self) -> D3DCOLOR {
        self.0
    }
}

// TODO: Implement this as a rust-safe wrapper.
pub struct Caps(pub D3DCAPS9);
