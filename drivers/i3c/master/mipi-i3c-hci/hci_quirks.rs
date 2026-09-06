//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/mipi-i3c-hci/hci_quirks.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// I3C HCI Quirks
//
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
// Guruvendra Punugupati <Guruvendra.Punugupati@amd.com>
//

// Timing registers
pub const HCI_SCL_I3C_OD_TIMING: c_uint = 0x214;
pub const HCI_SCL_I3C_PP_TIMING: c_uint = 0x218;
pub const HCI_SDA_HOLD_SWITCH_DLY_TIMING: c_uint = 0x230;
// Timing values to configure 9MHz frequency
pub const AMD_SCL_I3C_OD_TIMING: c_uint = 0x00cf00cf;
pub const AMD_SCL_I3C_PP_TIMING: c_uint = 0x00160016;
pub const QUEUE_THLD_CTRL: c_uint = 0xD0;
#[no_mangle]
pub unsafe extern "C" fn amd_set_od_pp_timing(hci: *mut i3c_hci) {
    void amd_set_od_pp_timing(struct i3c_hci *hci)
    {
    u32 data;
    reg_write(HCI_SCL_I3C_OD_TIMING, AMD_SCL_I3C_OD_TIMING);
    reg_write(HCI_SCL_I3C_PP_TIMING, AMD_SCL_I3C_PP_TIMING);
    data = reg_read(HCI_SDA_HOLD_SWITCH_DLY_TIMING);
// Configure maximum TX hold time
    data |= W0_MASK(18, 16);
    reg_write(HCI_SDA_HOLD_SWITCH_DLY_TIMING, data);
    }
#[no_mangle]
pub unsafe extern "C" fn amd_set_resp_buf_thld(hci: *mut i3c_hci) {
    void amd_set_resp_buf_thld(struct i3c_hci *hci)
    {
    u32 data;
    data = reg_read(QUEUE_THLD_CTRL);
    data = data & ~W0_MASK(15, 8);
    reg_write(QUEUE_THLD_CTRL, data);
    }
