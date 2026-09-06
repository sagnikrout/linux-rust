//! Automatically rewritten from C to Rust
//! Source: tools/power/acpi/common/cmfsize.c
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
// Module Name: cmfsize - Common get file size function
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("cmfsize")
//
// FUNCTION:    cm_get_file_size
//
// PARAMETERS:  file                    - Open file descriptor
//
// RETURN:      File Size. On error, -1 (ACPI_UINT32_MAX)
//
// DESCRIPTION: Get the size of a file. Uses seek-to-EOF. File must be open.
// Does not disturb the current file pointer.
//
#[no_mangle]
pub unsafe extern "C" fn cm_get_file_size(file: ACPI_FILE) -> u32 {
    u32 cm_get_file_size(ACPI_FILE file)
    {
    long file_size;
    long current_offset;
    acpi_status status;
// Save the current file pointer, seek to EOF to obtain file size
    current_offset = ftell(file);
    if (current_offset < 0) {
    goto offset_error;
    }
    status = fseek(file, 0, SEEK_END);
    if (ACPI_FAILURE(status)) {
    goto seek_error;
    }
    file_size = ftell(file);
    if (file_size < 0) {
    goto offset_error;
    }
// Restore original file pointer
    status = fseek(file, current_offset, SEEK_SET);
    if (ACPI_FAILURE(status)) {
    goto seek_error;
    }
    return ((u32)file_size);
    offset_error:
    fprintf(stderr, "Could not get file offset\n");
    return (ACPI_UINT32_MAX);
    seek_error:
    fprintf(stderr, "Could not set file offset\n");
    return (ACPI_UINT32_MAX);
    }
