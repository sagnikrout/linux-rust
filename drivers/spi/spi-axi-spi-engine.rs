//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-axi-spi-engine.c
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
// SPI-Engine SPI controller driver
// Copyright 2015 Analog Devices Inc.
// Copyright 2024 BayLibre, SAS
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

pub const SPI_ENGINE_REG_DATA_WIDTH: c_uint = 0x0C;

pub const SPI_ENGINE_REG_OFFLOAD_MEM_ADDR_WIDTH: c_uint = 0x10;
pub const SPI_ENGINE_REG_RESET: c_uint = 0x40;
pub const SPI_ENGINE_REG_INT_ENABLE: c_uint = 0x80;
pub const SPI_ENGINE_REG_INT_PENDING: c_uint = 0x84;
pub const SPI_ENGINE_REG_INT_SOURCE: c_uint = 0x88;
pub const SPI_ENGINE_REG_SYNC_ID: c_uint = 0xc0;
pub const SPI_ENGINE_REG_OFFLOAD_SYNC_ID: c_uint = 0xc4;
pub const SPI_ENGINE_REG_CMD_FIFO_ROOM: c_uint = 0xd0;
pub const SPI_ENGINE_REG_SDO_FIFO_ROOM: c_uint = 0xd4;
pub const SPI_ENGINE_REG_SDI_FIFO_LEVEL: c_uint = 0xd8;
pub const SPI_ENGINE_REG_CMD_FIFO: c_uint = 0xe0;
pub const SPI_ENGINE_REG_SDO_DATA_FIFO: c_uint = 0xe4;
pub const SPI_ENGINE_REG_SDI_DATA_FIFO: c_uint = 0xe8;
pub const SPI_ENGINE_REG_SDI_DATA_FIFO_PEEK: c_uint = 0xec;
pub const SPI_ENGINE_MAX_NUM_OFFLOADS: c_int = 32;

pub const SPI_ENGINE_INST_TRANSFER: c_uint = 0x0;
pub const SPI_ENGINE_INST_ASSERT: c_uint = 0x1;
pub const SPI_ENGINE_INST_WRITE: c_uint = 0x2;
pub const SPI_ENGINE_INST_MISC: c_uint = 0x3;
pub const SPI_ENGINE_INST_CS_INV: c_uint = 0x4;
pub const SPI_ENGINE_CMD_REG_CLK_DIV: c_uint = 0x0;
pub const SPI_ENGINE_CMD_REG_CONFIG: c_uint = 0x1;
pub const SPI_ENGINE_CMD_REG_XFER_BITS: c_uint = 0x2;
pub const SPI_ENGINE_CMD_REG_SDI_MASK: c_uint = 0x3;
pub const SPI_ENGINE_CMD_REG_SDO_MASK: c_uint = 0x4;
pub const SPI_ENGINE_MISC_SYNC: c_uint = 0x0;
pub const SPI_ENGINE_MISC_SLEEP: c_uint = 0x1;
pub const SPI_ENGINE_TRANSFER_WRITE: c_uint = 0x1;
pub const SPI_ENGINE_TRANSFER_READ: c_uint = 0x2;
// Arbitrary sync ID for use by host->cur_msg
pub const AXI_SPI_ENGINE_CUR_MSG_SYNC_ID: c_uint = 0x1;

    (((inst) << 12) | ((arg1) << 8) | (arg2))

    SPI_ENGINE_CMD(SPI_ENGINE_INST_TRANSFER, (flags), (n))

    SPI_ENGINE_CMD(SPI_ENGINE_INST_ASSERT, (delay), (cs))

    SPI_ENGINE_CMD(SPI_ENGINE_INST_WRITE, (reg), (val))

    SPI_ENGINE_CMD(SPI_ENGINE_INST_MISC, SPI_ENGINE_MISC_SLEEP, (delay))

    SPI_ENGINE_CMD(SPI_ENGINE_INST_MISC, SPI_ENGINE_MISC_SYNC, (id))

    SPI_ENGINE_CMD(SPI_ENGINE_INST_CS_INV, 0, (flags))
// default sizes - can be changed when SPI Engine firmware is compiled
pub const SPI_ENGINE_OFFLOAD_CMD_FIFO_SIZE: c_int = 16;
pub const SPI_ENGINE_OFFLOAD_SDO_FIFO_SIZE: c_int = 16;
// Extending SPI_MULTI_LANE_MODE values for optimizing messages.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_engine_program {
    pub length: c_uint,
    pub __counted_by(length): uint16_t instructions[],
}

//
// struct spi_engine_message_state - SPI engine per-message state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_engine_message_state {
// @cmd_length: Number of elements in cmd_buf array.
    pub cmd_length: unsigned,
// @cmd_buf: Array of commands not yet written to CMD FIFO.
    pub cmd_buf: *const u16,
// @tx_xfer: Next xfer with tx_buf not yet fully written to TX FIFO.
    pub tx_xfer: *mut spi_transfer,
// @tx_length: Size of tx_buf in bytes.
    pub tx_length: c_uint,
// @tx_buf: Bytes not yet written to TX FIFO.
    pub tx_buf: *const u8,
// @rx_xfer: Next xfer with rx_buf not yet fully written to RX FIFO.
    pub rx_xfer: *mut spi_transfer,
// @rx_length: Size of tx_buf in bytes.
    pub rx_length: c_uint,
