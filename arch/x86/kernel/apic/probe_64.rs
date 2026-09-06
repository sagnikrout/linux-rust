//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/probe_64.c
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
// Copyright 2004 James Cleverdon, IBM.
//
// Generic APIC sub-arch probe layer.
//
// Hacked for x86-64 by James Cleverdon from i386 architecture code by
// Martin Bligh, Andi Kleen, James Bottomley, John Stultz, and
// James Cleverdon.
//

// Select the appropriate APIC driver
#[no_mangle]
pub unsafe extern "C" fn x86_64_probe_apic() -> void __init {
    void __init x86_64_probe_apic(void)
    {
    struct apic **drv;
    enable_IR_x2apic();
    for (drv = __apicdrivers; drv < __apicdrivers_end; drv++) {
    if ((*drv).probe && (*drv).probe()) {
    apic_install_driver(*drv);
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn default_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> int __init {
    int __init default_acpi_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    struct apic **drv;
    for (drv = __apicdrivers; drv < __apicdrivers_end; drv++) {
    if ((*drv).acpi_madt_oem_check(oem_id, oem_table_id)) {
    apic_install_driver(*drv);
    return 1;
    }
    }
    return 0;
    }
