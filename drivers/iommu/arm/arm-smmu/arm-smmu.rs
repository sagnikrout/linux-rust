//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/arm/arm-smmu/arm-smmu.h
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
// IOMMU API for ARM architected SMMU implementations.
//
// Copyright (C) 2013 ARM Limited
//
// Author: Will Deacon <will.deacon@arm.com>
//

// Configuration registers
pub const ARM_SMMU_GR0_sCR0: c_uint = 0x0;

// Auxiliary Configuration register
pub const ARM_SMMU_GR0_sACR: c_uint = 0x10;
// Identification registers
pub const ARM_SMMU_GR0_ID0: c_uint = 0x20;

pub const ARM_SMMU_GR0_ID1: c_uint = 0x24;

pub const ARM_SMMU_GR0_ID2: c_uint = 0x28;

pub const ARM_SMMU_GR0_ID3: c_uint = 0x2c;
pub const ARM_SMMU_GR0_ID4: c_uint = 0x30;
pub const ARM_SMMU_GR0_ID5: c_uint = 0x34;
pub const ARM_SMMU_GR0_ID6: c_uint = 0x38;
pub const ARM_SMMU_GR0_ID7: c_uint = 0x3c;

pub const ARM_SMMU_GR0_sGFSR: c_uint = 0x48;

pub const ARM_SMMU_GR0_sGFSYNR0: c_uint = 0x50;
pub const ARM_SMMU_GR0_sGFSYNR1: c_uint = 0x54;
pub const ARM_SMMU_GR0_sGFSYNR2: c_uint = 0x58;
// Global TLB invalidation
pub const ARM_SMMU_GR0_TLBIVMID: c_uint = 0x64;
pub const ARM_SMMU_GR0_TLBIALLNSNH: c_uint = 0x68;
pub const ARM_SMMU_GR0_TLBIALLH: c_uint = 0x6c;
pub const ARM_SMMU_GR0_sTLBGSYNC: c_uint = 0x70;
pub const ARM_SMMU_GR0_sTLBGSTATUS: c_uint = 0x74;

