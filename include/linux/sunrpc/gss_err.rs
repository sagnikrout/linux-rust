//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/gss_err.h
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
// linux/include/sunrpc/gss_err.h
//
// Adapted from MIT Kerberos 5-1.2.1 include/gssapi/gssapi.h
//
// Copyright (c) 2002 The Regents of the University of Michigan.
// All rights reserved.
//
// Andy Adamson   <andros@umich.edu>
//
// Copyright 1993 by OpenVision Technologies, Inc.
//
// Permission to use, copy, modify, distribute, and sell this software
// and its documentation for any purpose is hereby granted without fee,
// provided that the above copyright notice appears in all copies and
// that both that copyright notice and this permission notice appear in
// supporting documentation, and that the name of OpenVision not be used
// in advertising or publicity pertaining to distribution of the software
// without specific, written prior permission. OpenVision makes no
// representations about the suitability of this software for any
// purpose.  It is provided "as is" without express or implied warranty.
//
// OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
// USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//
pub type OM_uint32 = c_uint;
//
// Flag bits for context-level services.
//
pub const GSS_C_DELEG_FLAG: c_int = 1;
pub const GSS_C_MUTUAL_FLAG: c_int = 2;
pub const GSS_C_REPLAY_FLAG: c_int = 4;
pub const GSS_C_SEQUENCE_FLAG: c_int = 8;
pub const GSS_C_CONF_FLAG: c_int = 16;
pub const GSS_C_INTEG_FLAG: c_int = 32;
pub const GSS_C_ANON_FLAG: c_int = 64;
pub const GSS_C_PROT_READY_FLAG: c_int = 128;
pub const GSS_C_TRANS_FLAG: c_int = 256;
//
// Credential usage options
//
pub const GSS_C_BOTH: c_int = 0;
pub const GSS_C_INITIATE: c_int = 1;
pub const GSS_C_ACCEPT: c_int = 2;
//
// Status code types for gss_display_status
//
pub const GSS_C_GSS_CODE: c_int = 1;
pub const GSS_C_MECH_CODE: c_int = 2;
//
// Expiration time of 2^32-1 seconds means infinite lifetime for a
// credential or security context
//

// Major status codes
pub const GSS_S_COMPLETE: c_int = 0;
//
// Some "helper" definitions to make the status code macros obvious.
//
pub const GSS_C_CALLING_ERROR_OFFSET: c_int = 24;
pub const GSS_C_ROUTINE_ERROR_OFFSET: c_int = 16;
pub const GSS_C_SUPPLEMENTARY_OFFSET: c_int = 0;

//
// The macros that test status codes for error conditions.  Note that the
// GSS_ERROR() macro has changed slightly from the V1 GSSAPI so that it now
// evaluates its argument only once.
//

//
// Now the actual status code definitions
//
// Calling errors:
//

//
// Routine errors:
//

//
// Supplementary info bits:
//

// XXXX these are not part of the GSSAPI C bindings!  (but should be)

// XXXX This is a necessary evil until the spec is fixed

