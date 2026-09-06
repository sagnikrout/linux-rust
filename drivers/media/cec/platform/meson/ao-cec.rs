//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/platform/meson/ao-cec.c
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


//
// Driver for Amlogic Meson AO CEC Controller
//
// Copyright (C) 2015 Amlogic, Inc. All rights reserved
// Copyright (C) 2017 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
//
// SPDX-License-Identifier: GPL-2.0+
//

// CEC Registers
//
// [2:1] cntl_clk
// - 0 = Disable clk (Power-off mode)
// - 1 = Enable gated clock (Normal mode)
// - 2 = Enable free-run clk (Debug mode)
//
pub const CEC_GEN_CNTL_REG: c_uint = 0x00;

pub const CEC_GEN_CNTL_CLK_DISABLE: c_int = 0;
pub const CEC_GEN_CNTL_CLK_ENABLE: c_int = 1;
pub const CEC_GEN_CNTL_CLK_ENABLE_DBG: c_int = 2;

//
// [7:0] cec_reg_addr
// [15:8] cec_reg_wrdata
// [16] cec_reg_wr
// - 0 = Read
// - 1 = Write
// [23] bus free
// [31:24] cec_reg_rddata
//
pub const CEC_RW_REG: c_uint = 0x04;

//
// [1] tx intr
// [2] rx intr
//
pub const CEC_INTR_MASKN_REG: c_uint = 0x08;
pub const CEC_INTR_CLR_REG: c_uint = 0x0c;
pub const CEC_INTR_STAT_REG: c_uint = 0x10;

