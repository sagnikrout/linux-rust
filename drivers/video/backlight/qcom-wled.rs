//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/qcom-wled.c
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
// Copyright (c) 2015, Sony Mobile Communications, AB.
//

// From DT binding
pub const WLED_MAX_STRINGS: c_int = 4;
pub const MOD_A: c_int = 0;
pub const MOD_B: c_int = 1;
pub const WLED_DEFAULT_BRIGHTNESS: c_int = 2048;
pub const WLED_SOFT_START_DLY_US: c_int = 10000;
pub const WLED3_SINK_REG_BRIGHT_MAX: c_uint = 0xFFF;
pub const WLED5_SINK_REG_BRIGHT_MAX_12B: c_uint = 0xFFF;
pub const WLED5_SINK_REG_BRIGHT_MAX_15B: c_uint = 0x7FFF;
// WLED3/WLED4 control registers
pub const WLED3_CTRL_REG_FAULT_STATUS: c_uint = 0x08;

pub const WLED3_CTRL_REG_INT_RT_STS: c_uint = 0x10;

pub const WLED3_CTRL_REG_MOD_EN: c_uint = 0x46;

pub const WLED3_CTRL_REG_MOD_EN_SHIFT: c_int = 7;
pub const WLED3_CTRL_REG_FEEDBACK_CONTROL: c_uint = 0x48;
pub const WLED3_CTRL_REG_FREQ: c_uint = 0x4c;

pub const WLED3_CTRL_REG_OVP: c_uint = 0x4d;

pub const WLED3_CTRL_REG_ILIMIT: c_uint = 0x4e;

// WLED3/WLED4 sink registers
pub const WLED3_SINK_REG_SYNC: c_uint = 0x47;
pub const WLED3_SINK_REG_SYNC_CLEAR: c_uint = 0x00;
pub const WLED3_SINK_REG_CURR_SINK: c_uint = 0x4f;

pub const WLED3_SINK_REG_CURR_SINK_SHFT: c_int = 5;
// WLED3 specific per-'string' registers below

pub const WLED3_SINK_REG_STR_MOD_SRC_INT: c_uint = 0x00;
pub const WLED3_SINK_REG_STR_MOD_SRC_EXT: c_uint = 0x01;

// WLED4 specific control registers
pub const WLED4_CTRL_REG_SHORT_PROTECT: c_uint = 0x5e;

pub const WLED4_CTRL_REG_SEC_ACCESS: c_uint = 0xd0;
pub const WLED4_CTRL_REG_SEC_UNLOCK: c_uint = 0xa5;
pub const WLED4_CTRL_REG_TEST1: c_uint = 0xe2;
pub const WLED4_CTRL_REG_TEST1_EXT_FET_DTEST2: c_uint = 0x09;
// WLED4 specific sink registers
pub const WLED4_SINK_REG_CURR_SINK: c_uint = 0x46;

pub const WLED4_SINK_REG_CURR_SINK_SHFT: c_int = 4;
// WLED4 specific per-'string' registers below

pub const WLED4_SINK_REG_STR_MOD_SRC_INT: c_uint = 0x00;
pub const WLED4_SINK_REG_STR_MOD_SRC_EXT: c_uint = 0x01;

// WLED5 specific control registers
pub const WLED5_CTRL_REG_OVP_INT_CTL: c_uint = 0x5f;

// WLED5 specific sink registers
pub const WLED5_SINK_REG_MOD_A_EN: c_uint = 0x50;
pub const WLED5_SINK_REG_MOD_B_EN: c_uint = 0x60;

pub const WLED5_SINK_REG_MOD_A_SRC_SEL: c_uint = 0x51;
pub const WLED5_SINK_REG_MOD_B_SRC_SEL: c_uint = 0x61;
pub const WLED5_SINK_REG_MOD_SRC_SEL_HIGH: c_int = 0;
pub const WLED5_SINK_REG_MOD_SRC_SEL_EXT: c_uint = 0x03;

pub const WLED5_SINK_REG_MOD_A_BRIGHTNESS_WIDTH_SEL: c_uint = 0x52;
pub const WLED5_SINK_REG_MOD_B_BRIGHTNESS_WIDTH_SEL: c_uint = 0x62;
pub const WLED5_SINK_REG_BRIGHTNESS_WIDTH_12B: c_int = 0;
pub const WLED5_SINK_REG_BRIGHTNESS_WIDTH_15B: c_int = 1;
pub const WLED5_SINK_REG_MOD_A_BRIGHTNESS_LSB: c_uint = 0x53;
pub const WLED5_SINK_REG_MOD_A_BRIGHTNESS_MSB: c_uint = 0x54;
pub const WLED5_SINK_REG_MOD_B_BRIGHTNESS_LSB: c_uint = 0x63;
pub const WLED5_SINK_REG_MOD_B_BRIGHTNESS_MSB: c_uint = 0x64;
pub const WLED5_SINK_REG_MOD_SYNC_BIT: c_uint = 0x65;

// WLED5 specific per-'string' registers below

