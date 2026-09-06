//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/nfit/mce.c
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
// NFIT - Machine Check Handler
//
// Copyright(c) 2013-2016 Intel Corporation. All rights reserved.
//

    static int nfit_handle_mce(struct notifier_block *nb, unsigned long val,
    void *data)
    {
    struct mce *mce = (struct mce *)data;
    struct acpi_nfit_desc *acpi_desc;
    struct nfit_spa *nfit_spa;
// We only care about uncorrectable memory errors
    if (!mce_is_memory_error(mce) || mce_is_correctable(mce))
    return NOTIFY_DONE;
// Verify the address reported in the MCE is valid.
    if (!mce_usable_address(mce))
    return NOTIFY_DONE;
//
// mce->addr contains the physical addr accessed that caused the
// machine check. We need to walk through the list of NFITs, and see
// if any of them matches that address, and only then start a scrub.
//
    mutex_lock(&acpi_desc_lock);
    list_for_each_entry(acpi_desc, &acpi_descs, list) {
    let mut align: c_uint = 1UL << MCI_MISC_ADDR_LSB(mce.misc);
    struct device *dev = acpi_desc.dev;
    let mut found_match: c_int = 0;
    mutex_lock(&acpi_desc.init_mutex);
    list_for_each_entry(nfit_spa, &acpi_desc.spas, list) {
    struct acpi_nfit_system_address *spa = nfit_spa.spa;
    if (nfit_spa_type(spa) != NFIT_SPA_PM)
    continue;
// find the spa that covers the mce addr
    if (spa.address > mce.addr)
    continue;
    if ((spa.address + spa.length - 1) < mce.addr)
    continue;
    found_match = 1;
    dev_dbg(dev, "addr in SPA %d (0x%llx, 0x%llx)\n",
    spa.range_index, spa.address, spa.length);
//
// We can break at the first match because we're going
// to rescan all the SPA ranges. There shouldn't be any
// aliasing anyway.
//
    break;
    }
    mutex_unlock(&acpi_desc.init_mutex);
    if (!found_match)
    continue;
// If this fails due to an -ENOMEM, there is little we can do
    nvdimm_bus_add_badrange(acpi_desc.nvdimm_bus,
    ALIGN_DOWN(mce.addr, align), align);
    nvdimm_region_notify(nfit_spa.nd_region,
    NVDIMM_REVALIDATE_POISON);
    if (acpi_desc.scrub_mode == HW_ERROR_SCRUB_ON) {
//
// We can ignore an -EBUSY here because if an ARS is
// already in progress, just let that be the last
// authoritative one
//
    acpi_nfit_ars_rescan(acpi_desc, 0);
    }
    mce.kflags |= MCE_HANDLED_NFIT;
    break;
    }
    mutex_unlock(&acpi_desc_lock);
    return NOTIFY_DONE;
    }
    static struct notifier_block nfit_mce_dec = {
    .notifier_call	= nfit_handle_mce,
    .priority	= MCE_PRIO_NFIT,
    };
#[no_mangle]
pub unsafe extern "C" fn nfit_mce_register() {
    void nfit_mce_register(void)
    {
    mce_register_decode_chain(&nfit_mce_dec);
    }
#[no_mangle]
pub unsafe extern "C" fn nfit_mce_unregister() {
    void nfit_mce_unregister(void)
    {
    mce_unregister_decode_chain(&nfit_mce_dec);
    }
