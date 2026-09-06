//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_fwif.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// Logging type
//
pub const ROGUE_FWIF_LOG_TYPE_NONE: c_uint = 0x00000000U;
pub const ROGUE_FWIF_LOG_TYPE_TRACE: c_uint = 0x00000001U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_MAIN: c_uint = 0x00000002U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_MTS: c_uint = 0x00000004U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_CLEANUP: c_uint = 0x00000008U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_CSW: c_uint = 0x00000010U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_BIF: c_uint = 0x00000020U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_PM: c_uint = 0x00000040U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_RTD: c_uint = 0x00000080U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_SPM: c_uint = 0x00000100U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_POW: c_uint = 0x00000200U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_HWR: c_uint = 0x00000400U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_HWP: c_uint = 0x00000800U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_RPM: c_uint = 0x00001000U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_DMA: c_uint = 0x00002000U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_MISC: c_uint = 0x00004000U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_DEBUG: c_uint = 0x80000000U;
pub const ROGUE_FWIF_LOG_TYPE_GROUP_MASK: c_uint = 0x80007FFEU;
pub const ROGUE_FWIF_LOG_TYPE_MASK: c_uint = 0x80007FFFU;
// String used in pvrdebug -h output

// Table entry to map log group strings to log type value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_log_group_map_entry {
    pub log_group_name: *const c_char,
    pub log_group_type: u32,
}

//
// ROGUE FW signature checks
//

//
// Trace Buffer
//
// Default size of ROGUE_FWIF_TRACEBUF_SPACE in DWords

pub const ROGUE_FW_POLL_TYPE_SET: c_uint = 0x80000000U;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_file_info_buf {
    pub path: [c_char; ROGUE_FW_TRACE_BUFFER_ASSERT_SIZE],
    pub info: [c_char; ROGUE_FW_TRACE_BUFFER_ASSERT_SIZE],
    pub line_num: u32,
    pub padding: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_tracebuf_space {
    pub trace_pointer: u32,
    pub trace_buffer_fw_addr: u32,
// To be used by host when reading from trace buffer
    pub trace_buffer: *mut u32,
    pub assert_buf: rogue_fwif_file_info_buf,
    pub __aligned(8): },
// Total number of FW fault logs stored

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fw_fault_info {
    pub cr_timer: aligned_u64,
    pub os_timer: aligned_u64,
    pub __aligned(8): u32 data,
    pub reserved: u32,
    pub fault_buf: rogue_fwif_file_info_buf,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_pow_state {
    ROGUE_FWIF_POW_OFF, /* idle and ready to full power down */
    ROGUE_FWIF_POW_ON, /* running HW commands */
    ROGUE_FWIF_POW_FORCED_IDLE, /* forced idle */
    ROGUE_FWIF_POW_IDLE, /* idle waiting for host handshake */
}

// Firmware HWR states
// The HW state is ok or locked up

// Tells if a HWR reset is in progress

// A DM unrelated lockup has been detected

// At least one DM is running without being close to a lockup

// At least one DM is close to lockup

// The FW has faulted and needs to restart

// The FW has requested the host to restart it

// The FW has requested the host to restart it, per PHR configuration

// A PHR triggered GPU reset has just finished

// Firmware per-DM HWR states
// DM is working if all flags are cleared

// DM is idle and ready for HWR

// DM need to skip to next cmd before resuming processing

// DM need partial render cleanup before resuming processing

// DM need to increment Recovery Count once fully recovered

// DM was identified as locking up and causing HWR

// DM was innocently affected by another lockup which caused HWR

// DM was identified as over-running and causing HWR

// DM was innocently affected by another DM over-running which caused HWR

// DM was forced into HWR as it delayed more important workloads

// DM was forced into HWR due to an uncorrected GPU ECC error

// Firmware's connection state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_connection_fw_state {
// Firmware is offline
    ROGUE_FW_CONNECTION_FW_OFFLINE = 0,
// Firmware is initialised
    ROGUE_FW_CONNECTION_FW_READY,
// Firmware connection is fully established
    ROGUE_FW_CONNECTION_FW_ACTIVE,
// Firmware is clearing up connection data
    ROGUE_FW_CONNECTION_FW_OFFLOADING,
    ROGUE_FW_CONNECTION_FW_STATE_COUNT
}

// OS' connection state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_connection_os_state {
// OS is offline
    ROGUE_FW_CONNECTION_OS_OFFLINE = 0,
// OS's KM driver is setup and waiting
    ROGUE_FW_CONNECTION_OS_READY,
// OS connection is fully established
    ROGUE_FW_CONNECTION_OS_ACTIVE,
    ROGUE_FW_CONNECTION_OS_STATE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_os_runtime_flags {
    pub 3: unsigned int os_state :,
    pub 1: unsigned int fl_ok :,
    pub 1: unsigned int fl_grow_pending :,
    pub 1: unsigned int isolated_os :,
    pub 26: unsigned int reserved :,
}

pub const PVR_SLR_LOG_ENTRIES: c_int = 10;
// MAX_CLIENT_CCB_NAME not visible to this header
pub const PVR_SLR_LOG_STRLEN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_slr_entry {
    pub timestamp: aligned_u64,
    pub fw_ctx_addr: u32,
    pub num_ufos: u32,
    pub ccb_name: [c_char; PVR_SLR_LOG_STRLEN],
    pub padding: [c_char; 2],
    pub __aligned(8): },
pub const MAX_THREAD_NUM: c_int = 2;
// firmware trace control data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_tracebuf {
    pub log_type: u32,
    pub tracebuf: [rogue_fwif_tracebuf_space; MAX_THREAD_NUM],
//
// Member initialised only when sTraceBuf is actually allocated (in
// ROGUETraceBufferInitOnDemandResources)
//
    pub tracebuf_size_in_dwords: u32,
// Compatibility and other flags
    pub tracebuf_flags: u32,
    pub __aligned(8): },
// firmware system data shared with the Host driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_sysdata {
// Configuration flags from host
    pub config_flags: u32,
// Extended configuration flags from host
    pub config_flags_ext: u32,
    pub pow_state: rogue_fwif_pow_state,
    pub hw_perf_ridx: u32,
    pub hw_perf_widx: u32,
    pub hw_perf_wrap_count: u32,
// Constant after setup, needed in FW
    pub hw_perf_size: u32,
// The number of times the FW drops a packet due to buffer full
    pub hw_perf_drop_count: u32,
//
// ui32HWPerfUt, ui32FirstDropOrdinal, ui32LastDropOrdinal only valid
// when FW is built with ROGUE_HWPERF_UTILIZATION &
// ROGUE_HWPERF_DROP_TRACKING defined in rogue_fw_hwperf.c
//
// Buffer utilisation, high watermark of bytes in use
    pub hw_perf_ut: u32,
// The ordinal of the first packet the FW dropped
    pub first_drop_ordinal: u32,
// The ordinal of the last packet the FW dropped
    pub last_drop_ordinal: u32,
// State flags for each Operating System mirrored from Fw coremem
    pub fault_info: [rogue_fw_fault_info; ROGUE_FWIF_FWFAULTINFO_MAX],
    pub fw_faults: u32,
    pub cr_poll_addr: [u32; MAX_THREAD_NUM],
    pub cr_poll_mask: [u32; MAX_THREAD_NUM],
    pub cr_poll_count: [u32; MAX_THREAD_NUM],
    pub start_idle_time: aligned_u64,

    pub __aligned(8): u32 fw_stats_buf[ROGUE_FWIF_STATS_FRAMEWORK_MAX],

    pub hwr_state_flags: u32,
    pub hwr_recovery_flags: [u32; PVR_FWIF_DM_MAX],
// Compatibility and other flags
    pub fw_sys_data_flags: u32,
// Identify whether MC config is P-P or P-S
    pub mc_config: u32,
    pub __aligned(8): },
// per-os firmware shared data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_osdata {
// Configuration flags from an OS
    pub fw_os_config_flags: u32,
