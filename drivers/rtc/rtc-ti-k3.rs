//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ti-k3.c
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
//
// Texas Instruments K3 RTC driver
//
// Copyright (C) 2021-2022 Texas Instruments Incorporated - https://www.ti.com
//

// Registers
pub const REG_K3RTC_S_CNT_LSW: c_uint = 0x08;
pub const REG_K3RTC_S_CNT_MSW: c_uint = 0x0c;
pub const REG_K3RTC_COMP: c_uint = 0x10;
pub const REG_K3RTC_ON_OFF_S_CNT_LSW: c_uint = 0x20;
pub const REG_K3RTC_ON_OFF_S_CNT_MSW: c_uint = 0x24;
pub const REG_K3RTC_SCRATCH0: c_uint = 0x30;
pub const REG_K3RTC_SCRATCH7: c_uint = 0x4c;
pub const REG_K3RTC_GENERAL_CTL: c_uint = 0x50;
pub const REG_K3RTC_IRQSTATUS_RAW_SYS: c_uint = 0x54;
pub const REG_K3RTC_IRQSTATUS_SYS: c_uint = 0x58;
pub const REG_K3RTC_IRQENABLE_SET_SYS: c_uint = 0x5c;
pub const REG_K3RTC_IRQENABLE_CLR_SYS: c_uint = 0x60;
pub const REG_K3RTC_SYNCPEND: c_uint = 0x68;
pub const REG_K3RTC_KICK0: c_uint = 0x70;
pub const REG_K3RTC_KICK1: c_uint = 0x74;
// Freeze when lsw is read and unfreeze when msw is read

// Magic values for lock/unlock
pub const K3RTC_KICK0_UNLOCK_VALUE: c_uint = 0x83e70b13;
pub const K3RTC_KICK1_UNLOCK_VALUE: c_uint = 0x95a4f1e0;
// Multiplier for ppb conversions

// Min and max values supported with 'offset' interface (swapped sign)

    static const struct regmap_config ti_k3_rtc_regmap_config = {
    .name = "peripheral-registers",
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = REG_K3RTC_KICK1,
    };
    enum ti_k3_rtc_fields {
    K3RTC_KICK0,
    K3RTC_KICK1,
    K3RTC_S_CNT_LSW,
    K3RTC_S_CNT_MSW,
    K3RTC_O32K_OSC_DEP_EN,
    K3RTC_UNLOCK,
    K3RTC_CNT_FMODE,
    K3RTC_PEND,
    K3RTC_RELOAD_FROM_BBD,
    K3RTC_COMP,
    K3RTC_ALM_S_CNT_LSW,
    K3RTC_ALM_S_CNT_MSW,
    K3RTC_IRQ_STATUS_RAW,
    K3RTC_IRQ_STATUS,
    K3RTC_IRQ_ENABLE_SET,
    K3RTC_IRQ_ENABLE_CLR,
    K3RTC_IRQ_STATUS_ALT,
    K3RTC_IRQ_ENABLE_CLR_ALT,
    K3_RTC_MAX_FIELDS
    };
    static const struct reg_field ti_rtc_reg_fields[] = {
    [K3RTC_KICK0] = REG_FIELD(REG_K3RTC_KICK0, 0, 31),
    [K3RTC_KICK1] = REG_FIELD(REG_K3RTC_KICK1, 0, 31),
    [K3RTC_S_CNT_LSW] = REG_FIELD(REG_K3RTC_S_CNT_LSW, 0, 31),
    [K3RTC_S_CNT_MSW] = REG_FIELD(REG_K3RTC_S_CNT_MSW, 0, 15),
    [K3RTC_O32K_OSC_DEP_EN] = REG_FIELD(REG_K3RTC_GENERAL_CTL, 21, 21),
    [K3RTC_UNLOCK] = REG_FIELD(REG_K3RTC_GENERAL_CTL, 23, 23),
    [K3RTC_CNT_FMODE] = REG_FIELD(REG_K3RTC_GENERAL_CTL, 24, 25),
    [K3RTC_PEND] = REG_FIELD(REG_K3RTC_SYNCPEND, 0, 1),
    [K3RTC_RELOAD_FROM_BBD] = REG_FIELD(REG_K3RTC_SYNCPEND, 31, 31),
    [K3RTC_COMP] = REG_FIELD(REG_K3RTC_COMP, 0, 31),
// We use on to off as alarm trigger
    [K3RTC_ALM_S_CNT_LSW] = REG_FIELD(REG_K3RTC_ON_OFF_S_CNT_LSW, 0, 31),
    [K3RTC_ALM_S_CNT_MSW] = REG_FIELD(REG_K3RTC_ON_OFF_S_CNT_MSW, 0, 15),
    [K3RTC_IRQ_STATUS_RAW] = REG_FIELD(REG_K3RTC_IRQSTATUS_RAW_SYS, 0, 0),
    [K3RTC_IRQ_STATUS] = REG_FIELD(REG_K3RTC_IRQSTATUS_SYS, 0, 0),
    [K3RTC_IRQ_ENABLE_SET] = REG_FIELD(REG_K3RTC_IRQENABLE_SET_SYS, 0, 0),
    [K3RTC_IRQ_ENABLE_CLR] = REG_FIELD(REG_K3RTC_IRQENABLE_CLR_SYS, 0, 0),
// Off to on is alternate
    [K3RTC_IRQ_STATUS_ALT] = REG_FIELD(REG_K3RTC_IRQSTATUS_SYS, 1, 1),
    [K3RTC_IRQ_ENABLE_CLR_ALT] = REG_FIELD(REG_K3RTC_IRQENABLE_CLR_SYS, 1, 1),
    };
