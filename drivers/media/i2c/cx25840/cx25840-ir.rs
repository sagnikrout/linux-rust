//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/cx25840/cx25840-ir.c
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
// Driver for the Conexant CX2584x Audio/Video decoder chip and related cores
//
// Integrated Consumer Infrared Controller
//
// Copyright (C) 2010  Andy Walls <awalls@md.metrocast.net>
//

    static unsigned int ir_debug;
    module_param(ir_debug, int, 0644);
    MODULE_PARM_DESC(ir_debug, "enable integrated IR debug messages");
pub const CX25840_IR_REG_BASE: c_uint = 0x200;
pub const CX25840_IR_CNTRL_REG: c_uint = 0x200;
pub const CNTRL_WIN_3_3: c_uint = 0x00000000;
pub const CNTRL_WIN_4_3: c_uint = 0x00000001;
pub const CNTRL_WIN_3_4: c_uint = 0x00000002;
pub const CNTRL_WIN_4_4: c_uint = 0x00000003;
pub const CNTRL_WIN: c_uint = 0x00000003;
pub const CNTRL_EDG_NONE: c_uint = 0x00000000;
pub const CNTRL_EDG_FALL: c_uint = 0x00000004;
pub const CNTRL_EDG_RISE: c_uint = 0x00000008;
pub const CNTRL_EDG_BOTH: c_uint = 0x0000000C;
pub const CNTRL_EDG: c_uint = 0x0000000C;
pub const CNTRL_DMD: c_uint = 0x00000010;
pub const CNTRL_MOD: c_uint = 0x00000020;
pub const CNTRL_RFE: c_uint = 0x00000040;
pub const CNTRL_TFE: c_uint = 0x00000080;
pub const CNTRL_RXE: c_uint = 0x00000100;
pub const CNTRL_TXE: c_uint = 0x00000200;
pub const CNTRL_RIC: c_uint = 0x00000400;
pub const CNTRL_TIC: c_uint = 0x00000800;
pub const CNTRL_CPL: c_uint = 0x00001000;
pub const CNTRL_LBM: c_uint = 0x00002000;
pub const CNTRL_R: c_uint = 0x00004000;
pub const CX25840_IR_TXCLK_REG: c_uint = 0x204;
pub const TXCLK_TCD: c_uint = 0x0000FFFF;
pub const CX25840_IR_RXCLK_REG: c_uint = 0x208;
pub const RXCLK_RCD: c_uint = 0x0000FFFF;
pub const CX25840_IR_CDUTY_REG: c_uint = 0x20C;
pub const CDUTY_CDC: c_uint = 0x0000000F;
pub const CX25840_IR_STATS_REG: c_uint = 0x210;
pub const STATS_RTO: c_uint = 0x00000001;
pub const STATS_ROR: c_uint = 0x00000002;
pub const STATS_RBY: c_uint = 0x00000004;
pub const STATS_TBY: c_uint = 0x00000008;
pub const STATS_RSR: c_uint = 0x00000010;
pub const STATS_TSR: c_uint = 0x00000020;
pub const CX25840_IR_IRQEN_REG: c_uint = 0x214;
pub const IRQEN_RTE: c_uint = 0x00000001;
pub const IRQEN_ROE: c_uint = 0x00000002;
pub const IRQEN_RSE: c_uint = 0x00000010;
pub const IRQEN_TSE: c_uint = 0x00000020;
pub const IRQEN_MSK: c_uint = 0x00000033;
pub const CX25840_IR_FILTR_REG: c_uint = 0x218;
pub const FILTR_LPF: c_uint = 0x0000FFFF;
pub const CX25840_IR_FIFO_REG: c_uint = 0x23C;
pub const FIFO_RXTX: c_uint = 0x0000FFFF;
pub const FIFO_RXTX_LVL: c_uint = 0x00010000;
pub const FIFO_RXTX_RTO: c_uint = 0x0001FFFF;
pub const FIFO_RX_NDV: c_uint = 0x00020000;
pub const FIFO_RX_DEPTH: c_int = 8;
pub const FIFO_TX_DEPTH: c_int = 8;

