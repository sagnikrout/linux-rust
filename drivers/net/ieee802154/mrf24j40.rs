//! Automatically rewritten from C to Rust
//! Source: drivers/net/ieee802154/mrf24j40.c
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
// Driver for Microchip MRF24J40 802.15.4 Wireless-PAN Networking controller
//
// Copyright (C) 2012 Alan Ott <alan@signal11.us>
// Signal 11 Software
//

// MRF24J40 Short Address Registers
pub const REG_RXMCR: c_uint = 0x00  /* Receive MAC control */;

pub const REG_PANIDL: c_uint = 0x01  /* PAN ID (low) */;
pub const REG_PANIDH: c_uint = 0x02  /* PAN ID (high) */;
pub const REG_SADRL: c_uint = 0x03  /* Short address (low) */;
pub const REG_SADRH: c_uint = 0x04  /* Short address (high) */;
pub const REG_EADR0: c_uint = 0x05  /* Long address (low) (high is EADR7) */;
pub const REG_EADR1: c_uint = 0x06;
pub const REG_EADR2: c_uint = 0x07;
pub const REG_EADR3: c_uint = 0x08;
pub const REG_EADR4: c_uint = 0x09;
pub const REG_EADR5: c_uint = 0x0A;
pub const REG_EADR6: c_uint = 0x0B;
pub const REG_EADR7: c_uint = 0x0C;
pub const REG_RXFLUSH: c_uint = 0x0D;
pub const REG_ORDER: c_uint = 0x10;
pub const REG_TXMCR: c_uint = 0x11  /* Transmit MAC control */;
pub const TXMCR_MIN_BE_SHIFT: c_int = 3;
pub const TXMCR_MIN_BE_MASK: c_uint = 0x18;
pub const TXMCR_CSMA_RETRIES_SHIFT: c_int = 0;
pub const TXMCR_CSMA_RETRIES_MASK: c_uint = 0x07;
pub const REG_ACKTMOUT: c_uint = 0x12;
pub const REG_ESLOTG1: c_uint = 0x13;
pub const REG_SYMTICKL: c_uint = 0x14;
pub const REG_SYMTICKH: c_uint = 0x15;
pub const REG_PACON0: c_uint = 0x16  /* Power Amplifier Control */;
pub const REG_PACON1: c_uint = 0x17  /* Power Amplifier Control */;
pub const REG_PACON2: c_uint = 0x18  /* Power Amplifier Control */;
pub const REG_TXBCON0: c_uint = 0x1A;
pub const REG_TXNCON: c_uint = 0x1B  /* Transmit Normal FIFO Control */;

pub const REG_TXG1CON: c_uint = 0x1C;
pub const REG_TXG2CON: c_uint = 0x1D;
pub const REG_ESLOTG23: c_uint = 0x1E;
pub const REG_ESLOTG45: c_uint = 0x1F;
pub const REG_ESLOTG67: c_uint = 0x20;
pub const REG_TXPEND: c_uint = 0x21;
pub const REG_WAKECON: c_uint = 0x22;
pub const REG_FROMOFFSET: c_uint = 0x23;
pub const REG_TXSTAT: c_uint = 0x24  /* TX MAC Status Register */;
pub const REG_TXBCON1: c_uint = 0x25;
pub const REG_GATECLK: c_uint = 0x26;
pub const REG_TXTIME: c_uint = 0x27;
pub const REG_HSYMTMRL: c_uint = 0x28;
pub const REG_HSYMTMRH: c_uint = 0x29;
pub const REG_SOFTRST: c_uint = 0x2A  /* Soft Reset */;
pub const REG_SECCON0: c_uint = 0x2C;
pub const REG_SECCON1: c_uint = 0x2D;
pub const REG_TXSTBL: c_uint = 0x2E  /* TX Stabilization */;
pub const REG_RXSR: c_uint = 0x30;
pub const REG_INTSTAT: c_uint = 0x31  /* Interrupt Status */;

pub const REG_INTCON: c_uint = 0x32  /* Interrupt Control */;

pub const REG_GPIO: c_uint = 0x33  /* GPIO */;
pub const REG_TRISGPIO: c_uint = 0x34  /* GPIO direction */;
pub const REG_SLPACK: c_uint = 0x35;
pub const REG_RFCTL: c_uint = 0x36  /* RF Control Mode Register */;

pub const REG_SECCR2: c_uint = 0x37;
pub const REG_BBREG0: c_uint = 0x38;
pub const REG_BBREG1: c_uint = 0x39  /* Baseband Registers */;

pub const REG_BBREG2: c_uint = 0x3A  /* */;
pub const BBREG2_CCA_MODE_SHIFT: c_int = 6;
pub const BBREG2_CCA_MODE_MASK: c_uint = 0xc0;
pub const REG_BBREG3: c_uint = 0x3B;
pub const REG_BBREG4: c_uint = 0x3C;
pub const REG_BBREG6: c_uint = 0x3E  /* */;
pub const REG_CCAEDTH: c_uint = 0x3F  /* Energy Detection Threshold */;
// MRF24J40 Long Address Registers
pub const REG_RFCON0: c_uint = 0x200  /* RF Control Registers */;
pub const RFCON0_CH_SHIFT: c_int = 4;
pub const RFCON0_CH_MASK: c_uint = 0xf0;
pub const RFOPT_RECOMMEND: c_int = 3;
pub const REG_RFCON1: c_uint = 0x201;
pub const REG_RFCON2: c_uint = 0x202;
pub const REG_RFCON3: c_uint = 0x203;
pub const TXPWRL_MASK: c_uint = 0xc0;
pub const TXPWRL_SHIFT: c_int = 6;
pub const TXPWRL_30: c_uint = 0x3;
pub const TXPWRL_20: c_uint = 0x2;
pub const TXPWRL_10: c_uint = 0x1;
pub const TXPWRL_0: c_uint = 0x0;
pub const TXPWRS_MASK: c_uint = 0x38;
pub const TXPWRS_SHIFT: c_int = 3;
pub const TXPWRS_6_3: c_uint = 0x7;
pub const TXPWRS_4_9: c_uint = 0x6;
pub const TXPWRS_3_7: c_uint = 0x5;
pub const TXPWRS_2_8: c_uint = 0x4;
pub const TXPWRS_1_9: c_uint = 0x3;
pub const TXPWRS_1_2: c_uint = 0x2;
pub const TXPWRS_0_5: c_uint = 0x1;
pub const TXPWRS_0: c_uint = 0x0;
pub const REG_RFCON5: c_uint = 0x205;
pub const REG_RFCON6: c_uint = 0x206;
pub const REG_RFCON7: c_uint = 0x207;
pub const REG_RFCON8: c_uint = 0x208;
pub const REG_SLPCAL0: c_uint = 0x209;
pub const REG_SLPCAL1: c_uint = 0x20A;
pub const REG_SLPCAL2: c_uint = 0x20B;
pub const REG_RFSTATE: c_uint = 0x20F;
pub const REG_RSSI: c_uint = 0x210;
pub const REG_SLPCON0: c_uint = 0x211  /* Sleep Clock Control Registers */;

