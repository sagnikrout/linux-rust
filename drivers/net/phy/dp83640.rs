//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/dp83640.c
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
//
// Driver for the National Semiconductor DP83640 PHYTER
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

pub const DP83640_PHY_ID: c_uint = 0x20005ce1;
pub const PAGESEL: c_uint = 0x13;
pub const MAX_RXTS: c_int = 64;
pub const N_EXT_TS: c_int = 6;
pub const N_PER_OUT: c_int = 7;
pub const PSF_PTPVER: c_int = 2;
pub const PSF_EVNT: c_uint = 0x4000;
pub const PSF_RX: c_uint = 0x2000;
pub const PSF_TX: c_uint = 0x1000;
pub const EXT_EVENT: c_int = 1;
pub const CAL_EVENT: c_int = 7;
pub const CAL_TRIGGER: c_int = 1;
pub const DP83640_N_PINS: c_int = 12;
pub const MII_DP83640_MICR: c_uint = 0x11;
pub const MII_DP83640_MISR: c_uint = 0x12;
pub const MII_DP83640_MICR_OE: c_uint = 0x1;
pub const MII_DP83640_MICR_IE: c_uint = 0x2;
pub const MII_DP83640_MISR_RHF_INT_EN: c_uint = 0x01;
pub const MII_DP83640_MISR_FHF_INT_EN: c_uint = 0x02;
pub const MII_DP83640_MISR_ANC_INT_EN: c_uint = 0x04;
pub const MII_DP83640_MISR_DUP_INT_EN: c_uint = 0x08;
pub const MII_DP83640_MISR_SPD_INT_EN: c_uint = 0x10;
pub const MII_DP83640_MISR_LINK_INT_EN: c_uint = 0x20;
pub const MII_DP83640_MISR_ED_INT_EN: c_uint = 0x40;
pub const MII_DP83640_MISR_LQ_INT_EN: c_uint = 0x80;
pub const MII_DP83640_MISR_ANC_INT: c_uint = 0x400;
pub const MII_DP83640_MISR_DUP_INT: c_uint = 0x800;
pub const MII_DP83640_MISR_SPD_INT: c_uint = 0x1000;
pub const MII_DP83640_MISR_LINK_INT: c_uint = 0x2000;

    MII_DP83640_MISR_DUP_INT |\
    MII_DP83640_MISR_SPD_INT |\
    MII_DP83640_MISR_LINK_INT)
// phyter seems to miss the mark by 16 ns
pub const ADJTIME_FIX: c_int = 16;

