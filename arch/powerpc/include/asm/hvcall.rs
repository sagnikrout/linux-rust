//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hvcall.h
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


// SPDX-License-Identifier: GPL-2.0

pub const H_SUCCESS: c_int = 0;

pub const H_NOT_AVAILABLE: c_int = 3;

pub const H_PARTIAL: c_int = 5;

pub const H_PAGE_REGISTERED: c_int = 15;
pub const H_PARTIAL_STORE: c_int = 16;

// Internal value used in book3s_hv kvm support; not returned to guests
pub const H_TOO_HARD: c_int = 9999;

// Long Busy is a condition that can be returned by the firmware
// when a call cannot be completed now, but the identical call
// should be retried later.  This prevents calls blocking in the
// firmware for long periods of time.  Annoyingly the firmware can return
// a range of return codes, hinting at how long we should wait before
// retrying.  If you don't care for the hint, the macro below is a good
// way to check for the long_busy return codes
//

// Flags

// Flags for H_REGISTER_VPA subfunction field

// VASI States
pub const H_VASI_INVALID: c_int = 0;
pub const H_VASI_ENABLED: c_int = 1;
pub const H_VASI_ABORTED: c_int = 2;
pub const H_VASI_SUSPENDING: c_int = 3;
pub const H_VASI_SUSPENDED: c_int = 4;
pub const H_VASI_RESUMED: c_int = 5;
pub const H_VASI_COMPLETED: c_int = 6;
// VASI signal codes. Only the Cancel code is valid for H_VASI_SIGNAL.
pub const H_VASI_SIGNAL_CANCEL: c_int = 1;
pub const H_VASI_SIGNAL_ABORT: c_int = 2;
pub const H_VASI_SIGNAL_SUSPEND: c_int = 3;
pub const H_VASI_SIGNAL_COMPLETE: c_int = 4;
pub const H_VASI_SIGNAL_ENABLE: c_int = 5;
pub const H_VASI_SIGNAL_FAILOVER: c_int = 6;
// Each control block has to be on a 4K boundary
pub const H_CB_ALIGNMENT: c_int = 4096;
// pSeries hypervisor opcodes
pub const H_REMOVE: c_uint = 0x04;
pub const H_ENTER: c_uint = 0x08;
pub const H_READ: c_uint = 0x0c;
pub const H_CLEAR_MOD: c_uint = 0x10;
pub const H_CLEAR_REF: c_uint = 0x14;
pub const H_PROTECT: c_uint = 0x18;
pub const H_GET_TCE: c_uint = 0x1c;
pub const H_PUT_TCE: c_uint = 0x20;
pub const H_SET_SPRG0: c_uint = 0x24;
pub const H_SET_DABR: c_uint = 0x28;
pub const H_PAGE_INIT: c_uint = 0x2c;
pub const H_SET_ASR: c_uint = 0x30;
pub const H_ASR_ON: c_uint = 0x34;
pub const H_ASR_OFF: c_uint = 0x38;
pub const H_LOGICAL_CI_LOAD: c_uint = 0x3c;
pub const H_LOGICAL_CI_STORE: c_uint = 0x40;
pub const H_LOGICAL_CACHE_LOAD: c_uint = 0x44;
pub const H_LOGICAL_CACHE_STORE: c_uint = 0x48;
pub const H_LOGICAL_ICBI: c_uint = 0x4c;
pub const H_LOGICAL_DCBF: c_uint = 0x50;
pub const H_GET_TERM_CHAR: c_uint = 0x54;
pub const H_PUT_TERM_CHAR: c_uint = 0x58;
pub const H_REAL_TO_LOGICAL: c_uint = 0x5c;
pub const H_HYPERVISOR_DATA: c_uint = 0x60;
pub const H_EOI: c_uint = 0x64;
pub const H_CPPR: c_uint = 0x68;
pub const H_IPI: c_uint = 0x6c;
pub const H_IPOLL: c_uint = 0x70;
pub const H_XIRR: c_uint = 0x74;
pub const H_PERFMON: c_uint = 0x7c;
pub const H_MIGRATE_DMA: c_uint = 0x78;
pub const H_REGISTER_VPA: c_uint = 0xDC;
pub const H_CEDE: c_uint = 0xE0;
pub const H_CONFER: c_uint = 0xE4;
pub const H_PROD: c_uint = 0xE8;
pub const H_GET_PPP: c_uint = 0xEC;
pub const H_SET_PPP: c_uint = 0xF0;
pub const H_PURR: c_uint = 0xF4;
pub const H_PIC: c_uint = 0xF8;
pub const H_REG_CRQ: c_uint = 0xFC;
pub const H_FREE_CRQ: c_uint = 0x100;
pub const H_VIO_SIGNAL: c_uint = 0x104;
pub const H_SEND_CRQ: c_uint = 0x108;
pub const H_COPY_RDMA: c_uint = 0x110;
pub const H_REGISTER_LOGICAL_LAN: c_uint = 0x114;
pub const H_FREE_LOGICAL_LAN: c_uint = 0x118;
pub const H_ADD_LOGICAL_LAN_BUFFER: c_uint = 0x11C;
pub const H_SEND_LOGICAL_LAN: c_uint = 0x120;
pub const H_BULK_REMOVE: c_uint = 0x124;
pub const H_MULTICAST_CTRL: c_uint = 0x130;
pub const H_SET_XDABR: c_uint = 0x134;
pub const H_STUFF_TCE: c_uint = 0x138;
pub const H_PUT_TCE_INDIRECT: c_uint = 0x13C;
pub const H_CHANGE_LOGICAL_LAN_MAC: c_uint = 0x14C;
pub const H_VTERM_PARTNER_INFO: c_uint = 0x150;
pub const H_REGISTER_VTERM: c_uint = 0x154;
pub const H_FREE_VTERM: c_uint = 0x158;
pub const H_RESET_EVENTS: c_uint = 0x15C;
pub const H_ALLOC_RESOURCE: c_uint = 0x160;
pub const H_FREE_RESOURCE: c_uint = 0x164;
pub const H_MODIFY_QP: c_uint = 0x168;
pub const H_QUERY_QP: c_uint = 0x16C;
pub const H_REREGISTER_PMR: c_uint = 0x170;
pub const H_REGISTER_SMR: c_uint = 0x174;
pub const H_QUERY_MR: c_uint = 0x178;
pub const H_QUERY_MW: c_uint = 0x17C;
pub const H_QUERY_HCA: c_uint = 0x180;
pub const H_QUERY_PORT: c_uint = 0x184;
pub const H_MODIFY_PORT: c_uint = 0x188;
pub const H_DEFINE_AQP1: c_uint = 0x18C;
pub const H_GET_TRACE_BUFFER: c_uint = 0x190;
pub const H_DEFINE_AQP0: c_uint = 0x194;
pub const H_RESIZE_MR: c_uint = 0x198;
pub const H_ATTACH_MCQP: c_uint = 0x19C;
pub const H_DETACH_MCQP: c_uint = 0x1A0;
pub const H_CREATE_RPT: c_uint = 0x1A4;
pub const H_REMOVE_RPT: c_uint = 0x1A8;
pub const H_REGISTER_RPAGES: c_uint = 0x1AC;
pub const H_DISABLE_AND_GET: c_uint = 0x1B0;
pub const H_ERROR_DATA: c_uint = 0x1B4;
pub const H_GET_HCA_INFO: c_uint = 0x1B8;
pub const H_GET_PERF_COUNT: c_uint = 0x1BC;
pub const H_MANAGE_TRACE: c_uint = 0x1C0;
pub const H_GET_CPU_CHARACTERISTICS: c_uint = 0x1C8;
pub const H_FREE_LOGICAL_LAN_BUFFER: c_uint = 0x1D4;
pub const H_QUERY_INT_STATE: c_uint = 0x1E4;
pub const H_POLL_PENDING: c_uint = 0x1D8;
pub const H_ILLAN_ATTRIBUTES: c_uint = 0x244;
pub const H_ADD_LOGICAL_LAN_BUFFERS: c_uint = 0x248;
pub const H_MODIFY_HEA_QP: c_uint = 0x250;
pub const H_QUERY_HEA_QP: c_uint = 0x254;
pub const H_QUERY_HEA: c_uint = 0x258;
pub const H_QUERY_HEA_PORT: c_uint = 0x25C;
pub const H_MODIFY_HEA_PORT: c_uint = 0x260;
pub const H_REG_BCMC: c_uint = 0x264;
pub const H_DEREG_BCMC: c_uint = 0x268;
pub const H_REGISTER_HEA_RPAGES: c_uint = 0x26C;
pub const H_DISABLE_AND_GET_HEA: c_uint = 0x270;
pub const H_GET_HEA_INFO: c_uint = 0x274;
pub const H_ALLOC_HEA_RESOURCE: c_uint = 0x278;
pub const H_ADD_CONN: c_uint = 0x284;
pub const H_DEL_CONN: c_uint = 0x288;
pub const H_JOIN: c_uint = 0x298;
pub const H_VASI_SIGNAL: c_uint = 0x2A0;
pub const H_VASI_STATE: c_uint = 0x2A4;
pub const H_VIOCTL: c_uint = 0x2A8;
pub const H_ENABLE_CRQ: c_uint = 0x2B0;
pub const H_GET_EM_PARMS: c_uint = 0x2B8;
pub const H_SET_MPP: c_uint = 0x2D0;
pub const H_GET_MPP: c_uint = 0x2D4;
pub const H_REG_SUB_CRQ: c_uint = 0x2DC;
pub const H_HOME_NODE_ASSOCIATIVITY: c_uint = 0x2EC;
pub const H_FREE_SUB_CRQ: c_uint = 0x2E0;
pub const H_SEND_SUB_CRQ: c_uint = 0x2E4;
pub const H_SEND_SUB_CRQ_INDIRECT: c_uint = 0x2E8;
pub const H_BEST_ENERGY: c_uint = 0x2F4;
pub const H_XIRR_X: c_uint = 0x2FC;
pub const H_RANDOM: c_uint = 0x300;
pub const H_COP: c_uint = 0x304;
pub const H_GET_MPP_X: c_uint = 0x314;
pub const H_SET_MODE: c_uint = 0x31C;
pub const H_BLOCK_REMOVE: c_uint = 0x328;
pub const H_CLEAR_HPT: c_uint = 0x358;
pub const H_REQUEST_VMC: c_uint = 0x360;
pub const H_RESIZE_HPT_PREPARE: c_uint = 0x36C;
pub const H_RESIZE_HPT_COMMIT: c_uint = 0x370;
pub const H_REGISTER_PROC_TBL: c_uint = 0x37C;
pub const H_SIGNAL_SYS_RESET: c_uint = 0x380;
pub const H_ALLOCATE_VAS_WINDOW: c_uint = 0x388;
pub const H_MODIFY_VAS_WINDOW: c_uint = 0x38C;
pub const H_DEALLOCATE_VAS_WINDOW: c_uint = 0x390;
pub const H_QUERY_VAS_WINDOW: c_uint = 0x394;
pub const H_QUERY_VAS_CAPABILITIES: c_uint = 0x398;
pub const H_QUERY_NX_CAPABILITIES: c_uint = 0x39C;
pub const H_GET_NX_FAULT: c_uint = 0x3A0;
pub const H_INT_GET_SOURCE_INFO: c_uint = 0x3A8;
pub const H_INT_SET_SOURCE_CONFIG: c_uint = 0x3AC;
pub const H_INT_GET_SOURCE_CONFIG: c_uint = 0x3B0;
pub const H_INT_GET_QUEUE_INFO: c_uint = 0x3B4;
pub const H_INT_SET_QUEUE_CONFIG: c_uint = 0x3B8;
pub const H_INT_GET_QUEUE_CONFIG: c_uint = 0x3BC;
pub const H_INT_SET_OS_REPORTING_LINE: c_uint = 0x3C0;
pub const H_INT_GET_OS_REPORTING_LINE: c_uint = 0x3C4;
pub const H_INT_ESB: c_uint = 0x3C8;
pub const H_INT_SYNC: c_uint = 0x3CC;
pub const H_INT_RESET: c_uint = 0x3D0;
pub const H_SCM_READ_METADATA: c_uint = 0x3E4;
pub const H_SCM_WRITE_METADATA: c_uint = 0x3E8;
pub const H_SCM_BIND_MEM: c_uint = 0x3EC;
pub const H_SCM_UNBIND_MEM: c_uint = 0x3F0;
pub const H_SCM_QUERY_BLOCK_MEM_BINDING: c_uint = 0x3F4;
pub const H_SCM_QUERY_LOGICAL_MEM_BINDING: c_uint = 0x3F8;
pub const H_SCM_UNBIND_ALL: c_uint = 0x3FC;
pub const H_SCM_HEALTH: c_uint = 0x400;
pub const H_SCM_PERFORMANCE_STATS: c_uint = 0x418;
pub const H_PKS_GET_CONFIG: c_uint = 0x41C;
pub const H_PKS_SET_PASSWORD: c_uint = 0x420;
pub const H_PKS_GEN_PASSWORD: c_uint = 0x424;
pub const H_PKS_WRITE_OBJECT: c_uint = 0x42C;
pub const H_PKS_GEN_KEY: c_uint = 0x430;
pub const H_PKS_READ_OBJECT: c_uint = 0x434;
pub const H_PKS_REMOVE_OBJECT: c_uint = 0x438;
pub const H_PKS_CONFIRM_OBJECT_FLUSHED: c_uint = 0x43C;
pub const H_RPT_INVALIDATE: c_uint = 0x448;
pub const H_SCM_FLUSH: c_uint = 0x44C;
pub const H_GET_ENERGY_SCALE_INFO: c_uint = 0x450;
pub const H_PKS_SIGNED_UPDATE: c_uint = 0x454;
pub const H_HTM: c_uint = 0x458;
pub const H_WATCHDOG: c_uint = 0x45C;
pub const H_GUEST_GET_CAPABILITIES: c_uint = 0x460;
pub const H_GUEST_SET_CAPABILITIES: c_uint = 0x464;
pub const H_GUEST_CREATE: c_uint = 0x470;
pub const H_GUEST_CREATE_VCPU: c_uint = 0x474;
pub const H_GUEST_GET_STATE: c_uint = 0x478;
pub const H_GUEST_SET_STATE: c_uint = 0x47C;
pub const H_GUEST_RUN_VCPU: c_uint = 0x480;
pub const H_GUEST_COPY_MEMORY: c_uint = 0x484;
pub const H_GUEST_DELETE: c_uint = 0x488;
pub const H_PKS_WRAP_OBJECT: c_uint = 0x490;
pub const H_PKS_UNWRAP_OBJECT: c_uint = 0x494;

