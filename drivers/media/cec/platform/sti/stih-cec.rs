//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/platform/sti/stih-cec.c
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


// SPDX-License-Identifier: GPL-2.0
//
// STIH4xx CEC driver
// Copyright (C) STMicroelectronics SA 2016
//

// CEC registers
pub const CEC_CLK_DIV: c_uint = 0x0;
pub const CEC_CTRL: c_uint = 0x4;
pub const CEC_IRQ_CTRL: c_uint = 0x8;
pub const CEC_STATUS: c_uint = 0xC;
pub const CEC_EXT_STATUS: c_uint = 0x10;
pub const CEC_TX_CTRL: c_uint = 0x14;
pub const CEC_FREE_TIME_THRESH: c_uint = 0x18;
pub const CEC_BIT_TOUT_THRESH: c_uint = 0x1C;
pub const CEC_BIT_PULSE_THRESH: c_uint = 0x20;
pub const CEC_DATA: c_uint = 0x24;
pub const CEC_TX_ARRAY_CTRL: c_uint = 0x28;
pub const CEC_CTRL2: c_uint = 0x2C;
pub const CEC_TX_ERROR_STS: c_uint = 0x30;
pub const CEC_ADDR_TABLE: c_uint = 0x34;
pub const CEC_DATA_ARRAY_CTRL: c_uint = 0x38;
pub const CEC_DATA_ARRAY_STATUS: c_uint = 0x3C;
pub const CEC_TX_DATA_BASE: c_uint = 0x40;
pub const CEC_TX_DATA_TOP: c_uint = 0x50;
pub const CEC_TX_DATA_SIZE: c_uint = 0x1;
pub const CEC_RX_DATA_BASE: c_uint = 0x54;
pub const CEC_RX_DATA_TOP: c_uint = 0x64;
pub const CEC_RX_DATA_SIZE: c_uint = 0x1;
// CEC_CTRL2

// CEC_DATA_ARRAY_CTRL

// CEC_TX_ARRAY_CTRL
pub const CEC_TX_N_OF_BYTES: c_uint = 0x1F;

// CEC_IRQ_CTRL

// CEC_CTRL

// CEC_STATUS

// Signal free time in bit periods (2.4ms)
pub const CEC_PRESENT_INIT_SFT: c_int = 7;
pub const CEC_NEW_INIT_SFT: c_int = 5;
pub const CEC_RETRANSMIT_SFT: c_int = 3;
// Constants for CEC_BIT_TOUT_THRESH register

// Constants for CEC_BIT_PULSE_THRESH register

// Constants for CEC_DATA_ARRAY_STATUS register
pub const CEC_RX_N_OF_BYTES: c_uint = 0x1F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stih_cec {
    pub adap: *mut cec_adapter,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub irq_status: u32,
    pub notifier: *mut cec_notifier,
}

