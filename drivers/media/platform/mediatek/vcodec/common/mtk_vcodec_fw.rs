//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/common/mtk_vcodec_fw.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vcodec_fw_type {
    VPU,
    SCP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vcodec_fw_use {
    DECODER,
    ENCODER,
}

extern "C" {
    pub fn mtk_vcodec_fw_release(fw: *mut mtk_vcodec_fw);
}
extern "C" {
    pub fn mtk_vcodec_fw_load_firmware(fw: *mut mtk_vcodec_fw) -> c_int;
}
extern "C" {
    pub fn mtk_vcodec_fw_get_vdec_capa(fw: *mut mtk_vcodec_fw) -> c_uint;
}
extern "C" {
    pub fn mtk_vcodec_fw_get_venc_capa(fw: *mut mtk_vcodec_fw) -> c_uint;
}
extern "C" {
    pub fn mtk_vcodec_fw_get_type(fw: *mut mtk_vcodec_fw) -> c_int;
}
