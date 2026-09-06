//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/ts4800-ts.c
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


//
// Touchscreen driver for the TS-4800 board
//
// Copyright (c) 2015 - Savoir-faire Linux
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// polling interval in ms
pub const POLL_INTERVAL: c_int = 3;
pub const DEBOUNCE_COUNT: c_int = 1;
// sensor values are 12-bit wide

pub const PENDOWN_MASK: c_uint = 0x1;
pub const X_OFFSET: c_uint = 0x0;
pub const Y_OFFSET: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts4800_ts {
    pub input: *mut input_dev,
    pub dev: *mut device,
    pub phys: [c_char; 32],
    pub base: *mut void __iomem,
    pub regmap: *mut regmap,
    pub reg: c_uint,
    pub bit: c_uint,
    pub pendown: bool,
    pub debounce: c_int,
}

#[no_mangle]
unsafe extern "C" fn ts4800_ts_open(input_dev: *mut input_dev) -> c_int {
    static int ts4800_ts_open(struct input_dev *input_dev)
    {
    struct ts4800_ts *ts = input_get_drvdata(input_dev);
    int error;
    ts.pendown = false;
    ts.debounce = DEBOUNCE_COUNT;
    error = regmap_update_bits(ts.regmap, ts.reg, ts.bit, ts.bit);
    if (error) {
    dev_warn(ts.dev, "Failed to enable touchscreen: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts4800_ts_close(input_dev: *mut input_dev) {
    static void ts4800_ts_close(struct input_dev *input_dev)
    {
    struct ts4800_ts *ts = input_get_drvdata(input_dev);
    int ret;
    ret = regmap_update_bits(ts.regmap, ts.reg, ts.bit, 0);
    if (ret)
    dev_warn(ts.dev, "Failed to disable touchscreen\n");
    }
#[no_mangle]
unsafe extern "C" fn ts4800_ts_poll(input_dev: *mut input_dev) {
    static void ts4800_ts_poll(struct input_dev *input_dev)
    {
    struct ts4800_ts *ts = input_get_drvdata(input_dev);
    let mut last_x: u16 = readw(ts.base + X_OFFSET);
    let mut last_y: u16 = readw(ts.base + Y_OFFSET);
    let mut pendown: bool = last_x & PENDOWN_MASK;
    if (pendown) {
    if (ts.debounce) {
    ts.debounce--;
    return;
    }
    if (!ts.pendown) {
    input_report_key(input_dev, BTN_TOUCH, 1);
    ts.pendown = true;
    }
    last_x = ((~last_x) >> 4) & MAX_12BIT;
    last_y = ((~last_y) >> 4) & MAX_12BIT;
    input_report_abs(input_dev, ABS_X, last_x);
    input_report_abs(input_dev, ABS_Y, last_y);
    input_sync(input_dev);
    } else if (ts.pendown) {
    ts.pendown = false;
    ts.debounce = DEBOUNCE_COUNT;
    input_report_key(input_dev, BTN_TOUCH, 0);
    input_sync(input_dev);
    }
    }
    static int ts4800_parse_dt(struct platform_device *pdev,
    struct ts4800_ts *ts)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    u32 reg, bit;
    int error;
    struct device_node *syscon_np __free(device_node) =
    of_parse_phandle(np, "syscon", 0);
    if (!syscon_np) {
    dev_err(dev, "no syscon property\n");
    return -ENODEV;
    }
    ts.regmap = syscon_node_to_regmap(syscon_np);
    if (IS_ERR(ts.regmap)) {
    dev_err(dev, "cannot get parent's regmap\n");
    return PTR_ERR(ts.regmap);
    }
    error = of_property_read_u32_index(np, "syscon", 1, &reg);
    if (error < 0) {
    dev_err(dev, "no offset in syscon\n");
    return error;
    }
    ts.reg = reg;
    error = of_property_read_u32_index(np, "syscon", 2, &bit);
    if (error < 0) {
    dev_err(dev, "no bit in syscon\n");
    return error;
    }
    ts.bit = BIT(bit);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts4800_ts_probe(pdev: *mut platform_device) -> c_int {
    static int ts4800_ts_probe(struct platform_device *pdev)
    {
    struct input_dev *input_dev;
    struct ts4800_ts *ts;
    int error;
    ts = devm_kzalloc(&pdev.dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    error = ts4800_parse_dt(pdev, ts);
    if (error)
    return error;
    ts.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ts.base))
    return PTR_ERR(ts.base);
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    snprintf(ts.phys, sizeof(ts.phys), "%s/input0", dev_name(&pdev.dev));
    ts.input = input_dev;
    ts.dev = &pdev.dev;
    input_set_drvdata(input_dev, ts);
    input_dev.name = "TS-4800 Touchscreen";
    input_dev.phys = ts.phys;
    input_dev.open = ts4800_ts_open;
    input_dev.close = ts4800_ts_close;
    input_set_capability(input_dev, EV_KEY, BTN_TOUCH);
    input_set_abs_params(input_dev, ABS_X, 0, MAX_12BIT, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, MAX_12BIT, 0, 0);
    error = input_setup_polling(input_dev, ts4800_ts_poll);
    if (error) {
    dev_err(&pdev.dev, "Unable to set up polling: %d\n", error);
    return error;
    }
    input_set_poll_interval(input_dev, POLL_INTERVAL);
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev,
    "Unable to register input device: %d\n", error);
    return error;
    }
    return 0;
    }
    static const struct of_device_id ts4800_ts_of_match[] = {
    { .compatible = "technologic,ts4800-ts", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ts4800_ts_of_match);
    static struct platform_driver ts4800_ts_driver = {
    .driver = {
    .name = "ts4800-ts",
    .of_match_table = ts4800_ts_of_match,
    },
    .probe = ts4800_ts_probe,
    };
    module_platform_driver(ts4800_ts_driver);
    MODULE_AUTHOR("Damien Riegel <damien.riegel@savoirfairelinux.com>");
    MODULE_DESCRIPTION("TS-4800 Touchscreen Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:ts4800_ts");
