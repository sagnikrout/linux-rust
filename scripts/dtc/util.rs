//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/dtc/util.h
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
// Copyright 2011 The Chromium Authors, All Rights Reserved.
// Copyright 2008 Jon Loeliger, Freescale Semiconductor, Inc.
//

// Macro flag: #define NORETURN

//
// Writes path to fp, escaping spaces with a backslash.
//
extern "C" {
    pub fn fprint_path_escaped(fp: *mut FILE, path: *const c_char);
}
extern "C" {
    pub fn PRINTF(_arg: 2, strp: *mut 3) xasprintf(char, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn PRINTF(_arg: 2, strp: *mut 3) xasprintf_append(char, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn PRINTF(_arg: 2, strp: *mut 0) xavsprintf_append(char, fmt: *const c_char, ap: va_list) -> c_int;
}
//
// Check a property of a given length to see if it is all printable and
// has a valid terminator. The property can contain either a single string,
// or multiple strings each of non-zero length.
//
// @param data	The string to check
// @param len	The string length including terminator
// @return 1 if a valid printable string, 0 if not
//
extern "C" {
    pub fn util_is_printable_string(data: *const c_void, len: c_int) -> bool;
}
//
// Parse an escaped character starting at index i in string s.  The resulting
// character will be returned and the index i will be updated to point at the
// character directly after the end of the encoding, this may be the '\0'
// terminator of the string.
//
extern "C" {
    pub fn get_escape_char(s: *const c_char, i: *mut c_int) -> c_char;
}
//
// Read a device tree file into a buffer. This will report any errors on
// stderr.
//
// @param filename	The filename to read, or - for stdin
// @param len		If non-NULL, the amount of data we managed to read
// @return Pointer to allocated buffer containing fdt, or NULL on error
//
// Read a device tree file into a buffer. Does not report errors, but only
// returns them. The value returned can be passed to strerror() to obtain
// an error message for the user.
//
// @param filename	The filename to read, or - for stdin
// @param buffp		Returns pointer to buffer containing fdt
// @param len		If non-NULL, the amount of data we managed to read
// @return 0 if ok, else an errno value representing the error
//
extern "C" {
    pub fn utilfdt_read_err(filename: *const c_char, buffp: *mut c_char, len: *mut usize) -> c_int;
}
//
// Write a device tree buffer to a file. This will report any errors on
// stderr.
//
// @param filename	The filename to write, or - for stdout
// @param blob		Pointer to buffer containing fdt
// @return 0 if ok, -1 on error
//
extern "C" {
    pub fn utilfdt_write(filename: *const c_char, blob: *const c_void) -> c_int;
}
//
// Write a device tree buffer to a file. Does not report errors, but only
// returns them. The value returned can be passed to strerror() to obtain
// an error message for the user.
//
// @param filename	The filename to write, or - for stdout
// @param blob		Pointer to buffer containing fdt
// @return 0 if ok, else an errno value representing the error
//
extern "C" {
    pub fn utilfdt_write_err(filename: *const c_char, blob: *const c_void) -> c_int;
}
//
// Decode a data type string. The purpose of this string
//
// The string consists of an optional character followed by the type:
// Modifier characters:
// hh or b	1 byte
// h	2 byte
// l	4 byte, default
//
// Type character:
// s	string
// i	signed integer
// u	unsigned integer
// x	hex
// r	raw
//
// TODO: Implement ll modifier (8 bytes)
// TODO: Implement o type (octal)
//
// @param fmt		Format string to process
// @param type		Returns type found(s/d/u/x), or 0 if none
// @param size		Returns size found(1,2,4,8) or 4 if none
// @return 0 if ok, -1 on error (no type given, or other invalid format)
//
extern "C" {
    pub fn utilfdt_decode_type(fmt: *const c_char, type: *mut c_int, size: *mut c_int) -> c_int;
}
//
// This is a usage message fragment for the -t option. It is the format
// supported by utilfdt_decode_type.
//

//
// Print property data in a readable format to stdout
//
// Properties that look like strings will be printed as strings. Otherwise
// the data will be displayed either as cells (if len is a multiple of 4
// bytes) or bytes.
//
// If len is 0 then this function does nothing.
//
// @param data	Pointers to property data
// @param len	Length of property data
//
extern "C" {
    pub fn utilfdt_print_data(data: *const c_char, len: c_int);
}
//
// Show source version and exit
//
extern "C" {
    pub fn util_version() -> void NORETURN;
}
//
// Show usage and exit
//
// This helps standardize the output of various utils.  You most likely want
// to use the usage() helper below rather than call this.
//
// @param errmsg	If non-NULL, an error message to display
// @param synopsis	The initial example usage text (and possible examples)
// @param short_opts	The string of short options
// @param long_opts	The structure of long options
// @param opts_help	An array of help strings (should align with long_opts)
//
// Show usage and exit
//
// If you name all your usage variables with usage_xxx, then you can call this
// help macro rather than expanding all arguments yourself.
//
// @param errmsg	If non-NULL, an error message to display
//

//
// Call getopt_long() with standard options
//
// Since all util code runs getopt in the same way, provide a helper.
//

// Helper for aligning long_opts array

// Helper for usage_short_opts string constant

// Helper for usage_long_opts option array

// Helper for usage_opts_help array

// Helper for getopt case statements

