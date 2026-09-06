//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/st-keyscan.c
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
// STMicroelectronics Key Scanning driver
//
// Copyright (c) 2014 STMicroelectonics Ltd.
// Author: Stuart Menefy <stuart.menefy@st.com>
//
// Based on sh_keysc.c, copyright 2008 Magnus Damm
//

pub const ST_KEYSCAN_MAXKEYS: c_int = 16;
pub const KEYSCAN_CONFIG_OFF: c_uint = 0x0;
pub const KEYSCAN_CONFIG_ENABLE: c_uint = 0x1;
pub const KEYSCAN_DEBOUNCE_TIME_OFF: c_uint = 0x4;
pub const KEYSCAN_MATRIX_STATE_OFF: c_uint = 0x8;
pub const KEYSCAN_MATRIX_DIM_OFF: c_uint = 0xc;
pub const KEYSCAN_MATRIX_DIM_X_SHIFT: c_uint = 0x0;
pub const KEYSCAN_MATRIX_DIM_Y_SHIFT: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_keyscan {
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub clk: *mut clk,
    pub input_dev: *mut input_dev,
    pub last_state: c_ulong,
    pub n_rows: c_uint,
    pub n_cols: c_uint,
    pub debounce_us: c_uint,
}

#[no_mangle]
unsafe extern "C" fn keyscan_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t keyscan_isr(int irq, void *dev_id)
    {
    struct st_keyscan *keypad = dev_id;
    unsigned short *keycode = keypad.input_dev.keycode;
    unsigned long state, change;
    int bit_nr;
    state = readl(keypad.base + KEYSCAN_MATRIX_STATE_OFF) & 0xffff;
    change = keypad.last_state ^ state;
    keypad.last_state = state;
    for_each_set_bit(bit_nr, &change, BITS_PER_LONG)
    input_report_key(keypad.input_dev,
    keycode[bit_nr], state & BIT(bit_nr));
    input_sync(keypad.input_dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn keyscan_start(keypad: *mut st_keyscan) -> c_int {
    static int keyscan_start(struct st_keyscan *keypad)
    {
    int error;
    error = clk_enable(keypad.clk);
    if (error)
    return error;
    writel(keypad.debounce_us * (clk_get_rate(keypad.clk) / 1000000),
    keypad.base + KEYSCAN_DEBOUNCE_TIME_OFF);
    writel(((keypad.n_cols - 1) << KEYSCAN_MATRIX_DIM_X_SHIFT) |
    ((keypad.n_rows - 1) << KEYSCAN_MATRIX_DIM_Y_SHIFT),
    keypad.base + KEYSCAN_MATRIX_DIM_OFF);
    writel(KEYSCAN_CONFIG_ENABLE, keypad.base + KEYSCAN_CONFIG_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keyscan_stop(keypad: *mut st_keyscan) {
    static void keyscan_stop(struct st_keyscan *keypad)
    {
    writel(0, keypad.base + KEYSCAN_CONFIG_OFF);
    clk_disable(keypad.clk);
    }
#[no_mangle]
unsafe extern "C" fn keyscan_open(dev: *mut input_dev) -> c_int {
    static int keyscan_open(struct input_dev *dev)
    {
    struct st_keyscan *keypad = input_get_drvdata(dev);
    return keyscan_start(keypad);
    }
#[no_mangle]
unsafe extern "C" fn keyscan_close(dev: *mut input_dev) {
    static void keyscan_close(struct input_dev *dev)
    {
    struct st_keyscan *keypad = input_get_drvdata(dev);
    keyscan_stop(keypad);
    }
#[no_mangle]
unsafe extern "C" fn keypad_matrix_key_parse_dt(keypad_data: *mut st_keyscan) -> c_int {
    static int keypad_matrix_key_parse_dt(struct st_keyscan *keypad_data)
    {
    struct device *dev = keypad_data.input_dev.dev.parent;
    struct device_node *np = dev.of_node;
    int error;
    error = matrix_keypad_parse_properties(dev, &keypad_data.n_rows,
    &keypad_data.n_cols);
    if (error) {
    dev_err(dev, "failed to parse keypad params\n");
    return error;
    }
    of_property_read_u32(np, "st,debounce-us", &keypad_data.debounce_us);
    dev_dbg(dev, "n_rows=%d n_col=%d debounce=%d\n",
    keypad_data.n_rows, keypad_data.n_cols,
    keypad_data.debounce_us);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keyscan_probe(pdev: *mut platform_device) -> c_int {
    static int keyscan_probe(struct platform_device *pdev)
    {
    struct st_keyscan *keypad_data;
    struct input_dev *input_dev;
    int error;
    if (!pdev.dev.of_node) {
    dev_err(&pdev.dev, "no DT data present\n");
    return -EINVAL;
    }
    keypad_data = devm_kzalloc(&pdev.dev, sizeof(*keypad_data),
    GFP_KERNEL);
    if (!keypad_data)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev) {
    dev_err(&pdev.dev, "failed to allocate the input device\n");
    return -ENOMEM;
    }
    input_dev.name = pdev.name;
    input_dev.phys = "keyscan-keys/input0";
    input_dev.dev.parent = &pdev.dev;
    input_dev.open = keyscan_open;
    input_dev.close = keyscan_close;
    input_dev.id.bustype = BUS_HOST;
    keypad_data.input_dev = input_dev;
    error = keypad_matrix_key_parse_dt(keypad_data);
    if (error)
    return error;
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    keypad_data.n_rows,
    keypad_data.n_cols,
    core::ptr::null_mut(), input_dev);
    if (error) {
    dev_err(&pdev.dev, "failed to build keymap\n");
    return error;
    }
    input_set_drvdata(input_dev, keypad_data);
    keypad_data.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(keypad_data.base))
    return PTR_ERR(keypad_data.base);
    keypad_data.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(keypad_data.clk)) {
    dev_err(&pdev.dev, "cannot get clock\n");
    return PTR_ERR(keypad_data.clk);
    }
    error = clk_enable(keypad_data.clk);
    if (error) {
    dev_err(&pdev.dev, "failed to enable clock\n");
    return error;
    }
    keyscan_stop(keypad_data);
    keypad_data.irq = platform_get_irq(pdev, 0);
    if (keypad_data.irq < 0)
    return -EINVAL;
    error = devm_request_irq(&pdev.dev, keypad_data.irq, keyscan_isr, 0,
    pdev.name, keypad_data);
    if (error) {
    dev_err(&pdev.dev, "failed to request IRQ\n");
    return error;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return error;
    }
    platform_set_drvdata(pdev, keypad_data);
    device_set_wakeup_capable(&pdev.dev, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keyscan_suspend(dev: *mut device) -> c_int {
    static int keyscan_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct st_keyscan *keypad = platform_get_drvdata(pdev);
    struct input_dev *input = keypad.input_dev;
    guard(mutex)(&input.mutex);
    if (device_may_wakeup(dev))
    enable_irq_wake(keypad.irq);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: input_device_enabled(input)) -> else {
    else if (input_device_enabled(input))
    keyscan_stop(keypad);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keyscan_resume(dev: *mut device) -> c_int {
    static int keyscan_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct st_keyscan *keypad = platform_get_drvdata(pdev);
    struct input_dev *input = keypad.input_dev;
    int error;
    guard(mutex)(&input.mutex);
    if (device_may_wakeup(dev)) {
    disable_irq_wake(keypad.irq);
    } else if (input_device_enabled(input)) {
    error = keyscan_start(keypad);
    if (error)
    return error;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(keyscan_dev_pm_ops,
    keyscan_suspend, keyscan_resume);
    static const struct of_device_id keyscan_of_match[] = {
    { .compatible = "st,sti-keyscan" },
    { },
    };
    MODULE_DEVICE_TABLE(of, keyscan_of_match);
    static struct platform_driver keyscan_device_driver = {
    .probe		= keyscan_probe,
    .driver		= {
    .name	= "st-keyscan",
    .pm	= pm_sleep_ptr(&keyscan_dev_pm_ops),
    .of_match_table = keyscan_of_match,
    }
    };
    module_platform_driver(keyscan_device_driver);
    MODULE_AUTHOR("Stuart Menefy <stuart.menefy@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics keyscan device driver");
    MODULE_LICENSE("GPL");
