//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/io_apic.h
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
// Intel IO-APIC support for SMP and UP systems.
//
// Copyright (C) 1997, 1998, 1999, 2000 Ingo Molnar
//
// The structure of the IO-APIC:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union IO_APIC_reg_00 {
    pub raw: u32,
    pub 8: ID :,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IO_APIC_reg_01 {
    pub raw: u32,
    pub 8: __reserved_1 :,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IO_APIC_reg_02 {
    pub raw: u32,
    pub 4: __reserved_1 :,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IO_APIC_reg_03 {
    pub raw: u32,
    pub 31: __reserved_1 :,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IO_APIC_route_entry {
    pub 8: destid_0_7 :,
}

pub const IOAPIC_MAP_ALLOC: c_uint = 0x1;
pub const IOAPIC_MAP_CHECK: c_uint = 0x2;

//
// # of IO-APICs and # of IRQ routing registers
//
extern "C" {
    pub fn mpc_ioapic_id(ioapic: c_int) -> c_int;
}
extern "C" {
    pub fn mpc_ioapic_addr(ioapic: c_int) -> c_uint;
}
// # of MP IRQ source entries
// MP IRQ source entries
// True if "noapic" boot option passed
// 1 if "noapic" boot option passed
// -1 if "noapic" boot option passed

//
// If we use the IO-APIC for IRQ routing, disable automatic
// assignment of PCI IRQ's.
//

extern "C" {
    pub fn ioapic_insert_resources();
}
extern "C" {
    pub fn arch_early_ioapic_init() -> c_int;
}
extern "C" {
    pub fn save_ioapic_entries() -> c_int;
}
extern "C" {
    pub fn mask_ioapic_entries();
}
extern "C" {
    pub fn restore_ioapic_entries() -> c_int;
}
extern "C" {
    pub fn setup_ioapic_ids_from_mpc();
}
extern "C" {
    pub fn mp_find_ioapic(gsi: u32) -> c_int;
}
extern "C" {
    pub fn mp_find_ioapic_pin(ioapic: c_int, gsi: u32) -> c_int;
}
extern "C" {
    pub fn mp_unmap_irq(irq: c_int);
}
extern "C" {
    pub fn mp_unregister_ioapic(gsi_base: u32) -> c_int;
}
extern "C" {
    pub fn mp_ioapic_registered(gsi_base: u32) -> c_int;
}
extern "C" {
    pub fn mp_save_irq(m: *mut mpc_intsrc);
}
extern "C" {
    pub fn disable_ioapic_support();
}
extern "C" {
    pub fn io_apic_init_mappings() -> void __init;
}
extern "C" {
    pub fn native_io_apic_read(apic: c_uint, reg: c_uint) -> c_uint;
}
extern "C" {
    pub fn native_restore_boot_irq_mode();
}
extern "C" {
    pub fn setup_IO_APIC();
}
extern "C" {
    pub fn enable_IO_APIC();
}
extern "C" {
    pub fn clear_IO_APIC();
}
extern "C" {
    pub fn restore_boot_irq_mode();
}
extern "C" {
    pub fn IO_APIC_get_PCI_irq_vector(bus: c_int, devfn: c_int, pin: c_int) -> c_int;
}
extern "C" {
    pub fn print_IO_APICs();
}

pub const IO_APIC_IRQ(x): c_int = 0;
pub const io_apic_assign_pci_irqs: c_int = 0;

