//! Automatically rewritten from C to Rust
//! Source: drivers/net/ieee802154/adf7242.c
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
// Analog Devices ADF7242 Low-Power IEEE 802.15.4 Transceiver
//
// Copyright 2009-2017 Analog Devices Inc.
//
// https://www.analog.com/ADF7242
//

pub const MAX_POLL_LOOPS: c_int = 200;
// All Registers
pub const REG_EXT_CTRL: c_uint = 0x100	/* RW External LNA/PA and internal PA control */;
pub const REG_TX_FSK_TEST: c_uint = 0x101	/* RW TX FSK test mode configuration */;
pub const REG_CCA1: c_uint = 0x105	/* RW RSSI threshold for CCA */;
pub const REG_CCA2: c_uint = 0x106	/* RW CCA mode configuration */;
pub const REG_BUFFERCFG: c_uint = 0x107	/* RW RX_BUFFER overwrite control */;
pub const REG_PKT_CFG: c_uint = 0x108	/* RW FCS evaluation configuration */;
pub const REG_DELAYCFG0: c_uint = 0x109	/* RW RC_RX command to SFD or sync word delay */;
pub const REG_DELAYCFG1: c_uint = 0x10A	/* RW RC_TX command to TX state */;
pub const REG_DELAYCFG2: c_uint = 0x10B	/* RW Mac delay extension */;
pub const REG_SYNC_WORD0: c_uint = 0x10C	/* RW sync word bits [7:0] of [23:0]  */;
pub const REG_SYNC_WORD1: c_uint = 0x10D	/* RW sync word bits [15:8] of [23:0]  */;
pub const REG_SYNC_WORD2: c_uint = 0x10E	/* RW sync word bits [23:16] of [23:0]	*/;
pub const REG_SYNC_CONFIG: c_uint = 0x10F	/* RW sync word configuration */;
pub const REG_RC_CFG: c_uint = 0x13E	/* RW RX / TX packet configuration */;
pub const REG_RC_VAR44: c_uint = 0x13F	/* RW RESERVED */;
pub const REG_CH_FREQ0: c_uint = 0x300	/* RW Channel Frequency Settings - Low */;
pub const REG_CH_FREQ1: c_uint = 0x301	/* RW Channel Frequency Settings - Middle */;
pub const REG_CH_FREQ2: c_uint = 0x302	/* RW Channel Frequency Settings - High */;
pub const REG_TX_FD: c_uint = 0x304	/* RW TX Frequency Deviation Register */;
pub const REG_DM_CFG0: c_uint = 0x305	/* RW RX Discriminator BW Register */;
pub const REG_TX_M: c_uint = 0x306	/* RW TX Mode Register */;
pub const REG_RX_M: c_uint = 0x307	/* RW RX Mode Register */;
pub const REG_RRB: c_uint = 0x30C	/* R RSSI Readback Register */;
pub const REG_LRB: c_uint = 0x30D	/* R Link Quality Readback Register */;
pub const REG_DR0: c_uint = 0x30E	/* RW bits [15:8] of [15:0] data rate setting */;
pub const REG_DR1: c_uint = 0x30F	/* RW bits [7:0] of [15:0] data rate setting */;
pub const REG_PRAMPG: c_uint = 0x313	/* RW RESERVED */;
pub const REG_TXPB: c_uint = 0x314	/* RW TX Packet Storage Base Address */;
pub const REG_RXPB: c_uint = 0x315	/* RW RX Packet Storage Base Address */;
pub const REG_TMR_CFG0: c_uint = 0x316	/* RW Wake up Timer Conf Register - High */;
pub const REG_TMR_CFG1: c_uint = 0x317	/* RW Wake up Timer Conf Register - Low */;
pub const REG_TMR_RLD0: c_uint = 0x318	/* RW Wake up Timer Value Register - High */;
pub const REG_TMR_RLD1: c_uint = 0x319	/* RW Wake up Timer Value Register - Low  */;
pub const REG_TMR_CTRL: c_uint = 0x31A	/* RW Wake up Timer Timeout flag */;
pub const REG_PD_AUX: c_uint = 0x31E	/* RW Battmon enable */;
pub const REG_GP_CFG: c_uint = 0x32C	/* RW GPIO Configuration */;
pub const REG_GP_OUT: c_uint = 0x32D	/* RW GPIO Configuration */;
pub const REG_GP_IN: c_uint = 0x32E	/* R GPIO Configuration */;
pub const REG_SYNT: c_uint = 0x335	/* RW bandwidth calibration timers */;
pub const REG_CAL_CFG: c_uint = 0x33D	/* RW Calibration Settings */;
pub const REG_PA_BIAS: c_uint = 0x36E	/* RW PA BIAS */;
pub const REG_SYNT_CAL: c_uint = 0x371	/* RW Oscillator and Doubler Configuration */;
pub const REG_IIRF_CFG: c_uint = 0x389	/* RW BB Filter Decimation Rate */;
pub const REG_CDR_CFG: c_uint = 0x38A	/* RW CDR kVCO */;
pub const REG_DM_CFG1: c_uint = 0x38B	/* RW Postdemodulator Filter */;
pub const REG_AGCSTAT: c_uint = 0x38E	/* R RXBB Ref Osc Calibration Engine Readback */;
pub const REG_RXCAL0: c_uint = 0x395	/* RW RX BB filter tuning, LSB */;
pub const REG_RXCAL1: c_uint = 0x396	/* RW RX BB filter tuning, MSB */;
pub const REG_RXFE_CFG: c_uint = 0x39B	/* RW RXBB Ref Osc & RXFE Calibration */;
pub const REG_PA_RR: c_uint = 0x3A7	/* RW Set PA ramp rate */;
pub const REG_PA_CFG: c_uint = 0x3A8	/* RW PA enable */;
pub const REG_EXTPA_CFG: c_uint = 0x3A9	/* RW External PA BIAS DAC */;
pub const REG_EXTPA_MSC: c_uint = 0x3AA	/* RW PA Bias Mode */;
pub const REG_ADC_RBK: c_uint = 0x3AE	/* R Readback temp */;
pub const REG_AGC_CFG1: c_uint = 0x3B2	/* RW GC Parameters */;
pub const REG_AGC_MAX: c_uint = 0x3B4	/* RW Slew rate	 */;
pub const REG_AGC_CFG2: c_uint = 0x3B6	/* RW RSSI Parameters */;
pub const REG_AGC_CFG3: c_uint = 0x3B7	/* RW RSSI Parameters */;
pub const REG_AGC_CFG4: c_uint = 0x3B8	/* RW RSSI Parameters */;
pub const REG_AGC_CFG5: c_uint = 0x3B9	/* RW RSSI & NDEC Parameters */;
pub const REG_AGC_CFG6: c_uint = 0x3BA	/* RW NDEC Parameters */;
pub const REG_OCL_CFG1: c_uint = 0x3C4	/* RW OCL System Parameters */;
pub const REG_IRQ1_EN0: c_uint = 0x3C7	/* RW Interrupt Mask set bits for IRQ1 */;
pub const REG_IRQ1_EN1: c_uint = 0x3C8	/* RW Interrupt Mask set bits for IRQ1 */;
pub const REG_IRQ2_EN0: c_uint = 0x3C9	/* RW Interrupt Mask set bits for IRQ2 */;
pub const REG_IRQ2_EN1: c_uint = 0x3CA	/* RW Interrupt Mask set bits for IRQ2 */;
pub const REG_IRQ1_SRC0: c_uint = 0x3CB	/* RW Interrupt Source bits for IRQ */;
pub const REG_IRQ1_SRC1: c_uint = 0x3CC	/* RW Interrupt Source bits for IRQ */;
pub const REG_OCL_BW0: c_uint = 0x3D2	/* RW OCL System Parameters */;
pub const REG_OCL_BW1: c_uint = 0x3D3	/* RW OCL System Parameters */;
pub const REG_OCL_BW2: c_uint = 0x3D4	/* RW OCL System Parameters */;
pub const REG_OCL_BW3: c_uint = 0x3D5	/* RW OCL System Parameters */;
pub const REG_OCL_BW4: c_uint = 0x3D6	/* RW OCL System Parameters */;
pub const REG_OCL_BWS: c_uint = 0x3D7	/* RW OCL System Parameters */;
pub const REG_OCL_CFG13: c_uint = 0x3E0	/* RW OCL System Parameters */;
pub const REG_GP_DRV: c_uint = 0x3E3	/* RW I/O pads Configuration and bg trim */;
pub const REG_BM_CFG: c_uint = 0x3E6	/* RW Batt. Monitor Threshold Voltage setting */;
pub const REG_SFD_15_4: c_uint = 0x3F4	/* RW Option to set non standard SFD */;
pub const REG_AFC_CFG: c_uint = 0x3F7	/* RW AFC mode and polarity */;
pub const REG_AFC_KI_KP: c_uint = 0x3F8	/* RW AFC ki and kp */;
pub const REG_AFC_RANGE: c_uint = 0x3F9	/* RW AFC range */;
pub const REG_AFC_READ: c_uint = 0x3FA	/* RW Readback frequency error */;
// REG_EXTPA_MSC

