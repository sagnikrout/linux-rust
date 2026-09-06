//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/powermac/pmac.h
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
// Declaration for the various functions exported by the
// pmac_* files. Mostly for use by pmac_setup
//
extern "C" {
    pub fn g5_phy_disable_cpu1();
}
extern "C" {
    pub fn pmac_time_init() -> c_long;
}
extern "C" {
    pub fn pmac_get_boot_time() -> time64_t;
}
extern "C" {
    pub fn pmac_get_rtc_time(: *mut rtc_time);
}
extern "C" {
    pub fn pmac_set_rtc_time(: *mut rtc_time) -> c_int;
}
extern "C" {
    pub fn pmac_read_rtc_time();
}
extern "C" {
    pub fn pmac_calibrate_decr();
}
extern "C" {
    pub fn pmac_pci_irq_fixup(: *mut pci_dev);
}
extern "C" {
    pub fn pmac_pci_init();
}
extern "C" {
    pub fn pmac_nvram_update();
}
extern "C" {
    pub fn pmac_nvram_read_byte(addr: c_int) -> c_uchar;
}
extern "C" {
    pub fn pmac_nvram_write_byte(addr: c_int, val: c_uchar);
}
extern "C" {
    pub fn pmac_pcibios_after_init();
}
extern "C" {
    pub fn pmac_setup_pci_dma();
}
extern "C" {
    pub fn pmac_check_ht_link();
}
extern "C" {
    pub fn pmac_setup_smp();
}
extern "C" {
    pub fn low_cpu_offline_self(__attribute__((noreturn): void));
}
extern "C" {
    pub fn pmac_nvram_init() -> c_int;
}
extern "C" {
    pub fn pmac_pic_init();
}