// Markers to signal that the host should perform a full sync check
    pub fw_sync_check_mark: u32,
    pub host_sync_check_mark: u32,
    pub forced_updates_requested: u32,
    pub slr_log_wp: u8,
    pub slr_log_first: rogue_fwif_slr_entry,
    pub slr_log: [rogue_fwif_slr_entry; PVR_SLR_LOG_ENTRIES],
    pub last_forced_update_time: aligned_u64,
// Interrupt count from Threads >
    pub interrupt_count: [u32; MAX_THREAD_NUM],
    pub kccb_cmds_executed: u32,
    pub power_sync_fw_addr: u32,
// Compatibility and other flags
    pub fw_os_data_flags: u32,
    pub padding: u32,
    pub __aligned(8): },
// Firmware trace time-stamp field breakup
// ROGUE_CR_TIMER register read (48 bits) value

// Extra debug-info (16 bits)

// Debug-info sub-fields
//
// Bit 0: ROGUE_CR_EVENT_STATUS_MMU_PAGE_FAULT bit from ROGUE_CR_EVENT_STATUS
// register
//

// Bit 1: ROGUE_CR_BIF_MMU_ENTRY_PENDING bit from ROGUE_CR_BIF_MMU_ENTRY register

// Bit 2: ROGUE_CR_SLAVE_EVENT register is non-zero

// Bit 3-15: Unused bits
pub const ROGUE_FWT_DEBUG_INFO_STR_MAXLEN: c_int = 64;

