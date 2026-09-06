//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/sbi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2015 Regents of the University of California
// Copyright (c) 2020 Western Digital Corporation or its affiliates.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_id {

    SBI_EXT_0_1_SET_TIMER = 0x0,
    SBI_EXT_0_1_CONSOLE_PUTCHAR = 0x1,
    SBI_EXT_0_1_CONSOLE_GETCHAR = 0x2,
    SBI_EXT_0_1_CLEAR_IPI = 0x3,
    SBI_EXT_0_1_SEND_IPI = 0x4,
    SBI_EXT_0_1_REMOTE_FENCE_I = 0x5,
    SBI_EXT_0_1_REMOTE_SFENCE_VMA = 0x6,
    SBI_EXT_0_1_REMOTE_SFENCE_VMA_ASID = 0x7,
    SBI_EXT_0_1_SHUTDOWN = 0x8,

    SBI_EXT_BASE = 0x10,
    SBI_EXT_TIME = 0x54494D45,
    SBI_EXT_IPI = 0x735049,
    SBI_EXT_RFENCE = 0x52464E43,
    SBI_EXT_HSM = 0x48534D,
    SBI_EXT_SRST = 0x53525354,
    SBI_EXT_SUSP = 0x53555350,
    SBI_EXT_PMU = 0x504D55,
    SBI_EXT_DBCN = 0x4442434E,
    SBI_EXT_STA = 0x535441,
    SBI_EXT_NACL = 0x4E41434C,
    SBI_EXT_FWFT = 0x46574654,
    SBI_EXT_MPXY = 0x4D505859,
    SBI_EXT_DBTR = 0x44425452,

// Experimentals extensions must lie within this range
    SBI_EXT_EXPERIMENTAL_START = 0x08000000,
    SBI_EXT_EXPERIMENTAL_END = 0x08FFFFFF,

