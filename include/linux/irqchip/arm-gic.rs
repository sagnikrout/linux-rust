//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/arm-gic.h
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
// include/linux/irqchip/arm-gic.h
//
// Copyright (C) 2002 ARM Limited, All Rights Reserved.
//
pub const GIC_CPU_CTRL: c_uint = 0x00;
pub const GIC_CPU_PRIMASK: c_uint = 0x04;
pub const GIC_CPU_BINPOINT: c_uint = 0x08;
pub const GIC_CPU_INTACK: c_uint = 0x0c;
pub const GIC_CPU_EOI: c_uint = 0x10;
pub const GIC_CPU_RUNNINGPRI: c_uint = 0x14;
pub const GIC_CPU_HIGHPRI: c_uint = 0x18;
pub const GIC_CPU_ALIAS_BINPOINT: c_uint = 0x1c;
pub const GIC_CPU_ACTIVEPRIO: c_uint = 0xd0;
pub const GIC_CPU_IDENT: c_uint = 0xfc;
pub const GIC_CPU_DEACTIVATE: c_uint = 0x1000;
pub const GICC_ENABLE: c_uint = 0x1;
pub const GICC_INT_PRI_THRESHOLD: c_uint = 0xf0;
pub const GIC_CPU_CTRL_EnableGrp0_SHIFT: c_int = 0;

pub const GIC_CPU_CTRL_EnableGrp1_SHIFT: c_int = 1;

pub const GIC_CPU_CTRL_AckCtl_SHIFT: c_int = 2;

pub const GIC_CPU_CTRL_FIQEn_SHIFT: c_int = 3;

pub const GIC_CPU_CTRL_CBPR_SHIFT: c_int = 4;

pub const GIC_CPU_CTRL_EOImodeNS_SHIFT: c_int = 9;

pub const GICC_IAR_INT_ID_MASK: c_uint = 0x3ff;
pub const GICC_INT_SPURIOUS: c_int = 1023;
pub const GICC_DIS_BYPASS_MASK: c_uint = 0x1e0;
pub const GIC_DIST_CTRL: c_uint = 0x000;
pub const GIC_DIST_CTR: c_uint = 0x004;
pub const GIC_DIST_IIDR: c_uint = 0x008;
pub const GIC_DIST_IGROUP: c_uint = 0x080;
pub const GIC_DIST_ENABLE_SET: c_uint = 0x100;
pub const GIC_DIST_ENABLE_CLEAR: c_uint = 0x180;
pub const GIC_DIST_PENDING_SET: c_uint = 0x200;
pub const GIC_DIST_PENDING_CLEAR: c_uint = 0x280;
pub const GIC_DIST_ACTIVE_SET: c_uint = 0x300;
pub const GIC_DIST_ACTIVE_CLEAR: c_uint = 0x380;
pub const GIC_DIST_PRI: c_uint = 0x400;
pub const GIC_DIST_TARGET: c_uint = 0x800;
pub const GIC_DIST_CONFIG: c_uint = 0xc00;
pub const GIC_DIST_SOFTINT: c_uint = 0xf00;
pub const GIC_DIST_SGI_PENDING_CLEAR: c_uint = 0xf10;
pub const GIC_DIST_SGI_PENDING_SET: c_uint = 0xf20;
pub const GICD_ENABLE: c_uint = 0x1;
pub const GICD_DISABLE: c_uint = 0x0;
pub const GICD_INT_ACTLOW_LVLTRIG: c_uint = 0x0;
pub const GICD_INT_EN_CLR_X32: c_uint = 0xffffffff;
pub const GICD_INT_EN_SET_SGI: c_uint = 0x0000ffff;
pub const GICD_INT_EN_CLR_PPI: c_uint = 0xffff0000;
pub const GICD_IIDR_IMPLEMENTER_SHIFT: c_int = 0;

pub const GICD_IIDR_REVISION_SHIFT: c_int = 12;

pub const GICD_IIDR_VARIANT_SHIFT: c_int = 16;

pub const GICD_IIDR_PRODUCT_ID_SHIFT: c_int = 24;

pub const GICH_HCR: c_uint = 0x0;
pub const GICH_VTR: c_uint = 0x4;
pub const GICH_VMCR: c_uint = 0x8;
pub const GICH_MISR: c_uint = 0x10;
pub const GICH_EISR0: c_uint = 0x20;
pub const GICH_EISR1: c_uint = 0x24;
pub const GICH_ELRSR0: c_uint = 0x30;
pub const GICH_ELRSR1: c_uint = 0x34;
pub const GICH_APR: c_uint = 0xf0;
pub const GICH_LR0: c_uint = 0x100;

pub const GICH_LR_PRIORITY_SHIFT: c_int = 23;

pub const GICH_VMCR_ENABLE_GRP0_SHIFT: c_int = 0;

pub const GICH_VMCR_ENABLE_GRP1_SHIFT: c_int = 1;

pub const GICH_VMCR_ACK_CTL_SHIFT: c_int = 2;

pub const GICH_VMCR_FIQ_EN_SHIFT: c_int = 3;

pub const GICH_VMCR_CBPR_SHIFT: c_int = 4;

pub const GICH_VMCR_EOI_MODE_SHIFT: c_int = 9;

pub const GICH_VMCR_PRIMASK_SHIFT: c_int = 27;

pub const GICH_VMCR_BINPOINT_SHIFT: c_int = 21;

pub const GICH_VMCR_ALIAS_BINPOINT_SHIFT: c_int = 18;

pub const GICV_PMR_PRIORITY_SHIFT: c_int = 3;

extern "C" {
    pub fn gic_cascade_irq(gic_nr: c_uint, irq: c_uint);
}
extern "C" {
    pub fn gic_cpu_if_down(gic_nr: c_uint) -> c_int;
}
extern "C" {
    pub fn gic_cpu_save(gic: *mut gic_chip_data);
}
extern "C" {
    pub fn gic_cpu_restore(gic: *mut gic_chip_data);
}
extern "C" {
    pub fn gic_dist_save(gic: *mut gic_chip_data);
}
extern "C" {
    pub fn gic_dist_restore(gic: *mut gic_chip_data);
}
//
// Subdrivers that need some preparatory work can initialize their
// chips and call this to register their GICs.
//
extern "C" {
    pub fn gic_of_init(node: *mut device_node, parent: *mut device_node) -> c_int;
}
//
// Initialises and registers a non-root or child GIC chip. Memory for
// the gic_chip_data structure is dynamically allocated.
//
extern "C" {
    pub fn gic_of_init_child(dev: *mut device, gic: *mut gic_chip_data, irq: c_int) -> c_int;
}
extern "C" {
    pub fn gic_send_sgi(cpu_id: c_uint, irq: c_uint);
}
extern "C" {
    pub fn gic_get_cpu_id(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn gic_migrate_target(new_cpu_id: c_uint);
}
extern "C" {
    pub fn gic_get_sgir_physaddr() -> c_ulong;
}

