//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/arm-gic-v3.h
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
// Copyright (C) 2013, 2014 ARM Limited, All Rights Reserved.
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// Distributor registers. We assume we're running non-secure, with ARE
// being set. Secure-only and non-ARE registers are not described.
//
pub const GICD_CTLR: c_uint = 0x0000;
pub const GICD_TYPER: c_uint = 0x0004;
pub const GICD_IIDR: c_uint = 0x0008;
pub const GICD_TYPER2: c_uint = 0x000C;
pub const GICD_STATUSR: c_uint = 0x0010;
pub const GICD_SETSPI_NSR: c_uint = 0x0040;
pub const GICD_CLRSPI_NSR: c_uint = 0x0048;
pub const GICD_SETSPI_SR: c_uint = 0x0050;
pub const GICD_CLRSPI_SR: c_uint = 0x0058;
pub const GICD_IGROUPR: c_uint = 0x0080;
pub const GICD_ISENABLER: c_uint = 0x0100;
pub const GICD_ICENABLER: c_uint = 0x0180;
pub const GICD_ISPENDR: c_uint = 0x0200;
pub const GICD_ICPENDR: c_uint = 0x0280;
pub const GICD_ISACTIVER: c_uint = 0x0300;
pub const GICD_ICACTIVER: c_uint = 0x0380;
pub const GICD_IPRIORITYR: c_uint = 0x0400;
pub const GICD_ICFGR: c_uint = 0x0C00;
pub const GICD_IGRPMODR: c_uint = 0x0D00;
pub const GICD_NSACR: c_uint = 0x0E00;
pub const GICD_IGROUPRnE: c_uint = 0x1000;
pub const GICD_ISENABLERnE: c_uint = 0x1200;
pub const GICD_ICENABLERnE: c_uint = 0x1400;
pub const GICD_ISPENDRnE: c_uint = 0x1600;
pub const GICD_ICPENDRnE: c_uint = 0x1800;
pub const GICD_ISACTIVERnE: c_uint = 0x1A00;
pub const GICD_ICACTIVERnE: c_uint = 0x1C00;
pub const GICD_IPRIORITYRnE: c_uint = 0x2000;
pub const GICD_ICFGRnE: c_uint = 0x3000;
pub const GICD_IROUTER: c_uint = 0x6000;
pub const GICD_IROUTERnE: c_uint = 0x8000;
pub const GICD_IDREGS: c_uint = 0xFFD0;
pub const GICD_PIDR2: c_uint = 0xFFE8;
pub const ESPI_BASE_INTID: c_int = 4096;
//
// Those registers are actually from GICv2, but the spec demands that they
// are implemented as RES0 if ARE is 1 (which we do in KVM's emulated GICv3).
//
pub const GICD_ITARGETSR: c_uint = 0x0800;
pub const GICD_SGIR: c_uint = 0x0F00;
pub const GICD_CPENDSGIR: c_uint = 0x0F10;
pub const GICD_SPENDSGIR: c_uint = 0x0F20;

pub const GICD_IIDR_IMPLEMENTER_SHIFT: c_int = 0;

pub const GICD_IIDR_REVISION_SHIFT: c_int = 12;

pub const GICD_IIDR_VARIANT_SHIFT: c_int = 16;

pub const GICD_IIDR_PRODUCT_ID_SHIFT: c_int = 24;

//
// In systems with a single security state (what we emulate in KVM)
// the meaning of the interrupt group enable bits is slightly different
//

pub const GIC_PIDR2_ARCH_MASK: c_uint = 0xf0;
pub const GIC_PIDR2_ARCH_GICv3: c_uint = 0x30;
pub const GIC_PIDR2_ARCH_GICv4: c_uint = 0x40;
pub const GIC_V3_DIST_SIZE: c_uint = 0x10000;

//
// Re-Distributor registers, offsets from RD_base
//

pub const GICR_IIDR: c_uint = 0x0004;
pub const GICR_TYPER: c_uint = 0x0008;

pub const GICR_WAKER: c_uint = 0x0014;
pub const GICR_SETLPIR: c_uint = 0x0040;
pub const GICR_CLRLPIR: c_uint = 0x0048;
pub const GICR_PROPBASER: c_uint = 0x0070;
pub const GICR_PENDBASER: c_uint = 0x0078;
pub const GICR_INVLPIR: c_uint = 0x00A0;
pub const GICR_INVALLR: c_uint = 0x00B0;
pub const GICR_SYNCR: c_uint = 0x00C0;

