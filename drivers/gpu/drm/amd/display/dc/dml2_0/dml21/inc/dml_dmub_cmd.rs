//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/dml_dmub_cmd.h
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
// always include for now

// Define to ensure that the "common" members always appear in the same
// order in different structs for back compat purposes
//

// Maximum number of streams on any ASIC.
pub const DMUB_MAX_STREAMS: c_int = 6;
// Maximum number of planes on any ASIC.
pub const DMUB_MAX_PLANES: c_int = 6;
// Maximum number of phantom planes on any ASIC

// Flattened structure containing SOC BB parameters stored in the VBIOS
// It is not practical to store the entire bounding box in VBIOS since the bounding box struct can gain new parameters.
// This also prevents alighment issues when new parameters are added to the SoC BB.
// The following parameters should be added since these values can't be obtained elsewhere:
// -dml2_soc_power_management_parameters
// -dml2_soc_vmin_clock_limits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_soc_bb_params {
    pub dram_clk_change_blackout_ns: u32,
    pub dram_clk_change_read_only_ns: u32,
    pub dram_clk_change_write_only_ns: u32,
    pub fclk_change_blackout_ns: u32,
    pub g7_ppt_blackout_ns: u32,
    pub stutter_enter_plus_exit_latency_ns: u32,
    pub stutter_exit_latency_ns: u32,
    pub z8_stutter_enter_plus_exit_latency_ns: u32,
    pub z8_stutter_exit_latency_ns: u32,
    pub z8_min_idle_time_ns: u32,
    pub type_b_dram_clk_change_blackout_ns: u32,
    pub type_b_ppt_blackout_ns: u32,
    pub vmin_limit_dispclk_khz: u32,
    pub vmin_limit_dcfclk_khz: u32,
    pub g7_temperature_read_blackout_ns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rect16 {
//
// Dirty rect x offset.
//
    pub src_x: uint16_t x; //,
//
// Dirty rect y offset.
//
    pub src_y: uint16_t y; //,
//
// Dirty rect width.
//
    pub rect_x: uint16_t width; // dest_width,,
//
// Dirty rect height.
//
    pub rect_y: uint16_t height; // dest_height,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_assisted_mclk_switch_version {
    pub 5: uint8_t minor :,
    pub 3: uint8_t major :,
}

// generic structures and enums
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_optc_position {
    pub vpos: u32,
    pub hpos: u32,
    pub frame: u32,
}

// HW and FW global configuration data for FAMS2
// FAMS2 types and structs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fams2_stream_type {
    FAMS2_STREAM_TYPE_NONE = 0,
    FAMS2_STREAM_TYPE_VBLANK = 1,
    FAMS2_STREAM_TYPE_VACTIVE = 2,
    FAMS2_STREAM_TYPE_DRR = 3,
    FAMS2_STREAM_TYPE_SUBVP = 4,
    FAMS2_STREAM_TYPE_ALTERNATE = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plane_pipe_rect {
    pub luma: dmub_rect16,
    pub chroma: dmub_rect16,
}

//
// Structure to hold the LSDMA source / dest copy parameters.
// Each field is an array of [2][4]:
// [2] - Instance 0 is the copy for current frame, instance 1 is the copy for next frame (instance 1 potentially unused if no next)
// [4] - One instance per pipe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_outputs {
    pub pipe: uint16_t src_x[2][4]; // src x position for the copy. Array of [2][4] for curr vs. next and each,
    pub copy: uint16_t src_y[2][4]; // src y position for the,
    pub next): uint16_t dst_x[2][4]; // dst x position for the copy (can change for curr vs.,
    pub next): uint16_t dst_y[2][4]; // dst y position for the copy (can change for curr vs.,
    pub match): uint16_t width[2][4]; // src and dst width for the copy (src and dst must,
    pub match): uint16_t height[2][4]; // src and dst height for the copy (src and dst must,
    pub next): uint16_t dst_pitch[4]; // dst pitch for the copy (same for curr and,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_alternate_stream_dynamic_state {
    pub is_tick_in_allow: uint64_t earliest_init_tick; // track earliest possible init tick for calculating,
    pub yet: uint32_t otg_frame_pending_clear[3]; // In this context pending means prefetch has never been completed for this frame,
    pub flip_pending_clear_order: [u8; 3],
    pub num_pending_flips: u8,
    pub times: uint32_t prefetch_start_line_x1000[3]; // can compute from existing params, but store because we use this multiple,
    pub times: uint16_t prefetch_end_line[3]; // can compute from existing params, but store because we use this multiple,
    pub recout_y: [u16; 3],
    pub flip_pending: [u8; 3],
    pub copy_from_earliest: [u8; 3],
    pub lsdma_bandwidth_mbps: u16,
    pub vstartup_line: u16,
    pub vready_line: u16,
    pub array: uint8_t cursor_size[3]; // Cursor array per plane for now - if we assume a single cursor, then we don't need an,
    pub finalized: uint8_t pad; // to maintain alignment for below fields - re-arrange structure once all fields are,