// CEC Commands
pub const CEC_TX_MSG_0_HEADER: c_uint = 0x00;
pub const CEC_TX_MSG_1_OPCODE: c_uint = 0x01;
pub const CEC_TX_MSG_2_OP1: c_uint = 0x02;
pub const CEC_TX_MSG_3_OP2: c_uint = 0x03;
pub const CEC_TX_MSG_4_OP3: c_uint = 0x04;
pub const CEC_TX_MSG_5_OP4: c_uint = 0x05;
pub const CEC_TX_MSG_6_OP5: c_uint = 0x06;
pub const CEC_TX_MSG_7_OP6: c_uint = 0x07;
pub const CEC_TX_MSG_8_OP7: c_uint = 0x08;
pub const CEC_TX_MSG_9_OP8: c_uint = 0x09;
pub const CEC_TX_MSG_A_OP9: c_uint = 0x0A;
pub const CEC_TX_MSG_B_OP10: c_uint = 0x0B;
pub const CEC_TX_MSG_C_OP11: c_uint = 0x0C;
pub const CEC_TX_MSG_D_OP12: c_uint = 0x0D;
pub const CEC_TX_MSG_E_OP13: c_uint = 0x0E;
pub const CEC_TX_MSG_F_OP14: c_uint = 0x0F;
pub const CEC_TX_MSG_LENGTH: c_uint = 0x10;
pub const CEC_TX_MSG_CMD: c_uint = 0x11;
pub const CEC_TX_WRITE_BUF: c_uint = 0x12;
pub const CEC_TX_CLEAR_BUF: c_uint = 0x13;
pub const CEC_RX_MSG_CMD: c_uint = 0x14;
pub const CEC_RX_CLEAR_BUF: c_uint = 0x15;
pub const CEC_LOGICAL_ADDR0: c_uint = 0x16;
pub const CEC_LOGICAL_ADDR1: c_uint = 0x17;
pub const CEC_LOGICAL_ADDR2: c_uint = 0x18;
pub const CEC_LOGICAL_ADDR3: c_uint = 0x19;
pub const CEC_LOGICAL_ADDR4: c_uint = 0x1A;
pub const CEC_CLOCK_DIV_H: c_uint = 0x1B;
pub const CEC_CLOCK_DIV_L: c_uint = 0x1C;
pub const CEC_QUIESCENT_25MS_BIT7_0: c_uint = 0x20;
pub const CEC_QUIESCENT_25MS_BIT11_8: c_uint = 0x21;
pub const CEC_STARTBITMINL2H_3MS5_BIT7_0: c_uint = 0x22;
pub const CEC_STARTBITMINL2H_3MS5_BIT8: c_uint = 0x23;
pub const CEC_STARTBITMAXL2H_3MS9_BIT7_0: c_uint = 0x24;
pub const CEC_STARTBITMAXL2H_3MS9_BIT8: c_uint = 0x25;
pub const CEC_STARTBITMINH_0MS6_BIT7_0: c_uint = 0x26;
pub const CEC_STARTBITMINH_0MS6_BIT8: c_uint = 0x27;
pub const CEC_STARTBITMAXH_1MS0_BIT7_0: c_uint = 0x28;
pub const CEC_STARTBITMAXH_1MS0_BIT8: c_uint = 0x29;
pub const CEC_STARTBITMINTOT_4MS3_BIT7_0: c_uint = 0x2A;
pub const CEC_STARTBITMINTOT_4MS3_BIT9_8: c_uint = 0x2B;
pub const CEC_STARTBITMAXTOT_4MS7_BIT7_0: c_uint = 0x2C;
pub const CEC_STARTBITMAXTOT_4MS7_BIT9_8: c_uint = 0x2D;
pub const CEC_LOGIC1MINL2H_0MS4_BIT7_0: c_uint = 0x2E;
pub const CEC_LOGIC1MINL2H_0MS4_BIT8: c_uint = 0x2F;
pub const CEC_LOGIC1MAXL2H_0MS8_BIT7_0: c_uint = 0x30;
pub const CEC_LOGIC1MAXL2H_0MS8_BIT8: c_uint = 0x31;
pub const CEC_LOGIC0MINL2H_1MS3_BIT7_0: c_uint = 0x32;
pub const CEC_LOGIC0MINL2H_1MS3_BIT8: c_uint = 0x33;
pub const CEC_LOGIC0MAXL2H_1MS7_BIT7_0: c_uint = 0x34;
pub const CEC_LOGIC0MAXL2H_1MS7_BIT8: c_uint = 0x35;
pub const CEC_LOGICMINTOTAL_2MS05_BIT7_0: c_uint = 0x36;
pub const CEC_LOGICMINTOTAL_2MS05_BIT9_8: c_uint = 0x37;
pub const CEC_LOGICMAXHIGH_2MS8_BIT7_0: c_uint = 0x38;
pub const CEC_LOGICMAXHIGH_2MS8_BIT8: c_uint = 0x39;
pub const CEC_LOGICERRLOW_3MS4_BIT7_0: c_uint = 0x3A;
pub const CEC_LOGICERRLOW_3MS4_BIT8: c_uint = 0x3B;
pub const CEC_NOMSMPPOINT_1MS05: c_uint = 0x3C;
pub const CEC_DELCNTR_LOGICERR: c_uint = 0x3E;
pub const CEC_TXTIME_17MS_BIT7_0: c_uint = 0x40;
pub const CEC_TXTIME_17MS_BIT10_8: c_uint = 0x41;
pub const CEC_TXTIME_2BIT_BIT7_0: c_uint = 0x42;
pub const CEC_TXTIME_2BIT_BIT10_8: c_uint = 0x43;
pub const CEC_TXTIME_4BIT_BIT7_0: c_uint = 0x44;
pub const CEC_TXTIME_4BIT_BIT10_8: c_uint = 0x45;
pub const CEC_STARTBITNOML2H_3MS7_BIT7_0: c_uint = 0x46;
pub const CEC_STARTBITNOML2H_3MS7_BIT8: c_uint = 0x47;
pub const CEC_STARTBITNOMH_0MS8_BIT7_0: c_uint = 0x48;
pub const CEC_STARTBITNOMH_0MS8_BIT8: c_uint = 0x49;
pub const CEC_LOGIC1NOML2H_0MS6_BIT7_0: c_uint = 0x4A;
pub const CEC_LOGIC1NOML2H_0MS6_BIT8: c_uint = 0x4B;
pub const CEC_LOGIC0NOML2H_1MS5_BIT7_0: c_uint = 0x4C;
pub const CEC_LOGIC0NOML2H_1MS5_BIT8: c_uint = 0x4D;
pub const CEC_LOGIC1NOMH_1MS8_BIT7_0: c_uint = 0x4E;
pub const CEC_LOGIC1NOMH_1MS8_BIT8: c_uint = 0x4F;
pub const CEC_LOGIC0NOMH_0MS9_BIT7_0: c_uint = 0x50;
pub const CEC_LOGIC0NOMH_0MS9_BIT8: c_uint = 0x51;
pub const CEC_LOGICERRLOW_3MS6_BIT7_0: c_uint = 0x52;
pub const CEC_LOGICERRLOW_3MS6_BIT8: c_uint = 0x53;
pub const CEC_CHKCONTENTION_0MS1: c_uint = 0x54;
pub const CEC_PREPARENXTBIT_0MS05_BIT7_0: c_uint = 0x56;
pub const CEC_PREPARENXTBIT_0MS05_BIT8: c_uint = 0x57;
pub const CEC_NOMSMPACKPOINT_0MS45: c_uint = 0x58;
pub const CEC_ACK0NOML2H_1MS5_BIT7_0: c_uint = 0x5A;
pub const CEC_ACK0NOML2H_1MS5_BIT8: c_uint = 0x5B;
pub const CEC_BUGFIX_DISABLE_0: c_uint = 0x60;
pub const CEC_BUGFIX_DISABLE_1: c_uint = 0x61;
pub const CEC_RX_MSG_0_HEADER: c_uint = 0x80;
pub const CEC_RX_MSG_1_OPCODE: c_uint = 0x81;
pub const CEC_RX_MSG_2_OP1: c_uint = 0x82;
pub const CEC_RX_MSG_3_OP2: c_uint = 0x83;
pub const CEC_RX_MSG_4_OP3: c_uint = 0x84;
pub const CEC_RX_MSG_5_OP4: c_uint = 0x85;
pub const CEC_RX_MSG_6_OP5: c_uint = 0x86;
pub const CEC_RX_MSG_7_OP6: c_uint = 0x87;
pub const CEC_RX_MSG_8_OP7: c_uint = 0x88;
pub const CEC_RX_MSG_9_OP8: c_uint = 0x89;
pub const CEC_RX_MSG_A_OP9: c_uint = 0x8A;
pub const CEC_RX_MSG_B_OP10: c_uint = 0x8B;
pub const CEC_RX_MSG_C_OP11: c_uint = 0x8C;
pub const CEC_RX_MSG_D_OP12: c_uint = 0x8D;
pub const CEC_RX_MSG_E_OP13: c_uint = 0x8E;
pub const CEC_RX_MSG_F_OP14: c_uint = 0x8F;
pub const CEC_RX_MSG_LENGTH: c_uint = 0x90;
pub const CEC_RX_MSG_STATUS: c_uint = 0x91;
pub const CEC_RX_NUM_MSG: c_uint = 0x92;
pub const CEC_TX_MSG_STATUS: c_uint = 0x93;
pub const CEC_TX_NUM_MSG: c_uint = 0x94;
// CEC_TX_MSG_CMD definition