pub const EPPI_BASE_INTID: c_int = 1056;

// encode a size field of width @w containing @n - 1 units

//
// Re-Distributor registers, offsets from SGI_base
//

pub const GIC_V3_REDIST_SIZE: c_uint = 0x20000;

//
// Re-Distributor registers, offsets from VLPI_base
//
pub const GICR_VPROPBASER: c_uint = 0x0070;
pub const GICR_VPROPBASER_IDBITS_MASK: c_uint = 0x1f;

//
// GICv4.1 VPROPBASER reinvention. A subtle mix between the old
// VPROPBASER and ITS_BASER. Just not quite any of the two.
//

pub const GICR_VPENDBASER: c_uint = 0x0078;

//
// GICv4.1 VPENDBASER, used for VPE residency. On top of these fields,
// also use the above Valid, PendingLast and Dirty.
//

pub const GICR_VSGIR: c_uint = 0x0080;

pub const GICR_VSGIPENDR: c_uint = 0x0088;

//
// ITS registers, offsets from ITS_base
//
pub const GITS_CTLR: c_uint = 0x0000;
pub const GITS_IIDR: c_uint = 0x0004;
pub const GITS_TYPER: c_uint = 0x0008;
pub const GITS_MPIDR: c_uint = 0x0018;
pub const GITS_CBASER: c_uint = 0x0080;
pub const GITS_CWRITER: c_uint = 0x0088;
pub const GITS_CREADR: c_uint = 0x0090;
pub const GITS_BASER: c_uint = 0x0100;
pub const GITS_IDREGS_BASE: c_uint = 0xffd0;
pub const GITS_PIDR0: c_uint = 0xffe0;
pub const GITS_PIDR1: c_uint = 0xffe4;

pub const GITS_PIDR4: c_uint = 0xffd0;
pub const GITS_CIDR0: c_uint = 0xfff0;
pub const GITS_CIDR1: c_uint = 0xfff4;
pub const GITS_CIDR2: c_uint = 0xfff8;
pub const GITS_CIDR3: c_uint = 0xfffc;
pub const GITS_TRANSLATER: c_uint = 0x10040;
pub const GITS_SGIR: c_uint = 0x20020;

pub const GITS_CTLR_ITS_NUMBER_SHIFT: c_int = 4;

pub const GITS_TYPER_ITT_ENTRY_SIZE_SHIFT: c_int = 4;

pub const GITS_TYPER_IDBITS_SHIFT: c_int = 8;
pub const GITS_TYPER_DEVBITS_SHIFT: c_int = 13;

pub const GITS_TYPER_HCC_SHIFT: c_int = 24;

pub const GITS_IIDR_REV_SHIFT: c_int = 12;

pub const GITS_IIDR_PRODUCTID_SHIFT: c_int = 24;

pub const GITS_BASER_NR_REGS: c_int = 8;

pub const GITS_BASER_PAGES_MAX: c_int = 256;

pub const GITS_BASER_TYPE_NONE: c_int = 0;
pub const GITS_BASER_TYPE_DEVICE: c_int = 1;
pub const GITS_BASER_TYPE_VCPU: c_int = 2;
pub const GITS_BASER_TYPE_RESERVED3: c_int = 3;
pub const GITS_BASER_TYPE_COLLECTION: c_int = 4;
pub const GITS_BASER_TYPE_RESERVED5: c_int = 5;
pub const GITS_BASER_TYPE_RESERVED6: c_int = 6;
pub const GITS_BASER_TYPE_RESERVED7: c_int = 7;

//
// ITS commands
//
pub const GITS_CMD_MAPD: c_uint = 0x08;
pub const GITS_CMD_MAPC: c_uint = 0x09;
pub const GITS_CMD_MAPTI: c_uint = 0x0a;
pub const GITS_CMD_MAPI: c_uint = 0x0b;
pub const GITS_CMD_MOVI: c_uint = 0x01;
pub const GITS_CMD_DISCARD: c_uint = 0x0f;
pub const GITS_CMD_INV: c_uint = 0x0c;
pub const GITS_CMD_MOVALL: c_uint = 0x0e;
pub const GITS_CMD_INVALL: c_uint = 0x0d;
pub const GITS_CMD_INT: c_uint = 0x03;
pub const GITS_CMD_CLEAR: c_uint = 0x04;
pub const GITS_CMD_SYNC: c_uint = 0x05;
//
// GICv4 ITS specific commands
//

// VMOVP, VSGI and INVDB are the odd ones, as they dont have a physical counterpart