pub const REG_SLPCON1: c_uint = 0x220;
pub const REG_WAKETIMEL: c_uint = 0x222  /* Wake-up Time Match Value Low */;
pub const REG_WAKETIMEH: c_uint = 0x223  /* Wake-up Time Match Value High */;
pub const REG_REMCNTL: c_uint = 0x224;
pub const REG_REMCNTH: c_uint = 0x225;
pub const REG_MAINCNT0: c_uint = 0x226;
pub const REG_MAINCNT1: c_uint = 0x227;
pub const REG_MAINCNT2: c_uint = 0x228;
pub const REG_MAINCNT3: c_uint = 0x229;
pub const REG_TESTMODE: c_uint = 0x22F  /* Test mode */;
pub const REG_ASSOEAR0: c_uint = 0x230;
pub const REG_ASSOEAR1: c_uint = 0x231;
pub const REG_ASSOEAR2: c_uint = 0x232;
pub const REG_ASSOEAR3: c_uint = 0x233;
pub const REG_ASSOEAR4: c_uint = 0x234;
pub const REG_ASSOEAR5: c_uint = 0x235;
pub const REG_ASSOEAR6: c_uint = 0x236;
pub const REG_ASSOEAR7: c_uint = 0x237;
pub const REG_ASSOSAR0: c_uint = 0x238;
pub const REG_ASSOSAR1: c_uint = 0x239;
pub const REG_UNONCE0: c_uint = 0x240;
pub const REG_UNONCE1: c_uint = 0x241;
pub const REG_UNONCE2: c_uint = 0x242;
pub const REG_UNONCE3: c_uint = 0x243;
pub const REG_UNONCE4: c_uint = 0x244;
pub const REG_UNONCE5: c_uint = 0x245;
pub const REG_UNONCE6: c_uint = 0x246;
pub const REG_UNONCE7: c_uint = 0x247;
pub const REG_UNONCE8: c_uint = 0x248;
pub const REG_UNONCE9: c_uint = 0x249;
pub const REG_UNONCE10: c_uint = 0x24A;
pub const REG_UNONCE11: c_uint = 0x24B;
pub const REG_UNONCE12: c_uint = 0x24C;
pub const REG_RX_FIFO: c_uint = 0x300  /* Receive FIFO */;
// Device configuration: Only channels 11-26 on page 0 are supported.
pub const MRF24J40_CHAN_MIN: c_int = 11;
pub const MRF24J40_CHAN_MAX: c_int = 26;

    - ((u32)1 << MRF24J40_CHAN_MIN))

    enum mrf24j40_modules { MRF24J40, MRF24J40MA, MRF24J40MC };
// Device Private Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrf24j40 {
    pub spi: *mut spi_device,
    pub hw: *mut ieee802154_hw,
    pub regmap_short: *mut regmap,
    pub regmap_long: *mut regmap,
// for writing txfifo
    pub tx_msg: spi_message,
    pub tx_hdr_buf: [u8; 2],
    pub tx_hdr_trx: spi_transfer,
    pub tx_len_buf: [u8; 2],
    pub tx_len_trx: spi_transfer,
    pub tx_buf_trx: spi_transfer,
    pub tx_skb: *mut sk_buff,
// post transmit message to send frame out
    pub tx_post_msg: spi_message,
    pub tx_post_buf: [u8; 2],
    pub tx_post_trx: spi_transfer,
// for protect/unprotect/read length rxfifo
    pub rx_msg: spi_message,
    pub rx_buf: [u8; 3],
    pub rx_trx: spi_transfer,
// receive handling
    pub rx_buf_msg: spi_message,
    pub rx_addr_buf: [u8; 2],
    pub rx_addr_trx: spi_transfer,
    pub rx_lqi_buf: [u8; 2],
    pub rx_lqi_trx: spi_transfer,
    pub rx_fifo_buf: [u8; RX_FIFO_SIZE],
    pub rx_fifo_buf_trx: spi_transfer,
// isr handling for reading intstat
    pub irq_msg: spi_message,
    pub irq_buf: [u8; 2],
    pub irq_trx: spi_transfer,
}

// regmap information for short address register access
pub const MRF24J40_SHORT_WRITE: c_uint = 0x01;
pub const MRF24J40_SHORT_READ: c_uint = 0x00;
pub const MRF24J40_SHORT_NUMREGS: c_uint = 0x3F;
// regmap information for long address register access
pub const MRF24J40_LONG_ACCESS: c_uint = 0x80;
pub const MRF24J40_LONG_NUMREGS: c_uint = 0x38F;
// Read/Write SPI Commands for Short and Long Address registers.

