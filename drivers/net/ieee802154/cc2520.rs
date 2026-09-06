//! Automatically rewritten from C to Rust
//! Source: drivers/net/ieee802154/cc2520.c
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
// Driver for TI CC2520 802.15.4 Wireless-PAN Networking controller
//
// Copyright (C) 2014 Varka Bhadram <varkab@cdac.in>
// Md.Jamal Mohiuddin <mjmohiuddin@cdac.in>
// P Sowjanya <sowjanyap@cdac.in>
//

pub const SPI_COMMAND_BUFFER: c_int = 3;
pub const HIGH: c_int = 1;
pub const LOW: c_int = 0;
pub const STATE_IDLE: c_int = 0;
pub const RSSI_VALID: c_int = 0;
pub const RSSI_OFFSET: c_int = 78;
pub const CC2520_RAM_SIZE: c_int = 640;
pub const CC2520_FIFO_SIZE: c_int = 128;
pub const CC2520RAM_TXFIFO: c_uint = 0x100;
pub const CC2520RAM_RXFIFO: c_uint = 0x180;
pub const CC2520RAM_IEEEADDR: c_uint = 0x3EA;
pub const CC2520RAM_PANID: c_uint = 0x3F2;
pub const CC2520RAM_SHORTADDR: c_uint = 0x3F4;
pub const CC2520_FREG_MASK: c_uint = 0x3F;
// status byte values

