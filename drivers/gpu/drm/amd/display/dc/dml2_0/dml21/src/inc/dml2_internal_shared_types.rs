//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/inc/dml2_internal_shared_types.h
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

//
// DML2 MCG Types and Interfaces
//
pub const DML_MCG_MAX_CLK_TABLE_SIZE: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dram_bw_to_min_clk_table_entry {
    pub pre_derate_dram_bw_kbps: c_ulonglong,
    pub min_uclk_khz: c_ulong,
    pub min_fclk_khz: c_ulong,
    pub min_dcfclk_khz: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcg_dram_bw_to_min_clk_table {
    pub entries: [dram_bw_to_min_clk_table_entry; DML_MCG_MAX_CLK_TABLE_SIZE],
    pub num_entries: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcg_min_clock_table {
    pub dispclk: c_uint,
    pub dppclk: c_uint,
    pub dscclk: c_uint,
    pub dtbclk: c_uint,
    pub phyclk: c_uint,
    pub fclk: c_uint,
    pub dcfclk: c_uint,
    pub max_clocks_khz: },
    pub dispclk: c_uint,
    pub dppclk: c_uint,
    pub dtbclk: c_uint,
    pub max_ss_clocks_khz: },
    pub dprefclk: c_uint,
    pub xtalclk: c_uint,
    pub pcierefclk: c_uint,
    pub dchubrefclk: c_uint,
    pub amclk: c_uint,
    pub fixed_clocks_khz: },
    pub dram_bw_table: dml2_mcg_dram_bw_to_min_clk_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcg_build_min_clock_table_params_in_out {
//
// Input
//
    pub soc_bb: *mut dml2_soc_bb,
    pub perform_pseudo_build: bool,
    pub clean_me_up: },
//
// Output
//
    pub min_clk_table: *mut dml2_mcg_min_clock_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_soc_operating_point {
    pub uclk_khz: c_uint,
    pub fclk_khz: c_uint,
    pub dcfclk_khz: c_uint,
    pub socclk_khz: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_sop_constraint {
    pub min_sop_index: c_uint,
    pub latency: dml2_memory_path_latency,
    pub clocks: dml2_soc_operating_point,
    pub active: double min_available_urgent_bandwidth_KBps; // minimum guaranteed urgent bandwidth at,
    pub dcn5: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_sop_table {
    pub is_initialized: bool,
    pub model: *const utm_qos_model,
    pub sop_min_available_urgent_bandwidths_KBps: [u32; MAX_UTM_SOP_COUNT],
    pub sop_optimal_dcfclks_khz: [u32; MAX_UTM_SOP_COUNT],
    pub sop_table): *const *const unsigned int (get_highest_sop_index)(struct dml2_sop_table,
    pub constraint): *const *const *const void (get_sop_constraint_at_index)(struct dml2_sop_table sop_table, unsigned int index, struct dml2_sop_constraint,
    pub index): *const *const *const *const bool (is_bw_supported_at_index)(struct dml2_sop_table sop_table, struct dml2_memory_path_bandwidth bw, unsigned int,
    pub sop): *const *const *const void (get_max_sop)(struct dml2_sop_table sop_table, struct dml2_soc_operating_point,
    pub sop): *const *const *const void (get_min_sop)(struct dml2_sop_table sop_table, struct dml2_soc_operating_point,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_utm_soc_bb {
    pub sop_table: dml2_sop_table,
    pub power_management_parameters: dml2_soc_power_management_parameters,
    pub vmin_limit: dml2_soc_vmin_clock_limits,
    pub dram_config: dml2_dram_params,
    pub qos_model: utm_qos_model,
    pub qos_model_dchub_v1: utm_qos_model_dchub_v1,
    pub qos_model_dchub_v2: utm_qos_model_dchub_v2,
    pub qos_model_dchub_v3: utm_qos_model_dchub_v3,
}

// TODO: remove once DML Core no longer depends on max SOP clocks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_cga_initialize_in_out {
    pub adjuster: *mut dml2_clock_granularity_adjuster,
    pub soc_bb: *const dml2_soc_bb,
    pub ip: *const dml2_core_ip_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_clock_granularity_adjuster {
    pub dcn_downspread_percent: double,
    pub dispclk_dppclk_vco_speed_mhz: double,
    pub dispclk_ramp_margin_percent: double,
    pub max_dispclk_mhz: double,
    pub in_out): *const *const void (initialize)(struct dml2_cga_initialize_in_out,
    pub dispclk_mhz): *const *const *const double (adjust_dispclk_mhz)(struct dml2_clock_granularity_adjuster adjuster, double,
    pub adjusted_dpprefclk_mhz): *const *const *const double dppclks_mhz, double adjusted_dppclks_mhz, double,
    pub adjusted_dtbrefclk_mhz): *const *const *const double dtbclks_mhz, double adjusted_dtbclks_mhz, double,
    pub dcfclk_deepsleep_mhz): double,
}

// Generic */ \
// Validate */ \
// Optimize */ \
// Populate */ \

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_status {
    DML2_STATUS_LIST(ENUM_FORMAT)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcg_instance {
    pub in_out): *mut *mut bool (build_min_clock_table)(struct dml2_mcg_build_min_clock_table_params_in_out,
}

//
// DML2 DPMM Types and Interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dpmm_map_mode_to_soc_dpm_params_in_out {
//
// Input
//
    pub ip: *mut dml2_core_ip_params,
    pub soc_bb: *mut dml2_soc_bb,
    pub min_clk_table: *mut dml2_mcg_min_clock_table,
    pub display_cfg: *const display_configuation_with_meta,
    pub utm_soc_bb: *const dml2_utm_soc_bb,
    pub solution: *const dml2_display_solution,
    pub perform_pseudo_map: bool,
    pub soc_bb: *mut dml2_core_internal_soc_bb,
    pub clean_me_up: },
