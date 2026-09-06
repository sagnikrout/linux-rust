//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/platform/meson/ao-cec-g12a.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for Amlogic Meson AO CEC G12A Controller
//
// Copyright (C) 2017 Amlogic, Inc. All rights reserved
// Copyright (C) 2019 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
//

// CEC Registers
pub const CECB_CLK_CNTL_REG0: c_uint = 0x00;

pub const CECB_CLK_CNTL_REG1: c_uint = 0x04;

//
// [14:12] Filter_del. For glitch-filtering CEC line, ignore signal
// change pulse width < filter_del * T(filter_tick) * 3.
// [9:8] Filter_tick_sel: Select which periodical pulse for
// glitch-filtering CEC line signal.
// - 0=Use T(xtal)*3 = 125ns;
// - 1=Use once-per-1us pulse;
// - 2=Use once-per-10us pulse;
// - 3=Use once-per-100us pulse.
// [3]   Sysclk_en. 0=Disable system clock; 1=Enable system clock.
// [2:1] cntl_clk
// - 0 = Disable clk (Power-off mode)
// - 1 = Enable gated clock (Normal mode)
// - 2 = Enable free-run clk (Debug mode)
// [0] SW_RESET 1=Apply reset; 0=No reset.
//
pub const CECB_GEN_CNTL_REG: c_uint = 0x08;

pub const CECB_GEN_CNTL_CLK_DISABLE: c_int = 0;
pub const CECB_GEN_CNTL_CLK_ENABLE: c_int = 1;
pub const CECB_GEN_CNTL_CLK_ENABLE_DBG: c_int = 2;

pub const CECB_GEN_CNTL_FILTER_TICK_125NS: c_int = 0;
pub const CECB_GEN_CNTL_FILTER_TICK_1US: c_int = 1;
pub const CECB_GEN_CNTL_FILTER_TICK_10US: c_int = 2;
pub const CECB_GEN_CNTL_FILTER_TICK_100US: c_int = 3;

//
// [7:0] cec_reg_addr
// [15:8] cec_reg_wrdata
// [16] cec_reg_wr
// - 0 = Read
// - 1 = Write
// [31:24] cec_reg_rddata
//
pub const CECB_RW_REG: c_uint = 0x0c;

//
// [0] DONE Interrupt
// [1] End Of Message Interrupt
// [2] Not Acknowlegde Interrupt
// [3] Arbitration Loss Interrupt
// [4] Initiator Error Interrupt
// [5] Follower Error Interrupt
// [6] Wake-Up Interrupt
//
pub const CECB_INTR_MASKN_REG: c_uint = 0x10;
pub const CECB_INTR_CLR_REG: c_uint = 0x14;
pub const CECB_INTR_STAT_REG: c_uint = 0x18;

// CEC Commands
pub const CECB_CTRL: c_uint = 0x00;

pub const CECB_CTRL_TYPE_RETRY: c_int = 0;
pub const CECB_CTRL_TYPE_NEW: c_int = 1;
pub const CECB_CTRL_TYPE_NEXT: c_int = 2;
pub const CECB_CTRL2: c_uint = 0x01;