//
// ITS error numbers
//
pub const E_ITS_MOVI_UNMAPPED_INTERRUPT: c_uint = 0x010107;
pub const E_ITS_MOVI_UNMAPPED_COLLECTION: c_uint = 0x010109;
pub const E_ITS_INT_UNMAPPED_INTERRUPT: c_uint = 0x010307;
pub const E_ITS_CLEAR_UNMAPPED_INTERRUPT: c_uint = 0x010507;
pub const E_ITS_MAPD_DEVICE_OOR: c_uint = 0x010801;
pub const E_ITS_MAPD_ITTSIZE_OOR: c_uint = 0x010802;
pub const E_ITS_MAPC_PROCNUM_OOR: c_uint = 0x010902;
pub const E_ITS_MAPC_COLLECTION_OOR: c_uint = 0x010903;
pub const E_ITS_MAPTI_UNMAPPED_DEVICE: c_uint = 0x010a04;
pub const E_ITS_MAPTI_ID_OOR: c_uint = 0x010a05;
pub const E_ITS_MAPTI_PHYSICALID_OOR: c_uint = 0x010a06;
pub const E_ITS_INV_UNMAPPED_INTERRUPT: c_uint = 0x010c07;
pub const E_ITS_INVALL_UNMAPPED_COLLECTION: c_uint = 0x010d09;
pub const E_ITS_MOVALL_PROCNUM_OOR: c_uint = 0x010e01;
pub const E_ITS_DISCARD_UNMAPPED_INTERRUPT: c_uint = 0x010f07;
//
// CPU interface registers
//

pub const ICC_CTLR_EL1_CBPR_SHIFT: c_int = 0;

pub const ICC_CTLR_EL1_PMHE_SHIFT: c_int = 6;

pub const ICC_CTLR_EL1_PRI_BITS_SHIFT: c_int = 8;

pub const ICC_CTLR_EL1_ID_BITS_SHIFT: c_int = 11;

pub const ICC_CTLR_EL1_SEIS_SHIFT: c_int = 14;

pub const ICC_CTLR_EL1_A3V_SHIFT: c_int = 15;

pub const ICC_PMR_EL1_SHIFT: c_int = 0;

pub const ICC_BPR0_EL1_SHIFT: c_int = 0;

pub const ICC_BPR1_EL1_SHIFT: c_int = 0;

pub const ICC_IGRPEN0_EL1_SHIFT: c_int = 0;

pub const ICC_IGRPEN1_EL1_SHIFT: c_int = 0;

// These are for GICv2 emulation only

pub const ICC_IAR1_EL1_SPURIOUS: c_uint = 0x3ff;

pub const ICC_SGI1R_TARGET_LIST_SHIFT: c_int = 0;

pub const ICC_SGI1R_AFFINITY_1_SHIFT: c_int = 16;

pub const ICC_SGI1R_SGI_ID_SHIFT: c_int = 24;

pub const ICC_SGI1R_AFFINITY_2_SHIFT: c_int = 32;

pub const ICC_SGI1R_IRQ_ROUTING_MODE_BIT: c_int = 40;
pub const ICC_SGI1R_RS_SHIFT: c_int = 44;

pub const ICC_SGI1R_AFFINITY_3_SHIFT: c_int = 48;

//
// We need a value to serve as a irq-type for LPIs. Choose one that will
// hopefully pique the interest of the reviewer.
//
pub const GIC_IRQ_TYPE_LPI: c_uint = 0xa110c8ed;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdists {
    pub rd_lock: raw_spinlock_t,
    pub rd_base: *mut void __iomem,
    pub pend_page: *mut page,
    pub phys_base: phys_addr_t,
    pub flags: u64,
    pub vpe_table_mask: *mut cpumask_t,
    pub vpe_l1_base: *mut c_void,
    pub rdist: *mut } __percpu,
    pub prop_table_pa: phys_addr_t,
    pub prop_table_va: *mut c_void,
    pub flags: u64,
    pub gicd_typer: u32,
    pub gicd_typer2: u32,
    pub cpuhp_memreserve_state: c_int,
    pub has_vlpis: bool,
    pub has_rvpeid: bool,
    pub has_direct_lpi: bool,
    pub has_vpend_valid_dirty: bool,
}

extern "C" {
    pub fn its_lpi_memreserve_init() -> int __init;
}
extern "C" {
    pub fn its_cpu_init() -> c_int;
}
extern "C" {
    pub fn mbi_init(fwnode: *mut fwnode_handle, parent: *mut irq_domain) -> c_int;
}

