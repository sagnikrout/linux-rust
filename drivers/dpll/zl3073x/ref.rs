//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/ref.h
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
// struct zl3073x_ref - input reference state
// @phase_comp: phase compensation
// @esync_n_div: divisor for embedded sync or n-divided signal formats
// @freq_base: frequency base
// @freq_mult: frequnecy multiplier
// @freq_ratio_m: FEC mode multiplier
// @freq_ratio_n: FEC mode divisor
// @sync_ctrl: reference sync control
// @config: reference config
// @meas_freq: measured input frequency in Hz
// @mon_status: reference monitor status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_ref {
    pub phase_comp: u64,
    pub esync_n_div: u32,
    pub freq_base: u16,
    pub freq_mult: u16,
    pub freq_ratio_m: u16,
    pub freq_ratio_n: u16,
    pub sync_ctrl: u8,
    pub config: u8,
    pub meas_freq: u32,
    pub mon_status: u8,
}

extern "C" {
    pub fn zl3073x_ref_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_ref_state_update(zldev: *mut zl3073x_dev, index: u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_ref_freq_factorize(freq: u32, base: *mut u16, mult: *mut u16) -> c_int;
}
//
// zl3073x_ref_meas_freq_get - get measured input frequency
// @ref: pointer to ref state
//
// Return: measured input frequency in Hz
//
// zl3073x_ref_freq_get - get given input reference frequency
// @ref: pointer to ref state
//
// Return: frequency of the given input reference
//
// zl3073x_ref_freq_set - set given input reference frequency
// @ref: pointer to ref state
// @freq: frequency to be set
//
// Return: 0 on success, <0 when frequency cannot be factorized
//
// zl3073x_ref_sync_mode_get - get sync control mode
// @ref: pointer to ref state
//
// Return: sync control mode (ZL_REF_SYNC_CTRL_MODE_*)
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_REF_SYNC_CTRL_MODE, _arg: ref->sync_ctrl) -> return;
}
//
// zl3073x_ref_sync_mode_set - set sync control mode
// @ref: pointer to ref state
// @mode: sync control mode (ZL_REF_SYNC_CTRL_MODE_*)
//
// zl3073x_ref_sync_pair_get - get sync pair reference index
// @ref: pointer to ref state
//
// Return: paired reference index
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_REF_SYNC_CTRL_PAIR, _arg: ref->sync_ctrl) -> return;
}
//
// zl3073x_ref_sync_pair_set - set sync pair reference index
// @ref: pointer to ref state
// @pair: paired reference index
//
// zl3073x_ref_is_diff - check if the given input reference is differential
// @ref: pointer to ref state
//
// Return: true if reference is differential, false if reference is single-ended
//
// zl3073x_ref_is_enabled - check if the given input reference is enabled
// @ref: pointer to ref state
//
// Return: true if input refernce is enabled, false otherwise
//
// zl3073x_ref_is_status_ok - check the given input reference status
// @ref: pointer to ref state
//
// Return: true if the status is ok, false otherwise
//