// IEEE-802.15.4 defined constants (2.4 GHz logical channels)
pub const CC2520_MINCHANNEL: c_int = 11;
pub const CC2520_MAXCHANNEL: c_int = 26;
pub const CC2520_CHANNEL_SPACING: c_int = 5;
// command strobes
pub const CC2520_CMD_SNOP: c_uint = 0x00;
pub const CC2520_CMD_IBUFLD: c_uint = 0x02;
pub const CC2520_CMD_SIBUFEX: c_uint = 0x03;
pub const CC2520_CMD_SSAMPLECCA: c_uint = 0x04;
pub const CC2520_CMD_SRES: c_uint = 0x0f;
pub const CC2520_CMD_MEMORY_MASK: c_uint = 0x0f;
pub const CC2520_CMD_MEMORY_READ: c_uint = 0x10;
pub const CC2520_CMD_MEMORY_WRITE: c_uint = 0x20;
pub const CC2520_CMD_RXBUF: c_uint = 0x30;
pub const CC2520_CMD_RXBUFCP: c_uint = 0x38;
pub const CC2520_CMD_RXBUFMOV: c_uint = 0x32;
pub const CC2520_CMD_TXBUF: c_uint = 0x3A;
pub const CC2520_CMD_TXBUFCP: c_uint = 0x3E;
pub const CC2520_CMD_RANDOM: c_uint = 0x3C;
pub const CC2520_CMD_SXOSCON: c_uint = 0x40;
pub const CC2520_CMD_STXCAL: c_uint = 0x41;
pub const CC2520_CMD_SRXON: c_uint = 0x42;
pub const CC2520_CMD_STXON: c_uint = 0x43;
pub const CC2520_CMD_STXONCCA: c_uint = 0x44;
pub const CC2520_CMD_SRFOFF: c_uint = 0x45;
pub const CC2520_CMD_SXOSCOFF: c_uint = 0x46;
pub const CC2520_CMD_SFLUSHRX: c_uint = 0x47;
pub const CC2520_CMD_SFLUSHTX: c_uint = 0x48;
pub const CC2520_CMD_SACK: c_uint = 0x49;
pub const CC2520_CMD_SACKPEND: c_uint = 0x4A;
pub const CC2520_CMD_SNACK: c_uint = 0x4B;
pub const CC2520_CMD_SRXMASKBITSET: c_uint = 0x4C;
pub const CC2520_CMD_SRXMASKBITCLR: c_uint = 0x4D;
pub const CC2520_CMD_RXMASKAND: c_uint = 0x4E;
pub const CC2520_CMD_RXMASKOR: c_uint = 0x4F;
pub const CC2520_CMD_MEMCP: c_uint = 0x50;
pub const CC2520_CMD_MEMCPR: c_uint = 0x52;
pub const CC2520_CMD_MEMXCP: c_uint = 0x54;
pub const CC2520_CMD_MEMXWR: c_uint = 0x56;
pub const CC2520_CMD_BCLR: c_uint = 0x58;
pub const CC2520_CMD_BSET: c_uint = 0x59;
pub const CC2520_CMD_CTR_UCTR: c_uint = 0x60;
pub const CC2520_CMD_CBCMAC: c_uint = 0x64;
pub const CC2520_CMD_UCBCMAC: c_uint = 0x66;
pub const CC2520_CMD_CCM: c_uint = 0x68;
pub const CC2520_CMD_UCCM: c_uint = 0x6A;
pub const CC2520_CMD_ECB: c_uint = 0x70;
pub const CC2520_CMD_ECBO: c_uint = 0x72;
pub const CC2520_CMD_ECBX: c_uint = 0x74;
pub const CC2520_CMD_INC: c_uint = 0x78;
pub const CC2520_CMD_ABORT: c_uint = 0x7F;
pub const CC2520_CMD_REGISTER_READ: c_uint = 0x80;
pub const CC2520_CMD_REGISTER_WRITE: c_uint = 0xC0;
// status registers
pub const CC2520_CHIPID: c_uint = 0x40;
pub const CC2520_VERSION: c_uint = 0x42;
pub const CC2520_EXTCLOCK: c_uint = 0x44;
pub const CC2520_MDMCTRL0: c_uint = 0x46;
pub const CC2520_MDMCTRL1: c_uint = 0x47;
pub const CC2520_FREQEST: c_uint = 0x48;
pub const CC2520_RXCTRL: c_uint = 0x4A;
pub const CC2520_FSCTRL: c_uint = 0x4C;
pub const CC2520_FSCAL0: c_uint = 0x4E;
pub const CC2520_FSCAL1: c_uint = 0x4F;
pub const CC2520_FSCAL2: c_uint = 0x50;
pub const CC2520_FSCAL3: c_uint = 0x51;
pub const CC2520_AGCCTRL0: c_uint = 0x52;
pub const CC2520_AGCCTRL1: c_uint = 0x53;
pub const CC2520_AGCCTRL2: c_uint = 0x54;
pub const CC2520_AGCCTRL3: c_uint = 0x55;
pub const CC2520_ADCTEST0: c_uint = 0x56;
pub const CC2520_ADCTEST1: c_uint = 0x57;
pub const CC2520_ADCTEST2: c_uint = 0x58;
pub const CC2520_MDMTEST0: c_uint = 0x5A;
pub const CC2520_MDMTEST1: c_uint = 0x5B;
pub const CC2520_DACTEST0: c_uint = 0x5C;
pub const CC2520_DACTEST1: c_uint = 0x5D;
pub const CC2520_ATEST: c_uint = 0x5E;
pub const CC2520_DACTEST2: c_uint = 0x5F;
pub const CC2520_PTEST0: c_uint = 0x60;
pub const CC2520_PTEST1: c_uint = 0x61;
pub const CC2520_RESERVED: c_uint = 0x62;
pub const CC2520_DPUBIST: c_uint = 0x7A;
pub const CC2520_ACTBIST: c_uint = 0x7C;
pub const CC2520_RAMBIST: c_uint = 0x7E;
// frame registers
pub const CC2520_FRMFILT0: c_uint = 0x00;
pub const CC2520_FRMFILT1: c_uint = 0x01;
pub const CC2520_SRCMATCH: c_uint = 0x02;
pub const CC2520_SRCSHORTEN0: c_uint = 0x04;
pub const CC2520_SRCSHORTEN1: c_uint = 0x05;
pub const CC2520_SRCSHORTEN2: c_uint = 0x06;
pub const CC2520_SRCEXTEN0: c_uint = 0x08;
pub const CC2520_SRCEXTEN1: c_uint = 0x09;
pub const CC2520_SRCEXTEN2: c_uint = 0x0A;
pub const CC2520_FRMCTRL0: c_uint = 0x0C;
pub const CC2520_FRMCTRL1: c_uint = 0x0D;
pub const CC2520_RXENABLE0: c_uint = 0x0E;
pub const CC2520_RXENABLE1: c_uint = 0x0F;
pub const CC2520_EXCFLAG0: c_uint = 0x10;
pub const CC2520_EXCFLAG1: c_uint = 0x11;
pub const CC2520_EXCFLAG2: c_uint = 0x12;
pub const CC2520_EXCMASKA0: c_uint = 0x14;
pub const CC2520_EXCMASKA1: c_uint = 0x15;
pub const CC2520_EXCMASKA2: c_uint = 0x16;
pub const CC2520_EXCMASKB0: c_uint = 0x18;
pub const CC2520_EXCMASKB1: c_uint = 0x19;
pub const CC2520_EXCMASKB2: c_uint = 0x1A;
pub const CC2520_EXCBINDX0: c_uint = 0x1C;
pub const CC2520_EXCBINDX1: c_uint = 0x1D;
pub const CC2520_EXCBINDY0: c_uint = 0x1E;
pub const CC2520_EXCBINDY1: c_uint = 0x1F;
pub const CC2520_GPIOCTRL0: c_uint = 0x20;
pub const CC2520_GPIOCTRL1: c_uint = 0x21;
pub const CC2520_GPIOCTRL2: c_uint = 0x22;
pub const CC2520_GPIOCTRL3: c_uint = 0x23;
pub const CC2520_GPIOCTRL4: c_uint = 0x24;
pub const CC2520_GPIOCTRL5: c_uint = 0x25;
pub const CC2520_GPIOPOLARITY: c_uint = 0x26;
pub const CC2520_GPIOCTRL: c_uint = 0x28;
pub const CC2520_DPUCON: c_uint = 0x2A;
pub const CC2520_DPUSTAT: c_uint = 0x2C;
pub const CC2520_FREQCTRL: c_uint = 0x2E;
pub const CC2520_FREQTUNE: c_uint = 0x2F;
pub const CC2520_TXPOWER: c_uint = 0x30;
pub const CC2520_TXCTRL: c_uint = 0x31;
pub const CC2520_FSMSTAT0: c_uint = 0x32;
pub const CC2520_FSMSTAT1: c_uint = 0x33;
pub const CC2520_FIFOPCTRL: c_uint = 0x34;
pub const CC2520_FSMCTRL: c_uint = 0x35;
pub const CC2520_CCACTRL0: c_uint = 0x36;
pub const CC2520_CCACTRL1: c_uint = 0x37;
pub const CC2520_RSSI: c_uint = 0x38;
pub const CC2520_RSSISTAT: c_uint = 0x39;
pub const CC2520_RXFIRST: c_uint = 0x3C;
pub const CC2520_RXFIFOCNT: c_uint = 0x3E;
pub const CC2520_TXFIFOCNT: c_uint = 0x3F;
// CC2520_FRMFILT0

