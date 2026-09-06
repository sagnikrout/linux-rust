//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/dml_top_types.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_project_id {
    dml2_project_invalid = 0,
    dml2_project_dcn4x_stage1,
    dml2_project_dcn4x_stage2,
    dml2_project_dcn4x_stage2_auto_drr_svp,
    dml2_project_dcn42,
    dml2_project_dcn4x_utm,
    dml2_project_dcn5x,
    dml2_project_dcn5x_utm,
    dml2_project_dcn6x_soc_var_a,
    dml2_project_dcn6x_soc_var_b,
    dml2_project_dcn6x = dml2_project_dcn6x_soc_var_b,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_pstate_change_support {
    dml2_pstate_change_vactive = 0,
    dml2_pstate_change_vblank = 1,
    dml2_pstate_change_vblank_and_vactive = 2,
    dml2_pstate_change_drr = 3,
    dml2_pstate_change_mall_svp = 4,
    dml2_pstate_change_mall_full_frame = 6,
    dml2_pstate_change_unsupported = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_output_type_and_rate__type {
    dml2_output_type_unknown = 0,
    dml2_output_type_dp = 1,
    dml2_output_type_edp = 2,
    dml2_output_type_dp2p0 = 3,
    dml2_output_type_hdmi = 4,
    dml2_output_type_hdmifrl = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_output_type_and_rate__rate {
    dml2_output_rate_unknown = 0,
    dml2_output_rate_dp_rate_hbr = 1,
    dml2_output_rate_dp_rate_hbr2 = 2,
    dml2_output_rate_dp_rate_hbr3 = 3,
    dml2_output_rate_dp_rate_uhbr10 = 4,
    dml2_output_rate_dp_rate_uhbr13p5 = 5,
    dml2_output_rate_dp_rate_uhbr20 = 6,
    dml2_output_rate_hdmi_rate_3x3 = 7,
    dml2_output_rate_hdmi_rate_6x3 = 8,
    dml2_output_rate_hdmi_rate_6x4 = 9,
    dml2_output_rate_hdmi_rate_8x4 = 10,
    dml2_output_rate_hdmi_rate_10x4 = 11,
    dml2_output_rate_hdmi_rate_12x4 = 12,
    dml2_output_rate_hdmi_rate_16x4 = 13,
    dml2_output_rate_hdmi_rate_20x4 = 14
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pmo_options {
    pub disable_vblank: bool,
    pub disable_svp: bool,
    pub disable_drr_var: bool,
    pub disable_drr_clamped: bool,
    pub disable_drr_var_when_var_active: bool,
    pub disable_drr_clamped_when_var_active: bool,
    pub disable_fams2: bool,
    pub /: *mut *mut bool disable_vactive_det_fill_bw_pad; / dml2_project_dcn4x_stage2_auto_drr_svp and above only,
    pub disable_dyn_odm: bool,
    pub disable_dyn_odm_for_multi_stream: bool,
    pub disable_dyn_odm_for_stream_with_svp: bool,
    pub force_mandatory_uclk_pstate_support: bool,
    pub disable_alternate_memory_training: bool,
    pub force_optional_uclk_pstate_support: bool,
    pub force_optional_mcache_support: bool,
    pub force_optional_ppt_temp_read_admissibility: bool,
    pub override_strategy_lists: [*mut dml2_pmo_pstate_strategy; DML2_MAX_PLANES],
    pub num_override_strategies_per_list: [c_uint; DML2_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_options {
    pub project_id: dml2_project_id,
    pub pmo_options: dml2_pmo_options,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_initialize_instance_in_out {
    pub dml2_instance: *mut dml2_instance,
    pub options: dml2_options,
    pub soc_bb: dml2_soc_bb,
    pub ip_caps: dml2_ip_capabilities,
    pub explicit_ip_bb: *mut c_void,
    pub explicit_ip_bb_size: c_uint,
    pub explicit_qos_model: *const utm_qos_model,
    pub overrides: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_reset_instance_in_out {
    pub dml2_instance: *mut dml2_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_check_mode_supported_in_out {
//
// Inputs
//
    pub dml2_instance: *mut dml2_instance,
    pub display_config: *const dml2_display_cfg,
//
// Outputs
//
    pub is_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mcache_surface_allocation {
    pub valid: bool,
//
// For iMALL, dedicated mall mcaches are required (sharing of last
// slice possible), for legacy phantom or phantom without return
// the only mall mcaches need to be valid.
//
    pub requires_dedicated_mall_mcache: bool,
    pub num_mcaches_plane0: c_uint,
    pub num_mcaches_plane1: c_uint,
//
// A plane is divided into vertical slices of mcaches,
// which wrap on the surface width.
//
// For example, if the surface width is 7680, and split into
// three slices of equal width, the boundary array would contain
// [2560, 5120, 7680]
//
// The assignments are
// 0 = [0 .. 2559]
// 1 = [2560 .. 5119]
// 2 = [5120 .. 7679]
// 0 = [7680 .. INF]
// The final element implicitly is the same as the first, and
// at first seems invalid since it is never referenced (since)
// it is outside the surface. However, its useful when shifting
// (see below).
//
// For any given valid mcache assignment, a shifted version, wrapped
// on the surface width boundary is also assumed to be valid.
//
// For example, shifting [2560, 5120, 7680] by -50 results in
// [2510, 5170, 7630].
//
// The assignments are now:
// 0 = [0 .. 2509]
// 1 = [2510 .. 5169]
// 2 = [5170 .. 7629]
// 0 = [7630 .. INF]
//
    pub 1]: int mcache_x_offsets_plane0[DML2_MAX_MCACHES +,
    pub 1]: int mcache_x_offsets_plane1[DML2_MAX_MCACHES +,
//
// Shift grainularity is not necessarily 1
//
    pub p0: c_int,
    pub p1: c_int,
    pub shift_granularity: },
//
// MCacheIDs have global scope in the SoC, and they are stored here.
// These IDs are generally not valid until all planes in a display
// configuration have had their mcache requirements calculated.
//
    pub 1]: int global_mcache_ids_plane0[DML2_MAX_MCACHES +,
    pub 1]: int global_mcache_ids_plane1[DML2_MAX_MCACHES +,
    pub 1]: int global_mcache_ids_mall_plane0[DML2_MAX_MCACHES +,
    pub 1]: int global_mcache_ids_mall_plane1[DML2_MAX_MCACHES +,
//
// Generally, plane0/1 slices must use a disjoint set of caches
// but in some cases the final segement of the two planes can
// use the same cache. If plane0_plane1 is set, then this is
// allowed.
//
// Similarly, the caches allocated to MALL prefetcher are generally
// disjoint, but if mall_prefetch is set, then the final segment
// between the main and the mall pixel requestor can use the same
// cache.
//
// Note that both bits may be set at the same time.
//
    pub mall_comb_mcache_p0: bool,
    pub mall_comb_mcache_p1: bool,
    pub plane0_plane1: bool,
    pub last_slice_sharing: },
    pub meta_row_bytes_plane0: c_int,
    pub meta_row_bytes_plane1: c_int,
    pub informative: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_pstate_method {
    dml2_pstate_method_na = 0,
// hw exclusive modes
    dml2_pstate_method_vactive = 1,
    dml2_pstate_method_vblank = 2,
    dml2_pstate_method_reserved_hw = 5,
// fw assisted exclusive modes
    dml2_pstate_method_fw_svp = 6,
    dml2_pstate_method_reserved_fw = 10,
// fw assisted modes requiring drr modulation
    dml2_pstate_method_fw_vactive_drr = 11,
    dml2_pstate_method_fw_vblank_drr = 12,
    dml2_pstate_method_fw_svp_drr = 13,
    dml2_pstate_method_reserved_fw_drr_clamped = 20,
    dml2_pstate_method_fw_drr = 21,
    dml2_pstate_method_reserved_fw_drr_var = 22,
    dml2_pstate_method_alternate,
    dml2_pstate_method_count
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_per_plane_programming {
    pub plane_descriptor: *const dml2_plane_parameters,
    pub dppclk_khz: c_ulong,
    pub dcn4x: },
    pub min_clocks: },
    pub mcache_allocation: dml2_mcache_surface_allocation,
// If a stream is using automatic or forced odm combine
// and the stream for this plane has num_odms_required > 1
// num_dpps_required is always equal to num_odms_required for
// ALL planes of the stream
// If a stream is using odm split, then this value is always 1
    pub num_dpps_required: c_uint,
    pub uclk_pstate_support_method: dml2_pstate_method,
// MALL size requirements for MALL SS and SubVP
    pub surface_size_mall_bytes: c_uint,
    pub svp_size_mall_bytes: c_uint,
    pub pipe_regs: [*mut dml2_dchub_per_pipe_register_set; DML2_MAX_PLANES],
    pub valid: bool,
    pub descriptor: dml2_plane_parameters,
    pub mcache_allocation: dml2_mcache_surface_allocation,
    pub pipe_regs: [*mut dml2_dchub_per_pipe_register_set; DML2_MAX_PLANES],
    pub phantom_plane: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dml2_global_sync_programming {
    pub vstartup_lines: c_uint,
    pub vupdate_offset_pixels: c_uint,
    pub vupdate_vupdate_width_pixels: c_uint,
    pub vready_offset_pixels: c_uint,
    pub pstate_keepout_start_lines: c_uint,
    pub dcn4x: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_per_stream_programming {
    pub stream_descriptor: *const dml2_stream_parameters,
    pub dscclk_khz: c_ulong,
    pub dtbclk_khz: c_ulong,
    pub phyclk_khz: c_ulong,
    pub dcn4x: },
    pub min_clocks: },
    pub global_sync: dml2_global_sync_programming,
    pub num_odms_required: c_uint,
    pub uclk_pstate_method: dml2_pstate_method,
    pub mcif_regs: [*mut dml2_mcif_per_pipe_register_set; DML2_MAX_WRITEBACK],
    pub enabled: bool,
    pub descriptor: dml2_stream_parameters,
    pub global_sync: dml2_global_sync_programming,
    pub phantom_stream: },
    pub fams2_base_params: dmub_cmd_fams2_config,
    pub fams2_sub_params: dmub_cmd_fams2_config,
    pub fams2_sub_params_v2: dmub_fams2_stream_static_sub_state_v2,
}

// -----------------
// Mode Support Information
// -----------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_mode_support_info {
    pub setting: bool ModeIsSupported; //<brief Is the mode support any voltage and combine,
    pub programming: bool ImmediateFlipSupport; //<brief Means mode support immediate flip at the max combine setting; determine in mode support and used in mode,
// Mode Support Reason
    pub WritebackLatencySupport: bool,
    pub ScaleRatioAndTapsSupport: bool,
    pub SourceFormatPixelAndScanSupport: bool,
    pub P2IWith420: bool,
    pub DSCOnlyIfNecessaryWithBPP: bool,
    pub DSC422NativeNotSupported: bool,
    pub LinkRateDoesNotMatchDPVersion: bool,
    pub LinkRateForMultistreamNotIndicated: bool,
    pub BPPForMultistreamNotIndicated: bool,
    pub MultistreamWithHDMIOreDP: bool,
    pub MSOOrODMSplitWithNonDPLink: bool,
    pub NotEnoughLanesForMSO: bool,
    pub NumberOfOTGSupport: bool,
    pub NumberOfHDMIFRLSupport: bool,
    pub NumberOfDP2p0Support: bool,
    pub NumberOfTDLUT33cubeSupport: bool,
    pub WritebackScaleRatioAndTapsSupport: bool,
    pub CursorSupport: bool,
    pub PitchSupport: bool,
    pub ViewportExceedsSurface: bool,
    pub ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified: bool,
    pub ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe: bool,
    pub InvalidCombinationOfMALLUseForPStateAndStaticScreen: bool,
    pub InvalidCombinationOfMALLUseForPState: bool,
    pub ExceededMALLSize: bool,
    pub EnoughWritebackUnits: bool,
    pub ExceededMultistreamSlots: bool,
    pub NotEnoughDSCUnits: bool,
    pub NotEnoughDSCSlices: bool,
    pub PixelsPerLinePerDSCUnitSupport: bool,
    pub DSCCLKRequiredMoreThanSupported: bool,
    pub DTBCLKRequiredMoreThanSupported: bool,
    pub LinkCapacitySupport: bool,
    pub ROBSupport: bool,
    pub OutstandingRequestsSupport: bool,
    pub OutstandingRequestsUrgencyAvoidance: bool,
    pub PTEBufferSizeNotExceeded: bool,
    pub DCCMetaBufferSizeNotExceeded: bool,
    pub TotalVerticalActiveBandwidthSupport: bool,
    pub VActiveBandwidthSupport: bool,
    pub FCLKChangeSupport: [dml2_pstate_change_support; DML2_MAX_PLANES],
    pub USRRetrainingSupport: bool,
    pub PrefetchSupported: bool,
    pub DynamicMetadataSupported: bool,
    pub VRatioInPrefetchSupported: bool,
    pub DISPCLK_DPPCLK_Support: bool,
    pub TotalAvailablePipesSupport: bool,
    pub ViewportSizeSupport: bool,
    pub ImmediateFlipSupportedForState: bool,
    pub MaxTotalVerticalActiveAvailableBandwidth: double,
    pub setting: bool MPCCombineEnable[DML2_MAX_PLANES]; /// <brief Indicate if the MPC Combine enable in the given state and optimize mpc combine,
    pub stage: dml2_odm_mode ODMMode[DML2_MAX_PLANES]; /// <brief ODM mode that is chosen in the mode check stage and will be used in mode programming,
    pub 4.: unsigned int DPPPerSurface[DML2_MAX_PLANES]; /// <brief How many DPPs are needed drive the surface to output. If MPCC or ODMC could be 2 or,
    pub mode_programming: bool DSCEnabled[DML2_MAX_PLANES]; /// <brief Indicate if the DSC is actually required; used in,
    pub required: bool FECEnabled[DML2_MAX_PLANES]; /// <brief Indicate if the FEC is actually,
    pub mode: unsigned int NumberOfDSCSlices[DML2_MAX_PLANES]; /// <brief Indicate how many slices needed to support the given,
    pub OutputBpp: [double; DML2_MAX_PLANES],
    pub OutputType: [dml2_output_type_and_rate__type; DML2_MAX_PLANES],
    pub OutputRate: [dml2_output_type_and_rate__rate; DML2_MAX_PLANES],
    pub AlignedYPitch: [c_uint; DML2_MAX_PLANES],
    pub AlignedCPitch: [c_uint; DML2_MAX_PLANES],
    pub g6_temp_read_support: bool,
    pub temp_read_or_ppt_support: bool,
    pub qos_bandwidth_support: bool,
    pub dcfclk_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_memory_path_latency {
    pub urgent_ramp: double,
    pub t_trip: double,
    pub meta_trip_to_mem: double,
    pub max_req_latency_urg: double,
    pub avg_req_latency_urg: double,
    pub max_req_latency_non_urg: double,
    pub avg_req_latency_non_urg: double,
    pub df_response_time_us: double,
    pub dcn5: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_memory_path_bandwidth {
    pub sec: double urgent_bandwidth_kbps; // kbytes per,
    pub sec: double non_urgent_bandwidth_kbps; // kbytes per,
    pub dcn5: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_qos_bound {
    pub latency_ub: dml2_memory_path_latency,
    pub bandwidth_lb: dml2_memory_path_bandwidth,
    pub lsdma_bandwidth_lb_kbps: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_cfg_programming {
    pub display_config: dml2_display_cfg,
    pub dcfclk_khz: c_ulong,
    pub fclk_khz: c_ulong,
    pub uclk_khz: c_ulong,
    pub socclk_khz: c_ulong,
    pub dispclk_khz: c_ulong,
    pub dcfclk_deepsleep_khz: c_ulong,
    pub dpp_ref_khz: c_ulong,
    pub dcn32x: },
    pub uclk_khz: c_ulong,
    pub fclk_khz: c_ulong,
    pub dcfclk_khz: c_ulong,
    pub active: },
    pub uclk_khz: c_ulong,
    pub fclk_khz: c_ulong,
    pub dcfclk_khz: c_ulong,
    pub idle: },
    pub uclk_khz: c_ulong,
    pub fclk_khz: c_ulong,
    pub dcfclk_khz: c_ulong,
    pub svp_prefetch: },
    pub uclk_khz: c_ulong,
    pub fclk_khz: c_ulong,
    pub dcfclk_khz: c_ulong,
    pub svp_prefetch_no_throttle: },
    pub deepsleep_dcfclk_khz: c_ulong,
    pub dispclk_khz: c_ulong,
    pub dpprefclk_khz: c_ulong,
    pub dtbrefclk_khz: c_ulong,
    pub socclk_khz: c_ulong,
    pub dispclk_did: u32,
    pub dpprefclk_did: u32,
    pub dtbrefclk_did: u32,
    pub divider_ids: },
    pub dcn4x: },
    pub min_clocks: },
    pub qos_bound: dml2_qos_bound,
    pub min_sop_index: c_uint,
    pub uclk_pstate_supported: bool,
    pub fclk_pstate_supported: bool,
// indicates this configuration requires FW to support
    pub fams2_required: bool,
    pub legacy_pstate_info_for_dmu: bool,
    pub fams2_global_config: dmub_cmd_fams2_global_config,
    pub transition: bool supported_in_blank; // Changing to configurations where this is false requires stutter to be disabled during the,
    pub //LP1: uint8_t base_percent_efficiency;,
    pub //LP2: uint8_t low_power_percent_efficiency;,
    pub stutter: },
    pub criteria: bool meets_eco; // Stutter cycles will meet Z8 ECO,
    pub transition: bool supported_in_blank; // Changing to configurations where this is false requires Z8 to be disabled during the,
    pub z8_stutter: },
    pub global_regs: dml2_dchub_global_register_set,
    pub mcif_global_regs: dml2_mcif_global_register_set,
    pub plane_programming: [dml2_per_plane_programming; DML2_MAX_PLANES],
    pub stream_programming: [dml2_per_stream_programming; DML2_MAX_PLANES],
