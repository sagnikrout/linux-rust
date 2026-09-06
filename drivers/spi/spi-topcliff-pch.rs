//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-topcliff-pch.c
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
// SPI bus driver for the Topcliff PCH used by Intel SoCs
//
// Copyright (C) 2011 LAPIS Semiconductor Co., Ltd.
//

// Register offsets
pub const PCH_SPCR: c_uint = 0x00	/* SPI control register */;
pub const PCH_SPBRR: c_uint = 0x04	/* SPI baud rate register */;
pub const PCH_SPSR: c_uint = 0x08	/* SPI status register */;
pub const PCH_SPDWR: c_uint = 0x0C	/* SPI write data register */;
pub const PCH_SPDRR: c_uint = 0x10	/* SPI read data register */;
pub const PCH_SSNXCR: c_uint = 0x18	/* SSN Expand Control Register */;
pub const PCH_SRST: c_uint = 0x1C	/* SPI reset register */;
pub const PCH_ADDRESS_SIZE: c_uint = 0x20;
pub const PCH_SPSR_TFD: c_uint = 0x000007C0;
pub const PCH_SPSR_RFD: c_uint = 0x0000F800;

pub const PCH_RX_THOLD: c_int = 7;
pub const PCH_RX_THOLD_MAX: c_int = 15;
pub const PCH_TX_THOLD: c_int = 2;
pub const PCH_MAX_BAUDRATE: c_int = 5000000;
pub const PCH_MAX_FIFO_DEPTH: c_int = 16;
pub const STATUS_RUNNING: c_int = 1;
pub const STATUS_EXITING: c_int = 2;
pub const PCH_SLEEP_TIME: c_int = 10;
pub const SSN_LOW: c_uint = 0x02U;
pub const SSN_HIGH: c_uint = 0x03U;
pub const SSN_NO_CONTROL: c_uint = 0x00U;
pub const PCH_MAX_CS: c_uint = 0xFF;
pub const PCI_DEVICE_ID_GE_SPI: c_uint = 0x8816;

    SPCR_ORIE_BIT|SPCR_MDFIE_BIT)
pub const SPCR_RFIC_FIELD: c_int = 20;
pub const SPCR_TFIC_FIELD: c_int = 16;

pub const PCH_CLOCK_HZ: c_int = 50000000;
pub const PCH_MAX_SPBR: c_int = 1023;
// Definition for ML7213/ML7223/ML7831 by LAPIS Semiconductor
pub const PCI_DEVICE_ID_ML7213_SPI: c_uint = 0x802c;
pub const PCI_DEVICE_ID_ML7223_SPI: c_uint = 0x800F;
pub const PCI_DEVICE_ID_ML7831_SPI: c_uint = 0x8816;
//
// Set the number of SPI instance max
// Intel EG20T PCH :		1ch
// LAPIS Semiconductor ML7213 IOH :	2ch
// LAPIS Semiconductor ML7223 IOH :	1ch
// LAPIS Semiconductor ML7831 IOH :	1ch
//
pub const PCH_SPI_MAX_DEV: c_int = 2;
pub const PCH_BUF_SIZE: c_int = 4096;
pub const PCH_DMA_TRANS_SIZE: c_int = 12;
    let mut use_dma: static int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_spi_dma_ctrl {
    pub dma_dev: *mut pci_dev,
    pub desc_tx: *mut dma_async_tx_descriptor,
    pub desc_rx: *mut dma_async_tx_descriptor,
    pub param_tx: pch_dma_slave,
    pub param_rx: pch_dma_slave,
    pub chan_tx: *mut dma_chan,
    pub chan_rx: *mut dma_chan,
    pub sg_tx_p: *mut scatterlist,
    pub sg_rx_p: *mut scatterlist,
    pub sg_tx: scatterlist,
    pub sg_rx: scatterlist,
    pub nent: c_int,
    pub tx_buf_virt: *mut c_void,
    pub rx_buf_virt: *mut c_void,
    pub tx_buf_dma: dma_addr_t,
    pub rx_buf_dma: dma_addr_t,
}

//
// struct pch_spi_data - Holds the SPI channel specific details
// @io_remap_addr:		The remapped PCI base address
// @io_base_addr:		Base address
// @host:			Pointer to the SPI controller structure
// @work:			Reference to work queue handler
// @wait:			Wait queue for waking up upon receiving an
// interrupt.
// @transfer_complete:		Status of SPI Transfer
// @bcurrent_msg_processing:	Status flag for message processing
// @lock:			Lock for protecting this structure
// @queue:			SPI Message queue
// @status:			Status of the SPI driver
// @bpw_len:			Length of data to be transferred in bits per
// word
// @transfer_active:		Flag showing active transfer
// @tx_index:			Transmit data count; for bookkeeping during
// transfer
// @rx_index:			Receive data count; for bookkeeping during
// transfer
// @pkt_tx_buff:		Buffer for data to be transmitted
// @pkt_rx_buff:		Buffer for received data
// @n_curnt_chip:		The chip number that this SPI driver currently
// operates on
// @current_chip:		Reference to the current chip that this SPI
// driver currently operates on
// @current_msg:		The current message that this SPI driver is
// handling
// @cur_trans:			The current transfer that this SPI driver is
// handling
// @board_dat:			Reference to the SPI device data structure
// @plat_dev:			platform_device structure
// @ch:				SPI channel number
// @dma:			Local DMA information
// @use_dma:			True if DMA is to be used
// @irq_reg_sts:		Status of IRQ registration
// @save_total_len:		Save length while data is being transferred
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_spi_data {
    pub io_remap_addr: *mut void __iomem,
    pub io_base_addr: c_ulong,
    pub host: *mut spi_controller,
    pub work: work_struct,
    pub wait: wait_queue_head_t,
    pub transfer_complete: u8,
    pub bcurrent_msg_processing: u8,
    pub lock: spinlock_t,
    pub queue: list_head,
    pub status: u8,
    pub bpw_len: u32,
    pub transfer_active: u8,
    pub tx_index: u32,
    pub rx_index: u32,
    pub pkt_tx_buff: *mut u16,
    pub pkt_rx_buff: *mut u16,
    pub n_curnt_chip: u8,
    pub current_chip: *mut spi_device,
    pub current_msg: *mut spi_message,
    pub cur_trans: *mut spi_transfer,
    pub board_dat: *mut pch_spi_board_data,
    pub plat_dev: *mut platform_device,
    pub ch: c_int,
    pub dma: pch_spi_dma_ctrl,
    pub use_dma: c_int,
    pub irq_reg_sts: u8,
    pub save_total_len: c_int,
}

//
// struct pch_spi_board_data - Holds the SPI device specific details
// @pdev:		Pointer to the PCI device
// @suspend_sts:	Status of suspend
// @num:		The number of SPI device instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_spi_board_data {
    pub pdev: *mut pci_dev,
    pub suspend_sts: u8,
    pub num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_pd_dev_save {
    pub num: c_int,
    pub pd_save: [*mut platform_device; PCH_SPI_MAX_DEV],
    pub board_dat: *mut pch_spi_board_data,
}

    static const struct pci_device_id pch_spi_pcidev_id[] = {
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_GE_SPI),    .driver_data = 1 },
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7213_SPI), .driver_data = 2 },
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7223_SPI), .driver_data = 1 },
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7831_SPI), .driver_data = 1 },
    { }
    };
