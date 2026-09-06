//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-hisi.c
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
// HiSilicon I2C Controller Driver for Kunpeng SoC
//
// Copyright (c) 2021 HiSilicon Technologies Co., Ltd.
//

pub const HISI_I2C_FRAME_CTRL: c_uint = 0x0000;

pub const HISI_I2C_SLV_ADDR: c_uint = 0x0004;

pub const HISI_I2C_CMD_TXDATA: c_uint = 0x0008;

pub const HISI_I2C_RXDATA: c_uint = 0x000c;

pub const HISI_I2C_SS_SCL_HCNT: c_uint = 0x0010;
pub const HISI_I2C_SS_SCL_LCNT: c_uint = 0x0014;
pub const HISI_I2C_FS_SCL_HCNT: c_uint = 0x0018;
pub const HISI_I2C_FS_SCL_LCNT: c_uint = 0x001c;
pub const HISI_I2C_HS_SCL_HCNT: c_uint = 0x0020;
pub const HISI_I2C_HS_SCL_LCNT: c_uint = 0x0024;
pub const HISI_I2C_FIFO_CTRL: c_uint = 0x0028;

pub const HISI_I2C_FIFO_STATE: c_uint = 0x002c;

pub const HISI_I2C_SDA_HOLD: c_uint = 0x0030;

pub const HISI_I2C_FS_SPK_LEN: c_uint = 0x0038;

pub const HISI_I2C_HS_SPK_LEN: c_uint = 0x003c;

pub const HISI_I2C_TX_INT_CLR: c_uint = 0x0040;

pub const HISI_I2C_INT_MSTAT: c_uint = 0x0044;
pub const HISI_I2C_INT_CLR: c_uint = 0x0048;
pub const HISI_I2C_INT_MASK: c_uint = 0x004C;
pub const HISI_I2C_TRANS_STATE: c_uint = 0x0050;
pub const HISI_I2C_TRANS_ERR: c_uint = 0x0054;
pub const HISI_I2C_VERSION: c_uint = 0x0058;

    (HISI_I2C_INT_TRANS_ERR | HISI_I2C_INT_FIFO_ERR)
pub const HISI_I2C_STD_SPEED_MODE: c_int = 0;
pub const HISI_I2C_FAST_SPEED_MODE: c_int = 1;
pub const HISI_I2C_HIGH_SPEED_MODE: c_int = 2;
pub const HISI_I2C_TX_FIFO_DEPTH: c_int = 64;
pub const HISI_I2C_RX_FIFO_DEPTH: c_int = 64;
pub const HISI_I2C_TX_F_AE_THRESH: c_int = 1;
pub const HISI_I2C_RX_F_AF_THRESH: c_int = 60;

    DIV_ROUND_UP_ULL((clk_rate_khz) * (ns), NSEC_PER_MSEC)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_i2c_controller {
    pub adapter: i2c_adapter,
    pub iobase: *mut void __iomem,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub irq: c_int,
// Intermediates for recording the transfer process
    pub completion: *mut completion,
    pub msgs: *mut i2c_msg,
    pub msg_num: c_int,
    pub msg_tx_idx: c_int,
    pub buf_tx_idx: c_int,
    pub msg_rx_idx: c_int,
    pub buf_rx_idx: c_int,
    pub tar_addr: u16,
    pub xfer_err: u32,
// I2C bus configuration
    pub t: i2c_timings,
    pub clk_rate_khz: u32,
    pub spk_len: u32,
}

