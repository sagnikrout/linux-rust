//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/spi/mcp251x.c
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
// CAN bus driver for Microchip 251x/25625 CAN Controller with SPI Interface
//
// MCP2510 support and bug fixes by Christian Pellegrin
// <chripell@evolware.org>
//
// Copyright 2009 Christian Pellegrin EVOL S.r.l.
//
// Copyright 2007 Raymarine UK, Ltd. All Rights Reserved.
// Written under contract by:
// Chris Elston, Katalix Systems, Ltd.
//
// Based on Microchip MCP251x CAN controller driver written by
// David Vrabel, Copyright 2006 Arcom Control Systems Ltd.
//
// Based on CAN bus driver for the CCAN controller written by
// - Sascha Hauer, Marc Kleine-Budde, Pengutronix
// - Simon Kallweit, intefo AG
// Copyright 2007
//

// SPI interface instruction set
pub const INSTRUCTION_WRITE: c_uint = 0x02;
pub const INSTRUCTION_READ: c_uint = 0x03;
pub const INSTRUCTION_BIT_MODIFY: c_uint = 0x05;

pub const INSTRUCTION_RESET: c_uint = 0xC0;
pub const RTS_TXB0: c_uint = 0x01;
pub const RTS_TXB1: c_uint = 0x02;
pub const RTS_TXB2: c_uint = 0x04;

// MPC251x registers
pub const BFPCTRL: c_uint = 0x0c;

pub const TXRTSCTRL: c_uint = 0x0d;

pub const CANSTAT: c_uint = 0x0e;
pub const CANCTRL: c_uint = 0x0f;

pub const TEC: c_uint = 0x1c;
pub const REC: c_uint = 0x1d;
pub const CNF1: c_uint = 0x2a;

pub const CNF2: c_uint = 0x29;

pub const CNF3: c_uint = 0x28;

pub const CANINTE: c_uint = 0x2b;

pub const CANINTF: c_uint = 0x2c;

pub const EFLG: c_uint = 0x2d;

pub const TXBCTRL_OFF: c_int = 0;
pub const TXBSIDH_OFF: c_int = 1;
pub const TXBSIDL_OFF: c_int = 2;
pub const TXBEID8_OFF: c_int = 3;
pub const TXBEID0_OFF: c_int = 4;
pub const TXBDLC_OFF: c_int = 5;
pub const TXBDAT_OFF: c_int = 6;

pub const RXBCTRL_OFF: c_int = 0;
pub const RXBSIDH_OFF: c_int = 1;
pub const RXBSIDL_OFF: c_int = 2;
pub const RXBEID8_OFF: c_int = 3;
pub const RXBEID0_OFF: c_int = 4;
pub const RXBDLC_OFF: c_int = 5;
pub const RXBDAT_OFF: c_int = 6;

    (((val) >> ((byte) * 8)) & 0xff)

    (((val) & 0xff) << ((byte) * 8))
// Buffer size required for the largest SPI transfer (i.e., reading a
// frame)
//
pub const CAN_FRAME_MAX_DATA_LEN: c_int = 8;

