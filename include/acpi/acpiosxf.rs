//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acpiosxf.h
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
// Name: acpiosxf.h - All interfaces to the OS Services Layer (OSL). These
// interfaces must be implemented by OSL to interface the
// ACPI components to the host operating system.
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

// Types for acpi_os_execute

pub const ACPI_MUTEX_SEM: c_int = 1;
// Functions for acpi_os_signal
pub const ACPI_SIGNAL_FATAL: c_int = 0;
pub const ACPI_SIGNAL_BREAKPOINT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_signal_fatal_info {
    pub type: u32,
    pub code: u32,
    pub argument: u32,
}

//
// OSL Initialization and shutdown primitives
//

extern "C" {
    pub fn acpi_os_initialize() -> acpi_status;
}

extern "C" {
    pub fn acpi_os_terminate() -> acpi_status;
}

//
// ACPI Table interfaces
//

extern "C" {
    pub fn acpi_os_get_root_pointer() -> acpi_physical_address;
}

//
// Spinlock primitives
//

extern "C" {
    pub fn acpi_os_create_lock(out_handle: *mut *mut acpi_spinlock) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_delete_lock(handle: acpi_spinlock);
}

extern "C" {
    pub fn acpi_os_acquire_lock(handle: acpi_spinlock) -> acpi_cpu_flags;
}

extern "C" {
    pub fn acpi_os_release_lock(handle: acpi_spinlock, flags: acpi_cpu_flags);
}

//
// RAW spinlock primitives. If the OS does not provide them, fallback to
// spinlock primitives
//

//
// Semaphore primitives
//

extern "C" {
    pub fn acpi_os_delete_semaphore(handle: acpi_semaphore) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_signal_semaphore(handle: acpi_semaphore, units: u32) -> acpi_status;
}

//
// Mutex primitives. May be configured to use semaphores instead via
// ACPI_MUTEX_TYPE (see platform/acenv.h)
//

extern "C" {
    pub fn acpi_os_create_mutex(out_handle: *mut *mut acpi_mutex) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_delete_mutex(handle: acpi_mutex);
}

extern "C" {
    pub fn acpi_os_acquire_mutex(handle: acpi_mutex, timeout: u16) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_release_mutex(handle: acpi_mutex);
}

//
// Memory allocation and mapping
//

extern "C" {
    pub fn acpi_os_free(memory: *mut c_void);
}

extern "C" {
    pub fn acpi_os_unmap_memory(logical_address: *mut c_void, size: acpi_size);
}

//
// Memory/Object Cache
//

extern "C" {
    pub fn acpi_os_delete_cache(cache: *mut *mut acpi_cache_t) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_purge_cache(cache: *mut *mut acpi_cache_t) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_release_object(cache: *mut *mut acpi_cache_t, object: *mut c_void) -> acpi_status;
}

//
// Interrupt handlers
//

//
// Threads and Scheduling
//

extern "C" {
    pub fn acpi_os_get_thread_id() -> acpi_thread_id;
}

extern "C" {
    pub fn acpi_os_wait_events_complete();
}

extern "C" {
    pub fn acpi_os_sleep(milliseconds: u64);
}

extern "C" {
    pub fn acpi_os_stall(microseconds: u32);
}

//
// Platform and hardware-independent I/O interfaces
//

extern "C" {
    pub fn acpi_os_read_port(address: acpi_io_address, value: *mut u32, width: u32) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_write_port(address: acpi_io_address, value: u32, width: u32) -> acpi_status;
}

//
// Platform and hardware-independent physical memory interfaces
//
extern "C" {
    pub fn acpi_os_read_iomem(virt_addr: *mut void __iomem, value: *mut u64, width: u32) -> c_int;
}

//
// Platform and hardware-independent PCI configuration space access
// Note: Can't use "Register" as a parameter, changed to "Reg" --
// certain compilers complain.
//

//
// Miscellaneous
//

extern "C" {
    pub fn acpi_os_readable(pointer: *mut c_void, length: acpi_size) -> u8;
}

extern "C" {
    pub fn acpi_os_writable(pointer: *mut c_void, length: acpi_size) -> u8;
}

extern "C" {
    pub fn acpi_os_get_timer() -> u64;
}

extern "C" {
    pub fn acpi_os_signal(function: u32, info: *mut c_void) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_enter_sleep(sleep_state: u8, rega_value: u32, regb_value: u32) -> acpi_status;
}

//
// Debug print routines
//

extern "C" {
    pub fn acpi_os_printf(format: *const c_char, ...) -> void ACPI_INTERNAL_VAR_XFACE;
}

extern "C" {
    pub fn acpi_os_vprintf(format: *const c_char, args: va_list);
}

extern "C" {
    pub fn acpi_os_redirect_output(destination: *mut c_void);
}

//
// Debug IO
//

extern "C" {
    pub fn acpi_os_get_line(buffer: *mut c_char, buffer_length: u32, bytes_read: *mut u32) -> acpi_status;
}

extern "C" {
    pub fn acpi_os_initialize_debugger() -> acpi_status;
}

extern "C" {
    pub fn acpi_os_terminate_debugger();
}

extern "C" {
    pub fn acpi_os_wait_command_ready() -> acpi_status;
}

extern "C" {
    pub fn acpi_os_notify_command_complete() -> acpi_status;
}

//
// Obtain ACPI table(s)
//

//
// Directory manipulation
//

// requeste_file_type values
pub const REQUEST_FILE_ONLY: c_int = 0;
pub const REQUEST_DIR_ONLY: c_int = 1;

extern "C" {
    pub fn acpi_os_close_directory(dir_handle: *mut c_void);
}