// @rx_buf: Bytes not yet written to the RX FIFO.
    pub rx_buf: *mut u8,
}

    enum {
    SPI_ENGINE_OFFLOAD_FLAG_ASSIGNED,
    SPI_ENGINE_OFFLOAD_FLAG_PREPARED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_engine_offload {
    pub spi_engine: *mut spi_engine,
    pub flags: c_ulong,
    pub offload_num: c_uint,
    pub spi_mode_config: c_uint,
    pub multi_lane_mode: c_uint,
    pub rx_primary_lane_mask: u8,
    pub tx_primary_lane_mask: u8,
    pub rx_all_lanes_mask: u8,
    pub tx_all_lanes_mask: u8,
    pub bits_per_word: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_engine {
    pub clk: *mut clk,
    pub ref_clk: *mut clk,
    pub lock: spinlock_t,
    pub base: *mut void __iomem,
    pub msg_state: spi_engine_message_state,
    pub msg_complete: completion,
    pub int_enable: c_uint,
// shadows hardware CS inversion flag state
    pub cs_inv: u8,
    pub offload_ctrl_mem_size: c_uint,
    pub offload_sdo_mem_size: c_uint,
    pub offload: *mut spi_offload,
    pub offload_caps: u32,
    pub offload_requires_sync: bool,
}

    static void spi_engine_primary_lane_flag(struct spi_device *spi,
    u8 *rx_lane_flags, u8 *tx_lane_flags)
    {
// rx_lane_flags = BIT(spi->rx_lane_map[0]);
// tx_lane_flags = BIT(spi->tx_lane_map[0]);
    }
    static void spi_engine_all_lanes_flags(struct spi_device *spi,
    u8 *rx_lane_flags, u8 *tx_lane_flags)
    {
    int i;
    for (i = 0; i < spi.num_rx_lanes; i++)
// rx_lane_flags |= BIT(spi->rx_lane_map[i]);
    for (i = 0; i < spi.num_tx_lanes; i++)
// tx_lane_flags |= BIT(spi->tx_lane_map[i]);
    }
    static void spi_engine_program_add_cmd(struct spi_engine_program *p,
    bool dry, uint16_t cmd)
    {
    p.length++;
    if (!dry)
    p.instructions[p.length - 1] = cmd;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_get_config(spi: *mut spi_device) -> c_uint {
    static unsigned int spi_engine_get_config(struct spi_device *spi)
    {
    let mut config: c_uint = 0;
    if (spi.mode & SPI_CPOL)
    config |= SPI_ENGINE_CONFIG_CPOL;
    if (spi.mode & SPI_CPHA)
    config |= SPI_ENGINE_CONFIG_CPHA;
    if (spi.mode & SPI_3WIRE)
    config |= SPI_ENGINE_CONFIG_3WIRE;
    if (spi.mode & SPI_MOSI_IDLE_HIGH)
    config |= SPI_ENGINE_CONFIG_SDO_IDLE_HIGH;
    if (spi.mode & SPI_MOSI_IDLE_LOW)
    config &= ~SPI_ENGINE_CONFIG_SDO_IDLE_HIGH;
    return config;
    }
    static void spi_engine_gen_xfer(struct spi_engine_program *p, bool dry,
    struct spi_transfer *xfer, u32 num_lanes)
    {
    unsigned int len;
    if (xfer.bits_per_word <= 8)
    len = xfer.len;
#[no_mangle]
pub unsafe extern "C" fn if(16: xfer->bits_per_word <=) -> else {
    else if (xfer.bits_per_word <= 16)
    len = xfer.len / 2;
    else
    len = xfer.len / 4;
    if (xfer.multi_lane_mode == SPI_MULTI_LANE_MODE_STRIPE)
    len /= num_lanes;
    while (len) {
    let mut n: c_uint = min(len, 256U);
    let mut flags: c_uint = 0;
    if (xfer.tx_buf || (xfer.offload_flags & SPI_OFFLOAD_XFER_TX_STREAM))
    flags |= SPI_ENGINE_TRANSFER_WRITE;
    if (xfer.rx_buf || (xfer.offload_flags & SPI_OFFLOAD_XFER_RX_STREAM))
    flags |= SPI_ENGINE_TRANSFER_READ;
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_TRANSFER(flags, n - 1));
    len -= n;
    }
    }
    static void spi_engine_gen_sleep(struct spi_engine_program *p, bool dry,
    int delay_ns, int inst_ns, u32 sclk_hz)
    {
    unsigned int t;
//
// Negative delay indicates error, e.g. from spi_delay_to_ns(). And if
// delay is less that the instruction execution time, there is no need
// for an extra sleep instruction since the instruction execution time
// will already cover the required delay.
//
    if (delay_ns < 0 || delay_ns <= inst_ns)
    return;
    t = DIV_ROUND_UP_ULL((u64)(delay_ns - inst_ns) * sclk_hz, NSEC_PER_SEC);
    while (t) {
    let mut n: c_uint = min(t, 256U);
    spi_engine_program_add_cmd(p, dry, SPI_ENGINE_CMD_SLEEP(n - 1));
    t -= n;
    }
    }
    static void spi_engine_gen_cs(struct spi_engine_program *p, bool dry,
    struct spi_device *spi, bool assert)
    {
    let mut mask: c_uint = 0xff;
    if (assert)
    mask ^= BIT(spi_get_chipselect(spi, 0));
    spi_engine_program_add_cmd(p, dry, SPI_ENGINE_CMD_ASSERT(0, mask));
    }
//
// Performs precompile steps on the message.
//
// The SPI core does most of the message/transfer validation and filling in
// fields for us via __spi_validate(). This fixes up anything remaining not
// done there.
//
// NB: This is separate from spi_engine_compile_message() because the latter
// is called twice and would otherwise result in double-evaluation.
//
// Returns 0 on success, -EINVAL on failure.
//
#[no_mangle]
unsafe extern "C" fn spi_engine_precompile_message(msg: *mut spi_message) -> c_int {
    static int spi_engine_precompile_message(struct spi_message *msg)
    {
    unsigned int clk_div, max_hz = msg.spi.controller.max_speed_hz;
    struct spi_transfer *xfer;
    let mut multi_lane_mode: c_int = SPI_ENGINE_MULTI_BUS_MODE_UNKNOWN;
    let mut min_bits_per_word: u8 = U8_MAX;
    let mut max_bits_per_word: u8 = 0;
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
// If we have an offload transfer, we can't rx to buffer
    if (msg.offload && xfer.rx_buf)
    return -EINVAL;
    clk_div = DIV_ROUND_UP(max_hz, xfer.speed_hz);
    xfer.effective_speed_hz = max_hz / min(clk_div, 256U);
    if (xfer.len) {
    min_bits_per_word = min(min_bits_per_word, xfer.bits_per_word);
    max_bits_per_word = max(max_bits_per_word, xfer.bits_per_word);
    }
    if (xfer.rx_buf || xfer.offload_flags & SPI_OFFLOAD_XFER_RX_STREAM ||
    xfer.tx_buf || xfer.offload_flags & SPI_OFFLOAD_XFER_TX_STREAM) {
    switch (xfer.multi_lane_mode) {
    case SPI_MULTI_LANE_MODE_SINGLE:
    case SPI_MULTI_LANE_MODE_STRIPE:
    break;
    default:
// Other modes, like mirror not supported
    return -EINVAL;
    }
// If all xfers have the same multi-lane mode, we can optimize.
    if (multi_lane_mode == SPI_ENGINE_MULTI_BUS_MODE_UNKNOWN)
    multi_lane_mode = xfer.multi_lane_mode;
#[no_mangle]
pub unsafe extern "C" fn if(xfer->multi_lane_mode: multi_lane_mode !=) -> else {
    else if (multi_lane_mode != xfer.multi_lane_mode)
    multi_lane_mode = SPI_ENGINE_MULTI_BUS_MODE_CONFLICTING;
    }
    }
//
// If all xfers in the message use the same bits_per_word, we can
// provide some optimization when using SPI offload.
//
    if (msg.offload) {
    struct spi_engine_offload *priv = msg.offload.priv;
    if (min_bits_per_word == max_bits_per_word)
    priv.bits_per_word = min_bits_per_word;
    else
    priv.bits_per_word = 0;
    priv.multi_lane_mode = multi_lane_mode;
    spi_engine_primary_lane_flag(msg.spi,
    &priv.rx_primary_lane_mask,
    &priv.tx_primary_lane_mask);
    spi_engine_all_lanes_flags(msg.spi,
    &priv.rx_all_lanes_mask,
    &priv.tx_all_lanes_mask);
    }
    return 0;
    }
    static void spi_engine_compile_message(struct spi_message *msg, bool dry,
    struct spi_engine_program *p)
    {
    struct spi_device *spi = msg.spi;
    struct spi_controller *host = spi.controller;
    struct spi_engine_offload *priv;
    struct spi_transfer *xfer;
    int clk_div, new_clk_div, inst_ns;
    let mut prev_multi_lane_mode: c_int = SPI_MULTI_LANE_MODE_SINGLE;
    let mut keep_cs: bool = false;
    let mut bits_per_word: u8 = 0;
//
// Take into account instruction execution time for more accurate sleep
// times, especially when the delay is small.
//
    inst_ns = DIV_ROUND_UP(NSEC_PER_SEC, host.max_speed_hz);
    clk_div = 1;
//
// As an optimization, SPI offload sets once this when the offload is
// enabled instead of repeating the instruction in each message.
//
    if (msg.offload) {
    priv = msg.offload.priv;
    priv.spi_mode_config = spi_engine_get_config(spi);
//
// If all xfers use the same bits_per_word, it can be optimized
// in the same way.
//
    bits_per_word = priv.bits_per_word;
    prev_multi_lane_mode = priv.multi_lane_mode;
    } else {
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_CONFIG,
    spi_engine_get_config(spi)));
    }
    xfer = list_first_entry(&msg.transfers, struct spi_transfer, transfer_list);
    spi_engine_gen_cs(p, dry, spi, !xfer.cs_off);
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
    if (xfer.rx_buf || xfer.offload_flags & SPI_OFFLOAD_XFER_RX_STREAM ||
    xfer.tx_buf || xfer.offload_flags & SPI_OFFLOAD_XFER_TX_STREAM) {
    if (xfer.multi_lane_mode != prev_multi_lane_mode) {
    u8 tx_lane_flags, rx_lane_flags;
    if (xfer.multi_lane_mode == SPI_MULTI_LANE_MODE_STRIPE)
    spi_engine_all_lanes_flags(spi, &rx_lane_flags,
    &tx_lane_flags);
    else
    spi_engine_primary_lane_flag(spi, &rx_lane_flags,
    &tx_lane_flags);
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDI_MASK,
    rx_lane_flags));
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDO_MASK,
    tx_lane_flags));
    }
    prev_multi_lane_mode = xfer.multi_lane_mode;
    }
    new_clk_div = host.max_speed_hz / xfer.effective_speed_hz;
    if (new_clk_div != clk_div) {
    clk_div = new_clk_div;
// actual divider used is register value + 1
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_CLK_DIV,
    clk_div - 1));
    }
    if (bits_per_word != xfer.bits_per_word && xfer.len) {
    bits_per_word = xfer.bits_per_word;
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_XFER_BITS,
    bits_per_word));
    }
    spi_engine_gen_xfer(p, dry, xfer, spi.num_rx_lanes);
    spi_engine_gen_sleep(p, dry, spi_delay_to_ns(&xfer.delay, xfer),
    inst_ns, xfer.effective_speed_hz);
    if (xfer.cs_change) {
    if (list_is_last(&xfer.transfer_list, &msg.transfers)) {
    keep_cs = true;
    } else {
    if (!xfer.cs_off)
    spi_engine_gen_cs(p, dry, spi, false);
    spi_engine_gen_sleep(p, dry, spi_delay_to_ns(
    &xfer.cs_change_delay, xfer), inst_ns,
    xfer.effective_speed_hz);
    if (!list_next_entry(xfer, transfer_list).cs_off)
    spi_engine_gen_cs(p, dry, spi, true);
    }
    } else if (!list_is_last(&xfer.transfer_list, &msg.transfers) &&
    xfer.cs_off != list_next_entry(xfer, transfer_list).cs_off) {
    spi_engine_gen_cs(p, dry, spi, xfer.cs_off);
    }
    }
    if (!keep_cs)
    spi_engine_gen_cs(p, dry, spi, false);
