//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/acpi/apei.c
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
// Arch-specific APEI-related functions.
//

#[no_mangle]
pub unsafe extern "C" fn arch_apei_enable_cmcff(hest_hdr: *mut acpi_hest_header, data: *mut c_void) -> c_int {
    int arch_apei_enable_cmcff(struct acpi_hest_header *hest_hdr, void *data)
    {

    int i;
    struct acpi_hest_ia_corrected *cmc;
    struct acpi_hest_ia_error_bank *mc_bank;
    cmc = (struct acpi_hest_ia_corrected *)hest_hdr;
    if (!cmc.enabled)
    return 0;
    mce_save_apei_thr_limit(cmc.notify.error_threshold_value);
//
// We expect HEST to provide a list of MC banks that report errors
// in firmware first mode. Otherwise, return non-zero value to
// indicate that we are done parsing HEST.
//
    if (!(cmc.flags & ACPI_HEST_FIRMWARE_FIRST) ||
    !cmc.num_hardware_banks)
    return 1;
    pr_info("HEST: Enabling Firmware First mode for corrected errors.\n");
    mc_bank = (struct acpi_hest_ia_error_bank *)(cmc + 1);
    for (i = 0; i < cmc.num_hardware_banks; i++, mc_bank++)
    mce_disable_bank(mc_bank.bank_number);

    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_apei_report_mem_error(sev: c_int, mem_err: *mut cper_sec_mem_err) {
    void arch_apei_report_mem_error(int sev, struct cper_sec_mem_err *mem_err)
    {

    apei_mce_report_mem_error(sev, mem_err);

    }
#[no_mangle]
pub unsafe extern "C" fn arch_apei_report_x86_error(ctx_info: *mut cper_ia_proc_ctx, lapic_id: u64) -> c_int {
    int arch_apei_report_x86_error(struct cper_ia_proc_ctx *ctx_info, u64 lapic_id)
    {
    return apei_smca_report_x86_error(ctx_info, lapic_id);
    }
