//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_print.h
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


//
// Copyright (C) 2016 Red Hat
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors:
// Rob Clark <robdclark@gmail.com>
//

// Do *not* use outside of drm_print.[ch]!
//
// DOC: print
//
// A simple wrapper for dev_printk(), seq_printf(), etc.  Allows same
// debug code to be used for both debugfs and printk logging.
//
// For example::
//
// void log_some_info(struct drm_printer *p)
// {
// drm_printf(p, "foo=%d\n", foo);
// drm_printf(p, "bar=%d\n", bar);
// }
//
// #ifdef CONFIG_DEBUG_FS
// void debugfs_show(struct seq_file *f)
// {
// struct drm_printer p = drm_seq_file_printer(f);
// log_some_info(&p);
// }
// #endif
//
// void some_other_function(...)
// {
// struct drm_printer p = drm_info_printer(drm->dev);
// log_some_info(&p);
// }
//
// enum drm_debug_category - The DRM debug categories
//
// Each of the DRM debug logging macros use a specific category, and the logging
// is filtered by the drm.debug module parameter. This enum specifies the values
// for the interface.
//
// Each DRM_DEBUG_<CATEGORY> macro logs to DRM_UT_<CATEGORY> category, except
// DRM_DEBUG() logs to DRM_UT_CORE.
//
// Enabling verbose debug messages is done through the drm.debug parameter, each
// category being enabled by a bit:
//
// - drm.debug=0x1 will enable CORE messages
// - drm.debug=0x2 will enable DRIVER messages
// - drm.debug=0x3 will enable CORE and DRIVER messages
// - ...
// - drm.debug=0x3ff will enable all messages
//
// An interesting feature is that it's possible to enable verbose logging at
// run-time by echoing the debug value in its sysfs node::
//
// # echo 0xf > /sys/module/drm/parameters/debug
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_debug_category {
// These names must match those in DYNAMIC_DEBUG_CLASSBITS
//
// @DRM_UT_CORE: Used in the generic drm code: drm_ioctl.c, drm_mm.c,
// drm_memory.c, ...
//
    DRM_UT_CORE,
//
// @DRM_UT_DRIVER: Used in the vendor specific part of the driver: i915,
// radeon, ... macro.
//
    DRM_UT_DRIVER,
//
// @DRM_UT_KMS: Used in the modesetting code.
//
    DRM_UT_KMS,
//
// @DRM_UT_PRIME: Used in the prime code.
//
    DRM_UT_PRIME,
//
// @DRM_UT_ATOMIC: Used in the atomic code.
//
    DRM_UT_ATOMIC,
//
// @DRM_UT_VBL: Used for verbose debug message in the vblank code.
//
    DRM_UT_VBL,
//
// @DRM_UT_STATE: Used for verbose atomic state debugging.
//
    DRM_UT_STATE,
//
// @DRM_UT_LEASE: Used in the lease code.
//
    DRM_UT_LEASE,
//
// @DRM_UT_DP: Used in the DP code.
//
    DRM_UT_DP,
//
// @DRM_UT_DRMRES: Used in the drm managed resources code.
//
    DRM_UT_DRMRES
}

extern "C" {
    pub fn unlikely(BIT(category): __drm_debug &) -> return;
}

//
// the drm.debug API uses dyndbg, so each drm_*dbg macro/callsite gets
// a descriptor, and only enabled callsites are reachable.  They use
// the private macro to avoid re-testing the enable-bit.
//

//
// struct drm_printer - drm output "stream"
//
// Do not use struct members directly.  Use drm_printer_seq_file(),
// drm_printer_info(), etc to initialize.  And drm_printf() for output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_printer {
// private:
    pub vaf): *mut *mut *mut void (printfn)(struct drm_printer p, struct va_format,
    pub str): *const *const *const void (puts)(struct drm_printer p, char,
    pub arg: *mut c_void,
    pub origin: *const c_void,
    pub prefix: *const c_char,
    pub series: c_uint,
    pub counter: c_uint,
    pub line: },
    pub category: drm_debug_category,
}

extern "C" {
    pub fn __drm_printfn_coredump(p: *mut drm_printer, vaf: *mut va_format);
}
extern "C" {
    pub fn __drm_puts_coredump(p: *mut drm_printer, str: *const c_char);
}
extern "C" {
    pub fn __drm_printfn_seq_file(p: *mut drm_printer, vaf: *mut va_format);
}
extern "C" {
    pub fn __drm_puts_seq_file(p: *mut drm_printer, str: *const c_char);
}
extern "C" {
    pub fn __drm_printfn_info(p: *mut drm_printer, vaf: *mut va_format);
}
extern "C" {
    pub fn __drm_printfn_dbg(p: *mut drm_printer, vaf: *mut va_format);
}
extern "C" {
    pub fn __drm_printfn_err(p: *mut drm_printer, vaf: *mut va_format);
}
extern "C" {
    pub fn __drm_printfn_line(p: *mut drm_printer, vaf: *mut va_format);
}
extern "C" {
    pub fn drm_printf(p: *mut drm_printer, f: *const c_char, ...);
}
extern "C" {
    pub fn drm_puts(p: *mut drm_printer, str: *const c_char);
}
extern "C" {
    pub fn drm_print_regset32(p: *mut drm_printer, regset: *mut debugfs_regset32);
}
//
// drm_vprintf - print to a &drm_printer stream
// @p: the &drm_printer
// @fmt: format string
// @va: the va_list
//
// drm_printf_indent - Print to a &drm_printer stream with indentation
// @printer: DRM printer
// @indent: Tab indentation level (max 5)
// @fmt: Format string
//