pub const WLED5_SINK_REG_SRC_SEL_MOD_A: c_int = 0;
pub const WLED5_SINK_REG_SRC_SEL_MOD_B: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wled_var_cfg {
    pub values: *const u32,
    pub (*fn)(u32): *mut u32,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wled_u32_opts {
    pub name: *const c_char,
    pub val_ptr: *mut u32,
    pub cfg: *const wled_var_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wled_bool_opts {
    pub name: *const c_char,
    pub val_ptr: *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wled_config {
    pub boost_i_limit: u32,
    pub ovp: u32,
    pub switch_freq: u32,
    pub num_strings: u32,
    pub string_i_limit: u32,
    pub enabled_strings: [u32; WLED_MAX_STRINGS],
    pub mod_sel: u32,
    pub cabc_sel: u32,
    pub cs_out_en: bool,
    pub ext_gen: bool,
    pub cabc: bool,
    pub external_pfet: bool,
    pub auto_detection_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wled {
    pub name: *const c_char,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub /: *mut *mut mutex lock; / Lock to avoid race from thread irq handler,
    pub last_short_event: ktime_t,
    pub start_ovp_fault_time: ktime_t,
    pub ctrl_addr: u16,
    pub sink_addr: u16,
    pub max_string_count: u16,
    pub auto_detection_ovp_count: u16,
    pub brightness: u32,
    pub max_brightness: u32,
    pub short_count: u32,
    pub auto_detect_count: u32,
    pub version: u32,
    pub disabled_by_short: bool,
    pub has_short_detect: bool,
    pub cabc_disabled: bool,
    pub short_irq: c_int,
    pub ovp_irq: c_int,
    pub cfg: wled_config,
    pub ovp_work: delayed_work,
// Configures the brightness. Applicable for wled3, wled4 and wled5
    pub brightness): *mut *mut *mut int (wled_set_brightness)(struct wled wled, u16,
// Configures the cabc register. Applicable for wled4 and wled5
    pub enable): *mut *mut *mut int (wled_cabc_config)(struct wled wled, bool,
//
// Toggles the sync bit for the brightness update to take place.
// Applicable for WLED3, WLED4 and WLED5.
//
    pub wled): *mut *mut int (wled_sync_toggle)(struct wled,
//
// Time to wait before checking the OVP status after wled module enable.
// Applicable for WLED4 and WLED5.
//
    pub wled): *mut *mut int (wled_ovp_delay)(struct wled,
//
// Determines if the auto string detection is required.
// Applicable for WLED4 and WLED5
//
    pub wled): *mut *mut bool (wled_auto_detection_required)(struct wled,
}

#[no_mangle]
unsafe extern "C" fn wled3_set_brightness(wled: *mut wled, brightness: u16) -> c_int {
    static int wled3_set_brightness(struct wled *wled, u16 brightness)
    {
    int rc, i;
    __le16 v;
    v = cpu_to_le16(brightness & WLED3_SINK_REG_BRIGHT_MAX);
    for (i = 0; i < wled.cfg.num_strings; ++i) {
    rc = regmap_bulk_write(wled.regmap, wled.ctrl_addr +
    WLED3_SINK_REG_BRIGHT(wled.cfg.enabled_strings[i]),
    &v, sizeof(v));
    if (rc < 0)
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wled4_set_brightness(wled: *mut wled, brightness: u16) -> c_int {
    static int wled4_set_brightness(struct wled *wled, u16 brightness)
    {
    int rc, i;
    let mut low_limit: u16 = wled.max_brightness * 4 / 1000;
    __le16 v;
// WLED4's lower limit of operation is 0.4%
    if (brightness > 0 && brightness < low_limit)
    brightness = low_limit;
    v = cpu_to_le16(brightness & WLED3_SINK_REG_BRIGHT_MAX);
    for (i = 0; i < wled.cfg.num_strings; ++i) {
    rc = regmap_bulk_write(wled.regmap, wled.sink_addr +
    WLED4_SINK_REG_BRIGHT(wled.cfg.enabled_strings[i]),
    &v, sizeof(v));
    if (rc < 0)
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wled5_set_brightness(wled: *mut wled, brightness: u16) -> c_int {
    static int wled5_set_brightness(struct wled *wled, u16 brightness)
    {
    int rc, offset;
    let mut low_limit: u16 = wled.max_brightness * 1 / 1000;
    __le16 v;
// WLED5's lower limit is 0.1%
    if (brightness < low_limit)
    brightness = low_limit;
    v = cpu_to_le16(brightness & WLED5_SINK_REG_BRIGHT_MAX_15B);
    offset = (wled.cfg.mod_sel == MOD_A) ?
    WLED5_SINK_REG_MOD_A_BRIGHTNESS_LSB :
    WLED5_SINK_REG_MOD_B_BRIGHTNESS_LSB;
    rc = regmap_bulk_write(wled.regmap, wled.sink_addr + offset,
    &v, sizeof(v));
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn wled_ovp_work(work: *mut work_struct) {
    static void wled_ovp_work(struct work_struct *work)
    {
    struct wled *wled = container_of(work,
    struct wled, ovp_work.work);
    enable_irq(wled.ovp_irq);
    }
#[no_mangle]
unsafe extern "C" fn wled_module_enable(wled: *mut wled, val: c_int) -> c_int {
    static int wled_module_enable(struct wled *wled, int val)
    {
    int rc;
    if (wled.disabled_by_short)
    return -ENXIO;
    rc = regmap_update_bits(wled.regmap, wled.ctrl_addr +
    WLED3_CTRL_REG_MOD_EN,
    WLED3_CTRL_REG_MOD_EN_MASK,
    val << WLED3_CTRL_REG_MOD_EN_SHIFT);
    if (rc < 0)
    return rc;
    if (wled.ovp_irq > 0) {
    if (val) {
//
// The hardware generates a storm of spurious OVP
// interrupts during soft start operations. So defer
// enabling the IRQ for 10ms to ensure that the
// soft start is complete.
//
    schedule_delayed_work(&wled.ovp_work, HZ / 100);
    } else {
    if (!cancel_delayed_work_sync(&wled.ovp_work))
    disable_irq(wled.ovp_irq);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wled3_sync_toggle(wled: *mut wled) -> c_int {
    static int wled3_sync_toggle(struct wled *wled)
    {
    int rc;
    let mut mask: c_uint = GENMASK(wled.max_string_count - 1, 0);
    rc = regmap_update_bits(wled.regmap,
    wled.sink_addr + WLED3_SINK_REG_SYNC,
    mask, WLED3_SINK_REG_SYNC_CLEAR);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.sink_addr + WLED3_SINK_REG_SYNC,
    mask, mask);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn wled5_mod_sync_toggle(wled: *mut wled) -> c_int {
    static int wled5_mod_sync_toggle(struct wled *wled)
    {
    int rc;
    u8 val;
    rc = regmap_update_bits(wled.regmap,
    wled.sink_addr + WLED5_SINK_REG_MOD_SYNC_BIT,
    WLED5_SINK_REG_SYNC_MASK, 0);
    if (rc < 0)
    return rc;
    val = (wled.cfg.mod_sel == MOD_A) ? WLED5_SINK_REG_SYNC_MOD_A_BIT :
    WLED5_SINK_REG_SYNC_MOD_B_BIT;
    return regmap_update_bits(wled.regmap,
    wled.sink_addr + WLED5_SINK_REG_MOD_SYNC_BIT,
    WLED5_SINK_REG_SYNC_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn wled_ovp_fault_status(wled: *mut wled, fault_set: *mut bool) -> c_int {
    static int wled_ovp_fault_status(struct wled *wled, bool *fault_set)
    {
    int rc;
    u32 int_rt_sts, fault_sts;
// fault_set = false;
    rc = regmap_read(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_INT_RT_STS,
    &int_rt_sts);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to read INT_RT_STS rc=%d\n", rc);
    return rc;
    }
    rc = regmap_read(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_FAULT_STATUS,
    &fault_sts);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to read FAULT_STATUS rc=%d\n", rc);
    return rc;
    }
    if (int_rt_sts & WLED3_CTRL_REG_OVP_FAULT_STATUS)
// fault_set = true;
    if (wled.version == 4 && (fault_sts & WLED3_CTRL_REG_OVP_FAULT_BIT))
// fault_set = true;
    if (wled.version == 5 && (fault_sts & (WLED3_CTRL_REG_OVP_FAULT_BIT |
    WLED5_CTRL_REG_OVP_PRE_ALARM_BIT)))
// fault_set = true;
    if (*fault_set)
    dev_dbg(wled.dev, "WLED OVP fault detected, int_rt_sts=0x%x fault_sts=0x%x\n",
    int_rt_sts, fault_sts);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn wled4_ovp_delay(wled: *mut wled) -> c_int {
    static int wled4_ovp_delay(struct wled *wled)
    {
    return WLED_SOFT_START_DLY_US;
    }
#[no_mangle]
unsafe extern "C" fn wled5_ovp_delay(wled: *mut wled) -> c_int {
    static int wled5_ovp_delay(struct wled *wled)
    {
    int rc, delay_us;
    u32 val;
    u8 ovp_timer_ms[8] = {1, 2, 4, 8, 12, 16, 20, 24};
// For WLED5, get the delay based on OVP timer
    rc = regmap_read(wled.regmap, wled.ctrl_addr +
    WLED5_CTRL_REG_OVP_INT_CTL, &val);
    if (rc < 0)
    delay_us =
    ovp_timer_ms[val & WLED5_CTRL_REG_OVP_INT_TIMER_MASK] * 1000;
    else
    delay_us = 2 * WLED_SOFT_START_DLY_US;
    dev_dbg(wled.dev, "delay_time_us: %d\n", delay_us);
    return delay_us;
    }
#[no_mangle]
unsafe extern "C" fn wled_update_status(bl: *mut backlight_device) -> c_int {
    static int wled_update_status(struct backlight_device *bl)
    {
    struct wled *wled = bl_get_data(bl);
    let mut brightness: u16 = backlight_get_brightness(bl);
    let mut rc: c_int = 0;
    mutex_lock(&wled.lock);
    if (brightness) {
    rc = wled.wled_set_brightness(wled, brightness);
    if (rc < 0) {
    dev_err(wled.dev, "wled failed to set brightness rc:%d\n",
    rc);
    goto unlock_mutex;
    }
    if (wled.version < 5) {
    rc = wled.wled_sync_toggle(wled);
    if (rc < 0) {
    dev_err(wled.dev, "wled sync failed rc:%d\n", rc);
    goto unlock_mutex;
    }
    } else {
//
// For WLED5 toggling the MOD_SYNC_BIT updates the
// brightness
//
    rc = wled5_mod_sync_toggle(wled);
    if (rc < 0) {
    dev_err(wled.dev, "wled mod sync failed rc:%d\n",
    rc);
    goto unlock_mutex;
    }
    }
    }
    if (!!brightness != !!wled.brightness) {
    rc = wled_module_enable(wled, !!brightness);
    if (rc < 0) {
    dev_err(wled.dev, "wled enable failed rc:%d\n", rc);
    goto unlock_mutex;
    }
    }
    wled.brightness = brightness;
    unlock_mutex:
    mutex_unlock(&wled.lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn wled4_cabc_config(wled: *mut wled, enable: bool) -> c_int {
    static int wled4_cabc_config(struct wled *wled, bool enable)
    {
    int i, j, rc;
    u8 val;
    for (i = 0; i < wled.cfg.num_strings; i++) {
    j = wled.cfg.enabled_strings[i];
    val = enable ? WLED4_SINK_REG_STR_CABC_MASK : 0;
    rc = regmap_update_bits(wled.regmap, wled.sink_addr +
    WLED4_SINK_REG_STR_CABC(j),
    WLED4_SINK_REG_STR_CABC_MASK, val);
    if (rc < 0)
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wled5_cabc_config(wled: *mut wled, enable: bool) -> c_int {
    static int wled5_cabc_config(struct wled *wled, bool enable)
    {
    int rc, offset;
    u8 reg;
    if (wled.cabc_disabled)
    return 0;
    reg = enable ? wled.cfg.cabc_sel : 0;
    offset = (wled.cfg.mod_sel == MOD_A) ? WLED5_SINK_REG_MOD_A_SRC_SEL :
    WLED5_SINK_REG_MOD_B_SRC_SEL;
    rc = regmap_update_bits(wled.regmap, wled.sink_addr + offset,
    WLED5_SINK_REG_MOD_SRC_SEL_MASK, reg);
    if (rc < 0) {
    pr_err("Error in configuring CABC rc=%d\n", rc);
    return rc;
    }
    if (!wled.cfg.cabc_sel)
    wled.cabc_disabled = true;
    return 0;
    }
pub const WLED_SHORT_DLY_MS: c_int = 20;
pub const WLED_SHORT_CNT_MAX: c_int = 5;

#[no_mangle]
unsafe extern "C" fn wled_short_irq_handler(irq: c_int, _wled: *mut c_void) -> irqreturn_t {
    static irqreturn_t wled_short_irq_handler(int irq, void *_wled)
    {
    struct wled *wled = _wled;
    int rc;
    s64 elapsed_time;
    wled.short_count++;
    mutex_lock(&wled.lock);
    rc = wled_module_enable(wled, false);
    if (rc < 0) {
    dev_err(wled.dev, "wled disable failed rc:%d\n", rc);
    goto unlock_mutex;
    }
    elapsed_time = ktime_us_delta(ktime_get(),
    wled.last_short_event);
    if (elapsed_time > WLED_SHORT_RESET_CNT_DLY_US)
    wled.short_count = 1;
    if (wled.short_count > WLED_SHORT_CNT_MAX) {
    dev_err(wled.dev, "Short triggered %d times, disabling WLED forever!\n",
    wled.short_count);
    wled.disabled_by_short = true;
    goto unlock_mutex;
    }
    wled.last_short_event = ktime_get();
    msleep(WLED_SHORT_DLY_MS);
    rc = wled_module_enable(wled, true);
    if (rc < 0)
    dev_err(wled.dev, "wled enable failed rc:%d\n", rc);
    unlock_mutex:
    mutex_unlock(&wled.lock);
    return IRQ_HANDLED;
    }
pub const AUTO_DETECT_BRIGHTNESS: c_int = 200;
#[no_mangle]
unsafe extern "C" fn wled_auto_string_detection(wled: *mut wled) {
    static void wled_auto_string_detection(struct wled *wled)
    {
    let mut rc: c_int = 0, i, j, delay_time_us;
    let mut sink_config: u32 = 0;
    let mut sink_test: u8 = 0, sink_valid = 0, val;
    bool fault_set;
// Read configured sink configuration
    rc = regmap_read(wled.regmap, wled.sink_addr +
    WLED4_SINK_REG_CURR_SINK, &sink_config);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to read SINK configuration rc=%d\n",
    rc);
    goto failed_detect;
    }
// Disable the module before starting detection
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_MOD_EN,
    WLED3_CTRL_REG_MOD_EN_MASK, 0);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to disable WLED module rc=%d\n", rc);
    goto failed_detect;
    }
// Set low brightness across all sinks
    rc = wled4_set_brightness(wled, AUTO_DETECT_BRIGHTNESS);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to set brightness for auto detection rc=%d\n",
    rc);
    goto failed_detect;
    }
    if (wled.cfg.cabc) {
    rc = wled.wled_cabc_config(wled, false);
    if (rc < 0)
    goto failed_detect;
    }
// Disable all sinks
    rc = regmap_write(wled.regmap,
    wled.sink_addr + WLED4_SINK_REG_CURR_SINK, 0);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to disable all sinks rc=%d\n", rc);
    goto failed_detect;
    }
// Iterate through the strings one by one
    for (i = 0; i < wled.cfg.num_strings; i++) {
    j = wled.cfg.enabled_strings[i];
    sink_test = BIT((WLED4_SINK_REG_CURR_SINK_SHFT + j));
// Enable feedback control
    rc = regmap_write(wled.regmap, wled.ctrl_addr +
    WLED3_CTRL_REG_FEEDBACK_CONTROL, j + 1);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to enable feedback for SINK %d rc = %d\n",
    j + 1, rc);
    goto failed_detect;
    }
// Enable the sink
    rc = regmap_write(wled.regmap, wled.sink_addr +
    WLED4_SINK_REG_CURR_SINK, sink_test);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to configure SINK %d rc=%d\n",
    j + 1, rc);
    goto failed_detect;
    }
// Enable the module
    rc = regmap_update_bits(wled.regmap, wled.ctrl_addr +
    WLED3_CTRL_REG_MOD_EN,
    WLED3_CTRL_REG_MOD_EN_MASK,
    WLED3_CTRL_REG_MOD_EN_MASK);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to enable WLED module rc=%d\n",
    rc);
    goto failed_detect;
    }
    delay_time_us = wled.wled_ovp_delay(wled);
    usleep_range(delay_time_us, delay_time_us + 1000);
    rc = wled_ovp_fault_status(wled, &fault_set);
    if (rc < 0) {
    dev_err(wled.dev, "Error in getting OVP fault_sts, rc=%d\n",
    rc);
    goto failed_detect;
    }
    if (fault_set)
    dev_dbg(wled.dev, "WLED OVP fault detected with SINK %d\n",
    j + 1);
    else
    sink_valid |= sink_test;
// Disable the module
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_MOD_EN,
    WLED3_CTRL_REG_MOD_EN_MASK, 0);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to disable WLED module rc=%d\n",
    rc);
    goto failed_detect;
    }
    }
    if (!sink_valid) {
    dev_err(wled.dev, "No valid WLED sinks found\n");
    wled.disabled_by_short = true;
    goto failed_detect;
    }
    if (sink_valid != sink_config) {
    dev_warn(wled.dev, "%x is not a valid sink configuration - using %x instead\n",
    sink_config, sink_valid);
    sink_config = sink_valid;
    }
// Write the new sink configuration
    rc = regmap_write(wled.regmap,
    wled.sink_addr + WLED4_SINK_REG_CURR_SINK,
    sink_config);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to reconfigure the default sink rc=%d\n",
    rc);
    goto failed_detect;
    }
// Enable valid sinks
    if (wled.version == 4) {
    for (i = 0; i < wled.cfg.num_strings; i++) {
    j = wled.cfg.enabled_strings[i];
    if (sink_config &
    BIT(WLED4_SINK_REG_CURR_SINK_SHFT + j))
    val = WLED4_SINK_REG_STR_MOD_MASK;
    else
// Disable modulator_en for unused sink
    val = 0;
    rc = regmap_write(wled.regmap, wled.sink_addr +
    WLED4_SINK_REG_STR_MOD_EN(j), val);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to configure MODULATOR_EN rc=%d\n",
    rc);
    goto failed_detect;
    }
    }
    }
// Enable CABC
    rc = wled.wled_cabc_config(wled, true);
    if (rc < 0)
    goto failed_detect;
// Restore the feedback setting
    rc = regmap_write(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_FEEDBACK_CONTROL, 0);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to restore feedback setting rc=%d\n",
    rc);
    goto failed_detect;
    }
// Restore brightness
    rc = wled4_set_brightness(wled, wled.brightness);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to set brightness after auto detection rc=%d\n",
    rc);
    goto failed_detect;
    }
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_MOD_EN,
    WLED3_CTRL_REG_MOD_EN_MASK,
    WLED3_CTRL_REG_MOD_EN_MASK);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to enable WLED module rc=%d\n", rc);
    goto failed_detect;
    }
    failed_detect:
    return;
    }
pub const WLED_AUTO_DETECT_OVP_COUNT: c_int = 5;

#[no_mangle]
unsafe extern "C" fn wled4_auto_detection_required(wled: *mut wled) -> bool {
    static bool wled4_auto_detection_required(struct wled *wled)
    {
    s64 elapsed_time_us;
    if (!wled.cfg.auto_detection_enabled)
    return false;
//
// Check if the OVP fault was an occasional one
// or if it's firing continuously, the latter qualifies
// for an auto-detection check.
//
    if (!wled.auto_detection_ovp_count) {
    wled.start_ovp_fault_time = ktime_get();
    wled.auto_detection_ovp_count++;
    } else {
    elapsed_time_us = ktime_us_delta(ktime_get(),
    wled.start_ovp_fault_time);
    if (elapsed_time_us > WLED_AUTO_DETECT_CNT_DLY_US)
    wled.auto_detection_ovp_count = 0;
    else
    wled.auto_detection_ovp_count++;
    if (wled.auto_detection_ovp_count >=
    WLED_AUTO_DETECT_OVP_COUNT) {
    wled.auto_detection_ovp_count = 0;
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn wled5_auto_detection_required(wled: *mut wled) -> bool {
    static bool wled5_auto_detection_required(struct wled *wled)
    {
    if (!wled.cfg.auto_detection_enabled)
    return false;
//
// Unlike WLED4, WLED5 has OVP fault density interrupt configuration
// i.e. to count the number of OVP alarms for a certain duration before
// triggering OVP fault interrupt. By default, number of OVP fault
// events counted before an interrupt is fired is 32 and the time
// interval is 12 ms. If we see one OVP fault interrupt, then that
// should qualify for a real OVP fault condition to run auto detection
// algorithm.
//
    return true;
    }
#[no_mangle]
unsafe extern "C" fn wled_auto_detection_at_init(wled: *mut wled) -> c_int {
    static int wled_auto_detection_at_init(struct wled *wled)
    {
    int rc;
    bool fault_set;
    if (!wled.cfg.auto_detection_enabled)
    return 0;
    rc = wled_ovp_fault_status(wled, &fault_set);
    if (rc < 0) {
    dev_err(wled.dev, "Error in getting OVP fault_sts, rc=%d\n",
    rc);
    return rc;
    }
    if (fault_set) {
    mutex_lock(&wled.lock);
    wled_auto_string_detection(wled);
    mutex_unlock(&wled.lock);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn wled_ovp_irq_handler(irq: c_int, _wled: *mut c_void) -> irqreturn_t {
    static irqreturn_t wled_ovp_irq_handler(int irq, void *_wled)
    {
    struct wled *wled = _wled;
    int rc;
    u32 int_sts, fault_sts;
    rc = regmap_read(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_INT_RT_STS, &int_sts);
    if (rc < 0) {
    dev_err(wled.dev, "Error in reading WLED3_INT_RT_STS rc=%d\n",
    rc);
    return IRQ_HANDLED;
    }
    rc = regmap_read(wled.regmap, wled.ctrl_addr +
    WLED3_CTRL_REG_FAULT_STATUS, &fault_sts);
    if (rc < 0) {
    dev_err(wled.dev, "Error in reading WLED_FAULT_STATUS rc=%d\n",
    rc);
    return IRQ_HANDLED;
    }
    if (fault_sts & (WLED3_CTRL_REG_OVP_FAULT_BIT |
    WLED3_CTRL_REG_ILIM_FAULT_BIT))
    dev_dbg(wled.dev, "WLED OVP fault detected, int_sts=%x fault_sts= %x\n",
    int_sts, fault_sts);
    if (fault_sts & WLED3_CTRL_REG_OVP_FAULT_BIT) {
    if (wled.wled_auto_detection_required(wled)) {
    mutex_lock(&wled.lock);
    wled_auto_string_detection(wled);
    mutex_unlock(&wled.lock);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wled3_setup(wled: *mut wled) -> c_int {
    static int wled3_setup(struct wled *wled)
    {
    u16 addr;
    let mut sink_en: u8 = 0;
    int rc, i, j;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_OVP,
    WLED3_CTRL_REG_OVP_MASK, wled.cfg.ovp);
    if (rc)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_ILIMIT,
    WLED3_CTRL_REG_ILIMIT_MASK,
    wled.cfg.boost_i_limit);
    if (rc)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_FREQ,
    WLED3_CTRL_REG_FREQ_MASK,
    wled.cfg.switch_freq);
    if (rc)
    return rc;
    for (i = 0; i < wled.cfg.num_strings; ++i) {
    j = wled.cfg.enabled_strings[i];
    addr = wled.ctrl_addr + WLED3_SINK_REG_STR_MOD_EN(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED3_SINK_REG_STR_MOD_MASK,
    WLED3_SINK_REG_STR_MOD_MASK);
    if (rc)
    return rc;
    if (wled.cfg.ext_gen) {
    addr = wled.ctrl_addr + WLED3_SINK_REG_STR_MOD_SRC(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED3_SINK_REG_STR_MOD_SRC_MASK,
    WLED3_SINK_REG_STR_MOD_SRC_EXT);
    if (rc)
    return rc;
    }
    addr = wled.ctrl_addr + WLED3_SINK_REG_STR_FULL_SCALE_CURR(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED3_SINK_REG_STR_FULL_SCALE_CURR_MASK,
    wled.cfg.string_i_limit);
    if (rc)
    return rc;
    addr = wled.ctrl_addr + WLED3_SINK_REG_STR_CABC(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED3_SINK_REG_STR_CABC_MASK,
    wled.cfg.cabc ?
    WLED3_SINK_REG_STR_CABC_MASK : 0);
    if (rc)
    return rc;
    sink_en |= BIT(j + WLED3_SINK_REG_CURR_SINK_SHFT);
    }
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_SINK_REG_CURR_SINK,
    WLED3_SINK_REG_CURR_SINK_MASK, sink_en);
    if (rc)
    return rc;
    return 0;
    }
    static const struct wled_config wled3_config_defaults = {
    .boost_i_limit = 3,
    .string_i_limit = 20,
    .ovp = 2,
    .num_strings = 3,
    .switch_freq = 5,
    .cs_out_en = false,
    .ext_gen = false,
    .cabc = false,
    .enabled_strings = {0, 1, 2},
    };
#[no_mangle]
unsafe extern "C" fn wled4_setup(wled: *mut wled) -> c_int {
    static int wled4_setup(struct wled *wled)
    {
    int rc, temp, i, j;
    u16 addr;
    let mut sink_en: u8 = 0;
    u32 sink_cfg;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_OVP,
    WLED3_CTRL_REG_OVP_MASK, wled.cfg.ovp);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_ILIMIT,
    WLED3_CTRL_REG_ILIMIT_MASK,
    wled.cfg.boost_i_limit);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_FREQ,
    WLED3_CTRL_REG_FREQ_MASK,
    wled.cfg.switch_freq);
    if (rc < 0)
    return rc;
    if (wled.cfg.external_pfet) {
// Unlock the secure register access
    rc = regmap_write(wled.regmap, wled.ctrl_addr +
    WLED4_CTRL_REG_SEC_ACCESS,
    WLED4_CTRL_REG_SEC_UNLOCK);
    if (rc < 0)
    return rc;
    rc = regmap_write(wled.regmap,
    wled.ctrl_addr + WLED4_CTRL_REG_TEST1,
    WLED4_CTRL_REG_TEST1_EXT_FET_DTEST2);
    if (rc < 0)
    return rc;
    }
    rc = regmap_read(wled.regmap, wled.sink_addr +
    WLED4_SINK_REG_CURR_SINK, &sink_cfg);
    if (rc < 0)
    return rc;
    for (i = 0; i < wled.cfg.num_strings; i++) {
    j = wled.cfg.enabled_strings[i];
    temp = j + WLED4_SINK_REG_CURR_SINK_SHFT;
    sink_en |= 1 << temp;
    }
    if (sink_cfg == sink_en) {
    rc = wled_auto_detection_at_init(wled);
    return rc;
    }
    rc = regmap_update_bits(wled.regmap,
    wled.sink_addr + WLED4_SINK_REG_CURR_SINK,
    WLED4_SINK_REG_CURR_SINK_MASK, 0);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap, wled.ctrl_addr +
    WLED3_CTRL_REG_MOD_EN,
    WLED3_CTRL_REG_MOD_EN_MASK, 0);
    if (rc < 0)
    return rc;
// Per sink/string configuration
    for (i = 0; i < wled.cfg.num_strings; i++) {
    j = wled.cfg.enabled_strings[i];
    addr = wled.sink_addr +
    WLED4_SINK_REG_STR_MOD_EN(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED4_SINK_REG_STR_MOD_MASK,
    WLED4_SINK_REG_STR_MOD_MASK);
    if (rc < 0)
    return rc;
    addr = wled.sink_addr +
    WLED4_SINK_REG_STR_FULL_SCALE_CURR(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED4_SINK_REG_STR_FULL_SCALE_CURR_MASK,
    wled.cfg.string_i_limit);
    if (rc < 0)
    return rc;
    }
    rc = wled4_cabc_config(wled, wled.cfg.cabc);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap, wled.ctrl_addr +
    WLED3_CTRL_REG_MOD_EN,
    WLED3_CTRL_REG_MOD_EN_MASK,
    WLED3_CTRL_REG_MOD_EN_MASK);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.sink_addr + WLED4_SINK_REG_CURR_SINK,
    WLED4_SINK_REG_CURR_SINK_MASK, sink_en);
    if (rc < 0)
    return rc;
    rc = wled.wled_sync_toggle(wled);
    if (rc < 0) {
    dev_err(wled.dev, "Failed to toggle sync reg rc:%d\n", rc);
    return rc;
    }
    rc = wled_auto_detection_at_init(wled);
    return rc;
    }
    static const struct wled_config wled4_config_defaults = {
    .boost_i_limit = 4,
    .string_i_limit = 10,
    .ovp = 1,
    .num_strings = 4,
    .switch_freq = 11,
    .cabc = false,
    .external_pfet = false,
    .auto_detection_enabled = false,
    .enabled_strings = {0, 1, 2, 3},
    };
#[no_mangle]
unsafe extern "C" fn wled5_setup(wled: *mut wled) -> c_int {
    static int wled5_setup(struct wled *wled)
    {
    int rc, temp, i, j, offset;
    let mut sink_en: u8 = 0;
    u16 addr;
    u32 val;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_OVP,
    WLED5_CTRL_REG_OVP_MASK, wled.cfg.ovp);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_ILIMIT,
    WLED3_CTRL_REG_ILIMIT_MASK,
    wled.cfg.boost_i_limit);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.ctrl_addr + WLED3_CTRL_REG_FREQ,
    WLED3_CTRL_REG_FREQ_MASK,
    wled.cfg.switch_freq);
    if (rc < 0)
    return rc;
// Per sink/string configuration
    for (i = 0; i < wled.cfg.num_strings; ++i) {
    j = wled.cfg.enabled_strings[i];
    addr = wled.sink_addr +
    WLED4_SINK_REG_STR_FULL_SCALE_CURR(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED4_SINK_REG_STR_FULL_SCALE_CURR_MASK,
    wled.cfg.string_i_limit);
    if (rc < 0)
    return rc;
    addr = wled.sink_addr + WLED5_SINK_REG_STR_SRC_SEL(j);
    rc = regmap_update_bits(wled.regmap, addr,
    WLED5_SINK_REG_SRC_SEL_MASK,
    wled.cfg.mod_sel == MOD_A ?
    WLED5_SINK_REG_SRC_SEL_MOD_A :
    WLED5_SINK_REG_SRC_SEL_MOD_B);
    temp = j + WLED4_SINK_REG_CURR_SINK_SHFT;
    sink_en |= 1 << temp;
    }
    rc = wled5_cabc_config(wled, wled.cfg.cabc_sel ? true : false);
    if (rc < 0)
    return rc;
// Enable one of the modulators A or B based on mod_sel
    addr = wled.sink_addr + WLED5_SINK_REG_MOD_A_EN;
    val = (wled.cfg.mod_sel == MOD_A) ? WLED5_SINK_REG_MOD_EN_MASK : 0;
    rc = regmap_update_bits(wled.regmap, addr,
    WLED5_SINK_REG_MOD_EN_MASK, val);
    if (rc < 0)
    return rc;
    addr = wled.sink_addr + WLED5_SINK_REG_MOD_B_EN;
    val = (wled.cfg.mod_sel == MOD_B) ? WLED5_SINK_REG_MOD_EN_MASK : 0;
    rc = regmap_update_bits(wled.regmap, addr,
    WLED5_SINK_REG_MOD_EN_MASK, val);
    if (rc < 0)
    return rc;
    offset = (wled.cfg.mod_sel == MOD_A) ?
    WLED5_SINK_REG_MOD_A_BRIGHTNESS_WIDTH_SEL :
    WLED5_SINK_REG_MOD_B_BRIGHTNESS_WIDTH_SEL;
    addr = wled.sink_addr + offset;
    val = (wled.max_brightness == WLED5_SINK_REG_BRIGHT_MAX_15B) ?
    WLED5_SINK_REG_BRIGHTNESS_WIDTH_15B :
    WLED5_SINK_REG_BRIGHTNESS_WIDTH_12B;
    rc = regmap_write(wled.regmap, addr, val);
    if (rc < 0)
    return rc;
    rc = regmap_update_bits(wled.regmap,
    wled.sink_addr + WLED4_SINK_REG_CURR_SINK,
    WLED4_SINK_REG_CURR_SINK_MASK, sink_en);
    if (rc < 0)
    return rc;
// This updates only FSC configuration in WLED5
    rc = wled.wled_sync_toggle(wled);
    if (rc < 0) {
    pr_err("Failed to toggle sync reg rc:%d\n", rc);
    return rc;
    }
    rc = wled_auto_detection_at_init(wled);
    if (rc < 0)
    return rc;
    return 0;
    }
    static const struct wled_config wled5_config_defaults = {
    .boost_i_limit = 5,
    .string_i_limit = 10,
    .ovp = 4,
    .num_strings = 4,
    .switch_freq = 11,
    .mod_sel = 0,
    .cabc_sel = 0,
    .cabc = false,
    .external_pfet = false,
    .auto_detection_enabled = false,
    .enabled_strings = {0, 1, 2, 3},
    };
    static const u32 wled3_boost_i_limit_values[] = {
    105, 385, 525, 805, 980, 1260, 1400, 1680,
    };
    static const struct wled_var_cfg wled3_boost_i_limit_cfg = {
    .values = wled3_boost_i_limit_values,
    .size = ARRAY_SIZE(wled3_boost_i_limit_values),
    };
    static const u32 wled4_boost_i_limit_values[] = {
    105, 280, 450, 620, 970, 1150, 1300, 1500,
    };
    static const struct wled_var_cfg wled4_boost_i_limit_cfg = {
    .values = wled4_boost_i_limit_values,
    .size = ARRAY_SIZE(wled4_boost_i_limit_values),
    };
#[no_mangle]
pub unsafe extern "C" fn wled5_boost_i_limit_values_fn(idx: u32) -> u32 {
    static inline u32 wled5_boost_i_limit_values_fn(u32 idx)
    {
    return 525 + (idx * 175);
    }
    static const struct wled_var_cfg wled5_boost_i_limit_cfg = {
    .fn = wled5_boost_i_limit_values_fn,
    .size = 8,
    };
    static const u32 wled3_ovp_values[] = {
    35, 32, 29, 27,
    };
    static const struct wled_var_cfg wled3_ovp_cfg = {
    .values = wled3_ovp_values,
    .size = ARRAY_SIZE(wled3_ovp_values),
    };
    static const u32 wled4_ovp_values[] = {
    31100, 29600, 19600, 18100,
    };
    static const struct wled_var_cfg wled4_ovp_cfg = {
    .values = wled4_ovp_values,
    .size = ARRAY_SIZE(wled4_ovp_values),
    };
    static const u32 pmi8994_wled_ovp_values[] = {
    31000, 29500, 19400, 17800,
    };
    static const struct wled_var_cfg pmi8994_wled_ovp_cfg = {
    .values = pmi8994_wled_ovp_values,
    .size = ARRAY_SIZE(pmi8994_wled_ovp_values),
    };
#[no_mangle]
pub unsafe extern "C" fn wled5_ovp_values_fn(idx: u32) -> u32 {
    static inline u32 wled5_ovp_values_fn(u32 idx)
    {
//
// 0000 - 38.5 V
// 0001 - 37 V ..
// 1111 - 16 V
//
    return 38500 - (idx * 1500);
    }
    static const struct wled_var_cfg wled5_ovp_cfg = {
    .fn = wled5_ovp_values_fn,
    .size = 16,
    };
#[no_mangle]
unsafe extern "C" fn wled3_switch_freq_values_fn(idx: u32) -> u32 {
    static u32 wled3_switch_freq_values_fn(u32 idx)
    {
    return 19200 / (2 * (1 + idx));
    }
    static const struct wled_var_cfg wled3_switch_freq_cfg = {
    .fn = wled3_switch_freq_values_fn,
    .size = 16,
    };
    static const struct wled_var_cfg wled3_string_i_limit_cfg = {
    .size = 26,
    };
    static const u32 wled4_string_i_limit_values[] = {
    0, 2500, 5000, 7500, 10000, 12500, 15000, 17500, 20000,
    22500, 25000, 27500, 30000,
    };
    static const struct wled_var_cfg wled4_string_i_limit_cfg = {
    .values = wled4_string_i_limit_values,
    .size = ARRAY_SIZE(wled4_string_i_limit_values),
    };
    static const struct wled_var_cfg wled5_mod_sel_cfg = {
    .size = 2,
    };
    static const struct wled_var_cfg wled5_cabc_sel_cfg = {
    .size = 4,
    };
#[no_mangle]
unsafe extern "C" fn wled_values(cfg: *const wled_var_cfg, idx: u32) -> u32 {
    static u32 wled_values(const struct wled_var_cfg *cfg, u32 idx)
    {
    if (idx >= cfg.size)
    return UINT_MAX;
    if (cfg.fn)
    return cfg.fn(idx);
    if (cfg.values)
    return cfg.values[idx];
    return idx;
    }
#[no_mangle]
unsafe extern "C" fn wled_configure(wled: *mut wled) -> c_int {
    static int wled_configure(struct wled *wled)
    {
    struct wled_config *cfg = &wled.cfg;
    struct device *dev = wled.dev;
    const __be32 *prop_addr;
    u32 size, val, c;
    int rc, i, j, string_len;
    const struct wled_u32_opts *u32_opts = core::ptr::null_mut();
    const struct wled_u32_opts wled3_opts[] = {
    {
    .name = "qcom,current-boost-limit",
    .val_ptr = &cfg.boost_i_limit,
    .cfg = &wled3_boost_i_limit_cfg,
    },
    {
    .name = "qcom,current-limit",
    .val_ptr = &cfg.string_i_limit,
    .cfg = &wled3_string_i_limit_cfg,
    },
    {
    .name = "qcom,ovp",
    .val_ptr = &cfg.ovp,
    .cfg = &wled3_ovp_cfg,
    },
    {
    .name = "qcom,switching-freq",
    .val_ptr = &cfg.switch_freq,
    .cfg = &wled3_switch_freq_cfg,
    },
    };
    const struct wled_u32_opts wled4_opts[] = {
    {
    .name = "qcom,current-boost-limit",
    .val_ptr = &cfg.boost_i_limit,
    .cfg = &wled4_boost_i_limit_cfg,
    },
    {
    .name = "qcom,current-limit-microamp",
    .val_ptr = &cfg.string_i_limit,
    .cfg = &wled4_string_i_limit_cfg,
    },
    {
    .name = "qcom,ovp-millivolt",
    .val_ptr = &cfg.ovp,
    .cfg = &wled4_ovp_cfg,
    },
    {
    .name = "qcom,switching-freq",
    .val_ptr = &cfg.switch_freq,
    .cfg = &wled3_switch_freq_cfg,
    },
    };
    const struct wled_u32_opts pmi8994_wled_opts[] = {
    {
    .name = "qcom,current-boost-limit",
    .val_ptr = &cfg.boost_i_limit,
    .cfg = &wled4_boost_i_limit_cfg,
    },
    {
    .name = "qcom,current-limit-microamp",
    .val_ptr = &cfg.string_i_limit,
    .cfg = &wled4_string_i_limit_cfg,
    },
    {
    .name = "qcom,ovp-millivolt",
    .val_ptr = &cfg.ovp,
    .cfg = &pmi8994_wled_ovp_cfg,
    },
    {
    .name = "qcom,switching-freq",
    .val_ptr = &cfg.switch_freq,
    .cfg = &wled3_switch_freq_cfg,
    },
    };
    const struct wled_u32_opts wled5_opts[] = {
    {
    .name = "qcom,current-boost-limit",
    .val_ptr = &cfg.boost_i_limit,
    .cfg = &wled5_boost_i_limit_cfg,
    },
    {
    .name = "qcom,current-limit-microamp",
    .val_ptr = &cfg.string_i_limit,
    .cfg = &wled4_string_i_limit_cfg,
    },
    {
    .name = "qcom,ovp-millivolt",
    .val_ptr = &cfg.ovp,
    .cfg = &wled5_ovp_cfg,
    },
    {
    .name = "qcom,switching-freq",
    .val_ptr = &cfg.switch_freq,
    .cfg = &wled3_switch_freq_cfg,
    },
    {
    .name = "qcom,modulator-sel",
    .val_ptr = &cfg.mod_sel,
    .cfg = &wled5_mod_sel_cfg,
    },
    {
    .name = "qcom,cabc-sel",
    .val_ptr = &cfg.cabc_sel,
    .cfg = &wled5_cabc_sel_cfg,
    },
    };
    const struct wled_bool_opts bool_opts[] = {
    { "qcom,cs-out", &cfg.cs_out_en, },
    { "qcom,ext-gen", &cfg.ext_gen, },
    { "qcom,cabc", &cfg.cabc, },
    { "qcom,external-pfet", &cfg.external_pfet, },
    { "qcom,auto-string-detection", &cfg.auto_detection_enabled, },
    };
    prop_addr = of_get_address(dev.of_node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (!prop_addr) {
    dev_err(wled.dev, "invalid IO resources\n");
    return -EINVAL;
    }
    wled.ctrl_addr = be32_to_cpu(*prop_addr);
    rc = of_property_read_string(dev.of_node, "label", &wled.name);
    if (rc) {
    wled.name = devm_kasprintf(dev, GFP_KERNEL, "%pOFn", dev.of_node);
    if (!wled.name)
    return -ENOMEM;
    }
    switch (wled.version) {
    case 3:
    u32_opts = wled3_opts;
    size = ARRAY_SIZE(wled3_opts);
// cfg = wled3_config_defaults;
    wled.wled_set_brightness = wled3_set_brightness;
    wled.wled_sync_toggle = wled3_sync_toggle;
    wled.max_string_count = 3;
    wled.sink_addr = wled.ctrl_addr;
    break;
    case 4:
    if (of_device_is_compatible(dev.of_node, "qcom,pmi8950-wled") ||
    of_device_is_compatible(dev.of_node, "qcom,pmi8994-wled")) {
    u32_opts = pmi8994_wled_opts;
    size = ARRAY_SIZE(pmi8994_wled_opts);
    } else {
    u32_opts = wled4_opts;
    size = ARRAY_SIZE(wled4_opts);
    }
// cfg = wled4_config_defaults;
    wled.wled_set_brightness = wled4_set_brightness;
    wled.wled_sync_toggle = wled3_sync_toggle;
    wled.wled_cabc_config = wled4_cabc_config;
    wled.wled_ovp_delay = wled4_ovp_delay;
    wled.wled_auto_detection_required =
    wled4_auto_detection_required;
    wled.max_string_count = 4;
    prop_addr = of_get_address(dev.of_node, 1, core::ptr::null_mut(), core::ptr::null_mut());
    if (!prop_addr) {
    dev_err(wled.dev, "invalid IO resources\n");
    return -EINVAL;
    }
    wled.sink_addr = be32_to_cpu(*prop_addr);
    break;
    case 5:
    u32_opts = wled5_opts;
    size = ARRAY_SIZE(wled5_opts);
// cfg = wled5_config_defaults;
    wled.wled_set_brightness = wled5_set_brightness;
    wled.wled_sync_toggle = wled3_sync_toggle;
    wled.wled_cabc_config = wled5_cabc_config;
    wled.wled_ovp_delay = wled5_ovp_delay;
    wled.wled_auto_detection_required =
    wled5_auto_detection_required;
    wled.max_string_count = 4;
    prop_addr = of_get_address(dev.of_node, 1, core::ptr::null_mut(), core::ptr::null_mut());
    if (!prop_addr) {
    dev_err(wled.dev, "invalid IO resources\n");
    return -EINVAL;
    }
    wled.sink_addr = be32_to_cpu(*prop_addr);
    break;
    default:
    dev_err(wled.dev, "Invalid WLED version\n");
    return -EINVAL;
    }
    for (i = 0; i < size; ++i) {
    rc = of_property_read_u32(dev.of_node, u32_opts[i].name, &val);
    if (rc == -EINVAL) {
    continue;
    } else if (rc) {
    dev_err(dev, "error reading '%s'\n", u32_opts[i].name);
    return rc;
    }
    c = UINT_MAX;
    for (j = 0; c != val; j++) {
    c = wled_values(u32_opts[i].cfg, j);
    if (c == UINT_MAX) {
    dev_err(dev, "invalid value for '%s'\n",
    u32_opts[i].name);
    return -EINVAL;
    }
    if (c == val)
    break;
    }
    dev_dbg(dev, "'%s' = %u\n", u32_opts[i].name, c);
// u32_opts[i].val_ptr = j;
    }
    for (i = 0; i < ARRAY_SIZE(bool_opts); ++i) {
    if (of_property_read_bool(dev.of_node, bool_opts[i].name))
// bool_opts[i].val_ptr = true;
    }
    string_len = of_property_count_elems_of_size(dev.of_node,
    "qcom,enabled-strings",
    sizeof(u32));
    if (string_len > 0) {
    if (string_len > wled.max_string_count) {
    dev_err(dev, "Cannot have more than %d strings\n",
    wled.max_string_count);
    return -EINVAL;
    }
    rc = of_property_read_u32_array(dev.of_node,
    "qcom,enabled-strings",
    wled.cfg.enabled_strings,
    string_len);
    if (rc) {
    dev_err(dev, "Failed to read %d elements from qcom,enabled-strings: %d\n",
    string_len, rc);
    return rc;
    }
    for (i = 0; i < string_len; ++i) {
    if (wled.cfg.enabled_strings[i] >= wled.max_string_count) {
    dev_err(dev,
    "qcom,enabled-strings index %d at %d is out of bounds\n",
    wled.cfg.enabled_strings[i], i);
    return -EINVAL;
    }
    }
    cfg.num_strings = string_len;
    }
    rc = of_property_read_u32(dev.of_node, "qcom,num-strings", &val);
    if (!rc) {
    if (val < 1 || val > wled.max_string_count) {
    dev_err(dev, "qcom,num-strings must be between 1 and %d\n",
    wled.max_string_count);
    return -EINVAL;
    }
    if (string_len > 0) {
    dev_warn(dev, "Only one of qcom,num-strings or qcom,enabled-strings"
    " should be set\n");
    if (val > string_len) {
    dev_err(dev, "qcom,num-strings exceeds qcom,enabled-strings\n");
    return -EINVAL;
    }
    }
    cfg.num_strings = val;
    }
    return 0;
    }
    static int wled_configure_short_irq(struct wled *wled,
    struct platform_device *pdev)
    {
    int rc;
    if (!wled.has_short_detect)
    return 0;
    rc = regmap_update_bits(wled.regmap, wled.ctrl_addr +
    WLED4_CTRL_REG_SHORT_PROTECT,
    WLED4_CTRL_REG_SHORT_EN_MASK,
    WLED4_CTRL_REG_SHORT_EN_MASK);
    if (rc < 0)
    return rc;
    wled.short_irq = platform_get_irq_byname(pdev, "short");
    if (wled.short_irq < 0) {
    dev_dbg(&pdev.dev, "short irq is not used\n");
    return 0;
    }
    rc = devm_request_threaded_irq(wled.dev, wled.short_irq,
    core::ptr::null_mut(), wled_short_irq_handler,
    IRQF_ONESHOT,
    "wled_short_irq", wled);
    if (rc < 0)
    dev_err(wled.dev, "Unable to request short_irq (err:%d)\n",
    rc);
    return rc;
    }
    static int wled_configure_ovp_irq(struct wled *wled,
    struct platform_device *pdev)
    {
    int rc;
    u32 val;
    wled.ovp_irq = platform_get_irq_byname(pdev, "ovp");
    if (wled.ovp_irq < 0) {
    dev_dbg(&pdev.dev, "OVP IRQ not found - disabling automatic string detection\n");
    return 0;
    }
    rc = devm_request_threaded_irq(wled.dev, wled.ovp_irq, core::ptr::null_mut(),
    wled_ovp_irq_handler, IRQF_ONESHOT,
    "wled_ovp_irq", wled);
    if (rc < 0) {
    wled.ovp_irq = 0;
    return 0;
    }
    rc = regmap_read(wled.regmap, wled.ctrl_addr +
    WLED3_CTRL_REG_MOD_EN, &val);
    if (rc < 0)
    return rc;
// Keep OVP irq disabled until module is enabled
    if (!(val & WLED3_CTRL_REG_MOD_EN_MASK))
    disable_irq(wled.ovp_irq);
    return 0;
    }
    static const struct backlight_ops wled_ops = {
    .update_status = wled_update_status,
    };
#[no_mangle]
unsafe extern "C" fn wled_probe(pdev: *mut platform_device) -> c_int {
    static int wled_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct backlight_device *bl;
    struct wled *wled;
    struct regmap *regmap;
    u32 val;
    int rc;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(&pdev.dev, "Unable to get regmap\n");
    return -EINVAL;
    }
    wled = devm_kzalloc(&pdev.dev, sizeof(*wled), GFP_KERNEL);
    if (!wled)
    return -ENOMEM;
    wled.regmap = regmap;
    wled.dev = &pdev.dev;
    wled.version = (uintptr_t)of_device_get_match_data(&pdev.dev);
    if (!wled.version) {
    dev_err(&pdev.dev, "Unknown device version\n");
    return -ENODEV;
    }
    mutex_init(&wled.lock);
    rc = wled_configure(wled);
    if (rc)
    return rc;
    val = WLED3_SINK_REG_BRIGHT_MAX;
    of_property_read_u32(pdev.dev.of_node, "max-brightness", &val);
    wled.max_brightness = val;
    switch (wled.version) {
    case 3:
    wled.cfg.auto_detection_enabled = false;
    rc = wled3_setup(wled);
    if (rc) {
    dev_err(&pdev.dev, "wled3_setup failed\n");
    return rc;
    }
    break;
    case 4:
    wled.has_short_detect = true;
    rc = wled4_setup(wled);
    if (rc) {
    dev_err(&pdev.dev, "wled4_setup failed\n");
    return rc;
    }
    break;
    case 5:
    wled.has_short_detect = true;
    if (wled.cfg.cabc_sel)
    wled.max_brightness = WLED5_SINK_REG_BRIGHT_MAX_12B;
    rc = wled5_setup(wled);
    if (rc) {
    dev_err(&pdev.dev, "wled5_setup failed\n");
    return rc;
    }
    break;
    default:
    dev_err(wled.dev, "Invalid WLED version\n");
    break;
    }
    INIT_DELAYED_WORK(&wled.ovp_work, wled_ovp_work);
    rc = wled_configure_short_irq(wled, pdev);
    if (rc < 0)
    return rc;
    rc = wled_configure_ovp_irq(wled, pdev);
    if (rc < 0)
    return rc;
    val = WLED_DEFAULT_BRIGHTNESS;
    of_property_read_u32(pdev.dev.of_node, "default-brightness", &val);
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.brightness = val;
    props.max_brightness = wled.max_brightness;
    bl = devm_backlight_device_register(&pdev.dev, wled.name,
    &pdev.dev, wled,
    &wled_ops, &props);
    return PTR_ERR_OR_ZERO(bl);
    };
#[no_mangle]
unsafe extern "C" fn wled_remove(pdev: *mut platform_device) {
    static void wled_remove(struct platform_device *pdev)
    {
    struct wled *wled = platform_get_drvdata(pdev);
    mutex_destroy(&wled.lock);
    cancel_delayed_work_sync(&wled.ovp_work);
    disable_irq(wled.short_irq);
    disable_irq(wled.ovp_irq);
    }
    static const struct of_device_id wled_match_table[] = {
    { .compatible = "qcom,pm8941-wled", .data = (void *)3 },
    { .compatible = "qcom,pmi8950-wled", .data = (void *)4 },
    { .compatible = "qcom,pmi8994-wled", .data = (void *)4 },
    { .compatible = "qcom,pmi8998-wled", .data = (void *)4 },
    { .compatible = "qcom,pm660l-wled", .data = (void *)4 },
    { .compatible = "qcom,pm6150l-wled", .data = (void *)5 },
    { .compatible = "qcom,pm8150l-wled", .data = (void *)5 },
    {}
    };
    MODULE_DEVICE_TABLE(of, wled_match_table);
    static struct platform_driver wled_driver = {
    .probe = wled_probe,
    .remove = wled_remove,
    .driver	= {
    .name = "qcom,wled",
    .of_match_table	= wled_match_table,
    },
    };
    module_platform_driver(wled_driver);
    MODULE_DESCRIPTION("Qualcomm WLED driver");
    MODULE_LICENSE("GPL v2");
