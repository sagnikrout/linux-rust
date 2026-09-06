//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/ir-hix5hd2.c
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
// Copyright (c) 2014 Linaro Ltd.
// Copyright (c) 2014 HiSilicon Limited.
//

pub const IR_ENABLE: c_uint = 0x00;
pub const IR_CONFIG: c_uint = 0x04;
pub const CNT_LEADS: c_uint = 0x08;
pub const CNT_LEADE: c_uint = 0x0c;
pub const CNT_SLEADE: c_uint = 0x10;
pub const CNT0_B: c_uint = 0x14;
pub const CNT1_B: c_uint = 0x18;
pub const IR_BUSY: c_uint = 0x1c;
pub const IR_DATAH: c_uint = 0x20;
pub const IR_DATAL: c_uint = 0x24;
pub const IR_INTM: c_uint = 0x28;
pub const IR_INTS: c_uint = 0x2c;
pub const IR_INTC: c_uint = 0x30;
pub const IR_START: c_uint = 0x34;
// interrupt mask

// IR_ENABLE register bits

pub const IR_CFG_WIDTH_MASK: c_uint = 0xffff;
pub const IR_CFG_WIDTH_SHIFT: c_int = 16;
pub const IR_CFG_FORMAT_MASK: c_uint = 0x3;
pub const IR_CFG_FORMAT_SHIFT: c_int = 14;
pub const IR_CFG_INT_LEVEL_MASK: c_uint = 0x3f;
pub const IR_CFG_INT_LEVEL_SHIFT: c_int = 8;
// only support raw mode

pub const IR_CFG_FREQ_MASK: c_uint = 0x7f;
pub const IR_CFG_FREQ_SHIFT: c_int = 0;
pub const IR_CFG_INT_THRESHOLD: c_int = 1;
// symbol start from low to high, symbol stream end at high
pub const IR_CFG_SYMBOL_FMT: c_int = 0;
pub const IR_CFG_SYMBOL_MAXWIDTH: c_uint = 0x3e80;

