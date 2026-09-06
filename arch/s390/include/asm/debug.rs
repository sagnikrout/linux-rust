//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/debug.h
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
// S/390 debug facility
//
// Copyright IBM Corp. 1999, 2020
//

// the entry information

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __debug_entry {
    pub 60: unsigned long clock :,
    pub 1: unsigned long exception :,
    pub 3: unsigned long level :,
    pub caller: *mut c_void,
    pub cpu: c_ushort,
    pub __packed: },
pub type debug_entry_t = __debug_entry;
    pub debug_view: struct,
    pub next: *mut debug_info,
    pub prev: *mut debug_info,
    pub ref_count: refcount_t,
    pub lock: raw_spinlock_t,
    pub level: c_int,
    pub nr_areas: c_int,
    pub pages_per_area: c_int,
    pub buf_size: c_int,
    pub entry_size: c_int,
    pub areas: *mut debug_entry_t,
    pub active_area: c_int,
    pub active_pages: *mut c_int,
    pub active_entries: *mut c_int,
    pub debugfs_root_entry: *mut dentry,
    pub debugfs_entries: [*mut dentry; DEBUG_MAX_VIEWS],
    pub views: [*mut debug_view; DEBUG_MAX_VIEWS],
    pub name: [c_char; DEBUG_MAX_NAME_LEN],
    pub mode: umode_t,
    pub debug_info_t: },
    pub out_buf_size): *mut *mut char out_buf, size_t,
    pub in_buf): *const c_char,
    pub out_buf_size): *mut *mut char out_buf, size_t,
    pub offset): *mut size_t in_buf_size, loff_t,
    pub out_buf_size): *mut *mut char out_buf, size_t,
pub const DEBUG_SPRINTF_MAX_ARGS: c_int = 10;
    pub inbuf): *const c_char,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_view {
    pub name: [c_char; DEBUG_MAX_NAME_LEN],
    pub prolog_proc: *mut debug_prolog_proc_t,
    pub header_proc: *mut debug_header_proc_t,
    pub format_proc: *mut debug_format_proc_t,
    pub input_proc: *mut debug_input_proc_t,
    pub private_data: *mut c_void,
}

// do NOT use the _common functions
// Debug Feature API:
extern "C" {
    pub fn debug_unregister(id: *mut debug_info_t);
}
extern "C" {
    pub fn debug_set_level(id: *mut debug_info_t, new_level: c_int);
}
extern "C" {
    pub fn debug_set_critical();
}
extern "C" {
    pub fn debug_stop_all();
}
//
// debug_level_enabled() - Returns true if debug events for the specified
// level would be logged. Otherwise returns false.
//
// @id:		handle for debug log
// @level:	debug level
//
// Return:
// - %true if level is less or equal to the current debug level.
//
// debug_event() - writes binary debug entry to active debug area
// (if level <= actual debug level)
//
// @id:		handle for debug log
// @level:	debug level
// @data:	pointer to data for debug entry
// @length:	length of data in bytes
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_event_common(_arg: id, _arg: level, _arg: data, _arg: length) -> return;
}
//
// debug_int_event() - writes unsigned integer debug entry to active debug area
// (if level <= actual debug level)
//
// @id:		handle for debug log
// @level:	debug level
// @tag:	integer value for debug entry
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_event_common(_arg: id, _arg: level, _arg: &t, int): sizeof(unsigned) -> return;
}
//
// debug_long_event() - writes unsigned long debug entry to active debug area
// (if level <= actual debug level)
//
// @id:		handle for debug log
// @level:	debug level
// @tag:	long integer value for debug entry
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_event_common(_arg: id, _arg: level, _arg: &t, long): sizeof(unsigned) -> return;
}
//
// debug_text_event() - writes string debug entry in ascii format to active
// debug area (if level <= actual debug level)
//
// @id:		handle for debug log
// @level:	debug level
// @txt:	string for debug entry
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_event_common(_arg: id, _arg: level, _arg: txt, _arg: strlen(txt)) -> return;
}
//
// IMPORTANT: Use "%s" in sprintf format strings with care! Only pointers are
// stored in the s390dbf. See Documentation/arch/s390/s390dbf.rst for more details!
//
// debug_sprintf_event() - writes debug entry with format string
// and varargs (longs) to active debug area
// (if level $<=$ actual debug level).
//
// @_id:	handle for debug log
// @_level:	debug level
// @_fmt:	format string for debug entry
// @...:	varargs used as in sprintf()
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
// floats and long long datatypes cannot be used as varargs.
//