pub const CECB_INTR_MASK: c_uint = 0x02;
pub const CECB_LADD_LOW: c_uint = 0x05;
pub const CECB_LADD_HIGH: c_uint = 0x06;
pub const CECB_TX_CNT: c_uint = 0x07;
pub const CECB_RX_CNT: c_uint = 0x08;
pub const CECB_STAT0: c_uint = 0x09;
pub const CECB_TX_DATA00: c_uint = 0x10;
pub const CECB_TX_DATA01: c_uint = 0x11;
pub const CECB_TX_DATA02: c_uint = 0x12;
pub const CECB_TX_DATA03: c_uint = 0x13;
pub const CECB_TX_DATA04: c_uint = 0x14;
pub const CECB_TX_DATA05: c_uint = 0x15;
pub const CECB_TX_DATA06: c_uint = 0x16;
pub const CECB_TX_DATA07: c_uint = 0x17;
pub const CECB_TX_DATA08: c_uint = 0x18;
pub const CECB_TX_DATA09: c_uint = 0x19;
pub const CECB_TX_DATA10: c_uint = 0x1A;
pub const CECB_TX_DATA11: c_uint = 0x1B;
pub const CECB_TX_DATA12: c_uint = 0x1C;
pub const CECB_TX_DATA13: c_uint = 0x1D;
pub const CECB_TX_DATA14: c_uint = 0x1E;
pub const CECB_TX_DATA15: c_uint = 0x1F;
pub const CECB_RX_DATA00: c_uint = 0x20;
pub const CECB_RX_DATA01: c_uint = 0x21;
pub const CECB_RX_DATA02: c_uint = 0x22;
pub const CECB_RX_DATA03: c_uint = 0x23;
pub const CECB_RX_DATA04: c_uint = 0x24;
pub const CECB_RX_DATA05: c_uint = 0x25;
pub const CECB_RX_DATA06: c_uint = 0x26;
pub const CECB_RX_DATA07: c_uint = 0x27;
pub const CECB_RX_DATA08: c_uint = 0x28;
pub const CECB_RX_DATA09: c_uint = 0x29;
pub const CECB_RX_DATA10: c_uint = 0x2A;
pub const CECB_RX_DATA11: c_uint = 0x2B;
pub const CECB_RX_DATA12: c_uint = 0x2C;
pub const CECB_RX_DATA13: c_uint = 0x2D;
pub const CECB_RX_DATA14: c_uint = 0x2E;
pub const CECB_RX_DATA15: c_uint = 0x2F;
pub const CECB_LOCK_BUF: c_uint = 0x30;

pub const CECB_WAKEUPCTRL: c_uint = 0x31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_ao_cec_g12a_data {
// Setup the internal CECB_CTRL2 register
    pub ctrl2_setup: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_ao_cec_g12a_device {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub regmap_cec: *mut regmap,
    pub cec_reg_lock: spinlock_t,
    pub notify: *mut cec_notifier,
    pub adap: *mut cec_adapter,
    pub rx_msg: cec_msg,
    pub oscin: *mut clk,
    pub core: *mut clk,
    pub data: *const meson_ao_cec_g12a_data,
}

    static const struct regmap_config meson_ao_cec_g12a_regmap_conf = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = CECB_INTR_STAT_REG,
    };
//
// The AO-CECB embeds a dual/divider to generate a more precise
// 32,768KHz clock for CEC core clock.
// ______   ______
// |      | |      |
// ______      | Div1 |-| Cnt1 |       ______
// |      |    /|______| |______|\     |      |
// Xtal-->| Gate |---|  ______   ______  X-X--| Gate |-->
// |______| |  \|      | |      |/  |  |______|
// |   | Div2 |-| Cnt2 |   |
// |   |______| |______|   |
// |_______________________|
//
// The dividing can be switched to single or dual, with a counter
// for each divider to set when the switching is done.
// The entire dividing mechanism can be also bypassed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_ao_cec_g12a_dualdiv_clk {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

    container_of(_hw, struct meson_ao_cec_g12a_dualdiv_clk, hw)	\
    static unsigned long
    meson_ao_cec_g12a_dualdiv_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct meson_ao_cec_g12a_dualdiv_clk *dualdiv_clk =
    hw_to_meson_ao_cec_g12a_dualdiv_clk(hw);
    unsigned long n1;
    u32 reg0, reg1;
    regmap_read(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0, &reg0);
    regmap_read(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0, &reg1);
    if (reg1 & CECB_CLK_CNTL_BYPASS_EN)
    return parent_rate;
    if (reg0 & CECB_CLK_CNTL_DUAL_EN) {
    unsigned long n2, m1, m2, f1, f2, p1, p2;
    n1 = FIELD_GET(CECB_CLK_CNTL_N1, reg0) + 1;
    n2 = FIELD_GET(CECB_CLK_CNTL_N2, reg0) + 1;
    m1 = FIELD_GET(CECB_CLK_CNTL_M1, reg1) + 1;
    m2 = FIELD_GET(CECB_CLK_CNTL_M1, reg1) + 1;
    f1 = DIV_ROUND_CLOSEST(parent_rate, n1);
    f2 = DIV_ROUND_CLOSEST(parent_rate, n2);
    p1 = DIV_ROUND_CLOSEST(100000000 * m1, f1 * (m1 + m2));
    p2 = DIV_ROUND_CLOSEST(100000000 * m2, f2 * (m1 + m2));
    return DIV_ROUND_UP(100000000, p1 + p2);
    }
    n1 = FIELD_GET(CECB_CLK_CNTL_N1, reg0) + 1;
    return DIV_ROUND_CLOSEST(parent_rate, n1);
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_dualdiv_clk_enable(hw: *mut clk_hw) -> c_int {
    static int meson_ao_cec_g12a_dualdiv_clk_enable(struct clk_hw *hw)
    {
    struct meson_ao_cec_g12a_dualdiv_clk *dualdiv_clk =
    hw_to_meson_ao_cec_g12a_dualdiv_clk(hw);
// Disable Input & Output
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0,
    CECB_CLK_CNTL_INPUT_EN | CECB_CLK_CNTL_OUTPUT_EN,
    0);
// Set N1 & N2
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0,
    CECB_CLK_CNTL_N1,
    FIELD_PREP(CECB_CLK_CNTL_N1, 733 - 1));
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0,
    CECB_CLK_CNTL_N2,
    FIELD_PREP(CECB_CLK_CNTL_N2, 732 - 1));
