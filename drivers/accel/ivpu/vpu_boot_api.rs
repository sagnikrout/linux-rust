//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/vpu_boot_api.h
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
// Copyright (c) 2020-2025, Intel Corporation.
//
// @addtogroup Boot
// @{
//
// @file
// @brief Boot API public header file.
//
// The below values will be used to construct the version info this way:
// fw_bin_header->api_version[VPU_BOOT_API_VER_ID] = (VPU_BOOT_API_VER_MAJOR << 16) |
// VPU_BOOT_API_VER_MINOR;
// VPU_BOOT_API_VER_PATCH will be ignored. KMD and compatibility is not affected if this changes
// This information is collected by using vpuip_2/application/vpuFirmware/make_std_fw_image.py
// If a header is missing this info we ignore the header, if a header is missing or contains
// partial info a build error will be generated.
//
// Major version changes that break backward compatibility.
// Major version must start from 1 and can only be incremented.
//
pub const VPU_BOOT_API_VER_MAJOR: c_int = 3;
//
// Minor version changes when API backward compatibility is preserved.
// Resets to 0 if Major version is incremented.
//
pub const VPU_BOOT_API_VER_MINOR: c_int = 29;
//
// API header changed (field names, documentation, formatting) but API itself has not been changed
//
pub const VPU_BOOT_API_VER_PATCH: c_int = 5;
//
// Index in the API version table
// Must be unique for each API
//
pub const VPU_BOOT_API_VER_INDEX: c_int = 0;

//
// Firmware image header format
//
pub const VPU_FW_HEADER_SIZE: c_int = 4096;
pub const VPU_FW_HEADER_VERSION: c_uint = 0x1;
pub const VPU_FW_VERSION_SIZE: c_int = 32;
pub const VPU_FW_API_VER_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_firmware_header {
    pub header_version: u32,
    pub image_format: u32,
    pub image_load_address: u64,
    pub image_size: u32,
    pub entry_point: u64,
    pub vpu_version: [u8; VPU_FW_VERSION_SIZE],
    pub compression_type: u32,
    pub firmware_version_load_address: u64,
    pub firmware_version_size: u32,
    pub boot_params_load_address: u64,
    pub api_version: [u32; VPU_FW_API_VER_NUM],
// Size of memory require for firmware execution
    pub runtime_size: u32,
    pub shave_nn_fw_size: u32,
//
// Size of primary preemption buffer, assuming a 2-job submission queue.
// NOTE: host driver is expected to adapt size accordingly to actual
// submission queue size and device capabilities.
//
    pub preemption_buffer_1_size: u32,
//
// Size of secondary preemption buffer, assuming a 2-job submission queue.
// NOTE: host driver is expected to adapt size accordingly to actual
// submission queue size and device capabilities.
//
    pub preemption_buffer_2_size: u32,
//
// Maximum preemption buffer size that the FW can use: no need for the host
// driver to allocate more space than that specified by these fields.
// A value of 0 means no declared limit.
//
    pub preemption_buffer_1_max_size: u32,
    pub preemption_buffer_2_max_size: u32,
// Space reserved for future preemption-related fields.
    pub preemption_reserved: [u32; 4],
// FW image read only section start address, 4KB aligned
    pub ro_section_start_address: u64,
// FW image read only section size, 4KB aligned
    pub ro_section_size: u32,
    pub reserved: u32,
}

//
// Firmware boot parameters format
//
// Values for boot_type field
pub const VPU_BOOT_TYPE_COLDBOOT: c_int = 0;
pub const VPU_BOOT_TYPE_WARMBOOT: c_int = 1;
// Value for magic filed
pub const VPU_BOOT_PARAMS_MAGIC: c_uint = 0x10000;
// VPU scheduling mode. By default, OS scheduling is used.
pub const VPU_SCHEDULING_MODE_OS: c_int = 0;
pub const VPU_SCHEDULING_MODE_HW: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPU_BOOT_L2_CACHE_CFG_TYPE {
    VPU_BOOT_L2_CACHE_CFG_UPA = 0,
    VPU_BOOT_L2_CACHE_CFG_NN = 1,
    VPU_BOOT_L2_CACHE_CFG_NUM = 2
}

// VPU MCA ECC signalling mode. By default, no signalling is used
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPU_BOOT_MCA_ECC_SIGNAL_TYPE {
    VPU_BOOT_MCA_ECC_NONE = 0,
    VPU_BOOT_MCA_ECC_CORR = 1,
    VPU_BOOT_MCA_ECC_FATAL = 2,
    VPU_BOOT_MCA_ECC_BOTH = 3
}

