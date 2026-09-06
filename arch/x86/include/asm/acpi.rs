//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/acpi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2001 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
// Copyright (C) 2001 Patrick Mochel <mochel@osdl.org>
//

extern "C" {
    pub fn acpi_pic_sci_set_trigger(int: unsigned, _arg: u16);
}
extern "C" {
    pub fn void(gsi: *mut *mut __acpi_unregister_gsi)(u32) -> extern;
}
extern "C" {
    pub fn acpi_gsi_to_irq(gsi: u32, irq: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn acpi_blacklisted() -> c_int;
}
// Low-level suspend routine.
extern "C" {
    pub fn int(_arg: *mut acpi_suspend_lowlevel)(void) -> extern;
}
// Physical address to resume after wakeup
extern "C" {
    pub fn acpi_get_wakeup_address() -> c_ulong;
}
extern "C" {
    pub fn cpu_feature_enabled(_arg: X86_FEATURE_XENPV) -> return;
}

extern "C" {
    pub fn asm_acpi_mp_play_dead(reset_vector: u64, pgd_pa: u64);
}
//
// Check if the CPU can handle C2 and deeper
//
// Early models (<=5) of AMD Opterons are not supposed to go into
// C2 state.
//
// Steppings 0x0A and later are good
//
// cap |= ACPI_PROC_CAP_C_CAPABILITY_SMP;
// Enable coordination with firmware's _TSD info
// cap |= ACPI_PROC_CAP_SMP_T_SWCOORD;
// cap |= ACPI_PROC_CAP_EST_CAPABILITY_SWSMP;
// cap |= ACPI_PROC_CAP_T_FFH;
// cap |= ACPI_PROC_CAP_COLLAB_PROC_PERF;
//
// If mwait/monitor is unsupported, C_C1_FFH and
// C2/C3_FFH will be disabled.
//
// cap &= ~(ACPI_PROC_CAP_C_C1_FFH | ACPI_PROC_CAP_C_C2C3_FFH);
//
// When Linux is running as Xen dom0, the hypervisor is the
// entity in charge of the processor power management, and so
// Xen needs to check the OS capabilities reported in the
// processor capabilities buffer matches what the hypervisor
// driver supports.
//
// Macro flag: #define ACPI_HAVE_ARCH_SET_ROOT_POINTER
// Macro flag: #define ACPI_HAVE_ARCH_GET_ROOT_POINTER
extern "C" {
    pub fn acpi_generic_reduced_hw_init();
}
extern "C" {
    pub fn x86_default_set_root_pointer(addr: u64);
}
extern "C" {
    pub fn x86_default_get_root_pointer() -> u64;
}

// A Xen PV domain needs a special acpi_os_ioremap() handling.

extern "C" {
    pub fn acpi_setup_mp_wakeup_mailbox(addr: u64);
}
extern "C" {
    pub fn acpi_get_mp_wakeup_mailbox_paddr() -> u64;
}

pub const acpi_lapic: c_int = 0;
pub const acpi_ioapic: c_int = 0;
pub const acpi_disable_cmcff: c_int = 0;

pub const ARCH_HAS_POWER_INIT: c_int = 1;

extern "C" {
    pub fn x86_acpi_numa_init() -> c_int;
}

//
// We currently have no way to look up the EFI memory map
// attributes for a region in a consistent way, because the
// memmap is discarded after efi_free_boot_services(). So if
// you call efi_mem_attributes() during boot and at runtime,
// you could theoretically see different attributes.
//
// We are yet to see any x86 platforms that require anything
// other than PAGE_KERNEL (some ARM64 platforms require the
// equivalent of PAGE_KERNEL_NOCACHE). Additionally, if SME
// is active, the ACPI information will not be encrypted,
// so return PAGE_KERNEL_NOENC until we know differently.
//