// Scope args for H_SCM_UNBIND_ALL

// H_VIOCTL functions
pub const H_GET_VIOA_DUMP_SIZE: c_uint = 0x01;
pub const H_GET_VIOA_DUMP: c_uint = 0x02;
pub const H_GET_ILLAN_NUM_VLAN_IDS: c_uint = 0x03;
pub const H_GET_ILLAN_VLAN_ID_LIST: c_uint = 0x04;
pub const H_GET_ILLAN_SWITCH_ID: c_uint = 0x05;
pub const H_DISABLE_MIGRATION: c_uint = 0x06;
pub const H_ENABLE_MIGRATION: c_uint = 0x07;
pub const H_GET_PARTNER_INFO: c_uint = 0x08;
pub const H_GET_PARTNER_WWPN_LIST: c_uint = 0x09;
pub const H_DISABLE_ALL_VIO_INTS: c_uint = 0x0A;
pub const H_DISABLE_VIO_INTERRUPT: c_uint = 0x0B;
pub const H_ENABLE_VIO_INTERRUPT: c_uint = 0x0C;
pub const H_GET_SESSION_TOKEN: c_uint = 0x19;
pub const H_SESSION_ERR_DETECTED: c_uint = 0x1A;
// Platform specific hcalls, used by KVM
pub const H_RTAS: c_uint = 0xf000;
//
// Platform specific hcalls, used by QEMU/SLOF. These are ignored by
// KVM and only kept here so we can identify them during tracing.
//
pub const H_LOGICAL_MEMOP: c_uint = 0xF001;