// Don't access this structure directly, access it through plane_programming.pipe_regs
    pub pipe_regs: [dml2_dchub_per_pipe_register_set; DML2_MAX_PLANES],
    pub mcif_regs: [dml2_mcif_per_pipe_register_set; DML2_MAX_WRITEBACK],
    pub urgent_us: double,
    pub writeback_urgent_us: double,
    pub writeback_pstate_us: double,
    pub writeback_fclk_pstate_us: double,
    pub cstate_exit_us: double,
    pub cstate_enter_plus_exit_us: double,
    pub z8_cstate_exit_us: double,
    pub z8_cstate_enter_plus_exit_us: double,
    pub pstate_change_us: double,
    pub fclk_pstate_change_us: double,
    pub usr_retraining_us: double,
    pub temp_read_or_ppt_watermark_us: double,
    pub writeback_temp_read_or_ppt_watermark_us: double,
    pub watermarks: },
    pub swath_width_plane0: c_uint,
    pub swath_height_plane0: c_uint,
    pub swath_height_plane1: c_uint,
    pub dpte_row_height_plane0: c_uint,
    pub dpte_row_height_plane1: c_uint,
    pub meta_row_height_plane0: c_uint,
    pub meta_row_height_plane1: c_uint,
    pub plane_info: [}; DML2_MAX_PLANES],
    pub total_num_dpps_required: c_uint,
    pub dpp: },
    pub total_surface_size_in_mall_bytes: c_ulonglong,
    pub subviewport_lines_needed_in_mall: [c_uint; DML2_MAX_PLANES],
    pub mall: },
    pub latency: double urgent_latency_us; // urgent ramp,
    pub max_non_urgent_latency_us: double,
    pub max_urgent_latency_us: double,
    pub avg_non_urgent_latency_us: double,
    pub avg_urgent_latency_us: double,
    pub wm_memory_trip_us: double,
    pub meta_trip_memory_us: double,
    pub nom: double fraction_of_urgent_bandwidth; //,
    pub fraction_of_urgent_bandwidth_immediate_flip: double,
    pub fraction_of_urgent_bandwidth_mall: double,
    pub max_active_fclk_change_latency_supported: double,
    pub min_return_latency_in_dcfclk: c_uint,
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub dram_vm_only_bw_mbps: double,
    pub svp_prefetch: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub dram_vm_only_bw_mbps: double,
    pub sys_active: },
    pub urg_bw_available: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub svp_prefetch: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub sys_active: },
    pub avg_bw_available: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub svp_prefetch: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub sys_active: },
    pub non_urg_bw_required: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub svp_prefetch: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub sys_active: },
    pub non_urg_bw_required_with_flip: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub svp_prefetch: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub sys_active: },
    pub urg_bw_required: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub svp_prefetch: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub sys_active: },
    pub urg_bw_required_with_flip: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub svp_prefetch: },
    pub sdp_bw_mbps: double,
    pub dram_bw_mbps: double,
    pub sys_active: },
    pub avg_bw_required: },
    pub qos: },
    pub det_size_in_kbytes: [c_ulonglong; DML2_MAX_PLANES],
    pub DETBufferSizeY: [c_ulonglong; DML2_MAX_PLANES],
    pub comp_buffer_size_kbytes: c_ulonglong,
    pub UnboundedRequestEnabled: bool,
    pub compbuf_reserved_space_64b: c_uint,
    pub crb: },
    pub max_uncompressed_block_plane0: c_uint,
    pub max_compressed_block_plane0: c_uint,
    pub independent_block_plane0: c_uint,
    pub max_uncompressed_block_plane1: c_uint,
    pub max_compressed_block_plane1: c_uint,
    pub independent_block_plane1: c_uint,
    pub dcc_control: [}; DML2_MAX_PLANES],
    pub stutter_efficiency: double,
    pub stutter_efficiency_with_vblank: double,
    pub stutter_num_bursts: double,
    pub stutter_efficiency: double,
    pub stutter_efficiency_with_vblank: double,
    pub stutter_num_bursts: double,
    pub stutter_period: double,
    pub stutter_efficiency: double,
    pub stutter_num_bursts: double,
    pub stutter_period: double,
    pub bestcase: },
    pub z8: },
    pub power_management: },
    pub min_ttu_vblank_us: [double; DML2_MAX_PLANES],
    pub vready_at_or_after_vsync: [bool; DML2_MAX_PLANES],
    pub min_dst_y_next_start: [double; DML2_MAX_PLANES],
    pub cstate_max_cap_mode: bool,
    pub hw_debug5: bool,
    pub dcfclk_deep_sleep_hysteresis: c_uint,
    pub dst_x_after_scaler: [c_uint; DML2_MAX_PLANES],
    pub dst_y_after_scaler: [c_uint; DML2_MAX_PLANES],
    pub prefetch_source_lines_plane0: [c_uint; DML2_MAX_PLANES],
    pub prefetch_source_lines_plane1: [c_uint; DML2_MAX_PLANES],
    pub ImmediateFlipSupportedForPipe: [bool; DML2_MAX_PLANES],
    pub UsesMALLForStaticScreen: [bool; DML2_MAX_PLANES],
    pub CursorDstXOffset: [c_uint; DML2_MAX_PLANES],
    pub CursorDstYOffset: [c_uint; DML2_MAX_PLANES],
    pub CursorChunkHDLAdjust: [c_uint; DML2_MAX_PLANES],
    pub dpte_group_bytes: [c_uint; DML2_MAX_PLANES],
    pub vm_group_bytes: [c_uint; DML2_MAX_PLANES],
    pub DisplayPipeRequestDeliveryTimeLuma: [double; DML2_MAX_PLANES],
    pub DisplayPipeRequestDeliveryTimeChroma: [double; DML2_MAX_PLANES],
    pub DisplayPipeRequestDeliveryTimeLumaPrefetch: [double; DML2_MAX_PLANES],
    pub DisplayPipeRequestDeliveryTimeChromaPrefetch: [double; DML2_MAX_PLANES],
    pub TimePerVMGroupVBlank: [double; DML2_MAX_PLANES],
    pub TimePerVMGroupFlip: [double; DML2_MAX_PLANES],
    pub TimePerVMRequestVBlank: [double; DML2_MAX_PLANES],
    pub TimePerVMRequestFlip: [double; DML2_MAX_PLANES],
    pub Tdmdl_vm: [double; DML2_MAX_PLANES],
    pub Tdmdl: [double; DML2_MAX_PLANES],
    pub VStartup: [c_uint; DML2_MAX_PLANES],
    pub VUpdateOffsetPix: [c_uint; DML2_MAX_PLANES],
    pub VUpdateWidthPix: [c_uint; DML2_MAX_PLANES],
    pub VReadyOffsetPix: [c_uint; DML2_MAX_PLANES],
    pub DST_Y_PER_PTE_ROW_NOM_L: [double; DML2_MAX_PLANES],
    pub DST_Y_PER_PTE_ROW_NOM_C: [double; DML2_MAX_PLANES],
    pub time_per_pte_group_nom_luma: [double; DML2_MAX_PLANES],
    pub time_per_pte_group_nom_chroma: [double; DML2_MAX_PLANES],
    pub time_per_pte_group_vblank_luma: [double; DML2_MAX_PLANES],
    pub time_per_pte_group_vblank_chroma: [double; DML2_MAX_PLANES],
    pub time_per_pte_group_flip_luma: [double; DML2_MAX_PLANES],
    pub time_per_pte_group_flip_chroma: [double; DML2_MAX_PLANES],
    pub VRatioPrefetchY: [double; DML2_MAX_PLANES],
    pub VRatioPrefetchC: [double; DML2_MAX_PLANES],
    pub DestinationLinesForPrefetch: [double; DML2_MAX_PLANES],
    pub DestinationLinesToRequestVMInVBlank: [double; DML2_MAX_PLANES],
    pub DestinationLinesToRequestRowInVBlank: [double; DML2_MAX_PLANES],
    pub DestinationLinesToRequestVMInImmediateFlip: [double; DML2_MAX_PLANES],
    pub DestinationLinesToRequestRowInImmediateFlip: [double; DML2_MAX_PLANES],
    pub DisplayPipeLineDeliveryTimeLuma: [double; DML2_MAX_PLANES],
    pub DisplayPipeLineDeliveryTimeChroma: [double; DML2_MAX_PLANES],
    pub DisplayPipeLineDeliveryTimeLumaPrefetch: [double; DML2_MAX_PLANES],
    pub DisplayPipeLineDeliveryTimeChromaPrefetch: [double; DML2_MAX_PLANES],
    pub WritebackRequiredBandwidth: double,
    pub WritebackAllowDRAMClockChangeEndPosition: [double; DML2_MAX_PLANES],
    pub WritebackAllowFCLKChangeEndPosition: [double; DML2_MAX_PLANES],
    pub DSCCLK_calculated: [double; DML2_MAX_PLANES],
    pub BIGK_FRAGMENT_SIZE: [c_uint; DML2_MAX_PLANES],
    pub PTE_BUFFER_MODE: [bool; DML2_MAX_PLANES],
    pub DSCDelay: [double; DML2_MAX_PLANES],
    pub MaxActiveDRAMClockChangeLatencySupported: [double; DML2_MAX_PLANES],
    pub LEGACY_ONLY: unsigned int PrefetchMode[DML2_MAX_PLANES]; //,
    pub ROBUrgencyAvoidance: bool,
    pub LowestPrefetchMargin: double,
    pub pstate_recout_reduction_lines: [c_uint; DML2_MAX_PLANES],
    pub misc: },
    pub mode_support_info: dml2_mode_support_info,
    pub LEGACY_ONLY: unsigned int voltage_level; //,
