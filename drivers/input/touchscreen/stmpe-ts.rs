//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/stmpe-ts.c
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
// STMicroelectronics STMPE811 Touchscreen Driver
//
// (C) 2010 Luotao Fu <l.fu@pengutronix.de>
// All rights reserved.
//

// Register layouts and functionalities are identical on all stmpexxx variants
// with touchscreen controller
//
pub const STMPE_REG_INT_STA: c_uint = 0x0B;
pub const STMPE_REG_TSC_CTRL: c_uint = 0x40;
pub const STMPE_REG_TSC_CFG: c_uint = 0x41;
pub const STMPE_REG_FIFO_TH: c_uint = 0x4A;
pub const STMPE_REG_FIFO_STA: c_uint = 0x4B;
pub const STMPE_REG_FIFO_SIZE: c_uint = 0x4C;
pub const STMPE_REG_TSC_DATA_XYZ: c_uint = 0x52;
pub const STMPE_REG_TSC_FRACTION_Z: c_uint = 0x56;
pub const STMPE_REG_TSC_I_DRIVE: c_uint = 0x58;
pub const OP_MOD_XYZ: c_int = 0;

pub const STMPE_IRQ_TOUCH_DET: c_int = 0;

pub const XY_MASK: c_uint = 0xfff;
//
// struct stmpe_touch - stmpe811 touch screen controller state
// @stmpe: pointer back to STMPE MFD container
// @idev: registered input device
// @work: a work item used to scan the device
// @dev: a pointer back to the MFD cell struct device
// @prop: Touchscreen properties
// @ave_ctrl: Sample average control
// (0 -> 1 sample, 1 -> 2 samples, 2 -> 4 samples, 3 -> 8 samples)
// @touch_det_delay: Touch detect interrupt delay
// (0 -> 10 us, 1 -> 50 us, 2 -> 100 us, 3 -> 500 us,
// 4-> 1 ms, 5 -> 5 ms, 6 -> 10 ms, 7 -> 50 ms)
// recommended is 3
// @settling: Panel driver settling time
// (0 -> 10 us, 1 -> 100 us, 2 -> 500 us, 3 -> 1 ms,
// 4 -> 5 ms, 5 -> 10 ms, 6 for 50 ms, 7 -> 100 ms)
// recommended is 2
// @fraction_z: Length of the fractional part in z
// (fraction_z ([0..7]) = Count of the fractional part)
// recommended is 7
// @i_drive: current limit value of the touchscreen drivers
// (0 -> 20 mA typical 35 mA max, 1 -> 50 mA typical 80 mA max)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_touch {
    pub stmpe: *mut stmpe,
    pub idev: *mut input_dev,
    pub work: delayed_work,
    pub dev: *mut device,
    pub prop: touchscreen_properties,
    pub ave_ctrl: u8,
    pub touch_det_delay: u8,
    pub settling: u8,
    pub fraction_z: u8,
    pub i_drive: u8,
}

