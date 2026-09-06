//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/isst_if.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Intel Speed Select Interface: OS to hardware Interface
// Copyright (c) 2019, Intel Corporation.
// All rights reserved.
//
// Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>
//

//
// struct isst_if_platform_info - Define platform information
// @api_version:	Version of the firmware document, which this driver
// can communicate
// @driver_version:	Driver version, which will help user to send right
// commands. Even if the firmware is capable, driver may
// not be ready
// @max_cmds_per_ioctl:	Returns the maximum number of commands driver will
// accept in a single ioctl
// @mbox_supported:	Support of mail box interface
// @mmio_supported:	Support of mmio interface for core-power feature
//
// Used to return output of IOCTL ISST_IF_GET_PLATFORM_INFO. This
// information can be used by the user space, to get the driver, firmware
// support and also number of commands to send in a single IOCTL request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_platform_info {
    pub api_version: __u16,
    pub driver_version: __u16,
    pub max_cmds_per_ioctl: __u16,
    pub mbox_supported: __u8,
    pub mmio_supported: __u8,
}

//
// struct isst_if_cpu_map - CPU mapping between logical and physical CPU
// @logical_cpu:	Linux logical CPU number
// @physical_cpu:	PUNIT CPU number
//
// Used to convert from Linux logical CPU to PUNIT CPU numbering scheme.
// The PUNIT CPU number is different than APIC ID based CPU numbering.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_cpu_map {
    pub logical_cpu: __u32,
    pub physical_cpu: __u32,
}

//
// struct isst_if_cpu_maps - structure for CPU map IOCTL
// @cmd_count:	Number of CPU mapping command in cpu_map[]
// @cpu_map:	Holds one or more CPU map data structure
//
// This structure used with ioctl ISST_IF_GET_PHY_ID to send
// one or more CPU mapping commands. Here IOCTL return value indicates
// number of commands sent or error number if no commands have been sent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_cpu_maps {
    pub cmd_count: __u32,
    pub cpu_map: [isst_if_cpu_map; 1],
}

//
// struct isst_if_io_reg - Read write PUNIT IO register
// @read_write:		Value 0: Read, 1: Write
// @logical_cpu:	Logical CPU number to get target PCI device.
// @reg:		PUNIT register offset
// @value:		For write operation value to write and for
// read placeholder read value
//
// Structure to specify read/write data to PUNIT registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_io_reg {
    pub /: *mut *mut __u32 read_write; / Read:0, Write:1,
    pub logical_cpu: __u32,
    pub reg: __u32,
    pub value: __u32,
}

//
// struct isst_if_io_regs - structure for IO register commands
// @req_count:	Number of io reg commands in io_reg[]
// @io_reg:	Holds one or more io_reg command structure
//
// This structure used with ioctl ISST_IF_IO_CMD to send
// one or more read/write commands to PUNIT. Here IOCTL return value
// indicates number of requests sent or error number if no requests have
// been sent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_io_regs {
    pub req_count: __u32,
    pub io_reg: [isst_if_io_reg; 1],
}

//
// struct isst_if_mbox_cmd - Structure to define mail box command
// @logical_cpu:	Logical CPU number to get target PCI device
// @parameter:		Mailbox parameter value
// @req_data:		Request data for the mailbox
// @resp_data:		Response data for mailbox command response
// @command:		Mailbox command value
// @sub_command:	Mailbox sub command value
// @reserved:		Unused, set to 0
//
// Structure to specify mailbox command to be sent to PUNIT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_mbox_cmd {
    pub logical_cpu: __u32,
    pub parameter: __u32,
    pub req_data: __u32,
    pub resp_data: __u32,
    pub command: __u16,
    pub sub_command: __u16,
    pub reserved: __u32,
}

//
// struct isst_if_mbox_cmds - structure for mailbox commands
// @cmd_count:	Number of mailbox commands in mbox_cmd[]
// @mbox_cmd:	Holds one or more mbox commands
//
// This structure used with ioctl ISST_IF_MBOX_COMMAND to send
// one or more mailbox commands to PUNIT. Here IOCTL return value
// indicates number of commands sent or error number if no commands have
// been sent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_mbox_cmds {
    pub cmd_count: __u32,
    pub mbox_cmd: [isst_if_mbox_cmd; 1],
}