// For DV only
// This is what dml core calculated, only on the full_vp width and assume we have
// unlimited # of mcache
    pub non_optimized_mcache_allocation: [dml2_mcache_surface_allocation; DML2_MAX_PLANES],
    pub failed_prefetch: bool,
    pub failed_uclk_pstate: bool,
    pub failed_mcache_validation: bool,
    pub failed_dpmm: bool,
    pub failed_mode_programming: bool,
    pub failed_mode_programming_dcfclk: bool,
    pub failed_mode_programming_prefetch: bool,
    pub failed_mode_programming_flip: bool,
    pub failed_map_watermarks: bool,
    pub informative: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_build_mode_programming_in_out {
//
// Inputs
//
    pub dml2_instance: *mut dml2_instance,
    pub display_config: *const dml2_display_cfg,
//
// Outputs
//
    pub programming: *mut dml2_display_cfg_programming,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_build_mcache_programming_in_out {
//
// Inputs
//
    pub dml2_instance: *mut dml2_instance,
    pub mcache_configurations: [dml2_plane_mcache_configuration_descriptor; DML2_MAX_PLANES],
    pub num_configurations: c_char,
//
// Outputs
//
// per_plane_pipe_mcache_regs[i][j] refers to the proper programming for the j-th pipe of the
// i-th plane (from mcache_configurations)
    pub per_plane_pipe_mcache_regs: [*mut dml2_hubp_pipe_mcache_regs; DML2_MAX_PLANES][DML2_MAX_DCN_PIPES],
// It's not a good idea to reference this directly, better to use the pointer structure above instead
    pub mcache_regs_set: [dml2_hubp_pipe_mcache_regs; DML2_MAX_DCN_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_unit_test_in_out {
//
// Inputs
//
    pub dml2_instance: *mut dml2_instance,
}
