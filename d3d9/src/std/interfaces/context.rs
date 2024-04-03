use std::{
    mem::{transmute, MaybeUninit},
    ptr,
    ptr::NonNull,
};

use winapi::shared::{
    d3d9::{Direct3DCreate9, IDirect3D9, IDirect3DDevice9, D3D_SDK_VERSION},
    d3d9caps::D3DCAPS9,
    d3d9types::{D3DADAPTER_IDENTIFIER9, D3DDISPLAYMODE, D3DPRESENT_PARAMETERS},
    windef::{HMONITOR, HWND},
};

use crate::{
    check_hresult, check_hresult_mut,
    com::Com,
    error::WindowsResult,
    std::{
        interfaces::device::Device,
        types::{
            Adapter, AdapterIdentifier, BehaviorFlags, Caps, DeviceType, DisplayMode, Format,
            MultiSampleType, PresentationParameters, ResourceType, Usage,
        },
    },
};

/// A safe wrapper around the [`IDirect3D9`] interface.
pub struct Context {
    inner: Com<IDirect3D9>,
}

impl Context {
    /// Returns a new [`Context`] instance.
    pub fn new() -> WindowsResult<Self> {
        Ok(Self {
            inner: unsafe {
                Com::with_ptr(
                    NonNull::new(Direct3DCreate9(D3D_SDK_VERSION))
                        .expect("pointer returned by Direct3DCreate9 is null"),
                )
            },
        })
    }

    pub fn check_depth_stencil_match(
        &self,
        adapter: Adapter,
        device_type: DeviceType,
        adapter_format: Format,
        render_target_format: Format,
        depth_stencil_format: Format,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.CheckDepthStencilMatch(
                adapter.into(),
                device_type as u32,
                adapter_format as u32,
                render_target_format as u32,
                depth_stencil_format as u32,
            ))?;
        }

        Ok(())
    }

    pub fn check_device_format(
        &self,
        adapter: Adapter,
        device_type: DeviceType,
        adapter_format: Format,
        usage: Usage,
        resource_type: ResourceType,
        check_format: Format,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.CheckDeviceFormat(
                adapter.into(),
                device_type as u32,
                adapter_format as u32,
                usage.0,
                resource_type as u32,
                check_format as u32,
            ))?;
        }

        Ok(())
    }

    pub fn check_device_format_conversion(
        &self,
        adapter: Adapter,
        device_type: DeviceType,
        source_format: Format,
        target_format: Format,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.CheckDeviceFormatConversion(
                adapter.into(),
                device_type as u32,
                source_format as u32,
                target_format as u32,
            ))?;
        }

        Ok(())
    }

    pub fn check_device_multi_sample_type(
        &self,
        adapter: Adapter,
        device_type: DeviceType,
        surface_format: Format,
        windowed: bool,
        multi_sample_type: MultiSampleType,
        quality_levels: &mut u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.CheckDeviceMultiSampleType(
                adapter.into(),
                device_type as u32,
                surface_format as u32,
                windowed as i32,
                multi_sample_type.into(),
                quality_levels,
            ))?;
        }

        Ok(())
    }

    pub fn check_device_type(
        &self,
        adapter: Adapter,
        device_type: DeviceType,
        adapter_format: Format,
        back_buffer_format: Format,
        windowed: bool,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.CheckDeviceType(
                adapter.into(),
                device_type as u32,
                adapter_format as u32,
                back_buffer_format as u32,
                windowed as i32,
            ))?;
        }

        Ok(())
    }

    pub fn create_device(
        &self,
        adapter: Adapter,
        device_type: DeviceType,
        window: HWND,
        behavior_flags: BehaviorFlags,
        presentation_parameters: &mut PresentationParameters,
    ) -> WindowsResult<Device> {
        let device = unsafe {
            let mut c_presentation_parameters: D3DPRESENT_PARAMETERS =
                presentation_parameters.clone().into();
            let mut c_device: *mut IDirect3DDevice9 = ptr::null_mut();

            check_hresult!(self.inner.CreateDevice(
                adapter.into(),
                device_type as u32,
                window,
                behavior_flags.into(),
                (&mut c_presentation_parameters) as *mut _,
                (&mut c_device) as *mut _,
            ))?;

            // CreateDevice *might* modify these properties.
            presentation_parameters.back_buffer_width = c_presentation_parameters.BackBufferWidth;
            presentation_parameters.back_buffer_height = c_presentation_parameters.BackBufferHeight;
            presentation_parameters.back_buffer_count = c_presentation_parameters.BackBufferCount;
            presentation_parameters.back_buffer_format =
                transmute(c_presentation_parameters.BackBufferFormat);

            Device::with_ptr(NonNull::new(c_device).expect("returned device is null"))
        };

        Ok(device)
    }

    pub fn enum_adapter_modes(
        &self,
        adapter: Adapter,
        format: Format,
        mode: u32,
    ) -> WindowsResult<DisplayMode> {
        unsafe {
            let mut c_display_mode = D3DDISPLAYMODE {
                Width: 0,
                Height: 0,
                RefreshRate: 0,
                Format: 0,
            };

            check_hresult_mut!(self.inner.EnumAdapterModes(
                adapter.into(),
                format as u32,
                mode,
                &mut c_display_mode as *mut _,
            ))?;

            Ok(c_display_mode.into())
        }
    }

    pub fn get_adapter_count(&self) -> u32 {
        unsafe { self.inner.GetAdapterCount() }
    }

    pub fn get_adapter_display_mode(&self, adapter: Adapter) -> WindowsResult<DisplayMode> {
        unsafe {
            let mut c_display_mode = D3DDISPLAYMODE {
                Width: 0,
                Height: 0,
                RefreshRate: 0,
                Format: 0,
            };

            check_hresult_mut!(self
                .inner
                .GetAdapterDisplayMode(adapter.into(), &mut c_display_mode as *mut _))?;

            Ok(c_display_mode.into())
        }
    }

    pub fn get_adapter_identifier(
        &self,
        adapter: Adapter,
        flags: u32,
    ) -> WindowsResult<AdapterIdentifier> {
        unsafe {
            let mut adapter_identifier: D3DADAPTER_IDENTIFIER9 =
                MaybeUninit::zeroed().assume_init();

            check_hresult_mut!(self.inner.GetAdapterIdentifier(
                adapter.into(),
                flags,
                &mut adapter_identifier as *mut _,
            ))?;

            Ok(adapter_identifier.into())
        }
    }

    pub fn get_adapter_mode_count(&self, adapter: Adapter, format: Format) -> u32 {
        unsafe {
            self.inner
                .GetAdapterModeCount(adapter.into(), format as u32)
        }
    }

    pub fn get_adapter_monitor(&self, adapter: Adapter) -> HMONITOR {
        unsafe { self.inner.GetAdapterMonitor(adapter.into()) }
    }

    pub fn get_device_caps(
        &self,
        adapter: Adapter,
        device_type: DeviceType,
    ) -> WindowsResult<Caps> {
        unsafe {
            let mut caps: D3DCAPS9 = MaybeUninit::zeroed().assume_init();
            check_hresult_mut!(self.inner.GetDeviceCaps(
                adapter.into(),
                device_type as u32,
                &mut caps as *mut _
            ))?;

            Ok(Caps(caps))
        }
    }
}