pub const CAN_FRAME_MAX_BITS: c_int = 128;
pub const TX_ECHO_SKB_MAX: c_int = 1;

    static const struct can_bittiming_const mcp251x_bittiming_const = {
    .name = DEVICE_NAME,
    .tseg1_min = 3,
    .tseg1_max = 16,
    .tseg2_min = 2,
    .tseg2_max = 8,
    .sjw_max = 4,
    .brp_min = 1,
    .brp_max = 64,
    .brp_inc = 1,
    };
    enum mcp251x_model {
    CAN_MCP251X_MCP2510	= 0x2510,
    CAN_MCP251X_MCP2515	= 0x2515,
    CAN_MCP251X_MCP25625	= 0x25625,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251x_priv {
    pub can: can_priv,
    pub net: *mut net_device,
    pub spi: *mut spi_device,
    pub model: enum mcp251x_model,
    pub /: *mut *mut mutex mcp_lock; / SPI device lock,
    pub spi_tx_buf: *mut u8,
    pub spi_rx_buf: *mut u8,
    pub tx_skb: *mut sk_buff,
    pub wq: *mut workqueue_struct,
    pub tx_work: work_struct,
    pub restart_work: work_struct,
    pub force_quit: c_int,
    pub after_suspend: c_int,
pub const AFTER_SUSPEND_UP: c_int = 1;
pub const AFTER_SUSPEND_DOWN: c_int = 2;
pub const AFTER_SUSPEND_POWER: c_int = 4;
pub const AFTER_SUSPEND_RESTART: c_int = 8;
    pub restart_tx: c_int,
    pub tx_busy: bool,
    pub power: *mut regulator,
    pub transceiver: *mut regulator,
    pub clk: *mut clk,

    pub gpio: gpio_chip,
    pub reg_bfpctrl: u8,

}

    static inline int mcp251x_is_##_model(struct spi_device *spi) \
    { \
    struct mcp251x_priv *priv = spi_get_drvdata(spi); \
    return priv.model == CAN_MCP251X_MCP##_model; \
    }
    MCP251X_IS(2510);
#[no_mangle]
unsafe extern "C" fn mcp251x_clean(net: *mut net_device) {
    static void mcp251x_clean(struct net_device *net)
    {
    struct mcp251x_priv *priv = netdev_priv(net);
    if (priv.tx_skb || priv.tx_busy)
    net.stats.tx_errors++;
    dev_kfree_skb(priv.tx_skb);
    if (priv.tx_busy)
    can_free_echo_skb(priv.net, 0, core::ptr::null_mut());
    priv.tx_skb = core::ptr::null_mut();
    priv.tx_busy = false;
    }
// Note about handling of error return of mcp251x_spi_trans: accessing
// registers via SPI is not really different conceptually than using
// normal I/O assembler instructions, although it's much more
// complicated from a practical POV. So it's not advisable to always
// check the return value of this function. Imagine that every
// read{b,l}, write{b,l} and friends would be bracketed in "if ( < 0)
// error();", it would be a great mess (well there are some situation
// when exception handling C++ like could be useful after all). So we
// just check that transfers are OK at the beginning of our
// conversation with the chip and to avoid doing really nasty things
// (like injecting bogus packets in the network stack).
//
#[no_mangle]
unsafe extern "C" fn mcp251x_spi_trans(spi: *mut spi_device, len: c_int) -> c_int {
    static int mcp251x_spi_trans(struct spi_device *spi, int len)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    struct spi_transfer t = {
    .tx_buf = priv.spi_tx_buf,
    .rx_buf = priv.spi_rx_buf,
    .len = len,
    .cs_change = 0,
    };
    struct spi_message m;
    int ret;
    spi_message_init(&m);
    spi_message_add_tail(&t, &m);
    ret = spi_sync(spi, &m);
    if (ret)
    dev_err(&spi.dev, "spi transfer failed: ret = %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_spi_write(spi: *mut spi_device, len: c_int) -> c_int {
    static int mcp251x_spi_write(struct spi_device *spi, int len)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    int ret;
    ret = spi_write(spi, priv.spi_tx_buf, len);
    if (ret)
    dev_err(&spi.dev, "spi write failed: ret = %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_read_reg(spi: *mut spi_device, reg: u8) -> u8 {
    static u8 mcp251x_read_reg(struct spi_device *spi, u8 reg)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    let mut val: u8 = 0;
    priv.spi_tx_buf[0] = INSTRUCTION_READ;
    priv.spi_tx_buf[1] = reg;
    if (spi.controller.flags & SPI_CONTROLLER_HALF_DUPLEX) {
    spi_write_then_read(spi, priv.spi_tx_buf, 2, &val, 1);
    } else {
    mcp251x_spi_trans(spi, 3);
    val = priv.spi_rx_buf[2];
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_read_2regs(spi: *mut spi_device, reg: u8, v1: *mut u8, v2: *mut u8) {
    static void mcp251x_read_2regs(struct spi_device *spi, u8 reg, u8 *v1, u8 *v2)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = INSTRUCTION_READ;
    priv.spi_tx_buf[1] = reg;
    if (spi.controller.flags & SPI_CONTROLLER_HALF_DUPLEX) {
    u8 val[2] = { 0 };
    spi_write_then_read(spi, priv.spi_tx_buf, 2, val, 2);
// v1 = val[0];
// v2 = val[1];
    } else {
    mcp251x_spi_trans(spi, 4);
// v1 = priv->spi_rx_buf[2];
// v2 = priv->spi_rx_buf[3];
    }
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_write_reg(spi: *mut spi_device, reg: u8, val: u8) {
    static void mcp251x_write_reg(struct spi_device *spi, u8 reg, u8 val)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = INSTRUCTION_WRITE;
    priv.spi_tx_buf[1] = reg;
    priv.spi_tx_buf[2] = val;
    mcp251x_spi_write(spi, 3);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_write_2regs(spi: *mut spi_device, reg: u8, v1: u8, v2: u8) {
    static void mcp251x_write_2regs(struct spi_device *spi, u8 reg, u8 v1, u8 v2)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = INSTRUCTION_WRITE;
    priv.spi_tx_buf[1] = reg;
    priv.spi_tx_buf[2] = v1;
    priv.spi_tx_buf[3] = v2;
    mcp251x_spi_write(spi, 4);
    }
    static int mcp251x_write_bits(struct spi_device *spi, u8 reg,
    u8 mask, u8 val)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = INSTRUCTION_BIT_MODIFY;
    priv.spi_tx_buf[1] = reg;
    priv.spi_tx_buf[2] = mask;
    priv.spi_tx_buf[3] = val;
    return mcp251x_spi_write(spi, 4);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_read_stat(spi: *mut spi_device) -> u8 {
    static u8 mcp251x_read_stat(struct spi_device *spi)
    {
    return mcp251x_read_reg(spi, CANSTAT) & CANCTRL_REQOP_MASK;
    }

    readx_poll_timeout(mcp251x_read_stat, addr, val, cond, \
    delay_us, timeout_us)

    enum {
    MCP251X_GPIO_TX0RTS = 0,		/* inputs */
    MCP251X_GPIO_TX1RTS,
    MCP251X_GPIO_TX2RTS,
    MCP251X_GPIO_RX0BF,			/* outputs */
    MCP251X_GPIO_RX1BF,
    };

    GENMASK(MCP251X_GPIO_TX2RTS, MCP251X_GPIO_TX0RTS)

    GENMASK(MCP251X_GPIO_RX1BF, MCP251X_GPIO_RX0BF)
    static const char * const mcp251x_gpio_names[] = {
    [MCP251X_GPIO_TX0RTS] = "TX0RTS",	/* inputs */
    [MCP251X_GPIO_TX1RTS] = "TX1RTS",
    [MCP251X_GPIO_TX2RTS] = "TX2RTS",
    [MCP251X_GPIO_RX0BF] = "RX0BF",		/* outputs */
    [MCP251X_GPIO_RX1BF] = "RX1BF",
    };
#[no_mangle]
pub unsafe extern "C" fn mcp251x_gpio_is_input(offset: c_uint) -> bool {
    static inline bool mcp251x_gpio_is_input(unsigned int offset)
    {
    return offset <= MCP251X_GPIO_TX2RTS;
    }
    static int mcp251x_gpio_request(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct mcp251x_priv *priv = gpiochip_get_data(chip);
    int ret;
    u8 val;
// nothing to be done for inputs
    if (mcp251x_gpio_is_input(offset))
    return 0;
    val = BFPCTRL_BFE(offset - MCP251X_GPIO_RX0BF);
    mutex_lock(&priv.mcp_lock);
    ret = mcp251x_write_bits(priv.spi, BFPCTRL, val, val);
    mutex_unlock(&priv.mcp_lock);
    if (ret)
    return ret;
    priv.reg_bfpctrl |= val;
    return 0;
    }
    static void mcp251x_gpio_free(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct mcp251x_priv *priv = gpiochip_get_data(chip);
    u8 val;
// nothing to be done for inputs
    if (mcp251x_gpio_is_input(offset))
    return;
    val = BFPCTRL_BFE(offset - MCP251X_GPIO_RX0BF);
    mutex_lock(&priv.mcp_lock);
    mcp251x_write_bits(priv.spi, BFPCTRL, val, 0);
    mutex_unlock(&priv.mcp_lock);
    priv.reg_bfpctrl &= ~val;
    }
    static int mcp251x_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    if (mcp251x_gpio_is_input(offset))
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int mcp251x_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct mcp251x_priv *priv = gpiochip_get_data(chip);
    u8 reg, mask, val;
    if (mcp251x_gpio_is_input(offset)) {
    reg = TXRTSCTRL;
    mask = TXRTSCTRL_RTS(offset);
    } else {
    reg = BFPCTRL;
    mask = BFPCTRL_BFS(offset - MCP251X_GPIO_RX0BF);
    }
    mutex_lock(&priv.mcp_lock);
    val = mcp251x_read_reg(priv.spi, reg);
    mutex_unlock(&priv.mcp_lock);
    return !!(val & mask);
    }
    static int mcp251x_gpio_get_multiple(struct gpio_chip *chip,
    unsigned long *maskp, unsigned long *bitsp)
    {
    struct mcp251x_priv *priv = gpiochip_get_data(chip);
    let mut bits: c_ulong = 0;
    u8 val;
    mutex_lock(&priv.mcp_lock);
    if (maskp[0] & MCP251X_GPIO_INPUT_MASK) {
    val = mcp251x_read_reg(priv.spi, TXRTSCTRL);
    val = FIELD_GET(TXRTSCTRL_RTS_MASK, val);
    bits |= FIELD_PREP(MCP251X_GPIO_INPUT_MASK, val);
    }
    if (maskp[0] & MCP251X_GPIO_OUTPUT_MASK) {
    val = mcp251x_read_reg(priv.spi, BFPCTRL);
    val = FIELD_GET(BFPCTRL_BFS_MASK, val);
    bits |= FIELD_PREP(MCP251X_GPIO_OUTPUT_MASK, val);
    }
    mutex_unlock(&priv.mcp_lock);
    bitsp[0] = bits;
    return 0;
    }
    static int mcp251x_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct mcp251x_priv *priv = gpiochip_get_data(chip);
    u8 mask, val;
    int ret;
    mask = BFPCTRL_BFS(offset - MCP251X_GPIO_RX0BF);
    val = value ? mask : 0;
    mutex_lock(&priv.mcp_lock);
    ret = mcp251x_write_bits(priv.spi, BFPCTRL, mask, val);
    mutex_unlock(&priv.mcp_lock);
    if (ret)
    return ret;
    priv.reg_bfpctrl &= ~mask;
    priv.reg_bfpctrl |= val;
    return 0;
    }
    static int
    mcp251x_gpio_set_multiple(struct gpio_chip *chip,
    unsigned long *maskp, unsigned long *bitsp)
    {
    struct mcp251x_priv *priv = gpiochip_get_data(chip);
    u8 mask, val;
    int ret;
    mask = FIELD_GET(MCP251X_GPIO_OUTPUT_MASK, maskp[0]);
    mask = FIELD_PREP(BFPCTRL_BFS_MASK, mask);
    val = FIELD_GET(MCP251X_GPIO_OUTPUT_MASK, bitsp[0]);
    val = FIELD_PREP(BFPCTRL_BFS_MASK, val);
    if (!mask)
    return 0;
    mutex_lock(&priv.mcp_lock);
    ret = mcp251x_write_bits(priv.spi, BFPCTRL, mask, val);
    mutex_unlock(&priv.mcp_lock);
    if (ret)
    return ret;
    priv.reg_bfpctrl &= ~mask;
    priv.reg_bfpctrl |= val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_gpio_restore(spi: *mut spi_device) {
    static void mcp251x_gpio_restore(struct spi_device *spi)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    mcp251x_write_reg(spi, BFPCTRL, priv.reg_bfpctrl);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_gpio_setup(priv: *mut mcp251x_priv) -> c_int {
    static int mcp251x_gpio_setup(struct mcp251x_priv *priv)
    {
    struct gpio_chip *gpio = &priv.gpio;
    if (!device_property_present(&priv.spi.dev, "gpio-controller"))
    return 0;
// gpiochip handles TX[0..2]RTS and RX[0..1]BF
    gpio.label = priv.spi.modalias;
    gpio.parent = &priv.spi.dev;
    gpio.owner = THIS_MODULE;
    gpio.request = mcp251x_gpio_request;
    gpio.free = mcp251x_gpio_free;
    gpio.get_direction = mcp251x_gpio_get_direction;
    gpio.get = mcp251x_gpio_get;
    gpio.get_multiple = mcp251x_gpio_get_multiple;
    gpio.set = mcp251x_gpio_set;
    gpio.set_multiple = mcp251x_gpio_set_multiple;
    gpio.base = -1;
    gpio.ngpio = ARRAY_SIZE(mcp251x_gpio_names);
    gpio.names = mcp251x_gpio_names;
    gpio.can_sleep = true;
    return devm_gpiochip_add_data(&priv.spi.dev, gpio, priv);
    }

#[no_mangle]
pub unsafe extern "C" fn mcp251x_gpio_restore(spi: *mut spi_device) {
    static inline void mcp251x_gpio_restore(struct spi_device *spi)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn mcp251x_gpio_setup(priv: *mut mcp251x_priv) -> c_int {
    static inline int mcp251x_gpio_setup(struct mcp251x_priv *priv)
    {
    return 0;
    }

    static void mcp251x_hw_tx_frame(struct spi_device *spi, u8 *buf,
    int len, int tx_buf_idx)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    if (mcp251x_is_2510(spi)) {
    int i;
    for (i = 1; i < TXBDAT_OFF + len; i++)
    mcp251x_write_reg(spi, TXBCTRL(tx_buf_idx) + i,
    buf[i]);
    } else {
    memcpy(priv.spi_tx_buf, buf, TXBDAT_OFF + len);
    mcp251x_spi_write(spi, TXBDAT_OFF + len);
    }
    }
    static void mcp251x_hw_tx(struct spi_device *spi, struct can_frame *frame,
    int tx_buf_idx)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    u32 sid, eid, exide, rtr;
    u8 buf[SPI_TRANSFER_BUF_LEN];
    exide = (frame.can_id & CAN_EFF_FLAG) ? 1 : 0; /* Extended ID Enable */
    if (exide)
    sid = (frame.can_id & CAN_EFF_MASK) >> 18;
    else
    sid = frame.can_id & CAN_SFF_MASK; /* Standard ID */
    eid = frame.can_id & CAN_EFF_MASK; /* Extended ID */
    rtr = (frame.can_id & CAN_RTR_FLAG) ? 1 : 0; /* Remote transmission */
    buf[TXBCTRL_OFF] = INSTRUCTION_LOAD_TXB(tx_buf_idx);
    buf[TXBSIDH_OFF] = sid >> SIDH_SHIFT;
    buf[TXBSIDL_OFF] = ((sid & SIDL_SID_MASK) << SIDL_SID_SHIFT) |
    (exide << SIDL_EXIDE_SHIFT) |
    ((eid >> SIDL_EID_SHIFT) & SIDL_EID_MASK);
    buf[TXBEID8_OFF] = GET_BYTE(eid, 1);
    buf[TXBEID0_OFF] = GET_BYTE(eid, 0);
    buf[TXBDLC_OFF] = (rtr << DLC_RTR_SHIFT) | frame.len;
    memcpy(buf + TXBDAT_OFF, frame.data, frame.len);
    mcp251x_hw_tx_frame(spi, buf, frame.len, tx_buf_idx);
// use INSTRUCTION_RTS, to avoid "repeated frame problem"
    priv.spi_tx_buf[0] = INSTRUCTION_RTS(1 << tx_buf_idx);
    mcp251x_spi_write(priv.spi, 1);
    }
    static void mcp251x_hw_rx_frame(struct spi_device *spi, u8 *buf,
    int buf_idx)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    if (mcp251x_is_2510(spi)) {
    int i, len;
    for (i = 1; i < RXBDAT_OFF; i++)
    buf[i] = mcp251x_read_reg(spi, RXBCTRL(buf_idx) + i);
    len = can_cc_dlc2len(buf[RXBDLC_OFF] & RXBDLC_LEN_MASK);
    for (; i < (RXBDAT_OFF + len); i++)
    buf[i] = mcp251x_read_reg(spi, RXBCTRL(buf_idx) + i);
    } else {
    priv.spi_tx_buf[RXBCTRL_OFF] = INSTRUCTION_READ_RXB(buf_idx);
    if (spi.controller.flags & SPI_CONTROLLER_HALF_DUPLEX) {
    spi_write_then_read(spi, priv.spi_tx_buf, 1,
    priv.spi_rx_buf,
    SPI_TRANSFER_BUF_LEN);
    memcpy(buf + 1, priv.spi_rx_buf,
    SPI_TRANSFER_BUF_LEN - 1);
    } else {
    mcp251x_spi_trans(spi, SPI_TRANSFER_BUF_LEN);
    memcpy(buf, priv.spi_rx_buf, SPI_TRANSFER_BUF_LEN);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_hw_rx(spi: *mut spi_device, buf_idx: c_int) {
    static void mcp251x_hw_rx(struct spi_device *spi, int buf_idx)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    struct sk_buff *skb;
    struct can_frame *frame;
    u8 buf[SPI_TRANSFER_BUF_LEN];
    skb = alloc_can_skb(priv.net, &frame);
    if (!skb) {
    dev_err(&spi.dev, "cannot allocate RX skb\n");
    priv.net.stats.rx_dropped++;
    return;
    }
    mcp251x_hw_rx_frame(spi, buf, buf_idx);
    if (buf[RXBSIDL_OFF] & RXBSIDL_IDE) {
// Extended ID format
    frame.can_id = CAN_EFF_FLAG;
    frame.can_id |=
// Extended ID part
    SET_BYTE(buf[RXBSIDL_OFF] & RXBSIDL_EID, 2) |
    SET_BYTE(buf[RXBEID8_OFF], 1) |
    SET_BYTE(buf[RXBEID0_OFF], 0) |
// Standard ID part
    (((buf[RXBSIDH_OFF] << RXBSIDH_SHIFT) |
    (buf[RXBSIDL_OFF] >> RXBSIDL_SHIFT)) << 18);
// Remote transmission request
    if (buf[RXBDLC_OFF] & RXBDLC_RTR)
    frame.can_id |= CAN_RTR_FLAG;
    } else {
// Standard ID format
    frame.can_id =
    (buf[RXBSIDH_OFF] << RXBSIDH_SHIFT) |
    (buf[RXBSIDL_OFF] >> RXBSIDL_SHIFT);
    if (buf[RXBSIDL_OFF] & RXBSIDL_SRR)
    frame.can_id |= CAN_RTR_FLAG;
    }
// Data length
    frame.len = can_cc_dlc2len(buf[RXBDLC_OFF] & RXBDLC_LEN_MASK);
    if (!(frame.can_id & CAN_RTR_FLAG)) {
    memcpy(frame.data, buf + RXBDAT_OFF, frame.len);
    priv.net.stats.rx_bytes += frame.len;
    }
    priv.net.stats.rx_packets++;
    netif_rx(skb);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_hw_sleep(spi: *mut spi_device) {
    static void mcp251x_hw_sleep(struct spi_device *spi)
    {
    mcp251x_write_reg(spi, CANCTRL, CANCTRL_REQOP_SLEEP);
    }
// May only be called when device is sleeping!
#[no_mangle]
unsafe extern "C" fn mcp251x_hw_wake(spi: *mut spi_device) -> c_int {
    static int mcp251x_hw_wake(struct spi_device *spi)
    {
    u8 value;
    int ret;
// Force wakeup interrupt to wake device, but don't execute IST
    disable_irq_nosync(spi.irq);
    mcp251x_write_2regs(spi, CANINTE, CANINTE_WAKIE, CANINTF_WAKIF);
// Wait for oscillator startup timer after wake up
    mdelay(MCP251X_OST_DELAY_MS);
// Put device into config mode
    mcp251x_write_reg(spi, CANCTRL, CANCTRL_REQOP_CONF);
// Wait for the device to enter config mode
    ret = mcp251x_read_stat_poll_timeout(spi, value, value == CANCTRL_REQOP_CONF,
    MCP251X_OST_DELAY_MS * 1000,
    USEC_PER_SEC);
    if (ret) {
    dev_err(&spi.dev, "MCP251x didn't enter in config mode\n");
    return ret;
    }
// Disable and clear pending interrupts
    mcp251x_write_2regs(spi, CANINTE, 0x00, 0x00);
    enable_irq(spi.irq);
    return 0;
    }
    static netdev_tx_t mcp251x_hard_start_xmit(struct sk_buff *skb,
    struct net_device *net)
    {
    struct mcp251x_priv *priv = netdev_priv(net);
    struct spi_device *spi = priv.spi;
    if (priv.tx_skb || priv.tx_busy) {
    dev_warn(&spi.dev, "hard_xmit called while tx busy\n");
    return NETDEV_TX_BUSY;
    }
    if (can_dev_dropped_skb(net, skb))
    return NETDEV_TX_OK;
    netif_stop_queue(net);
    priv.tx_skb = skb;
    queue_work(priv.wq, &priv.tx_work);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_do_set_mode(net: *mut net_device, mode: enum can_mode) -> c_int {
    static int mcp251x_do_set_mode(struct net_device *net, enum can_mode mode)
    {
    struct mcp251x_priv *priv = netdev_priv(net);
    switch (mode) {
    case CAN_MODE_START:
    mcp251x_clean(net);
// We have to delay work since SPI I/O may sleep
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    priv.restart_tx = 1;
    if (priv.can.restart_ms == 0)
    priv.after_suspend = AFTER_SUSPEND_RESTART;
    queue_work(priv.wq, &priv.restart_work);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_set_normal_mode(spi: *mut spi_device) -> c_int {
    static int mcp251x_set_normal_mode(struct spi_device *spi)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    u8 value;
    int ret;
// Enable interrupts
    mcp251x_write_reg(spi, CANINTE,
    CANINTE_ERRIE | CANINTE_TX2IE | CANINTE_TX1IE |
    CANINTE_TX0IE | CANINTE_RX1IE | CANINTE_RX0IE);
    if (priv.can.ctrlmode & CAN_CTRLMODE_LOOPBACK) {
// Put device into loopback mode
    mcp251x_write_reg(spi, CANCTRL, CANCTRL_REQOP_LOOPBACK);
    } else if (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY) {
// Put device into listen-only mode
    mcp251x_write_reg(spi, CANCTRL, CANCTRL_REQOP_LISTEN_ONLY);
    } else {
// Put device into normal mode
    mcp251x_write_reg(spi, CANCTRL, CANCTRL_REQOP_NORMAL);
// Wait for the device to enter normal mode
    ret = mcp251x_read_stat_poll_timeout(spi, value, value == 0,
    MCP251X_OST_DELAY_MS * 1000,
    USEC_PER_SEC);
    if (ret) {
    dev_err(&spi.dev, "MCP251x didn't enter in normal mode\n");
    return ret;
    }
    }
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_do_set_bittiming(net: *mut net_device) -> c_int {
    static int mcp251x_do_set_bittiming(struct net_device *net)
    {
    struct mcp251x_priv *priv = netdev_priv(net);
    struct can_bittiming *bt = &priv.can.bittiming;
    struct spi_device *spi = priv.spi;
    mcp251x_write_reg(spi, CNF1, ((bt.sjw - 1) << CNF1_SJW_SHIFT) |
    (bt.brp - 1));
    mcp251x_write_reg(spi, CNF2, CNF2_BTLMODE |
    (priv.can.ctrlmode & CAN_CTRLMODE_3_SAMPLES ?
    CNF2_SAM : 0) |
    ((bt.phase_seg1 - 1) << CNF2_PS1_SHIFT) |
    (bt.prop_seg - 1));
    mcp251x_write_bits(spi, CNF3, CNF3_PHSEG2_MASK,
    (bt.phase_seg2 - 1));
    dev_dbg(&spi.dev, "CNF: 0x%02x 0x%02x 0x%02x\n",
    mcp251x_read_reg(spi, CNF1),
    mcp251x_read_reg(spi, CNF2),
    mcp251x_read_reg(spi, CNF3));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_setup(net: *mut net_device, spi: *mut spi_device) -> c_int {
    static int mcp251x_setup(struct net_device *net, struct spi_device *spi)
    {
    mcp251x_do_set_bittiming(net);
    mcp251x_write_reg(spi, RXBCTRL(0),
    RXBCTRL_BUKT | RXBCTRL_RXM0 | RXBCTRL_RXM1);
    mcp251x_write_reg(spi, RXBCTRL(1),
    RXBCTRL_RXM0 | RXBCTRL_RXM1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_hw_reset(spi: *mut spi_device) -> c_int {
    static int mcp251x_hw_reset(struct spi_device *spi)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    u8 value;
    int ret;
// Wait for oscillator startup timer after power up
    mdelay(MCP251X_OST_DELAY_MS);
    priv.spi_tx_buf[0] = INSTRUCTION_RESET;
    ret = mcp251x_spi_write(spi, 1);
    if (ret)
    return ret;
// Wait for oscillator startup timer after reset
    mdelay(MCP251X_OST_DELAY_MS);
// Wait for reset to finish
    ret = mcp251x_read_stat_poll_timeout(spi, value, value == CANCTRL_REQOP_CONF,
    MCP251X_OST_DELAY_MS * 1000,
    USEC_PER_SEC);
    if (ret)
    dev_err(&spi.dev, "MCP251x didn't enter in conf mode after reset\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_hw_probe(spi: *mut spi_device) -> c_int {
    static int mcp251x_hw_probe(struct spi_device *spi)
    {
    u8 ctrl;
    int ret;
    ret = mcp251x_hw_reset(spi);
    if (ret)
    return ret;
    ctrl = mcp251x_read_reg(spi, CANCTRL);
    dev_dbg(&spi.dev, "CANCTRL 0x%02x\n", ctrl);
// Check for power up default value
    if ((ctrl & 0x17) != 0x07)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_power_enable(reg: *mut regulator, enable: c_int) -> c_int {
    static int mcp251x_power_enable(struct regulator *reg, int enable)
    {
    if (IS_ERR_OR_NULL(reg))
    return 0;
    if (enable)
    return regulator_enable(reg);
    else
    return regulator_disable(reg);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_stop(net: *mut net_device) -> c_int {
    static int mcp251x_stop(struct net_device *net)
    {
    struct mcp251x_priv *priv = netdev_priv(net);
    struct spi_device *spi = priv.spi;
    close_candev(net);
    priv.force_quit = 1;
    free_irq(spi.irq, priv);
    mutex_lock(&priv.mcp_lock);
// Disable and clear pending interrupts
    mcp251x_write_2regs(spi, CANINTE, 0x00, 0x00);
    mcp251x_write_reg(spi, TXBCTRL(0), 0);
    mcp251x_clean(net);
    mcp251x_hw_sleep(spi);
    mcp251x_power_enable(priv.transceiver, 0);
    priv.can.state = CAN_STATE_STOPPED;
    mutex_unlock(&priv.mcp_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_error_skb(net: *mut net_device, can_id: c_int, data1: c_int) {
    static void mcp251x_error_skb(struct net_device *net, int can_id, int data1)
    {
    struct sk_buff *skb;
    struct can_frame *frame;
    skb = alloc_can_err_skb(net, &frame);
    if (skb) {
    frame.can_id |= can_id;
    frame.data[1] = data1;
    netif_rx(skb);
    } else {
    netdev_err(net, "cannot allocate error skb\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_tx_work_handler(ws: *mut work_struct) {
    static void mcp251x_tx_work_handler(struct work_struct *ws)
    {
    struct mcp251x_priv *priv = container_of(ws, struct mcp251x_priv,
    tx_work);
    struct spi_device *spi = priv.spi;
    struct net_device *net = priv.net;
    struct can_frame *frame;
    mutex_lock(&priv.mcp_lock);
    if (priv.tx_skb) {
    if (priv.can.state == CAN_STATE_BUS_OFF) {
    mcp251x_clean(net);
    } else {
    frame = (struct can_frame *)priv.tx_skb.data;
    if (frame.len > CAN_FRAME_MAX_DATA_LEN)
    frame.len = CAN_FRAME_MAX_DATA_LEN;
    mcp251x_hw_tx(spi, frame, 0);
    priv.tx_busy = true;
    can_put_echo_skb(priv.tx_skb, net, 0, 0);
    priv.tx_skb = core::ptr::null_mut();
    }
    }
    mutex_unlock(&priv.mcp_lock);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_restart_work_handler(ws: *mut work_struct) {
    static void mcp251x_restart_work_handler(struct work_struct *ws)
    {
    struct mcp251x_priv *priv = container_of(ws, struct mcp251x_priv,
    restart_work);
    struct spi_device *spi = priv.spi;
    struct net_device *net = priv.net;
    mutex_lock(&priv.mcp_lock);
    if (priv.after_suspend) {
    if (priv.after_suspend & AFTER_SUSPEND_POWER) {
    mcp251x_hw_reset(spi);
    mcp251x_setup(net, spi);
    mcp251x_gpio_restore(spi);
    } else {
    mcp251x_hw_wake(spi);
    }
    priv.force_quit = 0;
    if (priv.after_suspend & AFTER_SUSPEND_RESTART) {
    mcp251x_set_normal_mode(spi);
    } else if (priv.after_suspend & AFTER_SUSPEND_UP) {
    netif_device_attach(net);
    mcp251x_clean(net);
    mcp251x_set_normal_mode(spi);
    netif_wake_queue(net);
    } else {
    mcp251x_hw_sleep(spi);
    }
    priv.after_suspend = 0;
    }
    if (priv.restart_tx) {
    priv.restart_tx = 0;
    mcp251x_write_reg(spi, TXBCTRL(0), 0);
    mcp251x_clean(net);
    netif_wake_queue(net);
    mcp251x_error_skb(net, CAN_ERR_RESTARTED, 0);
    }
    mutex_unlock(&priv.mcp_lock);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_can_ist(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mcp251x_can_ist(int irq, void *dev_id)
    {
    struct mcp251x_priv *priv = dev_id;
    struct spi_device *spi = priv.spi;
    struct net_device *net = priv.net;
    mutex_lock(&priv.mcp_lock);
    while (!priv.force_quit) {
    enum can_state new_state;
    u8 intf, eflag;
    let mut clear_intf: u8 = 0;
    let mut can_id: c_int = 0, data1 = 0;
    mcp251x_read_2regs(spi, CANINTF, &intf, &eflag);
// receive buffer 0
    if (intf & CANINTF_RX0IF) {
    mcp251x_hw_rx(spi, 0);
// Free one buffer ASAP
// (The MCP2515/25625 does this automatically.)
//
    if (mcp251x_is_2510(spi))
    mcp251x_write_bits(spi, CANINTF,
    CANINTF_RX0IF, 0x00);
// check if buffer 1 is already known to be full, no need to re-read
    if (!(intf & CANINTF_RX1IF)) {
    u8 intf1, eflag1;
// intf needs to be read again to avoid a race condition
    mcp251x_read_2regs(spi, CANINTF, &intf1, &eflag1);
// combine flags from both operations for error handling
    intf |= intf1;
    eflag |= eflag1;
    }
    }
// receive buffer 1
    if (intf & CANINTF_RX1IF) {
    mcp251x_hw_rx(spi, 1);
// The MCP2515/25625 does this automatically.
    if (mcp251x_is_2510(spi))
    clear_intf |= CANINTF_RX1IF;
    }
// mask out flags we don't care about
    intf &= CANINTF_RX | CANINTF_TX | CANINTF_ERR;
// any error or tx interrupt we need to clear?
    if (intf & (CANINTF_ERR | CANINTF_TX))
    clear_intf |= intf & (CANINTF_ERR | CANINTF_TX);
    if (clear_intf)
    mcp251x_write_bits(spi, CANINTF, clear_intf, 0x00);
    if (eflag & (EFLG_RX0OVR | EFLG_RX1OVR))
    mcp251x_write_bits(spi, EFLG, eflag, 0x00);
// Update can state
    if (eflag & EFLG_TXBO) {
    new_state = CAN_STATE_BUS_OFF;
    can_id |= CAN_ERR_BUSOFF;
    } else if (eflag & EFLG_TXEP) {
    new_state = CAN_STATE_ERROR_PASSIVE;
    can_id |= CAN_ERR_CRTL;
    data1 |= CAN_ERR_CRTL_TX_PASSIVE;
    } else if (eflag & EFLG_RXEP) {
    new_state = CAN_STATE_ERROR_PASSIVE;
    can_id |= CAN_ERR_CRTL;
    data1 |= CAN_ERR_CRTL_RX_PASSIVE;
    } else if (eflag & EFLG_TXWAR) {
    new_state = CAN_STATE_ERROR_WARNING;
    can_id |= CAN_ERR_CRTL;
    data1 |= CAN_ERR_CRTL_TX_WARNING;
    } else if (eflag & EFLG_RXWAR) {
    new_state = CAN_STATE_ERROR_WARNING;
    can_id |= CAN_ERR_CRTL;
    data1 |= CAN_ERR_CRTL_RX_WARNING;
    } else {
    new_state = CAN_STATE_ERROR_ACTIVE;
    }
// Update can state statistics
    switch (priv.can.state) {
    case CAN_STATE_ERROR_ACTIVE:
    if (new_state >= CAN_STATE_ERROR_WARNING &&
    new_state <= CAN_STATE_BUS_OFF)
    priv.can.can_stats.error_warning++;
    fallthrough;
    case CAN_STATE_ERROR_WARNING:
    if (new_state >= CAN_STATE_ERROR_PASSIVE &&
    new_state <= CAN_STATE_BUS_OFF)
    priv.can.can_stats.error_passive++;
    break;
    default:
    break;
    }
    priv.can.state = new_state;
    if (intf & CANINTF_ERRIF) {
// Handle overflow counters
    if (eflag & (EFLG_RX0OVR | EFLG_RX1OVR)) {
    if (eflag & EFLG_RX0OVR) {
    net.stats.rx_over_errors++;
    net.stats.rx_errors++;
    }
    if (eflag & EFLG_RX1OVR) {
    net.stats.rx_over_errors++;
    net.stats.rx_errors++;
    }
    can_id |= CAN_ERR_CRTL;
    data1 |= CAN_ERR_CRTL_RX_OVERFLOW;
    }
    mcp251x_error_skb(net, can_id, data1);
    }
    if (priv.can.state == CAN_STATE_BUS_OFF) {
    if (priv.can.restart_ms == 0) {
    priv.force_quit = 1;
    priv.can.can_stats.bus_off++;
    can_bus_off(net);
    mcp251x_hw_sleep(spi);
    break;
    }
    }
    if (intf == 0)
    break;
    if (intf & CANINTF_TX) {
    if (priv.tx_busy) {
    net.stats.tx_packets++;
    net.stats.tx_bytes += can_get_echo_skb(net, 0,
    core::ptr::null_mut());
    priv.tx_busy = false;
    }
    netif_wake_queue(net);
    }
    }
    mutex_unlock(&priv.mcp_lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_open(net: *mut net_device) -> c_int {
    static int mcp251x_open(struct net_device *net)
    {
    struct mcp251x_priv *priv = netdev_priv(net);
    struct spi_device *spi = priv.spi;
    let mut release_irq: bool = false;
    let mut flags: c_ulong = 0;
    int ret;
    ret = open_candev(net);
    if (ret) {
    dev_err(&spi.dev, "unable to set initial baudrate!\n");
    return ret;
    }
    mutex_lock(&priv.mcp_lock);
    ret = mcp251x_power_enable(priv.transceiver, 1);
    if (ret) {
    dev_err(&spi.dev, "failed to enable transceiver power: %pe\n", ERR_PTR(ret));
    goto out_close_candev;
    }
    priv.force_quit = 0;
    priv.tx_skb = core::ptr::null_mut();
    priv.tx_busy = false;
    if (!dev_fwnode(&spi.dev))
    flags = IRQF_TRIGGER_FALLING;
    ret = request_threaded_irq(spi.irq, core::ptr::null_mut(), mcp251x_can_ist,
    flags | IRQF_ONESHOT, dev_name(&spi.dev),
    priv);
    if (ret) {
    dev_err(&spi.dev, "failed to acquire irq %d\n", spi.irq);
    goto out_close;
    }
    ret = mcp251x_hw_wake(spi);
    if (ret)
    goto out_free_irq;
    ret = mcp251x_setup(net, spi);
    if (ret)
    goto out_free_irq;
    ret = mcp251x_set_normal_mode(spi);
    if (ret)
    goto out_free_irq;
    netif_wake_queue(net);
    mutex_unlock(&priv.mcp_lock);
    return 0;
    out_free_irq:
// The IRQ handler might be running, and if so it will be waiting
// for the lock. But free_irq() must wait for the handler to finish
// so calling it here would deadlock.
//
// Setting priv->force_quit will let the handler exit right away
// without any access to the hardware. This make it safe to call
// free_irq() after the lock is released.
//
    priv.force_quit = 1;
    release_irq = true;
    mcp251x_hw_sleep(spi);
    out_close:
    mcp251x_power_enable(priv.transceiver, 0);
    out_close_candev:
    close_candev(net);
    mutex_unlock(&priv.mcp_lock);
    if (release_irq)
    free_irq(spi.irq, priv);
    return ret;
    }
    static const struct net_device_ops mcp251x_netdev_ops = {
    .ndo_open = mcp251x_open,
    .ndo_stop = mcp251x_stop,
    .ndo_start_xmit = mcp251x_hard_start_xmit,
    };
    static const struct ethtool_ops mcp251x_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
    static const struct of_device_id mcp251x_of_match[] = {
    {
    .compatible	= "microchip,mcp2510",
    .data		= (void *)CAN_MCP251X_MCP2510,
    },
    {
    .compatible	= "microchip,mcp2515",
    .data		= (void *)CAN_MCP251X_MCP2515,
    },
    {
    .compatible	= "microchip,mcp25625",
    .data		= (void *)CAN_MCP251X_MCP25625,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, mcp251x_of_match);
    static const struct spi_device_id mcp251x_id_table[] = {
    {
    .name		= "mcp2510",
    .driver_data	= (kernel_ulong_t)CAN_MCP251X_MCP2510,
    },
    {
    .name		= "mcp2515",
    .driver_data	= (kernel_ulong_t)CAN_MCP251X_MCP2515,
    },
    {
    .name		= "mcp25625",
    .driver_data	= (kernel_ulong_t)CAN_MCP251X_MCP25625,
    },
    { }
    };
    MODULE_DEVICE_TABLE(spi, mcp251x_id_table);
#[no_mangle]
unsafe extern "C" fn mcp251x_can_probe(spi: *mut spi_device) -> c_int {
    static int mcp251x_can_probe(struct spi_device *spi)
    {
    struct net_device *net;
    struct mcp251x_priv *priv;
    struct clk *clk;
    u32 freq;
    int ret;
    clk = devm_clk_get_optional(&spi.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(&spi.dev, PTR_ERR(clk), "Cannot get clock\n");
    freq = clk_get_rate(clk);
    if (freq == 0)
    device_property_read_u32(&spi.dev, "clock-frequency", &freq);
// Sanity check
    if (freq < 1000000 || freq > 25000000)
    return dev_err_probe(&spi.dev, -ERANGE, "clock frequency out of range\n");
// Allocate can/net device
    net = alloc_candev(sizeof(struct mcp251x_priv), TX_ECHO_SKB_MAX);
    if (!net)
    return -ENOMEM;
    ret = clk_prepare_enable(clk);
    if (ret) {
    dev_err_probe(&spi.dev, ret, "Cannot enable clock\n");
    goto out_free;
    }
    net.netdev_ops = &mcp251x_netdev_ops;
    net.ethtool_ops = &mcp251x_ethtool_ops;
    net.flags |= IFF_ECHO;
    priv = netdev_priv(net);
    priv.can.bittiming_const = &mcp251x_bittiming_const;
    priv.can.do_set_mode = mcp251x_do_set_mode;
    priv.can.clock.freq = freq / 2;
    priv.can.ctrlmode_supported = CAN_CTRLMODE_3_SAMPLES |
    CAN_CTRLMODE_LOOPBACK | CAN_CTRLMODE_LISTENONLY;
    priv.model = (enum mcp251x_model)(uintptr_t)spi_get_device_match_data(spi);
    priv.net = net;
    priv.clk = clk;
    spi_set_drvdata(spi, priv);
// Configure the SPI bus
    spi.bits_per_word = 8;
    if (mcp251x_is_2510(spi))
    spi.max_speed_hz = spi.max_speed_hz ? : 5 * 1000 * 1000;
    else
    spi.max_speed_hz = spi.max_speed_hz ? : 10 * 1000 * 1000;
    ret = spi_setup(spi);
    if (ret) {
    dev_err_probe(&spi.dev, ret, "Cannot set up spi\n");
    goto out_clk;
    }
    priv.power = devm_regulator_get_optional(&spi.dev, "vdd");
    priv.transceiver = devm_regulator_get_optional(&spi.dev, "xceiver");
    if ((PTR_ERR(priv.power) == -EPROBE_DEFER) ||
    (PTR_ERR(priv.transceiver) == -EPROBE_DEFER)) {
    ret = -EPROBE_DEFER;
    dev_err_probe(&spi.dev, ret, "supply deferred\n");
    goto out_clk;
    }
    ret = mcp251x_power_enable(priv.power, 1);
    if (ret) {
    dev_err_probe(&spi.dev, ret, "Cannot enable power\n");
    goto out_clk;
    }
    priv.wq = alloc_workqueue("mcp251x_wq",
    WQ_FREEZABLE | WQ_MEM_RECLAIM | WQ_PERCPU,
    0);
    if (!priv.wq) {
    ret = -ENOMEM;
    goto out_clk;
    }
    INIT_WORK(&priv.tx_work, mcp251x_tx_work_handler);
    INIT_WORK(&priv.restart_work, mcp251x_restart_work_handler);
    priv.spi = spi;
    mutex_init(&priv.mcp_lock);
    priv.spi_tx_buf = devm_kzalloc(&spi.dev, SPI_TRANSFER_BUF_LEN,
    GFP_KERNEL);
    if (!priv.spi_tx_buf) {
    ret = -ENOMEM;
    goto error_probe;
    }
    priv.spi_rx_buf = devm_kzalloc(&spi.dev, SPI_TRANSFER_BUF_LEN,
    GFP_KERNEL);
    if (!priv.spi_rx_buf) {
    ret = -ENOMEM;
    goto error_probe;
    }
    SET_NETDEV_DEV(net, &spi.dev);
// Here is OK to not lock the MCP, no one knows about it yet
    ret = mcp251x_hw_probe(spi);
    if (ret) {
    dev_err_probe(&spi.dev, ret, "Cannot initialize MCP%x. Wrong wiring?\n",
    priv.model);
    goto error_probe;
    }
    mcp251x_hw_sleep(spi);
    ret = register_candev(net);
    if (ret) {
    dev_err_probe(&spi.dev, ret, "Cannot register CAN device\n");
    goto error_probe;
    }
    ret = mcp251x_gpio_setup(priv);
    if (ret) {
    dev_err_probe(&spi.dev, ret, "Cannot set up gpios\n");
    goto out_unregister_candev;
    }
    netdev_info(net, "MCP%x successfully initialized.\n", priv.model);
    return 0;
    out_unregister_candev:
    unregister_candev(net);
    error_probe:
    destroy_workqueue(priv.wq);
    priv.wq = core::ptr::null_mut();
    mcp251x_power_enable(priv.power, 0);
    out_clk:
    clk_disable_unprepare(clk);
    out_free:
    free_candev(net);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_can_remove(spi: *mut spi_device) {
    static void mcp251x_can_remove(struct spi_device *spi)
    {
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    struct net_device *net = priv.net;
    unregister_candev(net);
    mcp251x_power_enable(priv.power, 0);
    destroy_workqueue(priv.wq);
    priv.wq = core::ptr::null_mut();
    clk_disable_unprepare(priv.clk);
    free_candev(net);
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_can_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mcp251x_can_suspend(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    struct net_device *net = priv.net;
    priv.force_quit = 1;
    disable_irq(spi.irq);
// Note: at this point neither IST nor workqueues are running.
// open/stop cannot be called anyway so locking is not needed
//
    if (netif_running(net)) {
    netif_device_detach(net);
    mcp251x_hw_sleep(spi);
    mcp251x_power_enable(priv.transceiver, 0);
    priv.after_suspend = AFTER_SUSPEND_UP;
    } else {
    priv.after_suspend = AFTER_SUSPEND_DOWN;
    }
    mcp251x_power_enable(priv.power, 0);
    priv.after_suspend |= AFTER_SUSPEND_POWER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcp251x_can_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mcp251x_can_resume(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct mcp251x_priv *priv = spi_get_drvdata(spi);
    let mut ret: c_int = 0;
    if (priv.after_suspend & AFTER_SUSPEND_POWER) {
    ret = mcp251x_power_enable(priv.power, 1);
    if (ret) {
    dev_err(dev, "failed to restore power: %pe\n", ERR_PTR(ret));
    return ret;
    }
    }
    if (priv.after_suspend & AFTER_SUSPEND_UP) {
    ret = mcp251x_power_enable(priv.transceiver, 1);
    if (ret) {
    dev_err(dev, "failed to restore transceiver power: %pe\n", ERR_PTR(ret));
    if (priv.after_suspend & AFTER_SUSPEND_POWER)
    mcp251x_power_enable(priv.power, 0);
    return ret;
    }
    }
    if (priv.after_suspend & (AFTER_SUSPEND_POWER | AFTER_SUSPEND_UP))
    queue_work(priv.wq, &priv.restart_work);
    else
    priv.after_suspend = 0;
    priv.force_quit = 0;
    enable_irq(spi.irq);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(mcp251x_can_pm_ops, mcp251x_can_suspend,
    mcp251x_can_resume);
    static struct spi_driver mcp251x_can_driver = {
    .driver = {
    .name = DEVICE_NAME,
    .of_match_table = mcp251x_of_match,
    .pm = &mcp251x_can_pm_ops,
    },
    .id_table = mcp251x_id_table,
    .probe = mcp251x_can_probe,
    .remove = mcp251x_can_remove,
    };
    module_spi_driver(mcp251x_can_driver);
    MODULE_AUTHOR("Chris Elston <celston@katalix.com>, "
    "Christian Pellegrin <chripell@evolware.org>");
    MODULE_DESCRIPTION("Microchip 251x/25625 CAN driver");
    MODULE_LICENSE("GPL v2");
