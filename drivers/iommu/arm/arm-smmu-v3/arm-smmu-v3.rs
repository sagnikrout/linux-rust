//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/arm/arm-smmu-v3/arm-smmu-v3.h
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
// IOMMU API for ARM architected SMMUv3 implementations.
//
// Copyright (C) 2015 ARM Limited
//

// MMIO registers
pub const ARM_SMMU_IDR0: c_uint = 0x0;

pub const IDR0_ST_LVL_2LVL: c_int = 1;

pub const IDR0_STALL_MODEL_STALL: c_int = 0;
pub const IDR0_STALL_MODEL_FORCE: c_int = 2;

pub const IDR0_TTENDIAN_MIXED: c_int = 0;
pub const IDR0_TTENDIAN_LE: c_int = 2;
pub const IDR0_TTENDIAN_BE: c_int = 3;

pub const IDR0_HTTU_ACCESS: c_int = 1;
pub const IDR0_HTTU_ACCESS_DIRTY: c_int = 2;
pub const IDR0_HTTU_ACCESS_DIRTY_HAFT: c_int = 3;

pub const IDR0_TTF_AARCH64: c_int = 2;

pub const ARM_SMMU_IDR1: c_uint = 0x4;

pub const ARM_SMMU_IDR3: c_uint = 0xc;

pub const ARM_SMMU_IDR5: c_uint = 0x14;

pub const IDR5_OAS_32_BIT: c_int = 0;
pub const IDR5_OAS_36_BIT: c_int = 1;
pub const IDR5_OAS_40_BIT: c_int = 2;
pub const IDR5_OAS_42_BIT: c_int = 3;
pub const IDR5_OAS_44_BIT: c_int = 4;
pub const IDR5_OAS_48_BIT: c_int = 5;
pub const IDR5_OAS_52_BIT: c_int = 6;

pub const IDR5_VAX_52_BIT: c_int = 1;
pub const ARM_SMMU_IIDR: c_uint = 0x18;

pub const ARM_SMMU_AIDR: c_uint = 0x1C;
pub const ARM_SMMU_CR0: c_uint = 0x20;

pub const ARM_SMMU_CR0ACK: c_uint = 0x24;
pub const ARM_SMMU_CR1: c_uint = 0x28;

// CR1 cacheability fields don't quite follow the usual TCR-style encoding
pub const CR1_CACHE_NC: c_int = 0;
pub const CR1_CACHE_WB: c_int = 1;
pub const CR1_CACHE_WT: c_int = 2;
pub const ARM_SMMU_CR2: c_uint = 0x2c;

pub const ARM_SMMU_GBPA: c_uint = 0x44;

pub const ARM_SMMU_IRQ_CTRL: c_uint = 0x50;

pub const ARM_SMMU_IRQ_CTRLACK: c_uint = 0x54;
pub const ARM_SMMU_GERROR: c_uint = 0x60;

pub const GERROR_ERR_MASK: c_uint = 0x1fd;
pub const ARM_SMMU_GERRORN: c_uint = 0x64;
pub const ARM_SMMU_GERROR_IRQ_CFG0: c_uint = 0x68;
pub const ARM_SMMU_GERROR_IRQ_CFG1: c_uint = 0x70;
pub const ARM_SMMU_GERROR_IRQ_CFG2: c_uint = 0x74;
pub const ARM_SMMU_STRTAB_BASE: c_uint = 0x80;

pub const ARM_SMMU_STRTAB_BASE_CFG: c_uint = 0x88;

pub const STRTAB_BASE_CFG_FMT_LINEAR: c_int = 0;
pub const STRTAB_BASE_CFG_FMT_2LVL: c_int = 1;

