//! Automatically rewritten from C to Rust
//! Source: drivers/input/rmi4/rmi_f3a.c
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
// Copyright (c) 2012-2020 Synaptics Incorporated
//

pub const RMI_F3A_MAX_GPIO_COUNT: c_int = 128;

// Defs for Query 0
pub const RMI_F3A_GPIO_COUNT: c_uint = 0x7F;

pub const TRACKSTICK_RANGE_START: c_int = 3;
pub const TRACKSTICK_RANGE_END: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f3a_data {
// Query Data
    pub gpio_count: u8,
    pub register_count: u8,
    pub data_regs: [u8; RMI_F3A_DATA_REGS_MAX_SIZE],
    pub gpio_key_map: *mut u16,
    pub input: *mut input_dev,
    pub f03: *mut rmi_function,
    pub trackstick_buttons: bool,
}

    static void rmi_f3a_report_button(struct rmi_function *fn,
    struct f3a_data *f3a, unsigned int button)
    {
    let mut key_code: u16 = f3a.gpio_key_map[button];
    let mut key_down: bool = !(f3a.data_regs[0] & BIT(button));
    if (f3a.trackstick_buttons &&
    button >= TRACKSTICK_RANGE_START &&
    button <= TRACKSTICK_RANGE_END) {
    rmi_f03_overwrite_button(f3a.f03, key_code, key_down);
    } else {
    rmi_dbg(RMI_DEBUG_FN, &fn.dev,
    "%s: call input report key (0x%04x) value (0x%02x)",
    __func__, key_code, key_down);
    input_report_key(f3a.input, key_code, key_down);
    }
    }
