//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ucb1x00-ts.c
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
//
// Touchscreen driver for UCB1x00-based touchscreens
//
// Copyright (C) 2001 Russell King, All Rights Reserved.
// Copyright (C) 2005 Pavel Machek
//
// 21-Jan-2002 <jco@ict.es> :
//
// Added support for synchronous A/D mode. This mode is useful to
// avoid noise induced in the touchpanel by the LCD, provided that
// the UCB1x00 has a valid LCD sync signal routed to its ADCSYNC pin.
// It is important to note that the signal connected to the ADCSYNC
// pin should provide pulses even when the LCD is blanked, otherwise
// a pen touch needed to unblank the LCD will never be read.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucb1x00_ts {
    pub idev: *mut input_dev,
    pub ucb: *mut ucb1x00,
    pub irq_lock: spinlock_t,
    pub irq_disabled: unsigned,
    pub irq_wait: wait_queue_head_t,
    pub rtask: *mut task_struct,
    pub x_res: u16,
    pub y_res: u16,
    pub adcsync:1: c_uint,
}

    static int adcsync;
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_evt_add(ts: *mut ucb1x00_ts, pressure: u16, x: u16, y: u16) {
    static inline void ucb1x00_ts_evt_add(struct ucb1x00_ts *ts, u16 pressure, u16 x, u16 y)
    {
    struct input_dev *idev = ts.idev;
    input_report_abs(idev, ABS_X, x);
    input_report_abs(idev, ABS_Y, y);
    input_report_abs(idev, ABS_PRESSURE, pressure);
    input_report_key(idev, BTN_TOUCH, 1);
    input_sync(idev);
    }
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_event_release(ts: *mut ucb1x00_ts) {
    static inline void ucb1x00_ts_event_release(struct ucb1x00_ts *ts)
    {
    struct input_dev *idev = ts.idev;
    input_report_abs(idev, ABS_PRESSURE, 0);
    input_report_key(idev, BTN_TOUCH, 0);
    input_sync(idev);
    }
//
// Switch to interrupt mode.
//
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_mode_int(ts: *mut ucb1x00_ts) {
    static inline void ucb1x00_ts_mode_int(struct ucb1x00_ts *ts)
    {
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMX_POW | UCB_TS_CR_TSPX_POW |
    UCB_TS_CR_TSMY_GND | UCB_TS_CR_TSPY_GND |
    UCB_TS_CR_MODE_INT);
    }
//
// Switch to pressure mode, and read pressure.  We don't need to wait
// here, since both plates are being driven.
//
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_read_pressure(ts: *mut ucb1x00_ts) -> c_uint {
    static inline unsigned int ucb1x00_ts_read_pressure(struct ucb1x00_ts *ts)
    {
    if (machine_is_collie()) {
    ucb1x00_io_write(ts.ucb, COLLIE_TC35143_GPIO_TBL_CHK, 0);
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSPX_POW | UCB_TS_CR_TSMX_POW |
    UCB_TS_CR_MODE_POS | UCB_TS_CR_BIAS_ENA);
    udelay(55);
    return ucb1x00_adc_read(ts.ucb, UCB_ADC_INP_AD2, ts.adcsync);
    } else {
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMX_POW | UCB_TS_CR_TSPX_POW |
    UCB_TS_CR_TSMY_GND | UCB_TS_CR_TSPY_GND |
    UCB_TS_CR_MODE_PRES | UCB_TS_CR_BIAS_ENA);
    return ucb1x00_adc_read(ts.ucb, UCB_ADC_INP_TSPY, ts.adcsync);
    }
    }
//
// Switch to X position mode and measure Y plate.  We switch the plate
// configuration in pressure mode, then switch to position mode.  This
// gives a faster response time.  Even so, we need to wait about 55us
// for things to stabilise.
//
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_read_xpos(ts: *mut ucb1x00_ts) -> c_uint {
    static inline unsigned int ucb1x00_ts_read_xpos(struct ucb1x00_ts *ts)
    {
    if (machine_is_collie())
    ucb1x00_io_write(ts.ucb, 0, COLLIE_TC35143_GPIO_TBL_CHK);
    else {
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMX_GND | UCB_TS_CR_TSPX_POW |
    UCB_TS_CR_MODE_PRES | UCB_TS_CR_BIAS_ENA);
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMX_GND | UCB_TS_CR_TSPX_POW |
    UCB_TS_CR_MODE_PRES | UCB_TS_CR_BIAS_ENA);
    }
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMX_GND | UCB_TS_CR_TSPX_POW |
    UCB_TS_CR_MODE_POS | UCB_TS_CR_BIAS_ENA);
    udelay(55);
    return ucb1x00_adc_read(ts.ucb, UCB_ADC_INP_TSPY, ts.adcsync);
    }