// Stream mapping registers

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_s2cr_privcfg {
    S2CR_PRIVCFG_DEFAULT,
    S2CR_PRIVCFG_DIPAN,
    S2CR_PRIVCFG_UNPRIV,
    S2CR_PRIVCFG_PRIV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_s2cr_type {
    S2CR_TYPE_TRANS,
    S2CR_TYPE_BYPASS,
    S2CR_TYPE_FAULT,
}

// Context bank attribute registers

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_cbar_type {
    CBAR_TYPE_S2_TRANS,
    CBAR_TYPE_S1_TRANS_S2_BYPASS,
    CBAR_TYPE_S1_TRANS_S2_FAULT,
    CBAR_TYPE_S1_TRANS_S2_TRANS,
}

pub const ARM_SMMU_CBAR_S1_MEMATTR_WB: c_uint = 0xf;

pub const ARM_SMMU_CBAR_S1_BPSHCFG_NSH: c_int = 3;

pub const ARM_SMMU_CB_SCTLR: c_uint = 0x0;

pub const ARM_SMMU_CB_ACTLR: c_uint = 0x4;
pub const ARM_SMMU_GFX_PRR_CFG_LADDR: c_uint = 0x6008;
pub const ARM_SMMU_GFX_PRR_CFG_UADDR: c_uint = 0x600C;
pub const ARM_SMMU_CB_RESUME: c_uint = 0x8;

pub const ARM_SMMU_CB_TCR2: c_uint = 0x10;

pub const ARM_SMMU_TCR2_SEP_UPSTREAM: c_uint = 0x7;

pub const ARM_SMMU_CB_TTBR0: c_uint = 0x20;
pub const ARM_SMMU_CB_TTBR1: c_uint = 0x28;

pub const ARM_SMMU_CB_TCR: c_uint = 0x30;

pub const ARM_SMMU_CB_CONTEXTIDR: c_uint = 0x34;
pub const ARM_SMMU_CB_S1_MAIR0: c_uint = 0x38;
pub const ARM_SMMU_CB_S1_MAIR1: c_uint = 0x3c;
pub const ARM_SMMU_CB_PAR: c_uint = 0x50;

pub const ARM_SMMU_CB_FSR: c_uint = 0x58;

pub const ARM_SMMU_CB_FAR: c_uint = 0x60;
pub const ARM_SMMU_CB_FSYNR0: c_uint = 0x68;

pub const ARM_SMMU_CB_FSYNR1: c_uint = 0x6c;
pub const ARM_SMMU_CB_S1_TLBIVA: c_uint = 0x600;
pub const ARM_SMMU_CB_S1_TLBIASID: c_uint = 0x610;
pub const ARM_SMMU_CB_S1_TLBIVAL: c_uint = 0x620;
pub const ARM_SMMU_CB_S2_TLBIIPAS2: c_uint = 0x630;
pub const ARM_SMMU_CB_S2_TLBIIPAS2L: c_uint = 0x638;
pub const ARM_SMMU_CB_TLBSYNC: c_uint = 0x7f0;
pub const ARM_SMMU_CB_TLBSTATUS: c_uint = 0x7f4;
pub const ARM_SMMU_CB_ATS1PR: c_uint = 0x800;
pub const ARM_SMMU_CB_ATSR: c_uint = 0x8f0;

// Maximum number of context banks per SMMU
pub const ARM_SMMU_MAX_CBS: c_int = 128;

pub const TLB_SPIN_COUNT: c_int = 10;
// Shared driver definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_arch_version {
    ARM_SMMU_V1,
    ARM_SMMU_V1_64K,
    ARM_SMMU_V2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_implementation {
    GENERIC_SMMU,
    ARM_MMU500,
    CAVIUM_SMMUV2,
    QCOM_SMMUV2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_s2cr {
    pub group: *mut iommu_group,
    pub count: c_int,
    pub type: arm_smmu_s2cr_type,
    pub privcfg: arm_smmu_s2cr_privcfg,
    pub cbndx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_smr {
    pub mask: u16,
    pub id: u16,
    pub valid: bool,
    pub pinned: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_device {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub ioaddr: phys_addr_t,
    pub numpage: c_uint,
    pub pgshift: c_uint,

    pub features: u32,
    pub version: arm_smmu_arch_version,
    pub model: arm_smmu_implementation,
    pub impl: *const arm_smmu_impl,
    pub num_context_banks: u32,
    pub num_s2_context_banks: u32,
    pub ARM_SMMU_MAX_CBS): DECLARE_BITMAP(context_map,,
    pub cbs: *mut arm_smmu_cb,
    pub irptndx: core::sync::atomic::AtomicI32,
    pub num_mapping_groups: u32,
    pub streamid_mask: u16,
    pub smr_mask_mask: u16,
    pub smrs: *mut arm_smmu_smr,
    pub s2crs: *mut arm_smmu_s2cr,
    pub stream_map_mutex: mutex,
    pub va_size: c_ulong,
    pub ipa_size: c_ulong,
    pub pa_size: c_ulong,
    pub pgsize_bitmap: c_ulong,
    pub num_context_irqs: c_int,
    pub num_clks: c_int,
    pub irqs: *mut c_uint,
    pub clks: *mut clk_bulk_data,
    pub global_sync_lock: spinlock_t,
// IOMMU core code handle
    pub iommu: iommu_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_context_fmt {
    ARM_SMMU_CTX_FMT_NONE,
    ARM_SMMU_CTX_FMT_AARCH64,
    ARM_SMMU_CTX_FMT_AARCH32_L,
    ARM_SMMU_CTX_FMT_AARCH32_S,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cfg {
    pub cbndx: u8,
    pub irptndx: u8,
    pub asid: u16,
    pub vmid: u16,
}

pub const ARM_SMMU_INVALID_IRPTNDX: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_cb {
    pub ttbr: [u64; 2],
    pub tcr: [u32; 2],
    pub mair: [u32; 2],
    pub cfg: *mut arm_smmu_cfg,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smmu_domain_stage {
    ARM_SMMU_DOMAIN_S1 = 0,
    ARM_SMMU_DOMAIN_S2,
    ARM_SMMU_DOMAIN_NESTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_domain {
    pub smmu: *mut arm_smmu_device,
    pub pgtbl_ops: *mut io_pgtable_ops,
    pub pgtbl_quirks: c_ulong,
    pub flush_ops: *const iommu_flush_ops,
    pub cfg: arm_smmu_cfg,
    pub stage: arm_smmu_domain_stage,
    pub /: *mut *mut mutex init_mutex; / Protects smmu pointer,
    pub /: *mut *mut *mut spinlock_t cb_lock; / Serialises ATS1 ops and TLB syncs,
    pub domain: iommu_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_master_cfg {
    pub smmu: *mut arm_smmu_device,
    pub smendx: [i16; ],
}

//
// When TTBR1 is selected shift the TCR fields by 16 bits and disable
// translation in TTBR0
//
// Implementation details, yay!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_impl {
    pub offset): *mut *mut *mut u32 (read_reg)(struct arm_smmu_device smmu, int page, int,
    pub val): u32,
    pub offset): *mut *mut *mut u64 (read_reg64)(struct arm_smmu_device smmu, int page, int,
    pub val): u64,
    pub smmu): *mut *mut int (cfg_probe)(struct arm_smmu_device,
    pub smmu): *mut *mut int (reset)(struct arm_smmu_device,
    pub dev): *mut *mut io_pgtable_cfg cfg, device,
    pub status): c_int,
    pub dev): *mut *mut int (def_domain_type)(struct device,
    pub dev): *mut *mut irqreturn_t (global_fault)(int irq, void,
    pub dev): *mut *mut irqreturn_t (context_fault)(int irq, void,
    pub context_fault_needs_threaded_irq: bool,
    pub start): *mut *mut device dev, int,
    pub idx): *mut *mut *mut void (write_s2cr)(struct arm_smmu_device smmu, int,
    pub reg): *mut *mut *mut void (write_sctlr)(struct arm_smmu_device smmu, int idx, u32,
    pub dev): *mut *mut *mut void (probe_finalize)(struct arm_smmu_device smmu, struct device,
}

extern "C" {
    pub fn readl_relaxed(_arg: arm_smmu_page(smmu, offset: page) +) -> return;
}
extern "C" {
    pub fn readq_relaxed(_arg: arm_smmu_page(smmu, offset: page) +) -> return;
}
pub const ARM_SMMU_GR0: c_int = 0;
pub const ARM_SMMU_GR1: c_int = 1;

extern "C" {
    pub fn arm_smmu_impl_module_init() -> int __init;
}
extern "C" {
    pub fn arm_smmu_impl_module_exit() -> void __exit;
}
extern "C" {
    pub fn qcom_smmu_module_init() -> int __init;
}
extern "C" {
    pub fn qcom_smmu_module_exit() -> void __exit;
}
extern "C" {
    pub fn arm_smmu_write_context_bank(smmu: *mut arm_smmu_device, idx: c_int);
}
extern "C" {
    pub fn arm_mmu500_reset(smmu: *mut arm_smmu_device) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smmu_context_fault_info {
    pub iova: c_ulong,
    pub fsr: u32,
    pub fsynr: u32,
    pub cbfrsynra: u32,
}
