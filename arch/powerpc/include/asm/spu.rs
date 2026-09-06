//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/spu.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SPU core / file system interface and HW structures
//
// (C) Copyright IBM Deutschland Entwicklung GmbH 2005
//
// Author: Arnd Bergmann <arndb@de.ibm.com>
//

pub const MFC_PUT_CMD: c_uint = 0x20;
pub const MFC_PUTS_CMD: c_uint = 0x28;
pub const MFC_PUTR_CMD: c_uint = 0x30;
pub const MFC_PUTF_CMD: c_uint = 0x22;
pub const MFC_PUTB_CMD: c_uint = 0x21;
pub const MFC_PUTFS_CMD: c_uint = 0x2A;
pub const MFC_PUTBS_CMD: c_uint = 0x29;
pub const MFC_PUTRF_CMD: c_uint = 0x32;
pub const MFC_PUTRB_CMD: c_uint = 0x31;
pub const MFC_PUTL_CMD: c_uint = 0x24;
pub const MFC_PUTRL_CMD: c_uint = 0x34;
pub const MFC_PUTLF_CMD: c_uint = 0x26;
pub const MFC_PUTLB_CMD: c_uint = 0x25;
pub const MFC_PUTRLF_CMD: c_uint = 0x36;
pub const MFC_PUTRLB_CMD: c_uint = 0x35;
pub const MFC_GET_CMD: c_uint = 0x40;
pub const MFC_GETS_CMD: c_uint = 0x48;
pub const MFC_GETF_CMD: c_uint = 0x42;
pub const MFC_GETB_CMD: c_uint = 0x41;
pub const MFC_GETFS_CMD: c_uint = 0x4A;
pub const MFC_GETBS_CMD: c_uint = 0x49;
pub const MFC_GETL_CMD: c_uint = 0x44;
pub const MFC_GETLF_CMD: c_uint = 0x46;
pub const MFC_GETLB_CMD: c_uint = 0x45;
pub const MFC_SDCRT_CMD: c_uint = 0x80;
pub const MFC_SDCRTST_CMD: c_uint = 0x81;
pub const MFC_SDCRZ_CMD: c_uint = 0x89;
pub const MFC_SDCRS_CMD: c_uint = 0x8D;
pub const MFC_SDCRF_CMD: c_uint = 0x8F;
pub const MFC_GETLLAR_CMD: c_uint = 0xD0;
pub const MFC_PUTLLC_CMD: c_uint = 0xB4;
pub const MFC_PUTLLUC_CMD: c_uint = 0xB0;
pub const MFC_PUTQLLUC_CMD: c_uint = 0xB8;
pub const MFC_SNDSIG_CMD: c_uint = 0xA0;
pub const MFC_SNDSIGB_CMD: c_uint = 0xA1;
pub const MFC_SNDSIGF_CMD: c_uint = 0xA2;
pub const MFC_BARRIER_CMD: c_uint = 0xC0;
pub const MFC_EIEIO_CMD: c_uint = 0xC8;
pub const MFC_SYNC_CMD: c_uint = 0xCC;

pub const MFC_MIN_DMA_LIST_SIZE: c_uint = 0x0008  /*   8 bytes */;
pub const MFC_MAX_DMA_LIST_SIZE: c_uint = 0x4000  /* 16K bytes */;

