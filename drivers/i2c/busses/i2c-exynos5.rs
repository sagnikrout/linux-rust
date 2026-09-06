//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-exynos5.c
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
// i2c-exynos5.c - Samsung Exynos5 I2C Controller Driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//

//
// HSI2C controller from Samsung supports 2 modes of operation
// 1. Auto mode: Where in master automatically controls the whole transaction
// 2. Manual mode: Software controls the transaction by issuing commands
// START, READ, WRITE, STOP, RESTART in I2C_MANUAL_CMD register.
//
// Operation mode can be selected by setting AUTO_MODE bit in I2C_CONF register
//
// Special bits are available for both modes of operation to set commands
// and for checking transfer status
//
// Register Map
pub const HSI2C_CTL: c_uint = 0x00;
pub const HSI2C_FIFO_CTL: c_uint = 0x04;
pub const HSI2C_TRAILIG_CTL: c_uint = 0x08;
pub const HSI2C_CLK_CTL: c_uint = 0x0C;
pub const HSI2C_CLK_SLOT: c_uint = 0x10;
pub const HSI2C_INT_ENABLE: c_uint = 0x20;
pub const HSI2C_INT_STATUS: c_uint = 0x24;
pub const HSI2C_ERR_STATUS: c_uint = 0x2C;
pub const HSI2C_FIFO_STATUS: c_uint = 0x30;
pub const HSI2C_TX_DATA: c_uint = 0x34;
pub const HSI2C_RX_DATA: c_uint = 0x38;
pub const HSI2C_CONF: c_uint = 0x40;
pub const HSI2C_AUTO_CONF: c_uint = 0x44;
pub const HSI2C_TIMEOUT: c_uint = 0x48;
pub const HSI2C_MANUAL_CMD: c_uint = 0x4C;
pub const HSI2C_TRANS_STATUS: c_uint = 0x50;
pub const HSI2C_TIMING_HS1: c_uint = 0x54;
pub const HSI2C_TIMING_HS2: c_uint = 0x58;
pub const HSI2C_TIMING_HS3: c_uint = 0x5C;
pub const HSI2C_TIMING_FS1: c_uint = 0x60;
pub const HSI2C_TIMING_FS2: c_uint = 0x64;
pub const HSI2C_TIMING_FS3: c_uint = 0x68;
pub const HSI2C_TIMING_SLA: c_uint = 0x6C;
pub const HSI2C_ADDR: c_uint = 0x70;
// I2C_CTL Register bits

// I2C_FIFO_CTL Register bits

// I2C_TRAILING_CTL Register bits

// I2C_INT_EN Register bits

// I2C_INT_STAT Register bits

    HSI2C_INT_TRANS_ABORT |	\
    HSI2C_INT_NO_DEV_ACK |	\
    HSI2C_INT_NO_DEV |	\
    HSI2C_INT_TIMEOUT)
// I2C_FIFO_STAT Register bits

// I2C_CONF Register bits

// I2C_AUTO_CONF Register bits

// I2C_TIMEOUT Register bits

pub const HSI2C_TIMEOUT_MASK: c_uint = 0xff;
// I2C_MANUAL_CMD register bits

// I2C_TRANS_STATUS register bits

// I2C_TRANS_STATUS register bits for Exynos5 variant

