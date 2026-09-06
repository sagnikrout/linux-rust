//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/bcm_iproc_tsc.c
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
// Copyright (C) 2015 Broadcom Corporation
//

pub const PEN_DOWN_STATUS: c_int = 1;
pub const PEN_UP_STATUS: c_int = 0;
pub const X_MIN: c_int = 0;
pub const Y_MIN: c_int = 0;
pub const X_MAX: c_uint = 0xFFF;
pub const Y_MAX: c_uint = 0xFFF;
// Value given by controller for invalid coordinate.
pub const INVALID_COORD: c_uint = 0xFFFFFFFF;
// Register offsets
pub const REGCTL1: c_uint = 0x00;
pub const REGCTL2: c_uint = 0x04;
pub const INTERRUPT_THRES: c_uint = 0x08;
pub const INTERRUPT_MASK: c_uint = 0x0c;
pub const INTERRUPT_STATUS: c_uint = 0x10;
pub const CONTROLLER_STATUS: c_uint = 0x14;
pub const FIFO_DATA: c_uint = 0x18;
pub const FIFO_DATA_X_Y_MASK: c_uint = 0xFFFF;
pub const ANALOG_CONTROL: c_uint = 0x1c;
pub const AUX_DATA: c_uint = 0x20;
pub const DEBOUNCE_CNTR_STAT: c_uint = 0x24;
pub const SCAN_CNTR_STAT: c_uint = 0x28;
pub const REM_CNTR_STAT: c_uint = 0x2c;
pub const SETTLING_TIMER_STAT: c_uint = 0x30;
pub const SPARE_REG: c_uint = 0x34;
pub const SOFT_BYPASS_CONTROL: c_uint = 0x38;
pub const SOFT_BYPASS_DATA: c_uint = 0x3c;
// Bit values for INTERRUPT_MASK and INTERRUPT_STATUS regs

// Bit values for CONTROLLER_STATUS reg1

// Shift values for control reg1
pub const SCANNING_PERIOD_SHIFT: c_int = 24;
pub const DEBOUNCE_TIMEOUT_SHIFT: c_int = 16;
pub const SETTLING_TIMEOUT_SHIFT: c_int = 8;
pub const TOUCH_TIMEOUT_SHIFT: c_int = 0;
// Shift values for coordinates from fifo
pub const X_COORD_SHIFT: c_int = 0;
pub const Y_COORD_SHIFT: c_int = 16;
// Bit values for REGCTL2