// The datasheet indicates the theoretical maximum for SCK to be 10MHz
pub const MAX_SPI_SPEED_HZ: c_int = 10000000;

    static bool
    mrf24j40_short_reg_writeable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case REG_RXMCR:
    case REG_PANIDL:
    case REG_PANIDH:
    case REG_SADRL:
    case REG_SADRH:
    case REG_EADR0:
    case REG_EADR1:
    case REG_EADR2:
    case REG_EADR3:
    case REG_EADR4:
    case REG_EADR5:
    case REG_EADR6:
    case REG_EADR7:
    case REG_RXFLUSH:
    case REG_ORDER:
    case REG_TXMCR:
    case REG_ACKTMOUT:
    case REG_ESLOTG1:
    case REG_SYMTICKL:
    case REG_SYMTICKH:
    case REG_PACON0:
    case REG_PACON1:
    case REG_PACON2:
    case REG_TXBCON0:
    case REG_TXNCON:
    case REG_TXG1CON:
    case REG_TXG2CON:
    case REG_ESLOTG23:
    case REG_ESLOTG45:
    case REG_ESLOTG67:
    case REG_TXPEND:
    case REG_WAKECON:
    case REG_FROMOFFSET:
    case REG_TXBCON1:
    case REG_GATECLK:
    case REG_TXTIME:
    case REG_HSYMTMRL:
    case REG_HSYMTMRH:
    case REG_SOFTRST:
    case REG_SECCON0:
    case REG_SECCON1:
    case REG_TXSTBL:
    case REG_RXSR:
    case REG_INTCON:
    case REG_TRISGPIO:
    case REG_GPIO:
    case REG_RFCTL:
    case REG_SECCR2:
    case REG_SLPACK:
    case REG_BBREG0:
    case REG_BBREG1:
    case REG_BBREG2:
    case REG_BBREG3:
    case REG_BBREG4:
    case REG_BBREG6:
    case REG_CCAEDTH:
    return true;
    default:
    return false;
    }
    }
    static bool
    mrf24j40_short_reg_readable(struct device *dev, unsigned int reg)
    {
    bool rc;
// all writeable are also readable
    rc = mrf24j40_short_reg_writeable(dev, reg);
    if (rc)
    return rc;
// readonly regs
    switch (reg) {
    case REG_TXSTAT:
    case REG_INTSTAT:
    return true;
    default:
    return false;
    }
    }
    static bool
    mrf24j40_short_reg_volatile(struct device *dev, unsigned int reg)
    {
// can be changed during runtime
    switch (reg) {
    case REG_TXSTAT:
    case REG_INTSTAT:
    case REG_RXFLUSH:
    case REG_TXNCON:
    case REG_SOFTRST:
    case REG_RFCTL:
    case REG_TXBCON0:
    case REG_TXG1CON:
    case REG_TXG2CON:
    case REG_TXBCON1:
    case REG_SECCON0:
    case REG_RXSR:
    case REG_SLPACK:
    case REG_SECCR2:
    case REG_BBREG6:
// use them in spi_async and regmap so it's volatile
    case REG_BBREG1:
    return true;
    default:
    return false;
    }
    }
    static bool
    mrf24j40_short_reg_precious(struct device *dev, unsigned int reg)
    {
// don't clear irq line on read
    switch (reg) {
    case REG_INTSTAT:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config mrf24j40_short_regmap = {
    .name = "mrf24j40_short",
    .reg_bits = 7,
    .val_bits = 8,
    .pad_bits = 1,
    .write_flag_mask = MRF24J40_SHORT_WRITE,
    .read_flag_mask = MRF24J40_SHORT_READ,
    .cache_type = REGCACHE_MAPLE,
    .max_register = MRF24J40_SHORT_NUMREGS,
    .writeable_reg = mrf24j40_short_reg_writeable,
    .readable_reg = mrf24j40_short_reg_readable,
    .volatile_reg = mrf24j40_short_reg_volatile,
    .precious_reg = mrf24j40_short_reg_precious,
    };
    static bool
    mrf24j40_long_reg_writeable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case REG_RFCON0:
    case REG_RFCON1:
    case REG_RFCON2:
    case REG_RFCON3:
    case REG_RFCON5:
    case REG_RFCON6:
    case REG_RFCON7:
    case REG_RFCON8:
    case REG_SLPCAL2:
    case REG_SLPCON0:
    case REG_SLPCON1:
    case REG_WAKETIMEL:
    case REG_WAKETIMEH:
    case REG_REMCNTL:
    case REG_REMCNTH:
    case REG_MAINCNT0:
    case REG_MAINCNT1:
    case REG_MAINCNT2:
    case REG_MAINCNT3:
    case REG_TESTMODE:
    case REG_ASSOEAR0:
    case REG_ASSOEAR1:
    case REG_ASSOEAR2:
    case REG_ASSOEAR3:
    case REG_ASSOEAR4:
    case REG_ASSOEAR5:
    case REG_ASSOEAR6:
    case REG_ASSOEAR7:
    case REG_ASSOSAR0:
    case REG_ASSOSAR1:
    case REG_UNONCE0:
    case REG_UNONCE1:
    case REG_UNONCE2:
    case REG_UNONCE3:
    case REG_UNONCE4:
    case REG_UNONCE5:
    case REG_UNONCE6:
    case REG_UNONCE7:
    case REG_UNONCE8:
    case REG_UNONCE9:
    case REG_UNONCE10:
    case REG_UNONCE11:
    case REG_UNONCE12:
    return true;
    default:
    return false;
    }
    }
    static bool
    mrf24j40_long_reg_readable(struct device *dev, unsigned int reg)
    {
    bool rc;
// all writeable are also readable
    rc = mrf24j40_long_reg_writeable(dev, reg);
    if (rc)
    return rc;
// readonly regs
    switch (reg) {
    case REG_SLPCAL0:
    case REG_SLPCAL1:
    case REG_RFSTATE:
    case REG_RSSI:
    return true;
    default:
    return false;
    }
    }
    static bool
    mrf24j40_long_reg_volatile(struct device *dev, unsigned int reg)
    {
// can be changed during runtime
    switch (reg) {
    case REG_SLPCAL0:
    case REG_SLPCAL1:
    case REG_SLPCAL2:
    case REG_RFSTATE:
    case REG_RSSI:
    case REG_MAINCNT3:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config mrf24j40_long_regmap = {
    .name = "mrf24j40_long",
    .reg_bits = 11,
    .val_bits = 8,
    .pad_bits = 5,
    .write_flag_mask = MRF24J40_LONG_ACCESS,
    .read_flag_mask = MRF24J40_LONG_ACCESS,
    .cache_type = REGCACHE_MAPLE,
    .max_register = MRF24J40_LONG_NUMREGS,
    .writeable_reg = mrf24j40_long_reg_writeable,
    .readable_reg = mrf24j40_long_reg_readable,
    .volatile_reg = mrf24j40_long_reg_volatile,
    };
    static int mrf24j40_long_regmap_write(void *context, const void *data,
    size_t count)
    {
    struct spi_device *spi = context;
    u8 buf[3];
    if (count > 3)
    return -EINVAL;
// regmap supports read/write mask only in frist byte
// long write access need to set the 12th bit, so we
// make special handling for write.
//
    memcpy(buf, data, count);
    buf[1] |= (1 << 4);
    return spi_write(spi, buf, count);
    }
    static int
    mrf24j40_long_regmap_read(void *context, const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct spi_device *spi = context;
    return spi_write_then_read(spi, reg, reg_size, val, val_size);
    }
    static const struct regmap_bus mrf24j40_long_regmap_bus = {
    .write = mrf24j40_long_regmap_write,
    .read = mrf24j40_long_regmap_read,
    .reg_format_endian_default = REGMAP_ENDIAN_BIG,
    .val_format_endian_default = REGMAP_ENDIAN_BIG,
    };
#[no_mangle]
unsafe extern "C" fn write_tx_buf_complete(context: *mut c_void) {
    static void write_tx_buf_complete(void *context)
    {
    struct mrf24j40 *devrec = context;
    let mut fc: __le16 = ieee802154_get_fc_from_skb(devrec.tx_skb);
    let mut val: u8 = BIT_TXNTRIG;
    int ret;
    if (ieee802154_is_secen(fc))
    val |= BIT_TXNSECEN;
    if (ieee802154_is_ackreq(fc))
    val |= BIT_TXNACKREQ;
    devrec.tx_post_msg.complete = core::ptr::null_mut();
    devrec.tx_post_buf[0] = MRF24J40_WRITESHORT(REG_TXNCON);
    devrec.tx_post_buf[1] = val;
    ret = spi_async(devrec.spi, &devrec.tx_post_msg);
    if (ret)
    dev_err(printdev(devrec), "SPI write Failed for transmit buf\n");
    }
// This function relies on an undocumented write method. Once a write command
    and address is set, as many bytes of data as desired can be clocked into
    the device. The datasheet only shows setting one byte at a time. */
    static int write_tx_buf(struct mrf24j40 *devrec, u16 reg,
    const u8 *data, size_t length)
    {
    u16 cmd;
    int ret;
// Range check the length. 2 bytes are used for the length fields.
    if (length > TX_FIFO_SIZE-2) {
    dev_err(printdev(devrec), "write_tx_buf() was passed too large a buffer. Performing short write.\n");
    length = TX_FIFO_SIZE-2;
    }
    cmd = MRF24J40_WRITELONG(reg);
    devrec.tx_hdr_buf[0] = cmd >> 8 & 0xff;
    devrec.tx_hdr_buf[1] = cmd & 0xff;
    devrec.tx_len_buf[0] = 0x0; /* Header Length. Set to 0 for now. TODO */
    devrec.tx_len_buf[1] = length; /* Total length */
    devrec.tx_buf_trx.tx_buf = data;
    devrec.tx_buf_trx.len = length;
    ret = spi_async(devrec.spi, &devrec.tx_msg);
    if (ret)
    dev_err(printdev(devrec), "SPI write Failed for TX buf\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_tx(hw: *mut ieee802154_hw, skb: *mut sk_buff) -> c_int {
    static int mrf24j40_tx(struct ieee802154_hw *hw, struct sk_buff *skb)
    {
    struct mrf24j40 *devrec = hw.priv;
    dev_dbg(printdev(devrec), "tx packet of %d bytes\n", skb.len);
    devrec.tx_skb = skb;
    return write_tx_buf(devrec, 0x000, skb.data, skb.len);
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_ed(hw: *mut ieee802154_hw, level: *mut u8) -> c_int {
    static int mrf24j40_ed(struct ieee802154_hw *hw, u8 *level)
    {
// TODO:
    pr_warn("mrf24j40: ed not implemented\n");
// level = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_start(hw: *mut ieee802154_hw) -> c_int {
    static int mrf24j40_start(struct ieee802154_hw *hw)
    {
    struct mrf24j40 *devrec = hw.priv;
    dev_dbg(printdev(devrec), "start\n");
// Clear TXNIE and RXIE. Enable interrupts
    return regmap_update_bits(devrec.regmap_short, REG_INTCON,
    BIT_TXNIE | BIT_RXIE | BIT_SECIE, 0);
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_stop(hw: *mut ieee802154_hw) {
    static void mrf24j40_stop(struct ieee802154_hw *hw)
    {
    struct mrf24j40 *devrec = hw.priv;
    dev_dbg(printdev(devrec), "stop\n");
// Set TXNIE and RXIE. Disable Interrupts
    regmap_update_bits(devrec.regmap_short, REG_INTCON,
    BIT_TXNIE | BIT_RXIE, BIT_TXNIE | BIT_RXIE);
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_set_channel(hw: *mut ieee802154_hw, page: u8, channel: u8) -> c_int {
    static int mrf24j40_set_channel(struct ieee802154_hw *hw, u8 page, u8 channel)
    {
    struct mrf24j40 *devrec = hw.priv;
    u8 val;
    int ret;
    dev_dbg(printdev(devrec), "Set Channel %d\n", channel);
    WARN_ON(page != 0);
    WARN_ON(channel < MRF24J40_CHAN_MIN);
    WARN_ON(channel > MRF24J40_CHAN_MAX);
// Set Channel TODO
    val = (channel - 11) << RFCON0_CH_SHIFT | RFOPT_RECOMMEND;
    ret = regmap_update_bits(devrec.regmap_long, REG_RFCON0,
    RFCON0_CH_MASK, val);
    if (ret)
    return ret;
// RF Reset
    ret = regmap_update_bits(devrec.regmap_short, REG_RFCTL, BIT_RFRST,
    BIT_RFRST);
    if (ret)
    return ret;
    ret = regmap_update_bits(devrec.regmap_short, REG_RFCTL, BIT_RFRST, 0);
    if (!ret)
    udelay(SET_CHANNEL_DELAY_US); /* per datasheet */
    return ret;
    }
    static int mrf24j40_filter(struct ieee802154_hw *hw,
    struct ieee802154_hw_addr_filt *filt,
    unsigned long changed)
    {
    struct mrf24j40 *devrec = hw.priv;
    dev_dbg(printdev(devrec), "filter\n");
    if (changed & IEEE802154_AFILT_SADDR_CHANGED) {
// Short Addr
    u8 addrh, addrl;
    addrh = le16_to_cpu(filt.short_addr) >> 8 & 0xff;
    addrl = le16_to_cpu(filt.short_addr) & 0xff;
    regmap_write(devrec.regmap_short, REG_SADRH, addrh);
    regmap_write(devrec.regmap_short, REG_SADRL, addrl);
    dev_dbg(printdev(devrec),
    "Set short addr to %04hx\n", filt.short_addr);
    }
    if (changed & IEEE802154_AFILT_IEEEADDR_CHANGED) {
// Device Address
    u8 i, addr[8];
    memcpy(addr, &filt.ieee_addr, 8);
    for (i = 0; i < 8; i++)
    regmap_write(devrec.regmap_short, REG_EADR0 + i,
    addr[i]);

    pr_debug("Set long addr to: ");
    for (i = 0; i < 8; i++)
    pr_debug("%02hhx ", addr[7 - i]);
    pr_debug("\n");

    }
    if (changed & IEEE802154_AFILT_PANID_CHANGED) {
// PAN ID
    u8 panidl, panidh;
    panidh = le16_to_cpu(filt.pan_id) >> 8 & 0xff;
    panidl = le16_to_cpu(filt.pan_id) & 0xff;
    regmap_write(devrec.regmap_short, REG_PANIDH, panidh);
    regmap_write(devrec.regmap_short, REG_PANIDL, panidl);
    dev_dbg(printdev(devrec), "Set PANID to %04hx\n", filt.pan_id);
    }
    if (changed & IEEE802154_AFILT_PANC_CHANGED) {
// Pan Coordinator
    u8 val;
    int ret;
    if (filt.pan_coord)
    val = BIT_PANCOORD;
    else
    val = 0;
    ret = regmap_update_bits(devrec.regmap_short, REG_RXMCR,
    BIT_PANCOORD, val);
    if (ret)
    return ret;
// REG_SLOTTED is maintained as default (unslotted/CSMA-CA).
// REG_ORDER is maintained as default (no beacon/superframe).
//
    dev_dbg(printdev(devrec), "Set Pan Coord to %s\n",
    filt.pan_coord ? "on" : "off");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_handle_rx_read_buf_unlock(devrec: *mut mrf24j40) {
    static void mrf24j40_handle_rx_read_buf_unlock(struct mrf24j40 *devrec)
    {
    int ret;
// Turn back on reception of packets off the air.
    devrec.rx_msg.complete = core::ptr::null_mut();
    devrec.rx_buf[0] = MRF24J40_WRITESHORT(REG_BBREG1);
    devrec.rx_buf[1] = 0x00; /* CLR RXDECINV */
    ret = spi_async(devrec.spi, &devrec.rx_msg);
    if (ret)
    dev_err(printdev(devrec), "failed to unlock rx buffer\n");
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_handle_rx_read_buf_complete(context: *mut c_void) {
    static void mrf24j40_handle_rx_read_buf_complete(void *context)
    {
    struct mrf24j40 *devrec = context;
    let mut len: u8 = devrec.rx_buf[2];
    u8 rx_local_buf[RX_FIFO_SIZE];
    struct sk_buff *skb;
    memcpy(rx_local_buf, devrec.rx_fifo_buf, len);
    mrf24j40_handle_rx_read_buf_unlock(devrec);
    skb = dev_alloc_skb(IEEE802154_MTU);
    if (!skb) {
    dev_err(printdev(devrec), "failed to allocate skb\n");
    return;
    }
    skb_put_data(skb, rx_local_buf, len);
    ieee802154_rx_irqsafe(devrec.hw, skb, 0);

    print_hex_dump(KERN_DEBUG, "mrf24j40 rx: ", DUMP_PREFIX_OFFSET, 16, 1,
    rx_local_buf, len, 0);
    pr_debug("mrf24j40 rx: lqi: %02hhx rssi: %02hhx\n",
    devrec.rx_lqi_buf[0], devrec.rx_lqi_buf[1]);

    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_handle_rx_read_buf(context: *mut c_void) {
    static void mrf24j40_handle_rx_read_buf(void *context)
    {
    struct mrf24j40 *devrec = context;
    u16 cmd;
    int ret;
// if length is invalid read the full MTU
    if (!ieee802154_is_valid_psdu_len(devrec.rx_buf[2]))
    devrec.rx_buf[2] = IEEE802154_MTU;
    cmd = MRF24J40_READLONG(REG_RX_FIFO + 1);
    devrec.rx_addr_buf[0] = cmd >> 8 & 0xff;
    devrec.rx_addr_buf[1] = cmd & 0xff;
    devrec.rx_fifo_buf_trx.len = devrec.rx_buf[2];
    ret = spi_async(devrec.spi, &devrec.rx_buf_msg);
    if (ret) {
    dev_err(printdev(devrec), "failed to read rx buffer\n");
    mrf24j40_handle_rx_read_buf_unlock(devrec);
    }
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_handle_rx_read_len(context: *mut c_void) {
    static void mrf24j40_handle_rx_read_len(void *context)
    {
    struct mrf24j40 *devrec = context;
    u16 cmd;
    int ret;
// read the length of received frame
    devrec.rx_msg.complete = mrf24j40_handle_rx_read_buf;
    devrec.rx_trx.len = 3;
    cmd = MRF24J40_READLONG(REG_RX_FIFO);
    devrec.rx_buf[0] = cmd >> 8 & 0xff;
    devrec.rx_buf[1] = cmd & 0xff;
    ret = spi_async(devrec.spi, &devrec.rx_msg);
    if (ret) {
    dev_err(printdev(devrec), "failed to read rx buffer length\n");
    mrf24j40_handle_rx_read_buf_unlock(devrec);
    }
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_handle_rx(devrec: *mut mrf24j40) -> c_int {
    static int mrf24j40_handle_rx(struct mrf24j40 *devrec)
    {
// Turn off reception of packets off the air. This prevents the
// device from overwriting the buffer while we're reading it.
//
    devrec.rx_msg.complete = mrf24j40_handle_rx_read_len;
    devrec.rx_trx.len = 2;
    devrec.rx_buf[0] = MRF24J40_WRITESHORT(REG_BBREG1);
    devrec.rx_buf[1] = BIT_RXDECINV; /* SET RXDECINV */
    return spi_async(devrec.spi, &devrec.rx_msg);
    }
    static int
    mrf24j40_csma_params(struct ieee802154_hw *hw, u8 min_be, u8 max_be,
    u8 retries)
    {
    struct mrf24j40 *devrec = hw.priv;
    u8 val;
// min_be
    val = min_be << TXMCR_MIN_BE_SHIFT;
// csma backoffs
    val |= retries << TXMCR_CSMA_RETRIES_SHIFT;
    return regmap_update_bits(devrec.regmap_short, REG_TXMCR,
    TXMCR_MIN_BE_MASK | TXMCR_CSMA_RETRIES_MASK,
    val);
    }
    static int mrf24j40_set_cca_mode(struct ieee802154_hw *hw,
    const struct wpan_phy_cca *cca)
    {
    struct mrf24j40 *devrec = hw.priv;
    u8 val;
// mapping 802.15.4 to driver spec
    switch (cca.mode) {
    case NL802154_CCA_ENERGY:
    val = 2;
    break;
    case NL802154_CCA_CARRIER:
    val = 1;
    break;
    case NL802154_CCA_ENERGY_CARRIER:
    switch (cca.opt) {
    case NL802154_CCA_OPT_ENERGY_CARRIER_AND:
    val = 3;
    break;
    default:
    return -EINVAL;
    }
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(devrec.regmap_short, REG_BBREG2,
    BBREG2_CCA_MODE_MASK,
    val << BBREG2_CCA_MODE_SHIFT);
    }
// array for representing ed levels
    static const s32 mrf24j40_ed_levels[] = {
    -9000, -8900, -8800, -8700, -8600, -8500, -8400, -8300, -8200, -8100,
    -8000, -7900, -7800, -7700, -7600, -7500, -7400, -7300, -7200, -7100,
    -7000, -6900, -6800, -6700, -6600, -6500, -6400, -6300, -6200, -6100,
    -6000, -5900, -5800, -5700, -5600, -5500, -5400, -5300, -5200, -5100,
    -5000, -4900, -4800, -4700, -4600, -4500, -4400, -4300, -4200, -4100,
    -4000, -3900, -3800, -3700, -3600, -3500
    };
// map ed levels to register value
    static const s32 mrf24j40_ed_levels_map[][2] = {
    { -9000, 0 }, { -8900, 1 }, { -8800, 2 }, { -8700, 5 }, { -8600, 9 },
    { -8500, 13 }, { -8400, 18 }, { -8300, 23 }, { -8200, 27 },
    { -8100, 32 }, { -8000, 37 }, { -7900, 43 }, { -7800, 48 },
    { -7700, 53 }, { -7600, 58 }, { -7500, 63 }, { -7400, 68 },
    { -7300, 73 }, { -7200, 78 }, { -7100, 83 }, { -7000, 89 },
    { -6900, 95 }, { -6800, 100 }, { -6700, 107 }, { -6600, 111 },
    { -6500, 117 }, { -6400, 121 }, { -6300, 125 }, { -6200, 129 },
    { -6100, 133 },	{ -6000, 138 }, { -5900, 143 }, { -5800, 148 },
    { -5700, 153 }, { -5600, 159 },	{ -5500, 165 }, { -5400, 170 },
    { -5300, 176 }, { -5200, 183 }, { -5100, 188 }, { -5000, 193 },
    { -4900, 198 }, { -4800, 203 }, { -4700, 207 }, { -4600, 212 },
    { -4500, 216 }, { -4400, 221 }, { -4300, 225 }, { -4200, 228 },
    { -4100, 233 }, { -4000, 239 }, { -3900, 245 }, { -3800, 250 },
    { -3700, 253 }, { -3600, 254 }, { -3500, 255 },
    };
#[no_mangle]
unsafe extern "C" fn mrf24j40_set_cca_ed_level(hw: *mut ieee802154_hw, mbm: i32) -> c_int {
    static int mrf24j40_set_cca_ed_level(struct ieee802154_hw *hw, s32 mbm)
    {
    struct mrf24j40 *devrec = hw.priv;
    int i;
    for (i = 0; i < ARRAY_SIZE(mrf24j40_ed_levels_map); i++) {
    if (mrf24j40_ed_levels_map[i][0] == mbm)
    return regmap_write(devrec.regmap_short, REG_CCAEDTH,
    mrf24j40_ed_levels_map[i][1]);
    }
    return -EINVAL;
    }
    static const s32 mrf24j40ma_powers[] = {
    0, -50, -120, -190, -280, -370, -490, -630, -1000, -1050, -1120, -1190,
    -1280, -1370, -1490, -1630, -2000, -2050, -2120, -2190, -2280, -2370,
    -2490, -2630, -3000, -3050, -3120, -3190, -3280, -3370, -3490, -3630,
    };
#[no_mangle]
unsafe extern "C" fn mrf24j40_set_txpower(hw: *mut ieee802154_hw, mbm: i32) -> c_int {
    static int mrf24j40_set_txpower(struct ieee802154_hw *hw, s32 mbm)
    {
    struct mrf24j40 *devrec = hw.priv;
    s32 small_scale;
    u8 val;
    if (0 >= mbm && mbm > -1000) {
    val = TXPWRL_0 << TXPWRL_SHIFT;
    small_scale = mbm;
    } else if (-1000 >= mbm && mbm > -2000) {
    val = TXPWRL_10 << TXPWRL_SHIFT;
    small_scale = mbm + 1000;
    } else if (-2000 >= mbm && mbm > -3000) {
    val = TXPWRL_20 << TXPWRL_SHIFT;
    small_scale = mbm + 2000;
    } else if (-3000 >= mbm && mbm > -4000) {
    val = TXPWRL_30 << TXPWRL_SHIFT;
    small_scale = mbm + 3000;
    } else {
    return -EINVAL;
    }
    switch (small_scale) {
    case 0:
    val |= (TXPWRS_0 << TXPWRS_SHIFT);
    break;
    case -50:
    val |= (TXPWRS_0_5 << TXPWRS_SHIFT);
    break;
    case -120:
    val |= (TXPWRS_1_2 << TXPWRS_SHIFT);
    break;
    case -190:
    val |= (TXPWRS_1_9 << TXPWRS_SHIFT);
    break;
    case -280:
    val |= (TXPWRS_2_8 << TXPWRS_SHIFT);
    break;
    case -370:
    val |= (TXPWRS_3_7 << TXPWRS_SHIFT);
    break;
    case -490:
    val |= (TXPWRS_4_9 << TXPWRS_SHIFT);
    break;
    case -630:
    val |= (TXPWRS_6_3 << TXPWRS_SHIFT);
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(devrec.regmap_long, REG_RFCON3,
    TXPWRL_MASK | TXPWRS_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_set_promiscuous_mode(hw: *mut ieee802154_hw, on: bool) -> c_int {
    static int mrf24j40_set_promiscuous_mode(struct ieee802154_hw *hw, bool on)
    {
    struct mrf24j40 *devrec = hw.priv;
    int ret;
    if (on) {
// set PROMI, ERRPKT and NOACKRSP
    ret = regmap_update_bits(devrec.regmap_short, REG_RXMCR,
    BIT_PROMI | BIT_ERRPKT | BIT_NOACKRSP,
    BIT_PROMI | BIT_ERRPKT | BIT_NOACKRSP);
    } else {
// clear PROMI, ERRPKT and NOACKRSP
    ret = regmap_update_bits(devrec.regmap_short, REG_RXMCR,
    BIT_PROMI | BIT_ERRPKT | BIT_NOACKRSP,
    0);
    }
    return ret;
    }
    static const struct ieee802154_ops mrf24j40_ops = {
    .owner = THIS_MODULE,
    .xmit_async = mrf24j40_tx,
    .ed = mrf24j40_ed,
    .start = mrf24j40_start,
    .stop = mrf24j40_stop,
    .set_channel = mrf24j40_set_channel,
    .set_hw_addr_filt = mrf24j40_filter,
    .set_csma_params = mrf24j40_csma_params,
    .set_cca_mode = mrf24j40_set_cca_mode,
    .set_cca_ed_level = mrf24j40_set_cca_ed_level,
    .set_txpower = mrf24j40_set_txpower,
    .set_promiscuous_mode = mrf24j40_set_promiscuous_mode,
    };
#[no_mangle]
unsafe extern "C" fn mrf24j40_intstat_complete(context: *mut c_void) {
    static void mrf24j40_intstat_complete(void *context)
    {
    struct mrf24j40 *devrec = context;
    let mut intstat: u8 = devrec.irq_buf[1];
    enable_irq(devrec.spi.irq);
// Ignore Rx security decryption
    if (intstat & BIT_SECIF)
    regmap_write_async(devrec.regmap_short, REG_SECCON0,
    BIT_SECIGNORE);
// Check for TX complete
    if (intstat & BIT_TXNIF)
    ieee802154_xmit_complete(devrec.hw, devrec.tx_skb, false);
// Check for Rx
    if (intstat & BIT_RXIF)
    mrf24j40_handle_rx(devrec);
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mrf24j40_isr(int irq, void *data)
    {
    struct mrf24j40 *devrec = data;
    int ret;
    disable_irq_nosync(irq);
    devrec.irq_buf[0] = MRF24J40_READSHORT(REG_INTSTAT);
    devrec.irq_buf[1] = 0;
// Read the interrupt status
    ret = spi_async(devrec.spi, &devrec.irq_msg);
    if (ret) {
    enable_irq(irq);
    return IRQ_NONE;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_hw_init(devrec: *mut mrf24j40) -> c_int {
    static int mrf24j40_hw_init(struct mrf24j40 *devrec)
    {
    u32 irq_type;
    int ret;
// Initialize the device.
    From datasheet section 3.2: Initialization. */
    ret = regmap_write(devrec.regmap_short, REG_SOFTRST, 0x07);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_short, REG_PACON2, 0x98);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_short, REG_TXSTBL, 0x95);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_long, REG_RFCON0, 0x03);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_long, REG_RFCON1, 0x01);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_long, REG_RFCON2, 0x80);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_long, REG_RFCON6, 0x90);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_long, REG_RFCON7, 0x80);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_long, REG_RFCON8, 0x10);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_long, REG_SLPCON1, 0x21);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_short, REG_BBREG2, 0x80);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_short, REG_CCAEDTH, 0x60);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_short, REG_BBREG6, 0x40);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_short, REG_RFCTL, 0x04);
    if (ret)
    goto err_ret;
    ret = regmap_write(devrec.regmap_short, REG_RFCTL, 0x0);
    if (ret)
    goto err_ret;
    udelay(192);
// Set RX Mode. RXMCR<1:0>: 0x0 normal, 0x1 promisc, 0x2 error
    ret = regmap_update_bits(devrec.regmap_short, REG_RXMCR, 0x03, 0x00);
    if (ret)
    goto err_ret;
    if (spi_get_device_id(devrec.spi).driver_data == MRF24J40MC) {
// Enable external amplifier.
// From MRF24J40MC datasheet section 1.3: Operation.
//
    regmap_update_bits(devrec.regmap_long, REG_TESTMODE, 0x07,
    0x07);
// Set GPIO3 as output.
    regmap_update_bits(devrec.regmap_short, REG_TRISGPIO, 0x08,
    0x08);
// Set GPIO3 HIGH to enable U5 voltage regulator
    regmap_update_bits(devrec.regmap_short, REG_GPIO, 0x08, 0x08);
// Reduce TX pwr to meet FCC requirements.
// From MRF24J40MC datasheet section 3.1.1
//
    regmap_write(devrec.regmap_long, REG_RFCON3, 0x28);
    }
    irq_type = irq_get_trigger_type(devrec.spi.irq);
    if (irq_type == IRQ_TYPE_EDGE_RISING ||
    irq_type == IRQ_TYPE_EDGE_FALLING)
    dev_warn(&devrec.spi.dev,
    "Using edge triggered irq's are not recommended, because it can cause races and result in a non-functional driver!\n");
    switch (irq_type) {
    case IRQ_TYPE_EDGE_RISING:
    case IRQ_TYPE_LEVEL_HIGH:
// set interrupt polarity to rising
    ret = regmap_update_bits(devrec.regmap_long, REG_SLPCON0,
    BIT_INTEDGE, BIT_INTEDGE);
    if (ret)
    goto err_ret;
    break;
    default:
// default is falling edge
    break;
    }
    return 0;
    err_ret:
    return ret;
    }
    static void
    mrf24j40_setup_tx_spi_messages(struct mrf24j40 *devrec)
    {
    spi_message_init(&devrec.tx_msg);
    devrec.tx_msg.context = devrec;
    devrec.tx_msg.complete = write_tx_buf_complete;
    devrec.tx_hdr_trx.len = 2;
    devrec.tx_hdr_trx.tx_buf = devrec.tx_hdr_buf;
    spi_message_add_tail(&devrec.tx_hdr_trx, &devrec.tx_msg);
    devrec.tx_len_trx.len = 2;
    devrec.tx_len_trx.tx_buf = devrec.tx_len_buf;
    spi_message_add_tail(&devrec.tx_len_trx, &devrec.tx_msg);
    spi_message_add_tail(&devrec.tx_buf_trx, &devrec.tx_msg);
    spi_message_init(&devrec.tx_post_msg);
    devrec.tx_post_msg.context = devrec;
    devrec.tx_post_trx.len = 2;
    devrec.tx_post_trx.tx_buf = devrec.tx_post_buf;
    spi_message_add_tail(&devrec.tx_post_trx, &devrec.tx_post_msg);
    }
    static void
    mrf24j40_setup_rx_spi_messages(struct mrf24j40 *devrec)
    {
    spi_message_init(&devrec.rx_msg);
    devrec.rx_msg.context = devrec;
    devrec.rx_trx.len = 2;
    devrec.rx_trx.tx_buf = devrec.rx_buf;
    devrec.rx_trx.rx_buf = devrec.rx_buf;
    spi_message_add_tail(&devrec.rx_trx, &devrec.rx_msg);
    spi_message_init(&devrec.rx_buf_msg);
    devrec.rx_buf_msg.context = devrec;
    devrec.rx_buf_msg.complete = mrf24j40_handle_rx_read_buf_complete;
    devrec.rx_addr_trx.len = 2;
    devrec.rx_addr_trx.tx_buf = devrec.rx_addr_buf;
    spi_message_add_tail(&devrec.rx_addr_trx, &devrec.rx_buf_msg);
    devrec.rx_fifo_buf_trx.rx_buf = devrec.rx_fifo_buf;
    spi_message_add_tail(&devrec.rx_fifo_buf_trx, &devrec.rx_buf_msg);
    devrec.rx_lqi_trx.len = 2;
    devrec.rx_lqi_trx.rx_buf = devrec.rx_lqi_buf;
    spi_message_add_tail(&devrec.rx_lqi_trx, &devrec.rx_buf_msg);
    }
    static void
    mrf24j40_setup_irq_spi_messages(struct mrf24j40 *devrec)
    {
    spi_message_init(&devrec.irq_msg);
    devrec.irq_msg.context = devrec;
    devrec.irq_msg.complete = mrf24j40_intstat_complete;
    devrec.irq_trx.len = 2;
    devrec.irq_trx.tx_buf = devrec.irq_buf;
    devrec.irq_trx.rx_buf = devrec.irq_buf;
    spi_message_add_tail(&devrec.irq_trx, &devrec.irq_msg);
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_phy_setup(devrec: *mut mrf24j40) {
    static void  mrf24j40_phy_setup(struct mrf24j40 *devrec)
    {
    ieee802154_random_extended_addr(&devrec.hw.phy.perm_extended_addr);
    devrec.hw.phy.current_channel = 11;
// mrf24j40 supports max_minbe 0 - 3
    devrec.hw.phy.supported.max_minbe = 3;
// datasheet doesn't say anything about max_be, but we have min_be
// So we assume the max_be default.
//
    devrec.hw.phy.supported.min_maxbe = 5;
    devrec.hw.phy.supported.max_maxbe = 5;
    devrec.hw.phy.cca.mode = NL802154_CCA_CARRIER;
    devrec.hw.phy.supported.cca_modes = BIT(NL802154_CCA_ENERGY) |
    BIT(NL802154_CCA_CARRIER) |
    BIT(NL802154_CCA_ENERGY_CARRIER);
    devrec.hw.phy.supported.cca_opts = BIT(NL802154_CCA_OPT_ENERGY_CARRIER_AND);
    devrec.hw.phy.cca_ed_level = -6900;
    devrec.hw.phy.supported.cca_ed_levels = mrf24j40_ed_levels;
    devrec.hw.phy.supported.cca_ed_levels_size = ARRAY_SIZE(mrf24j40_ed_levels);
    switch (spi_get_device_id(devrec.spi).driver_data) {
    case MRF24J40:
    case MRF24J40MA:
    devrec.hw.phy.supported.tx_powers = mrf24j40ma_powers;
    devrec.hw.phy.supported.tx_powers_size = ARRAY_SIZE(mrf24j40ma_powers);
    devrec.hw.phy.flags |= WPAN_PHY_FLAG_TXPOWER;
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_probe(spi: *mut spi_device) -> c_int {
    static int mrf24j40_probe(struct spi_device *spi)
    {
    let mut ret: c_int = -ENOMEM, irq_type;
    struct ieee802154_hw *hw;
    struct mrf24j40 *devrec;
    dev_info(&spi.dev, "probe(). IRQ: %d\n", spi.irq);
// Register with the 802154 subsystem
    hw = ieee802154_alloc_hw(sizeof(*devrec), &mrf24j40_ops);
    if (!hw)
    goto err_ret;
    devrec = hw.priv;
    devrec.spi = spi;
    spi_set_drvdata(spi, devrec);
    devrec.hw = hw;
    devrec.hw.parent = &spi.dev;
    devrec.hw.phy.supported.channels[0] = CHANNEL_MASK;
    devrec.hw.flags = IEEE802154_HW_TX_OMIT_CKSUM | IEEE802154_HW_AFILT |
    IEEE802154_HW_CSMA_PARAMS |
    IEEE802154_HW_PROMISCUOUS;
    devrec.hw.phy.flags = WPAN_PHY_FLAG_CCA_MODE |
    WPAN_PHY_FLAG_CCA_ED_LEVEL;
    mrf24j40_setup_tx_spi_messages(devrec);
    mrf24j40_setup_rx_spi_messages(devrec);
    mrf24j40_setup_irq_spi_messages(devrec);
    devrec.regmap_short = devm_regmap_init_spi(spi,
    &mrf24j40_short_regmap);
    if (IS_ERR(devrec.regmap_short)) {
    ret = PTR_ERR(devrec.regmap_short);
    dev_err(&spi.dev, "Failed to allocate short register map: %d\n",
    ret);
    goto err_register_device;
    }
    devrec.regmap_long = devm_regmap_init(&spi.dev,
    &mrf24j40_long_regmap_bus,
    spi, &mrf24j40_long_regmap);
    if (IS_ERR(devrec.regmap_long)) {
    ret = PTR_ERR(devrec.regmap_long);
    dev_err(&spi.dev, "Failed to allocate long register map: %d\n",
    ret);
    goto err_register_device;
    }
    if (spi.max_speed_hz > MAX_SPI_SPEED_HZ) {
    dev_warn(&spi.dev, "spi clock above possible maximum: %d",
    MAX_SPI_SPEED_HZ);
    ret = -EINVAL;
    goto err_register_device;
    }
    ret = mrf24j40_hw_init(devrec);
    if (ret)
    goto err_register_device;
    mrf24j40_phy_setup(devrec);
// request IRQF_TRIGGER_LOW as fallback default
    irq_type = irq_get_trigger_type(spi.irq);
    if (!irq_type)
    irq_type = IRQF_TRIGGER_LOW;
    ret = devm_request_irq(&spi.dev, spi.irq, mrf24j40_isr,
    irq_type, dev_name(&spi.dev), devrec);
    if (ret) {
    dev_err(printdev(devrec), "Unable to get IRQ");
    goto err_register_device;
    }
    dev_dbg(printdev(devrec), "registered mrf24j40\n");
    ret = ieee802154_register_hw(devrec.hw);
    if (ret)
    goto err_register_device;
    return 0;
    err_register_device:
    ieee802154_free_hw(devrec.hw);
    err_ret:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mrf24j40_remove(spi: *mut spi_device) {
    static void mrf24j40_remove(struct spi_device *spi)
    {
    struct mrf24j40 *devrec = spi_get_drvdata(spi);
    dev_dbg(printdev(devrec), "remove\n");
    ieee802154_unregister_hw(devrec.hw);
    ieee802154_free_hw(devrec.hw);
// TODO: Will ieee802154_free_device() wait until ->xmit() is
// complete?
    }
    static const struct of_device_id mrf24j40_of_match[] = {
    { .compatible = "microchip,mrf24j40", .data = (void *)MRF24J40 },
    { .compatible = "microchip,mrf24j40ma", .data = (void *)MRF24J40MA },
    { .compatible = "microchip,mrf24j40mc", .data = (void *)MRF24J40MC },
    { },
    };
    MODULE_DEVICE_TABLE(of, mrf24j40_of_match);
    static const struct spi_device_id mrf24j40_ids[] = {
    { "mrf24j40", MRF24J40 },
    { "mrf24j40ma", MRF24J40MA },
    { "mrf24j40mc", MRF24J40MC },
    { },
    };
    MODULE_DEVICE_TABLE(spi, mrf24j40_ids);
    static struct spi_driver mrf24j40_driver = {
    .driver = {
    .of_match_table = mrf24j40_of_match,
    .name = "mrf24j40",
    },
    .id_table = mrf24j40_ids,
    .probe = mrf24j40_probe,
    .remove = mrf24j40_remove,
    };
    module_spi_driver(mrf24j40_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alan Ott");
    MODULE_DESCRIPTION("MRF24J40 SPI 802.15.4 Controller Driver");
