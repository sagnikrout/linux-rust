//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/spear-keyboard.c
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
// SPEAr Keyboard Driver
// Based on omap-keypad driver
//
// Copyright (C) 2010 ST Microelectronics
// Rajeev Kumar <rajeevkumar.linux@gmail.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// Keyboard Registers
pub const MODE_CTL_REG: c_uint = 0x00;
pub const STATUS_REG: c_uint = 0x0C;
pub const DATA_REG: c_uint = 0x10;
pub const INTR_MASK: c_uint = 0x54;
// Register Values
pub const NUM_ROWS: c_int = 16;
pub const NUM_COLS: c_int = 16;
pub const MODE_CTL_PCLK_FREQ_SHIFT: c_int = 9;
pub const MODE_CTL_PCLK_FREQ_MSK: c_uint = 0x7F;

pub const MODE_CTL_KEYNUM_SHIFT: c_int = 6;

pub const DATA_ROW_MASK: c_uint = 0xF0;
pub const DATA_COLUMN_MASK: c_uint = 0x0F;
pub const ROW_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_kbd {
    pub input: *mut input_dev,
    pub io_base: *mut void __iomem,
    pub clk: *mut clk,
    pub irq: c_uint,
    pub mode: u32,
    pub suspended_rate: u32,
    pub mode_ctl_reg: u32,
    pub last_key: c_ushort,
    pub NUM_COLS]: *mut *mut unsigned short keycodes[NUM_ROWS,
    pub irq_wake_enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn spear_kbd_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t spear_kbd_interrupt(int irq, void *dev_id)
    {
    struct spear_kbd *kbd = dev_id;
    struct input_dev *input = kbd.input;
    unsigned int key;
    u32 sts, val;
    sts = readl_relaxed(kbd.io_base + STATUS_REG);
    if (!(sts & STATUS_DATA_AVAIL))
    return IRQ_NONE;
    if (kbd.last_key != KEY_RESERVED) {
    input_report_key(input, kbd.last_key, 0);
    kbd.last_key = KEY_RESERVED;
    }
// following reads active (row, col) pair
    val = readl_relaxed(kbd.io_base + DATA_REG) &
    (DATA_ROW_MASK | DATA_COLUMN_MASK);
    key = kbd.keycodes[val];
    input_event(input, EV_MSC, MSC_SCAN, val);
    input_report_key(input, key, 1);
    input_sync(input);
    kbd.last_key = key;
// clear interrupt
    writel_relaxed(0, kbd.io_base + STATUS_REG);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spear_kbd_open(dev: *mut input_dev) -> c_int {
    static int spear_kbd_open(struct input_dev *dev)
    {
    struct spear_kbd *kbd = input_get_drvdata(dev);
    int error;
    u32 val;
    kbd.last_key = KEY_RESERVED;
    error = clk_enable(kbd.clk);
    if (error)
    return error;
// keyboard rate to be programmed is input clock (in MHz) - 1
    val = clk_get_rate(kbd.clk) / 1000000 - 1;
    val = (val & MODE_CTL_PCLK_FREQ_MSK) << MODE_CTL_PCLK_FREQ_SHIFT;
// program keyboard
    val = MODE_CTL_SCAN_RATE_80 | MODE_CTL_KEYBOARD | val |
    (kbd.mode << MODE_CTL_KEYNUM_SHIFT);
    writel_relaxed(val, kbd.io_base + MODE_CTL_REG);
    writel_relaxed(1, kbd.io_base + STATUS_REG);
// start key scan
    val = readl_relaxed(kbd.io_base + MODE_CTL_REG);
    val |= MODE_CTL_START_SCAN;
    writel_relaxed(val, kbd.io_base + MODE_CTL_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_kbd_close(dev: *mut input_dev) {
    static void spear_kbd_close(struct input_dev *dev)
    {
    struct spear_kbd *kbd = input_get_drvdata(dev);
    u32 val;
// stop key scan
    val = readl_relaxed(kbd.io_base + MODE_CTL_REG);
    val &= ~MODE_CTL_START_SCAN;
    writel_relaxed(val, kbd.io_base + MODE_CTL_REG);
    clk_disable(kbd.clk);
    kbd.last_key = KEY_RESERVED;
    }
#[no_mangle]
unsafe extern "C" fn spear_kbd_probe(pdev: *mut platform_device) -> c_int {
    static int spear_kbd_probe(struct platform_device *pdev)
    {
    struct spear_kbd *kbd;
    struct input_dev *input_dev;
    int irq;
    int error;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    kbd = devm_kzalloc(&pdev.dev, sizeof(*kbd), GFP_KERNEL);
    if (!kbd) {
    dev_err(&pdev.dev, "not enough memory for driver data\n");
    return -ENOMEM;
    }
    error = device_property_read_u32(&pdev.dev, "st,mode", &kbd.mode);
    if (error) {
    dev_err(&pdev.dev, "Invalid or missing mode\n");
    return error;
    }
    device_property_read_u32(&pdev.dev, "suspended_rate", &kbd.suspended_rate);
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev) {
    dev_err(&pdev.dev, "unable to allocate input device\n");
    return -ENOMEM;
    }
    kbd.input = input_dev;
    kbd.irq = irq;
    kbd.io_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(kbd.io_base))
    return PTR_ERR(kbd.io_base);
    kbd.clk = devm_clk_get_prepared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(kbd.clk))
    return PTR_ERR(kbd.clk);
    input_dev.name = "Spear Keyboard";
    input_dev.phys = "keyboard/input0";
    input_dev.id.bustype = BUS_HOST;
    input_dev.id.vendor = 0x0001;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    input_dev.open = spear_kbd_open;
    input_dev.close = spear_kbd_close;
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(), NUM_ROWS, NUM_COLS,
    kbd.keycodes, input_dev);
    if (error) {
    dev_err(&pdev.dev, "Failed to build keymap\n");
    return error;
    }
    if (device_property_read_bool(&pdev.dev, "autorepeat"))
    __set_bit(EV_REP, input_dev.evbit);
    input_set_capability(input_dev, EV_MSC, MSC_SCAN);
    input_set_drvdata(input_dev, kbd);
    error = devm_request_irq(&pdev.dev, irq, spear_kbd_interrupt, 0,
    "keyboard", kbd);
    if (error) {
    dev_err(&pdev.dev, "request_irq failed\n");
    return error;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev, "Unable to register keyboard device\n");
    return error;
    }
    device_init_wakeup(&pdev.dev, 1);
    platform_set_drvdata(pdev, kbd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_kbd_suspend(dev: *mut device) -> c_int {
    static int spear_kbd_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct spear_kbd *kbd = platform_get_drvdata(pdev);
    struct input_dev *input_dev = kbd.input;
    let mut rate: c_uint = 0, mode_ctl_reg, val;
    guard(mutex)(&input_dev.mutex);
// explicitly enable clock as we may program device
    clk_enable(kbd.clk);
    mode_ctl_reg = readl_relaxed(kbd.io_base + MODE_CTL_REG);
    if (device_may_wakeup(&pdev.dev)) {
    if (!enable_irq_wake(kbd.irq))
    kbd.irq_wake_enabled = true;
//
// reprogram the keyboard operating frequency as on some
// platform it may change during system suspended
//
    if (kbd.suspended_rate)
    rate = kbd.suspended_rate / 1000000 - 1;
    else
    rate = clk_get_rate(kbd.clk) / 1000000 - 1;
    val = mode_ctl_reg &
    ~(MODE_CTL_PCLK_FREQ_MSK << MODE_CTL_PCLK_FREQ_SHIFT);
    val |= (rate & MODE_CTL_PCLK_FREQ_MSK)
    << MODE_CTL_PCLK_FREQ_SHIFT;
    writel_relaxed(val, kbd.io_base + MODE_CTL_REG);
    } else {
    if (input_device_enabled(input_dev)) {
    writel_relaxed(mode_ctl_reg & ~MODE_CTL_START_SCAN,
    kbd.io_base + MODE_CTL_REG);
    clk_disable(kbd.clk);
    }
    }
// store current configuration
    if (input_device_enabled(input_dev))
    kbd.mode_ctl_reg = mode_ctl_reg;
// restore previous clk state
    clk_disable(kbd.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_kbd_resume(dev: *mut device) -> c_int {
    static int spear_kbd_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct spear_kbd *kbd = platform_get_drvdata(pdev);
    struct input_dev *input_dev = kbd.input;
    guard(mutex)(&input_dev.mutex);
    if (device_may_wakeup(&pdev.dev)) {
    if (kbd.irq_wake_enabled) {
    kbd.irq_wake_enabled = false;
    disable_irq_wake(kbd.irq);
    }
    } else {
    if (input_device_enabled(input_dev))
    clk_enable(kbd.clk);
    }
// restore current configuration
    if (input_device_enabled(input_dev))
    writel_relaxed(kbd.mode_ctl_reg, kbd.io_base + MODE_CTL_REG);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(spear_kbd_pm_ops,
    spear_kbd_suspend, spear_kbd_resume);

    static const struct of_device_id spear_kbd_id_table[] = {
    { .compatible = "st,spear300-kbd" },
    {}
    };
    MODULE_DEVICE_TABLE(of, spear_kbd_id_table);

    static struct platform_driver spear_kbd_driver = {
    .probe		= spear_kbd_probe,
    .driver		= {
    .name	= "keyboard",
    .pm	= pm_sleep_ptr(&spear_kbd_pm_ops),
    .of_match_table = of_match_ptr(spear_kbd_id_table),
    },
    };
    module_platform_driver(spear_kbd_driver);
    MODULE_AUTHOR("Rajeev Kumar");
    MODULE_DESCRIPTION("SPEAr Keyboard Driver");
    MODULE_LICENSE("GPL");
