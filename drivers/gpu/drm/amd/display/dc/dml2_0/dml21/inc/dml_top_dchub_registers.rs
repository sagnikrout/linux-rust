//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/dml_top_dchub_registers.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Macro flag: #define __dml2_TOP_DCHUB_REGISTERS_H__

// These types are uint32_t as they represent actual calculated register values for HW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_dlg_regs {
    pub refcyc_h_blank_end: u32,
    pub dlg_vblank_end: u32,
    pub min_dst_y_next_start: u32,
    pub refcyc_per_htotal: u32,
    pub refcyc_x_after_scaler: u32,
    pub dst_y_after_scaler: u32,
    pub dst_y_prefetch: u32,
    pub dst_y_per_vm_vblank: u32,
    pub dst_y_per_row_vblank: u32,
    pub dst_y_per_vm_flip: u32,
    pub dst_y_per_row_flip: u32,
    pub ref_freq_to_pix_freq: u32,
    pub vratio_prefetch: u32,
    pub vratio_prefetch_c: u32,
    pub refcyc_per_tdlut_group: u32,
    pub refcyc_per_pte_group_vblank_l: u32,
    pub refcyc_per_pte_group_vblank_c: u32,
    pub refcyc_per_pte_group_flip_l: u32,
    pub refcyc_per_pte_group_flip_c: u32,
    pub dst_y_per_pte_row_nom_l: u32,
    pub dst_y_per_pte_row_nom_c: u32,
    pub refcyc_per_pte_group_nom_l: u32,
    pub refcyc_per_pte_group_nom_c: u32,
    pub refcyc_per_line_delivery_pre_l: u32,
    pub refcyc_per_line_delivery_pre_c: u32,
    pub refcyc_per_line_delivery_l: u32,
    pub refcyc_per_line_delivery_c: u32,
    pub refcyc_per_vm_group_vblank: u32,
    pub refcyc_per_vm_group_flip: u32,
    pub refcyc_per_vm_req_vblank: u32,
    pub refcyc_per_vm_req_flip: u32,
    pub dst_y_offset_cur0: u32,
    pub chunk_hdl_adjust_cur0: u32,
    pub vready_after_vcount0: u32,
    pub dst_y_delta_drq_limit: u32,
    pub refcyc_per_vm_dmdata: u32,
    pub dmdata_dl_delta: u32,
    pub dst_y_svp_drq_limit: u32,
    pub force_prefetch_to_vblank: u32,
    pub force_cursor_to_disp_pref: u32,
