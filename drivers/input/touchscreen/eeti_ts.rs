//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/eeti_ts.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Touch Screen driver for EETI's I2C connected touch screen panels
// Copyright (c) 2009,2018 Daniel Mack <daniel@zonque.org>
//
// See EETI's software guide for the protocol specification:
// http://home.eeti.com.tw/documentation.html
//
// Based on migor_ts.c
// Copyright (c) 2008 Magnus Damm
// Copyright (c) 2007 Ujjwal Pande <ujjwal@kenati.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeti_ts {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub attn_gpio: *mut gpio_desc,
    pub props: touchscreen_properties,
    pub mutex: mutex,
    pub running: bool,
}

#[no_mangle]
unsafe extern "C" fn eeti_ts_report_event(eeti: *mut eeti_ts, buf: *mut u8) {
    static void eeti_ts_report_event(struct eeti_ts *eeti, u8 *buf)
    {
    unsigned int res;
    u16 x, y;
    res = REPORT_RES_BITS(buf[0] & (REPORT_BIT_AD0 | REPORT_BIT_AD1));
    x = get_unaligned_be16(&buf[1]);
    y = get_unaligned_be16(&buf[3]);
// fix the range to 11 bits
    x >>= res - EETI_TS_BITDEPTH;
    y >>= res - EETI_TS_BITDEPTH;
    if (buf[0] & REPORT_BIT_HAS_PRESSURE)
    input_report_abs(eeti.input, ABS_PRESSURE, buf[5]);
    touchscreen_report_pos(eeti.input, &eeti.props, x, y, false);
    input_report_key(eeti.input, BTN_TOUCH, buf[0] & REPORT_BIT_PRESSED);
    input_sync(eeti.input);
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_read(eeti: *mut eeti_ts) -> c_int {
    static int eeti_ts_read(struct eeti_ts *eeti)
    {
    int len, error;
    char buf[6];
    len = i2c_master_recv(eeti.client, buf, sizeof(buf));
    if (len != sizeof(buf)) {
    error = len < 0 ? len : -EIO;
    dev_err(&eeti.client.dev,
    "failed to read touchscreen data: %d\n",
    error);
    return error;
    }
// Motion packet
    if (buf[0] & 0x80)
    eeti_ts_report_event(eeti, buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t eeti_ts_isr(int irq, void *dev_id)
    {
    struct eeti_ts *eeti = dev_id;
    int error;
    guard(mutex)(&eeti.mutex);
    do {
//
// If we have attention GPIO, trust it. Otherwise we'll read
// once and exit. We assume that in this case we are using
// level triggered interrupt and it will get raised again
// if/when there is more data.
//
    if (eeti.attn_gpio &&
    !gpiod_get_value_cansleep(eeti.attn_gpio)) {
    break;
    }
    error = eeti_ts_read(eeti);
    if (error)
    break;
    } while (eeti.running && eeti.attn_gpio);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_start(eeti: *mut eeti_ts) {
    static void eeti_ts_start(struct eeti_ts *eeti)
    {
    guard(mutex)(&eeti.mutex);
    eeti.running = true;
    enable_irq(eeti.client.irq);
//
// Kick the controller in case we are using edge interrupt and
// we missed our edge while interrupt was disabled. We expect
// the attention GPIO to be wired in this case.
//
    if (eeti.attn_gpio && gpiod_get_value_cansleep(eeti.attn_gpio))
    eeti_ts_read(eeti);
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_stop(eeti: *mut eeti_ts) {
    static void eeti_ts_stop(struct eeti_ts *eeti)
    {
//
// Not locking here, just setting a flag and expect that the
// interrupt thread will notice the flag eventually.
//
    eeti.running = false;
    wmb();
    disable_irq(eeti.client.irq);
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_open(dev: *mut input_dev) -> c_int {
    static int eeti_ts_open(struct input_dev *dev)
    {
    struct eeti_ts *eeti = input_get_drvdata(dev);
    eeti_ts_start(eeti);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_close(dev: *mut input_dev) {
    static void eeti_ts_close(struct input_dev *dev)
    {
    struct eeti_ts *eeti = input_get_drvdata(dev);
    eeti_ts_stop(eeti);
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_probe(client: *mut i2c_client) -> c_int {
    static int eeti_ts_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct eeti_ts *eeti;
    struct input_dev *input;
    int error;
//
// In contrast to what's described in the datasheet, there seems
// to be no way of probing the presence of that device using I2C
// commands. So we need to blindly believe it is there, and wait
// for interrupts to occur.
//
    eeti = devm_kzalloc(dev, sizeof(*eeti), GFP_KERNEL);
    if (!eeti) {
    dev_err(dev, "failed to allocate driver data\n");
    return -ENOMEM;
    }
    mutex_init(&eeti.mutex);
    input = devm_input_allocate_device(dev);
    if (!input) {
    dev_err(dev, "Failed to allocate input device.\n");
    return -ENOMEM;
    }
    input_set_capability(input, EV_KEY, BTN_TOUCH);
    input_set_abs_params(input, ABS_X, 0, EETI_MAXVAL, 0, 0);
    input_set_abs_params(input, ABS_Y, 0, EETI_MAXVAL, 0, 0);
    input_set_abs_params(input, ABS_PRESSURE, 0, 0xff, 0, 0);
    touchscreen_parse_properties(input, false, &eeti.props);
    input.name = client.name;
    input.id.bustype = BUS_I2C;
    input.open = eeti_ts_open;
    input.close = eeti_ts_close;
    eeti.client = client;
    eeti.input = input;
    eeti.attn_gpio = devm_gpiod_get_optional(dev, "attn", GPIOD_IN);
    if (IS_ERR(eeti.attn_gpio))
    return PTR_ERR(eeti.attn_gpio);
    i2c_set_clientdata(client, eeti);
    input_set_drvdata(input, eeti);
    error = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), eeti_ts_isr,
    IRQF_ONESHOT,
    client.name, eeti);
    if (error) {
    dev_err(dev, "Unable to request touchscreen IRQ: %d\n",
    error);
    return error;
    }
//
// Disable the device for now. It will be enabled once the
// input device is opened.
//
    eeti_ts_stop(eeti);
    error = input_register_device(input);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_suspend(dev: *mut device) -> c_int {
    static int eeti_ts_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct eeti_ts *eeti = i2c_get_clientdata(client);
    struct input_dev *input_dev = eeti.input;
    scoped_guard(mutex, &input_dev.mutex) {
    if (input_device_enabled(input_dev))
    eeti_ts_stop(eeti);
    }
    if (device_may_wakeup(&client.dev))
    enable_irq_wake(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eeti_ts_resume(dev: *mut device) -> c_int {
    static int eeti_ts_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct eeti_ts *eeti = i2c_get_clientdata(client);
    struct input_dev *input_dev = eeti.input;
    if (device_may_wakeup(&client.dev))
    disable_irq_wake(client.irq);
    scoped_guard(mutex, &input_dev.mutex) {
    if (input_device_enabled(input_dev))
    eeti_ts_start(eeti);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(eeti_ts_pm, eeti_ts_suspend, eeti_ts_resume);
    static const struct i2c_device_id eeti_ts_id[] = {
    { .name = "eeti_ts" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, eeti_ts_id);

    static const struct of_device_id of_eeti_ts_match[] = {
    { .compatible = "eeti,exc3000-i2c", },
    { }
    };

    static struct i2c_driver eeti_ts_driver = {
    .driver = {
    .name = "eeti_ts",
    .pm = pm_sleep_ptr(&eeti_ts_pm),
    .of_match_table = of_match_ptr(of_eeti_ts_match),
    },
    .probe = eeti_ts_probe,
    .id_table = eeti_ts_id,
    };
    module_i2c_driver(eeti_ts_driver);
    MODULE_DESCRIPTION("EETI Touchscreen driver");
    MODULE_AUTHOR("Daniel Mack <daniel@zonque.org>");
    MODULE_LICENSE("GPL");
