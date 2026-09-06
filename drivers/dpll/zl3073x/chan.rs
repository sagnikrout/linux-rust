//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/chan.h
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
// struct zl3073x_chan - DPLL channel state
// @ctrl: DPLL control register value
// @mode_refsel: mode and reference selection register value
// @ref_prio: reference priority registers (4 bits per ref, P/N packed)
// @mon_status: monitor status register value
// @refsel_status: reference selection status register value
// @df_offset: frequency offset vs tracked reference in 2^-48 steps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_chan {
    pub ctrl: u8,
    pub mode_refsel: u8,
    pub 2]: u8 ref_prio[ZL3073X_NUM_REFS /,
    pub mon_status: u8,
    pub refsel_status: u8,
    pub df_offset: i64,
}

extern "C" {
    pub fn zl3073x_chan_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_chan_state_update(zldev: *mut zl3073x_dev, index: u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_chan_nco_mode_set(zldev: *mut zl3073x_dev, index: u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_chan_tod_ready_wait(zldev: *mut zl3073x_dev, ch: u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_chan_df_offset_set(zldev: *mut zl3073x_dev, ch: u8, offset: i64) -> c_int;
}
extern "C" {
    pub fn zl3073x_chan_tie_write(zldev: *mut zl3073x_dev, ch: u8, delta_ns: i64) -> c_int;
}
//
// zl3073x_chan_df_offset_get - get cached df_offset vs tracked reference
// @chan: pointer to channel state
//
// Return: frequency offset in 2^-48 steps
//
// zl3073x_chan_mode_get - get DPLL channel operating mode
// @chan: pointer to channel state
//
// Return: reference selection mode of the given DPLL channel
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_DPLL_MODE_REFSEL_MODE, _arg: chan->mode_refsel) -> return;
}
//
// zl3073x_chan_ref_get - get manually selected reference
// @chan: pointer to channel state
//
// Return: reference selected in forced reference lock mode
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_DPLL_MODE_REFSEL_REF, _arg: chan->mode_refsel) -> return;
}
//
// zl3073x_chan_mode_set - set DPLL channel operating mode
// @chan: pointer to channel state
// @mode: mode to set
//
// zl3073x_chan_ref_set - set manually selected reference
// @chan: pointer to channel state
// @ref: reference to set
//
// zl3073x_chan_ref_prio_get - get reference priority
// @chan: pointer to channel state
// @ref: input reference index
//
// Return: priority of the given reference <0, 15>
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_DPLL_REF_PRIO_REF_P, _arg: val) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: ZL_DPLL_REF_PRIO_REF_N, _arg: val) -> return;
}
//
// zl3073x_chan_ref_prio_set - set reference priority
// @chan: pointer to channel state
// @ref: input reference index
// @prio: priority to set
//
// zl3073x_chan_ref_is_selectable - check if reference is selectable
// @chan: pointer to channel state
// @ref: input reference index
//
// Return: true if the reference priority is not NONE, false otherwise
//
// zl3073x_chan_lock_state_get - get DPLL channel lock state
// @chan: pointer to channel state
//
// Return: lock state of the given DPLL channel
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_DPLL_MON_STATUS_STATE, _arg: chan->mon_status) -> return;
}
//
// zl3073x_chan_is_locked - check if channel is locked to a reference
// @chan: pointer to channel state
//
// Return: true if channel is locked, false otherwise
//
// zl3073x_chan_mode_is_auto - check if channel is in automatic mode
// @chan: pointer to channel state
//
// Return: true if channel is in automatic mode, false otherwise
//
// zl3073x_chan_mode_is_nco - check if channel is in NCO mode
// @chan: pointer to channel state
//
// Return: true if channel is in NCO mode, false otherwise
//
// zl3073x_chan_mode_is_reflock - check if channel is in reflock mode
// @chan: pointer to channel state
//
// Return: true if channel is in reflock mode, false otherwise
//
// zl3073x_chan_mode_supports_tie - check if channel mode supports TIE write
// @chan: pointer to channel state
//
// TIE write is supported in AUTO and REFLOCK modes regardless of lock state.
//
// Return: true if TIE write is supported, false otherwise
//
// zl3073x_chan_is_ho_ready - check if holdover is ready
// @chan: pointer to channel state
//
// Return: true if holdover is ready, false otherwise
//
// zl3073x_chan_refsel_state_get - get reference selection state
// @chan: pointer to channel state
//
// Return: reference selection state of the given DPLL channel
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_DPLL_REFSEL_STATUS_STATE, _arg: chan->refsel_status) -> return;
}
//
// zl3073x_chan_refsel_ref_get - get currently selected reference in auto mode
// @chan: pointer to channel state
//
// Return: reference selected by the DPLL in automatic mode
//
extern "C" {
    pub fn FIELD_GET(_arg: ZL_DPLL_REFSEL_STATUS_REFSEL, _arg: chan->refsel_status) -> return;
}
