//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-uniphier.c
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
// Copyright (C) 2015 Masahiro Yamada <yamada.masahiro@socionext.com>
//

pub const UNIPHIER_I2C_DTRM: c_uint = 0x00	/* TX register */;

pub const UNIPHIER_I2C_DREC: c_uint = 0x04	/* RX register */;

pub const UNIPHIER_I2C_MYAD: c_uint = 0x08	/* local target address */;
pub const UNIPHIER_I2C_CLK: c_uint = 0x0c	/* clock frequency control */;
pub const UNIPHIER_I2C_BRST: c_uint = 0x10	/* bus reset */;

pub const UNIPHIER_I2C_HOLD: c_uint = 0x14	/* hold time control */;
pub const UNIPHIER_I2C_BSTS: c_uint = 0x18	/* bus status monitor */;

pub const UNIPHIER_I2C_NOISE: c_uint = 0x1c	/* noise filter control */;
pub const UNIPHIER_I2C_SETUP: c_uint = 0x20	/* setup time control */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_i2c_priv {
    pub comp: completion,
    pub adap: i2c_adapter,
    pub membase: *mut void __iomem,
    pub clk: *mut clk,
    pub busy_cnt: c_uint,
    pub clk_cycle: c_uint,
}

#[no_mangle]
unsafe extern "C" fn uniphier_i2c_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t uniphier_i2c_interrupt(int irq, void *dev_id)
    {
    struct uniphier_i2c_priv *priv = dev_id;
//
// This hardware uses edge triggered interrupt.  Do not touch the
// hardware registers in this handler to make sure to catch the next
// interrupt edge.  Just send a complete signal and return.
//
    complete(&priv.comp);
    return IRQ_HANDLED;
    }
    static int uniphier_i2c_xfer_byte(struct i2c_adapter *adap, u32 txdata,
    u32 *rxdatap)
    {
    struct uniphier_i2c_priv *priv = i2c_get_adapdata(adap);
    unsigned long time_left;
    u32 rxdata;
    reinit_completion(&priv.comp);
    txdata |= UNIPHIER_I2C_DTRM_IRQEN;
    writel(txdata, priv.membase + UNIPHIER_I2C_DTRM);
    time_left = wait_for_completion_timeout(&priv.comp, adap.timeout);
    if (unlikely(!time_left))
    return -ETIMEDOUT;
    rxdata = readl(priv.membase + UNIPHIER_I2C_DREC);
    if (rxdatap)
// rxdatap = rxdata;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_send_byte(adap: *mut i2c_adapter, txdata: u32) -> c_int {
    static int uniphier_i2c_send_byte(struct i2c_adapter *adap, u32 txdata)
    {
    u32 rxdata;
    int ret;
    ret = uniphier_i2c_xfer_byte(adap, txdata, &rxdata);
    if (ret)
    return ret;
    if (unlikely(rxdata & UNIPHIER_I2C_DREC_LAB))
    return -EAGAIN;
    if (unlikely(rxdata & UNIPHIER_I2C_DREC_LRB))
    return -ENXIO;
    return 0;
    }
    static int uniphier_i2c_tx(struct i2c_adapter *adap, u16 addr, u16 len,
    const u8 *buf)
    {
    int ret;
    ret = uniphier_i2c_send_byte(adap, addr << 1 |
    UNIPHIER_I2C_DTRM_STA |
    UNIPHIER_I2C_DTRM_NACK);
    if (ret)
    return ret;
    while (len--) {
    ret = uniphier_i2c_send_byte(adap,
    UNIPHIER_I2C_DTRM_NACK | *buf++);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int uniphier_i2c_rx(struct i2c_adapter *adap, u16 addr, u16 len,
    u8 *buf)
    {
    int ret;
    ret = uniphier_i2c_send_byte(adap, addr << 1 |
    UNIPHIER_I2C_DTRM_STA |
    UNIPHIER_I2C_DTRM_NACK |
    UNIPHIER_I2C_DTRM_RD);
    if (ret)
    return ret;
    while (len--) {
    u32 rxdata;
    ret = uniphier_i2c_xfer_byte(adap,
    len ? 0 : UNIPHIER_I2C_DTRM_NACK,
    &rxdata);
    if (ret)
    return ret;
// buf++ = rxdata;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_stop(adap: *mut i2c_adapter) -> c_int {
    static int uniphier_i2c_stop(struct i2c_adapter *adap)
    {
    return uniphier_i2c_send_byte(adap, UNIPHIER_I2C_DTRM_STO |
    UNIPHIER_I2C_DTRM_NACK);
    }
    static int uniphier_i2c_xfer_one(struct i2c_adapter *adap,
    struct i2c_msg *msg, bool stop)
    {
    let mut is_read: bool = msg.flags & I2C_M_RD;
    let mut recovery: bool = false;
    int ret;
    if (is_read)
    ret = uniphier_i2c_rx(adap, msg.addr, msg.len, msg.buf);
    else
    ret = uniphier_i2c_tx(adap, msg.addr, msg.len, msg.buf);
    if (ret == -EAGAIN) /* could not acquire bus. bail out without STOP */
    return ret;
    if (ret == -ETIMEDOUT) {
// This error is fatal.  Needs recovery.
    stop = false;
    recovery = true;
    }
    if (stop) {
    let mut ret2: c_int = uniphier_i2c_stop(adap);
    if (ret2) {
// Failed to issue STOP.  The bus needs recovery.
    recovery = true;
    ret = ret ?: ret2;
    }
    }
    if (recovery)
    i2c_recover_bus(adap);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_check_bus_busy(adap: *mut i2c_adapter) -> c_int {
    static int uniphier_i2c_check_bus_busy(struct i2c_adapter *adap)
    {
    struct uniphier_i2c_priv *priv = i2c_get_adapdata(adap);
    if (!(readl(priv.membase + UNIPHIER_I2C_DREC) &
    UNIPHIER_I2C_DREC_BBN)) {
    if (priv.busy_cnt++ > 3) {
//
// If bus busy continues too long, it is probably
// in a wrong state.  Try bus recovery.
//
    i2c_recover_bus(adap);
    priv.busy_cnt = 0;
    }
    return -EAGAIN;
    }
    priv.busy_cnt = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int uniphier_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    struct i2c_msg *msg, *emsg = msgs + num;
    int ret;
    ret = uniphier_i2c_check_bus_busy(adap);
    if (ret)
    return ret;
    for (msg = msgs; msg < emsg; msg++) {
// Emit STOP if it is the last message or I2C_M_STOP is set.
    let mut stop: bool = (msg + 1 == emsg) || (msg.flags & I2C_M_STOP);
    ret = uniphier_i2c_xfer_one(adap, msg, stop);
    if (ret)
    return ret;
    }
    return num;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 uniphier_i2c_functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm uniphier_i2c_algo = {
    .xfer = uniphier_i2c_xfer,
    .functionality = uniphier_i2c_functionality,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_reset(priv: *mut uniphier_i2c_priv, reset_on: bool) {
    static void uniphier_i2c_reset(struct uniphier_i2c_priv *priv, bool reset_on)
    {
    let mut val: u32 = UNIPHIER_I2C_BRST_RSCL;
    val |= reset_on ? 0 : UNIPHIER_I2C_BRST_FOEN;
    writel(val, priv.membase + UNIPHIER_I2C_BRST);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_get_scl(adap: *mut i2c_adapter) -> c_int {
    static int uniphier_i2c_get_scl(struct i2c_adapter *adap)
    {
    struct uniphier_i2c_priv *priv = i2c_get_adapdata(adap);
    return !!(readl(priv.membase + UNIPHIER_I2C_BSTS) &
    UNIPHIER_I2C_BSTS_SCL);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_set_scl(adap: *mut i2c_adapter, val: c_int) {
    static void uniphier_i2c_set_scl(struct i2c_adapter *adap, int val)
    {
    struct uniphier_i2c_priv *priv = i2c_get_adapdata(adap);
    writel(val ? UNIPHIER_I2C_BRST_RSCL : 0,
    priv.membase + UNIPHIER_I2C_BRST);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_get_sda(adap: *mut i2c_adapter) -> c_int {
    static int uniphier_i2c_get_sda(struct i2c_adapter *adap)
    {
    struct uniphier_i2c_priv *priv = i2c_get_adapdata(adap);
    return !!(readl(priv.membase + UNIPHIER_I2C_BSTS) &
    UNIPHIER_I2C_BSTS_SDA);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_unprepare_recovery(adap: *mut i2c_adapter) {
    static void uniphier_i2c_unprepare_recovery(struct i2c_adapter *adap)
    {
    uniphier_i2c_reset(i2c_get_adapdata(adap), false);
    }
    static struct i2c_bus_recovery_info uniphier_i2c_bus_recovery_info = {
    .recover_bus = i2c_generic_scl_recovery,
    .get_scl = uniphier_i2c_get_scl,
    .set_scl = uniphier_i2c_set_scl,
    .get_sda = uniphier_i2c_get_sda,
    .unprepare_recovery = uniphier_i2c_unprepare_recovery,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_hw_init(priv: *mut uniphier_i2c_priv) {
    static void uniphier_i2c_hw_init(struct uniphier_i2c_priv *priv)
    {
    let mut cyc: c_uint = priv.clk_cycle;
    uniphier_i2c_reset(priv, true);
//
// Bit30-16: clock cycles of tLOW.
// Standard-mode: tLOW = 4.7 us, tHIGH = 4.0 us
// Fast-mode:     tLOW = 1.3 us, tHIGH = 0.6 us
// "tLow/tHIGH = 5/4" meets both.
//
    writel((cyc * 5 / 9 << 16) | cyc, priv.membase + UNIPHIER_I2C_CLK);
    uniphier_i2c_reset(priv, false);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_i2c_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_i2c_priv *priv;
    u32 bus_speed;
    unsigned long clk_rate;
    int irq, ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.membase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.membase))
    return PTR_ERR(priv.membase);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (of_property_read_u32(dev.of_node, "clock-frequency", &bus_speed))
    bus_speed = I2C_MAX_STANDARD_MODE_FREQ;
    if (!bus_speed || bus_speed > I2C_MAX_FAST_MODE_FREQ)
    return dev_err_probe(dev, -EINVAL, "invalid clock-frequency %d\n", bus_speed);
    priv.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk), "failed to enable clock\n");
    clk_rate = clk_get_rate(priv.clk);
    if (!clk_rate)
    return dev_err_probe(dev, -EINVAL, "input clock rate should not be zero\n");
    priv.clk_cycle = clk_rate / bus_speed;
    init_completion(&priv.comp);
    priv.adap.owner = THIS_MODULE;
    priv.adap.algo = &uniphier_i2c_algo;
    priv.adap.dev.parent = dev;
    priv.adap.dev.of_node = dev.of_node;
    strscpy(priv.adap.name, "UniPhier I2C", sizeof(priv.adap.name));
    priv.adap.bus_recovery_info = &uniphier_i2c_bus_recovery_info;
    i2c_set_adapdata(&priv.adap, priv);
    platform_set_drvdata(pdev, priv);
    uniphier_i2c_hw_init(priv);
    ret = devm_request_irq(dev, irq, uniphier_i2c_interrupt, 0, pdev.name,
    priv);
    if (ret)
    return ret;
    return i2c_add_adapter(&priv.adap);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_remove(pdev: *mut platform_device) {
    static void uniphier_i2c_remove(struct platform_device *pdev)
    {
    struct uniphier_i2c_priv *priv = platform_get_drvdata(pdev);
    i2c_del_adapter(&priv.adap);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused uniphier_i2c_suspend(struct device *dev)
    {
    struct uniphier_i2c_priv *priv = dev_get_drvdata(dev);
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_i2c_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused uniphier_i2c_resume(struct device *dev)
    {
    struct uniphier_i2c_priv *priv = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    uniphier_i2c_hw_init(priv);
    return 0;
    }
    static const struct dev_pm_ops uniphier_i2c_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(uniphier_i2c_suspend, uniphier_i2c_resume)
    };
    static const struct of_device_id uniphier_i2c_match[] = {
    { .compatible = "socionext,uniphier-i2c" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_i2c_match);
    static struct platform_driver uniphier_i2c_drv = {
    .probe  = uniphier_i2c_probe,
    .remove = uniphier_i2c_remove,
    .driver = {
    .name  = "uniphier-i2c",
    .of_match_table = uniphier_i2c_match,
    .pm = &uniphier_i2c_pm_ops,
    },
    };
    module_platform_driver(uniphier_i2c_drv);
    MODULE_AUTHOR("Masahiro Yamada <yamada.masahiro@socionext.com>");
    MODULE_DESCRIPTION("UniPhier I2C bus driver");
    MODULE_LICENSE("GPL");