pub const TS_CONTROLLER_AVGDATA_SHIFT: c_int = 8;

    do { \
    u32 val; \
    regmap_read(priv.regmap, reg, &val); \
    dev_dbg(dev, "%20s= 0x%08x\n", #reg, val); \
    } while (0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsc_param {
// Each step is 1024 us.  Valid 1-256
    pub scanning_period: u32,
// Each step is 512 us.  Valid 0-255
    pub debounce_timeout: u32,
//
// The settling duration (in ms) is the amount of time the tsc
// waits to allow the voltage to settle after turning on the
// drivers in detection mode. Valid values: 0-11
// 0 =  0.008 ms
// 1 =  0.01 ms
// 2 =  0.02 ms
// 3 =  0.04 ms
// 4 =  0.08 ms
// 5 =  0.16 ms
// 6 =  0.32 ms
// 7 =  0.64 ms
// 8 =  1.28 ms
// 9 =  2.56 ms
// 10 = 5.12 ms
// 11 = 10.24 ms
//
    pub settling_timeout: u32,
// touch timeout in sample counts
    pub touch_timeout: u32,
//
// Number of data samples which are averaged before a final data point
// is placed into the FIFO
//
    pub average_data: u32,
// FIFO threshold
    pub fifo_threshold: u32,
// Optional standard touchscreen properties.
    pub max_x: u32,
    pub max_y: u32,
    pub fuzz_x: u32,
    pub fuzz_y: u32,
    pub invert_x: bool,
    pub invert_y: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_ts_priv {
    pub pdev: *mut platform_device,
    pub idev: *mut input_dev,
    pub regmap: *mut regmap,
    pub tsc_clk: *mut clk,
    pub pen_status: c_int,
    pub cfg_params: tsc_param,
}

//
// Set default values the same as hardware reset values
// except for fifo_threshold with is set to 1.
//
    static const struct tsc_param iproc_default_config = {
    .scanning_period  = 0x5,  /* 1 to 256 */
    .debounce_timeout = 0x28, /* 0 to 255 */
    .settling_timeout = 0x7,  /* 0 to 11 */
    .touch_timeout    = 0xa,  /* 0 to 255 */
    .average_data     = 5,    /* entry 5 = 32 pts */
    .fifo_threshold   = 1,    /* 0 to 31 */
    .max_x            = X_MAX,
    .max_y            = Y_MAX,
    };
#[no_mangle]
unsafe extern "C" fn ts_reg_dump(priv: *mut iproc_ts_priv) {
    static void ts_reg_dump(struct iproc_ts_priv *priv)
    {
    struct device *dev = &priv.pdev.dev;
    dbg_reg(dev, priv, REGCTL1);
    dbg_reg(dev, priv, REGCTL2);
    dbg_reg(dev, priv, INTERRUPT_THRES);
    dbg_reg(dev, priv, INTERRUPT_MASK);
    dbg_reg(dev, priv, INTERRUPT_STATUS);
    dbg_reg(dev, priv, CONTROLLER_STATUS);
    dbg_reg(dev, priv, FIFO_DATA);
    dbg_reg(dev, priv, ANALOG_CONTROL);
    dbg_reg(dev, priv, AUX_DATA);
    dbg_reg(dev, priv, DEBOUNCE_CNTR_STAT);
    dbg_reg(dev, priv, SCAN_CNTR_STAT);
    dbg_reg(dev, priv, REM_CNTR_STAT);
    dbg_reg(dev, priv, SETTLING_TIMER_STAT);
    dbg_reg(dev, priv, SPARE_REG);
    dbg_reg(dev, priv, SOFT_BYPASS_CONTROL);
    dbg_reg(dev, priv, SOFT_BYPASS_DATA);
    }
#[no_mangle]
unsafe extern "C" fn iproc_touchscreen_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t iproc_touchscreen_interrupt(int irq, void *data)
    {
    struct platform_device *pdev = data;
    struct iproc_ts_priv *priv = platform_get_drvdata(pdev);
    u32 intr_status;
    u32 raw_coordinate;
    u16 x;
    u16 y;
    int i;
    let mut needs_sync: bool = false;
    regmap_read(priv.regmap, INTERRUPT_STATUS, &intr_status);
    intr_status &= TS_PEN_INTR_MASK | TS_FIFO_INTR_MASK;
    if (intr_status == 0)
    return IRQ_NONE;
// Clear all interrupt status bits, write-1-clear
    regmap_write(priv.regmap, INTERRUPT_STATUS, intr_status);
// Pen up/down
    if (intr_status & TS_PEN_INTR_MASK) {
    regmap_read(priv.regmap, CONTROLLER_STATUS, &priv.pen_status);
    if (priv.pen_status & TS_PEN_DOWN)
    priv.pen_status = PEN_DOWN_STATUS;
    else
    priv.pen_status = PEN_UP_STATUS;
    input_report_key(priv.idev, BTN_TOUCH,	priv.pen_status);
    needs_sync = true;
    dev_dbg(&priv.pdev.dev,
    "pen up-down (%d)\n", priv.pen_status);
    }
// coordinates in FIFO exceed the threshold
    if (intr_status & TS_FIFO_INTR_MASK) {
    for (i = 0; i < priv.cfg_params.fifo_threshold; i++) {
    regmap_read(priv.regmap, FIFO_DATA, &raw_coordinate);
    if (raw_coordinate == INVALID_COORD)
    continue;
//
// The x and y coordinate are 16 bits each
// with the x in the lower 16 bits and y in the
// upper 16 bits.
//
    x = (raw_coordinate >> X_COORD_SHIFT) &
    FIFO_DATA_X_Y_MASK;
    y = (raw_coordinate >> Y_COORD_SHIFT) &
    FIFO_DATA_X_Y_MASK;
// We only want to retain the 12 msb of the 16
    x = (x >> 4) & 0x0FFF;
    y = (y >> 4) & 0x0FFF;
// Adjust x y according to LCD tsc mount angle.
    if (priv.cfg_params.invert_x)
    x = priv.cfg_params.max_x - x;
    if (priv.cfg_params.invert_y)
    y = priv.cfg_params.max_y - y;
    input_report_abs(priv.idev, ABS_X, x);
    input_report_abs(priv.idev, ABS_Y, y);
    needs_sync = true;
    dev_dbg(&priv.pdev.dev, "xy (0x%x 0x%x)\n", x, y);
    }
    }
    if (needs_sync)
    input_sync(priv.idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn iproc_ts_start(idev: *mut input_dev) -> c_int {
    static int iproc_ts_start(struct input_dev *idev)
    {
    u32 val;
    u32 mask;
    int error;
    struct iproc_ts_priv *priv = input_get_drvdata(idev);
// Enable clock
    error = clk_prepare_enable(priv.tsc_clk);
    if (error) {
    dev_err(&priv.pdev.dev, "%s clk_prepare_enable failed %d\n",
    __func__, error);
    return error;
    }
//
// Interrupt is generated when:
// FIFO reaches the int_th value, and pen event(up/down)
//
    val = TS_PEN_INTR_MASK | TS_FIFO_INTR_MASK;
    regmap_update_bits(priv.regmap, INTERRUPT_MASK, val, val);
    val = priv.cfg_params.fifo_threshold;
    regmap_write(priv.regmap, INTERRUPT_THRES, val);
// Initialize control reg1
    val = 0;
    val |= priv.cfg_params.scanning_period << SCANNING_PERIOD_SHIFT;
    val |= priv.cfg_params.debounce_timeout << DEBOUNCE_TIMEOUT_SHIFT;
    val |= priv.cfg_params.settling_timeout << SETTLING_TIMEOUT_SHIFT;
    val |= priv.cfg_params.touch_timeout << TOUCH_TIMEOUT_SHIFT;
    regmap_write(priv.regmap, REGCTL1, val);
// Try to clear all interrupt status
    val = TS_FIFO_INTR_MASK | TS_PEN_INTR_MASK;
    regmap_update_bits(priv.regmap, INTERRUPT_STATUS, val, val);
// Initialize control reg2
    val = TS_CONTROLLER_EN_BIT | TS_WIRE_MODE_BIT;
    val |= priv.cfg_params.average_data << TS_CONTROLLER_AVGDATA_SHIFT;
    mask = (TS_CONTROLLER_AVGDATA_MASK);
    mask |= (TS_CONTROLLER_PWR_LDO |	/* PWR up LDO */
    TS_CONTROLLER_PWR_ADC |	/* PWR up ADC */
    TS_CONTROLLER_PWR_BGP |	/* PWR up BGP */
    TS_CONTROLLER_PWR_TS);	/* PWR up TS */
    mask |= val;
    regmap_update_bits(priv.regmap, REGCTL2, mask, val);
    ts_reg_dump(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_ts_stop(dev: *mut input_dev) {
    static void iproc_ts_stop(struct input_dev *dev)
    {
    u32 val;
    struct iproc_ts_priv *priv = input_get_drvdata(dev);
//
// Disable FIFO int_th and pen event(up/down)Interrupts only
// as the interrupt mask register is shared between ADC, TS and
// flextimer.
//
    val = TS_PEN_INTR_MASK | TS_FIFO_INTR_MASK;
    regmap_update_bits(priv.regmap, INTERRUPT_MASK, val, 0);
// Only power down touch screen controller
    val = TS_CONTROLLER_PWR_TS;
    regmap_update_bits(priv.regmap, REGCTL2, val, val);
    clk_disable(priv.tsc_clk);
    }
#[no_mangle]
unsafe extern "C" fn iproc_get_tsc_config(dev: *mut device, priv: *mut iproc_ts_priv) -> c_int {
    static int iproc_get_tsc_config(struct device *dev, struct iproc_ts_priv *priv)
    {
    struct device_node *np = dev.of_node;
    u32 val;
    priv.cfg_params = iproc_default_config;
    if (!np)
    return 0;
    if (of_property_read_u32(np, "scanning_period", &val) >= 0) {
    if (val < 1 || val > 256) {
    dev_err(dev, "scanning_period (%u) must be [1-256]\n",
    val);
    return -EINVAL;
    }
    priv.cfg_params.scanning_period = val;
    }
    if (of_property_read_u32(np, "debounce_timeout", &val) >= 0) {
    if (val > 255) {
    dev_err(dev, "debounce_timeout (%u) must be [0-255]\n",
    val);
    return -EINVAL;
    }
    priv.cfg_params.debounce_timeout = val;
    }
    if (of_property_read_u32(np, "settling_timeout", &val) >= 0) {
    if (val > 11) {
    dev_err(dev, "settling_timeout (%u) must be [0-11]\n",
    val);
    return -EINVAL;
    }
    priv.cfg_params.settling_timeout = val;
    }
    if (of_property_read_u32(np, "touch_timeout", &val) >= 0) {
    if (val > 255) {
    dev_err(dev, "touch_timeout (%u) must be [0-255]\n",
    val);
    return -EINVAL;
    }
    priv.cfg_params.touch_timeout = val;
    }
    if (of_property_read_u32(np, "average_data", &val) >= 0) {
    if (val > 8) {
    dev_err(dev, "average_data (%u) must be [0-8]\n", val);
    return -EINVAL;
    }
    priv.cfg_params.average_data = val;
    }
    if (of_property_read_u32(np, "fifo_threshold", &val) >= 0) {
    if (val > 31) {
    dev_err(dev, "fifo_threshold (%u)) must be [0-31]\n",
    val);
    return -EINVAL;
    }
    priv.cfg_params.fifo_threshold = val;
    }
// Parse optional properties.
    of_property_read_u32(np, "touchscreen-size-x", &priv.cfg_params.max_x);
    of_property_read_u32(np, "touchscreen-size-y", &priv.cfg_params.max_y);
    of_property_read_u32(np, "touchscreen-fuzz-x",
    &priv.cfg_params.fuzz_x);
    of_property_read_u32(np, "touchscreen-fuzz-y",
    &priv.cfg_params.fuzz_y);
    priv.cfg_params.invert_x =
    of_property_read_bool(np, "touchscreen-inverted-x");
    priv.cfg_params.invert_y =
    of_property_read_bool(np, "touchscreen-inverted-y");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_ts_probe(pdev: *mut platform_device) -> c_int {
    static int iproc_ts_probe(struct platform_device *pdev)
    {
    struct iproc_ts_priv *priv;
    struct input_dev *idev;
    int irq;
    int error;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// touchscreen controller memory mapped regs via syscon
    priv.regmap = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "ts_syscon");
    if (IS_ERR(priv.regmap)) {
    error = PTR_ERR(priv.regmap);
    dev_err(&pdev.dev, "unable to map I/O memory:%d\n", error);
    return error;
    }
    priv.tsc_clk = devm_clk_get(&pdev.dev, "tsc_clk");
    if (IS_ERR(priv.tsc_clk)) {
    error = PTR_ERR(priv.tsc_clk);
    dev_err(&pdev.dev,
    "failed getting clock tsc_clk: %d\n", error);
    return error;
    }
    priv.pdev = pdev;
    error = iproc_get_tsc_config(&pdev.dev, priv);
    if (error) {
    dev_err(&pdev.dev, "get_tsc_config failed: %d\n", error);
    return error;
    }
    idev = devm_input_allocate_device(&pdev.dev);
    if (!idev) {
    dev_err(&pdev.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    priv.idev = idev;
    priv.pen_status = PEN_UP_STATUS;
// Set input device info
    idev.name = IPROC_TS_NAME;
    idev.dev.parent = &pdev.dev;
    idev.id.bustype = BUS_HOST;
    idev.id.vendor = SERIO_UNKNOWN;
    idev.id.product = 0;
    idev.id.version = 0;
    idev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    __set_bit(BTN_TOUCH, idev.keybit);
    input_set_abs_params(idev, ABS_X, X_MIN, priv.cfg_params.max_x,
    priv.cfg_params.fuzz_x, 0);
    input_set_abs_params(idev, ABS_Y, Y_MIN, priv.cfg_params.max_y,
    priv.cfg_params.fuzz_y, 0);
    idev.open = iproc_ts_start;
    idev.close = iproc_ts_stop;
    input_set_drvdata(idev, priv);
    platform_set_drvdata(pdev, priv);
// get interrupt
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    error = devm_request_irq(&pdev.dev, irq,
    iproc_touchscreen_interrupt,
    IRQF_SHARED, IPROC_TS_NAME, pdev);
    if (error)
    return error;
    error = input_register_device(priv.idev);
    if (error) {
    dev_err(&pdev.dev,
    "failed to register input device: %d\n", error);
    return error;
    }
    return 0;
    }
    static const struct of_device_id iproc_ts_of_match[] = {
    {.compatible = "brcm,iproc-touchscreen", },
    { },
    };
    MODULE_DEVICE_TABLE(of, iproc_ts_of_match);
    static struct platform_driver iproc_ts_driver = {
    .probe = iproc_ts_probe,
    .driver = {
    .name	= IPROC_TS_NAME,
    .of_match_table = iproc_ts_of_match,
    },
    };
    module_platform_driver(iproc_ts_driver);
    MODULE_DESCRIPTION("IPROC Touchscreen driver");
    MODULE_AUTHOR("Broadcom");
    MODULE_LICENSE("GPL v2");
