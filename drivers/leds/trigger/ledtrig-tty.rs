//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-tty.c
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

pub const LEDTRIG_TTY_INTERVAL: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ledtrig_tty_data {
    pub led_cdev: *mut led_classdev,
    pub dwork: delayed_work,
    pub sysfs: completion,
    pub ttyname: *const c_char,
    pub tty: *mut tty_struct,
    pub tx: int rx,,
    pub mode_rx: bool,
    pub mode_tx: bool,
    pub mode_cts: bool,
    pub mode_dsr: bool,
    pub mode_dcd: bool,
    pub mode_rng: bool,
}

// Indicates which state the LED should now display
    enum led_trigger_tty_state {
    TTY_LED_BLINK,
    TTY_LED_ENABLE,
    TTY_LED_DISABLE,
    };
    enum led_trigger_tty_modes {
    TRIGGER_TTY_RX = 0,
    TRIGGER_TTY_TX,
    TRIGGER_TTY_CTS,
    TRIGGER_TTY_DSR,
    TRIGGER_TTY_DCD,
    TRIGGER_TTY_RNG,
    };
#[no_mangle]
unsafe extern "C" fn ledtrig_tty_wait_for_completion(dev: *mut device) -> c_int {
    static int ledtrig_tty_wait_for_completion(struct device *dev)
    {
    struct ledtrig_tty_data *trigger_data = led_trigger_get_drvdata(dev);
    int ret;
    ret = wait_for_completion_timeout(&trigger_data.sysfs,
    msecs_to_jiffies(LEDTRIG_TTY_INTERVAL * 20));
    if (ret == 0)
    return -ETIMEDOUT;
    return ret;
    }
    static ssize_t ttyname_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ledtrig_tty_data *trigger_data = led_trigger_get_drvdata(dev);
    let mut len: isize = 0;
    int completion;
    reinit_completion(&trigger_data.sysfs);
    completion = ledtrig_tty_wait_for_completion(dev);
    if (completion < 0)
    return completion;
    if (trigger_data.ttyname)
    len = sprintf(buf, "%s\n", trigger_data.ttyname);
    return len;
    }
    static ssize_t ttyname_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t size)
    {
    struct ledtrig_tty_data *trigger_data = led_trigger_get_drvdata(dev);
    char *ttyname;
    let mut ret: isize = size;
    int completion;
    if (size > 0 && buf[size - 1] == '\n')
    size -= 1;
    if (size) {
    ttyname = kmemdup_nul(buf, size, GFP_KERNEL);
    if (!ttyname)
    return -ENOMEM;
    } else {
    ttyname = core::ptr::null_mut();
    }
    reinit_completion(&trigger_data.sysfs);
    completion = ledtrig_tty_wait_for_completion(dev);
    if (completion < 0)
    return completion;
    kfree(trigger_data.ttyname);
    tty_kref_put(trigger_data.tty);
    trigger_data.tty = core::ptr::null_mut();
    trigger_data.ttyname = ttyname;
    return ret;
    }
    static DEVICE_ATTR_RW(ttyname);
    static ssize_t ledtrig_tty_attr_show(struct device *dev, char *buf,
    enum led_trigger_tty_modes attr)
    {
    struct ledtrig_tty_data *trigger_data = led_trigger_get_drvdata(dev);
    bool state;
    switch (attr) {
    case TRIGGER_TTY_RX:
    state = trigger_data.mode_rx;
    break;
    case TRIGGER_TTY_TX:
    state = trigger_data.mode_tx;
    break;
    case TRIGGER_TTY_CTS:
    state = trigger_data.mode_cts;
    break;
    case TRIGGER_TTY_DSR:
    state = trigger_data.mode_dsr;
    break;
    case TRIGGER_TTY_DCD:
    state = trigger_data.mode_dcd;
    break;
    case TRIGGER_TTY_RNG:
    state = trigger_data.mode_rng;
    break;
    }
    return sysfs_emit(buf, "%u\n", state);
    }
    static ssize_t ledtrig_tty_attr_store(struct device *dev, const char *buf,
    size_t size, enum led_trigger_tty_modes attr)
    {
    struct ledtrig_tty_data *trigger_data = led_trigger_get_drvdata(dev);
    bool state;
    int ret;
    ret = kstrtobool(buf, &state);
    if (ret)
    return ret;
    switch (attr) {
    case TRIGGER_TTY_RX:
    trigger_data.mode_rx = state;
    break;
    case TRIGGER_TTY_TX:
    trigger_data.mode_tx = state;
    break;
    case TRIGGER_TTY_CTS:
    trigger_data.mode_cts = state;
    break;
    case TRIGGER_TTY_DSR:
    trigger_data.mode_dsr = state;
    break;
    case TRIGGER_TTY_DCD:
    trigger_data.mode_dcd = state;
    break;
    case TRIGGER_TTY_RNG:
    trigger_data.mode_rng = state;
    break;
    }
    return size;
    }

    static ssize_t trigger_name##_show(struct device *dev, \
    struct device_attribute *attr, char *buf) \
    { \
    return ledtrig_tty_attr_show(dev, buf, trigger); \
    } \
    static ssize_t trigger_name##_store(struct device *dev, \
    struct device_attribute *attr, const char *buf, size_t size) \
    { \
    return ledtrig_tty_attr_store(dev, buf, size, trigger); \
    } \
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RW(_arg: trigger_name) -> static {
    static DEVICE_ATTR_RW(trigger_name)
    DEFINE_TTY_TRIGGER(rx, TRIGGER_TTY_RX);
    DEFINE_TTY_TRIGGER(tx, TRIGGER_TTY_TX);
    DEFINE_TTY_TRIGGER(cts, TRIGGER_TTY_CTS);
    DEFINE_TTY_TRIGGER(dsr, TRIGGER_TTY_DSR);
    DEFINE_TTY_TRIGGER(dcd, TRIGGER_TTY_DCD);
    DEFINE_TTY_TRIGGER(rng, TRIGGER_TTY_RNG);
#[no_mangle]
unsafe extern "C" fn ledtrig_tty_work(work: *mut work_struct) {
    static void ledtrig_tty_work(struct work_struct *work)
    {
    struct ledtrig_tty_data *trigger_data =
    container_of(work, struct ledtrig_tty_data, dwork.work);
    let mut state: enum led_trigger_tty_state = TTY_LED_DISABLE;
    let mut interval: c_ulong = LEDTRIG_TTY_INTERVAL;
    let mut invert: bool = false;
    int status;
    int ret;
    if (!trigger_data.ttyname)
    goto out;
// try to get the tty corresponding to $ttyname
    if (!trigger_data.tty) {
    dev_t devno;
    struct tty_struct *tty;
    int ret;
    ret = tty_dev_name_to_number(trigger_data.ttyname, &devno);
    if (ret < 0)
//
// A device with this name might appear later, so keep
// retrying.
//
    goto out;
    tty = tty_kopen_shared(devno);
    if (IS_ERR_OR_NULL(tty))
// What to do? retry or abort
    goto out;
    trigger_data.tty = tty;
    }
    status = tty_get_tiocm(trigger_data.tty);
    if (status > 0) {
    if (trigger_data.mode_cts) {
    if (status & TIOCM_CTS)
    state = TTY_LED_ENABLE;
    }
    if (trigger_data.mode_dsr) {
    if (status & TIOCM_DSR)
    state = TTY_LED_ENABLE;
    }
    if (trigger_data.mode_dcd) {
    if (status & TIOCM_CAR)
    state = TTY_LED_ENABLE;
    }
    if (trigger_data.mode_rng) {
    if (status & TIOCM_RNG)
    state = TTY_LED_ENABLE;
    }
    }
//
// The evaluation of rx/tx must be done after the evaluation
// of TIOCM_*, because rx/tx has priority.
//
    if (trigger_data.mode_rx || trigger_data.mode_tx) {
    struct serial_icounter_struct icount;
    ret = tty_get_icount(trigger_data.tty, &icount);
    if (ret)
    goto out;
    if (trigger_data.mode_tx && (icount.tx != trigger_data.tx)) {
    trigger_data.tx = icount.tx;
    invert = state == TTY_LED_ENABLE;
    state = TTY_LED_BLINK;
    }
    if (trigger_data.mode_rx && (icount.rx != trigger_data.rx)) {
    trigger_data.rx = icount.rx;
    invert = state == TTY_LED_ENABLE;
    state = TTY_LED_BLINK;
    }
    }
    out:
    switch (state) {
    case TTY_LED_BLINK:
    led_blink_set_oneshot(trigger_data.led_cdev, &interval,
    &interval, invert);
    break;
    case TTY_LED_ENABLE:
    led_set_brightness(trigger_data.led_cdev,
    trigger_data.led_cdev.blink_brightness);
    break;
    case TTY_LED_DISABLE:
    fallthrough;
    default:
    led_set_brightness(trigger_data.led_cdev, LED_OFF);
    break;
    }
    complete_all(&trigger_data.sysfs);
    schedule_delayed_work(&trigger_data.dwork,
    msecs_to_jiffies(LEDTRIG_TTY_INTERVAL * 2));
    }
    static struct attribute *ledtrig_tty_attrs[] = {
    &dev_attr_ttyname.attr,
    &dev_attr_rx.attr,
    &dev_attr_tx.attr,
    &dev_attr_cts.attr,
    &dev_attr_dsr.attr,
    &dev_attr_dcd.attr,
    &dev_attr_rng.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(ledtrig_tty);
#[no_mangle]
unsafe extern "C" fn ledtrig_tty_activate(led_cdev: *mut led_classdev) -> c_int {
    static int ledtrig_tty_activate(struct led_classdev *led_cdev)
    {
    struct ledtrig_tty_data *trigger_data;
    trigger_data = kzalloc_obj(*trigger_data);
    if (!trigger_data)
    return -ENOMEM;
// Enable default rx/tx mode
    trigger_data.mode_rx = true;
    trigger_data.mode_tx = true;
    led_set_trigger_data(led_cdev, trigger_data);
    INIT_DELAYED_WORK(&trigger_data.dwork, ledtrig_tty_work);
    trigger_data.led_cdev = led_cdev;
    init_completion(&trigger_data.sysfs);
    schedule_delayed_work(&trigger_data.dwork, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ledtrig_tty_deactivate(led_cdev: *mut led_classdev) {
    static void ledtrig_tty_deactivate(struct led_classdev *led_cdev)
    {
    struct ledtrig_tty_data *trigger_data = led_get_trigger_data(led_cdev);
    cancel_delayed_work_sync(&trigger_data.dwork);
    kfree(trigger_data.ttyname);
    tty_kref_put(trigger_data.tty);
    trigger_data.tty = core::ptr::null_mut();
    kfree(trigger_data);
    }
    static struct led_trigger ledtrig_tty = {
    .name = "tty",
    .activate = ledtrig_tty_activate,
    .deactivate = ledtrig_tty_deactivate,
    .groups = ledtrig_tty_groups,
    };
    module_led_trigger(ledtrig_tty);
    MODULE_AUTHOR("Uwe Kleine-König <u.kleine-koenig@pengutronix.de>");
    MODULE_DESCRIPTION("UART LED trigger");
    MODULE_LICENSE("GPL v2");