#[no_mangle]
unsafe extern "C" fn stih_cec_adap_enable(adap: *mut cec_adapter, enable: bool) -> c_int {
    static int stih_cec_adap_enable(struct cec_adapter *adap, bool enable)
    {
    struct stih_cec *cec = cec_get_drvdata(adap);
    if (enable) {
// The doc says (input TCLK_PERIOD * CEC_CLK_DIV) = 0.1ms
    let mut clk_freq: c_ulong = clk_get_rate(cec.clk);
    let mut cec_clk_div: u32 = clk_freq / 10000;
    writel(cec_clk_div, cec.regs + CEC_CLK_DIV);
// Configuration of the durations activating a timeout
    writel(CEC_SBIT_TOUT_47MS | (CEC_DBIT_TOUT_28MS << 4),
    cec.regs + CEC_BIT_TOUT_THRESH);
// Configuration of the smallest allowed duration for pulses
    writel(CEC_BIT_LPULSE_03MS | CEC_BIT_HPULSE_03MS,
    cec.regs + CEC_BIT_PULSE_THRESH);
// Minimum received bit period threshold
    writel(BIT(5) | BIT(7), cec.regs + CEC_TX_CTRL);
// Configuration of transceiver data arrays
    writel(CEC_TX_ARRAY_EN | CEC_RX_ARRAY_EN | CEC_TX_STOP_ON_NACK,
    cec.regs + CEC_DATA_ARRAY_CTRL);
// Configuration of the control bits for CEC Transceiver
    writel(CEC_IN_FILTER_EN | CEC_EN | CEC_RX_RESET_EN,
    cec.regs + CEC_CTRL);
// Clear logical addresses
    writel(0, cec.regs + CEC_ADDR_TABLE);
// Clear the status register
    writel(0x0, cec.regs + CEC_STATUS);
// Enable the interrupts
    writel(CEC_TX_DONE_IRQ_EN | CEC_RX_DONE_IRQ_EN |
    CEC_RX_SOM_IRQ_EN | CEC_RX_EOM_IRQ_EN |
    CEC_ERROR_IRQ_EN,
    cec.regs + CEC_IRQ_CTRL);
    } else {
// Clear logical addresses
    writel(0, cec.regs + CEC_ADDR_TABLE);
// Clear the status register
    writel(0x0, cec.regs + CEC_STATUS);
// Disable the interrupts
    writel(0, cec.regs + CEC_IRQ_CTRL);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stih_cec_adap_log_addr(adap: *mut cec_adapter, logical_addr: u8) -> c_int {
    static int stih_cec_adap_log_addr(struct cec_adapter *adap, u8 logical_addr)
    {
    struct stih_cec *cec = cec_get_drvdata(adap);
    let mut reg: u32 = readl(cec.regs + CEC_ADDR_TABLE);
    reg |= 1 << logical_addr;
    if (logical_addr == CEC_LOG_ADDR_INVALID)
    reg = 0;
    writel(reg, cec.regs + CEC_ADDR_TABLE);
    return 0;
    }
    static int stih_cec_adap_transmit(struct cec_adapter *adap, u8 attempts,
    u32 signal_free_time, struct cec_msg *msg)
    {
    struct stih_cec *cec = cec_get_drvdata(adap);
    int i;
// Copy message into registers
    for (i = 0; i < msg.len; i++)
    writeb(msg.msg[i], cec.regs + CEC_TX_DATA_BASE + i);
//
// Start transmission, configure hardware to add start and stop bits
// Signal free time is handled by the hardware
//
    writel(CEC_TX_AUTO_SOM_EN | CEC_TX_AUTO_EOM_EN | CEC_TX_START |
    msg.len, cec.regs + CEC_TX_ARRAY_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stih_tx_done(cec: *mut stih_cec, status: u32) {
    static void stih_tx_done(struct stih_cec *cec, u32 status)
    {
    if (status & CEC_TX_ERROR) {
    cec_transmit_attempt_done(cec.adap, CEC_TX_STATUS_ERROR);
    return;
    }
    if (status & CEC_TX_ARB_ERROR) {
    cec_transmit_attempt_done(cec.adap, CEC_TX_STATUS_ARB_LOST);
    return;
    }
    if (!(status & CEC_TX_ACK_GET_STS)) {
    cec_transmit_attempt_done(cec.adap, CEC_TX_STATUS_NACK);
    return;
    }
    cec_transmit_attempt_done(cec.adap, CEC_TX_STATUS_OK);
    }
#[no_mangle]
unsafe extern "C" fn stih_rx_done(cec: *mut stih_cec, status: u32) {
    static void stih_rx_done(struct stih_cec *cec, u32 status)
    {
    let mut msg: cec_msg = {};
    u8 i;
    if (status & CEC_RX_ERROR_MIN)
    return;
    if (status & CEC_RX_ERROR_MAX)
    return;
    msg.len = readl(cec.regs + CEC_DATA_ARRAY_STATUS) & 0x1f;
    if (!msg.len)
    return;
    if (msg.len > CEC_MAX_MSG_SIZE)
    msg.len = CEC_MAX_MSG_SIZE;
    for (i = 0; i < msg.len; i++)
    msg.msg[i] = readl(cec.regs + CEC_RX_DATA_BASE + i);
    cec_received_msg(cec.adap, &msg);
    }
#[no_mangle]
unsafe extern "C" fn stih_cec_irq_handler_thread(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t stih_cec_irq_handler_thread(int irq, void *priv)
    {
    struct stih_cec *cec = priv;
    if (cec.irq_status & CEC_TX_DONE_STS)
    stih_tx_done(cec, cec.irq_status);
    if (cec.irq_status & CEC_RX_DONE_STS)
    stih_rx_done(cec, cec.irq_status);
    cec.irq_status = 0;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stih_cec_irq_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t stih_cec_irq_handler(int irq, void *priv)
    {
    struct stih_cec *cec = priv;
    cec.irq_status = readl(cec.regs + CEC_STATUS);
    writel(cec.irq_status, cec.regs + CEC_STATUS);
    return IRQ_WAKE_THREAD;
    }
    static const struct cec_adap_ops sti_cec_adap_ops = {
    .adap_enable = stih_cec_adap_enable,
    .adap_log_addr = stih_cec_adap_log_addr,
    .adap_transmit = stih_cec_adap_transmit,
    };
#[no_mangle]
unsafe extern "C" fn stih_cec_probe(pdev: *mut platform_device) -> c_int {
    static int stih_cec_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct stih_cec *cec;
    struct device *hdmi_dev;
    int ret;
    hdmi_dev = cec_notifier_parse_hdmi_phandle(dev);
    if (IS_ERR(hdmi_dev))
    return PTR_ERR(hdmi_dev);
    cec = devm_kzalloc(dev, sizeof(*cec), GFP_KERNEL);
    if (!cec)
    return -ENOMEM;
    cec.dev = dev;
    cec.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cec.regs))
    return PTR_ERR(cec.regs);
    cec.irq = platform_get_irq(pdev, 0);
    if (cec.irq < 0)
    return cec.irq;
    ret = devm_request_threaded_irq(dev, cec.irq, stih_cec_irq_handler,
    stih_cec_irq_handler_thread, 0,
    pdev.name, cec);
    if (ret)
    return ret;
    cec.clk = devm_clk_get(dev, "cec-clk");
    if (IS_ERR(cec.clk)) {
    dev_err(dev, "Cannot get cec clock\n");
    return PTR_ERR(cec.clk);
    }
    cec.adap = cec_allocate_adapter(&sti_cec_adap_ops, cec, CEC_NAME,
    CEC_CAP_DEFAULTS |
    CEC_CAP_CONNECTOR_INFO,
    CEC_MAX_LOG_ADDRS);
    ret = PTR_ERR_OR_ZERO(cec.adap);
    if (ret)
    return ret;
    cec.notifier = cec_notifier_cec_adap_register(hdmi_dev, core::ptr::null_mut(),
    cec.adap);
    if (!cec.notifier) {
    ret = -ENOMEM;
    goto err_delete_adapter;
    }
    ret = cec_register_adapter(cec.adap, &pdev.dev);
    if (ret)
    goto err_notifier;
    platform_set_drvdata(pdev, cec);
    return 0;
    err_notifier:
    cec_notifier_cec_adap_unregister(cec.notifier, cec.adap);
    err_delete_adapter:
    cec_delete_adapter(cec.adap);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stih_cec_remove(pdev: *mut platform_device) {
    static void stih_cec_remove(struct platform_device *pdev)
    {
    struct stih_cec *cec = platform_get_drvdata(pdev);
    cec_notifier_cec_adap_unregister(cec.notifier, cec.adap);
    cec_unregister_adapter(cec.adap);
    }
    static const struct of_device_id stih_cec_match[] = {
    {
    .compatible	= "st,stih-cec",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, stih_cec_match);
    static struct platform_driver stih_cec_pdrv = {
    .probe	= stih_cec_probe,
    .remove = stih_cec_remove,
    .driver = {
    .name		= CEC_NAME,
    .of_match_table	= stih_cec_match,
    },
    };
    module_platform_driver(stih_cec_pdrv);
    MODULE_AUTHOR("Benjamin Gaignard <benjamin.gaignard@linaro.org>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("STIH4xx CEC driver");
