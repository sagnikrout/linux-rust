//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-rspi.c
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


// SPDX-License-Identifier: GPL-2.0
//
// SH RSPI driver
//
// Copyright (C) 2012, 2013  Renesas Solutions Corp.
// Copyright (C) 2014 Glider bvba
//
// Based on spi-sh.c:
// Copyright (C) 2011 Renesas Solutions Corp.
//

pub const RSPI_SPCR: c_uint = 0x00	/* Control Register */;
pub const RSPI_SSLP: c_uint = 0x01	/* Slave Select Polarity Register */;
pub const RSPI_SPPCR: c_uint = 0x02	/* Pin Control Register */;
pub const RSPI_SPSR: c_uint = 0x03	/* Status Register */;
pub const RSPI_SPDR: c_uint = 0x04	/* Data Register */;
pub const RSPI_SPSCR: c_uint = 0x08	/* Sequence Control Register */;
pub const RSPI_SPSSR: c_uint = 0x09	/* Sequence Status Register */;
pub const RSPI_SPBR: c_uint = 0x0a	/* Bit Rate Register */;
pub const RSPI_SPDCR: c_uint = 0x0b	/* Data Control Register */;
pub const RSPI_SPCKD: c_uint = 0x0c	/* Clock Delay Register */;
pub const RSPI_SSLND: c_uint = 0x0d	/* Slave Select Negation Delay Register */;
pub const RSPI_SPND: c_uint = 0x0e	/* Next-Access Delay Register */;
pub const RSPI_SPCR2: c_uint = 0x0f	/* Control Register 2 (SH only) */;
pub const RSPI_SPCMD0: c_uint = 0x10	/* Command Register 0 */;
pub const RSPI_SPCMD1: c_uint = 0x12	/* Command Register 1 */;
pub const RSPI_SPCMD2: c_uint = 0x14	/* Command Register 2 */;
pub const RSPI_SPCMD3: c_uint = 0x16	/* Command Register 3 */;
pub const RSPI_SPCMD4: c_uint = 0x18	/* Command Register 4 */;
pub const RSPI_SPCMD5: c_uint = 0x1a	/* Command Register 5 */;
pub const RSPI_SPCMD6: c_uint = 0x1c	/* Command Register 6 */;
pub const RSPI_SPCMD7: c_uint = 0x1e	/* Command Register 7 */;

pub const RSPI_NUM_SPCMD: c_int = 8;
pub const RSPI_RZ_NUM_SPCMD: c_int = 4;
pub const QSPI_NUM_SPCMD: c_int = 4;
// RSPI on RZ only
pub const RSPI_SPBFCR: c_uint = 0x20	/* Buffer Control Register */;
pub const RSPI_SPBFDR: c_uint = 0x22	/* Buffer Data Count Setting Register */;
// QSPI only
pub const QSPI_SPBFCR: c_uint = 0x18	/* Buffer Control Register */;
pub const QSPI_SPBDCR: c_uint = 0x1a	/* Buffer Data Count Register */;
pub const QSPI_SPBMUL0: c_uint = 0x1c	/* Transfer Data Length Multiplier Setting Register 0 */;
pub const QSPI_SPBMUL1: c_uint = 0x20	/* Transfer Data Length Multiplier Setting Register 1 */;
pub const QSPI_SPBMUL2: c_uint = 0x24	/* Transfer Data Length Multiplier Setting Register 2 */;
pub const QSPI_SPBMUL3: c_uint = 0x28	/* Transfer Data Length Multiplier Setting Register 3 */;

// SPCR - Control Register
pub const SPCR_SPRIE: c_uint = 0x80	/* Receive Interrupt Enable */;
pub const SPCR_SPE: c_uint = 0x40	/* Function Enable */;
pub const SPCR_SPTIE: c_uint = 0x20	/* Transmit Interrupt Enable */;
pub const SPCR_SPEIE: c_uint = 0x10	/* Error Interrupt Enable */;
pub const SPCR_MSTR: c_uint = 0x08	/* Master/Slave Mode Select */;
pub const SPCR_MODFEN: c_uint = 0x04	/* Mode Fault Error Detection Enable */;
// RSPI on SH only
pub const SPCR_TXMD: c_uint = 0x02	/* TX Only Mode (vs. Full Duplex) */;
pub const SPCR_SPMS: c_uint = 0x01	/* 3-wire Mode (vs. 4-wire) */;
// QSPI on R-Car Gen2 only
pub const SPCR_WSWAP: c_uint = 0x02	/* Word Swap of read-data for DMAC */;
pub const SPCR_BSWAP: c_uint = 0x01	/* Byte Swap of read-data for DMAC */;
// SSLP - Slave Select Polarity Register

// SPPCR - Pin Control Register
pub const SPPCR_MOIFE: c_uint = 0x20	/* MOSI Idle Value Fixing Enable */;
pub const SPPCR_MOIFV: c_uint = 0x10	/* MOSI Idle Fixed Value */;
pub const SPPCR_SPOM: c_uint = 0x04;
pub const SPPCR_SPLP2: c_uint = 0x02	/* Loopback Mode 2 (non-inverting) */;
pub const SPPCR_SPLP: c_uint = 0x01	/* Loopback Mode (inverting) */;
pub const SPPCR_IO3FV: c_uint = 0x04	/* Single-/Dual-SPI Mode IO3 Output Fixed Value */;
pub const SPPCR_IO2FV: c_uint = 0x04	/* Single-/Dual-SPI Mode IO2 Output Fixed Value */;
// SPSR - Status Register
pub const SPSR_SPRF: c_uint = 0x80	/* Receive Buffer Full Flag */;
pub const SPSR_TEND: c_uint = 0x40	/* Transmit End */;
pub const SPSR_SPTEF: c_uint = 0x20	/* Transmit Buffer Empty Flag */;
pub const SPSR_PERF: c_uint = 0x08	/* Parity Error Flag */;
pub const SPSR_MODF: c_uint = 0x04	/* Mode Fault Error Flag */;
pub const SPSR_IDLNF: c_uint = 0x02	/* RSPI Idle Flag */;
pub const SPSR_OVRF: c_uint = 0x01	/* Overrun Error Flag (RSPI only) */;
// SPSCR - Sequence Control Register
pub const SPSCR_SPSLN_MASK: c_uint = 0x07	/* Sequence Length Specification */;
// SPSSR - Sequence Status Register
pub const SPSSR_SPECM_MASK: c_uint = 0x70	/* Command Error Mask */;
pub const SPSSR_SPCP_MASK: c_uint = 0x07	/* Command Pointer Mask */;
// SPDCR - Data Control Register
pub const SPDCR_TXDMY: c_uint = 0x80	/* Dummy Data Transmission Enable */;
pub const SPDCR_SPLW1: c_uint = 0x40	/* Access Width Specification (RZ) */;
pub const SPDCR_SPLW0: c_uint = 0x20	/* Access Width Specification (RZ) */;