// Events for Channels 0-2
pub const MFC_DMA_TAG_STATUS_UPDATE_EVENT: c_uint = 0x00000001;
pub const MFC_DMA_TAG_CMD_STALL_NOTIFY_EVENT: c_uint = 0x00000002;
pub const MFC_DMA_QUEUE_AVAILABLE_EVENT: c_uint = 0x00000008;
pub const MFC_SPU_MAILBOX_WRITTEN_EVENT: c_uint = 0x00000010;
pub const MFC_DECREMENTER_EVENT: c_uint = 0x00000020;
pub const MFC_PU_INT_MAILBOX_AVAILABLE_EVENT: c_uint = 0x00000040;
pub const MFC_PU_MAILBOX_AVAILABLE_EVENT: c_uint = 0x00000080;
pub const MFC_SIGNAL_2_EVENT: c_uint = 0x00000100;
pub const MFC_SIGNAL_1_EVENT: c_uint = 0x00000200;
pub const MFC_LLR_LOST_EVENT: c_uint = 0x00000400;
pub const MFC_PRIV_ATTN_EVENT: c_uint = 0x00000800;
pub const MFC_MULTI_SRC_EVENT: c_uint = 0x00001000;
// Flag indicating progress during context switch.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu_utilization_state {
    SPU_UTIL_USER,
    SPU_UTIL_SYSTEM,
    SPU_UTIL_IOWAIT,
    SPU_UTIL_IDLE_LOADED,
    SPU_UTIL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu {
    pub name: *const c_char,
    pub local_store_phys: c_ulong,
    pub local_store: *mut u8,
    pub problem_phys: c_ulong,
    pub problem: *mut spu_problem __iomem,
    pub priv2: *mut spu_priv2 __iomem,
    pub cbe_list: list_head,
    pub full_list: list_head,
    pub alloc_state: { SPU_FREE, SPU_USED },
    pub number: c_int,
    pub irqs: [c_uint; 3],
    pub node: u32,
    pub flags: c_ulong,
    pub class_0_pending: u64,
    pub class_0_dar: u64,
    pub class_1_dar: u64,
    pub class_1_dsisr: u64,
    pub ls_size: usize,
    pub slb_replace: c_uint,
    pub mm: *mut mm_struct,
    pub ctx: *mut spu_context,
    pub rq: *mut spu_runqueue,
    pub timestamp: c_ulonglong,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub register_lock: spinlock_t,
    pub spu): *mut *mut void ( wbox_callback)(struct spu,
    pub spu): *mut *mut void ( ibox_callback)(struct spu,
    pub irq): *mut *mut *mut void ( stop_callback)(struct spu spu, int,
    pub spu): *mut *mut void ( mfc_callback)(struct spu,
    pub irq_c0: [c_char; 8],
    pub irq_c1: [c_char; 8],
    pub irq_c2: [c_char; 8],
    pub spe_id: u64,
    pub /: *mut *mut *mut void pdata; / platform private data,
// of based platforms only
    pub devnode: *mut device_node,
// native only
    pub priv1: *mut spu_priv1 __iomem,
// beat only
    pub shadow_int_mask_RW: [u64; 3],
    pub dev: device,
    pub has_mem_affinity: c_int,
    pub aff_list: list_head,
// protected by interrupt reentrancy
    pub util_state: spu_utilization_state,
    pub tstamp: c_ulonglong,
    pub times: [c_ulonglong; SPU_UTIL_MAX],
    pub vol_ctx_switch: c_ulonglong,
    pub invol_ctx_switch: c_ulonglong,
    pub min_flt: c_ulonglong,
    pub maj_flt: c_ulonglong,
    pub hash_flt: c_ulonglong,
    pub slb_flt: c_ulonglong,
    pub class2_intr: c_ulonglong,
    pub libassist: c_ulonglong,
    pub stats: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cbe_spu_info {
    pub list_mutex: mutex,
    pub spus: list_head,
    pub n_spus: c_int,
    pub nr_active: c_int,
    pub busy_spus: core::sync::atomic::AtomicI32,
    pub reserved_spus: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn spu_init_channels(spu: *mut spu);
}
extern "C" {
    pub fn spu_irq_setaffinity(spu: *mut spu, cpu: c_int);
}
extern "C" {
    pub fn spu_invalidate_slbs(spu: *mut spu);
}
extern "C" {
    pub fn spu_associate_mm(spu: *mut spu, mm: *mut mm_struct);
}
extern "C" {
    pub fn spu_64k_pages_available() -> c_int;
}
// Calls from the memory management to the SPU
extern "C" {
    pub fn spu_flush_all_slbs(mm: *mut mm_struct);
}
// system callbacks from the SPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_syscall_block {
    pub nr_ret: u64,
    pub parm: [u64; 6],
}

extern "C" {
    pub fn spu_sys_callback(s: *mut spu_syscall_block) -> c_long;
}
// syscalls implemented in spufs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spufs_calls {
    pub neighbor): *mut file,
    pub ustatus): *mut __u32 __user,
    pub (*coredump_extra_notes_size)(void): *mut c_int,
    pub cprm): *mut *mut int (coredump_extra_notes_write)(struct coredump_params,
    pub (*notify_spus_active)(void): *mut c_void,
    pub owner: *mut module,
}