//
// Output
//
    pub programming: *mut dml2_display_cfg_programming,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dpmm_map_watermarks_params_in_out {
//
// Input
//
    pub display_cfg: *const display_configuation_with_meta,
    pub solution: *const dml2_display_solution,
    pub core: *const dml2_core_instance,
//
// Output
//
    pub programming: *mut dml2_display_cfg_programming,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dpmm_scratch {
    pub programming: dml2_display_cfg_programming,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dpmm_instance {
    pub in_out): *mut *mut bool (map_mode_to_soc_dpm)(struct dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    pub in_out): *mut *mut bool (map_watermarks)(struct dml2_dpmm_map_watermarks_params_in_out,
    pub dpmm_scratch: dml2_dpmm_scratch,
}

//
// DML2 Core Types and Interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_initialize_in_out {
    pub project_id: dml2_project_id,
    pub instance: *mut dml2_core_instance,
    pub soc_bb: *mut dml2_soc_bb,
    pub ip_caps: *mut dml2_ip_capabilities,
    pub minimum_clock_table: *mut dml2_mcg_min_clock_table,
    pub utm_soc_bb: *const dml2_utm_soc_bb,
    pub clock_adjuster: *const dml2_clock_granularity_adjuster,
    pub explicit_ip_bb: *mut c_void,
    pub explicit_ip_bb_size: c_uint,
// FIXME_STAGE2 can remove but dcn3 version still need this
    pub soc_bb: *mut soc_bounding_box_st,
    pub soc_states: *mut soc_states_st,
    pub legacy: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_bandwidth_requirements {
    pub urgent_bandwidth_kbytes_per_sec: c_int,
    pub average_bandwidth_kbytes_per_sec: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_plane_support_info {
    pub dpps_used: c_int,
    pub dram_change_latency_hiding_margin_in_active: c_int,
    pub active_latency_hiding_us: c_int,
    pub mall_svp_size_requirement_ways: c_int,
    pub nominal_vblank_pstate_latency_hiding_us: c_int,
    pub vactive_det_fill_delay_us: [c_int; dml2_pstate_type_count],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_stream_support_info {
    pub odms_used: c_uint,
    pub odm_mode_mso_1to2): unsigned int num_odm_output_segments; // for odm split mode (e.g. a value of 2 for,
// FAMS2 SubVP support info
    pub phantom_min_v_active: c_uint,
    pub phantom_v_startup: c_uint,
    pub phantom_v_active: c_uint,
    pub phantom_v_total: c_uint,
    pub vblank_reserved_time_us: c_int,
    pub num_dsc_slices: c_int,
    pub dsc_enable: bool,
    pub alternate_svp0_dst_lines: c_uint,
    pub alternate_svp1_dst_lines: c_uint,
    pub max_vstartup_lines: c_uint,
    pub max_dst_y_after_scaler: c_uint,
    pub max_dst_y_prefetch: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_display_cfg_support_info {
    pub is_supported: bool,
    pub stream_support_info: [core_stream_support_info; DML2_MAX_PLANES],
    pub plane_support_info: [core_plane_support_info; DML2_MAX_PLANES],
    pub support_info: dml2_core_internal_mode_support_info,
    pub clean_me_up: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_mode_support_result {
    pub urgent_bw_sdp_kbps: c_ulong,
    pub average_bw_sdp_kbps: c_ulong,
    pub urgent_bw_dram_kbps: c_ulong,
    pub average_bw_dram_kbps: c_ulong,
    pub dcfclk_khz: c_ulong,
    pub fclk_khz: c_ulong,
    pub svp_prefetch: },
    pub urgent_bw_sdp_kbps: c_ulong,
    pub average_bw_sdp_kbps: c_ulong,
    pub urgent_bw_dram_kbps: c_ulong,
    pub average_bw_dram_kbps: c_ulong,
    pub dcfclk_khz: c_ulong,
    pub fclk_khz: c_ulong,
    pub active: },
    pub dispclk_khz: c_uint,
    pub dpprefclk_khz: c_uint,
    pub dtbrefclk_khz: c_uint,
    pub dcfclk_deepsleep_khz: c_uint,
    pub socclk_khz: c_uint,
    pub uclk_pstate_supported: c_uint,
    pub fclk_pstate_supported: c_uint,
    pub alternate_total_bytes_copy_svp0: c_uint,
    pub alternate_total_bytes_copy_svp1: c_uint,
    pub lsdma_bw_req_for_alt_kbps: c_uint,
    pub watermarks: dml2_core_internal_watermarks,
    pub global: },
    pub dscclk_khz: c_uint,
    pub dtbclk_khz: c_uint,
    pub phyclk_khz: c_uint,
    pub per_stream: [}; DML2_MAX_PLANES],
    pub dppclk_khz: c_uint,
    pub mall_svp_allocation_mblks: c_uint,
    pub mall_full_frame_allocation_mblks: c_uint,
    pub per_plane: [}; DML2_MAX_PLANES],
    pub cfg_support_info: core_display_cfg_support_info,
    pub bandwidth_upper_bound: dml2_memory_path_bandwidth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_stage1_state {
    pub performed: bool,
    pub success: bool,
    pub min_clk_index_for_latency: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_stage2_state {
    pub performed: bool,
    pub success: bool,
// Whether or not each plane supports mcache
// The number of valid elements == display_cfg.num_planes
// The indexing of pstate_switch_modes matches plane_descriptors[]
    pub per_plane_mcache_support: [bool; DML2_MAX_PLANES],
    pub mcache_allocations: [dml2_mcache_surface_allocation; DML2_MAX_PLANES],
}

pub const DML2_PMO_LEGACY_PREFETCH_MAX_TWAIT_OPTIONS: c_int = 8;
pub const DML2_PMO_PSTATE_CANDIDATE_LIST_SIZE: c_int = 10;
pub const DML2_PMO_STUTTER_CANDIDATE_LIST_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_implicit_svp_meta {
    pub valid: bool,
    pub v_active: c_ulong,
    pub v_total: c_ulong,
    pub v_front_porch: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pstate_per_method_common_meta {
// generic params
    pub allow_start_otg_vline: c_int,
    pub allow_end_otg_vline: c_int,
// scheduling params
    pub allow_time_us: double,
    pub disallow_time_us: double,
    pub period_us: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pstate_meta {
    pub valid: bool,
    pub otg_vline_time_us: double,
    pub scheduling_delay_otg_vlines: c_int,
    pub vertical_interrupt_ack_delay_otg_vlines: c_int,
    pub allow_to_target_delay_otg_vlines: c_int,
    pub contention_delay_otg_vlines: c_int,
    pub min_allow_width_otg_vlines: c_int,
    pub nom_vtotal: c_int,
    pub vblank_start: c_int,
    pub nom_refresh_rate_hz: double,
    pub nom_frame_time_us: double,
    pub max_vtotal: c_int,
    pub min_refresh_rate_hz: double,
    pub max_frame_time_us: double,
    pub blackout_otg_vlines: c_int,
    pub max_allow_delay_otg_vlines: c_int,
    pub nom_vblank_time_us: double,
    pub max_vactive_det_fill_delay_us: double,
    pub vactive_latency_hiding_us: double,
    pub reserved_vblank_required_us: double,
    pub max_vactive_det_fill_delay_otg_vlines: c_int,
    pub reserved_blank_required_vlines: c_int,
    pub common: dml2_pstate_per_method_common_meta,
    pub method_vactive: },
    pub common: dml2_pstate_per_method_common_meta,
    pub method_vblank: },
    pub programming_delay_otg_vlines: c_int,
    pub df_throttle_delay_otg_vlines: c_int,
    pub prefetch_to_mall_delay_otg_vlines: c_int,
    pub phantom_vactive: c_ulong,
    pub phantom_vfp: c_ulong,
    pub phantom_vtotal: c_ulong,
    pub common: dml2_pstate_per_method_common_meta,
    pub method_subvp: },
    pub required: int programming_delay_otg_vlines; // DMCUB <-> PMFW delays + any DMCUB/PMFW programming delays,
    pub allow: int pmfw_throttle_delay_otg_vlines; // PMFW time it takes to throttle other clients + assert DF P-State,
    pub common: dml2_pstate_per_method_common_meta,
    pub method_alternate: },
    pub programming_delay_otg_vlines: c_int,
    pub stretched_vtotal: c_int,
    pub common: dml2_pstate_per_method_common_meta,
    pub method_drr: },
}

