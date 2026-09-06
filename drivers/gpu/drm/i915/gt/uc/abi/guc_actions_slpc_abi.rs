//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/abi/guc_actions_slpc_abi.h
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
// Copyright © 2021 Intel Corporation
//

//
// DOC: SLPC SHARED DATA STRUCTURE
//
// +----+------+--------------------------------------------------------------+
// | CL | Bytes| Description                                                  |
// +====+======+==============================================================+
// | 1  | 0-3  | SHARED DATA SIZE                                             |
// |    +------+--------------------------------------------------------------+
// |    | 4-7  | GLOBAL STATE                                                 |
// |    +------+--------------------------------------------------------------+
// |    | 8-11 | DISPLAY DATA ADDRESS                                         |
// |    +------+--------------------------------------------------------------+
// |    | 12:63| PADDING                                                      |
// +----+------+--------------------------------------------------------------+
// |    | 0:63 | PADDING(PLATFORM INFO)                                       |
// +----+------+--------------------------------------------------------------+
// | 3  | 0-3  | TASK STATE DATA                                              |
// +    +------+--------------------------------------------------------------+
// |    | 4:63 | PADDING                                                      |
// +----+------+--------------------------------------------------------------+
// |4-21|0:1087| OVERRIDE PARAMS AND BIT FIELDS                               |
// +----+------+--------------------------------------------------------------+
// |    |      | PADDING + EXTRA RESERVED PAGE                                |
// +----+------+--------------------------------------------------------------+
//
// SLPC exposes certain parameters for global configuration by the host.
// These are referred to as override parameters, because in most cases
// the host will not need to modify the default values used by SLPC.
// SLPC remembers the default values which allows the host to easily restore
// them by simply unsetting the override. The host can set or unset override
// parameters during SLPC (re-)initialization using the SLPC Reset event.
// The host can also set or unset override parameters on the fly using the
// Parameter Set and Parameter Unset events
//
pub const SLPC_MAX_OVERRIDE_PARAMETERS: c_int = 256;

pub const SLPC_PAGE_SIZE_BYTES: c_int = 4096;
pub const SLPC_CACHELINE_SIZE_BYTES: c_int = 64;