// return status from spu_run, same as in libspe
pub const SPE_EVENT_DMA_ALIGNMENT: c_uint = 0x0008	/*A DMA alignment error */;
pub const SPE_EVENT_SPE_ERROR: c_uint = 0x0010	/*An illegal instruction error*/;
pub const SPE_EVENT_SPE_DATA_SEGMENT: c_uint = 0x0020	/*A DMA segmentation error    */;
pub const SPE_EVENT_SPE_DATA_STORAGE: c_uint = 0x0040	/*A DMA storage error */;
pub const SPE_EVENT_INVALID_DMA: c_uint = 0x0800	/* Invalid MFC DMA */;
//
// Flags for sys_spu_create.
//
pub const SPU_CREATE_EVENTS_ENABLED: c_uint = 0x0001;
pub const SPU_CREATE_GANG: c_uint = 0x0002;
pub const SPU_CREATE_NOSCHED: c_uint = 0x0004;
pub const SPU_CREATE_ISOLATE: c_uint = 0x0008;
pub const SPU_CREATE_AFFINITY_SPU: c_uint = 0x0010;
pub const SPU_CREATE_AFFINITY_MEM: c_uint = 0x0020;
pub const SPU_CREATE_FLAG_ALL: c_uint = 0x003f /* mask of all valid flags */;
extern "C" {
    pub fn register_spu_syscalls(calls: *mut spufs_calls) -> c_int;
}
extern "C" {
    pub fn unregister_spu_syscalls(calls: *mut spufs_calls);
}
extern "C" {
    pub fn spu_add_dev_attr(attr: *mut device_attribute) -> c_int;
}
extern "C" {
    pub fn spu_remove_dev_attr(attr: *mut device_attribute);
}
extern "C" {
    pub fn spu_add_dev_attr_group(attrs: *const attribute_group) -> c_int;
}
extern "C" {
    pub fn spu_remove_dev_attr_group(attrs: *const attribute_group);
}
extern "C" {
    pub fn notify_spus_active();
}
extern "C" {
    pub fn do_notify_spus_active();
}
//
// This defines the Local Store, Problem Area and Privilege Area of an SPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mfc_tag_size_class_cmd {
    pub mfc_size: u16,
    pub mfc_tag: u16,
    pub pad: u8,
    pub mfc_rclassid: u8,
    pub mfc_cmd: u16,
    pub u: },
    pub mfc_size_tag32: u32,
    pub mfc_class_cmd32: u32,
    pub by32: },
    pub all64: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc_cq_sr {
    pub mfc_cq_data0_RW: u64,
    pub mfc_cq_data1_RW: u64,
    pub mfc_cq_data2_RW: u64,
    pub mfc_cq_data3_RW: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_problem {

    pub /: *mut *mut u64 spc_mssync_RW; / 0x0000,
    pub 0x0008]: u8 pad_0x0008_0x3000[0x3000 -,
// DMA Area
    pub /: *mut *mut u8 pad_0x3000_0x3004[0x4]; / 0x3000,
    pub /: *mut *mut u32 mfc_lsa_W; / 0x3004,
    pub /: *mut *mut u64 mfc_ea_W; / 0x3008,
    pub /: *mut *mut mfc_tag_size_class_cmd mfc_union_W; / 0x3010,
    pub /: *mut *mut u8 pad_0x3018_0x3104[0xec]; / 0x3018,
    pub /: *mut *mut u32 dma_qstatus_R; / 0x3104,
    pub /: *mut *mut u8 pad_0x3108_0x3204[0xfc]; / 0x3108,
    pub /: *mut *mut u32 dma_querytype_RW; / 0x3204,
    pub /: *mut *mut u8 pad_0x3208_0x321c[0x14]; / 0x3208,
    pub /: *mut *mut u32 dma_querymask_RW; / 0x321c,
    pub /: *mut *mut u8 pad_0x3220_0x322c[0xc]; / 0x3220,
    pub /: *mut *mut u32 dma_tagstatus_R; / 0x322c,

    pub /: *mut *mut u8 pad_0x3230_0x4000[0x4000 - 0x3230]; / 0x3230,
// SPU Control Area
    pub /: *mut *mut u8 pad_0x4000_0x4004[0x4]; / 0x4000,
    pub /: *mut *mut u32 pu_mb_R; / 0x4004,
    pub /: *mut *mut u8 pad_0x4008_0x400c[0x4]; / 0x4008,
    pub /: *mut *mut u32 spu_mb_W; / 0x400c,
    pub /: *mut *mut u8 pad_0x4010_0x4014[0x4]; / 0x4010,
    pub /: *mut *mut u32 mb_stat_R; / 0x4014,
    pub /: *mut *mut u8 pad_0x4018_0x401c[0x4]; / 0x4018,
    pub /: *mut *mut u32 spu_runcntl_RW; / 0x401c,

    pub /: *mut *mut u8 pad_0x4020_0x4024[0x4]; / 0x4020,
    pub /: *mut *mut u32 spu_status_R; / 0x4024,
pub const SPU_STOP_STATUS_SHIFT: c_int = 16;
pub const SPU_STATUS_STOPPED: c_uint = 0x0;
pub const SPU_STATUS_RUNNING: c_uint = 0x1;
pub const SPU_STATUS_STOPPED_BY_STOP: c_uint = 0x2;
pub const SPU_STATUS_STOPPED_BY_HALT: c_uint = 0x4;
pub const SPU_STATUS_WAITING_FOR_CHANNEL: c_uint = 0x8;
pub const SPU_STATUS_SINGLE_STEP: c_uint = 0x10;
pub const SPU_STATUS_INVALID_INSTR: c_uint = 0x20;
pub const SPU_STATUS_INVALID_CH: c_uint = 0x40;
pub const SPU_STATUS_ISOLATED_STATE: c_uint = 0x80;
pub const SPU_STATUS_ISOLATED_LOAD_STATUS: c_uint = 0x200;
pub const SPU_STATUS_ISOLATED_EXIT_STATUS: c_uint = 0x400;
    pub /: *mut *mut u8 pad_0x4028_0x402c[0x4]; / 0x4028,
    pub /: *mut *mut u32 spu_spe_R; / 0x402c,
    pub /: *mut *mut u8 pad_0x4030_0x4034[0x4]; / 0x4030,
    pub /: *mut *mut u32 spu_npc_RW; / 0x4034,
    pub /: *mut *mut u8 pad_0x4038_0x14000[0x14000 - 0x4038]; / 0x4038,
// Signal Notification Area
    pub /: *mut *mut u8 pad_0x14000_0x1400c[0xc]; / 0x14000,
    pub /: *mut *mut u32 signal_notify1; / 0x1400c,
    pub /: *mut *mut u8 pad_0x14010_0x1c00c[0x7ffc]; / 0x14010,
    pub /: *mut *mut u32 signal_notify2; / 0x1c00c,
// C attribute field omitted
// SPU Privilege 2 State Area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_priv2 {
// MFC Registers
    pub /: *mut *mut u8 pad_0x0000_0x1100[0x1100 - 0x0000]; / 0x0000,
// SLB Management Registers
    pub /: *mut *mut u8 pad_0x1100_0x1108[0x8]; / 0x1100,
    pub /: *mut *mut u64 slb_index_W; / 0x1108,
pub const SLB_INDEX_MASK: c_uint = 0x7L;
    pub /: *mut *mut u64 slb_esid_RW; / 0x1110,
    pub /: *mut *mut u64 slb_vsid_RW; / 0x1118,

    pub /: *mut *mut u64 slb_invalidate_entry_W; / 0x1120,
    pub /: *mut *mut u64 slb_invalidate_all_W; / 0x1128,
    pub /: *mut *mut u8 pad_0x1130_0x2000[0x2000 - 0x1130]; / 0x1130,
// Context Save / Restore Area
    pub /: *mut *mut mfc_cq_sr spuq[16]; / 0x2000,
    pub /: *mut *mut mfc_cq_sr puq[8]; / 0x2200,
    pub /: *mut *mut u8 pad_0x2300_0x3000[0x3000 - 0x2300]; / 0x2300,
// MFC Control
    pub /: *mut *mut u64 mfc_control_RW; / 0x3000,

    pub /: *mut *mut u8 pad_0x3008_0x4000[0x4000 - 0x3008]; / 0x3008,
// Interrupt Mailbox
    pub /: *mut *mut u64 puint_mb_R; / 0x4000,
    pub /: *mut *mut u8 pad_0x4008_0x4040[0x4040 - 0x4008]; / 0x4008,
// SPU Control
    pub /: *mut *mut u64 spu_privcntl_RW; / 0x4040,

    pub /: *mut *mut u8 pad_0x4048_0x4058[0x10]; / 0x4048,
    pub /: *mut *mut u64 spu_lslr_RW; / 0x4058,
    pub /: *mut *mut u64 spu_chnlcntptr_RW; / 0x4060,
    pub /: *mut *mut u64 spu_chnlcnt_RW; / 0x4068,
    pub /: *mut *mut u64 spu_chnldata_RW; / 0x4070,
    pub /: *mut *mut u64 spu_cfg_RW; / 0x4078,
    pub /: *mut *mut u8 pad_0x4080_0x5000[0x5000 - 0x4080]; / 0x4080,
// PV2_ImplRegs: Implementation-specific privileged-state 2 regs
    pub /: *mut *mut u64 spu_pm_trace_tag_status_RW; / 0x5000,
    pub /: *mut *mut u64 spu_tag_status_query_RW; / 0x5008,

    pub /: *mut *mut u64 spu_cmd_buf1_RW; / 0x5010,

    pub /: *mut *mut u64 spu_cmd_buf2_RW; / 0x5018,

    pub /: *mut *mut u64 spu_atomic_status_RW; / 0x5020,
// C attribute field omitted
// SPU Privilege 1 State Area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_priv1 {
// Control and Configuration Area
    pub /: *mut *mut u64 mfc_sr1_RW; / 0x000,
pub const MFC_STATE1_LOCAL_STORAGE_DECODE_MASK: c_uint = 0x01ull;
pub const MFC_STATE1_BUS_TLBIE_MASK: c_uint = 0x02ull;
pub const MFC_STATE1_REAL_MODE_OFFSET_ENABLE_MASK: c_uint = 0x04ull;
pub const MFC_STATE1_PROBLEM_STATE_MASK: c_uint = 0x08ull;
pub const MFC_STATE1_RELOCATE_MASK: c_uint = 0x10ull;
pub const MFC_STATE1_MASTER_RUN_CONTROL_MASK: c_uint = 0x20ull;
pub const MFC_STATE1_TABLE_SEARCH_MASK: c_uint = 0x40ull;
    pub /: *mut *mut u64 mfc_lpid_RW; / 0x008,
    pub /: *mut *mut u64 spu_idr_RW; / 0x010,
    pub /: *mut *mut u64 mfc_vr_RO; / 0x018,

    pub /: *mut *mut u64 spu_vr_RO; / 0x020,

    pub /: *mut *mut u8 pad_0x28_0x100[0x100 - 0x28]; / 0x28,
// Interrupt Area
    pub /: *mut *mut u64 int_mask_RW[3]; / 0x100,
pub const CLASS0_ENABLE_DMA_ALIGNMENT_INTR: c_uint = 0x1L;
pub const CLASS0_ENABLE_INVALID_DMA_COMMAND_INTR: c_uint = 0x2L;
pub const CLASS0_ENABLE_SPU_ERROR_INTR: c_uint = 0x4L;
pub const CLASS0_ENABLE_MFC_FIR_INTR: c_uint = 0x8L;
pub const CLASS1_ENABLE_SEGMENT_FAULT_INTR: c_uint = 0x1L;
pub const CLASS1_ENABLE_STORAGE_FAULT_INTR: c_uint = 0x2L;
pub const CLASS1_ENABLE_LS_COMPARE_SUSPEND_ON_GET_INTR: c_uint = 0x4L;
pub const CLASS1_ENABLE_LS_COMPARE_SUSPEND_ON_PUT_INTR: c_uint = 0x8L;
pub const CLASS2_ENABLE_MAILBOX_INTR: c_uint = 0x1L;
pub const CLASS2_ENABLE_SPU_STOP_INTR: c_uint = 0x2L;
pub const CLASS2_ENABLE_SPU_HALT_INTR: c_uint = 0x4L;
pub const CLASS2_ENABLE_SPU_DMA_TAG_GROUP_COMPLETE_INTR: c_uint = 0x8L;
pub const CLASS2_ENABLE_MAILBOX_THRESHOLD_INTR: c_uint = 0x10L;
    pub /: *mut *mut u8 pad_0x118_0x140[0x28]; / 0x118,
    pub /: *mut *mut u64 int_stat_RW[3]; / 0x140,
pub const CLASS0_DMA_ALIGNMENT_INTR: c_uint = 0x1L;
pub const CLASS0_INVALID_DMA_COMMAND_INTR: c_uint = 0x2L;
pub const CLASS0_SPU_ERROR_INTR: c_uint = 0x4L;
pub const CLASS0_INTR_MASK: c_uint = 0x7L;
pub const CLASS1_SEGMENT_FAULT_INTR: c_uint = 0x1L;
pub const CLASS1_STORAGE_FAULT_INTR: c_uint = 0x2L;
pub const CLASS1_LS_COMPARE_SUSPEND_ON_GET_INTR: c_uint = 0x4L;
pub const CLASS1_LS_COMPARE_SUSPEND_ON_PUT_INTR: c_uint = 0x8L;
pub const CLASS1_INTR_MASK: c_uint = 0xfL;
pub const CLASS2_MAILBOX_INTR: c_uint = 0x1L;
pub const CLASS2_SPU_STOP_INTR: c_uint = 0x2L;
pub const CLASS2_SPU_HALT_INTR: c_uint = 0x4L;
pub const CLASS2_SPU_DMA_TAG_GROUP_COMPLETE_INTR: c_uint = 0x8L;
pub const CLASS2_MAILBOX_THRESHOLD_INTR: c_uint = 0x10L;
pub const CLASS2_INTR_MASK: c_uint = 0x1fL;
    pub /: *mut *mut u8 pad_0x158_0x180[0x28]; / 0x158,
    pub /: *mut *mut u64 int_route_RW; / 0x180,
// Interrupt Routing
    pub /: *mut *mut u8 pad_0x188_0x200[0x200 - 0x188]; / 0x188,
// Atomic Unit Control Area
    pub /: *mut *mut u64 mfc_atomic_flush_RW; / 0x200,
pub const mfc_atomic_flush_enable: c_uint = 0x1L;
    pub /: *mut *mut u8 pad_0x208_0x280[0x78]; / 0x208,
    pub /: *mut *mut u64 resource_allocation_groupID_RW; / 0x280,
    pub /: *mut *mut u64 resource_allocation_enable_RW; / 0x288,
    pub /: *mut *mut u8 pad_0x290_0x3c8[0x3c8 - 0x290]; / 0x290,
// SPU_Cache_ImplRegs: Implementation-dependent cache registers
    pub /: *mut *mut u64 smf_sbi_signal_sel; / 0x3c8,
pub const smf_sbi_mask_lsb: c_int = 56;

    pub /: *mut *mut u64 smf_ato_signal_sel; / 0x3d0,
pub const smf_ato_mask_lsb: c_int = 35;

    pub /: *mut *mut u8 pad_0x3d8_0x400[0x400 - 0x3d8]; / 0x3d8,
// TLB Management Registers
    pub /: *mut *mut u64 mfc_sdr_RW; / 0x400,
    pub /: *mut *mut u8 pad_0x408_0x500[0xf8]; / 0x408,
    pub /: *mut *mut u64 tlb_index_hint_RO; / 0x500,
    pub /: *mut *mut u64 tlb_index_W; / 0x508,
    pub /: *mut *mut u64 tlb_vpn_RW; / 0x510,
    pub /: *mut *mut u64 tlb_rpn_RW; / 0x518,
    pub /: *mut *mut u8 pad_0x520_0x540[0x20]; / 0x520,
    pub /: *mut *mut u64 tlb_invalidate_entry_W; / 0x540,
    pub /: *mut *mut u64 tlb_invalidate_all_W; / 0x548,
    pub /: *mut *mut u8 pad_0x550_0x580[0x580 - 0x550]; / 0x550,
// SPU_MMU_ImplRegs: Implementation-dependent MMU registers
    pub /: *mut *mut u64 smm_hid; / 0x580,
pub const PAGE_SIZE_MASK: c_uint = 0xf000000000000000ull;
pub const PAGE_SIZE_16MB_64KB: c_uint = 0x2000000000000000ull;
    pub /: *mut *mut u8 pad_0x588_0x600[0x600 - 0x588]; / 0x588,
// MFC Status/Control Area
    pub /: *mut *mut u64 mfc_accr_RW; / 0x600,

    pub /: *mut *mut u8 pad_0x608_0x610[0x8]; / 0x608,
    pub /: *mut *mut u64 mfc_dsisr_RW; / 0x610,

    pub /: *mut *mut u8 pad_0x618_0x620[0x8]; / 0x618,
    pub /: *mut *mut u64 mfc_dar_RW; / 0x620,
    pub /: *mut *mut u8 pad_0x628_0x700[0x700 - 0x628]; / 0x628,
// Replacement Management Table (RMT) Area
    pub /: *mut *mut u64 rmt_index_RW; / 0x700,
    pub /: *mut *mut u8 pad_0x708_0x710[0x8]; / 0x708,
    pub /: *mut *mut u64 rmt_data1_RW; / 0x710,
    pub /: *mut *mut u8 pad_0x718_0x800[0x800 - 0x718]; / 0x718,
// Control/Configuration Registers
    pub /: *mut *mut u64 mfc_dsir_R; / 0x800,

    pub /: *mut *mut u64 mfc_lsacr_RW; / 0x808,

    pub /: *mut *mut u64 mfc_lscrr_R; / 0x810,

pub const MFC_LSCRR_QI_SHIFT: c_int = 32;

    pub /: *mut *mut u8 pad_0x818_0x820[0x8]; / 0x818,
    pub /: *mut *mut u64 mfc_tclass_id_RW; / 0x820,

    pub /: *mut *mut u8 pad_0x828_0x900[0x900 - 0x828]; / 0x828,
// Real Mode Support Registers
    pub /: *mut *mut u64 mfc_rm_boundary; / 0x900,
    pub /: *mut *mut u8 pad_0x908_0x938[0x30]; / 0x908,
    pub /: *mut *mut u64 smf_dma_signal_sel; / 0x938,
pub const mfc_dma1_mask_lsb: c_int = 41;

pub const mfc_dma2_mask_lsb: c_int = 43;

    pub /: *mut *mut u8 pad_0x940_0xa38[0xf8]; / 0x940,
    pub /: *mut *mut u64 smm_signal_sel; / 0xa38,
pub const smm_sig_mask_lsb: c_int = 12;

    pub /: *mut *mut u8 pad_0xa40_0xc00[0xc00 - 0xa40]; / 0xa40,
// DMA Command Error Area
    pub /: *mut *mut u64 mfc_cer_R; / 0xc00,

    pub /: *mut *mut u8 pad_0xc08_0x1000[0x1000 - 0xc08]; / 0xc08,
// PV1_ImplRegs: Implementation-dependent privileged-state 1 regs
// DMA Command Error Area
    pub /: *mut *mut u64 spu_ecc_cntl_RW; / 0x1000,

    pub /: *mut *mut u64 spu_ecc_stat_RW; / 0x1008,

    pub /: *mut *mut u64 spu_ecc_addr_RW; / 0x1010,
    pub /: *mut *mut u64 spu_err_mask_RW; / 0x1018,

    pub /: *mut *mut u8 pad_0x1020_0x1028[0x1028 - 0x1020]; / 0x1020,
// SPU Debug-Trace Bus (DTB) Selection Registers
    pub /: *mut *mut u64 spu_trig0_sel; / 0x1028,
    pub /: *mut *mut u64 spu_trig1_sel; / 0x1030,
    pub /: *mut *mut u64 spu_trig2_sel; / 0x1038,
    pub /: *mut *mut u64 spu_trig3_sel; / 0x1040,
    pub /: *mut *mut u64 spu_trace_sel; / 0x1048,
pub const spu_trace_sel_mask: c_uint = 0x1f1fLL;
pub const spu_trace_sel_bus0_bits: c_uint = 0x1000LL;
pub const spu_trace_sel_bus2_bits: c_uint = 0x0010LL;
    pub /: *mut *mut u64 spu_event0_sel; / 0x1050,
    pub /: *mut *mut u64 spu_event1_sel; / 0x1058,
    pub /: *mut *mut u64 spu_event2_sel; / 0x1060,
    pub /: *mut *mut u64 spu_event3_sel; / 0x1068,
    pub /: *mut *mut u64 spu_trace_cntl; / 0x1070,
// C attribute field omitted

