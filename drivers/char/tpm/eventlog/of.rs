//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/eventlog/of.c
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
// Copyright 2012 IBM Corporation
//
// Author: Ashley Lai <ashleydlai@gmail.com>
// Nayna Jain <nayna@linux.vnet.ibm.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// Read the event log created by the firmware on PPC64
//

#[no_mangle]
unsafe extern "C" fn tpm_read_log_memory_region(chip: *mut tpm_chip) -> c_int {
    static int tpm_read_log_memory_region(struct tpm_chip *chip)
    {
    struct resource res;
    int rc;
    rc = of_reserved_mem_region_to_resource(chip.dev.parent.of_node, 0, &res);
    if (rc)
    return rc;
    chip.log.bios_event_log = devm_memremap(&chip.dev, res.start, resource_size(&res),
    MEMREMAP_WB);
    if (IS_ERR(chip.log.bios_event_log))
    return -ENOMEM;
    chip.log.bios_event_log_end = chip.log.bios_event_log + resource_size(&res);
    return chip.flags & TPM_CHIP_FLAG_TPM2 ? EFI_TCG2_EVENT_LOG_FORMAT_TCG_2 :
    EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2;
    }
#[no_mangle]
pub unsafe extern "C" fn tpm_read_log_of(chip: *mut tpm_chip) -> c_int {
    int tpm_read_log_of(struct tpm_chip *chip)
    {
    struct device_node *np;
    const u32 *sizep;
    const u64 *basep;
    struct tpm_bios_log *log;
    u32 size;
    u64 base;
    log = &chip.log;
    if (chip.dev.parent && chip.dev.parent.of_node)
    np = chip.dev.parent.of_node;
    else
    return -ENODEV;
    if (of_property_read_bool(np, "powered-while-suspended"))
    chip.flags |= TPM_CHIP_FLAG_ALWAYS_POWERED;
    sizep = of_get_property(np, "linux,sml-size", core::ptr::null_mut());
    basep = of_get_property(np, "linux,sml-base", core::ptr::null_mut());
    if (sizep == core::ptr::null_mut() && basep == core::ptr::null_mut())
    return tpm_read_log_memory_region(chip);
    if (sizep == core::ptr::null_mut() || basep == core::ptr::null_mut())
    return -EIO;
//
// For both vtpm/tpm, firmware has log addr and log size in big
// endian format. But in case of vtpm, there is a method called
// sml-handover which is run during kernel init even before
// device tree is setup. This sml-handover function takes care
// of endianness and writes to sml-base and sml-size in little
// endian format. For this reason, vtpm doesn't need conversion
// but physical tpm needs the conversion.
//
    if (of_property_match_string(np, "compatible", "IBM,vtpm") < 0 &&
    of_property_match_string(np, "compatible", "IBM,vtpm20") < 0) {
    size = be32_to_cpup(( __be32 *)sizep);
    base = be64_to_cpup(( __be64 *)basep);
    } else {
    size = *sizep;
    base = *basep;
    }
    if (size == 0) {
    dev_warn(&chip.dev, "%s: Event log area empty\n", __func__);
    return -EIO;
    }
    log.bios_event_log = devm_kmemdup(&chip.dev, __va(base), size, GFP_KERNEL);
    if (!log.bios_event_log)
    return -ENOMEM;
    log.bios_event_log_end = log.bios_event_log + size;
    if (chip.flags & TPM_CHIP_FLAG_TPM2)
    return EFI_TCG2_EVENT_LOG_FORMAT_TCG_2;
    return EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2;
    }
