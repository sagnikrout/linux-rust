//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi_parser.h
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
// Copyright (C) 2018 Linaro Ltd.

pub const WHICH_CAP_MIN: c_int = 0;
pub const WHICH_CAP_MAX: c_int = 1;
pub const WHICH_CAP_STEP: c_int = 2;
extern "C" {
    pub fn get_cap(_arg: inst, _arg: type, _arg: WHICH_CAP_MIN) -> return;
}
extern "C" {
    pub fn get_cap(_arg: inst, _arg: type, _arg: WHICH_CAP_MAX) -> return;
}
extern "C" {
    pub fn get_cap(_arg: inst, _arg: type, _arg: WHICH_CAP_STEP) -> return;
}
extern "C" {
    pub fn cap_min(_arg: inst, _arg: HFI_CAPABILITY_FRAME_WIDTH) -> return;
}
extern "C" {
    pub fn cap_max(_arg: inst, _arg: HFI_CAPABILITY_FRAME_WIDTH) -> return;
}
extern "C" {
    pub fn cap_step(_arg: inst, _arg: HFI_CAPABILITY_FRAME_WIDTH) -> return;
}
extern "C" {
    pub fn cap_min(_arg: inst, _arg: HFI_CAPABILITY_FRAME_HEIGHT) -> return;
}
extern "C" {
    pub fn cap_max(_arg: inst, _arg: HFI_CAPABILITY_FRAME_HEIGHT) -> return;
}
extern "C" {
    pub fn cap_step(_arg: inst, _arg: HFI_CAPABILITY_FRAME_HEIGHT) -> return;
}
extern "C" {
    pub fn cap_min(_arg: inst, _arg: HFI_CAPABILITY_FRAMERATE) -> return;
}
extern "C" {
    pub fn cap_max(_arg: inst, _arg: HFI_CAPABILITY_FRAMERATE) -> return;
}
extern "C" {
    pub fn cap_step(_arg: inst, _arg: HFI_CAPABILITY_FRAMERATE) -> return;
}
extern "C" {
    pub fn cap_max(_arg: inst, _arg: HFI_CAPABILITY_MAX_VIDEOCORES) -> return;
}
extern "C" {
    pub fn cap_max(_arg: inst, _arg: HFI_CAPABILITY_MBS_PER_FRAME) -> return;
}