//
// struct drm_print_iterator - local struct used with drm_printer_coredump
// @data: Pointer to the devcoredump output buffer, can be NULL if using
// drm_printer_coredump to determine size of devcoredump
// @start: The offset within the buffer to start writing
// @remain: The number of bytes to write for this iteration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_print_iterator {
    pub data: *mut c_void,
    pub start: isize,
    pub remain: isize,
// private:
    pub offset: isize,
}

//
// drm_coredump_printer - construct a &drm_printer that can output to a buffer
// from the read function for devcoredump
// @iter: A pointer to a struct drm_print_iterator for the read instance
//
// This wrapper extends drm_printf() to work with a dev_coredumpm() callback
// function. The passed in drm_print_iterator struct contains the buffer
// pointer, size and offset as passed in from devcoredump.
//
// For example::
//
// void coredump_read(char *buffer, loff_t offset, size_t count,
// void *data, size_t datalen)
// {
// struct drm_print_iterator iter;
// struct drm_printer p;
//
// iter.data = buffer;
// iter.start = offset;
// iter.remain = count;
//
// p = drm_coredump_printer(&iter);
//
// drm_printf(p, "foo=%d\n", foo);
// }
//
// void makecoredump(...)
// {
// ...
// dev_coredumpm(dev, THIS_MODULE, data, 0, GFP_KERNEL,
// coredump_read, ...)
// }
//
// The above example has a time complexity of O(N^2), where N is the size of the
// devcoredump. This is acceptable for small devcoredumps but scales poorly for
// larger ones.
//
// Another use case for drm_coredump_printer is to capture the devcoredump into
// a saved buffer before the dev_coredump() callback. This involves two passes:
// one to determine the size of the devcoredump and another to print it to a
// buffer. Then, in dev_coredump(), copy from the saved buffer into the
// devcoredump read buffer.
//
// For example::
//
// char *devcoredump_saved_buffer;
//
// ssize_t __coredump_print(char *buffer, ssize_t count, ...)
// {
// struct drm_print_iterator iter;
// struct drm_printer p;
//
// iter.data = buffer;
// iter.start = 0;
// iter.remain = count;
//
// p = drm_coredump_printer(&iter);
//
// drm_printf(p, "foo=%d\n", foo);
// ...
// return count - iter.remain;
// }
//
// void coredump_print(...)
// {
// ssize_t count;
//
// count = __coredump_print(NULL, INT_MAX, ...);
// devcoredump_saved_buffer = kvmalloc(count, GFP_KERNEL);
// __coredump_print(devcoredump_saved_buffer, count, ...);
// }
//
// void coredump_read(char *buffer, loff_t offset, size_t count,
// void *data, size_t datalen)
// {
// ...
// memcpy(buffer, devcoredump_saved_buffer + offset, count);
// ...
// }
//
// The above example has a time complexity of O(N*2), where N is the size of the
// devcoredump. This scales better than the previous example for larger
// devcoredumps.
//
// RETURNS:
// The &drm_printer object
//
// Set the internal offset of the iterator to zero
//
// drm_coredump_printer_is_full() - DRM coredump printer output is full
// @p: DRM coredump printer
//
// DRM printer output is full, useful to short circuit coredump printing once
// printer is full.
//
// RETURNS:
// True if DRM coredump printer output buffer is full, False otherwise
//
// drm_seq_file_printer - construct a &drm_printer that outputs to &seq_file
// @f:  the &struct seq_file to output to
//
// RETURNS:
// The &drm_printer object
//
// drm_info_printer - construct a &drm_printer that outputs to dev_printk()
// @dev: the &struct device pointer
//
// RETURNS:
// The &drm_printer object
//
// drm_dbg_printer - construct a &drm_printer for drm device specific output
// @drm: the &struct drm_device pointer, or NULL
// @category: the debug category to use
// @prefix: debug output prefix, or NULL for no prefix
//
// RETURNS:
// The &drm_printer object
//
// drm_err_printer - construct a &drm_printer that outputs to drm_err()
// @drm: the &struct drm_device pointer
// @prefix: debug output prefix, or NULL for no prefix
//
// RETURNS:
// The &drm_printer object
//
// drm_line_printer - construct a &drm_printer that prefixes outputs with line numbers
// @p: the &struct drm_printer which actually generates the output
// @prefix: optional output prefix, or NULL for no prefix
// @series: optional unique series identifier, or 0 to omit identifier in the output
//
// This printer can be used to increase the robustness of the captured output
// to make sure we didn't lost any intermediate lines of the output. Helpful
// while capturing some crash data.
//
// Example 1::
//
// void crash_dump(struct drm_device *drm)
// {
// static unsigned int id;
// struct drm_printer p = drm_err_printer(drm, "crash");
// struct drm_printer lp = drm_line_printer(&p, "dump", ++id);
//
// drm_printf(&lp, "foo");
// drm_printf(&lp, "bar");
// }
//
// Above code will print into the dmesg something like::
//
// [ ] 0000:00:00.0: [drm] *ERROR* crash dump 1.1: foo
// [ ] 0000:00:00.0: [drm] *ERROR* crash dump 1.2: bar
//
// Example 2::
//
// void line_dump(struct device *dev)
// {
// struct drm_printer p = drm_info_printer(dev);
// struct drm_printer lp = drm_line_printer(&p, NULL, 0);
//
// drm_printf(&lp, "foo");
// drm_printf(&lp, "bar");
// }
//
// Above code will print::
//
// [ ] 0000:00:00.0: [drm] 1: foo
// [ ] 0000:00:00.0: [drm] 2: bar
//
// RETURNS:
// The &drm_printer object
//
// struct device based logging
//
// Prefer drm_device based logging over device or printk based logging.
//
// DRM_DEV_ERROR() - Error output.
//
// NOTE: this is deprecated in favor of drm_err() or dev_err().
//
// @dev: device pointer
// @fmt: printf() like format string.
//