// "Platform specific hcalls", provided by PHYP
pub const H_GET_24X7_CATALOG_PAGE: c_uint = 0xF078;
pub const H_GET_24X7_DATA: c_uint = 0xF07C;
pub const H_GET_PERF_COUNTER_INFO: c_uint = 0xF080;
// Platform-specific hcalls used for nested HV KVM
pub const H_SET_PARTITION_TABLE: c_uint = 0xF800;
pub const H_ENTER_NESTED: c_uint = 0xF804;
pub const H_TLB_INVALIDATE: c_uint = 0xF808;
pub const H_COPY_TOFROM_GUEST: c_uint = 0xF80C;
// Flags for H_SVM_PAGE_IN
pub const H_PAGE_IN_SHARED: c_uint = 0x1;
// Platform-specific hcalls used by the Ultravisor
pub const H_SVM_PAGE_IN: c_uint = 0xEF00;
pub const H_SVM_PAGE_OUT: c_uint = 0xEF04;
pub const H_SVM_INIT_START: c_uint = 0xEF08;
pub const H_SVM_INIT_DONE: c_uint = 0xEF0C;
pub const H_SVM_INIT_ABORT: c_uint = 0xEF14;
// Values for 2nd argument to H_SET_MODE
pub const H_SET_MODE_RESOURCE_SET_CIABR: c_int = 1;
pub const H_SET_MODE_RESOURCE_SET_DAWR0: c_int = 2;
pub const H_SET_MODE_RESOURCE_ADDR_TRANS_MODE: c_int = 3;
pub const H_SET_MODE_RESOURCE_LE: c_int = 4;
pub const H_SET_MODE_RESOURCE_SET_DAWR1: c_int = 5;
// Values for argument to H_SIGNAL_SYS_RESET