//
// Switch to Y position mode and measure X plate.  We switch the plate
// configuration in pressure mode, then switch to position mode.  This
// gives a faster response time.  Even so, we need to wait about 55us
// for things to stabilise.
//
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_read_ypos(ts: *mut ucb1x00_ts) -> c_uint {
    static inline unsigned int ucb1x00_ts_read_ypos(struct ucb1x00_ts *ts)
    {
    if (machine_is_collie())
    ucb1x00_io_write(ts.ucb, 0, COLLIE_TC35143_GPIO_TBL_CHK);
    else {
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMY_GND | UCB_TS_CR_TSPY_POW |
    UCB_TS_CR_MODE_PRES | UCB_TS_CR_BIAS_ENA);
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMY_GND | UCB_TS_CR_TSPY_POW |
    UCB_TS_CR_MODE_PRES | UCB_TS_CR_BIAS_ENA);
    }
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMY_GND | UCB_TS_CR_TSPY_POW |
    UCB_TS_CR_MODE_POS | UCB_TS_CR_BIAS_ENA);
    udelay(55);
    return ucb1x00_adc_read(ts.ucb, UCB_ADC_INP_TSPX, ts.adcsync);
    }
//
// Switch to X plate resistance mode.  Set MX to ground, PX to
// supply.  Measure current.
//
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_read_xres(ts: *mut ucb1x00_ts) -> c_uint {
    static inline unsigned int ucb1x00_ts_read_xres(struct ucb1x00_ts *ts)
    {
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMX_GND | UCB_TS_CR_TSPX_POW |
    UCB_TS_CR_MODE_PRES | UCB_TS_CR_BIAS_ENA);
    return ucb1x00_adc_read(ts.ucb, 0, ts.adcsync);
    }
//
// Switch to Y plate resistance mode.  Set MY to ground, PY to
// supply.  Measure current.
//
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_read_yres(ts: *mut ucb1x00_ts) -> c_uint {
    static inline unsigned int ucb1x00_ts_read_yres(struct ucb1x00_ts *ts)
    {
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR,
    UCB_TS_CR_TSMY_GND | UCB_TS_CR_TSPY_POW |
    UCB_TS_CR_MODE_PRES | UCB_TS_CR_BIAS_ENA);
    return ucb1x00_adc_read(ts.ucb, 0, ts.adcsync);
    }