//
// DRM_DEV_ERROR_RATELIMITED() - Rate limited error output.
//
// NOTE: this is deprecated in favor of drm_err_ratelimited() or
// dev_err_ratelimited().
//
// @dev: device pointer
// @fmt: printf() like format string.
//
// Like DRM_ERROR() but won't flood the log.
//

// NOTE: this is deprecated in favor of drm_info() or dev_info().

// NOTE: this is deprecated in favor of drm_info_once() or dev_info_once().

//
// DRM_DEV_DEBUG() - Debug output for generic drm code
//
// NOTE: this is deprecated in favor of drm_dbg_core().
//
// @dev: device pointer
// @fmt: printf() like format string.
//

//
// DRM_DEV_DEBUG_DRIVER() - Debug output for vendor specific part of the driver
//
// NOTE: this is deprecated in favor of drm_dbg() or dev_dbg().
//
// @dev: device pointer
// @fmt: printf() like format string.
//

//
// DRM_DEV_DEBUG_KMS() - Debug output for modesetting code
//
// NOTE: this is deprecated in favor of drm_dbg_kms().
//
// @dev: device pointer
// @fmt: printf() like format string.
//

//
// struct drm_device based logging
//
// Prefer drm_device based logging over device or prink based logging.
//
// Helper to enforce struct drm_device type
// Helper for struct drm_device based logging.

//
// printk based logging
//
// Prefer drm_device based logging over device or prink based logging.
//
extern "C" {
    pub fn __drm_err(format: *const c_char, ...);
}

// Macros to make printk easier

// NOTE: this is deprecated in favor of pr_info().

// NOTE: this is deprecated in favor of pr_notice().

// NOTE: this is deprecated in favor of pr_warn().

// NOTE: this is deprecated in favor of pr_info_once().

// NOTE: this is deprecated in favor of pr_notice_once().

// NOTE: this is deprecated in favor of pr_warn_once().

// NOTE: this is deprecated in favor of pr_err().

// NOTE: this is deprecated in favor of pr_err_ratelimited().

// NOTE: this is deprecated in favor of drm_dbg_core(NULL, ...).

// NOTE: this is deprecated in favor of drm_dbg(NULL, ...).

// NOTE: this is deprecated in favor of drm_dbg_kms(NULL, ...).

// NOTE: this is deprecated in favor of drm_dbg_prime(NULL, ...).

// NOTE: this is deprecated in favor of drm_dbg_atomic(NULL, ...).

// NOTE: this is deprecated in favor of drm_dbg_vbl(NULL, ...).

// NOTE: this is deprecated in favor of drm_dbg_lease(NULL, ...).

// NOTE: this is deprecated in favor of drm_dbg_dp(NULL, ...).

//
// struct drm_device based WARNs
//
// drm_WARN*() acts like WARN*(), but with the key difference of
// using device specific information so that we know from which device
// warning is originating from.
//
// Prefer drm_device based drm_WARN* over regular WARN
//
// Helper for struct drm_device based WARNs

