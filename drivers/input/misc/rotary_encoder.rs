//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/rotary_encoder.c
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
// rotary_encoder.c
//
// (c) 2009 Daniel Mack <daniel@caiaq.de>
// Copyright (C) 2011 Johan Hovold <jhovold@gmail.com>
//
// state machine code inspired by code from Tim Ruetz
//
// A generic driver for rotary encoders connected to GPIO lines.
// See file:Documentation/input/devices/rotary-encoder.rst for more information
//

    enum rotary_encoder_encoding {
    ROTENC_GRAY,
    ROTENC_BINARY,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rotary_encoder {
    pub input: *mut input_dev,
    pub access_mutex: mutex,
    pub steps: u32,
    pub axis: u32,
    pub relative_axis: bool,
    pub rollover: bool,
    pub encoding: enum rotary_encoder_encoding,
    pub pos: c_uint,
    pub gpios: *mut gpio_descs,
    pub irq: *mut c_uint,
    pub armed: bool,
    pub /: *mut *mut signed char dir; / 1 - clockwise, -1 - CCW,
    pub last_stable: c_uint,
}

#[no_mangle]
unsafe extern "C" fn rotary_encoder_get_state(encoder: *mut rotary_encoder) -> c_uint {
    static unsigned int rotary_encoder_get_state(struct rotary_encoder *encoder)
    {
    int i;
    let mut ret: c_uint = 0;
    for (i = 0; i < encoder.gpios.ndescs; ++i) {
    let mut val: c_int = gpiod_get_value_cansleep(encoder.gpios.desc[i]);
// convert from gray encoding to normal
    if (encoder.encoding == ROTENC_GRAY && ret & 1)
    val = !val;
    ret = ret << 1 | val;
    }
    return ret & 3;
    }
#[no_mangle]
unsafe extern "C" fn rotary_encoder_report_event(encoder: *mut rotary_encoder) {
    static void rotary_encoder_report_event(struct rotary_encoder *encoder)
    {
    if (encoder.relative_axis) {
    input_report_rel(encoder.input,
    encoder.axis, encoder.dir);
    } else {
    let mut pos: c_uint = encoder.pos;
    if (encoder.dir < 0) {
// turning counter-clockwise
    if (encoder.rollover)
    pos += encoder.steps;
    if (pos)
    pos--;
    } else {
// turning clockwise
    if (encoder.rollover || pos < encoder.steps)
    pos++;
    }
    if (encoder.rollover)
    pos %= encoder.steps;
    encoder.pos = pos;
    input_report_abs(encoder.input, encoder.axis, encoder.pos);
    }
    input_sync(encoder.input);
    }
#[no_mangle]
unsafe extern "C" fn rotary_encoder_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rotary_encoder_irq(int irq, void *dev_id)
    {
    struct rotary_encoder *encoder = dev_id;
    unsigned int state;
    guard(mutex)(&encoder.access_mutex);
    state = rotary_encoder_get_state(encoder);
    switch (state) {
    case 0x0:
    if (encoder.armed) {
    rotary_encoder_report_event(encoder);
    encoder.armed = false;
    }
    break;
    case 0x1:
    case 0x3:
    if (encoder.armed)
    encoder.dir = 2 - state;
    break;
    case 0x2:
    encoder.armed = true;
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rotary_encoder_half_period_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rotary_encoder_half_period_irq(int irq, void *dev_id)
    {
    struct rotary_encoder *encoder = dev_id;
    unsigned int state;
    guard(mutex)(&encoder.access_mutex);
    state = rotary_encoder_get_state(encoder);
    if (state & 1) {
    encoder.dir = ((encoder.last_stable - state + 1) % 4) - 1;
    } else {
    if (state != encoder.last_stable) {
    rotary_encoder_report_event(encoder);
    encoder.last_stable = state;
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rotary_encoder_quarter_period_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rotary_encoder_quarter_period_irq(int irq, void *dev_id)
    {
    struct rotary_encoder *encoder = dev_id;
    unsigned int state;
    guard(mutex)(&encoder.access_mutex);
    state = rotary_encoder_get_state(encoder);
    if ((encoder.last_stable + 1) % 4 == state) {
    encoder.dir = 1;
    rotary_encoder_report_event(encoder);
    } else if (encoder.last_stable == (state + 1) % 4) {
    encoder.dir = -1;
    rotary_encoder_report_event(encoder);
    }
    encoder.last_stable = state;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rotary_encoder_probe(pdev: *mut platform_device) -> c_int {
    static int rotary_encoder_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rotary_encoder *encoder;
    struct input_dev *input;
    irq_handler_t handler;
    u32 steps_per_period;
    unsigned int i;
    int err;
    encoder = devm_kzalloc(dev, sizeof(struct rotary_encoder), GFP_KERNEL);
    if (!encoder)
    return -ENOMEM;
    mutex_init(&encoder.access_mutex);
    device_property_read_u32(dev, "rotary-encoder,steps", &encoder.steps);
    err = device_property_read_u32(dev, "rotary-encoder,steps-per-period",
    &steps_per_period);
    if (err) {
//
// The 'half-period' property has been deprecated, you must
// use 'steps-per-period' and set an appropriate value, but
// we still need to parse it to maintain compatibility. If
// neither property is present we fall back to the one step
// per period behavior.
//
    steps_per_period = device_property_read_bool(dev,
    "rotary-encoder,half-period") ? 2 : 1;
    }
    encoder.rollover =
    device_property_read_bool(dev, "rotary-encoder,rollover");
    if (!device_property_present(dev, "rotary-encoder,encoding") ||
    !device_property_match_string(dev, "rotary-encoder,encoding",
    "gray")) {
    dev_info(dev, "gray");
    encoder.encoding = ROTENC_GRAY;
    } else if (!device_property_match_string(dev, "rotary-encoder,encoding",
    "binary")) {
    dev_info(dev, "binary");
    encoder.encoding = ROTENC_BINARY;
    } else {
    dev_err(dev, "unknown encoding setting\n");
    return -EINVAL;
    }
    device_property_read_u32(dev, "linux,axis", &encoder.axis);
    encoder.relative_axis =
    device_property_read_bool(dev, "rotary-encoder,relative-axis");
    encoder.gpios = devm_gpiod_get_array(dev, core::ptr::null_mut(), GPIOD_IN);
    if (IS_ERR(encoder.gpios))
    return dev_err_probe(dev, PTR_ERR(encoder.gpios), "unable to get gpios\n");
    if (encoder.gpios.ndescs < 2) {
    dev_err(dev, "not enough gpios found\n");
    return -EINVAL;
    }
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    encoder.input = input;
    input.name = pdev.name;
    input.id.bustype = BUS_HOST;
    if (encoder.relative_axis)
    input_set_capability(input, EV_REL, encoder.axis);
    else
    input_set_abs_params(input,
    encoder.axis, 0, encoder.steps, 0, 1);
    switch (steps_per_period >> (encoder.gpios.ndescs - 2)) {
    case 4:
    handler = &rotary_encoder_quarter_period_irq;
    encoder.last_stable = rotary_encoder_get_state(encoder);
    break;
    case 2:
    handler = &rotary_encoder_half_period_irq;
    encoder.last_stable = rotary_encoder_get_state(encoder);
    break;
    case 1:
    handler = &rotary_encoder_irq;
    break;
    default:
    dev_err(dev, "'%d' is not a valid steps-per-period value\n",
    steps_per_period);
    return -EINVAL;
    }
    encoder.irq =
    devm_kcalloc(dev,
    encoder.gpios.ndescs, sizeof(*encoder.irq),
    GFP_KERNEL);
    if (!encoder.irq)
    return -ENOMEM;
    for (i = 0; i < encoder.gpios.ndescs; ++i) {
    encoder.irq[i] = gpiod_to_irq(encoder.gpios.desc[i]);
    err = devm_request_threaded_irq(dev, encoder.irq[i],
    core::ptr::null_mut(), handler,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    DRV_NAME, encoder);
    if (err) {
    dev_err(dev, "unable to request IRQ %d (gpio#%d)\n",
    encoder.irq[i], i);
    return err;
    }
    }
    err = input_register_device(input);
    if (err) {
    dev_err(dev, "failed to register input device\n");
    return err;
    }
    device_init_wakeup(dev,
    device_property_read_bool(dev, "wakeup-source"));
    platform_set_drvdata(pdev, encoder);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rotary_encoder_suspend(dev: *mut device) -> c_int {
    static int rotary_encoder_suspend(struct device *dev)
    {
    struct rotary_encoder *encoder = dev_get_drvdata(dev);
    unsigned int i;
    if (device_may_wakeup(dev)) {
    for (i = 0; i < encoder.gpios.ndescs; ++i)
    enable_irq_wake(encoder.irq[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rotary_encoder_resume(dev: *mut device) -> c_int {
    static int rotary_encoder_resume(struct device *dev)
    {
    struct rotary_encoder *encoder = dev_get_drvdata(dev);
    unsigned int i;
    if (device_may_wakeup(dev)) {
    for (i = 0; i < encoder.gpios.ndescs; ++i)
    disable_irq_wake(encoder.irq[i]);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(rotary_encoder_pm_ops,
    rotary_encoder_suspend, rotary_encoder_resume);

    static const struct of_device_id rotary_encoder_of_match[] = {
    { .compatible = "rotary-encoder", },
    { },
    };
    MODULE_DEVICE_TABLE(of, rotary_encoder_of_match);

    static struct platform_driver rotary_encoder_driver = {
    .probe		= rotary_encoder_probe,
    .driver		= {
    .name	= DRV_NAME,
    .pm	= pm_sleep_ptr(&rotary_encoder_pm_ops),
    .of_match_table = of_match_ptr(rotary_encoder_of_match),
    }
    };
    module_platform_driver(rotary_encoder_driver);
    MODULE_ALIAS("platform:" DRV_NAME);
    MODULE_DESCRIPTION("GPIO rotary encoder driver");
    MODULE_AUTHOR("Daniel Mack <daniel@caiaq.de>, Johan Hovold");
    MODULE_LICENSE("GPL v2");
