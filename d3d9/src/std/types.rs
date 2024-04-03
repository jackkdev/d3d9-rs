//! Provides wrappers/Rusty types over the `d3d9types.h` file.

use std::{ffi::CStr, mem::transmute, ptr};

use bitfield::bitfield;
use winapi::{
    shared::{
        d3d9::D3DADAPTER_DEFAULT,
        d3d9caps::D3DCAPS9,
        d3d9types::{
            D3DFMT_CxV8U8, D3DADAPTER_IDENTIFIER9, D3DCOLOR, D3DDECLMETHOD_CROSSUV,
            D3DDECLMETHOD_DEFAULT, D3DDECLMETHOD_LOOKUP, D3DDECLMETHOD_LOOKUPPRESAMPLED,
            D3DDECLMETHOD_PARTIALU, D3DDECLMETHOD_PARTIALV, D3DDECLMETHOD_UV, D3DDECLTYPE_D3DCOLOR,
            D3DDECLTYPE_DEC3N, D3DDECLTYPE_FLOAT1, D3DDECLTYPE_FLOAT16_2, D3DDECLTYPE_FLOAT16_4,
            D3DDECLTYPE_FLOAT2, D3DDECLTYPE_FLOAT3, D3DDECLTYPE_FLOAT4, D3DDECLTYPE_SHORT2,
            D3DDECLTYPE_SHORT2N, D3DDECLTYPE_SHORT4, D3DDECLTYPE_SHORT4N, D3DDECLTYPE_UBYTE4,
            D3DDECLTYPE_UBYTE4N, D3DDECLTYPE_UDEC3, D3DDECLTYPE_USHORT2N, D3DDECLTYPE_USHORT4N,
            D3DDECLUSAGE_BINORMAL, D3DDECLUSAGE_BLENDINDICES, D3DDECLUSAGE_BLENDWEIGHT,
            D3DDECLUSAGE_COLOR, D3DDECLUSAGE_DEPTH, D3DDECLUSAGE_FOG, D3DDECLUSAGE_NORMAL,
            D3DDECLUSAGE_POSITION, D3DDECLUSAGE_POSITIONT, D3DDECLUSAGE_PSIZE, D3DDECLUSAGE_SAMPLE,
            D3DDECLUSAGE_TANGENT, D3DDECLUSAGE_TESSFACTOR, D3DDECLUSAGE_TEXCOORD, D3DDEVTYPE_HAL,
            D3DDEVTYPE_NULLREF, D3DDEVTYPE_REF, D3DDEVTYPE_SW, D3DDISPLAYMODE, D3DFMT_A1,
            D3DFMT_A16B16G16R16, D3DFMT_A1R5G5B5, D3DFMT_A2B10G10R10, D3DFMT_A2R10G10B10,
            D3DFMT_A2W10V10U10, D3DFMT_A4L4, D3DFMT_A4R4G4B4, D3DFMT_A8, D3DFMT_A8B8G8R8,
            D3DFMT_A8L8, D3DFMT_A8P8, D3DFMT_A8R3G3B2, D3DFMT_A8R8G8B8, D3DFMT_BINARYBUFFER,
            D3DFMT_D15S1, D3DFMT_D16, D3DFMT_D16_LOCKABLE, D3DFMT_D24FS8, D3DFMT_D24S8,
            D3DFMT_D24X4S4, D3DFMT_D24X8, D3DFMT_D32, D3DFMT_D32F_LOCKABLE, D3DFMT_D32_LOCKABLE,
            D3DFMT_DXT1, D3DFMT_DXT2, D3DFMT_DXT3, D3DFMT_DXT4, D3DFMT_DXT5, D3DFMT_G16R16,
            D3DFMT_G16R16F, D3DFMT_G32R32F, D3DFMT_G8R8_G8B8, D3DFMT_INDEX16, D3DFMT_INDEX32,
            D3DFMT_L16, D3DFMT_L6V5U5, D3DFMT_L8, D3DFMT_P8, D3DFMT_Q16W16V16U16, D3DFMT_Q8W8V8U8,
            D3DFMT_R16F, D3DFMT_R32F, D3DFMT_R3G3B2, D3DFMT_R5G6B5, D3DFMT_R8G8B8,
            D3DFMT_R8G8_B8G8, D3DFMT_S8_LOCKABLE, D3DFMT_UNKNOWN, D3DFMT_UYVY, D3DFMT_V16U16,
            D3DFMT_V8U8, D3DFMT_VERTEXDATA, D3DFMT_X1R5G5B5, D3DFMT_X4R4G4B4, D3DFMT_X8B8G8R8,
            D3DFMT_X8L8V8U8, D3DFMT_X8R8G8B8, D3DFMT_YUY2, D3DPOOL_DEFAULT, D3DPOOL_MANAGED,
            D3DPRESENT_PARAMETERS, D3DPT_LINELIST, D3DPT_LINESTRIP, D3DPT_POINTLIST,
            D3DPT_TRIANGLEFAN, D3DPT_TRIANGLELIST, D3DPT_TRIANGLESTRIP,
            D3DQUERYTYPE_BANDWIDTHTIMINGS, D3DQUERYTYPE_CACHEUTILIZATION, D3DQUERYTYPE_EVENT,
            D3DQUERYTYPE_INTERFACETIMINGS, D3DQUERYTYPE_MEMORYPRESSURE, D3DQUERYTYPE_OCCLUSION,
            D3DQUERYTYPE_PIPELINETIMINGS, D3DQUERYTYPE_PIXELTIMINGS, D3DQUERYTYPE_RESOURCEMANAGER,
            D3DQUERYTYPE_TIMESTAMP, D3DQUERYTYPE_TIMESTAMPDISJOINT, D3DQUERYTYPE_TIMESTAMPFREQ,
            D3DQUERYTYPE_VCACHE, D3DQUERYTYPE_VERTEXSTATS, D3DQUERYTYPE_VERTEXTIMINGS,
            D3DRTYPE_CUBETEXTURE, D3DRTYPE_INDEXBUFFER, D3DRTYPE_SURFACE, D3DRTYPE_TEXTURE,
            D3DRTYPE_VERTEXBUFFER, D3DRTYPE_VOLUME, D3DRTYPE_VOLUMETEXTURE, D3DSBT_ALL,
            D3DSBT_PIXELSTATE, D3DSBT_VERTEXSTATE, D3DSWAPEFFECT_COPY, D3DSWAPEFFECT_DISCARD,
            D3DSWAPEFFECT_FLIP, D3DSWAPEFFECT_OVERLAY, D3DVERTEXELEMENT9,
        },
        guiddef::GUID,
        windef::HWND,
    },
    um::wingdi::RGNDATA,
};

