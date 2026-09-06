//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec_drv_base.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2016 MediaTek Inc.
// Author: PC Chen <pc.chen@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_common_if {
//
// (*init)() - initialize decode driver
// @ctx     : [in] mtk v4l2 context
// @h_vdec  : [out] driver handle
//
    pub ctx): *mut *mut int (init)(struct mtk_vcodec_dec_ctx,
//
// (*decode)() - trigger decode
// @h_vdec  : [in] driver handle
// @bs      : [in] input bitstream
// @fb      : [in] frame buffer to store decoded frame
// @res_chg : [out] resolution change happen
//
    pub res_chg): *mut *mut vdec_fb fb, bool,
//
// (*get_param)() - get driver's parameter
// @h_vdec : [in] driver handle
// @type   : [in] input parameter type
// @out    : [out] buffer to store query result
//
    pub out): *mut c_void,
//
// (*deinit)() - deinitialize driver.
// @h_vdec : [in] driver handle to be deinit
//
    pub h_vdec): *mut *mut void (deinit)(void,
}
