//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/common/mtk_vcodec_fw_priv.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_fw {
    pub type: mtk_vcodec_fw_type,
    pub ops: *const mtk_vcodec_fw_ops,
    pub pdev: *mut platform_device,
    pub scp: *mut mtk_scp,
    pub fw_use: mtk_vcodec_fw_use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_fw_ops {
    pub fw): *mut *mut int (load_firmware)(struct mtk_vcodec_fw,
    pub fw): *mut *mut unsigned int (get_vdec_capa)(struct mtk_vcodec_fw,
    pub fw): *mut *mut unsigned int (get_venc_capa)(struct mtk_vcodec_fw,
    pub dtcm_dmem_addr): *mut *mut *mut *mut void (map_dm_addr)(struct mtk_vcodec_fw fw, u32,
    pub priv): *mut c_void,
    pub wait): unsigned int len, unsigned int,
    pub fw): *mut *mut void (release)(struct mtk_vcodec_fw,
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

