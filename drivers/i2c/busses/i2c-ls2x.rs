//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-ls2x.c
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
// Loongson-2K/Loongson LS7A I2C controller mode driver
//
// Copyright (C) 2013 Loongson Technology Corporation Limited.
// Copyright (C) 2014-2017 Lemote, Inc.
// Copyright (C) 2018-2022 Loongson Technology Corporation Limited.
//
// Originally written by liushaozong
// Rewritten for mainline by Binbin Zhou <zhoubinbin@loongson.cn>
//

// I2C Registers
pub const I2C_LS2X_PRER_LO: c_uint = 0x0 /* Freq Division Low Byte Register */;
pub const I2C_LS2X_PRER_HI: c_uint = 0x1 /* Freq Division High Byte Register */;
pub const I2C_LS2X_CTR: c_uint = 0x2 /* Control Register */;
pub const I2C_LS2X_TXR: c_uint = 0x3 /* Transport Data Register */;
pub const I2C_LS2X_RXR: c_uint = 0x3 /* Receive Data Register */;
pub const I2C_LS2X_CR: c_uint = 0x4 /* Command Control Register */;
pub const I2C_LS2X_SR: c_uint = 0x4 /* State Register */;
// Command Control Register Bit

// State Register Bit

// Control Register Bit

// The PCLK frequency from LPB

// The default bus frequency, which is an empirical value

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2x_i2c_priv {
    pub adapter: i2c_adapter,
    pub base: *mut void __iomem,
    pub i2c_t: i2c_timings,
    pub cmd_complete: completion,
}