// tx_msg_status definition

// rx_msg_cmd

// rx_msg_status

// RX_CLEAR_BUF options
pub const CLEAR_START: c_int = 1;
pub const CLEAR_STOP: c_int = 0;
// CEC_LOGICAL_ADDRx options
pub const LOGICAL_ADDR_MASK: c_uint = 0xf;

pub const LOGICAL_ADDR_DISABLE: c_int = 0;
pub const CEC_CLK_RATE: c_int = 32768;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_ao_cec_device {
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub core: *mut clk,
    pub cec_reg_lock: spinlock_t,
    pub notify: *mut cec_notifier,
    pub adap: *mut cec_adapter,
    pub rx_msg: cec_msg,
}

    writel_relaxed((readl_relaxed(addr) & ~(mask)) | (val), addr)
#[no_mangle]
pub unsafe extern "C" fn meson_ao_cec_wait_busy(ao_cec: *mut meson_ao_cec_device) -> c_int {
    static inline int meson_ao_cec_wait_busy(struct meson_ao_cec_device *ao_cec)
    {
    let mut timeout: ktime_t = ktime_add_us(ktime_get(), 5000);
    while (readl_relaxed(ao_cec.base + CEC_RW_REG) & CEC_RW_BUS_BUSY) {
    if (ktime_compare(ktime_get(), timeout) > 0)
    return -ETIMEDOUT;
    }
    return 0;
    }
    static void meson_ao_cec_read(struct meson_ao_cec_device *ao_cec,
    unsigned long address, u8 *data,
    int *res)
    {
    unsigned long flags;
    let mut reg: u32 = FIELD_PREP(CEC_RW_ADDR, address);
    let mut ret: c_int = 0;
    if (res && *res)
    return;
    spin_lock_irqsave(&ao_cec.cec_reg_lock, flags);
    ret = meson_ao_cec_wait_busy(ao_cec);
    if (ret)
    goto read_out;
    writel_relaxed(reg, ao_cec.base + CEC_RW_REG);
    ret = meson_ao_cec_wait_busy(ao_cec);
    if (ret)
    goto read_out;
// data = FIELD_GET(CEC_RW_RD_DATA,
    readl_relaxed(ao_cec.base + CEC_RW_REG));
    read_out:
    spin_unlock_irqrestore(&ao_cec.cec_reg_lock, flags);
    if (res)
// res = ret;
    }
    static void meson_ao_cec_write(struct meson_ao_cec_device *ao_cec,
    unsigned long address, u8 data,
    int *res)
    {
    unsigned long flags;
    u32 reg = FIELD_PREP(CEC_RW_ADDR, address) |
    FIELD_PREP(CEC_RW_WR_DATA, data) |
    CEC_RW_WRITE_EN;
    let mut ret: c_int = 0;
    if (res && *res)
    return;
    spin_lock_irqsave(&ao_cec.cec_reg_lock, flags);
    ret = meson_ao_cec_wait_busy(ao_cec);
    if (ret)
    goto write_out;
    writel_relaxed(reg, ao_cec.base + CEC_RW_REG);
    write_out:
    spin_unlock_irqrestore(&ao_cec.cec_reg_lock, flags);
    if (res)
// res = ret;
    }
    static inline void meson_ao_cec_irq_setup(struct meson_ao_cec_device *ao_cec,
    bool enable)
    {
    let mut cfg: u32 = CEC_INTR_TX | CEC_INTR_RX;
    writel_bits_relaxed(cfg, enable ? cfg : 0,
    ao_cec.base + CEC_INTR_MASKN_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn meson_ao_cec_clear(ao_cec: *mut meson_ao_cec_device) -> c_int {
    static inline int meson_ao_cec_clear(struct meson_ao_cec_device *ao_cec)
    {
    let mut ret: c_int = 0;
    meson_ao_cec_write(ao_cec, CEC_RX_MSG_CMD, RX_DISABLE, &ret);
    meson_ao_cec_write(ao_cec, CEC_TX_MSG_CMD, TX_ABORT, &ret);
    meson_ao_cec_write(ao_cec, CEC_RX_CLEAR_BUF, 1, &ret);
    meson_ao_cec_write(ao_cec, CEC_TX_CLEAR_BUF, 1, &ret);
    if (ret)
    return ret;
    udelay(100);
    meson_ao_cec_write(ao_cec, CEC_RX_CLEAR_BUF, 0, &ret);
    meson_ao_cec_write(ao_cec, CEC_TX_CLEAR_BUF, 0, &ret);
    if (ret)
    return ret;
    udelay(100);
    meson_ao_cec_write(ao_cec, CEC_RX_MSG_CMD, RX_NO_OP, &ret);
    meson_ao_cec_write(ao_cec, CEC_TX_MSG_CMD, TX_NO_OP, &ret);
    return ret;
    }
    static int meson_ao_cec_arbit_bit_time_set(struct meson_ao_cec_device *ao_cec,
    unsigned int bit_set,
    unsigned int time_set)
    {
    let mut ret: c_int = 0;
    switch (bit_set) {
    case CEC_SIGNAL_FREE_TIME_RETRY:
    meson_ao_cec_write(ao_cec, CEC_TXTIME_4BIT_BIT7_0,
    time_set & 0xff, &ret);
    meson_ao_cec_write(ao_cec, CEC_TXTIME_4BIT_BIT10_8,
    (time_set >> 8) & 0x7, &ret);
    break;
    case CEC_SIGNAL_FREE_TIME_NEW_INITIATOR:
    meson_ao_cec_write(ao_cec, CEC_TXTIME_2BIT_BIT7_0,
    time_set & 0xff, &ret);
    meson_ao_cec_write(ao_cec, CEC_TXTIME_2BIT_BIT10_8,
    (time_set >> 8) & 0x7, &ret);
    break;
    case CEC_SIGNAL_FREE_TIME_NEXT_XFER:
    meson_ao_cec_write(ao_cec, CEC_TXTIME_17MS_BIT7_0,
    time_set & 0xff, &ret);
    meson_ao_cec_write(ao_cec, CEC_TXTIME_17MS_BIT10_8,
    (time_set >> 8) & 0x7, &ret);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_ao_cec_irq(int irq, void *data)
    {
    struct meson_ao_cec_device *ao_cec = data;
    let mut stat: u32 = readl_relaxed(ao_cec.base + CEC_INTR_STAT_REG);
    if (stat)
    return IRQ_WAKE_THREAD;
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_irq_tx(ao_cec: *mut meson_ao_cec_device) {
    static void meson_ao_cec_irq_tx(struct meson_ao_cec_device *ao_cec)
    {
    let mut tx_status: c_ulong = 0;
    u8 stat;
    let mut ret: c_int = 0;
    meson_ao_cec_read(ao_cec, CEC_TX_MSG_STATUS, &stat, &ret);
    if (ret)
    goto tx_reg_err;
    switch (stat) {
    case TX_DONE:
    tx_status = CEC_TX_STATUS_OK;
    break;
    case TX_BUSY:
    tx_status = CEC_TX_STATUS_ARB_LOST;
    break;
    case TX_IDLE:
    tx_status = CEC_TX_STATUS_LOW_DRIVE;
    break;
    case TX_ERROR:
    default:
    tx_status = CEC_TX_STATUS_NACK;
    break;
    }
// Clear Interruption
    writel_relaxed(CEC_INTR_TX, ao_cec.base + CEC_INTR_CLR_REG);
// Stop TX
    meson_ao_cec_write(ao_cec, CEC_TX_MSG_CMD, TX_NO_OP, &ret);
    if (ret)
    goto tx_reg_err;
    cec_transmit_attempt_done(ao_cec.adap, tx_status);
    return;
    tx_reg_err:
    cec_transmit_attempt_done(ao_cec.adap, CEC_TX_STATUS_ERROR);
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_irq_rx(ao_cec: *mut meson_ao_cec_device) {
    static void meson_ao_cec_irq_rx(struct meson_ao_cec_device *ao_cec)
    {
    int i, ret = 0;
    u8 reg;
    meson_ao_cec_read(ao_cec, CEC_RX_MSG_STATUS, &reg, &ret);
    if (reg != RX_DONE)
    goto rx_out;
    meson_ao_cec_read(ao_cec, CEC_RX_NUM_MSG, &reg, &ret);
    if (reg != 1)
    goto rx_out;
    meson_ao_cec_read(ao_cec, CEC_RX_MSG_LENGTH, &reg, &ret);
    ao_cec.rx_msg.len = reg + 1;
    if (ao_cec.rx_msg.len > CEC_MAX_MSG_SIZE)
    ao_cec.rx_msg.len = CEC_MAX_MSG_SIZE;
    for (i = 0; i < ao_cec.rx_msg.len; i++) {
    u8 byte;
    meson_ao_cec_read(ao_cec, CEC_RX_MSG_0_HEADER + i, &byte, &ret);
    ao_cec.rx_msg.msg[i] = byte;
    }
    if (ret)
    goto rx_out;
    cec_received_msg(ao_cec.adap, &ao_cec.rx_msg);
    rx_out:
// Clear Interruption
    writel_relaxed(CEC_INTR_RX, ao_cec.base + CEC_INTR_CLR_REG);
// Ack RX message
    meson_ao_cec_write(ao_cec, CEC_RX_MSG_CMD, RX_ACK_CURRENT, &ret);
    meson_ao_cec_write(ao_cec, CEC_RX_MSG_CMD, RX_NO_OP, &ret);
// Clear RX buffer
    meson_ao_cec_write(ao_cec, CEC_RX_CLEAR_BUF, CLEAR_START, &ret);
    meson_ao_cec_write(ao_cec, CEC_RX_CLEAR_BUF, CLEAR_STOP, &ret);
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_ao_cec_irq_thread(int irq, void *data)
    {
    struct meson_ao_cec_device *ao_cec = data;
    let mut stat: u32 = readl_relaxed(ao_cec.base + CEC_INTR_STAT_REG);
    if (stat & CEC_INTR_TX)
    meson_ao_cec_irq_tx(ao_cec);
    meson_ao_cec_irq_rx(ao_cec);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_set_log_addr(adap: *mut cec_adapter, logical_addr: u8) -> c_int {
    static int meson_ao_cec_set_log_addr(struct cec_adapter *adap, u8 logical_addr)
    {
    struct meson_ao_cec_device *ao_cec = adap.priv;
    let mut ret: c_int = 0;
    meson_ao_cec_write(ao_cec, CEC_LOGICAL_ADDR0,
    LOGICAL_ADDR_DISABLE, &ret);
    if (ret)
    return ret;
    ret = meson_ao_cec_clear(ao_cec);
    if (ret)
    return ret;
    if (logical_addr == CEC_LOG_ADDR_INVALID)
    return 0;
    meson_ao_cec_write(ao_cec, CEC_LOGICAL_ADDR0,
    logical_addr & LOGICAL_ADDR_MASK, &ret);
    if (ret)
    return ret;
    udelay(100);
    meson_ao_cec_write(ao_cec, CEC_LOGICAL_ADDR0,
    (logical_addr & LOGICAL_ADDR_MASK) |
    LOGICAL_ADDR_VALID, &ret);
    return ret;
    }
    static int meson_ao_cec_transmit(struct cec_adapter *adap, u8 attempts,
    u32 signal_free_time, struct cec_msg *msg)
    {
    struct meson_ao_cec_device *ao_cec = adap.priv;
    int i, ret = 0;
    u8 reg;
    meson_ao_cec_read(ao_cec, CEC_TX_MSG_STATUS, &reg, &ret);
    if (ret)
    return ret;
    if (reg == TX_BUSY) {
    dev_dbg(&ao_cec.pdev.dev, "%s: busy TX: aborting\n",
    __func__);
    meson_ao_cec_write(ao_cec, CEC_TX_MSG_CMD, TX_ABORT, &ret);
    }
    for (i = 0; i < msg.len; i++) {
    meson_ao_cec_write(ao_cec, CEC_TX_MSG_0_HEADER + i,
    msg.msg[i], &ret);
    }
    meson_ao_cec_write(ao_cec, CEC_TX_MSG_LENGTH, msg.len - 1, &ret);
    meson_ao_cec_write(ao_cec, CEC_TX_MSG_CMD, TX_REQ_CURRENT, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_adap_enable(adap: *mut cec_adapter, enable: bool) -> c_int {
    static int meson_ao_cec_adap_enable(struct cec_adapter *adap, bool enable)
    {
    struct meson_ao_cec_device *ao_cec = adap.priv;
    int ret;
    meson_ao_cec_irq_setup(ao_cec, false);
    writel_bits_relaxed(CEC_GEN_CNTL_RESET, CEC_GEN_CNTL_RESET,
    ao_cec.base + CEC_GEN_CNTL_REG);
    if (!enable)
    return 0;
// Enable gated clock (Normal mode).
    writel_bits_relaxed(CEC_GEN_CNTL_CLK_CTRL_MASK,
    FIELD_PREP(CEC_GEN_CNTL_CLK_CTRL_MASK,
    CEC_GEN_CNTL_CLK_ENABLE),
    ao_cec.base + CEC_GEN_CNTL_REG);
    udelay(100);
// Release Reset
    writel_bits_relaxed(CEC_GEN_CNTL_RESET, 0,
    ao_cec.base + CEC_GEN_CNTL_REG);
// Clear buffers
    ret = meson_ao_cec_clear(ao_cec);
    if (ret)
    return ret;
// CEC arbitration 3/5/7 bit time set.
    ret = meson_ao_cec_arbit_bit_time_set(ao_cec,
    CEC_SIGNAL_FREE_TIME_RETRY,
    0x118);
    if (ret)
    return ret;
    ret = meson_ao_cec_arbit_bit_time_set(ao_cec,
    CEC_SIGNAL_FREE_TIME_NEW_INITIATOR,
    0x000);
    if (ret)
    return ret;
    ret = meson_ao_cec_arbit_bit_time_set(ao_cec,
    CEC_SIGNAL_FREE_TIME_NEXT_XFER,
    0x2aa);
    if (ret)
    return ret;
    meson_ao_cec_irq_setup(ao_cec, true);
    return 0;
    }
    static const struct cec_adap_ops meson_ao_cec_ops = {
    .adap_enable = meson_ao_cec_adap_enable,
    .adap_log_addr = meson_ao_cec_set_log_addr,
    .adap_transmit = meson_ao_cec_transmit,
    };
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_probe(pdev: *mut platform_device) -> c_int {
    static int meson_ao_cec_probe(struct platform_device *pdev)
    {
    struct meson_ao_cec_device *ao_cec;
    struct device *hdmi_dev;
    int ret, irq;
    hdmi_dev = cec_notifier_parse_hdmi_phandle(&pdev.dev);
    if (IS_ERR(hdmi_dev))
    return PTR_ERR(hdmi_dev);
    ao_cec = devm_kzalloc(&pdev.dev, sizeof(*ao_cec), GFP_KERNEL);
    if (!ao_cec)
    return -ENOMEM;
    spin_lock_init(&ao_cec.cec_reg_lock);
    ao_cec.adap = cec_allocate_adapter(&meson_ao_cec_ops, ao_cec,
    "meson_ao_cec",
    CEC_CAP_DEFAULTS |
    CEC_CAP_CONNECTOR_INFO,
    1); /* Use 1 for now */
    if (IS_ERR(ao_cec.adap))
    return PTR_ERR(ao_cec.adap);
    ao_cec.adap.owner = THIS_MODULE;
    ao_cec.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ao_cec.base)) {
    ret = PTR_ERR(ao_cec.base);
    goto out_probe_adapter;
    }
    irq = platform_get_irq(pdev, 0);
    ret = devm_request_threaded_irq(&pdev.dev, irq,
    meson_ao_cec_irq,
    meson_ao_cec_irq_thread,
    0, core::ptr::null_mut(), ao_cec);
    if (ret)
    goto out_probe_adapter;
    ao_cec.core = devm_clk_get(&pdev.dev, "core");
    if (IS_ERR(ao_cec.core)) {
    dev_err(&pdev.dev, "core clock request failed\n");
    ret = PTR_ERR(ao_cec.core);
    goto out_probe_adapter;
    }
    ret = clk_prepare_enable(ao_cec.core);
    if (ret) {
    dev_err(&pdev.dev, "core clock enable failed\n");
    goto out_probe_adapter;
    }
    ret = clk_set_rate(ao_cec.core, CEC_CLK_RATE);
    if (ret) {
    dev_err(&pdev.dev, "core clock set rate failed\n");
    goto out_probe_clk;
    }
    device_reset_optional(&pdev.dev);
    ao_cec.pdev = pdev;
    platform_set_drvdata(pdev, ao_cec);
    ao_cec.notify = cec_notifier_cec_adap_register(hdmi_dev, core::ptr::null_mut(),
    ao_cec.adap);
    if (!ao_cec.notify) {
    ret = -ENOMEM;
    goto out_probe_clk;
    }
    ret = cec_register_adapter(ao_cec.adap, &pdev.dev);
    if (ret < 0)
    goto out_probe_notify;
// Setup Hardware
    writel_relaxed(CEC_GEN_CNTL_RESET,
    ao_cec.base + CEC_GEN_CNTL_REG);
    return 0;
    out_probe_notify:
    cec_notifier_cec_adap_unregister(ao_cec.notify, ao_cec.adap);
    out_probe_clk:
    clk_disable_unprepare(ao_cec.core);
    out_probe_adapter:
    cec_delete_adapter(ao_cec.adap);
    dev_err(&pdev.dev, "CEC controller registration failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_remove(pdev: *mut platform_device) {
    static void meson_ao_cec_remove(struct platform_device *pdev)
    {
    struct meson_ao_cec_device *ao_cec = platform_get_drvdata(pdev);
    clk_disable_unprepare(ao_cec.core);
    cec_notifier_cec_adap_unregister(ao_cec.notify, ao_cec.adap);
    cec_unregister_adapter(ao_cec.adap);
    }
    static const struct of_device_id meson_ao_cec_of_match[] = {
    { .compatible = "amlogic,meson-gx-ao-cec", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, meson_ao_cec_of_match);
    static struct platform_driver meson_ao_cec_driver = {
    .probe   = meson_ao_cec_probe,
    .remove = meson_ao_cec_remove,
    .driver  = {
    .name = "meson-ao-cec",
    .of_match_table = meson_ao_cec_of_match,
    },
    };
    module_platform_driver(meson_ao_cec_driver);
    MODULE_DESCRIPTION("Meson AO CEC Controller driver");
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_LICENSE("GPL");
