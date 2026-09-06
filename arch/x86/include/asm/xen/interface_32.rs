//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/interface_32.h
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
// arch-x86_32.h
//
// Guest OS interface to x86 32-bit Xen.
//
// Copyright (c) 2004, K A Fraser
//
// These flat segments are in the Xen-private section of every GDT. Since these
// are also present in the initial GDT, many OSes will be able to avoid
// installing their own GDT.
//
pub const FLAT_RING1_CS: c_uint = 0xe019    /* GDT index 259 */;
pub const FLAT_RING1_DS: c_uint = 0xe021    /* GDT index 260 */;
pub const FLAT_RING1_SS: c_uint = 0xe021    /* GDT index 260 */;
pub const FLAT_RING3_CS: c_uint = 0xe02b    /* GDT index 261 */;
pub const FLAT_RING3_DS: c_uint = 0xe033    /* GDT index 262 */;
pub const FLAT_RING3_SS: c_uint = 0xe033    /* GDT index 262 */;

// And the trap vector is...

pub const __MACH2PHYS_VIRT_START: c_uint = 0xF5800000;
pub const __MACH2PHYS_VIRT_END: c_uint = 0xF6800000;
pub const __MACH2PHYS_SHIFT: c_int = 2;
//
// Virtual addresses beyond this are not modifiable by guest OSes. The
// machine->physical mapping table starts at this address, read-only.
//
pub const __HYPERVISOR_VIRT_START: c_uint = 0xF5800000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_user_regs {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub eax: u32,
    pub /: *mut *mut uint16_t error_code; / private,
    pub /: *mut *mut uint16_t entry_vector; / private,
    pub eip: u32,
    pub cs: u16,
    pub saved_upcall_mask: u8,
    pub _pad0: u8,
    pub /: *mut *mut uint32_t eflags; / eflags.IF == !saved_upcall_mask,
    pub esp: u32,
    pub _pad1: uint16_t ss,,
    pub _pad2: uint16_t es,,
    pub _pad3: uint16_t ds,,
    pub _pad4: uint16_t fs,,
    pub _pad5: uint16_t gs,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_vcpu_info {
    pub cr2: c_ulong,
    pub /: *mut *mut unsigned long pad[5]; / sizeof(struct vcpu_info) == 64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_callback {
    pub cs: c_ulong,
    pub eip: c_ulong,
}

pub type xen_callback_t = xen_callback;

//
// Page-directory addresses above 4GB do not fit into architectural %cr3.
// When accessing %cr3, or equivalent field in vcpu_guest_context, guests
// must use the following accessor macros to pack/unpack valid MFNs.
//
// Note that Xen is using the fact that the pagetable base is always
// page-aligned, and putting the 12 MSB of the address into the 12 LSB
// of cr3.
//