// Need to set extra bit for enabling IR

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_soc_data {
    pub clk_reg: u32,
    pub flags: u32,
}

    static const struct hix5hd2_soc_data hix5hd2_data = {
    .clk_reg = 0x48,
    };
    static const struct hix5hd2_soc_data hi3796cv300_data = {
    .clk_reg = 0x60,
    .flags = HIX5HD2_FLAG_EXTRA_ENABLE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_ir_priv {
    pub irq: c_int,
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub rdev: *mut rc_dev,
    pub regmap: *mut regmap,
    pub clock: *mut clk,
    pub rate: c_ulong,
    pub socdata: *const hix5hd2_soc_data,
}

#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_clk_enable(dev: *mut hix5hd2_ir_priv, on: bool) -> c_int {
    static int hix5hd2_ir_clk_enable(struct hix5hd2_ir_priv *dev, bool on)
    {
    let mut clk_reg: u32 = dev.socdata.clk_reg;
    u32 val;
    let mut ret: c_int = 0;
    if (dev.regmap) {
    regmap_read(dev.regmap, clk_reg, &val);
    if (on) {
    val &= ~IR_CLK_RESET;
    val |= IR_CLK_ENABLE;
    } else {
    val &= ~IR_CLK_ENABLE;
    val |= IR_CLK_RESET;
    }
    regmap_write(dev.regmap, clk_reg, val);
    } else {
    if (on)
    ret = clk_prepare_enable(dev.clock);
    else
    clk_disable_unprepare(dev.clock);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hix5hd2_ir_enable(priv: *mut hix5hd2_ir_priv) {
    static inline void hix5hd2_ir_enable(struct hix5hd2_ir_priv *priv)
    {
    let mut val: u32 = IR_ENABLE_EN;
    if (priv.socdata.flags & HIX5HD2_FLAG_EXTRA_ENABLE)
    val |= IR_ENABLE_EN_EXTRA;
    writel_relaxed(val, priv.base + IR_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_config(priv: *mut hix5hd2_ir_priv) -> c_int {
    static int hix5hd2_ir_config(struct hix5hd2_ir_priv *priv)
    {
    let mut timeout: c_int = 10000;
    u32 val, rate;
    hix5hd2_ir_enable(priv);
    while (readl_relaxed(priv.base + IR_BUSY)) {
    if (timeout--) {
    udelay(1);
    } else {
    dev_err(priv.dev, "IR_BUSY timeout\n");
    return -ETIMEDOUT;
    }
    }
// Now only support raw mode, with symbol start from low to high
    rate = DIV_ROUND_CLOSEST(priv.rate, 1000000);
    val = IR_CFG_SYMBOL_MAXWIDTH & IR_CFG_WIDTH_MASK << IR_CFG_WIDTH_SHIFT;
    val |= IR_CFG_SYMBOL_FMT & IR_CFG_FORMAT_MASK << IR_CFG_FORMAT_SHIFT;
    val |= (IR_CFG_INT_THRESHOLD - 1) & IR_CFG_INT_LEVEL_MASK
    << IR_CFG_INT_LEVEL_SHIFT;
    val |= IR_CFG_MODE_RAW;
    val |= (rate - 1) & IR_CFG_FREQ_MASK << IR_CFG_FREQ_SHIFT;
    writel_relaxed(val, priv.base + IR_CONFIG);
    writel_relaxed(0x00, priv.base + IR_INTM);
// write arbitrary value to start
    writel_relaxed(0x01, priv.base + IR_START);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_open(rdev: *mut rc_dev) -> c_int {
    static int hix5hd2_ir_open(struct rc_dev *rdev)
    {
    struct hix5hd2_ir_priv *priv = rdev.priv;
    int ret;
    ret = hix5hd2_ir_clk_enable(priv, true);
    if (ret)
    return ret;
    ret = hix5hd2_ir_config(priv);
    if (ret) {
    hix5hd2_ir_clk_enable(priv, false);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_close(rdev: *mut rc_dev) {
    static void hix5hd2_ir_close(struct rc_dev *rdev)
    {
    struct hix5hd2_ir_priv *priv = rdev.priv;
    hix5hd2_ir_clk_enable(priv, false);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_rx_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t hix5hd2_ir_rx_interrupt(int irq, void *data)
    {
    u32 symb_num, symb_val, symb_time;
    u32 data_l, data_h;
    u32 irq_sr, i;
    struct hix5hd2_ir_priv *priv = data;
    irq_sr = readl_relaxed(priv.base + IR_INTS);
    if (irq_sr & INTMS_OVERFLOW) {
//
// we must read IR_DATAL first, then we can clean up
// IR_INTS availably since logic would not clear
// fifo when overflow, drv do the job
//
    ir_raw_event_overflow(priv.rdev);
    symb_num = readl_relaxed(priv.base + IR_DATAH);
    for (i = 0; i < symb_num; i++)
    readl_relaxed(priv.base + IR_DATAL);
    writel_relaxed(INT_CLR_OVERFLOW, priv.base + IR_INTC);
    dev_info(priv.dev, "overflow, level=%d\n",
    IR_CFG_INT_THRESHOLD);
    }
    if ((irq_sr & INTMS_SYMBRCV) || (irq_sr & INTMS_TIMEOUT)) {
    let mut ev: ir_raw_event = {};
    symb_num = readl_relaxed(priv.base + IR_DATAH);
    for (i = 0; i < symb_num; i++) {
    symb_val = readl_relaxed(priv.base + IR_DATAL);
    data_l = ((symb_val & 0xffff) * 10);
    data_h =  ((symb_val >> 16) & 0xffff) * 10;
    symb_time = (data_l + data_h) / 10;
    ev.duration = data_l;
    ev.pulse = true;
    ir_raw_event_store(priv.rdev, &ev);
    if (symb_time < IR_CFG_SYMBOL_MAXWIDTH) {
    ev.duration = data_h;
    ev.pulse = false;
    ir_raw_event_store(priv.rdev, &ev);
    } else {
    ir_raw_event_set_idle(priv.rdev, true);
    }
    }
    if (irq_sr & INTMS_SYMBRCV)
    writel_relaxed(INT_CLR_RCV, priv.base + IR_INTC);
    if (irq_sr & INTMS_TIMEOUT)
    writel_relaxed(INT_CLR_TIMEOUT, priv.base + IR_INTC);
    }
// Empty software fifo
    ir_raw_event_handle(priv.rdev);
    return IRQ_HANDLED;
    }
    static const struct of_device_id hix5hd2_ir_table[] = {
    { .compatible = "hisilicon,hix5hd2-ir", &hix5hd2_data, },
    { .compatible = "hisilicon,hi3796cv300-ir", &hi3796cv300_data, },
    {},
    };
    MODULE_DEVICE_TABLE(of, hix5hd2_ir_table);
#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_probe(pdev: *mut platform_device) -> c_int {
    static int hix5hd2_ir_probe(struct platform_device *pdev)
    {
    struct rc_dev *rdev;
    struct device *dev = &pdev.dev;
    struct hix5hd2_ir_priv *priv;
    struct device_node *node = pdev.dev.of_node;
    const char *map_name;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.socdata = device_get_match_data(dev);
    if (!priv.socdata) {
    dev_err(dev, "Unable to initialize IR data\n");
    return -ENODEV;
    }
    priv.regmap = syscon_regmap_lookup_by_phandle(node,
    "hisilicon,power-syscon");
    if (IS_ERR(priv.regmap)) {
    dev_info(dev, "no power-reg\n");
    priv.regmap = core::ptr::null_mut();
    }
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.irq = platform_get_irq(pdev, 0);
    if (priv.irq < 0)
    return priv.irq;
    rdev = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!rdev)
    return -ENOMEM;
    priv.clock = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clock)) {
    dev_err(dev, "clock not found\n");
    ret = PTR_ERR(priv.clock);
    goto err;
    }
    ret = clk_prepare_enable(priv.clock);
    if (ret)
    goto err;
    priv.rate = clk_get_rate(priv.clock);
    rdev.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER;
    rdev.priv = priv;
    rdev.open = hix5hd2_ir_open;
    rdev.close = hix5hd2_ir_close;
    rdev.driver_name = IR_HIX5HD2_NAME;
    map_name = of_get_property(node, "linux,rc-map-name", core::ptr::null_mut());
    rdev.map_name = map_name ?: RC_MAP_EMPTY;
    rdev.device_name = IR_HIX5HD2_NAME;
    rdev.input_phys = IR_HIX5HD2_NAME "/input0";
    rdev.input_id.bustype = BUS_HOST;
    rdev.input_id.vendor = 0x0001;
    rdev.input_id.product = 0x0001;
    rdev.input_id.version = 0x0100;
    rdev.rx_resolution = 10;
    rdev.timeout = IR_CFG_SYMBOL_MAXWIDTH * 10;
    ret = rc_register_device(rdev);
    if (ret < 0)
    goto clkerr;
    if (devm_request_irq(dev, priv.irq, hix5hd2_ir_rx_interrupt,
    0, pdev.name, priv) < 0) {
    dev_err(dev, "IRQ %d register failed\n", priv.irq);
    ret = -EINVAL;
    goto regerr;
    }
    priv.rdev = rdev;
    priv.dev = dev;
    platform_set_drvdata(pdev, priv);
    return ret;
    regerr:
    rc_unregister_device(rdev);
    clkerr:
    clk_disable_unprepare(priv.clock);
    err:
    rc_free_device(rdev);
    dev_err(dev, "Unable to register device (%d)\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_remove(pdev: *mut platform_device) {
    static void hix5hd2_ir_remove(struct platform_device *pdev)
    {
    struct hix5hd2_ir_priv *priv = platform_get_drvdata(pdev);
    clk_disable_unprepare(priv.clock);
    rc_unregister_device(priv.rdev);
    rc_free_device(priv.rdev);
    }

#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_suspend(dev: *mut device) -> c_int {
    static int hix5hd2_ir_suspend(struct device *dev)
    {
    struct hix5hd2_ir_priv *priv = dev_get_drvdata(dev);
    clk_disable_unprepare(priv.clock);
    hix5hd2_ir_clk_enable(priv, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_ir_resume(dev: *mut device) -> c_int {
    static int hix5hd2_ir_resume(struct device *dev)
    {
    struct hix5hd2_ir_priv *priv = dev_get_drvdata(dev);
    int ret;
    ret = hix5hd2_ir_clk_enable(priv, true);
    if (ret)
    return ret;
    ret = clk_prepare_enable(priv.clock);
    if (ret) {
    hix5hd2_ir_clk_enable(priv, false);
    return ret;
    }
    hix5hd2_ir_enable(priv);
    writel_relaxed(0x00, priv.base + IR_INTM);
    writel_relaxed(0xff, priv.base + IR_INTC);
    writel_relaxed(0x01, priv.base + IR_START);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(hix5hd2_ir_pm_ops, hix5hd2_ir_suspend,
    hix5hd2_ir_resume);
    static struct platform_driver hix5hd2_ir_driver = {
    .driver = {
    .name = IR_HIX5HD2_NAME,
    .of_match_table = hix5hd2_ir_table,
    .pm     = &hix5hd2_ir_pm_ops,
    },
    .probe = hix5hd2_ir_probe,
    .remove = hix5hd2_ir_remove,
    };
    module_platform_driver(hix5hd2_ir_driver);
    MODULE_DESCRIPTION("IR controller driver for hix5hd2 platforms");
    MODULE_AUTHOR("Guoxiong Yan <yanguoxiong@huawei.com>");
    MODULE_LICENSE("GPL v2");