// CC2520_FRMCTRL0

// CC2520_FRMCTRL1

// Driver private information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc2520_private {
    pub /: *mut *mut *mut spi_device spi; / SPI device structure,
    pub /: *mut *mut *mut ieee802154_hw hw; / IEEE-802.15.4 device,
    pub /: *mut *mut *mut u8 buf; / SPI TX/Rx data buffer,
    pub /: *mut *mut mutex buffer_mutex; / SPI buffer mutex,
    pub /: *mut *mut bool is_tx; / Flag for sync b/w Tx and Rx,
    pub /: *mut *mut bool amplified; / Flag for CC2591,
    pub /: *mut *mut *mut gpio_desc fifo_pin; / FIFO GPIO pin number,
    pub /: *mut *mut work_fifop_irqwork;/ Workqueue for FIFOP,
    pub is_tx*/: *mut *mut spinlock_t lock; / Lock for,
    pub /: *mut *mut completion tx_complete; / Work completion for Tx,
    pub /: *mut *mut bool promiscuous; / Flag for promiscuous mode,
}

// Generic Functions
    static int
    cc2520_cmd_strobe(struct cc2520_private *priv, u8 cmd)
    {
    int ret;
    struct spi_message msg;
    struct spi_transfer xfer = {
    .len = 0,
    .tx_buf = priv.buf,
    .rx_buf = priv.buf,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    mutex_lock(&priv.buffer_mutex);
    priv.buf[xfer.len++] = cmd;
    dev_vdbg(&priv.spi.dev,
    "command strobe buf[0] = %02x\n",
    priv.buf[0]);
    ret = spi_sync(priv.spi, &msg);
    dev_vdbg(&priv.spi.dev,
    "buf[0] = %02x\n", priv.buf[0]);
    mutex_unlock(&priv.buffer_mutex);
    return ret;
    }
    static int
    cc2520_get_status(struct cc2520_private *priv, u8 *status)
    {
    int ret;
    struct spi_message msg;
    struct spi_transfer xfer = {
    .len = 0,
    .tx_buf = priv.buf,
    .rx_buf = priv.buf,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    mutex_lock(&priv.buffer_mutex);
    priv.buf[xfer.len++] = CC2520_CMD_SNOP;
    dev_vdbg(&priv.spi.dev,
    "get status command buf[0] = %02x\n", priv.buf[0]);
    ret = spi_sync(priv.spi, &msg);
    if (!ret)
// status = priv->buf[0];
    dev_vdbg(&priv.spi.dev,
    "buf[0] = %02x\n", priv.buf[0]);
    mutex_unlock(&priv.buffer_mutex);
    return ret;
    }
    static int
    cc2520_write_register(struct cc2520_private *priv, u8 reg, u8 value)
    {
    int status;
    struct spi_message msg;
    struct spi_transfer xfer = {
    .len = 0,
    .tx_buf = priv.buf,
    .rx_buf = priv.buf,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    mutex_lock(&priv.buffer_mutex);
    if (reg <= CC2520_FREG_MASK) {
    priv.buf[xfer.len++] = CC2520_CMD_REGISTER_WRITE | reg;
    priv.buf[xfer.len++] = value;
    } else {
    priv.buf[xfer.len++] = CC2520_CMD_MEMORY_WRITE;
    priv.buf[xfer.len++] = reg;
    priv.buf[xfer.len++] = value;
    }
    status = spi_sync(priv.spi, &msg);
    if (msg.status)
    status = msg.status;
    mutex_unlock(&priv.buffer_mutex);
    return status;
    }
    static int
    cc2520_write_ram(struct cc2520_private *priv, u16 reg, u8 len, u8 *data)
    {
    int status;
    struct spi_message msg;
    struct spi_transfer xfer_head = {
    .len        = 0,
    .tx_buf        = priv.buf,
    .rx_buf        = priv.buf,
    };
    struct spi_transfer xfer_buf = {
    .len = len,
    .tx_buf = data,
    };
    mutex_lock(&priv.buffer_mutex);
    priv.buf[xfer_head.len++] = (CC2520_CMD_MEMORY_WRITE |
    ((reg >> 8) & 0xff));
    priv.buf[xfer_head.len++] = reg & 0xff;
    spi_message_init(&msg);
    spi_message_add_tail(&xfer_head, &msg);
    spi_message_add_tail(&xfer_buf, &msg);
    status = spi_sync(priv.spi, &msg);
    dev_dbg(&priv.spi.dev, "spi status = %d\n", status);
    if (msg.status)
    status = msg.status;
    mutex_unlock(&priv.buffer_mutex);
    return status;
    }
    static int
    cc2520_read_register(struct cc2520_private *priv, u8 reg, u8 *data)
    {
    int status;
    struct spi_message msg;
    struct spi_transfer xfer1 = {
    .len = 0,
    .tx_buf = priv.buf,
    .rx_buf = priv.buf,
    };
    struct spi_transfer xfer2 = {
    .len = 1,
    .rx_buf = data,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer1, &msg);
    spi_message_add_tail(&xfer2, &msg);
    mutex_lock(&priv.buffer_mutex);
    priv.buf[xfer1.len++] = CC2520_CMD_MEMORY_READ;
    priv.buf[xfer1.len++] = reg;
    status = spi_sync(priv.spi, &msg);
    dev_dbg(&priv.spi.dev,
    "spi status = %d\n", status);
    if (msg.status)
    status = msg.status;
    mutex_unlock(&priv.buffer_mutex);
    return status;
    }
    static int
    cc2520_write_txfifo(struct cc2520_private *priv, u8 pkt_len, u8 *data, u8 len)
    {
    int status;
// length byte must include FCS even
// if it is calculated in the hardware
//
    let mut len_byte: c_int = pkt_len;
    struct spi_message msg;
    struct spi_transfer xfer_head = {
    .len = 0,
    .tx_buf = priv.buf,
    .rx_buf = priv.buf,
    };
    struct spi_transfer xfer_len = {
    .len = 1,
    .tx_buf = &len_byte,
    };
    struct spi_transfer xfer_buf = {
    .len = len,
    .tx_buf = data,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer_head, &msg);
    spi_message_add_tail(&xfer_len, &msg);
    spi_message_add_tail(&xfer_buf, &msg);
    mutex_lock(&priv.buffer_mutex);
    priv.buf[xfer_head.len++] = CC2520_CMD_TXBUF;
    dev_vdbg(&priv.spi.dev,
    "TX_FIFO cmd buf[0] = %02x\n", priv.buf[0]);
    status = spi_sync(priv.spi, &msg);
    dev_vdbg(&priv.spi.dev, "status = %d\n", status);
    if (msg.status)
    status = msg.status;
    dev_vdbg(&priv.spi.dev, "status = %d\n", status);
    dev_vdbg(&priv.spi.dev, "buf[0] = %02x\n", priv.buf[0]);
    mutex_unlock(&priv.buffer_mutex);
    return status;
    }
    static int
    cc2520_read_rxfifo(struct cc2520_private *priv, u8 *data, u8 len)
    {
    int status;
    struct spi_message msg;
    struct spi_transfer xfer_head = {
    .len = 0,
    .tx_buf = priv.buf,
    .rx_buf = priv.buf,
    };
    struct spi_transfer xfer_buf = {
    .len = len,
    .rx_buf = data,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer_head, &msg);
    spi_message_add_tail(&xfer_buf, &msg);
    mutex_lock(&priv.buffer_mutex);
    priv.buf[xfer_head.len++] = CC2520_CMD_RXBUF;
    dev_vdbg(&priv.spi.dev, "read rxfifo buf[0] = %02x\n", priv.buf[0]);
    dev_vdbg(&priv.spi.dev, "buf[1] = %02x\n", priv.buf[1]);
    status = spi_sync(priv.spi, &msg);
    dev_vdbg(&priv.spi.dev, "status = %d\n", status);
    if (msg.status)
    status = msg.status;
    dev_vdbg(&priv.spi.dev, "status = %d\n", status);
    dev_vdbg(&priv.spi.dev,
    "return status buf[0] = %02x\n", priv.buf[0]);
    dev_vdbg(&priv.spi.dev, "length buf[1] = %02x\n", priv.buf[1]);
    mutex_unlock(&priv.buffer_mutex);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn cc2520_start(hw: *mut ieee802154_hw) -> c_int {
    static int cc2520_start(struct ieee802154_hw *hw)
    {
    return cc2520_cmd_strobe(hw.priv, CC2520_CMD_SRXON);
    }
#[no_mangle]
unsafe extern "C" fn cc2520_stop(hw: *mut ieee802154_hw) {
    static void cc2520_stop(struct ieee802154_hw *hw)
    {
    cc2520_cmd_strobe(hw.priv, CC2520_CMD_SRFOFF);
    }
    static int
    cc2520_tx(struct ieee802154_hw *hw, struct sk_buff *skb)
    {
    struct cc2520_private *priv = hw.priv;
    unsigned long flags;
    int rc;
    let mut status: u8 = 0;
    u8 pkt_len;
// In promiscuous mode we disable AUTOCRC so we can get the raw CRC
// values on RX. This means we need to manually add the CRC on TX.
//
    if (priv.promiscuous) {
    let mut crc: u16 = crc_ccitt(0, skb.data, skb.len);
    put_unaligned_le16(crc, skb_put(skb, 2));
    pkt_len = skb.len;
    } else {
    pkt_len = skb.len + 2;
    }
    rc = cc2520_cmd_strobe(priv, CC2520_CMD_SFLUSHTX);
    if (rc)
    goto err_tx;
    rc = cc2520_write_txfifo(priv, pkt_len, skb.data, skb.len);
    if (rc)
    goto err_tx;
    rc = cc2520_get_status(priv, &status);
    if (rc)
    goto err_tx;
    if (status & CC2520_STATUS_TX_UNDERFLOW) {
    rc = -EINVAL;
    dev_err(&priv.spi.dev, "cc2520 tx underflow exception\n");
    goto err_tx;
    }
    spin_lock_irqsave(&priv.lock, flags);
    WARN_ON(priv.is_tx);
    priv.is_tx = 1;
    spin_unlock_irqrestore(&priv.lock, flags);
    rc = cc2520_cmd_strobe(priv, CC2520_CMD_STXONCCA);
    if (rc)
    goto err;
    rc = wait_for_completion_interruptible(&priv.tx_complete);
    if (rc < 0)
    goto err;
    cc2520_cmd_strobe(priv, CC2520_CMD_SFLUSHTX);
    cc2520_cmd_strobe(priv, CC2520_CMD_SRXON);
    return rc;
    err:
    spin_lock_irqsave(&priv.lock, flags);
    priv.is_tx = 0;
    spin_unlock_irqrestore(&priv.lock, flags);
    err_tx:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cc2520_rx(priv: *mut cc2520_private) -> c_int {
    static int cc2520_rx(struct cc2520_private *priv)
    {
    let mut len: u8 = 0, lqi = 0, bytes = 1;
    struct sk_buff *skb;
// Read single length byte from the radio.
    cc2520_read_rxfifo(priv, &len, bytes);
    if (!ieee802154_is_valid_psdu_len(len)) {
// Corrupted frame received, clear frame buffer by
// reading entire buffer.
//
    dev_dbg(&priv.spi.dev, "corrupted frame received\n");
    len = IEEE802154_MTU;
    }
    skb = dev_alloc_skb(len);
    if (!skb)
    return -ENOMEM;
    if (cc2520_read_rxfifo(priv, skb_put(skb, len), len)) {
    dev_dbg(&priv.spi.dev, "frame reception failed\n");
    kfree_skb(skb);
    return -EINVAL;
    }
// In promiscuous mode, we configure the radio to include the
// CRC (AUTOCRC==0) and we pass on the packet unconditionally. If not
// in promiscuous mode, we check the CRC here, but leave the
// RSSI/LQI/CRC_OK bytes as they will get removed in the mac layer.
//
    if (!priv.promiscuous) {
    bool crc_ok;
// Check if the CRC is valid. With AUTOCRC set, the most
// significant bit of the last byte returned from the CC2520
// is CRC_OK flag. See section 20.3.4 of the datasheet.
//
    crc_ok = skb.data[len - 1] & BIT(7);
// If we failed CRC drop the packet in the driver layer.
    if (!crc_ok) {
    dev_dbg(&priv.spi.dev, "CRC check failed\n");
    kfree_skb(skb);
    return -EINVAL;
    }
// To calculate LQI, the lower 7 bits of the last byte (the
// correlation value provided by the radio) must be scaled to
// the range 0-255. According to section 20.6, the correlation
// value ranges from 50-110. Ideally this would be calibrated
// per hardware design, but we use roughly the datasheet values
// to get close enough while avoiding floating point.
//
    lqi = skb.data[len - 1] & 0x7f;
    if (lqi < 50)
    lqi = 50;
#[no_mangle]
pub unsafe extern "C" fn if(113: lqi >) -> else {
    else if (lqi > 113)
    lqi = 113;
    lqi = (lqi - 50) * 4;
    }
    ieee802154_rx_irqsafe(priv.hw, skb, lqi);
    dev_vdbg(&priv.spi.dev, "RXFIFO: %x %x\n", len, lqi);
    return 0;
    }
    static int
    cc2520_ed(struct ieee802154_hw *hw, u8 *level)
    {
    struct cc2520_private *priv = hw.priv;
    let mut status: u8 = 0xff;
    u8 rssi;
    int ret;
    ret = cc2520_read_register(priv, CC2520_RSSISTAT, &status);
    if (ret)
    return ret;
    if (status != RSSI_VALID)
    return -EINVAL;
    ret = cc2520_read_register(priv, CC2520_RSSI, &rssi);
    if (ret)
    return ret;
// level = RSSI(rssi) - OFFSET [dBm] : offset is 76dBm
// level = rssi - RSSI_OFFSET;
    return 0;
    }
    static int
    cc2520_set_channel(struct ieee802154_hw *hw, u8 page, u8 channel)
    {
    struct cc2520_private *priv = hw.priv;
    int ret;
    dev_dbg(&priv.spi.dev, "trying to set channel\n");
    WARN_ON(page != 0);
    WARN_ON(channel < CC2520_MINCHANNEL);
    WARN_ON(channel > CC2520_MAXCHANNEL);
    ret = cc2520_write_register(priv, CC2520_FREQCTRL,
    11 + 5 * (channel - 11));
    return ret;
    }
    static int
    cc2520_filter(struct ieee802154_hw *hw,
    struct ieee802154_hw_addr_filt *filt, unsigned long changed)
    {
    struct cc2520_private *priv = hw.priv;
    let mut ret: c_int = 0;
    if (changed & IEEE802154_AFILT_PANID_CHANGED) {
    let mut panid: u16 = le16_to_cpu(filt.pan_id);
    dev_vdbg(&priv.spi.dev, "%s called for pan id\n", __func__);
    ret = cc2520_write_ram(priv, CC2520RAM_PANID,
    sizeof(panid), (u8 *)&panid);
    }
    if (changed & IEEE802154_AFILT_IEEEADDR_CHANGED) {
    dev_vdbg(&priv.spi.dev,
    "%s called for IEEE addr\n", __func__);
    ret = cc2520_write_ram(priv, CC2520RAM_IEEEADDR,
    sizeof(filt.ieee_addr),
    (u8 *)&filt.ieee_addr);
    }
    if (changed & IEEE802154_AFILT_SADDR_CHANGED) {
    let mut addr: u16 = le16_to_cpu(filt.short_addr);
    dev_vdbg(&priv.spi.dev, "%s called for saddr\n", __func__);
    ret = cc2520_write_ram(priv, CC2520RAM_SHORTADDR,
    sizeof(addr), (u8 *)&addr);
    }
    if (changed & IEEE802154_AFILT_PANC_CHANGED) {
    u8 frmfilt0;
    dev_vdbg(&priv.spi.dev,
    "%s called for panc change\n", __func__);
    cc2520_read_register(priv, CC2520_FRMFILT0, &frmfilt0);
    if (filt.pan_coord)
    frmfilt0 |= FRMFILT0_PAN_COORDINATOR;
    else
    frmfilt0 &= ~FRMFILT0_PAN_COORDINATOR;
    ret = cc2520_write_register(priv, CC2520_FRMFILT0, frmfilt0);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cc2520_set_tx_power(priv: *mut cc2520_private, mbm: i32) -> c_int {
    static inline int cc2520_set_tx_power(struct cc2520_private *priv, s32 mbm)
    {
    u8 power;
    switch (mbm) {
    case 500:
    power = 0xF7;
    break;
    case 300:
    power = 0xF2;
    break;
    case 200:
    power = 0xAB;
    break;
    case 100:
    power = 0x13;
    break;
    case 0:
    power = 0x32;
    break;
    case -200:
    power = 0x81;
    break;
    case -400:
    power = 0x88;
    break;
    case -700:
    power = 0x2C;
    break;
    case -1800:
    power = 0x03;
    break;
    default:
    return -EINVAL;
    }
    return cc2520_write_register(priv, CC2520_TXPOWER, power);
    }
    static inline int cc2520_cc2591_set_tx_power(struct cc2520_private *priv,
    s32 mbm)
    {
    u8 power;
    switch (mbm) {
    case 1700:
    power = 0xF9;
    break;
    case 1600:
    power = 0xF0;
    break;
    case 1400:
    power = 0xA0;
    break;
    case 1100:
    power = 0x2C;
    break;
    case -100:
    power = 0x03;
    break;
    case -800:
    power = 0x01;
    break;
    default:
    return -EINVAL;
    }
    return cc2520_write_register(priv, CC2520_TXPOWER, power);
    }
pub const CC2520_MAX_TX_POWERS: c_uint = 0x8;
    static const s32 cc2520_powers[CC2520_MAX_TX_POWERS + 1] = {
    500, 300, 200, 100, 0, -200, -400, -700, -1800,
    };
pub const CC2520_CC2591_MAX_TX_POWERS: c_uint = 0x5;
    static const s32 cc2520_cc2591_powers[CC2520_CC2591_MAX_TX_POWERS + 1] = {
    1700, 1600, 1400, 1100, -100, -800,
    };
    static int
    cc2520_set_txpower(struct ieee802154_hw *hw, s32 mbm)
    {
    struct cc2520_private *priv = hw.priv;
    if (!priv.amplified)
    return cc2520_set_tx_power(priv, mbm);
    return cc2520_cc2591_set_tx_power(priv, mbm);
    }
    static int
    cc2520_set_promiscuous_mode(struct ieee802154_hw *hw, bool on)
    {
    struct cc2520_private *priv = hw.priv;
    u8 frmfilt0;
    dev_dbg(&priv.spi.dev, "%s : mode %d\n", __func__, on);
    priv.promiscuous = on;
    cc2520_read_register(priv, CC2520_FRMFILT0, &frmfilt0);
    if (on) {
// Disable automatic ACK, automatic CRC, and frame filtering.
    cc2520_write_register(priv, CC2520_FRMCTRL0, 0);
    frmfilt0 &= ~FRMFILT0_FRAME_FILTER_EN;
    } else {
    cc2520_write_register(priv, CC2520_FRMCTRL0, FRMCTRL0_AUTOACK |
    FRMCTRL0_AUTOCRC);
    frmfilt0 |= FRMFILT0_FRAME_FILTER_EN;
    }
    return cc2520_write_register(priv, CC2520_FRMFILT0, frmfilt0);
    }
    static const struct ieee802154_ops cc2520_ops = {
    .owner = THIS_MODULE,
    .start = cc2520_start,
    .stop = cc2520_stop,
    .xmit_sync = cc2520_tx,
    .ed = cc2520_ed,
    .set_channel = cc2520_set_channel,
    .set_hw_addr_filt = cc2520_filter,
    .set_txpower = cc2520_set_txpower,
    .set_promiscuous_mode = cc2520_set_promiscuous_mode,
    };
#[no_mangle]
unsafe extern "C" fn cc2520_register(priv: *mut cc2520_private) -> c_int {
    static int cc2520_register(struct cc2520_private *priv)
    {
    let mut ret: c_int = -ENOMEM;
    priv.hw = ieee802154_alloc_hw(sizeof(*priv), &cc2520_ops);
    if (!priv.hw)
    goto err_ret;
    priv.hw.priv = priv;
    priv.hw.parent = &priv.spi.dev;
    priv.hw.extra_tx_headroom = 0;
    ieee802154_random_extended_addr(&priv.hw.phy.perm_extended_addr);
// We do support only 2.4 Ghz
    priv.hw.phy.supported.channels[0] = 0x7FFF800;
    priv.hw.flags = IEEE802154_HW_TX_OMIT_CKSUM | IEEE802154_HW_AFILT |
    IEEE802154_HW_PROMISCUOUS;
    priv.hw.phy.flags = WPAN_PHY_FLAG_TXPOWER;
    if (!priv.amplified) {
    priv.hw.phy.supported.tx_powers = cc2520_powers;
    priv.hw.phy.supported.tx_powers_size = ARRAY_SIZE(cc2520_powers);
    priv.hw.phy.transmit_power = priv.hw.phy.supported.tx_powers[4];
    } else {
    priv.hw.phy.supported.tx_powers = cc2520_cc2591_powers;
    priv.hw.phy.supported.tx_powers_size = ARRAY_SIZE(cc2520_cc2591_powers);
    priv.hw.phy.transmit_power = priv.hw.phy.supported.tx_powers[0];
    }
    priv.hw.phy.current_channel = 11;
    dev_vdbg(&priv.spi.dev, "registered cc2520\n");
    ret = ieee802154_register_hw(priv.hw);
    if (ret)
    goto err_free_device;
    return 0;
    err_free_device:
    ieee802154_free_hw(priv.hw);
    err_ret:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cc2520_fifop_irqwork(work: *mut work_struct) {
    static void cc2520_fifop_irqwork(struct work_struct *work)
    {
    struct cc2520_private *priv
    = container_of(work, struct cc2520_private, fifop_irqwork);
    dev_dbg(&priv.spi.dev, "fifop interrupt received\n");
    if (gpiod_get_value(priv.fifo_pin))
    cc2520_rx(priv);
    else
    dev_dbg(&priv.spi.dev, "rxfifo overflow\n");
    cc2520_cmd_strobe(priv, CC2520_CMD_SFLUSHRX);
    cc2520_cmd_strobe(priv, CC2520_CMD_SFLUSHRX);
    }
#[no_mangle]
unsafe extern "C" fn cc2520_fifop_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cc2520_fifop_isr(int irq, void *data)
    {
    struct cc2520_private *priv = data;
    schedule_work(&priv.fifop_irqwork);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cc2520_sfd_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cc2520_sfd_isr(int irq, void *data)
    {
    struct cc2520_private *priv = data;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    if (priv.is_tx) {
    priv.is_tx = 0;
    spin_unlock_irqrestore(&priv.lock, flags);
    dev_dbg(&priv.spi.dev, "SFD for TX\n");
    complete(&priv.tx_complete);
    } else {
    spin_unlock_irqrestore(&priv.lock, flags);
    dev_dbg(&priv.spi.dev, "SFD for RX\n");
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cc2520_hw_init(priv: *mut cc2520_private) -> c_int {
    static int cc2520_hw_init(struct cc2520_private *priv)
    {
    let mut status: u8 = 0, state = 0xff;
    int ret;
    let mut timeout: c_int = 100;
    ret = cc2520_read_register(priv, CC2520_FSMSTAT1, &state);
    if (ret)
    goto err_ret;
    if (state != STATE_IDLE)
    return -EINVAL;
    do {
    ret = cc2520_get_status(priv, &status);
    if (ret)
    goto err_ret;
    if (timeout-- <= 0) {
    dev_err(&priv.spi.dev, "oscillator start failed!\n");
    return -ETIMEDOUT;
    }
    udelay(1);
    } while (!(status & CC2520_STATUS_XOSC32M_STABLE));
    dev_vdbg(&priv.spi.dev, "oscillator brought up\n");
// If the CC2520 is connected to a CC2591 amplifier, we must both
// configure GPIOs on the CC2520 to correctly configure the CC2591
// and change a couple settings of the CC2520 to work with the
// amplifier. See section 8 page 17 of TI application note AN065.
// http://www.ti.com/lit/an/swra229a/swra229a.pdf
//
    if (priv.amplified) {
    ret = cc2520_write_register(priv, CC2520_AGCCTRL1, 0x16);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_GPIOCTRL0, 0x46);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_GPIOCTRL5, 0x47);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_GPIOPOLARITY, 0x1e);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_TXCTRL, 0xc1);
    if (ret)
    goto err_ret;
    } else {
    ret = cc2520_write_register(priv, CC2520_AGCCTRL1, 0x11);
    if (ret)
    goto err_ret;
    }
// Registers default value: section 28.1 in Datasheet
// Set the CCA threshold to -50 dBm. This seems to have been copied
// from the TinyOS CC2520 driver and is much higher than the -84 dBm
// threshold suggested in the datasheet.
//
    ret = cc2520_write_register(priv, CC2520_CCACTRL0, 0x1A);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_MDMCTRL0, 0x85);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_MDMCTRL1, 0x14);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_RXCTRL, 0x3f);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_FSCTRL, 0x5a);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_FSCAL1, 0x2b);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_ADCTEST0, 0x10);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_ADCTEST1, 0x0e);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_ADCTEST2, 0x03);
    if (ret)
    goto err_ret;
// Configure registers correctly for this driver.
    ret = cc2520_write_register(priv, CC2520_FRMCTRL1,
    FRMCTRL1_SET_RXENMASK_ON_TX |
    FRMCTRL1_IGNORE_TX_UNDERF);
    if (ret)
    goto err_ret;
    ret = cc2520_write_register(priv, CC2520_FIFOPCTRL, 127);
    if (ret)
    goto err_ret;
    return 0;
    err_ret:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cc2520_probe(spi: *mut spi_device) -> c_int {
    static int cc2520_probe(struct spi_device *spi)
    {
    struct cc2520_private *priv;
    struct gpio_desc *fifop;
    struct gpio_desc *cca;
    struct gpio_desc *sfd;
    struct gpio_desc *reset;
    struct gpio_desc *vreg;
    int ret;
    priv = devm_kzalloc(&spi.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spi_set_drvdata(spi, priv);
// CC2591 front end for CC2520
// Assumption that CC2591 is not connected
    priv.amplified = false;
    if (device_property_read_bool(&spi.dev, "amplified"))
    priv.amplified = true;
    priv.spi = spi;
    priv.buf = devm_kzalloc(&spi.dev,
    SPI_COMMAND_BUFFER, GFP_KERNEL);
    if (!priv.buf)
    return -ENOMEM;
    mutex_init(&priv.buffer_mutex);
    INIT_WORK(&priv.fifop_irqwork, cc2520_fifop_irqwork);
    spin_lock_init(&priv.lock);
    init_completion(&priv.tx_complete);
// Request all the gpio's
    priv.fifo_pin = devm_gpiod_get(&spi.dev, "fifo", GPIOD_IN);
    if (IS_ERR(priv.fifo_pin)) {
    dev_err(&spi.dev, "fifo gpio is not valid\n");
    ret = PTR_ERR(priv.fifo_pin);
    goto err_hw_init;
    }
    cca = devm_gpiod_get(&spi.dev, "cca", GPIOD_IN);
    if (IS_ERR(cca)) {
    dev_err(&spi.dev, "cca gpio is not valid\n");
    ret = PTR_ERR(cca);
    goto err_hw_init;
    }
    fifop = devm_gpiod_get(&spi.dev, "fifop", GPIOD_IN);
    if (IS_ERR(fifop)) {
    dev_err(&spi.dev, "fifop gpio is not valid\n");
    ret = PTR_ERR(fifop);
    goto err_hw_init;
    }
    sfd = devm_gpiod_get(&spi.dev, "sfd", GPIOD_IN);
    if (IS_ERR(sfd)) {
    dev_err(&spi.dev, "sfd gpio is not valid\n");
    ret = PTR_ERR(sfd);
    goto err_hw_init;
    }
    reset = devm_gpiod_get(&spi.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(reset)) {
    dev_err(&spi.dev, "reset gpio is not valid\n");
    ret = PTR_ERR(reset);
    goto err_hw_init;
    }
    vreg = devm_gpiod_get(&spi.dev, "vreg", GPIOD_OUT_LOW);
    if (IS_ERR(vreg)) {
    dev_err(&spi.dev, "vreg gpio is not valid\n");
    ret = PTR_ERR(vreg);
    goto err_hw_init;
    }
    gpiod_set_value(vreg, HIGH);
    usleep_range(100, 150);
    gpiod_set_value(reset, HIGH);
    usleep_range(200, 250);
    ret = cc2520_hw_init(priv);
    if (ret)
    goto err_hw_init;
// Set up fifop interrupt
    ret = devm_request_irq(&spi.dev,
    gpiod_to_irq(fifop),
    cc2520_fifop_isr,
    IRQF_TRIGGER_RISING,
    dev_name(&spi.dev),
    priv);
    if (ret) {
    dev_err(&spi.dev, "could not get fifop irq\n");
    goto err_hw_init;
    }
// Set up sfd interrupt
    ret = devm_request_irq(&spi.dev,
    gpiod_to_irq(sfd),
    cc2520_sfd_isr,
    IRQF_TRIGGER_FALLING,
    dev_name(&spi.dev),
    priv);
    if (ret) {
    dev_err(&spi.dev, "could not get sfd irq\n");
    goto err_hw_init;
    }
    ret = cc2520_register(priv);
    if (ret)
    goto err_hw_init;
    return 0;
    err_hw_init:
    mutex_destroy(&priv.buffer_mutex);
    flush_work(&priv.fifop_irqwork);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cc2520_remove(spi: *mut spi_device) {
    static void cc2520_remove(struct spi_device *spi)
    {
    struct cc2520_private *priv = spi_get_drvdata(spi);
    mutex_destroy(&priv.buffer_mutex);
    flush_work(&priv.fifop_irqwork);
    ieee802154_unregister_hw(priv.hw);
    ieee802154_free_hw(priv.hw);
    }
    static const struct spi_device_id cc2520_ids[] = {
    {"cc2520", },
    {},
    };
    MODULE_DEVICE_TABLE(spi, cc2520_ids);
    static const struct of_device_id cc2520_of_ids[] = {
    {.compatible = "ti,cc2520", },
    {},
    };
    MODULE_DEVICE_TABLE(of, cc2520_of_ids);
// SPI driver structure
    static struct spi_driver cc2520_driver = {
    .driver = {
    .name = "cc2520",
    .of_match_table = cc2520_of_ids,
    },
    .id_table = cc2520_ids,
    .probe = cc2520_probe,
    .remove = cc2520_remove,
    };
    module_spi_driver(cc2520_driver);
    MODULE_AUTHOR("Varka Bhadram <varkab@cdac.in>");
    MODULE_DESCRIPTION("CC2520 Transceiver Driver");
    MODULE_LICENSE("GPL v2");
