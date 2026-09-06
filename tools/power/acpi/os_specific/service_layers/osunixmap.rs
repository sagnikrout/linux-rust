//! Automatically rewritten from C to Rust
//! Source: tools/power/acpi/os_specific/service_layers/osunixmap.c
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
// Module Name: osunixmap - Unix OSL for file mappings
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("osunixmap")

pub const O_BINARY: c_int = 0;

//
// FUNCTION:    acpi_os_get_page_size
//
// PARAMETERS:  None
//
// RETURN:      Page size of the platform.
//
// DESCRIPTION: Obtain page size of the platform.
//
#[no_mangle]
unsafe extern "C" fn acpi_os_get_page_size() -> acpi_size {
    static acpi_size acpi_os_get_page_size(void)
    {

    return PAGE_SIZE;

    return sysconf(_SC_PAGESIZE);

    }
//
// FUNCTION:    acpi_os_map_memory
//
// PARAMETERS:  where               - Physical address of memory to be mapped
// length              - How much memory to map
//
// RETURN:      Pointer to mapped memory. Null on error.
//
// DESCRIPTION: Map physical memory into local address space.
//
    void *acpi_os_map_memory(acpi_physical_address where, acpi_size length)
    {
    u8 *mapped_memory;
    acpi_physical_address offset;
    acpi_size page_size;
    int fd;
    fd = open(SYSTEM_MEMORY, O_RDONLY | O_BINARY);
    if (fd < 0) {
    fprintf(stderr, "Cannot open %s\n", SYSTEM_MEMORY);
    return (core::ptr::null_mut());
    }
// Align the offset to use mmap
    page_size = acpi_os_get_page_size();
    offset = where % page_size;
// Map the table header to get the length of the full table
    mapped_memory = mmap(core::ptr::null_mut(), (length + offset), PROT_READ, MMAP_FLAGS,
    fd, (where - offset));
    if (mapped_memory == MAP_FAILED) {
    fprintf(stderr, "Cannot map %s\n", SYSTEM_MEMORY);
    close(fd);
    return (core::ptr::null_mut());
    }
    close(fd);
    return (ACPI_CAST8(mapped_memory + offset));
    }
//
// FUNCTION:    acpi_os_unmap_memory
//
// PARAMETERS:  where               - Logical address of memory to be unmapped
// length              - How much memory to unmap
//
// RETURN:      None.
//
// DESCRIPTION: Delete a previously created mapping. Where and Length must
// correspond to a previous mapping exactly.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_os_unmap_memory(where: *mut c_void, length: acpi_size) {
    void acpi_os_unmap_memory(void *where, acpi_size length)
    {
    acpi_physical_address offset;
    acpi_size page_size;
    page_size = acpi_os_get_page_size();
    offset = ACPI_TO_INTEGER(where) % page_size;
    munmap((u8 *)where - offset, (length + offset));
    }
