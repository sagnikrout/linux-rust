//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma_driver_mips.h
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
pub const BCMA_MIPS_IPSFLAG: c_uint = 0x0F08;
// which sbflags get routed to mips interrupt 1
pub const BCMA_MIPS_IPSFLAG_IRQ1: c_uint = 0x0000003F;
pub const BCMA_MIPS_IPSFLAG_IRQ1_SHIFT: c_int = 0;
// which sbflags get routed to mips interrupt 2
pub const BCMA_MIPS_IPSFLAG_IRQ2: c_uint = 0x00003F00;
pub const BCMA_MIPS_IPSFLAG_IRQ2_SHIFT: c_int = 8;
// which sbflags get routed to mips interrupt 3
pub const BCMA_MIPS_IPSFLAG_IRQ3: c_uint = 0x003F0000;
pub const BCMA_MIPS_IPSFLAG_IRQ3_SHIFT: c_int = 16;
// which sbflags get routed to mips interrupt 4
pub const BCMA_MIPS_IPSFLAG_IRQ4: c_uint = 0x3F000000;
pub const BCMA_MIPS_IPSFLAG_IRQ4_SHIFT: c_int = 24;
// MIPS 74K core registers
pub const BCMA_MIPS_MIPS74K_CORECTL: c_uint = 0x0000;
pub const BCMA_MIPS_MIPS74K_EXCEPTBASE: c_uint = 0x0004;
pub const BCMA_MIPS_MIPS74K_BIST: c_uint = 0x000C;
pub const BCMA_MIPS_MIPS74K_INTMASK_INT0: c_uint = 0x0014;

pub const BCMA_MIPS_MIPS74K_NMIMASK: c_uint = 0x002C;
pub const BCMA_MIPS_MIPS74K_GPIOSEL: c_uint = 0x0040;
pub const BCMA_MIPS_MIPS74K_GPIOOUT: c_uint = 0x0044;
pub const BCMA_MIPS_MIPS74K_GPIOEN: c_uint = 0x0048;
pub const BCMA_MIPS_MIPS74K_CLKCTLST: c_uint = 0x01E0;
pub const BCMA_MIPS_OOBSELINA74: c_uint = 0x004;
pub const BCMA_MIPS_OOBSELOUTA30: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_drv_mips {
    pub core: *mut bcma_device,
    pub setup_done:1: u8,
    pub early_setup_done:1: u8,
}

extern "C" {
    pub fn bcma_cpu_clock(mcore: *mut bcma_drv_mips) -> u32;
}
