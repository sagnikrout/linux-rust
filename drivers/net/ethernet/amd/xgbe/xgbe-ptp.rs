//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/amd/xgbe/xgbe-ptp.c
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

#[no_mangle]
unsafe extern "C" fn xgbe_adjfine(info: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int xgbe_adjfine(struct ptp_clock_info *info, long scaled_ppm)
    {
    struct xgbe_prv_data *pdata = container_of(info,
    struct xgbe_prv_data,
    ptp_clock_info);
    unsigned long flags;
    u64 addend;
    addend = adjust_by_scaled_ppm(pdata.tstamp_addend, scaled_ppm);
    spin_lock_irqsave(&pdata.tstamp_lock, flags);
    xgbe_update_tstamp_addend(pdata, addend);
    spin_unlock_irqrestore(&pdata.tstamp_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgbe_adjtime(info: *mut ptp_clock_info, delta: i64) -> c_int {
    static int xgbe_adjtime(struct ptp_clock_info *info, s64 delta)
    {
    struct xgbe_prv_data *pdata = container_of(info,
    struct xgbe_prv_data,
    ptp_clock_info);
    let mut neg_adjust: c_uint = 0;
    unsigned int sec, nsec;
    u32 quotient, reminder;
    unsigned long flags;
    if (delta < 0) {
    neg_adjust = 1;
    delta = -delta;
    }
    quotient = div_u64_rem(delta, 1000000000ULL, &reminder);
    sec = quotient;
    nsec = reminder;
// Negative adjustment for Hw timer register.
    if (neg_adjust) {
    sec = -sec;
    if (XGMAC_IOREAD_BITS(pdata, MAC_TSCR, TSCTRLSSR))
    nsec = (1000000000UL - nsec);
    else
    nsec = (0x80000000UL - nsec);
    }
    nsec = (neg_adjust << 31) | nsec;
    spin_lock_irqsave(&pdata.tstamp_lock, flags);
    xgbe_update_tstamp_time(pdata, sec, nsec);
    spin_unlock_irqrestore(&pdata.tstamp_lock, flags);
    return 0;
    }
    static int xgbe_gettimex(struct ptp_clock_info *info, struct timespec64 *ts,
    struct ptp_system_timestamp *sts)
    {
    struct xgbe_prv_data *pdata = container_of(info,
    struct xgbe_prv_data,
    ptp_clock_info);
    unsigned long flags;
    u64 nsec;
    spin_lock_irqsave(&pdata.tstamp_lock, flags);
    ptp_read_system_prets(sts);
    nsec = xgbe_get_tstamp_time(pdata);
    ptp_read_system_postts(sts);
    spin_unlock_irqrestore(&pdata.tstamp_lock, flags);
// ts = ns_to_timespec64(nsec);
    return 0;
    }
    static int xgbe_settime(struct ptp_clock_info *info,
    const struct timespec64 *ts)
    {
    struct xgbe_prv_data *pdata = container_of(info,
    struct xgbe_prv_data,
    ptp_clock_info);
    unsigned long flags;
    spin_lock_irqsave(&pdata.tstamp_lock, flags);
    xgbe_set_tstamp_time(pdata, ts.tv_sec, ts.tv_nsec);
    spin_unlock_irqrestore(&pdata.tstamp_lock, flags);
    return 0;
    }
    static int xgbe_enable(struct ptp_clock_info *info,
    struct ptp_clock_request *request, int on)
    {
    struct xgbe_prv_data *pdata = container_of(info, struct xgbe_prv_data,
    ptp_clock_info);
    struct xgbe_pps_config *pps_cfg;
    unsigned long flags;
    int ret;
    dev_dbg(pdata.dev, "rq.type %d on %d\n", request.type, on);
    if (request.type != PTP_CLK_REQ_PEROUT)
    return -EOPNOTSUPP;
    pps_cfg = &pdata.pps[request.perout.index];
    pps_cfg.start.tv_sec = request.perout.start.sec;
    pps_cfg.start.tv_nsec = request.perout.start.nsec;
    pps_cfg.period.tv_sec = request.perout.period.sec;
    pps_cfg.period.tv_nsec = request.perout.period.nsec;
    spin_lock_irqsave(&pdata.tstamp_lock, flags);
    ret = xgbe_pps_config(pdata, pps_cfg, request.perout.index, on);
    spin_unlock_irqrestore(&pdata.tstamp_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn xgbe_ptp_register(pdata: *mut xgbe_prv_data) {
    void xgbe_ptp_register(struct xgbe_prv_data *pdata)
    {
    struct ptp_clock_info *info = &pdata.ptp_clock_info;
    struct ptp_clock *clock;
    snprintf(info.name, sizeof(info.name), "%s",
    netdev_name(pdata.netdev));
    info.owner = THIS_MODULE;
    info.max_adj = pdata.ptpclk_rate;
    info.adjfine = xgbe_adjfine;
    info.adjtime = xgbe_adjtime;
    info.gettimex64 = xgbe_gettimex;
    info.settime64 = xgbe_settime;
    info.n_per_out = pdata.hw_feat.pps_out_num;
    info.n_ext_ts = pdata.hw_feat.aux_snap_num;
    info.enable = xgbe_enable;
    clock = ptp_clock_register(info, pdata.dev);
    if (IS_ERR(clock)) {
    dev_err(pdata.dev, "ptp_clock_register failed\n");
    return;
    }
    pdata.ptp_clock = clock;
// Disable all timestamping to start
    XGMAC_IOWRITE(pdata, MAC_TSCR, 0);
    pdata.tstamp_config.tx_type = HWTSTAMP_TX_OFF;
    pdata.tstamp_config.rx_filter = HWTSTAMP_FILTER_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn xgbe_ptp_unregister(pdata: *mut xgbe_prv_data) {
    void xgbe_ptp_unregister(struct xgbe_prv_data *pdata)
    {
    if (pdata.ptp_clock)
    ptp_clock_unregister(pdata.ptp_clock);
    }