#[no_mangle]
unsafe extern "C" fn rmi_f3a_attention(irq: c_int, ctx: *mut c_void) -> irqreturn_t {
    static irqreturn_t rmi_f3a_attention(int irq, void *ctx)
    {
    struct rmi_function *fn = ctx;
    struct f3a_data *f3a = dev_get_drvdata(&fn.dev);
    struct rmi_driver_data *drvdata = dev_get_drvdata(&fn.rmi_dev.dev);
    int error;
    int i;
    if (drvdata.attn_data.data) {
    if (drvdata.attn_data.size < f3a.register_count) {
    dev_warn(&fn.dev,
    "F3A interrupted, but data is missing\n");
    return IRQ_HANDLED;
    }
    memcpy(f3a.data_regs, drvdata.attn_data.data,
    f3a.register_count);
    drvdata.attn_data.data += f3a.register_count;
    drvdata.attn_data.size -= f3a.register_count;
    } else {
    error = rmi_read_block(fn.rmi_dev, fn.fd.data_base_addr,
    f3a.data_regs, f3a.register_count);
    if (error) {
    dev_err(&fn.dev,
    "%s: Failed to read F3a data registers: %d\n",
    __func__, error);
    return IRQ_RETVAL(error);
    }
    }
    for (i = 0; i < f3a.gpio_count; i++)
    if (f3a.gpio_key_map[i] != KEY_RESERVED)
    rmi_f3a_report_button(fn, f3a, i);
    if (f3a.trackstick_buttons)
    rmi_f03_commit_buttons(f3a.f03);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f3a_config(fn: *mut rmi_function) -> c_int {
    static int rmi_f3a_config(struct rmi_function *fn)
    {
    struct f3a_data *f3a = dev_get_drvdata(&fn.dev);
    struct rmi_driver *drv = fn.rmi_dev.driver;
    const struct rmi_device_platform_data *pdata =
    rmi_get_platform_data(fn.rmi_dev);
    if (!f3a)
    return 0;
    if (pdata.gpio_data.trackstick_buttons) {
// Try [re-]establish link to F03.
    f3a.f03 = rmi_find_function(fn.rmi_dev, 0x03);
    f3a.trackstick_buttons = f3a.f03 != core::ptr::null_mut();
    }
    drv.set_irq_bits(fn.rmi_dev, fn.irq_mask);
    return 0;
    }
    static bool rmi_f3a_is_valid_button(int button, struct f3a_data *f3a,
    u8 *query1_regs, u8 *ctrl1_regs)
    {
// gpio exist && direction input
    return (query1_regs[0] & BIT(button)) && !(ctrl1_regs[0] & BIT(button));
    }
    static int rmi_f3a_map_gpios(struct rmi_function *fn, struct f3a_data *f3a,
    u8 *query1_regs, u8 *ctrl1_regs)
    {
    const struct rmi_device_platform_data *pdata =
    rmi_get_platform_data(fn.rmi_dev);
    struct input_dev *input = f3a.input;
    let mut button: c_uint = BTN_LEFT;
    let mut trackstick_button: c_uint = BTN_LEFT;
    let mut button_mapped: bool = false;
    int i;
    let mut button_count: c_int = min_t(u8, f3a.gpio_count, TRACKSTICK_RANGE_END);
    f3a.gpio_key_map = devm_kcalloc(&fn.dev,
    f3a.gpio_count,
    sizeof(f3a.gpio_key_map[0]),
    GFP_KERNEL);
    if (!f3a.gpio_key_map) {
    dev_err(&fn.dev, "Failed to allocate gpio map memory.\n");
    return -ENOMEM;
    }
    for (i = 0; i < button_count; i++) {
    if (!rmi_f3a_is_valid_button(i, f3a, query1_regs, ctrl1_regs))
    continue;
    if (pdata.gpio_data.trackstick_buttons &&
    i >= TRACKSTICK_RANGE_START &&
    i < TRACKSTICK_RANGE_END) {
    f3a.gpio_key_map[i] = trackstick_button++;
    } else if (!pdata.gpio_data.buttonpad || !button_mapped) {
    f3a.gpio_key_map[i] = button;
    input_set_capability(input, EV_KEY, button++);
    button_mapped = true;
    }
    }
    input.keycode = f3a.gpio_key_map;
    input.keycodesize = sizeof(f3a.gpio_key_map[0]);
    input.keycodemax = f3a.gpio_count;
    if (pdata.gpio_data.buttonpad || (button - BTN_LEFT == 1))
    __set_bit(INPUT_PROP_BUTTONPAD, input.propbit);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f3a_initialize(fn: *mut rmi_function, f3a: *mut f3a_data) -> c_int {
    static int rmi_f3a_initialize(struct rmi_function *fn, struct f3a_data *f3a)
    {
    u8 query1[RMI_F3A_MAX_REG_SIZE];
    u8 ctrl1[RMI_F3A_MAX_REG_SIZE];
    u8 buf;
    int error;
    error = rmi_read(fn.rmi_dev, fn.fd.query_base_addr, &buf);
    if (error < 0) {
    dev_err(&fn.dev, "Failed to read general info register: %d\n",
    error);
    return -ENODEV;
    }
    f3a.gpio_count = buf & RMI_F3A_GPIO_COUNT;
    f3a.register_count = DIV_ROUND_UP(f3a.gpio_count, 8);
// Query1 -> gpio exist
    error = rmi_read_block(fn.rmi_dev, fn.fd.query_base_addr + 1,
    query1, f3a.register_count);
    if (error) {
    dev_err(&fn.dev, "Failed to read query1 register\n");
    return error;
    }
// Ctrl1 -> gpio direction
    error = rmi_read_block(fn.rmi_dev, fn.fd.control_base_addr + 1,
    ctrl1, f3a.register_count);
    if (error) {
    dev_err(&fn.dev, "Failed to read control1 register\n");
    return error;
    }
    error = rmi_f3a_map_gpios(fn, f3a, query1, ctrl1);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f3a_probe(fn: *mut rmi_function) -> c_int {
    static int rmi_f3a_probe(struct rmi_function *fn)
    {
    struct rmi_device *rmi_dev = fn.rmi_dev;
    struct rmi_driver_data *drv_data = dev_get_drvdata(&rmi_dev.dev);
    struct f3a_data *f3a;
    int error;
    if (!drv_data.input) {
    dev_info(&fn.dev, "F3A: no input device found, ignoring\n");
    return -ENXIO;
    }
    f3a = devm_kzalloc(&fn.dev, sizeof(*f3a), GFP_KERNEL);
    if (!f3a)
    return -ENOMEM;
    f3a.input = drv_data.input;
    error = rmi_f3a_initialize(fn, f3a);
    if (error)
    return error;
    dev_set_drvdata(&fn.dev, f3a);
    return 0;
    }
    struct rmi_function_handler rmi_f3a_handler = {
    .driver = {
    .name = "rmi4_f3a",
    },
    .func = 0x3a,
    .probe = rmi_f3a_probe,
    .config = rmi_f3a_config,
    .attention = rmi_f3a_attention,
    };
