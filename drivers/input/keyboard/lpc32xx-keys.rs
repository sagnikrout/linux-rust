//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/lpc32xx-keys.c
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
// NXP LPC32xx SoC Key Scan Interface
//
// Authors:
// Kevin Wells <kevin.wells@nxp.com>
// Roland Stigge <stigge@antcom.de>
//
// Copyright (C) 2010 NXP Semiconductors
// Copyright (C) 2012 Roland Stigge
//
// This controller supports square key matrices from 1x1 up to 8x8
//

//
// Key scanner register offsets
//

pub const LPC32XX_KSCAN_SCOND_IN_IDLE: c_uint = 0x0;
pub const LPC32XX_KSCAN_SCOND_IN_SCANONCE: c_uint = 0x1;
pub const LPC32XX_KSCAN_SCOND_IN_IRQGEN: c_uint = 0x2;
pub const LPC32XX_KSCAN_SCOND_IN_SCAN_MATRIX: c_uint = 0x3;
pub const LPC32XX_KSCAN_IRQ_PENDING_CLR: c_uint = 0x1;

pub const LPC32XX_KSCAN_FTST_FORCESCANONCE: c_uint = 0x1;
pub const LPC32XX_KSCAN_FTST_USE32K_CLK: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_kscan_drv {
    pub input: *mut input_dev,
    pub clk: *mut clk,
    pub kscan_base: *mut void __iomem,
    pub /: *mut *mut u32 matrix_sz; / Size of matrix in XxY, ie. 3 = 3x3,
    pub /: *mut *mut u32 deb_clks; / Debounce clocks (based on 32KHz clock),
    pub /: *mut *mut u32 scan_delay; / Scan delay (based on 32KHz clock),
    pub row_shift: c_uint,
    pub /: *mut *mut *mut unsigned short keymap; / Pointer to key map for the scan matrix,
    pub lastkeystates: [u8; 8],
}

