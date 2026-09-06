//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/arm-gic-v5.h
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
// Copyright (C) 2025 ARM Limited, All Rights Reserved.
//

//
// INTID handling
//

//
// Architected PPIs
//
pub const GICV5_ARCH_PPI_S_DB_PPI: c_uint = 0x0;
pub const GICV5_ARCH_PPI_RL_DB_PPI: c_uint = 0x1;
pub const GICV5_ARCH_PPI_NS_DB_PPI: c_uint = 0x2;
pub const GICV5_ARCH_PPI_SW_PPI: c_uint = 0x3;
pub const GICV5_ARCH_PPI_HACDBSIRQ: c_uint = 0xf;
pub const GICV5_ARCH_PPI_CNTHVS: c_uint = 0x13;
pub const GICV5_ARCH_PPI_CNTHPS: c_uint = 0x14;
pub const GICV5_ARCH_PPI_PMBIRQ: c_uint = 0x15;
pub const GICV5_ARCH_PPI_COMMIRQ: c_uint = 0x16;
pub const GICV5_ARCH_PPI_PMUIRQ: c_uint = 0x17;
pub const GICV5_ARCH_PPI_CTIIRQ: c_uint = 0x18;
pub const GICV5_ARCH_PPI_GICMNT: c_uint = 0x19;
pub const GICV5_ARCH_PPI_CNTHP: c_uint = 0x1a;
pub const GICV5_ARCH_PPI_CNTV: c_uint = 0x1b;
pub const GICV5_ARCH_PPI_CNTHV: c_uint = 0x1c;
pub const GICV5_ARCH_PPI_CNTPS: c_uint = 0x1d;
pub const GICV5_ARCH_PPI_CNTP: c_uint = 0x1e;
pub const GICV5_ARCH_PPI_TRBIRQ: c_uint = 0x1f;
//
// Tables attributes
//

//
// IRS registers and tables structures
//
pub const GICV5_IRS_IDR0: c_uint = 0x0000;
pub const GICV5_IRS_IDR1: c_uint = 0x0004;
pub const GICV5_IRS_IDR2: c_uint = 0x0008;
pub const GICV5_IRS_IDR5: c_uint = 0x0014;
pub const GICV5_IRS_IDR6: c_uint = 0x0018;
pub const GICV5_IRS_IDR7: c_uint = 0x001c;
pub const GICV5_IRS_CR0: c_uint = 0x0080;
pub const GICV5_IRS_CR1: c_uint = 0x0084;
pub const GICV5_IRS_SYNCR: c_uint = 0x00c0;
pub const GICV5_IRS_SYNC_STATUSR: c_uint = 0x00c4;
pub const GICV5_IRS_SPI_SELR: c_uint = 0x0108;
pub const GICV5_IRS_SPI_CFGR: c_uint = 0x0114;
pub const GICV5_IRS_SPI_STATUSR: c_uint = 0x0118;
pub const GICV5_IRS_PE_SELR: c_uint = 0x0140;
pub const GICV5_IRS_PE_STATUSR: c_uint = 0x0144;
pub const GICV5_IRS_PE_CR0: c_uint = 0x0148;
pub const GICV5_IRS_IST_BASER: c_uint = 0x0180;
pub const GICV5_IRS_IST_CFGR: c_uint = 0x0190;
pub const GICV5_IRS_IST_STATUSR: c_uint = 0x0194;
pub const GICV5_IRS_MAP_L2_ISTR: c_uint = 0x01c0;

//
// ITS registers and tables structures
//
pub const GICV5_ITS_IDR1: c_uint = 0x0004;
pub const GICV5_ITS_IDR2: c_uint = 0x0008;
pub const GICV5_ITS_CR0: c_uint = 0x0080;
pub const GICV5_ITS_CR1: c_uint = 0x0084;
pub const GICV5_ITS_DT_BASER: c_uint = 0x00c0;
pub const GICV5_ITS_DT_CFGR: c_uint = 0x00d0;
pub const GICV5_ITS_DIDR: c_uint = 0x0100;
pub const GICV5_ITS_EIDR: c_uint = 0x0108;
pub const GICV5_ITS_INV_EVENTR: c_uint = 0x010c;
pub const GICV5_ITS_INV_DEVICER: c_uint = 0x0110;
pub const GICV5_ITS_STATUSR: c_uint = 0x0120;
pub const GICV5_ITS_SYNCR: c_uint = 0x0140;
pub const GICV5_ITS_SYNC_STATUSR: c_uint = 0x0148;