// outputs:
    pub subvp_start_line_a: [u16; 3],
    pub subvp_height_a: [u16; 3],
    pub subvp_next_start_line_a: [u16; 3],
    pub subvp_next_height_a: [u16; 3],
    pub subvp_start_line_b: [u16; 3],
    pub subvp_height_b: [u16; 3],
    pub subvp_next_start_line_b: [u16; 3],
    pub subvp_next_height_b: [u16; 3],
    pub subvp_c_start_line_a: [u16; 3],
    pub subvp_c_height_a: [u16; 3],
    pub subvp_c_next_start_line_a: [u16; 3],
    pub subvp_c_next_height_a: [u16; 3],
    pub subvp_c_start_line_b: [u16; 3],
    pub subvp_c_height_b: [u16; 3],
    pub subvp_c_next_start_line_b: [u16; 3],
    pub subvp_c_next_height_b: [u16; 3],
    pub subvp_position: [u8; 3],
    pub copy_from_primary: [u8; 3],
    pub finalized: uint8_t pad1[2]; // to maintain alignment for below fields - re-arrange structure once all fields are,
    pub program_go_line: u32,
    pub program_go_frame_count: u32,
    pub svp0_start_dst_line: u16,
    pub svp0_end_dst_line: u16,
    pub svp1_start_dst_line: u16,
    pub svp1_end_dst_line: u16,
    pub SVP1: lsdma_outputs lsdma[2]; // [2] - instance per SVP0 and,
    pub SVP1: lsdma_outputs lsdma_c[2]; // [2] - instance per SVP0 and,
}

