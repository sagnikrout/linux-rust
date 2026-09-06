//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utxfmutex.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utxfmutex - external AML mutex access functions
//

    ACPI_MODULE_NAME("utxfmutex")
// Local prototypes
    static acpi_status
    acpi_ut_get_mutex_object(acpi_handle handle,
    acpi_string pathname,
    union acpi_operand_object **ret_obj);
//
// FUNCTION:    acpi_ut_get_mutex_object
//
// PARAMETERS:  handle              - Mutex or prefix handle (optional)
// pathname            - Mutex pathname (optional)
// ret_obj             - Where the mutex object is returned
//
// RETURN:      Status
//
// DESCRIPTION: Get an AML mutex object. The mutex node is pointed to by
// Handle:Pathname. Either Handle or Pathname can be NULL, but
// not both.
//
    static acpi_status
    acpi_ut_get_mutex_object(acpi_handle handle,
    acpi_string pathname,
    union acpi_operand_object **ret_obj)
    {
    struct acpi_namespace_node *mutex_node;
    union acpi_operand_object *mutex_obj;
    acpi_status status;
// Parameter validation
    if (!ret_obj || (!handle && !pathname)) {
    return (AE_BAD_PARAMETER);
    }
// Get a the namespace node for the mutex
    mutex_node = handle;
    if (pathname != core::ptr::null_mut()) {
    status =
    acpi_get_handle(handle, pathname,
    ACPI_CAST_PTR(acpi_handle, &mutex_node));
    if (ACPI_FAILURE(status)) {
    return (status);
    }
    }
// Ensure that we actually have a Mutex object
    if (!mutex_node || (mutex_node.type != ACPI_TYPE_MUTEX)) {
    return (AE_TYPE);
    }
// Get the low-level mutex object
    mutex_obj = acpi_ns_get_attached_object(mutex_node);
    if (!mutex_obj) {
    return (AE_NULL_OBJECT);
    }
// ret_obj = mutex_obj;
    return (AE_OK);
    }
//
// FUNCTION:    acpi_acquire_mutex
//
// PARAMETERS:  handle              - Mutex or prefix handle (optional)
// pathname            - Mutex pathname (optional)
// timeout             - Max time to wait for the lock (millisec)
//
// RETURN:      Status
//
// DESCRIPTION: Acquire an AML mutex. This is a device driver interface to
// AML mutex objects, and allows for transaction locking between
// drivers and AML code. The mutex node is pointed to by
// Handle:Pathname. Either Handle or Pathname can be NULL, but
// not both.
//
    acpi_status
    acpi_acquire_mutex(acpi_handle handle, acpi_string pathname, u16 timeout)
    {
    acpi_status status;
    union acpi_operand_object *mutex_obj;
// Get the low-level mutex associated with Handle:Pathname
    status = acpi_ut_get_mutex_object(handle, pathname, &mutex_obj);
    if (ACPI_FAILURE(status)) {
    return (status);
    }
// Acquire the OS mutex
    status = acpi_os_acquire_mutex(mutex_obj.mutex.os_mutex, timeout);
    return (status);
    }
    ACPI_EXPORT_SYMBOL(acpi_acquire_mutex)
//
// FUNCTION:    acpi_release_mutex
//
// PARAMETERS:  handle              - Mutex or prefix handle (optional)
// pathname            - Mutex pathname (optional)
//
// RETURN:      Status
//
// DESCRIPTION: Release an AML mutex. This is a device driver interface to
// AML mutex objects, and allows for transaction locking between
// drivers and AML code. The mutex node is pointed to by
// Handle:Pathname. Either Handle or Pathname can be NULL, but
// not both.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_release_mutex(handle: acpi_handle, pathname: acpi_string) -> acpi_status {
    acpi_status acpi_release_mutex(acpi_handle handle, acpi_string pathname)
    {
    acpi_status status;
    union acpi_operand_object *mutex_obj;
// Get the low-level mutex associated with Handle:Pathname
    status = acpi_ut_get_mutex_object(handle, pathname, &mutex_obj);
    if (ACPI_FAILURE(status)) {
    return (status);
    }
// Release the OS mutex
    acpi_os_release_mutex(mutex_obj.mutex.os_mutex);
    return (AE_OK);
    }
    ACPI_EXPORT_SYMBOL(acpi_release_mutex)
