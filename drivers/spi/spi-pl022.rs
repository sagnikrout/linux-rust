//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-pl022.c
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
// A driver for the ARM PL022 PrimeCell SSP/SPI bus master.
//
// Copyright (C) 2008-2012 ST-Ericsson AB
// Copyright (C) 2006 STMicroelectronics Pvt. Ltd.
//
// Author: Linus Walleij <linus.walleij@stericsson.com>
//
// Initial version inspired by:
// linux-2.6.17-rc3-mm1/drivers/spi/pxa2xx_spi.c
// Initial adoption to PL022 by:
// Sachin Verma <sachin.verma@st.com>
//

//
// This macro is used to define some register default values.
// reg is masked with mask, the OR:ed with an (again masked)
// val shifted sb steps to the left.
//

    ((reg) = (((reg) & ~(mask)) | (((val)<<(sb)) & (mask))))
//
// This macro is also used to define some default values.
// It will just shift val by sb steps to the left and mask
// the result with mask.
//

    (((val)<<(sb)) & (mask))
pub const DRIVE_TX: c_int = 0;
pub const DO_NOT_DRIVE_TX: c_int = 1;
pub const DO_NOT_QUEUE_DMA: c_int = 0;
pub const QUEUE_DMA: c_int = 1;
pub const RX_TRANSFER: c_int = 1;
pub const TX_TRANSFER: c_int = 2;
//
// Macros to access SSP Registers with their offsets
//

//
// SSP Control Register 0  - SSP_CR0
//

//
// The ST version of this block moves som bits
// in SSP_CR0 and extends it to 32 bits
//

//
// SSP Control Register 0  - SSP_CR1
//

//
// The ST version of this block adds some bits
// in SSP_CR1
//

// This one is only in the PL023 variant

//
// SSP Status Register - SSP_SR
//

//
// SSP Clock Prescale Register  - SSP_CPSR
//

//
// SSP Interrupt Mask Set/Clear Register - SSP_IMSC
//

//
// SSP Raw Interrupt Status Register - SSP_RIS
//
// Receive Overrun Raw Interrupt status

// Receive Timeout Raw Interrupt status

// Receive FIFO Raw Interrupt status

// Transmit FIFO Raw Interrupt status

//
// SSP Masked Interrupt Status Register - SSP_MIS
//
// Receive Overrun Masked Interrupt status

// Receive Timeout Masked Interrupt status

// Receive FIFO Masked Interrupt status

// Transmit FIFO Masked Interrupt status

//
// SSP Interrupt Clear Register - SSP_ICR
//
// Receive Overrun Raw Clear Interrupt bit

// Receive Timeout Clear Interrupt bit

//
// SSP DMA Control Register - SSP_DMACR
//
// Receive DMA Enable bit

// Transmit DMA Enable bit

//
// SSP Chip Select Control Register - SSP_CSR
// (vendor extension)
//

//
// SSP Integration Test control Register - SSP_ITCR
//

//
// SSP Integration Test Input Register - SSP_ITIP
//

//
// SSP Integration Test output Register - SSP_ITOP
//

//
// SSP Test Data Register - SSP_TDR
//

//
// Message State
// we use the spi_message.state (void *) pointer to
// hold a single state value, that's why all this
// (void *) casting is done here.
//

//
// SSP State - Whether Enabled or Disabled
//

//
// SSP DMA State - Whether DMA Enabled or Disabled
//

//
// SSP Clock Defaults
//
pub const SSP_DEFAULT_CLKRATE: c_uint = 0x2;
pub const SSP_DEFAULT_PRESCALE: c_uint = 0x40;
//
// SSP Clock Parameter ranges
//
pub const CPSDVR_MIN: c_uint = 0x02;
pub const CPSDVR_MAX: c_uint = 0xFE;
pub const SCR_MIN: c_uint = 0x00;
pub const SCR_MAX: c_uint = 0xFF;
//
// SSP Interrupt related Macros
//
pub const DEFAULT_SSP_REG_IMSC: c_uint = 0x0UL;

    SSP_IMSC_MASK_RORIM | \
    SSP_IMSC_MASK_RTIM | \
    SSP_IMSC_MASK_RXIM | \
    SSP_IMSC_MASK_TXIM \
    )
pub const CLEAR_ALL_INTERRUPTS: c_uint = 0x3;
pub const SPI_POLLING_TIMEOUT: c_int = 1000;
//
// The type of reading going on this chip
//
    enum ssp_reading {
    READING_NULL,
    READING_U8,
    READING_U16,
    READING_U32
    };
//
// The type of writing going on this chip
//
    enum ssp_writing {
    WRITING_NULL,
    WRITING_U8,
    WRITING_U16,
    WRITING_U32
    };
//
// struct vendor_data - vendor-specific config parameters
// for PL022 derivates
// @fifodepth: depth of FIFOs (both)
// @max_bpw: maximum number of bits per word
// @unidir: supports unidirection transfers
// @extended_cr: 32 bit wide control register 0 with extra
// features and extra features in CR1 as found in the ST variants
// @pl023: supports a subset of the ST extensions called "PL023"
// @loopback: supports loopback mode
// @internal_cs_ctrl: supports chip select control register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vendor_data {
    pub fifodepth: c_int,
    pub max_bpw: c_int,
    pub unidir: bool,
    pub extended_cr: bool,
    pub pl023: bool,
    pub loopback: bool,
    pub internal_cs_ctrl: bool,
}

//
// struct pl022 - This is the private SSP driver data structure
// @adev: AMBA device model hookup
// @vendor: vendor data for the IP block
// @phybase: the physical memory where the SSP device resides
// @virtbase: the virtual memory where the SSP is mapped
// @clk: outgoing clock "SPICLK" for the SPI bus
// @host: SPI framework hookup
// @host_info: controller-specific data from machine setup
// @cur_transfer: Pointer to current spi_transfer
// @cur_chip: pointer to current clients chip(assigned from controller_state)
// @tx: current position in TX buffer to be read
// @tx_end: end position in TX buffer to be read
// @rx: current position in RX buffer to be written
// @rx_end: end position in RX buffer to be written
// @read: the type of read currently going on
// @write: the type of write currently going on
// @exp_fifo_level: expected FIFO level
// @rx_lev_trig: receive FIFO watermark level which triggers IRQ
// @tx_lev_trig: transmit FIFO watermark level which triggers IRQ
// @dma_rx_channel: optional channel for RX DMA
// @dma_tx_channel: optional channel for TX DMA
// @sgt_rx: scattertable for the RX transfer
// @sgt_tx: scattertable for the TX transfer
// @dummypage: a dummy page used for driving data on the bus with DMA
// @dma_running: indicates whether DMA is in operation
// @cur_cs: current chip select index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl022 {
    pub adev: *mut amba_device,
    pub vendor: *mut vendor_data,
    pub phybase: resource_size_t,
    pub virtbase: *mut void __iomem,
    pub clk: *mut clk,
    pub host: *mut spi_controller,
    pub host_info: *mut pl022_ssp_controller,
    pub cur_transfer: *mut spi_transfer,
    pub cur_chip: *mut chip_data,
    pub tx: *mut c_void,
    pub tx_end: *mut c_void,
    pub rx: *mut c_void,
    pub rx_end: *mut c_void,
    pub read: enum ssp_reading,
    pub write: enum ssp_writing,
    pub exp_fifo_level: u32,
    pub rx_lev_trig: enum ssp_rx_level_trig,
    pub tx_lev_trig: enum ssp_tx_level_trig,
// DMA settings

    pub dma_rx_channel: *mut dma_chan,
    pub dma_tx_channel: *mut dma_chan,
    pub sgt_rx: sg_table,
    pub sgt_tx: sg_table,
    pub dummypage: *mut c_char,
    pub dma_running: bool,

    pub cur_cs: c_int,
}

//
// struct chip_data - To maintain runtime state of SSP for each client chip
// @cr0: Value of control register CR0 of SSP - on later ST variants this
// register is 32 bits wide rather than just 16
// @cr1: Value of control register CR1 of SSP
// @dmacr: Value of DMA control Register of SSP
// @cpsr: Value of Clock prescale register
// @n_bytes: how many bytes(power of 2) reqd for a given data width of client
// @enable_dma: Whether to enable DMA or not
// @read: function ptr to be used to read when doing xfer for this chip
// @write: function ptr to be used to write when doing xfer for this chip
// @xfer_type: polling/interrupt/DMA
//
// Runtime state of the SSP controller, maintained per chip,
// This would be set according to the current message that would be served
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chip_data {
    pub cr0: u32,
    pub cr1: u16,
    pub dmacr: u16,
    pub cpsr: u16,
    pub n_bytes: u8,
    pub enable_dma: bool,
    pub read: enum ssp_reading,
    pub write: enum ssp_writing,
    pub xfer_type: c_int,
}