/// Represents an adapter (graphics card, integrated graphics, etc).
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

/// Represents the identifier structure for an [`Adapter`].
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

bitfield! {
    /// Flags for [`Context::create_device`].
    #[derive(Default)]
    pub struct BehaviorFlags(u32);

    pub fpu_preserve, set_fpu_preserve: 1;
    pub multithreaded, set_multithreaded: 2;
    pub pure_device, set_pure_device: 4;
    pub software_vertex_processing, set_software_vertex_processing: 5;
    pub hardware_vertex_processing, set_hardware_vertex_processing: 6;
    pub mixed_vertex_processing, set_mixed_vertex_processing: 7;
    pub disable_driver_management, set_disable_driver_management: 8;
    pub adapter_group_device, set_adapter_group_device: 9;
    pub disable_driver_management_ex, set_disable_driver_management_ex: 10;
    pub no_window_changes, set_no_window_changes: 11;

    // TODO: other flags.
}

impl Into<u32> for BehaviorFlags {
    fn into(self) -> u32 {
        self.0
    }
}

bitfield! {
    /// Flags for [`Device::clear`].
    #[derive(Default)]
    pub struct Clear(u32);

    impl Debug;

    pub target, set_target: 0;
    pub zbuffer, set_zbuffer: 1;
    pub stencil, set_stencil: 2;
}

impl Clear {
    pub fn with_target(mut self) -> Self {
        self.set_target(true);
        self
    }

    pub fn with_zbuffer(mut self) -> Self {
        self.set_zbuffer(true);
        self
    }

    pub fn with_stencil(mut self) -> Self {
        self.set_stencil(true);
        self
    }
}

impl Into<u32> for Clear {
    fn into(self) -> u32 {
        self.0
    }
}

/// Represents a color in Direct3D9.
#[repr(transparent)]
pub struct Color(pub u32);

impl Into<D3DCOLOR> for Color {
    fn into(self) -> D3DCOLOR {
        self.0
    }
}

/// Thin wrapper around [`D3DCAPS9`].
// TODO: Implement this as a rust-safe wrapper.
pub struct Caps(pub D3DCAPS9);

