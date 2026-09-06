//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/omap-iommu.h
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
// omap iommu: main structures
//
// Copyright (C) 2008-2009 Nokia Corporation
//
// Written by Hiroshi DOYU <Hiroshi.DOYU@nokia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iotlb_entry {
    pub da: u32,
    pub pa: u32,
    pub valid: u32 pgsz, prsvd,,
    pub mixed: u32 endian, elsz,,
}

//
// struct omap_iommu_device - omap iommu device data
// @pgtable:	page table used by an omap iommu attached to a domain
// @iommu_dev:	pointer to store an omap iommu instance attached to a domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_iommu_device {
    pub pgtable: *mut u32,
    pub iommu_dev: *mut omap_iommu,
}

//
// struct omap_iommu_domain - omap iommu domain
// @num_iommus: number of iommus in this domain
// @iommus:	omap iommu device data for all iommus in this domain
// @dev:	Device using this domain.
// @lock:	domain lock, should be taken when attaching/detaching
// @domain:	generic domain handle used by iommu core code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_iommu_domain {
    pub num_iommus: u32,
    pub iommus: *mut omap_iommu_device,
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub domain: iommu_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_iommu {
    pub name: *const c_char,
    pub regbase: *mut void __iomem,
    pub syscfg: *mut regmap,
    pub dev: *mut device,
    pub domain: *mut iommu_domain,
    pub debug_dir: *mut dentry,
    pub /: *mut *mut spinlock_t iommu_lock; / global for this whole object,
//
// We don't change iopgd for a situation like pgd for a task,
// but share it globally for each iommu.
//
    pub iopgd: *mut u32,
    pub /: *mut *mut spinlock_t page_table_lock; / protect iopgd,
    pub pd_dma: dma_addr_t,
    pub nr_tlb_entries: c_int,
    pub /: *mut *mut *mut void ctx; / iommu context: registres saved area,
    pub cr_ctx: *mut cr_regs,
    pub num_cr_ctx: u32,
    pub has_bus_err_back: c_int,
    pub id: u32,
    pub iommu: iommu_device,
    pub has_iommu_driver: bool,
    pub pwrst: u8,
}

//
// struct omap_iommu_arch_data - omap iommu private data
// @iommu_dev: handle of the OMAP iommu device
//
// This is an omap iommu private data object, which binds an iommu user
// to its iommu device. This object should be placed at the iommu user's
// dev_archdata so generic IOMMU API can be used without having to
// utilize omap-specific plumbing anymore.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_iommu_arch_data {
    pub iommu_dev: *mut omap_iommu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cr_regs {
    pub cam: u32,
    pub ram: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iotlb_lock {
    pub base: c_short,
    pub vict: c_short,
}

//
// MMU Register offsets
//
pub const MMU_REVISION: c_uint = 0x00;
pub const MMU_IRQSTATUS: c_uint = 0x18;
pub const MMU_IRQENABLE: c_uint = 0x1c;
pub const MMU_WALKING_ST: c_uint = 0x40;
pub const MMU_CNTL: c_uint = 0x44;
pub const MMU_FAULT_AD: c_uint = 0x48;
pub const MMU_TTB: c_uint = 0x4c;
pub const MMU_LOCK: c_uint = 0x50;
pub const MMU_LD_TLB: c_uint = 0x54;
pub const MMU_CAM: c_uint = 0x58;
pub const MMU_RAM: c_uint = 0x5c;
pub const MMU_GFLUSH: c_uint = 0x60;
pub const MMU_FLUSH_ENTRY: c_uint = 0x64;
pub const MMU_READ_CAM: c_uint = 0x68;
pub const MMU_READ_RAM: c_uint = 0x6c;
pub const MMU_EMU_FAULT_AD: c_uint = 0x70;
pub const MMU_GP_REG: c_uint = 0x88;
pub const MMU_REG_SIZE: c_int = 256;
//
// MMU Register bit definitions
//
// IRQSTATUS & IRQENABLE

// MMU_CNTL
pub const MMU_CNTL_SHIFT: c_int = 1;

// CAM
pub const MMU_CAM_VATAG_SHIFT: c_int = 12;

pub const MMU_CAM_PGSZ_MASK: c_int = 3;

// RAM
pub const MMU_RAM_PADDR_SHIFT: c_int = 12;

pub const MMU_RAM_ENDIAN_SHIFT: c_int = 9;

pub const MMU_RAM_ELSZ_SHIFT: c_int = 7;

pub const MMU_RAM_MIXED_SHIFT: c_int = 6;

pub const MMU_GP_REG_BUS_ERR_BACK_EN: c_uint = 0x1;

//
// DSP_SYSTEM registers and bit definitions (applicable only for DRA7xx DSP)
//
pub const DSP_SYS_REVISION: c_uint = 0x00;
pub const DSP_SYS_MMU_CONFIG: c_uint = 0x18;
pub const DSP_SYS_MMU_CONFIG_EN_SHIFT: c_int = 4;
//
// utilities for super page(16MB, 1MB, 64KB and 4KB)
//

//
// global functions
//
extern "C" {
    pub fn __iotlb_read_cr(obj: *mut omap_iommu, n: c_int) -> cr_regs;
}
extern "C" {
    pub fn iotlb_lock_get(obj: *mut omap_iommu, l: *mut iotlb_lock);
}
extern "C" {
    pub fn iotlb_lock_set(obj: *mut omap_iommu, l: *mut iotlb_lock);
}

extern "C" {
    pub fn omap_iommu_debugfs_init();
}
extern "C" {
    pub fn omap_iommu_debugfs_exit();
}
extern "C" {
    pub fn omap_iommu_debugfs_add(obj: *mut omap_iommu);
}
extern "C" {
    pub fn omap_iommu_debugfs_remove(obj: *mut omap_iommu);
}

//
// register accessors
//
extern "C" {
    pub fn __raw_readl(offs: obj->regbase +) -> return;
}
