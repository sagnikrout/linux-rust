//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/ftl.h
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
// Derived from (and probably identical to):
// ftl.h 1.7 1999/10/25 20:23:17
//
// The contents of this file are subject to the Mozilla Public License
// Version 1.1 (the "License"); you may not use this file except in
// compliance with the License. You may obtain a copy of the License
// at http://www.mozilla.org/MPL
//
// Software distributed under the License is distributed on an "AS IS"
// basis, WITHOUT WARRANTY OF ANY KIND, either express or implied. See
// the License for the specific language governing rights and
// limitations under the License.
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//
// Alternatively, the contents of this file may be used under the
// terms of the GNU General Public License version 2 (the "GPL"), in
// which case the provisions of the GPL are applicable instead of the
// above.  If you wish to allow the use of your version of this file
// only under the terms of the GPL and not to allow others to use
// your version of this file under the MPL, indicate your decision by
// deleting the provisions above and replace them with the notice and
// other provisions required by the GPL.  If you do not delete the
// provisions above, a recipient may use your version of this file
// under either the MPL or the GPL.
//
// Flags in erase_unit_header_t
pub const HIDDEN_AREA: c_uint = 0x01;
pub const REVERSE_POLARITY: c_uint = 0x02;
pub const DOUBLE_BAI: c_uint = 0x04;
// Definitions for block allocation information

pub const BLOCK_CONTROL: c_uint = 0x30;
pub const BLOCK_DATA: c_uint = 0x40;
pub const BLOCK_REPLACEMENT: c_uint = 0x60;
pub const BLOCK_BAD: c_uint = 0x70;