pub const SPDCR_SPLW: c_uint = 0x20	/* Access Width Specification (SH) */;
pub const SPDCR_SPRDTD: c_uint = 0x10	/* Receive Transmit Data Select (SH) */;
pub const SPDCR_SLSEL1: c_uint = 0x08;
pub const SPDCR_SLSEL0: c_uint = 0x04;
pub const SPDCR_SLSEL_MASK: c_uint = 0x0c	/* SSL1 Output Select (SH) */;
pub const SPDCR_SPFC1: c_uint = 0x02;
pub const SPDCR_SPFC0: c_uint = 0x01;
pub const SPDCR_SPFC_MASK: c_uint = 0x03	/* Frame Count Setting (1-4) (SH) */;
// SPCKD - Clock Delay Register
pub const SPCKD_SCKDL_MASK: c_uint = 0x07	/* Clock Delay Setting (1-8) */;
// SSLND - Slave Select Negation Delay Register
pub const SSLND_SLNDL_MASK: c_uint = 0x07	/* SSL Negation Delay Setting (1-8) */;
// SPND - Next-Access Delay Register
pub const SPND_SPNDL_MASK: c_uint = 0x07	/* Next-Access Delay Setting (1-8) */;
// SPCR2 - Control Register 2
pub const SPCR2_PTE: c_uint = 0x08	/* Parity Self-Test Enable */;
pub const SPCR2_SPIE: c_uint = 0x04	/* Idle Interrupt Enable */;
pub const SPCR2_SPOE: c_uint = 0x02	/* Odd Parity Enable (vs. Even) */;
pub const SPCR2_SPPE: c_uint = 0x01	/* Parity Enable */;
// SPCMDn - Command Registers
pub const SPCMD_SCKDEN: c_uint = 0x8000	/* Clock Delay Setting Enable */;
pub const SPCMD_SLNDEN: c_uint = 0x4000	/* SSL Negation Delay Setting Enable */;
pub const SPCMD_SPNDEN: c_uint = 0x2000	/* Next-Access Delay Enable */;
pub const SPCMD_LSBF: c_uint = 0x1000	/* LSB First */;
pub const SPCMD_SPB_MASK: c_uint = 0x0f00	/* Data Length Setting */;

pub const SPCMD_SPB_8BIT: c_uint = 0x0000	/* QSPI only */;
pub const SPCMD_SPB_16BIT: c_uint = 0x0100;
pub const SPCMD_SPB_20BIT: c_uint = 0x0000;
pub const SPCMD_SPB_24BIT: c_uint = 0x0100;
pub const SPCMD_SPB_32BIT: c_uint = 0x0200;
pub const SPCMD_SSLKP: c_uint = 0x0080	/* SSL Signal Level Keeping */;
pub const SPCMD_SPIMOD_MASK: c_uint = 0x0060	/* SPI Operating Mode (QSPI only) */;
pub const SPCMD_SPIMOD1: c_uint = 0x0040;
pub const SPCMD_SPIMOD0: c_uint = 0x0020;
pub const SPCMD_SPIMOD_SINGLE: c_int = 0;

pub const SPCMD_SPRW: c_uint = 0x0010	/* SPI Read/Write Access (Dual/Quad) */;

pub const SPCMD_BRDV_MASK: c_uint = 0x000c	/* Bit Rate Division Setting */;

pub const SPCMD_CPOL: c_uint = 0x0002	/* Clock Polarity Setting */;
pub const SPCMD_CPHA: c_uint = 0x0001	/* Clock Phase Setting */;
// SPBFCR - Buffer Control Register
pub const SPBFCR_TXRST: c_uint = 0x80	/* Transmit Buffer Data Reset */;
pub const SPBFCR_RXRST: c_uint = 0x40	/* Receive Buffer Data Reset */;
pub const SPBFCR_TXTRG_MASK: c_uint = 0x30	/* Transmit Buffer Data Triggering Number */;
pub const SPBFCR_RXTRG_MASK: c_uint = 0x07	/* Receive Buffer Data Triggering Number */;
// QSPI on R-Car Gen2
pub const SPBFCR_TXTRG_1B: c_uint = 0x00	/* 31 bytes (1 byte available) */;
pub const SPBFCR_TXTRG_32B: c_uint = 0x30	/* 0 byte (32 bytes available) */;
pub const SPBFCR_RXTRG_1B: c_uint = 0x00	/* 1 byte (31 bytes available) */;
pub const SPBFCR_RXTRG_32B: c_uint = 0x07	/* 32 bytes (0 byte available) */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rspi_data {
    pub addr: *mut void __iomem,
    pub speed_hz: u32,
    pub ctlr: *mut spi_controller,
    pub pdev: *mut platform_device,
    pub wait: wait_queue_head_t,
    pub /: *mut *mut spinlock_t lock; / Protects RMW-access to RSPI_SSLP,
    pub clk: *mut clk,
    pub spcmd: u16,
    pub spsr: u8,
    pub sppcr: u8,
    pub tx_irq: int rx_irq,,
    pub ops: *const spi_ops,
    pub dma_callbacked:1: unsigned,
    pub byte_access:1: unsigned,
}

