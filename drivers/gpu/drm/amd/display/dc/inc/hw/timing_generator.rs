//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/timing_generator.h
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
// Copyright 2012-2026 Advanced Micro Devices, Inc.
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

// Contains CRTC vertical/horizontal pixel counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crtc_position {
    pub vertical_count: u32,
    pub horizontal_count: u32,
    pub nominal_vcount: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_gsl_params {
    pub gsl_group: c_int,
    pub gsl_master: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_params {
    pub gsl0_en: c_int,
    pub gsl1_en: c_int,
    pub gsl2_en: c_int,
    pub gsl_master_en: c_int,
    pub gsl_master_mode: c_int,
    pub master_update_lock_gsl_en: c_int,
    pub gsl_window_start_x: c_int,
    pub gsl_window_end_x: c_int,
    pub gsl_window_start_y: c_int,
    pub gsl_window_end_y: c_int,
}

// define the structure of Dynamic Refresh Mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drr_params {
    pub vertical_total_min: u32,
    pub vertical_total_max: u32,
    pub vertical_total_mid: u32,
    pub vertical_total_mid_frame_num: u32,
    pub immediate_flip: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct long_vtotal_params {
    pub vertical_total_min: u32,
    pub vertical_total_max: u32,
    pub vertical_blank_start: u32,
}

pub const LEFT_EYE_3D_PRIMARY_SURFACE: c_int = 1;
pub const RIGHT_EYE_3D_PRIMARY_SURFACE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum crtc_state {
    CRTC_STATE_VBLANK = 0,
    CRTC_STATE_VACTIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vupdate_keepout_params {
    pub start_offset: c_int,
    pub end_offset: c_int,
    pub enable: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crtc_stereo_flags {
    pub 1: uint8_t PROGRAM_STEREO :,
    pub 1: uint8_t PROGRAM_POLARITY :,
    pub 1: uint8_t RIGHT_EYE_POLARITY :,
    pub 1: uint8_t FRAME_PACKED :,
    pub 1: uint8_t DISABLE_STEREO_DP_SYNC :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum crc_selection {
// Order must match values expected by hardware
    UNION_WINDOW_A_B = 0,
    UNION_WINDOW_A_NOT_B,
    UNION_WINDOW_NOT_A_B,
    UNION_WINDOW_NOT_A_NOT_B,
    INTERSECT_WINDOW_A_B,
    INTERSECT_WINDOW_A_NOT_B,
    INTERSECT_WINDOW_NOT_A_B,
    INTERSECT_WINDOW_NOT_A_NOT_B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otg_out_mux_dest {
    OUT_MUX_DIO = 0,
    OUT_MUX_HPO_FRL = 1,
    OUT_MUX_HPO_DP = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum h_timing_div_mode {
    H_TIMING_NO_DIV,
    H_TIMING_DIV_BY2,
    H_TIMING_RESERVED,
    H_TIMING_DIV_BY4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum timing_synchronization_type {
    NOT_SYNCHRONIZABLE,
    TIMING_SYNCHRONIZABLE,
    VBLANK_SYNCHRONIZABLE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum crc_poly_mode {
    CRC_POLY_MODE_16,
    CRC_POLY_MODE_32,
    CRC_POLY_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_params {
// Regions used to calculate CRC
    pub windowa_x_start: u16,
    pub windowa_x_end: u16,
    pub windowa_y_start: u16,
    pub windowa_y_end: u16,
    pub windowb_x_start: u16,
    pub windowb_x_end: u16,
    pub windowb_y_start: u16,
    pub windowb_y_end: u16,
    pub selection: crc_selection,
    pub dsc_mode: u8,
    pub odm_mode: u8,
    pub continuous_mode: bool,
    pub enable: bool,
    pub crc_eng_inst: u8,
    pub reset: bool,
    pub crc_poly_mode: crc_poly_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_otg_state {
    pub v_blank_start: u32,
    pub v_blank_end: u32,
    pub v_sync_a_pol: u32,
    pub v_total: u32,
    pub v_total_max: u32,
    pub v_total_min: u32,
    pub v_total_min_sel: u32,
    pub v_total_max_sel: u32,
    pub v_sync_a_start: u32,
    pub v_sync_a_end: u32,
    pub h_blank_start: u32,
    pub h_blank_end: u32,
    pub h_sync_a_start: u32,
    pub h_sync_a_end: u32,
    pub h_sync_a_pol: u32,
    pub h_total: u32,
    pub underflow_occurred_status: u32,
    pub otg_enabled: u32,
    pub blank_enabled: u32,
    pub vertical_interrupt1_en: u32,
    pub vertical_interrupt1_line: u32,
    pub vertical_interrupt2_en: u32,
    pub vertical_interrupt2_line: u32,
    pub vertical_interrupt2_dest: u32,
    pub otg_master_update_lock: u32,
    pub otg_double_buffer_control: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_optc_reg_state {
    pub optc_bytes_per_pixel: u32,
    pub optc_data_format_control: u32,
    pub optc_data_source_select: u32,
    pub optc_input_clock_control: u32,
    pub optc_input_global_control: u32,
    pub optc_input_spare_register: u32,
    pub optc_memory_config: u32,
    pub optc_rsmu_underflow: u32,
    pub optc_underflow_threshold: u32,
    pub optc_width_control: u32,
    pub otg_3d_structure_control: u32,
    pub otg_clock_control: u32,
    pub otg_control: u32,
    pub otg_count_control: u32,
    pub otg_count_reset: u32,
    pub otg_crc_cntl: u32,
    pub otg_crc_sig_blue_control_mask: u32,
    pub otg_crc_sig_red_green_mask: u32,
    pub otg_crc0_data_b: u32,
    pub otg_crc0_data_rg: u32,
    pub otg_crc0_windowa_x_control: u32,
    pub otg_crc0_windowa_x_control_readback: u32,
    pub otg_crc0_windowa_y_control: u32,
    pub otg_crc0_windowa_y_control_readback: u32,
    pub otg_crc0_windowb_x_control: u32,
    pub otg_crc0_windowb_x_control_readback: u32,
    pub otg_crc0_windowb_y_control: u32,
    pub otg_crc0_windowb_y_control_readback: u32,
    pub otg_crc1_data_b: u32,
    pub otg_crc1_data_rg: u32,
    pub otg_crc1_windowa_x_control: u32,
    pub otg_crc1_windowa_x_control_readback: u32,
    pub otg_crc1_windowa_y_control: u32,
    pub otg_crc1_windowa_y_control_readback: u32,
    pub otg_crc1_windowb_x_control: u32,
    pub otg_crc1_windowb_x_control_readback: u32,
    pub otg_crc1_windowb_y_control: u32,
    pub otg_crc1_windowb_y_control_readback: u32,
    pub otg_crc2_data_b: u32,
    pub otg_crc2_data_rg: u32,
    pub otg_crc3_data_b: u32,
    pub otg_crc3_data_rg: u32,
    pub otg_dlpc_control: u32,
    pub otg_double_buffer_control: u32,
    pub otg_drr_control2: u32,
    pub otg_drr_control: u32,
    pub otg_drr_timing_int_status: u32,
    pub otg_drr_trigger_window: u32,
    pub otg_drr_v_total_change: u32,
    pub otg_drr_v_total_reach_range: u32,
    pub otg_dsc_start_position: u32,
    pub otg_force_count_now_cntl: u32,
    pub otg_global_control0: u32,
    pub otg_global_control1: u32,
    pub otg_global_control2: u32,
    pub otg_global_control3: u32,
    pub otg_global_control4: u32,
    pub otg_global_sync_status: u32,
    pub otg_gsl_control: u32,
    pub otg_gsl_vsync_gap: u32,
    pub otg_gsl_window_x: u32,
    pub otg_gsl_window_y: u32,
    pub otg_h_blank_start_end: u32,
    pub otg_h_sync_a: u32,
    pub otg_h_sync_a_cntl: u32,
    pub otg_h_timing_cntl: u32,
    pub otg_h_total: u32,
    pub otg_interlace_control: u32,
    pub otg_interlace_status: u32,
    pub otg_interrupt_control: u32,
    pub otg_long_vblank_status: u32,
    pub otg_m_const_dto0: u32,
    pub otg_m_const_dto1: u32,
    pub otg_manual_force_vsync_next_line: u32,
    pub otg_master_en: u32,
    pub otg_master_update_lock: u32,
    pub otg_master_update_mode: u32,
    pub otg_nom_vert_position: u32,
    pub otg_pipe_update_status: u32,
    pub otg_pixel_data_readback0: u32,
    pub otg_pixel_data_readback1: u32,
    pub otg_request_control: u32,
    pub otg_snapshot_control: u32,
    pub otg_snapshot_frame: u32,
    pub otg_snapshot_position: u32,
    pub otg_snapshot_status: u32,
    pub otg_spare_register: u32,
    pub otg_static_screen_control: u32,
    pub otg_status: u32,
    pub otg_status_frame_count: u32,
    pub otg_status_hv_count: u32,
    pub otg_status_position: u32,
    pub otg_status_vf_count: u32,
    pub otg_stereo_control: u32,
    pub otg_stereo_force_next_eye: u32,
    pub otg_stereo_status: u32,
    pub otg_trig_manual_control: u32,
    pub otg_triga_cntl: u32,
    pub otg_triga_manual_trig: u32,
    pub otg_trigb_cntl: u32,
    pub otg_trigb_manual_trig: u32,
    pub otg_update_lock: u32,
    pub otg_v_blank_start_end: u32,
    pub otg_v_count_stop_control: u32,
    pub otg_v_count_stop_control2: u32,
    pub otg_v_sync_a: u32,
    pub otg_v_sync_a_cntl: u32,
    pub otg_v_total: u32,
    pub otg_v_total_control: u32,
    pub otg_v_total_int_status: u32,
    pub otg_v_total_max: u32,
    pub otg_v_total_mid: u32,
    pub otg_v_total_min: u32,
    pub otg_vert_sync_control: u32,
    pub otg_vertical_interrupt0_control: u32,
    pub otg_vertical_interrupt0_position: u32,
    pub otg_vertical_interrupt1_control: u32,
    pub otg_vertical_interrupt1_position: u32,
    pub otg_vertical_interrupt2_control: u32,
    pub otg_vertical_interrupt2_position: u32,
    pub otg_vready_param: u32,
    pub otg_vstartup_param: u32,
    pub otg_vsync_nom_int_status: u32,
    pub otg_vupdate_keepout: u32,
    pub otg_vupdate_param: u32,
}

//
// struct timing_generator - Entry point to Output Timing Generator feature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing_generator {
//
// @funcs: Timing generator control functions
//
    pub funcs: *const timing_generator_funcs,
    pub bp: *mut dc_bios,
    pub ctx: *mut dc_context,
    pub inst: u32,
}

//
// struct timing_generator_funcs - Control timing generator on a given device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing_generator_funcs {
    pub timing): *const dc_crtc_timing,
    pub end_line): u32,
    pub start_line): u32,
    pub start_line): u32,
    pub tg): *mut *mut bool (enable_crtc)(struct timing_generator,
    pub tg): *mut *mut bool (disable_crtc)(struct timing_generator,
    pub tg): *mut *mut void (phantom_crtc_post_enable)(struct timing_generator,
    pub tg): *mut *mut void (disable_phantom_crtc)(struct timing_generator,
    pub tg): *mut *mut bool (immediate_disable_crtc)(struct timing_generator,
    pub tg): *mut *mut bool (is_counter_moving)(struct timing_generator,
    pub position): *mut crtc_position,
    pub tg): *mut *mut uint32_t (get_frame_count)(struct timing_generator,
    pub v_position): *mut u32,
    pub otg_active_height): *mut u32,
    pub otg_timing): *const dc_crtc_timing,
    pub early_cntl): u32,
    pub state): crtc_state,
    pub enable_blanking): bool,
    pub tg): *mut *mut bool (is_blanked)(struct timing_generator,
    pub color): *const *const *const void (set_overscan_blank_color) (struct timing_generator tg, struct tg_color,
    pub color): *const *const *const void (set_blank_color)(struct timing_generator tg, struct tg_color,
    pub overscan_color): *const tg_color,
    pub tg): *mut *mut void (disable_vga)(struct timing_generator,
    pub tg): *mut *mut bool (did_triggered_reset_occur)(struct timing_generator,
    pub gsl_params): *const dcp_gsl_params,
    pub tg): *mut *mut void (unlock)(struct timing_generator,
    pub tg): *mut *mut void (lock)(struct timing_generator,
    pub tg): *mut *mut void (lock_doublebuffer_disable)(struct timing_generator,
    pub tg): *mut *mut void (lock_doublebuffer_enable)(struct timing_generator,
    pub tg): *mut *mut void(triplebuffer_unlock)(struct timing_generator,
    pub tg): *mut *mut void(triplebuffer_lock)(struct timing_generator,
    pub source_tg_inst): c_int,
    pub crtc_tp): *mut crtc_trigger_info,
    pub tg): *mut *mut void (disable_reset_trigger)(struct timing_generator,
    pub tg): *mut *mut void (tear_down_global_swap_lock)(struct timing_generator,
    pub timing): *const bool enable, struct dc_crtc_timing,
    pub params): *const *const *const void (set_drr)(struct timing_generator tg, struct drr_params,
    pub vtotal_max): *mut *mut *mut void (set_vtotal_min_max)(struct timing_generator optc, int vtotal_min, int,
    pub refresh_rate): *mut *mut *mut void (get_last_used_drr_vtotal)(struct timing_generator optc, uint32_t,
    pub num_frames): u32,
    pub color_depth): dc_color_depth,
    pub width): *mut *mut *mut bool (arm_vert_intr)(struct timing_generator tg, uint8_t,
    pub pstate_keepout): c_int,
    pub enable): *mut *mut *mut void (enable_optc_clock)(struct timing_generator tg, bool,
    pub flags): *const *const dc_crtc_timing timing, crtc_stereo_flags,
    pub tg): *mut *mut bool (is_stereo_left_eye)(struct timing_generator,
    pub enable): *mut *mut *mut void (set_blank_data_double_buffer)(struct timing_generator tg, bool,
    pub tg): *mut *mut void (tg_init)(struct timing_generator,
    pub tg): *mut *mut bool (is_tg_enabled)(struct timing_generator,
    pub tg): *mut *mut bool (is_optc_underflow_occurred)(struct timing_generator,
    pub tg): *mut *mut void (clear_optc_underflow)(struct timing_generator,
    pub dwb_pipe_inst): u32,
    pub seg1_src_sel): *mut u32,
    pub timing): *const *const bool (is_two_pixels_per_container)(struct dc_crtc_timing,
//
// Configure CRCs for the given timing generator. Return false if TG is
// not on.
//
    pub params): *const crc_params,