//
// internal_cs_control - Control chip select signals via SSP_CSR.
// @pl022: SSP driver private data structure
// @enable: select/delect the chip
//
// Used on controller with internal chip select control via SSP_CSR register
// (vendor extension). Each of the 5 LSB in the register controls one chip
// select signal.
//
#[no_mangle]
unsafe extern "C" fn internal_cs_control(pl022: *mut pl022, enable: bool) {
    static void internal_cs_control(struct pl022 *pl022, bool enable)
    {
    u32 tmp;
    tmp = readw(SSP_CSR(pl022.virtbase));
    if (enable)
    tmp &= ~BIT(pl022.cur_cs);
    else
    tmp |= BIT(pl022.cur_cs);
    writew(tmp, SSP_CSR(pl022.virtbase));
    }
#[no_mangle]
unsafe extern "C" fn pl022_cs_control(spi: *mut spi_device, enable: bool) {
    static void pl022_cs_control(struct spi_device *spi, bool enable)
    {
    struct pl022 *pl022 = spi_controller_get_devdata(spi.controller);
    if (pl022.vendor.internal_cs_ctrl)
    internal_cs_control(pl022, enable);
    }
//
// flush - flush the FIFO to reach a clean state
// @pl022: SSP driver private data structure
//
#[no_mangle]
unsafe extern "C" fn flush(pl022: *mut pl022) -> c_int {
    static int flush(struct pl022 *pl022)
    {
    let mut limit: c_ulong = loops_per_jiffy << 1;
    dev_dbg(&pl022.adev.dev, "flush\n");
    do {
    while (readw(SSP_SR(pl022.virtbase)) & SSP_SR_MASK_RNE)
    readw(SSP_DR(pl022.virtbase));
    } while ((readw(SSP_SR(pl022.virtbase)) & SSP_SR_MASK_BSY) && limit--);
    pl022.exp_fifo_level = 0;
    return limit;
    }
//
// restore_state - Load configuration of current chip
// @pl022: SSP driver private data structure
//
#[no_mangle]
unsafe extern "C" fn restore_state(pl022: *mut pl022) {
    static void restore_state(struct pl022 *pl022)
    {
    struct chip_data *chip = pl022.cur_chip;
    if (pl022.vendor.extended_cr)
    writel(chip.cr0, SSP_CR0(pl022.virtbase));
    else
    writew(chip.cr0, SSP_CR0(pl022.virtbase));
    writew(chip.cr1, SSP_CR1(pl022.virtbase));
    writew(chip.dmacr, SSP_DMACR(pl022.virtbase));
    writew(chip.cpsr, SSP_CPSR(pl022.virtbase));
    writew(DISABLE_ALL_INTERRUPTS, SSP_IMSC(pl022.virtbase));
    writew(CLEAR_ALL_INTERRUPTS, SSP_ICR(pl022.virtbase));
    }
//
// Default SSP Register Values
//

    GEN_MASK_BITS(SSP_DATA_BITS_12, SSP_CR0_MASK_DSS, 0)	| \
    GEN_MASK_BITS(SSP_INTERFACE_MOTOROLA_SPI, SSP_CR0_MASK_FRF, 4) | \
    GEN_MASK_BITS(SSP_CLK_POL_IDLE_LOW, SSP_CR0_MASK_SPO, 6) | \
    GEN_MASK_BITS(SSP_CLK_SECOND_EDGE, SSP_CR0_MASK_SPH, 7) | \
    GEN_MASK_BITS(SSP_DEFAULT_CLKRATE, SSP_CR0_MASK_SCR, 8) \
    )
// ST versions have slightly different bit layout

    GEN_MASK_BITS(SSP_DATA_BITS_12, SSP_CR0_MASK_DSS_ST, 0)	| \
    GEN_MASK_BITS(SSP_MICROWIRE_CHANNEL_FULL_DUPLEX, SSP_CR0_MASK_HALFDUP_ST, 5) | \
    GEN_MASK_BITS(SSP_CLK_POL_IDLE_LOW, SSP_CR0_MASK_SPO, 6) | \
    GEN_MASK_BITS(SSP_CLK_SECOND_EDGE, SSP_CR0_MASK_SPH, 7) | \
    GEN_MASK_BITS(SSP_DEFAULT_CLKRATE, SSP_CR0_MASK_SCR, 8) | \
    GEN_MASK_BITS(SSP_BITS_8, SSP_CR0_MASK_CSS_ST, 16)	| \
    GEN_MASK_BITS(SSP_INTERFACE_MOTOROLA_SPI, SSP_CR0_MASK_FRF_ST, 21) \
    )
// The PL023 version is slightly different again

    GEN_MASK_BITS(SSP_DATA_BITS_12, SSP_CR0_MASK_DSS_ST, 0)	| \
    GEN_MASK_BITS(SSP_CLK_POL_IDLE_LOW, SSP_CR0_MASK_SPO, 6) | \
    GEN_MASK_BITS(SSP_CLK_SECOND_EDGE, SSP_CR0_MASK_SPH, 7) | \
    GEN_MASK_BITS(SSP_DEFAULT_CLKRATE, SSP_CR0_MASK_SCR, 8) \
    )

    GEN_MASK_BITS(LOOPBACK_DISABLED, SSP_CR1_MASK_LBM, 0) | \
    GEN_MASK_BITS(SSP_DISABLED, SSP_CR1_MASK_SSE, 1) | \
    GEN_MASK_BITS(SSP_MASTER, SSP_CR1_MASK_MS, 2) | \
    GEN_MASK_BITS(DO_NOT_DRIVE_TX, SSP_CR1_MASK_SOD, 3) \
    )
// ST versions extend this register to use all 16 bits

    DEFAULT_SSP_REG_CR1 | \
    GEN_MASK_BITS(SSP_RX_MSB, SSP_CR1_MASK_RENDN_ST, 4) | \
    GEN_MASK_BITS(SSP_TX_MSB, SSP_CR1_MASK_TENDN_ST, 5) | \
    GEN_MASK_BITS(SSP_MWIRE_WAIT_ZERO, SSP_CR1_MASK_MWAIT_ST, 6) |\
    GEN_MASK_BITS(SSP_RX_1_OR_MORE_ELEM, SSP_CR1_MASK_RXIFLSEL_ST, 7) | \
    GEN_MASK_BITS(SSP_TX_1_OR_MORE_EMPTY_LOC, SSP_CR1_MASK_TXIFLSEL_ST, 10) \
    )
//
// The PL023 variant has further differences: no loopback mode, no microwire
// support, and a new clock feedback delay setting.
//

    GEN_MASK_BITS(SSP_DISABLED, SSP_CR1_MASK_SSE, 1) | \
    GEN_MASK_BITS(SSP_MASTER, SSP_CR1_MASK_MS, 2) | \
    GEN_MASK_BITS(DO_NOT_DRIVE_TX, SSP_CR1_MASK_SOD, 3) | \
    GEN_MASK_BITS(SSP_RX_MSB, SSP_CR1_MASK_RENDN_ST, 4) | \
    GEN_MASK_BITS(SSP_TX_MSB, SSP_CR1_MASK_TENDN_ST, 5) | \
    GEN_MASK_BITS(SSP_RX_1_OR_MORE_ELEM, SSP_CR1_MASK_RXIFLSEL_ST, 7) | \
    GEN_MASK_BITS(SSP_TX_1_OR_MORE_EMPTY_LOC, SSP_CR1_MASK_TXIFLSEL_ST, 10) | \
    GEN_MASK_BITS(SSP_FEEDBACK_CLK_DELAY_NONE, SSP_CR1_MASK_FBCLKDEL_ST, 13) \
    )

    GEN_MASK_BITS(SSP_DEFAULT_PRESCALE, SSP_CPSR_MASK_CPSDVSR, 0) \
    )

    GEN_MASK_BITS(SSP_DMA_DISABLED, SSP_DMACR_MASK_RXDMAE, 0) | \
    GEN_MASK_BITS(SSP_DMA_DISABLED, SSP_DMACR_MASK_TXDMAE, 1) \
    )