//
// Cacheline size aligned (Total size needed for
// SLPM_KMD_MAX_OVERRIDE_PARAMETERS=256 is 1088 bytes)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slpc_task_enable {
    SLPC_PARAM_TASK_DEFAULT = 0,
    SLPC_PARAM_TASK_ENABLED,
    SLPC_PARAM_TASK_DISABLED,
    SLPC_PARAM_TASK_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slpc_global_state {
    SLPC_GLOBAL_STATE_NOT_RUNNING = 0,
    SLPC_GLOBAL_STATE_INITIALIZING = 1,
    SLPC_GLOBAL_STATE_RESETTING = 2,
    SLPC_GLOBAL_STATE_RUNNING = 3,
    SLPC_GLOBAL_STATE_SHUTTING_DOWN = 4,
    SLPC_GLOBAL_STATE_ERROR = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slpc_param_id {
    SLPC_PARAM_TASK_ENABLE_GTPERF = 0,
    SLPC_PARAM_TASK_DISABLE_GTPERF = 1,
    SLPC_PARAM_TASK_ENABLE_BALANCER = 2,
    SLPC_PARAM_TASK_DISABLE_BALANCER = 3,
    SLPC_PARAM_TASK_ENABLE_DCC = 4,
    SLPC_PARAM_TASK_DISABLE_DCC = 5,
    SLPC_PARAM_GLOBAL_MIN_GT_UNSLICE_FREQ_MHZ = 6,
    SLPC_PARAM_GLOBAL_MAX_GT_UNSLICE_FREQ_MHZ = 7,
    SLPC_PARAM_GLOBAL_MIN_GT_SLICE_FREQ_MHZ = 8,
    SLPC_PARAM_GLOBAL_MAX_GT_SLICE_FREQ_MHZ = 9,
    SLPC_PARAM_GTPERF_THRESHOLD_MAX_FPS = 10,
    SLPC_PARAM_GLOBAL_DISABLE_GT_FREQ_MANAGEMENT = 11,
    SLPC_PARAM_GTPERF_ENABLE_FRAMERATE_STALLING = 12,
    SLPC_PARAM_GLOBAL_DISABLE_RC6_MODE_CHANGE = 13,
    SLPC_PARAM_GLOBAL_OC_UNSLICE_FREQ_MHZ = 14,
    SLPC_PARAM_GLOBAL_OC_SLICE_FREQ_MHZ = 15,
    SLPC_PARAM_GLOBAL_ENABLE_IA_GT_BALANCING = 16,
    SLPC_PARAM_GLOBAL_ENABLE_ADAPTIVE_BURST_TURBO = 17,
    SLPC_PARAM_GLOBAL_ENABLE_EVAL_MODE = 18,
    SLPC_PARAM_GLOBAL_ENABLE_BALANCER_IN_NON_GAMING_MODE = 19,
    SLPC_PARAM_GLOBAL_RT_MODE_TURBO_FREQ_DELTA_MHZ = 20,
    SLPC_PARAM_PWRGATE_RC_MODE = 21,
    SLPC_PARAM_EDR_MODE_COMPUTE_TIMEOUT_MS = 22,
    SLPC_PARAM_EDR_QOS_FREQ_MHZ = 23,
    SLPC_PARAM_MEDIA_FF_RATIO_MODE = 24,
    SLPC_PARAM_ENABLE_IA_FREQ_LIMITING = 25,
    SLPC_PARAM_STRATEGIES = 26,
    SLPC_PARAM_POWER_PROFILE = 27,
    SLPC_PARAM_IGNORE_EFFICIENT_FREQUENCY = 28,
    SLPC_MAX_PARAM = 32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slpc_media_ratio_mode {
    SLPC_MEDIA_RATIO_MODE_DYNAMIC_CONTROL = 0,
    SLPC_MEDIA_RATIO_MODE_FIXED_ONE_TO_ONE = 1,
    SLPC_MEDIA_RATIO_MODE_FIXED_ONE_TO_TWO = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slpc_gucrc_mode {
    SLPC_GUCRC_MODE_HW = 0,
    SLPC_GUCRC_MODE_GUCRC_NO_RC6 = 1,
    SLPC_GUCRC_MODE_GUCRC_STATIC_TIMEOUT = 2,
    SLPC_GUCRC_MODE_GUCRC_DYNAMIC_HYSTERESIS = 3,

    SLPC_GUCRC_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slpc_event_id {
    SLPC_EVENT_RESET = 0,
    SLPC_EVENT_SHUTDOWN = 1,
    SLPC_EVENT_PLATFORM_INFO_CHANGE = 2,
    SLPC_EVENT_DISPLAY_MODE_CHANGE = 3,
    SLPC_EVENT_FLIP_COMPLETE = 4,
    SLPC_EVENT_QUERY_TASK_STATE = 5,
    SLPC_EVENT_PARAMETER_SET = 6,
    SLPC_EVENT_PARAMETER_UNSET = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slpc_task_state_data {
    pub task_status_padding: u32,
    pub status: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slpc_shared_data_header {
// Total size in bytes of this shared buffer.
    pub size: u32,
    pub global_state: u32,
    pub display_data_addr: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slpc_override_params {
    pub bits: [u32; SLPC_OVERRIDE_BITFIELD_SIZE],
    pub values: [u32; SLPC_MAX_OVERRIDE_PARAMETERS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slpc_shared_data {
    pub header: slpc_shared_data_header,
    pub slpc_shared_data_header)]: sizeof(struct,
    pub platform_info_pad: [u8; SLPC_SHARED_DATA_SIZE_BYTE_PLATFORM_INFO],
    pub task_state_data: slpc_task_state_data,
    pub slpc_task_state_data)]: sizeof(struct,
    pub override_params: slpc_override_params,
    pub slpc_override_params)]: sizeof(struct,
    pub shared_data_pad: [u8; SLPC_SHARED_DATA_SIZE_BYTE_OTHER],
// PAGE 2 (4096 bytes), mode based parameter will be removed soon
    pub reserved_mode_definition: [u8; 4096],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slpc_context_frequency_request {
    pub frequency_request:16: u32,
    pub reserved:12: u32,
    pub is_compute:1: u32,
    pub ignore_busyness:1: u32,
    pub is_minimum:1: u32,
    pub is_predefined:1: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slpc_optimized_strategies {
    pub compute:1: u32,
    pub async_flip:1: u32,
    pub media:1: u32,
    pub vsync_flip:1: u32,
    pub reserved:28: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slpc_power_profiles {
    SLPC_POWER_PROFILES_BASE = 0x0,
    SLPC_POWER_PROFILES_POWER_SAVING = 0x1
}

//
// DOC: SLPC H2G MESSAGE FORMAT
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MBZ                                                  |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_HOST2GUC_PC_SLPM_REQUEST` = 0x3003     |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:8 | **EVENT_ID**                                                 |
// +   +-------+--------------------------------------------------------------+
// |   |   7:0 | **EVENT_ARGC** - number of data arguments                    |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **EVENT_DATA1**                                              |
// +---+-------+--------------------------------------------------------------+
// |...|  31:0 | ...                                                          |
// +---+-------+--------------------------------------------------------------+
// |2+n|  31:0 | **EVENT_DATAn**                                              |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_HOST2GUC_PC_SLPC_REQUEST: c_uint = 0x3003;

pub const HOST2GUC_PC_SLPC_EVENT_MAX_INPUT_ARGS: c_int = 9;

