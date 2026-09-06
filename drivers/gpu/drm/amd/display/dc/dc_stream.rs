//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_stream.h
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
// Copyright 2012-14 Advanced Micro Devices, Inc.
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

//
// Stream Interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing_sync_info {
    pub group_id: c_int,
    pub group_size: c_int,
    pub master: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mall_stream_config {
// MALL stream config to indicate if the stream is phantom or not.
// We will use a phantom stream to indicate that the pipe is phantom.
//
    pub type: mall_stream_type,
    pub stream: *mut *mut dc_stream_state paired_stream; // master / slave,
    pub /: *mut *mut bool subvp_limit_cursor_size; / stream has/is using subvp limiting hw cursor support,
    pub /: *mut *mut bool cursor_size_limit_subvp; / stream is using hw cursor config preventing subvp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_stream_status {
    pub primary_otg_inst: c_int,
    pub stream_enc_inst: c_int,
//
// @plane_count: Total of planes attached to a single stream
//
    pub plane_count: c_int,
    pub audio_inst: c_int,
    pub timing_sync_info: timing_sync_info,
    pub plane_states: [*mut dc_plane_state; MAX_SURFACES],
    pub is_abm_supported: bool,
    pub mall_stream_config: mall_stream_config,
    pub fpo_in_use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_writeback_info {
    pub wb_enabled: bool,
    pub dwb_pipe_inst: c_int,
    pub dwb_params: dc_dwb_params,
    pub mcif_buf_params: mcif_buf_params,
    pub mcif_warmup_params: mcif_warmup_params,
// the plane that is the input to TOP_MUX for MPCC that is the DWB source
    pub writeback_source_plane: *mut dc_plane_state,
// source MPCC instance.  for use by internally by dc
    pub mpcc_inst: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_writeback_update {
    pub num_wb_info: c_uint,
    pub writeback_info: [dc_writeback_info; MAX_DWB_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vertical_interrupt_ref_point {
    START_V_UPDATE = 0,
    START_V_SYNC,
    INVALID_POINT

// For now, only v_update interrupt is used.
// START_V_BLANK,
// START_V_ACTIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct periodic_interrupt_config {
    pub ref_point: vertical_interrupt_ref_point,
    pub lines_offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_mst_stream_bw_update {
    pub increased: bool is_increase; // is bandwidth reduced or,
    pub kbps: uint32_t mst_stream_bw; // new mst bandwidth in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union stream_update_flags {
    pub scaling:1: u32,
    pub out_tf:1: u32,
    pub out_csc:1: u32,
    pub abm_level:1: u32,
    pub dpms_off:1: u32,
    pub gamut_remap:1: u32,
    pub wb_update:1: u32,
    pub 1: uint32_t dsc_changed :,
    pub 1: uint32_t mst_bw :,
    pub 1: uint32_t crtc_timing_adjust :,
    pub 1: uint32_t fams_changed :,
    pub 1: uint32_t scaler_sharpener :,
    pub 1: uint32_t sharpening_required :,
    pub 1: uint32_t cursor_attr :,
    pub 1: uint32_t cursor_pos :,
    pub 1: uint32_t periodic_interrupt :,
    pub 1: uint32_t info_frame :,
    pub 1: uint32_t dmdata :,
    pub 1: uint32_t dither :,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_pattern {
    pub type: dp_test_pattern,
    pub color_space: dp_test_pattern_color_space,
    pub p_link_settings: *const link_training_settings,
    pub p_custom_pattern: *const c_uchar,
    pub cust_pattern_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_stream_debug_options {
    pub force_odm_combine_segments: u8,
//
// When force_odm_combine_segments is non zero, allow dc to
// temporarily transition to ODM bypass when minimal transition state
// is required to prevent visual glitches showing on the screen
//
    pub allow_transition_for_forced_odm: u8,
}

pub const LUMINANCE_DATA_TABLE_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luminance_data {
    pub is_valid: bool,
    pub refresh_rate_hz: [c_int; LUMINANCE_DATA_TABLE_SIZE],
    pub luminance_millinits: [c_int; LUMINANCE_DATA_TABLE_SIZE],
    pub flicker_criteria_milli_nits_GAMING: c_int,
    pub flicker_criteria_milli_nits_STATIC: c_int,
    pub nominal_refresh_rate: c_uint,
    pub dm_max_decrease_from_nominal: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_drr_trigger_mode {
    DRR_TRIGGER_ON_FLIP = 0,
    DRR_TRIGGER_ON_FLIP_AND_CURSOR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_stream_state {
// sink is deprecated, new code should not reference
// this pointer
    pub sink: *mut dc_sink,
    pub link: *mut dc_link,
// For dynamic link encoder assignment, update the link encoder assigned to
// a stream via the volatile dc_state rather than the static dc_link.
//
    pub link_enc: *mut link_encoder,
    pub debug: dc_stream_debug_options,
    pub sink_patches: dc_panel_patch,
    pub timing: dc_crtc_timing,
    pub adjust: dc_crtc_timing_adjust,
    pub vrr_infopacket: dc_info_packet,
    pub vsc_infopacket: dc_info_packet,
    pub vsp_infopacket: dc_info_packet,
    pub hfvsif_infopacket: dc_info_packet,
    pub vtem_infopacket: dc_info_packet,
    pub adaptive_sync_infopacket: dc_info_packet,
    pub avi_infopacket: dc_info_packet,
    pub dsc_packed_pps: [u8; 128],
    pub /: *mut *mut rect src; / composition area,
    pub /: *mut *mut rect dst; / stream addressable area,
    pub audio_info: audio_info,
    pub hdr_static_metadata: dc_info_packet,
    pub dmdata_address: PHYSICAL_ADDRESS_LOC,
    pub use_dynamic_meta: bool,
    pub out_transfer_func: dc_transfer_func,
    pub gamut_remap_matrix: colorspace_transform,
    pub csc_color_matrix: dc_csc_transform,
    pub output_color_space: dc_color_space,
    pub content_type: display_content_type,
    pub dither_option: dc_dither_option,
    pub view_format: view_3d_format,
    pub use_vsc_sdp_for_colorimetry: bool,
    pub ignore_msa_timing_param: bool,
//
// @allow_freesync:
//
// It say if Freesync is enabled or not.
//
    pub allow_freesync: bool,
//
// @vrr_active_variable:
//
// It describes if VRR is in use.
//
    pub vrr_active_variable: bool,
    pub freesync_on_desktop: bool,
    pub vrr_active_fixed: bool,
    pub converter_disable_audio: bool,
    pub qs_bit: u8,
    pub qy_bit: u8,
// TODO: custom INFO packets
// TODO: ABM info (DMCU)
// TODO: CEA VIC
// DMCU info
    pub abm_level: c_uint,
    pub periodic_interrupt: periodic_interrupt_config,
// from core_stream struct
    pub ctx: *mut dc_context,
// used by DCP and FMT
    pub bit_depth_params: bit_depth_reduction_params,
    pub clamping: clamping_and_pixel_encoding_params,
    pub phy_pix_clk: c_int,
    pub signal: signal_type,
    pub dpms_off: bool,
    pub dm_stream_context: *mut c_void,
    pub cursor_attributes: dc_cursor_attributes,
    pub cursor_position: dc_cursor_position,
    pub hw_cursor_req: bool,
    pub mode: uint32_t sdr_white_level; // for boosting (SDR) cursor in HDR,
// from stream struct
    pub refcount: kref,
    pub triggered_crtc_reset: crtc_trigger_info,
// writeback
    pub num_wb_info: c_uint,
    pub writeback_info: [dc_writeback_info; MAX_DWB_PIPES],
    pub func_shaper: *const dc_transfer_func,
    pub lut3d_func: *const dc_3dlut,
// Computed state bits
    pub 1: bool mode_changed :,
// Output from DC when stream state is committed or altered
// DC may only access these values during:
// dc_commit_state, dc_commit_state_no_check, dc_commit_streams
// values may not change outside of those calls
//
// For interrupt management, some hardware instance
// offsets need to be exposed to DM
    pub otg_offset: u8,
    pub out: },
    pub apply_edp_fast_boot_optimization: bool,
    pub apply_seamless_boot_optimization: bool,
    pub apply_boot_odm_mode: u32,
    pub stream_id: u32,
    pub test_pattern: test_pattern,
    pub update_flags: stream_update_flags,
    pub has_non_synchronizable_pclk: bool,
    pub vblank_synchronized: bool,
    pub is_phantom: bool,
    pub lumin_data: luminance_data,
    pub scaler_sharpener_update: bool,
    pub sharpening_required: bool,
    pub drr_trigger_mode: dc_drr_trigger_mode,
    pub blending_linearity: dc_blending_linearity,
    pub update_scratch: *mut dc_update_scratch_space,
    pub firmware_controlled_hdr_info_packet: bool,
}

pub const ABM_LEVEL_IMMEDIATE_DISABLE: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_stream_update {
    pub stream: *mut dc_stream_state,
    pub src: rect,
    pub dst: rect,
    pub out_transfer_func: *mut dc_transfer_func,
    pub hdr_static_metadata: *mut dc_info_packet,
    pub abm_level: *mut c_uint,
    pub periodic_interrupt: *mut periodic_interrupt_config,
    pub vrr_infopacket: *mut dc_info_packet,
    pub vsc_infopacket: *mut dc_info_packet,
    pub vsp_infopacket: *mut dc_info_packet,
    pub hfvsif_infopacket: *mut dc_info_packet,
    pub vtem_infopacket: *mut dc_info_packet,
    pub adaptive_sync_infopacket: *mut dc_info_packet,
    pub avi_infopacket: *mut dc_info_packet,
    pub dpms_off: *mut bool,
    pub integer_scaling_update: bool,
    pub allow_freesync: *mut bool,
    pub vrr_active_variable: *mut bool,
    pub vrr_active_fixed: *mut bool,
    pub gamut_remap: *mut colorspace_transform,
    pub output_color_space: *mut dc_color_space,
    pub dither_option: *mut dc_dither_option,
    pub output_csc_transform: *mut dc_csc_transform,
    pub wb_update: *mut dc_writeback_update,
    pub dsc_config: *mut dc_dsc_config,
    pub mst_bw_update: *mut dc_mst_stream_bw_update,
    pub func_shaper: *mut dc_transfer_func,
    pub lut3d_func: *mut dc_3dlut,
    pub pending_test_pattern: *mut test_pattern,
    pub crtc_timing_adjust: *mut dc_crtc_timing_adjust,
    pub cursor_attributes: *mut dc_cursor_attributes,
    pub cursor_position: *mut dc_cursor_position,
    pub hw_cursor_req: *mut bool,
    pub scaler_sharpener_update: *mut bool,
    pub sharpening_required: *mut bool,
    pub blending_linearity: *mut dc_blending_linearity,
    pub drr_trigger_mode: *mut dc_drr_trigger_mode,
}

//
// Setup stream attributes if no stream updates are provided
// there will be no impact on the stream parameters
//
// Set up surface attributes and associate to a stream
// The surfaces parameter is an absolute set of all surface active for the stream.
// If no surfaces are provided, the stream will be blanked; no memory read.
// Any flip related attribute changes must be done through this interface.
//
// After this call:
// Surfaces attributes are programmed and configured to be composed into stream.
// This does not trigger a flip.  No surface address is programmed.
//
// Set up surface attributes and associate to a stream
// The surfaces parameter is an absolute set of all surface active for the stream.
// If no surfaces are provided, the stream will be blanked; no memory read.
// Any flip related attribute changes must be done through this interface.
//
// After this call:
// Surfaces attributes are programmed and configured to be composed into stream.
// This does not trigger a flip.  No surface address is programmed.
//
// Log the current stream state.
//
extern "C" {
    pub fn dc_stream_log(dc: *const dc, stream: *const dc_stream_state);
}
extern "C" {
    pub fn dc_get_current_stream_count(dc: *mut dc) -> u8;
}
//
// Return the current frame counter.
//
extern "C" {
    pub fn dc_stream_get_vblank_counter(stream: *const dc_stream_state) -> u32;
}
//
// Send dp sdp message.
//
// TODO: Return parsed values rather than direct register read
// This has a dependency on the caller (amdgpu_display_get_crtc_scanoutpos)
// being refactored properly to be dce-specific
//
extern "C" {
    pub fn dc_stream_dmdata_status_done(dc: *mut dc, stream: *mut dc_stream_state) -> bool;
}
extern "C" {
    pub fn dc_validate_stream(dc: *mut dc, stream: *mut dc_stream_state) -> dc_status;
}
//
// Enable stereo when commit_streams is not required,
// for example, frame alternate.
//
// Triggers multi-stream synchronization.
extern "C" {
    pub fn dc_trigger_sync(dc: *mut dc, context: *mut dc_state);
}
// Shim: packs args into dc_state_update and calls dc_check_state_update().
//
// Create a new default stream for the requested sink
//
extern "C" {
    pub fn update_stream_signal(stream: *mut dc_stream_state, sink: *mut dc_sink);
}
extern "C" {
    pub fn dc_stream_retain(dc_stream: *mut dc_stream_state);
}
extern "C" {
    pub fn dc_stream_release(dc_stream: *mut dc_stream_state);
}
//
// Cursor interfaces - To manages the cursor within a stream
//
// TODO: Deprecated once we switch to dc_set_cursor_position

extern "C" {
    pub fn dc_stream_init_rmcm_3dlut(dc: *mut dc);
}
extern "C" {
    pub fn dc_stream_is_cursor_limit_pending(dc: *mut dc, stream: *mut dc_stream_state) -> bool;
}
extern "C" {
    pub fn dc_stream_can_clear_cursor_limit(dc: *mut dc, stream: *mut dc_stream_state) -> bool;
}
