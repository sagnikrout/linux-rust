//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_netc.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// NXP NETC V4 Timer driver
// Copyright 2025 NXP
//

pub const NETC_TMR_PCI_VENDOR_NXP: c_uint = 0x1131;
pub const NETC_TMR_CTRL: c_uint = 0x0080;

pub const NETC_TMR_TEVENT: c_uint = 0x0084;

    TMR_TEVENT_ETSEN(i) | \
    TMR_TEVENT_ETS_OVEN(i))
pub const NETC_TMR_TEMASK: c_uint = 0x0088;
pub const NETC_TMR_STAT: c_uint = 0x0094;

pub const NETC_TMR_CNT_L: c_uint = 0x0098;
pub const NETC_TMR_CNT_H: c_uint = 0x009c;
pub const NETC_TMR_ADD: c_uint = 0x00a0;
pub const NETC_TMR_PRSC: c_uint = 0x00a8;
pub const NETC_TMR_ECTRL: c_uint = 0x00ac;
pub const NETC_TMR_OFF_L: c_uint = 0x00b0;
pub const NETC_TMR_OFF_H: c_uint = 0x00b4;
// i = 0, 1, i indicates the index of TMR_ALARM

// i = 0, 1, 2. i indicates the index of TMR_FIPER.

pub const NETC_TMR_FIPER_CTRL: c_uint = 0x00dc;

// i = 0, 1, i indicates the index of TMR_ETTS

pub const NETC_TMR_CUR_TIME_L: c_uint = 0x00f0;
pub const NETC_TMR_CUR_TIME_H: c_uint = 0x00f4;
pub const NETC_TMR_REGS_BAR: c_int = 0;
pub const NETC_GLOBAL_OFFSET: c_uint = 0x10000;
pub const NETC_GLOBAL_IPBRR0: c_uint = 0xbf8;

pub const NETC_REV_4_1: c_uint = 0x0401;
pub const NETC_TMR_FIPER_NUM: c_int = 3;

pub const NETC_TMR_DEFAULT_PRSC: c_int = 2;

