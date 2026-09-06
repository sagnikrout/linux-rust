//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/vmw_pvrdma/pvrdma_ring.h
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
// Copyright (c) 2012-2016 VMware, Inc.  All rights reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of EITHER the GNU General Public License
// version 2 as published by the Free Software Foundation or the BSD
// 2-Clause License. This program is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY; WITHOUT EVEN THE IMPLIED
// WARRANTY OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License version 2 for more details at
// http://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html.
//
// You should have received a copy of the GNU General Public License
// along with this program available in the file COPYING in the main
// directory of this source tree.
//
// The BSD 2-Clause License
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
// COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
// INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ring {
    pub /: *mut *mut atomic_t prod_tail; / Producer tail.,
    pub /: *mut *mut atomic_t cons_head; / Consumer head.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ring_state {
    pub /: *mut *mut pvrdma_ring tx; / Tx ring.,
    pub /: *mut *mut pvrdma_ring rx; / Rx ring.,
}

// Generates fewer instructions than a less-than.
// out_tail = tail & (max_elems - 1);
// out_head = head & (max_elems - 1);
