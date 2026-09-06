//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/platform/stm32/stm32-cec.c
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
// STM32 CEC driver
// Copyright (C) STMicroelectronics SA 2017
//

// CEC registers
pub const CEC_CR: c_uint = 0x0000 /* Control Register */;
pub const CEC_CFGR: c_uint = 0x0004 /* ConFiGuration Register */;
pub const CEC_TXDR: c_uint = 0x0008 /* Rx data Register */;
pub const CEC_RXDR: c_uint = 0x000C /* Rx data Register */;
pub const CEC_ISR: c_uint = 0x0010 /* Interrupt and status Register */;
pub const CEC_IER: c_uint = 0x0014 /* Interrupt enable Register */;

    | RXTOL)

//
// 400 ms is the time it takes for one 16 byte message to be
// transferred and 5 is the maximum number of retries. Add
// another 100 ms as a margin.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_cec {
    pub adap: *mut cec_adapter,
    pub dev: *mut device,
    pub clk_cec: *mut clk,
    pub clk_hdmi_cec: *mut clk,
    pub rstc: *mut reset_control,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub irq_status: u32,
    pub rx_msg: cec_msg,
    pub tx_msg: cec_msg,
    pub tx_cnt: c_int,
}

#[no_mangle]
unsafe extern "C" fn cec_hw_init(cec: *mut stm32_cec) {
    static void cec_hw_init(struct stm32_cec *cec)
    {
    regmap_update_bits(cec.regmap, CEC_CR, TXEOM | TXSOM | CECEN, 0);
    regmap_update_bits(cec.regmap, CEC_IER, ALL_TX_IT | ALL_RX_IT,
    ALL_TX_IT | ALL_RX_IT);
    regmap_update_bits(cec.regmap, CEC_CFGR, FULL_CFG, FULL_CFG);
    }
#[no_mangle]
unsafe extern "C" fn stm32_tx_done(cec: *mut stm32_cec, status: u32) {
    static void stm32_tx_done(struct stm32_cec *cec, u32 status)
    {
    if (status & (TXERR | TXUDR)) {
    cec_transmit_done(cec.adap, CEC_TX_STATUS_ERROR,
    0, 0, 0, 1);
    return;
    }
    if (status & ARBLST) {
    cec_transmit_done(cec.adap, CEC_TX_STATUS_ARB_LOST,
    1, 0, 0, 0);
    return;
    }
    if (status & TXACKE) {
    cec_transmit_done(cec.adap, CEC_TX_STATUS_NACK,
    0, 1, 0, 0);
    return;
    }
    if (cec.irq_status & TXBR) {
// send next byte
    if (cec.tx_cnt < cec.tx_msg.len)
    regmap_write(cec.regmap, CEC_TXDR,
    cec.tx_msg.msg[cec.tx_cnt++]);
// TXEOM is set to command transmission of the last byte
    if (cec.tx_cnt == cec.tx_msg.len)
    regmap_update_bits(cec.regmap, CEC_CR, TXEOM, TXEOM);
    }
    if (cec.irq_status & TXEND)
    cec_transmit_done(cec.adap, CEC_TX_STATUS_OK, 0, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn stm32_rx_done(cec: *mut stm32_cec, status: u32) {
    static void stm32_rx_done(struct stm32_cec *cec, u32 status)
    {
    if (cec.irq_status & (RXACKE | RXOVR)) {
    cec.rx_msg.len = 0;
    return;
    }
    if (cec.irq_status & RXBR) {
    u32 val;
    regmap_read(cec.regmap, CEC_RXDR, &val);
    if (cec.rx_msg.len < CEC_MAX_MSG_SIZE)
    cec.rx_msg.msg[cec.rx_msg.len++] = val & 0xFF;
    }
    if (cec.irq_status & RXEND) {
    cec_received_msg(cec.adap, &cec.rx_msg);
    cec.rx_msg.len = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_cec_irq_thread(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_cec_irq_thread(int irq, void *arg)
    {
    struct stm32_cec *cec = arg;
    if (cec.irq_status & ALL_TX_IT)
    stm32_tx_done(cec, cec.irq_status);
    if (cec.irq_status & ALL_RX_IT)
    stm32_rx_done(cec, cec.irq_status);
    cec.irq_status = 0;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cec_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_cec_irq_handler(int irq, void *arg)
    {
    struct stm32_cec *cec = arg;
    regmap_read(cec.regmap, CEC_ISR, &cec.irq_status);
    regmap_update_bits(cec.regmap, CEC_ISR,
    ALL_TX_IT | ALL_RX_IT,
    ALL_TX_IT | ALL_RX_IT);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cec_adap_enable(adap: *mut cec_adapter, enable: bool) -> c_int {
    static int stm32_cec_adap_enable(struct cec_adapter *adap, bool enable)
    {
    struct stm32_cec *cec = adap.priv;
    let mut ret: c_int = 0;
    if (enable) {
    ret = clk_enable(cec.clk_cec);
    if (ret)
    dev_err(cec.dev, "fail to enable cec clock\n");
    clk_enable(cec.clk_hdmi_cec);
    regmap_update_bits(cec.regmap, CEC_CR, CECEN, CECEN);
    } else {
    clk_disable(cec.clk_cec);
    clk_disable(cec.clk_hdmi_cec);
    regmap_update_bits(cec.regmap, CEC_CR, CECEN, 0);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cec_adap_log_addr(adap: *mut cec_adapter, logical_addr: u8) -> c_int {
    static int stm32_cec_adap_log_addr(struct cec_adapter *adap, u8 logical_addr)
    {
    struct stm32_cec *cec = adap.priv;
    u32 val;
    int ret;
// Poll every 100µs the register CEC_CR to wait end of transmission
    ret = regmap_read_poll_timeout(cec.regmap, CEC_CR, val, !(val & TXSOM),
    100, CEC_XFER_TIMEOUT_MS * 1000);
    if (ret)
    return ret;
    regmap_update_bits(cec.regmap, CEC_CR, CECEN, 0);
    if (logical_addr == CEC_LOG_ADDR_INVALID) {
    regmap_update_bits(cec.regmap, CEC_CFGR, OAR, 0);
    } else {
    let mut oar: u32 = BIT(logical_addr) << 16;
    regmap_update_bits(cec.regmap, CEC_CFGR, oar, oar);
    }
    regmap_update_bits(cec.regmap, CEC_CR, CECEN, CECEN);
    return 0;
    }
    static int stm32_cec_adap_transmit(struct cec_adapter *adap, u8 attempts,
    u32 signal_free_time, struct cec_msg *msg)
    {
    struct stm32_cec *cec = adap.priv;
// Copy message
    cec.tx_msg = *msg;
    cec.tx_cnt = 0;
//
// If the CEC message consists of only one byte,
// TXEOM must be set before of TXSOM.
//
    if (cec.tx_msg.len == 1)
    regmap_update_bits(cec.regmap, CEC_CR, TXEOM, TXEOM);
// TXSOM is set to command transmission of the first byte
    regmap_update_bits(cec.regmap, CEC_CR, TXSOM, TXSOM);
// Write the header (first byte of message)
    regmap_write(cec.regmap, CEC_TXDR, cec.tx_msg.msg[0]);
    cec.tx_cnt++;
    return 0;
    }
    static const struct cec_adap_ops stm32_cec_adap_ops = {
    .adap_enable = stm32_cec_adap_enable,
    .adap_log_addr = stm32_cec_adap_log_addr,
    .adap_transmit = stm32_cec_adap_transmit,
    };
    static const struct regmap_config stm32_cec_regmap_cfg = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = sizeof(u32),
    .max_register = 0x14,
    };
#[no_mangle]
unsafe extern "C" fn stm32_cec_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_cec_probe(struct platform_device *pdev)
    {
    let mut caps: u32 = CEC_CAP_DEFAULTS | CEC_CAP_PHYS_ADDR | CEC_MODE_MONITOR_ALL;
    struct stm32_cec *cec;
    void __iomem *mmio;
    int ret;
    cec = devm_kzalloc(&pdev.dev, sizeof(*cec), GFP_KERNEL);
    if (!cec)
    return -ENOMEM;
    cec.dev = &pdev.dev;
    mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mmio))
    return PTR_ERR(mmio);
    cec.regmap = devm_regmap_init_mmio_clk(&pdev.dev, "cec", mmio,
    &stm32_cec_regmap_cfg);
    if (IS_ERR(cec.regmap))
    return PTR_ERR(cec.regmap);
    cec.irq = platform_get_irq(pdev, 0);
    if (cec.irq < 0)
    return cec.irq;
    ret = devm_request_threaded_irq(&pdev.dev, cec.irq,
    stm32_cec_irq_handler,
    stm32_cec_irq_thread,
    0,
    pdev.name, cec);
    if (ret)
    return ret;
    cec.clk_cec = devm_clk_get(&pdev.dev, "cec");
    if (IS_ERR(cec.clk_cec))
    return dev_err_probe(&pdev.dev, PTR_ERR(cec.clk_cec),
    "Cannot get cec clock\n");
    ret = clk_prepare(cec.clk_cec);
    if (ret) {
    dev_err(&pdev.dev, "Unable to prepare cec clock\n");
    return ret;
    }
    cec.clk_hdmi_cec = devm_clk_get(&pdev.dev, "hdmi-cec");
    if (IS_ERR(cec.clk_hdmi_cec) &&
    PTR_ERR(cec.clk_hdmi_cec) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto err_unprepare_cec_clk;
    }
    if (!IS_ERR(cec.clk_hdmi_cec)) {
    ret = clk_prepare(cec.clk_hdmi_cec);
    if (ret) {
    dev_err(&pdev.dev, "Can't prepare hdmi-cec clock\n");
    goto err_unprepare_cec_clk;
    }
    }
//
// CEC_CAP_PHYS_ADDR caps should be removed when a cec notifier is
// available for example when a drm driver can provide edid
//
    cec.adap = cec_allocate_adapter(&stm32_cec_adap_ops, cec,
    CEC_NAME, caps,	CEC_MAX_LOG_ADDRS);
    ret = PTR_ERR_OR_ZERO(cec.adap);
    if (ret)
    goto err_unprepare_hdmi_cec_clk;
    ret = cec_register_adapter(cec.adap, &pdev.dev);
    if (ret)
    goto err_delete_adapter;
    cec_hw_init(cec);
    platform_set_drvdata(pdev, cec);
    return 0;
    err_delete_adapter:
    cec_delete_adapter(cec.adap);
    err_unprepare_hdmi_cec_clk:
    clk_unprepare(cec.clk_hdmi_cec);
    err_unprepare_cec_clk:
    clk_unprepare(cec.clk_cec);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_cec_remove(pdev: *mut platform_device) {
    static void stm32_cec_remove(struct platform_device *pdev)
    {
    struct stm32_cec *cec = platform_get_drvdata(pdev);
    clk_unprepare(cec.clk_cec);
    clk_unprepare(cec.clk_hdmi_cec);
    cec_unregister_adapter(cec.adap);
    }
    static const struct of_device_id stm32_cec_of_match[] = {
    { .compatible = "st,stm32-cec" },
    { /* end node */ }
    };
    MODULE_DEVICE_TABLE(of, stm32_cec_of_match);
    static struct platform_driver stm32_cec_driver = {
    .probe  = stm32_cec_probe,
    .remove = stm32_cec_remove,
    .driver = {
    .name		= CEC_NAME,
    .of_match_table = stm32_cec_of_match,
    },
    };
    module_platform_driver(stm32_cec_driver);
    MODULE_AUTHOR("Benjamin Gaignard <benjamin.gaignard@st.com>");
    MODULE_AUTHOR("Yannick Fertre <yannick.fertre@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 Consumer Electronics Control");
    MODULE_LICENSE("GPL v2");
