//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/bootx.h
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
//
// This file describes the structure passed from the BootX application
// (for MacOS) when it is used to boot Linux.
//
// Written by Benjamin Herrenschmidt.
//

// All this requires PowerPC alignment

// On kernel entry:
//
// r3 = 0x426f6f58    ('BooX')
// r4 = pointer to boot_infos
// r5 = NULL
//
// Data and instruction translation disabled, interrupts
// disabled, kernel loaded at physical 0x00000000 on PCI
// machines (will be different on NuBus).
//
pub const BOOT_INFO_VERSION: c_int = 5;
pub const BOOT_INFO_COMPATIBLE_VERSION: c_int = 1;
// Bit in the architecture flag mask. More to be defined in
//
pub const BOOT_ARCH_PCI: c_uint = 0x00000001UL;
pub const BOOT_ARCH_NUBUS: c_uint = 0x00000002UL;
pub const BOOT_ARCH_NUBUS_PDM: c_uint = 0x00000010UL;
pub const BOOT_ARCH_NUBUS_PERFORMA: c_uint = 0x00000020UL;
pub const BOOT_ARCH_NUBUS_POWERBOOK: c_uint = 0x00000040UL;
// Maximum number of ranges in phys memory map
pub const MAX_MEM_MAP_SIZE: c_int = 26;
// This is the format of an element in the physical memory map. Note that
// Here are the boot informations that are passed to the bootstrap
// Note that the kernel arguments and the device tree are appended
// at the end of this structure.
// Version of this structure
// backward compatible down to version:
// NEW (vers. 2) this holds the current _logical_ base addr of
// NEW (vers. 4) Apple's machine identification
// NEW (vers. 4) Detected hw architecture
// The device tree (internal addresses relative to the beginning of the tree,
// device tree offset relative to the beginning of this structure).
// On pre-PCI macintosh (BOOT_ARCH_PCI bit set to 0 in architecture), this
// field is 0.
//
// Some infos about the current MacOS display
// Optional offset in the registry to the current
// MacOS display. (Can be 0 when not detected)
// Optional pointer to boot ramdisk (offset from this structure)
// Kernel command line arguments (offset from this structure)
// ALL BELOW NEW (vers. 4)
// This defines the physical memory. Valid with BOOT_ARCH_NUBUS flag
// The framebuffer size (optional, currently 0)
// NEW (vers. 5)
// Total params size (args + colormap + device tree + ramdisk)

