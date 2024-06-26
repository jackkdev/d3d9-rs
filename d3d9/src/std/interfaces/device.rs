use std::{cmp::min, marker::PhantomData, mem::transmute, ptr, ptr::NonNull};

use winapi::{
    shared::{
        d3d9::{
            IDirect3DCubeTexture9, IDirect3DDevice9, IDirect3DIndexBuffer9, IDirect3DPixelShader9,
            IDirect3DQuery9, IDirect3DStateBlock9, IDirect3DSurface9, IDirect3DSwapChain9,
            IDirect3DTexture9, IDirect3DVertexBuffer9, IDirect3DVertexDeclaration9,
            IDirect3DVertexShader9, IDirect3DVolumeTexture9,
        },
        d3d9types::{D3DPRESENT_PARAMETERS, D3DRS_CLIPPING, D3DVERTEXELEMENT9, D3DVIEWPORT9},
        windef::HWND,
    },
    um::winnt::VOID,
};

use crate::{
    check_hresult, check_hresult_mut,
    com::Com,
    error::WindowsResult,
    std::{
        interfaces::{
            CubeTexture, IndexBuffer, PixelShader, Query, StateBlock, Surface, SwapChain, Texture,
            VertexBuffer, VertexDeclaration, VertexShader, VolumeTexture,
        },
        types::{
            Clear, Color, Format, Handle, MultiSampleType, Pool, PresentationParameters,
            PrimitiveType, QueryType, Rect, RegionData, StateBlockType, Usage, VertexElement, FVF,
        },
    },
};

#[derive(Clone)]
pub struct Device {
    inner: Com<IDirect3DDevice9>,
}

impl Device {
    pub fn with_ptr(inner: NonNull<IDirect3DDevice9>) -> Self {
        Self {
            inner: Com::with_ptr(inner),
        }
    }

    pub fn as_ptr(&self) -> *mut IDirect3DDevice9 {
        self.inner.as_ptr()
    }

