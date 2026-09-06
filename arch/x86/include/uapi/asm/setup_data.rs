//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/setup_data.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// setup_data/setup_indirect types
pub const SETUP_NONE: c_int = 0;
pub const SETUP_E820_EXT: c_int = 1;
pub const SETUP_DTB: c_int = 2;
pub const SETUP_PCI: c_int = 3;
pub const SETUP_EFI: c_int = 4;
pub const SETUP_APPLE_PROPERTIES: c_int = 5;
pub const SETUP_JAILHOUSE: c_int = 6;
pub const SETUP_CC_BLOB: c_int = 7;
pub const SETUP_IMA: c_int = 8;
pub const SETUP_RNG_SEED: c_int = 9;
pub const SETUP_KEXEC_KHO: c_int = 10;

// extensible setup data list node
#[repr(C)]
#[derive(Copy, Clone)]
pub struct setup_data {
    pub next: __u64,
    pub type: __u32,
    pub len: __u32,
    pub data: [__u8; ],
}

// extensible setup indirect data node
#[repr(C)]
#[derive(Copy, Clone)]
pub struct setup_indirect {
    pub type: __u32,
    pub /: *mut *mut __u32 reserved; / Reserved, must be set to zero.,
    pub len: __u64,
    pub addr: __u64,
}

//
// The E820 memory region entry of the boot protocol ABI:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_e820_entry {
    pub addr: __u64,
    pub size: __u64,
    pub type: __u32,
    pub __attribute__((packed)): },
//
// The boot loader is passing platform information via this Jailhouse-specific
// setup data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jailhouse_setup_data {
    pub version: __u16,
    pub compatible_version: __u16,
// C attribute field omitted
    pub pm_timer_address: __u16,
    pub num_cpus: __u16,
    pub pci_mmconfig_base: __u64,
    pub tsc_khz: __u32,
    pub apic_khz: __u32,
    pub standard_ioapic: __u8,
    pub cpu_ids: [__u8; 255],
// C attribute field omitted
    pub flags: __u32,
// C attribute field omitted
    pub __attribute__((packed)): },
//
// IMA buffer setup data information from the previous kernel during kexec
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_setup_data {
    pub addr: __u64,
    pub size: __u64,
    pub __attribute__((packed)): },
//
// Locations of kexec handover metadata
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_data {
    pub fdt_addr: __u64,
    pub fdt_size: __u64,
    pub scratch_addr: __u64,
    pub scratch_size: __u64,
    pub __attribute__((packed)): },

