//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/st_rc.c
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
// Copyright (C) 2013 STMicroelectronics Limited
// Author: Srinivas Kandagatla <srinivas.kandagatla@st.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_rc_device {
    pub dev: *mut device,
    pub irq: c_int,
    pub irq_wake: c_int,
    pub sys_clock: *mut clk,
    pub /: *mut *mut *mut void __iomem base; / Register base address,
    pub /: *mut *mut *mut void __iomem rx_base;/ RX Register base address,
    pub rdev: *mut rc_dev,
    pub overclocking: bool,
    pub sample_mult: c_int,
    pub sample_div: c_int,
    pub rxuhfmode: bool,
    pub rstc: *mut reset_control,
}

// Registers
pub const IRB_SAMPLE_RATE_COMM: c_uint = 0x64	/* sample freq divisor*/;
pub const IRB_CLOCK_SEL: c_uint = 0x70	/* clock select       */;
pub const IRB_CLOCK_SEL_STATUS: c_uint = 0x74	/* clock status       */;
// IRB IR/UHF receiver registers
pub const IRB_RX_ON: c_uint = 0x40	/* pulse time capture */;

pub const IRB_RX_INT_EN: c_uint = 0x48	/* IRQ enable (R/W)   */;
pub const IRB_RX_INT_STATUS: c_uint = 0x4c	/* IRQ status (R/W)   */;
pub const IRB_RX_EN: c_uint = 0x50	/* Receive enable     */;
pub const IRB_MAX_SYM_PERIOD: c_uint = 0x54	/* max sym value      */;
pub const IRB_RX_INT_CLEAR: c_uint = 0x58	/* overrun status     */;
pub const IRB_RX_STATUS: c_uint = 0x6c	/* receive status     */;
pub const IRB_RX_NOISE_SUPPR: c_uint = 0x5c	/* noise suppression  */;
pub const IRB_RX_POLARITY_INV: c_uint = 0x68	/* polarity inverter  */;
//
// IRQ set: Enable full FIFO                 1  -> bit  3;
// Enable overrun IRQ               1  -> bit  2;
// Enable last symbol IRQ           1  -> bit  1:
// Enable RX interrupt              1  -> bit  0;
//
pub const IRB_RX_INTS: c_uint = 0x0f;
pub const IRB_RX_OVERRUN_INT: c_uint = 0x04;
// maximum symbol period (microsecs),timeout to detect end of symbol train
pub const MAX_SYMB_TIME: c_uint = 0x5000;
pub const IRB_SAMPLE_FREQ: c_int = 10000000;
pub const IRB_FIFO_NOT_EMPTY: c_uint = 0xff00;
pub const IRB_OVERFLOW: c_uint = 0x4;
pub const IRB_TIMEOUT: c_uint = 0xffff;