/// Represents the type of a [`VertexElement`].
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum DeclType {
    Float1 = D3DDECLTYPE_FLOAT1 as u8,
    Float2 = D3DDECLTYPE_FLOAT2 as u8,
    Float3 = D3DDECLTYPE_FLOAT3 as u8,
    Float4 = D3DDECLTYPE_FLOAT4 as u8,
    Color = D3DDECLTYPE_D3DCOLOR as u8,
    UByte4 = D3DDECLTYPE_UBYTE4 as u8,
    Short2 = D3DDECLTYPE_SHORT2 as u8,
    Short4 = D3DDECLTYPE_SHORT4 as u8,
    UByte4N = D3DDECLTYPE_UBYTE4N as u8,
    Short2N = D3DDECLTYPE_SHORT2N as u8,
    Short4N = D3DDECLTYPE_SHORT4N as u8,
    UShort2N = D3DDECLTYPE_USHORT2N as u8,
    UShort4N = D3DDECLTYPE_USHORT4N as u8,
    UDec3 = D3DDECLTYPE_UDEC3 as u8,
    Dec3N = D3DDECLTYPE_DEC3N as u8,
    Float16By2 = D3DDECLTYPE_FLOAT16_2 as u8,
    Float16By4 = D3DDECLTYPE_FLOAT16_4 as u8,
}

/// Represents the method type of a [`VertexElement`].
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum DeclMethod {
    Default = D3DDECLMETHOD_DEFAULT as u8,
    PartialU = D3DDECLMETHOD_PARTIALU as u8,
    PartialV = D3DDECLMETHOD_PARTIALV as u8,
    CrossUV = D3DDECLMETHOD_CROSSUV as u8,
    UV = D3DDECLMETHOD_UV as u8,
    LookUp = D3DDECLMETHOD_LOOKUP as u8,
    LookUpPreSampled = D3DDECLMETHOD_LOOKUPPRESAMPLED as u8,
}

/// Represents the usage/semantic type of a [`VertexElement`].
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum DeclUsage {
    Position = D3DDECLUSAGE_POSITION as u8,
    BlendWeight = D3DDECLUSAGE_BLENDWEIGHT as u8,
    BlendIndices = D3DDECLUSAGE_BLENDINDICES as u8,
    Normal = D3DDECLUSAGE_NORMAL as u8,
    PSize = D3DDECLUSAGE_PSIZE as u8,
    TexCoord = D3DDECLUSAGE_TEXCOORD as u8,
    Tangent = D3DDECLUSAGE_TANGENT as u8,
    BiNormal = D3DDECLUSAGE_BINORMAL as u8,
    TessFactor = D3DDECLUSAGE_TESSFACTOR as u8,
    PositionT = D3DDECLUSAGE_POSITIONT as u8,
    Color = D3DDECLUSAGE_COLOR as u8,
    Fog = D3DDECLUSAGE_FOG as u8,
    Depth = D3DDECLUSAGE_DEPTH as u8,
    Sample = D3DDECLUSAGE_SAMPLE as u8,
}

/// Represents the type of device.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum DeviceType {
    Hal = D3DDEVTYPE_HAL,
    NullRef = D3DDEVTYPE_NULLREF,
    Ref = D3DDEVTYPE_REF,
    Sw = D3DDEVTYPE_SW,
}

/// Represents the display mode of a monitor.
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

/// Represents a buffer/surface format.
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
    Index16 = D3DFMT_INDEX16,
    Index32 = D3DFMT_INDEX32,
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

bitfield! {
    /// Represents the fixed vertex function pipeline configuration.
    pub struct FVF(u32);

    impl Debug;

    pub xyz, set_xyz: 1;
    pub xyzrhw, set_xyzrhw: 2;
    pub xyzb1, set_xyzb1: 1, 2;
    pub xyzb2, set_xyzb2: 3;
    pub xyzb3, set_xyzb3: 1, 3;
    pub xyzb4, set_xyzb4: 2, 3;
    pub xyzb5, set_xyzb5: 1, 2, 3;
    pub xyzw, set_xyzw: 1, 14;

    pub normal, set_normal: 4;
    pub psize, set_psize: 5;
    pub diffuse, set_diffuse: 6;
    pub specular, set_specular: 7;

    pub tex1, set_tex1: 8;
    pub tex2, set_tex2: 9;
    pub tex3, set_tex3: 8, 9;
    pub tex4, set_tex4: 10;
    pub tex5, set_tex5: 8, 10;
    pub tex6, set_tex6: 9, 10;
    pub tex7, set_tex7: 8, 9, 10;
    pub tex8, set_tex8: 11;
}

/// Represents a general purpose resource in Direct3D 9.
#[derive(Clone, Copy, Debug)]
pub struct Handle(pub u32);

impl Into<u32> for Handle {
    fn into(self) -> u32 {
        self.0
    }
}

/// Represents the type of multi-sampling for buffers/surfaces.
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

/// Represents a memory pool location.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Pool {
    Default = D3DPOOL_DEFAULT,
    Managed = D3DPOOL_MANAGED,
}

/// Represents the "presentation parameters" for a [`SwapChain`].
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