//
// We use this union internally for convenience, but callers to tx_write
// and rx_read will be expecting records of type struct ir_raw_event.
// Always ensure the size of this union is dictated by struct ir_raw_event.
//
    union cx25840_ir_fifo_rec {
    u32 hw_fifo_data;
    struct ir_raw_event ir_core_data;
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25840_ir_state {
    pub c: *mut i2c_client,
    pub rx_params: v4l2_subdev_ir_parameters,
    pub /: *mut *mut mutex rx_params_lock; / protects Rx parameter settings cache,
    pub rxclk_divider: core::sync::atomic::AtomicI32,
    pub rx_invert: core::sync::atomic::AtomicI32,
    pub rx_kfifo: kfifo,
    pub /: *mut *mut spinlock_t rx_kfifo_lock; / protect Rx data kfifo,
    pub tx_params: v4l2_subdev_ir_parameters,
    pub /: *mut *mut mutex tx_params_lock; / protects Tx parameter settings cache,
    pub txclk_divider: core::sync::atomic::AtomicI32,
}

    static inline struct cx25840_ir_state *to_ir_state(struct v4l2_subdev *sd)
    {
    struct cx25840_state *state = to_state(sd);
    return state ? state.ir_state : core::ptr::null_mut();
    }
//
// Rx and Tx Clock Divider register computations
//
// Note the largest clock divider value of 0xffff corresponds to:
// (0xffff + 1) * 1000 / 108/2 MHz = 1,213,629.629... ns
// which fits in 21 bits, so we'll use unsigned int for time arguments.
//
#[no_mangle]
pub unsafe extern "C" fn count_to_clock_divider(d: c_uint) -> u16 {
    static inline u16 count_to_clock_divider(unsigned int d)
    {
    if (d > RXCLK_RCD + 1)
    d = RXCLK_RCD;
#[no_mangle]
pub unsafe extern "C" fn if(2: d <) -> else {
    else if (d < 2)
    d = 1;
    else
    d--;
    return (u16) d;
    }
#[no_mangle]
pub unsafe extern "C" fn carrier_freq_to_clock_divider(freq: c_uint) -> u16 {
    static inline u16 carrier_freq_to_clock_divider(unsigned int freq)
    {
    return count_to_clock_divider(
    DIV_ROUND_CLOSEST(CX25840_IR_REFCLK_FREQ, freq * 16));
    }
#[no_mangle]
pub unsafe extern "C" fn clock_divider_to_carrier_freq(divider: c_uint) -> c_uint {
    static inline unsigned int clock_divider_to_carrier_freq(unsigned int divider)
    {
    return DIV_ROUND_CLOSEST(CX25840_IR_REFCLK_FREQ, (divider + 1) * 16);
    }
    static inline unsigned int clock_divider_to_freq(unsigned int divider,
    unsigned int rollovers)
    {
    return DIV_ROUND_CLOSEST(CX25840_IR_REFCLK_FREQ,
    (divider + 1) * rollovers);
    }
//
// Low Pass Filter register calculations
//
// Note the largest count value of 0xffff corresponds to:
// 0xffff * 1000 / 108/2 MHz = 1,213,611.11... ns
// which fits in 21 bits, so we'll use unsigned int for time arguments.
//
#[no_mangle]
pub unsafe extern "C" fn count_to_lpf_count(d: c_uint) -> u16 {
    static inline u16 count_to_lpf_count(unsigned int d)
    {
    if (d > FILTR_LPF)
    d = FILTR_LPF;
#[no_mangle]
pub unsafe extern "C" fn if(4: d <) -> else {
    else if (d < 4)
    d = 0;
    return (u16) d;
    }
#[no_mangle]
pub unsafe extern "C" fn ns_to_lpf_count(ns: c_uint) -> u16 {
    static inline u16 ns_to_lpf_count(unsigned int ns)
    {
    return count_to_lpf_count(
    DIV_ROUND_CLOSEST(CX25840_IR_REFCLK_FREQ / 1000000 * ns, 1000));
    }
#[no_mangle]
pub unsafe extern "C" fn lpf_count_to_ns(count: c_uint) -> c_uint {
    static inline unsigned int lpf_count_to_ns(unsigned int count)
    {
// Duration of the Low Pass Filter rejection window in ns
    return DIV_ROUND_CLOSEST(count * 1000,
    CX25840_IR_REFCLK_FREQ / 1000000);
    }
#[no_mangle]
pub unsafe extern "C" fn lpf_count_to_us(count: c_uint) -> c_uint {
    static inline unsigned int lpf_count_to_us(unsigned int count)
    {
// Duration of the Low Pass Filter rejection window in us
    return DIV_ROUND_CLOSEST(count, CX25840_IR_REFCLK_FREQ / 1000000);
    }
//
// FIFO register pulse width count computations
//
#[no_mangle]
unsafe extern "C" fn clock_divider_to_resolution(divider: u16) -> u32 {
    static u32 clock_divider_to_resolution(u16 divider)
    {
//
// Resolution is the duration of 1 tick of the readable portion of
// the pulse width counter as read from the FIFO.  The two lsb's are
// not readable, hence the << 2.  This function returns ns.
//
    return DIV_ROUND_CLOSEST((1 << 2)  * ((u32) divider + 1) * 1000,
    CX25840_IR_REFCLK_FREQ / 1000000);
    }
#[no_mangle]
unsafe extern "C" fn pulse_width_count_to_ns(count: u16, divider: u16) -> u64 {
    static u64 pulse_width_count_to_ns(u16 count, u16 divider)
    {
    u64 n;
    u32 rem;
//
// The 2 lsb's of the pulse width timer count are not readable, hence
// the (count << 2) | 0x3
//
    n = (((u64) count << 2) | 0x3) * (divider + 1) * 1000; /* millicycles */
    rem = do_div(n, CX25840_IR_REFCLK_FREQ / 1000000);     /* / MHz => ns */
    if (rem >= CX25840_IR_REFCLK_FREQ / 1000000 / 2)
    n++;
    return n;
    }

// Keep as we will need this for Transmit functionality
#[no_mangle]
unsafe extern "C" fn ns_to_pulse_width_count(ns: u32, divider: u16) -> u16 {
    static u16 ns_to_pulse_width_count(u32 ns, u16 divider)
    {
    u64 n;
    u32 d;
    u32 rem;
//
// The 2 lsb's of the pulse width timer count are not accessible, hence
// the (1 << 2)
//
    n = ((u64) ns) * CX25840_IR_REFCLK_FREQ / 1000000; /* millicycles */
    d = (1 << 2) * ((u32) divider + 1) * 1000; /* millicycles/count */
    rem = do_div(n, d);
    if (rem >= d / 2)
    n++;
    if (n > FIFO_RXTX)
    n = FIFO_RXTX;
#[no_mangle]
pub unsafe extern "C" fn if(0: n ==) -> else {
    else if (n == 0)
    n = 1;
    return (u16) n;
    }

#[no_mangle]
unsafe extern "C" fn pulse_width_count_to_us(count: u16, divider: u16) -> c_uint {
    static unsigned int pulse_width_count_to_us(u16 count, u16 divider)
    {
    u64 n;
    u32 rem;
//
// The 2 lsb's of the pulse width timer count are not readable, hence
// the (count << 2) | 0x3
//
    n = (((u64) count << 2) | 0x3) * (divider + 1);    /* cycles      */
    rem = do_div(n, CX25840_IR_REFCLK_FREQ / 1000000); /* / MHz => us */
    if (rem >= CX25840_IR_REFCLK_FREQ / 1000000 / 2)
    n++;
    return (unsigned int) n;
    }
//
// Pulse Clocks computations: Combined Pulse Width Count & Rx Clock Counts
//
// The total pulse clock count is an 18 bit pulse width timer count as the most
// significant part and (up to) 16 bit clock divider count as a modulus.
// When the Rx clock divider ticks down to 0, it increments the 18 bit pulse
// width timer count's least significant bit.
//
#[no_mangle]
unsafe extern "C" fn ns_to_pulse_clocks(ns: u32) -> u64 {
    static u64 ns_to_pulse_clocks(u32 ns)
    {
    u64 clocks;
    u32 rem;
    clocks = CX25840_IR_REFCLK_FREQ / 1000000 * (u64) ns; /* millicycles  */
    rem = do_div(clocks, 1000);                         /* /1000 = cycles */
    if (rem >= 1000 / 2)
    clocks++;
    return clocks;
    }
#[no_mangle]
unsafe extern "C" fn pulse_clocks_to_clock_divider(count: u64) -> u16 {
    static u16 pulse_clocks_to_clock_divider(u64 count)
    {
    do_div(count, (FIFO_RXTX << 2) | 0x3);
// net result needs to be rounded down and decremented by 1
    if (count > RXCLK_RCD + 1)
    count = RXCLK_RCD;
#[no_mangle]
pub unsafe extern "C" fn if(2: count <) -> else {
    else if (count < 2)
    count = 1;
    else
    count--;
    return (u16) count;
    }
//
// IR Control Register helpers
//
    enum tx_fifo_watermark {
    TX_FIFO_HALF_EMPTY = 0,
    TX_FIFO_EMPTY      = CNTRL_TIC,
    };
    enum rx_fifo_watermark {
    RX_FIFO_HALF_FULL = 0,
    RX_FIFO_NOT_EMPTY = CNTRL_RIC,
    };
    static inline void control_tx_irq_watermark(struct i2c_client *c,
    enum tx_fifo_watermark level)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~CNTRL_TIC, level);
    }
    static inline void control_rx_irq_watermark(struct i2c_client *c,
    enum rx_fifo_watermark level)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~CNTRL_RIC, level);
    }
#[no_mangle]
pub unsafe extern "C" fn control_tx_enable(c: *mut i2c_client, enable: bool) {
    static inline void control_tx_enable(struct i2c_client *c, bool enable)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~(CNTRL_TXE | CNTRL_TFE),
    enable ? (CNTRL_TXE | CNTRL_TFE) : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn control_rx_enable(c: *mut i2c_client, enable: bool) {
    static inline void control_rx_enable(struct i2c_client *c, bool enable)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~(CNTRL_RXE | CNTRL_RFE),
    enable ? (CNTRL_RXE | CNTRL_RFE) : 0);
    }
    static inline void control_tx_modulation_enable(struct i2c_client *c,
    bool enable)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~CNTRL_MOD,
    enable ? CNTRL_MOD : 0);
    }
    static inline void control_rx_demodulation_enable(struct i2c_client *c,
    bool enable)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~CNTRL_DMD,
    enable ? CNTRL_DMD : 0);
    }
    static inline void control_rx_s_edge_detection(struct i2c_client *c,
    u32 edge_types)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~CNTRL_EDG_BOTH,
    edge_types & CNTRL_EDG_BOTH);
    }
    static void control_rx_s_carrier_window(struct i2c_client *c,
    unsigned int carrier,
    unsigned int *carrier_range_low,
    unsigned int *carrier_range_high)
    {
    u32 v;
    let mut c16: c_uint = carrier * 16;
    if (*carrier_range_low < DIV_ROUND_CLOSEST(c16, 16 + 3)) {
    v = CNTRL_WIN_3_4;
// carrier_range_low = DIV_ROUND_CLOSEST(c16, 16 + 4);
    } else {
    v = CNTRL_WIN_3_3;
// carrier_range_low = DIV_ROUND_CLOSEST(c16, 16 + 3);
    }
    if (*carrier_range_high > DIV_ROUND_CLOSEST(c16, 16 - 3)) {
    v |= CNTRL_WIN_4_3;
// carrier_range_high = DIV_ROUND_CLOSEST(c16, 16 - 4);
    } else {
    v |= CNTRL_WIN_3_3;
// carrier_range_high = DIV_ROUND_CLOSEST(c16, 16 - 3);
    }
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~CNTRL_WIN, v);
    }
    static inline void control_tx_polarity_invert(struct i2c_client *c,
    bool invert)
    {
    cx25840_and_or4(c, CX25840_IR_CNTRL_REG, ~CNTRL_CPL,
    invert ? CNTRL_CPL : 0);
    }
//
// IR Rx & Tx Clock Register helpers
//
    static unsigned int txclk_tx_s_carrier(struct i2c_client *c,
    unsigned int freq,
    u16 *divider)
    {
// divider = carrier_freq_to_clock_divider(freq);
    cx25840_write4(c, CX25840_IR_TXCLK_REG, *divider);
    return clock_divider_to_carrier_freq(*divider);
    }
    static unsigned int rxclk_rx_s_carrier(struct i2c_client *c,
    unsigned int freq,
    u16 *divider)
    {
// divider = carrier_freq_to_clock_divider(freq);
    cx25840_write4(c, CX25840_IR_RXCLK_REG, *divider);
    return clock_divider_to_carrier_freq(*divider);
    }
    static u32 txclk_tx_s_max_pulse_width(struct i2c_client *c, u32 ns,
    u16 *divider)
    {
    u64 pulse_clocks;
    if (ns > IR_MAX_DURATION)
    ns = IR_MAX_DURATION;
    pulse_clocks = ns_to_pulse_clocks(ns);
// divider = pulse_clocks_to_clock_divider(pulse_clocks);
    cx25840_write4(c, CX25840_IR_TXCLK_REG, *divider);
    return (u32) pulse_width_count_to_ns(FIFO_RXTX, *divider);
    }
    static u32 rxclk_rx_s_max_pulse_width(struct i2c_client *c, u32 ns,
    u16 *divider)
    {
    u64 pulse_clocks;
    if (ns > IR_MAX_DURATION)
    ns = IR_MAX_DURATION;
    pulse_clocks = ns_to_pulse_clocks(ns);
// divider = pulse_clocks_to_clock_divider(pulse_clocks);
    cx25840_write4(c, CX25840_IR_RXCLK_REG, *divider);
    return (u32) pulse_width_count_to_ns(FIFO_RXTX, *divider);
    }
//
// IR Tx Carrier Duty Cycle register helpers
//
    static unsigned int cduty_tx_s_duty_cycle(struct i2c_client *c,
    unsigned int duty_cycle)
    {
    u32 n;
    n = DIV_ROUND_CLOSEST(duty_cycle * 100, 625); /* 16ths of 100% */
    if (n != 0)
    n--;
    if (n > 15)
    n = 15;
    cx25840_write4(c, CX25840_IR_CDUTY_REG, n);
    return DIV_ROUND_CLOSEST((n + 1) * 100, 16);
    }
//
// IR Filter Register helpers
//
#[no_mangle]
unsafe extern "C" fn filter_rx_s_min_width(c: *mut i2c_client, min_width_ns: u32) -> u32 {
    static u32 filter_rx_s_min_width(struct i2c_client *c, u32 min_width_ns)
    {
    let mut count: u32 = ns_to_lpf_count(min_width_ns);
    cx25840_write4(c, CX25840_IR_FILTR_REG, count);
    return lpf_count_to_ns(count);
    }
//
// IR IRQ Enable Register helpers
//
#[no_mangle]
pub unsafe extern "C" fn irqenable_rx(sd: *mut v4l2_subdev, mask: u32) {
    static inline void irqenable_rx(struct v4l2_subdev *sd, u32 mask)
    {
    struct cx25840_state *state = to_state(sd);
    if (is_cx23885(state) || is_cx23887(state))
    mask ^= IRQEN_MSK;
    mask &= (IRQEN_RTE | IRQEN_ROE | IRQEN_RSE);
    cx25840_and_or4(state.c, CX25840_IR_IRQEN_REG,
    ~(IRQEN_RTE | IRQEN_ROE | IRQEN_RSE), mask);
    }
#[no_mangle]
pub unsafe extern "C" fn irqenable_tx(sd: *mut v4l2_subdev, mask: u32) {
    static inline void irqenable_tx(struct v4l2_subdev *sd, u32 mask)
    {
    struct cx25840_state *state = to_state(sd);
    if (is_cx23885(state) || is_cx23887(state))
    mask ^= IRQEN_MSK;
    mask &= IRQEN_TSE;
    cx25840_and_or4(state.c, CX25840_IR_IRQEN_REG, ~IRQEN_TSE, mask);
    }
//
// V4L2 Subdevice IR Ops
//
#[no_mangle]
pub unsafe extern "C" fn cx25840_ir_irq_handler(sd: *mut v4l2_subdev, status: u32, handled: *mut bool) -> c_int {
    int cx25840_ir_irq_handler(struct v4l2_subdev *sd, u32 status, bool *handled)
    {
    struct cx25840_state *state = to_state(sd);
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    struct i2c_client *c = core::ptr::null_mut();
    unsigned long flags;
    union cx25840_ir_fifo_rec rx_data[FIFO_RX_DEPTH];
    unsigned int i, j, k;
    u32 events, v;
    int tsr, rsr, rto, ror, tse, rse, rte, roe, kror;
    u32 cntrl, irqen, stats;
// handled = false;
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    c = ir_state.c;
// Only support the IR controller for the CX2388[57] AV Core for now
    if (!(is_cx23885(state) || is_cx23887(state)))
    return -ENODEV;
    cntrl = cx25840_read4(c, CX25840_IR_CNTRL_REG);
    irqen = cx25840_read4(c, CX25840_IR_IRQEN_REG);
    if (is_cx23885(state) || is_cx23887(state))
    irqen ^= IRQEN_MSK;
    stats = cx25840_read4(c, CX25840_IR_STATS_REG);
    tsr = stats & STATS_TSR; /* Tx FIFO Service Request */
    rsr = stats & STATS_RSR; /* Rx FIFO Service Request */
    rto = stats & STATS_RTO; /* Rx Pulse Width Timer Time Out */
    ror = stats & STATS_ROR; /* Rx FIFO Over Run */
    tse = irqen & IRQEN_TSE; /* Tx FIFO Service Request IRQ Enable */
    rse = irqen & IRQEN_RSE; /* Rx FIFO Service Request IRQ Enable */
    rte = irqen & IRQEN_RTE; /* Rx Pulse Width Timer Time Out IRQ Enable */
    roe = irqen & IRQEN_ROE; /* Rx FIFO Over Run IRQ Enable */
    v4l2_dbg(2, ir_debug, sd, "IR IRQ Status:  %s %s %s %s %s %s\n",
    tsr ? "tsr" : "   ", rsr ? "rsr" : "   ",
    rto ? "rto" : "   ", ror ? "ror" : "   ",
    stats & STATS_TBY ? "tby" : "   ",
    stats & STATS_RBY ? "rby" : "   ");
    v4l2_dbg(2, ir_debug, sd, "IR IRQ Enables: %s %s %s %s\n",
    tse ? "tse" : "   ", rse ? "rse" : "   ",
    rte ? "rte" : "   ", roe ? "roe" : "   ");
//
// Transmitter interrupt service
//
    if (tse && tsr) {
//
// TODO:
// Check the watermark threshold setting
// Pull FIFO_TX_DEPTH or FIFO_TX_DEPTH/2 entries from tx_kfifo
// Push the data to the hardware FIFO.
// If there was nothing more to send in the tx_kfifo, disable
// the TSR IRQ and notify the v4l2_device.
// If there was something in the tx_kfifo, check the tx_kfifo
// level and notify the v4l2_device, if it is low.
//
// For now, inhibit TSR interrupt until Tx is implemented
    irqenable_tx(sd, 0);
    events = V4L2_SUBDEV_IR_TX_FIFO_SERVICE_REQ;
    v4l2_subdev_notify(sd, V4L2_SUBDEV_IR_TX_NOTIFY, &events);
// handled = true;
    }
//
// Receiver interrupt service
//
    kror = 0;
    if ((rse && rsr) || (rte && rto)) {
//
// Receive data on RSR to clear the STATS_RSR.
// Receive data on RTO, since we may not have yet hit the RSR
// watermark when we receive the RTO.
//
    for (i = 0, v = FIFO_RX_NDV;
    (v & FIFO_RX_NDV) && !kror; i = 0) {
    for (j = 0;
    (v & FIFO_RX_NDV) && j < FIFO_RX_DEPTH; j++) {
    v = cx25840_read4(c, CX25840_IR_FIFO_REG);
    rx_data[i].hw_fifo_data = v & ~FIFO_RX_NDV;
    i++;
    }
    if (i == 0)
    break;
    j = i * sizeof(union cx25840_ir_fifo_rec);
    k = kfifo_in_locked(&ir_state.rx_kfifo,
    (unsigned char *) rx_data, j,
    &ir_state.rx_kfifo_lock);
    if (k != j)
    kror++; /* rx_kfifo over run */
    }
// handled = true;
    }
    events = 0;
    v = 0;
    if (kror) {
    events |= V4L2_SUBDEV_IR_RX_SW_FIFO_OVERRUN;
    v4l2_err(sd, "IR receiver software FIFO overrun\n");
    }
    if (roe && ror) {
//
// The RX FIFO Enable (CNTRL_RFE) must be toggled to clear
// the Rx FIFO Over Run status (STATS_ROR)
//
    v |= CNTRL_RFE;
    events |= V4L2_SUBDEV_IR_RX_HW_FIFO_OVERRUN;
    v4l2_err(sd, "IR receiver hardware FIFO overrun\n");
    }
    if (rte && rto) {
//
// The IR Receiver Enable (CNTRL_RXE) must be toggled to clear
// the Rx Pulse Width Timer Time Out (STATS_RTO)
//
    v |= CNTRL_RXE;
    events |= V4L2_SUBDEV_IR_RX_END_OF_RX_DETECTED;
    }
    if (v) {
// Clear STATS_ROR & STATS_RTO as needed by resetting hardware
    cx25840_write4(c, CX25840_IR_CNTRL_REG, cntrl & ~v);
    cx25840_write4(c, CX25840_IR_CNTRL_REG, cntrl);
// handled = true;
    }
    spin_lock_irqsave(&ir_state.rx_kfifo_lock, flags);
    if (kfifo_len(&ir_state.rx_kfifo) >= CX25840_IR_RX_KFIFO_SIZE / 2)
    events |= V4L2_SUBDEV_IR_RX_FIFO_SERVICE_REQ;
    spin_unlock_irqrestore(&ir_state.rx_kfifo_lock, flags);
    if (events)
    v4l2_subdev_notify(sd, V4L2_SUBDEV_IR_RX_NOTIFY, &events);
    return 0;
    }
// Receiver
    static int cx25840_ir_rx_read(struct v4l2_subdev *sd, u8 *buf, size_t count,
    ssize_t *num)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    bool invert;
    u16 divider;
    unsigned int i, n;
    union cx25840_ir_fifo_rec *p;
    unsigned u, v, w;
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    invert = (bool) atomic_read(&ir_state.rx_invert);
    divider = (u16) atomic_read(&ir_state.rxclk_divider);
    n = count / sizeof(union cx25840_ir_fifo_rec)
// sizeof(union cx25840_ir_fifo_rec);
    if (n == 0) {
// num = 0;
    return 0;
    }
    n = kfifo_out_locked(&ir_state.rx_kfifo, buf, n,
    &ir_state.rx_kfifo_lock);
    n /= sizeof(union cx25840_ir_fifo_rec);
// num = n * sizeof(union cx25840_ir_fifo_rec);
    for (p = (union cx25840_ir_fifo_rec *) buf, i = 0; i < n; p++, i++) {
    if ((p.hw_fifo_data & FIFO_RXTX_RTO) == FIFO_RXTX_RTO) {
// Assume RTO was because of no IR light input
    u = 0;
    w = 1;
    } else {
    u = (p.hw_fifo_data & FIFO_RXTX_LVL) ? 1 : 0;
    if (invert)
    u = u ? 0 : 1;
    w = 0;
    }
    v = (unsigned) pulse_width_count_to_ns(
    (u16)(p.hw_fifo_data & FIFO_RXTX), divider) / 1000;
    if (v > IR_MAX_DURATION)
    v = IR_MAX_DURATION;
    p.ir_core_data = (struct ir_raw_event)
    { .pulse = u, .duration = v, .timeout = w };
    v4l2_dbg(2, ir_debug, sd, "rx read: %10u ns  %s  %s\n",
    v, u ? "mark" : "space", w ? "(timed out)" : "");
    if (w)
    v4l2_dbg(2, ir_debug, sd, "rx read: end of rx\n");
    }
    return 0;
    }
    static int cx25840_ir_rx_g_parameters(struct v4l2_subdev *sd,
    struct v4l2_subdev_ir_parameters *p)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    mutex_lock(&ir_state.rx_params_lock);
    memcpy(p, &ir_state.rx_params,
    sizeof(struct v4l2_subdev_ir_parameters));
    mutex_unlock(&ir_state.rx_params_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cx25840_ir_rx_shutdown(sd: *mut v4l2_subdev) -> c_int {
    static int cx25840_ir_rx_shutdown(struct v4l2_subdev *sd)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    struct i2c_client *c;
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    c = ir_state.c;
    mutex_lock(&ir_state.rx_params_lock);
// Disable or slow down all IR Rx circuits and counters
    irqenable_rx(sd, 0);
    control_rx_enable(c, false);
    control_rx_demodulation_enable(c, false);
    control_rx_s_edge_detection(c, CNTRL_EDG_NONE);
    filter_rx_s_min_width(c, 0);
    cx25840_write4(c, CX25840_IR_RXCLK_REG, RXCLK_RCD);
    ir_state.rx_params.shutdown = true;
    mutex_unlock(&ir_state.rx_params_lock);
    return 0;
    }
    static int cx25840_ir_rx_s_parameters(struct v4l2_subdev *sd,
    struct v4l2_subdev_ir_parameters *p)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    struct i2c_client *c;
    struct v4l2_subdev_ir_parameters *o;
    u16 rxclk_divider;
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    if (p.shutdown)
    return cx25840_ir_rx_shutdown(sd);
    if (p.mode != V4L2_SUBDEV_IR_MODE_PULSE_WIDTH)
    return -ENOSYS;
    c = ir_state.c;
    o = &ir_state.rx_params;
    mutex_lock(&ir_state.rx_params_lock);
    o.shutdown = p.shutdown;
    p.mode = V4L2_SUBDEV_IR_MODE_PULSE_WIDTH;
    o.mode = p.mode;
    p.bytes_per_data_element = sizeof(union cx25840_ir_fifo_rec);
    o.bytes_per_data_element = p.bytes_per_data_element;
// Before we tweak the hardware, we have to disable the receiver
    irqenable_rx(sd, 0);
    control_rx_enable(c, false);
    control_rx_demodulation_enable(c, p.modulation);
    o.modulation = p.modulation;
    if (p.modulation) {
    p.carrier_freq = rxclk_rx_s_carrier(c, p.carrier_freq,
    &rxclk_divider);
    o.carrier_freq = p.carrier_freq;
    p.duty_cycle = 50;
    o.duty_cycle = p.duty_cycle;
    control_rx_s_carrier_window(c, p.carrier_freq,
    &p.carrier_range_lower,
    &p.carrier_range_upper);
    o.carrier_range_lower = p.carrier_range_lower;
    o.carrier_range_upper = p.carrier_range_upper;
    p.max_pulse_width =
    (u32) pulse_width_count_to_ns(FIFO_RXTX, rxclk_divider);
    } else {
    p.max_pulse_width =
    rxclk_rx_s_max_pulse_width(c, p.max_pulse_width,
    &rxclk_divider);
    }
    o.max_pulse_width = p.max_pulse_width;
    atomic_set(&ir_state.rxclk_divider, rxclk_divider);
    p.noise_filter_min_width =
    filter_rx_s_min_width(c, p.noise_filter_min_width);
    o.noise_filter_min_width = p.noise_filter_min_width;
    p.resolution = clock_divider_to_resolution(rxclk_divider);
    o.resolution = p.resolution;
// FIXME - make this dependent on resolution for better performance
    control_rx_irq_watermark(c, RX_FIFO_HALF_FULL);
    control_rx_s_edge_detection(c, CNTRL_EDG_BOTH);
    o.invert_level = p.invert_level;
    atomic_set(&ir_state.rx_invert, p.invert_level);
    o.interrupt_enable = p.interrupt_enable;
    o.enable = p.enable;
    if (p.enable) {
    unsigned long flags;
    spin_lock_irqsave(&ir_state.rx_kfifo_lock, flags);
    kfifo_reset(&ir_state.rx_kfifo);
    spin_unlock_irqrestore(&ir_state.rx_kfifo_lock, flags);
    if (p.interrupt_enable)
    irqenable_rx(sd, IRQEN_RSE | IRQEN_RTE | IRQEN_ROE);
    control_rx_enable(c, p.enable);
    }
    mutex_unlock(&ir_state.rx_params_lock);
    return 0;
    }
// Transmitter
    static int cx25840_ir_tx_write(struct v4l2_subdev *sd, u8 *buf, size_t count,
    ssize_t *num)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;

//
// FIXME - the code below is an incomplete and untested sketch of what
// may need to be done.  The critical part is to get 4 (or 8) pulses
// from the tx_kfifo, or converted from ns to the proper units from the
// input, and push them off to the hardware Tx FIFO right away, if the
// HW TX fifo needs service.  The rest can be pushed to the tx_kfifo in
// a less critical timeframe.  Also watch out for overruning the
// tx_kfifo - don't let it happen and let the caller know not all his
// pulses were written.
//
    u32 *ns_pulse = (u32 *) buf;
    unsigned int n;
    u32 fifo_pulse[FIFO_TX_DEPTH];
    u32 mark;
// Compute how much we can fit in the tx kfifo
    n = CX25840_IR_TX_KFIFO_SIZE - kfifo_len(ir_state.tx_kfifo);
    n = min(n, (unsigned int) count);
    n /= sizeof(u32);
// FIXME - turn on Tx Fifo service interrupt
// check hardware fifo level, and other stuff
//
    for (i = 0; i < n; ) {
    for (j = 0; j < FIFO_TX_DEPTH / 2 && i < n; j++) {
    mark = ns_pulse[i] & LEVEL_MASK;
    fifo_pulse[j] = ns_to_pulse_width_count(
    ns_pulse[i] &
    ~LEVEL_MASK,
    ir_state.txclk_divider);
    if (mark)
    fifo_pulse[j] &= FIFO_RXTX_LVL;
    i++;
    }
    kfifo_put(ir_state.tx_kfifo, (u8 *) fifo_pulse,
    j * sizeof(u32));
    }
// num = n * sizeof(u32);

// For now enable the Tx FIFO Service interrupt & pretend we did work
    irqenable_tx(sd, IRQEN_TSE);
// num = count;

    return 0;
    }
    static int cx25840_ir_tx_g_parameters(struct v4l2_subdev *sd,
    struct v4l2_subdev_ir_parameters *p)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    mutex_lock(&ir_state.tx_params_lock);
    memcpy(p, &ir_state.tx_params,
    sizeof(struct v4l2_subdev_ir_parameters));
    mutex_unlock(&ir_state.tx_params_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cx25840_ir_tx_shutdown(sd: *mut v4l2_subdev) -> c_int {
    static int cx25840_ir_tx_shutdown(struct v4l2_subdev *sd)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    struct i2c_client *c;
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    c = ir_state.c;
    mutex_lock(&ir_state.tx_params_lock);
// Disable or slow down all IR Tx circuits and counters
    irqenable_tx(sd, 0);
    control_tx_enable(c, false);
    control_tx_modulation_enable(c, false);
    cx25840_write4(c, CX25840_IR_TXCLK_REG, TXCLK_TCD);
    ir_state.tx_params.shutdown = true;
    mutex_unlock(&ir_state.tx_params_lock);
    return 0;
    }
    static int cx25840_ir_tx_s_parameters(struct v4l2_subdev *sd,
    struct v4l2_subdev_ir_parameters *p)
    {
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    struct i2c_client *c;
    struct v4l2_subdev_ir_parameters *o;
    u16 txclk_divider;
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    if (p.shutdown)
    return cx25840_ir_tx_shutdown(sd);
    if (p.mode != V4L2_SUBDEV_IR_MODE_PULSE_WIDTH)
    return -ENOSYS;
    c = ir_state.c;
    o = &ir_state.tx_params;
    mutex_lock(&ir_state.tx_params_lock);
    o.shutdown = p.shutdown;
    p.mode = V4L2_SUBDEV_IR_MODE_PULSE_WIDTH;
    o.mode = p.mode;
    p.bytes_per_data_element = sizeof(union cx25840_ir_fifo_rec);
    o.bytes_per_data_element = p.bytes_per_data_element;
// Before we tweak the hardware, we have to disable the transmitter
    irqenable_tx(sd, 0);
    control_tx_enable(c, false);
    control_tx_modulation_enable(c, p.modulation);
    o.modulation = p.modulation;
    if (p.modulation) {
    p.carrier_freq = txclk_tx_s_carrier(c, p.carrier_freq,
    &txclk_divider);
    o.carrier_freq = p.carrier_freq;
    p.duty_cycle = cduty_tx_s_duty_cycle(c, p.duty_cycle);
    o.duty_cycle = p.duty_cycle;
    p.max_pulse_width =
    (u32) pulse_width_count_to_ns(FIFO_RXTX, txclk_divider);
    } else {
    p.max_pulse_width =
    txclk_tx_s_max_pulse_width(c, p.max_pulse_width,
    &txclk_divider);
    }
    o.max_pulse_width = p.max_pulse_width;
    atomic_set(&ir_state.txclk_divider, txclk_divider);
    p.resolution = clock_divider_to_resolution(txclk_divider);
    o.resolution = p.resolution;
// FIXME - make this dependent on resolution for better performance
    control_tx_irq_watermark(c, TX_FIFO_HALF_EMPTY);
    control_tx_polarity_invert(c, p.invert_carrier_sense);
    o.invert_carrier_sense = p.invert_carrier_sense;
//
// FIXME: we don't have hardware help for IO pin level inversion
// here like we have on the CX23888.
// Act on this with some mix of logical inversion of data levels,
// carrier polarity, and carrier duty cycle.
//
    o.invert_level = p.invert_level;
    o.interrupt_enable = p.interrupt_enable;
    o.enable = p.enable;
    if (p.enable) {
// reset tx_fifo here
    if (p.interrupt_enable)
    irqenable_tx(sd, IRQEN_TSE);
    control_tx_enable(c, p.enable);
    }
    mutex_unlock(&ir_state.tx_params_lock);
    return 0;
    }
//
// V4L2 Subdevice Core Ops support
//
#[no_mangle]
pub unsafe extern "C" fn cx25840_ir_log_status(sd: *mut v4l2_subdev) -> c_int {
    int cx25840_ir_log_status(struct v4l2_subdev *sd)
    {
    struct cx25840_state *state = to_state(sd);
    struct i2c_client *c = state.c;
    char *s;
    int i, j;
    u32 cntrl, txclk, rxclk, cduty, stats, irqen, filtr;
// The CX23888 chip doesn't have an IR controller on the A/V core
    if (is_cx23888(state))
    return 0;
    cntrl = cx25840_read4(c, CX25840_IR_CNTRL_REG);
    txclk = cx25840_read4(c, CX25840_IR_TXCLK_REG) & TXCLK_TCD;
    rxclk = cx25840_read4(c, CX25840_IR_RXCLK_REG) & RXCLK_RCD;
    cduty = cx25840_read4(c, CX25840_IR_CDUTY_REG) & CDUTY_CDC;
    stats = cx25840_read4(c, CX25840_IR_STATS_REG);
    irqen = cx25840_read4(c, CX25840_IR_IRQEN_REG);
    if (is_cx23885(state) || is_cx23887(state))
    irqen ^= IRQEN_MSK;
    filtr = cx25840_read4(c, CX25840_IR_FILTR_REG) & FILTR_LPF;
    v4l2_info(sd, "IR Receiver:\n");
    v4l2_info(sd, "\tEnabled:                           %s\n",
    cntrl & CNTRL_RXE ? "yes" : "no");
    v4l2_info(sd, "\tDemodulation from a carrier:       %s\n",
    cntrl & CNTRL_DMD ? "enabled" : "disabled");
    v4l2_info(sd, "\tFIFO:                              %s\n",
    cntrl & CNTRL_RFE ? "enabled" : "disabled");
    switch (cntrl & CNTRL_EDG) {
    case CNTRL_EDG_NONE:
    s = "disabled";
    break;
    case CNTRL_EDG_FALL:
    s = "falling edge";
    break;
    case CNTRL_EDG_RISE:
    s = "rising edge";
    break;
    case CNTRL_EDG_BOTH:
    s = "rising & falling edges";
    break;
    default:
    s = "??? edge";
    break;
    }
    v4l2_info(sd, "\tPulse timers' start/stop trigger:  %s\n", s);
    v4l2_info(sd, "\tFIFO data on pulse timer overflow: %s\n",
    cntrl & CNTRL_R ? "not loaded" : "overflow marker");
    v4l2_info(sd, "\tFIFO interrupt watermark:          %s\n",
    cntrl & CNTRL_RIC ? "not empty" : "half full or greater");
    v4l2_info(sd, "\tLoopback mode:                     %s\n",
    cntrl & CNTRL_LBM ? "loopback active" : "normal receive");
    if (cntrl & CNTRL_DMD) {
    v4l2_info(sd, "\tExpected carrier (16 clocks):      %u Hz\n",
    clock_divider_to_carrier_freq(rxclk));
    switch (cntrl & CNTRL_WIN) {
    case CNTRL_WIN_3_3:
    i = 3;
    j = 3;
    break;
    case CNTRL_WIN_4_3:
    i = 4;
    j = 3;
    break;
    case CNTRL_WIN_3_4:
    i = 3;
    j = 4;
    break;
    case CNTRL_WIN_4_4:
    i = 4;
    j = 4;
    break;
    default:
    i = 0;
    j = 0;
    break;
    }
    v4l2_info(sd, "\tNext carrier edge window:	    16 clocks -%1d/+%1d, %u to %u Hz\n",
    i, j,
    clock_divider_to_freq(rxclk, 16 + j),
    clock_divider_to_freq(rxclk, 16 - i));
    }
    v4l2_info(sd, "\tMax measurable pulse width:        %u us, %llu ns\n",
    pulse_width_count_to_us(FIFO_RXTX, rxclk),
    pulse_width_count_to_ns(FIFO_RXTX, rxclk));
    v4l2_info(sd, "\tLow pass filter:                   %s\n",
    filtr ? "enabled" : "disabled");
    if (filtr)
    v4l2_info(sd, "\tMin acceptable pulse width (LPF):  %u us, %u ns\n",
    lpf_count_to_us(filtr),
    lpf_count_to_ns(filtr));
    v4l2_info(sd, "\tPulse width timer timed-out:       %s\n",
    stats & STATS_RTO ? "yes" : "no");
    v4l2_info(sd, "\tPulse width timer time-out intr:   %s\n",
    irqen & IRQEN_RTE ? "enabled" : "disabled");
    v4l2_info(sd, "\tFIFO overrun:                      %s\n",
    stats & STATS_ROR ? "yes" : "no");
    v4l2_info(sd, "\tFIFO overrun interrupt:            %s\n",
    irqen & IRQEN_ROE ? "enabled" : "disabled");
    v4l2_info(sd, "\tBusy:                              %s\n",
    stats & STATS_RBY ? "yes" : "no");
    v4l2_info(sd, "\tFIFO service requested:            %s\n",
    stats & STATS_RSR ? "yes" : "no");
    v4l2_info(sd, "\tFIFO service request interrupt:    %s\n",
    irqen & IRQEN_RSE ? "enabled" : "disabled");
    v4l2_info(sd, "IR Transmitter:\n");
    v4l2_info(sd, "\tEnabled:                           %s\n",
    cntrl & CNTRL_TXE ? "yes" : "no");
    v4l2_info(sd, "\tModulation onto a carrier:         %s\n",
    cntrl & CNTRL_MOD ? "enabled" : "disabled");
    v4l2_info(sd, "\tFIFO:                              %s\n",
    cntrl & CNTRL_TFE ? "enabled" : "disabled");
    v4l2_info(sd, "\tFIFO interrupt watermark:          %s\n",
    cntrl & CNTRL_TIC ? "not empty" : "half full or less");
    v4l2_info(sd, "\tCarrier polarity:                  %s\n",
    cntrl & CNTRL_CPL ? "space:burst mark:noburst"
    : "space:noburst mark:burst");
    if (cntrl & CNTRL_MOD) {
    v4l2_info(sd, "\tCarrier (16 clocks):               %u Hz\n",
    clock_divider_to_carrier_freq(txclk));
    v4l2_info(sd, "\tCarrier duty cycle:                %2u/16\n",
    cduty + 1);
    }
    v4l2_info(sd, "\tMax pulse width:                   %u us, %llu ns\n",
    pulse_width_count_to_us(FIFO_RXTX, txclk),
    pulse_width_count_to_ns(FIFO_RXTX, txclk));
    v4l2_info(sd, "\tBusy:                              %s\n",
    stats & STATS_TBY ? "yes" : "no");
    v4l2_info(sd, "\tFIFO service requested:            %s\n",
    stats & STATS_TSR ? "yes" : "no");
    v4l2_info(sd, "\tFIFO service request interrupt:    %s\n",
    irqen & IRQEN_TSE ? "enabled" : "disabled");
    return 0;
    }
    const struct v4l2_subdev_ir_ops cx25840_ir_ops = {
    .rx_read = cx25840_ir_rx_read,
    .rx_g_parameters = cx25840_ir_rx_g_parameters,
    .rx_s_parameters = cx25840_ir_rx_s_parameters,
    .tx_write = cx25840_ir_tx_write,
    .tx_g_parameters = cx25840_ir_tx_g_parameters,
    .tx_s_parameters = cx25840_ir_tx_s_parameters,
    };
    static const struct v4l2_subdev_ir_parameters default_rx_params = {
    .bytes_per_data_element = sizeof(union cx25840_ir_fifo_rec),
    .mode = V4L2_SUBDEV_IR_MODE_PULSE_WIDTH,
    .enable = false,
    .interrupt_enable = false,
    .shutdown = true,
    .modulation = true,
    .carrier_freq = 36000, /* 36 kHz - RC-5, and RC-6 carrier */
// RC-5: 666,667 ns = 1/36 kHz * 32 cycles * 1 mark * 0.75
// RC-6: 333,333 ns = 1/36 kHz * 16 cycles * 1 mark * 0.75
    .noise_filter_min_width = 333333, /* ns */
    .carrier_range_lower = 35000,
    .carrier_range_upper = 37000,
    .invert_level = false,
    };
    static const struct v4l2_subdev_ir_parameters default_tx_params = {
    .bytes_per_data_element = sizeof(union cx25840_ir_fifo_rec),
    .mode = V4L2_SUBDEV_IR_MODE_PULSE_WIDTH,
    .enable = false,
    .interrupt_enable = false,
    .shutdown = true,
    .modulation = true,
    .carrier_freq = 36000, /* 36 kHz - RC-5 carrier */
    .duty_cycle = 25,      /* 25 %   - RC-5 carrier */
    .invert_level = false,
    .invert_carrier_sense = false,
    };
#[no_mangle]
pub unsafe extern "C" fn cx25840_ir_probe(sd: *mut v4l2_subdev) -> c_int {
    int cx25840_ir_probe(struct v4l2_subdev *sd)
    {
    struct cx25840_state *state = to_state(sd);
    struct cx25840_ir_state *ir_state;
    struct v4l2_subdev_ir_parameters default_params;
// Only init the IR controller for the CX2388[57] AV Core for now
    if (!(is_cx23885(state) || is_cx23887(state)))
    return 0;
    ir_state = devm_kzalloc(&state.c.dev, sizeof(*ir_state), GFP_KERNEL);
    if (ir_state == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&ir_state.rx_kfifo_lock);
    if (kfifo_alloc(&ir_state.rx_kfifo,
    CX25840_IR_RX_KFIFO_SIZE, GFP_KERNEL))
    return -ENOMEM;
    ir_state.c = state.c;
    state.ir_state = ir_state;
// Ensure no interrupts arrive yet
    if (is_cx23885(state) || is_cx23887(state))
    cx25840_write4(ir_state.c, CX25840_IR_IRQEN_REG, IRQEN_MSK);
    else
    cx25840_write4(ir_state.c, CX25840_IR_IRQEN_REG, 0);
    mutex_init(&ir_state.rx_params_lock);
    default_params = default_rx_params;
    v4l2_subdev_call(sd, ir, rx_s_parameters, &default_params);
    mutex_init(&ir_state.tx_params_lock);
    default_params = default_tx_params;
    v4l2_subdev_call(sd, ir, tx_s_parameters, &default_params);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cx25840_ir_remove(sd: *mut v4l2_subdev) -> c_int {
    int cx25840_ir_remove(struct v4l2_subdev *sd)
    {
    struct cx25840_state *state = to_state(sd);
    struct cx25840_ir_state *ir_state = to_ir_state(sd);
    if (ir_state == core::ptr::null_mut())
    return -ENODEV;
    cx25840_ir_rx_shutdown(sd);
    cx25840_ir_tx_shutdown(sd);
    kfifo_free(&ir_state.rx_kfifo);
    state.ir_state = core::ptr::null_mut();
    return 0;
    }