//
// debug_exception() - writes binary debug entry to active debug area
// (if level <= actual debug level)
// and switches to next debug area
//
// @id:		handle for debug log
// @level:	debug level
// @data:	pointer to data for debug entry
// @length:	length of data in bytes
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_exception_common(_arg: id, _arg: level, _arg: data, _arg: length) -> return;
}
//
// debug_int_exception() - writes unsigned int debug entry to active debug area
// (if level <= actual debug level)
// and switches to next debug area
//
// @id:		handle for debug log
// @level:	debug level
// @tag:	integer value for debug entry
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_exception_common(_arg: id, _arg: level, _arg: &t, int): sizeof(unsigned) -> return;
}
//
// debug_long_exception() - writes long debug entry to active debug area
// (if level <= actual debug level)
// and switches to next debug area
//
// @id:		handle for debug log
// @level:	debug level
// @tag:	long integer value for debug entry
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_exception_common(_arg: id, _arg: level, _arg: &t, long): sizeof(unsigned) -> return;
}
//
// debug_text_exception() - writes string debug entry in ascii format to active
// debug area (if level <= actual debug level)
// and switches to next debug area
// area
//
// @id:	handle for debug log
// @level:	debug level
// @txt:	string for debug entry
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
extern "C" {
    pub fn debug_exception_common(_arg: id, _arg: level, _arg: txt, _arg: strlen(txt)) -> return;
}
//
// IMPORTANT: Use "%s" in sprintf format strings with care! Only pointers are
// stored in the s390dbf. See Documentation/arch/s390/s390dbf.rst for more details!
//
// debug_sprintf_exception() - writes debug entry with format string and
// varargs (longs) to active debug area
// (if level <= actual debug level)
// and switches to next debug area.
//
// @_id:	handle for debug log
// @_level:	debug level
// @_fmt:	format string for debug entry
// @...:	varargs used as in sprintf()
//
// Return:
// - Address of written debug entry
// - %NULL if error
//
// floats and long long datatypes cannot be used as varargs.
//

extern "C" {
    pub fn debug_register_view(id: *mut debug_info_t, view: *mut debug_view) -> c_int;
}
extern "C" {
    pub fn debug_unregister_view(id: *mut debug_info_t, view: *mut debug_view) -> c_int;
}

//
// Note: Initial page and area numbers must be fixed to allow static
// initialization. This enables very early tracing. Changes to these values
// must be reflected in __DEFINE_STATIC_AREA.
//
pub const EARLY_PAGES: c_int = 8;
pub const EARLY_AREAS: c_int = 1;

//
// Define static areas for early trace data. During boot debug_register_static()
// will replace these with dynamically allocated areas to allow custom page and
// area sizes, and dynamic resizing.
//

//
// DEFINE_STATIC_DEBUG_INFO - Define static debug_info_t
//
// @var: Name of debug_info_t variable
// @name: Name of debug log (e.g. used for debugfs entry)
// @pages: Number of pages per area
// @nr_areas: Number of debug areas
// @buf_size: Size of data area in each debug entry
// @view: Pointer to debug view struct
//
// Define a static debug_info_t for early tracing. The associated debugfs log
// is automatically registered with the specified debug view.
//
// Important: Users of this macro must not call any of the
// debug_register/_unregister() functions for this debug_info_t!
//
// Note: Tracing will start with a fixed number of initial pages and areas.
// The debug area will be changed to use the specified numbers during
// arch_initcall.
//

extern "C" {
    pub fn debug_register_static(id: *mut debug_info_t, pages_per_area: c_int, nr_areas: c_int);
}

