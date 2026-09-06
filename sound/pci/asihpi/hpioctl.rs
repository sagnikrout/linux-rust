//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpioctl.h
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
extern "C" {
    pub fn asihpi_adapter_remove(pci_dev: *mut pci_dev);
}
extern "C" {
    pub fn asihpi_init() -> void __init;
}
extern "C" {
    pub fn asihpi_exit() -> void __exit;
}
extern "C" {
    pub fn asihpi_hpi_release(file: *mut file) -> c_int;
}
extern "C" {
    pub fn asihpi_hpi_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
// This is called from hpifunc.c functions, called by ALSA
// (or other kernel process) In this case there is no file descriptor
// available for the message cache code
//
extern "C" {
    pub fn hpi_send_recv(phm: *mut hpi_message, phr: *mut hpi_response);
}
