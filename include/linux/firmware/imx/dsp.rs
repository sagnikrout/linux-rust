//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/imx/dsp.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019 NXP
//
// Header file for the DSP IPC implementation
//

pub const DSP_MU_CHAN_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_dsp_chan {
    pub ipc: *mut imx_dsp_ipc,
    pub cl: mbox_client,
    pub ch: *mut mbox_chan,
    pub name: *mut c_char,
    pub idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_dsp_ops {
    pub ipc): *mut *mut void (handle_reply)(struct imx_dsp_ipc,
    pub ipc): *mut *mut void (handle_request)(struct imx_dsp_ipc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_dsp_ipc {
// Host <-> DSP communication uses 2 txdb and 2 rxdb channels
    pub chans: [imx_dsp_chan; DSP_MU_CHAN_NUM],
    pub dev: *mut device,
    pub ops: *mut imx_dsp_ops,
    pub private_data: *mut c_void,
}

extern "C" {
    pub fn imx_dsp_ring_doorbell(dsp: *mut imx_dsp_ipc, chan_idx: c_uint) -> c_int;
}
extern "C" {
    pub fn imx_dsp_free_channel(ipc: *mut imx_dsp_ipc, idx: c_int);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