    pub fn begin_scene(&self) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.BeginScene())?;
        }
        Ok(())
    }

    pub fn begin_state_block(&self) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.BeginStateBlock())?;
        }
        Ok(())
    }

    pub fn clear(
        &self,
        count: u32,
        rects: Option<&[Rect]>,
        flags: Clear,
        color: Color,
        z: f32,
        stencil: u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.Clear(
                count,
                rects
                    .map(|inner| transmute(inner.as_ptr()))
                    .unwrap_or(ptr::null()),
                flags.0,
                color.into(),
                z,
                stencil
            ))?;
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
            ))?;
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

            check_hresult_mut!(self.inner.CreateAdditionalSwapChain(
                &mut c_presentation_parameters as *mut _,
                &mut c_swap_chain as *mut _
            ))?;

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

            check_hresult_mut!(self.inner.CreateCubeTexture(
                edge_length,
                levels,
                usage,
                format as u32,
                pool as u32,
                &mut c_cube_texture as *mut _,
                ptr::null_mut()
            ))?;

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
            ))?;

            Ok(Surface::with_ptr(
                NonNull::new(c_surface).expect("returned surface is null"),
            ))
        }
    }

    pub fn create_index_buffer(
        &self,
        length: u32,
        usage: Usage,
        format: Format,
        pool: Pool,
    ) -> WindowsResult<IndexBuffer> {
        unsafe {
            let mut c_index_buffer: *mut IDirect3DIndexBuffer9 = ptr::null_mut();

            check_hresult_mut!(self.inner.CreateIndexBuffer(
                length,
                usage.0,
                format as u32,
                pool as u32,
                &mut c_index_buffer as *mut _,
                ptr::null_mut()
            ))?;

            Ok(IndexBuffer::with_ptr(
                NonNull::new(c_index_buffer).expect("returned index buffer is null"),
            ))
        }
    }

    pub fn create_offscreen_plain_surface(
        &self,
        width: u32,
        height: u32,
        format: Format,
        pool: Pool,
    ) -> WindowsResult<Surface> {
        unsafe {
            let mut c_surface: *mut IDirect3DSurface9 = ptr::null_mut();

            check_hresult_mut!(self.inner.CreateOffscreenPlainSurface(
                width,
                height,
                format as u32,
                pool as u32,
                &mut c_surface as *mut _,
                ptr::null_mut()
            ))?;

            Ok(Surface::with_ptr(
                NonNull::new(c_surface).expect("returned off screen plain surface is null"),
            ))
        }
    }

    pub fn create_pixel_shader(&self, shader: &[u32]) -> WindowsResult<PixelShader> {
        unsafe {
            let mut c_pixel_shader: *mut IDirect3DPixelShader9 = ptr::null_mut();

            check_hresult_mut!(self
                .inner
                .CreatePixelShader(shader.as_ptr(), &mut c_pixel_shader as *mut _))?;

            Ok(PixelShader::with_ptr(
                NonNull::new(c_pixel_shader).expect("returned pixel shader is null"),
            ))
        }
    }

    pub fn create_query(&self, query_type: QueryType) -> WindowsResult<Query> {
        unsafe {
            let mut c_query: *mut IDirect3DQuery9 = ptr::null_mut();

            check_hresult_mut!(self
                .inner
                .CreateQuery(query_type as u32, &mut c_query as *mut _))?;

            Ok(Query::with_ptr(
                NonNull::new(c_query).expect("returned query is null"),
            ))
        }
    }

    pub fn create_render_target(
        &self,
        width: u32,
        height: u32,
        format: Format,
        multi_sample_type: MultiSampleType,
        multi_sample_quality: u32,
        lockable: bool,
    ) -> WindowsResult<Surface> {
        unsafe {
            let mut c_surface: *mut IDirect3DSurface9 = ptr::null_mut();

            check_hresult!(self.inner.CreateRenderTarget(
                width,
                height,
                format as u32,
                multi_sample_type.into(),
                multi_sample_quality,
                lockable as i32,
                &mut c_surface as *mut _,
                ptr::null_mut()
            ))?;

            Ok(Surface::with_ptr(
                NonNull::new(c_surface).expect("returned render target surface is null"),
            ))
        }
    }

    pub fn create_state_block(
        &self,
        state_block_type: StateBlockType,
    ) -> WindowsResult<StateBlock> {
        unsafe {
            let mut c_state_block: *mut IDirect3DStateBlock9 = ptr::null_mut();

            check_hresult_mut!(self
                .inner
                .CreateStateBlock(state_block_type as u32, &mut c_state_block as *mut _))?;

            Ok(StateBlock::with_ptr(
                NonNull::new(c_state_block).expect("returned state block is null"),
            ))
        }
    }

    pub fn create_texture(
        &self,
        width: u32,
        height: u32,
        levels: u32,
        usage: Usage,
        format: Format,
        pool: Pool,
    ) -> WindowsResult<Texture> {
        unsafe {
            let mut c_texture: *mut IDirect3DTexture9 = ptr::null_mut();

            check_hresult_mut!(self.inner.CreateTexture(
                width,
                height,
                levels,
                usage.0,
                format as u32,
                pool as u32,
                &mut c_texture as *mut _,
                ptr::null_mut(),
            ))?;

            Ok(Texture::with_ptr(
                NonNull::new(c_texture).expect("returned texture is null"),
                (width, height),
            ))
        }
    }

    pub fn create_vertex_buffer(
        &self,
        length: u32,
        usage: Usage,
        fvf: FVF,
        pool: Pool,
    ) -> WindowsResult<VertexBuffer> {
        unsafe {
            let mut c_vertex_buffer: *mut IDirect3DVertexBuffer9 = ptr::null_mut();

            check_hresult_mut!(self.inner.CreateVertexBuffer(
                length,
                usage.0,
                fvf.0,
                pool as u32,
                &mut c_vertex_buffer as *mut _,
                ptr::null_mut(),
            ))?;

            Ok(VertexBuffer::with_ptr(
                NonNull::new(c_vertex_buffer).expect("returned vertex buffer is null"),
            ))
        }
    }

    pub fn create_vertex_declaration(
        &self,
        vertex_elements: &[VertexElement],
    ) -> WindowsResult<VertexDeclaration> {
        unsafe {
            let mut c_vertex_declaration: *mut IDirect3DVertexDeclaration9 = ptr::null_mut();

            let mut real_elements: Vec<D3DVERTEXELEMENT9> = vertex_elements
                .into_iter()
                .map(|element| element.clone().into())
                .collect();

            real_elements.push(D3DVERTEXELEMENT9 {
                Stream: 0xFF,
                Offset: 0,
                Type: 17,
                Method: 0,
                Usage: 0,
                UsageIndex: 0,
            });

            check_hresult_mut!(self.inner.CreateVertexDeclaration(
                real_elements.as_ptr(),
                &mut c_vertex_declaration as *mut _
            ))?;

            Ok(VertexDeclaration::with_ptr(
                NonNull::new(c_vertex_declaration).expect("returned vertex declaration is null"),
            ))
        }
    }

    pub fn create_vertex_shader(&self, shader: &[u32]) -> WindowsResult<VertexShader> {
        unsafe {
            let mut c_vertex_shader: *mut IDirect3DVertexShader9 = ptr::null_mut();

            check_hresult_mut!(self
                .inner
                .CreateVertexShader(shader.as_ptr(), &mut c_vertex_shader as *mut _))?;

            Ok(VertexShader::with_ptr(
                NonNull::new(c_vertex_shader).expect("returned vertex shader is null"),
            ))
        }
    }

    pub fn create_volume_texture(
        &self,
        width: u32,
        height: u32,
        depth: u32,
        levels: u32,
        usage: Usage,
        format: Format,
        pool: Pool,
    ) -> WindowsResult<VolumeTexture> {
        unsafe {
            let mut c_volume_texture: *mut IDirect3DVolumeTexture9 = ptr::null_mut();

            check_hresult_mut!(self.inner.CreateVolumeTexture(
                width,
                height,
                depth,
                levels,
                usage.0,
                format as u32,
                pool as u32,
                &mut c_volume_texture as *mut _,
                ptr::null_mut()
            ))?;

            Ok(VolumeTexture::with_ptr(
                NonNull::new(c_volume_texture).expect("returned volume texture is null"),
            ))
        }
    }

    pub fn delete_patch(&self, handle: Handle) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.DeletePatch(handle.into()))?;
        }

        Ok(())
    }

    pub fn draw_indexed_primitive(
        &self,
        primitive_type: PrimitiveType,
        base_vertex_index: i32,
        minimum_vertex_index: u32,
        n_vertices: u32,
        indices_index: u32,
        n_primitives: u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.DrawIndexedPrimitive(
                primitive_type as u32,
                base_vertex_index,
                minimum_vertex_index,
                n_vertices,
                indices_index,
                n_primitives
            ))?;
        }

        Ok(())
    }

    pub fn draw_indexed_primitive_up<V, I>(
        &self,
        primitive_type: PrimitiveType,
        minimum_vertex_index: u32,
        n_primitives: u32,
        indices: Vec<I>,
        indices_format: Format,
        vertices: Vec<V>,
        vertices_stride: u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.DrawIndexedPrimitiveUP(
                primitive_type as u32,
                minimum_vertex_index,
                vertices.len() as u32,
                n_primitives,
                indices.as_ptr() as *const VOID,
                indices_format as u32,
                vertices.as_ptr() as *const VOID,
                vertices_stride
            ))?;
        }

        Ok(())
    }

    pub fn draw_primitive(
        &self,
        primitive_type: PrimitiveType,
        start_vertex: u32,
        n_primitives: u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.DrawPrimitive(
                primitive_type as u32,
                start_vertex,
                n_primitives
            ))?;
        }

        Ok(())
    }

    pub fn draw_primitive_up<V>(
        &self,
        primitive_type: PrimitiveType,
        n_primitives: u32,
        vertices: Vec<V>,
        vertices_stride: u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.DrawPrimitiveUP(
                primitive_type as u32,
                n_primitives,
                vertices.as_ptr() as *const VOID,
                vertices_stride
            ))?;
        }

        Ok(())
    }

    // DrawRectPatch
    // DrawTriPatch

    pub fn end_scene(&self) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.EndScene())?;
        }

        Ok(())
    }

    pub fn end_state_block(&self) -> WindowsResult<StateBlock> {
        unsafe {
            let mut c_state_block: *mut IDirect3DStateBlock9 = ptr::null_mut();

            check_hresult_mut!(self.inner.EndStateBlock(&mut c_state_block as *mut _))?;

            Ok(StateBlock::with_ptr(
                NonNull::new(c_state_block).expect("returned state block is null"),
            ))
        }
    }

    // ...

    pub fn present(
        &self,
        src_rect: Option<&Rect>,
        dest_rect: Option<&Rect>,
        window_override: Option<HWND>,
        dirty_region_data: Option<&RegionData>,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.Present(
                src_rect
                    .map(|inner| inner as *const _ as *const _)
                    .unwrap_or(ptr::null()),
                dest_rect
                    .map(|inner| inner as *const _ as *const _)
                    .unwrap_or(ptr::null()),
                window_override.unwrap_or(ptr::null_mut()),
                dirty_region_data
                    .map(|inner| &inner.0 as *const _)
                    .unwrap_or(ptr::null())
            ))?;
        }

        Ok(())
    }

    pub fn set_vertex_declaration(
        &self,
        vertex_declaration: &VertexDeclaration,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetVertexDeclaration(vertex_declaration.as_ptr()))?;
        }

        Ok(())
    }

    pub fn set_indices(&self, index_buffer: &IndexBuffer) -> WindowsResult<()> {
        unsafe { check_hresult!(self.inner.SetIndices(index_buffer.as_ptr())) }
    }

    pub fn set_vertex_shader(&self, vertex_shader: &VertexShader) -> WindowsResult<()> {
        unsafe { check_hresult!(self.inner.SetVertexShader(vertex_shader.as_ptr())) }
    }

    pub fn set_pixel_shader(&self, pixel_shader: &PixelShader) -> WindowsResult<()> {
        unsafe { check_hresult!(self.inner.SetPixelShader(pixel_shader.as_ptr())) }
    }

    pub fn set_stream_source(
        &self,
        index: u32,
        vertex_buffer: &VertexBuffer,
        byte_offset: u32,
        stride: u32,
    ) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetStreamSource(
                index,
                vertex_buffer.as_ptr(),
                byte_offset,
                stride
            ))?;
        }

        Ok(())
    }

    pub fn set_viewport(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        max_z: f32,
        min_z: f32,
    ) -> WindowsResult<()> {
        unsafe {
            let viewport = D3DVIEWPORT9 {
                X: x,
                Y: y,
                Width: width,
                Height: height,
                MaxZ: max_z,
                MinZ: min_z,
            };
            check_hresult!(self.inner.SetViewport(&viewport as *const _))?;
        }

        Ok(())
    }

    pub fn set_clipping(&self, value: bool) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetRenderState(D3DRS_CLIPPING, value as u32))?;
        }

        Ok(())
    }

    pub fn set_texture(&self, stage: u32, texture: &Texture) -> WindowsResult<()> {
        unsafe {
            check_hresult!(self.inner.SetTexture(stage, texture.as_ptr() as *mut _))?;
        }

        Ok(())
    }
}