//
// struct isst_if_msr_cmd - Structure to define msr command
// @read_write:		Value 0: Read, 1: Write
// @logical_cpu:	Logical CPU number
// @msr:		MSR number
// @data:		For write operation, data to write, for read
// place holder
//
// Structure to specify MSR command related to PUNIT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_msr_cmd {
    pub /: *mut *mut __u32 read_write; / Read:0, Write:1,
    pub logical_cpu: __u32,
    pub msr: __u64,
    pub data: __u64,
}

//
// struct isst_if_msr_cmds - structure for msr commands
// @cmd_count:	Number of mailbox commands in msr_cmd[]
// @msr_cmd:	Holds one or more msr commands
//
// This structure used with ioctl ISST_IF_MSR_COMMAND to send
// one or more MSR commands. IOCTL return value indicates number of
// commands sent or error number if no commands have been sent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_msr_cmds {
    pub cmd_count: __u32,
    pub msr_cmd: [isst_if_msr_cmd; 1],
}

//
// struct isst_core_power - Structure to get/set core_power feature
// @get_set:	0: Get, 1: Set
// @socket_id:	Socket/package id
// @power_domain_id: Power Domain id
// @enable:	Feature enable status
// @supported:	Power domain supports SST_CP interface
// @priority_type: Priority type for the feature (ordered/proportional)
//
// Structure to get/set core_power feature state using IOCTL
// ISST_IF_CORE_POWER_STATE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_core_power {
    pub get_set: __u8,
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub enable: __u8,
    pub supported: __u8,
    pub priority_type: __u8,
}

//
// struct isst_clos_param - Structure to get/set clos praram
// @get_set:	0: Get, 1: Set
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @clos:	Clos ID for the parameters
// @min_freq_mhz: Minimum frequency in MHz
// @max_freq_mhz: Maximum frequency in MHz
// @prop_prio:	Proportional priority from 0-15
//
// Structure to get/set per clos property using IOCTL
// ISST_IF_CLOS_PARAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_clos_param {
    pub get_set: __u8,
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub clos: __u8,
    pub min_freq_mhz: __u16,
    pub max_freq_mhz: __u16,
    pub prop_prio: __u8,
}

//
// struct isst_if_clos_assoc - Structure to assign clos to a CPU
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @logical_cpu: CPU number
// @clos:	Clos ID to assign to the logical CPU
//
// Structure to get/set core_power feature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_clos_assoc {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub logical_cpu: __u16,
    pub clos: __u16,
}

//
// struct isst_if_clos_assoc_cmds - Structure to assign clos to CPUs
// @cmd_count:	Number of cmds (cpus) in this request
// @get_set:	Request is for get or set
// @punit_cpu_map: Set to 1 if the CPU number is punit numbering not
// Linux CPU number
// @assoc_info: CLOS data for this CPU
//
// Structure used to get/set associate CPUs to clos using IOCTL
// ISST_IF_CLOS_ASSOC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_clos_assoc_cmds {
    pub cmd_count: __u16,
    pub get_set: __u16,
    pub punit_cpu_map: __u16,
    pub assoc_info: [isst_if_clos_assoc; 1],
}

//
// struct isst_tpmi_instance_count - Get number of TPMI instances per socket
// @socket_id:	Socket/package id
// @count:	Number of instances
// @valid_mask: Mask of instances as there can be holes
//
// Structure used to get TPMI instances information using
// IOCTL ISST_IF_COUNT_TPMI_INSTANCES.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_tpmi_instance_count {
    pub socket_id: __u8,
    pub count: __u8,
    pub valid_mask: __u16,
}

