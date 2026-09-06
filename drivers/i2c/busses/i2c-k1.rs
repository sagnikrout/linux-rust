//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-k1.c
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
// Copyright (C) 2024-2025 Troy Mitchell <troymitchell988@gmail.com>
//

// spacemit i2c registers
pub const SPACEMIT_ICR: c_uint = 0x0		/* Control register */;
pub const SPACEMIT_ISR: c_uint = 0x4		/* Status register */;
pub const SPACEMIT_IDBR: c_uint = 0xc		/* Data buffer register */;
pub const SPACEMIT_IRCR: c_uint = 0x18		/* Reset cycle counter */;
pub const SPACEMIT_ILCR: c_uint = 0x10		/* Load Count Register */;
pub const SPACEMIT_IWCR: c_uint = 0x14		/* Wait Count Register */;
pub const SPACEMIT_IBMR: c_uint = 0x1c		/* Bus monitor register */;
// SPACEMIT_ICR register fields

// Bits 4-7 are reserved

// Bit 9 is reserved

// Bit 12 is reserved

// Bits 15-17 are reserved

// Bits 23-24 are reserved

    SPACEMIT_CR_DRFIE | SPACEMIT_CR_BEIE | \
    SPACEMIT_CR_TXDONEIE | SPACEMIT_CR_TXEIE | \
    SPACEMIT_CR_RXHFIE | SPACEMIT_CR_RXFIE | \
    SPACEMIT_CR_RXOVIE | SPACEMIT_CR_MSDIE)
// SPACEMIT_ISR register fields
// Bits 0-13 are reserved

// Bit 25 is reserved

    SPACEMIT_SR_TXE | SPACEMIT_SR_TXDONE | SPACEMIT_SR_MSD | \
    SPACEMIT_SR_SSD | SPACEMIT_SR_SAD | SPACEMIT_SR_BED | \
    SPACEMIT_SR_GCAD | SPACEMIT_SR_IRF | SPACEMIT_SR_ITE | \
    SPACEMIT_SR_ALD)

// the cycles of SCL during bus reset

// SPACEMIT_IBMR register fields

// SPACEMIT_IWCR register fields

// Required by I2C IP for correct SCL timing

    FIELD_PREP(SPACEMIT_WCR_HS_COUNT1, 1) | \
    FIELD_PREP(SPACEMIT_WCR_HS_COUNT2, 5))
// i2c bus recover timeout: us
pub const SPACEMIT_I2C_BUS_BUSY_TIMEOUT: c_int = 100000;

pub const SPACEMIT_BUS_RESET_CLK_CNT_MAX: c_int = 9;

    enum spacemit_i2c_state {
    SPACEMIT_STATE_IDLE,
    SPACEMIT_STATE_START,
    SPACEMIT_STATE_READ,
    SPACEMIT_STATE_WRITE,
    };
    enum spacemit_i2c_mode {
    SPACEMIT_MODE_STANDARD,
    SPACEMIT_MODE_FAST
    };
// i2c-spacemit driver's main struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spacemit_i2c_dev {
    pub dev: *mut device,
    pub adapt: i2c_adapter,
    pub scl_clk_hw: clk_hw,
    pub scl_clk: *mut clk,
    pub mode: enum spacemit_i2c_mode,
// hardware resources
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub clock_freq: u32,
    pub msgs: *mut i2c_msg,
    pub msg_num: u32,
// index of the current message being processed
    pub msg_idx: u32,
    pub msg_buf: *mut u8,
// the number of unprocessed bytes remaining in the current message
    pub unprocessed: u32,
    pub state: enum spacemit_i2c_state,
    pub read: bool,
    pub use_pio: bool,
    pub complete: completion,
    pub status: u32,
}

