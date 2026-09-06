//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctsrc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File	ctsrc.h
//
// @Brief
// This file contains the definition of the Sample Rate Convertor
// resource management object.
//
// @Author	Liu Chun
// @Date 	May 13 2008
//

pub const SRC_STATE_OFF: c_uint = 0x0;
pub const SRC_STATE_INIT: c_uint = 0x4;
pub const SRC_STATE_RUN: c_uint = 0x5;
pub const SRC_SF_U8: c_uint = 0x0;
pub const SRC_SF_S16: c_uint = 0x1;
pub const SRC_SF_S24: c_uint = 0x2;
pub const SRC_SF_S32: c_uint = 0x3;
pub const SRC_SF_F32: c_uint = 0x4;
// Define the descriptor of a src resource
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SRCMODE {
    MEMRD,		/* Read data from host memory */
    MEMWR,		/* Write data to host memory */
    ARCRW,		/* Read from and write to audio ring channel */
    NUM_SRCMODES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct src {
    pub /: *mut *mut rsc rsc; / Basic resource info,
    pub /: *mut *mut *mut src intlv; / Pointer to next interleaved SRC in a series,
    pub /: *const *const *const src_rsc_ops ops; / SRC specific operations,
// Number of contiguous srcs for interleaved usage
    pub multi: c_uchar,
    pub /: *mut *mut unsigned char mode; / Working mode of this SRC resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_rsc_ops {
    pub state): *mut *mut *mut int (set_state)(struct src src, unsigned int,
    pub bm): *mut *mut *mut int (set_bm)(struct src src, unsigned int,
    pub sf): *mut *mut *mut int (set_sf)(struct src src, unsigned int,
    pub pm): *mut *mut *mut int (set_pm)(struct src src, unsigned int,
    pub rom): *mut *mut *mut int (set_rom)(struct src src, unsigned int,
    pub vo): *mut *mut *mut int (set_vo)(struct src src, unsigned int,
    pub st): *mut *mut *mut int (set_st)(struct src src, unsigned int,
    pub bp): *mut *mut *mut int (set_bp)(struct src src, unsigned int,
    pub cisz): *mut *mut *mut int (set_cisz)(struct src src, unsigned int,
    pub ca): *mut *mut *mut int (set_ca)(struct src src, unsigned int,
    pub sa): *mut *mut *mut int (set_sa)(struct src src, unsigned int,
    pub la): *mut *mut *mut int (set_la)(struct src src, unsigned int,
    pub pitch): *mut *mut *mut int (set_pitch)(struct src src, unsigned int,
    pub src): *mut *mut int (set_clr_zbufs)(struct src,
    pub src): *mut *mut int (commit_write)(struct src,
    pub src): *mut *mut int (get_ca)(struct src,
    pub src): *mut *mut int (init)(struct src,
    pub src): *mut *mut *mut src (next_interleave)(src,
}

// Define src resource request description info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_desc {
// Number of contiguous master srcs for interleaved usage
    pub multi: c_uchar,
    pub msr: c_uchar,
    pub /: *mut *mut unsigned char mode; / Working mode of the requested srcs,
}

// Define src manager object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_mgr {
    pub /: *mut *mut rsc_mgr mgr; / Basic resource manager info,
    pub /: *mut *mut *mut snd_card card; / pointer to this card,
    pub mgr_lock: spinlock_t,
// request src resource
    pub rsrc): *const *const src_desc desc, src,
// return src resource
    pub src): *mut *mut *mut int (put_src)(struct src_mgr mgr, struct src,
    pub src): *mut *mut *mut int (src_enable_s)(struct src_mgr mgr, struct src,
    pub src): *mut *mut *mut int (src_enable)(struct src_mgr mgr, struct src,
    pub src): *mut *mut *mut int (src_disable)(struct src_mgr mgr, struct src,
    pub mgr): *mut *mut int (commit_write)(struct src_mgr,
}

// Define the descriptor of a SRC Input Mapper resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcimp {
    pub rsc: rsc,
    pub idx: [c_uchar; 8],
    pub /: *mut *mut unsigned int mapped; / A bit-map indicating which conj rsc is mapped,
    pub mgr: *mut srcimp_mgr,
    pub ops: *const srcimp_rsc_ops,
    pub imappers: [imapper; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcimp_rsc_ops {
    pub input): *mut *mut *mut *mut int (map)(struct srcimp srcimp, struct src user, struct rsc,
    pub srcimp): *mut *mut int (unmap)(struct srcimp,
}

// Define SRCIMP resource request description info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcimp_desc {
    pub msr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcimp_mgr {
    pub /: *mut *mut rsc_mgr mgr; / Basic resource manager info,
    pub /: *mut *mut *mut snd_card card; / pointer to this card,
    pub mgr_lock: spinlock_t,
    pub imap_lock: spinlock_t,
    pub imappers: list_head,
    pub init_imap: *mut imapper,
    pub init_imap_added: c_uint,
// request srcimp resource
    pub rsrcimp): *mut srcimp,
// return srcimp resource
    pub srcimp): *mut *mut *mut int (put_srcimp)(struct srcimp_mgr mgr, struct srcimp,
    pub entry): *mut *mut *mut int (imap_add)(struct srcimp_mgr mgr, struct imapper,
    pub entry): *mut *mut *mut int (imap_delete)(struct srcimp_mgr mgr, struct imapper,
}

// Constructor and destructor of SRC resource manager
extern "C" {
    pub fn src_mgr_create(hw: *mut hw, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn src_mgr_destroy(ptr: *mut c_void) -> c_int;
}
// Constructor and destructor of SRCIMP resource manager
extern "C" {
    pub fn srcimp_mgr_create(hw: *mut hw, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn srcimp_mgr_destroy(ptr: *mut c_void) -> c_int;
}