/// Represents the type of primitive to render.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum PrimitiveType {
    PointList = D3DPT_POINTLIST,
    LineList = D3DPT_LINELIST,
    LineStripe = D3DPT_LINESTRIP,
    TriangleList = D3DPT_TRIANGLELIST,
    TriangleStrip = D3DPT_TRIANGLESTRIP,
    TriangleFan = D3DPT_TRIANGLEFAN,
}

/// Represents the type of a [`Query`].
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum QueryType {
    VertexCache = D3DQUERYTYPE_VCACHE,
    ResourceManager = D3DQUERYTYPE_RESOURCEMANAGER,
    VertexStats = D3DQUERYTYPE_VERTEXSTATS,
    Event = D3DQUERYTYPE_EVENT,
    Occlusion = D3DQUERYTYPE_OCCLUSION,
    Timestamp = D3DQUERYTYPE_TIMESTAMP,
    TimestampDisjoint = D3DQUERYTYPE_TIMESTAMPDISJOINT,
    TimestampRefQ = D3DQUERYTYPE_TIMESTAMPFREQ,
    PipelineTimings = D3DQUERYTYPE_PIPELINETIMINGS,
    InterfaceTimings = D3DQUERYTYPE_INTERFACETIMINGS,
    VertexTimings = D3DQUERYTYPE_VERTEXTIMINGS,
    PixelTimings = D3DQUERYTYPE_PIXELTIMINGS,
    BandwidthTimings = D3DQUERYTYPE_BANDWIDTHTIMINGS,
    CacheUtilization = D3DQUERYTYPE_CACHEUTILIZATION,
    MemoryPressure = D3DQUERYTYPE_MEMORYPRESSURE,
}

/// Represents the `RECT` structure used by certain [`Device`] functions.
#[repr(C)]
pub struct Rect {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

/// Temporary wrapper for [`RGNDATA`].
pub struct RegionData(pub RGNDATA);

/// Represents the type of Direct3D resource.
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

/// Represents the type of a [`StateBlock`].
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum StateBlockType {
    All = D3DSBT_ALL,
    Pixel = D3DSBT_PIXELSTATE,
    Vertex = D3DSBT_VERTEXSTATE,
}

/// Represents the swap-effect mode of a [`Device`].
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum SwapEffect {
    Discard = D3DSWAPEFFECT_DISCARD,
    Flip = D3DSWAPEFFECT_FLIP,
    Copy = D3DSWAPEFFECT_COPY,
    Overlay = D3DSWAPEFFECT_OVERLAY,
}

bitfield! {
    /// Represents the usage type for a buffer resource.
    #[derive(Default)]
    pub struct Usage(u32);

    impl Debug;

    pub render_target, set_render_target: 0;
    pub depth_stencil, set_depth_stencil: 1;
    pub dynamic, set_dynamic: 9;
    pub non_secure, set_non_secure: 23;
    pub auto_genmipmap, set_auto_genmipmap: 10;
    pub dmap, set_dmap: 14;
    pub query_legacy_bumpmap, set_query_legacy_bumpmap: 15;
    pub query_srgbread, set_query_srgbread: 16;
    pub query_filter, set_query_filter: 17;
    pub query_srgbwrite, set_query_srgbwrite: 18;
    pub query_post_pixel_shader_blending, set_query_post_pixel_shader_blending: 19;
    pub query_vertex_texture, set_query_vertex_texture: 20;
    pub query_wrap_and_mip, set_query_wrap_and_mip: 21;
    pub write_only, set_write_only: 3;
    pub software_processing, set_software_processing: 4;
    pub do_not_clip, set_do_not_clip: 5;
    pub points, set_points: 6;
    pub rt_patches, set_rt_patches: 7;
    pub n_patches, set_n_patches: 8;
    pub text_api, set_text_api: 28;
    pub restricted_content, set_restricted_content: 11;
    pub restrict_shared_resource, set_restrict_shared_resource: 13;
    pub restrict_shared_resource_driver, set_restrict_shared_resource_driver: 12;
}

/// Represents the layout and size of a vertex within the pipeline.
#[derive(Clone, Copy, Debug)]
pub struct VertexElement {
    pub stream: u16,
    pub offset: u16,
    pub kind: DeclType,
    pub method: DeclMethod,
    pub usage: DeclUsage,
    pub usage_index: u8,
}

impl Into<D3DVERTEXELEMENT9> for VertexElement {
    fn into(self) -> D3DVERTEXELEMENT9 {
        D3DVERTEXELEMENT9 {
            Stream: self.stream,
            Offset: self.offset,
            Type: self.kind as u8,
            Method: self.method as u8,
            Usage: self.usage as u8,
            UsageIndex: self.usage_index,
        }
    }
}