//
// pch_spi_writereg() - Performs  register writes
// @host:	Pointer to struct spi_controller.
// @idx:	Register offset.
// @val:	Value to be written to register.
//
#[no_mangle]
pub unsafe extern "C" fn pch_spi_writereg(host: *mut spi_controller, idx: c_int, val: u32) {
    static inline void pch_spi_writereg(struct spi_controller *host, int idx, u32 val)
    {
    struct pch_spi_data *data = spi_controller_get_devdata(host);
    iowrite32(val, (data.io_remap_addr + idx));
    }
//
// pch_spi_readreg() - Performs register reads
// @host:	Pointer to struct spi_controller.
// @idx:	Register offset.
//
#[no_mangle]
pub unsafe extern "C" fn pch_spi_readreg(host: *mut spi_controller, idx: c_int) -> u32 {
    static inline u32 pch_spi_readreg(struct spi_controller *host, int idx)
    {
    struct pch_spi_data *data = spi_controller_get_devdata(host);
    return ioread32(data.io_remap_addr + idx);
    }
    static inline void pch_spi_setclr_reg(struct spi_controller *host, int idx,
    u32 set, u32 clr)
    {
    let mut tmp: u32 = pch_spi_readreg(host, idx);
    tmp = (tmp & ~clr) | set;
    pch_spi_writereg(host, idx, tmp);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_set_host_mode(host: *mut spi_controller) {
    static void pch_spi_set_host_mode(struct spi_controller *host)
    {
    pch_spi_setclr_reg(host, PCH_SPCR, SPCR_MSTR_BIT, 0);
    }
//
// pch_spi_clear_fifo() - Clears the Transmit and Receive FIFOs
// @host:	Pointer to struct spi_controller.
//
#[no_mangle]
unsafe extern "C" fn pch_spi_clear_fifo(host: *mut spi_controller) {
    static void pch_spi_clear_fifo(struct spi_controller *host)
    {
    pch_spi_setclr_reg(host, PCH_SPCR, SPCR_FICLR_BIT, 0);
    pch_spi_setclr_reg(host, PCH_SPCR, 0, SPCR_FICLR_BIT);
    }
    static void pch_spi_handler_sub(struct pch_spi_data *data, u32 reg_spsr_val,
    void __iomem *io_remap_addr)
    {
    u32 n_read, tx_index, rx_index, bpw_len;
    u16 *pkt_rx_buffer, *pkt_tx_buff;
    int read_cnt;
    u32 reg_spcr_val;
    void __iomem *spsr;
    void __iomem *spdrr;
    void __iomem *spdwr;
    spsr = io_remap_addr + PCH_SPSR;
    iowrite32(reg_spsr_val, spsr);
    if (data.transfer_active) {
    rx_index = data.rx_index;
    tx_index = data.tx_index;
    bpw_len = data.bpw_len;
    pkt_rx_buffer = data.pkt_rx_buff;
    pkt_tx_buff = data.pkt_tx_buff;
    spdrr = io_remap_addr + PCH_SPDRR;
    spdwr = io_remap_addr + PCH_SPDWR;
    n_read = PCH_READABLE(reg_spsr_val);
    for (read_cnt = 0; (read_cnt < n_read); read_cnt++) {
    pkt_rx_buffer[rx_index++] = ioread32(spdrr);
    if (tx_index < bpw_len)
    iowrite32(pkt_tx_buff[tx_index++], spdwr);
    }
// disable RFI if not needed
    if ((bpw_len - rx_index) <= PCH_MAX_FIFO_DEPTH) {
    reg_spcr_val = ioread32(io_remap_addr + PCH_SPCR);
    reg_spcr_val &= ~SPCR_RFIE_BIT; /* disable RFI */
// reset rx threshold
    reg_spcr_val &= ~MASK_RFIC_SPCR_BITS;
    reg_spcr_val |= (PCH_RX_THOLD_MAX << SPCR_RFIC_FIELD);
    iowrite32(reg_spcr_val, (io_remap_addr + PCH_SPCR));
    }
// update counts
    data.tx_index = tx_index;
    data.rx_index = rx_index;
// if transfer complete interrupt
    if (reg_spsr_val & SPSR_FI_BIT) {
    if ((tx_index == bpw_len) && (rx_index == tx_index)) {
// disable interrupts
    pch_spi_setclr_reg(data.host, PCH_SPCR, 0,
    PCH_ALL);
// transfer is completed;
    inform pch_spi_process_messages */
    data.transfer_complete = true;
    data.transfer_active = false;
    wake_up(&data.wait);
    } else {
    dev_vdbg(&data.host.dev,
    "%s : Transfer is not completed",
    __func__);
    }
    }
    }
    }
//
// pch_spi_handler() - Interrupt handler
// @irq:	The interrupt number.
// @dev_id:	Pointer to struct pch_spi_board_data.
//
#[no_mangle]
unsafe extern "C" fn pch_spi_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pch_spi_handler(int irq, void *dev_id)
    {
    u32 reg_spsr_val;
    void __iomem *spsr;
    void __iomem *io_remap_addr;
    let mut ret: irqreturn_t = IRQ_NONE;
    struct pch_spi_data *data = dev_id;
    struct pch_spi_board_data *board_dat = data.board_dat;
    if (board_dat.suspend_sts) {
    dev_dbg(&board_dat.pdev.dev,
    "%s returning due to suspend\n", __func__);
    return IRQ_NONE;
    }
    io_remap_addr = data.io_remap_addr;
    spsr = io_remap_addr + PCH_SPSR;
    reg_spsr_val = ioread32(spsr);
    if (reg_spsr_val & SPSR_ORF_BIT) {
    dev_err(&board_dat.pdev.dev, "%s Over run error\n", __func__);
    if (data.current_msg.complete) {
    data.transfer_complete = true;
    data.current_msg.status = -EIO;
    data.current_msg.complete(data.current_msg.context);
    data.bcurrent_msg_processing = false;
    data.current_msg = core::ptr::null_mut();
    data.cur_trans = core::ptr::null_mut();
    }
    }
    if (data.use_dma)
    return IRQ_NONE;
// Check if the interrupt is for SPI device
    if (reg_spsr_val & (SPSR_FI_BIT | SPSR_RFI_BIT)) {
    pch_spi_handler_sub(data, reg_spsr_val, io_remap_addr);
    ret = IRQ_HANDLED;
    }
    dev_dbg(&board_dat.pdev.dev, "%s EXIT return value=%d\n",
    __func__, ret);
    return ret;
    }
//
// pch_spi_set_baud_rate() - Sets SPBR field in SPBRR
// @host:	Pointer to struct spi_controller.
// @speed_hz:	Baud rate.
//
#[no_mangle]
unsafe extern "C" fn pch_spi_set_baud_rate(host: *mut spi_controller, speed_hz: u32) {
    static void pch_spi_set_baud_rate(struct spi_controller *host, u32 speed_hz)
    {
    let mut n_spbr: u32 = PCH_CLOCK_HZ / (speed_hz * 2);
// if baud rate is less than we can support limit it
    if (n_spbr > PCH_MAX_SPBR)
    n_spbr = PCH_MAX_SPBR;
    pch_spi_setclr_reg(host, PCH_SPBRR, n_spbr, MASK_SPBRR_SPBR_BITS);
    }
//
// pch_spi_set_bits_per_word() - Sets SIZE field in SPBRR
// @host:		Pointer to struct spi_controller.
// @bits_per_word:	Bits per word for SPI transfer.
//
    static void pch_spi_set_bits_per_word(struct spi_controller *host,
    u8 bits_per_word)
    {
    if (bits_per_word == 8)
    pch_spi_setclr_reg(host, PCH_SPBRR, 0, SPBRR_SIZE_BIT);
    else
    pch_spi_setclr_reg(host, PCH_SPBRR, SPBRR_SIZE_BIT, 0);
    }
//
// pch_spi_setup_transfer() - Configures the PCH SPI hardware for transfer
// @spi:	Pointer to struct spi_device.
//
#[no_mangle]
unsafe extern "C" fn pch_spi_setup_transfer(spi: *mut spi_device) {
    static void pch_spi_setup_transfer(struct spi_device *spi)
    {
    let mut flags: u32 = 0;
    dev_dbg(&spi.dev, "%s SPBRR content =%x setting baud rate=%d\n",
    __func__, pch_spi_readreg(spi.controller, PCH_SPBRR),
    spi.max_speed_hz);
    pch_spi_set_baud_rate(spi.controller, spi.max_speed_hz);
// set bits per word
    pch_spi_set_bits_per_word(spi.controller, spi.bits_per_word);
    if (!(spi.mode & SPI_LSB_FIRST))
    flags |= SPCR_LSBF_BIT;
    if (spi.mode & SPI_CPOL)
    flags |= SPCR_CPOL_BIT;
    if (spi.mode & SPI_CPHA)
    flags |= SPCR_CPHA_BIT;
    pch_spi_setclr_reg(spi.controller, PCH_SPCR, flags,
    (SPCR_LSBF_BIT | SPCR_CPOL_BIT | SPCR_CPHA_BIT));
// Clear the FIFO by toggling  FICLR to 1 and back to 0
    pch_spi_clear_fifo(spi.controller);
    }
//
// pch_spi_reset() - Clears SPI registers
// @host:	Pointer to struct spi_controller.
//
#[no_mangle]
unsafe extern "C" fn pch_spi_reset(host: *mut spi_controller) {
    static void pch_spi_reset(struct spi_controller *host)
    {
// write 1 to reset SPI
    pch_spi_writereg(host, PCH_SRST, 0x1);
// clear reset
    pch_spi_writereg(host, PCH_SRST, 0x0);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_transfer(pspi: *mut spi_device, pmsg: *mut spi_message) -> c_int {
    static int pch_spi_transfer(struct spi_device *pspi, struct spi_message *pmsg)
    {
    struct pch_spi_data *data = spi_controller_get_devdata(pspi.controller);
    int retval;
    unsigned long flags;
// We won't process any messages if we have been asked to terminate
    if (data.status == STATUS_EXITING) {
    dev_err(&pspi.dev, "%s status = STATUS_EXITING.\n", __func__);
    retval = -ESHUTDOWN;
    goto err_out;
    }
// If suspended ,return -EINVAL
    if (data.board_dat.suspend_sts) {
    dev_err(&pspi.dev, "%s suspend; returning EINVAL\n", __func__);
    retval = -EINVAL;
    goto err_out;
    }
// set status of message
    pmsg.actual_length = 0;
    dev_dbg(&pspi.dev, "%s - pmsg.status =%d\n", __func__, pmsg.status);
    pmsg.status = -EINPROGRESS;
    spin_lock_irqsave(&data.lock, flags);
// add message to queue
    list_add_tail(&pmsg.queue, &data.queue);
    spin_unlock_irqrestore(&data.lock, flags);
    dev_dbg(&pspi.dev, "%s - Invoked list_add_tail\n", __func__);
    schedule_work(&data.work);
    dev_dbg(&pspi.dev, "%s - Invoked queue work\n", __func__);
    retval = 0;
    err_out:
    dev_dbg(&pspi.dev, "%s RETURN=%d\n", __func__, retval);
    return retval;
    }
    static inline void pch_spi_select_chip(struct pch_spi_data *data,
    struct spi_device *pspi)
    {
    if (data.current_chip != core::ptr::null_mut()) {
    if (spi_get_chipselect(pspi, 0) != data.n_curnt_chip) {
    dev_dbg(&pspi.dev, "%s : different slave\n", __func__);
    data.current_chip = core::ptr::null_mut();
    }
    }
    data.current_chip = pspi;
    data.n_curnt_chip = spi_get_chipselect(data.current_chip, 0);
    dev_dbg(&pspi.dev, "%s :Invoking pch_spi_setup_transfer\n", __func__);
    pch_spi_setup_transfer(pspi);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_set_tx(data: *mut pch_spi_data, bpw: *mut c_int) {
    static void pch_spi_set_tx(struct pch_spi_data *data, int *bpw)
    {
    int size;
    u32 n_writes;
    int j;
    struct spi_message *pmsg, *tmp;
    const u8 *tx_buf;
    const u16 *tx_sbuf;
// set baud rate if needed
    if (data.cur_trans.speed_hz) {
    dev_dbg(&data.host.dev, "%s:setting baud rate\n", __func__);
    pch_spi_set_baud_rate(data.host, data.cur_trans.speed_hz);
    }
// set bits per word if needed
    if (data.cur_trans.bits_per_word &&
    (data.current_msg.spi.bits_per_word != data.cur_trans.bits_per_word)) {
    dev_dbg(&data.host.dev, "%s:set bits per word\n", __func__);
    pch_spi_set_bits_per_word(data.host,
    data.cur_trans.bits_per_word);
// bpw = data->cur_trans->bits_per_word;
    } else {
// bpw = data->current_msg->spi->bits_per_word;
    }
// reset Tx/Rx index
    data.tx_index = 0;
    data.rx_index = 0;
    data.bpw_len = data.cur_trans.len / (*bpw / 8);
// find alloc size
    size = data.cur_trans.len * sizeof(*data.pkt_tx_buff);
// allocate memory for pkt_tx_buff & pkt_rx_buffer
    data.pkt_tx_buff = kzalloc(size, GFP_KERNEL);
    if (data.pkt_tx_buff != core::ptr::null_mut()) {
    data.pkt_rx_buff = kzalloc(size, GFP_KERNEL);
    if (!data.pkt_rx_buff) {
    kfree(data.pkt_tx_buff);
    data.pkt_tx_buff = core::ptr::null_mut();
    }
    }
    if (!data.pkt_rx_buff) {
// flush queue and set status of all transfers to -ENOMEM
    list_for_each_entry_safe(pmsg, tmp, data.queue.next, queue) {
    pmsg.status = -ENOMEM;
    if (pmsg.complete)
    pmsg.complete(pmsg.context);
// delete from queue
    list_del_init(&pmsg.queue);
    }
    return;
    }
// copy Tx Data
    if (data.cur_trans.tx_buf != core::ptr::null_mut()) {
    if (*bpw == 8) {
    tx_buf = data.cur_trans.tx_buf;
    for (j = 0; j < data.bpw_len; j++)
    data.pkt_tx_buff[j] = *tx_buf++;
    } else {
    tx_sbuf = data.cur_trans.tx_buf;
    for (j = 0; j < data.bpw_len; j++)
    data.pkt_tx_buff[j] = *tx_sbuf++;
    }
    }
// if len greater than PCH_MAX_FIFO_DEPTH, write 16,else len bytes
    n_writes = data.bpw_len;
    if (n_writes > PCH_MAX_FIFO_DEPTH)
    n_writes = PCH_MAX_FIFO_DEPTH;
    dev_dbg(&data.host.dev,
    "\n%s:Pulling down SSN low - writing 0x2 to SSNXCR\n",
    __func__);
    pch_spi_writereg(data.host, PCH_SSNXCR, SSN_LOW);
    for (j = 0; j < n_writes; j++)
    pch_spi_writereg(data.host, PCH_SPDWR, data.pkt_tx_buff[j]);
// update tx_index
    data.tx_index = j;
// reset transfer complete flag
    data.transfer_complete = false;
    data.transfer_active = true;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_nomore_transfer(data: *mut pch_spi_data) {
    static void pch_spi_nomore_transfer(struct pch_spi_data *data)
    {
    struct spi_message *pmsg, *tmp;
    dev_dbg(&data.host.dev, "%s called\n", __func__);
// Invoke complete callback
// [To the spi core..indicating end of transfer]
    data.current_msg.status = 0;
    if (data.current_msg.complete) {
    dev_dbg(&data.host.dev,
    "%s:Invoking callback of SPI core\n", __func__);
    data.current_msg.complete(data.current_msg.context);
    }
// update status in global variable
    data.bcurrent_msg_processing = false;
    dev_dbg(&data.host.dev,
    "%s:data.bcurrent_msg_processing = false\n", __func__);
    data.current_msg = core::ptr::null_mut();
    data.cur_trans = core::ptr::null_mut();
// check if we have items in list and not suspending
// return 1 if list empty
    if ((list_empty(&data.queue) == 0) &&
    (!data.board_dat.suspend_sts) &&
    (data.status != STATUS_EXITING)) {
// We have some more work to do (either there is more tranint
// bpw;sfer requests in the current message or there are
// more messages)
//
    dev_dbg(&data.host.dev, "%s:Invoke queue_work\n", __func__);
    schedule_work(&data.work);
    } else if (data.board_dat.suspend_sts ||
    data.status == STATUS_EXITING) {
    dev_dbg(&data.host.dev,
    "%s suspend/remove initiated, flushing queue\n",
    __func__);
    list_for_each_entry_safe(pmsg, tmp, data.queue.next, queue) {
    pmsg.status = -EIO;
    if (pmsg.complete)
    pmsg.complete(pmsg.context);
// delete from queue
    list_del_init(&pmsg.queue);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_set_ir(data: *mut pch_spi_data) {
    static void pch_spi_set_ir(struct pch_spi_data *data)
    {
// enable interrupts, set threshold, enable SPI
    if ((data.bpw_len) > PCH_MAX_FIFO_DEPTH)
// set receive threshold to PCH_RX_THOLD
    pch_spi_setclr_reg(data.host, PCH_SPCR,
    PCH_RX_THOLD << SPCR_RFIC_FIELD |
    SPCR_FIE_BIT | SPCR_RFIE_BIT |
    SPCR_ORIE_BIT | SPCR_SPE_BIT,
    MASK_RFIC_SPCR_BITS | PCH_ALL);
    else
// set receive threshold to maximum
    pch_spi_setclr_reg(data.host, PCH_SPCR,
    PCH_RX_THOLD_MAX << SPCR_RFIC_FIELD |
    SPCR_FIE_BIT | SPCR_ORIE_BIT |
    SPCR_SPE_BIT,
    MASK_RFIC_SPCR_BITS | PCH_ALL);
// Wait until the transfer completes; go to sleep after
    initiating the transfer. */
    dev_dbg(&data.host.dev,
    "%s:waiting for transfer to get over\n", __func__);
    wait_event_interruptible(data.wait, data.transfer_complete);
// clear all interrupts
    pch_spi_writereg(data.host, PCH_SPSR,
    pch_spi_readreg(data.host, PCH_SPSR));
// Disable interrupts and SPI transfer
    pch_spi_setclr_reg(data.host, PCH_SPCR, 0, PCH_ALL | SPCR_SPE_BIT);
// clear FIFO
    pch_spi_clear_fifo(data.host);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_copy_rx_data(data: *mut pch_spi_data, bpw: c_int) {
    static void pch_spi_copy_rx_data(struct pch_spi_data *data, int bpw)
    {
    int j;
    u8 *rx_buf;
    u16 *rx_sbuf;
// copy Rx Data
    if (!data.cur_trans.rx_buf)
    return;
    if (bpw == 8) {
    rx_buf = data.cur_trans.rx_buf;
    for (j = 0; j < data.bpw_len; j++)
// rx_buf++ = data->pkt_rx_buff[j] & 0xFF;
    } else {
    rx_sbuf = data.cur_trans.rx_buf;
    for (j = 0; j < data.bpw_len; j++)
// rx_sbuf++ = data->pkt_rx_buff[j];
    }
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_copy_rx_data_for_dma(data: *mut pch_spi_data, bpw: c_int) {
    static void pch_spi_copy_rx_data_for_dma(struct pch_spi_data *data, int bpw)
    {
    int j;
    u8 *rx_buf;
    u16 *rx_sbuf;
    const u8 *rx_dma_buf;
    const u16 *rx_dma_sbuf;
// copy Rx Data
    if (!data.cur_trans.rx_buf)
    return;
    if (bpw == 8) {
    rx_buf = data.cur_trans.rx_buf;
    rx_dma_buf = data.dma.rx_buf_virt;
    for (j = 0; j < data.bpw_len; j++)
// rx_buf++ = *rx_dma_buf++ & 0xFF;
    data.cur_trans.rx_buf = rx_buf;
    } else {
    rx_sbuf = data.cur_trans.rx_buf;
    rx_dma_sbuf = data.dma.rx_buf_virt;
    for (j = 0; j < data.bpw_len; j++)
// rx_sbuf++ = *rx_dma_sbuf++;
    data.cur_trans.rx_buf = rx_sbuf;
    }
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_start_transfer(data: *mut pch_spi_data) -> c_int {
    static int pch_spi_start_transfer(struct pch_spi_data *data)
    {
    struct pch_spi_dma_ctrl *dma;
    unsigned long flags;
    int rtn;
    dma = &data.dma;
    spin_lock_irqsave(&data.lock, flags);
// disable interrupts, SPI set enable
    pch_spi_setclr_reg(data.host, PCH_SPCR, SPCR_SPE_BIT, PCH_ALL);
    spin_unlock_irqrestore(&data.lock, flags);
// Wait until the transfer completes; go to sleep after
    initiating the transfer. */
    dev_dbg(&data.host.dev,
    "%s:waiting for transfer to get over\n", __func__);
    rtn = wait_event_interruptible_timeout(data.wait,
    data.transfer_complete,
    msecs_to_jiffies(2 * HZ));
    if (!rtn)
    dev_err(&data.host.dev,
    "%s wait-event timeout\n", __func__);
    dma_sync_sg_for_cpu(&data.host.dev, dma.sg_rx_p, dma.nent,
    DMA_FROM_DEVICE);
    dma_sync_sg_for_cpu(&data.host.dev, dma.sg_tx_p, dma.nent,
    DMA_FROM_DEVICE);
    memset(data.dma.tx_buf_virt, 0, PAGE_SIZE);
    async_tx_ack(dma.desc_rx);
    async_tx_ack(dma.desc_tx);
    kfree(dma.sg_tx_p);
    kfree(dma.sg_rx_p);
    spin_lock_irqsave(&data.lock, flags);
// clear fifo threshold, disable interrupts, disable SPI transfer
    pch_spi_setclr_reg(data.host, PCH_SPCR, 0,
    MASK_RFIC_SPCR_BITS | MASK_TFIC_SPCR_BITS | PCH_ALL |
    SPCR_SPE_BIT);
// clear all interrupts
    pch_spi_writereg(data.host, PCH_SPSR,
    pch_spi_readreg(data.host, PCH_SPSR));
// clear FIFO
    pch_spi_clear_fifo(data.host);
    spin_unlock_irqrestore(&data.lock, flags);
    return rtn;
    }
#[no_mangle]
unsafe extern "C" fn pch_dma_rx_complete(arg: *mut c_void) {
    static void pch_dma_rx_complete(void *arg)
    {
    struct pch_spi_data *data = arg;
// transfer is completed;inform pch_spi_process_messages_dma
    data.transfer_complete = true;
    wake_up_interruptible(&data.wait);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_filter(chan: *mut dma_chan, slave: *mut c_void) -> bool {
    static bool pch_spi_filter(struct dma_chan *chan, void *slave)
    {
    struct pch_dma_slave *param = slave;
    if ((chan.chan_id == param.chan_id) &&
    (param.dma_dev == chan.device.dev)) {
    chan.private = param;
    return true;
    } else {
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_request_dma(data: *mut pch_spi_data, bpw: c_int) {
    static void pch_spi_request_dma(struct pch_spi_data *data, int bpw)
    {
    dma_cap_mask_t mask;
    struct dma_chan *chan;
    struct pci_dev *dma_dev;
    struct pch_dma_slave *param;
    struct pch_spi_dma_ctrl *dma;
    unsigned int width;
    if (bpw == 8)
    width = PCH_DMA_WIDTH_1_BYTE;
    else
    width = PCH_DMA_WIDTH_2_BYTES;
    dma = &data.dma;
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
// Get DMA's dev information
    dma_dev = pci_get_slot(data.board_dat.pdev.bus,
    PCI_DEVFN(PCI_SLOT(data.board_dat.pdev.devfn), 0));
// Set Tx DMA
    param = &dma.param_tx;
    param.dma_dev = &dma_dev.dev;
    param.chan_id = data.ch * 2; /* Tx = 0, 2 */
    param.tx_reg = data.io_base_addr + PCH_SPDWR;
    param.width = width;
    chan = dma_request_channel(mask, pch_spi_filter, param);
    if (!chan) {
    dev_err(&data.host.dev,
    "ERROR: dma_request_channel FAILS(Tx)\n");
    goto out;
    }
    dma.chan_tx = chan;
// Set Rx DMA
    param = &dma.param_rx;
    param.dma_dev = &dma_dev.dev;
    param.chan_id = data.ch * 2 + 1; /* Rx = Tx + 1 */
    param.rx_reg = data.io_base_addr + PCH_SPDRR;
    param.width = width;
    chan = dma_request_channel(mask, pch_spi_filter, param);
    if (!chan) {
    dev_err(&data.host.dev,
    "ERROR: dma_request_channel FAILS(Rx)\n");
    dma_release_channel(dma.chan_tx);
    dma.chan_tx = core::ptr::null_mut();
    goto out;
    }
    dma.chan_rx = chan;
    dma.dma_dev = dma_dev;
    return;
    out:
    pci_dev_put(dma_dev);
    data.use_dma = 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_release_dma(data: *mut pch_spi_data) {
    static void pch_spi_release_dma(struct pch_spi_data *data)
    {
    struct pch_spi_dma_ctrl *dma;
    dma = &data.dma;
    if (dma.chan_tx) {
    dma_release_channel(dma.chan_tx);
    dma.chan_tx = core::ptr::null_mut();
    }
    if (dma.chan_rx) {
    dma_release_channel(dma.chan_rx);
    dma.chan_rx = core::ptr::null_mut();
    }
    pci_dev_put(dma.dma_dev);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_handle_dma(data: *mut pch_spi_data, bpw: *mut c_int) {
    static void pch_spi_handle_dma(struct pch_spi_data *data, int *bpw)
    {
    const u8 *tx_buf;
    const u16 *tx_sbuf;
    u8 *tx_dma_buf;
    u16 *tx_dma_sbuf;
    struct scatterlist *sg;
    struct dma_async_tx_descriptor *desc_tx;
    struct dma_async_tx_descriptor *desc_rx;
    int num;
    int i;
    int size;
    int rem;
    int head;
    unsigned long flags;
    struct pch_spi_dma_ctrl *dma;
    dma = &data.dma;
// set baud rate if needed
    if (data.cur_trans.speed_hz) {
    dev_dbg(&data.host.dev, "%s:setting baud rate\n", __func__);
    spin_lock_irqsave(&data.lock, flags);
    pch_spi_set_baud_rate(data.host, data.cur_trans.speed_hz);
    spin_unlock_irqrestore(&data.lock, flags);
    }
// set bits per word if needed
    if (data.cur_trans.bits_per_word &&
    (data.current_msg.spi.bits_per_word !=
    data.cur_trans.bits_per_word)) {
    dev_dbg(&data.host.dev, "%s:set bits per word\n", __func__);
    spin_lock_irqsave(&data.lock, flags);
    pch_spi_set_bits_per_word(data.host,
    data.cur_trans.bits_per_word);
    spin_unlock_irqrestore(&data.lock, flags);
// bpw = data->cur_trans->bits_per_word;
    } else {
// bpw = data->current_msg->spi->bits_per_word;
    }
    data.bpw_len = data.cur_trans.len / (*bpw / 8);
    if (data.bpw_len > PCH_BUF_SIZE) {
    data.bpw_len = PCH_BUF_SIZE;
    data.cur_trans.len -= PCH_BUF_SIZE;
    }
// copy Tx Data
    if (data.cur_trans.tx_buf != core::ptr::null_mut()) {
    if (*bpw == 8) {
    tx_buf = data.cur_trans.tx_buf;
    tx_dma_buf = dma.tx_buf_virt;
    for (i = 0; i < data.bpw_len; i++)
// tx_dma_buf++ = *tx_buf++;
    } else {
    tx_sbuf = data.cur_trans.tx_buf;
    tx_dma_sbuf = dma.tx_buf_virt;
    for (i = 0; i < data.bpw_len; i++)
// tx_dma_sbuf++ = *tx_sbuf++;
    }
    }
// Calculate Rx parameter for DMA transmitting
    if (data.bpw_len > PCH_DMA_TRANS_SIZE) {
    if (data.bpw_len % PCH_DMA_TRANS_SIZE) {
    num = data.bpw_len / PCH_DMA_TRANS_SIZE + 1;
    rem = data.bpw_len % PCH_DMA_TRANS_SIZE;
    } else {
    num = data.bpw_len / PCH_DMA_TRANS_SIZE;
    rem = PCH_DMA_TRANS_SIZE;
    }
    size = PCH_DMA_TRANS_SIZE;
    } else {
    num = 1;
    size = data.bpw_len;
    rem = data.bpw_len;
    }
    dev_dbg(&data.host.dev, "%s num=%d size=%d rem=%d\n",
    __func__, num, size, rem);
    spin_lock_irqsave(&data.lock, flags);
// set receive fifo threshold and transmit fifo threshold
    pch_spi_setclr_reg(data.host, PCH_SPCR,
    ((size - 1) << SPCR_RFIC_FIELD) |
    (PCH_TX_THOLD << SPCR_TFIC_FIELD),
    MASK_RFIC_SPCR_BITS | MASK_TFIC_SPCR_BITS);
    spin_unlock_irqrestore(&data.lock, flags);
// RX
    dma.sg_rx_p = kmalloc_objs(*dma.sg_rx_p, num, GFP_ATOMIC);
    if (!dma.sg_rx_p)
    return;
    sg_init_table(dma.sg_rx_p, num); /* Initialize SG table */
// offset, length setting
    sg = dma.sg_rx_p;
    for (i = 0; i < num; i++, sg++) {
    if (i == (num - 2)) {
    sg.offset = size * i;
    sg.offset = sg.offset * (*bpw / 8);
    sg_set_page(sg, virt_to_page(dma.rx_buf_virt), rem,
    sg.offset);
    sg_dma_len(sg) = rem;
    } else if (i == (num - 1)) {
    sg.offset = size * (i - 1) + rem;
    sg.offset = sg.offset * (*bpw / 8);
    sg_set_page(sg, virt_to_page(dma.rx_buf_virt), size,
    sg.offset);
    sg_dma_len(sg) = size;
    } else {
    sg.offset = size * i;
    sg.offset = sg.offset * (*bpw / 8);
    sg_set_page(sg, virt_to_page(dma.rx_buf_virt), size,
    sg.offset);
    sg_dma_len(sg) = size;
    }
    sg_dma_address(sg) = dma.rx_buf_dma + sg.offset;
    }
    sg = dma.sg_rx_p;
    desc_rx = dmaengine_prep_slave_sg(dma.chan_rx, sg,
    num, DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_rx) {
    dev_err(&data.host.dev,
    "%s:dmaengine_prep_slave_sg Failed\n", __func__);
    return;
    }
    dma_sync_sg_for_device(&data.host.dev, sg, num, DMA_FROM_DEVICE);
    desc_rx.callback = pch_dma_rx_complete;
    desc_rx.callback_param = data;
    dma.nent = num;
    dma.desc_rx = desc_rx;
// Calculate Tx parameter for DMA transmitting
    if (data.bpw_len > PCH_MAX_FIFO_DEPTH) {
    head = PCH_MAX_FIFO_DEPTH - PCH_DMA_TRANS_SIZE;
    if (data.bpw_len % PCH_DMA_TRANS_SIZE > 4) {
    num = data.bpw_len / PCH_DMA_TRANS_SIZE + 1;
    rem = data.bpw_len % PCH_DMA_TRANS_SIZE - head;
    } else {
    num = data.bpw_len / PCH_DMA_TRANS_SIZE;
    rem = data.bpw_len % PCH_DMA_TRANS_SIZE +
    PCH_DMA_TRANS_SIZE - head;
    }
    size = PCH_DMA_TRANS_SIZE;
    } else {
    num = 1;
    size = data.bpw_len;
    rem = data.bpw_len;
    head = 0;
    }
    dma.sg_tx_p = kmalloc_objs(*dma.sg_tx_p, num, GFP_ATOMIC);
    if (!dma.sg_tx_p)
    return;
    sg_init_table(dma.sg_tx_p, num); /* Initialize SG table */
// offset, length setting
    sg = dma.sg_tx_p;
    for (i = 0; i < num; i++, sg++) {
    if (i == 0) {
    sg.offset = 0;
    sg_set_page(sg, virt_to_page(dma.tx_buf_virt), size + head,
    sg.offset);
    sg_dma_len(sg) = size + head;
    } else if (i == (num - 1)) {
    sg.offset = head + size * i;
    sg.offset = sg.offset * (*bpw / 8);
    sg_set_page(sg, virt_to_page(dma.tx_buf_virt), rem,
    sg.offset);
    sg_dma_len(sg) = rem;
    } else {
    sg.offset = head + size * i;
    sg.offset = sg.offset * (*bpw / 8);
    sg_set_page(sg, virt_to_page(dma.tx_buf_virt), size,
    sg.offset);
    sg_dma_len(sg) = size;
    }
    sg_dma_address(sg) = dma.tx_buf_dma + sg.offset;
    }
    sg = dma.sg_tx_p;
    desc_tx = dmaengine_prep_slave_sg(dma.chan_tx,
    sg, num, DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_tx) {
    dev_err(&data.host.dev,
    "%s:dmaengine_prep_slave_sg Failed\n", __func__);
    return;
    }
    dma_sync_sg_for_device(&data.host.dev, sg, num, DMA_TO_DEVICE);
    desc_tx.callback = core::ptr::null_mut();
    desc_tx.callback_param = data;
    dma.nent = num;
    dma.desc_tx = desc_tx;
    dev_dbg(&data.host.dev, "%s:Pulling down SSN low - writing 0x2 to SSNXCR\n", __func__);
    spin_lock_irqsave(&data.lock, flags);
    pch_spi_writereg(data.host, PCH_SSNXCR, SSN_LOW);
    desc_rx.tx_submit(desc_rx);
    desc_tx.tx_submit(desc_tx);
    spin_unlock_irqrestore(&data.lock, flags);
// reset transfer complete flag
    data.transfer_complete = false;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_process_messages(pwork: *mut work_struct) {
    static void pch_spi_process_messages(struct work_struct *pwork)
    {
    struct spi_message *pmsg, *tmp;
    struct pch_spi_data *data;
    int bpw;
    data = container_of(pwork, struct pch_spi_data, work);
    dev_dbg(&data.host.dev, "%s data initialized\n", __func__);
    spin_lock(&data.lock);
// check if suspend has been initiated;if yes flush queue
    if (data.board_dat.suspend_sts || (data.status == STATUS_EXITING)) {
    dev_dbg(&data.host.dev,
    "%s suspend/remove initiated, flushing queue\n", __func__);
    list_for_each_entry_safe(pmsg, tmp, data.queue.next, queue) {
    pmsg.status = -EIO;
    if (pmsg.complete) {
    spin_unlock(&data.lock);
    pmsg.complete(pmsg.context);
    spin_lock(&data.lock);
    }
// delete from queue
    list_del_init(&pmsg.queue);
    }
    spin_unlock(&data.lock);
    return;
    }
    data.bcurrent_msg_processing = true;
    dev_dbg(&data.host.dev,
    "%s Set data.bcurrent_msg_processing= true\n", __func__);
// Get the message from the queue and delete it from there.
    data.current_msg = list_entry(data.queue.next, struct spi_message,
    queue);
    list_del_init(&data.current_msg.queue);
    data.current_msg.status = 0;
    pch_spi_select_chip(data, data.current_msg.spi);
    spin_unlock(&data.lock);
    if (data.use_dma)
    pch_spi_request_dma(data,
    data.current_msg.spi.bits_per_word);
    pch_spi_writereg(data.host, PCH_SSNXCR, SSN_NO_CONTROL);
    do {
    int cnt;
// If we are already processing a message get the next
    transfer structure from the message otherwise retrieve
    the 1st transfer request from the message. */
    spin_lock(&data.lock);
    if (data.cur_trans == core::ptr::null_mut()) {
    data.cur_trans =
    list_entry(data.current_msg.transfers.next,
    struct spi_transfer, transfer_list);
    dev_dbg(&data.host.dev,
    "%s :Getting 1st transfer message\n",
    __func__);
    } else {
    data.cur_trans =
    list_entry(data.cur_trans.transfer_list.next,
    struct spi_transfer, transfer_list);
    dev_dbg(&data.host.dev,
    "%s :Getting next transfer message\n",
    __func__);
    }
    spin_unlock(&data.lock);
    if (!data.cur_trans.len)
    goto out;
    cnt = (data.cur_trans.len - 1) / PCH_BUF_SIZE + 1;
    data.save_total_len = data.cur_trans.len;
    if (data.use_dma) {
    int i;
    char *save_rx_buf = data.cur_trans.rx_buf;
    for (i = 0; i < cnt; i++) {
    pch_spi_handle_dma(data, &bpw);
    if (!pch_spi_start_transfer(data)) {
    data.transfer_complete = true;
    data.current_msg.status = -EIO;
    data.current_msg.complete
    (data.current_msg.context);
    data.bcurrent_msg_processing = false;
    data.current_msg = core::ptr::null_mut();
    data.cur_trans = core::ptr::null_mut();
    goto out;
    }
    pch_spi_copy_rx_data_for_dma(data, bpw);
    }
    data.cur_trans.rx_buf = save_rx_buf;
    } else {
    pch_spi_set_tx(data, &bpw);
    pch_spi_set_ir(data);
    pch_spi_copy_rx_data(data, bpw);
    kfree(data.pkt_rx_buff);
    data.pkt_rx_buff = core::ptr::null_mut();
    kfree(data.pkt_tx_buff);
    data.pkt_tx_buff = core::ptr::null_mut();
    }
// increment message count
    data.cur_trans.len = data.save_total_len;
    data.current_msg.actual_length += data.cur_trans.len;
    dev_dbg(&data.host.dev,
    "%s:data.current_msg.actual_length=%d\n",
    __func__, data.current_msg.actual_length);
    spi_transfer_delay_exec(data.cur_trans);
    spin_lock(&data.lock);
// No more transfer in this message.
    if ((data.cur_trans.transfer_list.next) ==
    &(data.current_msg.transfers)) {
    pch_spi_nomore_transfer(data);
    }
    spin_unlock(&data.lock);
    } while (data.cur_trans != core::ptr::null_mut());
    out:
    pch_spi_writereg(data.host, PCH_SSNXCR, SSN_HIGH);
    if (data.use_dma)
    pch_spi_release_dma(data);
    }
    static void pch_spi_free_resources(struct pch_spi_board_data *board_dat,
    struct pch_spi_data *data)
    {
    dev_dbg(&board_dat.pdev.dev, "%s ENTRY\n", __func__);
    flush_work(&data.work);
    }
    static int pch_spi_get_resources(struct pch_spi_board_data *board_dat,
    struct pch_spi_data *data)
    {
    dev_dbg(&board_dat.pdev.dev, "%s ENTRY\n", __func__);
// reset PCH SPI h/w
    pch_spi_reset(data.host);
    dev_dbg(&board_dat.pdev.dev,
    "%s pch_spi_reset invoked successfully\n", __func__);
    dev_dbg(&board_dat.pdev.dev, "%s data.irq_reg_sts=true\n", __func__);
    return 0;
    }
    static void pch_free_dma_buf(struct pch_spi_board_data *board_dat,
    struct pch_spi_data *data)
    {
    struct pch_spi_dma_ctrl *dma;
    dma = &data.dma;
    if (dma.tx_buf_dma)
    dma_free_coherent(&board_dat.pdev.dev, PCH_BUF_SIZE,
    dma.tx_buf_virt, dma.tx_buf_dma);
    if (dma.rx_buf_dma)
    dma_free_coherent(&board_dat.pdev.dev, PCH_BUF_SIZE,
    dma.rx_buf_virt, dma.rx_buf_dma);
    }
    static int pch_alloc_dma_buf(struct pch_spi_board_data *board_dat,
    struct pch_spi_data *data)
    {
    struct pch_spi_dma_ctrl *dma;
    int ret;
    dma = &data.dma;
    ret = 0;
// Get Consistent memory for Tx DMA
    dma.tx_buf_virt = dma_alloc_coherent(&board_dat.pdev.dev,
    PCH_BUF_SIZE, &dma.tx_buf_dma, GFP_KERNEL);
    if (!dma.tx_buf_virt)
    ret = -ENOMEM;
// Get Consistent memory for Rx DMA
    dma.rx_buf_virt = dma_alloc_coherent(&board_dat.pdev.dev,
    PCH_BUF_SIZE, &dma.rx_buf_dma, GFP_KERNEL);
    if (!dma.rx_buf_virt)
    ret = -ENOMEM;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_pd_probe(plat_dev: *mut platform_device) -> c_int {
    static int pch_spi_pd_probe(struct platform_device *plat_dev)
    {
    int ret;
    struct spi_controller *host;
    struct pch_spi_board_data *board_dat = dev_get_platdata(&plat_dev.dev);
    struct pch_spi_data *data;
    dev_dbg(&plat_dev.dev, "%s:debug\n", __func__);
    host = spi_alloc_host(&board_dat.pdev.dev,
    sizeof(struct pch_spi_data));
    if (!host) {
    dev_err(&plat_dev.dev, "spi_alloc_host[%d] failed.\n",
    plat_dev.id);
    return -ENOMEM;
    }
    data = spi_controller_get_devdata(host);
    data.host = host;
    platform_set_drvdata(plat_dev, data);
// baseaddress + address offset)
    data.io_base_addr = pci_resource_start(board_dat.pdev, 1) +
    PCH_ADDRESS_SIZE * plat_dev.id;
    data.io_remap_addr = pci_iomap(board_dat.pdev, 1, 0);
    if (!data.io_remap_addr) {
    dev_err(&plat_dev.dev, "%s pci_iomap failed\n", __func__);
    ret = -ENOMEM;
    goto err_pci_iomap;
    }
    data.io_remap_addr += PCH_ADDRESS_SIZE * plat_dev.id;
    dev_dbg(&plat_dev.dev, "[ch%d] remap_addr=%p\n",
    plat_dev.id, data.io_remap_addr);
// initialize members of SPI host
    host.num_chipselect = PCH_MAX_CS;
    host.transfer = pch_spi_transfer;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_LSB_FIRST;
    host.bits_per_word_mask = SPI_BPW_MASK(8) | SPI_BPW_MASK(16);
    host.max_speed_hz = PCH_MAX_BAUDRATE;
    host.flags = SPI_CONTROLLER_MUST_RX | SPI_CONTROLLER_MUST_TX;
    data.board_dat = board_dat;
    data.plat_dev = plat_dev;
    data.n_curnt_chip = 255;
    data.status = STATUS_RUNNING;
    data.ch = plat_dev.id;
    data.use_dma = use_dma;
    INIT_LIST_HEAD(&data.queue);
    spin_lock_init(&data.lock);
    INIT_WORK(&data.work, pch_spi_process_messages);
    init_waitqueue_head(&data.wait);
    ret = pch_spi_get_resources(board_dat, data);
    if (ret) {
    dev_err(&plat_dev.dev, "%s fail(retval=%d)\n", __func__, ret);
    goto err_spi_get_resources;
    }
    ret = request_irq(board_dat.pdev.irq, pch_spi_handler,
    IRQF_SHARED, KBUILD_MODNAME, data);
    if (ret) {
    dev_err(&plat_dev.dev,
    "%s request_irq failed\n", __func__);
    goto err_request_irq;
    }
    data.irq_reg_sts = true;
    pch_spi_set_host_mode(host);
    if (use_dma) {
    dev_info(&plat_dev.dev, "Use DMA for data transfers\n");
    ret = pch_alloc_dma_buf(board_dat, data);
    if (ret)
    goto err_spi_register_controller;
    }
    ret = spi_register_controller(host);
    if (ret != 0) {
    dev_err(&plat_dev.dev,
    "%s spi_register_controller FAILED\n", __func__);
    goto err_spi_register_controller;
    }
    return 0;
    err_spi_register_controller:
    pch_free_dma_buf(board_dat, data);
    free_irq(board_dat.pdev.irq, data);
    err_request_irq:
    pch_spi_free_resources(board_dat, data);
    err_spi_get_resources:
    pci_iounmap(board_dat.pdev, data.io_remap_addr);
    err_pci_iomap:
    spi_controller_put(host);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_pd_remove(plat_dev: *mut platform_device) {
    static void pch_spi_pd_remove(struct platform_device *plat_dev)
    {
    struct pch_spi_board_data *board_dat = dev_get_platdata(&plat_dev.dev);
    struct pch_spi_data *data = platform_get_drvdata(plat_dev);
    int count;
    unsigned long flags;
    dev_dbg(&plat_dev.dev, "%s:[ch%d] irq=%d\n",
    __func__, plat_dev.id, board_dat.pdev.irq);
    spi_unregister_controller(data.host);
// check for any pending messages; no action is taken if the queue
// is still full; but at least we tried.  Unload anyway
    count = 500;
    spin_lock_irqsave(&data.lock, flags);
    data.status = STATUS_EXITING;
    while ((list_empty(&data.queue) == 0) && --count) {
    dev_dbg(&board_dat.pdev.dev, "%s :queue not empty\n",
    __func__);
    spin_unlock_irqrestore(&data.lock, flags);
    msleep(PCH_SLEEP_TIME);
    spin_lock_irqsave(&data.lock, flags);
    }
    spin_unlock_irqrestore(&data.lock, flags);
    pch_spi_free_resources(board_dat, data);
// disable interrupts & free IRQ
    if (data.irq_reg_sts) {
// disable interrupts
    pch_spi_setclr_reg(data.host, PCH_SPCR, 0, PCH_ALL);
    data.irq_reg_sts = false;
    free_irq(board_dat.pdev.irq, data);
    }
    if (use_dma)
    pch_free_dma_buf(board_dat, data);
    pci_iounmap(board_dat.pdev, data.io_remap_addr);
    spi_controller_put(data.host);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_pd_suspend(dev: *mut device) -> c_int {
    static int pch_spi_pd_suspend(struct device *dev)
    {
    u8 count;
    struct pch_spi_board_data *board_dat = dev_get_platdata(dev);
    struct pch_spi_data *data = dev_get_drvdata(dev);
    dev_dbg(dev, "%s ENTRY\n", __func__);
    if (!board_dat) {
    dev_err(dev, "%s pci_get_drvdata returned core::ptr::null_mut()\n", __func__);
    return -EFAULT;
    }
// check if the current message is processed:
    Only after thats done the transfer will be suspended */
    count = 255;
    while ((--count) > 0) {
    if (!(data.bcurrent_msg_processing))
    break;
    msleep(PCH_SLEEP_TIME);
    }
// Free IRQ
    if (data.irq_reg_sts) {
// disable all interrupts
    pch_spi_setclr_reg(data.host, PCH_SPCR, 0, PCH_ALL);
    pch_spi_reset(data.host);
    free_irq(board_dat.pdev.irq, data);
    data.irq_reg_sts = false;
    dev_dbg(dev, "%s free_irq invoked successfully.\n", __func__);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_pd_resume(dev: *mut device) -> c_int {
    static int pch_spi_pd_resume(struct device *dev)
    {
    struct pch_spi_board_data *board_dat = dev_get_platdata(dev);
    struct pch_spi_data *data = dev_get_drvdata(dev);
    int retval;
    if (!board_dat) {
    dev_err(dev, "%s pci_get_drvdata returned core::ptr::null_mut()\n", __func__);
    return -EFAULT;
    }
    if (!data.irq_reg_sts) {
// register IRQ
    retval = request_irq(board_dat.pdev.irq, pch_spi_handler,
    IRQF_SHARED, KBUILD_MODNAME, data);
    if (retval < 0) {
    dev_err(dev, "%s request_irq failed\n", __func__);
    return retval;
    }
// reset PCH SPI h/w
    pch_spi_reset(data.host);
    pch_spi_set_host_mode(data.host);
    data.irq_reg_sts = true;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pch_spi_pd_pm_ops,
    pch_spi_pd_suspend, pch_spi_pd_resume);
    static struct platform_driver pch_spi_pd_driver = {
    .driver = {
    .name = "pch-spi",
    .pm = pm_sleep_ptr(&pch_spi_pd_pm_ops),
    },
    .probe = pch_spi_pd_probe,
    .remove = pch_spi_pd_remove,
    };
#[no_mangle]
unsafe extern "C" fn pch_spi_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int pch_spi_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct pch_spi_board_data *board_dat;
    struct platform_device *pd_dev = core::ptr::null_mut();
    int retval;
    int i;
    struct pch_pd_dev_save *pd_dev_save;
    pd_dev_save = kzalloc_obj(*pd_dev_save);
    if (!pd_dev_save)
    return -ENOMEM;
    board_dat = kzalloc_obj(*board_dat);
    if (!board_dat) {
    retval = -ENOMEM;
    goto err_no_mem;
    }
    retval = pci_request_regions(pdev, KBUILD_MODNAME);
    if (retval) {
    dev_err(&pdev.dev, "%s request_region failed\n", __func__);
    goto pci_request_regions;
    }
    board_dat.pdev = pdev;
    board_dat.num = id.driver_data;
    pd_dev_save.num = id.driver_data;
    pd_dev_save.board_dat = board_dat;
    retval = pci_enable_device(pdev);
    if (retval) {
    dev_err(&pdev.dev, "%s pci_enable_device failed\n", __func__);
    goto pci_enable_device;
    }
    for (i = 0; i < board_dat.num; i++) {
    pd_dev = platform_device_alloc("pch-spi", i);
    if (!pd_dev) {
    dev_err(&pdev.dev, "platform_device_alloc failed\n");
    retval = -ENOMEM;
    goto err_platform_device;
    }
    pd_dev_save.pd_save[i] = pd_dev;
    pd_dev.dev.parent = &pdev.dev;
    retval = platform_device_add_data(pd_dev, board_dat,
    sizeof(*board_dat));
    if (retval) {
    dev_err(&pdev.dev,
    "platform_device_add_data failed\n");
    platform_device_put(pd_dev);
    goto err_platform_device;
    }
    retval = platform_device_add(pd_dev);
    if (retval) {
    dev_err(&pdev.dev, "platform_device_add failed\n");
    platform_device_put(pd_dev);
    goto err_platform_device;
    }
    }
    pci_set_drvdata(pdev, pd_dev_save);
    return 0;
    err_platform_device:
    while (--i >= 0)
    platform_device_unregister(pd_dev_save.pd_save[i]);
    pci_disable_device(pdev);
    pci_enable_device:
    pci_release_regions(pdev);
    pci_request_regions:
    kfree(board_dat);
    err_no_mem:
    kfree(pd_dev_save);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_remove(pdev: *mut pci_dev) {
    static void pch_spi_remove(struct pci_dev *pdev)
    {
    int i;
    struct pch_pd_dev_save *pd_dev_save = pci_get_drvdata(pdev);
    dev_dbg(&pdev.dev, "%s ENTRY:pdev=%p\n", __func__, pdev);
    for (i = 0; i < pd_dev_save.num; i++)
    platform_device_unregister(pd_dev_save.pd_save[i]);
    pci_disable_device(pdev);
    pci_release_regions(pdev);
    kfree(pd_dev_save.board_dat);
    kfree(pd_dev_save);
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_suspend(dev: *mut device) -> c_int {
    static int pch_spi_suspend(struct device *dev)
    {
    struct pch_pd_dev_save *pd_dev_save = dev_get_drvdata(dev);
    dev_dbg(dev, "%s ENTRY\n", __func__);
    pd_dev_save.board_dat.suspend_sts = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_spi_resume(dev: *mut device) -> c_int {
    static int pch_spi_resume(struct device *dev)
    {
    struct pch_pd_dev_save *pd_dev_save = dev_get_drvdata(dev);
    dev_dbg(dev, "%s ENTRY\n", __func__);
// set suspend status to false
    pd_dev_save.board_dat.suspend_sts = false;
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pch_spi_pm_ops, pch_spi_suspend, pch_spi_resume);
    static struct pci_driver pch_spi_pcidev_driver = {
    .name = "pch_spi",
    .id_table = pch_spi_pcidev_id,
    .probe = pch_spi_probe,
    .remove = pch_spi_remove,
    .driver.pm = pm_sleep_ptr(&pch_spi_pm_ops),
    };
#[no_mangle]
unsafe extern "C" fn pch_spi_init() -> int __init {
    static int __init pch_spi_init(void)
    {
    int ret;
    ret = platform_driver_register(&pch_spi_pd_driver);
    if (ret)
    return ret;
    ret = pci_register_driver(&pch_spi_pcidev_driver);
    if (ret) {
    platform_driver_unregister(&pch_spi_pd_driver);
    return ret;
    }
    return 0;
    }
    module_init(pch_spi_init);
#[no_mangle]
unsafe extern "C" fn pch_spi_exit() -> void __exit {
    static void __exit pch_spi_exit(void)
    {
    pci_unregister_driver(&pch_spi_pcidev_driver);
    platform_driver_unregister(&pch_spi_pd_driver);
    }
    module_exit(pch_spi_exit);
    module_param(use_dma, int, 0644);
    MODULE_PARM_DESC(use_dma,
    "to use DMA for data transfers pass 1 else 0; default 1");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Intel EG20T PCH/LAPIS Semiconductor ML7xxx IOH SPI Driver");
    MODULE_DEVICE_TABLE(pci, pch_spi_pcidev_id);
