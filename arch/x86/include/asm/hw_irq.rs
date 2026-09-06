//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/hw_irq.h
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
// (C) 1992, 1993 Linus Torvalds, (C) 1997 Ingo Molnar
//
// moved some of the old arch/i386/kernel/irq.h to here. VY
//
// IRQ/IPI changes taken from work by Thomas Radke
// <tomsoft@informatik.tu-chemnitz.de>
//
// hacked by Andi Kleen for x86-64.
// unified by tglx
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_alloc_type {
    X86_IRQ_ALLOC_TYPE_IOAPIC = 1,
    X86_IRQ_ALLOC_TYPE_HPET,
    X86_IRQ_ALLOC_TYPE_PCI_MSI,
    X86_IRQ_ALLOC_TYPE_PCI_MSIX,
    X86_IRQ_ALLOC_TYPE_DMAR,
    X86_IRQ_ALLOC_TYPE_AMDVI,
    X86_IRQ_ALLOC_TYPE_UV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioapic_alloc_info {
    pub pin: c_int,
    pub node: c_int,
    pub 1: u32 is_level :,
    pub 1: u32 active_low :,
    pub 1: u32 valid :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_alloc_info {
    pub limit: c_int,
    pub blade: c_int,
    pub offset: c_ulong,
    pub name: *mut c_char,
}

//
// irq_alloc_info - X86 specific interrupt allocation info
// @type:	X86 specific allocation type
// @flags:	Flags for allocation tweaks
// @devid:	Device ID for allocations
// @hwirq:	Associated hw interrupt number in the domain
// @mask:	CPU mask for vector allocation
// @desc:	Pointer to msi descriptor
// @data:	Allocation specific data
//
// @ioapic:	IOAPIC specific allocation data
// @uv:		UV specific allocation data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_alloc_info {
    pub type: irq_alloc_type,
    pub flags: u32,
    pub devid: u32,
    pub hwirq: irq_hw_number_t,
    pub mask: *const cpumask,
    pub desc: *mut msi_desc,
    pub data: *mut c_void,
    pub ioapic: ioapic_alloc_info,
    pub uv: uv_alloc_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_cfg {
    pub dest_apicid: c_uint,
    pub vector: c_uint,
}

extern "C" {
    pub fn vector_schedule_cleanup(: *mut irq_cfg);
}
extern "C" {
    pub fn irq_complete_move(cfg: *mut irq_cfg);
}

extern "C" {
    pub fn apic_ack_edge(data: *mut irq_data);
}

extern "C" {
    pub fn lock_vector_lock();
}
extern "C" {
    pub fn unlock_vector_lock();
}

extern "C" {
    pub fn elcr_set_level_irq(irq: c_uint);
}

