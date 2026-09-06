//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/hpet.h
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

pub const HPET_MMAP_SIZE: c_int = 1024;
pub const HPET_ID: c_uint = 0x000;
pub const HPET_PERIOD: c_uint = 0x004;
pub const HPET_CFG: c_uint = 0x010;
pub const HPET_STATUS: c_uint = 0x020;
pub const HPET_COUNTER: c_uint = 0x0f0;

pub const HPET_T0_CFG: c_uint = 0x100;
pub const HPET_T0_CMP: c_uint = 0x108;
pub const HPET_T0_ROUTE: c_uint = 0x110;
pub const HPET_T1_CFG: c_uint = 0x120;
pub const HPET_T1_CMP: c_uint = 0x128;
pub const HPET_T1_ROUTE: c_uint = 0x130;
pub const HPET_T2_CFG: c_uint = 0x140;
pub const HPET_T2_CMP: c_uint = 0x148;
pub const HPET_T2_ROUTE: c_uint = 0x150;
pub const HPET_ID_REV: c_uint = 0x000000ff;
pub const HPET_ID_NUMBER: c_uint = 0x00001f00;
pub const HPET_ID_64BIT: c_uint = 0x00002000;
pub const HPET_ID_LEGSUP: c_uint = 0x00008000;
pub const HPET_ID_VENDOR: c_uint = 0xffff0000;
pub const HPET_ID_NUMBER_SHIFT: c_int = 8;
pub const HPET_ID_VENDOR_SHIFT: c_int = 16;
pub const HPET_CFG_ENABLE: c_uint = 0x001;
pub const HPET_CFG_LEGACY: c_uint = 0x002;
pub const HPET_LEGACY_8254: c_int = 2;
pub const HPET_LEGACY_RTC: c_int = 8;
pub const HPET_TN_LEVEL: c_uint = 0x0002;
pub const HPET_TN_ENABLE: c_uint = 0x0004;
pub const HPET_TN_PERIODIC: c_uint = 0x0008;
pub const HPET_TN_PERIODIC_CAP: c_uint = 0x0010;
pub const HPET_TN_64BIT_CAP: c_uint = 0x0020;
pub const HPET_TN_SETVAL: c_uint = 0x0040;
pub const HPET_TN_32BIT: c_uint = 0x0100;
pub const HPET_TN_ROUTE: c_uint = 0x3e00;
pub const HPET_TN_FSB: c_uint = 0x4000;
pub const HPET_TN_FSB_CAP: c_uint = 0x8000;
pub const HPET_TN_ROUTE_SHIFT: c_int = 9;
// Max HPET Period is 10^8 femto sec as in HPET spec

//
// Min HPET period is 10^5 femto sec just for safety. If it is less than this,
// then 32 bit HPET counter wrapsaround in less than 0.5 sec.
//

// hpet memory map physical address
extern "C" {
    pub fn is_hpet_enabled() -> c_int;
}
extern "C" {
    pub fn hpet_enable() -> c_int;
}
extern "C" {
    pub fn hpet_disable();
}
extern "C" {
    pub fn hpet_readl(a: c_uint) -> c_uint;
}
extern "C" {
    pub fn force_hpet_resume();
}

extern "C" {
    pub fn irqreturn_t(interrupt: *mut *mut rtc_irq_handler)(int, cookie: *mut c_void) -> typedef;
}
extern "C" {
    pub fn hpet_mask_rtc_irq_bit(bit_mask: c_ulong) -> c_int;
}
extern "C" {
    pub fn hpet_set_rtc_irq_bit(bit_mask: c_ulong) -> c_int;
}
extern "C" {
    pub fn hpet_set_periodic_freq(freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn hpet_rtc_timer_init() -> c_int;
}
extern "C" {
    pub fn hpet_rtc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hpet_register_irq_handler(handler: rtc_irq_handler) -> c_int;
}
extern "C" {
    pub fn hpet_unregister_irq_handler(handler: rtc_irq_handler);
}

pub const hpet_readl(a): c_int = 0;