//
// Interrupt service routine.
// This gets called whenever an I2C interrupt occurs.
//
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_isr(this_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ls2x_i2c_isr(int this_irq, void *dev_id)
    {
    struct ls2x_i2c_priv *priv = dev_id;
    if (!(readb(priv.base + I2C_LS2X_SR) & LS2X_SR_IF))
    return IRQ_NONE;
    writeb(LS2X_CR_IACK, priv.base + I2C_LS2X_CR);
    complete(&priv.cmd_complete);
    return IRQ_HANDLED;
    }
//
// The ls2x i2c controller supports standard mode and fast mode, so the
// maximum bus frequency is '400kHz'.
// The bus frequency is set to the empirical value of '33KHz' by default,
// but it can also be taken from ACPI or FDT for compatibility with more
// devices.
//
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_adjust_bus_speed(priv: *mut ls2x_i2c_priv) {
    static void ls2x_i2c_adjust_bus_speed(struct ls2x_i2c_priv *priv)
    {
    u16 val;
    struct i2c_timings *t = &priv.i2c_t;
    struct device *dev = priv.adapter.dev.parent;
    let mut acpi_speed: u32 = i2c_acpi_find_bus_speed(dev);
    i2c_parse_fw_timings(dev, t, false);
    if (acpi_speed || t.bus_freq_hz)
    t.bus_freq_hz = max(t.bus_freq_hz, acpi_speed);
    else
    t.bus_freq_hz = LS2X_I2C_FREQ_STD;
//
// According to the chip manual, we can only access the registers as bytes,
// otherwise the high bits will be truncated.
// So set the I2C frequency with a sequential writeb() instead of writew().
//
    val = LS2X_I2C_PCLK_FREQ / (5 * t.bus_freq_hz) - 1;
    writeb(FIELD_GET(GENMASK(7, 0), val), priv.base + I2C_LS2X_PRER_LO);
    writeb(FIELD_GET(GENMASK(15, 8), val), priv.base + I2C_LS2X_PRER_HI);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_init(priv: *mut ls2x_i2c_priv) {
    static void ls2x_i2c_init(struct ls2x_i2c_priv *priv)
    {
// Set i2c frequency setting mode and disable interrupts.
    writeb(readb(priv.base + I2C_LS2X_CTR) & ~CTR_FREQ_MASK,
    priv.base + I2C_LS2X_CTR);
    ls2x_i2c_adjust_bus_speed(priv);
// Set i2c normal operating mode and enable interrupts.
    writeb(readb(priv.base + I2C_LS2X_CTR) | CTR_READY_MASK,
    priv.base + I2C_LS2X_CTR);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_xfer_byte(priv: *mut ls2x_i2c_priv, txdata: u8, rxdatap: *mut u8) -> c_int {
    static int ls2x_i2c_xfer_byte(struct ls2x_i2c_priv *priv, u8 txdata, u8 *rxdatap)
    {
    u8 rxdata;
    unsigned long time_left;
    writeb(txdata, priv.base + I2C_LS2X_CR);
    time_left = wait_for_completion_timeout(&priv.cmd_complete,
    priv.adapter.timeout);
    if (!time_left)
    return -ETIMEDOUT;
    rxdata = readb(priv.base + I2C_LS2X_SR);
    if (rxdatap)
// rxdatap = rxdata;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_send_byte(priv: *mut ls2x_i2c_priv, txdata: u8) -> c_int {
    static int ls2x_i2c_send_byte(struct ls2x_i2c_priv *priv, u8 txdata)
    {
    int ret;
    u8 rxdata;
    ret = ls2x_i2c_xfer_byte(priv, txdata, &rxdata);
    if (ret)
    return ret;
    if (rxdata & LS2X_SR_AL)
    return -EAGAIN;
    if (rxdata & LS2X_SR_NOACK)
    return -ENXIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_stop(priv: *mut ls2x_i2c_priv) -> c_int {
    static int ls2x_i2c_stop(struct ls2x_i2c_priv *priv)
    {
    u8 value;
    writeb(LS2X_CR_STOP, priv.base + I2C_LS2X_CR);
    return readb_poll_timeout(priv.base + I2C_LS2X_SR, value,
    !(value & LS2X_SR_BUSY), 100,
    jiffies_to_usecs(priv.adapter.timeout));
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_start(priv: *mut ls2x_i2c_priv, msgs: *mut i2c_msg) -> c_int {
    static int ls2x_i2c_start(struct ls2x_i2c_priv *priv, struct i2c_msg *msgs)
    {
    reinit_completion(&priv.cmd_complete);
    writeb(i2c_8bit_addr_from_msg(msgs), priv.base + I2C_LS2X_TXR);
    return ls2x_i2c_send_byte(priv, LS2X_CR_START | LS2X_CR_WRITE);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_rx(priv: *mut ls2x_i2c_priv, msg: *mut i2c_msg) -> c_int {
    static int ls2x_i2c_rx(struct ls2x_i2c_priv *priv, struct i2c_msg *msg)
    {
    int ret;
    u8 rxdata, *buf = msg.buf;
    let mut len: u16 = msg.len;
// Contains steps to send start condition and address.
    ret = ls2x_i2c_start(priv, msg);
    if (ret)
    return ret;
    while (len--) {
    ret = ls2x_i2c_xfer_byte(priv,
    LS2X_CR_READ | (len ? 0 : LS2X_CR_ACK),
    &rxdata);
    if (ret)
    return ret;
// buf++ = readb(priv->base + I2C_LS2X_RXR);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_tx(priv: *mut ls2x_i2c_priv, msg: *mut i2c_msg) -> c_int {
    static int ls2x_i2c_tx(struct ls2x_i2c_priv *priv, struct i2c_msg *msg)
    {
    int ret;
    u8 *buf = msg.buf;
    let mut len: u16 = msg.len;
// Contains steps to send start condition and address.
    ret = ls2x_i2c_start(priv, msg);
    if (ret)
    return ret;
    while (len--) {
    writeb(*buf++, priv.base + I2C_LS2X_TXR);
    ret = ls2x_i2c_send_byte(priv, LS2X_CR_WRITE);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int ls2x_i2c_xfer_one(struct ls2x_i2c_priv *priv,
    struct i2c_msg *msg, bool stop)
    {
    int ret;
    if (msg.flags & I2C_M_RD)
    ret = ls2x_i2c_rx(priv, msg);
    else
    ret = ls2x_i2c_tx(priv, msg);
    if (ret < 0) {
// Fatel error. Needs reinit.
    if (ret == -ETIMEDOUT)
    ls2x_i2c_init(priv);
    return ret;
    }
    if (stop) {
// Failed to issue STOP. Needs reinit.
    ret = ls2x_i2c_stop(priv);
    if (ret)
    ls2x_i2c_init(priv);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int ls2x_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    int ret;
    struct i2c_msg *msg, *emsg = msgs + num;
    struct ls2x_i2c_priv *priv = i2c_get_adapdata(adap);
    for (msg = msgs; msg < emsg; msg++) {
    ret = ls2x_i2c_xfer_one(priv, msg, msg == emsg - 1);
    if (ret)
    return ret;
    }
    return num;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_func(adap: *mut i2c_adapter) -> c_uint {
    static unsigned int ls2x_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm ls2x_i2c_algo = {
    .xfer = ls2x_i2c_xfer,
    .functionality = ls2x_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int ls2x_i2c_probe(struct platform_device *pdev)
    {
    int ret, irq;
    struct i2c_adapter *adap;
    struct ls2x_i2c_priv *priv;
    struct device *dev = &pdev.dev;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// Map hardware registers
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
// Add the i2c adapter
    adap = &priv.adapter;
    adap.retries = 5;
    adap.nr = pdev.id;
    adap.dev.parent = dev;
    adap.owner = THIS_MODULE;
    adap.algo = &ls2x_i2c_algo;
    adap.timeout = msecs_to_jiffies(100);
    device_set_node(&adap.dev, dev_fwnode(dev));
    i2c_set_adapdata(adap, priv);
    strscpy(adap.name, pdev.name, sizeof(adap.name));
    init_completion(&priv.cmd_complete);
    platform_set_drvdata(pdev, priv);
    ls2x_i2c_init(priv);
    ret = devm_request_irq(dev, irq, ls2x_i2c_isr, IRQF_SHARED, "ls2x-i2c",
    priv);
    if (ret < 0)
    return ret;
    return devm_i2c_add_adapter(dev, adap);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_suspend(dev: *mut device) -> c_int {
    static int ls2x_i2c_suspend(struct device *dev)
    {
    struct ls2x_i2c_priv *priv = dev_get_drvdata(dev);
// Disable interrupts
    writeb(readb(priv.base + I2C_LS2X_CTR) & ~LS2X_CTR_IEN,
    priv.base + I2C_LS2X_CTR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_i2c_resume(dev: *mut device) -> c_int {
    static int ls2x_i2c_resume(struct device *dev)
    {
    ls2x_i2c_init(dev_get_drvdata(dev));
    return 0;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(ls2x_i2c_pm_ops,
    ls2x_i2c_suspend, ls2x_i2c_resume, core::ptr::null_mut());
    static const struct of_device_id ls2x_i2c_id_table[] = {
    { .compatible = "loongson,ls2k-i2c" },
    { .compatible = "loongson,ls7a-i2c" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ls2x_i2c_id_table);
    static const struct acpi_device_id ls2x_i2c_acpi_match[] = {
    { "LOON0004" }, /* Loongson LS7A */
    { }
    };
    MODULE_DEVICE_TABLE(acpi, ls2x_i2c_acpi_match);
    static struct platform_driver ls2x_i2c_driver = {
    .probe		= ls2x_i2c_probe,
    .driver		= {
    .name	= "ls2x-i2c",
    .pm	= pm_sleep_ptr(&ls2x_i2c_pm_ops),
    .of_match_table = ls2x_i2c_id_table,
    .acpi_match_table = ls2x_i2c_acpi_match,
    },
    };
    module_platform_driver(ls2x_i2c_driver);
    MODULE_DESCRIPTION("Loongson LS2X I2C Bus driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited");
    MODULE_LICENSE("GPL");