// REG_PA_CFG

pub const PA_DBIAS_HIGH_POWER: c_int = 21;
pub const PA_DBIAS_LOW_POWER: c_int = 13;
// REG_PA_BIAS

pub const PA_BIAS_HIGH_POWER: c_int = 63;
pub const PA_BIAS_LOW_POWER: c_int = 55;
pub const REG_PAN_ID0: c_uint = 0x112;
pub const REG_PAN_ID1: c_uint = 0x113;
pub const REG_SHORT_ADDR_0: c_uint = 0x114;
pub const REG_SHORT_ADDR_1: c_uint = 0x115;
pub const REG_IEEE_ADDR_0: c_uint = 0x116;
pub const REG_IEEE_ADDR_1: c_uint = 0x117;
pub const REG_IEEE_ADDR_2: c_uint = 0x118;
pub const REG_IEEE_ADDR_3: c_uint = 0x119;
pub const REG_IEEE_ADDR_4: c_uint = 0x11A;
pub const REG_IEEE_ADDR_5: c_uint = 0x11B;
pub const REG_IEEE_ADDR_6: c_uint = 0x11C;
pub const REG_IEEE_ADDR_7: c_uint = 0x11D;
pub const REG_FFILT_CFG: c_uint = 0x11E;
pub const REG_AUTO_CFG: c_uint = 0x11F;
pub const REG_AUTO_TX1: c_uint = 0x120;
pub const REG_AUTO_TX2: c_uint = 0x121;
pub const REG_AUTO_STATUS: c_uint = 0x122;
// REG_FFILT_CFG

// REG_AUTO_CFG

// REG_AUTO_TX1

// REG_AUTO_TX2