// dynamic stream state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_legacy_stream_dynamic_state {
    pub force_allow_at_vblank: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_subvp_stream_dynamic_state {
    pub viewport_start_hubp_vline: u16,
    pub viewport_height_hubp_vlines: u16,
    pub viewport_start_c_hubp_vline: u16,
    pub viewport_height_c_hubp_vlines: u16,
    pub phantom_viewport_height_hubp_vlines: u16,
    pub phantom_viewport_height_c_hubp_vlines: u16,
    pub microschedule_start_otg_vline: u16,
    pub mall_start_otg_vline: u16,
    pub mall_start_hubp_vline: u16,
    pub mall_start_c_hubp_vline: u16,
    pub force_allow_at_vblank_only: u8,
    pub swath_height: u8,
    pub swath_height_c: u8,
    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_drr_stream_dynamic_state {
    pub stretched_vtotal: u16,
    pub use_cur_vtotal: u8,
    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_alternate_stream_static_state {
    pub total_bytes_to_copy: u32,
    pub stream: uint16_t svp0_dst_lines; // per,
    pub stream: uint16_t svp1_dst_lines; // per,
    pub planes: uint16_t min_lead_dst_lines; // per stream, should be max(nominal_req_limit, vstartup_to_vactive). Does not have to be maxed over all,
    pub planes: uint16_t svp_req_limit; // per stream, should be the same value in time between all streams max(2 swaths, dst_y_pre) over all,
    pub fw_delays: u16,
    pub vstartup_start: u16,
    pub rec_height: [u16; 3],
    pub viewport_start: [u16; 3],
    pub 270): uint16_t viewport_size[3]; // for now size will be the number of lines perpendicular to scan direction (height for 0 / 180, width for 90 and,
    pub viewport_start_c: [u16; 3],
    pub viewport_size_c: [u16; 3],
    pub surface_pitch: [u16; 3],
    pub surface_pitch_c: [u16; 3],
    pub surface_height: [u16; 3],
    pub surface_height_c: [u16; 3],
    pub element_size: [u8; 3],
    pub element_size_c: [u8; 3],
    pub command: uint8_t swizzle_mode[3]; // TODO: Add mapping, should be value used in LSDMA,
    pub line): uint8_t vready_offset_lines; // vready offset from vstartup in lines (rounded up, as the actual offset may be a fraction of a,
    pub dst_y_prefetch_x1000: [u16; 3],
    pub total_swaths: [u16; 3],
    pub total_swaths_c: [u16; 3],
    pub prefetch_swaths: [u8; 3],
    pub prefetch_swaths_c: [u8; 3],
    pub swath_height: [u8; 3],
    pub swath_height_c: [u8; 3],
    pub block_256b_width: [u16; 3],
    pub block_256b_height: [u16; 3],
    pub block_256b_width_c: [u16; 3],
    pub block_256b_height_c: [u16; 3],
    pub macro_tile_width: [u16; 3],
    pub macro_tile_width_c: [u16; 3],
    pub 1: uint8_t is_multi_planar :,
    pub 1: uint8_t is_yuv420 :,
    pub 1: uint8_t prefetch_relative_vblank :,
    pub rotation: uint8_t vertical_access : 1; // vertical_access = 1 means 90 or 270,
    pub 2160): uint8_t access_direction : 1; // access_direction = 1 means bigger to smaller coordinations (e.g., scan from 2160 to 0 as opposed to regular 0 to,
    pub 1: uint8_t dcc :,
    pub TMZ): uint8_t tmz : 1; // TODO: Need to assign outside of DML (DML not aware of,
    pub bits: },
    pub all: u8,
    pub config: [}; 3],
    pub max_cursor_size: u8,
    pub pre_hdl_delta_x1000: [u16; 3],
    pub pre_hdl_delta_c_x1000: [u16; 3],
    pub rec_hdl_delta_x1000: [u16; 3],
    pub rec_hdl_delta_c_x1000: [u16; 3],
    pub dst_y_per_vm_vblank_x1000: [u16; 3],
    pub dst_y_per_row_vblank_x1000: [u16; 3],
    pub dst_y_after_scaler: [u16; 3],
    pub vinit_prefill: [u16; 3],
    pub vinit_prefill_c: [u16; 3],
    pub vratio_x1000: [u16; 3],
    pub vratio_c_x1000: [u16; 3],
    pub pipe_viewports: [plane_pipe_rect; 4],
// TODO - remove these deprecated vars
    pub pipes: uint32_t pipe_copy_offset[2][4]; // [2] - SVP0/1, [4] - 4,
    pub pipe_copy_offset_c: [u32; 2][4],
// bits 47:16 of the surface address
    pub pipes: uint32_t pipe_copy_addr_47_16[2][4]; // [2] - SVP0/1, [4] - 4,
    pub pipe_copy_addr_47_16_c: [u32; 2][4],
    pub pipe_copy_max_size: [u32; 2][4],
    pub pipe_copy_max_size_c: [u32; 2][4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_stream_dynamic_state {
    pub ref_tick: u64,
    pub cur_vtotal: u32,
    pub adjusted_allow_end_otg_vline: u16,
    pub pad: [u8; 2],
    pub ref_otg_pos: dmub_optc_position,
    pub target_otg_pos: dmub_optc_position,
    pub legacy: dmub_fams2_legacy_stream_dynamic_state,
    pub subvp: dmub_fams2_subvp_stream_dynamic_state,
    pub drr: dmub_fams2_drr_stream_dynamic_state,
    pub alternate: dmub_fams2_alternate_stream_dynamic_state,
    pub sub_state: },
}

