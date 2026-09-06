//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/google/gve/gve_ptp.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2025 Google LLC
//

// Interval to schedule a nic timestamp calibration, 250ms.
pub const GVE_NIC_TS_SYNC_INTERVAL_MS: c_int = 250;
// Read the nic timestamp from hardware via the admin queue.
#[no_mangle]
pub unsafe extern "C" fn gve_clock_nic_ts_read(priv: *mut gve_priv) -> c_int {
    int gve_clock_nic_ts_read(struct gve_priv *priv)
    {
    u64 nic_raw;
    int err;
    err = gve_adminq_report_nic_ts(priv, priv.nic_ts_report_bus);
    if (err)
    return err;
    nic_raw = be64_to_cpu(priv.nic_ts_report.nic_timestamp);
    WRITE_ONCE(priv.last_sync_nic_counter, nic_raw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gve_ptp_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int gve_ptp_adjfine(struct ptp_clock_info *ptp, long scaled_ppm)
    {
    return -EOPNOTSUPP;
    }
    static int gve_ptp_gettimex64(struct ptp_clock_info *info,
    struct timespec64 *ts,
    struct ptp_system_timestamp *sts)
    {
    return -EOPNOTSUPP;
    }
    static int gve_ptp_settime64(struct ptp_clock_info *info,
    const struct timespec64 *ts)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn gve_ptp_do_aux_work(info: *mut ptp_clock_info) -> c_long {
    static long gve_ptp_do_aux_work(struct ptp_clock_info *info)
    {
    const struct gve_ptp *ptp = container_of(info, struct gve_ptp, info);
    struct gve_priv *priv = ptp.priv;
    int err;
    if (gve_get_reset_in_progress(priv) || !gve_get_admin_queue_ok(priv))
    goto out;
    err = gve_clock_nic_ts_read(priv);
    if (err && net_ratelimit())
    dev_err(&priv.pdev.dev,
    "%s read err %d\n", __func__, err);
    out:
    return msecs_to_jiffies(GVE_NIC_TS_SYNC_INTERVAL_MS);
    }
    static const struct ptp_clock_info gve_ptp_caps = {
    .owner          = THIS_MODULE,
    .name		= "gve clock",
    .adjfine	= gve_ptp_adjfine,
    .gettimex64	= gve_ptp_gettimex64,
    .settime64	= gve_ptp_settime64,
    .do_aux_work	= gve_ptp_do_aux_work,
    };
#[no_mangle]
unsafe extern "C" fn gve_ptp_init(priv: *mut gve_priv) -> c_int {
    static int gve_ptp_init(struct gve_priv *priv)
    {
    struct gve_ptp *ptp;
    int err;
    priv.ptp = kzalloc_obj(*priv.ptp);
    if (!priv.ptp)
    return -ENOMEM;
    ptp = priv.ptp;
    ptp.info = gve_ptp_caps;
    ptp.clock = ptp_clock_register(&ptp.info, &priv.pdev.dev);
    if (IS_ERR(ptp.clock)) {
    dev_err(&priv.pdev.dev, "PTP clock registration failed\n");
    err  = PTR_ERR(ptp.clock);
    goto free_ptp;
    }
    ptp.priv = priv;
    return 0;
    free_ptp:
    kfree(ptp);
    priv.ptp = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn gve_ptp_release(priv: *mut gve_priv) {
    static void gve_ptp_release(struct gve_priv *priv)
    {
    struct gve_ptp *ptp = priv.ptp;
    if (!ptp)
    return;
    if (ptp.clock)
    ptp_clock_unregister(ptp.clock);
    kfree(ptp);
    priv.ptp = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn gve_init_clock(priv: *mut gve_priv) -> c_int {
    int gve_init_clock(struct gve_priv *priv)
    {
    int err;
    err = gve_ptp_init(priv);
    if (err)
    return err;
    priv.nic_ts_report =
    dma_alloc_coherent(&priv.pdev.dev,
    sizeof(struct gve_nic_ts_report),
    &priv.nic_ts_report_bus,
    GFP_KERNEL);
    if (!priv.nic_ts_report) {
    dev_err(&priv.pdev.dev, "%s dma alloc error\n", __func__);
    err = -ENOMEM;
    goto release_ptp;
    }
    err = gve_clock_nic_ts_read(priv);
    if (err) {
    dev_err(&priv.pdev.dev, "failed to read NIC clock %d\n", err);
    goto release_nic_ts_report;
    }
    ptp_schedule_worker(priv.ptp.clock,
    msecs_to_jiffies(GVE_NIC_TS_SYNC_INTERVAL_MS));
    return 0;
    release_nic_ts_report:
    dma_free_coherent(&priv.pdev.dev,
    sizeof(struct gve_nic_ts_report),
    priv.nic_ts_report, priv.nic_ts_report_bus);
    priv.nic_ts_report = core::ptr::null_mut();
    release_ptp:
    gve_ptp_release(priv);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn gve_teardown_clock(priv: *mut gve_priv) {
    void gve_teardown_clock(struct gve_priv *priv)
    {
    gve_ptp_release(priv);
    if (priv.nic_ts_report) {
    dma_free_coherent(&priv.pdev.dev,
    sizeof(struct gve_nic_ts_report),
    priv.nic_ts_report, priv.nic_ts_report_bus);
    priv.nic_ts_report = core::ptr::null_mut();
    }
    }