//
// @get_crc: Get CRCs for the given timing generator. Return false if
// CRCs are not enabled (via configure_crc).
//
    pub b_cb): *mut *mut *mut uint32_t r_cr, uint32_t g_y, uint32_t,
    pub optc): *mut *mut void (program_manual_trigger)(struct timing_generator,
    pub optc): *mut *mut void (setup_manual_trigger)(struct timing_generator,
    pub hw_crtc_timing): *mut dc_crtc_timing,
    pub program_fp2): *const *const dc_crtc_timing dc_crtc_timing, bool,
    pub dsc_slice_width): u32,
    pub dsc_mode): *mut u32,
    pub dc_crtc_timing): *const *const *const void (set_odm_bypass)(struct timing_generator optc, struct dc_crtc_timing,
//
// @set_odm_combine: Set up the ODM block to read from the correct
// OPP(s) and turn on/off ODM memory.
//
    pub last_segment_width): int segment_width, int,
    pub odm_segments): *mut *mut *mut void (get_odm_combine_segments)(struct timing_generator tg, int,
    pub manual_mode): *mut *mut *mut void (set_h_timing_div_manual_mode)(struct timing_generator optc, bool,
    pub params): *const *const *const void (set_gsl)(struct timing_generator optc, struct gsl_params,
    pub gsl_ready_signal): u32,
    pub dest): *mut *mut *mut void (set_out_mux)(struct timing_generator tg, enum otg_out_mux_dest,
    pub window_end): uint32_t window_start, uint32_t,
    pub fva_adj): *mut *mut *mut int (set_fva_factor)(struct timing_generator optc, struct fva_adj,
    pub max_pixclk_100hz): c_uint,
    pub limit): u32,
    pub slave_clock_divider): u8,
    pub vmax): int vmin, int,
    pub vtotal_change_limit): u32,
    pub tg): *mut *mut void (init_odm)(struct timing_generator,
    pub tg): *mut *mut void (wait_drr_doublebuffer_pending_clear)(struct timing_generator,
    pub params): *const *const *const void (set_long_vtotal)(struct timing_generator optc, struct long_vtotal_params,
    pub tg): *mut *mut void (wait_odm_doublebuffer_pending_clear)(struct timing_generator,
    pub optc): *mut *mut void (wait_otg_disable)(struct timing_generator,
    pub tg): *mut *mut bool (get_optc_double_buffer_pending)(struct timing_generator,
    pub tg): *mut *mut bool (get_otg_double_buffer_pending)(struct timing_generator,
    pub tg): *mut *mut bool (get_pipe_update_pending)(struct timing_generator,
    pub enable): *mut *mut *mut void (set_vupdate_keepout)(struct timing_generator tg, bool,
    pub locked): *mut *mut *mut bool (wait_update_lock_status)(struct timing_generator tg, bool,
    pub s): *mut *mut *mut void (read_otg_state)(struct timing_generator tg, struct dcn_otg_state,
    pub optc_reg_state): *mut *mut *mut void (optc_read_reg_state)(struct timing_generator tg, struct dcn_optc_reg_state,
    pub pwa_param): *mut *mut *mut void (enable_otg_pwa)(struct timing_generator tg, struct otc_pwa_frame_sync,
    pub tg): *mut *mut void (disable_otg_pwa)(struct timing_generator,
}
