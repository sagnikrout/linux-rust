//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/t3cdev.h
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
// Copyright (C) 2006-2008 Chelsio Communications.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
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
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const T3CNAMSIZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t3ctype {
    T3A = 0,
    T3B,
    T3C,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t3cdev {
    pub /: *mut *mut char name[T3CNAMSIZ]; / T3C device name,
    pub type: t3ctype,
    pub /: *mut *mut list_head ofld_dev_list; / for list linking,
    pub /: *mut *mut *mut net_device lldev; / LL dev associated with T3C messages,
    pub /: *mut *mut *mut proc_dir_entry proc_dir; / root of proc dir for this T3C,
    pub skb): *mut *mut *mut int (send)(struct t3cdev dev, struct sk_buff,
    pub n): *mut *mut *mut *mut *mut int (recv)(struct t3cdev dev, struct sk_buff skb, int,
    pub data): *mut *mut *mut int (ctl)(struct t3cdev dev, unsigned int req, void,
    pub neigh): *mut *mut *mut void (neigh_update)(struct t3cdev dev, struct neighbour,
    pub /: *mut *mut *mut void priv; / driver private data,
    pub /: *mut *mut *mut void __rcu l2opt; / optional layer 2 data,
    pub /: *mut *mut *mut void l3opt; / optional layer 3 data,
    pub /: *mut *mut *mut void l4opt; / optional layer 4 data,
    pub /: *mut *mut *mut void ulp; / ulp stuff,
    pub /: *mut *mut *mut void ulp_iscsi; / ulp iscsi,
}