pub const NETC_TMR_ALARM_NUM: c_int = 2;
pub const NETC_TMR_DEFAULT_ETTF_THR: c_int = 7;
// 1588 timer reference clock source select

    enum netc_pp_type {
    NETC_PP_PPS = 1,
    NETC_PP_PEROUT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_pp {
    pub type: enum netc_pp_type,
    pub enabled: bool,
    pub alarm_id: c_int,
    pub /: *mut *mut u32 period; / pulse period, ns,
    pub /: *mut *mut u64 stime; / start time, ns,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_timer {
    pub base: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub /: *mut *mut spinlock_t lock; / Prevent concurrent access to registers,
    pub clock: *mut ptp_clock,
    pub caps: ptp_clock_info,
    pub clk_select: u32,
    pub clk_freq: u32,
    pub oclk_prsc: u32,
// High 32-bit is integer part, low 32-bit is fractional part
    pub period: u64,
    pub irq: c_int,
    pub irq_name: [c_char; 24],
    pub revision: c_int,
    pub tmr_emask: u32,
    pub pps_channel: u8,
    pub fs_alarm_num: u8,
    pub fs_alarm_bitmap: u8,
    pub /: *mut *mut netc_pp pp[NETC_TMR_FIPER_NUM]; / periodic pulse,
}

    static const char *const timer_clk_src[] = {
    "ccm",
    "ext"
    };
#[no_mangle]
unsafe extern "C" fn netc_timer_cnt_write(priv: *mut netc_timer, ns: u64) {
    static void netc_timer_cnt_write(struct netc_timer *priv, u64 ns)
    {
    let mut tmr_cnt_h: u32 = upper_32_bits(ns);
    let mut tmr_cnt_l: u32 = lower_32_bits(ns);
// Writes to the TMR_CNT_L register copies the written value
// into the shadow TMR_CNT_L register. Writes to the TMR_CNT_H
// register copies the values written into the shadow TMR_CNT_H
// register. Contents of the shadow registers are copied into
// the TMR_CNT_L and TMR_CNT_H registers following a write into
// the TMR_CNT_H register. So the user must writes to TMR_CNT_L
// register first. Other H/L registers should have the same
// behavior.
//
    netc_timer_wr(priv, NETC_TMR_CNT_L, tmr_cnt_l);
    netc_timer_wr(priv, NETC_TMR_CNT_H, tmr_cnt_h);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_offset_read(priv: *mut netc_timer) -> u64 {
    static u64 netc_timer_offset_read(struct netc_timer *priv)
    {
    u32 tmr_off_l, tmr_off_h;
    u64 offset;
    tmr_off_l = netc_timer_rd(priv, NETC_TMR_OFF_L);
    tmr_off_h = netc_timer_rd(priv, NETC_TMR_OFF_H);
    offset = (((u64)tmr_off_h) << 32) | tmr_off_l;
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_offset_write(priv: *mut netc_timer, offset: u64) {
    static void netc_timer_offset_write(struct netc_timer *priv, u64 offset)
    {
    let mut tmr_off_h: u32 = upper_32_bits(offset);
    let mut tmr_off_l: u32 = lower_32_bits(offset);
    netc_timer_wr(priv, NETC_TMR_OFF_L, tmr_off_l);
    netc_timer_wr(priv, NETC_TMR_OFF_H, tmr_off_h);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_cur_time_read(priv: *mut netc_timer) -> u64 {
    static u64 netc_timer_cur_time_read(struct netc_timer *priv)
    {
    u32 time_h, time_l;
    u64 ns;
// The user should read NETC_TMR_CUR_TIME_L first to
// get correct current time.
//
    time_l = netc_timer_rd(priv, NETC_TMR_CUR_TIME_L);
    time_h = netc_timer_rd(priv, NETC_TMR_CUR_TIME_H);
    ns = (u64)time_h << 32 | time_l;
    return ns;
    }
    static void netc_timer_alarm_write(struct netc_timer *priv,
    u64 alarm, int index)
    {
    let mut alarm_h: u32 = upper_32_bits(alarm);
    let mut alarm_l: u32 = lower_32_bits(alarm);
    netc_timer_wr(priv, NETC_TMR_ALARM_L(index), alarm_l);
    netc_timer_wr(priv, NETC_TMR_ALARM_H(index), alarm_h);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_get_integral_period(priv: *mut netc_timer) -> u32 {
    static u32 netc_timer_get_integral_period(struct netc_timer *priv)
    {
    u32 tmr_ctrl, integral_period;
    tmr_ctrl = netc_timer_rd(priv, NETC_TMR_CTRL);
    integral_period = FIELD_GET(TMR_CTRL_TCLK_PERIOD, tmr_ctrl);
    return integral_period;
    }
    static u32 netc_timer_calculate_fiper_pw(struct netc_timer *priv,
    u32 fiper)
    {
    u64 divisor, pulse_width;
// Set the FIPER pulse width to half FIPER interval by default.
// pulse_width = (fiper / 2) / TMR_GCLK_period,
// TMR_GCLK_period = NSEC_PER_SEC / TMR_GCLK_freq,
// TMR_GCLK_freq = (clk_freq / oclk_prsc) Hz,
// so pulse_width = fiper * clk_freq / (2 * NSEC_PER_SEC * oclk_prsc).
//
    divisor = mul_u32_u32(2 * NSEC_PER_SEC, priv.oclk_prsc);
    pulse_width = div64_u64(mul_u32_u32(fiper, priv.clk_freq), divisor);
// The FIPER_PW field only has 5 bits, need to update oclk_prsc
    if (pulse_width > NETC_TMR_FIPER_MAX_PW)
    pulse_width = NETC_TMR_FIPER_MAX_PW;
    return pulse_width;
    }
    static void netc_timer_set_pps_alarm(struct netc_timer *priv, int channel,
    u32 integral_period)
    {
    struct netc_pp *pp = &priv.pp[channel];
    u64 alarm;
// Get the alarm value
    alarm = netc_timer_cur_time_read(priv) +  NSEC_PER_MSEC;
    alarm = roundup_u64(alarm, NSEC_PER_SEC);
    alarm = roundup_u64(alarm, integral_period);
    netc_timer_alarm_write(priv, alarm, pp.alarm_id);
    }
    static void netc_timer_set_perout_alarm(struct netc_timer *priv, int channel,
    u32 integral_period)
    {
    let mut cur_time: u64 = netc_timer_cur_time_read(priv);
    struct netc_pp *pp = &priv.pp[channel];
    u64 alarm, delta, min_time;
    let mut period: u32 = pp.period;
    let mut stime: u64 = pp.stime;
    min_time = cur_time + NSEC_PER_MSEC + period;
    if (stime < min_time) {
    delta = min_time - stime;
    stime += roundup_u64(delta, period);
    }
    alarm = roundup_u64(stime - period, integral_period);
    netc_timer_alarm_write(priv, alarm, pp.alarm_id);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_get_alarm_id(priv: *mut netc_timer) -> c_int {
    static int netc_timer_get_alarm_id(struct netc_timer *priv)
    {
    int i;
    for (i = 0; i < priv.fs_alarm_num; i++) {
    if (!(priv.fs_alarm_bitmap & BIT(i))) {
    priv.fs_alarm_bitmap |= BIT(i);
    break;
    }
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_get_gclk_period(priv: *mut netc_timer) -> u64 {
    static u64 netc_timer_get_gclk_period(struct netc_timer *priv)
    {
// TMR_GCLK_freq = (clk_freq / oclk_prsc) Hz.
// TMR_GCLK_period = NSEC_PER_SEC / TMR_GCLK_freq.
// TMR_GCLK_period = (NSEC_PER_SEC * oclk_prsc) / clk_freq
//
    return div_u64(mul_u32_u32(NSEC_PER_SEC, priv.oclk_prsc),
    priv.clk_freq);
    }
    static void netc_timer_enable_periodic_pulse(struct netc_timer *priv,
    u8 channel)
    {
    u32 fiper_pw, fiper, fiper_ctrl, integral_period;
    struct netc_pp *pp = &priv.pp[channel];
    let mut alarm_id: c_int = pp.alarm_id;
    integral_period = netc_timer_get_integral_period(priv);
// Set to desired FIPER interval in ns - TCLK_PERIOD
    fiper = pp.period - integral_period;
    fiper_pw = netc_timer_calculate_fiper_pw(priv, fiper);
    fiper_ctrl = netc_timer_rd(priv, NETC_TMR_FIPER_CTRL);
    fiper_ctrl &= ~(FIPER_CTRL_DIS(channel) | FIPER_CTRL_PW(channel) |
    FIPER_CTRL_FS_ALARM(channel));
    fiper_ctrl |= FIPER_CTRL_SET_PW(channel, fiper_pw);
    fiper_ctrl |= alarm_id ? FIPER_CTRL_FS_ALARM(channel) : 0;
    priv.tmr_emask |= TMR_TEVENT_ALMEN(alarm_id);
    if (pp.type == NETC_PP_PPS) {
    priv.tmr_emask |= TMR_TEVNET_PPEN(channel);
    netc_timer_set_pps_alarm(priv, channel, integral_period);
    } else {
    netc_timer_set_perout_alarm(priv, channel, integral_period);
    }
    netc_timer_wr(priv, NETC_TMR_TEMASK, priv.tmr_emask);
    netc_timer_wr(priv, NETC_TMR_FIPER(channel), fiper);
    netc_timer_wr(priv, NETC_TMR_FIPER_CTRL, fiper_ctrl);
    }
    static void netc_timer_disable_periodic_pulse(struct netc_timer *priv,
    u8 channel)
    {
    struct netc_pp *pp = &priv.pp[channel];
    let mut alarm_id: c_int = pp.alarm_id;
    u32 fiper_ctrl;
    if (!pp.enabled)
    return;
    priv.tmr_emask &= ~(TMR_TEVNET_PPEN(channel) |
    TMR_TEVENT_ALMEN(alarm_id));
    fiper_ctrl = netc_timer_rd(priv, NETC_TMR_FIPER_CTRL);
    fiper_ctrl |= FIPER_CTRL_DIS(channel);
    netc_timer_alarm_write(priv, NETC_TMR_DEFAULT_ALARM, alarm_id);
    netc_timer_wr(priv, NETC_TMR_TEMASK, priv.tmr_emask);
    netc_timer_wr(priv, NETC_TMR_FIPER(channel), NETC_TMR_DEFAULT_FIPER);
    netc_timer_wr(priv, NETC_TMR_FIPER_CTRL, fiper_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_select_pps_channel(priv: *mut netc_timer) -> u8 {
    static u8 netc_timer_select_pps_channel(struct netc_timer *priv)
    {
    int i;
    for (i = 0; i < NETC_TMR_FIPER_NUM; i++) {
    if (!priv.pp[i].enabled)
    return i;
    }
    return NETC_TMR_INVALID_CHANNEL;
    }
// Note that users should not use this API to output PPS signal on
// external pins, because PTP_CLK_REQ_PPS trigger internal PPS event
// for input into kernel PPS subsystem. See:
// https://lore.kernel.org/r/20201117213826.18235-1-a.fatoum@pengutronix.de
//
    static int netc_timer_enable_pps(struct netc_timer *priv,
    struct ptp_clock_request *rq, int on)
    {
    struct device *dev = &priv.pdev.dev;
    unsigned long flags;
    struct netc_pp *pp;
    let mut err: c_int = 0;
    spin_lock_irqsave(&priv.lock, flags);
    if (on) {
    int alarm_id;
    u8 channel;
    if (priv.pps_channel < NETC_TMR_FIPER_NUM) {
    channel = priv.pps_channel;
    } else {
    channel = netc_timer_select_pps_channel(priv);
    if (channel == NETC_TMR_INVALID_CHANNEL) {
    dev_err(dev, "No available FIPERs\n");
    err = -EBUSY;
    goto unlock_spinlock;
    }
    }
    pp = &priv.pp[channel];
    if (pp.enabled)
    goto unlock_spinlock;
    alarm_id = netc_timer_get_alarm_id(priv);
    if (alarm_id == priv.fs_alarm_num) {
    dev_err(dev, "No available ALARMs\n");
    err = -EBUSY;
    goto unlock_spinlock;
    }
    pp.enabled = true;
    pp.type = NETC_PP_PPS;
    pp.alarm_id = alarm_id;
    pp.period = NSEC_PER_SEC;
    priv.pps_channel = channel;
    netc_timer_enable_periodic_pulse(priv, channel);
    } else {
// pps_channel is invalid if PPS is not enabled, so no
// processing is needed.
//
    if (priv.pps_channel >= NETC_TMR_FIPER_NUM)
    goto unlock_spinlock;
    netc_timer_disable_periodic_pulse(priv, priv.pps_channel);
    pp = &priv.pp[priv.pps_channel];
    priv.fs_alarm_bitmap &= ~BIT(pp.alarm_id);
    memset(pp, 0, sizeof(*pp));
    priv.pps_channel = NETC_TMR_INVALID_CHANNEL;
    }
    unlock_spinlock:
    spin_unlock_irqrestore(&priv.lock, flags);
    return err;
    }
    static int net_timer_enable_perout(struct netc_timer *priv,
    struct ptp_clock_request *rq, int on)
    {
    struct device *dev = &priv.pdev.dev;
    let mut channel: u32 = rq.perout.index;
    unsigned long flags;
    struct netc_pp *pp;
    let mut err: c_int = 0;
    spin_lock_irqsave(&priv.lock, flags);
    pp = &priv.pp[channel];
    if (pp.type == NETC_PP_PPS) {
    dev_err(dev, "FIPER%u is being used for PPS\n", channel);
    err = -EBUSY;
    goto unlock_spinlock;
    }
    if (on) {
    u64 period_ns, gclk_period, min_period;
    struct timespec64 period, stime;
    u32 integral_period;
    int alarm_id;
    period.tv_sec = rq.perout.period.sec;
    period.tv_nsec = rq.perout.period.nsec;
    period_ns = timespec64_to_ns(&period);
    integral_period = netc_timer_get_integral_period(priv);
    gclk_period = netc_timer_get_gclk_period(priv);
    min_period = gclk_period * 4 + integral_period;
    if (period_ns > NETC_TMR_DEFAULT_FIPER ||
    period_ns < min_period) {
    dev_err(dev, "The period range is %llu ~ %lu\n",
    min_period, NETC_TMR_DEFAULT_FIPER);
    err = -EINVAL;
    goto unlock_spinlock;
    }
    if (pp.enabled) {
    alarm_id = pp.alarm_id;
    } else {
    alarm_id = netc_timer_get_alarm_id(priv);
    if (alarm_id == priv.fs_alarm_num) {
    dev_err(dev, "No available ALARMs\n");
    err = -EBUSY;
    goto unlock_spinlock;
    }
    pp.type = NETC_PP_PEROUT;
    pp.enabled = true;
    pp.alarm_id = alarm_id;
    }
    stime.tv_sec = rq.perout.start.sec;
    stime.tv_nsec = rq.perout.start.nsec;
    pp.stime = timespec64_to_ns(&stime);
    pp.period = period_ns;
    netc_timer_enable_periodic_pulse(priv, channel);
    } else {
    if (!pp.enabled)
    goto unlock_spinlock;
    netc_timer_disable_periodic_pulse(priv, channel);
    priv.fs_alarm_bitmap &= ~BIT(pp.alarm_id);
    memset(pp, 0, sizeof(*pp));
    }
    unlock_spinlock:
    spin_unlock_irqrestore(&priv.lock, flags);
    return err;
    }
    static void netc_timer_handle_etts_event(struct netc_timer *priv, int index,
    bool update_event)
    {
    struct ptp_clock_event event;
    let mut etts_l: u32 = 0, etts_h = 0;
    while (netc_timer_rd(priv, NETC_TMR_STAT) & TMR_STAT_ETS_VLD(index)) {
    etts_l = netc_timer_rd(priv, NETC_TMR_ETTS_L(index));
    etts_h = netc_timer_rd(priv, NETC_TMR_ETTS_H(index));
    }
// Invalid time stamp
    if (!etts_l && !etts_h)
    return;
    if (update_event) {
    event.type = PTP_CLOCK_EXTTS;
    event.index = index;
    event.timestamp = (u64)etts_h << 32;
    event.timestamp |= etts_l;
    ptp_clock_event(priv.clock, &event);
    }
    }
    static int netc_timer_enable_extts(struct netc_timer *priv,
    struct ptp_clock_request *rq, int on)
    {
    let mut index: c_int = rq.extts.index;
    unsigned long flags;
    u32 tmr_ctrl;
// Reject requests to enable time stamping on both edges
    if ((rq.extts.flags & PTP_EXTTS_EDGES) == PTP_EXTTS_EDGES)
    return -EOPNOTSUPP;
    spin_lock_irqsave(&priv.lock, flags);
    netc_timer_handle_etts_event(priv, rq.extts.index, false);
    if (on) {
    tmr_ctrl = netc_timer_rd(priv, NETC_TMR_CTRL);
    if (rq.extts.flags & PTP_FALLING_EDGE)
    tmr_ctrl |= TMR_ETEP(index);
    else
    tmr_ctrl &= ~TMR_ETEP(index);
    netc_timer_wr(priv, NETC_TMR_CTRL, tmr_ctrl);
    priv.tmr_emask |= TMR_TEVENT_ETS(index);
    } else {
    priv.tmr_emask &= ~TMR_TEVENT_ETS(index);
    }
    netc_timer_wr(priv, NETC_TMR_TEMASK, priv.tmr_emask);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_disable_fiper(priv: *mut netc_timer) {
    static void netc_timer_disable_fiper(struct netc_timer *priv)
    {
    let mut fiper_ctrl: u32 = netc_timer_rd(priv, NETC_TMR_FIPER_CTRL);
    int i;
    for (i = 0; i < NETC_TMR_FIPER_NUM; i++) {
    if (!priv.pp[i].enabled)
    continue;
    fiper_ctrl |= FIPER_CTRL_DIS(i);
    netc_timer_wr(priv, NETC_TMR_FIPER(i), NETC_TMR_DEFAULT_FIPER);
    }
    netc_timer_wr(priv, NETC_TMR_FIPER_CTRL, fiper_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_enable_fiper(priv: *mut netc_timer) {
    static void netc_timer_enable_fiper(struct netc_timer *priv)
    {
    let mut integral_period: u32 = netc_timer_get_integral_period(priv);
    let mut fiper_ctrl: u32 = netc_timer_rd(priv, NETC_TMR_FIPER_CTRL);
    int i;
    for (i = 0; i < NETC_TMR_FIPER_NUM; i++) {
    struct netc_pp *pp = &priv.pp[i];
    u32 fiper;
    if (!pp.enabled)
    continue;
    fiper_ctrl &= ~FIPER_CTRL_DIS(i);
    if (pp.type == NETC_PP_PPS)
    netc_timer_set_pps_alarm(priv, i, integral_period);
#[no_mangle]
pub unsafe extern "C" fn if(NETC_PP_PEROUT: pp->type ==) -> else {
    else if (pp.type == NETC_PP_PEROUT)
    netc_timer_set_perout_alarm(priv, i, integral_period);
    fiper = pp.period - integral_period;
    netc_timer_wr(priv, NETC_TMR_FIPER(i), fiper);
    }
    netc_timer_wr(priv, NETC_TMR_FIPER_CTRL, fiper_ctrl);
    }
    static int netc_timer_enable(struct ptp_clock_info *ptp,
    struct ptp_clock_request *rq, int on)
    {
    struct netc_timer *priv = ptp_to_netc_timer(ptp);
    switch (rq.type) {
    case PTP_CLK_REQ_PPS:
    return netc_timer_enable_pps(priv, rq, on);
    case PTP_CLK_REQ_PEROUT:
    return net_timer_enable_perout(priv, rq, on);
    case PTP_CLK_REQ_EXTTS:
    return netc_timer_enable_extts(priv, rq, on);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int netc_timer_perout_loopback(struct ptp_clock_info *ptp,
    unsigned int index, int on)
    {
    struct netc_timer *priv = ptp_to_netc_timer(ptp);
    unsigned long flags;
    u32 tmr_ctrl;
    spin_lock_irqsave(&priv.lock, flags);
    tmr_ctrl = netc_timer_rd(priv, NETC_TMR_CTRL);
    if (on)
    tmr_ctrl |= TMR_CTRL_PPL(index);
    else
    tmr_ctrl &= ~TMR_CTRL_PPL(index);
    netc_timer_wr(priv, NETC_TMR_CTRL, tmr_ctrl);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_adjust_period(priv: *mut netc_timer, period: u64) {
    static void netc_timer_adjust_period(struct netc_timer *priv, u64 period)
    {
    let mut fractional_period: u32 = lower_32_bits(period);
    let mut integral_period: u32 = upper_32_bits(period);
    u32 tmr_ctrl, old_tmr_ctrl;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    old_tmr_ctrl = netc_timer_rd(priv, NETC_TMR_CTRL);
    tmr_ctrl = u32_replace_bits(old_tmr_ctrl, integral_period,
    TMR_CTRL_TCLK_PERIOD);
    if (tmr_ctrl != old_tmr_ctrl) {
    netc_timer_disable_fiper(priv);
    netc_timer_wr(priv, NETC_TMR_CTRL, tmr_ctrl);
    netc_timer_enable_fiper(priv);
    }
    netc_timer_wr(priv, NETC_TMR_ADD, fractional_period);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int netc_timer_adjfine(struct ptp_clock_info *ptp, long scaled_ppm)
    {
    struct netc_timer *priv = ptp_to_netc_timer(ptp);
    u64 new_period;
    new_period = adjust_by_scaled_ppm(priv.period, scaled_ppm);
    netc_timer_adjust_period(priv, new_period);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int {
    static int netc_timer_adjtime(struct ptp_clock_info *ptp, s64 delta)
    {
    struct netc_timer *priv = ptp_to_netc_timer(ptp);
    unsigned long flags;
    s64 tmr_off;
    spin_lock_irqsave(&priv.lock, flags);
    netc_timer_disable_fiper(priv);
// Adjusting TMROFF instead of TMR_CNT is that the timer
// counter keeps increasing during reading and writing
// TMR_CNT, which will cause latency.
//
    tmr_off = netc_timer_offset_read(priv);
    tmr_off += delta;
    netc_timer_offset_write(priv, tmr_off);
    netc_timer_enable_fiper(priv);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    static int netc_timer_gettimex64(struct ptp_clock_info *ptp,
    struct timespec64 *ts,
    struct ptp_system_timestamp *sts)
    {
    struct netc_timer *priv = ptp_to_netc_timer(ptp);
    unsigned long flags;
    u64 ns;
    spin_lock_irqsave(&priv.lock, flags);
    ptp_read_system_prets(sts);
    ns = netc_timer_cur_time_read(priv);
    ptp_read_system_postts(sts);
    spin_unlock_irqrestore(&priv.lock, flags);
// ts = ns_to_timespec64(ns);
    return 0;
    }
    static int netc_timer_settime64(struct ptp_clock_info *ptp,
    const struct timespec64 *ts)
    {
    struct netc_timer *priv = ptp_to_netc_timer(ptp);
    let mut ns: u64 = timespec64_to_ns(ts);
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    netc_timer_disable_fiper(priv);
    netc_timer_offset_write(priv, 0);
    netc_timer_cnt_write(priv, ns);
    netc_timer_enable_fiper(priv);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    static const struct ptp_clock_info netc_timer_ptp_caps = {
    .owner		= THIS_MODULE,
    .name		= "NETC Timer PTP clock",
    .max_adj	= 500000000,
    .n_pins		= 0,
    .n_alarm	= 2,
    .pps		= 1,
    .n_per_out	= 3,
    .n_ext_ts	= 2,
    .n_per_lp	= 2,
    .supported_extts_flags = PTP_RISING_EDGE | PTP_FALLING_EDGE |
    PTP_STRICT_FLAGS,
    .adjfine	= netc_timer_adjfine,
    .adjtime	= netc_timer_adjtime,
    .gettimex64	= netc_timer_gettimex64,
    .settime64	= netc_timer_settime64,
    .enable		= netc_timer_enable,
    .perout_loopback = netc_timer_perout_loopback,
    };
#[no_mangle]
unsafe extern "C" fn netc_timer_init(priv: *mut netc_timer) {
    static void netc_timer_init(struct netc_timer *priv)
    {
    let mut fractional_period: u32 = lower_32_bits(priv.period);
    let mut integral_period: u32 = upper_32_bits(priv.period);
    u32 tmr_ctrl, fiper_ctrl;
    struct timespec64 now;
    u64 ns;
    int i;
// Software must enable timer first and the clock selected must be
// active, otherwise, the registers which are in the timer clock
// domain are not accessible.
//
    tmr_ctrl = FIELD_PREP(TMR_CTRL_CK_SEL, priv.clk_select) |
    TMR_CTRL_TE | TMR_CTRL_FS;
    netc_timer_wr(priv, NETC_TMR_CTRL, tmr_ctrl);
    netc_timer_wr(priv, NETC_TMR_PRSC, priv.oclk_prsc);
    netc_timer_wr(priv, NETC_TMR_TEMASK, 0);
// Disable FIPER by default
    fiper_ctrl = netc_timer_rd(priv, NETC_TMR_FIPER_CTRL);
    for (i = 0; i < NETC_TMR_FIPER_NUM; i++) {
    fiper_ctrl |= FIPER_CTRL_DIS(i);
    fiper_ctrl &= ~FIPER_CTRL_PG(i);
    }
    netc_timer_wr(priv, NETC_TMR_FIPER_CTRL, fiper_ctrl);
    netc_timer_wr(priv, NETC_TMR_ECTRL, NETC_TMR_DEFAULT_ETTF_THR);
    netc_timer_offset_write(priv, 0);
    ktime_get_real_ts64(&now);
    ns = timespec64_to_ns(&now);
    netc_timer_cnt_write(priv, ns);
// Allow atomic writes to TCLK_PERIOD and TMR_ADD, An update to
// TCLK_PERIOD does not take effect until TMR_ADD is written.
//
    tmr_ctrl |= FIELD_PREP(TMR_CTRL_TCLK_PERIOD, integral_period) |
    TMR_COMP_MODE;
    netc_timer_wr(priv, NETC_TMR_CTRL, tmr_ctrl);
    netc_timer_wr(priv, NETC_TMR_ADD, fractional_period);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_pci_probe(pdev: *mut pci_dev) -> c_int {
    static int netc_timer_pci_probe(struct pci_dev *pdev)
    {
    struct device *dev = &pdev.dev;
    struct netc_timer *priv;
    int err;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    pcie_flr(pdev);
    err = pci_enable_device_mem(pdev);
    if (err)
    return dev_err_probe(dev, err, "Failed to enable device\n");
    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    err = pci_request_mem_regions(pdev, KBUILD_MODNAME);
    if (err) {
    dev_err(dev, "pci_request_regions() failed, err:%pe\n",
    ERR_PTR(err));
    goto disable_dev;
    }
    pci_set_master(pdev);
    priv.pdev = pdev;
    priv.base = pci_ioremap_bar(pdev, NETC_TMR_REGS_BAR);
    if (!priv.base) {
    err = -ENOMEM;
    goto release_mem_regions;
    }
    pci_set_drvdata(pdev, priv);
    return 0;
    release_mem_regions:
    pci_release_mem_regions(pdev);
    disable_dev:
    pci_disable_device(pdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_pci_remove(pdev: *mut pci_dev) {
    static void netc_timer_pci_remove(struct pci_dev *pdev)
    {
    struct netc_timer *priv = pci_get_drvdata(pdev);
    iounmap(priv.base);
    pci_release_mem_regions(pdev);
    pci_disable_device(pdev);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_get_reference_clk_source(priv: *mut netc_timer) -> c_int {
    static int netc_timer_get_reference_clk_source(struct netc_timer *priv)
    {
    struct device *dev = &priv.pdev.dev;
    struct clk *clk;
    int i;
// Select NETC system clock as the reference clock by default
    priv.clk_select = NETC_TMR_SYSTEM_CLK;
    priv.clk_freq = NETC_TMR_SYSCLK_333M;
// Update the clock source of the reference clock if the clock
// is specified in DT node.
//
    for (i = 0; i < ARRAY_SIZE(timer_clk_src); i++) {
    clk = devm_clk_get_optional_enabled(dev, timer_clk_src[i]);
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk),
    "Failed to enable clock\n");
    if (clk) {
    priv.clk_freq = clk_get_rate(clk);
    priv.clk_select = i ? NETC_TMR_EXT_OSC :
    NETC_TMR_CCM_TIMER1;
    break;
    }
    }
// The period is a 64-bit number, the high 32-bit is the integer
// part of the period, the low 32-bit is the fractional part of
// the period. In order to get the desired 32-bit fixed-point
// format, multiply the numerator of the fraction by 2^32.
//
    priv.period = div_u64((u64)NSEC_PER_SEC << 32, priv.clk_freq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_parse_dt(priv: *mut netc_timer) -> c_int {
    static int netc_timer_parse_dt(struct netc_timer *priv)
    {
    return netc_timer_get_reference_clk_source(priv);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t netc_timer_isr(int irq, void *data)
    {
    struct netc_timer *priv = data;
    struct ptp_clock_event event;
    u32 tmr_event;
    spin_lock(&priv.lock);
    tmr_event = netc_timer_rd(priv, NETC_TMR_TEVENT);
    tmr_event &= priv.tmr_emask;
// Clear interrupts status
    netc_timer_wr(priv, NETC_TMR_TEVENT, tmr_event);
    if (!tmr_event) {
    spin_unlock(&priv.lock);
    return IRQ_NONE;
    }
    if (tmr_event & TMR_TEVENT_ALMEN(0))
    netc_timer_alarm_write(priv, NETC_TMR_DEFAULT_ALARM, 0);
    if (tmr_event & TMR_TEVENT_ALMEN(1))
    netc_timer_alarm_write(priv, NETC_TMR_DEFAULT_ALARM, 1);
    if (tmr_event & TMR_TEVENT_PPEN_ALL) {
    event.type = PTP_CLOCK_PPS;
    ptp_clock_event(priv.clock, &event);
    }
    if (tmr_event & TMR_TEVENT_ETS(0))
    netc_timer_handle_etts_event(priv, 0, true);
    if (tmr_event & TMR_TEVENT_ETS(1))
    netc_timer_handle_etts_event(priv, 1, true);
    spin_unlock(&priv.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_init_msix_irq(priv: *mut netc_timer) -> c_int {
    static int netc_timer_init_msix_irq(struct netc_timer *priv)
    {
    struct pci_dev *pdev = priv.pdev;
    int err, n;
    n = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_MSIX);
    if (n != 1) {
    err = (n < 0) ? n : -EPERM;
    dev_err(&pdev.dev, "pci_alloc_irq_vectors() failed\n");
    return err;
    }
    priv.irq = pci_irq_vector(pdev, 0);
    err = request_irq(priv.irq, netc_timer_isr, IRQF_NO_AUTOEN,
    priv.irq_name, priv);
    if (err) {
    dev_err(&pdev.dev, "request_irq() failed\n");
    pci_free_irq_vectors(pdev);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_free_msix_irq(priv: *mut netc_timer) {
    static void netc_timer_free_msix_irq(struct netc_timer *priv)
    {
    struct pci_dev *pdev = priv.pdev;
    free_irq(priv.irq, priv);
    pci_free_irq_vectors(pdev);
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_get_global_ip_rev(priv: *mut netc_timer) -> c_int {
    static int netc_timer_get_global_ip_rev(struct netc_timer *priv)
    {
    u32 val;
    val = netc_timer_rd(priv, NETC_GLOBAL_OFFSET + NETC_GLOBAL_IPBRR0);
    return val & IPBRR0_IP_REV;
    }
    static int netc_timer_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    struct netc_timer *priv;
    int err;
    err = netc_timer_pci_probe(pdev);
    if (err)
    return err;
    priv = pci_get_drvdata(pdev);
    priv.revision = netc_timer_get_global_ip_rev(priv);
    if (priv.revision == NETC_REV_4_1)
    priv.fs_alarm_num = 1;
    else
    priv.fs_alarm_num = NETC_TMR_ALARM_NUM;
    err = netc_timer_parse_dt(priv);
    if (err)
    goto timer_pci_remove;
    priv.caps = netc_timer_ptp_caps;
    priv.oclk_prsc = NETC_TMR_DEFAULT_PRSC;
    priv.pps_channel = NETC_TMR_INVALID_CHANNEL;
    spin_lock_init(&priv.lock);
    snprintf(priv.irq_name, sizeof(priv.irq_name), "ptp-netc %s",
    pci_name(pdev));
    err = netc_timer_init_msix_irq(priv);
    if (err)
    goto timer_pci_remove;
    netc_timer_init(priv);
    priv.clock = ptp_clock_register(&priv.caps, dev);
    if (IS_ERR(priv.clock)) {
    err = PTR_ERR(priv.clock);
    goto free_msix_irq;
    }
    enable_irq(priv.irq);
    return 0;
    free_msix_irq:
    netc_timer_free_msix_irq(priv);
    timer_pci_remove:
    netc_timer_pci_remove(pdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn netc_timer_remove(pdev: *mut pci_dev) {
    static void netc_timer_remove(struct pci_dev *pdev)
    {
    struct netc_timer *priv = pci_get_drvdata(pdev);
    disable_irq(priv.irq);
    ptp_clock_unregister(priv.clock);
    netc_timer_wr(priv, NETC_TMR_TEMASK, 0);
    netc_timer_wr(priv, NETC_TMR_CTRL, 0);
    netc_timer_free_msix_irq(priv);
    netc_timer_pci_remove(pdev);
    }
    static const struct pci_device_id netc_timer_id_table[] = {
    { PCI_DEVICE(NETC_TMR_PCI_VENDOR_NXP, 0xee02) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, netc_timer_id_table);
    static struct pci_driver netc_timer_driver = {
    .name = KBUILD_MODNAME,
    .id_table = netc_timer_id_table,
    .probe = netc_timer_probe,
    .remove = netc_timer_remove,
    };
    module_pci_driver(netc_timer_driver);
    MODULE_DESCRIPTION("NXP NETC Timer PTP Driver");
    MODULE_LICENSE("Dual BSD/GPL");