//
// Restore clockdiv to default so that future gen_sleep commands don't
// have to be aware of the current register state.
//
    if (clk_div != 1)
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_CLK_DIV, 0));
// Restore single lane mode unless offload disable will restore it later.
    if (prev_multi_lane_mode == SPI_MULTI_LANE_MODE_STRIPE &&
    (!msg.offload || priv.multi_lane_mode != SPI_MULTI_LANE_MODE_STRIPE)) {
    u8 rx_lane_flags, tx_lane_flags;
    spi_engine_primary_lane_flag(spi, &rx_lane_flags, &tx_lane_flags);
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDI_MASK, rx_lane_flags));
    spi_engine_program_add_cmd(p, dry,
    SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDO_MASK, tx_lane_flags));
    }
    }
    static void spi_engine_xfer_next(struct spi_message *msg,
    struct spi_transfer **_xfer)
    {
    struct spi_transfer *xfer = *_xfer;
    if (!xfer) {
    xfer = list_first_entry(&msg.transfers,
    struct spi_transfer, transfer_list);
    } else if (list_is_last(&xfer.transfer_list, &msg.transfers)) {
    xfer = core::ptr::null_mut();
    } else {
    xfer = list_next_entry(xfer, transfer_list);
    }
// _xfer = xfer;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_tx_next(msg: *mut spi_message) {
    static void spi_engine_tx_next(struct spi_message *msg)
    {
    struct spi_engine_message_state *st = msg.state;
    struct spi_transfer *xfer = st.tx_xfer;
    do {
    spi_engine_xfer_next(msg, &xfer);
    } while (xfer && !xfer.tx_buf);
    st.tx_xfer = xfer;
    if (xfer) {
    st.tx_length = xfer.len;
    st.tx_buf = xfer.tx_buf;
    } else {
    st.tx_buf = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_rx_next(msg: *mut spi_message) {
    static void spi_engine_rx_next(struct spi_message *msg)
    {
    struct spi_engine_message_state *st = msg.state;
    struct spi_transfer *xfer = st.rx_xfer;
    do {
    spi_engine_xfer_next(msg, &xfer);
    } while (xfer && !xfer.rx_buf);
    st.rx_xfer = xfer;
    if (xfer) {
    st.rx_length = xfer.len;
    st.rx_buf = xfer.rx_buf;
    } else {
    st.rx_buf = core::ptr::null_mut();
    }
    }
    static bool spi_engine_write_cmd_fifo(struct spi_engine *spi_engine,
    struct spi_message *msg)
    {
    void __iomem *addr = spi_engine.base + SPI_ENGINE_REG_CMD_FIFO;
    struct spi_engine_message_state *st = msg.state;
    unsigned int n, m, i;
    const uint16_t *buf;
    n = readl_relaxed(spi_engine.base + SPI_ENGINE_REG_CMD_FIFO_ROOM);
    while (n && st.cmd_length) {
    m = min(n, st.cmd_length);
    buf = st.cmd_buf;
    for (i = 0; i < m; i++)
    writel_relaxed(buf[i], addr);
    st.cmd_buf += m;
    st.cmd_length -= m;
    n -= m;
    }
    return st.cmd_length != 0;
    }
    static bool spi_engine_write_tx_fifo(struct spi_engine *spi_engine,
    struct spi_message *msg)
    {
    void __iomem *addr = spi_engine.base + SPI_ENGINE_REG_SDO_DATA_FIFO;
    struct spi_engine_message_state *st = msg.state;
    unsigned int n, m, i;
    n = readl_relaxed(spi_engine.base + SPI_ENGINE_REG_SDO_FIFO_ROOM);
    while (n && st.tx_length) {
    if (st.tx_xfer.bits_per_word <= 8) {
    const u8 *buf = st.tx_buf;
    m = min(n, st.tx_length);
    for (i = 0; i < m; i++)
    writel_relaxed(buf[i], addr);
    st.tx_buf += m;
    st.tx_length -= m;
    } else if (st.tx_xfer.bits_per_word <= 16) {
    const u16 *buf = (const u16 *)st.tx_buf;
    m = min(n, st.tx_length / 2);
    for (i = 0; i < m; i++)
    writel_relaxed(buf[i], addr);
    st.tx_buf += m * 2;
    st.tx_length -= m * 2;
    } else {
    const u32 *buf = (const u32 *)st.tx_buf;
    m = min(n, st.tx_length / 4);
    for (i = 0; i < m; i++)
    writel_relaxed(buf[i], addr);
    st.tx_buf += m * 4;
    st.tx_length -= m * 4;
    }
    n -= m;
    if (st.tx_length == 0)
    spi_engine_tx_next(msg);
    }
    return st.tx_length != 0;
    }
    static bool spi_engine_read_rx_fifo(struct spi_engine *spi_engine,
    struct spi_message *msg)
    {
    void __iomem *addr = spi_engine.base + SPI_ENGINE_REG_SDI_DATA_FIFO;
    struct spi_engine_message_state *st = msg.state;
    unsigned int n, m, i;
    n = readl_relaxed(spi_engine.base + SPI_ENGINE_REG_SDI_FIFO_LEVEL);
    while (n && st.rx_length) {
    if (st.rx_xfer.bits_per_word <= 8) {
    u8 *buf = st.rx_buf;
    m = min(n, st.rx_length);
    for (i = 0; i < m; i++)
    buf[i] = readl_relaxed(addr);
    st.rx_buf += m;
    st.rx_length -= m;
    } else if (st.rx_xfer.bits_per_word <= 16) {
    u16 *buf = (u16 *)st.rx_buf;
    m = min(n, st.rx_length / 2);
    for (i = 0; i < m; i++)
    buf[i] = readl_relaxed(addr);
    st.rx_buf += m * 2;
    st.rx_length -= m * 2;
    } else {
    u32 *buf = (u32 *)st.rx_buf;
    m = min(n, st.rx_length / 4);
    for (i = 0; i < m; i++)
    buf[i] = readl_relaxed(addr);
    st.rx_buf += m * 4;
    st.rx_length -= m * 4;
    }
    n -= m;
    if (st.rx_length == 0)
    spi_engine_rx_next(msg);
    }
    return st.rx_length != 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t spi_engine_irq(int irq, void *devid)
    {
    struct spi_controller *host = devid;
    struct spi_message *msg = host.cur_msg;
    struct spi_engine *spi_engine = spi_controller_get_devdata(host);
    let mut disable_int: c_uint = 0;
    unsigned int pending;
    let mut completed_id: c_int = -1;
    pending = readl_relaxed(spi_engine.base + SPI_ENGINE_REG_INT_PENDING);
    if (pending & SPI_ENGINE_INT_SYNC) {
    writel_relaxed(SPI_ENGINE_INT_SYNC,
    spi_engine.base + SPI_ENGINE_REG_INT_PENDING);
    completed_id = readl_relaxed(
    spi_engine.base + SPI_ENGINE_REG_SYNC_ID);
    }
    spin_lock(&spi_engine.lock);
    if (pending & SPI_ENGINE_INT_CMD_ALMOST_EMPTY) {
    if (!spi_engine_write_cmd_fifo(spi_engine, msg))
    disable_int |= SPI_ENGINE_INT_CMD_ALMOST_EMPTY;
    }
    if (pending & SPI_ENGINE_INT_SDO_ALMOST_EMPTY) {
    if (!spi_engine_write_tx_fifo(spi_engine, msg))
    disable_int |= SPI_ENGINE_INT_SDO_ALMOST_EMPTY;
    }
    if (pending & (SPI_ENGINE_INT_SDI_ALMOST_FULL | SPI_ENGINE_INT_SYNC)) {
    if (!spi_engine_read_rx_fifo(spi_engine, msg))
    disable_int |= SPI_ENGINE_INT_SDI_ALMOST_FULL;
    }
    if (pending & SPI_ENGINE_INT_SYNC && msg) {
    if (completed_id == AXI_SPI_ENGINE_CUR_MSG_SYNC_ID) {
    msg.status = 0;
    msg.actual_length = msg.frame_length;
    complete(&spi_engine.msg_complete);
    disable_int |= SPI_ENGINE_INT_SYNC;
    }
    }
    if (disable_int) {
    spi_engine.int_enable &= ~disable_int;
    writel_relaxed(spi_engine.int_enable,
    spi_engine.base + SPI_ENGINE_REG_INT_ENABLE);
    }
    spin_unlock(&spi_engine.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_offload_prepare(msg: *mut spi_message) -> c_int {
    static int spi_engine_offload_prepare(struct spi_message *msg)
    {
    struct spi_controller *host = msg.spi.controller;
    struct spi_engine *spi_engine = spi_controller_get_devdata(host);
    struct spi_engine_program *p = msg.opt_state;
    struct spi_engine_offload *priv = msg.offload.priv;
    struct spi_transfer *xfer;
    void __iomem *cmd_addr;
    void __iomem *sdo_addr;
    let mut tx_word_count: usize = 0;
    unsigned int i;
    if (p.length > spi_engine.offload_ctrl_mem_size)
    return -EINVAL;
// count total number of tx words in message
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
// no support for reading to rx_buf
    if (xfer.rx_buf)
    return -EINVAL;
    if (!xfer.tx_buf)
    continue;
    if (xfer.bits_per_word <= 8)
    tx_word_count += xfer.len;
#[no_mangle]
pub unsafe extern "C" fn if(16: xfer->bits_per_word <=) -> else {
    else if (xfer.bits_per_word <= 16)
    tx_word_count += xfer.len / 2;
    else
    tx_word_count += xfer.len / 4;
    }
    if (tx_word_count && !(spi_engine.offload_caps & SPI_OFFLOAD_CAP_TX_STATIC_DATA))
    return -EINVAL;
    if (tx_word_count > spi_engine.offload_sdo_mem_size)
    return -EINVAL;
//
// This protects against calling spi_optimize_message() with an offload
// that has already been prepared with a different message.
//
    if (test_and_set_bit_lock(SPI_ENGINE_OFFLOAD_FLAG_PREPARED, &priv.flags))
    return -EBUSY;
    cmd_addr = spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_CMD_FIFO(priv.offload_num);
    sdo_addr = spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_SDO_FIFO(priv.offload_num);
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
    if (!xfer.tx_buf)
    continue;
    if (xfer.bits_per_word <= 8) {
    const u8 *buf = xfer.tx_buf;
    for (i = 0; i < xfer.len; i++)
    writel_relaxed(buf[i], sdo_addr);
    } else if (xfer.bits_per_word <= 16) {
    const u16 *buf = xfer.tx_buf;
    for (i = 0; i < xfer.len / 2; i++)
    writel_relaxed(buf[i], sdo_addr);
    } else {
    const u32 *buf = xfer.tx_buf;
    for (i = 0; i < xfer.len / 4; i++)
    writel_relaxed(buf[i], sdo_addr);
    }
    }
    for (i = 0; i < p.length; i++)
    writel_relaxed(p.instructions[i], cmd_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_offload_unprepare(offload: *mut spi_offload) {
    static void spi_engine_offload_unprepare(struct spi_offload *offload)
    {
    struct spi_engine_offload *priv = offload.priv;
    struct spi_engine *spi_engine = priv.spi_engine;
    writel_relaxed(1, spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_RESET(priv.offload_num));
    writel_relaxed(0, spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_RESET(priv.offload_num));
    clear_bit_unlock(SPI_ENGINE_OFFLOAD_FLAG_PREPARED, &priv.flags);
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_optimize_message(msg: *mut spi_message) -> c_int {
    static int spi_engine_optimize_message(struct spi_message *msg)
    {
    struct spi_controller *host = msg.spi.controller;
    struct spi_engine *spi_engine = spi_controller_get_devdata(host);
    struct spi_engine_program p_dry, *p;
    int ret;
    ret = spi_engine_precompile_message(msg);
    if (ret)
    return ret;
    p_dry.length = 0;
    spi_engine_compile_message(msg, true, &p_dry);
    p = kzalloc_flex(*p, instructions, p_dry.length + 1);
    if (!p)
    return -ENOMEM;
    spi_engine_compile_message(msg, false, p);
//
// Non-offload needs SYNC for completion interrupt. Older versions of
// the IP core also need SYNC for offload to work properly.
//
    if (!msg.offload || spi_engine.offload_requires_sync)
    spi_engine_program_add_cmd(p, false, SPI_ENGINE_CMD_SYNC(
    msg.offload ? 0 : AXI_SPI_ENGINE_CUR_MSG_SYNC_ID));
    msg.opt_state = p;
    if (msg.offload) {
    ret = spi_engine_offload_prepare(msg);
    if (ret) {
    msg.opt_state = core::ptr::null_mut();
    kfree(p);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_unoptimize_message(msg: *mut spi_message) -> c_int {
    static int spi_engine_unoptimize_message(struct spi_message *msg)
    {
    if (msg.offload)
    spi_engine_offload_unprepare(msg.offload);
    kfree(msg.opt_state);
    return 0;
    }
    static struct spi_offload
// spi_engine_get_offload(struct spi_device *spi,
    const struct spi_offload_config *config)
    {
    struct spi_controller *host = spi.controller;
    struct spi_engine *spi_engine = spi_controller_get_devdata(host);
    struct spi_engine_offload *priv;
    if (!spi_engine.offload)
    return ERR_PTR(-ENODEV);
    if (config.capability_flags & ~spi_engine.offload_caps)
    return ERR_PTR(-EINVAL);
    priv = spi_engine.offload.priv;
    if (test_and_set_bit_lock(SPI_ENGINE_OFFLOAD_FLAG_ASSIGNED, &priv.flags))
    return ERR_PTR(-EBUSY);
    return spi_engine.offload;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_put_offload(offload: *mut spi_offload) {
    static void spi_engine_put_offload(struct spi_offload *offload)
    {
    struct spi_engine_offload *priv = offload.priv;
    clear_bit_unlock(SPI_ENGINE_OFFLOAD_FLAG_ASSIGNED, &priv.flags);
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_setup(device: *mut spi_device) -> c_int {
    static int spi_engine_setup(struct spi_device *device)
    {
    struct spi_controller *host = device.controller;
    struct spi_engine *spi_engine = spi_controller_get_devdata(host);
    unsigned int reg;
    if (device.mode & SPI_CS_HIGH)
    spi_engine.cs_inv |= BIT(spi_get_chipselect(device, 0));
    else
    spi_engine.cs_inv &= ~BIT(spi_get_chipselect(device, 0));
    writel_relaxed(SPI_ENGINE_CMD_SYNC(0),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    writel_relaxed(SPI_ENGINE_CMD_CS_INV(spi_engine.cs_inv),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    if (host.num_data_lanes > 1) {
    u8 rx_lane_flags, tx_lane_flags;
    spi_engine_primary_lane_flag(device, &rx_lane_flags, &tx_lane_flags);
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDI_MASK,
    rx_lane_flags),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDO_MASK,
    tx_lane_flags),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    }
//
// In addition to setting the flags, we have to do a CS assert command
// to make the new setting actually take effect.
//
    writel_relaxed(SPI_ENGINE_CMD_ASSERT(0, 0xff),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    writel_relaxed(SPI_ENGINE_CMD_SYNC(1),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    return readl_relaxed_poll_timeout(spi_engine.base + SPI_ENGINE_REG_SYNC_ID,
    reg, reg == 1, 1, 1000);
    }
    static int spi_engine_transfer_one_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct spi_engine *spi_engine = spi_controller_get_devdata(host);
    struct spi_engine_message_state *st = &spi_engine.msg_state;
    struct spi_engine_program *p = msg.opt_state;
    let mut int_enable: c_uint = 0;
    unsigned long flags;
    if (msg.offload) {
    dev_err(&host.dev, "Single transfer offload not supported\n");
    msg.status = -EOPNOTSUPP;
    goto out;
    }
// reinitialize message state for this transfer
    memset(st, 0, sizeof(*st));
    st.cmd_buf = p.instructions;
    st.cmd_length = p.length;
    msg.state = st;
    reinit_completion(&spi_engine.msg_complete);
    if (trace_spi_transfer_start_enabled()) {
    struct spi_transfer *xfer;
    list_for_each_entry(xfer, &msg.transfers, transfer_list)
    trace_call__spi_transfer_start(msg, xfer);
    }
    spin_lock_irqsave(&spi_engine.lock, flags);
    if (spi_engine_write_cmd_fifo(spi_engine, msg))
    int_enable |= SPI_ENGINE_INT_CMD_ALMOST_EMPTY;
    spi_engine_tx_next(msg);
    if (spi_engine_write_tx_fifo(spi_engine, msg))
    int_enable |= SPI_ENGINE_INT_SDO_ALMOST_EMPTY;
    spi_engine_rx_next(msg);
    if (st.rx_length != 0)
    int_enable |= SPI_ENGINE_INT_SDI_ALMOST_FULL;
    int_enable |= SPI_ENGINE_INT_SYNC;
    writel_relaxed(int_enable,
    spi_engine.base + SPI_ENGINE_REG_INT_ENABLE);
    spi_engine.int_enable = int_enable;
    spin_unlock_irqrestore(&spi_engine.lock, flags);
    if (!wait_for_completion_timeout(&spi_engine.msg_complete,
    msecs_to_jiffies(5000))) {
    dev_err(&host.dev,
    "Timeout occurred while waiting for transfer to complete. Hardware is probably broken.\n");
    msg.status = -ETIMEDOUT;
    }
    if (trace_spi_transfer_stop_enabled()) {
    struct spi_transfer *xfer;
    list_for_each_entry(xfer, &msg.transfers, transfer_list)
    trace_call__spi_transfer_stop(msg, xfer);
    }
    out:
    spi_finalize_current_message(host);
    return msg.status;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_trigger_enable(offload: *mut spi_offload) -> c_int {
    static int spi_engine_trigger_enable(struct spi_offload *offload)
    {
    struct spi_engine_offload *priv = offload.priv;
    struct spi_engine *spi_engine = priv.spi_engine;
    unsigned int reg;
    int ret;
    writel_relaxed(SPI_ENGINE_CMD_SYNC(0),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_CONFIG,
    priv.spi_mode_config),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    if (priv.bits_per_word)
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_XFER_BITS,
    priv.bits_per_word),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    if (priv.multi_lane_mode == SPI_MULTI_LANE_MODE_STRIPE) {
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDI_MASK,
    priv.rx_all_lanes_mask),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDO_MASK,
    priv.tx_all_lanes_mask),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    }
    writel_relaxed(SPI_ENGINE_CMD_SYNC(1),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    ret = readl_relaxed_poll_timeout(spi_engine.base + SPI_ENGINE_REG_SYNC_ID,
    reg, reg == 1, 1, 1000);
    if (ret)
    return ret;
    reg = readl_relaxed(spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_CTRL(priv.offload_num));
    reg |= SPI_ENGINE_OFFLOAD_CTRL_ENABLE;
    writel_relaxed(reg, spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_CTRL(priv.offload_num));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_trigger_disable(offload: *mut spi_offload) {
    static void spi_engine_trigger_disable(struct spi_offload *offload)
    {
    struct spi_engine_offload *priv = offload.priv;
    struct spi_engine *spi_engine = priv.spi_engine;
    unsigned int reg;
    reg = readl_relaxed(spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_CTRL(priv.offload_num));
    reg &= ~SPI_ENGINE_OFFLOAD_CTRL_ENABLE;
    writel_relaxed(reg, spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_CTRL(priv.offload_num));
// Restore single-lane mode.
    if (priv.multi_lane_mode == SPI_MULTI_LANE_MODE_STRIPE) {
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDI_MASK,
    priv.rx_primary_lane_mask),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    writel_relaxed(SPI_ENGINE_CMD_WRITE(SPI_ENGINE_CMD_REG_SDO_MASK,
    priv.tx_primary_lane_mask),
    spi_engine.base + SPI_ENGINE_REG_CMD_FIFO);
    }
    }
    static struct dma_chan
// spi_engine_tx_stream_request_dma_chan(struct spi_offload *offload)
    {
    struct spi_engine_offload *priv = offload.priv;
    char name[16];
    snprintf(name, sizeof(name), "offload%u-tx", priv.offload_num);
    return dma_request_chan(offload.provider_dev, name);
    }
    static struct dma_chan
// spi_engine_rx_stream_request_dma_chan(struct spi_offload *offload)
    {
    struct spi_engine_offload *priv = offload.priv;
    char name[16];
    snprintf(name, sizeof(name), "offload%u-rx", priv.offload_num);
    return dma_request_chan(offload.provider_dev, name);
    }
    static const struct spi_offload_ops spi_engine_offload_ops = {
    .trigger_enable = spi_engine_trigger_enable,
    .trigger_disable = spi_engine_trigger_disable,
    .tx_stream_request_dma_chan = spi_engine_tx_stream_request_dma_chan,
    .rx_stream_request_dma_chan = spi_engine_rx_stream_request_dma_chan,
    };
#[no_mangle]
unsafe extern "C" fn spi_engine_release_hw(p: *mut c_void) {
    static void spi_engine_release_hw(void *p)
    {
    struct spi_engine *spi_engine = p;
    writel_relaxed(0xff, spi_engine.base + SPI_ENGINE_REG_INT_PENDING);
    writel_relaxed(0x00, spi_engine.base + SPI_ENGINE_REG_INT_ENABLE);
    writel_relaxed(0x01, spi_engine.base + SPI_ENGINE_REG_RESET);
    }
#[no_mangle]
unsafe extern "C" fn spi_engine_probe(pdev: *mut platform_device) -> c_int {
    static int spi_engine_probe(struct platform_device *pdev)
    {
    struct spi_engine *spi_engine;
    struct spi_controller *host;
    unsigned int version, data_width_reg_val;
    int irq, ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*spi_engine));
    if (!host)
    return -ENOMEM;
    spi_engine = spi_controller_get_devdata(host);
    spin_lock_init(&spi_engine.lock);
    init_completion(&spi_engine.msg_complete);
//
// REVISIT: for now, all SPI Engines only have one offload. In the
// future, this should be read from a memory mapped register to
// determine the number of offloads enabled at HDL compile time. For
// now, we can tell if an offload is present if there is a trigger
// source wired up to it.
//
    if (device_property_present(&pdev.dev, "trigger-sources")) {
    struct spi_engine_offload *priv;
    spi_engine.offload =
    devm_spi_offload_alloc(&pdev.dev,
    sizeof(struct spi_engine_offload));
    if (IS_ERR(spi_engine.offload))
    return PTR_ERR(spi_engine.offload);
    priv = spi_engine.offload.priv;
    priv.spi_engine = spi_engine;
    priv.offload_num = 0;
    spi_engine.offload.ops = &spi_engine_offload_ops;
    spi_engine.offload_caps = SPI_OFFLOAD_CAP_TRIGGER;
    if (device_property_match_string(&pdev.dev, "dma-names", "offload0-rx") >= 0) {
    spi_engine.offload_caps |= SPI_OFFLOAD_CAP_RX_STREAM_DMA;
    spi_engine.offload.xfer_flags |= SPI_OFFLOAD_XFER_RX_STREAM;
    }
    if (device_property_match_string(&pdev.dev, "dma-names", "offload0-tx") >= 0) {
    spi_engine.offload_caps |= SPI_OFFLOAD_CAP_TX_STREAM_DMA;
    spi_engine.offload.xfer_flags |= SPI_OFFLOAD_XFER_TX_STREAM;
    } else {
//
// HDL compile option to enable TX DMA stream also disables
// the SDO memory, so can't do both at the same time.
//
    spi_engine.offload_caps |= SPI_OFFLOAD_CAP_TX_STATIC_DATA;
    }
    }
    spi_engine.clk = devm_clk_get_enabled(&pdev.dev, "s_axi_aclk");
    if (IS_ERR(spi_engine.clk))
    return PTR_ERR(spi_engine.clk);
    spi_engine.ref_clk = devm_clk_get_enabled(&pdev.dev, "spi_clk");
    if (IS_ERR(spi_engine.ref_clk))
    return PTR_ERR(spi_engine.ref_clk);
    spi_engine.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spi_engine.base))
    return PTR_ERR(spi_engine.base);
    version = readl(spi_engine.base + ADI_AXI_REG_VERSION);
    if (ADI_AXI_PCORE_VER_MAJOR(version) > 2) {
    dev_err(&pdev.dev, "Unsupported peripheral version %u.%u.%u\n",
    ADI_AXI_PCORE_VER_MAJOR(version),
    ADI_AXI_PCORE_VER_MINOR(version),
    ADI_AXI_PCORE_VER_PATCH(version));
    return -ENODEV;
    }
    data_width_reg_val = readl(spi_engine.base + SPI_ENGINE_REG_DATA_WIDTH);
    if (adi_axi_pcore_ver_gteq(version, 1, 1)) {
    unsigned int sizes = readl(spi_engine.base +
    SPI_ENGINE_REG_OFFLOAD_MEM_ADDR_WIDTH);
    spi_engine.offload_ctrl_mem_size = 1 <<
    FIELD_GET(SPI_ENGINE_SPI_OFFLOAD_MEM_WIDTH_CMD, sizes);
    spi_engine.offload_sdo_mem_size = 1 <<
    FIELD_GET(SPI_ENGINE_SPI_OFFLOAD_MEM_WIDTH_SDO, sizes);
    } else {
    spi_engine.offload_ctrl_mem_size = SPI_ENGINE_OFFLOAD_CMD_FIFO_SIZE;
    spi_engine.offload_sdo_mem_size = SPI_ENGINE_OFFLOAD_SDO_FIFO_SIZE;
    }
// IP v1.5 dropped the requirement for SYNC in offload messages.
    spi_engine.offload_requires_sync = !adi_axi_pcore_ver_gteq(version, 1, 5);
    writel_relaxed(0x00, spi_engine.base + SPI_ENGINE_REG_RESET);
    writel_relaxed(0xff, spi_engine.base + SPI_ENGINE_REG_INT_PENDING);
    writel_relaxed(0x00, spi_engine.base + SPI_ENGINE_REG_INT_ENABLE);
    ret = devm_add_action_or_reset(&pdev.dev, spi_engine_release_hw,
    spi_engine);
    if (ret)
    return ret;
    ret = devm_request_irq(&pdev.dev, irq, spi_engine_irq, 0, pdev.name,
    host);
    if (ret)
    return ret;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_3WIRE;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(1, 32);
    host.max_speed_hz = clk_get_rate(spi_engine.ref_clk) / 2;
    host.transfer_one_message = spi_engine_transfer_one_message;
    host.optimize_message = spi_engine_optimize_message;
    host.unoptimize_message = spi_engine_unoptimize_message;
    host.get_offload = spi_engine_get_offload;
    host.put_offload = spi_engine_put_offload;
    host.num_chipselect = 8;
    if (adi_axi_pcore_ver_gteq(version, 1, 2)) {
    host.mode_bits |= SPI_CS_HIGH;
    host.setup = spi_engine_setup;
    }
    if (adi_axi_pcore_ver_gteq(version, 1, 3))
    host.mode_bits |= SPI_MOSI_IDLE_LOW | SPI_MOSI_IDLE_HIGH;
    if (adi_axi_pcore_ver_gteq(version, 2, 0))
    host.num_data_lanes = FIELD_GET(SPI_ENGINE_REG_DATA_WIDTH_NUM_OF_SDIO_MASK,
    data_width_reg_val);
    if (host.max_speed_hz == 0)
    return dev_err_probe(&pdev.dev, -EINVAL, "spi_clk rate is 0");
    return devm_spi_register_controller(&pdev.dev, host);
    }
    static const struct of_device_id spi_engine_match_table[] = {
    { .compatible = "adi,axi-spi-engine-1.00.a" },
    { },
    };
    MODULE_DEVICE_TABLE(of, spi_engine_match_table);
    static struct platform_driver spi_engine_driver = {
    .probe = spi_engine_probe,
    .driver = {
    .name = "spi-engine",
    .of_match_table = spi_engine_match_table,
    },
    };
    module_platform_driver(spi_engine_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("Analog Devices SPI engine peripheral driver");
    MODULE_LICENSE("GPL");