// mask of synchronized timings by stream index
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_synchronized_timing_groups {
    pub num_timing_groups: c_uint,
    pub synchronized_timing_group_masks: [c_uint; DML2_MAX_PLANES],
    pub group_is_drr_enabled: [bool; DML2_MAX_PLANES],
    pub group_is_drr_active: [bool; DML2_MAX_PLANES],
    pub group_line_time_us: [double; DML2_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_stage3_state {
    pub performed: bool,
    pub success: bool,
// The pstate support mode for each plane
// The number of valid elements == display_cfg.num_planes
// The indexing of pstate_switch_modes matches plane_descriptors[]
    pub pstate_switch_modes: [dml2_pstate_method; DML2_MAX_PLANES],
// Meta-data for implicit SVP generation, indexed by stream index
    pub stream_svp_meta: [dml2_implicit_svp_meta; DML2_MAX_PLANES],
// Meta-data for FAMS2
    pub fams2_required: bool,
    pub stream_pstate_meta: [dml2_pstate_meta; DML2_MAX_PLANES],
    pub min_clk_index_for_latency: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_stage4_state {
    pub performed: bool,
    pub success: bool,
    pub unoptimizable_streams: [bool; DML2_MAX_DCN_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_stage5_state {
    pub performed: bool,
    pub success: bool,
    pub optimal_reserved_time_in_vblank_us: bool,
    pub vblank_includes_z8_optimization: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_configuation_with_meta {
    pub display_config: dml2_display_cfg,
    pub mode_support_result: dml2_core_mode_support_result,
// Stage 1 = Min Clocks for Latency
    pub stage1: dml2_optimization_stage1_state,
// Stage 2 = MCache
    pub stage2: dml2_optimization_stage2_state,
// Stage 3 = UCLK PState
    pub stage3: dml2_optimization_stage3_state,
// Stage 4 = Vmin
    pub stage4: dml2_optimization_stage4_state,
// Stage 5 = Stutter
    pub stage5: dml2_optimization_stage5_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_pstate_strategy {
    pub per_stream_pstate_method: [dml2_pstate_method; DML2_MAX_PLANES],
    pub allow_state_increase: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_validation_result {
    pub is_mode_support_valid: bool,
    pub is_prefetch_valid: bool,
    pub mode_support: dml2_core_mode_support_result,
    pub is_mcache_allocation_valid: bool,
    pub mcache_allocations: [dml2_mcache_surface_allocation; DML2_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_change {
    pub 0: bool sop_index : 1; // bit,
    pub 1: bool mpc_combine_overrides :,
    pub 1: bool odm_combine_overrides :,
    pub 1: bool reserved_vblank_time :,
    pub 1: bool mcache_allocation :,
    pub 1: bool uclk_pstate_method :,
    pub 1: bool fclk_pstate_support :,
    pub 1: bool stutter_support :,
    pub 8: bool dcfclk_override : 1; // bit,
    pub 1: bool ppt_temp_read_pstate_support :,
    pub 6: unsigned char reserved :,
    pub bits: },
    pub raw: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_worksheet {
    pub orig_dispcfg: *const dml2_display_cfg,
    pub plane: unsigned int timing_group_ids[DML2_MAX_PLANES]; // per,
    pub timing_group_count: c_uint,
    pub is_default_pipe_usage_attempted: bool,
    pub is_single_stream_odm_case: bool,
    pub per_plane_status: [bool; DML2_MAX_PLANES],
    pub pipe_vp_startx: [c_int; DML2_MAX_DCN_PIPES],
    pub pipe_vp_endx: [c_int; DML2_MAX_DCN_PIPES],
    pub plane0: },
    pub pipe_vp_startx: [c_int; DML2_MAX_DCN_PIPES],
    pub pipe_vp_endx: [c_int; DML2_MAX_DCN_PIPES],
    pub plane1: },
    pub mcache: },
    pub stream_plane_mask: [c_uint; DML2_MAX_PLANES],
    pub stream: dml2_pstate_meta stream_pstate_meta[DML2_MAX_PLANES]; // per,
// Meta-data for implicit SVP generation, indexed by stream index
    pub stream_svp_meta: [dml2_implicit_svp_meta; DML2_MAX_PLANES],
    pub pstate_strategy_candidates: [dml2_pmo_pstate_strategy; DML2_PMO_PSTATE_CANDIDATE_LIST_SIZE],
    pub num_pstate_candidates: c_int,
    pub cur_pstate_candidate: c_int,
// Initial value of reserved vblank time as pstate optimize may overwrite and clear current
    pub init_reserved_vblank_time_ns: [c_long; DML2_MAX_PLANES],
    pub plane: unsigned int init_max_vactive_det_fill_delay_us[DML2_MAX_PLANES]; // per,
    pub uclk_pstate: },
    pub is_initialized: bool,
    pub unoptimizable_streams: [bool; DML2_MAX_DCN_PIPES],
    pub init_odms_used: [c_uint; DML2_MAX_DCN_PIPES],
    pub vmin: },
    pub init_reserved_vblank_time_ns: [c_long; DML2_MAX_PLANES],
    pub should_optimize_z8_stutter: bool,
    pub is_z8_stutter_attempted: bool,
    pub should_optimize_stutter: bool,
    pub is_stutter_attempted: bool,
    pub stutter: },
    pub passing_index: c_uint,
    pub failing_index: c_uint,
    pub is_index0_tested: bool,
    pub qos: },
    pub max_available_bandwidth_kbps: double,
    pub dcfclk_vmin: },
    pub is_attempted: bool,
    pub fclk_ppt_temp_read_pstate: },
//
// unified structure for storing current optimization tuning variables. Please only store the final tuning knobs
// we should see exactly what will be optimized in display solution at a quick glance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_config {
    pub min_sop_index: c_uint,
    pub plane: unsigned int mpc_combine_overrides[DML2_MAX_PLANES]; // per,
    pub stream: unsigned int odm_combine_overrides[DML2_MAX_PLANES]; // per,
    pub reserved_vblank_time_ns: [c_long; DML2_MAX_PLANES],
    pub mcache_allocations: [dml2_mcache_surface_allocation; DML2_MAX_PLANES],
    pub uclk_pstate_support: bool,
    pub plane: dml2_pstate_method uclk_pstate_switch_modes[DML2_MAX_PLANES]; // per,
    pub plane: int max_vactive_det_fill_delay_us[DML2_MAX_PLANES][dml2_pstate_type_count]; // per,
// Meta-data for FAMS2
    pub fams2_required: bool,
    pub legacy_pstate_info_for_dmu: bool,
    pub stream: dml2_pstate_meta stream_pstate_meta[DML2_MAX_PLANES]; // per,
    pub fclk_pstate_support: bool,
    pub ppt_temp_read_support: bool,
    pub stutter_support_in_vblank: bool,
    pub z8_stutter_support_in_vblank: bool,
    pub enable_vmin_dcfclk: bool,
    pub config: },
// changes that have not yet been validated
    pub unvalidated_change: dml2_optimization_change,
    pub cur: },
