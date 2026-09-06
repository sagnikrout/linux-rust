//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/exstorob.c
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
// Module Name: exstorob - AML object store support, store to object
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("exstorob")
//
// FUNCTION:    acpi_ex_store_buffer_to_buffer
//
// PARAMETERS:  source_desc         - Source object to copy
// target_desc         - Destination object of the copy
//
// RETURN:      Status
//
// DESCRIPTION: Copy a buffer object to another buffer object.
//
    acpi_status
    acpi_ex_store_buffer_to_buffer(union acpi_operand_object *source_desc,
    union acpi_operand_object *target_desc)
    {
    u32 length;
    u8 *buffer;
    ACPI_FUNCTION_TRACE_PTR(ex_store_buffer_to_buffer, source_desc);
// If Source and Target are the same, just return
    if (source_desc == target_desc) {
    return_ACPI_STATUS(AE_OK);
    }
// We know that source_desc is a buffer by now
    buffer = ACPI_CAST_PTR(u8, source_desc.buffer.pointer);
    length = source_desc.buffer.length;
//
// If target is a buffer of length zero or is a static buffer,
// allocate a new buffer of the proper length
//
    if ((target_desc.buffer.length == 0) ||
    (target_desc.common.flags & AOPOBJ_STATIC_POINTER)) {
    target_desc.buffer.pointer = ACPI_ALLOCATE(length);
    if (!target_desc.buffer.pointer) {
    return_ACPI_STATUS(AE_NO_MEMORY);
    }
    target_desc.buffer.length = length;
    }
// Copy source buffer to target buffer
    if (length <= target_desc.buffer.length) {
// Clear existing buffer and copy in the new one
    memset(target_desc.buffer.pointer, 0,
    target_desc.buffer.length);
    memcpy(target_desc.buffer.pointer, buffer, length);

//
// NOTE: ACPI versions up to 3.0 specified that the buffer must be
// truncated if the string is smaller than the buffer. However, "other"
// implementations of ACPI never did this and thus became the defacto
// standard. ACPI 3.0A changes this behavior such that the buffer
// is no longer truncated.
//
// OBSOLETE BEHAVIOR:
// If the original source was a string, we must truncate the buffer,
// according to the ACPI spec. Integer-to-Buffer and Buffer-to-Buffer
// copy must not truncate the original buffer.
//
    if (original_src_type == ACPI_TYPE_STRING) {
// Set the new length of the target
    target_desc.buffer.length = length;
    }

    } else {
// Truncate the source, copy only what will fit
    memcpy(target_desc.buffer.pointer, buffer,
    target_desc.buffer.length);
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "Truncating source buffer from %X to %X\n",
    length, target_desc.buffer.length));
    }
// Copy flags
    target_desc.buffer.flags = source_desc.buffer.flags;
    target_desc.common.flags &= ~AOPOBJ_STATIC_POINTER;
    return_ACPI_STATUS(AE_OK);
    }
//
// FUNCTION:    acpi_ex_store_string_to_string
//
// PARAMETERS:  source_desc         - Source object to copy
// target_desc         - Destination object of the copy
//
// RETURN:      Status
//
// DESCRIPTION: Copy a String object to another String object
//
    acpi_status
    acpi_ex_store_string_to_string(union acpi_operand_object *source_desc,
    union acpi_operand_object *target_desc)
    {
    u32 length;
    u8 *buffer;
    ACPI_FUNCTION_TRACE_PTR(ex_store_string_to_string, source_desc);
// If Source and Target are the same, just return
    if (source_desc == target_desc) {
    return_ACPI_STATUS(AE_OK);
    }
// We know that source_desc is a string by now
    buffer = ACPI_CAST_PTR(u8, source_desc.string.pointer);
    length = source_desc.string.length;
//
// Replace existing string value if it will fit and the string
// pointer is not a static pointer (part of an ACPI table)
//
    if ((length < target_desc.string.length) &&
    (!(target_desc.common.flags & AOPOBJ_STATIC_POINTER))) {
//
// String will fit in existing non-static buffer.
// Clear old string and copy in the new one
//
    memset(target_desc.string.pointer, 0,
    (acpi_size)target_desc.string.length + 1);
    memcpy(target_desc.string.pointer, buffer, length);
    } else {
//
// Free the current buffer, then allocate a new buffer
// large enough to hold the value
//
    if (target_desc.string.pointer &&
    (!(target_desc.common.flags & AOPOBJ_STATIC_POINTER))) {
// Only free if not a pointer into the DSDT
    ACPI_FREE(target_desc.string.pointer);
    }
    target_desc.string.pointer =
    ACPI_ALLOCATE_ZEROED((acpi_size)length + 1);
    if (!target_desc.string.pointer) {
    return_ACPI_STATUS(AE_NO_MEMORY);
    }
    target_desc.common.flags &= ~AOPOBJ_STATIC_POINTER;
    memcpy(target_desc.string.pointer, buffer, length);
    }
// Set the new target length
    target_desc.string.length = length;
    return_ACPI_STATUS(AE_OK);
    }