//
// load_ssp_default_config - Load default configuration for SSP
// @pl022: SSP driver private data structure
//
#[no_mangle]
unsafe extern "C" fn load_ssp_default_config(pl022: *mut pl022) {
    static void load_ssp_default_config(struct pl022 *pl022)
    {
    if (pl022.vendor.pl023) {
    writel(DEFAULT_SSP_REG_CR0_ST_PL023, SSP_CR0(pl022.virtbase));
    writew(DEFAULT_SSP_REG_CR1_ST_PL023, SSP_CR1(pl022.virtbase));
    } else if (pl022.vendor.extended_cr) {
    writel(DEFAULT_SSP_REG_CR0_ST, SSP_CR0(pl022.virtbase));
    writew(DEFAULT_SSP_REG_CR1_ST, SSP_CR1(pl022.virtbase));
    } else {
    writew(DEFAULT_SSP_REG_CR0, SSP_CR0(pl022.virtbase));
    writew(DEFAULT_SSP_REG_CR1, SSP_CR1(pl022.virtbase));
    }
    writew(DEFAULT_SSP_REG_DMACR, SSP_DMACR(pl022.virtbase));
    writew(DEFAULT_SSP_REG_CPSR, SSP_CPSR(pl022.virtbase));
    writew(DISABLE_ALL_INTERRUPTS, SSP_IMSC(pl022.virtbase));
    writew(CLEAR_ALL_INTERRUPTS, SSP_ICR(pl022.virtbase));
    }
//
// This will write to TX and read from RX according to the parameters
// set in pl022.
//
#[no_mangle]
unsafe extern "C" fn readwriter(pl022: *mut pl022) {
    static void readwriter(struct pl022 *pl022)
    {
//
// The FIFO depth is different between primecell variants.
// I believe filling in too much in the FIFO might cause
// errons in 8bit wide transfers on ARM variants (just 8 words
// FIFO, means only 8x8 = 64 bits in FIFO) at least.
//
// To prevent this issue, the TX FIFO is only filled to the
// unused RX FIFO fill length, regardless of what the TX
// FIFO status flag indicates.
//
    dev_dbg(&pl022.adev.dev,
    "%s, rx: %p, rxend: %p, tx: %p, txend: %p\n",
    __func__, pl022.rx, pl022.rx_end, pl022.tx, pl022.tx_end);
// Read as much as you can
    while ((readw(SSP_SR(pl022.virtbase)) & SSP_SR_MASK_RNE)
    && (pl022.rx < pl022.rx_end)) {
    switch (pl022.read) {
    case READING_NULL:
    readw(SSP_DR(pl022.virtbase));
    break;
    case READING_U8:
// (u8 *) (pl022->rx) =
    readw(SSP_DR(pl022.virtbase)) & 0xFFU;
    break;
    case READING_U16:
// (u16 *) (pl022->rx) =
    (u16) readw(SSP_DR(pl022.virtbase));
    break;
    case READING_U32:
// (u32 *) (pl022->rx) =
    readl(SSP_DR(pl022.virtbase));
    break;
    }
    pl022.rx += (pl022.cur_chip.n_bytes);
    pl022.exp_fifo_level--;
    }
//
// Write as much as possible up to the RX FIFO size
//
    while ((pl022.exp_fifo_level < pl022.vendor.fifodepth)
    && (pl022.tx < pl022.tx_end)) {
    switch (pl022.write) {
    case WRITING_NULL:
    writew(0x0, SSP_DR(pl022.virtbase));
    break;
    case WRITING_U8:
    writew(*(u8 *) (pl022.tx), SSP_DR(pl022.virtbase));
    break;
    case WRITING_U16:
    writew((*(u16 *) (pl022.tx)), SSP_DR(pl022.virtbase));
    break;
    case WRITING_U32:
    writel(*(u32 *) (pl022.tx), SSP_DR(pl022.virtbase));
    break;
    }
    pl022.tx += (pl022.cur_chip.n_bytes);
    pl022.exp_fifo_level++;
//
// This inner reader takes care of things appearing in the RX
// FIFO as we're transmitting. This will happen a lot since the
// clock starts running when you put things into the TX FIFO,
// and then things are continuously clocked into the RX FIFO.
//
    while ((readw(SSP_SR(pl022.virtbase)) & SSP_SR_MASK_RNE)
    && (pl022.rx < pl022.rx_end)) {
    switch (pl022.read) {
    case READING_NULL:
    readw(SSP_DR(pl022.virtbase));
    break;
    case READING_U8:
// (u8 *) (pl022->rx) =
    readw(SSP_DR(pl022.virtbase)) & 0xFFU;
    break;
    case READING_U16:
// (u16 *) (pl022->rx) =
    (u16) readw(SSP_DR(pl022.virtbase));
    break;
    case READING_U32:
// (u32 *) (pl022->rx) =
    readl(SSP_DR(pl022.virtbase));
    break;
    }
    pl022.rx += (pl022.cur_chip.n_bytes);
    pl022.exp_fifo_level--;
    }
    }
//
// When we exit here the TX FIFO should be full and the RX FIFO
// should be empty
//
    }
//
// This DMA functionality is only compiled in if we have
// access to the generic DMA devices/DMA engine.
//

#[no_mangle]
unsafe extern "C" fn unmap_free_dma_scatter(pl022: *mut pl022) {
    static void unmap_free_dma_scatter(struct pl022 *pl022)
    {
// Unmap and free the SG tables
    dma_unmap_sg(pl022.dma_tx_channel.device.dev, pl022.sgt_tx.sgl,
    pl022.sgt_tx.nents, DMA_TO_DEVICE);
    dma_unmap_sg(pl022.dma_rx_channel.device.dev, pl022.sgt_rx.sgl,
    pl022.sgt_rx.nents, DMA_FROM_DEVICE);
    sg_free_table(&pl022.sgt_rx);
    sg_free_table(&pl022.sgt_tx);
    }
#[no_mangle]
unsafe extern "C" fn dma_callback(data: *mut c_void) {
    static void dma_callback(void *data)
    {
    struct pl022 *pl022 = data;
    BUG_ON(!pl022.sgt_rx.sgl);

//
// Optionally dump out buffers to inspect contents, this is
// good if you want to convince yourself that the loopback
// read/write contents are the same, when adopting to a new
// DMA engine.
//
    {
    struct scatterlist *sg;
    unsigned int i;
    dma_sync_sg_for_cpu(&pl022.adev.dev,
    pl022.sgt_rx.sgl,
    pl022.sgt_rx.nents,
    DMA_FROM_DEVICE);
    for_each_sg(pl022.sgt_rx.sgl, sg, pl022.sgt_rx.nents, i) {
    dev_dbg(&pl022.adev.dev, "SPI RX SG ENTRY: %d", i);
    print_hex_dump(KERN_ERR, "SPI RX: ",
    DUMP_PREFIX_OFFSET,
    16,
    1,
    sg_virt(sg),
    sg_dma_len(sg),
    1);
    }
    for_each_sg(pl022.sgt_tx.sgl, sg, pl022.sgt_tx.nents, i) {
    dev_dbg(&pl022.adev.dev, "SPI TX SG ENTRY: %d", i);
    print_hex_dump(KERN_ERR, "SPI TX: ",
    DUMP_PREFIX_OFFSET,
    16,
    1,
    sg_virt(sg),
    sg_dma_len(sg),
    1);
    }
    }

    unmap_free_dma_scatter(pl022);
    spi_finalize_current_transfer(pl022.host);
    }
    static void setup_dma_scatter(struct pl022 *pl022,
    void *buffer,
    unsigned int length,
    struct sg_table *sgtab)
    {
    struct scatterlist *sg;
    let mut bytesleft: c_int = length;
    void *bufp = buffer;
    int mapbytes;
    int i;
    if (buffer) {
    for_each_sg(sgtab.sgl, sg, sgtab.nents, i) {
//
// If there are less bytes left than what fits
// in the current page (plus page alignment offset)
// we just feed in this, else we stuff in as much
// as we can.
//
    mapbytes = min_t(int, bytesleft,
    PAGE_SIZE - offset_in_page(bufp));
    sg_set_page(sg, virt_to_page(bufp),
    mapbytes, offset_in_page(bufp));
    bufp += mapbytes;
    bytesleft -= mapbytes;
    dev_dbg(&pl022.adev.dev,
    "set RX/TX target page @ %p, %d bytes, %d left\n",
    bufp, mapbytes, bytesleft);
    }
    } else {
// Map the dummy buffer on every page
    for_each_sg(sgtab.sgl, sg, sgtab.nents, i) {
    mapbytes = min_t(int, bytesleft, PAGE_SIZE);
    sg_set_page(sg, virt_to_page(pl022.dummypage),
    mapbytes, 0);
    bytesleft -= mapbytes;
    dev_dbg(&pl022.adev.dev,
    "set RX/TX to dummy page %d bytes, %d left\n",
    mapbytes, bytesleft);
    }
    }
    BUG_ON(bytesleft);
    }
//
// configure_dma - configures the channels for the next transfer
// @pl022: SSP driver's private data structure
//
#[no_mangle]
unsafe extern "C" fn configure_dma(pl022: *mut pl022) -> c_int {
    static int configure_dma(struct pl022 *pl022)
    {
    struct dma_slave_config rx_conf = {
    .src_addr = SSP_DR(pl022.phybase),
    .direction = DMA_DEV_TO_MEM,
    .device_fc = false,
    };
    struct dma_slave_config tx_conf = {
    .dst_addr = SSP_DR(pl022.phybase),
    .direction = DMA_MEM_TO_DEV,
    .device_fc = false,
    };
    unsigned int pages;
    int ret;
    int rx_sglen, tx_sglen;
    struct dma_chan *rxchan = pl022.dma_rx_channel;
    struct dma_chan *txchan = pl022.dma_tx_channel;
    struct dma_async_tx_descriptor *rxdesc;
    struct dma_async_tx_descriptor *txdesc;
// Check that the channels are available
    if (!rxchan || !txchan)
    return -ENODEV;
//
// If supplied, the DMA burstsize should equal the FIFO trigger level.
// Notice that the DMA engine uses one-to-one mapping. Since we can
// not trigger on 2 elements this needs explicit mapping rather than
// calculation.
//
    switch (pl022.rx_lev_trig) {
    case SSP_RX_1_OR_MORE_ELEM:
    rx_conf.src_maxburst = 1;
    break;
    case SSP_RX_4_OR_MORE_ELEM:
    rx_conf.src_maxburst = 4;
    break;
    case SSP_RX_8_OR_MORE_ELEM:
    rx_conf.src_maxburst = 8;
    break;
    case SSP_RX_16_OR_MORE_ELEM:
    rx_conf.src_maxburst = 16;
    break;
    case SSP_RX_32_OR_MORE_ELEM:
    rx_conf.src_maxburst = 32;
    break;
    default:
    rx_conf.src_maxburst = pl022.vendor.fifodepth >> 1;
    break;
    }
    switch (pl022.tx_lev_trig) {
    case SSP_TX_1_OR_MORE_EMPTY_LOC:
    tx_conf.dst_maxburst = 1;
    break;
    case SSP_TX_4_OR_MORE_EMPTY_LOC:
    tx_conf.dst_maxburst = 4;
    break;
    case SSP_TX_8_OR_MORE_EMPTY_LOC:
    tx_conf.dst_maxburst = 8;
    break;
    case SSP_TX_16_OR_MORE_EMPTY_LOC:
    tx_conf.dst_maxburst = 16;
    break;
    case SSP_TX_32_OR_MORE_EMPTY_LOC:
    tx_conf.dst_maxburst = 32;
    break;
    default:
    tx_conf.dst_maxburst = pl022.vendor.fifodepth >> 1;
    break;
    }
    switch (pl022.read) {
    case READING_NULL:
// Use the same as for writing
    rx_conf.src_addr_width = DMA_SLAVE_BUSWIDTH_UNDEFINED;
    break;
    case READING_U8:
    rx_conf.src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    break;
    case READING_U16:
    rx_conf.src_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    case READING_U32:
    rx_conf.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    }
    switch (pl022.write) {
    case WRITING_NULL:
// Use the same as for reading
    tx_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_UNDEFINED;
    break;
    case WRITING_U8:
    tx_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    break;
    case WRITING_U16:
    tx_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    case WRITING_U32:
    tx_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    }
// SPI peculiarity: we need to read and write the same width
    if (rx_conf.src_addr_width == DMA_SLAVE_BUSWIDTH_UNDEFINED)
    rx_conf.src_addr_width = tx_conf.dst_addr_width;
    if (tx_conf.dst_addr_width == DMA_SLAVE_BUSWIDTH_UNDEFINED)
    tx_conf.dst_addr_width = rx_conf.src_addr_width;
    BUG_ON(rx_conf.src_addr_width != tx_conf.dst_addr_width);
    dmaengine_slave_config(rxchan, &rx_conf);
    dmaengine_slave_config(txchan, &tx_conf);
// Create sglists for the transfers
    pages = DIV_ROUND_UP(pl022.cur_transfer.len, PAGE_SIZE);
    dev_dbg(&pl022.adev.dev, "using %d pages for transfer\n", pages);
    ret = sg_alloc_table(&pl022.sgt_rx, pages, GFP_ATOMIC);
    if (ret)
    goto err_alloc_rx_sg;
    ret = sg_alloc_table(&pl022.sgt_tx, pages, GFP_ATOMIC);
    if (ret)
    goto err_alloc_tx_sg;
// Fill in the scatterlists for the RX+TX buffers
    setup_dma_scatter(pl022, pl022.rx,
    pl022.cur_transfer.len, &pl022.sgt_rx);
    setup_dma_scatter(pl022, pl022.tx,
    pl022.cur_transfer.len, &pl022.sgt_tx);
// Map DMA buffers
    rx_sglen = dma_map_sg(rxchan.device.dev, pl022.sgt_rx.sgl,
    pl022.sgt_rx.nents, DMA_FROM_DEVICE);
    if (!rx_sglen)
    goto err_rx_sgmap;
    tx_sglen = dma_map_sg(txchan.device.dev, pl022.sgt_tx.sgl,
    pl022.sgt_tx.nents, DMA_TO_DEVICE);
    if (!tx_sglen)
    goto err_tx_sgmap;
// Send both scatterlists
    rxdesc = dmaengine_prep_slave_sg(rxchan,
    pl022.sgt_rx.sgl,
    rx_sglen,
    DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!rxdesc)
    goto err_rxdesc;
    txdesc = dmaengine_prep_slave_sg(txchan,
    pl022.sgt_tx.sgl,
    tx_sglen,
    DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!txdesc)
    goto err_txdesc;
// Put the callback on the RX transfer only, that should finish last
    rxdesc.callback = dma_callback;
    rxdesc.callback_param = pl022;
// Submit and fire RX and TX with TX last so we're ready to read!
    dmaengine_submit(rxdesc);
    dmaengine_submit(txdesc);
    dma_async_issue_pending(rxchan);
    dma_async_issue_pending(txchan);
    pl022.dma_running = true;
    return 0;
    err_txdesc:
    dmaengine_terminate_all(txchan);
    err_rxdesc:
    dmaengine_terminate_all(rxchan);
    dma_unmap_sg(txchan.device.dev, pl022.sgt_tx.sgl,
    pl022.sgt_tx.nents, DMA_TO_DEVICE);
    err_tx_sgmap:
    dma_unmap_sg(rxchan.device.dev, pl022.sgt_rx.sgl,
    pl022.sgt_rx.nents, DMA_FROM_DEVICE);
    err_rx_sgmap:
    sg_free_table(&pl022.sgt_tx);
    err_alloc_tx_sg:
    sg_free_table(&pl022.sgt_rx);
    err_alloc_rx_sg:
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn pl022_dma_probe(pl022: *mut pl022) -> c_int {
    static int pl022_dma_probe(struct pl022 *pl022)
    {
    dma_cap_mask_t mask;
// Try to acquire a generic DMA engine slave channel
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
//
// We need both RX and TX channels to do DMA, else do none
// of them.
//
    pl022.dma_rx_channel = dma_request_channel(mask,
    pl022.host_info.dma_filter,
    pl022.host_info.dma_rx_param);
    if (!pl022.dma_rx_channel) {
    dev_dbg(&pl022.adev.dev, "no RX DMA channel!\n");
    goto err_no_rxchan;
    }
    pl022.dma_tx_channel = dma_request_channel(mask,
    pl022.host_info.dma_filter,
    pl022.host_info.dma_tx_param);
    if (!pl022.dma_tx_channel) {
    dev_dbg(&pl022.adev.dev, "no TX DMA channel!\n");
    goto err_no_txchan;
    }
    pl022.dummypage = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!pl022.dummypage)
    goto err_no_dummypage;
    dev_info(&pl022.adev.dev, "setup for DMA on RX %s, TX %s\n",
    dma_chan_name(pl022.dma_rx_channel),
    dma_chan_name(pl022.dma_tx_channel));
    return 0;
    err_no_dummypage:
    dma_release_channel(pl022.dma_tx_channel);
    err_no_txchan:
    dma_release_channel(pl022.dma_rx_channel);
    pl022.dma_rx_channel = core::ptr::null_mut();
    err_no_rxchan:
    dev_err(&pl022.adev.dev,
    "Failed to work in dma mode, work without dma!\n");
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn pl022_dma_autoprobe(pl022: *mut pl022) -> c_int {
    static int pl022_dma_autoprobe(struct pl022 *pl022)
    {
    struct device *dev = &pl022.adev.dev;
    struct dma_chan *chan;
    int err;
// automatically configure DMA channels from platform, normally using DT
    chan = dma_request_chan(dev, "rx");
    if (IS_ERR(chan)) {
    err = PTR_ERR(chan);
    goto err_no_rxchan;
    }
    pl022.dma_rx_channel = chan;
    chan = dma_request_chan(dev, "tx");
    if (IS_ERR(chan)) {
    err = PTR_ERR(chan);
    goto err_no_txchan;
    }
    pl022.dma_tx_channel = chan;
    pl022.dummypage = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!pl022.dummypage) {
    err = -ENOMEM;
    goto err_no_dummypage;
    }
    return 0;
    err_no_dummypage:
    dma_release_channel(pl022.dma_tx_channel);
    pl022.dma_tx_channel = core::ptr::null_mut();
    err_no_txchan:
    dma_release_channel(pl022.dma_rx_channel);
    pl022.dma_rx_channel = core::ptr::null_mut();
    err_no_rxchan:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn terminate_dma(pl022: *mut pl022) {
    static void terminate_dma(struct pl022 *pl022)
    {
    if (!pl022.dma_running)
    return;
    struct dma_chan *rxchan = pl022.dma_rx_channel;
    struct dma_chan *txchan = pl022.dma_tx_channel;
    dmaengine_terminate_all(rxchan);
    dmaengine_terminate_all(txchan);
    unmap_free_dma_scatter(pl022);
    pl022.dma_running = false;
    }
#[no_mangle]
unsafe extern "C" fn pl022_dma_remove(pl022: *mut pl022) {
    static void pl022_dma_remove(struct pl022 *pl022)
    {
    terminate_dma(pl022);
    if (pl022.dma_tx_channel)
    dma_release_channel(pl022.dma_tx_channel);
    if (pl022.dma_rx_channel)
    dma_release_channel(pl022.dma_rx_channel);
    kfree(pl022.dummypage);
    }

#[no_mangle]
pub unsafe extern "C" fn configure_dma(pl022: *mut pl022) -> c_int {
    static inline int configure_dma(struct pl022 *pl022)
    {
    return -ENODEV;
    }
#[no_mangle]
pub unsafe extern "C" fn pl022_dma_autoprobe(pl022: *mut pl022) -> c_int {
    static inline int pl022_dma_autoprobe(struct pl022 *pl022)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pl022_dma_probe(pl022: *mut pl022) -> c_int {
    static inline int pl022_dma_probe(struct pl022 *pl022)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn terminate_dma(pl022: *mut pl022) {
    static inline void terminate_dma(struct pl022 *pl022)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn pl022_dma_remove(pl022: *mut pl022) {
    static inline void pl022_dma_remove(struct pl022 *pl022)
    {
    }

//
// pl022_interrupt_handler - Interrupt handler for SSP controller
// @irq: IRQ number
// @dev_id: Local device data
//
// This function handles interrupts generated for an interrupt based transfer.
// If a receive overrun (ROR) interrupt is there then we disable SSP, flag the
// current transfer with SPI_TRANS_FAIL_IO and call
// spi_finalize_current_transfer() to let the core finish the message.
// Otherwise it reads data from RX FIFO till there is no more data, and writes
// data in TX FIFO till it is not full. When the transfer is complete we call
// spi_finalize_current_transfer() so the core can schedule the next one.
//
#[no_mangle]
unsafe extern "C" fn pl022_interrupt_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pl022_interrupt_handler(int irq, void *dev_id)
    {
    struct pl022 *pl022 = dev_id;
    let mut irq_status: u16 = 0;
// Read the Interrupt Status Register
    irq_status = readw(SSP_MIS(pl022.virtbase));
    if (unlikely(!irq_status))
    return IRQ_NONE;
//
// This handles the FIFO interrupts, the timeout
// interrupts are flatly ignored, they cannot be
// trusted.
//
    if (unlikely(irq_status & SSP_MIS_MASK_RORMIS)) {
//
// Overrun interrupt - bail out since our Data has been
// corrupted
//
    dev_err(&pl022.adev.dev, "FIFO overrun\n");
    if (readw(SSP_SR(pl022.virtbase)) & SSP_SR_MASK_RFF)
    dev_err(&pl022.adev.dev,
    "RXFIFO is full\n");
//
// Disable and clear interrupts, disable SSP,
// mark message with bad status so it can be
// retried.
//
    writew(DISABLE_ALL_INTERRUPTS,
    SSP_IMSC(pl022.virtbase));
    writew(CLEAR_ALL_INTERRUPTS, SSP_ICR(pl022.virtbase));
    writew((readw(SSP_CR1(pl022.virtbase)) &
    (~SSP_CR1_MASK_SSE)), SSP_CR1(pl022.virtbase));
    pl022.cur_transfer.error |= SPI_TRANS_FAIL_IO;
    spi_finalize_current_transfer(pl022.host);
    return IRQ_HANDLED;
    }
    readwriter(pl022);
    if (pl022.tx == pl022.tx_end) {
// Disable Transmit interrupt, enable receive interrupt
    writew((readw(SSP_IMSC(pl022.virtbase)) &
    ~SSP_IMSC_MASK_TXIM) | SSP_IMSC_MASK_RXIM,
    SSP_IMSC(pl022.virtbase));
    }
//
// Since all transactions must write as much as shall be read,
// we can conclude the entire transaction once RX is complete.
// At this point, all TX will always be finished.
//
    if (pl022.rx >= pl022.rx_end) {
    writew(DISABLE_ALL_INTERRUPTS,
    SSP_IMSC(pl022.virtbase));
    writew(CLEAR_ALL_INTERRUPTS, SSP_ICR(pl022.virtbase));
    if (unlikely(pl022.rx > pl022.rx_end)) {
    dev_warn(&pl022.adev.dev, "read %u surplus "
    "bytes (did you request an odd "
    "number of bytes on a 16bit bus?)\n",
    (u32) (pl022.rx - pl022.rx_end));
    }
    spi_finalize_current_transfer(pl022.host);
    return IRQ_HANDLED;
    }
    return IRQ_HANDLED;
    }
//
// This sets up the pointers to memory for the next message to
// send out on the SPI bus.
//
    static int set_up_next_transfer(struct pl022 *pl022,
    struct spi_transfer *transfer)
    {
    int residue;
// Sanity check the message for this bus width
    residue = pl022.cur_transfer.len % pl022.cur_chip.n_bytes;
    if (unlikely(residue != 0)) {
    dev_err(&pl022.adev.dev,
    "message of %u bytes to transmit but the current "
    "chip bus has a data width of %u bytes!\n",
    pl022.cur_transfer.len,
    pl022.cur_chip.n_bytes);
    dev_err(&pl022.adev.dev, "skipping this message\n");
    return -EIO;
    }
    pl022.tx = (void *)transfer.tx_buf;
    pl022.tx_end = pl022.tx + pl022.cur_transfer.len;
    pl022.rx = (void *)transfer.rx_buf;
    pl022.rx_end = pl022.rx + pl022.cur_transfer.len;
    pl022.write =
    pl022.tx ? pl022.cur_chip.write : WRITING_NULL;
    pl022.read = pl022.rx ? pl022.cur_chip.read : READING_NULL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_interrupt_dma_transfer(pl022: *mut pl022) -> c_int {
    static int do_interrupt_dma_transfer(struct pl022 *pl022)
    {
    int ret;
//
// Default is to enable all interrupts except RX -
// this will be enabled once TX is complete
//
    let mut irqflags: u32 = (u32)(ENABLE_ALL_INTERRUPTS & ~SSP_IMSC_MASK_RXIM);
    ret = set_up_next_transfer(pl022, pl022.cur_transfer);
    if (ret)
    return ret;
// If we're using DMA, set up DMA here
    if (pl022.cur_chip.enable_dma) {
// Configure DMA transfer
    if (configure_dma(pl022)) {
    dev_dbg(&pl022.adev.dev,
    "configuration of DMA failed, fall back to interrupt mode\n");
    goto err_config_dma;
    }
// Disable interrupts in DMA mode, IRQ from DMA controller
    irqflags = DISABLE_ALL_INTERRUPTS;
    }
    err_config_dma:
// Enable SSP, turn on interrupts
    writew((readw(SSP_CR1(pl022.virtbase)) | SSP_CR1_MASK_SSE),
    SSP_CR1(pl022.virtbase));
    writew(irqflags, SSP_IMSC(pl022.virtbase));
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn print_current_status(pl022: *mut pl022) {
    static void print_current_status(struct pl022 *pl022)
    {
    u32 read_cr0;
    u16 read_cr1, read_dmacr, read_sr;
    if (pl022.vendor.extended_cr)
    read_cr0 = readl(SSP_CR0(pl022.virtbase));
    else
    read_cr0 = readw(SSP_CR0(pl022.virtbase));
    read_cr1 = readw(SSP_CR1(pl022.virtbase));
    read_dmacr = readw(SSP_DMACR(pl022.virtbase));
    read_sr = readw(SSP_SR(pl022.virtbase));
    dev_warn(&pl022.adev.dev, "spi-pl022 CR0: %x\n", read_cr0);
    dev_warn(&pl022.adev.dev, "spi-pl022 CR1: %x\n", read_cr1);
    dev_warn(&pl022.adev.dev, "spi-pl022 DMACR: %x\n", read_dmacr);
    dev_warn(&pl022.adev.dev, "spi-pl022 SR: %x\n", read_sr);
    dev_warn(&pl022.adev.dev,
    "spi-pl022 exp_fifo_level/fifodepth: %u/%d\n",
    pl022.exp_fifo_level,
    pl022.vendor.fifodepth);
    }
#[no_mangle]
unsafe extern "C" fn do_polling_transfer(pl022: *mut pl022) -> c_int {
    static int do_polling_transfer(struct pl022 *pl022)
    {
    int ret;
    unsigned long time, timeout;
// Configuration Changing Per Transfer
    ret = set_up_next_transfer(pl022, pl022.cur_transfer);
    if (ret)
    return ret;
// Flush FIFOs and enable SSP
    flush(pl022);
    writew((readw(SSP_CR1(pl022.virtbase)) | SSP_CR1_MASK_SSE),
    SSP_CR1(pl022.virtbase));
    dev_dbg(&pl022.adev.dev, "polling transfer ongoing ...\n");
    timeout = jiffies + msecs_to_jiffies(SPI_POLLING_TIMEOUT);
    while (pl022.tx < pl022.tx_end || pl022.rx < pl022.rx_end) {
    time = jiffies;
    readwriter(pl022);
    if (time_after(time, timeout)) {
    dev_warn(&pl022.adev.dev,
    "%s: timeout!\n", __func__);
    print_current_status(pl022);
    return -ETIMEDOUT;
    }
    cpu_relax();
    }
    return 0;
    }
    static int pl022_transfer_one(struct spi_controller *host, struct spi_device *spi,
    struct spi_transfer *transfer)
    {
    struct pl022 *pl022 = spi_controller_get_devdata(host);
    pl022.cur_transfer = transfer;
// Setup the SPI using the per chip configuration
    pl022.cur_chip = spi_get_ctldata(spi);
    pl022.cur_cs = spi_get_chipselect(spi, 0);
    restore_state(pl022);
    flush(pl022);
    if (pl022.cur_chip.xfer_type == POLLING_TRANSFER)
    return do_polling_transfer(pl022);
    else
    return do_interrupt_dma_transfer(pl022);
    }
#[no_mangle]
unsafe extern "C" fn pl022_handle_err(ctlr: *mut spi_controller, message: *mut spi_message) {
    static void pl022_handle_err(struct spi_controller *ctlr, struct spi_message *message)
    {
    struct pl022 *pl022 = spi_controller_get_devdata(ctlr);
    terminate_dma(pl022);
    writew(DISABLE_ALL_INTERRUPTS, SSP_IMSC(pl022.virtbase));
    writew(CLEAR_ALL_INTERRUPTS, SSP_ICR(pl022.virtbase));
    }
#[no_mangle]
unsafe extern "C" fn pl022_unprepare_transfer_hardware(host: *mut spi_controller) -> c_int {
    static int pl022_unprepare_transfer_hardware(struct spi_controller *host)
    {
    struct pl022 *pl022 = spi_controller_get_devdata(host);
// nothing more to do - disable spi/ssp and power off
    writew((readw(SSP_CR1(pl022.virtbase)) &
    (~SSP_CR1_MASK_SSE)), SSP_CR1(pl022.virtbase));
    return 0;
    }
    static int verify_controller_parameters(struct pl022 *pl022,
    struct pl022_config_chip const *chip_info)
    {
    if ((chip_info.iface < SSP_INTERFACE_MOTOROLA_SPI)
    || (chip_info.iface > SSP_INTERFACE_UNIDIRECTIONAL)) {
    dev_err(&pl022.adev.dev,
    "interface is configured incorrectly\n");
    return -EINVAL;
    }
    if ((chip_info.iface == SSP_INTERFACE_UNIDIRECTIONAL) &&
    (!pl022.vendor.unidir)) {
    dev_err(&pl022.adev.dev,
    "unidirectional mode not supported in this "
    "hardware version\n");
    return -EINVAL;
    }
    if ((chip_info.hierarchy != SSP_MASTER)
    && (chip_info.hierarchy != SSP_SLAVE)) {
    dev_err(&pl022.adev.dev,
    "hierarchy is configured incorrectly\n");
    return -EINVAL;
    }
    if ((chip_info.com_mode != INTERRUPT_TRANSFER)
    && (chip_info.com_mode != DMA_TRANSFER)
    && (chip_info.com_mode != POLLING_TRANSFER)) {
    dev_err(&pl022.adev.dev,
    "Communication mode is configured incorrectly\n");
    return -EINVAL;
    }
    switch (chip_info.rx_lev_trig) {
    case SSP_RX_1_OR_MORE_ELEM:
    case SSP_RX_4_OR_MORE_ELEM:
    case SSP_RX_8_OR_MORE_ELEM:
// These are always OK, all variants can handle this
    break;
    case SSP_RX_16_OR_MORE_ELEM:
    if (pl022.vendor.fifodepth < 16) {
    dev_err(&pl022.adev.dev,
    "RX FIFO Trigger Level is configured incorrectly\n");
    return -EINVAL;
    }
    break;
    case SSP_RX_32_OR_MORE_ELEM:
    if (pl022.vendor.fifodepth < 32) {
    dev_err(&pl022.adev.dev,
    "RX FIFO Trigger Level is configured incorrectly\n");
    return -EINVAL;
    }
    break;
    default:
    dev_err(&pl022.adev.dev,
    "RX FIFO Trigger Level is configured incorrectly\n");
    return -EINVAL;
    }
    switch (chip_info.tx_lev_trig) {
    case SSP_TX_1_OR_MORE_EMPTY_LOC:
    case SSP_TX_4_OR_MORE_EMPTY_LOC:
    case SSP_TX_8_OR_MORE_EMPTY_LOC:
// These are always OK, all variants can handle this
    break;
    case SSP_TX_16_OR_MORE_EMPTY_LOC:
    if (pl022.vendor.fifodepth < 16) {
    dev_err(&pl022.adev.dev,
    "TX FIFO Trigger Level is configured incorrectly\n");
    return -EINVAL;
    }
    break;
    case SSP_TX_32_OR_MORE_EMPTY_LOC:
    if (pl022.vendor.fifodepth < 32) {
    dev_err(&pl022.adev.dev,
    "TX FIFO Trigger Level is configured incorrectly\n");
    return -EINVAL;
    }
    break;
    default:
    dev_err(&pl022.adev.dev,
    "TX FIFO Trigger Level is configured incorrectly\n");
    return -EINVAL;
    }
    if (chip_info.iface == SSP_INTERFACE_NATIONAL_MICROWIRE) {
    if ((chip_info.ctrl_len < SSP_BITS_4)
    || (chip_info.ctrl_len > SSP_BITS_32)) {
    dev_err(&pl022.adev.dev,
    "CTRL LEN is configured incorrectly\n");
    return -EINVAL;
    }
    if ((chip_info.wait_state != SSP_MWIRE_WAIT_ZERO)
    && (chip_info.wait_state != SSP_MWIRE_WAIT_ONE)) {
    dev_err(&pl022.adev.dev,
    "Wait State is configured incorrectly\n");
    return -EINVAL;
    }
// Half duplex is only available in the ST Micro version
    if (pl022.vendor.extended_cr) {
    if ((chip_info.duplex !=
    SSP_MICROWIRE_CHANNEL_FULL_DUPLEX)
    && (chip_info.duplex !=
    SSP_MICROWIRE_CHANNEL_HALF_DUPLEX)) {
    dev_err(&pl022.adev.dev,
    "Microwire duplex mode is configured incorrectly\n");
    return -EINVAL;
    }
    } else {
    if (chip_info.duplex != SSP_MICROWIRE_CHANNEL_FULL_DUPLEX) {
    dev_err(&pl022.adev.dev,
    "Microwire half duplex mode requested,"
    " but this is only available in the"
    " ST version of PL022\n");
    return -EINVAL;
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn spi_rate(rate: u32, cpsdvsr: u16, scr: u16) -> u32 {
    static inline u32 spi_rate(u32 rate, u16 cpsdvsr, u16 scr)
    {
    return rate / (cpsdvsr * (1 + scr));
    }
    static int calculate_effective_freq(struct pl022 *pl022, int freq, struct
    ssp_clock_params * clk_freq)
    {
// Lets calculate the frequency parameters
    let mut cpsdvsr: u16 = CPSDVR_MIN, scr = SCR_MIN;
    u32 rate, max_tclk, min_tclk, best_freq = 0, best_cpsdvsr = 0,
    best_scr = 0, tmp, found = 0;
    rate = clk_get_rate(pl022.clk);
// cpsdvscr = 2 & scr 0
    max_tclk = spi_rate(rate, CPSDVR_MIN, SCR_MIN);
// cpsdvsr = 254 & scr = 255
    min_tclk = spi_rate(rate, CPSDVR_MAX, SCR_MAX);
    if (freq > max_tclk)
    dev_warn(&pl022.adev.dev,
    "Max speed that can be programmed is %d Hz, you requested %d\n",
    max_tclk, freq);
    if (freq < min_tclk) {
    dev_err(&pl022.adev.dev,
    "Requested frequency: %d Hz is less than minimum possible %d Hz\n",
    freq, min_tclk);
    return -EINVAL;
    }
//
// best_freq will give closest possible available rate (<= requested
// freq) for all values of scr & cpsdvsr.
//
    while ((cpsdvsr <= CPSDVR_MAX) && !found) {
    while (scr <= SCR_MAX) {
    tmp = spi_rate(rate, cpsdvsr, scr);
    if (tmp > freq) {
// we need lower freq
    scr++;
    continue;
    }
//
// If found exact value, mark found and break.
// If found more closer value, update and break.
//
    if (tmp > best_freq) {
    best_freq = tmp;
    best_cpsdvsr = cpsdvsr;
    best_scr = scr;
    if (tmp == freq)
    found = 1;
    }
//
// increased scr will give lower rates, which are not
// required
//
    break;
    }
    cpsdvsr += 2;
    scr = SCR_MIN;
    }
    WARN(!best_freq, "pl022: Matching cpsdvsr and scr not found for %d Hz rate \n",
    freq);
    clk_freq.cpsdvsr = (u8) (best_cpsdvsr & 0xFF);
    clk_freq.scr = (u8) (best_scr & 0xFF);
    dev_dbg(&pl022.adev.dev,
    "SSP Target Frequency is: %u, Effective Frequency is %u\n",
    freq, best_freq);
    dev_dbg(&pl022.adev.dev, "SSP cpsdvsr = %d, scr = %d\n",
    clk_freq.cpsdvsr, clk_freq.scr);
    return 0;
    }
//
// A piece of default chip info unless the platform
// supplies it.
//
    static const struct pl022_config_chip pl022_default_chip_info = {
    .com_mode = INTERRUPT_TRANSFER,
    .iface = SSP_INTERFACE_MOTOROLA_SPI,
    .hierarchy = SSP_MASTER,
    .slave_tx_disable = DO_NOT_DRIVE_TX,
    .rx_lev_trig = SSP_RX_1_OR_MORE_ELEM,
    .tx_lev_trig = SSP_TX_1_OR_MORE_EMPTY_LOC,
    .ctrl_len = SSP_BITS_8,
    .wait_state = SSP_MWIRE_WAIT_ZERO,
    .duplex = SSP_MICROWIRE_CHANNEL_FULL_DUPLEX,
    };
//
// pl022_setup - setup function registered to SPI host framework
// @spi: spi device which is requesting setup
//
// This function is registered to the SPI framework for this SPI host
// controller. If it is the first time when setup is called by this device,
// this function will initialize the runtime state for this chip and save
// the same in the device structure. Else it will update the runtime info
// with the updated chip info. Nothing is really being written to the
// controller hardware here, that is not done until the actual transfer
// commence.
//
#[no_mangle]
unsafe extern "C" fn pl022_setup(spi: *mut spi_device) -> c_int {
    static int pl022_setup(struct spi_device *spi)
    {
    struct pl022_config_chip const *chip_info;
    struct pl022_config_chip chip_info_dt;
    struct chip_data *chip;
    let mut clk_freq: ssp_clock_params = { .cpsdvsr = 0, .scr = 0};
    let mut status: c_int = 0;
    struct pl022 *pl022 = spi_controller_get_devdata(spi.controller);
    let mut bits: c_uint = spi.bits_per_word;
    u32 tmp;
    struct device_node *np = spi.dev.of_node;
    if (!spi.max_speed_hz)
    return -EINVAL;
// Get controller_state if one is supplied
    chip = spi_get_ctldata(spi);
    if (chip == core::ptr::null_mut()) {
    chip = kzalloc_obj(struct chip_data);
    if (!chip)
    return -ENOMEM;
    dev_dbg(&spi.dev,
    "allocated memory for controller's runtime state\n");
    }
// Get controller data if one is supplied
    chip_info = spi.controller_data;
    if (chip_info == core::ptr::null_mut()) {
    if (np) {
    chip_info_dt = pl022_default_chip_info;
    chip_info_dt.hierarchy = SSP_MASTER;
    of_property_read_u32(np, "pl022,interface",
    &chip_info_dt.iface);
    of_property_read_u32(np, "pl022,com-mode",
    &chip_info_dt.com_mode);
    of_property_read_u32(np, "pl022,rx-level-trig",
    &chip_info_dt.rx_lev_trig);
    of_property_read_u32(np, "pl022,tx-level-trig",
    &chip_info_dt.tx_lev_trig);
    of_property_read_u32(np, "pl022,ctrl-len",
    &chip_info_dt.ctrl_len);
    of_property_read_u32(np, "pl022,wait-state",
    &chip_info_dt.wait_state);
    of_property_read_u32(np, "pl022,duplex",
    &chip_info_dt.duplex);
    chip_info = &chip_info_dt;
    } else {
    chip_info = &pl022_default_chip_info;
// spi_board_info.controller_data not is supplied
    dev_dbg(&spi.dev,
    "using default controller_data settings\n");
    }
    } else
    dev_dbg(&spi.dev,
    "using user supplied controller_data settings\n");
//
// We can override with custom divisors, else we use the board
// frequency setting
//
    if ((0 == chip_info.clk_freq.cpsdvsr)
    && (0 == chip_info.clk_freq.scr)) {
    status = calculate_effective_freq(pl022,
    spi.max_speed_hz,
    &clk_freq);
    if (status < 0)
    goto err_config_params;
    } else {
    memcpy(&clk_freq, &chip_info.clk_freq, sizeof(clk_freq));
    if ((clk_freq.cpsdvsr % 2) != 0)
    clk_freq.cpsdvsr =
    clk_freq.cpsdvsr - 1;
    }
    if ((clk_freq.cpsdvsr < CPSDVR_MIN)
    || (clk_freq.cpsdvsr > CPSDVR_MAX)) {
    status = -EINVAL;
    dev_err(&spi.dev,
    "cpsdvsr is configured incorrectly\n");
    goto err_config_params;
    }
    status = verify_controller_parameters(pl022, chip_info);
    if (status) {
    dev_err(&spi.dev, "controller data is incorrect");
    goto err_config_params;
    }
    pl022.rx_lev_trig = chip_info.rx_lev_trig;
    pl022.tx_lev_trig = chip_info.tx_lev_trig;
// Now set controller state based on controller data
    chip.xfer_type = chip_info.com_mode;
// Check bits per word with vendor specific range
    if ((bits <= 3) || (bits > pl022.vendor.max_bpw)) {
    status = -ENOTSUPP;
    dev_err(&spi.dev, "illegal data size for this controller!\n");
    dev_err(&spi.dev, "This controller can only handle 4 <= n <= %d bit words\n",
    pl022.vendor.max_bpw);
    goto err_config_params;
    } else if (bits <= 8) {
    dev_dbg(&spi.dev, "4 <= n <=8 bits per word\n");
    chip.n_bytes = 1;
    chip.read = READING_U8;
    chip.write = WRITING_U8;
    } else if (bits <= 16) {
    dev_dbg(&spi.dev, "9 <= n <= 16 bits per word\n");
    chip.n_bytes = 2;
    chip.read = READING_U16;
    chip.write = WRITING_U16;
    } else {
    dev_dbg(&spi.dev, "17 <= n <= 32 bits per word\n");
    chip.n_bytes = 4;
    chip.read = READING_U32;
    chip.write = WRITING_U32;
    }
// Now Initialize all register settings required for this chip
    chip.cr0 = 0;
    chip.cr1 = 0;
    chip.dmacr = 0;
    chip.cpsr = 0;
    if ((chip_info.com_mode == DMA_TRANSFER)
    && ((pl022.host_info).enable_dma)) {
    chip.enable_dma = true;
    dev_dbg(&spi.dev, "DMA mode set in controller state\n");
    SSP_WRITE_BITS(chip.dmacr, SSP_DMA_ENABLED,
    SSP_DMACR_MASK_RXDMAE, 0);
    SSP_WRITE_BITS(chip.dmacr, SSP_DMA_ENABLED,
    SSP_DMACR_MASK_TXDMAE, 1);
    } else {
    chip.enable_dma = false;
    dev_dbg(&spi.dev, "DMA mode NOT set in controller state\n");
    SSP_WRITE_BITS(chip.dmacr, SSP_DMA_DISABLED,
    SSP_DMACR_MASK_RXDMAE, 0);
    SSP_WRITE_BITS(chip.dmacr, SSP_DMA_DISABLED,
    SSP_DMACR_MASK_TXDMAE, 1);
    }
    chip.cpsr = clk_freq.cpsdvsr;
// Special setup for the ST micro extended control registers
    if (pl022.vendor.extended_cr) {
    u32 etx;
    if (pl022.vendor.pl023) {
// These bits are only in the PL023
    SSP_WRITE_BITS(chip.cr1, chip_info.clkdelay,
    SSP_CR1_MASK_FBCLKDEL_ST, 13);
    } else {
// These bits are in the PL022 but not PL023
    SSP_WRITE_BITS(chip.cr0, chip_info.duplex,
    SSP_CR0_MASK_HALFDUP_ST, 5);
    SSP_WRITE_BITS(chip.cr0, chip_info.ctrl_len,
    SSP_CR0_MASK_CSS_ST, 16);
    SSP_WRITE_BITS(chip.cr0, chip_info.iface,
    SSP_CR0_MASK_FRF_ST, 21);
    SSP_WRITE_BITS(chip.cr1, chip_info.wait_state,
    SSP_CR1_MASK_MWAIT_ST, 6);
    }
    SSP_WRITE_BITS(chip.cr0, bits - 1,
    SSP_CR0_MASK_DSS_ST, 0);
    if (spi.mode & SPI_LSB_FIRST) {
    tmp = SSP_RX_LSB;
    etx = SSP_TX_LSB;
    } else {
    tmp = SSP_RX_MSB;
    etx = SSP_TX_MSB;
    }
    SSP_WRITE_BITS(chip.cr1, tmp, SSP_CR1_MASK_RENDN_ST, 4);
    SSP_WRITE_BITS(chip.cr1, etx, SSP_CR1_MASK_TENDN_ST, 5);
    SSP_WRITE_BITS(chip.cr1, chip_info.rx_lev_trig,
    SSP_CR1_MASK_RXIFLSEL_ST, 7);
    SSP_WRITE_BITS(chip.cr1, chip_info.tx_lev_trig,
    SSP_CR1_MASK_TXIFLSEL_ST, 10);
    } else {
    SSP_WRITE_BITS(chip.cr0, bits - 1,
    SSP_CR0_MASK_DSS, 0);
    SSP_WRITE_BITS(chip.cr0, chip_info.iface,
    SSP_CR0_MASK_FRF, 4);
    }
// Stuff that is common for all versions
    if (spi.mode & SPI_CPOL)
    tmp = SSP_CLK_POL_IDLE_HIGH;
    else
    tmp = SSP_CLK_POL_IDLE_LOW;
    SSP_WRITE_BITS(chip.cr0, tmp, SSP_CR0_MASK_SPO, 6);
    if (spi.mode & SPI_CPHA)
    tmp = SSP_CLK_SECOND_EDGE;
    else
    tmp = SSP_CLK_FIRST_EDGE;
    SSP_WRITE_BITS(chip.cr0, tmp, SSP_CR0_MASK_SPH, 7);
    SSP_WRITE_BITS(chip.cr0, clk_freq.scr, SSP_CR0_MASK_SCR, 8);
// Loopback is available on all versions except PL023
    if (pl022.vendor.loopback) {
    if (spi.mode & SPI_LOOP)
    tmp = LOOPBACK_ENABLED;
    else
    tmp = LOOPBACK_DISABLED;
    SSP_WRITE_BITS(chip.cr1, tmp, SSP_CR1_MASK_LBM, 0);
    }
    SSP_WRITE_BITS(chip.cr1, SSP_DISABLED, SSP_CR1_MASK_SSE, 1);
    SSP_WRITE_BITS(chip.cr1, chip_info.hierarchy, SSP_CR1_MASK_MS, 2);
    SSP_WRITE_BITS(chip.cr1, chip_info.slave_tx_disable, SSP_CR1_MASK_SOD,
    3);
// Save controller_state
    spi_set_ctldata(spi, chip);
    return status;
    err_config_params:
    spi_set_ctldata(spi, core::ptr::null_mut());
    kfree(chip);
    return status;
    }
//
// pl022_cleanup - cleanup function registered to SPI host framework
// @spi: spi device which is requesting cleanup
//
// This function is registered to the SPI framework for this SPI host
// controller. It will free the runtime state of chip.
//
#[no_mangle]
unsafe extern "C" fn pl022_cleanup(spi: *mut spi_device) {
    static void pl022_cleanup(struct spi_device *spi)
    {
    struct chip_data *chip = spi_get_ctldata(spi);
    spi_set_ctldata(spi, core::ptr::null_mut());
    kfree(chip);
    }
    static struct pl022_ssp_controller *
    pl022_platform_data_dt_get(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct pl022_ssp_controller *pd;
    if (!np) {
    dev_err(dev, "no dt node defined\n");
    return core::ptr::null_mut();
    }
    pd = devm_kzalloc(dev, sizeof(struct pl022_ssp_controller), GFP_KERNEL);
    if (!pd)
    return core::ptr::null_mut();
    pd.bus_id = -1;
    of_property_read_u32(np, "pl022,autosuspend-delay",
    &pd.autosuspend_delay);
    pd.rt = of_property_read_bool(np, "pl022,rt");
    return pd;
    }
#[no_mangle]
unsafe extern "C" fn pl022_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int pl022_probe(struct amba_device *adev, const struct amba_id *id)
    {
    struct device *dev = &adev.dev;
    struct pl022_ssp_controller *platform_info =
    dev_get_platdata(&adev.dev);
    struct spi_controller *host;
    struct pl022 *pl022 = core::ptr::null_mut();	/*Data for this driver */
    let mut status: c_int = 0;
    dev_info(&adev.dev,
    "ARM PL022 driver, device ID: 0x%08x\n", adev.periphid);
    if (!platform_info && IS_ENABLED(CONFIG_OF))
    platform_info = pl022_platform_data_dt_get(dev);
    if (!platform_info) {
    dev_err(dev, "probe: no platform data defined\n");
    return -ENODEV;
    }
// Allocate host with space for data
    host = devm_spi_alloc_host(dev, sizeof(struct pl022));
    if (host == core::ptr::null_mut()) {
    dev_err(&adev.dev, "probe - cannot alloc SPI host\n");
    return -ENOMEM;
    }
    pl022 = spi_controller_get_devdata(host);
    pl022.host = host;
    pl022.host_info = platform_info;
    pl022.adev = adev;
    pl022.vendor = id.data;
//
// Bus Number Which has been Assigned to this SSP controller
// on this board
//
    host.bus_num = platform_info.bus_id;
    host.cleanup = pl022_cleanup;
    host.setup = pl022_setup;
    host.auto_runtime_pm = true;
    host.transfer_one = pl022_transfer_one;
    host.set_cs = pl022_cs_control;
    host.handle_err = pl022_handle_err;
    host.unprepare_transfer_hardware = pl022_unprepare_transfer_hardware;
    host.rt = platform_info.rt;
    host.use_gpio_descriptors = true;
//
// Supports mode 0-3, loopback, and active low CS. Transfers are
// always MS bit first on the original pl022.
//
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH | SPI_LOOP;
    if (pl022.vendor.extended_cr)
    host.mode_bits |= SPI_LSB_FIRST;
    dev_dbg(&adev.dev, "BUSNO: %d\n", host.bus_num);
    status = amba_request_regions(adev, core::ptr::null_mut());
    if (status)
    return status;
    pl022.phybase = adev.res.start;
    pl022.virtbase = devm_ioremap(dev, adev.res.start,
    resource_size(&adev.res));
    if (pl022.virtbase == core::ptr::null_mut()) {
    status = -ENOMEM;
    goto err_no_ioremap;
    }
    dev_info(&adev.dev, "mapped registers from %pa to %p\n",
    &adev.res.start, pl022.virtbase);
    pl022.clk = devm_clk_get_enabled(&adev.dev, core::ptr::null_mut());
    if (IS_ERR(pl022.clk)) {
    status = PTR_ERR(pl022.clk);
    dev_err(&adev.dev, "could not retrieve SSP/SPI bus clock\n");
    goto err_no_clk;
    }
// Disable SSP
    writew((readw(SSP_CR1(pl022.virtbase)) & (~SSP_CR1_MASK_SSE)),
    SSP_CR1(pl022.virtbase));
    load_ssp_default_config(pl022);
    status = devm_request_irq(dev, adev.irq[0], pl022_interrupt_handler,
    0, "pl022", pl022);
    if (status < 0) {
    dev_err(&adev.dev, "probe - cannot get IRQ (%d)\n", status);
    goto err_no_irq;
    }
// Get DMA channels, try autoconfiguration first
    status = pl022_dma_autoprobe(pl022);
    if (status == -EPROBE_DEFER) {
    dev_dbg(dev, "deferring probe to get DMA channel\n");
    goto err_no_irq;
    }
// If that failed, use channels from platform_info
    if (status == 0)
    platform_info.enable_dma = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: platform_info->enable_dma) -> else {
    status = pl022_dma_probe(pl022);
    if (status != 0)
    platform_info.enable_dma = 0;
    }
// Register with the SPI framework
    amba_set_drvdata(adev, pl022);
    status = spi_register_controller(host);
    if (status != 0) {
    dev_err_probe(&adev.dev, status,
    "problem registering spi host\n");
    goto err_spi_register;
    }
    dev_dbg(dev, "probe succeeded\n");
// let runtime pm put suspend
    if (platform_info.autosuspend_delay > 0) {
    dev_info(&adev.dev,
    "will use autosuspend for runtime pm, delay %dms\n",
    platform_info.autosuspend_delay);
    pm_runtime_set_autosuspend_delay(dev,
    platform_info.autosuspend_delay);
    pm_runtime_use_autosuspend(dev);
    }
    pm_runtime_put(dev);
    return 0;
    err_spi_register:
    if (platform_info.enable_dma)
    pl022_dma_remove(pl022);
    err_no_irq:
    err_no_clk:
    err_no_ioremap:
    amba_release_regions(adev);
    return status;
    }
    static void
    pl022_remove(struct amba_device *adev)
    {
    struct pl022 *pl022 = amba_get_drvdata(adev);
    if (!pl022)
    return;
    spi_unregister_controller(pl022.host);
//
// undo pm_runtime_put() in probe.  I assume that we're not
// accessing the primecell here.
//
    pm_runtime_get_noresume(&adev.dev);
    load_ssp_default_config(pl022);
    if (pl022.host_info.enable_dma)
    pl022_dma_remove(pl022);
    amba_release_regions(adev);
    }
#[no_mangle]
unsafe extern "C" fn pl022_suspend(dev: *mut device) -> c_int {
    static int pl022_suspend(struct device *dev)
    {
    struct pl022 *pl022 = dev_get_drvdata(dev);
    int ret;
    ret = spi_controller_suspend(pl022.host);
    if (ret)
    return ret;
    ret = pm_runtime_force_suspend(dev);
    if (ret) {
    spi_controller_resume(pl022.host);
    return ret;
    }
    pinctrl_pm_select_sleep_state(dev);
    dev_dbg(dev, "suspended\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl022_resume(dev: *mut device) -> c_int {
    static int pl022_resume(struct device *dev)
    {
    struct pl022 *pl022 = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_resume(dev);
    if (ret)
    dev_err(dev, "problem resuming\n");
// Start the queue running
    ret = spi_controller_resume(pl022.host);
    if (!ret)
    dev_dbg(dev, "resumed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pl022_runtime_suspend(dev: *mut device) -> c_int {
    static int pl022_runtime_suspend(struct device *dev)
    {
    struct pl022 *pl022 = dev_get_drvdata(dev);
    clk_disable_unprepare(pl022.clk);
    pinctrl_pm_select_idle_state(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl022_runtime_resume(dev: *mut device) -> c_int {
    static int pl022_runtime_resume(struct device *dev)
    {
    struct pl022 *pl022 = dev_get_drvdata(dev);
    pinctrl_pm_select_default_state(dev);
    clk_prepare_enable(pl022.clk);
    return 0;
    }
    static const struct dev_pm_ops pl022_dev_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(pl022_suspend, pl022_resume)
    RUNTIME_PM_OPS(pl022_runtime_suspend, pl022_runtime_resume, core::ptr::null_mut())
    };
    static struct vendor_data vendor_arm = {
    .fifodepth = 8,
    .max_bpw = 16,
    .unidir = false,
    .extended_cr = false,
    .pl023 = false,
    .loopback = true,
    .internal_cs_ctrl = false,
    };
    static struct vendor_data vendor_st = {
    .fifodepth = 32,
    .max_bpw = 32,
    .unidir = false,
    .extended_cr = true,
    .pl023 = false,
    .loopback = true,
    .internal_cs_ctrl = false,
    };
    static struct vendor_data vendor_st_pl023 = {
    .fifodepth = 32,
    .max_bpw = 32,
    .unidir = false,
    .extended_cr = true,
    .pl023 = true,
    .loopback = false,
    .internal_cs_ctrl = false,
    };
    static struct vendor_data vendor_lsi = {
    .fifodepth = 8,
    .max_bpw = 16,
    .unidir = false,
    .extended_cr = false,
    .pl023 = false,
    .loopback = true,
    .internal_cs_ctrl = true,
    };
    static const struct amba_id pl022_ids[] = {
    {
//
// ARM PL022 variant, this has a 16bit wide
// and 8 locations deep TX/RX FIFO
//
    .id	= 0x00041022,
    .mask	= 0x000fffff,
    .data	= &vendor_arm,
    },
    {
//
// ST Micro derivative, this has 32bit wide
// and 32 locations deep TX/RX FIFO
//
    .id	= 0x01080022,
    .mask	= 0xffffffff,
    .data	= &vendor_st,
    },
    {
//
// ST-Ericsson derivative "PL023" (this is not
// an official ARM number), this is a PL022 SSP block
// stripped to SPI mode only, it has 32bit wide
// and 32 locations deep TX/RX FIFO but no extended
// CR0/CR1 register
//
    .id	= 0x00080023,
    .mask	= 0xffffffff,
    .data	= &vendor_st_pl023,
    },
    {
//
// PL022 variant that has a chip select control register whih
// allows control of 5 output signals nCS[0:4].
//
    .id	= 0x000b6022,
    .mask	= 0x000fffff,
    .data	= &vendor_lsi,
    },
    { 0, 0 },
    };
    MODULE_DEVICE_TABLE(amba, pl022_ids);
    static struct amba_driver pl022_driver = {
    .drv = {
    .name	= "ssp-pl022",
    .pm	= pm_ptr(&pl022_dev_pm_ops),
    },
    .id_table	= pl022_ids,
    .probe		= pl022_probe,
    .remove		= pl022_remove,
    };
#[no_mangle]
unsafe extern "C" fn pl022_init() -> int __init {
    static int __init pl022_init(void)
    {
    return amba_driver_register(&pl022_driver);
    }
    subsys_initcall(pl022_init);
#[no_mangle]
unsafe extern "C" fn pl022_exit() -> void __exit {
    static void __exit pl022_exit(void)
    {
    amba_driver_unregister(&pl022_driver);
    }
    module_exit(pl022_exit);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@stericsson.com>");
    MODULE_DESCRIPTION("PL022 SSP Controller Driver");
    MODULE_LICENSE("GPL");