#[no_mangle]
unsafe extern "C" fn spacemit_i2c_scl_clk_disable_unprepare(data: *mut c_void) {
    static void spacemit_i2c_scl_clk_disable_unprepare(void *data)
    {
    clk_disable_unprepare(data);
    }
//
// Calculate the ILCR divider value (lv) from the target SCL rate.
//
// Hardware timing formulas:
// - standard mode: SCL = FCLK / (2 * SLV + 8)
// - fast mode:     SCL = FCLK / (2 * FLV + 10)
//
    static u32 spacemit_i2c_calc_lv(struct spacemit_i2c_dev *i2c,
    unsigned long parent_rate,
    unsigned long target_rate)
    {
    u32 offset, denom;
    offset = (i2c.mode == SPACEMIT_MODE_STANDARD) ? 8 : 10;
    denom = DIV_ROUND_CLOSEST(parent_rate, target_rate);
    return (denom <= offset) ? 0 : DIV_ROUND_CLOSEST(denom - offset, 2);
    }
    static int spacemit_i2c_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct spacemit_i2c_dev *i2c = container_of(hw, struct spacemit_i2c_dev, scl_clk_hw);
    u32 lv, lcr, mask;
    lv = spacemit_i2c_calc_lv(i2c, parent_rate, rate);
    mask = (i2c.mode == SPACEMIT_MODE_STANDARD) ?
    SPACEMIT_LCR_LV_STANDARD_MASK : SPACEMIT_LCR_LV_FAST_MASK;
    lcr = readl(i2c.base + SPACEMIT_ILCR);
    lcr &= ~mask;
    lcr |= field_prep(mask, lv);
    writel(lcr, i2c.base + SPACEMIT_ILCR);
    return 0;
    }
    static int spacemit_i2c_clk_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct spacemit_i2c_dev *i2c = container_of(hw, struct spacemit_i2c_dev, scl_clk_hw);
    u32 lv, offset;
    lv = spacemit_i2c_calc_lv(i2c, req.best_parent_rate, req.rate);
    offset = (i2c.mode == SPACEMIT_MODE_STANDARD) ? 8 : 10;
    req.rate = DIV_ROUND_CLOSEST(req.best_parent_rate, lv * 2 + offset);
    return 0;
    }
    static unsigned long spacemit_i2c_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct spacemit_i2c_dev *i2c = container_of(hw, struct spacemit_i2c_dev, scl_clk_hw);
    u32 lcr, lv = 0;
    lcr = readl(i2c.base + SPACEMIT_ILCR);
    if (i2c.mode == SPACEMIT_MODE_STANDARD) {
    lv = FIELD_GET(SPACEMIT_LCR_LV_STANDARD_MASK, lcr);
    return DIV_ROUND_CLOSEST(parent_rate, lv * 2 + 8);
    }
    lv = FIELD_GET(SPACEMIT_LCR_LV_FAST_MASK, lcr);
    return DIV_ROUND_CLOSEST(parent_rate, lv * 2 + 10);
    }
    static const struct clk_ops spacemit_i2c_clk_ops = {
    .set_rate = spacemit_i2c_clk_set_rate,
    .determine_rate = spacemit_i2c_clk_determine_rate,
    .recalc_rate = spacemit_i2c_clk_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_enable(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_enable(struct spacemit_i2c_dev *i2c)
    {
    u32 val;
    val = readl(i2c.base + SPACEMIT_ICR);
    val |= SPACEMIT_CR_IUE;
    writel(val, i2c.base + SPACEMIT_ICR);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_disable(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_disable(struct spacemit_i2c_dev *i2c)
    {
    u32 val;
    val = readl(i2c.base + SPACEMIT_ICR);
    val &= ~SPACEMIT_CR_IUE;
    writel(val, i2c.base + SPACEMIT_ICR);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_register_scl_clk(i2c: *mut spacemit_i2c_dev) -> c_int {
    static int spacemit_i2c_register_scl_clk(struct spacemit_i2c_dev *i2c)
    {
    let mut init: clk_init_data = {};
    char name[64];
    int ret;
    ret = snprintf(name, sizeof(name), "%s_scl_clk", dev_name(i2c.dev));
    if (ret >= ARRAY_SIZE(name))
    dev_warn(i2c.dev, "scl clock name truncated");
    init.name = name;
    init.ops = &spacemit_i2c_clk_ops;
    init.parent_data = (struct clk_parent_data[]) {
    { .fw_name = "func" },
    };
    init.num_parents = 1;
    i2c.scl_clk_hw.init = &init;
    return devm_clk_hw_register(i2c.dev, &i2c.scl_clk_hw);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_reset(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_reset(struct spacemit_i2c_dev *i2c)
    {
    writel(SPACEMIT_CR_UR, i2c.base + SPACEMIT_ICR);
    udelay(5);
    writel(0, i2c.base + SPACEMIT_ICR);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_handle_err(i2c: *mut spacemit_i2c_dev) -> c_int {
    static int spacemit_i2c_handle_err(struct spacemit_i2c_dev *i2c)
    {
    dev_dbg(i2c.dev, "i2c error status: 0x%08x\n", i2c.status);
// Arbitration Loss Detected
    if (i2c.status & SPACEMIT_SR_ALD) {
    spacemit_i2c_reset(i2c);
    return -EAGAIN;
    }
// Bus Error No ACK/NAK
    if (i2c.status & SPACEMIT_SR_BED)
    spacemit_i2c_reset(i2c);
    return i2c.status & SPACEMIT_SR_ACKNAK ? -ENXIO : -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn spacemit_i2c_delay(i2c: *mut spacemit_i2c_dev, us: c_uint) {
    static inline void spacemit_i2c_delay(struct spacemit_i2c_dev *i2c, unsigned int us)
    {
    if (i2c.use_pio)
    udelay(us);
    else
    fsleep(us);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_conditionally_reset_bus(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_conditionally_reset_bus(struct spacemit_i2c_dev *i2c)
    {
    u32 status;
    u8 clk_cnt;
// if bus is locked, reset unit. 0: locked
    status = readl(i2c.base + SPACEMIT_IBMR);
    if ((status & SPACEMIT_BMR_SDA) && (status & SPACEMIT_BMR_SCL))
    return;
    spacemit_i2c_reset(i2c);
    spacemit_i2c_delay(i2c, 10);
    for (clk_cnt = 0; clk_cnt < SPACEMIT_BUS_RESET_CLK_CNT_MAX; clk_cnt++) {
    status = readl(i2c.base + SPACEMIT_IBMR);
    if (status & SPACEMIT_BMR_SDA)
    return;
// There's nothing left to save here, we are about to exit
    writel(FIELD_PREP(SPACEMIT_RCR_FIELD_RST_CYC, 1),
    i2c.base + SPACEMIT_IRCR);
    writel(SPACEMIT_CR_RSTREQ, i2c.base + SPACEMIT_ICR);
    usleep_range(20, 30);
    }
// check sda again here
    status = readl(i2c.base + SPACEMIT_IBMR);
    if (!(status & SPACEMIT_BMR_SDA))
    dev_warn_ratelimited(i2c.dev, "unit reset failed\n");
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_wait_bus_idle(i2c: *mut spacemit_i2c_dev) -> c_int {
    static int spacemit_i2c_wait_bus_idle(struct spacemit_i2c_dev *i2c)
    {
    int ret;
    u32 val;
    val = readl(i2c.base + SPACEMIT_ISR);
    if (!(val & (SPACEMIT_SR_UB | SPACEMIT_SR_IBB)))
    return 0;
    if (i2c.use_pio)
    ret = readl_poll_timeout_atomic(i2c.base + SPACEMIT_ISR,
    val, !(val & (SPACEMIT_SR_UB | SPACEMIT_SR_IBB)),
    1500, SPACEMIT_I2C_BUS_BUSY_TIMEOUT);
    else
    ret = readl_poll_timeout(i2c.base + SPACEMIT_ISR,
    val, !(val & (SPACEMIT_SR_UB | SPACEMIT_SR_IBB)),
    1500, SPACEMIT_I2C_BUS_BUSY_TIMEOUT);
    if (ret)
    spacemit_i2c_reset(i2c);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_check_bus_release(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_check_bus_release(struct spacemit_i2c_dev *i2c)
    {
// in case bus is not released after transfer completes
    if (readl(i2c.base + SPACEMIT_ISR) & SPACEMIT_SR_EBB) {
    spacemit_i2c_conditionally_reset_bus(i2c);
    spacemit_i2c_delay(i2c, 90);
    }
    }
    static inline void
    spacemit_i2c_clear_int_status(struct spacemit_i2c_dev *i2c, u32 mask)
    {
    writel(mask & SPACEMIT_I2C_INT_STATUS_MASK, i2c.base + SPACEMIT_ISR);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_init(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_init(struct spacemit_i2c_dev *i2c)
    {
    let mut val: u32 = 0;
    if (!i2c.use_pio) {
//
// Enable interrupt bits for all xfer mode:
// bus error, arbitration loss detected.
//
    val |= SPACEMIT_CR_BEIE | SPACEMIT_CR_ALDIE;
//
// Unmask interrupt bits for interrupt xfer mode:
// When IDBR receives a byte, an interrupt is triggered.
//
// For the tx empty interrupt, it will be enabled in the
// i2c_start().
// We don't want a TX empty interrupt until we start
// a transfer in i2c_start().
//
    val |= SPACEMIT_CR_DRFIE;
//
// Enable master stop interrupt bit.
// For transaction complete signal, we use master stop
// interrupt, so we don't need to unmask SPACEMIT_CR_TXDONEIE.
//
    val |= SPACEMIT_CR_MSDIE;
    }
    if (i2c.mode == SPACEMIT_MODE_FAST)
    val |= SPACEMIT_CR_MODE_FAST;
// disable response to general call
    val |= SPACEMIT_CR_GCD;
// enable SCL clock output
    val |= SPACEMIT_CR_SCLE;
// enable master stop detected
    val |= SPACEMIT_CR_MSDE;
    writel(val, i2c.base + SPACEMIT_ICR);
//
// The glitch fix in the K1 I2C controller introduces a delay
// on restart signals, so we disable the fix here.
//
    val = readl(i2c.base + SPACEMIT_IRCR);
    val |= SPACEMIT_RCR_SDA_GLITCH_NOFIX;
    writel(val, i2c.base + SPACEMIT_IRCR);
    spacemit_i2c_clear_int_status(i2c, SPACEMIT_I2C_INT_STATUS_MASK);
//
// Initialize IWCR to the value specified by the I2C IP designer.
// The SCL frequency formulas (SCL = FCLK / (2*SLV+8) for standard
// mode, SCL = FCLK / (2*FLV+10) for fast mode) are only valid when
// IWCR contains this specific value.
//
    writel(SPACEMIT_IWCR_INIT_VALUE, i2c.base + SPACEMIT_IWCR);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_start(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_start(struct spacemit_i2c_dev *i2c)
    {
    u32 target_addr_rw, val;
    struct i2c_msg *cur_msg = i2c.msgs + i2c.msg_idx;
    i2c.read = !!(cur_msg.flags & I2C_M_RD);
    i2c.state = SPACEMIT_STATE_START;
    target_addr_rw = (cur_msg.addr & 0x7f) << 1;
    if (cur_msg.flags & I2C_M_RD)
    target_addr_rw |= 1;
    writel(target_addr_rw, i2c.base + SPACEMIT_IDBR);
// send start pulse
    val = readl(i2c.base + SPACEMIT_ICR);
    val &= ~SPACEMIT_CR_STOP;
    val |= SPACEMIT_CR_START | SPACEMIT_CR_TB;
// Enable the TX empty interrupt
    if (!i2c.use_pio)
    val |= SPACEMIT_CR_DTEIE;
    writel(val, i2c.base + SPACEMIT_ICR);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_is_last_msg(i2c: *mut spacemit_i2c_dev) -> bool {
    static bool spacemit_i2c_is_last_msg(struct spacemit_i2c_dev *i2c)
    {
    if (i2c.msg_idx != i2c.msg_num - 1)
    return false;
    if (i2c.read)
    return i2c.unprocessed == 1;
    return !i2c.unprocessed;
    }
#[no_mangle]
pub unsafe extern "C" fn spacemit_i2c_complete(i2c: *mut spacemit_i2c_dev) {
    static inline void spacemit_i2c_complete(struct spacemit_i2c_dev *i2c)
    {
// SPACEMIT_STATE_IDLE avoids triggering the next byte
    i2c.state = SPACEMIT_STATE_IDLE;
    if (i2c.use_pio)
    return;
    complete(&i2c.complete);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_handle_write(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_handle_write(struct spacemit_i2c_dev *i2c)
    {
// If there's no space in the IDBR, we're done
    if (!(i2c.status & SPACEMIT_SR_ITE))
    return;
// if transfer completes, SPACEMIT_ISR will handle it
    if (i2c.status & SPACEMIT_SR_MSD)
    return;
    if (i2c.unprocessed) {
    writel(*i2c.msg_buf++, i2c.base + SPACEMIT_IDBR);
    i2c.unprocessed--;
    return;
    }
    spacemit_i2c_complete(i2c);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_handle_read(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_handle_read(struct spacemit_i2c_dev *i2c)
    {
// If there's nothing in the IDBR, we're done
    if (!(i2c.status & SPACEMIT_SR_IRF))
    return;
    if (i2c.unprocessed) {
// i2c->msg_buf++ = readl(i2c->base + SPACEMIT_IDBR);
    i2c.unprocessed--;
    return;
    }
// if transfer completes, SPACEMIT_ISR will handle it
    if (i2c.status & (SPACEMIT_SR_MSD | SPACEMIT_SR_ACKNAK))
    return;
// it has to append stop bit in icr that read last byte
    if (i2c.unprocessed)
    return;
    spacemit_i2c_complete(i2c);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_handle_start(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_handle_start(struct spacemit_i2c_dev *i2c)
    {
    i2c.state = i2c.read ? SPACEMIT_STATE_READ : SPACEMIT_STATE_WRITE;
    if (i2c.state == SPACEMIT_STATE_WRITE)
    spacemit_i2c_handle_write(i2c);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_err_check(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_err_check(struct spacemit_i2c_dev *i2c)
    {
    u32 val;
//
// Send transaction complete signal:
// error happens, detect master stop
//
    if (!(i2c.status & (SPACEMIT_SR_ERR | SPACEMIT_SR_MSD)))
    return;
//
// Here the transaction is already done, we don't need any
// other interrupt signals from now, in case any interrupt
// happens before spacemit_i2c_xfer to disable irq and i2c unit,
// we mask all the interrupt signals and clear the interrupt
// status.
//
    val = readl(i2c.base + SPACEMIT_ICR);
    val &= ~SPACEMIT_I2C_INT_CTRL_MASK;
    writel(val, i2c.base + SPACEMIT_ICR);
    spacemit_i2c_clear_int_status(i2c, SPACEMIT_I2C_INT_STATUS_MASK);
    spacemit_i2c_complete(i2c);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_handle_state(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_handle_state(struct spacemit_i2c_dev *i2c)
    {
    u32 val;
    if (i2c.status & SPACEMIT_SR_ERR)
    goto err_out;
    switch (i2c.state) {
    case SPACEMIT_STATE_START:
    spacemit_i2c_handle_start(i2c);
    break;
    case SPACEMIT_STATE_READ:
    spacemit_i2c_handle_read(i2c);
    break;
    case SPACEMIT_STATE_WRITE:
    spacemit_i2c_handle_write(i2c);
    break;
    default:
    break;
    }
    if (i2c.state != SPACEMIT_STATE_IDLE) {
    val = readl(i2c.base + SPACEMIT_ICR);
    val &= ~(SPACEMIT_CR_TB | SPACEMIT_CR_ACKNAK |
    SPACEMIT_CR_STOP | SPACEMIT_CR_START);
    val |= SPACEMIT_CR_TB;
    if (!i2c.use_pio)
    val |= SPACEMIT_CR_ALDIE;
    if (spacemit_i2c_is_last_msg(i2c)) {
// trigger next byte with stop
    val |= SPACEMIT_CR_STOP;
    if (i2c.read)
    val |= SPACEMIT_CR_ACKNAK;
    }
    writel(val, i2c.base + SPACEMIT_ICR);
    }
    err_out:
    spacemit_i2c_err_check(i2c);
    }
//
// In PIO mode, this function is used as a replacement for
// wait_for_completion_timeout(), whose return value indicates
// the remaining time.
//
// We do not have a meaningful remaining-time value here, so
// return a non-zero value on success to indicate "not timed out".
// Returning 1 ensures callers treating the return value as
// time_left will not incorrectly report a timeout.
//
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_wait_pio_xfer(i2c: *mut spacemit_i2c_dev) -> c_int {
    static int spacemit_i2c_wait_pio_xfer(struct spacemit_i2c_dev *i2c)
    {
    u32 mask, msec = jiffies_to_msecs(i2c.adapt.timeout);
    let mut timeout: ktime_t = ktime_add_ms(ktime_get(), msec);
    int ret;
    mask = SPACEMIT_SR_IRF | SPACEMIT_SR_ITE;
    do {
    i2c.status = readl(i2c.base + SPACEMIT_ISR);
    spacemit_i2c_clear_int_status(i2c, i2c.status);
    if (i2c.status & mask)
    spacemit_i2c_handle_state(i2c);
    else
    udelay(SPACEMIT_POLL_INTERVAL);
    } while (i2c.unprocessed && ktime_compare(ktime_get(), timeout) < 0);
    if (i2c.unprocessed)
    return 0;
    if (i2c.read)
    return 1;
//
// If this is the last byte to write of the current message,
// we have to wait here. Otherwise, control will proceed directly
// to start(), which would overwrite the current data.
//
    ret = readl_poll_timeout_atomic(i2c.base + SPACEMIT_ISR,
    i2c.status, i2c.status & SPACEMIT_SR_ITE,
    SPACEMIT_POLL_INTERVAL, SPACEMIT_POLL_TIMEOUT);
    if (ret)
    return 0;
//
// For writes: in interrupt mode, an ITE (write-empty) interrupt is triggered
// after the last byte, and the MSD-related handling takes place there.
// In PIO mode, however, we need to explicitly call err_check() to emulate this
// step, otherwise the next transfer will fail.
//
    if (i2c.msg_idx == i2c.msg_num - 1) {
    mask = SPACEMIT_SR_MSD | SPACEMIT_SR_ERR;
//
// In some cases, MSD may not arrive immediately;
// wait here to handle that.
//
    ret = readl_poll_timeout_atomic(i2c.base + SPACEMIT_ISR,
    i2c.status, i2c.status & mask,
    SPACEMIT_POLL_INTERVAL, SPACEMIT_POLL_TIMEOUT);
    if (ret)
    return 0;
    spacemit_i2c_err_check(i2c);
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_wait_xfer_complete(i2c: *mut spacemit_i2c_dev) -> c_int {
    static int spacemit_i2c_wait_xfer_complete(struct spacemit_i2c_dev *i2c)
    {
    if (i2c.use_pio)
    return spacemit_i2c_wait_pio_xfer(i2c);
    return wait_for_completion_timeout(&i2c.complete,
    i2c.adapt.timeout);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_xfer_msg(i2c: *mut spacemit_i2c_dev) -> c_int {
    static int spacemit_i2c_xfer_msg(struct spacemit_i2c_dev *i2c)
    {
    unsigned long time_left;
    struct i2c_msg *msg;
    for (i2c.msg_idx = 0; i2c.msg_idx < i2c.msg_num; i2c.msg_idx++) {
    msg = &i2c.msgs[i2c.msg_idx];
    i2c.msg_buf = msg.buf;
    i2c.unprocessed = msg.len;
    i2c.status = 0;
    reinit_completion(&i2c.complete);
    spacemit_i2c_start(i2c);
    time_left = spacemit_i2c_wait_xfer_complete(i2c);
    if (!time_left) {
    dev_err(i2c.dev, "msg completion timeout\n");
    spacemit_i2c_conditionally_reset_bus(i2c);
    spacemit_i2c_reset(i2c);
    return -ETIMEDOUT;
    }
    if (i2c.status & SPACEMIT_SR_ERR)
    return spacemit_i2c_handle_err(i2c);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_irq_handler(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t spacemit_i2c_irq_handler(int irq, void *devid)
    {
    struct spacemit_i2c_dev *i2c = devid;
    u32 status;
    status = readl(i2c.base + SPACEMIT_ISR);
    if (!status)
    return IRQ_NONE;
    i2c.status = status;
    spacemit_i2c_clear_int_status(i2c, status);
    spacemit_i2c_handle_state(i2c);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_calc_timeout(i2c: *mut spacemit_i2c_dev) {
    static void spacemit_i2c_calc_timeout(struct spacemit_i2c_dev *i2c)
    {
    unsigned long timeout;
    let mut idx: c_int = 0, cnt = 0;
    if (i2c.use_pio) {
    i2c.adapt.timeout = msecs_to_jiffies(SPACEMIT_WAIT_TIMEOUT);
    return;
    }
    for (; idx < i2c.msg_num; idx++)
    cnt += (i2c.msgs + idx).len + 1;
//
// Multiply by 9 because each byte in I2C transmission requires
// 9 clock cycles: 8 bits of data plus 1 ACK/NACK bit.
//
    timeout = cnt * 9 * USEC_PER_SEC / i2c.clock_freq;
    i2c.adapt.timeout = usecs_to_jiffies(timeout + USEC_PER_SEC / 10) / i2c.msg_num;
    }
    static inline int
    spacemit_i2c_xfer_common(struct i2c_adapter *adapt, struct i2c_msg *msgs, int num, bool use_pio)
    {
    struct spacemit_i2c_dev *i2c = i2c_get_adapdata(adapt);
    int ret;
    i2c.use_pio = use_pio;
    i2c.msgs = msgs;
    i2c.msg_num = num;
    spacemit_i2c_calc_timeout(i2c);
    spacemit_i2c_init(i2c);
    spacemit_i2c_enable(i2c);
    ret = spacemit_i2c_wait_bus_idle(i2c);
    if (!ret) {
    ret = spacemit_i2c_xfer_msg(i2c);
    if (ret < 0)
    dev_dbg(i2c.dev, "i2c transfer error: %d\n", ret);
    } else {
    spacemit_i2c_check_bus_release(i2c);
    }
    spacemit_i2c_disable(i2c);
    if (ret == -ETIMEDOUT || ret == -EAGAIN)
    dev_err(i2c.dev, "i2c transfer failed, ret %d err 0x%lx\n",
    ret, i2c.status & SPACEMIT_SR_ERR);
    return ret < 0 ? ret : num;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_xfer(adapt: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int spacemit_i2c_xfer(struct i2c_adapter *adapt, struct i2c_msg *msgs, int num)
    {
    return spacemit_i2c_xfer_common(adapt, msgs, num, false);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_pio_xfer_atomic(adapt: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int spacemit_i2c_pio_xfer_atomic(struct i2c_adapter *adapt, struct i2c_msg *msgs, int num)
    {
    return spacemit_i2c_xfer_common(adapt, msgs, num, true);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 spacemit_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | (I2C_FUNC_SMBUS_EMUL & ~I2C_FUNC_SMBUS_QUICK);
    }
    static const struct i2c_algorithm spacemit_i2c_algo = {
    .xfer = spacemit_i2c_xfer,
    .xfer_atomic = spacemit_i2c_pio_xfer_atomic,
    .functionality = spacemit_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int spacemit_i2c_probe(struct platform_device *pdev)
    {
    struct clk *clk;
    struct device *dev = &pdev.dev;
    struct device_node *of_node = pdev.dev.of_node;
    struct spacemit_i2c_dev *i2c;
    struct reset_control *rst;
    int ret;
    i2c = devm_kzalloc(dev, sizeof(*i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    of_property_read_u32(of_node, "clock-frequency", &i2c.clock_freq);
// For now, this driver doesn't support high-speed.
    if (i2c.clock_freq > I2C_MAX_STANDARD_MODE_FREQ &&
    i2c.clock_freq <= I2C_MAX_FAST_MODE_FREQ) {
    i2c.mode = SPACEMIT_MODE_FAST;
    } else if (i2c.clock_freq && i2c.clock_freq <= I2C_MAX_STANDARD_MODE_FREQ) {
    i2c.mode = SPACEMIT_MODE_STANDARD;
    } else {
    dev_info(dev, "clock-frequency not set or out of range, using fast mode\n");
    i2c.mode = SPACEMIT_MODE_FAST;
    i2c.clock_freq = I2C_MAX_FAST_MODE_FREQ;
    }
    i2c.dev = &pdev.dev;
    i2c.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2c.base))
    return dev_err_probe(dev, PTR_ERR(i2c.base), "failed to do ioremap");
    i2c.irq = platform_get_irq(pdev, 0);
    if (i2c.irq < 0)
    return i2c.irq;
    clk = devm_clk_get_enabled(dev, "func");
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "failed to enable func clock");
    ret = spacemit_i2c_register_scl_clk(i2c);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register scl clock\n");
    i2c.scl_clk = devm_clk_hw_get_clk(dev, &i2c.scl_clk_hw, "scl");
    if (IS_ERR(i2c.scl_clk))
    return dev_err_probe(dev, PTR_ERR(i2c.scl_clk),
    "failed to get scl clock\n");
    clk = devm_clk_get_enabled(dev, "bus");
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "failed to enable bus clock");
    rst = devm_reset_control_get_optional_exclusive_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(rst))
    return dev_err_probe(dev, PTR_ERR(rst),
    "failed to acquire deasserted reset\n");
    ret = clk_set_rate(i2c.scl_clk, i2c.clock_freq);
    if (ret)
    return dev_err_probe(dev, ret, "failed to set rate for SCL clock");
    ret = clk_prepare_enable(i2c.scl_clk);
    if (ret)
    return dev_err_probe(dev, ret, "failed to prepare and enable clock");
    ret = devm_add_action_or_reset(dev, spacemit_i2c_scl_clk_disable_unprepare,
    i2c.scl_clk);
    if (ret)
    return ret;
    spacemit_i2c_reset(i2c);
    i2c_set_adapdata(&i2c.adapt, i2c);
    i2c.adapt.owner = THIS_MODULE;
    i2c.adapt.algo = &spacemit_i2c_algo;
    i2c.adapt.dev.parent = i2c.dev;
    i2c.adapt.nr = pdev.id;
    i2c.adapt.dev.of_node = of_node;
    strscpy(i2c.adapt.name, "spacemit-i2c-adapter", sizeof(i2c.adapt.name));
    init_completion(&i2c.complete);
    ret = devm_request_irq(i2c.dev, i2c.irq, spacemit_i2c_irq_handler,
    IRQF_NO_SUSPEND, dev_name(i2c.dev), i2c);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, i2c);
    ret = i2c_add_numbered_adapter(&i2c.adapt);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to add i2c adapter");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_i2c_remove(pdev: *mut platform_device) {
    static void spacemit_i2c_remove(struct platform_device *pdev)
    {
    struct spacemit_i2c_dev *i2c = platform_get_drvdata(pdev);
    i2c_del_adapter(&i2c.adapt);
    }
    static const struct of_device_id spacemit_i2c_of_match[] = {
    { .compatible = "spacemit,k1-i2c", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, spacemit_i2c_of_match);
    static struct platform_driver spacemit_i2c_driver = {
    .probe = spacemit_i2c_probe,
    .remove = spacemit_i2c_remove,
    .driver = {
    .name = "i2c-k1",
    .of_match_table = spacemit_i2c_of_match,
    },
    };
    module_platform_driver(spacemit_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("I2C bus driver for SpacemiT K1 SoC");
