//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/hp680_ts_input.c
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

pub const HP680_TS_ABS_X_MIN: c_int = 40;
pub const HP680_TS_ABS_X_MAX: c_int = 950;
pub const HP680_TS_ABS_Y_MIN: c_int = 80;
pub const HP680_TS_ABS_Y_MAX: c_int = 910;
pub const PHDR: c_uint = 0xa400012e;
pub const SCPDR: c_uint = 0xa4000136;
    static void do_softint(struct work_struct *work);
    static struct input_dev *hp680_ts_dev;
    static DECLARE_DELAYED_WORK(work, do_softint);
#[no_mangle]
unsafe extern "C" fn do_softint(work: *mut work_struct) {
    static void do_softint(struct work_struct *work)
    {
    let mut absx: c_int = 0, absy = 0;
    u8 scpdr;
    let mut touched: c_int = 0;
    if (__raw_readb(PHDR) & PHDR_TS_PEN_DOWN) {
    scpdr = __raw_readb(SCPDR);
    scpdr |= SCPDR_TS_SCAN_ENABLE;
    scpdr &= ~SCPDR_TS_SCAN_Y;
    __raw_writeb(scpdr, SCPDR);
    udelay(30);
    absy = adc_single(ADC_CHANNEL_TS_Y);
    scpdr = __raw_readb(SCPDR);
    scpdr |= SCPDR_TS_SCAN_Y;
    scpdr &= ~SCPDR_TS_SCAN_X;
    __raw_writeb(scpdr, SCPDR);
    udelay(30);
    absx = adc_single(ADC_CHANNEL_TS_X);
    scpdr = __raw_readb(SCPDR);
    scpdr |= SCPDR_TS_SCAN_X;
    scpdr &= ~SCPDR_TS_SCAN_ENABLE;
    __raw_writeb(scpdr, SCPDR);
    udelay(100);
    touched = __raw_readb(PHDR) & PHDR_TS_PEN_DOWN;
    }
    if (touched) {
    input_report_key(hp680_ts_dev, BTN_TOUCH, 1);
    input_report_abs(hp680_ts_dev, ABS_X, absx);
    input_report_abs(hp680_ts_dev, ABS_Y, absy);
    } else {
    input_report_key(hp680_ts_dev, BTN_TOUCH, 0);
    }
    input_sync(hp680_ts_dev);
    enable_irq(HP680_TS_IRQ);
    }
#[no_mangle]
unsafe extern "C" fn hp680_ts_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t hp680_ts_interrupt(int irq, void *dev)
    {
    disable_irq_nosync(irq);
    schedule_delayed_work(&work, HZ / 20);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hp680_ts_init() -> int __init {
    static int __init hp680_ts_init(void)
    {
    int err;
    hp680_ts_dev = input_allocate_device();
    if (!hp680_ts_dev)
    return -ENOMEM;
    hp680_ts_dev.evbit[0] = BIT_MASK(EV_ABS) | BIT_MASK(EV_KEY);
    hp680_ts_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(hp680_ts_dev, ABS_X,
    HP680_TS_ABS_X_MIN, HP680_TS_ABS_X_MAX, 0, 0);
    input_set_abs_params(hp680_ts_dev, ABS_Y,
    HP680_TS_ABS_Y_MIN, HP680_TS_ABS_Y_MAX, 0, 0);
    hp680_ts_dev.name = "HP Jornada touchscreen";
    hp680_ts_dev.phys = "hp680_ts/input0";
    if (request_irq(HP680_TS_IRQ, hp680_ts_interrupt,
    0, MODNAME, core::ptr::null_mut()) < 0) {
    printk(KERN_ERR "hp680_touchscreen.c: Can't allocate irq %d\n",
    HP680_TS_IRQ);
    err = -EBUSY;
    goto fail1;
    }
    err = input_register_device(hp680_ts_dev);
    if (err)
    goto fail2;
    return 0;
    fail2:	free_irq(HP680_TS_IRQ, core::ptr::null_mut());
    cancel_delayed_work_sync(&work);
    fail1:	input_free_device(hp680_ts_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hp680_ts_exit() -> void __exit {
    static void __exit hp680_ts_exit(void)
    {
    free_irq(HP680_TS_IRQ, core::ptr::null_mut());
    cancel_delayed_work_sync(&work);
    input_unregister_device(hp680_ts_dev);
    }
    module_init(hp680_ts_init);
    module_exit(hp680_ts_exit);
    MODULE_AUTHOR("Andriy Skulysh, askulysh@image.kiev.ua");
    MODULE_DESCRIPTION("HP Jornada 680 touchscreen driver");
    MODULE_LICENSE("GPL");