// post validation
    pub validation_result: dml2_validation_result,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_solution {
    pub orig_dispcfg: *const dml2_display_cfg,
// current display configuration
    pub dispcfg: dml2_display_cfg,
    pub plane: unsigned int timing_group_ids[DML2_MAX_PLANES]; // per,
    pub timing_group_count: c_uint,
// additional DML internally decided configurations
    pub sop_constraint: dml2_sop_constraint,
    pub mcache_allocations: [dml2_mcache_surface_allocation; DML2_MAX_PLANES],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_uclk_pstate_params {
    pub support: bool,
// Uclk pstate related
    pub pstate_switch_modes: [dml2_pstate_method; DML2_MAX_PLANES],
// Meta-data for FAMS2
    pub fams2_required: bool,
    pub legacy_pstate_info_for_dmu: bool,
    pub stream: dml2_pstate_meta stream_pstate_meta[DML2_MAX_PLANES]; // per,
    pub uclk_pstate_params: },
    pub fclk_pstate_support: bool,
    pub ppt_temp_read_support: bool,
    pub stutter_support_in_vblank: bool,
    pub z8_stutter_support_in_vblank: bool,
// pre-validation states
    pub unvalidated_change: dml2_optimization_change,
// post validation
    pub validation_result: dml2_validation_result,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_stage_optimizer {
    pub pmo: *const dml2_pmo_instance,
//
// to optimize stack memory usage, large local variables are pre-allocated in this heap memory. The scope of
// func_locals is bounded by each optimizer's function defined in the union. To share states across an
// optimizer's functions, define it in the dedicated optimizer state in optimization worksheet.
//
    pub func_locals: *mut dml2_stage_optimizer_function_locals,
//
// init interface builds the initial states associated with the current stage optimizer into the worksheet. It
// is for state initialization only. It should not apply new optimization or cause changes to current validation
// result. It is safe to assume that the worksheet passed in or exited from this interface is always validated.
//
    pub worksheet): *mut *mut *mut void (init)(struct dml2_pmo_stage_optimizer stage, struct dml2_optimization_worksheet,
//
// optimize_next interface controls current optimization's stop conditions. When the interface returns false, it
// means the stage optimizer no longer needs to attempt further optimization. The current worksheet should be
// left unmodified. When the interface returns true, it means the stage optimizer applied new optimization to
// the worksheet. DML top will need to validate and test permissibility again. The worksheet passed in is based
// off the optimization decision from last attempt. It may or may not be validated or permissible. It is upto
// DML top to keep track of the last valid permissible worksheet. This interface is also responsible to clear
// corresponding valid bits in worksheet's validation result based on what optimization it gets applied. When
// the valid bits are cleared, it will be revalidated by top. Otherwise, DML top will assume it is safe to skip
// certain re-validations based on the remaining valid bits. Stage optimizers should clear only the necessary
// valid bits based on the optimization applied to speed up the process.
//
    pub worksheet): *mut *mut *mut bool (optimize_next)(struct dml2_pmo_stage_optimizer stage, struct dml2_optimization_worksheet,
//
// test_permissibility interface should only check against current optimizer policy specific
// minimum requirements. Test permissibility result is orthogonal to validation result. It is
// safe to assume the worksheet constant passed in is always validated. The interface checks if
// the validated result fulfills the minimum requirements additionally imposed by current stage
// optimizer in order to consider the current optimization as a potential candidate. Stage
// optimizer may still attempt further optimization even if the current one is permissible.
//
    pub worksheet): *const dml2_optimization_worksheet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_mode_support_in_out {
//
// Inputs
//
    pub instance: *mut dml2_core_instance,
    pub display_cfg: *const display_configuation_with_meta,
    pub min_clk_table: *mut dml2_mcg_min_clock_table,
    pub min_clk_index: c_int,
//
// Outputs
//
    pub mode_support_result: dml2_core_mode_support_result,
// Inputs
    pub display_cfg: *mut dml_display_cfg_st,
// Outputs
    pub support_info: *mut dml_mode_support_info_st,
    pub out_lowest_state_idx: c_uint,
    pub min_fclk_khz: c_uint,
    pub min_dcfclk_khz: c_uint,
    pub min_dram_speed_mts: c_uint,
    pub min_socclk_khz: c_uint,
    pub min_dscclk_khz: c_uint,
    pub min_dtbclk_khz: c_uint,
    pub min_phyclk_khz: c_uint,
    pub legacy: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_mode_programming_in_out {
//
// Inputs
//
    pub instance: *mut dml2_core_instance,
    pub display_cfg: *const display_configuation_with_meta,
    pub cfg_support_info: *const core_display_cfg_support_info,
//
// Outputs (also Input the clk freq are also from programming struct)
//
    pub programming: *mut dml2_display_cfg_programming,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_populate_informative_in_out {
//
// Inputs
//
    pub instance: *mut dml2_core_instance,
// If this is set, then the mode was supported, and mode programming
// was successfully run.
// Otherwise, mode programming was not run, because mode support failed.
    pub mode_is_supported: bool,
//
// Outputs
//
    pub programming: *mut dml2_display_cfg_programming,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_calculate_mcache_allocation_in_out {
//
// Inputs
//
    pub instance: *mut dml2_core_instance,
    pub plane_descriptor: *const dml2_plane_parameters,
    pub plane_index: c_uint,
//
// Outputs
//
    pub mcache_allocation: *mut dml2_mcache_surface_allocation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_internal_state_inputs {
    pub dummy: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_internal_state_intermediates {
    pub dummy: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_calculate_mp_context {
    pub display_cfg: *const dml2_display_cfg,
    pub ip: *const dml2_core_ip_params,
    pub soc_bb: *const dml2_utm_soc_bb,
    pub ms: *const dml2_core_internal_mode_support,
    pub dummies: *mut dml2_core_calcs_mode_programming_locals,
    pub func_params: *mut dml2_core_internal_scratch,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_calculate_ms_context {
    pub display_cfg: *const dml2_display_cfg,
    pub ip: *const dml2_core_ip_params,
    pub soc_bb: *const dml2_utm_soc_bb,
    pub clock_adjuster: *const dml2_clock_granularity_adjuster,
    pub dummies: *mut dml2_core_calcs_mode_support_locals,
    pub func_params: *mut dml2_core_internal_scratch,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_mode_support_locals {
    pub mode_support_ex_params: dml2_core_calcs_mode_support_ex,
    pub calc_ms_ctx: dml2_core_calculate_ms_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_mode_programming_locals {
    pub mode_programming_ex_params: dml2_core_calcs_mode_programming_ex,
    pub calc_mp_ctx: dml2_core_calculate_mp_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_scratch {
    pub mode_support_locals: dml2_core_mode_support_locals,
    pub mode_programming_locals: dml2_core_mode_programming_locals,
    pub main_stream_index_from_svp_stream_index: [c_int; DML2_MAX_PLANES],
    pub svp_stream_index_from_main_stream_index: [c_int; DML2_MAX_PLANES],
    pub main_plane_index_to_phantom_plane_index: [c_int; DML2_MAX_PLANES],
    pub phantom_plane_index_to_main_plane_index: [c_int; DML2_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_core_instance {
    pub minimum_clock_table: *mut dml2_mcg_min_clock_table,
    pub utm_soc_bb: *const dml2_utm_soc_bb,
    pub clock_adjuster: *const dml2_clock_granularity_adjuster,
    pub inputs: dml2_core_internal_state_inputs,
    pub intermediates: dml2_core_internal_state_intermediates,
    pub scratch: dml2_core_scratch,
    pub in_out): *mut *mut bool (initialize)(struct dml2_core_initialize_in_out,
    pub in_out): *mut *mut bool (mode_support)(struct dml2_core_mode_support_in_out,
    pub validation_result): *mut dml2_validation_result,
    pub programming): *mut dml2_display_cfg_programming,
    pub in_out): *mut *mut bool (mode_programming)(struct dml2_core_mode_programming_in_out,
    pub in_out): *mut *mut bool (populate_informative)(struct dml2_core_populate_informative_in_out,
    pub in_out): *mut *mut bool (calculate_mcache_allocation)(struct dml2_calculate_mcache_allocation_in_out,
    pub mode_lib: dml2_core_internal_display_mode_lib,
    pub clean_me_up: },
}

//
// DML2 PMO Types and Interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_initialize_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub soc_bb: *mut dml2_soc_bb,
    pub ip_caps: *mut dml2_ip_capabilities,
    pub options: *mut dml2_pmo_options,
    pub mcg_clock_table_size: c_int,
    pub utm_soc_bb: *const dml2_utm_soc_bb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_optimize_dcc_mcache_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub display_config: *const dml2_display_cfg,
    pub dcc_mcache_supported: *mut bool,
    pub cfg_support_info: *mut core_display_cfg_support_info,
//
// Output
//
    pub optimized_display_cfg: *mut dml2_display_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_init_for_vmin_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_test_for_vmin_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub display_config: *const display_configuation_with_meta,
    pub vmin_limits: *const dml2_soc_vmin_clock_limits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_optimize_for_vmin_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
//
// Output
//
    pub optimized_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_init_for_pstate_support_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_test_for_pstate_support_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_optimize_for_pstate_support_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
    pub last_candidate_failed: bool,
//
// Output
//
    pub optimized_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_init_for_stutter_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_test_for_stutter_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_optimize_for_stutter_in_out {
//
// Input
//
    pub instance: *mut dml2_pmo_instance,
    pub base_display_config: *mut display_configuation_with_meta,
    pub last_candidate_failed: bool,
//
// Output
//
    pub optimized_display_config: *mut display_configuation_with_meta,
}

pub const PMO_DCN4_MAX_DISPLAYS: c_int = 4;
pub const PMO_DCN4_MAX_NUM_VARIANTS: c_int = 2;
pub const PMO_DCN4_MAX_BASE_STRATEGIES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_scheduling_check_locals {
    pub group_common_pstate_meta: [dml2_pstate_per_method_common_meta; DML2_MAX_PLANES],
    pub sorted_group_gtl_disallow_index: [c_uint; DML2_MAX_PLANES],
    pub sorted_group_gtl_period_index: [c_uint; DML2_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_scratch {
    pub reserved_time_candidates: [double; DML2_MAX_PLANES][DML2_PMO_LEGACY_PREFETCH_MAX_TWAIT_OPTIONS],
    pub reserved_time_candidates_count: [c_int; DML2_MAX_PLANES],
    pub current_candidate: [c_int; DML2_MAX_PLANES],
    pub min_latency_index: c_int,
    pub max_latency_index: c_int,
    pub cur_latency_index: c_int,
    pub stream_mask: c_int,
    pub pmo_dcn3: },
    pub 2]: *mut *mut *mut *mut dml2_pmo_pstate_strategy expanded_override_strategy_list[2  2  2,
    pub num_expanded_override_strategies: c_uint,
    pub pstate_strategy_candidates: [dml2_pmo_pstate_strategy; DML2_PMO_PSTATE_CANDIDATE_LIST_SIZE],
    pub num_pstate_candidates: c_int,
    pub cur_pstate_candidate: c_int,
    pub stream_plane_mask: [c_uint; DML2_MAX_PLANES],
    pub stream_vactive_capability_mask: c_uint,
    pub min_latency_index: c_int,
    pub max_latency_index: c_int,
    pub cur_latency_index: c_int,
// Stores all the implicit SVP meta information indexed by stream index of the display
// configuration under inspection, built at optimization stage init
    pub stream_svp_meta: [dml2_implicit_svp_meta; DML2_MAX_PLANES],
    pub stream_pstate_meta: [dml2_pstate_meta; DML2_MAX_PLANES],
    pub optimal_vblank_reserved_time_for_stutter_us: [c_uint; DML2_PMO_STUTTER_CANDIDATE_LIST_SIZE],
    pub num_stutter_candidates: c_uint,
    pub cur_stutter_candidate: c_uint,
    pub z8_vblank_optimizable: bool,
// mask of synchronized timings by stream index
    pub num_timing_groups: c_uint,
    pub synchronized_timing_group_masks: [c_uint; DML2_MAX_PLANES],
    pub group_is_drr_enabled: [bool; DML2_MAX_PLANES],
    pub group_is_drr_active: [bool; DML2_MAX_PLANES],
    pub group_line_time_us: [double; DML2_MAX_PLANES],
// scheduling check locals
    pub group_common_pstate_meta: [dml2_pstate_per_method_common_meta; DML2_MAX_PLANES],
    pub sorted_group_gtl_disallow_index: [c_uint; DML2_MAX_PLANES],
    pub sorted_group_gtl_period_index: [c_uint; DML2_MAX_PLANES],
    pub group_phase_offset: [double; DML2_MAX_PLANES],
    pub pmo_dcn4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union dml2_stage_optimizer_function_locals {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_stage_optimizer_uclk_pstate_init_locals {
    pub allow_delay_us: double,
    pub blackout_us: double,
    pub watermark_us: double,
    pub stream_vactive_capability_mask: c_uint,
    pub 2]: *mut *mut *mut *mut dml2_pmo_pstate_strategy expanded_override_strategy_list[2  2  2,
    pub num_expanded_override_strategies: c_uint,
// mask of synchronized timings by stream index
    pub synchronized_timing_groups: dml2_pmo_synchronized_timing_groups,
// scheduling check locals
    pub scheduling_check_locals: dml2_scheduling_check_locals,
    pub uclk_pstate_init: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_stage_optimizer_fclk_ppt_temp_read_pstate_optimize_locals {
    pub pstate_blackout_us: double,
    pub pstate_watermark_us: double,
    pub pstate_allow_delay_us: double,
    pub per_stream_pstate_meta: [dml2_pstate_meta; DML2_MAX_PLANES],
    pub per_stream_pstate_method: [dml2_pstate_method; DML2_MAX_PLANES],
    pub scheduling_check_locals: dml2_scheduling_check_locals,
    pub fclk_ppt_temp_read_pstate_optimize: },
    pub func_locals: },
    pub pmo_dcn5: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_init_data {
// populated once during initialization
    pub 2]: *mut *mut dml2_pmo_pstate_strategy expanded_strategy_list_1_display[PMO_DCN4_MAX_BASE_STRATEGIES,
    pub 4]: *mut *mut *mut dml2_pmo_pstate_strategy expanded_strategy_list_2_display[PMO_DCN4_MAX_BASE_STRATEGIES  4,
    pub 6]: *mut *mut *mut *mut dml2_pmo_pstate_strategy expanded_strategy_list_3_display[PMO_DCN4_MAX_BASE_STRATEGIES  6  6,
    pub 8]: *mut *mut *mut *mut *mut dml2_pmo_pstate_strategy expanded_strategy_list_4_display[PMO_DCN4_MAX_BASE_STRATEGIES  8  8  8,
    pub num_expanded_strategies_per_list: [c_uint; PMO_DCN4_MAX_DISPLAYS],
    pub pmo_dcn4: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_pmo_stage_index {
    dml2_pmo_stage_index_start = 0,
    dml2_pmo_stage_index_mcache = dml2_pmo_stage_index_start,
    dml2_pmo_stage_index_uclk_pstate,
    dml2_pmo_stage_index_qos,
    dml2_pmo_stage_index_vmin,
    dml2_pmo_stage_index_stutter,
    dml2_pmo_stage_index_vmin_dcfclk,
    dml2_pmo_stage_index_fclk_ppt_temp_read_pstate,
    dml2_pmo_stage_index_max,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_instance {
    pub soc_bb: *mut dml2_soc_bb,
    pub ip_caps: *mut dml2_ip_capabilities,
    pub options: *mut dml2_pmo_options,
    pub disp_clk_vmin_threshold: c_int,
    pub mpc_combine_limit: c_int,
    pub odm_combine_limit: c_int,
    pub mcg_clock_table_size: c_int,
    pub utm_soc_bb: *const dml2_utm_soc_bb,
    pub prefetch_end_to_mall_start_us: c_int,
    pub fw_processing_delay_us: c_int,
    pub refresh_rate_limit_min: c_int,
    pub refresh_rate_limit_max: c_int,
    pub subvp: },
    pub v1: },
    pub refresh_rate_limit_min: c_int,
    pub refresh_rate_limit_max: c_int,
    pub subvp: },
    pub refresh_rate_limit_min: c_int,
    pub refresh_rate_limit_max: c_int,
    pub drr: },
    pub v2: },
    pub fams_params: },
    pub in_out): *mut *mut bool (initialize)(struct dml2_pmo_initialize_in_out,
    pub in_out): *mut *mut bool (optimize_dcc_mcache)(struct dml2_pmo_optimize_dcc_mcache_in_out,
    pub in_out): *mut *mut bool (init_for_vmin)(struct dml2_pmo_init_for_vmin_in_out,
    pub in_out): *mut *mut bool (test_for_vmin)(struct dml2_pmo_test_for_vmin_in_out,
    pub in_out): *mut *mut bool (optimize_for_vmin)(struct dml2_pmo_optimize_for_vmin_in_out,
    pub in_out): *mut *mut bool (init_for_uclk_pstate)(struct dml2_pmo_init_for_pstate_support_in_out,
    pub in_out): *mut *mut bool (test_for_uclk_pstate)(struct dml2_pmo_test_for_pstate_support_in_out,
    pub in_out): *mut *mut bool (optimize_for_uclk_pstate)(struct dml2_pmo_optimize_for_pstate_support_in_out,
    pub in_out): *mut *mut bool (init_for_stutter)(struct dml2_pmo_init_for_stutter_in_out,
    pub in_out): *mut *mut bool (test_for_stutter)(struct dml2_pmo_test_for_stutter_in_out,
    pub in_out): *mut *mut bool (optimize_for_stutter)(struct dml2_pmo_optimize_for_stutter_in_out,
//
// obtain a list of mandatory stage optimizers ordered by priority. Caller must perform stage optimization in
// the same order.
// return - stage optimizer count
//
    pub optimers): *mut dml2_pmo_stage_optimizer,
//
// obtain a list of option stage optimizers ordered by priority, Caller must perform stage optimization in the
// same order. Caller must complete stage optimization for all mandatory stage optimizers before performing
// optional stage optimization.
// return - stage optimizer count
//
    pub optimers): *mut dml2_pmo_stage_optimizer,
//
// initialize an optimization worksheet based on the display config passed in.
//
    pub worksheet): *mut dml2_optimization_worksheet,
//
// convert an optimization worksheet to a display solution.
//
    pub solution): *mut dml2_display_solution,
//
// when validation is completed with an updated worksheet's validation result, PMO needs to reset pre validation
// states stored in worksheet.
//
    pub worksheet): *mut dml2_optimization_worksheet,
//
// perform a mini validate solution to rule out common optimization config problems after optimize_next is
// called. This interface is a performance optimization to avoid of performing expensive full validate solution
// for common optimization problems. It also generalizes sanity check for all stage optimizers. So the concern
// of sanity check optimization config is isolated out of each stage optimizer.
//
    pub worksheet): *const dml2_optimization_worksheet,
    pub init_data: dml2_pmo_init_data,
    pub scratch: dml2_pmo_scratch,
    pub stage_optimizers: [dml2_pmo_stage_optimizer; dml2_pmo_stage_index_max],
}

//
// DML2 MCache Types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct top_mcache_validate_admissability_in_out {
    pub dml2_instance: *mut dml2_instance,
    pub display_cfg: *const dml2_display_cfg,
    pub cfg_support_info: *const core_display_cfg_support_info,
    pub mcache_allocations: *mut dml2_mcache_surface_allocation,
    pub per_plane_status: [bool; DML2_MAX_PLANES],
    pub mode_support_info: *const dml_mode_support_info_st,
    pub legacy: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct top_mcache_assign_ids_in_out {
//
// Input
//
    pub mcache_allocations: *const dml2_mcache_surface_allocation,
    pub plane_count: c_int,
    pub per_pipe_viewport_x_start: [c_int; DML2_MAX_PLANES][DML2_MAX_DCN_PIPES],
    pub per_pipe_viewport_x_end: [c_int; DML2_MAX_PLANES][DML2_MAX_DCN_PIPES],
    pub pipe_count_per_plane: [c_int; DML2_MAX_PLANES],
    pub pipe/hubp: *mut *mut dml2_display_mcache_regs current_mcache_regs[DML2_MAX_PLANES][DML2_MAX_DCN_PIPES]; //One set per,
//
// Output
//
    pub pipe/hubp: dml2_display_mcache_regs mcache_regs[DML2_MAX_PLANES][DML2_MAX_DCN_PIPES]; //One set per,
    pub mcache_programming: *mut dml2_build_mcache_programming_in_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct top_mcache_calc_mcache_count_and_offsets_in_out {
//
// Inputs
//
    pub dml2_instance: *mut dml2_instance,
    pub display_config: *const dml2_display_cfg,
//
// Outputs
//
    pub mcache_allocations: *mut dml2_mcache_surface_allocation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct top_mcache_assign_global_mcache_ids_in_out {
//
// Inputs/Outputs
//
    pub allocations: *mut dml2_mcache_surface_allocation,
    pub num_allocations: c_int,
}

//
// DML2 Top Types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_initialize_instance_locals {
    pub dummy: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_init_function_locals {
    pub init_params: dml2_pmo_init_for_pstate_support_in_out,
    pub uclk_pstate: },
    pub stutter_params: dml2_pmo_init_for_stutter_in_out,
    pub stutter: },
    pub init_params: dml2_pmo_init_for_vmin_in_out,
    pub vmin: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_test_function_locals {
    pub calc_mcache_count_params: top_mcache_calc_mcache_count_and_offsets_in_out,
    pub assign_global_mcache_ids_params: top_mcache_assign_global_mcache_ids_in_out,
    pub validate_admissibility_params: top_mcache_validate_admissability_in_out,
    pub test_mcache: },
    pub pmo_test_vmin_params: dml2_pmo_test_for_vmin_in_out,
    pub test_vmin: },
    pub test_params: dml2_pmo_test_for_pstate_support_in_out,
    pub uclk_pstate: },
    pub stutter_params: dml2_pmo_test_for_stutter_in_out,
    pub stutter: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_optimize_function_locals {
    pub optimize_mcache_params: dml2_pmo_optimize_dcc_mcache_in_out,
    pub optimize_mcache: },
    pub pmo_optimize_vmin_params: dml2_pmo_optimize_for_vmin_in_out,
    pub optimize_vmin: },
    pub optimize_params: dml2_pmo_optimize_for_pstate_support_in_out,
    pub uclk_pstate: },
    pub stutter_params: dml2_pmo_optimize_for_stutter_in_out,
    pub stutter: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_optimization_phase_locals {
    pub cur_candidate_display_cfg: display_configuation_with_meta,
    pub next_candidate_display_cfg: display_configuation_with_meta,
    pub mode_support_params: dml2_core_mode_support_in_out,
    pub init_function_locals: dml2_optimization_init_function_locals,
    pub test_function_locals: dml2_optimization_test_function_locals,
    pub optimize_function_locals: dml2_optimization_optimize_function_locals,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_check_mode_supported_locals {
    pub display_cfg_working_copy: dml2_display_cfg,
    pub mode_support_params: dml2_core_mode_support_in_out,
    pub optimization_phase_locals: dml2_optimization_phase_locals,
    pub base_display_config_with_meta: display_configuation_with_meta,
    pub optimized_display_config_with_meta: display_configuation_with_meta,
    pub dppm_map_mode_params: dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimization_init_function_params {
    pub locals: *mut dml2_optimization_init_function_locals,
    pub dml: *mut dml2_instance,
    pub display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimization_test_function_params {
    pub locals: *mut dml2_optimization_test_function_locals,
    pub dml: *mut dml2_instance,
    pub display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimization_optimize_function_params {
    pub last_candidate_supported: bool,
    pub locals: *mut dml2_optimization_optimize_function_locals,
    pub dml: *mut dml2_instance,
    pub display_config: *mut display_configuation_with_meta,
    pub optimized_display_config: *mut display_configuation_with_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimization_phase_params {
    pub dml: *mut dml2_instance,
    pub Configuration: *const *const display_configuation_with_meta display_config; // Initial Display,
    pub complete: *const *const *const bool (init_function)(struct optimization_init_function_params params); // Test function to determine optimization is,
    pub complete: *const *const *const bool (test_function)(struct optimization_test_function_params params); // Test function to determine optimization is,
    pub configuration: *const *const *const bool (optimize_function)(struct optimization_optimize_function_params params); // Function which produces a more optimized display,
    pub configuration: *mut *mut display_configuation_with_meta optimized_display_config; // The optimized display,
    pub all_or_nothing: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_build_mode_programming_locals {
    pub mode_support_params: dml2_core_mode_support_in_out,
    pub mode_programming_params: dml2_core_mode_programming_in_out,
    pub informative_params: dml2_core_populate_informative_in_out,
    pub optimize_mcache_params: dml2_pmo_optimize_dcc_mcache_in_out,
    pub base_display_config_with_meta: display_configuation_with_meta,
    pub optimized_display_config_with_meta: display_configuation_with_meta,
    pub dppm_map_mode_params: dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    pub dppm_map_watermarks_params: dml2_dpmm_map_watermarks_params_in_out,
    pub optimization_phase_locals: dml2_optimization_phase_locals,
    pub min_clock_for_latency_phase: optimization_phase_params,
    pub mcache_phase: optimization_phase_params,
    pub uclk_pstate_phase: optimization_phase_params,
    pub vmin_phase: optimization_phase_params,
    pub stutter_phase: optimization_phase_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_legacy_core_build_mode_programming_wrapper_locals {
    pub mode_support_params: dml2_core_mode_support_in_out,
    pub mode_programming_params: dml2_core_mode_programming_in_out,
    pub informative_params: dml2_core_populate_informative_in_out,
    pub calc_mcache_count_params: top_mcache_calc_mcache_count_and_offsets_in_out,
    pub validate_admissibility_params: top_mcache_validate_admissability_in_out,
    pub mcache_allocations: [dml2_mcache_surface_allocation; DML2_MAX_PLANES],
    pub assign_global_mcache_ids_params: top_mcache_assign_global_mcache_ids_in_out,
    pub optimize_mcache_params: dml2_pmo_optimize_dcc_mcache_in_out,
    pub optimized_display_cfg: dml2_display_cfg,
    pub core_support_info: core_display_cfg_support_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_top_mcache_verify_mcache_size_locals {
    pub calc_mcache_params: dml2_calculate_mcache_allocation_in_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_top_mcache_validate_admissability_locals {
    pub pipe_vp_startx: [c_int; DML2_MAX_DCN_PIPES],
    pub pipe_vp_endx: [c_int; DML2_MAX_DCN_PIPES],
    pub plane0: },
    pub pipe_vp_startx: [c_int; DML2_MAX_DCN_PIPES],
    pub pipe_vp_endx: [c_int; DML2_MAX_DCN_PIPES],
    pub plane1: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_top_display_cfg_support_info {
    pub display_config: *const dml2_display_cfg,
    pub core_info: core_display_cfg_support_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_top_funcs {
    pub in_out): *mut *mut bool (check_mode_supported)(struct dml2_check_mode_supported_in_out,
    pub in_out): *mut *mut bool (build_mode_programming)(struct dml2_build_mode_programming_in_out,
    pub in_out): *mut *mut bool (build_mcache_programming)(struct dml2_build_mcache_programming_in_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_instance {
    pub project_id: dml2_project_id,
    pub core_instance: dml2_core_instance,
    pub mcg_instance: dml2_mcg_instance,
    pub dpmm_instance: dml2_dpmm_instance,
    pub pmo_instance: dml2_pmo_instance,
    pub soc_bbox: dml2_soc_bb,
    pub ip_caps: dml2_ip_capabilities,
    pub min_clk_table: dml2_mcg_min_clock_table,
    pub utm_soc_bb: dml2_utm_soc_bb,
    pub clock_adjuster: dml2_clock_granularity_adjuster,
    pub pmo_options: dml2_pmo_options,
    pub funcs: dml2_top_funcs,
    pub initialize_instance_locals: dml2_initialize_instance_locals,
    pub mcache_verify_mcache_size_locals: dml2_top_mcache_verify_mcache_size_locals,
    pub mcache_validate_admissability_locals: dml2_top_mcache_validate_admissability_locals,
    pub check_mode_supported_locals: dml2_check_mode_supported_locals,
    pub build_mode_programming_locals: dml2_build_mode_programming_locals,
    pub worksheet: dml2_optimization_worksheet,
    pub worksheet_backup: dml2_optimization_worksheet,
    pub solution: dml2_display_solution,
    pub scratch: },
    pub legacy_core_build_mode_programming_wrapper_locals: dml2_legacy_core_build_mode_programming_wrapper_locals,
    pub scratch: },
    pub legacy: },
}
