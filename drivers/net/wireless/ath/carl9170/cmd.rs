//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/cmd.h
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
// Atheros CARL9170 driver
//
// Basic HW register/memory/command access functions
//
// Copyright 2008, Johannes Berg <johannes@sipsolutions.net>
// Copyright 2010, Christian Lamparter <chunkeey@googlemail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, see
// http://www.gnu.org/licenses/.
//
// This file incorporates work covered by the following copyright and
// permission notice:
// Copyright (c) 2007-2008 Atheros Communications, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// basic HW access
extern "C" {
    pub fn carl9170_write_reg(ar: *mut ar9170, reg: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn carl9170_read_reg(ar: *mut ar9170, reg: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn carl9170_echo_test(ar: *mut ar9170, v: u32) -> c_int;
}
extern "C" {
    pub fn carl9170_reboot(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_mac_reset(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_powersave(ar: *mut ar9170, power_on: bool) -> c_int;
}
extern "C" {
    pub fn carl9170_collect_tally(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_bcn_ctrl(_arg: ar, _arg: vif_id, _arg: CARL9170_BCN_CTRL_DRAIN, _arg: 0, _arg: 0) -> return;
}
//
// Macros to facilitate writing multiple registers in a single
// write-combining USB command. Note that when the first group
// fails the whole thing will fail without any others attempted,
// but you won't know which write in the group failed.
//