// MRQ
    pub refcyc_per_meta_chunk_vblank_l: u32,
    pub refcyc_per_meta_chunk_vblank_c: u32,
    pub refcyc_per_meta_chunk_flip_l: u32,
    pub refcyc_per_meta_chunk_flip_c: u32,
    pub dst_y_per_meta_row_nom_l: u32,
    pub dst_y_per_meta_row_nom_c: u32,
    pub refcyc_per_meta_chunk_nom_l: u32,
    pub refcyc_per_meta_chunk_nom_c: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_ttu_regs {
    pub qos_level_low_wm: u32,
    pub qos_level_high_wm: u32,
    pub min_ttu_vblank: u32,
    pub qos_level_flip: u32,
    pub refcyc_per_req_delivery_l: u32,
    pub refcyc_per_req_delivery_c: u32,
    pub refcyc_per_req_delivery_cur0: u32,
    pub refcyc_per_req_delivery_pre_l: u32,
    pub refcyc_per_req_delivery_pre_c: u32,
    pub refcyc_per_req_delivery_pre_cur0: u32,
    pub qos_level_fixed_l: u32,
    pub qos_level_fixed_c: u32,
    pub qos_level_fixed_cur0: u32,
    pub qos_ramp_disable_l: u32,
    pub qos_ramp_disable_c: u32,
    pub qos_ramp_disable_cur0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_arb_regs {
    pub max_req_outstanding: u32,
    pub min_req_outstanding: u32,
    pub sat_level_us: u32,
    pub hvm_max_qos_commit_threshold: u32,
    pub hvm_min_req_outstand_commit_threshold: u32,
    pub compbuf_reserved_space_kbytes: u32,
    pub compbuf_size: u32,
    pub sdpif_request_rate_limit: u32,
    pub allow_sdpif_rate_limit_when_cstate_req: u32,
    pub dcfclk_deep_sleep_hysteresis: u32,
    pub pstate_stall_threshold: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_cursor_dlg_regs {
    pub CURSOR0_DST_X_OFFSET: uint32_t dst_x_offset; //,
    pub CURSOR0_DST_Y_OFFSET: uint32_t dst_y_offset; //,
    pub CURSOR0_CHUNK_HDL_ADJUST: uint32_t chunk_hdl_adjust; //,
    pub qos_level_fixed: u32,
    pub qos_ramp_disable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_plane_rq_regs {
    pub chunk_size: u32,
    pub min_chunk_size: u32,
    pub dpte_group_size: u32,
    pub mpte_group_size: u32,
    pub swath_height: u32,
    pub pte_row_height_linear: u32,
// MRQ
    pub meta_chunk_size: u32,
    pub min_meta_chunk_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_rq_regs {
    pub rq_regs_l: dml2_display_plane_rq_regs,
    pub rq_regs_c: dml2_display_plane_rq_regs,
    pub drq_expansion_mode: u32,
    pub prq_expansion_mode: u32,
    pub crq_expansion_mode: u32,
    pub plane1_base_address: u32,
    pub unbounded_request_enabled: u32,
    pub pte_buffer_mode: bool,
    pub force_one_row_for_frame: bool,
// MRQ
    pub mrq_expansion_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_mcache_regs {
    pub mcache_id_first: u32,
    pub mcache_id_second: u32,
    pub split_location: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_hubp_pipe_mcache_regs {
    pub p0: dml2_display_mcache_regs,
    pub p1: dml2_display_mcache_regs,
    pub main: },
    pub p0: dml2_display_mcache_regs,
    pub p1: dml2_display_mcache_regs,
    pub mall: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dchub_per_pipe_register_set {
    pub rq_regs: dml2_display_rq_regs,
    pub ttu_regs: dml2_display_ttu_regs,
    pub dlg_regs: dml2_display_dlg_regs,
    pub det_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcif_per_pipe_register_set {
    pub format: unsigned int time_per_pixel; // U6.6,
    pub arbitration_slice: c_uint,
    pub slice_lines: c_uint,
    pub max_scaled_time_ns: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dchub_watermark_regs {
// watermarks
    pub urgent: u32,
    pub sr_enter: u32,
    pub sr_exit: u32,
    pub sr_enter_z8: u32,
    pub sr_exit_z8: u32,
    pub sr_enter_low_power: u32,
    pub sr_exit_low_power: u32,
    pub uclk_pstate: u32,
    pub fclk_pstate: u32,
    pub temp_read_or_ppt: u32,
    pub temp_read: u32,
}

// qos
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_dchub_watermark_reg_set_index {
    DML2_DCHUB_WATERMARK_SET_A = 0,
    DML2_DCHUB_WATERMARK_SET_B = 1,
    DML2_DCHUB_WATERMARK_SET_C = 2,
    DML2_DCHUB_WATERMARK_SET_D = 3,
    DML2_DCHUB_WATERMARK_SET_NUM = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dchub_global_register_set {
    pub arb_regs: dml2_display_arb_regs,
    pub wm_regs: [dml2_dchub_watermark_regs; DML2_DCHUB_WATERMARK_SET_NUM],
    pub num_watermark_sets: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcif_watermark_regs {
// watermarks
    pub /: *mut *mut uint32_t urgent; / (CLI),
    pub uclk_pstate: u32,
    pub fclk_pstate: u32,
    pub temp_read_or_ppt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcif_global_register_set {
    pub wm_regs: [dml2_mcif_watermark_regs; DML2_DCHUB_WATERMARK_SET_NUM],
    pub num_watermark_sets: c_uint,
}