// static stream state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_legacy_stream_static_state {
    pub vactive_det_fill_delay_otg_vlines: u8,
    pub programming_delay_otg_vlines: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_subvp_stream_static_state {
    pub vratio_numerator: u16,
    pub vratio_denominator: u16,
    pub phantom_vtotal: u16,
    pub phantom_vactive: u16,
    pub 1: uint8_t is_multi_planar :,
    pub 1: uint8_t is_yuv420 :,
    pub bits: },
    pub all: u8,
    pub config: },
    pub programming_delay_otg_vlines: u8,
    pub prefetch_to_mall_otg_vlines: u8,
    pub phantom_otg_inst: u8,
    pub phantom_pipe_mask: u8,
    pub passthrough): uint8_t phantom_plane_pipe_masks[DMUB_MAX_PHANTOM_PLANES]; // phantom pipe mask per plane (for flip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_drr_stream_static_state {
    pub nom_stretched_vtotal: u16,
    pub programming_delay_otg_vlines: u8,
    pub only_stretch_if_required: u8,
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_legacy_stream_static_state {
    pub vactive_det_fill_delay_otg_vlines: u16,
    pub programming_delay_otg_vlines: u16,
    pub disallow_time_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_subvp_stream_static_state {
    pub vratio_numerator: u16,
    pub vratio_denominator: u16,
    pub phantom_vtotal: u16,
    pub phantom_vactive: u16,
    pub programming_delay_otg_vlines: u16,
    pub prefetch_to_mall_otg_vlines: u16,
    pub 1: uint8_t is_multi_planar :,
    pub 1: uint8_t is_yuv420 :,
    pub bits: },
    pub all: u8,
    pub config: },
    pub phantom_otg_inst: u8,
    pub phantom_pipe_mask: u8,
    pub pad0: u8,
    pub passthrough): uint8_t phantom_plane_pipe_masks[DMUB_MAX_PHANTOM_PLANES]; // phantom pipe mask per plane (for flip,
    pub 4)]: uint8_t pad1[4 - (DMUB_MAX_PHANTOM_PLANES %,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_drr_stream_static_state {
    pub nom_stretched_vtotal: u16,
    pub programming_delay_otg_vlines: u16,
    pub only_stretch_if_required: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_stream_static_sub_state {
    pub legacy: dmub_fams2_legacy_stream_static_state,
    pub subvp: dmub_fams2_subvp_stream_static_state,
    pub drr: dmub_fams2_drr_stream_static_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_cmd_stream_static_sub_state {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_stream_static_sub_state_v2 {
    pub alternate: dmub_fams2_cmd_alternate_stream_static_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_stream_static_state {
    pub type: fams2_stream_type,
    pub otg_vline_time_ns: u32,
    pub otg_vline_time_ticks: u32,
    pub htotal: u16,
    pub vtotal: uint16_t vtotal; // nominal,
    pub vblank_start: u16,
    pub vblank_end: u16,
    pub max_vtotal: u16,
    pub allow_start_otg_vline: u16,
    pub allow_end_otg_vline: u16,
    pub changed: uint16_t drr_keepout_otg_vline; // after this vline, vtotal cannot be,
    pub start: uint8_t scheduling_delay_otg_vlines; // min time to budget for ready to microschedule,
    pub execution: uint8_t contention_delay_otg_vlines; // time to budget for contention on,
    pub firing: uint8_t vline_int_ack_delay_otg_vlines; // min time to budget for vertical interrupt,
    pub vline: uint8_t allow_to_target_delay_otg_vlines; // time from allow vline to target,
    pub enabled: uint8_t is_drr : 1; // stream is DRR,
    pub nominal: uint8_t clamp_vtotal_min : 1; // clamp vtotal to min instead of,
    pub blank: uint8_t min_ttu_vblank_usable : 1; // if min ttu vblank is above wm, no force pstate is needed in,
    pub bits: },
    pub all: u8,
    pub config: },
    pub otg_inst: u8,
    pub config: uint8_t pipe_mask; // pipe mask for the whole,
    pub num_planes: u8,
    pub passthrough): uint8_t plane_pipe_masks[DMUB_MAX_PLANES]; // pipe mask per plane (for flip,
    pub 4]: uint8_t pad[DMUB_MAX_PLANES %,
    pub sub_state: dmub_fams2_stream_static_sub_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_stream_static_base_state {
    pub type: fams2_stream_type,
    pub otg_vline_time_ns: u32,
    pub otg_vline_time_ticks: u32,
    pub htotal: u16,
    pub vtotal: uint16_t vtotal; // nominal,
    pub vblank_start: u16,
    pub vblank_end: u16,
    pub max_vtotal: u16,
    pub allow_start_otg_vline: u16,
    pub allow_end_otg_vline: u16,
    pub changed: uint16_t drr_keepout_otg_vline; // after this vline, vtotal cannot be,
    pub start: uint16_t scheduling_delay_otg_vlines; // min time to budget for ready to microschedule,
    pub execution: uint16_t contention_delay_otg_vlines; // time to budget for contention on,
    pub firing: uint16_t vline_int_ack_delay_otg_vlines; // min time to budget for vertical interrupt,
    pub vline: uint16_t allow_to_target_delay_otg_vlines; // time from allow vline to target,
    pub enabled: uint8_t is_drr : 1; // stream is DRR,
    pub nominal: uint8_t clamp_vtotal_min : 1; // clamp vtotal to min instead of,
    pub blank: uint8_t min_ttu_vblank_usable : 1; // if min ttu vblank is above wm, no force pstate is needed in,
    pub bits: },
    pub all: u8,
    pub config: },
    pub otg_inst: u8,
    pub config: uint8_t pipe_mask; // pipe mask for the whole,
    pub num_planes: u8,
    pub passthrough): uint8_t plane_pipe_masks[DMUB_MAX_PLANES]; // pipe mask per plane (for flip,
    pub 4]: uint8_t pad[DMUB_MAX_PLANES %,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_stream_static_state_v1 {
    pub base: dmub_fams2_cmd_stream_static_base_state,
    pub sub_state: dmub_fams2_stream_static_sub_state_v2,
}

//
// enum dmub_fams2_allow_delay_check_mode - macroscheduler mode for breaking on excessive
// p-state request to allow latency
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_fams2_allow_delay_check_mode {
// No check for request to allow delay
    FAMS2_ALLOW_DELAY_CHECK_NONE = 0,
// Check for request to allow delay
    FAMS2_ALLOW_DELAY_CHECK_FROM_START = 1,
// Check for prepare to allow delay
    FAMS2_ALLOW_DELAY_CHECK_FROM_PREPARE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_global_feature_config {
    pub 1: uint32_t enable :,
    pub 1: uint32_t enable_ppt_check :,
    pub 1: uint32_t enable_stall_recovery :,
    pub 1: uint32_t enable_debug :,
    pub 1: uint32_t enable_offload_flip :,
    pub 1: uint32_t enable_visual_confirm :,
    pub 2: uint32_t allow_delay_check_mode :,
    pub 1: uint32_t legacy_method_no_fams2 :,
    pub 23: uint32_t reserved :,
    pub bits: },
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fams2_global_config {
    pub begin: uint32_t max_allow_delay_us; // max delay to assert allow from uclk change,
    pub lock: uint32_t lock_wait_time_us; // time to forecast acquisition of,
    pub num_streams: u32,
    pub features: dmub_fams2_global_feature_config,
    pub recovery_timeout_us: u32,
    pub hwfq_flip_programming_delay_us: u32,
    pub target: uint32_t max_allow_to_target_delta_us; // how early DCN could assert P-State allow compared to the P-State,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_fams2_config {
    pub global: dmub_cmd_fams2_global_config,
    pub //v0: dmub_fams2_stream_static_state stream;,
    pub base: dmub_fams2_cmd_stream_static_base_state,
    pub sub_state: dmub_fams2_cmd_stream_static_sub_state,
    pub //v1: } stream_v1;,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_config_v2 {
    pub global: dmub_cmd_fams2_global_config,
    pub //v1: dmub_fams2_stream_static_state_v1 stream_v1[DMUB_MAX_STREAMS];,
}

//
// OS/FW agnostic memcpy
//

//
// OS/FW agnostic memset
//

// #endif