//
// struct ti_k3_rtc - Private data for ti-k3-rtc
// @irq:		IRQ
// @sync_timeout_us:	data sync timeout period in uSec
// @rate_32k:		32k clock rate in Hz
// @rtc_dev:		rtc device
// @regmap:		rtc mmio regmap
// @r_fields:		rtc register fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_k3_rtc {
    pub irq: c_uint,
    pub sync_timeout_us: u32,
    pub rate_32k: c_ulong,
    pub rtc_dev: *mut rtc_device,
    pub regmap: *mut regmap,
    pub r_fields: [*mut regmap_field; K3_RTC_MAX_FIELDS],
}

#[no_mangle]
unsafe extern "C" fn k3rtc_field_read(priv: *mut ti_k3_rtc, f: enum ti_k3_rtc_fields) -> c_int {
    static int k3rtc_field_read(struct ti_k3_rtc *priv, enum ti_k3_rtc_fields f)
    {
    int ret;
    int val;
    ret = regmap_field_read(priv.r_fields[f], &val);
//
// We shouldn't be seeing regmap fail on us for mmio reads
// This is possible if clock context fails, but that isn't the case for us
//
    if (WARN_ON_ONCE(ret))
    return ret;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn k3rtc_field_write(priv: *mut ti_k3_rtc, f: enum ti_k3_rtc_fields, val: u32) {
    static void k3rtc_field_write(struct ti_k3_rtc *priv, enum ti_k3_rtc_fields f, u32 val)
    {
    regmap_field_write(priv.r_fields[f], val);
    }
//
// k3rtc_fence  - Ensure a register sync took place between the two domains
// @priv:      pointer to priv data
//
// Return: 0 if the sync took place, else returns -ETIMEDOUT
//
#[no_mangle]
unsafe extern "C" fn k3rtc_fence(priv: *mut ti_k3_rtc) -> c_int {
    static int k3rtc_fence(struct ti_k3_rtc *priv)
    {
    int ret;
    ret = regmap_field_read_poll_timeout(priv.r_fields[K3RTC_PEND], ret,
    !ret, 2, priv.sync_timeout_us);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn k3rtc_check_unlocked(priv: *mut ti_k3_rtc) -> c_int {
    static inline int k3rtc_check_unlocked(struct ti_k3_rtc *priv)
    {
    int ret;
    ret = k3rtc_field_read(priv, K3RTC_UNLOCK);
    if (ret < 0)
    return ret;
    return (ret) ? 0 : 1;
    }
#[no_mangle]
unsafe extern "C" fn k3rtc_unlock_rtc(priv: *mut ti_k3_rtc) -> c_int {
    static int k3rtc_unlock_rtc(struct ti_k3_rtc *priv)
    {
    int ret;
    ret = k3rtc_check_unlocked(priv);
    if (!ret)
    return ret;
    k3rtc_field_write(priv, K3RTC_KICK0, K3RTC_KICK0_UNLOCK_VALUE);
    k3rtc_field_write(priv, K3RTC_KICK1, K3RTC_KICK1_UNLOCK_VALUE);
// Skip fence since we are going to check the unlock bit as fence
    ret = regmap_field_read_poll_timeout(priv.r_fields[K3RTC_UNLOCK], ret,
    ret, 2, priv.sync_timeout_us);
    return ret;
    }
//
// This is the list of SoCs affected by TI's i2327 errata causing the RTC
// state-machine to break if not unlocked fast enough during boot. These
// SoCs must have the bootloader unlock this device very early in the
// boot-flow before we (Linux) can use this device.
//
    static const struct soc_device_attribute has_erratum_i2327[] = {
    { .family = "AM62X", .revision = "SR1.0" },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn k3rtc_configure(dev: *mut device) -> c_int {
    static int k3rtc_configure(struct device *dev)
    {
    int ret;
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
//
// HWBUG: The compare state machine is broken if the RTC module
// is NOT unlocked in under one second of boot - which is pretty long
// time from the perspective of Linux driver (module load, u-boot
// shell all can take much longer than this.
//
// In such occurrence, it is assumed that the RTC module is unusable
//
    if (soc_device_match(has_erratum_i2327)) {
    ret = k3rtc_check_unlocked(priv);
// If there is an error OR if we are locked, return error
    if (ret) {
    dev_err(dev,
    HW_ERR "Erratum i2327 unlock QUIRK! Cannot operate!!\n");
    return -EFAULT;
    }
    } else {
// May need to explicitly unlock first time
    ret = k3rtc_unlock_rtc(priv);
    if (ret) {
    dev_err(dev, "Failed to unlock(%d)!\n", ret);
    return ret;
    }
    }
// Enable Shadow register sync on 32k clock boundary
    k3rtc_field_write(priv, K3RTC_O32K_OSC_DEP_EN, 0x1);
//
// Wait at least clock sync time before proceeding further programming.
// This ensures that the 32k based sync is active.
//
    usleep_range(priv.sync_timeout_us, priv.sync_timeout_us + 5);
// We need to ensure fence here to make sure sync here
    ret = k3rtc_fence(priv);
    if (ret) {
    dev_err(dev,
    "Failed fence osc_dep enable(%d) - is 32k clk working?!\n", ret);
    return ret;
    }
//
// FMODE setting: Reading lower seconds will freeze value on higher
// seconds. This also implies that we must *ALWAYS* read lower seconds
// prior to reading higher seconds
//
    k3rtc_field_write(priv, K3RTC_CNT_FMODE, K3RTC_CNT_FMODE_S_CNT_VALUE);
// Clear any spurious IRQ sources if any
    k3rtc_field_write(priv, K3RTC_IRQ_STATUS_ALT, 0x1);
    k3rtc_field_write(priv, K3RTC_IRQ_STATUS, 0x1);
// Disable all IRQs
    k3rtc_field_write(priv, K3RTC_IRQ_ENABLE_CLR_ALT, 0x1);
    k3rtc_field_write(priv, K3RTC_IRQ_ENABLE_CLR, 0x1);
// And.. Let us Sync the writes in
    return k3rtc_fence(priv);
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ti_k3_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    u32 seconds_lo, seconds_hi;
    seconds_lo = k3rtc_field_read(priv, K3RTC_S_CNT_LSW);
    seconds_hi = k3rtc_field_read(priv, K3RTC_S_CNT_MSW);
    rtc_time64_to_tm((((time64_t)seconds_hi) << 32) | (time64_t)seconds_lo, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ti_k3_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    time64_t seconds;
    seconds = rtc_tm_to_time64(tm);
//
// Read operation on LSW will freeze the RTC, so to update
// the time, we cannot use field operations. Just write since the
// reserved bits are ignored.
//
    regmap_write(priv.regmap, REG_K3RTC_S_CNT_LSW, seconds);
    regmap_write(priv.regmap, REG_K3RTC_S_CNT_MSW, seconds >> 32);
    return k3rtc_fence(priv);
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int ti_k3_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    u32 reg;
    let mut offset: u32 = enabled ? K3RTC_IRQ_ENABLE_SET : K3RTC_IRQ_ENABLE_CLR;
    reg = k3rtc_field_read(priv, K3RTC_IRQ_ENABLE_SET);
    if ((enabled && reg) || (!enabled && !reg))
    return 0;
    k3rtc_field_write(priv, offset, 0x1);
//
// Ensure the write sync is through - NOTE: it should be OK to have
// ISR to fire as we are checking sync (which should be done in a 32k
// cycle or so).
//
    return k3rtc_fence(priv);
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int ti_k3_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    u32 seconds_lo, seconds_hi;
    seconds_lo = k3rtc_field_read(priv, K3RTC_ALM_S_CNT_LSW);
    seconds_hi = k3rtc_field_read(priv, K3RTC_ALM_S_CNT_MSW);
    rtc_time64_to_tm((((time64_t)seconds_hi) << 32) | (time64_t)seconds_lo, &alarm.time);
    alarm.enabled = k3rtc_field_read(priv, K3RTC_IRQ_ENABLE_SET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int ti_k3_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    time64_t seconds;
    int ret;
    seconds = rtc_tm_to_time64(&alarm.time);
    k3rtc_field_write(priv, K3RTC_ALM_S_CNT_LSW, seconds);
    k3rtc_field_write(priv, K3RTC_ALM_S_CNT_MSW, (seconds >> 32));
// Make sure the alarm time is synced in
    ret = k3rtc_fence(priv);
    if (ret) {
    dev_err(dev, "Failed to fence(%d)! Potential config issue?\n", ret);
    return ret;
    }
// Alarm IRQ enable will do a sync
    return ti_k3_rtc_alarm_irq_enable(dev, alarm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_read_offset(dev: *mut device, offset: *mut c_long) -> c_int {
    static int ti_k3_rtc_read_offset(struct device *dev, long *offset)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    let mut ticks_per_hr: u32 = priv.rate_32k * 3600;
    int comp;
    s64 tmp;
    comp = k3rtc_field_read(priv, K3RTC_COMP);
// Convert from RTC calibration register format to ppb format
    tmp = comp * (s64)K3RTC_PPB_MULT;
    if (tmp < 0)
    tmp -= ticks_per_hr / 2LL;
    else
    tmp += ticks_per_hr / 2LL;
    tmp = div_s64(tmp, ticks_per_hr);
// Offset value operates in negative way, so swap sign
// offset = (long)-tmp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_set_offset(dev: *mut device, offset: c_long) -> c_int {
    static int ti_k3_rtc_set_offset(struct device *dev, long offset)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    let mut ticks_per_hr: u32 = priv.rate_32k * 3600;
    int comp;
    s64 tmp;
// Make sure offset value is within supported range
    if (offset < K3RTC_MIN_OFFSET || offset > K3RTC_MAX_OFFSET)
    return -ERANGE;
// Convert from ppb format to RTC calibration register format
    tmp = offset * (s64)ticks_per_hr;
    if (tmp < 0)
    tmp -= K3RTC_PPB_MULT / 2LL;
    else
    tmp += K3RTC_PPB_MULT / 2LL;
    tmp = div_s64(tmp, K3RTC_PPB_MULT);
// Offset value operates in negative way, so swap sign
    comp = (int)-tmp;
    k3rtc_field_write(priv, K3RTC_COMP, comp);
    return k3rtc_fence(priv);
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_interrupt(irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ti_k3_rtc_interrupt(s32 irq, void *dev_id)
    {
    struct device *dev = dev_id;
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    u32 reg;
    int ret;
//
// IRQ assertion can be very fast, however, the IRQ Status clear
// de-assert depends on 32k clock edge in the 32k domain
// If we clear the status prior to the first 32k clock edge,
// the status bit is cleared, but the IRQ stays re-asserted.
//
// To prevent this condition, we need to wait for clock sync time.
// We can either do that by polling the 32k observability signal for
// a toggle OR we could just sleep and let the processor do other
// stuff.
//
    usleep_range(priv.sync_timeout_us, priv.sync_timeout_us + 2);
// Lets make sure that this is a valid interrupt
    reg = k3rtc_field_read(priv, K3RTC_IRQ_STATUS);
    if (!reg) {
    let mut raw: u32 = k3rtc_field_read(priv, K3RTC_IRQ_STATUS_RAW);
    dev_err(dev,
    HW_ERR
    "Erratum i2327/IRQ trig: status: 0x%08x / 0x%08x\n", reg, raw);
    return IRQ_NONE;
    }
//
// Write 1 to clear status reg
// We cannot use a field operation here due to a potential race between
// 32k domain and vbus domain.
//
    regmap_write(priv.regmap, REG_K3RTC_IRQSTATUS_SYS, 0x1);
// Sync the write in
    ret = k3rtc_fence(priv);
    if (ret) {
    dev_err(dev, "Failed to fence irq status clr(%d)!\n", ret);
    return IRQ_NONE;
    }
//
// Force the 32k status to be reloaded back in to ensure status is
// reflected back correctly.
//
    k3rtc_field_write(priv, K3RTC_RELOAD_FROM_BBD, 0x1);
// Ensure the write sync is through
    ret = k3rtc_fence(priv);
    if (ret) {
    dev_err(dev, "Failed to fence reload from bbd(%d)!\n", ret);
    return IRQ_NONE;
    }
// Now we ensure that the status bit is cleared
    ret = regmap_field_read_poll_timeout(priv.r_fields[K3RTC_IRQ_STATUS],
    ret, !ret, 2, priv.sync_timeout_us);
    if (ret) {
    dev_err(dev, "Time out waiting for status clear\n");
    return IRQ_NONE;
    }
// Notify RTC core on event
    rtc_update_irq(priv.rtc_dev, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
    static const struct rtc_class_ops ti_k3_rtc_ops = {
    .read_time = ti_k3_rtc_read_time,
    .set_time = ti_k3_rtc_set_time,
    .read_alarm = ti_k3_rtc_read_alarm,
    .set_alarm = ti_k3_rtc_set_alarm,
    .read_offset = ti_k3_rtc_read_offset,
    .set_offset = ti_k3_rtc_set_offset,
    .alarm_irq_enable = ti_k3_rtc_alarm_irq_enable,
    };
    static int ti_k3_rtc_scratch_read(void *priv_data, unsigned int offset,
    void *val, size_t bytes)
    {
    struct ti_k3_rtc *priv = (struct ti_k3_rtc *)priv_data;
    return regmap_bulk_read(priv.regmap, REG_K3RTC_SCRATCH0 + offset, val, bytes / 4);
    }
    static int ti_k3_rtc_scratch_write(void *priv_data, unsigned int offset,
    void *val, size_t bytes)
    {
    struct ti_k3_rtc *priv = (struct ti_k3_rtc *)priv_data;
    int ret;
    ret = regmap_bulk_write(priv.regmap, REG_K3RTC_SCRATCH0 + offset, val, bytes / 4);
    if (ret)
    return ret;
    return k3rtc_fence(priv);
    }
    static struct nvmem_config ti_k3_rtc_nvmem_config = {
    .name = "ti_k3_rtc_scratch",
    .word_size = 4,
    .stride = 4,
    .size = REG_K3RTC_SCRATCH7 - REG_K3RTC_SCRATCH0 + 4,
    .reg_read = ti_k3_rtc_scratch_read,
    .reg_write = ti_k3_rtc_scratch_write,
    };
#[no_mangle]
unsafe extern "C" fn k3rtc_get_32kclk(dev: *mut device, priv: *mut ti_k3_rtc) -> c_int {
    static int k3rtc_get_32kclk(struct device *dev, struct ti_k3_rtc *priv)
    {
    struct clk *clk;
    clk = devm_clk_get_enabled(dev, "osc32k");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    priv.rate_32k = clk_get_rate(clk);
// Make sure we are exact 32k clock. Else, try to compensate delay
    if (priv.rate_32k != 32768)
    dev_warn(dev, "Clock rate %ld is not 32768! Could misbehave!\n",
    priv.rate_32k);
//
// Sync timeout should be two 32k clk sync cycles = ~61uS. We double
// it to comprehend intermediate bus segment and cpu frequency
// deltas
//
    priv.sync_timeout_us = (u32)(DIV_ROUND_UP_ULL(1000000, priv.rate_32k) * 4);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k3rtc_get_vbusclk(dev: *mut device, priv: *mut ti_k3_rtc) -> c_int {
    static int k3rtc_get_vbusclk(struct device *dev, struct ti_k3_rtc *priv)
    {
    struct clk *clk;
// Note: VBUS isn't a context clock, it is needed for hardware operation
    clk = devm_clk_get_enabled(dev, "vbus");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ti_k3_rtc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ti_k3_rtc *priv;
    void __iomem *rtc_base;
    int ret;
    priv = devm_kzalloc(dev, sizeof(struct ti_k3_rtc), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    rtc_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rtc_base))
    return PTR_ERR(rtc_base);
    priv.regmap = devm_regmap_init_mmio(dev, rtc_base, &ti_k3_rtc_regmap_config);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    ret = devm_regmap_field_bulk_alloc(dev, priv.regmap, priv.r_fields,
    ti_rtc_reg_fields, K3_RTC_MAX_FIELDS);
    if (ret)
    return ret;
    ret = k3rtc_get_32kclk(dev, priv);
    if (ret)
    return ret;
    ret = k3rtc_get_vbusclk(dev, priv);
    if (ret)
    return ret;
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    priv.irq = (unsigned int)ret;
    priv.rtc_dev = devm_rtc_allocate_device(dev);
    if (IS_ERR(priv.rtc_dev))
    return PTR_ERR(priv.rtc_dev);
    priv.rtc_dev.ops = &ti_k3_rtc_ops;
    priv.rtc_dev.range_max = (1ULL << 48) - 1;	/* 48Bit seconds */
    ti_k3_rtc_nvmem_config.priv = priv;
    ret = devm_request_threaded_irq(dev, priv.irq, core::ptr::null_mut(),
    ti_k3_rtc_interrupt,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    dev_name(dev), dev);
    if (ret) {
    dev_err(dev, "Could not request IRQ: %d\n", ret);
    return ret;
    }
    platform_set_drvdata(pdev, priv);
    ret = k3rtc_configure(dev);
    if (ret)
    return ret;
    if (device_property_present(dev, "wakeup-source"))
    device_init_wakeup(dev, true);
    else
    device_set_wakeup_capable(dev, true);
    ret = devm_rtc_register_device(priv.rtc_dev);
    if (ret)
    return ret;
    return devm_rtc_nvmem_register(priv.rtc_dev, &ti_k3_rtc_nvmem_config);
    }
    static const struct of_device_id ti_k3_rtc_of_match_table[] = {
    {.compatible = "ti,am62-rtc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ti_k3_rtc_of_match_table);
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ti_k3_rtc_suspend(struct device *dev)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    return enable_irq_wake(priv.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_k3_rtc_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ti_k3_rtc_resume(struct device *dev)
    {
    struct ti_k3_rtc *priv = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (k3rtc_check_unlocked(priv)) {
// RTC locked implies low power mode exit where RTC loses context
    ret = k3rtc_configure(dev);
    if (ret)
    return ret;
    }
    if (device_may_wakeup(dev))
    disable_irq_wake(priv.irq);
    return ret;
    }
    static SIMPLE_DEV_PM_OPS(ti_k3_rtc_pm_ops, ti_k3_rtc_suspend, ti_k3_rtc_resume);
    static struct platform_driver ti_k3_rtc_driver = {
    .probe = ti_k3_rtc_probe,
    .driver = {
    .name = "rtc-ti-k3",
    .of_match_table = ti_k3_rtc_of_match_table,
    .pm = &ti_k3_rtc_pm_ops,
    },
    };
    module_platform_driver(ti_k3_rtc_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("TI K3 RTC driver");
    MODULE_AUTHOR("Nishanth Menon");