#[no_mangle]
pub unsafe extern "C" fn ucb1x00_ts_pen_down(ts: *mut ucb1x00_ts) -> c_int {
    static inline int ucb1x00_ts_pen_down(struct ucb1x00_ts *ts)
    {
    let mut val: c_uint = ucb1x00_reg_read(ts.ucb, UCB_TS_CR);
    if (machine_is_collie())
    return (!(val & (UCB_TS_CR_TSPX_LOW)));
    else
    return (val & (UCB_TS_CR_TSPX_LOW | UCB_TS_CR_TSMX_LOW));
    }
//
// This is a RT kernel thread that handles the ADC accesses
// (mainly so we can use semaphores in the UCB1200 core code
// to serialise accesses to the ADC).
//
#[no_mangle]
unsafe extern "C" fn ucb1x00_thread(_ts: *mut c_void) -> c_int {
    static int ucb1x00_thread(void *_ts)
    {
    struct ucb1x00_ts *ts = _ts;
    DECLARE_WAITQUEUE(wait, current);
    bool frozen, ignore = false;
    let mut valid: c_int = 0;
    set_freezable();
    add_wait_queue(&ts.irq_wait, &wait);
    while (!kthread_freezable_should_stop(&frozen)) {
    unsigned int x, y, p;
    signed long timeout;
    if (frozen)
    ignore = true;
    ucb1x00_adc_enable(ts.ucb);
    x = ucb1x00_ts_read_xpos(ts);
    y = ucb1x00_ts_read_ypos(ts);
    p = ucb1x00_ts_read_pressure(ts);
//
// Switch back to interrupt mode.
//
    ucb1x00_ts_mode_int(ts);
    ucb1x00_adc_disable(ts.ucb);
    msleep(10);
    ucb1x00_enable(ts.ucb);
    if (ucb1x00_ts_pen_down(ts)) {
    set_current_state(TASK_INTERRUPTIBLE);
    spin_lock_irq(&ts.irq_lock);
    if (ts.irq_disabled) {
    ts.irq_disabled = 0;
    enable_irq(ts.ucb.irq_base + UCB_IRQ_TSPX);
    }
    spin_unlock_irq(&ts.irq_lock);
    ucb1x00_disable(ts.ucb);
//
// If we spat out a valid sample set last time,
// spit out a "pen off" sample here.
//
    if (valid) {
    ucb1x00_ts_event_release(ts);
    valid = 0;
    }
    timeout = MAX_SCHEDULE_TIMEOUT;
    } else {
    ucb1x00_disable(ts.ucb);
//
// Filtering is policy.  Policy belongs in user
// space.  We therefore leave it to user space
// to do any filtering they please.
//
    if (!ignore) {
    ucb1x00_ts_evt_add(ts, p, x, y);
    valid = 1;
    }
    set_current_state(TASK_INTERRUPTIBLE);
    timeout = HZ / 100;
    }
    schedule_timeout(timeout);
    }
    remove_wait_queue(&ts.irq_wait, &wait);
    ts.rtask = core::ptr::null_mut();
    return 0;
    }
//
// We only detect touch screen _touches_ with this interrupt
// handler, and even then we just schedule our task.
//
#[no_mangle]
unsafe extern "C" fn ucb1x00_ts_irq(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ucb1x00_ts_irq(int irq, void *id)
    {
    struct ucb1x00_ts *ts = id;
    spin_lock(&ts.irq_lock);
    ts.irq_disabled = 1;
    disable_irq_nosync(ts.ucb.irq_base + UCB_IRQ_TSPX);
    spin_unlock(&ts.irq_lock);
    wake_up(&ts.irq_wait);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ucb1x00_ts_open(idev: *mut input_dev) -> c_int {
    static int ucb1x00_ts_open(struct input_dev *idev)
    {
    struct ucb1x00_ts *ts = input_get_drvdata(idev);
    let mut flags: c_ulong = 0;
    let mut ret: c_int = 0;
    BUG_ON(ts.rtask);
    if (machine_is_collie())
    flags = IRQF_TRIGGER_RISING;
    else
    flags = IRQF_TRIGGER_FALLING;
    ts.irq_disabled = 0;
    init_waitqueue_head(&ts.irq_wait);
    ret = request_irq(ts.ucb.irq_base + UCB_IRQ_TSPX, ucb1x00_ts_irq,
    flags, "ucb1x00-ts", ts);
    if (ret < 0)
    goto out;
//
// If we do this at all, we should allow the user to
// measure and read the X and Y resistance at any time.
//
    ucb1x00_adc_enable(ts.ucb);
    ts.x_res = ucb1x00_ts_read_xres(ts);
    ts.y_res = ucb1x00_ts_read_yres(ts);
    ucb1x00_adc_disable(ts.ucb);
    ts.rtask = kthread_run(ucb1x00_thread, ts, "ktsd");
    if (!IS_ERR(ts.rtask)) {
    ret = 0;
    } else {
    free_irq(ts.ucb.irq_base + UCB_IRQ_TSPX, ts);
    ts.rtask = core::ptr::null_mut();
    ret = -EFAULT;
    }
    out:
    return ret;
    }
//
// Release touchscreen resources.  Disable IRQs.
//
#[no_mangle]
unsafe extern "C" fn ucb1x00_ts_close(idev: *mut input_dev) {
    static void ucb1x00_ts_close(struct input_dev *idev)
    {
    struct ucb1x00_ts *ts = input_get_drvdata(idev);
    if (ts.rtask)
    kthread_stop(ts.rtask);
    ucb1x00_enable(ts.ucb);
    free_irq(ts.ucb.irq_base + UCB_IRQ_TSPX, ts);
    ucb1x00_reg_write(ts.ucb, UCB_TS_CR, 0);
    ucb1x00_disable(ts.ucb);
    }
//
// Initialisation.
//
#[no_mangle]
unsafe extern "C" fn ucb1x00_ts_add(dev: *mut ucb1x00_dev) -> c_int {
    static int ucb1x00_ts_add(struct ucb1x00_dev *dev)
    {
    struct ucb1x00_ts *ts;
    struct input_dev *idev;
    int err;
    ts = kzalloc_obj(struct ucb1x00_ts);
    idev = input_allocate_device();
    if (!ts || !idev) {
    err = -ENOMEM;
    goto fail;
    }
    ts.ucb = dev.ucb;
    ts.idev = idev;
    ts.adcsync = adcsync ? UCB_SYNC : UCB_NOSYNC;
    spin_lock_init(&ts.irq_lock);
    idev.name       = "Touchscreen panel";
    idev.id.product = ts.ucb.id;
    idev.open       = ucb1x00_ts_open;
    idev.close      = ucb1x00_ts_close;
    idev.dev.parent = &ts.ucb.dev;
    idev.evbit[0]   = BIT_MASK(EV_ABS) | BIT_MASK(EV_KEY);
    idev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_drvdata(idev, ts);
    ucb1x00_adc_enable(ts.ucb);
    ts.x_res = ucb1x00_ts_read_xres(ts);
    ts.y_res = ucb1x00_ts_read_yres(ts);
    ucb1x00_adc_disable(ts.ucb);
    input_set_abs_params(idev, ABS_X, 0, ts.x_res, 0, 0);
    input_set_abs_params(idev, ABS_Y, 0, ts.y_res, 0, 0);
    input_set_abs_params(idev, ABS_PRESSURE, 0, 0, 0, 0);
    err = input_register_device(idev);
    if (err)
    goto fail;
    dev.priv = ts;
    return 0;
    fail:
    input_free_device(idev);
    kfree(ts);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ucb1x00_ts_remove(dev: *mut ucb1x00_dev) {
    static void ucb1x00_ts_remove(struct ucb1x00_dev *dev)
    {
    struct ucb1x00_ts *ts = dev.priv;
    input_unregister_device(ts.idev);
    kfree(ts);
    }
    static struct ucb1x00_driver ucb1x00_ts_driver = {
    .add		= ucb1x00_ts_add,
    .remove		= ucb1x00_ts_remove,
    };
#[no_mangle]
unsafe extern "C" fn ucb1x00_ts_init() -> int __init {
    static int __init ucb1x00_ts_init(void)
    {
    return ucb1x00_register_driver(&ucb1x00_ts_driver);
    }
#[no_mangle]
unsafe extern "C" fn ucb1x00_ts_exit() -> void __exit {
    static void __exit ucb1x00_ts_exit(void)
    {
    ucb1x00_unregister_driver(&ucb1x00_ts_driver);
    }
    module_param(adcsync, int, 0444);
    module_init(ucb1x00_ts_init);
    module_exit(ucb1x00_ts_exit);
    MODULE_AUTHOR("Russell King <rmk@arm.linux.org.uk>");
    MODULE_DESCRIPTION("UCB1x00 touchscreen driver");
    MODULE_LICENSE("GPL");