#[no_mangle]
unsafe extern "C" fn lpc32xx_mod_states(kscandat: *mut lpc32xx_kscan_drv, col: c_int) {
    static void lpc32xx_mod_states(struct lpc32xx_kscan_drv *kscandat, int col)
    {
    struct input_dev *input = kscandat.input;
    unsigned row, changed, scancode, keycode;
    u8 key;
    key = readl(LPC32XX_KS_DATA(kscandat.kscan_base, col));
    changed = key ^ kscandat.lastkeystates[col];
    kscandat.lastkeystates[col] = key;
    for (row = 0; changed; row++, changed >>= 1) {
    if (changed & 1) {
// Key state changed, signal an event
    scancode = MATRIX_SCAN_CODE(row, col,
    kscandat.row_shift);
    keycode = kscandat.keymap[scancode];
    input_event(input, EV_MSC, MSC_SCAN, scancode);
    input_report_key(input, keycode, key & (1 << row));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_kscan_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc32xx_kscan_irq(int irq, void *dev_id)
    {
    struct lpc32xx_kscan_drv *kscandat = dev_id;
    int i;
    for (i = 0; i < kscandat.matrix_sz; i++)
    lpc32xx_mod_states(kscandat, i);
    writel(1, LPC32XX_KS_IRQ(kscandat.kscan_base));
    input_sync(kscandat.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_kscan_open(dev: *mut input_dev) -> c_int {
    static int lpc32xx_kscan_open(struct input_dev *dev)
    {
    struct lpc32xx_kscan_drv *kscandat = input_get_drvdata(dev);
    int error;
    error = clk_prepare_enable(kscandat.clk);
    if (error)
    return error;
    writel(1, LPC32XX_KS_IRQ(kscandat.kscan_base));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_kscan_close(dev: *mut input_dev) {
    static void lpc32xx_kscan_close(struct input_dev *dev)
    {
    struct lpc32xx_kscan_drv *kscandat = input_get_drvdata(dev);
    writel(1, LPC32XX_KS_IRQ(kscandat.kscan_base));
    clk_disable_unprepare(kscandat.clk);
    }
    static int lpc32xx_parse_dt(struct device *dev,
    struct lpc32xx_kscan_drv *kscandat)
    {
    struct device_node *np = dev.of_node;
    let mut rows: u32 = 0, columns = 0;
    int err;
    err = matrix_keypad_parse_properties(dev, &rows, &columns);
    if (err)
    return err;
    if (rows != columns) {
    dev_err(dev, "rows and columns must be equal!\n");
    return -EINVAL;
    }
    kscandat.matrix_sz = rows;
    kscandat.row_shift = get_count_order(columns);
    of_property_read_u32(np, "nxp,debounce-delay-ms", &kscandat.deb_clks);
    of_property_read_u32(np, "nxp,scan-delay-ms", &kscandat.scan_delay);
    if (!kscandat.deb_clks || !kscandat.scan_delay) {
    dev_err(dev, "debounce or scan delay not specified\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_kscan_probe(pdev: *mut platform_device) -> c_int {
    static int lpc32xx_kscan_probe(struct platform_device *pdev)
    {
    struct lpc32xx_kscan_drv *kscandat;
    struct input_dev *input;
    size_t keymap_size;
    int error;
    int irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    kscandat = devm_kzalloc(&pdev.dev, sizeof(*kscandat),
    GFP_KERNEL);
    if (!kscandat)
    return -ENOMEM;
    error = lpc32xx_parse_dt(&pdev.dev, kscandat);
    if (error) {
    dev_err(&pdev.dev, "failed to parse device tree\n");
    return error;
    }
    keymap_size = sizeof(kscandat.keymap[0]) *
    (kscandat.matrix_sz << kscandat.row_shift);
    kscandat.keymap = devm_kzalloc(&pdev.dev, keymap_size, GFP_KERNEL);
    if (!kscandat.keymap)
    return -ENOMEM;
    kscandat.input = input = devm_input_allocate_device(&pdev.dev);
    if (!input) {
    dev_err(&pdev.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
// Setup key input
    input.name		= pdev.name;
    input.phys		= "lpc32xx/input0";
    input.id.vendor	= 0x0001;
    input.id.product	= 0x0001;
    input.id.version	= 0x0100;
    input.open		= lpc32xx_kscan_open;
    input.close		= lpc32xx_kscan_close;
    input.dev.parent	= &pdev.dev;
    input_set_capability(input, EV_MSC, MSC_SCAN);
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    kscandat.matrix_sz,
    kscandat.matrix_sz,
    kscandat.keymap, kscandat.input);
    if (error) {
    dev_err(&pdev.dev, "failed to build keymap\n");
    return error;
    }
    input_set_drvdata(kscandat.input, kscandat);
    kscandat.kscan_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(kscandat.kscan_base))
    return PTR_ERR(kscandat.kscan_base);
// Get the key scanner clock
    kscandat.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(kscandat.clk)) {
    dev_err(&pdev.dev, "failed to get clock\n");
    return PTR_ERR(kscandat.clk);
    }
// Configure the key scanner
    error = clk_prepare_enable(kscandat.clk);
    if (error)
    return error;
    writel(kscandat.deb_clks, LPC32XX_KS_DEB(kscandat.kscan_base));
    writel(kscandat.scan_delay, LPC32XX_KS_SCAN_CTL(kscandat.kscan_base));
    writel(LPC32XX_KSCAN_FTST_USE32K_CLK,
    LPC32XX_KS_FAST_TST(kscandat.kscan_base));
    writel(kscandat.matrix_sz,
    LPC32XX_KS_MATRIX_DIM(kscandat.kscan_base));
    writel(1, LPC32XX_KS_IRQ(kscandat.kscan_base));
    clk_disable_unprepare(kscandat.clk);
    error = devm_request_irq(&pdev.dev, irq, lpc32xx_kscan_irq, 0,
    pdev.name, kscandat);
    if (error) {
    dev_err(&pdev.dev, "failed to request irq\n");
    return error;
    }
    error = input_register_device(kscandat.input);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return error;
    }
    platform_set_drvdata(pdev, kscandat);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_kscan_suspend(dev: *mut device) -> c_int {
    static int lpc32xx_kscan_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct lpc32xx_kscan_drv *kscandat = platform_get_drvdata(pdev);
    struct input_dev *input = kscandat.input;
    guard(mutex)(&input.mutex);
    if (input_device_enabled(input)) {
// Clear IRQ and disable clock
    writel(1, LPC32XX_KS_IRQ(kscandat.kscan_base));
    clk_disable_unprepare(kscandat.clk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_kscan_resume(dev: *mut device) -> c_int {
    static int lpc32xx_kscan_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct lpc32xx_kscan_drv *kscandat = platform_get_drvdata(pdev);
    struct input_dev *input = kscandat.input;
    int error;
    guard(mutex)(&input.mutex);
    if (input_device_enabled(input)) {
// Enable clock and clear IRQ
    error = clk_prepare_enable(kscandat.clk);
    if (error)
    return error;
    writel(1, LPC32XX_KS_IRQ(kscandat.kscan_base));
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(lpc32xx_kscan_pm_ops, lpc32xx_kscan_suspend,
    lpc32xx_kscan_resume);
    static const struct of_device_id lpc32xx_kscan_match[] = {
    { .compatible = "nxp,lpc3220-key" },
    {},
    };
    MODULE_DEVICE_TABLE(of, lpc32xx_kscan_match);
    static struct platform_driver lpc32xx_kscan_driver = {
    .probe		= lpc32xx_kscan_probe,
    .driver		= {
    .name	= DRV_NAME,
    .pm	= pm_sleep_ptr(&lpc32xx_kscan_pm_ops),
    .of_match_table = lpc32xx_kscan_match,
    }
    };
    module_platform_driver(lpc32xx_kscan_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Kevin Wells <kevin.wells@nxp.com>");
    MODULE_AUTHOR("Roland Stigge <stigge@antcom.de>");
    MODULE_DESCRIPTION("Key scanner driver for LPC32XX devices");
