//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nls/nls_ucs2_utils.h
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
// Some of the source code in this file came from fs/cifs/cifs_unicode.c
// and then via server/unicode.c
// cifs_unicode:  Unicode kernel case support
//
// Function:
// Convert a unicode character to upper or lower case using
// compressed tables.
//
// Copyright (c) International Business Machines  Corp., 2000,2009
//
// Notes:
// These APIs are based on the C library functions.  The semantics
// should match the C functions but with expanded size operands.
//
// The upper/lower functions are based on a table created by mkupr.
// This is a compressed table of upper and lower case conversion.
//

//
// Windows maps these to the user defined 16 bit Unicode range since they are
// reserved symbols (along with \ and /), otherwise illegal to store
// in filenames in NTFS
//

//
// UniStrcat:  Concatenate the second string to the first
//
// Returns:
// Address of the first string
//
// NULL*/;	/* To end of first string
// NULL*/;	/* copy string 2 over
//
// UniStrchr:  Find a character in a string
//
// Returns:
// Address of first occurrence of character in string
// or NULL if the character is not in the string
//
// UniStrcmp:  Compare two strings
//
// Returns:
// < 0:  First string is less than second
// = 0:  Strings are equal
// > 0:  First string is greater than second
//
// UniStrcpy:  Copy a string
//
// NULL*/;
//
// UniStrlen:  Return the length of a string (in 16 bit Unicode chars not bytes)
//
// UniStrnlen:  Return the length (in 16 bit Unicode chars not bytes) of a
// string (length limited)
//
// UniStrncat:  Concatenate length limited string
//
// NULL*/;
// ucs1 = 0;		/* Null terminate the result
//
// UniStrncmp:  Compare length limited string
//
// UniStrncmp_le:  Compare length limited string - native to little-endian
//
// UniStrncpy:  Copy length limited string with pad
//
// ucs1++ = *ucs2++;
// ucs1++ = 0;
//
// UniStrncpy_le:  Copy length limited string with pad to little-endian
//
// ucs1++ = __le16_to_cpu(*ucs2++);
// ucs1++ = 0;
//
// UniStrstr:  Find a string in a string
//
// Returns:
// Address of first match found
// NULL if no matching string is found
//
// Partial match found

//
// UniToupper:  Convert a unicode character to upper case
//
// Latin characters
//
// UniStrupr:  Upper case a unicode string
//
// up = cpu_to_le16(UniToupper(le16_to_cpu(*up)));