#[no_mangle]
unsafe extern "C" fn rspi_write8(rspi: *const rspi_data, data: u8, offset: u16) {
    static void rspi_write8(const struct rspi_data *rspi, u8 data, u16 offset)
    {
    iowrite8(data, rspi.addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn rspi_write16(rspi: *const rspi_data, data: u16, offset: u16) {
    static void rspi_write16(const struct rspi_data *rspi, u16 data, u16 offset)
    {
    iowrite16(data, rspi.addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn rspi_write32(rspi: *const rspi_data, data: u32, offset: u16) {
    static void rspi_write32(const struct rspi_data *rspi, u32 data, u16 offset)
    {
    iowrite32(data, rspi.addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn rspi_read8(rspi: *const rspi_data, offset: u16) -> u8 {
    static u8 rspi_read8(const struct rspi_data *rspi, u16 offset)
    {
    return ioread8(rspi.addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn rspi_read16(rspi: *const rspi_data, offset: u16) -> u16 {
    static u16 rspi_read16(const struct rspi_data *rspi, u16 offset)
    {
    return ioread16(rspi.addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn rspi_write_data(rspi: *const rspi_data, data: u16) {
    static void rspi_write_data(const struct rspi_data *rspi, u16 data)
    {
    if (rspi.byte_access)
    rspi_write8(rspi, data, RSPI_SPDR);
    else /* 16 bit */
    rspi_write16(rspi, data, RSPI_SPDR);
    }
#[no_mangle]
unsafe extern "C" fn rspi_read_data(rspi: *const rspi_data) -> u16 {
    static u16 rspi_read_data(const struct rspi_data *rspi)
    {
    if (rspi.byte_access)
    return rspi_read8(rspi, RSPI_SPDR);
    else /* 16 bit */
    return rspi_read16(rspi, RSPI_SPDR);
    }
// optional functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_ops {
    pub access_size): *mut *mut *mut int (set_config_register)(struct rspi_data rspi, int,
    int (*transfer_one)(struct spi_controller *ctlr,
    pub xfer): *mut *mut spi_device spi, spi_transfer,
    pub extra_mode_bits: u16,
    pub min_div: u16,
    pub max_div: u16,
    pub flags: u16,
    pub fifo_size: u16,
    pub num_hw_ss: u8,
}

#[no_mangle]
unsafe extern "C" fn rspi_set_rate(rspi: *mut rspi_data) {
    static void rspi_set_rate(struct rspi_data *rspi)
    {
    unsigned long clksrc;
    let mut brdv: c_int = 0, spbr;
    clksrc = clk_get_rate(rspi.clk);
    spbr = DIV_ROUND_UP(clksrc, 2 * rspi.speed_hz) - 1;
    while (spbr > 255 && brdv < 3) {
    brdv++;
    spbr = DIV_ROUND_UP(spbr + 1, 2) - 1;
    }
    rspi_write8(rspi, clamp(spbr, 0, 255), RSPI_SPBR);
    rspi.spcmd |= SPCMD_BRDV(brdv);
    rspi.speed_hz = DIV_ROUND_UP(clksrc, (2U << brdv) * (spbr + 1));
    }
//
// functions for RSPI on legacy SH
//
#[no_mangle]
unsafe extern "C" fn rspi_set_config_register(rspi: *mut rspi_data, access_size: c_int) -> c_int {
    static int rspi_set_config_register(struct rspi_data *rspi, int access_size)
    {
// Sets output mode, MOSI signal, and (optionally) loopback
    rspi_write8(rspi, rspi.sppcr, RSPI_SPPCR);
// Sets transfer bit rate
    rspi_set_rate(rspi);
// Disable dummy transmission, set 16-bit word access, 1 frame
    rspi_write8(rspi, 0, RSPI_SPDCR);
    rspi.byte_access = 0;
// Sets RSPCK, SSL, next-access delay value
    rspi_write8(rspi, 0x00, RSPI_SPCKD);
    rspi_write8(rspi, 0x00, RSPI_SSLND);
    rspi_write8(rspi, 0x00, RSPI_SPND);
// Sets parity, interrupt mask
    rspi_write8(rspi, 0x00, RSPI_SPCR2);
// Resets sequencer
    rspi_write8(rspi, 0, RSPI_SPSCR);
    rspi.spcmd |= SPCMD_SPB_8_TO_16(access_size);
    rspi_write16(rspi, rspi.spcmd, RSPI_SPCMD0);
// Sets RSPI mode
    rspi_write8(rspi, SPCR_MSTR, RSPI_SPCR);
    return 0;
    }
//
// functions for RSPI on RZ
//
#[no_mangle]
unsafe extern "C" fn rspi_rz_set_config_register(rspi: *mut rspi_data, access_size: c_int) -> c_int {
    static int rspi_rz_set_config_register(struct rspi_data *rspi, int access_size)
    {
// Sets output mode, MOSI signal, and (optionally) loopback
    rspi_write8(rspi, rspi.sppcr, RSPI_SPPCR);
// Sets transfer bit rate
    rspi_set_rate(rspi);
// Disable dummy transmission, set byte access
    rspi_write8(rspi, SPDCR_SPLBYTE, RSPI_SPDCR);
    rspi.byte_access = 1;
// Sets RSPCK, SSL, next-access delay value
    rspi_write8(rspi, 0x00, RSPI_SPCKD);
    rspi_write8(rspi, 0x00, RSPI_SSLND);
    rspi_write8(rspi, 0x00, RSPI_SPND);
// Resets sequencer
    rspi_write8(rspi, 0, RSPI_SPSCR);
    rspi.spcmd |= SPCMD_SPB_8_TO_16(access_size);
    rspi_write16(rspi, rspi.spcmd, RSPI_SPCMD0);
// Sets RSPI mode
    rspi_write8(rspi, SPCR_MSTR, RSPI_SPCR);
    return 0;
    }
//
// functions for QSPI
//
#[no_mangle]
unsafe extern "C" fn qspi_set_config_register(rspi: *mut rspi_data, access_size: c_int) -> c_int {
    static int qspi_set_config_register(struct rspi_data *rspi, int access_size)
    {
    unsigned long clksrc;
    let mut brdv: c_int = 0, spbr;
// Sets output mode, MOSI signal, and (optionally) loopback
    rspi_write8(rspi, rspi.sppcr, RSPI_SPPCR);
// Sets transfer bit rate
    clksrc = clk_get_rate(rspi.clk);
    if (rspi.speed_hz >= clksrc) {
    spbr = 0;
    rspi.speed_hz = clksrc;
    } else {
    spbr = DIV_ROUND_UP(clksrc, 2 * rspi.speed_hz);
    while (spbr > 255 && brdv < 3) {
    brdv++;
    spbr = DIV_ROUND_UP(spbr, 2);
    }
    spbr = clamp(spbr, 0, 255);
    rspi.speed_hz = DIV_ROUND_UP(clksrc, (2U << brdv) * spbr);
    }
    rspi_write8(rspi, spbr, RSPI_SPBR);
    rspi.spcmd |= SPCMD_BRDV(brdv);
// Disable dummy transmission, set byte access
    rspi_write8(rspi, 0, RSPI_SPDCR);
    rspi.byte_access = 1;
// Sets RSPCK, SSL, next-access delay value
    rspi_write8(rspi, 0x00, RSPI_SPCKD);
    rspi_write8(rspi, 0x00, RSPI_SSLND);
    rspi_write8(rspi, 0x00, RSPI_SPND);
// Data Length Setting
    if (access_size == 8)
    rspi.spcmd |= SPCMD_SPB_8BIT;
#[no_mangle]
pub unsafe extern "C" fn if(16: access_size ==) -> else {
    else if (access_size == 16)
    rspi.spcmd |= SPCMD_SPB_16BIT;
    else
    rspi.spcmd |= SPCMD_SPB_32BIT;
    rspi.spcmd |= SPCMD_SCKDEN | SPCMD_SLNDEN | SPCMD_SPNDEN;
// Resets transfer data length
    rspi_write32(rspi, 0, QSPI_SPBMUL0);
// Resets transmit and receive buffer
    rspi_write8(rspi, SPBFCR_TXRST | SPBFCR_RXRST, QSPI_SPBFCR);
// Sets buffer to allow normal operation
    rspi_write8(rspi, 0x00, QSPI_SPBFCR);
// Resets sequencer
    rspi_write8(rspi, 0, RSPI_SPSCR);
    rspi_write16(rspi, rspi.spcmd, RSPI_SPCMD0);
// Sets RSPI mode
    rspi_write8(rspi, SPCR_MSTR, RSPI_SPCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qspi_update(rspi: *const rspi_data, mask: u8, val: u8, reg: u8) {
    static void qspi_update(const struct rspi_data *rspi, u8 mask, u8 val, u8 reg)
    {
    u8 data;
    data = rspi_read8(rspi, reg);
    data &= ~mask;
    data |= (val & mask);
    rspi_write8(rspi, data, reg);
    }
    static unsigned int qspi_set_send_trigger(struct rspi_data *rspi,
    unsigned int len)
    {
    unsigned int n;
    n = min(len, QSPI_BUFFER_SIZE);
    if (len >= QSPI_BUFFER_SIZE) {
// sets triggering number to 32 bytes
    qspi_update(rspi, SPBFCR_TXTRG_MASK,
    SPBFCR_TXTRG_32B, QSPI_SPBFCR);
    } else {
// sets triggering number to 1 byte
    qspi_update(rspi, SPBFCR_TXTRG_MASK,
    SPBFCR_TXTRG_1B, QSPI_SPBFCR);
    }
    return n;
    }
#[no_mangle]
unsafe extern "C" fn qspi_set_receive_trigger(rspi: *mut rspi_data, len: c_uint) -> c_int {
    static int qspi_set_receive_trigger(struct rspi_data *rspi, unsigned int len)
    {
    unsigned int n;
    n = min(len, QSPI_BUFFER_SIZE);
    if (len >= QSPI_BUFFER_SIZE) {
// sets triggering number to 32 bytes
    qspi_update(rspi, SPBFCR_RXTRG_MASK,
    SPBFCR_RXTRG_32B, QSPI_SPBFCR);
    } else {
// sets triggering number to 1 byte
    qspi_update(rspi, SPBFCR_RXTRG_MASK,
    SPBFCR_RXTRG_1B, QSPI_SPBFCR);
    }
    return n;
    }
#[no_mangle]
unsafe extern "C" fn rspi_enable_irq(rspi: *const rspi_data, enable: u8) {
    static void rspi_enable_irq(const struct rspi_data *rspi, u8 enable)
    {
    rspi_write8(rspi, rspi_read8(rspi, RSPI_SPCR) | enable, RSPI_SPCR);
    }
#[no_mangle]
unsafe extern "C" fn rspi_disable_irq(rspi: *const rspi_data, disable: u8) {
    static void rspi_disable_irq(const struct rspi_data *rspi, u8 disable)
    {
    rspi_write8(rspi, rspi_read8(rspi, RSPI_SPCR) & ~disable, RSPI_SPCR);
    }
    static int rspi_wait_for_interrupt(struct rspi_data *rspi, u8 wait_mask,
    u8 enable_bit)
    {
    int ret;
    rspi.spsr = rspi_read8(rspi, RSPI_SPSR);
    if (rspi.spsr & wait_mask)
    return 0;
    rspi_enable_irq(rspi, enable_bit);
    ret = wait_event_timeout(rspi.wait, rspi.spsr & wait_mask, HZ);
    if (ret == 0 && !(rspi.spsr & wait_mask))
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rspi_wait_for_tx_empty(rspi: *mut rspi_data) -> c_int {
    static inline int rspi_wait_for_tx_empty(struct rspi_data *rspi)
    {
    return rspi_wait_for_interrupt(rspi, SPSR_SPTEF, SPCR_SPTIE);
    }
#[no_mangle]
pub unsafe extern "C" fn rspi_wait_for_rx_full(rspi: *mut rspi_data) -> c_int {
    static inline int rspi_wait_for_rx_full(struct rspi_data *rspi)
    {
    return rspi_wait_for_interrupt(rspi, SPSR_SPRF, SPCR_SPRIE);
    }
#[no_mangle]
unsafe extern "C" fn rspi_data_out(rspi: *mut rspi_data, data: u8) -> c_int {
    static int rspi_data_out(struct rspi_data *rspi, u8 data)
    {
    let mut error: c_int = rspi_wait_for_tx_empty(rspi);
    if (error < 0) {
    dev_err(&rspi.ctlr.dev, "transmit timeout\n");
    return error;
    }
    rspi_write_data(rspi, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rspi_data_in(rspi: *mut rspi_data) -> c_int {
    static int rspi_data_in(struct rspi_data *rspi)
    {
    int error;
    u8 data;
    error = rspi_wait_for_rx_full(rspi);
    if (error < 0) {
    dev_err(&rspi.ctlr.dev, "receive timeout\n");
    return error;
    }
    data = rspi_read_data(rspi);
    return data;
    }
    static int rspi_pio_transfer(struct rspi_data *rspi, const u8 *tx, u8 *rx,
    unsigned int n)
    {
    while (n-- > 0) {
    if (tx) {
    let mut ret: c_int = rspi_data_out(rspi, *tx++);
    if (ret < 0)
    return ret;
    }
    if (rx) {
    let mut ret: c_int = rspi_data_in(rspi);
    if (ret < 0)
    return ret;
// rx++ = ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rspi_dma_complete(arg: *mut c_void) {
    static void rspi_dma_complete(void *arg)
    {
    struct rspi_data *rspi = arg;
    rspi.dma_callbacked = 1;
    wake_up_interruptible(&rspi.wait);
    }
    static int rspi_dma_transfer(struct rspi_data *rspi, struct sg_table *tx,
    struct sg_table *rx)
    {
    struct dma_async_tx_descriptor *desc_tx = core::ptr::null_mut(), *desc_rx = core::ptr::null_mut();
    let mut irq_mask: u8 = 0;
    let mut other_irq: c_uint = 0;
    dma_cookie_t cookie;
    int ret;
// First prepare and submit the DMA request(s), as this may fail
    if (rx) {
    desc_rx = dmaengine_prep_slave_sg(rspi.ctlr.dma_rx, rx.sgl,
    rx.nents, DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_rx) {
    ret = -EAGAIN;
    goto no_dma_rx;
    }
    desc_rx.callback = rspi_dma_complete;
    desc_rx.callback_param = rspi;
    cookie = dmaengine_submit(desc_rx);
    if (dma_submit_error(cookie)) {
    ret = cookie;
    goto no_dma_rx;
    }
    irq_mask |= SPCR_SPRIE;
    }
    if (tx) {
    desc_tx = dmaengine_prep_slave_sg(rspi.ctlr.dma_tx, tx.sgl,
    tx.nents, DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_tx) {
    ret = -EAGAIN;
    goto no_dma_tx;
    }
    if (rx) {
// No callback
    desc_tx.callback = core::ptr::null_mut();
    } else {
    desc_tx.callback = rspi_dma_complete;
    desc_tx.callback_param = rspi;
    }
    cookie = dmaengine_submit(desc_tx);
    if (dma_submit_error(cookie)) {
    ret = cookie;
    goto no_dma_tx;
    }
    irq_mask |= SPCR_SPTIE;
    }
//
// DMAC needs SPxIE, but if SPxIE is set, the IRQ routine will be
// called. So, this driver disables the IRQ while DMA transfer.
//
    if (tx)
    disable_irq(other_irq = rspi.tx_irq);
    if (rx && rspi.rx_irq != other_irq)
    disable_irq(rspi.rx_irq);
    rspi_enable_irq(rspi, irq_mask);
    rspi.dma_callbacked = 0;
// Now start DMA
    if (rx)
    dma_async_issue_pending(rspi.ctlr.dma_rx);
    if (tx)
    dma_async_issue_pending(rspi.ctlr.dma_tx);
    ret = wait_event_interruptible_timeout(rspi.wait,
    rspi.dma_callbacked, HZ);
    if (ret > 0 && rspi.dma_callbacked) {
    ret = 0;
    if (tx)
    dmaengine_synchronize(rspi.ctlr.dma_tx);
    if (rx)
    dmaengine_synchronize(rspi.ctlr.dma_rx);
    } else {
    if (!ret) {
    dev_err(&rspi.ctlr.dev, "DMA timeout\n");
    ret = -ETIMEDOUT;
    }
    if (tx)
    dmaengine_terminate_sync(rspi.ctlr.dma_tx);
    if (rx)
    dmaengine_terminate_sync(rspi.ctlr.dma_rx);
    }
    rspi_disable_irq(rspi, irq_mask);
    if (tx)
    enable_irq(rspi.tx_irq);
    if (rx && rspi.rx_irq != other_irq)
    enable_irq(rspi.rx_irq);
    return ret;
    no_dma_tx:
    if (rx)
    dmaengine_terminate_sync(rspi.ctlr.dma_rx);
    no_dma_rx:
    if (ret == -EAGAIN) {
    dev_warn_once(&rspi.ctlr.dev,
    "DMA not available, falling back to PIO\n");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rspi_receive_init(rspi: *const rspi_data) {
    static void rspi_receive_init(const struct rspi_data *rspi)
    {
    u8 spsr;
    spsr = rspi_read8(rspi, RSPI_SPSR);
    if (spsr & SPSR_SPRF)
    rspi_read_data(rspi);	/* dummy read */
    if (spsr & SPSR_OVRF)
    rspi_write8(rspi, rspi_read8(rspi, RSPI_SPSR) & ~SPSR_OVRF,
    RSPI_SPSR);
    }
#[no_mangle]
unsafe extern "C" fn rspi_rz_receive_init(rspi: *const rspi_data) {
    static void rspi_rz_receive_init(const struct rspi_data *rspi)
    {
    rspi_receive_init(rspi);
    rspi_write8(rspi, SPBFCR_TXRST | SPBFCR_RXRST, RSPI_SPBFCR);
    rspi_write8(rspi, 0, RSPI_SPBFCR);
    }
#[no_mangle]
unsafe extern "C" fn qspi_receive_init(rspi: *const rspi_data) {
    static void qspi_receive_init(const struct rspi_data *rspi)
    {
    u8 spsr;
    spsr = rspi_read8(rspi, RSPI_SPSR);
    if (spsr & SPSR_SPRF)
    rspi_read_data(rspi);   /* dummy read */
    rspi_write8(rspi, SPBFCR_TXRST | SPBFCR_RXRST, QSPI_SPBFCR);
    rspi_write8(rspi, 0, QSPI_SPBFCR);
    }
    static bool __rspi_can_dma(const struct rspi_data *rspi,
    const struct spi_transfer *xfer)
    {
    return xfer.len > rspi.ops.fifo_size;
    }
    static bool rspi_can_dma(struct spi_controller *ctlr, struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct rspi_data *rspi = spi_controller_get_devdata(ctlr);
    return __rspi_can_dma(rspi, xfer);
    }
    static int rspi_dma_check_then_transfer(struct rspi_data *rspi,
    struct spi_transfer *xfer)
    {
    if (!rspi.ctlr.can_dma || !__rspi_can_dma(rspi, xfer))
    return -EAGAIN;
// rx_buf can be NULL on RSPI on SH in TX-only Mode
    return rspi_dma_transfer(rspi, &xfer.tx_sg,
    xfer.rx_buf ? &xfer.rx_sg : core::ptr::null_mut());
    }
    static int rspi_common_transfer(struct rspi_data *rspi,
    struct spi_transfer *xfer)
    {
    int ret;
    xfer.effective_speed_hz = rspi.speed_hz;
    ret = rspi_dma_check_then_transfer(rspi, xfer);
    if (ret != -EAGAIN)
    return ret;
    ret = rspi_pio_transfer(rspi, xfer.tx_buf, xfer.rx_buf, xfer.len);
    if (ret < 0)
    return ret;
// Wait for the last transmission
    rspi_wait_for_tx_empty(rspi);
    return 0;
    }
    static int rspi_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi, struct spi_transfer *xfer)
    {
    struct rspi_data *rspi = spi_controller_get_devdata(ctlr);
    u8 spcr;
    spcr = rspi_read8(rspi, RSPI_SPCR);
    if (xfer.rx_buf) {
    rspi_receive_init(rspi);
    spcr &= ~SPCR_TXMD;
    } else {
    spcr |= SPCR_TXMD;
    }
    rspi_write8(rspi, spcr, RSPI_SPCR);
    return rspi_common_transfer(rspi, xfer);
    }
    static int rspi_rz_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct rspi_data *rspi = spi_controller_get_devdata(ctlr);
    rspi_rz_receive_init(rspi);
    return rspi_common_transfer(rspi, xfer);
    }
    static int qspi_trigger_transfer_out_in(struct rspi_data *rspi, const u8 *tx,
    u8 *rx, unsigned int len)
    {
    unsigned int i, n;
    int ret;
    while (len > 0) {
    n = qspi_set_send_trigger(rspi, len);
    qspi_set_receive_trigger(rspi, len);
    ret = rspi_wait_for_tx_empty(rspi);
    if (ret < 0) {
    dev_err(&rspi.ctlr.dev, "transmit timeout\n");
    return ret;
    }
    for (i = 0; i < n; i++)
    rspi_write_data(rspi, *tx++);
    ret = rspi_wait_for_rx_full(rspi);
    if (ret < 0) {
    dev_err(&rspi.ctlr.dev, "receive timeout\n");
    return ret;
    }
    for (i = 0; i < n; i++)
// rx++ = rspi_read_data(rspi);
    len -= n;
    }
    return 0;
    }
    static int qspi_transfer_out_in(struct rspi_data *rspi,
    struct spi_transfer *xfer)
    {
    int ret;
    qspi_receive_init(rspi);
    ret = rspi_dma_check_then_transfer(rspi, xfer);
    if (ret != -EAGAIN)
    return ret;
    return qspi_trigger_transfer_out_in(rspi, xfer.tx_buf,
    xfer.rx_buf, xfer.len);
    }
#[no_mangle]
unsafe extern "C" fn qspi_transfer_out(rspi: *mut rspi_data, xfer: *mut spi_transfer) -> c_int {
    static int qspi_transfer_out(struct rspi_data *rspi, struct spi_transfer *xfer)
    {
    const u8 *tx = xfer.tx_buf;
    let mut n: c_uint = xfer.len;
    unsigned int i, len;
    int ret;
    if (rspi.ctlr.can_dma && __rspi_can_dma(rspi, xfer)) {
    ret = rspi_dma_transfer(rspi, &xfer.tx_sg, core::ptr::null_mut());
    if (ret != -EAGAIN)
    return ret;
    }
    while (n > 0) {
    len = qspi_set_send_trigger(rspi, n);
    ret = rspi_wait_for_tx_empty(rspi);
    if (ret < 0) {
    dev_err(&rspi.ctlr.dev, "transmit timeout\n");
    return ret;
    }
    for (i = 0; i < len; i++)
    rspi_write_data(rspi, *tx++);
    n -= len;
    }
// Wait for the last transmission
    rspi_wait_for_tx_empty(rspi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qspi_transfer_in(rspi: *mut rspi_data, xfer: *mut spi_transfer) -> c_int {
    static int qspi_transfer_in(struct rspi_data *rspi, struct spi_transfer *xfer)
    {
    u8 *rx = xfer.rx_buf;
    let mut n: c_uint = xfer.len;
    unsigned int i, len;
    int ret;
    if (rspi.ctlr.can_dma && __rspi_can_dma(rspi, xfer)) {
    ret = rspi_dma_transfer(rspi, core::ptr::null_mut(), &xfer.rx_sg);
    if (ret != -EAGAIN)
    return ret;
    }
    while (n > 0) {
    len = qspi_set_receive_trigger(rspi, n);
    ret = rspi_wait_for_rx_full(rspi);
    if (ret < 0) {
    dev_err(&rspi.ctlr.dev, "receive timeout\n");
    return ret;
    }
    for (i = 0; i < len; i++)
// rx++ = rspi_read_data(rspi);
    n -= len;
    }
    return 0;
    }
    static int qspi_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi, struct spi_transfer *xfer)
    {
    struct rspi_data *rspi = spi_controller_get_devdata(ctlr);
    xfer.effective_speed_hz = rspi.speed_hz;
    if (spi.mode & SPI_LOOP) {
    return qspi_transfer_out_in(rspi, xfer);
    } else if (xfer.tx_nbits > SPI_NBITS_SINGLE) {
// Quad or Dual SPI Write
    return qspi_transfer_out(rspi, xfer);
    } else if (xfer.rx_nbits > SPI_NBITS_SINGLE) {
// Quad or Dual SPI Read
    return qspi_transfer_in(rspi, xfer);
    } else {
// Single SPI Transfer
    return qspi_transfer_out_in(rspi, xfer);
    }
    }
#[no_mangle]
unsafe extern "C" fn qspi_transfer_mode(xfer: *const spi_transfer) -> u16 {
    static u16 qspi_transfer_mode(const struct spi_transfer *xfer)
    {
    if (xfer.tx_buf)
    switch (xfer.tx_nbits) {
    case SPI_NBITS_QUAD:
    return SPCMD_SPIMOD_QUAD;
    case SPI_NBITS_DUAL:
    return SPCMD_SPIMOD_DUAL;
    default:
    return 0;
    }
    if (xfer.rx_buf)
    switch (xfer.rx_nbits) {
    case SPI_NBITS_QUAD:
    return SPCMD_SPIMOD_QUAD | SPCMD_SPRW;
    case SPI_NBITS_DUAL:
    return SPCMD_SPIMOD_DUAL | SPCMD_SPRW;
    default:
    return 0;
    }
    return 0;
    }
    static int qspi_setup_sequencer(struct rspi_data *rspi,
    const struct spi_message *msg)
    {
    const struct spi_transfer *xfer;
    let mut i: c_uint = 0, len = 0;
    let mut current_mode: u16 = 0xffff, mode;
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
    mode = qspi_transfer_mode(xfer);
    if (mode == current_mode) {
    len += xfer.len;
    continue;
    }
// Transfer mode change
    if (i) {
// Set transfer data length of previous transfer
    rspi_write32(rspi, len, QSPI_SPBMUL(i - 1));
    }
    if (i >= QSPI_NUM_SPCMD) {
    dev_err(&msg.spi.dev,
    "Too many different transfer modes");
    return -EINVAL;
    }
// Program transfer mode for this transfer
    rspi_write16(rspi, rspi.spcmd | mode, RSPI_SPCMD(i));
    current_mode = mode;
    len = xfer.len;
    i++;
    }
    if (i) {
// Set final transfer data length and sequence length
    rspi_write32(rspi, len, QSPI_SPBMUL(i - 1));
    rspi_write8(rspi, i - 1, RSPI_SPSCR);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rspi_setup(spi: *mut spi_device) -> c_int {
    static int rspi_setup(struct spi_device *spi)
    {
    struct rspi_data *rspi = spi_controller_get_devdata(spi.controller);
    u8 sslp;
    if (spi_get_csgpiod(spi, 0))
    return 0;
    pm_runtime_get_sync(&rspi.pdev.dev);
    spin_lock_irq(&rspi.lock);
    sslp = rspi_read8(rspi, RSPI_SSLP);
    if (spi.mode & SPI_CS_HIGH)
    sslp |= SSLP_SSLP(spi_get_chipselect(spi, 0));
    else
    sslp &= ~SSLP_SSLP(spi_get_chipselect(spi, 0));
    rspi_write8(rspi, sslp, RSPI_SSLP);
    spin_unlock_irq(&rspi.lock);
    pm_runtime_put(&rspi.pdev.dev);
    return 0;
    }
    static int rspi_prepare_message(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    struct rspi_data *rspi = spi_controller_get_devdata(ctlr);
    struct spi_device *spi = msg.spi;
    const struct spi_transfer *xfer;
    int ret;
//
// As the Bit Rate Register must not be changed while the device is
// active, all transfers in a message must use the same bit rate.
// In theory, the sequencer could be enabled, and each Command Register
// could divide the base bit rate by a different value.
// However, most RSPI variants do not have Transfer Data Length
// Multiplier Setting Registers, so each sequence step would be limited
// to a single word, making this feature unsuitable for large
// transfers, which would gain most from it.
//
    rspi.speed_hz = spi.max_speed_hz;
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
    if (xfer.speed_hz < rspi.speed_hz)
    rspi.speed_hz = xfer.speed_hz;
    }
    rspi.spcmd = SPCMD_SSLKP;
    if (spi.mode & SPI_CPOL)
    rspi.spcmd |= SPCMD_CPOL;
    if (spi.mode & SPI_CPHA)
    rspi.spcmd |= SPCMD_CPHA;
    if (spi.mode & SPI_LSB_FIRST)
    rspi.spcmd |= SPCMD_LSBF;
// Configure slave signal to assert
    rspi.spcmd |= SPCMD_SSLA(spi_get_csgpiod(spi, 0) ? rspi.ctlr.unused_native_cs
    : spi_get_chipselect(spi, 0));
// CMOS output mode and MOSI signal from previous transfer
    rspi.sppcr = 0;
    if (spi.mode & SPI_LOOP)
    rspi.sppcr |= SPPCR_SPLP;
    rspi.ops.set_config_register(rspi, 8);
    if (msg.spi.mode &
    (SPI_TX_DUAL | SPI_TX_QUAD | SPI_RX_DUAL | SPI_RX_QUAD)) {
// Setup sequencer for messages with multiple transfer modes
    ret = qspi_setup_sequencer(rspi, msg);
    if (ret < 0)
    return ret;
    }
// Enable SPI function in master mode
    rspi_write8(rspi, rspi_read8(rspi, RSPI_SPCR) | SPCR_SPE, RSPI_SPCR);
    return 0;
    }
    static int rspi_unprepare_message(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    struct rspi_data *rspi = spi_controller_get_devdata(ctlr);
// Disable SPI function
    rspi_write8(rspi, rspi_read8(rspi, RSPI_SPCR) & ~SPCR_SPE, RSPI_SPCR);
// Reset sequencer for Single SPI Transfers
    rspi_write16(rspi, rspi.spcmd, RSPI_SPCMD0);
    rspi_write8(rspi, 0, RSPI_SPSCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rspi_irq_mux(irq: c_int, _sr: *mut c_void) -> irqreturn_t {
    static irqreturn_t rspi_irq_mux(int irq, void *_sr)
    {
    struct rspi_data *rspi = _sr;
    u8 spsr;
    let mut ret: irqreturn_t = IRQ_NONE;
    let mut disable_irq: u8 = 0;
    rspi.spsr = spsr = rspi_read8(rspi, RSPI_SPSR);
    if (spsr & SPSR_SPRF)
    disable_irq |= SPCR_SPRIE;
    if (spsr & SPSR_SPTEF)
    disable_irq |= SPCR_SPTIE;
    if (disable_irq) {
    ret = IRQ_HANDLED;
    rspi_disable_irq(rspi, disable_irq);
    wake_up(&rspi.wait);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rspi_irq_rx(irq: c_int, _sr: *mut c_void) -> irqreturn_t {
    static irqreturn_t rspi_irq_rx(int irq, void *_sr)
    {
    struct rspi_data *rspi = _sr;
    u8 spsr;
    rspi.spsr = spsr = rspi_read8(rspi, RSPI_SPSR);
    if (spsr & SPSR_SPRF) {
    rspi_disable_irq(rspi, SPCR_SPRIE);
    wake_up(&rspi.wait);
    return IRQ_HANDLED;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rspi_irq_tx(irq: c_int, _sr: *mut c_void) -> irqreturn_t {
    static irqreturn_t rspi_irq_tx(int irq, void *_sr)
    {
    struct rspi_data *rspi = _sr;
    u8 spsr;
    rspi.spsr = spsr = rspi_read8(rspi, RSPI_SPSR);
    if (spsr & SPSR_SPTEF) {
    rspi_disable_irq(rspi, SPCR_SPTIE);
    wake_up(&rspi.wait);
    return IRQ_HANDLED;
    }
    return 0;
    }
    static struct dma_chan *rspi_request_dma_chan(struct device *dev,
    enum dma_transfer_direction dir,
    unsigned int id,
    dma_addr_t port_addr)
    {
    dma_cap_mask_t mask;
    struct dma_chan *chan;
    struct dma_slave_config cfg;
    int ret;
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
    chan = dma_request_slave_channel_compat(mask, shdma_chan_filter,
    (void *)(unsigned long)id, dev,
    dir == DMA_MEM_TO_DEV ? "tx" : "rx");
    if (!chan) {
    dev_warn(dev, "dma_request_slave_channel_compat failed\n");
    return core::ptr::null_mut();
    }
    memset(&cfg, 0, sizeof(cfg));
    cfg.dst_addr = port_addr + RSPI_SPDR;
    cfg.src_addr = port_addr + RSPI_SPDR;
    cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    cfg.direction = dir;
    ret = dmaengine_slave_config(chan, &cfg);
    if (ret) {
    dev_warn(dev, "dmaengine_slave_config failed %d\n", ret);
    dma_release_channel(chan);
    return core::ptr::null_mut();
    }
    return chan;
    }
    static int rspi_request_dma(struct device *dev, struct spi_controller *ctlr,
    const struct resource *res)
    {
    unsigned int dma_tx_id, dma_rx_id;
    if (dev.of_node) {
// In the OF case we will get the slave IDs from the DT
    dma_tx_id = 0;
    dma_rx_id = 0;
    } else {
// The driver assumes no error.
    return 0;
    }
    ctlr.dma_tx = rspi_request_dma_chan(dev, DMA_MEM_TO_DEV, dma_tx_id,
    res.start);
    if (!ctlr.dma_tx)
    return -ENODEV;
    ctlr.dma_rx = rspi_request_dma_chan(dev, DMA_DEV_TO_MEM, dma_rx_id,
    res.start);
    if (!ctlr.dma_rx) {
    dma_release_channel(ctlr.dma_tx);
    ctlr.dma_tx = core::ptr::null_mut();
    return -ENODEV;
    }
    ctlr.can_dma = rspi_can_dma;
    dev_info(dev, "DMA available");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rspi_release_dma(ctlr: *mut spi_controller) {
    static void rspi_release_dma(struct spi_controller *ctlr)
    {
    if (ctlr.dma_tx)
    dma_release_channel(ctlr.dma_tx);
    if (ctlr.dma_rx)
    dma_release_channel(ctlr.dma_rx);
    }
#[no_mangle]
unsafe extern "C" fn rspi_remove(pdev: *mut platform_device) {
    static void rspi_remove(struct platform_device *pdev)
    {
    struct rspi_data *rspi = platform_get_drvdata(pdev);
    spi_unregister_controller(rspi.ctlr);
    rspi_release_dma(rspi.ctlr);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct spi_ops rspi_ops = {
    .set_config_register =	rspi_set_config_register,
    .transfer_one =		rspi_transfer_one,
    .min_div =		2,
    .max_div =		4096,
    .flags =		SPI_CONTROLLER_MUST_TX,
    .fifo_size =		8,
    .num_hw_ss =		2,
    };
    static const struct spi_ops rspi_rz_ops __maybe_unused = {
    .set_config_register =	rspi_rz_set_config_register,
    .transfer_one =		rspi_rz_transfer_one,
    .min_div =		2,
    .max_div =		4096,
    .flags =		SPI_CONTROLLER_MUST_RX | SPI_CONTROLLER_MUST_TX,
    .fifo_size =		8,	/* 8 for TX, 32 for RX */
    .num_hw_ss =		1,
    };
    static const struct spi_ops qspi_ops __maybe_unused = {
    .set_config_register =	qspi_set_config_register,
    .transfer_one =		qspi_transfer_one,
    .extra_mode_bits =	SPI_TX_DUAL | SPI_TX_QUAD |
    SPI_RX_DUAL | SPI_RX_QUAD,
    .min_div =		1,
    .max_div =		4080,
    .flags =		SPI_CONTROLLER_MUST_RX | SPI_CONTROLLER_MUST_TX,
    .fifo_size =		32,
    .num_hw_ss =		1,
    };
    static const struct of_device_id rspi_of_match[] __maybe_unused = {
// RSPI on legacy SH
    { .compatible = "renesas,rspi", .data = &rspi_ops },
// RSPI on RZ/A1H
    { .compatible = "renesas,rspi-rz", .data = &rspi_rz_ops },
// QSPI on R-Car Gen2
    { .compatible = "renesas,qspi", .data = &qspi_ops },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rspi_of_match);

#[no_mangle]
unsafe extern "C" fn rspi_parse_dt(dev: *mut device, ctlr: *mut spi_controller) -> c_int {
    static int rspi_parse_dt(struct device *dev, struct spi_controller *ctlr)
    {
    struct reset_control *rstc;
    u32 num_cs;
    int error;
// Parse DT properties
    error = of_property_read_u32(dev.of_node, "num-cs", &num_cs);
    if (error) {
    dev_err(dev, "of_property_read_u32 num-cs failed %d\n", error);
    return error;
    }
    ctlr.num_chipselect = num_cs;
    rstc = devm_reset_control_get_optional_exclusive_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(rstc))
    return dev_err_probe(dev, PTR_ERR(rstc),
    "failed to get reset ctrl and deassert reset\n");
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn rspi_parse_dt(dev: *mut device, ctlr: *mut spi_controller) -> c_int {
    static inline int rspi_parse_dt(struct device *dev, struct spi_controller *ctlr)
    {
    return -EINVAL;
    }

    static int rspi_request_irq(struct device *dev, unsigned int irq,
    irq_handler_t handler, const char *suffix,
    void *dev_id)
    {
    const char *name = devm_kasprintf(dev, GFP_KERNEL, "%s:%s",
    dev_name(dev), suffix);
    if (!name)
    return -ENOMEM;
    return devm_request_irq(dev, irq, handler, 0, name, dev_id);
    }
#[no_mangle]
unsafe extern "C" fn rspi_probe(pdev: *mut platform_device) -> c_int {
    static int rspi_probe(struct platform_device *pdev)
    {
    struct resource *res;
    struct spi_controller *ctlr;
    struct rspi_data *rspi;
    int ret;
    const struct spi_ops *ops;
    unsigned long clksrc;
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(struct rspi_data));
    if (ctlr == core::ptr::null_mut())
    return -ENOMEM;
    ops = of_device_get_match_data(&pdev.dev);
    if (ops) {
    ret = rspi_parse_dt(&pdev.dev, ctlr);
    if (ret)
    return ret;
    } else {
    ops = (struct spi_ops *)pdev.id_entry.driver_data;
    ctlr.num_chipselect = 2; /* default */
    }
    rspi = spi_controller_get_devdata(ctlr);
    platform_set_drvdata(pdev, rspi);
    rspi.ops = ops;
    rspi.ctlr = ctlr;
    rspi.addr = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(rspi.addr))
    return PTR_ERR(rspi.addr);
    rspi.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rspi.clk)) {
    dev_err(&pdev.dev, "cannot get clock\n");
    return PTR_ERR(rspi.clk);
    }
    rspi.pdev = pdev;
    pm_runtime_enable(&pdev.dev);
    init_waitqueue_head(&rspi.wait);
    spin_lock_init(&rspi.lock);
    ctlr.bus_num = pdev.id;
    ctlr.setup = rspi_setup;
    ctlr.auto_runtime_pm = true;
    ctlr.transfer_one = ops.transfer_one;
    ctlr.prepare_message = rspi_prepare_message;
    ctlr.unprepare_message = rspi_unprepare_message;
    ctlr.mode_bits = SPI_CPHA | SPI_CPOL | SPI_CS_HIGH | SPI_LSB_FIRST |
    SPI_LOOP | ops.extra_mode_bits;
    clksrc = clk_get_rate(rspi.clk);
    ctlr.min_speed_hz = DIV_ROUND_UP(clksrc, ops.max_div);
    ctlr.max_speed_hz = DIV_ROUND_UP(clksrc, ops.min_div);
    ctlr.flags = ops.flags;
    ctlr.use_gpio_descriptors = true;
    ctlr.max_native_cs = rspi.ops.num_hw_ss;
    ret = platform_get_irq_byname_optional(pdev, "rx");
    if (ret < 0) {
    ret = platform_get_irq_byname_optional(pdev, "mux");
    if (ret < 0)
    ret = platform_get_irq(pdev, 0);
    if (ret >= 0)
    rspi.rx_irq = rspi.tx_irq = ret;
    } else {
    rspi.rx_irq = ret;
    ret = platform_get_irq_byname(pdev, "tx");
    if (ret >= 0)
    rspi.tx_irq = ret;
    }
    if (rspi.rx_irq == rspi.tx_irq) {
// Single multiplexed interrupt
    ret = rspi_request_irq(&pdev.dev, rspi.rx_irq, rspi_irq_mux,
    "mux", rspi);
    } else {
// Multi-interrupt mode, only SPRI and SPTI are used
    ret = rspi_request_irq(&pdev.dev, rspi.rx_irq, rspi_irq_rx,
    "rx", rspi);
    if (!ret)
    ret = rspi_request_irq(&pdev.dev, rspi.tx_irq,
    rspi_irq_tx, "tx", rspi);
    }
    if (ret < 0) {
    dev_err(&pdev.dev, "request_irq error\n");
    goto error2;
    }
    ret = rspi_request_dma(&pdev.dev, ctlr, res);
    if (ret < 0)
    dev_warn(&pdev.dev, "DMA not available, using PIO\n");
    ret = spi_register_controller(ctlr);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to register controller\n");
    goto error3;
    }
    dev_info(&pdev.dev, "probed\n");
    return 0;
    error3:
    rspi_release_dma(ctlr);
    error2:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
    static const struct platform_device_id spi_driver_ids[] = {
    { .name = "rspi", .driver_data = (kernel_ulong_t)&rspi_ops },
    { }
    };
    MODULE_DEVICE_TABLE(platform, spi_driver_ids);
#[no_mangle]
unsafe extern "C" fn rspi_suspend(dev: *mut device) -> c_int {
    static int rspi_suspend(struct device *dev)
    {
    struct rspi_data *rspi = dev_get_drvdata(dev);
    return spi_controller_suspend(rspi.ctlr);
    }
#[no_mangle]
unsafe extern "C" fn rspi_resume(dev: *mut device) -> c_int {
    static int rspi_resume(struct device *dev)
    {
    struct rspi_data *rspi = dev_get_drvdata(dev);
    return spi_controller_resume(rspi.ctlr);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(rspi_pm_ops, rspi_suspend, rspi_resume);
    static struct platform_driver rspi_driver = {
    .probe =	rspi_probe,
    .remove =	rspi_remove,
    .id_table =	spi_driver_ids,
    .driver		= {
    .name = "renesas_spi",
    .pm = pm_sleep_ptr(&rspi_pm_ops),
    .of_match_table = of_match_ptr(rspi_of_match),
    },
    };
    module_platform_driver(rspi_driver);
    MODULE_DESCRIPTION("Renesas RSPI bus driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Yoshihiro Shimoda");