// I2C_TRANS_STATUS register bits for Exynos7 variant
pub const HSI2C_MASTER_ST_MASK: c_uint = 0xf;
pub const HSI2C_MASTER_ST_IDLE: c_uint = 0x0;
pub const HSI2C_MASTER_ST_START: c_uint = 0x1;
pub const HSI2C_MASTER_ST_RESTART: c_uint = 0x2;
pub const HSI2C_MASTER_ST_STOP: c_uint = 0x3;
pub const HSI2C_MASTER_ST_MASTER_ID: c_uint = 0x4;
pub const HSI2C_MASTER_ST_ADDR0: c_uint = 0x5;
pub const HSI2C_MASTER_ST_ADDR1: c_uint = 0x6;
pub const HSI2C_MASTER_ST_ADDR2: c_uint = 0x7;
pub const HSI2C_MASTER_ST_ADDR_SR: c_uint = 0x8;
pub const HSI2C_MASTER_ST_READ: c_uint = 0x9;
pub const HSI2C_MASTER_ST_WRITE: c_uint = 0xa;
pub const HSI2C_MASTER_ST_NO_ACK: c_uint = 0xb;
pub const HSI2C_MASTER_ST_LOSE: c_uint = 0xc;
pub const HSI2C_MASTER_ST_WAIT: c_uint = 0xd;
pub const HSI2C_MASTER_ST_WAIT_CMD: c_uint = 0xe;
// I2C_ADDR register bits

    enum i2c_type_exynos {
    I2C_TYPE_EXYNOS5,
    I2C_TYPE_EXYNOS7,
    I2C_TYPE_EXYNOSAUTOV9,
    I2C_TYPE_EXYNOS8895,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos5_i2c {
    pub adap: i2c_adapter,
    pub msg: *mut i2c_msg,
    pub msg_complete: completion,
    pub msg_ptr: c_uint,
    pub irq: c_uint,
    pub regs: *mut void __iomem,
    pub /: *mut *mut *mut clk clk; / operating clock,
    pub /: *mut *mut *mut clk pclk; / bus clock,
    pub dev: *mut device,
    pub state: c_int,
    pub /: *mut *mut spinlock_t lock; / IRQ synchronization,
//
// Since the TRANS_DONE bit is cleared on read, and we may read it
// either during an IRQ or after a transaction, keep track of its
// state here.
//
    pub trans_done: c_int,
//
// Called from atomic context, don't use interrupts.
//
    pub atomic: c_uint,
// Controller operating frequency
    pub op_clock: c_uint,
// Version of HS-I2C Hardware
    pub variant: *const exynos_hsi2c_variant,
}

//
// struct exynos_hsi2c_variant - platform specific HSI2C driver data
// @fifo_depth: the fifo depth supported by the HSI2C module
// @hw: the hardware variant of Exynos I2C controller
//
// Specifies platform specific configuration of HSI2C module.
// Note: A structure for driver specific platform data is used for future
// expansion of its usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_hsi2c_variant {
    pub fifo_depth: c_uint,
    pub hw: enum i2c_type_exynos,
}

    static const struct exynos_hsi2c_variant exynos5250_hsi2c_data = {
    .fifo_depth	= 64,
    .hw		= I2C_TYPE_EXYNOS5,
    };
    static const struct exynos_hsi2c_variant exynos5260_hsi2c_data = {
    .fifo_depth	= 16,
    .hw		= I2C_TYPE_EXYNOS5,
    };
    static const struct exynos_hsi2c_variant exynos7_hsi2c_data = {
    .fifo_depth	= 16,
    .hw		= I2C_TYPE_EXYNOS7,
    };
    static const struct exynos_hsi2c_variant exynosautov9_hsi2c_data = {
    .fifo_depth	= 64,
    .hw		= I2C_TYPE_EXYNOSAUTOV9,
    };
    static const struct exynos_hsi2c_variant exynos8895_hsi2c_data = {
    .fifo_depth	= 64,
    .hw		= I2C_TYPE_EXYNOS8895,
    };
    static const struct of_device_id exynos5_i2c_match[] = {
    {
    .compatible = "samsung,exynos5-hsi2c",
    .data = &exynos5250_hsi2c_data
    }, {
    .compatible = "samsung,exynos5250-hsi2c",
    .data = &exynos5250_hsi2c_data
    }, {
    .compatible = "samsung,exynos5260-hsi2c",
    .data = &exynos5260_hsi2c_data
    }, {
    .compatible = "samsung,exynos7-hsi2c",
    .data = &exynos7_hsi2c_data
    }, {
    .compatible = "samsung,exynosautov9-hsi2c",
    .data = &exynosautov9_hsi2c_data
    }, {
    .compatible = "samsung,exynos8895-hsi2c",
    .data = &exynos8895_hsi2c_data
    }, {},
    };
    MODULE_DEVICE_TABLE(of, exynos5_i2c_match);
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_clr_pend_irq(i2c: *mut exynos5_i2c) {
    static void exynos5_i2c_clr_pend_irq(struct exynos5_i2c *i2c)
    {
    writel(readl(i2c.regs + HSI2C_INT_STATUS),
    i2c.regs + HSI2C_INT_STATUS);
    }
//
// exynos5_i2c_set_timing: updates the registers with appropriate
// timing values calculated
//
// Timing values for operation are calculated against 100kHz, 400kHz
// or 1MHz controller operating frequency.
//
// Returns 0 on success, -EINVAL if the cycle length cannot
// be calculated.
//
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_set_timing(i2c: *mut exynos5_i2c, hs_timings: bool) -> c_int {
    static int exynos5_i2c_set_timing(struct exynos5_i2c *i2c, bool hs_timings)
    {
    u32 i2c_timing_s1;
    u32 i2c_timing_s2;
    u32 i2c_timing_s3;
    u32 i2c_timing_sla;
    unsigned int t_start_su, t_start_hd;
    unsigned int t_stop_su;
    unsigned int t_data_su, t_data_hd;
    unsigned int t_scl_l, t_scl_h;
    unsigned int t_sr_release;
    unsigned int t_ftl_cycle;
    let mut clkin: c_uint = clk_get_rate(i2c.clk);
    unsigned int op_clk = hs_timings ? i2c.op_clock :
    (i2c.op_clock >= I2C_MAX_FAST_MODE_PLUS_FREQ) ? I2C_MAX_STANDARD_MODE_FREQ :
    i2c.op_clock;
    int div, clk_cycle, temp;
//
// In case of HSI2C controllers in ExynosAutoV9:
//
// FSCL = IPCLK / ((CLK_DIV + 1) * 16)
// T_SCL_LOW = IPCLK * (CLK_DIV + 1) * (N + M)
// [N : number of 0's in the TSCL_H_HS]
// [M : number of 0's in the TSCL_L_HS]
// T_SCL_HIGH = IPCLK * (CLK_DIV + 1) * (N + M)
// [N : number of 1's in the TSCL_H_HS]
// [M : number of 1's in the TSCL_L_HS]
//
// Result of (N + M) is always 8.
// In general case, we don't need to control timing_s1 and timing_s2.
//
    if (i2c.variant.hw == I2C_TYPE_EXYNOSAUTOV9) {
    div = ((clkin / (16 * i2c.op_clock)) - 1);
    i2c_timing_s3 = div << 16;
    if (hs_timings)
    writel(i2c_timing_s3, i2c.regs + HSI2C_TIMING_HS3);
    else
    writel(i2c_timing_s3, i2c.regs + HSI2C_TIMING_FS3);
    return 0;
    }
//
// In case of HSI2C controller in Exynos5 series
// FPCLK / FI2C =
// (CLK_DIV + 1) * (TSCLK_L + TSCLK_H + 2) + 8 + 2 * FLT_CYCLE
//
// In case of HSI2C controllers in Exynos7 series
// FPCLK / FI2C =
// (CLK_DIV + 1) * (TSCLK_L + TSCLK_H + 2) + 8 + FLT_CYCLE
//
// clk_cycle := TSCLK_L + TSCLK_H
// temp := (CLK_DIV + 1) * (clk_cycle + 2)
//
// In case of HSI2C controllers in Exynos8895
// FPCLK / FI2C =
// (CLK_DIV + 1) * (TSCLK_L + TSCLK_H + 2) +
// 2 * ((FLT_CYCLE + 3) - (FLT_CYCLE + 3) % (CLK_DIV + 1))
//
// clk_cycle := TSCLK_L + TSCLK_H
// temp := (FPCLK / FI2C) - (FLT_CYCLE + 3) * 2
//
// Constraints: 4 <= temp, 0 <= CLK_DIV < 256, 2 <= clk_cycle <= 510
//
// To split SCL clock into low, high periods appropriately, one
// proportion factor for each I2C mode is used, which is calculated
// using this formula.
// ```
// ((t_low_min + (scl_clock - t_low_min - t_high_min) / 2) / scl_clock)
// ```
// where:
// t_low_min is the minimal value of low period of the SCL clock in us;
// t_high_min is the minimal value of high period of the SCL clock in us;
// scl_clock is converted from SCL clock frequency into us.
//
// Below are the proportion factors for these I2C modes:
// t_low_min, t_high_min, scl_clock, proportion
// Standard Mode:     4.7us,      4.0us,      10us,      0.535
// Fast Mode:         1.3us,      0.6us,     2.5us,       0.64
// Fast-Plus Mode:    0.5us,     0.26us,       1us,       0.62
//
    t_ftl_cycle = (readl(i2c.regs + HSI2C_CONF) >> 16) & 0x7;
    if (i2c.variant.hw == I2C_TYPE_EXYNOS8895)
    temp = clkin / op_clk - (t_ftl_cycle + 3) * 2;
#[no_mangle]
pub unsafe extern "C" fn if(I2C_TYPE_EXYNOS7: i2c->variant->hw ==) -> else {
    else if (i2c.variant.hw == I2C_TYPE_EXYNOS7)
    temp = clkin / op_clk - 8 - t_ftl_cycle;
    else
    temp = clkin / op_clk - 8 - (t_ftl_cycle * 2);
    div = temp / 512;
    if (i2c.variant.hw == I2C_TYPE_EXYNOS8895)
    clk_cycle = (temp + ((t_ftl_cycle + 3) % (div + 1)) * 2) /
    (div + 1) - 2;
    else
    clk_cycle = temp / (div + 1) - 2;
    if (temp < 4 || div >= 256 || clk_cycle < 2) {
    dev_err(i2c.dev, "%s clock set-up failed\n",
    hs_timings ? "HS" : "FS");
    return -EINVAL;
    }
//
// Scale clk_cycle to get t_scl_l using the proption factors for individual I2C modes.
//
    if (op_clk <= I2C_MAX_STANDARD_MODE_FREQ)
    t_scl_l = clk_cycle * 535 / 1000;
#[no_mangle]
pub unsafe extern "C" fn if(I2C_MAX_FAST_MODE_FREQ: op_clk <=) -> else {
    else if (op_clk <= I2C_MAX_FAST_MODE_FREQ)
    t_scl_l = clk_cycle * 64 / 100;
    else
    t_scl_l = clk_cycle * 62 / 100;
    if (t_scl_l > 0xFF)
    t_scl_l = 0xFF;
    t_scl_h = clk_cycle - t_scl_l;
    t_start_su = t_scl_l;
    t_start_hd = t_scl_l;
    t_stop_su = t_scl_l;
    t_data_su = t_scl_l / 2;
    t_data_hd = t_scl_l / 2;
    t_sr_release = clk_cycle;
    i2c_timing_s1 = t_start_su << 24 | t_start_hd << 16 | t_stop_su << 8;
    i2c_timing_s2 = t_data_su << 24 | t_scl_l << 8 | t_scl_h << 0;
    i2c_timing_s3 = div << 16 | t_sr_release << 0;
    i2c_timing_sla = t_data_hd << 0;
    dev_dbg(i2c.dev, "tSTART_SU: %X, tSTART_HD: %X, tSTOP_SU: %X\n",
    t_start_su, t_start_hd, t_stop_su);
    dev_dbg(i2c.dev, "tDATA_SU: %X, tSCL_L: %X, tSCL_H: %X\n",
    t_data_su, t_scl_l, t_scl_h);
    dev_dbg(i2c.dev, "nClkDiv: %X, tSR_RELEASE: %X\n",
    div, t_sr_release);
    dev_dbg(i2c.dev, "tDATA_HD: %X\n", t_data_hd);
    if (hs_timings) {
    writel(i2c_timing_s1, i2c.regs + HSI2C_TIMING_HS1);
    writel(i2c_timing_s2, i2c.regs + HSI2C_TIMING_HS2);
    writel(i2c_timing_s3, i2c.regs + HSI2C_TIMING_HS3);
    } else {
    writel(i2c_timing_s1, i2c.regs + HSI2C_TIMING_FS1);
    writel(i2c_timing_s2, i2c.regs + HSI2C_TIMING_FS2);
    writel(i2c_timing_s3, i2c.regs + HSI2C_TIMING_FS3);
    }
    writel(i2c_timing_sla, i2c.regs + HSI2C_TIMING_SLA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_hsi2c_clock_setup(i2c: *mut exynos5_i2c) -> c_int {
    static int exynos5_hsi2c_clock_setup(struct exynos5_i2c *i2c)
    {
// always set Fast Speed timings
    let mut ret: c_int = exynos5_i2c_set_timing(i2c, false);
    if (ret < 0 || i2c.op_clock < I2C_MAX_FAST_MODE_PLUS_FREQ)
    return ret;
    return exynos5_i2c_set_timing(i2c, true);
    }
//
// exynos5_i2c_init: configures the controller for I2C functionality
// Programs I2C controller for Master mode operation
//
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_init(i2c: *mut exynos5_i2c) {
    static void exynos5_i2c_init(struct exynos5_i2c *i2c)
    {
    let mut i2c_conf: u32 = readl(i2c.regs + HSI2C_CONF);
    let mut i2c_timeout: u32 = readl(i2c.regs + HSI2C_TIMEOUT);
// Clear to disable Timeout
    i2c_timeout &= ~HSI2C_TIMEOUT_EN;
    writel(i2c_timeout, i2c.regs + HSI2C_TIMEOUT);
    writel((HSI2C_FUNC_MODE_I2C | HSI2C_MASTER),
    i2c.regs + HSI2C_CTL);
    writel(HSI2C_TRAILING_COUNT, i2c.regs + HSI2C_TRAILIG_CTL);
    if (i2c.op_clock >= I2C_MAX_FAST_MODE_PLUS_FREQ) {
    writel(HSI2C_MASTER_ID(MASTER_ID(i2c.adap.nr)),
    i2c.regs + HSI2C_ADDR);
    i2c_conf |= HSI2C_HS_MODE;
    }
    writel(i2c_conf | HSI2C_AUTO_MODE, i2c.regs + HSI2C_CONF);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_reset(i2c: *mut exynos5_i2c) {
    static void exynos5_i2c_reset(struct exynos5_i2c *i2c)
    {
    u32 i2c_ctl;
// Set and clear the bit for reset
    i2c_ctl = readl(i2c.regs + HSI2C_CTL);
    i2c_ctl |= HSI2C_SW_RST;
    writel(i2c_ctl, i2c.regs + HSI2C_CTL);
    i2c_ctl = readl(i2c.regs + HSI2C_CTL);
    i2c_ctl &= ~HSI2C_SW_RST;
    writel(i2c_ctl, i2c.regs + HSI2C_CTL);
// We don't expect calculations to fail during the run
    exynos5_hsi2c_clock_setup(i2c);
// Initialize the configure registers
    exynos5_i2c_init(i2c);
    }
//
// exynos5_i2c_irq: top level IRQ servicing routine
//
// INT_STATUS registers gives the interrupt details. Further,
// FIFO_STATUS or TRANS_STATUS registers are to be check for detailed
// state of the bus.
//
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_irq(irqno: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t exynos5_i2c_irq(int irqno, void *dev_id)
    {
    struct exynos5_i2c *i2c = dev_id;
    u32 fifo_level, int_status, fifo_status, trans_status;
    unsigned char byte;
    let mut len: c_int = 0;
    i2c.state = -EINVAL;
    spin_lock(&i2c.lock);
    int_status = readl(i2c.regs + HSI2C_INT_STATUS);
    writel(int_status, i2c.regs + HSI2C_INT_STATUS);
// handle interrupt related to the transfer status
    switch (i2c.variant.hw) {
    case I2C_TYPE_EXYNOSAUTOV9:
    fallthrough;
    case I2C_TYPE_EXYNOS8895:
    fallthrough;
    case I2C_TYPE_EXYNOS7:
    if (int_status & HSI2C_INT_TRANS_DONE) {
    i2c.trans_done = 1;
    i2c.state = 0;
    } else if (int_status & HSI2C_INT_TRANS_ABORT) {
    dev_dbg(i2c.dev, "Deal with arbitration lose\n");
    i2c.state = -EAGAIN;
    goto stop;
    } else if (int_status & HSI2C_INT_NO_DEV_ACK) {
    dev_dbg(i2c.dev, "No ACK from device\n");
    i2c.state = -ENXIO;
    goto stop;
    } else if (int_status & HSI2C_INT_NO_DEV) {
    dev_dbg(i2c.dev, "No device\n");
    i2c.state = -ENXIO;
    goto stop;
    } else if (int_status & HSI2C_INT_TIMEOUT) {
    dev_dbg(i2c.dev, "Accessing device timed out\n");
    i2c.state = -ETIMEDOUT;
    goto stop;
    }
    break;
    case I2C_TYPE_EXYNOS5:
    if (!(int_status & HSI2C_INT_I2C))
    break;
    trans_status = readl(i2c.regs + HSI2C_TRANS_STATUS);
    if (trans_status & HSI2C_NO_DEV_ACK) {
    dev_dbg(i2c.dev, "No ACK from device\n");
    i2c.state = -ENXIO;
    goto stop;
    } else if (trans_status & HSI2C_NO_DEV) {
    dev_dbg(i2c.dev, "No device\n");
    i2c.state = -ENXIO;
    goto stop;
    } else if (trans_status & HSI2C_TRANS_ABORT) {
    dev_dbg(i2c.dev, "Deal with arbitration lose\n");
    i2c.state = -EAGAIN;
    goto stop;
    } else if (trans_status & HSI2C_TIMEOUT_AUTO) {
    dev_dbg(i2c.dev, "Accessing device timed out\n");
    i2c.state = -ETIMEDOUT;
    goto stop;
    } else if (trans_status & HSI2C_TRANS_DONE) {
    i2c.trans_done = 1;
    i2c.state = 0;
    }
    break;
    }
    if ((i2c.msg.flags & I2C_M_RD) && (int_status &
    (HSI2C_INT_TRAILING | HSI2C_INT_RX_ALMOSTFULL))) {
    fifo_status = readl(i2c.regs + HSI2C_FIFO_STATUS);
    fifo_level = HSI2C_RX_FIFO_LVL(fifo_status);
    len = min(fifo_level, i2c.msg.len - i2c.msg_ptr);
    while (len > 0) {
    byte = (unsigned char)
    readl(i2c.regs + HSI2C_RX_DATA);
    i2c.msg.buf[i2c.msg_ptr++] = byte;
    len--;
    }
    i2c.state = 0;
    } else if (int_status & HSI2C_INT_TX_ALMOSTEMPTY) {
    fifo_status = readl(i2c.regs + HSI2C_FIFO_STATUS);
    fifo_level = HSI2C_TX_FIFO_LVL(fifo_status);
    len = i2c.variant.fifo_depth - fifo_level;
    if (len > (i2c.msg.len - i2c.msg_ptr)) {
    let mut int_en: u32 = readl(i2c.regs + HSI2C_INT_ENABLE);
    int_en &= ~HSI2C_INT_TX_ALMOSTEMPTY_EN;
    writel(int_en, i2c.regs + HSI2C_INT_ENABLE);
    len = i2c.msg.len - i2c.msg_ptr;
    }
    while (len > 0) {
    byte = i2c.msg.buf[i2c.msg_ptr++];
    writel(byte, i2c.regs + HSI2C_TX_DATA);
    len--;
    }
    i2c.state = 0;
    }
    stop:
    if ((i2c.trans_done && (i2c.msg.len == i2c.msg_ptr)) ||
    (i2c.state < 0)) {
    writel(0, i2c.regs + HSI2C_INT_ENABLE);
    exynos5_i2c_clr_pend_irq(i2c);
    complete(&i2c.msg_complete);
    }
    spin_unlock(&i2c.lock);
    return IRQ_HANDLED;
    }
//
// exynos5_i2c_wait_bus_idle
//
// Wait for the bus to go idle, indicated by the MASTER_BUSY bit being
// cleared.
//
// Returns -EBUSY if the bus cannot be bought to idle
//
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_wait_bus_idle(i2c: *mut exynos5_i2c) -> c_int {
    static int exynos5_i2c_wait_bus_idle(struct exynos5_i2c *i2c)
    {
    unsigned long stop_time;
    u32 trans_status;
// wait for 100 milli seconds for the bus to be idle
    stop_time = jiffies + msecs_to_jiffies(100) + 1;
    do {
    trans_status = readl(i2c.regs + HSI2C_TRANS_STATUS);
    if (!(trans_status & HSI2C_MASTER_BUSY))
    return 0;
    usleep_range(50, 200);
    } while (time_before(jiffies, stop_time));
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_bus_recover(i2c: *mut exynos5_i2c) {
    static void exynos5_i2c_bus_recover(struct exynos5_i2c *i2c)
    {
    u32 val;
    val = readl(i2c.regs + HSI2C_CTL) | HSI2C_RXCHON;
    writel(val, i2c.regs + HSI2C_CTL);
    val = readl(i2c.regs + HSI2C_CONF) & ~HSI2C_AUTO_MODE;
    writel(val, i2c.regs + HSI2C_CONF);
//
// Specification says master should send nine clock pulses. It can be
// emulated by sending manual read command (nine pulses for read eight
// bits + one pulse for NACK).
//
    writel(HSI2C_CMD_READ_DATA, i2c.regs + HSI2C_MANUAL_CMD);
    exynos5_i2c_wait_bus_idle(i2c);
    writel(HSI2C_CMD_SEND_STOP, i2c.regs + HSI2C_MANUAL_CMD);
    exynos5_i2c_wait_bus_idle(i2c);
    val = readl(i2c.regs + HSI2C_CTL) & ~HSI2C_RXCHON;
    writel(val, i2c.regs + HSI2C_CTL);
    val = readl(i2c.regs + HSI2C_CONF) | HSI2C_AUTO_MODE;
    writel(val, i2c.regs + HSI2C_CONF);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_bus_check(i2c: *mut exynos5_i2c) {
    static void exynos5_i2c_bus_check(struct exynos5_i2c *i2c)
    {
    unsigned long timeout;
    if (i2c.variant.hw == I2C_TYPE_EXYNOS5)
    return;
//
// HSI2C_MASTER_ST_LOSE state (in Exynos7 and ExynosAutoV9 variants)
// before transaction indicates that bus is stuck (SDA is low).
// In such case bus recovery can be performed.
//
    timeout = jiffies + msecs_to_jiffies(100);
    for (;;) {
    let mut st: u32 = readl(i2c.regs + HSI2C_TRANS_STATUS);
    if ((st & HSI2C_MASTER_ST_MASK) != HSI2C_MASTER_ST_LOSE)
    return;
    if (time_is_before_jiffies(timeout))
    return;
    exynos5_i2c_bus_recover(i2c);
    }
    }
//
// exynos5_i2c_message_start: Configures the bus and starts the xfer
// i2c: struct exynos5_i2c pointer for the current bus
// stop: Enables stop after transfer if set. Set for last transfer of
// in the list of messages.
//
// Configures the bus for read/write function
// Sets chip address to talk to, message length to be sent.
// Enables appropriate interrupts and sends start xfer command.
//
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_message_start(i2c: *mut exynos5_i2c, stop: c_int) {
    static void exynos5_i2c_message_start(struct exynos5_i2c *i2c, int stop)
    {
    u32 i2c_ctl;
    let mut int_en: u32 = 0;
    let mut i2c_auto_conf: u32 = 0;
    let mut i2c_addr: u32 = 0;
    u32 fifo_ctl;
    unsigned long flags;
    unsigned short trig_lvl;
    if (i2c.variant.hw == I2C_TYPE_EXYNOS5)
    int_en |= HSI2C_INT_I2C;
    else
    int_en |= HSI2C_INT_I2C_TRANS;
    i2c_ctl = readl(i2c.regs + HSI2C_CTL);
    i2c_ctl &= ~(HSI2C_TXCHON | HSI2C_RXCHON);
    fifo_ctl = HSI2C_RXFIFO_EN | HSI2C_TXFIFO_EN;
    if (i2c.msg.flags & I2C_M_RD) {
    i2c_ctl |= HSI2C_RXCHON;
    i2c_auto_conf |= HSI2C_READ_WRITE;
    trig_lvl = (i2c.msg.len > i2c.variant.fifo_depth) ?
    (i2c.variant.fifo_depth * 3 / 4) : i2c.msg.len;
    fifo_ctl |= HSI2C_RXFIFO_TRIGGER_LEVEL(trig_lvl);
    int_en |= (HSI2C_INT_RX_ALMOSTFULL_EN |
    HSI2C_INT_TRAILING_EN);
    } else {
    i2c_ctl |= HSI2C_TXCHON;
    trig_lvl = (i2c.msg.len > i2c.variant.fifo_depth) ?
    (i2c.variant.fifo_depth * 1 / 4) : i2c.msg.len;
    fifo_ctl |= HSI2C_TXFIFO_TRIGGER_LEVEL(trig_lvl);
    int_en |= HSI2C_INT_TX_ALMOSTEMPTY_EN;
    }
    i2c_addr = HSI2C_SLV_ADDR_MAS(i2c.msg.addr);
    if (i2c.op_clock >= I2C_MAX_FAST_MODE_PLUS_FREQ)
    i2c_addr |= HSI2C_MASTER_ID(MASTER_ID(i2c.adap.nr));
    writel(i2c_addr, i2c.regs + HSI2C_ADDR);
    writel(fifo_ctl, i2c.regs + HSI2C_FIFO_CTL);
    writel(i2c_ctl, i2c.regs + HSI2C_CTL);
    exynos5_i2c_bus_check(i2c);
//
// Enable interrupts before starting the transfer so that we don't
// miss any INT_I2C interrupts.
//
    spin_lock_irqsave(&i2c.lock, flags);
    writel(int_en, i2c.regs + HSI2C_INT_ENABLE);
    if (stop == 1)
    i2c_auto_conf |= HSI2C_STOP_AFTER_TRANS;
    i2c_auto_conf |= i2c.msg.len;
    i2c_auto_conf |= HSI2C_MASTER_RUN;
    writel(i2c_auto_conf, i2c.regs + HSI2C_AUTO_CONF);
    spin_unlock_irqrestore(&i2c.lock, flags);
    }
    static bool exynos5_i2c_poll_irqs_timeout(struct exynos5_i2c *i2c,
    unsigned long timeout)
    {
    let mut time_left: c_ulong = jiffies + timeout;
    while (time_before(jiffies, time_left) &&
    !((i2c.trans_done && (i2c.msg.len == i2c.msg_ptr)) ||
    (i2c.state < 0))) {
    while (readl(i2c.regs + HSI2C_INT_ENABLE) &
    readl(i2c.regs + HSI2C_INT_STATUS))
    exynos5_i2c_irq(i2c.irq, i2c);
    usleep_range(100, 200);
    }
    return time_before(jiffies, time_left);
    }
    static int exynos5_i2c_xfer_msg(struct exynos5_i2c *i2c,
    struct i2c_msg *msgs, int stop)
    {
    unsigned long time_left;
    int ret;
    i2c.msg = msgs;
    i2c.msg_ptr = 0;
    i2c.trans_done = 0;
    reinit_completion(&i2c.msg_complete);
    exynos5_i2c_message_start(i2c, stop);
    if (!i2c.atomic)
    time_left = wait_for_completion_timeout(&i2c.msg_complete,
    EXYNOS5_I2C_TIMEOUT);
    else
    time_left = exynos5_i2c_poll_irqs_timeout(i2c,
    EXYNOS5_I2C_TIMEOUT);
    if (time_left == 0)
    ret = -ETIMEDOUT;
    else
    ret = i2c.state;
//
// If this is the last message to be transferred (stop == 1)
// Then check if the bus can be brought back to idle.
//
    if (ret == 0 && stop)
    ret = exynos5_i2c_wait_bus_idle(i2c);
    if (ret < 0) {
    exynos5_i2c_reset(i2c);
    if (ret == -ETIMEDOUT)
    dev_warn(i2c.dev, "%s timeout\n",
    (msgs.flags & I2C_M_RD) ? "rx" : "tx");
    }
// Return the state as in interrupt routine
    return ret;
    }
    static int exynos5_i2c_xfer(struct i2c_adapter *adap,
    struct i2c_msg *msgs, int num)
    {
    struct exynos5_i2c *i2c = adap.algo_data;
    int i, ret;
    ret = clk_enable(i2c.pclk);
    if (ret)
    return ret;
    ret = clk_enable(i2c.clk);
    if (ret)
    goto err_pclk;
    for (i = 0; i < num; ++i) {
    ret = exynos5_i2c_xfer_msg(i2c, msgs + i, i + 1 == num);
    if (ret)
    break;
    }
    clk_disable(i2c.clk);
    err_pclk:
    clk_disable(i2c.pclk);
    return ret ?: num;
    }
    static int exynos5_i2c_xfer_atomic(struct i2c_adapter *adap,
    struct i2c_msg *msgs, int num)
    {
    struct exynos5_i2c *i2c = adap.algo_data;
    int ret;
    disable_irq(i2c.irq);
    i2c.atomic = true;
    ret = exynos5_i2c_xfer(adap, msgs, num);
    i2c.atomic = false;
    enable_irq(i2c.irq);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 exynos5_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | (I2C_FUNC_SMBUS_EMUL & ~I2C_FUNC_SMBUS_QUICK);
    }
    static const struct i2c_algorithm exynos5_i2c_algorithm = {
    .xfer = exynos5_i2c_xfer,
    .xfer_atomic = exynos5_i2c_xfer_atomic,
    .functionality = exynos5_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int exynos5_i2c_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct exynos5_i2c *i2c;
    int ret;
    i2c = devm_kzalloc(&pdev.dev, sizeof(struct exynos5_i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    if (of_property_read_u32(np, "clock-frequency", &i2c.op_clock))
    i2c.op_clock = I2C_MAX_STANDARD_MODE_FREQ;
    strscpy(i2c.adap.name, "exynos5-i2c", sizeof(i2c.adap.name));
    i2c.adap.owner   = THIS_MODULE;
    i2c.adap.algo    = &exynos5_i2c_algorithm;
    i2c.adap.retries = 3;
    i2c.dev = &pdev.dev;
    i2c.clk = devm_clk_get(&pdev.dev, "hsi2c");
    if (IS_ERR(i2c.clk)) {
    dev_err(&pdev.dev, "cannot get clock\n");
    return -ENOENT;
    }
    i2c.pclk = devm_clk_get_optional(&pdev.dev, "hsi2c_pclk");
    if (IS_ERR(i2c.pclk)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(i2c.pclk),
    "cannot get pclk");
    }
    ret = clk_prepare_enable(i2c.pclk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(i2c.clk);
    if (ret)
    goto err_pclk;
    i2c.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2c.regs)) {
    ret = PTR_ERR(i2c.regs);
    goto err_clk;
    }
    i2c.adap.dev.of_node = np;
    i2c.adap.algo_data = i2c;
    i2c.adap.dev.parent = &pdev.dev;
// Clear pending interrupts from u-boot or misc causes
    exynos5_i2c_clr_pend_irq(i2c);
    spin_lock_init(&i2c.lock);
    init_completion(&i2c.msg_complete);
    i2c.irq = ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    goto err_clk;
    ret = devm_request_irq(&pdev.dev, i2c.irq, exynos5_i2c_irq,
    IRQF_NO_SUSPEND, dev_name(&pdev.dev), i2c);
    if (ret != 0) {
    dev_err(&pdev.dev, "cannot request HS-I2C IRQ %d\n", i2c.irq);
    goto err_clk;
    }
    i2c.variant = of_device_get_match_data(&pdev.dev);
    ret = exynos5_hsi2c_clock_setup(i2c);
    if (ret)
    goto err_clk;
    exynos5_i2c_reset(i2c);
    ret = i2c_add_adapter(&i2c.adap);
    if (ret < 0)
    goto err_clk;
    platform_set_drvdata(pdev, i2c);
    clk_disable(i2c.clk);
    clk_disable(i2c.pclk);
    return 0;
    err_clk:
    clk_disable_unprepare(i2c.clk);
    err_pclk:
    clk_disable_unprepare(i2c.pclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_remove(pdev: *mut platform_device) {
    static void exynos5_i2c_remove(struct platform_device *pdev)
    {
    struct exynos5_i2c *i2c = platform_get_drvdata(pdev);
    i2c_del_adapter(&i2c.adap);
    clk_unprepare(i2c.clk);
    clk_unprepare(i2c.pclk);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_suspend_noirq(dev: *mut device) -> c_int {
    static int exynos5_i2c_suspend_noirq(struct device *dev)
    {
    struct exynos5_i2c *i2c = dev_get_drvdata(dev);
    i2c_mark_adapter_suspended(&i2c.adap);
    clk_unprepare(i2c.clk);
    clk_unprepare(i2c.pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_i2c_resume_noirq(dev: *mut device) -> c_int {
    static int exynos5_i2c_resume_noirq(struct device *dev)
    {
    struct exynos5_i2c *i2c = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    ret = clk_prepare_enable(i2c.pclk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(i2c.clk);
    if (ret)
    goto err_pclk;
    ret = exynos5_hsi2c_clock_setup(i2c);
    if (ret)
    goto err_clk;
    exynos5_i2c_init(i2c);
    clk_disable(i2c.clk);
    clk_disable(i2c.pclk);
    i2c_mark_adapter_resumed(&i2c.adap);
    return 0;
    err_clk:
    clk_disable_unprepare(i2c.clk);
    err_pclk:
    clk_disable_unprepare(i2c.pclk);
    return ret;
    }
    static const struct dev_pm_ops exynos5_i2c_dev_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(exynos5_i2c_suspend_noirq,
    exynos5_i2c_resume_noirq)
    };
    static struct platform_driver exynos5_i2c_driver = {
    .probe		= exynos5_i2c_probe,
    .remove		= exynos5_i2c_remove,
    .driver		= {
    .name	= "exynos5-hsi2c",
    .pm	= pm_sleep_ptr(&exynos5_i2c_dev_pm_ops),
    .of_match_table = exynos5_i2c_match,
    },
    };
    module_platform_driver(exynos5_i2c_driver);
    MODULE_DESCRIPTION("Exynos5 HS-I2C Bus driver");
    MODULE_AUTHOR("Naveen Krishna Chatradhi <ch.naveen@samsung.com>");
    MODULE_AUTHOR("Taekgyun Ko <taeggyun.ko@samsung.com>");
    MODULE_LICENSE("GPL v2");
