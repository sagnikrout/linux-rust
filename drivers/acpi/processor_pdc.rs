//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/processor_pdc.c
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
// Copyright (C) 2005 Intel Corporation
// Copyright (C) 2009 Hewlett-Packard Development Company, L.P.
//
// Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
// - Added _PDC for platforms with Intel CPUs
//

#[no_mangle]
unsafe extern "C" fn acpi_set_pdc_bits(buf: *mut u32) {
    static void acpi_set_pdc_bits(u32 *buf)
    {
    buf[0] = ACPI_PDC_REVISION_ID;
    buf[1] = 1;
    buf[2] = 0;
// Twiddle arch-specific bits needed for _PDC
    arch_acpi_set_proc_cap_bits(&buf[2]);
    }
    static struct acpi_object_list *acpi_processor_alloc_pdc(void)
    {
    struct acpi_object_list *obj_list;
    union acpi_object *obj;
    u32 *buf;
// allocate and initialize pdc. It will be used later.
    obj_list = kmalloc_obj(struct acpi_object_list);
    if (!obj_list)
    goto out;
    obj = kmalloc_obj(union acpi_object);
    if (!obj) {
    kfree(obj_list);
    goto out;
    }
    buf = kmalloc(12, GFP_KERNEL);
    if (!buf) {
    kfree(obj);
    kfree(obj_list);
    goto out;
    }
    acpi_set_pdc_bits(buf);
    obj.type = ACPI_TYPE_BUFFER;
    obj.buffer.length = 12;
    obj.buffer.pointer = (u8 *) buf;
    obj_list.count = 1;
    obj_list.pointer = obj;
    return obj_list;
    out:
    pr_err("Memory allocation error\n");
    return core::ptr::null_mut();
    }
//
// _PDC is required for a BIOS-OS handshake for most of the newer
// ACPI processor features.
//
    static acpi_status
    acpi_processor_eval_pdc(acpi_handle handle, struct acpi_object_list *pdc_in)
    {
    let mut status: acpi_status = AE_OK;
    status = acpi_evaluate_object(handle, "_PDC", pdc_in, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    acpi_handle_debug(handle,
    "Could not evaluate _PDC, using legacy perf control\n");
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_processor_set_pdc(handle: acpi_handle) {
    void acpi_processor_set_pdc(acpi_handle handle)
    {
    struct acpi_object_list *obj_list;
    if (arch_has_acpi_pdc() == false)
    return;
    obj_list = acpi_processor_alloc_pdc();
    if (!obj_list)
    return;
    acpi_processor_eval_pdc(handle, obj_list);
    kfree(obj_list.pointer.buffer.pointer);
    kfree(obj_list.pointer);
    kfree(obj_list);
    }
    static acpi_status __init
    early_init_pdc(acpi_handle handle, u32 lvl, void *context, void **rv)
    {
    if (processor_physically_present(handle) == false)
    return AE_OK;
    acpi_processor_set_pdc(handle);
    return AE_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_early_processor_set_pdc() -> void __init {
    void __init acpi_early_processor_set_pdc(void)
    {
    acpi_proc_quirk_mwait_check();
    acpi_walk_namespace(ACPI_TYPE_PROCESSOR, ACPI_ROOT_OBJECT,
    ACPI_UINT32_MAX,
    early_init_pdc, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    acpi_get_devices(ACPI_PROCESSOR_DEVICE_HID, early_init_pdc, core::ptr::null_mut(), core::ptr::null_mut());
    }