pub const ENDIAN_FLAG: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp83640_skb_info {
    pub ptp_type: c_int,
    pub tmo: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_rxts {
    pub /: *mut *mut u16 ns_lo; / ns[15:0],
    pub /: *mut *mut u16 ns_hi; / overflow[1:0], ns[29:16],
    pub /: *mut *mut u16 sec_lo; / sec[15:0],
    pub /: *mut *mut u16 sec_hi; / sec[31:16],
    pub /: *mut *mut u16 seqid; / sequenceId[15:0],
    pub /: *mut *mut u16 msgtype; / messageType[3:0], hash[11:0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_txts {
    pub /: *mut *mut u16 ns_lo; / ns[15:0],
    pub /: *mut *mut u16 ns_hi; / overflow[1:0], ns[29:16],
    pub /: *mut *mut u16 sec_lo; / sec[15:0],
    pub /: *mut *mut u16 sec_hi; / sec[31:16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxts {
    pub list: list_head,
    pub tmo: c_ulong,
    pub ns: u64,
    pub seqid: u16,
    pub msgtype: u8,
    pub hash: u16,
}

    struct dp83640_clock;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp83640_private {
    pub list: list_head,
    pub clock: *mut dp83640_clock,
    pub phydev: *mut phy_device,
    pub mii_ts: mii_timestamper,
    pub ts_work: delayed_work,
    pub hwts_tx_en: c_int,
    pub hwts_rx_en: c_int,
    pub layer: c_int,
    pub version: c_int,
// remember state of cfg0 during calibration
    pub cfg0: c_int,
// remember the last event time stamp
    pub edata: phy_txts,
// list of rx timestamps
    pub rxts: list_head,
    pub rxpool: list_head,
    pub rx_pool_data: [rxts; MAX_RXTS],
// protects above three fields from concurrent access
    pub rx_lock: spinlock_t,
// queues of incoming and outgoing packets
    pub rx_queue: sk_buff_head,
    pub tx_queue: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp83640_clock {
// protects extended registers from concurrent access
    pub extreg_lock: mutex,
// remembers which page was last selected
    pub page: c_int,
// our advertised capabilities
    pub caps: ptp_clock_info,
// protects the three fields below from concurrent access
    pub clock_lock: mutex,
// the one phyter from which we shall read
    pub chosen: *mut dp83640_private,
// list of the other attached phyters, not chosen
    pub phylist: list_head,
// reference to our PTP hardware clock
    pub ptp_clock: *mut ptp_clock,
// protected by the PTP core pin configuration lock
    pub pin_config: [ptp_pin_desc; DP83640_N_PINS],
}

// globals
    enum {
    CALIBRATE_GPIO,
    PEROUT_GPIO,
    EXTTS0_GPIO,
    EXTTS1_GPIO,
    EXTTS2_GPIO,
    EXTTS3_GPIO,
    EXTTS4_GPIO,
    EXTTS5_GPIO,
    GPIO_TABLE_SIZE
    };
    let mut chosen_phy: static int = -1;
    static ushort gpio_tab[GPIO_TABLE_SIZE] = {
    1, 2, 3, 4, 8, 9, 10, 11
    };
    module_param(chosen_phy, int, 0444);
    module_param_array(gpio_tab, ushort, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(chosen_phy,
    "The address of the PHY to use for the ancillary clock features");
    MODULE_PARM_DESC(gpio_tab,
    "Which GPIO line to use for which purpose: cal,perout,extts1,...,extts6");
#[no_mangle]
unsafe extern "C" fn dp83640_gpio_defaults(pd: *mut ptp_pin_desc) {
    static void dp83640_gpio_defaults(struct ptp_pin_desc *pd)
    {
    int i, index;
    for (i = 0; i < DP83640_N_PINS; i++) {
    snprintf(pd[i].name, sizeof(pd[i].name), "GPIO%d", 1 + i);
    pd[i].index = i;
    }
    for (i = 0; i < GPIO_TABLE_SIZE; i++) {
    if (gpio_tab[i] < 1 || gpio_tab[i] > DP83640_N_PINS) {
    pr_err("gpio_tab[%d]=%hu out of range", i, gpio_tab[i]);
    return;
    }
    }
    index = gpio_tab[CALIBRATE_GPIO] - 1;
    pd[index].func = PTP_PF_PHYSYNC;
    pd[index].chan = 0;
    index = gpio_tab[PEROUT_GPIO] - 1;
    pd[index].func = PTP_PF_PEROUT;
    pd[index].chan = 0;
    for (i = EXTTS0_GPIO; i < GPIO_TABLE_SIZE; i++) {
    index = gpio_tab[i] - 1;
    pd[index].func = PTP_PF_EXTTS;
    pd[index].chan = i - EXTTS0_GPIO;
    }
    }
    static void rx_timestamp_work(struct work_struct *work);
// extended register access functions
pub const BROADCAST_ADDR: c_int = 31;
    static inline int broadcast_write(struct phy_device *phydev, u32 regnum,
    u16 val)
    {
    return mdiobus_write(phydev.mdio.bus, BROADCAST_ADDR, regnum, val);
    }
// Caller must hold extreg_lock.
#[no_mangle]
unsafe extern "C" fn ext_read(phydev: *mut phy_device, page: c_int, regnum: u32) -> c_int {
    static int ext_read(struct phy_device *phydev, int page, u32 regnum)
    {
    struct dp83640_private *dp83640 = phydev.priv;
    int val;
    if (dp83640.clock.page != page) {
    broadcast_write(phydev, PAGESEL, page);
    dp83640.clock.page = page;
    }
    val = phy_read(phydev, regnum);
    return val;
    }
// Caller must hold extreg_lock.
    static void ext_write(int broadcast, struct phy_device *phydev,
    int page, u32 regnum, u16 val)
    {
    struct dp83640_private *dp83640 = phydev.priv;
    if (dp83640.clock.page != page) {
    broadcast_write(phydev, PAGESEL, page);
    dp83640.clock.page = page;
    }
    if (broadcast)
    broadcast_write(phydev, regnum, val);
    else
    phy_write(phydev, regnum, val);
    }
// Caller must hold extreg_lock.
    static int tdr_write(int bc, struct phy_device *dev,
    const struct timespec64 *ts, u16 cmd)
    {
    ext_write(bc, dev, PAGE4, PTP_TDR, ts.tv_nsec & 0xffff);/* ns[15:0]  */
    ext_write(bc, dev, PAGE4, PTP_TDR, ts.tv_nsec >> 16);   /* ns[31:16] */
    ext_write(bc, dev, PAGE4, PTP_TDR, ts.tv_sec & 0xffff); /* sec[15:0] */
    ext_write(bc, dev, PAGE4, PTP_TDR, ts.tv_sec >> 16);    /* sec[31:16]*/
    ext_write(bc, dev, PAGE4, PTP_CTL, cmd);
    return 0;
    }
// convert phy timestamps into driver timestamps
#[no_mangle]
unsafe extern "C" fn phy2rxts(p: *mut phy_rxts, rxts: *mut rxts) {
    static void phy2rxts(struct phy_rxts *p, struct rxts *rxts)
    {
    u32 sec;
    sec = p.sec_lo;
    sec |= p.sec_hi << 16;
    rxts.ns = p.ns_lo;
    rxts.ns |= (p.ns_hi & 0x3fff) << 16;
    rxts.ns += ((u64)sec) * 1000000000ULL;
    rxts.seqid = p.seqid;
    rxts.msgtype = (p.msgtype >> 12) & 0xf;
    rxts.hash = p.msgtype & 0x0fff;
    rxts.tmo = jiffies + SKB_TIMESTAMP_TIMEOUT;
    }
#[no_mangle]
unsafe extern "C" fn phy2txts(p: *mut phy_txts) -> u64 {
    static u64 phy2txts(struct phy_txts *p)
    {
    u64 ns;
    u32 sec;
    sec = p.sec_lo;
    sec |= p.sec_hi << 16;
    ns = p.ns_lo;
    ns |= (p.ns_hi & 0x3fff) << 16;
    ns += ((u64)sec) * 1000000000ULL;
    return ns;
    }
    static int periodic_output(struct dp83640_clock *clock,
    struct ptp_clock_request *clkreq, bool on,
    int trigger)
    {
    struct dp83640_private *dp83640 = clock.chosen;
    struct phy_device *phydev = dp83640.phydev;
    u32 sec, nsec, pwidth;
    u16 gpio, ptp_trig, val;
    if (on) {
    gpio = 1 + ptp_find_pin(clock.ptp_clock, PTP_PF_PEROUT,
    trigger);
    if (gpio < 1)
    return -EINVAL;
    } else {
    gpio = 0;
    }
    ptp_trig = TRIG_WR |
    (trigger & TRIG_CSEL_MASK) << TRIG_CSEL_SHIFT |
    (gpio & TRIG_GPIO_MASK) << TRIG_GPIO_SHIFT |
    TRIG_PER |
    TRIG_PULSE;
    val = (trigger & TRIG_SEL_MASK) << TRIG_SEL_SHIFT;
    if (!on) {
    val |= TRIG_DIS;
    mutex_lock(&clock.extreg_lock);
    ext_write(0, phydev, PAGE5, PTP_TRIG, ptp_trig);
    ext_write(0, phydev, PAGE4, PTP_CTL, val);
    mutex_unlock(&clock.extreg_lock);
    return 0;
    }
    sec = clkreq.perout.start.sec;
    nsec = clkreq.perout.start.nsec;
    pwidth = clkreq.perout.period.sec * 1000000000UL;
    pwidth += clkreq.perout.period.nsec;
    pwidth /= 2;
    mutex_lock(&clock.extreg_lock);
    ext_write(0, phydev, PAGE5, PTP_TRIG, ptp_trig);
// load trigger
    val |= TRIG_LOAD;
    ext_write(0, phydev, PAGE4, PTP_CTL, val);
    ext_write(0, phydev, PAGE4, PTP_TDR, nsec & 0xffff);   /* ns[15:0] */
    ext_write(0, phydev, PAGE4, PTP_TDR, nsec >> 16);      /* ns[31:16] */
    ext_write(0, phydev, PAGE4, PTP_TDR, sec & 0xffff);    /* sec[15:0] */
    ext_write(0, phydev, PAGE4, PTP_TDR, sec >> 16);       /* sec[31:16] */
    ext_write(0, phydev, PAGE4, PTP_TDR, pwidth & 0xffff); /* ns[15:0] */
    ext_write(0, phydev, PAGE4, PTP_TDR, pwidth >> 16);    /* ns[31:16] */
// Triggers 0 and 1 has programmable pulsewidth2
    if (trigger < 2) {
    ext_write(0, phydev, PAGE4, PTP_TDR, pwidth & 0xffff);
    ext_write(0, phydev, PAGE4, PTP_TDR, pwidth >> 16);
    }
// enable trigger
    val &= ~TRIG_LOAD;
    val |= TRIG_EN;
    ext_write(0, phydev, PAGE4, PTP_CTL, val);
    mutex_unlock(&clock.extreg_lock);
    return 0;
    }
// ptp clock methods
#[no_mangle]
unsafe extern "C" fn ptp_dp83640_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int ptp_dp83640_adjfine(struct ptp_clock_info *ptp, long scaled_ppm)
    {
    struct dp83640_clock *clock =
    container_of(ptp, struct dp83640_clock, caps);
    struct phy_device *phydev = clock.chosen.phydev;
    u64 rate;
    let mut neg_adj: c_int = 0;
    u16 hi, lo;
    if (scaled_ppm < 0) {
    neg_adj = 1;
    scaled_ppm = -scaled_ppm;
    }
    rate = scaled_ppm;
    rate <<= 13;
    rate = div_u64(rate, 15625);
    hi = (rate >> 16) & PTP_RATE_HI_MASK;
    if (neg_adj)
    hi |= PTP_RATE_DIR;
    lo = rate & 0xffff;
    mutex_lock(&clock.extreg_lock);
    ext_write(1, phydev, PAGE4, PTP_RATEH, hi);
    ext_write(1, phydev, PAGE4, PTP_RATEL, lo);
    mutex_unlock(&clock.extreg_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_dp83640_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int {
    static int ptp_dp83640_adjtime(struct ptp_clock_info *ptp, s64 delta)
    {
    struct dp83640_clock *clock =
    container_of(ptp, struct dp83640_clock, caps);
    struct phy_device *phydev = clock.chosen.phydev;
    struct timespec64 ts;
    int err;
    delta += ADJTIME_FIX;
    ts = ns_to_timespec64(delta);
    mutex_lock(&clock.extreg_lock);
    err = tdr_write(1, phydev, &ts, PTP_STEP_CLK);
    mutex_unlock(&clock.extreg_lock);
    return err;
    }
    static int ptp_dp83640_gettime(struct ptp_clock_info *ptp,
    struct timespec64 *ts)
    {
    struct dp83640_clock *clock =
    container_of(ptp, struct dp83640_clock, caps);
    struct phy_device *phydev = clock.chosen.phydev;
    unsigned int val[4];
    mutex_lock(&clock.extreg_lock);
    ext_write(0, phydev, PAGE4, PTP_CTL, PTP_RD_CLK);
    val[0] = ext_read(phydev, PAGE4, PTP_TDR); /* ns[15:0] */
    val[1] = ext_read(phydev, PAGE4, PTP_TDR); /* ns[31:16] */
    val[2] = ext_read(phydev, PAGE4, PTP_TDR); /* sec[15:0] */
    val[3] = ext_read(phydev, PAGE4, PTP_TDR); /* sec[31:16] */
    mutex_unlock(&clock.extreg_lock);
    ts.tv_nsec = val[0] | (val[1] << 16);
    ts.tv_sec  = val[2] | (val[3] << 16);
    return 0;
    }
    static int ptp_dp83640_settime(struct ptp_clock_info *ptp,
    const struct timespec64 *ts)
    {
    struct dp83640_clock *clock =
    container_of(ptp, struct dp83640_clock, caps);
    struct phy_device *phydev = clock.chosen.phydev;
    int err;
    mutex_lock(&clock.extreg_lock);
    err = tdr_write(1, phydev, ts, PTP_LOAD_CLK);
    mutex_unlock(&clock.extreg_lock);
    return err;
    }
    static int ptp_dp83640_enable(struct ptp_clock_info *ptp,
    struct ptp_clock_request *rq, int on)
    {
    struct dp83640_clock *clock =
    container_of(ptp, struct dp83640_clock, caps);
    struct phy_device *phydev = clock.chosen.phydev;
    unsigned int index;
    u16 evnt, event_num, gpio_num;
    switch (rq.type) {
    case PTP_CLK_REQ_EXTTS:
// Reject requests to enable time stamping on both edges.
    if ((rq.extts.flags & PTP_STRICT_FLAGS) &&
    (rq.extts.flags & PTP_ENABLE_FEATURE) &&
    (rq.extts.flags & PTP_EXTTS_EDGES) == PTP_EXTTS_EDGES)
    return -EOPNOTSUPP;
    index = rq.extts.index;
    if (index >= N_EXT_TS)
    return -EINVAL;
    event_num = EXT_EVENT + index;
    evnt = EVNT_WR | (event_num & EVNT_SEL_MASK) << EVNT_SEL_SHIFT;
    if (on) {
    gpio_num = 1 + ptp_find_pin(clock.ptp_clock,
    PTP_PF_EXTTS, index);
    if (gpio_num < 1)
    return -EINVAL;
    evnt |= (gpio_num & EVNT_GPIO_MASK) << EVNT_GPIO_SHIFT;
    if (rq.extts.flags & PTP_FALLING_EDGE)
    evnt |= EVNT_FALL;
    else
    evnt |= EVNT_RISE;
    }
    mutex_lock(&clock.extreg_lock);
    ext_write(0, phydev, PAGE5, PTP_EVNT, evnt);
    mutex_unlock(&clock.extreg_lock);
    return 0;
    case PTP_CLK_REQ_PEROUT:
    if (rq.perout.index >= N_PER_OUT)
    return -EINVAL;
    return periodic_output(clock, rq, on, rq.perout.index);
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static int ptp_dp83640_verify(struct ptp_clock_info *ptp, unsigned int pin,
    enum ptp_pin_function func, unsigned int chan)
    {
    struct dp83640_clock *clock =
    container_of(ptp, struct dp83640_clock, caps);
    if (clock.caps.pin_config[pin].func == PTP_PF_PHYSYNC &&
    !list_empty(&clock.phylist))
    return 1;
    if (func == PTP_PF_PHYSYNC)
    return 1;
    return 0;
    }
    static u8 status_frame_dst[6] = { 0x01, 0x1B, 0x19, 0x00, 0x00, 0x00 };
    static u8 status_frame_src[6] = { 0x08, 0x00, 0x17, 0x0B, 0x6B, 0x0F };
#[no_mangle]
unsafe extern "C" fn enable_status_frames(phydev: *mut phy_device, on: bool) {
    static void enable_status_frames(struct phy_device *phydev, bool on)
    {
    struct dp83640_private *dp83640 = phydev.priv;
    struct dp83640_clock *clock = dp83640.clock;
    let mut cfg0: u16 = 0, ver;
    if (on)
    cfg0 = PSF_EVNT_EN | PSF_RXTS_EN | PSF_TXTS_EN | ENDIAN_FLAG;
    ver = (PSF_PTPVER & VERSIONPTP_MASK) << VERSIONPTP_SHIFT;
    mutex_lock(&clock.extreg_lock);
    ext_write(0, phydev, PAGE5, PSF_CFG0, cfg0);
    ext_write(0, phydev, PAGE6, PSF_CFG1, ver);
    mutex_unlock(&clock.extreg_lock);
    if (!phydev.attached_dev) {
    phydev_warn(phydev,
    "expected to find an attached netdevice\n");
    return;
    }
    if (on) {
    if (dev_mc_add(phydev.attached_dev, status_frame_dst))
    phydev_warn(phydev, "failed to add mc address\n");
    } else {
    if (dev_mc_del(phydev.attached_dev, status_frame_dst))
    phydev_warn(phydev, "failed to delete mc address\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn is_status_frame(skb: *mut sk_buff, type: c_int) -> bool {
    static bool is_status_frame(struct sk_buff *skb, int type)
    {
    struct ethhdr *h = eth_hdr(skb);
    if (PTP_CLASS_V2_L2 == type &&
    !memcmp(h.h_source, status_frame_src, sizeof(status_frame_src)))
    return true;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn expired(rxts: *mut rxts) -> c_int {
    static int expired(struct rxts *rxts)
    {
    return time_after(jiffies, rxts.tmo);
    }
// Caller must hold rx_lock.
#[no_mangle]
unsafe extern "C" fn prune_rx_ts(dp83640: *mut dp83640_private) {
    static void prune_rx_ts(struct dp83640_private *dp83640)
    {
    struct list_head *this, *next;
    struct rxts *rxts;
    list_for_each_safe(this, next, &dp83640.rxts) {
    rxts = list_entry(this, struct rxts, list);
    if (expired(rxts)) {
    list_del_init(&rxts.list);
    list_add(&rxts.list, &dp83640.rxpool);
    }
    }
    }
// synchronize the phyters so they act as one clock
#[no_mangle]
unsafe extern "C" fn enable_broadcast(phydev: *mut phy_device, init_page: c_int, on: c_int) {
    static void enable_broadcast(struct phy_device *phydev, int init_page, int on)
    {
    int val;
    phy_write(phydev, PAGESEL, 0);
    val = phy_read(phydev, PHYCR2);
    if (on)
    val |= BC_WRITE;
    else
    val &= ~BC_WRITE;
    phy_write(phydev, PHYCR2, val);
    phy_write(phydev, PAGESEL, init_page);
    }
#[no_mangle]
unsafe extern "C" fn recalibrate(clock: *mut dp83640_clock) {
    static void recalibrate(struct dp83640_clock *clock)
    {
    s64 now, diff;
    struct phy_txts event_ts;
    struct timespec64 ts;
    struct dp83640_private *tmp;
    struct phy_device *master = clock.chosen.phydev;
    u16 cal_gpio, cfg0, evnt, ptp_trig, trigger, val;
    trigger = CAL_TRIGGER;
    cal_gpio = 1 + ptp_find_pin_unlocked(clock.ptp_clock, PTP_PF_PHYSYNC, 0);
    if (cal_gpio < 1) {
    pr_err("PHY calibration pin not available - PHY is not calibrated.");
    return;
    }
    mutex_lock(&clock.extreg_lock);
//
// enable broadcast, disable status frames, enable ptp clock
//
    list_for_each_entry(tmp, &clock.phylist, list) {
    enable_broadcast(tmp.phydev, clock.page, 1);
    tmp.cfg0 = ext_read(tmp.phydev, PAGE5, PSF_CFG0);
    ext_write(0, tmp.phydev, PAGE5, PSF_CFG0, 0);
    ext_write(0, tmp.phydev, PAGE4, PTP_CTL, PTP_ENABLE);
    }
    enable_broadcast(master, clock.page, 1);
    cfg0 = ext_read(master, PAGE5, PSF_CFG0);
    ext_write(0, master, PAGE5, PSF_CFG0, 0);
    ext_write(0, master, PAGE4, PTP_CTL, PTP_ENABLE);
//
// enable an event timestamp
//
    evnt = EVNT_WR | EVNT_RISE | EVNT_SINGLE;
    evnt |= (CAL_EVENT & EVNT_SEL_MASK) << EVNT_SEL_SHIFT;
    evnt |= (cal_gpio & EVNT_GPIO_MASK) << EVNT_GPIO_SHIFT;
    list_for_each_entry(tmp, &clock.phylist, list)
    ext_write(0, tmp.phydev, PAGE5, PTP_EVNT, evnt);
    ext_write(0, master, PAGE5, PTP_EVNT, evnt);
//
// configure a trigger
//
    ptp_trig = TRIG_WR | TRIG_IF_LATE | TRIG_PULSE;
    ptp_trig |= (trigger  & TRIG_CSEL_MASK) << TRIG_CSEL_SHIFT;
    ptp_trig |= (cal_gpio & TRIG_GPIO_MASK) << TRIG_GPIO_SHIFT;
    ext_write(0, master, PAGE5, PTP_TRIG, ptp_trig);
// load trigger
    val = (trigger & TRIG_SEL_MASK) << TRIG_SEL_SHIFT;
    val |= TRIG_LOAD;
    ext_write(0, master, PAGE4, PTP_CTL, val);
// enable trigger
    val &= ~TRIG_LOAD;
    val |= TRIG_EN;
    ext_write(0, master, PAGE4, PTP_CTL, val);
// disable trigger
    val = (trigger & TRIG_SEL_MASK) << TRIG_SEL_SHIFT;
    val |= TRIG_DIS;
    ext_write(0, master, PAGE4, PTP_CTL, val);
//
// read out and correct offsets
//
    val = ext_read(master, PAGE4, PTP_STS);
    phydev_info(master, "master PTP_STS  0x%04hx\n", val);
    val = ext_read(master, PAGE4, PTP_ESTS);
    phydev_info(master, "master PTP_ESTS 0x%04hx\n", val);
    event_ts.ns_lo  = ext_read(master, PAGE4, PTP_EDATA);
    event_ts.ns_hi  = ext_read(master, PAGE4, PTP_EDATA);
    event_ts.sec_lo = ext_read(master, PAGE4, PTP_EDATA);
    event_ts.sec_hi = ext_read(master, PAGE4, PTP_EDATA);
    now = phy2txts(&event_ts);
    list_for_each_entry(tmp, &clock.phylist, list) {
    val = ext_read(tmp.phydev, PAGE4, PTP_STS);
    phydev_info(tmp.phydev, "slave  PTP_STS  0x%04hx\n", val);
    val = ext_read(tmp.phydev, PAGE4, PTP_ESTS);
    phydev_info(tmp.phydev, "slave  PTP_ESTS 0x%04hx\n", val);
    event_ts.ns_lo  = ext_read(tmp.phydev, PAGE4, PTP_EDATA);
    event_ts.ns_hi  = ext_read(tmp.phydev, PAGE4, PTP_EDATA);
    event_ts.sec_lo = ext_read(tmp.phydev, PAGE4, PTP_EDATA);
    event_ts.sec_hi = ext_read(tmp.phydev, PAGE4, PTP_EDATA);
    diff = now - (s64) phy2txts(&event_ts);
    phydev_info(tmp.phydev, "slave offset %lld nanoseconds\n",
    diff);
    diff += ADJTIME_FIX;
    ts = ns_to_timespec64(diff);
    tdr_write(0, tmp.phydev, &ts, PTP_STEP_CLK);
    }
//
// restore status frames
//
    list_for_each_entry(tmp, &clock.phylist, list)
    ext_write(0, tmp.phydev, PAGE5, PSF_CFG0, tmp.cfg0);
    ext_write(0, master, PAGE5, PSF_CFG0, cfg0);
    mutex_unlock(&clock.extreg_lock);
    }
// time stamping methods
#[no_mangle]
pub unsafe extern "C" fn exts_chan_to_edata(ch: c_int) -> u16 {
    static inline u16 exts_chan_to_edata(int ch)
    {
    return 1 << ((ch + EXT_EVENT) * 2);
    }
    static int decode_evnt(struct dp83640_private *dp83640,
    void *data, int len, u16 ests)
    {
    struct phy_txts *phy_txts;
    struct ptp_clock_event event;
    int i, parsed;
    let mut words: c_int = (ests >> EVNT_TS_LEN_SHIFT) & EVNT_TS_LEN_MASK;
    let mut ext_status: u16 = 0;
// calculate length of the event timestamp status message
    if (ests & MULT_EVNT)
    parsed = (words + 2) * sizeof(u16);
    else
    parsed = (words + 1) * sizeof(u16);
// check if enough data is available
    if (len < parsed)
    return len;
    if (ests & MULT_EVNT) {
    ext_status = *(u16 *) data;
    data += sizeof(ext_status);
    }
    phy_txts = data;
    switch (words) {
    case 3:
    dp83640.edata.sec_hi = phy_txts.sec_hi;
    fallthrough;
    case 2:
    dp83640.edata.sec_lo = phy_txts.sec_lo;
    fallthrough;
    case 1:
    dp83640.edata.ns_hi = phy_txts.ns_hi;
    fallthrough;
    case 0:
    dp83640.edata.ns_lo = phy_txts.ns_lo;
    }
    if (!ext_status) {
    i = ((ests >> EVNT_NUM_SHIFT) & EVNT_NUM_MASK) - EXT_EVENT;
    ext_status = exts_chan_to_edata(i);
    }
    event.type = PTP_CLOCK_EXTTS;
    event.timestamp = phy2txts(&dp83640.edata);
// Compensate for input path and synchronization delays
    event.timestamp -= 35;
    for (i = 0; i < N_EXT_TS; i++) {
    if (ext_status & exts_chan_to_edata(i)) {
    event.index = i;
    ptp_clock_event(dp83640.clock.ptp_clock, &event);
    }
    }
    return parsed;
    }
pub const DP83640_PACKET_HASH_LEN: c_int = 10;
#[no_mangle]
unsafe extern "C" fn match(skb: *mut sk_buff, type: c_uint, rxts: *mut rxts) -> c_int {
    static int match(struct sk_buff *skb, unsigned int type, struct rxts *rxts)
    {
    struct ptp_header *hdr;
    u8 msgtype;
    u16 seqid;
    u16 hash;
// check sequenceID, messageType, 12 bit hash of offset 20-29
    hdr = ptp_parse_header(skb, type);
    if (!hdr)
    return 0;
    msgtype = ptp_get_msgtype(hdr, type);
    if (rxts.msgtype != (msgtype & 0xf))
    return 0;
    seqid = be16_to_cpu(hdr.sequence_id);
    if (rxts.seqid != seqid)
    return 0;
    hash = ether_crc(DP83640_PACKET_HASH_LEN,
    (unsigned char *)&hdr.source_port_identity) >> 20;
    if (rxts.hash != hash)
    return 0;
    return 1;
    }
    static void decode_rxts(struct dp83640_private *dp83640,
    struct phy_rxts *phy_rxts)
    {
    struct rxts *rxts;
    struct skb_shared_hwtstamps *shhwtstamps = core::ptr::null_mut();
    struct sk_buff *skb;
    unsigned long flags;
    u8 overflow;
    overflow = (phy_rxts.ns_hi >> 14) & 0x3;
    if (overflow)
    pr_debug("rx timestamp queue overflow, count %d\n", overflow);
    spin_lock_irqsave(&dp83640.rx_lock, flags);
    prune_rx_ts(dp83640);
    if (list_empty(&dp83640.rxpool)) {
    pr_debug("rx timestamp pool is empty\n");
    goto out;
    }
    rxts = list_first_entry(&dp83640.rxpool, struct rxts, list);
    list_del_init(&rxts.list);
    phy2rxts(phy_rxts, rxts);
    spin_lock(&dp83640.rx_queue.lock);
    skb_queue_walk(&dp83640.rx_queue, skb) {
    struct dp83640_skb_info *skb_info;
    skb_info = (struct dp83640_skb_info *)skb.cb;
    if (match(skb, skb_info.ptp_type, rxts)) {
    __skb_unlink(skb, &dp83640.rx_queue);
    shhwtstamps = skb_hwtstamps(skb);
    memset(shhwtstamps, 0, sizeof(*shhwtstamps));
    shhwtstamps.hwtstamp = ns_to_ktime(rxts.ns);
    list_add(&rxts.list, &dp83640.rxpool);
    break;
    }
    }
    spin_unlock(&dp83640.rx_queue.lock);
    if (!shhwtstamps)
    list_add_tail(&rxts.list, &dp83640.rxts);
    out:
    spin_unlock_irqrestore(&dp83640.rx_lock, flags);
    if (shhwtstamps)
    netif_rx(skb);
    }
    static void decode_txts(struct dp83640_private *dp83640,
    struct phy_txts *phy_txts)
    {
    struct skb_shared_hwtstamps shhwtstamps;
    struct dp83640_skb_info *skb_info;
    struct sk_buff *skb;
    u8 overflow;
    u64 ns;
// We must already have the skb that triggered this.
    again:
    skb = skb_dequeue(&dp83640.tx_queue);
    if (!skb) {
    pr_debug("have timestamp but tx_queue empty\n");
    return;
    }
    overflow = (phy_txts.ns_hi >> 14) & 0x3;
    if (overflow) {
    pr_debug("tx timestamp queue overflow, count %d\n", overflow);
    while (skb) {
    kfree_skb(skb);
    skb = skb_dequeue(&dp83640.tx_queue);
    }
    return;
    }
    skb_info = (struct dp83640_skb_info *)skb.cb;
    if (time_after(jiffies, skb_info.tmo)) {
    kfree_skb(skb);
    goto again;
    }
    ns = phy2txts(phy_txts);
    memset(&shhwtstamps, 0, sizeof(shhwtstamps));
    shhwtstamps.hwtstamp = ns_to_ktime(ns);
    skb_complete_tx_timestamp(skb, &shhwtstamps);
    }
    static void decode_status_frame(struct dp83640_private *dp83640,
    struct sk_buff *skb)
    {
    struct phy_rxts *phy_rxts;
    struct phy_txts *phy_txts;
    u8 *ptr;
    int len, size;
    u16 ests, type;
    ptr = skb.data + 2;
    for (len = skb_headlen(skb) - 2; len > sizeof(type); len -= size) {
    type = *(u16 *)ptr;
    ests = type & 0x0fff;
    type = type & 0xf000;
    len -= sizeof(type);
    ptr += sizeof(type);
    if (PSF_RX == type && len >= sizeof(*phy_rxts)) {
    phy_rxts = (struct phy_rxts *) ptr;
    decode_rxts(dp83640, phy_rxts);
    size = sizeof(*phy_rxts);
    } else if (PSF_TX == type && len >= sizeof(*phy_txts)) {
    phy_txts = (struct phy_txts *) ptr;
    decode_txts(dp83640, phy_txts);
    size = sizeof(*phy_txts);
    } else if (PSF_EVNT == type) {
    size = decode_evnt(dp83640, ptr, len, ests);
    } else {
    size = 0;
    break;
    }
    ptr += size;
    }
    }
#[no_mangle]
unsafe extern "C" fn dp83640_clock_init(clock: *mut dp83640_clock) {
    static void dp83640_clock_init(struct dp83640_clock *clock)
    {
    mutex_init(&clock.extreg_lock);
    mutex_init(&clock.clock_lock);
    INIT_LIST_HEAD(&clock.phylist);
    clock.caps.pin_config = clock.pin_config;
    clock.caps.owner = THIS_MODULE;
    sprintf(clock.caps.name, "dp83640 timer");
    clock.caps.max_adj	= 1953124;
    clock.caps.n_alarm	= 0;
    clock.caps.n_ext_ts	= N_EXT_TS;
    clock.caps.n_per_out	= N_PER_OUT;
    clock.caps.n_pins	= DP83640_N_PINS;
    clock.caps.pps		= 0;
    clock.caps.supported_extts_flags = PTP_RISING_EDGE |
    PTP_FALLING_EDGE |
    PTP_STRICT_FLAGS;
    clock.caps.adjfine	= ptp_dp83640_adjfine;
    clock.caps.adjtime	= ptp_dp83640_adjtime;
    clock.caps.gettime64	= ptp_dp83640_gettime;
    clock.caps.settime64	= ptp_dp83640_settime;
    clock.caps.enable	= ptp_dp83640_enable;
    clock.caps.verify	= ptp_dp83640_verify;
// Initialize the runtime pin configuration from gpio_tab.
    dp83640_gpio_defaults(clock.caps.pin_config);
    }
    static int choose_this_phy(struct dp83640_clock *clock,
    struct phy_device *phydev)
    {
    if (chosen_phy == -1 && !clock.chosen)
    return 1;
    if (chosen_phy == phydev.mdio.addr)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83640_soft_reset(phydev: *mut phy_device) -> c_int {
    static int dp83640_soft_reset(struct phy_device *phydev)
    {
    int ret;
    ret = genphy_soft_reset(phydev);
    if (ret < 0)
    return ret;
// From DP83640 datasheet: "Software driver code must wait 3 us
// following a software reset before allowing further serial MII
// operations with the DP83640."
//
    udelay(10);		/* Taking udelay inaccuracy into account */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83640_config_init(phydev: *mut phy_device) -> c_int {
    static int dp83640_config_init(struct phy_device *phydev)
    {
    struct dp83640_private *dp83640 = phydev.priv;
    struct dp83640_clock *clock = dp83640.clock;
    if (clock.chosen && !list_empty(&clock.phylist))
    recalibrate(clock);
    else {
    mutex_lock(&clock.extreg_lock);
    enable_broadcast(phydev, clock.page, 1);
    mutex_unlock(&clock.extreg_lock);
    }
    enable_status_frames(phydev, true);
    mutex_lock(&clock.extreg_lock);
    ext_write(0, phydev, PAGE4, PTP_CTL, PTP_ENABLE);
    mutex_unlock(&clock.extreg_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83640_ack_interrupt(phydev: *mut phy_device) -> c_int {
    static int dp83640_ack_interrupt(struct phy_device *phydev)
    {
    let mut err: c_int = phy_read(phydev, MII_DP83640_MISR);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83640_config_intr(phydev: *mut phy_device) -> c_int {
    static int dp83640_config_intr(struct phy_device *phydev)
    {
    int micr;
    int misr;
    int err;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    err = dp83640_ack_interrupt(phydev);
    if (err)
    return err;
    misr = phy_read(phydev, MII_DP83640_MISR);
    if (misr < 0)
    return misr;
    misr |=
    (MII_DP83640_MISR_ANC_INT_EN |
    MII_DP83640_MISR_DUP_INT_EN |
    MII_DP83640_MISR_SPD_INT_EN |
    MII_DP83640_MISR_LINK_INT_EN);
    err = phy_write(phydev, MII_DP83640_MISR, misr);
    if (err < 0)
    return err;
    micr = phy_read(phydev, MII_DP83640_MICR);
    if (micr < 0)
    return micr;
    micr |=
    (MII_DP83640_MICR_OE |
    MII_DP83640_MICR_IE);
    return phy_write(phydev, MII_DP83640_MICR, micr);
    } else {
    micr = phy_read(phydev, MII_DP83640_MICR);
    if (micr < 0)
    return micr;
    micr &=
    ~(MII_DP83640_MICR_OE |
    MII_DP83640_MICR_IE);
    err = phy_write(phydev, MII_DP83640_MICR, micr);
    if (err < 0)
    return err;
    misr = phy_read(phydev, MII_DP83640_MISR);
    if (misr < 0)
    return misr;
    misr &=
    ~(MII_DP83640_MISR_ANC_INT_EN |
    MII_DP83640_MISR_DUP_INT_EN |
    MII_DP83640_MISR_SPD_INT_EN |
    MII_DP83640_MISR_LINK_INT_EN);
    err = phy_write(phydev, MII_DP83640_MISR, misr);
    if (err)
    return err;
    return dp83640_ack_interrupt(phydev);
    }
    }
#[no_mangle]
unsafe extern "C" fn dp83640_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t dp83640_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read(phydev, MII_DP83640_MISR);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & MII_DP83640_MISR_INT_MASK))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
    static int dp83640_hwtstamp_get(struct mii_timestamper *mii_ts,
    struct kernel_hwtstamp_config *cfg)
    {
    struct dp83640_private *dp83640 =
    container_of(mii_ts, struct dp83640_private, mii_ts);
    cfg.rx_filter = dp83640.hwts_rx_en;
    cfg.tx_type = dp83640.hwts_tx_en;
    return 0;
    }
    static int dp83640_hwtstamp_set(struct mii_timestamper *mii_ts,
    struct kernel_hwtstamp_config *cfg,
    struct netlink_ext_ack *extack)
    {
    struct dp83640_private *dp83640 =
    container_of(mii_ts, struct dp83640_private, mii_ts);
    u16 txcfg0, rxcfg0;
    if (cfg.tx_type < 0 || cfg.tx_type > HWTSTAMP_TX_ONESTEP_SYNC)
    return -ERANGE;
    dp83640.hwts_tx_en = cfg.tx_type;
    switch (cfg.rx_filter) {
    case HWTSTAMP_FILTER_NONE:
    dp83640.hwts_rx_en = 0;
    dp83640.layer = 0;
    dp83640.version = 0;
    break;
    case HWTSTAMP_FILTER_PTP_V1_L4_EVENT:
    case HWTSTAMP_FILTER_PTP_V1_L4_SYNC:
    case HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ:
    dp83640.hwts_rx_en = HWTSTAMP_FILTER_PTP_V1_L4_EVENT;
    dp83640.layer = PTP_CLASS_L4;
    dp83640.version = PTP_CLASS_V1;
    cfg.rx_filter = HWTSTAMP_FILTER_PTP_V1_L4_EVENT;
    break;
    case HWTSTAMP_FILTER_PTP_V2_L4_EVENT:
    case HWTSTAMP_FILTER_PTP_V2_L4_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ:
    dp83640.hwts_rx_en = HWTSTAMP_FILTER_PTP_V2_L4_EVENT;
    dp83640.layer = PTP_CLASS_L4;
    dp83640.version = PTP_CLASS_V2;
    cfg.rx_filter = HWTSTAMP_FILTER_PTP_V2_L4_EVENT;
    break;
    case HWTSTAMP_FILTER_PTP_V2_L2_EVENT:
    case HWTSTAMP_FILTER_PTP_V2_L2_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ:
    dp83640.hwts_rx_en = HWTSTAMP_FILTER_PTP_V2_L2_EVENT;
    dp83640.layer = PTP_CLASS_L2;
    dp83640.version = PTP_CLASS_V2;
    cfg.rx_filter = HWTSTAMP_FILTER_PTP_V2_L2_EVENT;
    break;
    case HWTSTAMP_FILTER_PTP_V2_EVENT:
    case HWTSTAMP_FILTER_PTP_V2_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_DELAY_REQ:
    dp83640.hwts_rx_en = HWTSTAMP_FILTER_PTP_V2_EVENT;
    dp83640.layer = PTP_CLASS_L4 | PTP_CLASS_L2;
    dp83640.version = PTP_CLASS_V2;
    cfg.rx_filter = HWTSTAMP_FILTER_PTP_V2_EVENT;
    break;
    default:
    return -ERANGE;
    }
    txcfg0 = (dp83640.version & TX_PTP_VER_MASK) << TX_PTP_VER_SHIFT;
    rxcfg0 = (dp83640.version & TX_PTP_VER_MASK) << TX_PTP_VER_SHIFT;
    if (dp83640.layer & PTP_CLASS_L2) {
    txcfg0 |= TX_L2_EN;
    rxcfg0 |= RX_L2_EN;
    }
    if (dp83640.layer & PTP_CLASS_L4) {
    txcfg0 |= TX_IPV6_EN | TX_IPV4_EN;
    rxcfg0 |= RX_IPV6_EN | RX_IPV4_EN;
    }
    if (dp83640.hwts_tx_en)
    txcfg0 |= TX_TS_EN;
    if (dp83640.hwts_tx_en == HWTSTAMP_TX_ONESTEP_SYNC)
    txcfg0 |= SYNC_1STEP | CHK_1STEP;
    if (dp83640.hwts_rx_en)
    rxcfg0 |= RX_TS_EN;
    mutex_lock(&dp83640.clock.extreg_lock);
    ext_write(0, dp83640.phydev, PAGE5, PTP_TXCFG0, txcfg0);
    ext_write(0, dp83640.phydev, PAGE5, PTP_RXCFG0, rxcfg0);
    mutex_unlock(&dp83640.clock.extreg_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx_timestamp_work(work: *mut work_struct) {
    static void rx_timestamp_work(struct work_struct *work)
    {
    struct dp83640_private *dp83640 =
    container_of(work, struct dp83640_private, ts_work.work);
    struct sk_buff *skb;
// Deliver expired packets.
    while ((skb = skb_dequeue(&dp83640.rx_queue))) {
    struct dp83640_skb_info *skb_info;
    skb_info = (struct dp83640_skb_info *)skb.cb;
    if (!time_after(jiffies, skb_info.tmo)) {
    skb_queue_head(&dp83640.rx_queue, skb);
    break;
    }
    netif_rx(skb);
    }
    if (!skb_queue_empty(&dp83640.rx_queue))
    schedule_delayed_work(&dp83640.ts_work, SKB_TIMESTAMP_TIMEOUT);
    }
    static bool dp83640_rxtstamp(struct mii_timestamper *mii_ts,
    struct sk_buff *skb, int type)
    {
    struct dp83640_private *dp83640 =
    container_of(mii_ts, struct dp83640_private, mii_ts);
    struct dp83640_skb_info *skb_info = (struct dp83640_skb_info *)skb.cb;
    struct list_head *this, *next;
    struct rxts *rxts;
    struct skb_shared_hwtstamps *shhwtstamps = core::ptr::null_mut();
    unsigned long flags;
    if (is_status_frame(skb, type)) {
    decode_status_frame(dp83640, skb);
    kfree_skb(skb);
    return true;
    }
    if (!dp83640.hwts_rx_en)
    return false;
    if ((type & dp83640.version) == 0 || (type & dp83640.layer) == 0)
    return false;
    spin_lock_irqsave(&dp83640.rx_lock, flags);
    prune_rx_ts(dp83640);
    list_for_each_safe(this, next, &dp83640.rxts) {
    rxts = list_entry(this, struct rxts, list);
    if (match(skb, type, rxts)) {
    shhwtstamps = skb_hwtstamps(skb);
    memset(shhwtstamps, 0, sizeof(*shhwtstamps));
    shhwtstamps.hwtstamp = ns_to_ktime(rxts.ns);
    list_del_init(&rxts.list);
    list_add(&rxts.list, &dp83640.rxpool);
    break;
    }
    }
    spin_unlock_irqrestore(&dp83640.rx_lock, flags);
    if (!shhwtstamps) {
    skb_info.ptp_type = type;
    skb_info.tmo = jiffies + SKB_TIMESTAMP_TIMEOUT;
    skb_queue_tail(&dp83640.rx_queue, skb);
    schedule_delayed_work(&dp83640.ts_work, SKB_TIMESTAMP_TIMEOUT);
    } else {
    netif_rx(skb);
    }
    return true;
    }
    static void dp83640_txtstamp(struct mii_timestamper *mii_ts,
    struct sk_buff *skb, int type)
    {
    struct dp83640_skb_info *skb_info = (struct dp83640_skb_info *)skb.cb;
    struct dp83640_private *dp83640 =
    container_of(mii_ts, struct dp83640_private, mii_ts);
    switch (dp83640.hwts_tx_en) {
    case HWTSTAMP_TX_ONESTEP_SYNC:
    if (ptp_msg_is_sync(skb, type)) {
    kfree_skb(skb);
    return;
    }
    fallthrough;
    case HWTSTAMP_TX_ON:
    skb_shinfo(skb).tx_flags |= SKBTX_IN_PROGRESS;
    skb_info.tmo = jiffies + SKB_TIMESTAMP_TIMEOUT;
    skb_queue_tail(&dp83640.tx_queue, skb);
    break;
    case HWTSTAMP_TX_OFF:
    default:
    kfree_skb(skb);
    break;
    }
    }
    static int dp83640_ts_info(struct mii_timestamper *mii_ts,
    struct kernel_ethtool_ts_info *info)
    {
    struct dp83640_private *dp83640 =
    container_of(mii_ts, struct dp83640_private, mii_ts);
    info.so_timestamping =
    SOF_TIMESTAMPING_TX_HARDWARE |
    SOF_TIMESTAMPING_RX_HARDWARE |
    SOF_TIMESTAMPING_RAW_HARDWARE;
    info.phc_index = ptp_clock_index(dp83640.clock.ptp_clock);
    info.tx_types =
    (1 << HWTSTAMP_TX_OFF) |
    (1 << HWTSTAMP_TX_ON) |
    (1 << HWTSTAMP_TX_ONESTEP_SYNC);
    info.rx_filters =
    (1 << HWTSTAMP_FILTER_NONE) |
    (1 << HWTSTAMP_FILTER_PTP_V1_L4_EVENT) |
    (1 << HWTSTAMP_FILTER_PTP_V2_L4_EVENT) |
    (1 << HWTSTAMP_FILTER_PTP_V2_L2_EVENT) |
    (1 << HWTSTAMP_FILTER_PTP_V2_EVENT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83640_probe(phydev: *mut phy_device) -> c_int {
    static int dp83640_probe(struct phy_device *phydev)
    {
    struct dp83640_private *dp83640;
    struct dp83640_clock *clock;
    int err, i;
    if (phydev.mdio.addr == BROADCAST_ADDR)
    return 0;
    err = phy_package_join(phydev, BROADCAST_ADDR, sizeof(*clock));
    if (err)
    return err;
    clock = phy_package_get_priv(phydev);
// Ensure other PHY probes wait for shared clock initialization.
    phy_package_lock(phydev);
    if (phy_package_probe_once(phydev))
    dp83640_clock_init(clock);
    phy_package_unlock(phydev);
    mutex_lock(&clock.clock_lock);
    dp83640 = kzalloc_obj(struct dp83640_private);
    if (!dp83640) {
    err = -ENOMEM;
    goto no_memory;
    }
    dp83640.phydev = phydev;
    dp83640.mii_ts.rxtstamp = dp83640_rxtstamp;
    dp83640.mii_ts.txtstamp = dp83640_txtstamp;
    dp83640.mii_ts.hwtstamp_set = dp83640_hwtstamp_set;
    dp83640.mii_ts.hwtstamp_get = dp83640_hwtstamp_get;
    dp83640.mii_ts.ts_info  = dp83640_ts_info;
    INIT_DELAYED_WORK(&dp83640.ts_work, rx_timestamp_work);
    INIT_LIST_HEAD(&dp83640.rxts);
    INIT_LIST_HEAD(&dp83640.rxpool);
    for (i = 0; i < MAX_RXTS; i++)
    list_add(&dp83640.rx_pool_data[i].list, &dp83640.rxpool);
// Timestamp selected by default to keep legacy API
    phydev.default_timestamp = true;
    phydev.mii_ts = &dp83640.mii_ts;
    phydev.priv = dp83640;
    spin_lock_init(&dp83640.rx_lock);
    skb_queue_head_init(&dp83640.rx_queue);
    skb_queue_head_init(&dp83640.tx_queue);
    dp83640.clock = clock;
    if (choose_this_phy(clock, phydev)) {
    clock.chosen = dp83640;
    clock.ptp_clock = ptp_clock_register(&clock.caps,
    &phydev.mdio.dev);
    if (IS_ERR(clock.ptp_clock)) {
    err = PTR_ERR(clock.ptp_clock);
    goto no_register;
    }
    } else
    list_add_tail(&dp83640.list, &clock.phylist);
    mutex_unlock(&clock.clock_lock);
    return 0;
    no_register:
    clock.chosen = core::ptr::null_mut();
    clock.ptp_clock = core::ptr::null_mut();
    phydev.default_timestamp = false;
    phydev.mii_ts = core::ptr::null_mut();
    phydev.priv = core::ptr::null_mut();
    kfree(dp83640);
    no_memory:
    mutex_unlock(&clock.clock_lock);
    phy_package_leave(phydev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dp83640_remove(phydev: *mut phy_device) {
    static void dp83640_remove(struct phy_device *phydev)
    {
    struct dp83640_clock *clock;
    struct list_head *this, *next;
    struct dp83640_private *tmp, *dp83640 = phydev.priv;
    if (phydev.mdio.addr == BROADCAST_ADDR)
    return;
    phydev.mii_ts = core::ptr::null_mut();
    enable_status_frames(phydev, false);
    cancel_delayed_work_sync(&dp83640.ts_work);
    skb_queue_purge(&dp83640.rx_queue);
    skb_queue_purge(&dp83640.tx_queue);
    clock = dp83640.clock;
    mutex_lock(&clock.clock_lock);
    if (dp83640 == clock.chosen) {
    ptp_clock_unregister(clock.ptp_clock);
    clock.chosen = core::ptr::null_mut();
    } else {
    list_for_each_safe(this, next, &clock.phylist) {
    tmp = list_entry(this, struct dp83640_private, list);
    if (tmp == dp83640) {
    list_del_init(&tmp.list);
    break;
    }
    }
    }
    mutex_unlock(&clock.clock_lock);
    kfree(dp83640);
    phy_package_leave(phydev);
    }
    static struct phy_driver dp83640_driver[] = {
    {
    .phy_id		= DP83640_PHY_ID,
    .phy_id_mask	= 0xfffffff0,
    .name		= "NatSemi DP83640",
// PHY_BASIC_FEATURES
    .probe		= dp83640_probe,
    .remove		= dp83640_remove,
    .soft_reset	= dp83640_soft_reset,
    .config_init	= dp83640_config_init,
    .config_intr    = dp83640_config_intr,
    .handle_interrupt = dp83640_handle_interrupt,
    },
    };
    module_phy_driver(dp83640_driver);
    MODULE_DESCRIPTION("National Semiconductor DP83640 PHY driver");
    MODULE_AUTHOR("Richard Cochran <richardcochran@gmail.com>");
    MODULE_LICENSE("GPL");
    static const struct mdio_device_id __maybe_unused dp83640_tbl[] = {
    { DP83640_PHY_ID, 0xfffffff0 },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, dp83640_tbl);