#[no_mangle]
unsafe extern "C" fn __stmpe_reset_fifo(stmpe: *mut stmpe) -> c_int {
    static int __stmpe_reset_fifo(struct stmpe *stmpe)
    {
    int ret;
    ret = stmpe_set_bits(stmpe, STMPE_REG_FIFO_STA,
    STMPE_FIFO_STA_RESET, STMPE_FIFO_STA_RESET);
    if (ret)
    return ret;
    return stmpe_set_bits(stmpe, STMPE_REG_FIFO_STA,
    STMPE_FIFO_STA_RESET, 0);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_work(work: *mut work_struct) {
    static void stmpe_work(struct work_struct *work)
    {
    int int_sta;
    let mut timeout: u32 = 40;
    struct stmpe_touch *ts =
    container_of(work, struct stmpe_touch, work.work);
    int_sta = stmpe_reg_read(ts.stmpe, STMPE_REG_INT_STA);
//
// touch_det sometimes get desasserted or just get stuck. This appears
// to be a silicon bug, We still have to clarify this with the
// manufacture. As a workaround We release the key anyway if the
// touch_det keeps coming in after 4ms, while the FIFO contains no value
// during the whole time.
//
    while ((int_sta & (1 << STMPE_IRQ_TOUCH_DET)) && (timeout > 0)) {
    timeout--;
    int_sta = stmpe_reg_read(ts.stmpe, STMPE_REG_INT_STA);
    udelay(100);
    }
// reset the FIFO before we report release event
    __stmpe_reset_fifo(ts.stmpe);
    input_report_abs(ts.idev, ABS_PRESSURE, 0);
    input_report_key(ts.idev, BTN_TOUCH, 0);
    input_sync(ts.idev);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_ts_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t stmpe_ts_handler(int irq, void *data)
    {
    u8 data_set[4];
    int x, y, z;
    struct stmpe_touch *ts = data;
//
// Cancel scheduled polling for release if we have new value
// available. Wait if the polling is already running.
//
    cancel_delayed_work_sync(&ts.work);
//
// The FIFO sometimes just crashes and stops generating interrupts. This
// appears to be a silicon bug. We still have to clarify this with
// the manufacture. As a workaround we disable the TSC while we are
// collecting data and flush the FIFO after reading
//
    stmpe_set_bits(ts.stmpe, STMPE_REG_TSC_CTRL,
    STMPE_TSC_CTRL_TSC_EN, 0);
    stmpe_block_read(ts.stmpe, STMPE_REG_TSC_DATA_XYZ, 4, data_set);
    x = (data_set[0] << 4) | (data_set[1] >> 4);
    y = ((data_set[1] & 0xf) << 8) | data_set[2];
    z = data_set[3];
    touchscreen_report_pos(ts.idev, &ts.prop, x, y, false);
    input_report_abs(ts.idev, ABS_PRESSURE, z);
    input_report_key(ts.idev, BTN_TOUCH, 1);
    input_sync(ts.idev);
// flush the FIFO after we have read out our values.
    __stmpe_reset_fifo(ts.stmpe);
// reenable the tsc
    stmpe_set_bits(ts.stmpe, STMPE_REG_TSC_CTRL,
    STMPE_TSC_CTRL_TSC_EN, STMPE_TSC_CTRL_TSC_EN);
// start polling for touch_det to detect release
    schedule_delayed_work(&ts.work, msecs_to_jiffies(50));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_init_hw(ts: *mut stmpe_touch) -> c_int {
    static int stmpe_init_hw(struct stmpe_touch *ts)
    {
    int ret;
    u8 tsc_cfg, tsc_cfg_mask;
    struct stmpe *stmpe = ts.stmpe;
    struct device *dev = ts.dev;
    ret = stmpe_enable(stmpe, STMPE_BLOCK_TOUCHSCREEN | STMPE_BLOCK_ADC);
    if (ret) {
    dev_err(dev, "Could not enable clock for ADC and TS\n");
    return ret;
    }
    ret = stmpe811_adc_common_init(stmpe);
    if (ret) {
    stmpe_disable(stmpe, STMPE_BLOCK_TOUCHSCREEN | STMPE_BLOCK_ADC);
    return ret;
    }
    tsc_cfg = STMPE_AVE_CTRL(ts.ave_ctrl) |
    STMPE_DET_DELAY(ts.touch_det_delay) |
    STMPE_SETTLING(ts.settling);
    tsc_cfg_mask = STMPE_AVE_CTRL(0xff) | STMPE_DET_DELAY(0xff) |
    STMPE_SETTLING(0xff);
    ret = stmpe_set_bits(stmpe, STMPE_REG_TSC_CFG, tsc_cfg_mask, tsc_cfg);
    if (ret) {
    dev_err(dev, "Could not config touch\n");
    return ret;
    }
    ret = stmpe_set_bits(stmpe, STMPE_REG_TSC_FRACTION_Z,
    STMPE_FRACTION_Z(0xff), STMPE_FRACTION_Z(ts.fraction_z));
    if (ret) {
    dev_err(dev, "Could not config touch\n");
    return ret;
    }
    ret = stmpe_set_bits(stmpe, STMPE_REG_TSC_I_DRIVE,
    STMPE_I_DRIVE(0xff), STMPE_I_DRIVE(ts.i_drive));
    if (ret) {
    dev_err(dev, "Could not config touch\n");
    return ret;
    }
// set FIFO to 1 for single point reading
    ret = stmpe_reg_write(stmpe, STMPE_REG_FIFO_TH, 1);
    if (ret) {
    dev_err(dev, "Could not set FIFO\n");
    return ret;
    }
    ret = stmpe_set_bits(stmpe, STMPE_REG_TSC_CTRL,
    STMPE_OP_MODE(0xff), STMPE_OP_MODE(OP_MOD_XYZ));
    if (ret) {
    dev_err(dev, "Could not set mode\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_ts_open(dev: *mut input_dev) -> c_int {
    static int stmpe_ts_open(struct input_dev *dev)
    {
    struct stmpe_touch *ts = input_get_drvdata(dev);
    let mut ret: c_int = 0;
    ret = __stmpe_reset_fifo(ts.stmpe);
    if (ret)
    return ret;
    return stmpe_set_bits(ts.stmpe, STMPE_REG_TSC_CTRL,
    STMPE_TSC_CTRL_TSC_EN, STMPE_TSC_CTRL_TSC_EN);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_ts_close(dev: *mut input_dev) {
    static void stmpe_ts_close(struct input_dev *dev)
    {
    struct stmpe_touch *ts = input_get_drvdata(dev);
    cancel_delayed_work_sync(&ts.work);
    stmpe_set_bits(ts.stmpe, STMPE_REG_TSC_CTRL,
    STMPE_TSC_CTRL_TSC_EN, 0);
    }
    static void stmpe_ts_get_platform_info(struct platform_device *pdev,
    struct stmpe_touch *ts)
    {
    struct device_node *np = pdev.dev.of_node;
    u32 val;
    if (np) {
    if (!of_property_read_u32(np, "st,sample-time", &val))
    ts.stmpe.sample_time = val;
    if (!of_property_read_u32(np, "st,mod-12b", &val))
    ts.stmpe.mod_12b = val;
    if (!of_property_read_u32(np, "st,ref-sel", &val))
    ts.stmpe.ref_sel = val;
    if (!of_property_read_u32(np, "st,adc-freq", &val))
    ts.stmpe.adc_freq = val;
    if (!of_property_read_u32(np, "st,ave-ctrl", &val))
    ts.ave_ctrl = val;
    if (!of_property_read_u32(np, "st,touch-det-delay", &val))
    ts.touch_det_delay = val;
    if (!of_property_read_u32(np, "st,settling", &val))
    ts.settling = val;
    if (!of_property_read_u32(np, "st,fraction-z", &val))
    ts.fraction_z = val;
    if (!of_property_read_u32(np, "st,i-drive", &val))
    ts.i_drive = val;
    }
    }
#[no_mangle]
unsafe extern "C" fn stmpe_input_probe(pdev: *mut platform_device) -> c_int {
    static int stmpe_input_probe(struct platform_device *pdev)
    {
    struct stmpe *stmpe = dev_get_drvdata(pdev.dev.parent);
    struct stmpe_touch *ts;
    struct input_dev *idev;
    int error;
    int ts_irq;
    ts_irq = platform_get_irq_byname(pdev, "FIFO_TH");
    if (ts_irq < 0)
    return ts_irq;
    ts = devm_kzalloc(&pdev.dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    idev = devm_input_allocate_device(&pdev.dev);
    if (!idev)
    return -ENOMEM;
    platform_set_drvdata(pdev, ts);
    ts.stmpe = stmpe;
    ts.idev = idev;
    ts.dev = &pdev.dev;
    stmpe_ts_get_platform_info(pdev, ts);
    INIT_DELAYED_WORK(&ts.work, stmpe_work);
    error = devm_request_threaded_irq(&pdev.dev, ts_irq,
    core::ptr::null_mut(), stmpe_ts_handler,
    IRQF_ONESHOT, STMPE_TS_NAME, ts);
    if (error) {
    dev_err(&pdev.dev, "Failed to request IRQ %d\n", ts_irq);
    return error;
    }
    error = stmpe_init_hw(ts);
    if (error)
    return error;
    idev.name = STMPE_TS_NAME;
    idev.phys = STMPE_TS_NAME"/input0";
    idev.id.bustype = BUS_I2C;
    idev.open = stmpe_ts_open;
    idev.close = stmpe_ts_close;
    input_set_drvdata(idev, ts);
    input_set_capability(idev, EV_KEY, BTN_TOUCH);
    input_set_abs_params(idev, ABS_X, 0, XY_MASK, 0, 0);
    input_set_abs_params(idev, ABS_Y, 0, XY_MASK, 0, 0);
    input_set_abs_params(idev, ABS_PRESSURE, 0x0, 0xff, 0, 0);
    touchscreen_parse_properties(idev, false, &ts.prop);
    error = input_register_device(idev);
    if (error) {
    dev_err(&pdev.dev, "Could not register input device\n");
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_ts_remove(pdev: *mut platform_device) {
    static void stmpe_ts_remove(struct platform_device *pdev)
    {
    struct stmpe_touch *ts = platform_get_drvdata(pdev);
    stmpe_disable(ts.stmpe, STMPE_BLOCK_TOUCHSCREEN);
    }
    static struct platform_driver stmpe_ts_driver = {
    .driver = {
    .name = STMPE_TS_NAME,
    },
    .probe = stmpe_input_probe,
    .remove = stmpe_ts_remove,
    };
    module_platform_driver(stmpe_ts_driver);
    MODULE_ALIAS("platform:stmpe-ts");
    MODULE_AUTHOR("Luotao Fu <l.fu@pengutronix.de>");
    MODULE_DESCRIPTION("STMPEXXX touchscreen driver");
    MODULE_LICENSE("GPL");
