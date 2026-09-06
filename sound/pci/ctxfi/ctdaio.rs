//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctdaio.h
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
// @File	ctdaio.h
//
// @Brief
// This file contains the definition of Digital Audio Input Output
// resource management object.
//
// @Author	Liu Chun
// @Date 	May 23 2008
//

// Define the descriptor of a daio resource
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DAIOTYP {
    LINEO1,
    LINEO2,
    LINEO3,
    LINEO4,
    SPDIFOO,	/* S/PDIF Out (Flexijack/Optical) */
    LINEIM,
    SPDIFIO,	/* S/PDIF In (Flexijack/Optical) on the card */
    MIC,		/* Dedicated mic on Titanium HD */
    RCA,		/* Dedicated RCA on SE-300PCIE */
    SPDIFI_BAY,	/* S/PDIF In on internal drive bay */
    NUM_DAIOTYP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct daio {
    pub /: *mut *mut rsc rscl; / Basic resource info for left TX/RX,
    pub /: *mut *mut rsc rscr; / Basic resource info for right TX/RX,
    pub type: DAIOTYP,
    pub output: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dao {
    pub daio: daio,
    pub /: *const *const *const dao_rsc_ops ops; / DAO specific operations,
    pub imappers: *mut imapper,
    pub mgr: *mut daio_mgr,
    pub hw: *mut hw,
    pub ctrl_blk: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dai {
    pub daio: daio,
    pub /: *const *const *const dai_rsc_ops ops; / DAI specific operations,
    pub hw: *mut hw,
    pub ctrl_blk: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dao_desc {
    pub msr:4: c_uint,
    pub passthru:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dao_rsc_ops {
    pub spos): *mut *mut *mut int (set_spos)(struct dao dao, unsigned int,
    pub dao): *mut *mut int (commit_write)(struct dao,
    pub spos): *mut *mut *mut int (get_spos)(struct dao dao, unsigned int,
    pub desc): *const *const *const int (reinit)(struct dao dao, struct dao_desc,
    pub input): *mut *mut *mut int (set_left_input)(struct dao dao, struct rsc,
    pub input): *mut *mut *mut int (set_right_input)(struct dao dao, struct rsc,
    pub dao): *mut *mut int (clear_left_input)(struct dao,
    pub dao): *mut *mut int (clear_right_input)(struct dao,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dai_rsc_ops {
    pub src): *mut *mut *mut int (set_srt_srcl)(struct dai dai, struct rsc,
    pub src): *mut *mut *mut int (set_srt_srcr)(struct dai dai, struct rsc,
    pub msr): *mut *mut *mut int (set_srt_msr)(struct dai dai, unsigned int,
    pub enb): *mut *mut *mut int (set_enb_src)(struct dai dai, unsigned int,
    pub enb): *mut *mut *mut int (set_enb_srt)(struct dai dai, unsigned int,
    pub dai): *mut *mut int (commit_write)(struct dai,
}

// Define daio resource request description info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct daio_desc {
    pub type:4: c_uint,
    pub msr:4: c_uint,
    pub passthru:1: c_uint,
    pub output:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct daio_mgr {
    pub /: *mut *mut rsc_mgr mgr; / Basic resource manager info,
    pub /: *mut *mut *mut snd_card card; / pointer to this card,
    pub mgr_lock: spinlock_t,
    pub imap_lock: spinlock_t,
    pub imappers: list_head,
    pub init_imap: *mut imapper,
    pub init_imap_added: c_uint,
// request one daio resource
    pub rdaio): *const *const daio_desc desc, daio,
// return one daio resource
    pub daio): *mut *mut *mut int (put_daio)(struct daio_mgr mgr, struct daio,
    pub daio): *mut *mut *mut int (daio_enable)(struct daio_mgr mgr, struct daio,
    pub daio): *mut *mut *mut int (daio_disable)(struct daio_mgr mgr, struct daio,
    pub entry): *mut *mut *mut int (imap_add)(struct daio_mgr mgr, struct imapper,
    pub entry): *mut *mut *mut int (imap_delete)(struct daio_mgr mgr, struct imapper,
    pub mgr): *mut *mut int (commit_write)(struct daio_mgr,
}

// Constructor and destructor of daio resource manager
extern "C" {
    pub fn daio_mgr_create(hw: *mut hw, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn daio_mgr_destroy(ptr: *mut c_void) -> c_int;
}