// >= 0 values are CPU number
// H_GET_CPU_CHARACTERISTICS return values

// Flag values used in H_REGISTER_PROC_TBL hcall
pub const PROC_TABLE_OP_MASK: c_uint = 0x18;
pub const PROC_TABLE_DEREG: c_uint = 0x10;
pub const PROC_TABLE_NEW: c_uint = 0x18;
pub const PROC_TABLE_TYPE_MASK: c_uint = 0x06;
pub const PROC_TABLE_HPT_SLB: c_uint = 0x00;
pub const PROC_TABLE_HPT_PT: c_uint = 0x02;
pub const PROC_TABLE_RADIX: c_uint = 0x04;
pub const PROC_TABLE_GTSE: c_uint = 0x01;
//
// Defines for
// H_RPT_INVALIDATE - Invalidate RPT translation lookaside information.
//
// Type of translation to invalidate (type)
pub const H_RPTI_TYPE_NESTED: c_uint = 0x0001	/* Invalidate nested guest partition-scope */;
pub const H_RPTI_TYPE_TLB: c_uint = 0x0002	/* Invalidate TLB */;
pub const H_RPTI_TYPE_PWC: c_uint = 0x0004	/* Invalidate Page Walk Cache */;
// Invalidate caching of Process Table Entries if H_RPTI_TYPE_NESTED is clear
pub const H_RPTI_TYPE_PRT: c_uint = 0x0008;
// Invalidate caching of Partition Table Entries if H_RPTI_TYPE_NESTED is set
pub const H_RPTI_TYPE_PAT: c_uint = 0x0008;

