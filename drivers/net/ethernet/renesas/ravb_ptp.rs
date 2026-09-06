//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/renesas/ravb_ptp.c
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


// SPDX-License-Identifier: GPL-2.0+
// PTP 1588 clock using the Renesas Ethernet AVB
//
// Copyright (C) 2013-2015 Renesas Electronics Corporation
// Copyright (C) 2015 Renesas Solutions Corp.
// Copyright (C) 2015-2016 Cogent Embedded, Inc. <source@cogentembedded.com>
//

#[no_mangle]
unsafe extern "C" fn ravb_ptp_tcr_request(priv: *mut ravb_private, request: u32) -> c_int {
    static int ravb_ptp_tcr_request(struct ravb_private *priv, u32 request)
    {
    struct net_device *ndev = priv.ndev;
    int error;
    error = ravb_wait(ndev, GCCR, GCCR_TCR, GCCR_TCR_NOREQ);
    if (error)
    return error;
    ravb_modify(ndev, GCCR, request, request);
    return ravb_wait(ndev, GCCR, GCCR_TCR, GCCR_TCR_NOREQ);
    }
// Caller must hold the lock
#[no_mangle]
unsafe extern "C" fn ravb_ptp_time_read(priv: *mut ravb_private, ts: *mut timespec64) -> c_int {
    static int ravb_ptp_time_read(struct ravb_private *priv, struct timespec64 *ts)
    {
    struct net_device *ndev = priv.ndev;
    int error;
    error = ravb_ptp_tcr_request(priv, GCCR_TCR_CAPTURE);
    if (error)
    return error;
    ts.tv_nsec = ravb_read(ndev, GCT0);
    ts.tv_sec  = ravb_read(ndev, GCT1) |
    ((s64)ravb_read(ndev, GCT2) << 32);
    return 0;
    }
// Caller must hold the lock
    static int ravb_ptp_time_write(struct ravb_private *priv,
    const struct timespec64 *ts)
    {
    struct net_device *ndev = priv.ndev;
    int error;
    u32 gccr;
    error = ravb_ptp_tcr_request(priv, GCCR_TCR_RESET);
    if (error)
    return error;
    gccr = ravb_read(ndev, GCCR);
    if (gccr & GCCR_LTO)
    return -EBUSY;
    ravb_write(ndev, ts.tv_nsec, GTO0);
    ravb_write(ndev, ts.tv_sec,  GTO1);
    ravb_write(ndev, (ts.tv_sec >> 32) & 0xffff, GTO2);
    ravb_write(ndev, gccr | GCCR_LTO, GCCR);
    return 0;
    }
// Caller must hold the lock
#[no_mangle]
unsafe extern "C" fn ravb_ptp_update_compare(priv: *mut ravb_private, ns: u32) -> c_int {
    static int ravb_ptp_update_compare(struct ravb_private *priv, u32 ns)
    {
    struct net_device *ndev = priv.ndev;
// When the comparison value (GPTC.PTCV) is in range of
// [x-1 to x+1] (x is the configured increment value in
// GTI.TIV), it may happen that a comparison match is
// not detected when the timer wraps around.
//
    let mut gti_ns_plus_1: u32 = (priv.ptp.current_addend >> 20) + 1;
    u32 gccr;
    if (ns < gti_ns_plus_1)
    ns = gti_ns_plus_1;
#[no_mangle]
pub unsafe extern "C" fn if(gti_ns_plus_1: ns > 0 -) -> else {
    else if (ns > 0 - gti_ns_plus_1)
    ns = 0 - gti_ns_plus_1;
    gccr = ravb_read(ndev, GCCR);
    if (gccr & GCCR_LPTC)
    return -EBUSY;
    ravb_write(ndev, ns, GPTC);
    ravb_write(ndev, gccr | GCCR_LPTC, GCCR);
    return 0;
    }
// PTP clock operations
#[no_mangle]
unsafe extern "C" fn ravb_ptp_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int ravb_ptp_adjfine(struct ptp_clock_info *ptp, long scaled_ppm)
    {
    struct ravb_private *priv = container_of(ptp, struct ravb_private,
    ptp.info);
    struct net_device *ndev = priv.ndev;
    unsigned long flags;
    u32 addend;
    u32 gccr;
    addend = (u32)adjust_by_scaled_ppm(priv.ptp.default_addend,
    scaled_ppm);
    spin_lock_irqsave(&priv.lock, flags);
    priv.ptp.current_addend = addend;
    gccr = ravb_read(ndev, GCCR);
    if (gccr & GCCR_LTI) {
    spin_unlock_irqrestore(&priv.lock, flags);
    return -EBUSY;
    }
    ravb_write(ndev, addend & GTI_TIV, GTI);
    ravb_write(ndev, gccr | GCCR_LTI, GCCR);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ravb_ptp_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int {
    static int ravb_ptp_adjtime(struct ptp_clock_info *ptp, s64 delta)
    {
    struct ravb_private *priv = container_of(ptp, struct ravb_private,
    ptp.info);
    struct timespec64 ts;
    unsigned long flags;
    int error;
    spin_lock_irqsave(&priv.lock, flags);
    error = ravb_ptp_time_read(priv, &ts);
    if (!error) {
    let mut now: u64 = ktime_to_ns(timespec64_to_ktime(ts));
    ts = ns_to_timespec64(now + delta);
    error = ravb_ptp_time_write(priv, &ts);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ravb_ptp_gettime64(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int {
    static int ravb_ptp_gettime64(struct ptp_clock_info *ptp, struct timespec64 *ts)
    {
    struct ravb_private *priv = container_of(ptp, struct ravb_private,
    ptp.info);
    unsigned long flags;
    int error;
    spin_lock_irqsave(&priv.lock, flags);
    error = ravb_ptp_time_read(priv, ts);
    spin_unlock_irqrestore(&priv.lock, flags);
    return error;
    }
    static int ravb_ptp_settime64(struct ptp_clock_info *ptp,
    const struct timespec64 *ts)
    {
    struct ravb_private *priv = container_of(ptp, struct ravb_private,
    ptp.info);
    unsigned long flags;
    int error;
    spin_lock_irqsave(&priv.lock, flags);
    error = ravb_ptp_time_write(priv, ts);
    spin_unlock_irqrestore(&priv.lock, flags);
    return error;
    }
    static int ravb_ptp_extts(struct ptp_clock_info *ptp,
    struct ptp_extts_request *req, int on)
    {
    struct ravb_private *priv = container_of(ptp, struct ravb_private,
    ptp.info);
    const struct ravb_hw_info *info = priv.info;
    struct net_device *ndev = priv.ndev;
    unsigned long flags;
    if (req.index)
    return -EINVAL;
    if (priv.ptp.extts[req.index] == on)
    return 0;
    priv.ptp.extts[req.index] = on;
    spin_lock_irqsave(&priv.lock, flags);
    if (!info.irq_en_dis)
    ravb_modify(ndev, GIC, GIC_PTCE, on ? GIC_PTCE : 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: on) -> else {
    else if (on)
    ravb_write(ndev, GIE_PTCS, GIE);
    else
    ravb_write(ndev, GID_PTCD, GID);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    static int ravb_ptp_perout(struct ptp_clock_info *ptp,
    struct ptp_perout_request *req, int on)
    {
    struct ravb_private *priv = container_of(ptp, struct ravb_private,
    ptp.info);
    const struct ravb_hw_info *info = priv.info;
    struct net_device *ndev = priv.ndev;
    struct ravb_ptp_perout *perout;
    unsigned long flags;
    let mut error: c_int = 0;
    if (req.index)
    return -EINVAL;
    if (on) {
    u64 start_ns;
    u64 period_ns;
    start_ns = req.start.sec * NSEC_PER_SEC + req.start.nsec;
    period_ns = req.period.sec * NSEC_PER_SEC + req.period.nsec;
    if (start_ns > U32_MAX) {
    netdev_warn(ndev,
    "ptp: start value (nsec) is over limit. Maximum size of start is only 32 bits\n");
    return -ERANGE;
    }
    if (period_ns > U32_MAX) {
    netdev_warn(ndev,
    "ptp: period value (nsec) is over limit. Maximum size of period is only 32 bits\n");
    return -ERANGE;
    }
    spin_lock_irqsave(&priv.lock, flags);
    perout = &priv.ptp.perout[req.index];
    perout.target = (u32)start_ns;
    perout.period = (u32)period_ns;
    error = ravb_ptp_update_compare(priv, (u32)start_ns);
    if (!error) {
// Unmask interrupt
    if (!info.irq_en_dis)
    ravb_modify(ndev, GIC, GIC_PTME, GIC_PTME);
    else
    ravb_write(ndev, GIE_PTMS0, GIE);
    }
    } else	{
    spin_lock_irqsave(&priv.lock, flags);
    perout = &priv.ptp.perout[req.index];
    perout.period = 0;
// Mask interrupt
    if (!info.irq_en_dis)
    ravb_modify(ndev, GIC, GIC_PTME, 0);
    else
    ravb_write(ndev, GID_PTMD0, GID);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    return error;
    }
    static int ravb_ptp_enable(struct ptp_clock_info *ptp,
    struct ptp_clock_request *req, int on)
    {
    switch (req.type) {
    case PTP_CLK_REQ_EXTTS:
    return ravb_ptp_extts(ptp, &req.extts, on);
    case PTP_CLK_REQ_PEROUT:
    return ravb_ptp_perout(ptp, &req.perout, on);
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct ptp_clock_info ravb_ptp_info = {
    .owner		= THIS_MODULE,
    .name		= "ravb clock",
    .max_adj	= 50000000,
    .n_ext_ts	= N_EXT_TS,
    .n_per_out	= N_PER_OUT,
    .supported_extts_flags = PTP_RISING_EDGE | PTP_FALLING_EDGE,
    .adjfine	= ravb_ptp_adjfine,
    .adjtime	= ravb_ptp_adjtime,
    .gettime64	= ravb_ptp_gettime64,
    .settime64	= ravb_ptp_settime64,
    .enable		= ravb_ptp_enable,
    };
// Caller must hold the lock
#[no_mangle]
pub unsafe extern "C" fn ravb_ptp_interrupt(ndev: *mut net_device) {
    void ravb_ptp_interrupt(struct net_device *ndev)
    {
    struct ravb_private *priv = netdev_priv(ndev);
    struct ptp_clock *clock = READ_ONCE(priv.ptp.clock);
    let mut gis: u32 = ravb_read(ndev, GIS);
    gis &= ravb_read(ndev, GIC);
    if ((gis & GIS_PTCF) && clock) {
    struct ptp_clock_event event;
    event.type = PTP_CLOCK_EXTTS;
    event.index = 0;
    event.timestamp = ravb_read(ndev, GCPT);
    ptp_clock_event(clock, &event);
    }
    if (gis & GIS_PTMF) {
    struct ravb_ptp_perout *perout = priv.ptp.perout;
    if (perout.period) {
    perout.target += perout.period;
    ravb_ptp_update_compare(priv, perout.target);
    }
    }
    ravb_write(ndev, ~(gis | GIS_RESERVED), GIS);
    }
#[no_mangle]
pub unsafe extern "C" fn ravb_ptp_init(ndev: *mut net_device, pdev: *mut platform_device) {
    void ravb_ptp_init(struct net_device *ndev, struct platform_device *pdev)
    {
    struct ravb_private *priv = netdev_priv(ndev);
    struct ptp_clock *clock;
    unsigned long flags;
    priv.ptp.info = ravb_ptp_info;
    priv.ptp.default_addend = ravb_read(ndev, GTI);
    priv.ptp.current_addend = priv.ptp.default_addend;
    spin_lock_irqsave(&priv.lock, flags);
    ravb_wait(ndev, GCCR, GCCR_TCR, GCCR_TCR_NOREQ);
    ravb_modify(ndev, GCCR, GCCR_TCSS, GCCR_TCSS_ADJGPTP);
    spin_unlock_irqrestore(&priv.lock, flags);
    clock = ptp_clock_register(&priv.ptp.info, &pdev.dev);
    if (IS_ERR(clock)) {
    netdev_err(ndev, "failed to register PTP clock: %pe\n", clock);
    clock = core::ptr::null_mut();
    }
    WRITE_ONCE(priv.ptp.clock, clock);
    if (clock)
    WRITE_ONCE(priv.ptp.phc_index, ptp_clock_index(clock));
    }
#[no_mangle]
unsafe extern "C" fn ravb_ptp_disable(ndev: *mut net_device) {
    static void ravb_ptp_disable(struct net_device *ndev)
    {
    ravb_write(ndev, 0, GIC);
    ravb_write(ndev, 0, GIS);
    }
#[no_mangle]
unsafe extern "C" fn ravb_ptp_sync_irqs(ndev: *mut net_device) {
    static void ravb_ptp_sync_irqs(struct net_device *ndev)
    {
    struct ravb_private *priv = netdev_priv(ndev);
    synchronize_irq(ndev.irq);
    if (priv.info.err_mgmt_irqs) {
    synchronize_irq(priv.err_irq);
    synchronize_irq(priv.mgmt_irq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ravb_ptp_stop(ndev: *mut net_device) {
    void ravb_ptp_stop(struct net_device *ndev)
    {
    struct ravb_private *priv = netdev_priv(ndev);
    struct ptp_clock *clock;
    WRITE_ONCE(priv.ptp.phc_index, -1);
    clock = xchg(&priv.ptp.clock, core::ptr::null_mut());
    ravb_ptp_disable(ndev);
    ravb_ptp_sync_irqs(ndev);
    if (clock)
    ptp_clock_unregister(clock);
    }
