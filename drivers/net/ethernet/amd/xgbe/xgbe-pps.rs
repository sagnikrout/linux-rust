//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/amd/xgbe/xgbe-pps.c
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-3-Clause)
//
// Copyright (c) 2014-2025, Advanced Micro Devices, Inc.
// Copyright (c) 2014, Synopsys, Inc.
// All rights reserved
//
// Author: Raju Rangoju <Raju.Rangoju@amd.com>
//

#[no_mangle]
unsafe extern "C" fn get_pps_mask(x: c_uint) -> u32 {
    static u32 get_pps_mask(unsigned int x)
    {
    return GENMASK(PPS_MAXIDX(x), PPS_MINIDX(x));
    }
#[no_mangle]
unsafe extern "C" fn get_pps_cmd(x: c_uint, val: u32) -> u32 {
    static u32 get_pps_cmd(unsigned int x, u32 val)
    {
    return (val & GENMASK(3, 0)) << PPS_MINIDX(x);
    }
#[no_mangle]
unsafe extern "C" fn get_target_mode_sel(x: c_uint, val: u32) -> u32 {
    static u32 get_target_mode_sel(unsigned int x, u32 val)
    {
    return (val & GENMASK(1, 0)) << (PPS_MAXIDX(x) - 2);
    }
    int xgbe_pps_config(struct xgbe_prv_data *pdata,
    struct xgbe_pps_config *cfg, int index, bool on)
    {
    let mut ppscr: c_uint = 0;
    unsigned int tnsec;
    u64 period;
// Check if target time register is busy
    tnsec = XGMAC_IOREAD(pdata, MAC_PPSx_TTNSR(index));
    if (XGMAC_GET_BITS(tnsec, MAC_PPSx_TTNSR, TRGTBUSY0))
    return -EBUSY;
    ppscr = XGMAC_IOREAD(pdata, MAC_PPSCR);
    ppscr &= ~get_pps_mask(index);
    if (!on) {
// Disable PPS output
    ppscr |= get_pps_cmd(index, XGBE_PPSCMD_STOP);
    ppscr |= PPSEN0;
    XGMAC_IOWRITE(pdata, MAC_PPSCR, ppscr);
    return 0;
    }
// Configure start time
    XGMAC_IOWRITE(pdata, MAC_PPSx_TTSR(index), cfg.start.tv_sec);
    XGMAC_IOWRITE(pdata, MAC_PPSx_TTNSR(index), cfg.start.tv_nsec);
    period = cfg.period.tv_sec * NSEC_PER_SEC + cfg.period.tv_nsec;
    period = div_u64(period, XGBE_V2_TSTAMP_SSINC);
    if (period < 4)
    return -EINVAL;
// Configure interval and pulse width (50% duty cycle)
    XGMAC_IOWRITE(pdata, MAC_PPSx_INTERVAL(index), period - 1);
    XGMAC_IOWRITE(pdata, MAC_PPSx_WIDTH(index), (period >> 1) - 1);
// Enable PPS with pulse train mode
    ppscr |= get_pps_cmd(index, XGBE_PPSCMD_START);
    ppscr |= get_target_mode_sel(index, XGBE_PPSTARGET_PULSE);
    ppscr |= PPSEN0;
    XGMAC_IOWRITE(pdata, MAC_PPSCR, ppscr);
    return 0;
    }