//
// Logging destinations.
//
// Logging output can be directed to different logging destinations. This enum
// defines the list of logging destinations supported by the VPU firmware (NOTE:
// a specific VPU FW binary may support only a subset of such output
// destinations, depending on the target platform and compile options).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_trace_destination {
    VPU_TRACE_DESTINATION_PIPEPRINT = 0x1,
    VPU_TRACE_DESTINATION_VERBOSE_TRACING = 0x2,
    VPU_TRACE_DESTINATION_NORTH_PEAK = 0x4,
}

//
// Processor bit shifts (for loggable HW components).
//
pub const VPU_TRACE_PROC_BIT_RESERVED: c_int = 0;
pub const VPU_TRACE_PROC_BIT_LRT: c_int = 1;
pub const VPU_TRACE_PROC_BIT_LNN: c_int = 2;
pub const VPU_TRACE_PROC_BIT_SHV_0: c_int = 3;
pub const VPU_TRACE_PROC_BIT_SHV_1: c_int = 4;
pub const VPU_TRACE_PROC_BIT_SHV_2: c_int = 5;
pub const VPU_TRACE_PROC_BIT_SHV_3: c_int = 6;
pub const VPU_TRACE_PROC_BIT_SHV_4: c_int = 7;
pub const VPU_TRACE_PROC_BIT_SHV_5: c_int = 8;
pub const VPU_TRACE_PROC_BIT_SHV_6: c_int = 9;
pub const VPU_TRACE_PROC_BIT_SHV_7: c_int = 10;
pub const VPU_TRACE_PROC_BIT_SHV_8: c_int = 11;
pub const VPU_TRACE_PROC_BIT_SHV_9: c_int = 12;
pub const VPU_TRACE_PROC_BIT_SHV_10: c_int = 13;
pub const VPU_TRACE_PROC_BIT_SHV_11: c_int = 14;
pub const VPU_TRACE_PROC_BIT_SHV_12: c_int = 15;
pub const VPU_TRACE_PROC_BIT_SHV_13: c_int = 16;
pub const VPU_TRACE_PROC_BIT_SHV_14: c_int = 17;
pub const VPU_TRACE_PROC_BIT_SHV_15: c_int = 18;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_0: c_int = 19;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_1: c_int = 20;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_2: c_int = 21;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_3: c_int = 22;
pub const VPU_TRACE_PROC_NO_OF_HW_DEVS: c_int = 23;
// VPU 30xx HW component IDs are sequential, so define first and last IDs.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_boot_l2_cache_config {
    pub use: u8,
    pub cfg: u8,
}

