//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pci-direct.h
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

// Direct PCI access. This is used for PCI accesses in early boot before
extern "C" {
    pub fn read_pci_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32;
}
extern "C" {
    pub fn read_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8) -> u8;
}
extern "C" {
    pub fn read_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8) -> u16;
}
extern "C" {
    pub fn write_pci_config(bus: u8, slot: u8, func: u8, offset: u8, val: u32);
}
extern "C" {
    pub fn write_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8, val: u8);
}
extern "C" {
    pub fn write_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8, val: u16);
}
extern "C" {
    pub fn early_pci_allowed() -> c_int;
}