// Vendor extensions must lie within this range
    SBI_EXT_VENDOR_START = 0x09000000,
    SBI_EXT_VENDOR_END = 0x09FFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_base_fid {
    SBI_EXT_BASE_GET_SPEC_VERSION = 0,
    SBI_EXT_BASE_GET_IMP_ID,
    SBI_EXT_BASE_GET_IMP_VERSION,
    SBI_EXT_BASE_PROBE_EXT,
    SBI_EXT_BASE_GET_MVENDORID,
    SBI_EXT_BASE_GET_MARCHID,
    SBI_EXT_BASE_GET_MIMPID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_time_fid {
    SBI_EXT_TIME_SET_TIMER = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_ipi_fid {
    SBI_EXT_IPI_SEND_IPI = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_rfence_fid {
    SBI_EXT_RFENCE_REMOTE_FENCE_I = 0,
    SBI_EXT_RFENCE_REMOTE_SFENCE_VMA,
    SBI_EXT_RFENCE_REMOTE_SFENCE_VMA_ASID,
    SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA_VMID,
    SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA,
    SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA_ASID,
    SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_hsm_fid {
    SBI_EXT_HSM_HART_START = 0,
    SBI_EXT_HSM_HART_STOP,
    SBI_EXT_HSM_HART_STATUS,
    SBI_EXT_HSM_HART_SUSPEND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_hsm_hart_state {
    SBI_HSM_STATE_STARTED = 0,
    SBI_HSM_STATE_STOPPED,
    SBI_HSM_STATE_START_PENDING,
    SBI_HSM_STATE_STOP_PENDING,
    SBI_HSM_STATE_SUSPENDED,
    SBI_HSM_STATE_SUSPEND_PENDING,
    SBI_HSM_STATE_RESUME_PENDING,
}

pub const SBI_HSM_SUSP_BASE_MASK: c_uint = 0x7fffffff;
pub const SBI_HSM_SUSP_NON_RET_BIT: c_uint = 0x80000000;
pub const SBI_HSM_SUSP_PLAT_BASE: c_uint = 0x10000000;
pub const SBI_HSM_SUSPEND_RET_DEFAULT: c_uint = 0x00000000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_srst_fid {
    SBI_EXT_SRST_RESET = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_srst_reset_type {
    SBI_SRST_RESET_TYPE_SHUTDOWN = 0,
    SBI_SRST_RESET_TYPE_COLD_REBOOT,
    SBI_SRST_RESET_TYPE_WARM_REBOOT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_srst_reset_reason {
    SBI_SRST_RESET_REASON_NONE = 0,
    SBI_SRST_RESET_REASON_SYS_FAILURE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_susp_fid {
    SBI_EXT_SUSP_SYSTEM_SUSPEND = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_susp_sleep_type {
    SBI_SUSP_SLEEP_TYPE_SUSPEND_TO_RAM = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_pmu_fid {
    SBI_EXT_PMU_NUM_COUNTERS = 0,
    SBI_EXT_PMU_COUNTER_GET_INFO,
    SBI_EXT_PMU_COUNTER_CFG_MATCH,
    SBI_EXT_PMU_COUNTER_START,
    SBI_EXT_PMU_COUNTER_STOP,
    SBI_EXT_PMU_COUNTER_FW_READ,
    SBI_EXT_PMU_COUNTER_FW_READ_HI,
    SBI_EXT_PMU_SNAPSHOT_SET_SHMEM,
    SBI_EXT_PMU_EVENT_GET_INFO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sbi_pmu_ctr_info {
    pub value: c_ulong,
    pub csr:12: c_ulong,
    pub width:6: c_ulong,

    pub reserved:13: c_ulong,

    pub reserved:45: c_ulong,

    pub type:1: c_ulong,
}

// Data structure to contain the pmu snapshot data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_pmu_snapshot_data {
    pub ctr_overflow_mask: u64,
    pub ctr_values: [u64; 64],
    pub reserved: [u64; 447],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_pmu_event_info {
    pub event_idx: u32,
    pub output: u32,
    pub event_data: u64,
}

pub const RISCV_PMU_EVENT_INFO_OUTPUT_MASK: c_uint = 0x01;

// SBI v3.0 allows extended hpmeventX width value

pub const RISCV_PMU_RAW_EVENT_IDX: c_uint = 0x20000;
pub const RISCV_PMU_RAW_EVENT_V2_IDX: c_uint = 0x30000;
pub const RISCV_PLAT_FW_EVENT: c_uint = 0xFFFF;
// General pmu event codes specified in SBI PMU extension
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_pmu_hw_generic_events_t {
    SBI_PMU_HW_NO_EVENT			= 0,
    SBI_PMU_HW_CPU_CYCLES			= 1,
    SBI_PMU_HW_INSTRUCTIONS			= 2,
    SBI_PMU_HW_CACHE_REFERENCES		= 3,
    SBI_PMU_HW_CACHE_MISSES			= 4,
    SBI_PMU_HW_BRANCH_INSTRUCTIONS		= 5,
    SBI_PMU_HW_BRANCH_MISSES		= 6,
    SBI_PMU_HW_BUS_CYCLES			= 7,
    SBI_PMU_HW_STALLED_CYCLES_FRONTEND	= 8,
    SBI_PMU_HW_STALLED_CYCLES_BACKEND	= 9,
    SBI_PMU_HW_REF_CPU_CYCLES		= 10,

    SBI_PMU_HW_GENERAL_MAX,
}

//
// Special "firmware" events provided by the firmware, even if the hardware
// does not support performance events. These events are encoded as a raw
// event type in Linux kernel perf framework.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_pmu_fw_generic_events_t {
    SBI_PMU_FW_MISALIGNED_LOAD	= 0,
    SBI_PMU_FW_MISALIGNED_STORE	= 1,
    SBI_PMU_FW_ACCESS_LOAD		= 2,
    SBI_PMU_FW_ACCESS_STORE		= 3,
    SBI_PMU_FW_ILLEGAL_INSN		= 4,
    SBI_PMU_FW_SET_TIMER		= 5,
    SBI_PMU_FW_IPI_SENT		= 6,
    SBI_PMU_FW_IPI_RCVD		= 7,
    SBI_PMU_FW_FENCE_I_SENT		= 8,
    SBI_PMU_FW_FENCE_I_RCVD		= 9,
    SBI_PMU_FW_SFENCE_VMA_SENT	= 10,
    SBI_PMU_FW_SFENCE_VMA_RCVD	= 11,
    SBI_PMU_FW_SFENCE_VMA_ASID_SENT	= 12,
    SBI_PMU_FW_SFENCE_VMA_ASID_RCVD	= 13,

    SBI_PMU_FW_HFENCE_GVMA_SENT	= 14,
    SBI_PMU_FW_HFENCE_GVMA_RCVD	= 15,
    SBI_PMU_FW_HFENCE_GVMA_VMID_SENT = 16,
    SBI_PMU_FW_HFENCE_GVMA_VMID_RCVD = 17,

    SBI_PMU_FW_HFENCE_VVMA_SENT	= 18,
    SBI_PMU_FW_HFENCE_VVMA_RCVD	= 19,
    SBI_PMU_FW_HFENCE_VVMA_ASID_SENT = 20,
    SBI_PMU_FW_HFENCE_VVMA_ASID_RCVD = 21,
    SBI_PMU_FW_MAX,
}

// SBI PMU event types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_pmu_event_type {
    SBI_PMU_EVENT_TYPE_HW = 0x0,
    SBI_PMU_EVENT_TYPE_CACHE = 0x1,
    SBI_PMU_EVENT_TYPE_RAW = 0x2,
    SBI_PMU_EVENT_TYPE_RAW_V2 = 0x3,
    SBI_PMU_EVENT_TYPE_FW = 0xf,
}

// SBI PMU event types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_pmu_ctr_type {
    SBI_PMU_CTR_TYPE_HW = 0x0,
    SBI_PMU_CTR_TYPE_FW,
}

// Helper macros to decode event idx
pub const SBI_PMU_EVENT_IDX_OFFSET: c_int = 20;
pub const SBI_PMU_EVENT_IDX_MASK: c_uint = 0xFFFFF;
pub const SBI_PMU_EVENT_IDX_CODE_MASK: c_uint = 0xFFFF;
pub const SBI_PMU_EVENT_IDX_TYPE_MASK: c_uint = 0xF0000;
pub const SBI_PMU_EVENT_RAW_IDX: c_uint = 0x20000;
pub const SBI_PMU_FIXED_CTR_MASK: c_uint = 0x07;
pub const SBI_PMU_EVENT_CACHE_ID_CODE_MASK: c_uint = 0xFFF8;
pub const SBI_PMU_EVENT_CACHE_OP_ID_CODE_MASK: c_uint = 0x06;
pub const SBI_PMU_EVENT_CACHE_RESULT_ID_CODE_MASK: c_uint = 0x01;
pub const SBI_PMU_EVENT_CACHE_ID_SHIFT: c_int = 3;
pub const SBI_PMU_EVENT_CACHE_OP_SHIFT: c_int = 1;
pub const SBI_PMU_EVENT_IDX_INVALID: c_uint = 0xFFFFFFFF;
// Flags defined for config matching function

// Flags defined for counter start function

// Flags defined for counter stop function

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_dbcn_fid {
    SBI_EXT_DBCN_CONSOLE_WRITE = 0,
    SBI_EXT_DBCN_CONSOLE_READ = 1,
    SBI_EXT_DBCN_CONSOLE_WRITE_BYTE = 2,
}

// SBI STA (steal-time accounting) extension
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_sta_fid {
    SBI_EXT_STA_STEAL_TIME_SET_SHMEM = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbi_sta_struct {
    pub sequence: __le32,
    pub flags: __le32,
    pub steal: __le64,
    pub preempted: u8,
    pub pad: [u8; 47],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_nacl_fid {
    SBI_EXT_NACL_PROBE_FEATURE = 0x0,
    SBI_EXT_NACL_SET_SHMEM = 0x1,
    SBI_EXT_NACL_SYNC_CSR = 0x2,
    SBI_EXT_NACL_SYNC_HFENCE = 0x3,
    SBI_EXT_NACL_SYNC_SRET = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_nacl_feature {
    SBI_NACL_FEAT_SYNC_CSR = 0x0,
    SBI_NACL_FEAT_SYNC_HFENCE = 0x1,
    SBI_NACL_FEAT_SYNC_SRET = 0x2,
    SBI_NACL_FEAT_AUTOSWAP_CSR = 0x3,
}

pub const SBI_NACL_SHMEM_ADDR_SHIFT: c_int = 12;
pub const SBI_NACL_SHMEM_SCRATCH_OFFSET: c_uint = 0x0000;
pub const SBI_NACL_SHMEM_SCRATCH_SIZE: c_uint = 0x1000;
pub const SBI_NACL_SHMEM_SRET_OFFSET: c_uint = 0x0000;
pub const SBI_NACL_SHMEM_SRET_SIZE: c_uint = 0x0200;

pub const SBI_NACL_SHMEM_AUTOSWAP_SIZE: c_uint = 0x0080;

pub const SBI_NACL_SHMEM_UNUSED_SIZE: c_uint = 0x0580;

pub const SBI_NACL_SHMEM_HFENCE_SIZE: c_uint = 0x0780;

pub const SBI_NACL_SHMEM_DBITMAP_SIZE: c_uint = 0x0080;

// Macro flag: #define SBI_NACL_SHMEM_HFENCE_ENTRY_PNUM(__num)\
// Macro flag: #define SBI_NACL_SHMEM_HFENCE_ENTRY_PCOUNT(__num)\
pub const SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_BITS: c_int = 1;

pub const SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD1_BITS: c_int = 3;

pub const SBI_NACL_SHMEM_HFENCE_CONFIG_TYPE_BITS: c_int = 4;

pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA: c_uint = 0x0;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_ALL: c_uint = 0x1;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_VMID: c_uint = 0x2;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_VMID_ALL: c_uint = 0x3;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA: c_uint = 0x4;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ALL: c_uint = 0x5;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ASID: c_uint = 0x6;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ASID_ALL: c_uint = 0x7;
pub const SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD2_BITS: c_int = 1;

pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ORDER_BITS: c_int = 7;

pub const SBI_NACL_SHMEM_HFENCE_ORDER_BASE: c_int = 12;

pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ASID_BITS: c_int = 9;
pub const SBI_NACL_SHMEM_HFENCE_CONFIG_VMID_BITS: c_int = 7;

pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ASID_BITS: c_int = 16;
pub const SBI_NACL_SHMEM_HFENCE_CONFIG_VMID_BITS: c_int = 14;

pub const SBI_NACL_SHMEM_SRET_X_LAST: c_int = 31;
// SBI function IDs for FW feature extension
pub const SBI_EXT_FWFT_SET: c_uint = 0x0;
pub const SBI_EXT_FWFT_GET: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_fwft_feature_t {
    SBI_FWFT_MISALIGNED_EXC_DELEG		= 0x0,
    SBI_FWFT_LANDING_PAD			= 0x1,
    SBI_FWFT_SHADOW_STACK			= 0x2,
    SBI_FWFT_DOUBLE_TRAP			= 0x3,
    SBI_FWFT_PTE_AD_HW_UPDATING		= 0x4,
    SBI_FWFT_POINTER_MASKING_PMLEN		= 0x5,
    SBI_FWFT_LOCAL_RESERVED_START		= 0x6,
    SBI_FWFT_LOCAL_RESERVED_END		= 0x3fffffff,
    SBI_FWFT_LOCAL_PLATFORM_START		= 0x40000000,
    SBI_FWFT_LOCAL_PLATFORM_END		= 0x7fffffff,

    SBI_FWFT_GLOBAL_RESERVED_START		= 0x80000000,
    SBI_FWFT_GLOBAL_RESERVED_END		= 0xbfffffff,
    SBI_FWFT_GLOBAL_PLATFORM_START		= 0xc0000000,
    SBI_FWFT_GLOBAL_PLATFORM_END		= 0xffffffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_mpxy_fid {
    SBI_EXT_MPXY_GET_SHMEM_SIZE,
    SBI_EXT_MPXY_SET_SHMEM,
    SBI_EXT_MPXY_GET_CHANNEL_IDS,
    SBI_EXT_MPXY_READ_ATTRS,
    SBI_EXT_MPXY_WRITE_ATTRS,
    SBI_EXT_MPXY_SEND_MSG_WITH_RESP,
    SBI_EXT_MPXY_SEND_MSG_WITHOUT_RESP,
    SBI_EXT_MPXY_GET_NOTIFICATION_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_mpxy_attribute_id {
// Standard channel attributes managed by MPXY framework
    SBI_MPXY_ATTR_MSG_PROT_ID		= 0x00000000,
    SBI_MPXY_ATTR_MSG_PROT_VER		= 0x00000001,
    SBI_MPXY_ATTR_MSG_MAX_LEN		= 0x00000002,
    SBI_MPXY_ATTR_MSG_SEND_TIMEOUT		= 0x00000003,
    SBI_MPXY_ATTR_MSG_COMPLETION_TIMEOUT	= 0x00000004,
    SBI_MPXY_ATTR_CHANNEL_CAPABILITY	= 0x00000005,
    SBI_MPXY_ATTR_SSE_EVENT_ID		= 0x00000006,
    SBI_MPXY_ATTR_MSI_CONTROL		= 0x00000007,
    SBI_MPXY_ATTR_MSI_ADDR_LO		= 0x00000008,
    SBI_MPXY_ATTR_MSI_ADDR_HI		= 0x00000009,
    SBI_MPXY_ATTR_MSI_DATA			= 0x0000000A,
    SBI_MPXY_ATTR_EVENTS_STATE_CONTROL	= 0x0000000B,
    SBI_MPXY_ATTR_STD_ATTR_MAX_IDX,
//
// Message protocol specific attributes, managed by
// the message protocol specification.
//
    SBI_MPXY_ATTR_MSGPROTO_ATTR_START	= 0x80000000,
    SBI_MPXY_ATTR_MSGPROTO_ATTR_END		= 0xffffffff
}

// Possible values of MSG_PROT_ID attribute as-per SBI v3.0 (or higher)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_mpxy_msgproto_id {
    SBI_MPXY_MSGPROTO_RPMI_ID = 0x0,
}

// RPMI message protocol specific MPXY attributes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_mpxy_rpmi_attribute_id {
    SBI_MPXY_RPMI_ATTR_SERVICEGROUP_ID = SBI_MPXY_ATTR_MSGPROTO_ATTR_START,
    SBI_MPXY_RPMI_ATTR_SERVICEGROUP_VERSION,
    SBI_MPXY_RPMI_ATTR_IMPL_ID,
    SBI_MPXY_RPMI_ATTR_IMPL_VERSION,
    SBI_MPXY_RPMI_ATTR_MAX_ID
}

// Encoding of MSG_PROT_VER attribute

// Capabilities available through CHANNEL_CAPABILITY attribute

// SBI debug triggers function IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbi_ext_dbtr_fid {
    SBI_EXT_DBTR_NUM_TRIGGERS = 0,
    SBI_EXT_DBTR_SETUP_SHMEM,
    SBI_EXT_DBTR_TRIG_READ,
    SBI_EXT_DBTR_TRIG_INSTALL,
    SBI_EXT_DBTR_TRIG_UPDATE,
    SBI_EXT_DBTR_TRIG_UNINSTALL,
    SBI_EXT_DBTR_TRIG_ENABLE,
    SBI_EXT_DBTR_TRIG_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbi_dbtr_data_msg {
    pub tstate: c_ulong,
    pub tdata1: c_ulong,
    pub tdata2: c_ulong,
    pub tdata3: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbi_dbtr_id_msg {
    pub idx: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sbi_dbtr_shmem_entry {
    pub data: sbi_dbtr_data_msg,
    pub id: sbi_dbtr_id_msg,
}

// SBI spec version fields
pub const SBI_SPEC_VERSION_DEFAULT: c_uint = 0x1;
pub const SBI_SPEC_VERSION_MAJOR_SHIFT: c_int = 24;
pub const SBI_SPEC_VERSION_MAJOR_MASK: c_uint = 0x7f;
pub const SBI_SPEC_VERSION_MINOR_MASK: c_uint = 0xffffff;
// SBI return error codes
pub const SBI_SUCCESS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbiret {
    pub error: c_long,
    pub value: c_long,
}

extern "C" {
    pub fn sbi_init();
}
extern "C" {
    pub fn __sbi_base_ecall(fid: c_int) -> c_long;
}

extern "C" {
    pub fn sbi_console_putchar(ch: c_int);
}
extern "C" {
    pub fn sbi_console_getchar() -> c_int;
}

extern "C" {
    pub fn sbi_get_mvendorid() -> c_long;
}
extern "C" {
    pub fn sbi_get_marchid() -> c_long;
}
extern "C" {
    pub fn sbi_get_mimpid() -> c_long;
}
extern "C" {
    pub fn sbi_set_timer(stime_value: u64);
}
extern "C" {
    pub fn sbi_shutdown();
}
extern "C" {
    pub fn sbi_send_ipi(cpu: c_uint);
}
extern "C" {
    pub fn sbi_remote_fence_i(cpu_mask: *const cpumask) -> c_int;
}
extern "C" {
    pub fn sbi_probe_extension(ext: c_int) -> c_long;
}
extern "C" {
    pub fn sbi_fwft_set(feature: u32, value: c_ulong, flags: c_ulong) -> c_int;
}
//
// sbi_fwft_set_online_cpus() - Set a feature on all online cpus
// @feature: The feature to be set
// @value: The feature value to be set
// @flags: FWFT feature set flags
//
// Return: 0 on success, appropriate linux error code otherwise.
//
extern "C" {
    pub fn sbi_fwft_set_cpumask(_arg: cpu_online_mask, _arg: feature, _arg: value, _arg: flags) -> return;
}
// Check if current SBI specification version is 0.1 or not
// Get the major version of SBI
// Get the minor version of SBI
// Make SBI version
extern "C" {
    pub fn sbi_debug_console_write(bytes: *const c_char, num_bytes: c_uint) -> c_int;
}
extern "C" {
    pub fn sbi_debug_console_read(bytes: *mut c_char, num_bytes: c_uint) -> c_int;
}

extern "C" {
    pub fn riscv_get_mvendorid() -> c_ulong;
}
extern "C" {
    pub fn riscv_get_marchid() -> c_ulong;
}
extern "C" {
    pub fn riscv_cached_mvendorid(cpu_id: c_uint) -> c_ulong;
}
extern "C" {
    pub fn riscv_cached_marchid(cpu_id: c_uint) -> c_ulong;
}
extern "C" {
    pub fn riscv_cached_mimpid(cpu_id: c_uint) -> c_ulong;
}

extern "C" {
    pub fn sbi_ipi_init();
}

