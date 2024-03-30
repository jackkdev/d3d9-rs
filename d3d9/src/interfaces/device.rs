use std::{marker::PhantomData, mem::transmute, ptr, ptr::NonNull};

use winapi::shared::{
    d3d9::{IDirect3DCubeTexture9, IDirect3DDevice9, IDirect3DSurface9, IDirect3DSwapChain9},
    d3d9types::D3DPRESENT_PARAMETERS,
};

use crate::{
    check_hresult,
    com::Com,
    error::WindowsResult,
    interfaces::{cube_texture::CubeTexture, surface::Surface, swap_chain::SwapChain},
    types::{Color, Format, MultiSampleType, Pool, PresentationParameters, Rect},
};

#[derive(Clone)]
pub struct Device<'context> {
    inner: Com<IDirect3DDevice9>,
    _lifetime: PhantomData<&'context ()>,
}

impl<'context> Device<'context> {
    pub fn with_ptr(inner: NonNull<IDirect3DDevice9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
            _lifetime: PhantomData::default(),
        }
    }

    pub fn begin_scene(&self) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.BeginScene());
        }
        Ok(())
    }

    pub fn begin_state_block(&self) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.BeginStateBlock());
        }
        Ok(())
    }

    pub fn clear(
        &self,
        count: u32,
        rects: &[Rect],
        flags: u32,
        color: Color,
        z: f32,
        stencil: u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.Clear(
                count,
                transmute(rects.as_ptr()),
                flags,
                color.into(),
                z,
                stencil
            ));
        }
        Ok(())
    }

    pub fn color_fill(
        &self,
        surface: &Surface,
        rect: Option<Rect>,
        color: Color,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.ColorFill(
                surface.as_ptr(),
                rect.as_ref()
                    .map(|rect| rect as *const _ as *mut _)
                    .unwrap_or(ptr::null_mut()),
                color.into()
            ));
        }

        Ok(())
    }

    pub fn create_additional_swap_chain(
        &self,
        presentation_parameters: &mut PresentationParameters,
    ) -> WindowsResult<SwapChain> {
        unsafe {
            let mut c_presentation_parameters: D3DPRESENT_PARAMETERS =
                presentation_parameters.clone().into();
            let mut c_swap_chain: *mut IDirect3DSwapChain9 = ptr::null_mut();

            check_hresult!(self.inner.CreateAdditionalSwapChain(
                &mut c_presentation_parameters as *mut _,
                &mut c_swap_chain as *mut _
            ));

            // CreateAdditionalSwapChain *might* modify these properties.
            presentation_parameters.back_buffer_width = c_presentation_parameters.BackBufferWidth;
            presentation_parameters.back_buffer_height = c_presentation_parameters.BackBufferHeight;
            presentation_parameters.back_buffer_count = c_presentation_parameters.BackBufferCount;
            presentation_parameters.back_buffer_format =
                transmute(c_presentation_parameters.BackBufferFormat);

            Ok(SwapChain::with_ptr(
                NonNull::new(c_swap_chain).expect("returned swap chain is null"),
            ))
        }
    }

    pub fn create_cube_texture(
        &self,
        edge_length: u32,
        levels: u32,
        usage: u32,
        format: Format,
        pool: Pool,
    ) -> WindowsResult<CubeTexture> {
        unsafe {
            let mut c_cube_texture: *mut IDirect3DCubeTexture9 = ptr::null_mut();

            check_hresult!(self.inner.CreateCubeTexture(
                edge_length,
                levels,
                usage,
                format as u32,
                pool as u32,
                &mut c_cube_texture as *mut _,
                ptr::null_mut()
            ));

            Ok(CubeTexture::with_ptr(
                NonNull::new(c_cube_texture).expect("returned cube texture is null"),
            ))
        }
    }

    pub fn create_depth_stencil_surface(
        &self,
        width: u32,
        height: u32,
        format: Format,
        multi_sample_type: MultiSampleType,
        multi_sample_quality: u32,
        discard: bool,
    ) -> WindowsResult<Surface> {
        unsafe {
            let mut c_surface: *mut IDirect3DSurface9 = ptr::null_mut();

            check_hresult!(self.inner.CreateDepthStencilSurface(
                width,
                height,
                format as u32,
                multi_sample_type.into(),
                multi_sample_quality,
                discard as i32,
                &mut c_surface as *mut _,
                ptr::null_mut()
            ));

            Ok(Surface::with_ptr(
                NonNull::new(c_surface).expect("returned surface is null"),
            ))
        }
    }
}