pub const CMD_SPI_NOP: c_uint = 0xFF /* No operation. Use for dummy writes */;
pub const CMD_SPI_PKT_WR: c_uint = 0x10 /* Write telegram to the Packet RAM;
// starting from the TX packet base address
// pointer tx_packet_base
//
pub const CMD_SPI_PKT_RD: c_uint = 0x30 /* Read telegram from the Packet RAM;
// starting from RX packet base address
// pointer rxpb.rx_packet_base
//

// Packet RAM sequentially
//

// Packet RAM sequentially
//

// RAM as random block
//

// Packet RAM random block
//
pub const CMD_SPI_PRAM_WR: c_uint = 0x1E /* Write data sequentially to current;
// PRAM page selected
//
pub const CMD_SPI_PRAM_RD: c_uint = 0x3E /* Read data sequentially from current;
// PRAM page selected
//
pub const CMD_RC_SLEEP: c_uint = 0xB1 /* Invoke transition of radio controller;
// into SLEEP state
//
pub const CMD_RC_IDLE: c_uint = 0xB2 /* Invoke transition of radio controller;
// into IDLE state
//
pub const CMD_RC_PHY_RDY: c_uint = 0xB3 /* Invoke transition of radio controller;
// into PHY_RDY state
//
pub const CMD_RC_RX: c_uint = 0xB4 /* Invoke transition of radio controller;
// into RX state
//
pub const CMD_RC_TX: c_uint = 0xB5 /* Invoke transition of radio controller;
// into TX state
//
pub const CMD_RC_MEAS: c_uint = 0xB6 /* Invoke transition of radio controller;
// into MEAS state
//
pub const CMD_RC_CCA: c_uint = 0xB7 /* Invoke Clear channel assessment */;
pub const CMD_RC_CSMACA: c_uint = 0xC1 /* initiates CSMA-CA channel access;
// sequence and frame transmission
//
pub const CMD_RC_PC_RESET: c_uint = 0xC7 /* Program counter reset */;
pub const CMD_RC_RESET: c_uint = 0xC8 /* Resets the ADF7242 and puts it in;
// the sleep state
//

// STATUS

pub const RC_STATUS_IDLE: c_int = 1;
pub const RC_STATUS_MEAS: c_int = 2;
pub const RC_STATUS_PHY_RDY: c_int = 3;
pub const RC_STATUS_RX: c_int = 4;
pub const RC_STATUS_TX: c_int = 5;
pub const RC_STATUS_MASK: c_uint = 0xF;
// AUTO_STATUS
pub const SUCCESS: c_int = 0;
pub const SUCCESS_DATPEND: c_int = 1;
pub const FAILURE_CSMACA: c_int = 2;
pub const FAILURE_NOACK: c_int = 3;
pub const AUTO_STATUS_MASK: c_uint = 0x3;
pub const PRAM_PAGESIZE: c_int = 256;
// IRQ1

pub const FLAG_XMIT: c_int = 0;
pub const FLAG_START: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf7242_local {
    pub spi: *mut spi_device,
    pub tx_complete: completion,
    pub hw: *mut ieee802154_hw,
    pub /: *mut *mut mutex bmux; / protect SPI messages,
    pub stat_msg: spi_message,
    pub stat_xfer: spi_transfer,
    pub debugfs_root: *mut dentry,
    pub work: delayed_work,
    pub wqueue: *mut workqueue_struct,
    pub flags: c_ulong,
    pub tx_stat: c_int,
    pub promiscuous: bool,
    pub rssi: i8,
    pub max_frame_retries: u8,
    pub max_cca_retries: u8,
    pub max_be: u8,
    pub min_be: u8,
// DMA (thus cache coherency maintenance) requires the
// transfer buffers to live in their own cache lines.
//
    pub ____cacheline_aligned: u8 buf[3],
    pub buf_reg_tx: [u8; 3],
    pub buf_read_tx: [u8; 4],
    pub buf_read_rx: [u8; 4],
    pub buf_stat_rx: u8,
    pub buf_stat_tx: u8,
    pub buf_cmd: u8,
}

    static int adf7242_soft_reset(struct adf7242_local *lp, int line);
#[no_mangle]
unsafe extern "C" fn adf7242_status(lp: *mut adf7242_local, stat: *mut u8) -> c_int {
    static int adf7242_status(struct adf7242_local *lp, u8 *stat)
    {
    int status;
    mutex_lock(&lp.bmux);
    status = spi_sync(lp.spi, &lp.stat_msg);
// stat = lp->buf_stat_rx;
    mutex_unlock(&lp.bmux);
    return status;
    }
    static int adf7242_wait_status(struct adf7242_local *lp, unsigned int status,
    unsigned int mask, int line)
    {
    let mut cnt: c_int = 0, ret = 0;
    u8 stat;
    do {
    adf7242_status(lp, &stat);
    cnt++;
    } while (((stat & mask) != status) && (cnt < MAX_POLL_LOOPS));
    if (cnt >= MAX_POLL_LOOPS) {
    ret = -ETIMEDOUT;
    if (!(stat & STAT_RC_READY)) {
    adf7242_soft_reset(lp, line);
    adf7242_status(lp, &stat);
    if ((stat & mask) == status)
    ret = 0;
    }
    if (ret < 0)
    dev_warn(&lp.spi.dev,
    "%s:line %d Timeout status 0x%x (%d)\n",
    __func__, line, stat, cnt);
    }
    dev_vdbg(&lp.spi.dev, "%s : loops=%d line %d\n", __func__, cnt, line);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_wait_rc_ready(lp: *mut adf7242_local, line: c_int) -> c_int {
    static int adf7242_wait_rc_ready(struct adf7242_local *lp, int line)
    {
    return adf7242_wait_status(lp, STAT_RC_READY | STAT_SPI_READY,
    STAT_RC_READY | STAT_SPI_READY, line);
    }
#[no_mangle]
unsafe extern "C" fn adf7242_wait_spi_ready(lp: *mut adf7242_local, line: c_int) -> c_int {
    static int adf7242_wait_spi_ready(struct adf7242_local *lp, int line)
    {
    return adf7242_wait_status(lp, STAT_SPI_READY,
    STAT_SPI_READY, line);
    }
#[no_mangle]
unsafe extern "C" fn adf7242_write_fbuf(lp: *mut adf7242_local, data: *mut u8, len: u8) -> c_int {
    static int adf7242_write_fbuf(struct adf7242_local *lp, u8 *data, u8 len)
    {
    u8 *buf = lp.buf;
    int status;
    struct spi_message msg;
    struct spi_transfer xfer_head = {
    .len = 2,
    .tx_buf = buf,
    };
    struct spi_transfer xfer_buf = {
    .len = len,
    .tx_buf = data,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer_head, &msg);
    spi_message_add_tail(&xfer_buf, &msg);
    adf7242_wait_spi_ready(lp, __LINE__);
    mutex_lock(&lp.bmux);
    buf[0] = CMD_SPI_PKT_WR;
    buf[1] = len + 2;
    status = spi_sync(lp.spi, &msg);
    mutex_unlock(&lp.bmux);
    return status;
    }
    static int adf7242_read_fbuf(struct adf7242_local *lp,
    u8 *data, size_t len, bool packet_read)
    {
    u8 *buf = lp.buf;
    int status;
    struct spi_message msg;
    struct spi_transfer xfer_head = {
    .len = 3,
    .tx_buf = buf,
    .rx_buf = buf,
    };
    struct spi_transfer xfer_buf = {
    .len = len,
    .rx_buf = data,
    };
    spi_message_init(&msg);
    spi_message_add_tail(&xfer_head, &msg);
    spi_message_add_tail(&xfer_buf, &msg);
    adf7242_wait_spi_ready(lp, __LINE__);
    mutex_lock(&lp.bmux);
    if (packet_read) {
    buf[0] = CMD_SPI_PKT_RD;
    buf[1] = CMD_SPI_NOP;
    buf[2] = 0;	/* PHR */
    } else {
    buf[0] = CMD_SPI_PRAM_RD;
    buf[1] = 0;
    buf[2] = CMD_SPI_NOP;
    }
    status = spi_sync(lp.spi, &msg);
    mutex_unlock(&lp.bmux);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_read_reg(lp: *mut adf7242_local, addr: u16, data: *mut u8) -> c_int {
    static int adf7242_read_reg(struct adf7242_local *lp, u16 addr, u8 *data)
    {
    int status;
    struct spi_message msg;
    struct spi_transfer xfer = {
    .len = 4,
    .tx_buf = lp.buf_read_tx,
    .rx_buf = lp.buf_read_rx,
    };
    adf7242_wait_spi_ready(lp, __LINE__);
    mutex_lock(&lp.bmux);
    lp.buf_read_tx[0] = CMD_SPI_MEM_RD(addr);
    lp.buf_read_tx[1] = addr;
    lp.buf_read_tx[2] = CMD_SPI_NOP;
    lp.buf_read_tx[3] = CMD_SPI_NOP;
    spi_message_init(&msg);
    spi_message_add_tail(&xfer, &msg);
    status = spi_sync(lp.spi, &msg);
    if (msg.status)
    status = msg.status;
    if (!status)
// data = lp->buf_read_rx[3];
    mutex_unlock(&lp.bmux);
    dev_vdbg(&lp.spi.dev, "%s : REG 0x%X, VAL 0x%X\n", __func__,
    addr, *data);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_write_reg(lp: *mut adf7242_local, addr: u16, data: u8) -> c_int {
    static int adf7242_write_reg(struct adf7242_local *lp, u16 addr, u8 data)
    {
    int status;
    adf7242_wait_spi_ready(lp, __LINE__);
    mutex_lock(&lp.bmux);
    lp.buf_reg_tx[0] = CMD_SPI_MEM_WR(addr);
    lp.buf_reg_tx[1] = addr;
    lp.buf_reg_tx[2] = data;
    status = spi_write(lp.spi, lp.buf_reg_tx, 3);
    mutex_unlock(&lp.bmux);
    dev_vdbg(&lp.spi.dev, "%s : REG 0x%X, VAL 0x%X\n",
    __func__, addr, data);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_cmd(lp: *mut adf7242_local, cmd: c_uint) -> c_int {
    static int adf7242_cmd(struct adf7242_local *lp, unsigned int cmd)
    {
    int status;
    dev_vdbg(&lp.spi.dev, "%s : CMD=0x%X\n", __func__, cmd);
    if (cmd != CMD_RC_PC_RESET_NO_WAIT)
    adf7242_wait_rc_ready(lp, __LINE__);
    mutex_lock(&lp.bmux);
    lp.buf_cmd = cmd;
    status = spi_write(lp.spi, &lp.buf_cmd, 1);
    mutex_unlock(&lp.bmux);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_upload_firmware(lp: *mut adf7242_local, data: *mut u8, len: u16) -> c_int {
    static int adf7242_upload_firmware(struct adf7242_local *lp, u8 *data, u16 len)
    {
    struct spi_message msg;
    let mut xfer_buf: spi_transfer = { };
    int status, i, page = 0;
    u8 *buf = lp.buf;
    struct spi_transfer xfer_head = {
    .len = 2,
    .tx_buf = buf,
    };
    buf[0] = CMD_SPI_PRAM_WR;
    buf[1] = 0;
    spi_message_init(&msg);
    spi_message_add_tail(&xfer_head, &msg);
    spi_message_add_tail(&xfer_buf, &msg);
    for (i = len; i >= 0; i -= PRAM_PAGESIZE) {
    adf7242_write_reg(lp, REG_PRAMPG, page);
    xfer_buf.len = (i >= PRAM_PAGESIZE) ? PRAM_PAGESIZE : i;
    xfer_buf.tx_buf = &data[page * PRAM_PAGESIZE];
    mutex_lock(&lp.bmux);
    status = spi_sync(lp.spi, &msg);
    mutex_unlock(&lp.bmux);
    page++;
    }
    return status;
    }
    static int adf7242_verify_firmware(struct adf7242_local *lp,
    const u8 *data, size_t len)
    {

    int i, j;
    unsigned int page;
    u8 *buf = kmalloc(PRAM_PAGESIZE, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    for (page = 0, i = len; i >= 0; i -= PRAM_PAGESIZE, page++) {
    let mut nb: usize = (i >= PRAM_PAGESIZE) ? PRAM_PAGESIZE : i;
    adf7242_write_reg(lp, REG_PRAMPG, page);
    adf7242_read_fbuf(lp, buf, nb, false);
    for (j = 0; j < nb; j++) {
    if (buf[j] != data[page * PRAM_PAGESIZE + j]) {
    kfree(buf);
    return -EIO;
    }
    }
    }
    kfree(buf);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_clear_irqstat(lp: *mut adf7242_local) {
    static void adf7242_clear_irqstat(struct adf7242_local *lp)
    {
    adf7242_write_reg(lp, REG_IRQ1_SRC1, IRQ_CCA_COMPLETE | IRQ_SFD_RX |
    IRQ_SFD_TX | IRQ_RX_PKT_RCVD | IRQ_TX_PKT_SENT |
    IRQ_FRAME_VALID | IRQ_ADDRESS_VALID | IRQ_CSMA_CA);
    }
#[no_mangle]
unsafe extern "C" fn adf7242_cmd_rx(lp: *mut adf7242_local) -> c_int {
    static int adf7242_cmd_rx(struct adf7242_local *lp)
    {
// Wait until the ACK is sent
    adf7242_wait_status(lp, RC_STATUS_PHY_RDY, RC_STATUS_MASK, __LINE__);
    adf7242_clear_irqstat(lp);
    mod_delayed_work(lp.wqueue, &lp.work, msecs_to_jiffies(400));
    return adf7242_cmd(lp, CMD_RC_RX);
    }
#[no_mangle]
unsafe extern "C" fn adf7242_rx_cal_work(work: *mut work_struct) {
    static void adf7242_rx_cal_work(struct work_struct *work)
    {
    struct adf7242_local *lp =
    container_of(work, struct adf7242_local, work.work);
// Reissuing RC_RX every 400ms - to adjust for offset
// drift in receiver (datasheet page 61, OCL section)
//
    if (!test_bit(FLAG_XMIT, &lp.flags)) {
    adf7242_cmd(lp, CMD_RC_PHY_RDY);
    adf7242_cmd_rx(lp);
    }
    }
#[no_mangle]
unsafe extern "C" fn adf7242_set_txpower(hw: *mut ieee802154_hw, mbm: c_int) -> c_int {
    static int adf7242_set_txpower(struct ieee802154_hw *hw, int mbm)
    {
    struct adf7242_local *lp = hw.priv;
    u8 pwr, bias_ctrl, dbias, tmp;
    let mut db: c_int = mbm / 100;
    dev_vdbg(&lp.spi.dev, "%s : Power %d dB\n", __func__, db);
    if (db > 5 || db < -26)
    return -EINVAL;
    db = DIV_ROUND_CLOSEST(db + 29, 2);
    if (db > 15) {
    dbias = PA_DBIAS_HIGH_POWER;
    bias_ctrl = PA_BIAS_HIGH_POWER;
    } else {
    dbias = PA_DBIAS_LOW_POWER;
    bias_ctrl = PA_BIAS_LOW_POWER;
    }
    pwr = clamp_t(u8, db, 3, 15);
    adf7242_read_reg(lp, REG_PA_CFG, &tmp);
    tmp &= ~PA_BRIDGE_DBIAS(~0);
    tmp |= PA_BRIDGE_DBIAS(dbias);
    adf7242_write_reg(lp, REG_PA_CFG, tmp);
    adf7242_read_reg(lp, REG_PA_BIAS, &tmp);
    tmp &= ~PA_BIAS_CTRL(~0);
    tmp |= PA_BIAS_CTRL(bias_ctrl);
    adf7242_write_reg(lp, REG_PA_BIAS, tmp);
    adf7242_read_reg(lp, REG_EXTPA_MSC, &tmp);
    tmp &= ~PA_PWR(~0);
    tmp |= PA_PWR(pwr);
    return adf7242_write_reg(lp, REG_EXTPA_MSC, tmp);
    }
    static int adf7242_set_csma_params(struct ieee802154_hw *hw, u8 min_be,
    u8 max_be, u8 retries)
    {
    struct adf7242_local *lp = hw.priv;
    int ret;
    dev_vdbg(&lp.spi.dev, "%s : min_be=%d max_be=%d retries=%d\n",
    __func__, min_be, max_be, retries);
    if (min_be > max_be || max_be > 8 || retries > 5)
    return -EINVAL;
    ret = adf7242_write_reg(lp, REG_AUTO_TX1,
    MAX_FRAME_RETRIES(lp.max_frame_retries) |
    MAX_CCA_RETRIES(retries));
    if (ret)
    return ret;
    lp.max_cca_retries = retries;
    lp.max_be = max_be;
    lp.min_be = min_be;
    return adf7242_write_reg(lp, REG_AUTO_TX2, CSMA_MAX_BE(max_be) |
    CSMA_MIN_BE(min_be));
    }
#[no_mangle]
unsafe extern "C" fn adf7242_set_frame_retries(hw: *mut ieee802154_hw, retries: i8) -> c_int {
    static int adf7242_set_frame_retries(struct ieee802154_hw *hw, s8 retries)
    {
    struct adf7242_local *lp = hw.priv;
    let mut ret: c_int = 0;
    dev_vdbg(&lp.spi.dev, "%s : Retries = %d\n", __func__, retries);
    if (retries < -1 || retries > 15)
    return -EINVAL;
    if (retries >= 0)
    ret = adf7242_write_reg(lp, REG_AUTO_TX1,
    MAX_FRAME_RETRIES(retries) |
    MAX_CCA_RETRIES(lp.max_cca_retries));
    lp.max_frame_retries = retries;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_ed(hw: *mut ieee802154_hw, level: *mut u8) -> c_int {
    static int adf7242_ed(struct ieee802154_hw *hw, u8 *level)
    {
    struct adf7242_local *lp = hw.priv;
// level = lp->rssi;
    dev_vdbg(&lp.spi.dev, "%s :Exit level=%d\n",
    __func__, *level);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_start(hw: *mut ieee802154_hw) -> c_int {
    static int adf7242_start(struct ieee802154_hw *hw)
    {
    struct adf7242_local *lp = hw.priv;
    adf7242_cmd(lp, CMD_RC_PHY_RDY);
    adf7242_clear_irqstat(lp);
    enable_irq(lp.spi.irq);
    set_bit(FLAG_START, &lp.flags);
    return adf7242_cmd_rx(lp);
    }
#[no_mangle]
unsafe extern "C" fn adf7242_stop(hw: *mut ieee802154_hw) {
    static void adf7242_stop(struct ieee802154_hw *hw)
    {
    struct adf7242_local *lp = hw.priv;
    disable_irq(lp.spi.irq);
    cancel_delayed_work_sync(&lp.work);
    adf7242_cmd(lp, CMD_RC_IDLE);
    clear_bit(FLAG_START, &lp.flags);
    adf7242_clear_irqstat(lp);
    }
#[no_mangle]
unsafe extern "C" fn adf7242_channel(hw: *mut ieee802154_hw, page: u8, channel: u8) -> c_int {
    static int adf7242_channel(struct ieee802154_hw *hw, u8 page, u8 channel)
    {
    struct adf7242_local *lp = hw.priv;
    unsigned long freq;
    dev_dbg(&lp.spi.dev, "%s :Channel=%d\n", __func__, channel);
    might_sleep();
    WARN_ON(page != 0);
    WARN_ON(channel < 11);
    WARN_ON(channel > 26);
    freq = (2405 + 5 * (channel - 11)) * 100;
    adf7242_cmd(lp, CMD_RC_PHY_RDY);
    adf7242_write_reg(lp, REG_CH_FREQ0, freq);
    adf7242_write_reg(lp, REG_CH_FREQ1, freq >> 8);
    adf7242_write_reg(lp, REG_CH_FREQ2, freq >> 16);
    if (test_bit(FLAG_START, &lp.flags))
    return adf7242_cmd_rx(lp);
    else
    return adf7242_cmd(lp, CMD_RC_PHY_RDY);
    }
    static int adf7242_set_hw_addr_filt(struct ieee802154_hw *hw,
    struct ieee802154_hw_addr_filt *filt,
    unsigned long changed)
    {
    struct adf7242_local *lp = hw.priv;
    u8 reg;
    dev_dbg(&lp.spi.dev, "%s :Changed=0x%lX\n", __func__, changed);
    might_sleep();
    if (changed & IEEE802154_AFILT_IEEEADDR_CHANGED) {
    u8 addr[8], i;
    memcpy(addr, &filt.ieee_addr, 8);
    for (i = 0; i < 8; i++)
    adf7242_write_reg(lp, REG_IEEE_ADDR_0 + i, addr[i]);
    }
    if (changed & IEEE802154_AFILT_SADDR_CHANGED) {
    let mut saddr: u16 = le16_to_cpu(filt.short_addr);
    adf7242_write_reg(lp, REG_SHORT_ADDR_0, saddr);
    adf7242_write_reg(lp, REG_SHORT_ADDR_1, saddr >> 8);
    }
    if (changed & IEEE802154_AFILT_PANID_CHANGED) {
    let mut pan_id: u16 = le16_to_cpu(filt.pan_id);
    adf7242_write_reg(lp, REG_PAN_ID0, pan_id);
    adf7242_write_reg(lp, REG_PAN_ID1, pan_id >> 8);
    }
    if (changed & IEEE802154_AFILT_PANC_CHANGED) {
    adf7242_read_reg(lp, REG_AUTO_CFG, &reg);
    if (filt.pan_coord)
    reg |= IS_PANCOORD;
    else
    reg &= ~IS_PANCOORD;
    adf7242_write_reg(lp, REG_AUTO_CFG, reg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_set_promiscuous_mode(hw: *mut ieee802154_hw, on: bool) -> c_int {
    static int adf7242_set_promiscuous_mode(struct ieee802154_hw *hw, bool on)
    {
    struct adf7242_local *lp = hw.priv;
    dev_dbg(&lp.spi.dev, "%s : mode %d\n", __func__, on);
    lp.promiscuous = on;
    if (on) {
    adf7242_write_reg(lp, REG_AUTO_CFG, 0);
    return adf7242_write_reg(lp, REG_FFILT_CFG,
    ACCEPT_BEACON_FRAMES |
    ACCEPT_DATA_FRAMES |
    ACCEPT_MACCMD_FRAMES |
    ACCEPT_ALL_ADDRESS |
    ACCEPT_ACK_FRAMES |
    ACCEPT_RESERVED_FRAMES);
    } else {
    adf7242_write_reg(lp, REG_FFILT_CFG,
    ACCEPT_BEACON_FRAMES |
    ACCEPT_DATA_FRAMES |
    ACCEPT_MACCMD_FRAMES |
    ACCEPT_RESERVED_FRAMES);
    return adf7242_write_reg(lp, REG_AUTO_CFG, RX_AUTO_ACK_EN);
    }
    }
#[no_mangle]
unsafe extern "C" fn adf7242_set_cca_ed_level(hw: *mut ieee802154_hw, mbm: i32) -> c_int {
    static int adf7242_set_cca_ed_level(struct ieee802154_hw *hw, s32 mbm)
    {
    struct adf7242_local *lp = hw.priv;
    let mut level: i8 = clamp_t(s8, mbm / 100, S8_MIN, S8_MAX);
    dev_dbg(&lp.spi.dev, "%s : level %d\n", __func__, level);
    return adf7242_write_reg(lp, REG_CCA1, level);
    }
#[no_mangle]
unsafe extern "C" fn adf7242_xmit(hw: *mut ieee802154_hw, skb: *mut sk_buff) -> c_int {
    static int adf7242_xmit(struct ieee802154_hw *hw, struct sk_buff *skb)
    {
    struct adf7242_local *lp = hw.priv;
    int ret;
// ensure existing instances of the IRQ handler have completed
    disable_irq(lp.spi.irq);
    set_bit(FLAG_XMIT, &lp.flags);
    cancel_delayed_work_sync(&lp.work);
    reinit_completion(&lp.tx_complete);
    adf7242_cmd(lp, CMD_RC_PHY_RDY);
    adf7242_clear_irqstat(lp);
    ret = adf7242_write_fbuf(lp, skb.data, skb.len);
    if (ret)
    goto err;
    ret = adf7242_cmd(lp, CMD_RC_CSMACA);
    if (ret)
    goto err;
    enable_irq(lp.spi.irq);
    ret = wait_for_completion_interruptible_timeout(&lp.tx_complete,
    HZ / 10);
    if (ret < 0)
    goto err;
    if (ret == 0) {
    dev_dbg(&lp.spi.dev, "Timeout waiting for TX interrupt\n");
    ret = -ETIMEDOUT;
    goto err;
    }
    if (lp.tx_stat != SUCCESS) {
    dev_dbg(&lp.spi.dev,
    "Error xmit: Retry count exceeded Status=0x%x\n",
    lp.tx_stat);
    ret = -ECOMM;
    } else {
    ret = 0;
    }
    err:
    clear_bit(FLAG_XMIT, &lp.flags);
    adf7242_cmd_rx(lp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_rx(lp: *mut adf7242_local) -> c_int {
    static int adf7242_rx(struct adf7242_local *lp)
    {
    struct sk_buff *skb;
    size_t len;
    int ret;
    u8 lqi, len_u8, *data;
    ret = adf7242_read_reg(lp, 0, &len_u8);
    if (ret)
    return ret;
    len = len_u8;
    if (!ieee802154_is_valid_psdu_len(len)) {
    dev_dbg(&lp.spi.dev,
    "corrupted frame received len %d\n", (int)len);
    len = IEEE802154_MTU;
    }
    skb = dev_alloc_skb(len);
    if (!skb) {
    adf7242_cmd_rx(lp);
    return -ENOMEM;
    }
    data = skb_put(skb, len);
    ret = adf7242_read_fbuf(lp, data, len, true);
    if (ret < 0) {
    kfree_skb(skb);
    adf7242_cmd_rx(lp);
    return ret;
    }
    lqi = data[len - 2];
    lp.rssi = data[len - 1];
    ret = adf7242_cmd_rx(lp);
    skb_trim(skb, len - 2);	/* Don't put RSSI/LQI or CRC into the frame */
    ieee802154_rx_irqsafe(lp.hw, skb, lqi);
    dev_dbg(&lp.spi.dev, "%s: ret=%d len=%d lqi=%d rssi=%d\n",
    __func__, ret, (int)len, (int)lqi, lp.rssi);
    return ret;
    }
    static const struct ieee802154_ops adf7242_ops = {
    .owner = THIS_MODULE,
    .xmit_sync = adf7242_xmit,
    .ed = adf7242_ed,
    .set_channel = adf7242_channel,
    .set_hw_addr_filt = adf7242_set_hw_addr_filt,
    .start = adf7242_start,
    .stop = adf7242_stop,
    .set_csma_params = adf7242_set_csma_params,
    .set_frame_retries = adf7242_set_frame_retries,
    .set_txpower = adf7242_set_txpower,
    .set_promiscuous_mode = adf7242_set_promiscuous_mode,
    .set_cca_ed_level = adf7242_set_cca_ed_level,
    };
#[no_mangle]
unsafe extern "C" fn adf7242_debug(lp: *mut adf7242_local, irq1: u8) {
    static void adf7242_debug(struct adf7242_local *lp, u8 irq1)
    {

    u8 stat;
    adf7242_status(lp, &stat);
    dev_dbg(&lp.spi.dev, "%s IRQ1 = %X:\n%s%s%s%s%s%s%s%s\n",
    __func__, irq1,
    irq1 & IRQ_CCA_COMPLETE ? "IRQ_CCA_COMPLETE\n" : "",
    irq1 & IRQ_SFD_RX ? "IRQ_SFD_RX\n" : "",
    irq1 & IRQ_SFD_TX ? "IRQ_SFD_TX\n" : "",
    irq1 & IRQ_RX_PKT_RCVD ? "IRQ_RX_PKT_RCVD\n" : "",
    irq1 & IRQ_TX_PKT_SENT ? "IRQ_TX_PKT_SENT\n" : "",
    irq1 & IRQ_CSMA_CA ? "IRQ_CSMA_CA\n" : "",
    irq1 & IRQ_FRAME_VALID ? "IRQ_FRAME_VALID\n" : "",
    irq1 & IRQ_ADDRESS_VALID ? "IRQ_ADDRESS_VALID\n" : "");
    dev_dbg(&lp.spi.dev, "%s STATUS = %X:\n%s\n%s\n%s\n%s\n%s%s%s%s%s\n",
    __func__, stat,
    stat & STAT_SPI_READY ? "SPI_READY" : "SPI_BUSY",
    stat & STAT_IRQ_STATUS ? "IRQ_PENDING" : "IRQ_CLEAR",
    stat & STAT_RC_READY ? "RC_READY" : "RC_BUSY",
    stat & STAT_CCA_RESULT ? "CHAN_IDLE" : "CHAN_BUSY",
    (stat & 0xf) == RC_STATUS_IDLE ? "RC_STATUS_IDLE" : "",
    (stat & 0xf) == RC_STATUS_MEAS ? "RC_STATUS_MEAS" : "",
    (stat & 0xf) == RC_STATUS_PHY_RDY ? "RC_STATUS_PHY_RDY" : "",
    (stat & 0xf) == RC_STATUS_RX ? "RC_STATUS_RX" : "",
    (stat & 0xf) == RC_STATUS_TX ? "RC_STATUS_TX" : "");

    }
#[no_mangle]
unsafe extern "C" fn adf7242_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t adf7242_isr(int irq, void *data)
    {
    struct adf7242_local *lp = data;
    unsigned int xmit;
    u8 irq1;
    mod_delayed_work(lp.wqueue, &lp.work, msecs_to_jiffies(400));
    adf7242_read_reg(lp, REG_IRQ1_SRC1, &irq1);
    if (!(irq1 & (IRQ_RX_PKT_RCVD | IRQ_CSMA_CA)))
    dev_err(&lp.spi.dev, "%s :ERROR IRQ1 = 0x%X\n",
    __func__, irq1);
    adf7242_debug(lp, irq1);
    xmit = test_bit(FLAG_XMIT, &lp.flags);
    if (xmit && (irq1 & IRQ_CSMA_CA)) {
    adf7242_wait_status(lp, RC_STATUS_PHY_RDY,
    RC_STATUS_MASK, __LINE__);
    if (ADF7242_REPORT_CSMA_CA_STAT) {
    u8 astat;
    adf7242_read_reg(lp, REG_AUTO_STATUS, &astat);
    astat &= AUTO_STATUS_MASK;
    dev_dbg(&lp.spi.dev, "AUTO_STATUS = %X:\n%s%s%s%s\n",
    astat,
    astat == SUCCESS ? "SUCCESS" : "",
    astat ==
    SUCCESS_DATPEND ? "SUCCESS_DATPEND" : "",
    astat == FAILURE_CSMACA ? "FAILURE_CSMACA" : "",
    astat == FAILURE_NOACK ? "FAILURE_NOACK" : "");
// save CSMA-CA completion status
    lp.tx_stat = astat;
    } else {
    lp.tx_stat = SUCCESS;
    }
    complete(&lp.tx_complete);
    adf7242_clear_irqstat(lp);
    } else if (!xmit && (irq1 & IRQ_RX_PKT_RCVD) &&
    (irq1 & IRQ_FRAME_VALID)) {
    adf7242_rx(lp);
    } else if (!xmit && test_bit(FLAG_START, &lp.flags)) {
// Invalid packet received - drop it and restart
    dev_dbg(&lp.spi.dev, "%s:%d : ERROR IRQ1 = 0x%X\n",
    __func__, __LINE__, irq1);
    adf7242_cmd(lp, CMD_RC_PHY_RDY);
    adf7242_cmd_rx(lp);
    } else {
// This can only be xmit without IRQ, likely a RX packet.
// we get an TX IRQ shortly - do nothing or let the xmit
// timeout handle this
//
    dev_dbg(&lp.spi.dev, "%s:%d : ERROR IRQ1 = 0x%X, xmit %d\n",
    __func__, __LINE__, irq1, xmit);
    adf7242_wait_status(lp, RC_STATUS_PHY_RDY,
    RC_STATUS_MASK, __LINE__);
    complete(&lp.tx_complete);
    adf7242_clear_irqstat(lp);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_soft_reset(lp: *mut adf7242_local, line: c_int) -> c_int {
    static int adf7242_soft_reset(struct adf7242_local *lp, int line)
    {
    dev_warn(&lp.spi.dev, "%s (line %d)\n", __func__, line);
    if (test_bit(FLAG_START, &lp.flags))
    disable_irq_nosync(lp.spi.irq);
    adf7242_cmd(lp, CMD_RC_PC_RESET_NO_WAIT);
    usleep_range(200, 250);
    adf7242_write_reg(lp, REG_PKT_CFG, ADDON_EN | BIT(2));
    adf7242_cmd(lp, CMD_RC_PHY_RDY);
    adf7242_set_promiscuous_mode(lp.hw, lp.promiscuous);
    adf7242_set_csma_params(lp.hw, lp.min_be, lp.max_be,
    lp.max_cca_retries);
    adf7242_clear_irqstat(lp);
    if (test_bit(FLAG_START, &lp.flags)) {
    enable_irq(lp.spi.irq);
    return adf7242_cmd(lp, CMD_RC_RX);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_hw_init(lp: *mut adf7242_local) -> c_int {
    static int adf7242_hw_init(struct adf7242_local *lp)
    {
    int ret;
    const struct firmware *fw;
    adf7242_cmd(lp, CMD_RC_RESET);
    adf7242_cmd(lp, CMD_RC_IDLE);
// get ADF7242 addon firmware
// build this driver as module
// and place under /lib/firmware/adf7242_firmware.bin
// or compile firmware into the kernel.
//
    ret = request_firmware(&fw, FIRMWARE, &lp.spi.dev);
    if (ret) {
    dev_err(&lp.spi.dev,
    "request_firmware() failed with %d\n", ret);
    return ret;
    }
    ret = adf7242_upload_firmware(lp, (u8 *)fw.data, fw.size);
    if (ret) {
    dev_err(&lp.spi.dev,
    "upload firmware failed with %d\n", ret);
    release_firmware(fw);
    return ret;
    }
    ret = adf7242_verify_firmware(lp, (u8 *)fw.data, fw.size);
    if (ret) {
    dev_err(&lp.spi.dev,
    "verify firmware failed with %d\n", ret);
    release_firmware(fw);
    return ret;
    }
    adf7242_cmd(lp, CMD_RC_PC_RESET);
    release_firmware(fw);
    adf7242_write_reg(lp, REG_FFILT_CFG,
    ACCEPT_BEACON_FRAMES |
    ACCEPT_DATA_FRAMES |
    ACCEPT_MACCMD_FRAMES |
    ACCEPT_RESERVED_FRAMES);
    adf7242_write_reg(lp, REG_AUTO_CFG, RX_AUTO_ACK_EN);
    adf7242_write_reg(lp, REG_PKT_CFG, ADDON_EN | BIT(2));
    adf7242_write_reg(lp, REG_EXTPA_MSC, 0xF1);
    adf7242_write_reg(lp, REG_RXFE_CFG, 0x1D);
    adf7242_write_reg(lp, REG_IRQ1_EN0, 0);
    adf7242_write_reg(lp, REG_IRQ1_EN1, IRQ_RX_PKT_RCVD | IRQ_CSMA_CA);
    adf7242_clear_irqstat(lp);
    adf7242_write_reg(lp, REG_IRQ1_SRC0, 0xFF);
    adf7242_cmd(lp, CMD_RC_IDLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_stats_show(file: *mut seq_file, offset: *mut c_void) -> c_int {
    static int adf7242_stats_show(struct seq_file *file, void *offset)
    {
    struct adf7242_local *lp = spi_get_drvdata(file.private);
    u8 stat, irq1;
    adf7242_status(lp, &stat);
    adf7242_read_reg(lp, REG_IRQ1_SRC1, &irq1);
    seq_printf(file, "IRQ1 = %X:\n%s%s%s%s%s%s%s%s\n", irq1,
    irq1 & IRQ_CCA_COMPLETE ? "IRQ_CCA_COMPLETE\n" : "",
    irq1 & IRQ_SFD_RX ? "IRQ_SFD_RX\n" : "",
    irq1 & IRQ_SFD_TX ? "IRQ_SFD_TX\n" : "",
    irq1 & IRQ_RX_PKT_RCVD ? "IRQ_RX_PKT_RCVD\n" : "",
    irq1 & IRQ_TX_PKT_SENT ? "IRQ_TX_PKT_SENT\n" : "",
    irq1 & IRQ_CSMA_CA ? "IRQ_CSMA_CA\n" : "",
    irq1 & IRQ_FRAME_VALID ? "IRQ_FRAME_VALID\n" : "",
    irq1 & IRQ_ADDRESS_VALID ? "IRQ_ADDRESS_VALID\n" : "");
    seq_printf(file, "STATUS = %X:\n%s\n%s\n%s\n%s\n%s%s%s%s%s\n", stat,
    stat & STAT_SPI_READY ? "SPI_READY" : "SPI_BUSY",
    stat & STAT_IRQ_STATUS ? "IRQ_PENDING" : "IRQ_CLEAR",
    stat & STAT_RC_READY ? "RC_READY" : "RC_BUSY",
    stat & STAT_CCA_RESULT ? "CHAN_IDLE" : "CHAN_BUSY",
    (stat & 0xf) == RC_STATUS_IDLE ? "RC_STATUS_IDLE" : "",
    (stat & 0xf) == RC_STATUS_MEAS ? "RC_STATUS_MEAS" : "",
    (stat & 0xf) == RC_STATUS_PHY_RDY ? "RC_STATUS_PHY_RDY" : "",
    (stat & 0xf) == RC_STATUS_RX ? "RC_STATUS_RX" : "",
    (stat & 0xf) == RC_STATUS_TX ? "RC_STATUS_TX" : "");
    seq_printf(file, "RSSI = %d\n", lp.rssi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_debugfs_init(lp: *mut adf7242_local) {
    static void adf7242_debugfs_init(struct adf7242_local *lp)
    {
    char debugfs_dir_name[DNAME_INLINE_LEN + 1];
    snprintf(debugfs_dir_name, sizeof(debugfs_dir_name),
    "adf7242-%s", dev_name(&lp.spi.dev));
    lp.debugfs_root = debugfs_create_dir(debugfs_dir_name, core::ptr::null_mut());
    debugfs_create_devm_seqfile(&lp.spi.dev, "status", lp.debugfs_root,
    adf7242_stats_show);
    }
    static const s32 adf7242_powers[] = {
    500, 400, 300, 200, 100, 0, -100, -200, -300, -400, -500, -600, -700,
    -800, -900, -1000, -1100, -1200, -1300, -1400, -1500, -1600, -1700,
    -1800, -1900, -2000, -2100, -2200, -2300, -2400, -2500, -2600,
    };
    static const s32 adf7242_ed_levels[] = {
    -9000, -8900, -8800, -8700, -8600, -8500, -8400, -8300, -8200, -8100,
    -8000, -7900, -7800, -7700, -7600, -7500, -7400, -7300, -7200, -7100,
    -7000, -6900, -6800, -6700, -6600, -6500, -6400, -6300, -6200, -6100,
    -6000, -5900, -5800, -5700, -5600, -5500, -5400, -5300, -5200, -5100,
    -5000, -4900, -4800, -4700, -4600, -4500, -4400, -4300, -4200, -4100,
    -4000, -3900, -3800, -3700, -3600, -3500, -3400, -3200, -3100, -3000
    };
#[no_mangle]
unsafe extern "C" fn adf7242_probe(spi: *mut spi_device) -> c_int {
    static int adf7242_probe(struct spi_device *spi)
    {
    struct ieee802154_hw *hw;
    struct adf7242_local *lp;
    int ret, irq_type;
    if (!spi.irq) {
    dev_err(&spi.dev, "no IRQ specified\n");
    return -EINVAL;
    }
    hw = ieee802154_alloc_hw(sizeof(*lp), &adf7242_ops);
    if (!hw)
    return -ENOMEM;
    lp = hw.priv;
    lp.hw = hw;
    lp.spi = spi;
    hw.priv = lp;
    hw.parent = &spi.dev;
    hw.extra_tx_headroom = 0;
// We support only 2.4 Ghz
    hw.phy.supported.channels[0] = 0x7FFF800;
    hw.flags = IEEE802154_HW_OMIT_CKSUM |
    IEEE802154_HW_CSMA_PARAMS |
    IEEE802154_HW_FRAME_RETRIES | IEEE802154_HW_AFILT |
    IEEE802154_HW_PROMISCUOUS;
    hw.phy.flags = WPAN_PHY_FLAG_TXPOWER |
    WPAN_PHY_FLAG_CCA_ED_LEVEL |
    WPAN_PHY_FLAG_CCA_MODE;
    hw.phy.supported.cca_modes = BIT(NL802154_CCA_ENERGY);
    hw.phy.supported.cca_ed_levels = adf7242_ed_levels;
    hw.phy.supported.cca_ed_levels_size = ARRAY_SIZE(adf7242_ed_levels);
    hw.phy.cca.mode = NL802154_CCA_ENERGY;
    hw.phy.supported.tx_powers = adf7242_powers;
    hw.phy.supported.tx_powers_size = ARRAY_SIZE(adf7242_powers);
    hw.phy.supported.min_minbe = 0;
    hw.phy.supported.max_minbe = 8;
    hw.phy.supported.min_maxbe = 3;
    hw.phy.supported.max_maxbe = 8;
    hw.phy.supported.min_frame_retries = 0;
    hw.phy.supported.max_frame_retries = 15;
    hw.phy.supported.min_csma_backoffs = 0;
    hw.phy.supported.max_csma_backoffs = 5;
    ieee802154_random_extended_addr(&hw.phy.perm_extended_addr);
    mutex_init(&lp.bmux);
    init_completion(&lp.tx_complete);
// Setup Status Message
    lp.stat_xfer.len = 1;
    lp.stat_xfer.tx_buf = &lp.buf_stat_tx;
    lp.stat_xfer.rx_buf = &lp.buf_stat_rx;
    lp.buf_stat_tx = CMD_SPI_NOP;
    spi_message_init(&lp.stat_msg);
    spi_message_add_tail(&lp.stat_xfer, &lp.stat_msg);
    spi_set_drvdata(spi, lp);
    INIT_DELAYED_WORK(&lp.work, adf7242_rx_cal_work);
    lp.wqueue = alloc_ordered_workqueue(dev_name(&spi.dev),
    WQ_MEM_RECLAIM);
    if (unlikely(!lp.wqueue)) {
    ret = -ENOMEM;
    goto err_alloc_wq;
    }
    ret = adf7242_hw_init(lp);
    if (ret)
    goto err_hw_init;
    irq_type = irq_get_trigger_type(spi.irq);
    if (!irq_type)
    irq_type = IRQF_TRIGGER_HIGH;
    ret = devm_request_threaded_irq(&spi.dev, spi.irq, core::ptr::null_mut(), adf7242_isr,
    irq_type | IRQF_ONESHOT,
    dev_name(&spi.dev), lp);
    if (ret)
    goto err_hw_init;
    disable_irq(spi.irq);
    ret = ieee802154_register_hw(lp.hw);
    if (ret)
    goto err_hw_init;
    dev_set_drvdata(&spi.dev, lp);
    adf7242_debugfs_init(lp);
    dev_info(&spi.dev, "mac802154 IRQ-%d registered\n", spi.irq);
    return ret;
    err_hw_init:
    destroy_workqueue(lp.wqueue);
    err_alloc_wq:
    mutex_destroy(&lp.bmux);
    ieee802154_free_hw(lp.hw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adf7242_remove(spi: *mut spi_device) {
    static void adf7242_remove(struct spi_device *spi)
    {
    struct adf7242_local *lp = spi_get_drvdata(spi);
    debugfs_remove_recursive(lp.debugfs_root);
    ieee802154_unregister_hw(lp.hw);
    cancel_delayed_work_sync(&lp.work);
    destroy_workqueue(lp.wqueue);
    mutex_destroy(&lp.bmux);
    ieee802154_free_hw(lp.hw);
    }
    static const struct of_device_id adf7242_of_match[] = {
    { .compatible = "adi,adf7242", },
    { .compatible = "adi,adf7241", },
    { },
    };
    MODULE_DEVICE_TABLE(of, adf7242_of_match);
    static const struct spi_device_id adf7242_device_id[] = {
    { .name = "adf7242", },
    { .name = "adf7241", },
    { },
    };
    MODULE_DEVICE_TABLE(spi, adf7242_device_id);
    static struct spi_driver adf7242_driver = {
    .id_table = adf7242_device_id,
    .driver = {
    .of_match_table = adf7242_of_match,
    .name = "adf7242",
    },
    .probe = adf7242_probe,
    .remove = adf7242_remove,
    };
    module_spi_driver(adf7242_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("ADF7242 IEEE802.15.4 Transceiver Driver");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE(FIRMWARE);
