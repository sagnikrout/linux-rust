//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/intern.h
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
//
// CAAM/SEC 4.x driver backend
// Private/internal definitions between modules
//
// Copyright 2008-2011 Freescale Semiconductor, Inc.
// Copyright 2019, 2023 NXP
//

// Currently comes from Kconfig param as a ^2 (driver-required)

//
// Maximum size for crypto-engine software queue based on Job Ring
// size (JOBR_DEPTH) and a THRESHOLD (reserved for the non-crypto-API
// requests that are not passed through crypto-engine)
//
pub const THRESHOLD: c_int = 15;

// Kconfig params for interrupt coalescing if selected (else zero)

pub const JOBR_INTC: c_int = 0;
pub const JOBR_INTC_TIME_THLD: c_int = 0;
pub const JOBR_INTC_COUNT_THLD: c_int = 0;

//
// Storage for tracking each in-process entry moving across a ring
// Each entry on an output ring needs one of these
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_jrentry_info {
    pub arg): *mut *mut *mut *mut void (callbk)(struct device dev, u32 desc, u32 status, void,
    pub /: *mut *mut *mut void cbkarg; / Argument per ring entry,
    pub /: *mut *mut *mut u32 desc_addr_virt; / Stored virt addr for postprocessing,
    pub /: *mut *mut dma_addr_t desc_addr_dma; / Stored bus addr for done matching,
    pub /: *mut *mut u32 desc_size; / Stored size for postprocessing, header derived,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_jr_state {
    pub inpbusaddr: dma_addr_t,
    pub outbusaddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_jr_dequeue_params {
    pub dev: *mut device,
    pub enable_itr: c_int,
}

// Private sub-storage for a single JobR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_drv_private_jr {
    pub /: *mut *mut list_head list_node; / Job Ring device list,
    pub dev: *mut device,
    pub ridx: c_int,
    pub /: *mut *mut *mut caam_job_ring __iomem rregs; / JobR's register space,
    pub irqtask: tasklet_struct,
    pub tasklet_params: caam_jr_dequeue_params,
    pub /: *mut *mut int irq; / One per queue,
    pub hwrng: bool,
// Number of scatterlist crypt transforms active on the JobR
    pub ____cacheline_aligned: atomic_t tfm_count,
// Job ring info
    pub /: *mut *mut *mut caam_jrentry_info entinfo; / Alloc'ed 1 per ring entry,
    pub /: *mut *mut spinlock_t inplock ____cacheline_aligned; / Input ring index lock,
    pub /: *mut *mut u32 inpring_avail; / Number of free entries in input ring,
    pub /: *mut *mut int head; / entinfo (s/w ring) head index,
    pub alloc: *mut *mut *mut void inpring; / Base of input ring,,
// DMA-safe
    pub /: *mut *mut int out_ring_read_index; / Output index "tail",
    pub /: *mut *mut int tail; / entinfo (s/w ring) tail index,
    pub /: *mut *mut *mut void outring; / Base of output ring, DMA-safe,
    pub engine: *mut crypto_engine,
    pub /: *mut *mut caam_jr_state state; / State of the JR during PM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_ctl_state {
    pub deco_mid: [masterid; 16],
    pub jr_mid: [masterid; 4],
    pub mcr: u32,
    pub scfgr: u32,
}

//
// Driver-private storage for a single CAAM block instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_drv_private {
// Physical-presence section
    pub /: *mut *mut *mut caam_ctrl __iomem ctrl; / controller region,
    pub /: *mut *mut *mut caam_deco __iomem deco; / DECO/CCB views,
    pub assure: *mut caam_assurance __iomem,
    pub /: *mut *mut *mut caam_queue_if __iomem qi; / QI control region,
    pub /: *mut *mut *mut caam_job_ring __iomem jr[4]; / JobR's register space,
    pub domain: *mut iommu_domain,
//
// Detected geometry block. Filled in from device tree if powerpc,
// or from register-based version detection code
//
    pub /: *mut *mut u8 total_jobrs; / Total Job Rings in device,
    pub /: *mut *mut u8 qi_present; / Nonzero if QI present in device,
    pub /: *mut *mut u8 blob_present; / Nonzero if BLOB support present in device,
    pub /: *mut *mut u8 mc_en; / Nonzero if MC f/w is active,
    pub /: *mut *mut u8 optee_en; / Nonzero if OP-TEE f/w is active,
    pub /: *mut *mut u8 no_page0; / Nonzero if register page 0 is not controlled by Linux,
    pub /: *mut *mut bool pr_support; / RNG prediction resistance available,
    pub /: *mut *mut int secvio_irq; / Security violation interrupt number,
    pub /: *mut *mut int virt_en; / Virtualization enabled in CAAM,
    pub /: *mut *mut int era; / CAAM Era (internal HW revision),
pub const RNG4_MAX_HANDLES: c_int = 2;
// RNG4 block
    pub State: *mut *mut u32 rng4_sh_init; / This bitmap shows which of the,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
//
// debugfs entries for developer view into driver/device
// variables at runtime.
//

    pub /: *mut *mut *mut dentry ctl; / controller dir,
    pub ctl_tdsk_wrap: debugfs_blob_wrapper ctl_kek_wrap, ctl_tkek_wrap,,

    pub /: *mut *mut int caam_off_during_pm; / If the CAAM is reset after suspend,
    pub /: *mut *mut caam_ctl_state state; / State of the CTL during PM,
}

extern "C" {
    pub fn caam_algapi_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn caam_algapi_exit();
}

extern "C" {
    pub fn caam_algapi_hash_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn caam_algapi_hash_exit();
}

extern "C" {
    pub fn caam_pkc_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn caam_pkc_exit();
}

extern "C" {
    pub fn caam_rng_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn caam_rng_exit(dev: *mut device);
}

extern "C" {
    pub fn caam_qi_algapi_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn caam_qi_algapi_exit();
}

extern "C" {
    pub fn DMA_BIT_MASK(_arg: 32) -> return;
}
extern "C" {
    pub fn DMA_BIT_MASK(_arg: 49) -> return;
}
extern "C" {
    pub fn DMA_BIT_MASK(_arg: 40) -> return;
}
extern "C" {
    pub fn DMA_BIT_MASK(_arg: 36) -> return;
}