// Invalidation targets (target)
pub const H_RPTI_TARGET_CMMU: c_uint = 0x01 /* All virtual processors in the partition */;
pub const H_RPTI_TARGET_CMMU_LOCAL: c_uint = 0x02 /* Current virtual processor */;
// All nest/accelerator agents in use by the partition
pub const H_RPTI_TARGET_NMMU: c_uint = 0x04;
// Page size mask (page sizes)
pub const H_RPTI_PAGE_4K: c_uint = 0x01;
pub const H_RPTI_PAGE_64K: c_uint = 0x02;
pub const H_RPTI_PAGE_2M: c_uint = 0x04;
pub const H_RPTI_PAGE_1G: c_uint = 0x08;

// Flags for H_GUEST_{S,G}_STATE

// Flag values used for H_{S,G}SET_GUEST_CAPABILITIES

//
// Defines for H_HTM - Macros for hardware trace macro (HTM) function.
//

pub const H_HTM_OP_CAPABILITIES: c_uint = 0x01;
pub const H_HTM_OP_STATUS: c_uint = 0x02;
pub const H_HTM_OP_SETUP: c_uint = 0x03;
pub const H_HTM_OP_CONFIGURE: c_uint = 0x04;
pub const H_HTM_OP_START: c_uint = 0x05;
pub const H_HTM_OP_STOP: c_uint = 0x06;
pub const H_HTM_OP_DECONFIGURE: c_uint = 0x07;
pub const H_HTM_OP_DUMP_DETAILS: c_uint = 0x08;
pub const H_HTM_OP_DUMP_DATA: c_uint = 0x09;
pub const H_HTM_OP_DUMP_SYSMEM_CONF: c_uint = 0x0a;
pub const H_HTM_OP_DUMP_SYSPROC_CONF: c_uint = 0x0b;

