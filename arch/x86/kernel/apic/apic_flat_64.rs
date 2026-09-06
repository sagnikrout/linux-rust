//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/apic_flat_64.c
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
// Flat APIC subarch code.
//
// Hacked for x86-64 by James Cleverdon from i386 architecture code by
// Martin Bligh, Andi Kleen, James Bottomley, John Stultz, and
// James Cleverdon.
//

#[no_mangle]
unsafe extern "C" fn physflat_get_apic_id(x: u32) -> u32 {
    static u32 physflat_get_apic_id(u32 x)
    {
    return (x >> 24) & 0xFF;
    }
#[no_mangle]
unsafe extern "C" fn physflat_probe() -> c_int {
    static int physflat_probe(void)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn physflat_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int {
    static int physflat_acpi_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    return 1;
    }
    static struct apic apic_physflat __ro_after_init = {
    .name				= "physical flat",
    .probe				= physflat_probe,
    .acpi_madt_oem_check		= physflat_acpi_madt_oem_check,
    .dest_mode_logical		= false,
    .disable_esr			= 0,
    .cpu_present_to_apicid		= default_cpu_present_to_apicid,
    .max_apic_id			= 0xFE,
    .get_apic_id			= physflat_get_apic_id,
    .calc_dest_apicid		= apic_default_calc_apicid,
    .send_IPI			= default_send_IPI_single_phys,
    .send_IPI_mask			= default_send_IPI_mask_sequence_phys,
    .send_IPI_mask_allbutself	= default_send_IPI_mask_allbutself_phys,
    .send_IPI_allbutself		= default_send_IPI_allbutself,
    .send_IPI_all			= default_send_IPI_all,
    .send_IPI_self			= default_send_IPI_self,
    .nmi_to_offline_cpu		= true,
    .read				= native_apic_mem_read,
    .write				= native_apic_mem_write,
    .eoi				= native_apic_mem_eoi,
    .icr_read			= native_apic_icr_read,
    .icr_write			= native_apic_icr_write,
    .wait_icr_idle			= apic_mem_wait_icr_idle,
    .safe_wait_icr_idle		= apic_mem_wait_icr_idle_timeout,
    };
    apic_driver(apic_physflat);
    let mut __ro_after_init: *mut apic apic = &apic_physflat;
    EXPORT_SYMBOL_GPL(apic);