//
// When HW scheduling mode is enabled, a present period is defined.
// It will be used by VPU to swap between normal and focus priorities
// to prevent starving of normal priority band (when implemented).
// Host must provide a valid value at boot time in
// `vpu_focus_present_timer_ms`. If the value provided by the host is not within the
// defined range a default value will be used. Here we define the min. and max.
// allowed values and the and default value of the present period. Units are milliseconds.
//
pub const VPU_PRESENT_CALL_PERIOD_MS_DEFAULT: c_int = 50;
pub const VPU_PRESENT_CALL_PERIOD_MS_MIN: c_int = 16;
pub const VPU_PRESENT_CALL_PERIOD_MS_MAX: c_int = 10000;
//
// Macros to enable various power profiles within the NPU.
// To be defined as part of 32 bit mask.
//
pub const POWER_PROFILE_SURVIVABILITY: c_uint = 0x1;
//
// Enum for dvfs_mode boot param.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_governor {
    VPU_GOV_DEFAULT = 0, /** Default Governor for the system */
    VPU_GOV_MAX_PERFORMANCE = 1, /** Maximum performance governor */
    VPU_GOV_ON_DEMAND = 2, /** On Demand frequency control governor */
    VPU_GOV_POWER_SAVE = 3, /** Power save governor */
    VPU_GOV_ON_DEMAND_PRIORITY_AWARE = 4 /** On Demand priority based governor */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_boot_params {
    pub magic: u32,
    pub vpu_id: u32,
    pub vpu_count: u32,
    pub reserved_0: [u32; 5],
// Clock frequencies: 0x20 - 0xFF
    pub frequency: u32,
    pub reserved_1: [u32; 12],
    pub perf_clk_frequency: u32,
    pub reserved_2: [u32; 42],
// Memory regions: 0x100 - 0x1FF
    pub ipc_header_area_start: u64,
    pub ipc_header_area_size: u32,
    pub shared_region_base: u64,
    pub shared_region_size: u32,
    pub ipc_payload_area_start: u64,
    pub ipc_payload_area_size: u32,
    pub global_aliased_pio_base: u64,
    pub global_aliased_pio_size: u32,
    pub autoconfig: u32,
    pub cache_defaults: [vpu_boot_l2_cache_config; VPU_BOOT_L2_CACHE_CFG_NUM],
    pub reserved_3: [u32; 3],
//
// ShaveNN FW section VPU base address
// On VPU2.7 HW this address must be within 2GB range starting from L2C_PAGE_TABLE base
//
    pub shave_nn_fw_base: u64,
    pub /: *mut *mut *mut u64 save_restore_ret_address; / stores the address of FW's restore entry point,
    pub reserved_4: [u32; 43],
// IRQ re-direct numbers: 0x200 - 0x2FF
    pub watchdog_irq_mss: i32,
    pub watchdog_irq_nce: i32,
// ARM -> VPU doorbell interrupt. ARM is notifying VPU of async command or compute job.
    pub host_to_vpu_irq: u32,
// VPU -> ARM job done interrupt. VPU is notifying ARM of compute job completion.
    pub job_done_irq: u32,
// Padding.
    pub reserved_5: [u32; 60],
// Silicon information: 0x300 - 0x3FF
    pub host_version_id: u32,
    pub si_stepping: u32,
    pub device_id: u64,
    pub feature_exclusion: u64,
    pub sku: u64,
// PLL ratio for minimum clock frequency
    pub min_freq_pll_ratio: u32,
// PLL ratio for maximum clock frequency
    pub max_freq_pll_ratio: u32,
//
// Initial log level threshold (messages with log level severity less than
// the threshold will not be logged); applies to every enabled logging
// destination and loggable HW component. See 'mvLog_t' enum for acceptable
// values.
// TODO: EISW-33556: Move log level definition (mvLog_t) to this file.
//
    pub default_trace_level: u32,
    pub boot_type: u32,
    pub punit_telemetry_sram_base: u64,
    pub punit_telemetry_sram_size: u64,
    pub vpu_telemetry_enable: u32,
    pub crit_tracing_buff_addr: u64,
    pub crit_tracing_buff_size: u32,
    pub verbose_tracing_buff_addr: u64,
    pub verbose_tracing_buff_size: u32,
    pub /: *mut *mut *mut u64 verbose_tracing_sw_component_mask; / TO BE REMOVED,
//
// Mask of destinations to which logging messages are delivered; bitwise OR
// of values defined in vpu_trace_destination enum.
//
    pub trace_destination_mask: u32,
//
// Mask of hardware components for which logging is enabled; bitwise OR of
// bits defined by the VPU_TRACE_PROC_BIT_* macros.
//
    pub trace_hw_component_mask: u64,
// Mask of trace message formats supported by the driver
    pub tracing_buff_message_format_mask: u64,
    pub trace_reserved_1: [u64; 2],
    pub reserved_6: u32,
// PLL ratio for efficient clock frequency
    pub pn_freq_pll_ratio: u32,
//
// DVFS Mode:
// 0 - Default, DVFS mode selected by the firmware
// 1 - Max Performance
// 2 - On Demand
// 3 - Power Save
// 4 - On Demand Priority Aware
//
    pub dvfs_mode: u32,
//
// Depending on DVFS Mode:
// On-demand: Default if 0.
// Bit 0-7   - uint8_t: Highest residency percent
// Bit 8-15  - uint8_t: High residency percent
// Bit 16-23 - uint8_t: Low residency percent
// Bit 24-31 - uint8_t: Lowest residency percent
// Bit 32-35 - unsigned 4b: PLL Ratio increase amount on highest residency
// Bit 36-39 - unsigned 4b: PLL Ratio increase amount on high residency
// Bit 40-43 - unsigned 4b: PLL Ratio decrease amount on low residency
// Bit 44-47 - unsigned 4b: PLL Ratio decrease amount on lowest frequency
// Bit 48-55 - uint8_t: Period (ms) for residency decisions
// Bit 56-63 - uint8_t: Averaging windows (as multiples of period. Max: 30 decimal)
// Power Save/Max Performance: Unused
//
    pub dvfs_param: u64,
//
// D0i3 delayed entry
// Bit 0: Disable CPU state save on D0i2 entry flow.
// 0: Every D0i2 entry saves state. Save state IPC message ignored.
// 1: IPC message required to save state on D0i3 entry flow.
// NOTE: This parameter is deprecated starting NPU50xx+. Bit 0 is now hardcoded to 1,
// meaning CPU state save always requires IPC message on D0i3 entry flow.
//
    pub d0i3_delayed_entry: u32,
// Time spent by VPU in D0i3 state
    pub d0i3_residency_time_us: u64,
// Value of VPU perf counter at the time of entering D0i3 state .
    pub d0i3_entry_vpu_ts: u64,
//
// The system time of the host operating system in microseconds.
// E.g the number of microseconds since 1st of January 1970, or whatever
// date the host operating system uses to maintain system time.
// This value will be used to track system time on the VPU.
// The KMD is required to update this value on every VPU reset.
//
    pub system_time_us: u64,
    pub reserved_7: [u32; 2],
//
// The delta between device monotonic time and the current value of the
// HW timestamp register, in ticks. Written by the firmware during boot.
// Can be used by the KMD to calculate device time.
//
    pub device_time_delta_ticks: u64,
    pub reserved_8: [u32; 30],
// Power States transitions timestamps: 0x440 - 0x46F
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_states_timestamps {
// VPU_IDLE -> VPU_ACTIVE transition initiated timestamp
    pub vpu_active_state_requested: u64,
// VPU_IDLE -> VPU_ACTIVE transition completed timestamp
    pub vpu_active_state_achieved: u64,
// VPU_ACTIVE -> VPU_IDLE transition initiated timestamp
    pub vpu_idle_state_requested: u64,
// VPU_ACTIVE -> VPU_IDLE transition completed timestamp
    pub vpu_idle_state_achieved: u64,
// VPU_IDLE -> VPU_STANDBY transition initiated timestamp
    pub vpu_standby_state_requested: u64,
// VPU_IDLE -> VPU_STANDBY transition completed timestamp
    pub vpu_standby_state_achieved: u64,
    pub power_states_timestamps: },
// VPU scheduling mode. Values defined by VPU_SCHEDULING_MODE_* macros.
    pub vpu_scheduling_mode: u32,
// Present call period in milliseconds.
    pub vpu_focus_present_timer_ms: u32,
// VPU ECC Signaling
    pub vpu_uses_ecc_mca_signal: u32,
// Values defined by POWER_PROFILE* macros
    pub power_profile: u32,
// Microsecond value for DCT active cycle
    pub dct_active_us: u32,
// Microsecond value for DCT inactive cycle
    pub dct_inactive_us: u32,
// Unused/reserved: 0x488 - 0xFFF
    pub reserved_9: [u32; 734],
}