pub const H_HTM_TYPE_NEST: c_uint = 0x01;
pub const H_HTM_TYPE_CORE: c_uint = 0x02;
pub const H_HTM_TYPE_LLAT: c_uint = 0x03;
pub const H_HTM_TYPE_GLOBAL: c_uint = 0xff;

//
// plpar_hcall_norets: - Make a pseries hypervisor call with no return arguments
// @opcode: The hypervisor call to make.
//
// This call supports up to 7 arguments and only returns the status of
// the hcall. Use this version where possible, its slightly faster than
// the other plpar_hcalls.
//
extern "C" {
    pub fn plpar_hcall_norets(opcode: c_ulong, ...) -> c_long;
}
// Variant which does not do hcall tracing
extern "C" {
    pub fn plpar_hcall_norets_notrace(opcode: c_ulong, ...) -> c_long;
}
//
// plpar_hcall: - Make a pseries hypervisor call
// @opcode: The hypervisor call to make.
// @retbuf: Buffer to store up to 4 return arguments in.
//
// This call supports up to 6 arguments and 4 return arguments. Use
// PLPAR_HCALL_BUFSIZE to size the return argument buffer.
//
// Used for all but the craziest of phyp interfaces (see plpar_hcall9)
//
pub const PLPAR_HCALL_BUFSIZE: c_int = 4;
extern "C" {
    pub fn plpar_hcall(opcode: c_ulong, PLPAR_HCALL_BUFSIZE]: unsigned long retbuf[static, ...) -> c_long;
}
//
// plpar_hcall_raw: - Make a hypervisor call without calculating hcall stats
// @opcode: The hypervisor call to make.
// @retbuf: Buffer to store up to 4 return arguments in.
//
// This call supports up to 6 arguments and 4 return arguments. Use
// PLPAR_HCALL_BUFSIZE to size the return argument buffer.
//
// Used when phyp interface needs to be called in real mode. Similar to
// plpar_hcall, but plpar_hcall_raw works in real mode and does not
// calculate hypervisor call statistics.
//
extern "C" {
    pub fn plpar_hcall_raw(opcode: c_ulong, PLPAR_HCALL_BUFSIZE]: unsigned long retbuf[static, ...) -> c_long;
}
//
// plpar_hcall9: - Make a pseries hypervisor call with up to 9 return arguments
// @opcode: The hypervisor call to make.
// @retbuf: Buffer to store up to 9 return arguments in.
//
// This call supports up to 9 arguments and 9 return arguments. Use
// PLPAR_HCALL9_BUFSIZE to size the return argument buffer.
//
pub const PLPAR_HCALL9_BUFSIZE: c_int = 9;
extern "C" {
    pub fn plpar_hcall9(opcode: c_ulong, PLPAR_HCALL9_BUFSIZE]: unsigned long retbuf[static, ...) -> c_long;
}
extern "C" {
    pub fn plpar_hcall9_raw(opcode: c_ulong, PLPAR_HCALL9_BUFSIZE]: unsigned long retbuf[static, ...) -> c_long;
}
// pseries hcall tracing
extern "C" {
    pub fn __trace_hcall_entry(opcode: c_ulong, args: *mut c_ulong);
}
extern "C" {
    pub fn __trace_hcall_exit(opcode: c_long, retval: c_long, retbuf: *mut c_ulong);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvcall_mpp_data {
    pub entitled_mem: c_ulong,
    pub mapped_mem: c_ulong,
    pub group_num: c_ushort,
    pub pool_num: c_ushort,
    pub mem_weight: c_uchar,
    pub unallocated_mem_weight: c_uchar,
    pub /: *mut *mut unsigned long unallocated_entitlement; / value in bytes,
    pub pool_size: c_ulong,
    pub loan_request: signed long,
    pub backing_mem: c_ulong,
}

extern "C" {
    pub fn h_get_mpp(mpp_data: *mut hvcall_mpp_data) -> c_long;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvcall_mpp_x_data {
    pub coalesced_bytes: c_ulong,
    pub pool_coalesced_bytes: c_ulong,
    pub pool_purr_cycles: c_ulong,
    pub pool_spurr_cycles: c_ulong,
    pub reserved: [c_ulong; 3],
}

extern "C" {
    pub fn h_get_mpp_x(mpp_x_data: *mut hvcall_mpp_x_data) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct h_cpu_char_result {
    pub character: u64,
    pub behaviour: u64,
}

//
// Register state for entering a nested guest with H_ENTER_NESTED.
// New member must be added at the end.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_guest_state {
    pub /: *mut *mut u64 version; / version of this structure layout, must be first,
    pub lpid: u32,
    pub vcpu_token: u32,
// These registers are hypervisor privileged (at least for writing)
    pub lpcr: u64,
    pub pcr: u64,
    pub amor: u64,
    pub dpdes: u64,
    pub hfscr: u64,
    pub tb_offset: i64,
    pub dawr0: u64,
    pub dawrx0: u64,
    pub ciabr: u64,
    pub hdec_expiry: u64,
    pub purr: u64,
    pub spurr: u64,
    pub ic: u64,
    pub vtb: u64,
    pub hdar: u64,
    pub hdsisr: u64,
    pub heir: u64,
    pub asdr: u64,
// These are OS privileged but need to be set late in guest entry
    pub srr0: u64,
    pub srr1: u64,
    pub sprg: [u64; 4],
    pub pidr: u64,
    pub cfar: u64,
    pub ppr: u64,
// Version 1 ends here
    pub dawr1: u64,
    pub dawrx1: u64,
// Version 2 ends here
}

// Latest version of hv_guest_state structure
pub const HV_GUEST_STATE_VERSION: c_int = 2;
extern "C" {
    pub fn offsetofend(hv_guest_state: struct, _arg: ppr) -> return;
}
extern "C" {
    pub fn offsetofend(hv_guest_state: struct, _arg: dawrx1) -> return;
}
//
// From the document "H_GetPerformanceCounterInfo Interface" v1.07
//
// H_GET_PERF_COUNTER_INFO argument
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_get_perf_counter_info_params {
    pub /: *mut *mut __be32 counter_request; / I,
    pub /: *mut *mut __be32 starting_index; / IO,
    pub /: *mut *mut __be16 secondary_index; / IO,
    pub /: *mut *mut __be16 returned_values; / O,
    pub /: *mut *mut *mut __be32 detail_rc; / O, only needed when called via _norets(),
//
// O, size each of counter_value element in bytes, only set for version
// >= 0x3
//
    pub cv_element_size: __be16,
// I, 0 (zero) for versions < 0x3
    pub counter_info_version_in: __u8,
// O, 0 (zero) if version < 0x3. Must be set to 0 when making hcall
    pub counter_info_version_out: __u8,
    pub reserved: [__u8; 0xC],
    pub counter_value: [__u8; ],
    pub __packed: },
pub const HGPCI_REQ_BUFFER_SIZE: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_gpci_request_buffer {
    pub params: hv_get_perf_counter_info_params,
    pub bytes: [u8; HGPCI_MAX_DATA_BYTES],
    pub __packed: },