// Set M1 & M2
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG1,
    CECB_CLK_CNTL_M1,
    FIELD_PREP(CECB_CLK_CNTL_M1, 8 - 1));
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG1,
    CECB_CLK_CNTL_M2,
    FIELD_PREP(CECB_CLK_CNTL_M2, 11 - 1));
// Enable Dual divisor
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0,
    CECB_CLK_CNTL_DUAL_EN, CECB_CLK_CNTL_DUAL_EN);
// Disable divisor bypass
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG1,
    CECB_CLK_CNTL_BYPASS_EN, 0);
// Enable Input & Output
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0,
    CECB_CLK_CNTL_INPUT_EN | CECB_CLK_CNTL_OUTPUT_EN,
    CECB_CLK_CNTL_INPUT_EN | CECB_CLK_CNTL_OUTPUT_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_dualdiv_clk_disable(hw: *mut clk_hw) {
    static void meson_ao_cec_g12a_dualdiv_clk_disable(struct clk_hw *hw)
    {
    struct meson_ao_cec_g12a_dualdiv_clk *dualdiv_clk =
    hw_to_meson_ao_cec_g12a_dualdiv_clk(hw);
    regmap_update_bits(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0,
    CECB_CLK_CNTL_INPUT_EN | CECB_CLK_CNTL_OUTPUT_EN,
    0);
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_dualdiv_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int meson_ao_cec_g12a_dualdiv_clk_is_enabled(struct clk_hw *hw)
    {
    struct meson_ao_cec_g12a_dualdiv_clk *dualdiv_clk =
    hw_to_meson_ao_cec_g12a_dualdiv_clk(hw);
    int val;
    regmap_read(dualdiv_clk.regmap, CECB_CLK_CNTL_REG0, &val);
    return !!(val & (CECB_CLK_CNTL_INPUT_EN | CECB_CLK_CNTL_OUTPUT_EN));
    }
    static const struct clk_ops meson_ao_cec_g12a_dualdiv_clk_ops = {
    .recalc_rate	= meson_ao_cec_g12a_dualdiv_clk_recalc_rate,
    .is_enabled	= meson_ao_cec_g12a_dualdiv_clk_is_enabled,
    .enable		= meson_ao_cec_g12a_dualdiv_clk_enable,
    .disable	= meson_ao_cec_g12a_dualdiv_clk_disable,
    };
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_setup_clk(ao_cec: *mut meson_ao_cec_g12a_device) -> c_int {
    static int meson_ao_cec_g12a_setup_clk(struct meson_ao_cec_g12a_device *ao_cec)
    {
    struct meson_ao_cec_g12a_dualdiv_clk *dualdiv_clk;
    struct device *dev = &ao_cec.pdev.dev;
    struct clk_init_data init;
    const char *parent_name;
    struct clk *clk;
    char *name;
    dualdiv_clk = devm_kzalloc(dev, sizeof(*dualdiv_clk), GFP_KERNEL);
    if (!dualdiv_clk)
    return -ENOMEM;
    name = kasprintf(GFP_KERNEL, "%s#dualdiv_clk", dev_name(dev));
    if (!name)
    return -ENOMEM;
    parent_name = __clk_get_name(ao_cec.oscin);
    init.name = name;
    init.ops = &meson_ao_cec_g12a_dualdiv_clk_ops;
    init.flags = 0;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    dualdiv_clk.regmap = ao_cec.regmap;
    dualdiv_clk.hw.init = &init;
    clk = devm_clk_register(dev, &dualdiv_clk.hw);
    kfree(name);
    if (IS_ERR(clk)) {
    dev_err(dev, "failed to register clock\n");
    return PTR_ERR(clk);
    }
    ao_cec.core = clk;
    return 0;
    }
    static int meson_ao_cec_g12a_read(void *context, unsigned int addr,
    unsigned int *data)
    {
    struct meson_ao_cec_g12a_device *ao_cec = context;
    let mut reg: u32 = FIELD_PREP(CECB_RW_ADDR, addr);
    let mut ret: c_int = 0;
    ret = regmap_write(ao_cec.regmap, CECB_RW_REG, reg);
    if (ret)
    return ret;
    ret = regmap_read_poll_timeout(ao_cec.regmap, CECB_RW_REG, reg,
    !(reg & CECB_RW_BUS_BUSY),
    5, 1000);
    if (ret)
    return ret;
    ret = regmap_read(ao_cec.regmap, CECB_RW_REG, &reg);
// data = FIELD_GET(CECB_RW_RD_DATA, reg);
    return ret;
    }
    static int meson_ao_cec_g12a_write(void *context, unsigned int addr,
    unsigned int data)
    {
    struct meson_ao_cec_g12a_device *ao_cec = context;
    u32 reg = FIELD_PREP(CECB_RW_ADDR, addr) |
    FIELD_PREP(CECB_RW_WR_DATA, data) |
    CECB_RW_WRITE_EN;
    return regmap_write(ao_cec.regmap, CECB_RW_REG, reg);
    }
    static const struct regmap_config meson_ao_cec_g12a_cec_regmap_conf = {
    .name = "core",
    .reg_bits = 8,
    .val_bits = 8,
    .reg_read = meson_ao_cec_g12a_read,
    .reg_write = meson_ao_cec_g12a_write,
    .max_register = 0xffff,
    };
    static inline void
    meson_ao_cec_g12a_irq_setup(struct meson_ao_cec_g12a_device *ao_cec,
    bool enable)
    {
    u32 cfg = CECB_INTR_DONE | CECB_INTR_EOM | CECB_INTR_NACK |
    CECB_INTR_ARB_LOSS | CECB_INTR_INITIATOR_ERR |
    CECB_INTR_FOLLOWER_ERR;
    regmap_write(ao_cec.regmap, CECB_INTR_MASKN_REG,
    enable ? cfg : 0);
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_irq_rx(ao_cec: *mut meson_ao_cec_g12a_device) {
    static void meson_ao_cec_g12a_irq_rx(struct meson_ao_cec_g12a_device *ao_cec)
    {
    int i, ret = 0;
    u32 val;
    ret = regmap_read(ao_cec.regmap_cec, CECB_RX_CNT, &val);
    ao_cec.rx_msg.len = val;
    if (ao_cec.rx_msg.len > CEC_MAX_MSG_SIZE)
    ao_cec.rx_msg.len = CEC_MAX_MSG_SIZE;
    for (i = 0; i < ao_cec.rx_msg.len; i++) {
    ret |= regmap_read(ao_cec.regmap_cec,
    CECB_RX_DATA00 + i, &val);
    ao_cec.rx_msg.msg[i] = val & 0xff;
    }
    ret |= regmap_write(ao_cec.regmap_cec, CECB_LOCK_BUF, 0);
    if (ret)
    return;
    cec_received_msg(ao_cec.adap, &ao_cec.rx_msg);
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_ao_cec_g12a_irq(int irq, void *data)
    {
    struct meson_ao_cec_g12a_device *ao_cec = data;
    u32 stat;
    regmap_read(ao_cec.regmap, CECB_INTR_STAT_REG, &stat);
    if (stat)
    return IRQ_WAKE_THREAD;
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_ao_cec_g12a_irq_thread(int irq, void *data)
    {
    struct meson_ao_cec_g12a_device *ao_cec = data;
    u32 stat;
    regmap_read(ao_cec.regmap, CECB_INTR_STAT_REG, &stat);
    regmap_write(ao_cec.regmap, CECB_INTR_CLR_REG, stat);
    if (stat & CECB_INTR_DONE)
    cec_transmit_attempt_done(ao_cec.adap, CEC_TX_STATUS_OK);
    if (stat & CECB_INTR_EOM)
    meson_ao_cec_g12a_irq_rx(ao_cec);
    if (stat & CECB_INTR_NACK)
    cec_transmit_attempt_done(ao_cec.adap, CEC_TX_STATUS_NACK);
    if (stat & CECB_INTR_ARB_LOSS) {
    regmap_write(ao_cec.regmap_cec, CECB_TX_CNT, 0);
    regmap_update_bits(ao_cec.regmap_cec, CECB_CTRL,
    CECB_CTRL_SEND | CECB_CTRL_TYPE, 0);
    cec_transmit_attempt_done(ao_cec.adap, CEC_TX_STATUS_ARB_LOST);
    }
// Initiator reports an error on the CEC bus
    if (stat & CECB_INTR_INITIATOR_ERR)
    cec_transmit_attempt_done(ao_cec.adap, CEC_TX_STATUS_ERROR);
// Follower reports a receive error, just reset RX buffer
    if (stat & CECB_INTR_FOLLOWER_ERR)
    regmap_write(ao_cec.regmap_cec, CECB_LOCK_BUF, 0);
    return IRQ_HANDLED;
    }
    static int
    meson_ao_cec_g12a_set_log_addr(struct cec_adapter *adap, u8 logical_addr)
    {
    struct meson_ao_cec_g12a_device *ao_cec = adap.priv;
    let mut ret: c_int = 0;
    if (logical_addr == CEC_LOG_ADDR_INVALID) {
// Assume this will allways succeed
    regmap_write(ao_cec.regmap_cec, CECB_LADD_LOW, 0);
    regmap_write(ao_cec.regmap_cec, CECB_LADD_HIGH, 0);
    return 0;
    } else if (logical_addr < 8) {
    ret = regmap_update_bits(ao_cec.regmap_cec, CECB_LADD_LOW,
    BIT(logical_addr),
    BIT(logical_addr));
    } else {
    ret = regmap_update_bits(ao_cec.regmap_cec, CECB_LADD_HIGH,
    BIT(logical_addr - 8),
    BIT(logical_addr - 8));
    }
// Always set Broadcast/Unregistered 15 address
    ret |= regmap_update_bits(ao_cec.regmap_cec, CECB_LADD_HIGH,
    BIT(CEC_LOG_ADDR_UNREGISTERED - 8),
    BIT(CEC_LOG_ADDR_UNREGISTERED - 8));
    return ret ? -EIO : 0;
    }
    static int meson_ao_cec_g12a_transmit(struct cec_adapter *adap, u8 attempts,
    u32 signal_free_time, struct cec_msg *msg)
    {
    struct meson_ao_cec_g12a_device *ao_cec = adap.priv;
    unsigned int type;
    let mut ret: c_int = 0;
    u32 val;
    int i;
// Check if RX is in progress
    ret = regmap_read(ao_cec.regmap_cec, CECB_LOCK_BUF, &val);
    if (ret)
    return ret;
    if (val & CECB_LOCK_BUF_EN)
    return -EBUSY;
// Check if TX Busy
    ret = regmap_read(ao_cec.regmap_cec, CECB_CTRL, &val);
    if (ret)
    return ret;
    if (val & CECB_CTRL_SEND)
    return -EBUSY;
    switch (signal_free_time) {
    case CEC_SIGNAL_FREE_TIME_RETRY:
    type = CECB_CTRL_TYPE_RETRY;
    break;
    case CEC_SIGNAL_FREE_TIME_NEXT_XFER:
    type = CECB_CTRL_TYPE_NEXT;
    break;
    case CEC_SIGNAL_FREE_TIME_NEW_INITIATOR:
    default:
    type = CECB_CTRL_TYPE_NEW;
    break;
    }
    for (i = 0; i < msg.len; i++)
    ret |= regmap_write(ao_cec.regmap_cec, CECB_TX_DATA00 + i,
    msg.msg[i]);
    ret |= regmap_write(ao_cec.regmap_cec, CECB_TX_CNT, msg.len);
    if (ret)
    return -EIO;
    ret = regmap_update_bits(ao_cec.regmap_cec, CECB_CTRL,
    CECB_CTRL_SEND |
    CECB_CTRL_TYPE,
    CECB_CTRL_SEND |
    FIELD_PREP(CECB_CTRL_TYPE, type));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_adap_enable(adap: *mut cec_adapter, enable: bool) -> c_int {
    static int meson_ao_cec_g12a_adap_enable(struct cec_adapter *adap, bool enable)
    {
    struct meson_ao_cec_g12a_device *ao_cec = adap.priv;
    meson_ao_cec_g12a_irq_setup(ao_cec, false);
    regmap_update_bits(ao_cec.regmap, CECB_GEN_CNTL_REG,
    CECB_GEN_CNTL_RESET, CECB_GEN_CNTL_RESET);
    if (!enable)
    return 0;
// Setup Filter
    regmap_update_bits(ao_cec.regmap, CECB_GEN_CNTL_REG,
    CECB_GEN_CNTL_FILTER_TICK_SEL |
    CECB_GEN_CNTL_FILTER_DEL,
    FIELD_PREP(CECB_GEN_CNTL_FILTER_TICK_SEL,
    CECB_GEN_CNTL_FILTER_TICK_1US) |
    FIELD_PREP(CECB_GEN_CNTL_FILTER_DEL, 7));
// Enable System Clock
    regmap_update_bits(ao_cec.regmap, CECB_GEN_CNTL_REG,
    CECB_GEN_CNTL_SYS_CLK_EN,
    CECB_GEN_CNTL_SYS_CLK_EN);
// Enable gated clock (Normal mode).
    regmap_update_bits(ao_cec.regmap, CECB_GEN_CNTL_REG,
    CECB_GEN_CNTL_CLK_CTRL_MASK,
    FIELD_PREP(CECB_GEN_CNTL_CLK_CTRL_MASK,
    CECB_GEN_CNTL_CLK_ENABLE));
// Release Reset
    regmap_update_bits(ao_cec.regmap, CECB_GEN_CNTL_REG,
    CECB_GEN_CNTL_RESET, 0);
    if (ao_cec.data.ctrl2_setup)
    regmap_write(ao_cec.regmap_cec, CECB_CTRL2,
    FIELD_PREP(CECB_CTRL2_RISE_DEL_MAX, 2));
    meson_ao_cec_g12a_irq_setup(ao_cec, true);
    return 0;
    }
    static const struct cec_adap_ops meson_ao_cec_g12a_ops = {
    .adap_enable = meson_ao_cec_g12a_adap_enable,
    .adap_log_addr = meson_ao_cec_g12a_set_log_addr,
    .adap_transmit = meson_ao_cec_g12a_transmit,
    };
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_probe(pdev: *mut platform_device) -> c_int {
    static int meson_ao_cec_g12a_probe(struct platform_device *pdev)
    {
    struct meson_ao_cec_g12a_device *ao_cec;
    struct device *hdmi_dev;
    void __iomem *base;
    int ret, irq;
    hdmi_dev = cec_notifier_parse_hdmi_phandle(&pdev.dev);
    if (IS_ERR(hdmi_dev))
    return PTR_ERR(hdmi_dev);
    ao_cec = devm_kzalloc(&pdev.dev, sizeof(*ao_cec), GFP_KERNEL);
    if (!ao_cec)
    return -ENOMEM;
    ao_cec.data = of_device_get_match_data(&pdev.dev);
    if (!ao_cec.data) {
    dev_err(&pdev.dev, "failed to get match data\n");
    return -ENODEV;
    }
    spin_lock_init(&ao_cec.cec_reg_lock);
    ao_cec.pdev = pdev;
    ao_cec.adap = cec_allocate_adapter(&meson_ao_cec_g12a_ops, ao_cec,
    "meson_g12a_ao_cec",
    CEC_CAP_DEFAULTS |
    CEC_CAP_CONNECTOR_INFO,
    CEC_MAX_LOG_ADDRS);
    if (IS_ERR(ao_cec.adap))
    return PTR_ERR(ao_cec.adap);
    ao_cec.adap.owner = THIS_MODULE;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base)) {
    ret = PTR_ERR(base);
    goto out_probe_adapter;
    }
    ao_cec.regmap = devm_regmap_init_mmio(&pdev.dev, base,
    &meson_ao_cec_g12a_regmap_conf);
    if (IS_ERR(ao_cec.regmap)) {
    ret = PTR_ERR(ao_cec.regmap);
    goto out_probe_adapter;
    }
    ao_cec.regmap_cec = devm_regmap_init(&pdev.dev, core::ptr::null_mut(), ao_cec,
    &meson_ao_cec_g12a_cec_regmap_conf);
    if (IS_ERR(ao_cec.regmap_cec)) {
    ret = PTR_ERR(ao_cec.regmap_cec);
    goto out_probe_adapter;
    }
    irq = platform_get_irq(pdev, 0);
    ret = devm_request_threaded_irq(&pdev.dev, irq,
    meson_ao_cec_g12a_irq,
    meson_ao_cec_g12a_irq_thread,
    0, core::ptr::null_mut(), ao_cec);
    if (ret)
    goto out_probe_adapter;
    ao_cec.oscin = devm_clk_get(&pdev.dev, "oscin");
    if (IS_ERR(ao_cec.oscin)) {
    dev_err(&pdev.dev, "oscin clock request failed\n");
    ret = PTR_ERR(ao_cec.oscin);
    goto out_probe_adapter;
    }
    ret = meson_ao_cec_g12a_setup_clk(ao_cec);
    if (ret)
    goto out_probe_adapter;
    ret = clk_prepare_enable(ao_cec.core);
    if (ret) {
    dev_err(&pdev.dev, "core clock enable failed\n");
    goto out_probe_adapter;
    }
    device_reset_optional(&pdev.dev);
    platform_set_drvdata(pdev, ao_cec);
    ao_cec.notify = cec_notifier_cec_adap_register(hdmi_dev, core::ptr::null_mut(),
    ao_cec.adap);
    if (!ao_cec.notify) {
    ret = -ENOMEM;
    goto out_probe_core_clk;
    }
    ret = cec_register_adapter(ao_cec.adap, &pdev.dev);
    if (ret < 0)
    goto out_probe_notify;
// Setup Hardware
    regmap_write(ao_cec.regmap, CECB_GEN_CNTL_REG, CECB_GEN_CNTL_RESET);
    return 0;
    out_probe_notify:
    cec_notifier_cec_adap_unregister(ao_cec.notify, ao_cec.adap);
    out_probe_core_clk:
    clk_disable_unprepare(ao_cec.core);
    out_probe_adapter:
    cec_delete_adapter(ao_cec.adap);
    dev_err(&pdev.dev, "CEC controller registration failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_ao_cec_g12a_remove(pdev: *mut platform_device) {
    static void meson_ao_cec_g12a_remove(struct platform_device *pdev)
    {
    struct meson_ao_cec_g12a_device *ao_cec = platform_get_drvdata(pdev);
    clk_disable_unprepare(ao_cec.core);
    cec_notifier_cec_adap_unregister(ao_cec.notify, ao_cec.adap);
    cec_unregister_adapter(ao_cec.adap);
    }
    static const struct meson_ao_cec_g12a_data ao_cec_g12a_data = {
    .ctrl2_setup = false,
    };
    static const struct meson_ao_cec_g12a_data ao_cec_sm1_data = {
    .ctrl2_setup = true,
    };
    static const struct of_device_id meson_ao_cec_g12a_of_match[] = {
    {
    .compatible = "amlogic,meson-g12a-ao-cec",
    .data = &ao_cec_g12a_data,
    },
    {
    .compatible = "amlogic,meson-sm1-ao-cec",
    .data = &ao_cec_sm1_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, meson_ao_cec_g12a_of_match);
    static struct platform_driver meson_ao_cec_g12a_driver = {
    .probe   = meson_ao_cec_g12a_probe,
    .remove = meson_ao_cec_g12a_remove,
    .driver  = {
    .name = "meson-ao-cec-g12a",
    .of_match_table = of_match_ptr(meson_ao_cec_g12a_of_match),
    },
    };
    module_platform_driver(meson_ao_cec_g12a_driver);
    MODULE_DESCRIPTION("Meson AO CEC G12A Controller driver");
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_LICENSE("GPL");