#[no_mangle]
unsafe extern "C" fn st_rc_send_lirc_timeout(rdev: *mut rc_dev) {
    static void st_rc_send_lirc_timeout(struct rc_dev *rdev)
    {
    let mut ev: ir_raw_event = { .timeout = true, .duration = rdev.timeout };
    ir_raw_event_store(rdev, &ev);
    }
//
// RX graphical example to better understand the difference between ST IR block
// output and standard definition used by LIRC (and most of the world!)
//
// mark                                     mark
// |-IRB_RX_ON-|                            |-IRB_RX_ON-|
// ___  ___  ___                            ___  ___  ___             _
// | |  | |  | |                            | |  | |  | |             |
// | |  | |  | |         space 0            | |  | |  | |   space 1   |
// _____| |__| |__| |____________________________| |__| |__| |_____________|
//
// |--------------- IRB_RX_SYS -------------|------ IRB_RX_SYS -------|
//
// |------------- encoding bit 0 -----------|---- encoding bit 1 -----|
//
// ST hardware returns mark (IRB_RX_ON) and total symbol time (IRB_RX_SYS), so
// convert to standard mark/space we have to calculate space=(IRB_RX_SYS-mark)
// The mark time represents the amount of time the carrier (usually 36-40kHz)
// is detected.The above examples shows Pulse Width Modulation encoding where
// bit 0 is represented by space>mark.
//
#[no_mangle]
unsafe extern "C" fn st_rc_rx_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t st_rc_rx_interrupt(int irq, void *data)
    {
    unsigned long timeout;
    unsigned int symbol, mark = 0;
    struct st_rc_device *dev = data;
    let mut last_symbol: c_int = 0;
    u32 status, int_status;
    let mut ev: ir_raw_event = {};
    if (dev.irq_wake)
    pm_wakeup_event(dev.dev, 0);
// FIXME: is 10ms good enough ?
    timeout = jiffies +  msecs_to_jiffies(10);
    do {
    status  = readl(dev.rx_base + IRB_RX_STATUS);
    if (!(status & (IRB_FIFO_NOT_EMPTY | IRB_OVERFLOW)))
    break;
    int_status = readl(dev.rx_base + IRB_RX_INT_STATUS);
    if (unlikely(int_status & IRB_RX_OVERRUN_INT)) {
// discard the entire collection in case of errors!
    ir_raw_event_overflow(dev.rdev);
    dev_info(dev.dev, "IR RX overrun\n");
    writel(IRB_RX_OVERRUN_INT,
    dev.rx_base + IRB_RX_INT_CLEAR);
    continue;
    }
    symbol = readl(dev.rx_base + IRB_RX_SYS);
    mark = readl(dev.rx_base + IRB_RX_ON);
    if (symbol == IRB_TIMEOUT)
    last_symbol = 1;
// Ignore any noise
    if ((mark > 2) && (symbol > 1)) {
    symbol -= mark;
    if (dev.overclocking) { /* adjustments to timings */
    symbol *= dev.sample_mult;
    symbol /= dev.sample_div;
    mark *= dev.sample_mult;
    mark /= dev.sample_div;
    }
    ev.duration = mark;
    ev.pulse = true;
    ir_raw_event_store(dev.rdev, &ev);
    if (!last_symbol) {
    ev.duration = symbol;
    ev.pulse = false;
    ir_raw_event_store(dev.rdev, &ev);
    } else  {
    st_rc_send_lirc_timeout(dev.rdev);
    }
    }
    last_symbol = 0;
    } while (time_is_after_jiffies(timeout));
    writel(IRB_RX_INTS, dev.rx_base + IRB_RX_INT_CLEAR);
// Empty software fifo
    ir_raw_event_handle(dev.rdev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn st_rc_hardware_init(dev: *mut st_rc_device) -> c_int {
    static int st_rc_hardware_init(struct st_rc_device *dev)
    {
    int ret;
    int baseclock, freqdiff;
    let mut rx_max_symbol_per: c_uint = MAX_SYMB_TIME;
    unsigned int rx_sampling_freq_div;
// Enable the IP
    reset_control_deassert(dev.rstc);
    ret = clk_prepare_enable(dev.sys_clock);
    if (ret) {
    dev_err(dev.dev, "Failed to prepare/enable system clock\n");
    return ret;
    }
    baseclock = clk_get_rate(dev.sys_clock);
// IRB input pins are inverted internally from high to low.
    writel(1, dev.rx_base + IRB_RX_POLARITY_INV);
    rx_sampling_freq_div = baseclock / IRB_SAMPLE_FREQ;
    writel(rx_sampling_freq_div, dev.base + IRB_SAMPLE_RATE_COMM);
    freqdiff = baseclock - (rx_sampling_freq_div * IRB_SAMPLE_FREQ);
    if (freqdiff) { /* over clocking, workout the adjustment factors */
    dev.overclocking = true;
    dev.sample_mult = 1000;
    dev.sample_div = baseclock / (10000 * rx_sampling_freq_div);
    rx_max_symbol_per = (rx_max_symbol_per * 1000)/dev.sample_div;
    }
    writel(rx_max_symbol_per, dev.rx_base + IRB_MAX_SYM_PERIOD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rc_remove(pdev: *mut platform_device) {
    static void st_rc_remove(struct platform_device *pdev)
    {
    struct st_rc_device *rc_dev = platform_get_drvdata(pdev);
    dev_pm_clear_wake_irq(&pdev.dev);
    device_init_wakeup(&pdev.dev, false);
    clk_disable_unprepare(rc_dev.sys_clock);
    rc_unregister_device(rc_dev.rdev);
    rc_free_device(rc_dev.rdev);
    }
#[no_mangle]
unsafe extern "C" fn st_rc_open(rdev: *mut rc_dev) -> c_int {
    static int st_rc_open(struct rc_dev *rdev)
    {
    struct st_rc_device *dev = rdev.priv;
    unsigned long flags;
    local_irq_save(flags);
// enable interrupts and receiver
    writel(IRB_RX_INTS, dev.rx_base + IRB_RX_INT_EN);
    writel(0x01, dev.rx_base + IRB_RX_EN);
    local_irq_restore(flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rc_close(rdev: *mut rc_dev) {
    static void st_rc_close(struct rc_dev *rdev)
    {
    struct st_rc_device *dev = rdev.priv;
// disable interrupts and receiver
    writel(0x00, dev.rx_base + IRB_RX_EN);
    writel(0x00, dev.rx_base + IRB_RX_INT_EN);
    }
#[no_mangle]
unsafe extern "C" fn st_rc_probe(pdev: *mut platform_device) -> c_int {
    static int st_rc_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = -EINVAL;
    struct rc_dev *rdev;
    struct device *dev = &pdev.dev;
    struct st_rc_device *rc_dev;
    struct device_node *np = pdev.dev.of_node;
    const char *rx_mode;
    rc_dev = devm_kzalloc(dev, sizeof(struct st_rc_device), GFP_KERNEL);
    if (!rc_dev)
    return -ENOMEM;
    rdev = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!rdev)
    return -ENOMEM;
    if (np && !of_property_read_string(np, "rx-mode", &rx_mode)) {
    if (!strcmp(rx_mode, "uhf")) {
    rc_dev.rxuhfmode = true;
    } else if (!strcmp(rx_mode, "infrared")) {
    rc_dev.rxuhfmode = false;
    } else {
    dev_err(dev, "Unsupported rx mode [%s]\n", rx_mode);
    goto err;
    }
    } else {
    goto err;
    }
    rc_dev.sys_clock = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(rc_dev.sys_clock)) {
    dev_err(dev, "System clock not found\n");
    ret = PTR_ERR(rc_dev.sys_clock);
    goto err;
    }
    rc_dev.irq = platform_get_irq(pdev, 0);
    if (rc_dev.irq < 0) {
    ret = rc_dev.irq;
    goto err;
    }
    rc_dev.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rc_dev.base)) {
    ret = PTR_ERR(rc_dev.base);
    goto err;
    }
    if (rc_dev.rxuhfmode)
    rc_dev.rx_base = rc_dev.base + 0x40;
    else
    rc_dev.rx_base = rc_dev.base;
    rc_dev.rstc = devm_reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(rc_dev.rstc)) {
    ret = PTR_ERR(rc_dev.rstc);
    goto err;
    }
    rc_dev.dev = dev;
    platform_set_drvdata(pdev, rc_dev);
    ret = st_rc_hardware_init(rc_dev);
    if (ret)
    goto err;
    rdev.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER;
// rx sampling rate is 10Mhz
    rdev.rx_resolution = 100;
    rdev.timeout = MAX_SYMB_TIME;
    rdev.priv = rc_dev;
    rdev.open = st_rc_open;
    rdev.close = st_rc_close;
    rdev.driver_name = IR_ST_NAME;
    rdev.map_name = RC_MAP_EMPTY;
    rdev.device_name = "ST Remote Control Receiver";
    ret = rc_register_device(rdev);
    if (ret < 0)
    goto clkerr;
    rc_dev.rdev = rdev;
    if (devm_request_irq(dev, rc_dev.irq, st_rc_rx_interrupt,
    0, IR_ST_NAME, rc_dev) < 0) {
    dev_err(dev, "IRQ %d register failed\n", rc_dev.irq);
    ret = -EINVAL;
    goto rcerr;
    }
// enable wake via this device
    device_init_wakeup(dev, true);
    dev_pm_set_wake_irq(dev, rc_dev.irq);
//
// for LIRC_MODE_MODE2 or LIRC_MODE_PULSE or LIRC_MODE_RAW
// lircd expects a long space first before a signal train to sync.
//
    st_rc_send_lirc_timeout(rdev);
    dev_info(dev, "setup in %s mode\n", rc_dev.rxuhfmode ? "UHF" : "IR");
    return ret;
    rcerr:
    rc_unregister_device(rdev);
    clkerr:
    clk_disable_unprepare(rc_dev.sys_clock);
    err:
    rc_free_device(rdev);
    dev_err(dev, "Unable to register device (%d)\n", ret);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn st_rc_suspend(dev: *mut device) -> c_int {
    static int st_rc_suspend(struct device *dev)
    {
    struct st_rc_device *rc_dev = dev_get_drvdata(dev);
    if (device_may_wakeup(dev)) {
    if (!enable_irq_wake(rc_dev.irq))
    rc_dev.irq_wake = 1;
    else
    return -EINVAL;
    } else {
    pinctrl_pm_select_sleep_state(dev);
    writel(0x00, rc_dev.rx_base + IRB_RX_EN);
    writel(0x00, rc_dev.rx_base + IRB_RX_INT_EN);
    clk_disable_unprepare(rc_dev.sys_clock);
    reset_control_assert(rc_dev.rstc);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rc_resume(dev: *mut device) -> c_int {
    static int st_rc_resume(struct device *dev)
    {
    int ret;
    struct st_rc_device *rc_dev = dev_get_drvdata(dev);
    struct rc_dev	*rdev = rc_dev.rdev;
    if (rc_dev.irq_wake) {
    disable_irq_wake(rc_dev.irq);
    rc_dev.irq_wake = 0;
    } else {
    pinctrl_pm_select_default_state(dev);
    ret = st_rc_hardware_init(rc_dev);
    if (ret)
    return ret;
    if (rdev.users) {
    writel(IRB_RX_INTS, rc_dev.rx_base + IRB_RX_INT_EN);
    writel(0x01, rc_dev.rx_base + IRB_RX_EN);
    }
    }
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(st_rc_pm_ops, st_rc_suspend, st_rc_resume);

    static const struct of_device_id st_rc_match[] = {
    { .compatible = "st,comms-irb", },
    {},
    };
    MODULE_DEVICE_TABLE(of, st_rc_match);

    static struct platform_driver st_rc_driver = {
    .driver = {
    .name = IR_ST_NAME,
    .of_match_table = of_match_ptr(st_rc_match),
    .pm     = &st_rc_pm_ops,
    },
    .probe = st_rc_probe,
    .remove = st_rc_remove,
    };
    module_platform_driver(st_rc_driver);
    MODULE_DESCRIPTION("RC Transceiver driver for STMicroelectronics platforms");
    MODULE_AUTHOR("STMicroelectronics (R&D) Ltd");
    MODULE_LICENSE("GPL");