// Note that there is no shift for the address by design

// Note that there is no shift for the address by design

// Note that there is no shift for the address by design

pub const GICV5_ITS_DT_ITT_CFGR_STRUCTURE_LINEAR: c_int = 0;
pub const GICV5_ITS_DT_ITT_CFGR_STRUCTURE_TWO_LEVEL: c_int = 1;

//
// IWB registers
//
pub const GICV5_IWB_IDR0: c_uint = 0x0000;
pub const GICV5_IWB_CR0: c_uint = 0x0080;
pub const GICV5_IWB_WENABLE_STATUSR: c_uint = 0x00c0;
pub const GICV5_IWB_WENABLER: c_uint = 0x2000;
pub const GICV5_IWB_WTMR: c_uint = 0x4000;

pub const GICV5_GSI_IWB_TYPE: c_uint = 0x7;

//
// Global Data structures and functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gicv5_chip_data {
    pub fwnode: *mut fwnode_handle,
    pub ppi_domain: *mut irq_domain,
    pub spi_domain: *mut irq_domain,
    pub lpi_domain: *mut irq_domain,
    pub ipi_domain: *mut irq_domain,
    pub global_spi_count: u32,
    pub cpuif_pri_bits: u8,
    pub cpuif_id_bits: u8,
    pub irs_pri_bits: u8,
    pub virt_capable: bool,
    pub l1ist_addr: *mut __le64,
    pub l2_size: u32,
    pub l2_bits: u8,
    pub l2: bool,
    pub ist: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gicv5_irs_chip_data {
    pub entry: list_head,
    pub fwnode: *mut fwnode_handle,
    pub irs_base: *mut void __iomem,
    pub res: resource,
    pub flags: u32,
    pub spi_min: u32,
    pub spi_range: u32,
    pub spi_config_lock: raw_spinlock_t,
}

// val = tmp;

extern "C" {
    pub fn gicv5_init_lpi_domain() -> void __init;
}
extern "C" {
    pub fn gicv5_free_lpi_domain() -> void __init;
}
extern "C" {
    pub fn gicv5_irs_of_probe(parent: *mut device_node) -> c_int;
}
extern "C" {
    pub fn gicv5_irs_acpi_probe() -> c_int;
}
extern "C" {
    pub fn gicv5_irs_remove();
}
extern "C" {
    pub fn gicv5_irs_enable() -> c_int;
}
extern "C" {
    pub fn gicv5_irs_its_probe();
}
extern "C" {
    pub fn gicv5_irs_register_cpu(cpuid: c_int) -> c_int;
}
extern "C" {
    pub fn gicv5_irs_cpu_to_iaffid(cpu_id: c_int, iaffid: *mut u16) -> c_int;
}
extern "C" {
    pub fn gicv5_spi_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int;
}
extern "C" {
    pub fn gicv5_irs_iste_alloc(lpi: u32) -> c_int;
}
extern "C" {
    pub fn gicv5_irs_syncr();
}
// Embedded in kvm.arch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gicv5_vpe {
    pub resident: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gicv5_its_devtab_cfg {
    pub devtab: *mut __le64,
    pub linear: },
    pub l1devtab: *mut __le64,
    pub l2ptrs: *mut __le64,
    pub l2: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gicv5_its_itt_cfg {
    pub itt: *mut __le64,
    pub num_ents: c_uint,
    pub linear: },
    pub l1itt: *mut __le64,
    pub l2ptrs: *mut __le64,
    pub num_l1_ents: c_uint,
    pub l2sz: u8,
    pub l2: },
}

extern "C" {
    pub fn gicv5_init_lpis(max: u32);
}
extern "C" {
    pub fn gicv5_deinit_lpis();
}
extern "C" {
    pub fn gicv5_its_of_probe(parent: *mut device_node) -> void __init;
}
extern "C" {
    pub fn gicv5_its_acpi_probe() -> void __init;
}