//
// struct isst_perf_level_info - Structure to get information on SST-PP levels
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @logical_cpu: CPU number
// @clos:	Clos ID to assign to the logical CPU
// @max_level: Maximum performance level supported by the platform
// @feature_rev: The feature revision for SST-PP supported by the platform
// @level_mask: Mask of supported performance levels
// @current_level: Current performance level
// @feature_state: SST-BF and SST-TF (enabled/disabled) status at current level
// @locked: SST-PP performance level change is locked/unlocked
// @enabled: SST-PP feature is enabled or not
// @sst_tf_support: SST-TF support status at this level
// @sst_bf_support: SST-BF support status at this level
//
// Structure to get SST-PP details using IOCTL ISST_IF_PERF_LEVELS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_perf_level_info {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub max_level: __u8,
    pub feature_rev: __u8,
    pub level_mask: __u8,
    pub current_level: __u8,
    pub feature_state: __u8,
    pub locked: __u8,
    pub enabled: __u8,
    pub sst_tf_support: __u8,
    pub sst_bf_support: __u8,
}

//
// struct isst_perf_level_control - Structure to set SST-PP level
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @level:	level to set
//
// Structure used change SST-PP level using IOCTL ISST_IF_PERF_SET_LEVEL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_perf_level_control {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub level: __u8,
}

//
// struct isst_perf_feature_control - Structure to activate SST-BF/SST-TF
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @feature:	bit 0 = SST-BF state, bit 1 = SST-TF state
//
// Structure used to enable SST-BF/SST-TF using IOCTL ISST_IF_PERF_SET_FEATURE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_perf_feature_control {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub feature: __u8,
}

pub const TRL_MAX_BUCKETS: c_int = 8;
pub const TRL_MAX_LEVELS: c_int = 6;
//
// struct isst_perf_level_data_info - Structure to get SST-PP level details
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @level:	SST-PP level for which caller wants to get information
// @tdp_ratio: TDP Ratio
// @base_freq_mhz: Base frequency in MHz
// @base_freq_avx2_mhz: AVX2 Base frequency in MHz
// @base_freq_avx512_mhz: AVX512 base frequency in MHz
// @base_freq_amx_mhz: AMX base frequency in MHz
// @thermal_design_power_w: Thermal design (TDP) power
// @tjunction_max_c: Max junction temperature
// @max_memory_freq_mhz: Max memory frequency in MHz
// @cooling_type: Type of cooling is used
// @p0_freq_mhz: core maximum frequency
// @p1_freq_mhz: Core TDP frequency
// @pn_freq_mhz: Core maximum efficiency frequency
// @pm_freq_mhz: Core minimum frequency
// @p0_fabric_freq_mhz: Fabric (Uncore) maximum frequency
// @p1_fabric_freq_mhz: Fabric (Uncore) TDP frequency
// @pn_fabric_freq_mhz: Fabric (Uncore) minimum efficiency frequency
// @pm_fabric_freq_mhz: Fabric (Uncore) minimum frequency
// @max_buckets: Maximum trl buckets
// @max_trl_levels: Maximum trl levels
// @bucket_core_counts: Number of cores per bucket
// @trl_freq_mhz: maximum frequency
// for a bucket and trl level
//
// Structure used to get information on frequencies and TDP for a SST-PP
// level using ISST_IF_GET_PERF_LEVEL_INFO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_perf_level_data_info {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub level: __u16,
    pub tdp_ratio: __u16,
    pub base_freq_mhz: __u16,
    pub base_freq_avx2_mhz: __u16,
    pub base_freq_avx512_mhz: __u16,
    pub base_freq_amx_mhz: __u16,
    pub thermal_design_power_w: __u16,
    pub tjunction_max_c: __u16,
    pub max_memory_freq_mhz: __u16,
    pub cooling_type: __u16,
    pub p0_freq_mhz: __u16,
    pub p1_freq_mhz: __u16,
    pub pn_freq_mhz: __u16,
    pub pm_freq_mhz: __u16,
    pub p0_fabric_freq_mhz: __u16,
    pub p1_fabric_freq_mhz: __u16,
    pub pn_fabric_freq_mhz: __u16,
    pub pm_fabric_freq_mhz: __u16,
    pub max_buckets: __u16,
    pub max_trl_levels: __u16,
    pub bucket_core_counts: [__u16; TRL_MAX_BUCKETS],
    pub trl_freq_mhz: [__u16; TRL_MAX_LEVELS][TRL_MAX_BUCKETS],
}

