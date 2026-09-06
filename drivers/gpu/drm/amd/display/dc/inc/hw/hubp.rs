//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/hubp.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//
// DOC: overview
//
// Display Controller Hub (DCHUB) is the gateway between the Scalable Data Port
// (SDP) and DCN. This component has multiple features, such as memory
// arbitration, rotation, and cursor manipulation.
//
// There is one HUBP allocated per pipe, which fetches data and converts
// different pixel formats (i.e. ARGB8888, NV12, etc) into linear, interleaved
// and fixed-depth streams of pixel data.
//

pub const OPP_ID_INVALID: c_uint = 0xf;
pub const MAX_TTU: c_uint = 0xffffff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cursor_pitch {
    CURSOR_PITCH_64_PIXELS = 0,
    CURSOR_PITCH_128_PIXELS,
    CURSOR_PITCH_256_PIXELS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cursor_lines_per_chunk {
    CURSOR_LINE_PER_CHUNK_1 = 0, /* new for DCN2 */
    CURSOR_LINE_PER_CHUNK_2 = 1,
    CURSOR_LINE_PER_CHUNK_4,
    CURSOR_LINE_PER_CHUNK_8,
    CURSOR_LINE_PER_CHUNK_16
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hubp_ind_block_size {
    hubp_ind_block_unconstrained = 0,
    hubp_ind_block_64b,
    hubp_ind_block_128b,
    hubp_ind_block_64b_no_128bcl,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hubp_3dlut_fl_mode {
    hubp_3dlut_fl_mode_disable = 0,
    hubp_3dlut_fl_mode_native_1 = 1,
    hubp_3dlut_fl_mode_native_2 = 2,
    hubp_3dlut_fl_mode_transform = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hubp_3dlut_fl_format {
    hubp_3dlut_fl_format_unorm_12msb_bitslice = 0,
    hubp_3dlut_fl_format_unorm_12lsb_bitslice = 1,
    hubp_3dlut_fl_format_float_fp1_5_10 = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hubp_3dlut_fl_addressing_mode {
    hubp_3dlut_fl_addressing_mode_sw_linear = 0,
    hubp_3dlut_fl_addressing_mode_simple_linear = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hubp_3dlut_fl_width {
    hubp_3dlut_fl_width_17 = 17,
    hubp_3dlut_fl_width_33 = 33,
    hubp_3dlut_fl_width_17_transformed    = 4916, //mpc default
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hubp_3dlut_fl_crossbar_bit_slice {
    hubp_3dlut_fl_crossbar_bit_slice_0_15 = 0,
    hubp_3dlut_fl_crossbar_bit_slice_16_31 = 1,
    hubp_3dlut_fl_crossbar_bit_slice_32_47 = 2,
    hubp_3dlut_fl_crossbar_bit_slice_48_63 = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp {
    pub funcs: *const hubp_funcs,
    pub ctx: *mut dc_context,
    pub request_address: dc_plane_address,
    pub inst: c_int,
// run time states
    pub opp_id: c_int,
    pub mpcc_id: c_int,
    pub curs_attr: dc_cursor_attributes,
    pub curs_pos: dc_cursor_position,
    pub cursor_offload: bool,
    pub power_gated: bool,
    pub pos: cursor_position_cache_hubp,
    pub att: cursor_attribute_cache_hubp,
    pub cur_rect: cursor_rect,
    pub use_mall_for_cursor: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface_flip_registers {
    pub DCSURF_SURFACE_CONTROL: u32,
    pub DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH: u32,
    pub DCSURF_PRIMARY_META_SURFACE_ADDRESS: u32,
    pub DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH: u32,
    pub DCSURF_PRIMARY_SURFACE_ADDRESS: u32,
    pub DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH_C: u32,
    pub DCSURF_PRIMARY_META_SURFACE_ADDRESS_C: u32,
    pub DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C: u32,
    pub DCSURF_PRIMARY_SURFACE_ADDRESS_C: u32,
    pub DCSURF_SECONDARY_META_SURFACE_ADDRESS_HIGH: u32,
    pub DCSURF_SECONDARY_META_SURFACE_ADDRESS: u32,
    pub DCSURF_SECONDARY_SURFACE_ADDRESS_HIGH: u32,
    pub DCSURF_SECONDARY_SURFACE_ADDRESS: u32,
    pub tmz_surface: u8,
    pub immediate: bool,
    pub vmid: u8,
    pub grph_stereo: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_funcs {
    pub pipe_dest): *mut _vcs_dpi_display_pipe_dest_params_st,
    pub timing): *mut dc_crtc_timing,
    pub ttu_regs): *mut _vcs_dpi_display_ttu_regs_st,
    pub pipe_regs): *mut dml2_dchub_per_pipe_register_set,
    pub blk_size): hubp_ind_block_size,
    pub hubp): *mut *mut void (hubp_reset)(struct hubp,
    pub viewport_c): *const rect,
    pub flip_immediate): bool,
    pub rotation): dc_rotation_angle,
    pub apt): *mut vm_system_aperture_param,
    pub vm0): *const vm_context0_param,
    pub compa_level): c_uint,
    pub hubp): *mut *mut bool (hubp_is_flip_pending)(struct hubp,
    pub blank): *mut *mut *mut void (set_blank)(struct hubp hubp, bool,
    pub blank): *mut *mut *mut void (set_blank_regs)(struct hubp hubp, bool,
    pub hubp): *mut *mut void (phantom_hubp_post_enable)(struct hubp,
    pub blank): *mut *mut *mut void (set_hubp_blank_en)(struct hubp hubp, bool,
    pub attr): *const dc_cursor_attributes,
    pub param): *const dc_cursor_mi_param,
    pub hubp): *mut *mut void (hubp_disconnect)(struct hubp,
    pub enable): *mut *mut *mut void (hubp_clk_cntl)(struct hubp hubp, bool,
    pub otg_inst): *mut *mut *mut void (hubp_vtg_sel)(struct hubp hubp, uint32_t,
    pub hubp): *mut *mut void (hubp_read_state)(struct hubp,
    pub reg_state): *mut *mut *mut void (hubp_read_reg_state)(struct hubp hubp, struct dcn_hubp_reg_state,
    pub hubp): *mut *mut void (hubp_clear_underflow)(struct hubp,
    pub disable_hubp): *mut *mut *mut void (hubp_disable_control)(struct hubp hubp, bool,
    pub hubp): *mut *mut unsigned int (hubp_get_underflow_status)(struct hubp,
    pub hubp): *mut *mut void (hubp_init)(struct hubp,
    pub attr): *const dc_dmdata_attributes,
    pub dmdata_sw_data): *const u32,
    pub hubp): *mut *mut bool (dmdata_status_done)(struct hubp,
    pub enable): bool,
    pub hubp): *mut hubp,
    pub enable): bool,
    pub dml_ttu_attr): *mut _vcs_dpi_display_ttu_regs_st,
    pub enable): bool,
    pub hubp): *mut *mut bool (hubp_in_blank)(struct hubp,
    pub reset): *mut *mut *mut void (hubp_soft_reset)(struct hubp hubp, bool,
    pub hubp): *mut *mut void (hubp_set_flip_int)(struct hubp,
    pub allow): *mut *mut *mut void (hubp_update_force_pstate_disallow)(struct hubp hubp, bool,
    pub allow): *mut *mut *mut void (hubp_update_force_cursor_pstate_disallow)(struct hubp hubp, bool,
    pub c_cursor): *mut *mut *mut void (hubp_update_mall_sel)(struct hubp hubp, uint32_t mall_sel, bool,
    pub enable): *mut *mut *mut void (hubp_prepare_subvp_buffering)(struct hubp hubp, bool,
    pub lock): bool,
    pub min_dst_y_next_start_optimized): c_uint,
    pub hubp): *mut *mut void (hubp_wait_pipe_read_start)(struct hubp,
    pub mcache_regs): *mut *mut *mut void (hubp_program_mcache_id_and_split_coordinate)(struct hubp hubp, struct dml2_hubp_pipe_mcache_regs,
    pub address): *const dc_plane_address,
    pub config): *const dc_3dlut_dma,
    pub refcyc_per_3dlut_group): *mut *mut *mut void (hubp_program_3dlut_fl_dlg_param)(struct hubp hubp, int,
    pub enable): *mut *mut *mut void (hubp_enable_3dlut_fl)(struct hubp hubp, bool,
    pub format): dc_cm_lut_pixel_format,
    pub hubp): *mut *mut uint32_t (hubp_get_3dlut_fl_done)(struct hubp,
    pub hubp): *mut *mut void (hubp_clear_tiling)(struct hubp,
    pub hubp): *mut *mut uint32_t (hubp_get_current_read_line)(struct hubp,
    pub hubp): *mut *mut uint32_t (hubp_get_det_config_error)(struct hubp,
}