pub const ARM_SMMU_CMDQ_BASE: c_uint = 0x90;
pub const ARM_SMMU_CMDQ_PROD: c_uint = 0x98;
pub const ARM_SMMU_CMDQ_CONS: c_uint = 0x9c;
pub const ARM_SMMU_EVTQ_BASE: c_uint = 0xa0;
pub const ARM_SMMU_EVTQ_PROD: c_uint = 0xa8;
pub const ARM_SMMU_EVTQ_CONS: c_uint = 0xac;
pub const ARM_SMMU_EVTQ_IRQ_CFG0: c_uint = 0xb0;
pub const ARM_SMMU_EVTQ_IRQ_CFG1: c_uint = 0xb8;
pub const ARM_SMMU_EVTQ_IRQ_CFG2: c_uint = 0xbc;
pub const ARM_SMMU_PRIQ_BASE: c_uint = 0xc0;
pub const ARM_SMMU_PRIQ_PROD: c_uint = 0xc8;
pub const ARM_SMMU_PRIQ_CONS: c_uint = 0xcc;
pub const ARM_SMMU_PRIQ_IRQ_CFG0: c_uint = 0xd0;
pub const ARM_SMMU_PRIQ_IRQ_CFG1: c_uint = 0xd8;
pub const ARM_SMMU_PRIQ_IRQ_CFG2: c_uint = 0xdc;
pub const ARM_SMMU_REG_SZ: c_uint = 0xe00;
// Common MSI config fields

// Common memory attribute values
pub const ARM_SMMU_SH_NSH: c_int = 0;
pub const ARM_SMMU_SH_OSH: c_int = 2;
pub const ARM_SMMU_SH_ISH: c_int = 3;
pub const ARM_SMMU_MEMATTR_DEVICE_nGnRE: c_uint = 0x1;
pub const ARM_SMMU_MEMATTR_OIWB: c_uint = 0xf;

// Ensure DMA allocations are naturally aligned

//
// Stream table.
//
// Linear: Enough to cover 1 << IDR1.SIDSIZE entries
// 2lvl: 128k L1 entries,
// 256 lazy entries per table (each table covers a PCI bus)
//
pub const STRTAB_SPLIT: c_int = 8;

