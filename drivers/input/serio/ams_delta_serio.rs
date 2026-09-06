//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/ams_delta_serio.c
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
// Amstrad E3 (Delta) keyboard port driver
//
// Copyright (c) 2006 Matt Callow
// Copyright (c) 2010 Janusz Krzysztofik
//
// Thanks to Cliff Lawson for his help
//
// The Amstrad Delta keyboard (aka mailboard) uses normal PC-AT style serial
// transmission.  The keyboard port is formed of two GPIO lines, for clock
// and data.  Due to strict timing requirements of the interface,
// the serial data stream is read and processed by a FIQ handler.
// The resulting words are fetched by this driver from a circular buffer.
//
// Standard AT keyboard driver (atkbd) is used for handling the keyboard data.
// However, when used with the E3 mailboard that producecs non-standard
// scancodes, a custom key table must be prepared and loaded from userspace.
//

    MODULE_AUTHOR("Matt Callow");
    MODULE_DESCRIPTION("AMS Delta (E3) keyboard port driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ams_delta_serio {
    pub serio: *mut serio,
    pub vcc: *mut regulator,
    pub fiq_buffer: *mut c_uint,
}

#[no_mangle]
unsafe extern "C" fn check_data(serio: *mut serio, data: c_int) -> c_int {
    static int check_data(struct serio *serio, int data)
    {
    int i, parity = 0;
// check valid stop bit
    if (!(data & 0x400)) {
    dev_warn(&serio.dev, "invalid stop bit, data=0x%X\n", data);
    return SERIO_FRAME;
    }
// calculate the parity
    for (i = 1; i < 10; i++) {
    if (data & (1 << i))
    parity++;
    }
// it should be odd
    if (!(parity & 0x01)) {
    dev_warn(&serio.dev,
    "parity check failed, data=0x%X parity=0x%X\n", data,
    parity);
    return SERIO_PARITY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams_delta_serio_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ams_delta_serio_interrupt(int irq, void *dev_id)
    {
    struct ams_delta_serio *priv = dev_id;
    int *circ_buff = &priv.fiq_buffer[FIQ_CIRC_BUFF];
    int data, dfl;
    u8 scancode;
    priv.fiq_buffer[FIQ_IRQ_PEND] = 0;
//
// Read data from the circular buffer, check it
// and then pass it on the serio
//
    while (priv.fiq_buffer[FIQ_KEYS_CNT] > 0) {
    data = circ_buff[priv.fiq_buffer[FIQ_HEAD_OFFSET]++];
    priv.fiq_buffer[FIQ_KEYS_CNT]--;
    if (priv.fiq_buffer[FIQ_HEAD_OFFSET] ==
    priv.fiq_buffer[FIQ_BUF_LEN])
    priv.fiq_buffer[FIQ_HEAD_OFFSET] = 0;
    dfl = check_data(priv.serio, data);
    scancode = (u8) (data >> 1) & 0xFF;
    serio_interrupt(priv.serio, scancode, dfl);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ams_delta_serio_open(serio: *mut serio) -> c_int {
    static int ams_delta_serio_open(struct serio *serio)
    {
    struct ams_delta_serio *priv = serio.port_data;
// enable keyboard
    return regulator_enable(priv.vcc);
    }
#[no_mangle]
unsafe extern "C" fn ams_delta_serio_close(serio: *mut serio) {
    static void ams_delta_serio_close(struct serio *serio)
    {
    struct ams_delta_serio *priv = serio.port_data;
// disable keyboard
    regulator_disable(priv.vcc);
    }
#[no_mangle]
unsafe extern "C" fn ams_delta_serio_init(pdev: *mut platform_device) -> c_int {
    static int ams_delta_serio_init(struct platform_device *pdev)
    {
    struct ams_delta_serio *priv;
    struct serio *serio;
    int irq, err;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.fiq_buffer = pdev.dev.platform_data;
    if (!priv.fiq_buffer)
    return -EINVAL;
    priv.vcc = devm_regulator_get(&pdev.dev, "vcc");
    if (IS_ERR(priv.vcc)) {
    err = PTR_ERR(priv.vcc);
    dev_err(&pdev.dev, "regulator request failed (%d)\n", err);
//
// When running on a non-dt platform and requested regulator
// is not available, devm_regulator_get() never returns
// -EPROBE_DEFER as it is not able to justify if the regulator
// may still appear later.  On the other hand, the board can
// still set full constriants flag at late_initcall in order
// to instruct devm_regulator_get() to returnn a dummy one
// if sufficient.  Hence, if we get -ENODEV here, let's convert
// it to -EPROBE_DEFER and wait for the board to decide or
// let Deferred Probe infrastructure handle this error.
//
    if (err == -ENODEV)
    err = -EPROBE_DEFER;
    return err;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -ENXIO;
    err = devm_request_irq(&pdev.dev, irq, ams_delta_serio_interrupt,
    IRQ_TYPE_EDGE_RISING, DRIVER_NAME, priv);
    if (err < 0) {
    dev_err(&pdev.dev, "IRQ request failed (%d)\n", err);
    return err;
    }
    serio = kzalloc_obj(*serio);
    if (!serio)
    return -ENOMEM;
    priv.serio = serio;
    serio.id.type = SERIO_8042;
    serio.open = ams_delta_serio_open;
    serio.close = ams_delta_serio_close;
    strscpy(serio.name, "AMS DELTA keyboard adapter", sizeof(serio.name));
    strscpy(serio.phys, dev_name(&pdev.dev), sizeof(serio.phys));
    serio.dev.parent = &pdev.dev;
    serio.port_data = priv;
    serio_register_port(serio);
    platform_set_drvdata(pdev, priv);
    dev_info(&serio.dev, "%s\n", serio.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams_delta_serio_exit(pdev: *mut platform_device) {
    static void ams_delta_serio_exit(struct platform_device *pdev)
    {
    struct ams_delta_serio *priv = platform_get_drvdata(pdev);
    serio_unregister_port(priv.serio);
    }
    static struct platform_driver ams_delta_serio_driver = {
    .probe	= ams_delta_serio_init,
    .remove	= ams_delta_serio_exit,
    .driver	= {
    .name	= DRIVER_NAME
    },
    };
    module_platform_driver(ams_delta_serio_driver);
