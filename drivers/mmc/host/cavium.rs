//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/cavium.h
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


//
// Driver for MMC and SSD cards for Cavium OCTEON and ThunderX SOCs.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2012-2017 Cavium Inc.
//

pub const CAVIUM_MAX_MMC: c_int = 4;
// DMA register addresses

// register addresses

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_mmc_host {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub dma_base: *mut void __iomem,
    pub reg_off: c_int,
    pub reg_off_dma: c_int,
    pub emm_cfg: u64,
    pub /: *mut *mut u64 n_minus_one; / OCTEON II workaround location,
    pub last_slot: c_int,
    pub clk: *mut clk,
    pub sys_freq: c_int,
    pub current_req: *mut mmc_request,
    pub smi: sg_mapping_iter,
    pub dma_active: bool,
    pub use_sg: bool,
    pub has_ciu3: bool,
    pub big_dma_addr: bool,
    pub need_irq_handler_lock: bool,
    pub irq_handler_lock: spinlock_t,
    pub mmc_serializer: semaphore,
    pub global_pwr_gpiod: *mut gpio_desc,
    pub shared_power_users: core::sync::atomic::AtomicI32,
    pub slot: [*mut cvm_mmc_slot; CAVIUM_MAX_MMC],
    pub slot_pdev: [*mut platform_device; CAVIUM_MAX_MMC],
    pub int): *mut *mut *mut void (set_shared_power)(struct cvm_mmc_host ,,
    pub ): *mut *mut void (acquire_bus)(struct cvm_mmc_host,
    pub ): *mut *mut void (release_bus)(struct cvm_mmc_host,
    pub u64): *mut *mut *mut void (int_enable)(struct cvm_mmc_host ,,
// required on some MIPS models
    pub u64): *mut *mut mmc_data ,,
    pub ): *mut *mut void (dmar_fixup_done)(struct cvm_mmc_host,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_mmc_slot {
    pub /: *mut *mut *mut mmc_host mmc; / slot-level mmc_core object,
    pub /: *mut *mut *mut cvm_mmc_host host; / common hw for all slots,
    pub clock: u64,
    pub cached_switch: u64,
    pub cached_rca: u64,
    pub /: *mut *mut unsigned int cmd_cnt; / sample delay,
    pub /: *mut *mut unsigned int dat_cnt; / sample delay,
    pub bus_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_mmc_cr_type {
    pub ctype: u8,
    pub rtype: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvm_mmc_cr_mods {
    pub ctype_xor: u8,
    pub rtype_xor: u8,
}

// Bitfield definitions

// Protoypes
extern "C" {
    pub fn cvm_mmc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cvm_mmc_of_slot_probe(dev: *mut device, host: *mut cvm_mmc_host) -> c_int;
}
extern "C" {
    pub fn cvm_mmc_of_slot_remove(slot: *mut cvm_mmc_slot) -> c_int;
}