pub const MAX_FABRIC_COUNT: c_int = 8;
//
// struct isst_perf_level_fabric_info - Structure to get SST-PP fabric details
// @socket_id:		Socket/package id
// @power_domain_id:	Power Domain id
// @level:		SST-PP level for which caller wants to get information
// @max_fabrics:	Count of fabrics in resonse
// @p0_fabric_freq_mhz: Fabric (Uncore) maximum frequency
// @p1_fabric_freq_mhz: Fabric (Uncore) TDP frequency
// @pm_fabric_freq_mhz: Fabric (Uncore) minimum frequency
//
// Structure used to get information on frequencies for fabrics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_perf_level_fabric_info {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub level: __u16,
    pub max_fabrics: __u16,
    pub p0_fabric_freq_mhz: [__u16; MAX_FABRIC_COUNT],
    pub p1_fabric_freq_mhz: [__u16; MAX_FABRIC_COUNT],
    pub pm_fabric_freq_mhz: [__u16; MAX_FABRIC_COUNT],
}

//
// struct isst_perf_level_cpu_mask - Structure to get SST-PP level CPU mask
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @level:	SST-PP level for which caller wants to get information
// @punit_cpu_map: Set to 1 if the CPU number is punit numbering not
// Linux CPU number. If 0 CPU buffer is copied to user space
// supplied cpu_buffer of size cpu_buffer_size. Punit
// cpu mask is copied to "mask" field.
// @mask:	cpu mask for this PP level (punit CPU numbering)
// @cpu_buffer_size: size of cpu_buffer also used to return the copied CPU
// buffer size.
// @cpu_buffer:	Buffer to copy CPU mask when punit_cpu_map is 0
//
// Structure used to get cpumask for a SST-PP level using
// IOCTL ISST_IF_GET_PERF_LEVEL_CPU_MASK. Also used to get CPU mask for
// IOCTL ISST_IF_GET_BASE_FREQ_CPU_MASK for SST-BF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_perf_level_cpu_mask {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub level: __u8,
    pub punit_cpu_map: __u8,
    pub mask: __u64,
    pub cpu_buffer_size: __u16,
    pub cpu_buffer: [__s8; 1],
}

//
// struct isst_base_freq_info - Structure to get SST-BF frequencies
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @level:	SST-PP level for which caller wants to get information
// @high_base_freq_mhz: High priority CPU base frequency
// @low_base_freq_mhz: Low priority CPU base frequency
// @tjunction_max_c: Max junction temperature
// @thermal_design_power_w: Thermal design power in watts
//
// Structure used to get SST-BF information using
// IOCTL ISST_IF_GET_BASE_FREQ_INFO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_base_freq_info {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub level: __u16,
    pub high_base_freq_mhz: __u16,
    pub low_base_freq_mhz: __u16,
    pub tjunction_max_c: __u16,
    pub thermal_design_power_w: __u16,
}

//
// struct isst_turbo_freq_info - Structure to get SST-TF frequencies
// @socket_id:	Socket/package id
// @power_domain_id:	Power Domain id
// @level:	SST-PP level for which caller wants to get information
// @max_clip_freqs: Maximum number of low priority core clipping frequencies
// @max_buckets: Maximum trl buckets
// @max_trl_levels: Maximum trl levels
// @lp_clip_freq_mhz: Clip frequencies per trl level
// @bucket_core_counts: Maximum number of cores for a bucket
// @trl_freq_mhz: Frequencies per trl level for each bucket
//
// Structure used to get SST-TF information using
// IOCTL ISST_IF_GET_TURBO_FREQ_INFO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_turbo_freq_info {
    pub socket_id: __u8,
    pub power_domain_id: __u8,
    pub level: __u16,
    pub max_clip_freqs: __u16,
    pub max_buckets: __u16,
    pub max_trl_levels: __u16,
    pub lp_clip_freq_mhz: [__u16; TRL_MAX_LEVELS],
    pub bucket_core_counts: [__u16; TRL_MAX_BUCKETS],
    pub trl_freq_mhz: [__u16; TRL_MAX_LEVELS][TRL_MAX_BUCKETS],
}

pub const ISST_IF_MAGIC: c_uint = 0xFE;

