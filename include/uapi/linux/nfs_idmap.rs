//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs_idmap.h
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
// include/uapi/linux/nfs_idmap.h
//
// UID and GID to name mapping for clients.
//
// Copyright (c) 2002 The Regents of the University of Michigan.
// All rights reserved.
//
// Marius Aamodt Eriksen <marius@umich.edu>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of the University nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// XXX from bits/utmp.h
pub const IDMAP_NAMESZ: c_int = 128;
pub const IDMAP_TYPE_USER: c_int = 0;
pub const IDMAP_TYPE_GROUP: c_int = 1;
pub const IDMAP_CONV_IDTONAME: c_int = 0;
pub const IDMAP_CONV_NAMETOID: c_int = 1;
pub const IDMAP_STATUS_INVALIDMSG: c_uint = 0x01;
pub const IDMAP_STATUS_AGAIN: c_uint = 0x02;
pub const IDMAP_STATUS_LOOKUPFAIL: c_uint = 0x04;
pub const IDMAP_STATUS_SUCCESS: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idmap_msg {
    pub im_type: __u8,
    pub im_conv: __u8,
    pub im_name: [c_char; IDMAP_NAMESZ],
    pub im_id: __u32,
    pub im_status: __u8,
}
