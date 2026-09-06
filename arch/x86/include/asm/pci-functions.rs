//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pci-functions.h
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
// PCI BIOS function numbering for conventional PCI BIOS
// systems
//
pub const PCIBIOS_PCI_FUNCTION_ID: c_uint = 0xb1XX;
pub const PCIBIOS_PCI_BIOS_PRESENT: c_uint = 0xb101;
pub const PCIBIOS_FIND_PCI_DEVICE: c_uint = 0xb102;
pub const PCIBIOS_FIND_PCI_CLASS_CODE: c_uint = 0xb103;
pub const PCIBIOS_GENERATE_SPECIAL_CYCLE: c_uint = 0xb106;
pub const PCIBIOS_READ_CONFIG_BYTE: c_uint = 0xb108;
pub const PCIBIOS_READ_CONFIG_WORD: c_uint = 0xb109;
pub const PCIBIOS_READ_CONFIG_DWORD: c_uint = 0xb10a;
pub const PCIBIOS_WRITE_CONFIG_BYTE: c_uint = 0xb10b;
pub const PCIBIOS_WRITE_CONFIG_WORD: c_uint = 0xb10c;
pub const PCIBIOS_WRITE_CONFIG_DWORD: c_uint = 0xb10d;
pub const PCIBIOS_GET_ROUTING_OPTIONS: c_uint = 0xb10e;
pub const PCIBIOS_SET_PCI_HW_INT: c_uint = 0xb10f;