//
// HWR Data
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_hwrtype {
    ROGUE_HWRTYPE_UNKNOWNFAILURE = 0,
    ROGUE_HWRTYPE_OVERRUN = 1,
    ROGUE_HWRTYPE_POLLFAILURE = 2,
    ROGUE_HWRTYPE_BIF0FAULT = 3,
    ROGUE_HWRTYPE_BIF1FAULT = 4,
    ROGUE_HWRTYPE_TEXASBIF0FAULT = 5,
    ROGUE_HWRTYPE_MMUFAULT = 6,
    ROGUE_HWRTYPE_MMUMETAFAULT = 7,
    ROGUE_HWRTYPE_MIPSTLBFAULT = 8,
    ROGUE_HWRTYPE_ECCFAULT = 9,
    ROGUE_HWRTYPE_MMURISCVFAULT = 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_bifinfo {
    pub bif_req_status: aligned_u64,
    pub bif_mmu_status: aligned_u64,
    pub /: *mut *mut aligned_u64 pc_address; / phys address of the page catalogue,
    pub reserved: aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_eccinfo {
    pub fault_gpu: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_mmuinfo {
    pub mmu_status: [aligned_u64; 2],
    pub /: *mut *mut aligned_u64 pc_address; / phys address of the page catalogue,
    pub reserved: aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_pollinfo {
    pub thread_num: u32,
    pub cr_poll_addr: u32,
    pub cr_poll_mask: u32,
    pub cr_poll_last_value: u32,
    pub reserved: aligned_u64,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_tlbinfo {
    pub bad_addr: u32,
    pub entry_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_hwrinfo {
    pub bif_info: rogue_bifinfo,
    pub mmu_info: rogue_mmuinfo,
    pub poll_info: rogue_pollinfo,
    pub tlb_info: rogue_tlbinfo,
    pub ecc_info: rogue_eccinfo,
    pub hwr_data: },
    pub cr_timer: aligned_u64,
    pub os_timer: aligned_u64,
    pub frame_num: u32,
    pub pid: u32,
    pub active_hwrt_data: u32,
    pub hwr_number: u32,
    pub event_status: u32,
    pub hwr_recovery_flags: u32,
    pub hwr_type: rogue_hwrtype,
    pub dm: u32,
    pub core_id: u32,
    pub cr_time_of_kick: aligned_u64,
    pub cr_time_hw_reset_start: aligned_u64,
    pub cr_time_hw_reset_finish: aligned_u64,
    pub cr_time_freelist_ready: aligned_u64,
    pub reserved: [aligned_u64; 2],
    pub __aligned(8): },
// Number of first HWR logs recorded (never overwritten by newer logs)

// Number of latest HWR logs (older logs are overwritten by newer logs)

// Total number of HWR logs stored in a buffer

// Index of the last log in the HWR log buffer

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwrinfobuf {
    pub hwr_info: [rogue_hwrinfo; ROGUE_FWIF_HWINFO_MAX],
    pub hwr_counter: u32,
    pub write_index: u32,
    pub dd_req_count: u32,
    pub /: *mut *mut u32 hwr_info_buf_flags; / Compatibility and other flags,
    pub hwr_dm_locked_up_count: [u32; PVR_FWIF_DM_MAX],
    pub hwr_dm_overran_count: [u32; PVR_FWIF_DM_MAX],
    pub hwr_dm_recovered_count: [u32; PVR_FWIF_DM_MAX],
    pub hwr_dm_false_detect_count: [u32; PVR_FWIF_DM_MAX],
    pub __aligned(8): },

//
// ROGUE firmware Init Config Data
//
// Flag definitions affecting the firmware globally

// Bit 5 is reserved.

// Bit 9 is reserved.
// Bit 10 is reserved.
// Bit 11 is reserved.

// Bit 15 is reserved.

// Extended Flag definitions affecting the firmware globally

// [7]   YUV10 override
// [6:4] Quality
// [3]   Quality enable
// [2:1] Compression scheme
// [0]   Lossy group
//

// Flag definitions affecting only workloads submitted by a particular OS

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_activepm_conf {
    ROGUE_ACTIVEPM_FORCE_OFF = 0,
    ROGUE_ACTIVEPM_FORCE_ON = 1,
    ROGUE_ACTIVEPM_DEFAULT = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_rd_power_island_conf {
    ROGUE_RD_POWER_ISLAND_FORCE_OFF = 0,
    ROGUE_RD_POWER_ISLAND_FORCE_ON = 1,
    ROGUE_RD_POWER_ISLAND_DEFAULT = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fw_register_list {
// Register number
    pub reg_num: u16,
// Indirect register number (or 0 if not used)
    pub indirect_reg_num: u16,
// Start value for indirect register
    pub indirect_start_val: u16,
// End value for indirect register
    pub indirect_end_val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_dllist_node {
    pub p: u32,
    pub n: u32,
}

//
// This number is used to represent an invalid page catalogue physical address
//
pub const ROGUE_FWIF_INVALID_PC_PHYADDR: c_uint = 0xFFFFFFFFFFFFFFFFLLU;
// This number is used to represent unallocated page catalog base register
pub const ROGUE_FW_BIF_INVALID_PCSET: c_uint = 0xFFFFFFFFU;
// Firmware memory context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwmemcontext {
// device physical address of context's page catalogue
    pub pc_dev_paddr: aligned_u64,
//
// associated page catalog base register (ROGUE_FW_BIF_INVALID_PCSET ==
// unallocated)
//
    pub page_cat_base_reg_set: u32,
// breakpoint address
    pub breakpoint_addr: u32,
// breakpoint handler address
    pub bp_handler_addr: u32,
// DM and enable control for BP
    pub breakpoint_ctl: u32,
// Compatibility and other flags
    pub fw_mem_ctx_flags: u32,
    pub padding: u32,
    pub __aligned(8): },
//
// FW context state flags
//

pub const ROGUE_NUM_GEOM_CORES_MAX: c_int = 4;
//
// FW-accessible TA state which must be written out to memory on context store
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_geom_ctx_state_per_geom {
// To store in mid-TA
    pub geom_reg_vdm_call_stack_pointer: aligned_u64,
// Initial value (in case is 'lost' due to a lock-up
    pub geom_reg_vdm_call_stack_pointer_init: aligned_u64,
    pub geom_reg_vbs_so_prim: [u32; 4],
    pub geom_current_idx: u16,
    pub padding: [u16; 3],
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_geom_ctx_state {
// FW-accessible TA state which must be written out to memory on context store
    pub geom_core: [rogue_fwif_geom_ctx_state_per_geom; ROGUE_NUM_GEOM_CORES_MAX],
    pub __aligned(8): },
//
// FW-accessible ISP state which must be written out to memory on context store
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_frag_ctx_state {
    pub frag_reg_pm_deallocated_mask_status: u32,
    pub frag_reg_dm_pds_mtilefree_status: u32,
// Compatibility and other flags
    pub ctx_state_flags: u32,
//
// frag_reg_isp_store should be the last element of the structure as this
// is an array whose size is determined at runtime after detecting the
// ROGUE core
//
    pub frag_reg_isp_store: [u32; ],
    pub __aligned(8): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_compute_ctx_state {
    pub /: *mut *mut u32 ctx_state_flags; / Target buffer and other flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwcommoncontext {
// CCB details for this firmware context
    pub /: *mut *mut u32 ccbctl_fw_addr; / CCB control,
    pub /: *mut *mut u32 ccb_fw_addr; / CCB base,
    pub ccb_meta_dma_addr: rogue_fwif_dma_addr,
// Context suspend state
// geom/frag context suspend state, read/written by FW
    pub __aligned(8): u32 context_state_addr,
// Flags e.g. for context switching
    pub fw_com_ctx_flags: u32,
    pub priority: u32,
    pub priority_seq_num: u32,
// Framework state
// Register updates for Framework
    pub __aligned(8): u32 rf_cmd_addr,
// Statistic updates waiting to be passed back to the host...
// True when some stats are pending
    pub __aligned(4): bool stats_pending,
// Number of stores on this context since last update
    pub stats_num_stores: i32,
// Number of OOMs on this context since last update
    pub stats_num_out_of_memory: i32,
// Number of PRs on this context since last update
    pub stats_num_partial_renders: i32,
// Data Master type
    pub dm: u32,
// Device Virtual Address of the signal the context is waiting on
    pub wait_signal_address: aligned_u64,
// List entry for the wait-signal list
    pub __aligned(8): rogue_fwif_dllist_node wait_signal_node,
// List entry for the buffer stalled list
    pub __aligned(8): rogue_fwif_dllist_node buf_stalled_node,
// Address of the circular buffer queue pointers
    pub cbuf_queue_ctrl_addr: aligned_u64,
    pub robustness_address: aligned_u64,
// Max HWR deadline limit in ms
    pub max_deadline_ms: u32,
// Following HWR circular buffer read-offset needs resetting
    pub read_offset_needs_reset: bool,
// List entry for the waiting list
    pub __aligned(8): rogue_fwif_dllist_node waiting_node,
// List entry for the run list
    pub __aligned(8): rogue_fwif_dllist_node run_node,
// UFO that last failed (or NULL)
    pub last_failed_ufo: rogue_fwif_ufo,
// Memory context
    pub fw_mem_context_fw_addr: u32,
// References to the host side originators
// the Server Common Context
    pub server_common_context_id: u32,
// associated process ID
    pub pid: u32,
// True when Geom DM OOM is not allowed
    pub __aligned(4): bool geom_oom_disabled,
    pub __aligned(8): },
// Firmware render context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwrendercontext {
// Geometry firmware context.
    pub geom_context: rogue_fwif_fwcommoncontext,
// Fragment firmware context.
    pub frag_context: rogue_fwif_fwcommoncontext,
    pub static_render_context_state: rogue_fwif_static_rendercontext_state,
// Number of commands submitted to the WorkEst FW CCB
    pub work_est_ccb_submitted: u32,
// Compatibility and other flags
    pub fw_render_ctx_flags: u32,
    pub __aligned(8): },
// Firmware compute context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwcomputecontext {
// Firmware context for the CDM
    pub cdm_context: rogue_fwif_fwcommoncontext,
// Number of commands submitted to the WorkEst FW CCB
    pub work_est_ccb_submitted: u32,
// Compatibility and other flags
    pub compute_ctx_flags: u32,
    pub wgp_state: u32,
    pub wgp_checksum: u32,
    pub core_mask_a: u32,
    pub core_mask_b: u32,
    pub __aligned(8): },
// Firmware TDM context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwtdmcontext {
// Firmware context for the TDM
    pub tdm_context: rogue_fwif_fwcommoncontext,
// Number of commands submitted to the WorkEst FW CCB
    pub work_est_ccb_submitted: u32,
    pub __aligned(8): },
// Firmware TQ3D context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwtransfercontext {
// Firmware context for TQ3D.
    pub tq_context: rogue_fwif_fwcommoncontext,
    pub __aligned(8): },
//
// Defines for CMD_TYPE corruption detection and forward compatibility check
//
// CMD_TYPE 32bit contains:
// 31:16	Reserved for magic value to detect corruption (16 bits)
// 15		Reserved for ROGUE_CCB_TYPE_TASK (1 bit)
// 14:0		Bits available for CMD_TYPEs (15 bits)
//
// Magic value to detect corruption

// Kernel CCB control for ROGUE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_ccb_ctl {
// write offset into array of commands (MUST be aligned to 16 bytes!)
    pub write_offset: u32,
// Padding to ensure read and write offsets are in separate cache lines.
    pub sizeof(u32)]: u8 padding[128 -,
// read offset into array of commands
    pub read_offset: u32,
// Offset wrapping mask (Total capacity of the CCB - 1)
    pub wrap_mask: u32,
// size of each command in bytes
    pub cmd_size: u32,
    pub padding2: u32,
    pub __aligned(8): },
// Kernel CCB command structure for ROGUE

//
// can't use PM_TLB0 bit from BIFPM_CTRL reg because it collides with PT
// bit from BIF_CTRL reg
//

// BIF_CTRL_INVAL_TLB1_EN

// MMU_CTRL_INVAL_ALL_CONTEXTS_EN

// indicates FW should interrupt the host

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_mmucachedata {
    pub cache_flags: u32,
    pub mmu_cache_sync_fw_addr: u32,
    pub mmu_cache_sync_update_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_bpdata {
// Memory context
    pub fw_mem_context_fw_addr: u32,
// Breakpoint address
    pub bp_addr: u32,
// Breakpoint handler
    pub bp_handler_addr: u32,
// Breakpoint control
    pub bp_dm: u32,
    pub bp_data_flags: u32,
// Number of temporary registers to overallocate
    pub temp_regs: u32,
// Number of shared registers to overallocate
    pub shared_regs: u32,
// DM associated with the breakpoint
    pub dm: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_kccb_cmd_kick_data {
// address of the firmware context
    pub context_fw_addr: u32,
// Client CCB woff update
    pub client_woff_update: u32,
// Client CCB wrap mask update after CCCB growth
    pub client_wrap_mask_update: u32,
// number of CleanupCtl pointers attached
    pub num_cleanup_ctl: u32,
// CleanupCtl structures associated with command
//
// offset to the CmdHeader which houses the workload estimation kick
// data.
//
    pub work_est_cmd_header_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_kccb_cmd_combined_geom_frag_kick_data {
    pub geom_cmd_kick_data: rogue_fwif_kccb_cmd_kick_data,
    pub frag_cmd_kick_data: rogue_fwif_kccb_cmd_kick_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_kccb_cmd_force_update_data {
// address of the firmware context
    pub context_fw_addr: u32,
// Client CCB fence offset
    pub ccb_fence_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_cleanup_type {
// FW common context cleanup
    ROGUE_FWIF_CLEANUP_FWCOMMONCONTEXT,
// FW HW RT data cleanup
    ROGUE_FWIF_CLEANUP_HWRTDATA,
// FW freelist cleanup
    ROGUE_FWIF_CLEANUP_FREELIST,
// FW ZS Buffer cleanup
    ROGUE_FWIF_CLEANUP_ZSBUFFER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cleanup_request {
// Cleanup type
    pub cleanup_type: rogue_fwif_cleanup_type,
// FW common context to cleanup
    pub context_fw_addr: u32,
// HW RT to cleanup
    pub hwrt_data_fw_addr: u32,
// Freelist to cleanup
    pub freelist_fw_addr: u32,
// ZS Buffer to cleanup
    pub zs_buffer_fw_addr: u32,
    pub cleanup_data: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_power_type {
    ROGUE_FWIF_POW_OFF_REQ = 1,
    ROGUE_FWIF_POW_FORCED_IDLE_REQ,
    ROGUE_FWIF_POW_NUM_UNITS_CHANGE,
    ROGUE_FWIF_POW_APM_LATENCY_CHANGE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_power_force_idle_type {
    ROGUE_FWIF_POWER_FORCE_IDLE = 1,
    ROGUE_FWIF_POWER_CANCEL_FORCED_IDLE,
    ROGUE_FWIF_POWER_HOST_TIMEOUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_power_request {
// Type of power request
    pub pow_type: rogue_fwif_power_type,
// Number of active Dusts
    pub num_of_dusts: u32,
// If the operation is mandatory
    pub __aligned(4): bool forced,
//
// Type of Request. Consolidating Force Idle, Cancel Forced
// Idle, Host Timeout
//
    pub pow_request_type: rogue_fwif_power_force_idle_type,
    pub power_req_data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_slcflushinvaldata {
// Context to fence on (only useful when bDMContext == TRUE)
    pub context_fw_addr: u32,
// Invalidate the cache as well as flushing
    pub __aligned(4): bool inval,
// The data to flush/invalidate belongs to a specific DM context
    pub __aligned(4): bool dm_context,
// Optional address of range (only useful when bDMContext == FALSE)
    pub address: aligned_u64,
// Optional size of range (only useful when bDMContext == FALSE)
    pub size: aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_hwperf_update_config {
    ROGUE_FWIF_HWPERF_CTRL_TOGGLE = 0,
    ROGUE_FWIF_HWPERF_CTRL_SET = 1,
    ROGUE_FWIF_HWPERF_CTRL_EMIT_FEATURES_EV = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwperf_ctrl {
    pub /: *mut *mut rogue_fwif_hwperf_update_config opcode; / Control operation code,
    pub /: *mut *mut aligned_u64 mask; / Mask of events to toggle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwperf_config_enable_blks {
// Number of ROGUE_HWPERF_CONFIG_MUX_CNTBLK in the array
    pub num_blocks: u32,
// Address of the ROGUE_HWPERF_CONFIG_MUX_CNTBLK array
    pub block_configs_fw_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwperf_config_da_blks {
// Number of ROGUE_HWPERF_CONFIG_CNTBLK in the array
    pub num_blocks: u32,
// Address of the ROGUE_HWPERF_CONFIG_CNTBLK array
    pub block_configs_fw_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_coreclkspeedchange_data {
    pub /: *mut *mut u32 new_clock_speed; / New clock speed,
}

pub const ROGUE_FWIF_HWPERF_CTRL_BLKS_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwperf_ctrl_blks {
    pub enable: bool,
// Number of block IDs in the array
    pub num_blocks: u32,
// Array of ROGUE_HWPERF_CNTBLK_ID values
    pub block_ids: [u16; ROGUE_FWIF_HWPERF_CTRL_BLKS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwperf_select_custom_cntrs {
    pub custom_block: u16,
    pub num_counters: u16,
    pub custom_counter_ids_fw_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_zsbuffer_backing_data {
    pub /: *mut *mut u32 zs_buffer_fw_addr; / ZS-Buffer FW address,
    pub /: *mut *mut bool done __aligned(4); / action backing/unbacking succeeded,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_freelist_gs_data {
// Freelist FW address
    pub freelist_fw_addr: u32,
// Amount of the Freelist change
    pub delta_pages: u32,
// New amount of pages on the freelist (including ready pages)
    pub new_pages: u32,
// Number of ready pages to be held in reserve until OOM
    pub ready_pages: u32,
}

pub const MAX_FREELISTS_SIZE: c_int = 3;
pub const MAX_HW_GEOM_FRAG_CONTEXTS_SIZE: c_int = 3;

pub const ROGUE_FWIF_FREELISTS_RECONSTRUCTION_FAILED_FLAG: c_uint = 0x80000000U;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_freelists_reconstruction_data {
    pub freelist_count: u32,
    pub freelist_ids: [u32; ROGUE_FWIF_MAX_FREELISTS_TO_RECONSTRUCT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_write_offset_update_data {
//
// Context to that may need to be resumed following write offset update
//
    pub context_fw_addr: u32,
    pub __aligned(8): },
//
// Proactive DVFS Structures
//
pub const NUM_OPP_VALUES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdvfs_opp {
    pub /: *mut *mut u32 volt; / V,
    pub /: *mut *mut u32 freq; / Hz,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_pdvfs_opp {
    pub opp_values: [pdvfs_opp; NUM_OPP_VALUES],
    pub min_opp_point: u32,
    pub max_opp_point: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_pdvfs_max_freq_data {
    pub max_opp_point: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_pdvfs_min_freq_data {
    pub min_opp_point: u32,
    pub __aligned(8): },
//
// Register configuration structures
//
pub const ROGUE_FWIF_REG_CFG_MAX_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_regdata_cmd_type {
    ROGUE_FWIF_REGCFG_CMD_ADD = 101,
    ROGUE_FWIF_REGCFG_CMD_CLEAR = 102,
    ROGUE_FWIF_REGCFG_CMD_ENABLE = 103,
    ROGUE_FWIF_REGCFG_CMD_DISABLE = 104
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_reg_cfg_type {
// Sidekick power event
    ROGUE_FWIF_REG_CFG_TYPE_PWR_ON = 0,
// Rascal / dust power event
    ROGUE_FWIF_REG_CFG_TYPE_DUST_CHANGE,
// Geometry kick
    ROGUE_FWIF_REG_CFG_TYPE_GEOM,
// Fragment kick
    ROGUE_FWIF_REG_CFG_TYPE_FRAG,
// Compute kick
    ROGUE_FWIF_REG_CFG_TYPE_CDM,
// TLA kick
    ROGUE_FWIF_REG_CFG_TYPE_TLA,
// TDM kick
    ROGUE_FWIF_REG_CFG_TYPE_TDM,
// Applies to all types. Keep as last element
    ROGUE_FWIF_REG_CFG_TYPE_ALL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_reg_cfg_rec {
    pub sddr: u64,
    pub mask: u64,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_regconfig_data {
    pub cmd_type: rogue_fwif_regdata_cmd_type,
    pub reg_config_type: rogue_fwif_reg_cfg_type,
    pub __aligned(8): rogue_fwif_reg_cfg_rec reg_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_reg_cfg {
//
// PDump WRW command write granularity is 32 bits.
// Add padding to ensure array size is 32 bit granular.
//
    pub __aligned(8): sizeof(u32))],
    pub __aligned(8): reg_configs[ROGUE_FWIF_REG_CFG_MAX_SIZE],
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_os_state_change {
    ROGUE_FWIF_OS_ONLINE = 1,
    ROGUE_FWIF_OS_OFFLINE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_os_state_change_data {
    pub osid: u32,
    pub new_os_state: rogue_fwif_os_state_change,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_counter_dump_request {
    ROGUE_FWIF_PWR_COUNTER_DUMP_START = 1,
    ROGUE_FWIF_PWR_COUNTER_DUMP_STOP,
    ROGUE_FWIF_PWR_COUNTER_DUMP_SAMPLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_counter_dump_data {
    pub counter_dump_request: rogue_fwif_counter_dump_request,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_kccb_cmd_type {
// Common commands
    ROGUE_FWIF_KCCB_CMD_KICK = 101U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
    ROGUE_FWIF_KCCB_CMD_MMUCACHE = 102U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
    ROGUE_FWIF_KCCB_CMD_BP = 103U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// SLC flush and invalidation request
    ROGUE_FWIF_KCCB_CMD_SLCFLUSHINVAL = 105U |
    ROGUE_CMD_MAGIC_DWORD_SHIFTED,
//
// Requests cleanup of a FW resource (type specified in the command
// data)
//
    ROGUE_FWIF_KCCB_CMD_CLEANUP = 106U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Power request
    ROGUE_FWIF_KCCB_CMD_POW = 107U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Backing for on-demand ZS-Buffer done
    ROGUE_FWIF_KCCB_CMD_ZSBUFFER_BACKING_UPDATE =
    108U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Unbacking for on-demand ZS-Buffer done
    ROGUE_FWIF_KCCB_CMD_ZSBUFFER_UNBACKING_UPDATE =
    109U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Freelist Grow done
    ROGUE_FWIF_KCCB_CMD_FREELIST_GROW_UPDATE =
    110U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Freelists Reconstruction done
    ROGUE_FWIF_KCCB_CMD_FREELISTS_RECONSTRUCTION_UPDATE =
    112U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
//
// Informs the firmware that the host has added more data to a CDM2
// Circular Buffer
//
    ROGUE_FWIF_KCCB_CMD_NOTIFY_WRITE_OFFSET_UPDATE =
    114U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Health check request
    ROGUE_FWIF_KCCB_CMD_HEALTH_CHECK = 115U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Forcing signalling of all unmet UFOs for a given CCB offset
    ROGUE_FWIF_KCCB_CMD_FORCE_UPDATE = 116U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// There is a geometry and a fragment command in this single kick
    ROGUE_FWIF_KCCB_CMD_COMBINED_GEOM_FRAG_KICK = 117U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Informs the FW that a Guest OS has come online / offline.
    ROGUE_FWIF_KCCB_CMD_OS_ONLINE_STATE_CONFIGURE	= 118U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// Commands only permitted to the native or host OS
    ROGUE_FWIF_KCCB_CMD_REGCONFIG = 200U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// Configure HWPerf events (to be generated) and HWPerf buffer address (if required)
    ROGUE_FWIF_KCCB_CMD_HWPERF_UPDATE_CONFIG = 201U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// Enable or disable multiple HWPerf blocks (reusing existing configuration)
    ROGUE_FWIF_KCCB_CMD_HWPERF_CTRL_BLKS = 203U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Core clock speed change event
    ROGUE_FWIF_KCCB_CMD_CORECLKSPEEDCHANGE = 204U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

//
// Ask the firmware to update its cached ui32LogType value from the (shared)
// tracebuf control structure
//
    ROGUE_FWIF_KCCB_CMD_LOGTYPE_UPDATE = 206U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Set a maximum frequency/OPP point
    ROGUE_FWIF_KCCB_CMD_PDVFS_LIMIT_MAX_FREQ = 207U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
//
// Changes the relative scheduling priority for a particular OSid. It can
// only be serviced for the Host DDK
//
    ROGUE_FWIF_KCCB_CMD_OSID_PRIORITY_CHANGE = 208U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Set or clear firmware state flags
    ROGUE_FWIF_KCCB_CMD_STATEFLAGS_CTRL = 209U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// Set a minimum frequency/OPP point
    ROGUE_FWIF_KCCB_CMD_PDVFS_LIMIT_MIN_FREQ = 212U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Configure Periodic Hardware Reset behaviour
    ROGUE_FWIF_KCCB_CMD_PHR_CFG = 213U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// Configure Safety Firmware Watchdog
    ROGUE_FWIF_KCCB_CMD_WDG_CFG = 215U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Controls counter dumping in the FW
    ROGUE_FWIF_KCCB_CMD_COUNTER_DUMP = 216U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Configure, clear and enable multiple HWPerf blocks
    ROGUE_FWIF_KCCB_CMD_HWPERF_CONFIG_ENABLE_BLKS = 217U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Configure the custom counters for HWPerf
    ROGUE_FWIF_KCCB_CMD_HWPERF_SELECT_CUSTOM_CNTRS = 218U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// Configure directly addressable counters for HWPerf
    ROGUE_FWIF_KCCB_CMD_HWPERF_CONFIG_BLKS = 220U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
}

// Kernel CCB command packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_kccb_cmd {
// Command type
    pub cmd_type: rogue_fwif_kccb_cmd_type,
// Compatibility and other flags
    pub kccb_flags: u32,
//
// NOTE: Make sure that uCmdData is the last member of this struct
// This is to calculate actual command size for device mem copy.
// (Refer ROGUEGetCmdMemCopySize())
//
// Data for Kick command
    pub cmd_kick_data: rogue_fwif_kccb_cmd_kick_data,
// Data for combined geom/frag Kick command
// Data for MMU cache command
    pub mmu_cache_data: rogue_fwif_mmucachedata,
// Data for Breakpoint Commands
    pub bp_data: rogue_fwif_bpdata,
// Data for SLC Flush/Inval commands
    pub slc_flush_inval_data: rogue_fwif_slcflushinvaldata,
// Data for cleanup commands
    pub cleanup_data: rogue_fwif_cleanup_request,
// Data for power request commands
    pub pow_data: rogue_fwif_power_request,
// Data for HWPerf control command
    pub hw_perf_ctrl: rogue_fwif_hwperf_ctrl,
//
// Data for HWPerf configure, clear and enable performance
// counter block command
//
// Data for HWPerf enable or disable performance counter block
// commands
//
    pub hw_perf_ctrl_blks: rogue_fwif_hwperf_ctrl_blks,
// Data for HWPerf configure the custom counters to read
// Data for HWPerf configure Directly Addressable blocks
    pub hw_perf_cfg_da_blks: rogue_fwif_hwperf_config_da_blks,
// Data for core clock speed change
// Feedback for Z/S Buffer backing/unbacking
    pub zs_buffer_backing_data: rogue_fwif_zsbuffer_backing_data,
// Feedback for Freelist grow/shrink
    pub free_list_gs_data: rogue_fwif_freelist_gs_data,
// Feedback for Freelists reconstruction
// Data for custom register configuration
    pub reg_config_data: rogue_fwif_regconfig_data,
// Data for informing the FW about the write offset update
// Data for setting the max frequency/OPP
    pub pdvfs_max_freq_data: rogue_fwif_pdvfs_max_freq_data,
// Data for setting the min frequency/OPP
    pub pdvfs_min_freq_data: rogue_fwif_pdvfs_min_freq_data,
// Data for updating the Guest Online states
    pub cmd_os_online_state_data: rogue_fwif_os_state_change_data,
// Dev address for TBI buffer allocated on demand
    pub tbi_buffer_fw_addr: u32,
// Data for dumping of register ranges
    pub counter_dump_config_data: rogue_fwif_counter_dump_data,
// Data for signalling all unmet fences for a given CCB
    pub force_update_data: rogue_fwif_kccb_cmd_force_update_data,
    pub __aligned(8): } cmd_data,
    pub __aligned(8): },
    pub rogue_fwif_kccb_cmd): PVR_FW_STRUCT_SIZE_ASSERT(struct,
//
// Firmware CCB command structure for ROGUE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd_zsbuffer_backing_data {
    pub zs_buffer_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd_freelist_gs_data {
    pub freelist_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd_freelists_reconstruction_data {
    pub freelist_count: u32,
    pub hwr_counter: u32,
    pub freelist_ids: [u32; ROGUE_FWIF_MAX_FREELISTS_TO_RECONSTRUCT],
}

// 1 if a page fault happened

// 1 if applicable to all contexts

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd_context_reset_data {
// Context affected by the reset
    pub server_common_context_id: u32,
//
// Reason for reset
// The valid values for reset_reason are the ones from
// enum rogue_context_reset_reason
//
    pub reset_reason: u32,
// Data Master affected by the reset
    pub dm: u32,
// Job ref running at the time of reset
    pub reset_job_ref: u32,
// ROGUE_FWIF_FWCCB_CMD_CONTEXT_RESET_FLAG bitfield
    pub flags: u32,
// At what page catalog address
    pub pc_address: aligned_u64,
// Page fault address (only when applicable)
    pub fault_address: aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd_fw_pagefault_data {
// Page fault address
    pub fw_fault_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_fwccb_cmd_type {
// Requests ZSBuffer to be backed with physical pages
    ROGUE_FWIF_FWCCB_CMD_ZSBUFFER_BACKING = 101U |
    ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Requests ZSBuffer to be unbacked
    ROGUE_FWIF_FWCCB_CMD_ZSBUFFER_UNBACKING = 102U |
    ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Requests an on-demand freelist grow/shrink
    ROGUE_FWIF_FWCCB_CMD_FREELIST_GROW = 103U |
    ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Requests freelists reconstruction
    ROGUE_FWIF_FWCCB_CMD_FREELISTS_RECONSTRUCTION =
    104U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Notifies host of a HWR event on a context
    ROGUE_FWIF_FWCCB_CMD_CONTEXT_RESET_NOTIFICATION =
    105U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Requests an on-demand debug dump
    ROGUE_FWIF_FWCCB_CMD_DEBUG_DUMP = 106U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
// Requests an on-demand update on process stats
    ROGUE_FWIF_FWCCB_CMD_UPDATE_STATS = 107U |
    ROGUE_CMD_MAGIC_DWORD_SHIFTED,

    ROGUE_FWIF_FWCCB_CMD_CORE_CLK_RATE_CHANGE =
    108U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
    ROGUE_FWIF_FWCCB_CMD_REQUEST_GPU_RESTART =
    109U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,

// Notifies host of a FW pagefault
    ROGUE_FWIF_FWCCB_CMD_CONTEXT_FW_PF_NOTIFICATION =
    112U | ROGUE_CMD_MAGIC_DWORD_SHIFTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_fwccb_cmd_update_stats_type {
//
// PVRSRVStatsUpdateRenderContextStats should increase the value of the
// ui32TotalNumPartialRenders stat
//
    ROGUE_FWIF_FWCCB_CMD_UPDATE_NUM_PARTIAL_RENDERS = 1,
//
// PVRSRVStatsUpdateRenderContextStats should increase the value of the
// ui32TotalNumOutOfMemory stat
//
    ROGUE_FWIF_FWCCB_CMD_UPDATE_NUM_OUT_OF_MEMORY,
//
// PVRSRVStatsUpdateRenderContextStats should increase the value of the
// ui32NumGeomStores stat
//
    ROGUE_FWIF_FWCCB_CMD_UPDATE_NUM_GEOM_STORES,
//
// PVRSRVStatsUpdateRenderContextStats should increase the value of the
// ui32NumFragStores stat
//
    ROGUE_FWIF_FWCCB_CMD_UPDATE_NUM_FRAG_STORES,
//
// PVRSRVStatsUpdateRenderContextStats should increase the value of the
// ui32NumCDMStores stat
//
    ROGUE_FWIF_FWCCB_CMD_UPDATE_NUM_CDM_STORES,
//
// PVRSRVStatsUpdateRenderContextStats should increase the value of the
// ui32NumTDMStores stat
//
    ROGUE_FWIF_FWCCB_CMD_UPDATE_NUM_TDM_STORES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd_update_stats_data {
// Element to update
    pub element_to_update: rogue_fwif_fwccb_cmd_update_stats_type,
// The pid of the process whose stats are being updated
    pub pid_owner: u32,
// Adjustment to be made to the statistic
    pub adjustment_value: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd_core_clk_rate_change_data {
    pub core_clk_rate: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_fwccb_cmd {
// Command type
    pub cmd_type: rogue_fwif_fwccb_cmd_type,
// Compatibility and other flags
    pub fwccb_flags: u32,
// Data for Z/S-Buffer on-demand (un)backing
// Data for on-demand freelist grow/shrink
    pub cmd_free_list_gs: rogue_fwif_fwccb_cmd_freelist_gs_data,
// Data for freelists reconstruction
// Data for context reset notification
// Data for updating process stats
    pub cmd_fw_pagefault: rogue_fwif_fwccb_cmd_fw_pagefault_data,
    pub __aligned(8): } cmd_data,
    pub __aligned(8): },
    pub rogue_fwif_fwccb_cmd): PVR_FW_STRUCT_SIZE_ASSERT(struct,
//
// Workload estimation Firmware CCB command structure for ROGUE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_workest_fwccb_cmd {
// Index for return data array
    pub return_data_index: u16,
// The cycles the workload took on the hardware
    pub cycles_taken: u32,
}

//
// Client CCB commands for ROGUE
//
// Required memory alignment for 64-bit variables accessible by Meta
// (The gcc meta aligns 64-bit variables to 64-bit; therefore, memory shared
// between the host and meta that contains 64-bit variables has to maintain
// this alignment)
//

// Leave a gap between CCB specific commands and generic commands

//
// Pre and Post timestamp commands are supposed to sandwich the DM cmd. The
// padding code with the CCB wrap upsets the FW if we don't have the task type
// bit cleared for POST_TIMESTAMPs. That's why we have 2 different cmd types.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_workest_kick_data {
// Index for the KM Workload estimation return data array
    pub __aligned(8): u16 return_data_index,
// Predicted time taken to do the work in cycles
    pub __aligned(8): u32 cycles_prediction,
// Deadline for the workload
    pub deadline: aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_ccb_cmd_header {
    pub cmd_type: u32,
    pub cmd_size: u32,
//
// external job reference - provided by client and used in debug for
// tracking submitted work
//
    pub ext_job_ref: u32,
//
// internal job reference - generated by services and used in debug for
// tracking submitted work
//
    pub int_job_ref: u32,
// Workload Estimation - Workload Estimation Data
    pub __aligned(8): rogue_fwif_workest_kick_data work_est_kick_data,
}

//
// Client CCB commands which are only required by the kernel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_cmd_priority {
    pub priority: i32,
}

//
// Signature and Checksums Buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_sigbuf_ctl {
// Ptr to Signature Buffer memory
    pub buffer_fw_addr: u32,
// Amount of space left for storing regs in the buffer
    pub left_size_in_regs: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_counter_dump_ctl {
// Ptr to counter dump buffer
    pub buffer_fw_addr: u32,
// Amount of space for storing in the buffer
    pub size_in_dwords: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_firmware_gcov_ctl {
// Ptr to firmware gcov buffer
    pub buffer_fw_addr: u32,
// Amount of space for storing in the buffer
    pub size: u32,
    pub __aligned(8): },
//
// ROGUE Compatibility checks
//
// WARNING: Whenever the layout of ROGUE_FWIF_COMPCHECKS_BVNC changes, the
// following define should be increased by 1 to indicate to the compatibility
// logic that layout has changed.
//
pub const ROGUE_FWIF_COMPCHECKS_LAYOUT_VERSION: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_compchecks_bvnc {
// WARNING: This field must be defined as first one in this structure
    pub layout_version: u32,
    pub bvnc: aligned_u64,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_init_options {
    pub os_count_support: u8,
    pub padding: [u8; 7],
    pub __aligned(8): },

    pub ROGUE_FWIF_COMPCHECKS_LAYOUT_VERSION: compchecks->layout_version =,
    pub 0: compchecks->bvnc =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_compchecks {
// hardware BVNC (from the ROGUE registers)
    pub hw_bvnc: rogue_fwif_compchecks_bvnc,
// firmware BVNC
    pub fw_bvnc: rogue_fwif_compchecks_bvnc,
// identifier of the FW processor version
    pub fw_processor_version: u32,
// software DDK version
    pub ddk_version: u32,
// software DDK build no.
    pub ddk_build: u32,
// build options bit-field
    pub build_options: u32,
// initialisation options bit-field
    pub init_options: rogue_fwif_init_options,
// Information is valid
    pub __aligned(4): bool updated,
    pub padding: u32,
    pub __aligned(8): },
//
// Updated configuration post FW data init.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_runtime_cfg {
// APM latency in ms before signalling IDLE to the host
    pub active_pm_latency_ms: u32,
// Compatibility and other flags
    pub runtime_cfg_flags: u32,
//
// If set, APM latency does not reset to system default each GPU power
// transition
//
    pub __aligned(4): bool active_pm_latency_persistant,
// Core clock speed, currently only used to calculate timer ticks
    pub core_clock_speed: u32,
// Last number of dusts change requested by the host
    pub default_dusts_num_init: u32,
// Periodic Hardware Reset configuration values
    pub phr_mode: u32,
// New number of milliseconds C/S is allowed to last
    pub hcs_deadline_ms: u32,
// The watchdog period in microseconds
    pub wdg_period_us: u32,
// Array of priorities per OS
    pub osid_priority: [u32; ROGUE_FW_MAX_NUM_OS],
// On-demand allocated HWPerf buffer address, to be passed to the FW
    pub hwperf_buf_fw_addr: u32,
    pub __aligned(4): bool padding,
}

//
// Control data for ROGUE
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_tpu_dm {
    ROGUE_FWIF_TPU_DM_PDM = 0,
    ROGUE_FWIF_TPU_DM_VDM = 1,
    ROGUE_FWIF_TPU_DM_CDM = 2,
    ROGUE_FWIF_TPU_DM_TDM = 3,
    ROGUE_FWIF_TPU_DM_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_gpio_val_mode {
// No GPIO validation
    ROGUE_FWIF_GPIO_VAL_OFF = 0,
//
// Simple test case that initiates by sending data via the GPIO and then
// sends back any data received over the GPIO
//
    ROGUE_FWIF_GPIO_VAL_GENERAL = 1,
//
// More complex test case that writes and reads data across the entire
// GPIO AP address range.
//
    ROGUE_FWIF_GPIO_VAL_AP = 2,
// Validates the GPIO Testbench.
    ROGUE_FWIF_GPIO_VAL_TESTBENCH = 5,
// Send and then receive each byte in the range 0-255.
    ROGUE_FWIF_GPIO_VAL_LOOPBACK = 6,
// Send and then receive each power-of-2 byte in the range 0-255.
    ROGUE_FWIF_GPIO_VAL_LOOPBACK_LITE = 7,
    ROGUE_FWIF_GPIO_VAL_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_perf_conf {
    FW_PERF_CONF_NONE = 0,
    FW_PERF_CONF_ICACHE = 1,
    FW_PERF_CONF_DCACHE = 2,
    FW_PERF_CONF_JTLB_INSTR = 5,
    FW_PERF_CONF_INSTRUCTIONS = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_boot_stage {
    FW_BOOT_STAGE_TLB_INIT_FAILURE = -2,
    FW_BOOT_STAGE_NOT_AVAILABLE = -1,
    FW_BOOT_NOT_STARTED = 0,
    FW_BOOT_BLDR_STARTED = 1,
    FW_BOOT_CACHE_DONE,
    FW_BOOT_TLB_DONE,
    FW_BOOT_MAIN_STARTED,
    FW_BOOT_ALIGNCHECKS_DONE,
    FW_BOOT_INIT_DONE,
}

//
// Kernel CCB return slot responses. Usage of bit-fields instead of bare
// integers allows FW to possibly pack-in several responses for each single kCCB
// command.
//
// Command executed (return status from FW)

// A cleanup was requested but resource busy

// Poll failed in FW for a HW operation to complete

// Reset value of a kCCB return slot (set by host)
pub const ROGUE_FWIF_KCCB_RTN_SLOT_NO_RESPONSE: c_uint = 0x0U;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_connection_ctl {
// Fw-Os connection states
    pub connection_fw_state: rogue_fwif_connection_fw_state,
    pub connection_os_state: rogue_fwif_connection_os_state,
    pub alive_fw_token: u32,
    pub alive_os_token: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_osinit {
// Kernel CCB
    pub kernel_ccbctl_fw_addr: u32,
    pub kernel_ccb_fw_addr: u32,
    pub kernel_ccb_rtn_slots_fw_addr: u32,
// Firmware CCB
    pub firmware_ccbctl_fw_addr: u32,
    pub firmware_ccb_fw_addr: u32,
// Workload Estimation Firmware CCB
    pub work_est_firmware_ccbctl_fw_addr: u32,
    pub work_est_firmware_ccb_fw_addr: u32,
    pub rogue_fwif_hwr_info_buf_ctl_fw_addr: u32,
    pub hwr_debug_dump_limit: u32,
    pub fw_os_data_fw_addr: u32,
// Compatibility checks to be populated by the Firmware
    pub rogue_comp_checks: rogue_fwif_compchecks,
    pub __aligned(8): },
// BVNC Features
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_hwperf_bvnc_block {
// Counter block ID, see ROGUE_HWPERF_CNTBLK_ID
    pub block_id: u16,
// Number of counters in this block type
    pub num_counters: u16,
// Number of blocks of this type
    pub num_blocks: u16,
    pub reserved: u16,
}

// BVNC Features
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_hwperf_bvnc {
// BVNC string
    pub bvnc_string: [c_char; ROGUE_HWPERF_MAX_BVNC_LEN],
// See ROGUE_HWPERF_FEATURE_FLAGS
    pub bvnc_km_feature_flags: u32,
// Number of blocks described in aBvncBlocks
    pub num_bvnc_blocks: u16,
// Number of GPU cores present
    pub bvnc_gpu_cores: u16,
// Supported Performance Blocks for BVNC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_sysinit {
// Fault read address
    pub fault_phys_addr: aligned_u64,
// PDS execution base
    pub pds_exec_base: aligned_u64,
// UCS execution base
    pub usc_exec_base: aligned_u64,
// FBCDC bindless texture state table base
    pub fbcdc_state_table_base: aligned_u64,
    pub fbcdc_large_state_table_base: aligned_u64,
// Texture state base
    pub texture_heap_base: aligned_u64,
// Event filter for Firmware events
    pub hw_perf_filter: u64,
    pub slc3_fence_dev_addr: aligned_u64,
    pub __aligned(8): u32 tpu_trilinear_frac_mask[ROGUE_FWIF_TPU_DM_LAST],
// Signature and Checksum Buffers for DMs
    pub sigbuf_ctl: [rogue_fwif_sigbuf_ctl; PVR_FWIF_DM_MAX],
    pub pdvfs_opp_info: rogue_fwif_pdvfs_opp,
    pub coremem_data_store: rogue_fwif_dma_addr,
    pub counter_dump_ctl: rogue_fwif_counter_dump_ctl,
    pub filter_flags: u32,
    pub runtime_cfg_fw_addr: u32,
    pub trace_buf_ctl_fw_addr: u32,
    pub fw_sys_data_fw_addr: u32,
    pub gpu_util_fw_cb_ctl_fw_addr: u32,
    pub reg_cfg_fw_addr: u32,
    pub hwperf_ctl_fw_addr: u32,
    pub align_checks: u32,
// Core clock speed at FW boot time
    pub initial_core_clock_speed: u32,
// APM latency in ms before signalling IDLE to the host
    pub active_pm_latency_ms: u32,
// Flag to be set by the Firmware after successful start
    pub __aligned(4): bool firmware_started,
// Host/FW Trace synchronisation Partition Marker
    pub marker_val: u32,
// Firmware initialization complete time
    pub firmware_started_timestamp: u32,
    pub jones_disable_mask: u32,
// Firmware performance counter config
    pub firmware_perf: fw_perf_conf,
//
// FW Pointer to memory containing core clock rate in Hz.
// Firmware (PDVFS) updates the memory when running on non primary FW
// thread to communicate to host driver.
//
    pub core_clock_rate_fw_addr: u32,
    pub gpio_validation_mode: rogue_fwif_gpio_val_mode,
// Used in HWPerf for decoding BVNC Features
    pub bvnc_km_feature_flags: rogue_hwperf_bvnc,
// Value to write into ROGUE_CR_TFBC_COMPRESSION_CONTROL
    pub tfbc_compression_control: u32,
    pub __aligned(8): },
//
// Timer correlation shared data and defines
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_time_corr {
    pub os_timestamp: aligned_u64,
    pub os_mono_timestamp: aligned_u64,
    pub cr_timestamp: aligned_u64,
//
// Utility variable used to convert CR timer deltas to OS timer deltas
// (nS), where the deltas are relative to the timestamps above:
// deltaOS = (deltaCR * K) >> decimal_shift, see full explanation below
//
    pub cr_delta_to_os_delta_kns: aligned_u64,
    pub core_clock_speed: u32,
    pub reserved: u32,
    pub __aligned(8): },
//
// The following macros are used to help converting FW timestamps to the Host
// time domain. On the FW the ROGUE_CR_TIMER counter is used to keep track of
// time; it increments by 1 every 256 GPU clock ticks, so the general
// formula to perform the conversion is:
//
// [ GPU clock speed in Hz, if (scale == 10^9) then deltaOS is in nS,
// otherwise if (scale == 10^6) then deltaOS is in uS ]
//
// deltaCR * 256                                   256 * scale
// deltaOS = --------------- * scale = deltaCR * K    [ K = --------------- ]
// GPUclockspeed                                  GPUclockspeed
//
// The actual K is multiplied by 2^20 (and deltaCR * K is divided by 2^20)
// to get some better accuracy and to avoid returning 0 in the integer
// division 256000000/GPUfreq if GPUfreq is greater than 256MHz.
// This is the same as keeping K as a decimal number.
//
// The maximum deltaOS is slightly more than 5hrs for all GPU frequencies
// (deltaCR * K is more or less a constant), and it's relative to the base
// OS timestamp sampled as a part of the timer correlation data.
// This base is refreshed on GPU power-on, DVFS transition and periodic
// frequency calibration (executed every few seconds if the FW is doing
// some work), so as long as the GPU is doing something and one of these
// events is triggered then deltaCR * K will not overflow and deltaOS will be
// correct.
//

//
// GPU Utilisation
//
// See rogue_common.h for a list of GPU states

//
// The OS timestamps computed by the FW are approximations of the real time,
// which means they could be slightly behind or ahead the real timer on the
// Host. In some cases we can perform subtractions between FW approximated
// timestamps and real OS timestamps, so we need a form of protection against
// negative results if for instance the FW one is a bit ahead of time.
//

//
// The timer correlation array must be big enough to ensure old entries won't be
// overwritten before all the HWPerf events linked to those entries are
// processed by the MISR. The update frequency of this array depends on how fast
// the system can change state (basically how small the APM latency is) and
// perform DVFS transitions.
//
// The minimum size is 2 (not 1) to avoid race conditions between the FW reading
// an entry while the Host is updating it. With 2 entries in the worst case the
// FW will read old data, which is still quite ok if the Host is updating the
// timer correlation at that time.
//

// Make sure the timer correlation array size is a power of 2
    pub two"): "ROGUE_FWIF_TIME_CORR_ARRAY_SIZE must be a power of,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_gpu_util_fwcb {
    pub time_corr: [rogue_fwif_time_corr; ROGUE_FWIF_TIME_CORR_ARRAY_SIZE],
    pub time_corr_seq_count: u32,
// Compatibility and other flags
    pub gpu_util_flags: u32,
// Last GPU state + OS time of the last state update
    pub last_word: aligned_u64,
// Counters for the amount of time the GPU was active/idle/blocked
    pub stats_counters: [aligned_u64; PVR_FWIF_GPU_UTIL_STATE_NUM],
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_rta_ctl {
// Render number
    pub render_target_index: u32,
// index in RTA
    pub current_render_target: u32,
// total active RTs
    pub active_render_targets: u32,
// total active RTs from the first TA kick, for OOM
    pub cumul_active_render_targets: u32,
// Array of valid RT indices
    pub valid_render_targets_fw_addr: u32,
// Array of number of occurred partial renders per render target
    pub rta_num_partial_renders_fw_addr: u32,
// Number of render targets in the array
    pub max_rts: u32,
// Compatibility and other flags
    pub rta_ctl_flags: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_freelist {
    pub freelist_dev_addr: aligned_u64,
    pub current_dev_addr: aligned_u64,
    pub current_stack_top: u32,
    pub max_pages: u32,
    pub grow_pages: u32,
// HW pages
    pub current_pages: u32,
    pub allocated_page_count: u32,
    pub allocated_mmu_page_count: u32,
    pub freelist_id: u32,
    pub __aligned(4): bool grow_pending,
// Pages that should be used only when OOM is reached
    pub ready_pages: u32,
// Compatibility and other flags
    pub freelist_flags: u32,
// PM Global PB on which Freelist is loaded
    pub pm_global_pb: u32,
    pub padding: u32,
    pub __aligned(8): },
//
// HWRTData
//
// HWRTData flags
// Deprecated flags 1:0

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_fwif_rtdata_state {
    ROGUE_FWIF_RTDATA_STATE_NONE = 0,
    ROGUE_FWIF_RTDATA_STATE_KICK_GEOM,
    ROGUE_FWIF_RTDATA_STATE_KICK_GEOM_FIRST,
    ROGUE_FWIF_RTDATA_STATE_GEOM_FINISHED,
    ROGUE_FWIF_RTDATA_STATE_KICK_FRAG,
    ROGUE_FWIF_RTDATA_STATE_FRAG_FINISHED,
    ROGUE_FWIF_RTDATA_STATE_FRAG_CONTEXT_STORED,
    ROGUE_FWIF_RTDATA_STATE_GEOM_OUTOFMEM,
    ROGUE_FWIF_RTDATA_STATE_PARTIALRENDERFINISHED,
//
// In case of HWR, we can't set the RTDATA state to NONE, as this will
// cause any TA to become a first TA. To ensure all related TA's are
// skipped, we use the HWR state
//
    ROGUE_FWIF_RTDATA_STATE_HWR,
    ROGUE_FWIF_RTDATA_STATE_UNKNOWN = 0x7FFFFFFFU
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwrtdata_common {
    pub __aligned(4): bool geom_caches_need_zeroing,
    pub screen_pixel_max: u32,
    pub multi_sample_ctl: aligned_u64,
    pub flipped_multi_sample_ctl: u64,
    pub tpc_stride: u32,
    pub tpc_size: u32,
    pub te_screen: u32,
    pub mtile_stride: u32,
    pub teaa: u32,
    pub te_mtile1: u32,
    pub te_mtile2: u32,
    pub isp_merge_lower_x: u32,
    pub isp_merge_lower_y: u32,
    pub isp_merge_upper_x: u32,
    pub isp_merge_upper_y: u32,
    pub isp_merge_scale_x: u32,
    pub isp_merge_scale_y: u32,
    pub rgn_header_size: u32,
    pub isp_mtile_size: u32,
    pub padding: u32,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_fwif_hwrtdata {
// MList Data Store
    pub pm_mlist_dev_addr: aligned_u64,
    pub vce_cat_base: [aligned_u64; 4],
    pub vce_last_cat_base: [aligned_u64; 4],
    pub te_cat_base: [aligned_u64; 4],
    pub te_last_cat_base: [aligned_u64; 4],
    pub alist_cat_base: aligned_u64,
    pub alist_last_cat_base: aligned_u64,
    pub pm_alist_stack_pointer: aligned_u64,
    pub pm_mlist_stack_pointer: u32,
    pub hwrt_data_common_fw_addr: u32,
    pub hwrt_data_flags: u32,
    pub state: rogue_fwif_rtdata_state,
    pub __aligned(8): u32 freelists_fw_addr[MAX_FREELISTS_SIZE],
    pub freelist_hwr_snapshot: [u32; MAX_FREELISTS_SIZE],
    pub vheap_table_dev_addr: aligned_u64,
    pub rta_ctl: rogue_fwif_rta_ctl,
    pub tail_ptrs_dev_addr: aligned_u64,
    pub macrotile_array_dev_addr: aligned_u64,
    pub rgn_header_dev_addr: aligned_u64,
    pub rtc_dev_addr: aligned_u64,
    pub __aligned(8): u32 owner_geom_not_used_by_host,
    pub __aligned(4): bool geom_caches_need_zeroing,
    pub __aligned(64): rogue_fwif_cleanup_ctl cleanup_state,
    pub __aligned(8): },
//
// Sync checkpoints
//
pub const PVR_SYNC_CHECKPOINT_UNDEF: c_uint = 0x000;
pub const PVR_SYNC_CHECKPOINT_ACTIVE: c_uint = 0xac1     /* Checkpoint has not signaled. */;
pub const PVR_SYNC_CHECKPOINT_SIGNALED: c_uint = 0x519   /* Checkpoint has signaled. */;
pub const PVR_SYNC_CHECKPOINT_ERRORED: c_uint = 0xeff    /* Checkpoint has been errored. */;