pub const STRTAB_STE_DWORDS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_ste {
    pub data: [__le64; STRTAB_STE_DWORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_strtab_l2 {
    pub stes: [arm_smmu_ste; STRTAB_NUM_L2_STES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_strtab_l1 {
    pub l2ptr: __le64,
}

pub const STRTAB_STE_0_CFG_ABORT: c_int = 0;
pub const STRTAB_STE_0_CFG_BYPASS: c_int = 4;
pub const STRTAB_STE_0_CFG_S1_TRANS: c_int = 5;
pub const STRTAB_STE_0_CFG_S2_TRANS: c_int = 6;
pub const STRTAB_STE_0_CFG_NESTED: c_int = 7;

pub const STRTAB_STE_0_S1FMT_LINEAR: c_int = 0;
pub const STRTAB_STE_0_S1FMT_64K_L2: c_int = 2;

pub const STRTAB_STE_1_S1DSS_TERMINATE: c_uint = 0x0;
pub const STRTAB_STE_1_S1DSS_BYPASS: c_uint = 0x1;
pub const STRTAB_STE_1_S1DSS_SSID0: c_uint = 0x2;

// These bits can be controlled by userspace for STRTAB_STE_0_CFG_NESTED

//
// Context descriptors.
//
// Linear: when less than 1024 SSIDs are supported
// 2lvl: at most 1024 L1 entries,
// 1024 lazy entries per table.
//
pub const CTXDESC_L2_ENTRIES: c_int = 1024;

pub const CTXDESC_CD_DWORDS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cd {
    pub data: [__le64; CTXDESC_CD_DWORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cdtab_l2 {
    pub cds: [arm_smmu_cd; CTXDESC_L2_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cdtab_l1 {
    pub l2ptr: __le64,
}

//
// When the SMMU only supports linear context descriptor tables, pick a
// reasonable size limit (64kB).
//

// Command queue
pub const CMDQ_ENT_SZ_SHIFT: c_int = 4;

pub const CMDQ_ERR_CERROR_NONE_IDX: c_int = 0;
pub const CMDQ_ERR_CERROR_ILL_IDX: c_int = 1;
pub const CMDQ_ERR_CERROR_ABT_IDX: c_int = 2;
pub const CMDQ_ERR_CERROR_ATC_INV_IDX: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cmd {
    pub data: [u64; CMDQ_ENT_DWORDS],
}

//
// This is used to size the command queue and therefore must be at least
// BITS_PER_LONG so that the valid_map works correctly (it relies on the
// total number of queue entries being a multiple of BITS_PER_LONG).
//

pub const CMDQ_TLBI_RANGE_NUM_MAX: c_int = 31;

pub const ATC_INV_SIZE_ALL: c_int = 52;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pri_resp {
    PRI_RESP_DENY = 0,
    PRI_RESP_FAIL = 1,
    PRI_RESP_SUCC = 2,
}

pub const CMDQ_SYNC_0_CS_NONE: c_int = 0;
pub const CMDQ_SYNC_0_CS_IRQ: c_int = 1;
pub const CMDQ_SYNC_0_CS_SEV: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_cmdq_opcode {
    CMDQ_OP_PREFETCH_CFG = 0x1,
    CMDQ_OP_CFGI_STE = 0x3,
    CMDQ_OP_CFGI_ALL = 0x4,
    CMDQ_OP_CFGI_CD = 0x5,
    CMDQ_OP_CFGI_CD_ALL = 0x6,
    CMDQ_OP_TLBI_NH_ALL = 0x10,
    CMDQ_OP_TLBI_NH_ASID = 0x11,
    CMDQ_OP_TLBI_NH_VA = 0x12,
    CMDQ_OP_TLBI_NH_VAA = 0x13,
    CMDQ_OP_TLBI_EL2_ALL = 0x20,
    CMDQ_OP_TLBI_EL2_ASID = 0x21,
    CMDQ_OP_TLBI_EL2_VA = 0x22,
    CMDQ_OP_TLBI_S12_VMALL = 0x28,
    CMDQ_OP_TLBI_S2_IPA = 0x2a,
    CMDQ_OP_TLBI_NSNH_ALL = 0x30,
    CMDQ_OP_ATC_INV = 0x40,
    CMDQ_OP_PRI_RESP = 0x41,
    CMDQ_OP_RESUME = 0x44,
    CMDQ_OP_CMD_SYNC = 0x46,
}

extern "C" {
    pub fn arm_smmu_make_cmd_atc_inv(_arg: sid, _arg: ssid, _arg: 0, _arg: ATC_INV_SIZE_ALL) -> return;
}
//
// TLBI commands - the non-sized variants just need opcode + asid/vmid.
// For sized variants the caller sets up data[0] with the immutable fields
// (opcode + asid/vmid) and the range loop fills in per-iteration fields.
//
// Event queue
pub const EVTQ_ENT_SZ_SHIFT: c_int = 5;

pub const EVT_ID_BAD_STREAMID_CONFIG: c_uint = 0x02;
pub const EVT_ID_STE_FETCH_FAULT: c_uint = 0x03;
pub const EVT_ID_BAD_STE_CONFIG: c_uint = 0x04;
pub const EVT_ID_STREAM_DISABLED_FAULT: c_uint = 0x06;
pub const EVT_ID_BAD_SUBSTREAMID_CONFIG: c_uint = 0x08;
pub const EVT_ID_CD_FETCH_FAULT: c_uint = 0x09;
pub const EVT_ID_BAD_CD_CONFIG: c_uint = 0x0a;
pub const EVT_ID_TRANSLATION_FAULT: c_uint = 0x10;
pub const EVT_ID_ADDR_SIZE_FAULT: c_uint = 0x11;
pub const EVT_ID_ACCESS_FAULT: c_uint = 0x12;
pub const EVT_ID_PERMISSION_FAULT: c_uint = 0x13;
pub const EVT_ID_VMS_FETCH_FAULT: c_uint = 0x25;

pub const EVTQ_1_CLASS_TT: c_uint = 0x01;

// PRI queue
pub const PRIQ_ENT_SZ_SHIFT: c_int = 4;

// High-level queue structures

pub const ARM_SMMU_POLL_SPIN_COUNT: c_int = 10;
pub const MSI_IOVA_BASE: c_uint = 0x8000000;
pub const MSI_IOVA_LENGTH: c_uint = 0x100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_ll_queue {
    pub val: u64,
    pub prod: u32,
    pub cons: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_queue {
    pub llq: arm_smmu_ll_queue,
    pub /: *mut *mut int irq; / Wired interrupt,
    pub base: *mut __le64,
    pub base_dma: dma_addr_t,
    pub q_base: u64,
    pub ent_dwords: usize,
    pub prod_reg: *mut u32 __iomem,
    pub cons_reg: *mut u32 __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_queue_poll {
    pub timeout: ktime_t,
    pub delay: c_uint,
    pub spin_cnt: c_uint,
    pub wfe: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cmdq {
    pub q: arm_smmu_queue,
    pub valid_map: *mut atomic_long_t,
    pub owner_prod: core::sync::atomic::AtomicI32,
    pub lock: core::sync::atomic::AtomicI32,
    pub cmd): *mut *mut bool (supports_cmd)(struct arm_smmu_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cmdq_batch {
    pub cmds: [arm_smmu_cmd; CMDQ_BATCH_ENTRIES],
    pub cmdq: *mut arm_smmu_cmdq,
    pub num: c_int,
}

//
// The order here also determines the sequence in which commands are sent to the
// command queue. E.g. TLBI must be done before ATC_INV.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_inv_type {
    INV_TYPE_S1_ASID,
    INV_TYPE_S2_VMID,
    INV_TYPE_S2_VMID_S1_CLEAR,
    INV_TYPE_ATS,
    INV_TYPE_ATS_FULL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_inv {
    pub smmu: *mut arm_smmu_device,
    pub type: u8,
    pub size_opcode: u8,
    pub nsize_opcode: u8,
    pub /: *mut *mut u32 id; / ASID or VMID or SID,
    pub /: *mut *mut size_t pgsize; / ARM_SMMU_FEAT_RANGE_INV,
    pub /: *mut *mut u32 ssid; / INV_TYPE_ATS,
}

//
// struct arm_smmu_invs - Per-domain invalidation array
// @max_invs: maximum capacity of the flexible array
// @num_invs: number of invalidations in the flexible array. May be smaller than
// @max_invs after a tailing trash entry is excluded, but must not be
// greater than @max_invs
// @num_trashes: number of trash entries in the array for arm_smmu_invs_purge().
// Must not be greater than @num_invs
// @rwlock: optional rwlock to fence ATS operations
// @has_ats: flag if the array contains an INV_TYPE_ATS or INV_TYPE_ATS_FULL
// @rcu: rcu head for kfree_rcu()
// @inv: flexible invalidation array
//
// The arm_smmu_invs is an RCU data structure. During a ->attach_dev callback,
// arm_smmu_invs_merge(), arm_smmu_invs_unref() and arm_smmu_invs_purge() will
// be used to allocate a new copy of an old array for addition and deletion in
// the old domain's and new domain's invs arrays.
//
// The arm_smmu_invs_unref() mutates a given array, by internally reducing the
// users counts of some given entries. This exists to support a no-fail routine
// like attaching to an IOMMU_DOMAIN_BLOCKED. And it could pair with a followup
// arm_smmu_invs_purge() call to generate a new clean array.
//
// Concurrent invalidation thread will push every invalidation described in the
// array into the command queue for each invalidation event. It is designed like
// this to optimize the invalidation fast path by avoiding locks.
//
// A domain can be shared across SMMU instances. When an instance gets removed,
// it would delete all the entries that belong to that SMMU instance. Then, a
// synchronize_rcu() would have to be called to sync the array, to prevent any
// concurrent invalidation thread accessing the old array from issuing commands
// to the command queue of a removed SMMU instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_invs {
    pub max_invs: usize,
    pub num_invs: usize,
    pub num_trashes: usize,
    pub rwlock: rwlock_t,
    pub has_ats: bool,
    pub rcu: rcu_head,
    pub __counted_by(max_invs): arm_smmu_inv inv[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_evtq {
    pub q: arm_smmu_queue,
    pub iopf: *mut iopf_queue,
    pub max_stalls: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_priq {
    pub q: arm_smmu_queue,
}

// High-level stream table and context descriptor structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_ctx_desc {
    pub asid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_ctx_desc_cfg {
    pub table: *mut arm_smmu_cd,
    pub num_ents: c_uint,
    pub linear: },
    pub l1tab: *mut arm_smmu_cdtab_l1,
    pub l2ptrs: *mut arm_smmu_cdtab_l2,
    pub num_l1_ents: c_uint,
    pub l2: },
}

// log2 of the maximum number of CDs supported by this table
// True if the cd table has SSIDS > 0 in use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_s2_cfg {
    pub vmid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_strtab_cfg {
    pub table: *mut arm_smmu_ste,
    pub ste_dma: dma_addr_t,
    pub num_ents: c_uint,
    pub linear: },
    pub l1tab: *mut arm_smmu_strtab_l1,
    pub l2ptrs: *mut arm_smmu_strtab_l2,
    pub l1_dma: dma_addr_t,
    pub num_l1_ents: c_uint,
    pub l2: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_impl_ops {
    pub smmu): *mut *mut int (device_reset)(struct arm_smmu_device,
    pub smmu): *mut *mut void (device_disable)(struct arm_smmu_device,
    pub smmu): *mut *mut void (device_remove)(struct arm_smmu_device,
    pub smmu): *mut *mut int (init_structures)(struct arm_smmu_device,
    pub cmd): *mut *mut arm_smmu_device smmu, arm_smmu_cmd,
//
// An implementation should define its own type other than the default
// IOMMU_HW_INFO_TYPE_ARM_SMMUV3. And it must validate the input @type
// to return its own structure.
//
    pub type): *mut iommu_hw_info_type,
    pub viommu_type): *mut *mut size_t (get_viommu_size)(enum iommu_viommu_type,
    pub user_data): *const iommu_user_data,
}

// An SMMUv3 instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_device {
    pub dev: *mut device,
    pub impl_dev: *mut device,
    pub impl_ops: *const arm_smmu_impl_ops,
    pub base: *mut void __iomem,
    pub page1: *mut void __iomem,

    pub features: u32,

    pub options: u32,
    pub cmdq: arm_smmu_cmdq,
    pub evtq: arm_smmu_evtq,
    pub priq: arm_smmu_priq,
    pub gerr_irq: c_int,
    pub combined_irq: c_int,
    pub /: *mut *mut unsigned long oas; / PA,
    pub pgsize_bitmap: c_ulong,

    pub asid_bits: c_uint,

    pub vmid_bits: c_uint,
    pub vmid_map: ida,
    pub ssid_bits: c_uint,
    pub sid_bits: c_uint,
    pub strtab_cfg: arm_smmu_strtab_cfg,
// IOMMU core code handle
    pub iommu: iommu_device,
    pub streams: rb_root,
    pub streams_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_stream {
    pub id: u32,
    pub master: *mut arm_smmu_master,
    pub node: rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_vmaster {
    pub vsmmu: *mut arm_vsmmu,
    pub vsid: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_event {
    pub 1: class_tt :,
    pub id: u8,
    pub class: u8,
    pub stag: u16,
    pub sid: u32,
    pub ssid: u32,
    pub iova: u64,
    pub ipa: u64,
    pub fetch_addr: u64,
    pub dev: *mut device,
}

// SMMU private data for each master
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_master {
    pub smmu: *mut arm_smmu_device,
    pub dev: *mut device,
    pub streams: *mut arm_smmu_stream,
//
// Scratch memory for a to_merge or to_unref array to build a per-domain
// invalidation array. It'll be pre-allocated with enough enries for all
// possible build scenarios. It can be used by only one caller at a time
// until the arm_smmu_invs_merge/unref() finishes. Must be locked by the
// iommu_group mutex.
//
    pub build_invs: *mut arm_smmu_invs,
    pub /: *mut *mut *mut arm_smmu_vmaster vmaster; / use smmu->streams_mutex,
// Locked by the iommu core using the group mutex
    pub cd_table: arm_smmu_ctx_desc_cfg,
    pub num_streams: c_uint,
    pub 1: bool ats_enabled :,
    pub 1: bool ste_ats_enabled :,
    pub stall_enabled: bool,
    pub ats_always_on: bool,
    pub ssid_bits: c_uint,
    pub iopf_refcount: c_uint,
}

// SMMU private data for an IOMMU domain
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_domain_stage {
    ARM_SMMU_DOMAIN_S1 = 0,
    ARM_SMMU_DOMAIN_S2,
    ARM_SMMU_DOMAIN_SVA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_domain {
    pub smmu: *mut arm_smmu_device,
    pub pgtbl_ops: *mut io_pgtable_ops,
    pub nr_ats_masters: core::sync::atomic::AtomicI32,
    pub stage: arm_smmu_domain_stage,
    pub cd: arm_smmu_ctx_desc,
    pub s2_cfg: arm_smmu_s2_cfg,
}

// List of struct arm_smmu_master_domain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_nested_domain {
    pub domain: iommu_domain,
    pub vsmmu: *mut arm_vsmmu,
    pub 1: bool enable_ats :,
    pub ste: [__le64; 2],
}

// The following are exposed for testing purposes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_entry_writer {
    pub ops: *const arm_smmu_entry_writer_ops,
    pub master: *mut arm_smmu_master,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_entry_writer_ops {
    pub used): *const *const *const void (get_used)(__le64 entry, __le64,
    pub safe_bits): *mut __le64,
    pub writer): *mut *mut void (sync)(struct arm_smmu_entry_writer,
}

extern "C" {
    pub fn arm_smmu_make_abort_ste(target: *mut arm_smmu_ste);
}

extern "C" {
    pub fn arm_smmu_get_ste_used(ent: *const __le64, used_bits: *mut __le64);
}
extern "C" {
    pub fn arm_smmu_get_cd_used(ent: *const __le64, used_bits: *mut __le64);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_master_domain {
    pub devices_elm: list_head,
    pub master: *mut arm_smmu_master,
//
// For nested domains the master_domain is threaded onto the S2 parent,
// this points to the IOMMU_DOMAIN_NESTED to disambiguate the masters.
//
    pub domain: *mut iommu_domain,
    pub ssid: ioasid_t,
    pub 1: bool nested_ats_flush :,
    pub 1: bool using_iopf :,
}

extern "C" {
    pub fn container_of(_arg: dom, arm_smmu_domain: struct, _arg: domain) -> return;
}
extern "C" {
    pub fn container_of(_arg: dom, arm_smmu_nested_domain: struct, _arg: domain) -> return;
}
// No concurrency with invalidation is possible at this point
extern "C" {
    pub fn arm_smmu_clear_cd(master: *mut arm_smmu_master, ssid: ioasid_t);
}
//
// struct arm_smmu_inv_state - Per-domain invalidation array state
// @invs_ptr: points to the domain->invs (unwinding nesting/etc.) or is NULL if
// no change should be made
// @old_invs: the original invs array
// @new_invs: for new domain, this is the new invs array to update domain->invs;
// for old domain, this is the master->build_invs to pass in as the
// to_unref argument to an arm_smmu_invs_unref() call
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_inv_state {
    pub invs_ptr: *mut arm_smmu_invs __rcu,
    pub old_invs: *mut arm_smmu_invs,
    pub new_invs: *mut arm_smmu_invs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_attach_state {
// Inputs
    pub old_domain: *mut iommu_domain,
    pub master: *mut arm_smmu_master,
    pub cd_needs_ats: bool,
    pub disable_ats: bool,
    pub ssid: ioasid_t,
// Resulting state
    pub vmaster: *mut arm_smmu_vmaster,
    pub old_domain_invst: arm_smmu_inv_state,
    pub new_domain_invst: arm_smmu_inv_state,
    pub ats_enabled: bool,
}

extern "C" {
    pub fn arm_smmu_attach_commit(state: *mut arm_smmu_attach_state);
}
extern "C" {
    pub fn arm_smmu_erratum_repeat_tlbi_cfgi() -> bool;
}

extern "C" {
    pub fn arm_smmu_sva_supported(smmu: *mut arm_smmu_device) -> bool;
}
extern "C" {
    pub fn arm_smmu_sva_notifier_synchronize();
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_vsmmu {
    pub core: iommufd_viommu,
    pub smmu: *mut arm_smmu_device,
    pub s2_parent: *mut arm_smmu_domain,
    pub vmid: u16,
}

extern "C" {
    pub fn arm_smmu_attach_commit_vmaster(state: *mut arm_smmu_attach_state);
}
extern "C" {
    pub fn arm_smmu_master_clear_vmaster(master: *mut arm_smmu_master);
}
extern "C" {
    pub fn arm_vmaster_report_event(vmaster: *mut arm_smmu_vmaster, evt: *mut u64) -> c_int;
}