#[no_mangle]
unsafe extern "C" fn hisi_i2c_enable_int(ctlr: *mut hisi_i2c_controller, mask: u32) {
    static void hisi_i2c_enable_int(struct hisi_i2c_controller *ctlr, u32 mask)
    {
    writel_relaxed(mask, ctlr.iobase + HISI_I2C_INT_MASK);
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_disable_int(ctlr: *mut hisi_i2c_controller, mask: u32) {
    static void hisi_i2c_disable_int(struct hisi_i2c_controller *ctlr, u32 mask)
    {
    writel_relaxed((~mask) & HISI_I2C_INT_ALL, ctlr.iobase + HISI_I2C_INT_MASK);
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_clear_int(ctlr: *mut hisi_i2c_controller, mask: u32) {
    static void hisi_i2c_clear_int(struct hisi_i2c_controller *ctlr, u32 mask)
    {
    writel_relaxed(mask, ctlr.iobase + HISI_I2C_INT_CLR);
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_clear_tx_int(ctlr: *mut hisi_i2c_controller, mask: u32) {
    static void hisi_i2c_clear_tx_int(struct hisi_i2c_controller *ctlr, u32 mask)
    {
    writel_relaxed(mask, ctlr.iobase + HISI_I2C_TX_INT_CLR);
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_handle_errors(ctlr: *mut hisi_i2c_controller) {
    static void hisi_i2c_handle_errors(struct hisi_i2c_controller *ctlr)
    {
    let mut int_err: u32 = ctlr.xfer_err, reg;
    if (int_err & HISI_I2C_INT_FIFO_ERR) {
    reg = readl(ctlr.iobase + HISI_I2C_FIFO_STATE);
    if (reg & HISI_I2C_FIFO_STATE_RX_RERR)
    dev_err(ctlr.dev, "rx fifo error read\n");
    if (reg & HISI_I2C_FIFO_STATE_RX_WERR)
    dev_err(ctlr.dev, "rx fifo error write\n");
    if (reg & HISI_I2C_FIFO_STATE_TX_RERR)
    dev_err(ctlr.dev, "tx fifo error read\n");
    if (reg & HISI_I2C_FIFO_STATE_TX_WERR)
    dev_err(ctlr.dev, "tx fifo error write\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_start_xfer(ctlr: *mut hisi_i2c_controller) -> c_int {
    static int hisi_i2c_start_xfer(struct hisi_i2c_controller *ctlr)
    {
    struct i2c_msg *msg = ctlr.msgs;
    u32 reg;
    reg = readl(ctlr.iobase + HISI_I2C_FRAME_CTRL);
    reg &= ~HISI_I2C_FRAME_CTRL_ADDR_TEN;
    if (msg.flags & I2C_M_TEN)
    reg |= HISI_I2C_FRAME_CTRL_ADDR_TEN;
    writel(reg, ctlr.iobase + HISI_I2C_FRAME_CTRL);
    reg = readl(ctlr.iobase + HISI_I2C_SLV_ADDR);
    reg &= ~HISI_I2C_SLV_ADDR_VAL;
    reg |= FIELD_PREP(HISI_I2C_SLV_ADDR_VAL, msg.addr);
    writel(reg, ctlr.iobase + HISI_I2C_SLV_ADDR);
    reg = readl(ctlr.iobase + HISI_I2C_FIFO_CTRL);
    reg |= HISI_I2C_FIFO_RX_CLR | HISI_I2C_FIFO_TX_CLR;
    writel(reg, ctlr.iobase + HISI_I2C_FIFO_CTRL);
    reg &= ~(HISI_I2C_FIFO_RX_CLR | HISI_I2C_FIFO_TX_CLR);
    writel(reg, ctlr.iobase + HISI_I2C_FIFO_CTRL);
    hisi_i2c_clear_int(ctlr, HISI_I2C_INT_ALL);
    hisi_i2c_clear_tx_int(ctlr, HISI_I2C_TX_AEMPTY_INT);
    hisi_i2c_enable_int(ctlr, HISI_I2C_INT_ALL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_reset_xfer(ctlr: *mut hisi_i2c_controller) {
    static void hisi_i2c_reset_xfer(struct hisi_i2c_controller *ctlr)
    {
    ctlr.msg_num = 0;
    ctlr.xfer_err = 0;
    ctlr.msg_tx_idx = 0;
    ctlr.msg_rx_idx = 0;
    ctlr.buf_tx_idx = 0;
    ctlr.buf_rx_idx = 0;
    }
//
// Initialize the transfer information and start the I2C bus transfer.
// We only configure the transfer and do some pre/post works here, and
// wait for the transfer done. The major transfer process is performed
// in the IRQ handler.
//
    static int hisi_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    struct hisi_i2c_controller *ctlr = i2c_get_adapdata(adap);
    DECLARE_COMPLETION_ONSTACK(done);
    let mut ret: c_int = num;
    hisi_i2c_reset_xfer(ctlr);
    ctlr.completion = &done;
    ctlr.msg_num = num;
    ctlr.msgs = msgs;
    hisi_i2c_start_xfer(ctlr);
    if (!wait_for_completion_timeout(ctlr.completion, adap.timeout)) {
    hisi_i2c_disable_int(ctlr, HISI_I2C_INT_ALL);
    synchronize_irq(ctlr.irq);
    i2c_recover_bus(&ctlr.adapter);
    dev_err(ctlr.dev, "bus transfer timeout\n");
    ret = -EIO;
    }
    if (ctlr.xfer_err) {
    hisi_i2c_handle_errors(ctlr);
    ret = -EIO;
    }
    hisi_i2c_reset_xfer(ctlr);
    ctlr.completion = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 hisi_i2c_functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_10BIT_ADDR | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm hisi_i2c_algo = {
    .xfer = hisi_i2c_xfer,
    .functionality = hisi_i2c_functionality,
    };
#[no_mangle]
unsafe extern "C" fn hisi_i2c_read_rx_fifo(ctlr: *mut hisi_i2c_controller) -> c_int {
    static int hisi_i2c_read_rx_fifo(struct hisi_i2c_controller *ctlr)
    {
    struct i2c_msg *cur_msg;
    u32 fifo_state;
    while (ctlr.msg_rx_idx < ctlr.msg_num) {
    cur_msg = ctlr.msgs + ctlr.msg_rx_idx;
    if (!(cur_msg.flags & I2C_M_RD)) {
    ctlr.msg_rx_idx++;
    continue;
    }
    fifo_state = readl(ctlr.iobase + HISI_I2C_FIFO_STATE);
    while (!(fifo_state & HISI_I2C_FIFO_STATE_RX_EMPTY) &&
    ctlr.buf_rx_idx < cur_msg.len) {
    cur_msg.buf[ctlr.buf_rx_idx++] = readl(ctlr.iobase + HISI_I2C_RXDATA);
    fifo_state = readl(ctlr.iobase + HISI_I2C_FIFO_STATE);
    }
    if (ctlr.buf_rx_idx == cur_msg.len) {
    ctlr.buf_rx_idx = 0;
    ctlr.msg_rx_idx++;
    }
    if (fifo_state & HISI_I2C_FIFO_STATE_RX_EMPTY)
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_xfer_msg(ctlr: *mut hisi_i2c_controller) {
    static void hisi_i2c_xfer_msg(struct hisi_i2c_controller *ctlr)
    {
    let mut max_write: c_int = HISI_I2C_TX_FIFO_DEPTH - HISI_I2C_TX_F_AE_THRESH;
    let mut need_restart: bool = false, last_msg;
    struct i2c_msg *cur_msg;
    u32 cmd, fifo_state;
    while (ctlr.msg_tx_idx < ctlr.msg_num) {
    cur_msg = ctlr.msgs + ctlr.msg_tx_idx;
    last_msg = (ctlr.msg_tx_idx == ctlr.msg_num - 1);
// Signal the SR bit when we start transferring a new message
    if (ctlr.msg_tx_idx && !ctlr.buf_tx_idx)
    need_restart = true;
    fifo_state = readl(ctlr.iobase + HISI_I2C_FIFO_STATE);
    while (!(fifo_state & HISI_I2C_FIFO_STATE_TX_FULL) &&
    ctlr.buf_tx_idx < cur_msg.len && max_write) {
    cmd = 0;
    if (need_restart) {
    cmd |= HISI_I2C_CMD_TXDATA_SR_EN;
    need_restart = false;
    }
// Signal the STOP bit at the last frame of the last message
    if (ctlr.buf_tx_idx == cur_msg.len - 1 && last_msg)
    cmd |= HISI_I2C_CMD_TXDATA_P_EN;
    if (cur_msg.flags & I2C_M_RD)
    cmd |= HISI_I2C_CMD_TXDATA_RW;
    else
    cmd |= FIELD_PREP(HISI_I2C_CMD_TXDATA_DATA,
    cur_msg.buf[ctlr.buf_tx_idx]);
    writel(cmd, ctlr.iobase + HISI_I2C_CMD_TXDATA);
    ctlr.buf_tx_idx++;
    max_write--;
    fifo_state = readl(ctlr.iobase + HISI_I2C_FIFO_STATE);
    }
// Update the transfer index after per message transfer is done.
    if (ctlr.buf_tx_idx == cur_msg.len) {
    ctlr.buf_tx_idx = 0;
    ctlr.msg_tx_idx++;
    }
    if ((fifo_state & HISI_I2C_FIFO_STATE_TX_FULL) ||
    max_write == 0)
    break;
    }
//
// Disable the TX_EMPTY interrupt after finishing all the messages to
// avoid overwhelming the CPU.
//
    if (ctlr.msg_tx_idx == ctlr.msg_num)
    hisi_i2c_disable_int(ctlr, HISI_I2C_INT_TX_EMPTY);
    hisi_i2c_clear_tx_int(ctlr, HISI_I2C_TX_AEMPTY_INT);
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_irq(irq: c_int, context: *mut c_void) -> irqreturn_t {
    static irqreturn_t hisi_i2c_irq(int irq, void *context)
    {
    struct hisi_i2c_controller *ctlr = context;
    u32 int_stat;
//
// Don't handle the interrupt if cltr->completion is NULL. We may
// reach here because the interrupt is spurious or the transfer is
// started by another port (e.g. firmware) rather than us.
//
    if (!ctlr.completion)
    return IRQ_NONE;
    int_stat = readl(ctlr.iobase + HISI_I2C_INT_MSTAT);
    hisi_i2c_clear_int(ctlr, int_stat);
    if (!(int_stat & HISI_I2C_INT_ALL))
    return IRQ_NONE;
    if (int_stat & HISI_I2C_INT_TX_EMPTY)
    hisi_i2c_xfer_msg(ctlr);
    if (int_stat & HISI_I2C_INT_ERR) {
    ctlr.xfer_err = int_stat;
    goto out;
    }
// Drain the rx fifo before finish the transfer
    if (int_stat & (HISI_I2C_INT_TRANS_CPLT | HISI_I2C_INT_RX_FULL))
    hisi_i2c_read_rx_fifo(ctlr);
    out:
//
// Only use TRANS_CPLT to indicate the completion. On error cases we'll
// get two interrupts, INT_ERR first then TRANS_CPLT.
//
    if (int_stat & HISI_I2C_INT_TRANS_CPLT) {
    hisi_i2c_disable_int(ctlr, HISI_I2C_INT_ALL);
    hisi_i2c_clear_int(ctlr, HISI_I2C_INT_ALL);
    hisi_i2c_clear_tx_int(ctlr, HISI_I2C_TX_AEMPTY_INT);
    complete(ctlr.completion);
    }
    return IRQ_HANDLED;
    }
//
// Helper function for calculating and configuring the HIGH and LOW
// periods of SCL clock. The caller will pass the ratio of the
// counts (divide / divisor) according to the target speed mode,
// and the target registers.
//
    static void hisi_i2c_set_scl(struct hisi_i2c_controller *ctlr,
    u32 divide, u32 divisor,
    u32 reg_hcnt, u32 reg_lcnt)
    {
    u32 total_cnt, t_scl_hcnt, t_scl_lcnt, scl_fall_cnt, scl_rise_cnt;
    u32 scl_hcnt, scl_lcnt;
// Total SCL clock cycles per speed period
    total_cnt = DIV_ROUND_UP_ULL(ctlr.clk_rate_khz * HZ_PER_KHZ, ctlr.t.bus_freq_hz);
// Total HIGH level SCL clock cycles including edges
    t_scl_hcnt = DIV_ROUND_UP_ULL(total_cnt * divide, divisor);
// Total LOW level SCL clock cycles including edges
    t_scl_lcnt = total_cnt - t_scl_hcnt;
// Fall edge SCL clock cycles
    scl_fall_cnt = NSEC_TO_CYCLES(ctlr.t.scl_fall_ns, ctlr.clk_rate_khz);
// Rise edge SCL clock cycles
    scl_rise_cnt = NSEC_TO_CYCLES(ctlr.t.scl_rise_ns, ctlr.clk_rate_khz);
// Calculated HIGH and LOW periods of SCL clock
    scl_hcnt = t_scl_hcnt - ctlr.spk_len - 7 - scl_fall_cnt;
    scl_lcnt = t_scl_lcnt - 1 - scl_rise_cnt;
    writel(scl_hcnt, ctlr.iobase + reg_hcnt);
    writel(scl_lcnt, ctlr.iobase + reg_lcnt);
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_configure_bus(ctlr: *mut hisi_i2c_controller) {
    static void hisi_i2c_configure_bus(struct hisi_i2c_controller *ctlr)
    {
    u32 reg, sda_hold_cnt, speed_mode;
    i2c_parse_fw_timings(ctlr.dev, &ctlr.t, true);
    ctlr.spk_len = NSEC_TO_CYCLES(ctlr.t.digital_filter_width_ns, ctlr.clk_rate_khz);
    switch (ctlr.t.bus_freq_hz) {
    case I2C_MAX_FAST_MODE_FREQ:
    speed_mode = HISI_I2C_FAST_SPEED_MODE;
    hisi_i2c_set_scl(ctlr, 26, 76, HISI_I2C_FS_SCL_HCNT, HISI_I2C_FS_SCL_LCNT);
    break;
    case I2C_MAX_HIGH_SPEED_MODE_FREQ:
    speed_mode = HISI_I2C_HIGH_SPEED_MODE;
    hisi_i2c_set_scl(ctlr, 6, 22, HISI_I2C_HS_SCL_HCNT, HISI_I2C_HS_SCL_LCNT);
    break;
    case I2C_MAX_STANDARD_MODE_FREQ:
    default:
    speed_mode = HISI_I2C_STD_SPEED_MODE;
// For default condition force the bus speed to standard mode.
    ctlr.t.bus_freq_hz = I2C_MAX_STANDARD_MODE_FREQ;
    hisi_i2c_set_scl(ctlr, 40, 87, HISI_I2C_SS_SCL_HCNT, HISI_I2C_SS_SCL_LCNT);
    break;
    }
    reg = readl(ctlr.iobase + HISI_I2C_FRAME_CTRL);
    reg &= ~HISI_I2C_FRAME_CTRL_SPEED_MODE;
    reg |= FIELD_PREP(HISI_I2C_FRAME_CTRL_SPEED_MODE, speed_mode);
    writel(reg, ctlr.iobase + HISI_I2C_FRAME_CTRL);
    sda_hold_cnt = NSEC_TO_CYCLES(ctlr.t.sda_hold_ns, ctlr.clk_rate_khz);
    reg = FIELD_PREP(HISI_I2C_SDA_HOLD_TX, sda_hold_cnt);
    writel(reg, ctlr.iobase + HISI_I2C_SDA_HOLD);
    writel(ctlr.spk_len, ctlr.iobase + HISI_I2C_FS_SPK_LEN);
    reg = FIELD_PREP(HISI_I2C_FIFO_RX_AF_THRESH, HISI_I2C_RX_F_AF_THRESH);
    reg |= FIELD_PREP(HISI_I2C_FIFO_TX_AE_THRESH, HISI_I2C_TX_F_AE_THRESH);
    writel(reg, ctlr.iobase + HISI_I2C_FIFO_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hisi_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_i2c_probe(struct platform_device *pdev)
    {
    struct hisi_i2c_controller *ctlr;
    struct device *dev = &pdev.dev;
    struct i2c_adapter *adapter;
    u64 clk_rate_hz;
    u32 hw_version;
    int ret;
    ctlr = devm_kzalloc(dev, sizeof(*ctlr), GFP_KERNEL);
    if (!ctlr)
    return -ENOMEM;
    ctlr.iobase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctlr.iobase))
    return PTR_ERR(ctlr.iobase);
    ctlr.irq = platform_get_irq(pdev, 0);
    if (ctlr.irq < 0)
    return ctlr.irq;
    ctlr.dev = dev;
    hisi_i2c_disable_int(ctlr, HISI_I2C_INT_ALL);
    ret = devm_request_irq(dev, ctlr.irq, hisi_i2c_irq, 0, "hisi-i2c", ctlr);
    if (ret)
    return ret;
    ctlr.clk = devm_clk_get_optional_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR_OR_NULL(ctlr.clk)) {
    ret = device_property_read_u64(dev, "clk_rate", &clk_rate_hz);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get clock frequency\n");
    } else {
    clk_rate_hz = clk_get_rate(ctlr.clk);
    }
    ctlr.clk_rate_khz = DIV_ROUND_UP_ULL(clk_rate_hz, HZ_PER_KHZ);
    hisi_i2c_configure_bus(ctlr);
    adapter = &ctlr.adapter;
    snprintf(adapter.name, sizeof(adapter.name),
    "HiSilicon I2C Controller %s", dev_name(dev));
    adapter.owner = THIS_MODULE;
    adapter.algo = &hisi_i2c_algo;
    adapter.dev.parent = dev;
    i2c_set_adapdata(adapter, ctlr);
    ret = devm_i2c_add_adapter(dev, adapter);
    if (ret)
    return ret;
    hw_version = readl(ctlr.iobase + HISI_I2C_VERSION);
    dev_info(ctlr.dev, "speed mode is %s. hw version 0x%x\n",
    i2c_freq_mode_string(ctlr.t.bus_freq_hz), hw_version);
    return 0;
    }
    static const struct acpi_device_id hisi_i2c_acpi_ids[] = {
    { "HISI03D1", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, hisi_i2c_acpi_ids);
    static const struct of_device_id hisi_i2c_dts_ids[] = {
    { .compatible = "hisilicon,ascend910-i2c", },
    { }
    };
    MODULE_DEVICE_TABLE(of, hisi_i2c_dts_ids);
    static struct platform_driver hisi_i2c_driver = {
    .probe		= hisi_i2c_probe,
    .driver		= {
    .name	= "hisi-i2c",
    .acpi_match_table = hisi_i2c_acpi_ids,
    .of_match_table = hisi_i2c_dts_ids,
    },
    };
    module_platform_driver(hisi_i2c_driver);
    MODULE_AUTHOR("Yicong Yang <yangyicong@hisilicon.com>");
    MODULE_DESCRIPTION("HiSilicon I2C Controller Driver");
    MODULE_LICENSE("GPL");