// Magic numbers set between host and vpu to detect corruption of tracing init

// Tracing buffer message format definitions
pub const VPU_TRACING_FORMAT_STRING: c_int = 0;
pub const VPU_TRACING_FORMAT_MIPI: c_int = 2;
//
// Header of the tracing buffer.
// The below defined header will be stored at the beginning of
// each allocated tracing buffer, followed by a series of 256b
// of ASCII trace message entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_tracing_buffer_header {
//
// Magic number set by host to detect corruption
// @see VPU_TRACING_BUFFER_CANARY
//
    pub host_canary_start: u32,
// offset from start of buffer for trace entries
    pub read_index: u32,
// keeps track of wrapping on the reader side
    pub read_wrap_count: u32,
    pub pad_to_cache_line_size_0: [u32; 13],
// End of first cache line
//
// Magic number set by host to detect corruption
// @see VPU_TRACING_BUFFER_CANARY
//
    pub vpu_canary_start: u32,
// offset from start of buffer from write start
    pub write_index: u32,
// counter for buffer wrapping
    pub wrap_count: u32,
// legacy field - do not use
    pub reserved_0: u32,
//
// Size of the log buffer including this header (`header_size`) and space
// reserved for all messages. If `alignment` is greater than 0, the `size`
// must be a multiple of `alignment`.
//
    pub size: u32,
// Header version
    pub header_version: u16,
// Header size
    pub header_size: u16,
//
// Format of the messages in the trace buffer
// 0 - null terminated string
// 1 - size + null terminated string
// 2 - MIPI-SysT encoding
//
    pub format: u32,
//
// Message alignment
// 0 - messages are place 1 after another
// n - every message starts and multiple on offset
//
    pub /: *mut *mut *mut u32 alignment; / 64, 128, 256,
// Name of the logging entity, i.e "LRT", "LNN", "SHV0", etc
    pub name: [c_char; 16],
    pub pad_to_cache_line_size_1: [u32; 4],
// End of second cache line
}

// @}
