//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-mpc512x-psc.c
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
// MPC512x PSC in SPI mode driver.
//
// Copyright (C) 2007,2008 Freescale Semiconductor Inc.
// Original port from 52xx driver:
// Hongjun Chen <hong-jun.chen@freescale.com>
//
// Fork of mpc52xx_psc_spi.c:
// Copyright (C) 2006 TOPTICA Photonics AG., Dragos Carp
//

    enum {
    TYPE_MPC5121,
    TYPE_MPC5125,
    };
//
// This macro abstracts the differences in the PSC register layout between
// MPC5121 (which uses a struct mpc52xx_psc) and MPC5125 (using mpc5125_psc).
//

    void *__ret = core::ptr::null_mut();						\
    switch (mps.type) {						\
    case TYPE_MPC5121: {						\
    struct mpc52xx_psc __iomem *psc = mps.psc;	\
    __ret = &psc.regname;				\
    };							\
    break;							\
    case TYPE_MPC5125: {						\
    struct mpc5125_psc __iomem *psc = mps.psc;	\
    __ret = &psc.regname;				\
    };							\
    break;							\
    }								\
    __ret; })
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_psc_spi {
// driver internal data
    pub type: c_int,
    pub psc: *mut void __iomem,
    pub fifo: *mut mpc512x_psc_fifo __iomem,
    pub irq: c_int,
    pub bits_per_word: u8,
    pub mclk_rate: u32,
    pub txisrdone: completion,
}

// controller state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_psc_spi_cs {
    pub bits_per_word: c_int,
    pub speed_hz: c_int,
}

// set clock freq, clock ramp, bits per work
// if t is NULL then reset the values to the default values
//
    static int mpc512x_psc_spi_transfer_setup(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct mpc512x_psc_spi_cs *cs = spi.controller_state;
    cs.speed_hz = (t && t.speed_hz)
    ? t.speed_hz : spi.max_speed_hz;
    cs.bits_per_word = (t && t.bits_per_word)
    ? t.bits_per_word : spi.bits_per_word;
    cs.bits_per_word = ((cs.bits_per_word + 7) / 8) * 8;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_activate_cs(spi: *mut spi_device) {
    static void mpc512x_psc_spi_activate_cs(struct spi_device *spi)
    {
    struct mpc512x_psc_spi_cs *cs = spi.controller_state;
    struct mpc512x_psc_spi *mps = spi_controller_get_devdata(spi.controller);
    u32 sicr;
    u32 ccr;
    int speed;
    u16 bclkdiv;
    sicr = in_be32(psc_addr(mps, sicr));
// Set clock phase and polarity
    if (spi.mode & SPI_CPHA)
    sicr |= 0x00001000;
    else
    sicr &= ~0x00001000;
    if (spi.mode & SPI_CPOL)
    sicr |= 0x00002000;
    else
    sicr &= ~0x00002000;
    if (spi.mode & SPI_LSB_FIRST)
    sicr |= 0x10000000;
    else
    sicr &= ~0x10000000;
    out_be32(psc_addr(mps, sicr), sicr);
    ccr = in_be32(psc_addr(mps, ccr));
    ccr &= 0xFF000000;
    speed = cs.speed_hz;
    if (!speed)
    speed = 1000000;	/* default 1MHz */
    bclkdiv = (mps.mclk_rate / speed) - 1;
    ccr |= (((bclkdiv & 0xff) << 16) | (((bclkdiv >> 8) & 0xff) << 8));
    out_be32(psc_addr(mps, ccr), ccr);
    mps.bits_per_word = cs.bits_per_word;
    if (spi_get_csgpiod(spi, 0)) {
// gpiolib will deal with the inversion
    gpiod_set_value(spi_get_csgpiod(spi, 0), 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_deactivate_cs(spi: *mut spi_device) {
    static void mpc512x_psc_spi_deactivate_cs(struct spi_device *spi)
    {
    if (spi_get_csgpiod(spi, 0)) {
// gpiolib will deal with the inversion
    gpiod_set_value(spi_get_csgpiod(spi, 0), 0);
    }
    }
// extract and scale size field in txsz or rxsz

pub const EOFBYTE: c_int = 1;
    static int mpc512x_psc_spi_transfer_rxtx(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct mpc512x_psc_spi *mps = spi_controller_get_devdata(spi.controller);
    struct mpc512x_psc_fifo __iomem *fifo = mps.fifo;
    let mut tx_len: usize = t.len;
    let mut rx_len: usize = t.len;
    u8 *tx_buf = (u8 *)t.tx_buf;
    u8 *rx_buf = (u8 *)t.rx_buf;
    if (!tx_buf && !rx_buf && t.len)
    return -EINVAL;
    while (rx_len || tx_len) {
    size_t txcount;
    u8 data;
    size_t fifosz;
    size_t rxcount;
    int rxtries;
//
// send the TX bytes in as large a chunk as possible
// but neither exceed the TX nor the RX FIFOs
//
    fifosz = MPC512x_PSC_FIFO_SZ(in_be32(&fifo.txsz));
    txcount = min(fifosz, tx_len);
    fifosz = MPC512x_PSC_FIFO_SZ(in_be32(&fifo.rxsz));
    fifosz -= in_be32(&fifo.rxcnt) + 1;
    txcount = min(fifosz, txcount);
    if (txcount) {
// fill the TX FIFO
    while (txcount-- > 0) {
    data = tx_buf ? *tx_buf++ : 0;
    if (tx_len == EOFBYTE && t.cs_change)
    setbits32(&fifo.txcmd,
    MPC512x_PSC_FIFO_EOF);
    out_8(&fifo.txdata_8, data);
    tx_len--;
    }
// have the ISR trigger when the TX FIFO is empty
    reinit_completion(&mps.txisrdone);
    out_be32(&fifo.txisr, MPC512x_PSC_FIFO_EMPTY);
    out_be32(&fifo.tximr, MPC512x_PSC_FIFO_EMPTY);
    wait_for_completion(&mps.txisrdone);
    }
//
// consume as much RX data as the FIFO holds, while we
// iterate over the transfer's TX data length
//
// only insist in draining all the remaining RX bytes
// when the TX bytes were exhausted (that's at the very
// end of this transfer, not when still iterating over
// the transfer's chunks)
//
    rxtries = 50;
    do {
//
// grab whatever was in the FIFO when we started
// looking, don't bother fetching what was added to
// the FIFO while we read from it -- we'll return
// here eventually and prefer sending out remaining
// TX data
//
    fifosz = in_be32(&fifo.rxcnt);
    rxcount = min(fifosz, rx_len);
    while (rxcount-- > 0) {
    data = in_8(&fifo.rxdata_8);
    if (rx_buf)
// rx_buf++ = data;
    rx_len--;
    }
//
// come back later if there still is TX data to send,
// bail out of the RX drain loop if all of the TX data
// was sent and all of the RX data was received (i.e.
// when the transmission has completed)
//
    if (tx_len)
    break;
    if (!rx_len)
    break;
//
// TX data transmission has completed while RX data
// is still pending -- that's a transient situation
// which depends on wire speed and specific
// hardware implementation details (buffering) yet
// should resolve very quickly
//
// just yield for a moment to not hog the CPU for
// too long when running SPI at low speed
//
// the timeout range is rather arbitrary and tries
// to balance throughput against system load; the
// chosen values result in a minimal timeout of 50
// times 10us and thus work at speeds as low as
// some 20kbps, while the maximum timeout at the
// transfer's end could be 5ms _if_ nothing else
// ticks in the system _and_ RX data still wasn't
// received, which only occurs in situations that
// are exceptional; removing the unpredictability
// of the timeout either decreases throughput
// (longer timeouts), or puts more load on the
// system (fixed short timeouts) or requires the
// use of a timeout API instead of a counter and an
// unknown inner delay
//
    usleep_range(10, 100);
    } while (--rxtries > 0);
    if (!tx_len && rx_len && !rxtries) {
//
// not enough RX bytes even after several retries
// and the resulting rather long timeout?
//
    rxcount = in_be32(&fifo.rxcnt);
    dev_warn(&spi.dev,
    "short xfer, missing %zd RX bytes, FIFO level %zd\n",
    rx_len, rxcount);
    }
//
// drain and drop RX data which "should not be there" in
// the first place, for undisturbed transmission this turns
// into a NOP (except for the FIFO level fetch)
//
    if (!tx_len && !rx_len) {
    while (in_be32(&fifo.rxcnt))
    in_8(&fifo.rxdata_8);
    }
    }
    return 0;
    }
    static int mpc512x_psc_spi_msg_xfer(struct spi_controller *host,
    struct spi_message *m)
    {
    struct spi_device *spi;
    unsigned cs_change;
    int status;
    struct spi_transfer *t;
    spi = m.spi;
    cs_change = 1;
    status = 0;
    list_for_each_entry(t, &m.transfers, transfer_list) {
    status = mpc512x_psc_spi_transfer_setup(spi, t);
    if (status < 0)
    break;
    if (cs_change)
    mpc512x_psc_spi_activate_cs(spi);
    cs_change = t.cs_change;
    status = mpc512x_psc_spi_transfer_rxtx(spi, t);
    if (status)
    break;
    m.actual_length += t.len;
    spi_transfer_delay_exec(t);
    if (cs_change)
    mpc512x_psc_spi_deactivate_cs(spi);
    }
    m.status = status;
    if (m.complete)
    m.complete(m.context);
    if (status || !cs_change)
    mpc512x_psc_spi_deactivate_cs(spi);
    mpc512x_psc_spi_transfer_setup(spi, core::ptr::null_mut());
    spi_finalize_current_message(host);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_prep_xfer_hw(host: *mut spi_controller) -> c_int {
    static int mpc512x_psc_spi_prep_xfer_hw(struct spi_controller *host)
    {
    struct mpc512x_psc_spi *mps = spi_controller_get_devdata(host);
    dev_dbg(&host.dev, "%s()\n", __func__);
// Zero MR2
    in_8(psc_addr(mps, mr2));
    out_8(psc_addr(mps, mr2), 0x0);
// enable transmitter/receiver
    out_8(psc_addr(mps, command), MPC52xx_PSC_TX_ENABLE | MPC52xx_PSC_RX_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_unprep_xfer_hw(host: *mut spi_controller) -> c_int {
    static int mpc512x_psc_spi_unprep_xfer_hw(struct spi_controller *host)
    {
    struct mpc512x_psc_spi *mps = spi_controller_get_devdata(host);
    struct mpc512x_psc_fifo __iomem *fifo = mps.fifo;
    dev_dbg(&host.dev, "%s()\n", __func__);
// disable transmitter/receiver and fifo interrupt
    out_8(psc_addr(mps, command), MPC52xx_PSC_TX_DISABLE | MPC52xx_PSC_RX_DISABLE);
    out_be32(&fifo.tximr, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_setup(spi: *mut spi_device) -> c_int {
    static int mpc512x_psc_spi_setup(struct spi_device *spi)
    {
    struct mpc512x_psc_spi_cs *cs = spi.controller_state;
    if (spi.bits_per_word % 8)
    return -EINVAL;
    if (!cs) {
    cs = kzalloc_obj(*cs);
    if (!cs)
    return -ENOMEM;
    spi.controller_state = cs;
    }
    cs.bits_per_word = spi.bits_per_word;
    cs.speed_hz = spi.max_speed_hz;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_cleanup(spi: *mut spi_device) {
    static void mpc512x_psc_spi_cleanup(struct spi_device *spi)
    {
    kfree(spi.controller_state);
    }
    static int mpc512x_psc_spi_port_config(struct spi_controller *host,
    struct mpc512x_psc_spi *mps)
    {
    struct mpc512x_psc_fifo __iomem *fifo = mps.fifo;
    u32 sicr;
    u32 ccr;
    int speed;
    u16 bclkdiv;
// Reset the PSC into a known state
    out_8(psc_addr(mps, command), MPC52xx_PSC_RST_RX);
    out_8(psc_addr(mps, command), MPC52xx_PSC_RST_TX);
    out_8(psc_addr(mps, command), MPC52xx_PSC_TX_DISABLE | MPC52xx_PSC_RX_DISABLE);
// Disable psc interrupts all useful interrupts are in fifo
    out_be16(psc_addr(mps, isr_imr.imr), 0);
// Disable fifo interrupts, will be enabled later
    out_be32(&fifo.tximr, 0);
    out_be32(&fifo.rximr, 0);
// Setup fifo slice address and size
// out_be32(&fifo->txsz, 0x0fe00004);
// out_be32(&fifo->rxsz, 0x0ff00004);
    sicr =	0x01000000 |	/* SIM = 0001 -- 8 bit */
    0x00800000 |	/* GenClk = 1 -- internal clk */
    0x00008000 |	/* SPI = 1 */
    0x00004000 |	/* MSTR = 1   -- SPI host */
    0x00000800;	/* UseEOF = 1 -- SS low until EOF */
    out_be32(psc_addr(mps, sicr), sicr);
    ccr = in_be32(psc_addr(mps, ccr));
    ccr &= 0xFF000000;
    speed = 1000000;	/* default 1MHz */
    bclkdiv = (mps.mclk_rate / speed) - 1;
    ccr |= (((bclkdiv & 0xff) << 16) | (((bclkdiv >> 8) & 0xff) << 8));
    out_be32(psc_addr(mps, ccr), ccr);
// Set 2ms DTL delay
    out_8(psc_addr(mps, ctur), 0x00);
    out_8(psc_addr(mps, ctlr), 0x82);
// we don't use the alarms
    out_be32(&fifo.rxalarm, 0xfff);
    out_be32(&fifo.txalarm, 0);
// Enable FIFO slices for Rx/Tx
    out_be32(&fifo.rxcmd,
    MPC512x_PSC_FIFO_ENABLE_SLICE | MPC512x_PSC_FIFO_ENABLE_DMA);
    out_be32(&fifo.txcmd,
    MPC512x_PSC_FIFO_ENABLE_SLICE | MPC512x_PSC_FIFO_ENABLE_DMA);
    mps.bits_per_word = 8;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mpc512x_psc_spi_isr(int irq, void *dev_id)
    {
    struct mpc512x_psc_spi *mps = (struct mpc512x_psc_spi *)dev_id;
    struct mpc512x_psc_fifo __iomem *fifo = mps.fifo;
// clear interrupt and wake up the rx/tx routine
    if (in_be32(&fifo.txisr) &
    in_be32(&fifo.tximr) & MPC512x_PSC_FIFO_EMPTY) {
    out_be32(&fifo.txisr, MPC512x_PSC_FIFO_EMPTY);
    out_be32(&fifo.tximr, 0);
    complete(&mps.txisrdone);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn mpc512x_psc_spi_of_probe(pdev: *mut platform_device) -> c_int {
    static int mpc512x_psc_spi_of_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mpc512x_psc_spi *mps;
    struct spi_controller *host;
    int ret;
    void *tempp;
    struct clk *clk;
    host = devm_spi_alloc_host(dev, sizeof(*mps));
    if (host == core::ptr::null_mut())
    return -ENOMEM;
    dev_set_drvdata(dev, host);
    mps = spi_controller_get_devdata(host);
    mps.type = (int)device_get_match_data(dev);
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH | SPI_LSB_FIRST;
    host.setup = mpc512x_psc_spi_setup;
    host.prepare_transfer_hardware = mpc512x_psc_spi_prep_xfer_hw;
    host.transfer_one_message = mpc512x_psc_spi_msg_xfer;
    host.unprepare_transfer_hardware = mpc512x_psc_spi_unprep_xfer_hw;
    host.use_gpio_descriptors = true;
    host.cleanup = mpc512x_psc_spi_cleanup;
    tempp = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(tempp))
    return dev_err_probe(dev, PTR_ERR(tempp), "could not ioremap I/O port range\n");
    mps.psc = tempp;
    mps.fifo =
    (struct mpc512x_psc_fifo *)(tempp + sizeof(struct mpc52xx_psc));
    mps.irq = platform_get_irq(pdev, 0);
    if (mps.irq < 0)
    return mps.irq;
    ret = devm_request_irq(dev, mps.irq, mpc512x_psc_spi_isr, IRQF_SHARED,
    "mpc512x-psc-spi", mps);
    if (ret)
    return ret;
    init_completion(&mps.txisrdone);
    clk = devm_clk_get_enabled(dev, "mclk");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    mps.mclk_rate = clk_get_rate(clk);
    clk = devm_clk_get_enabled(dev, "ipg");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    ret = mpc512x_psc_spi_port_config(host, mps);
    if (ret < 0)
    return ret;
    return devm_spi_register_controller(dev, host);
    }
    static const struct of_device_id mpc512x_psc_spi_of_match[] = {
    { .compatible = "fsl,mpc5121-psc-spi", .data = (void *)TYPE_MPC5121 },
    { .compatible = "fsl,mpc5125-psc-spi", .data = (void *)TYPE_MPC5125 },
    {},
    };
    MODULE_DEVICE_TABLE(of, mpc512x_psc_spi_of_match);
    static struct platform_driver mpc512x_psc_spi_of_driver = {
    .probe = mpc512x_psc_spi_of_probe,
    .driver = {
    .name = "mpc512x-psc-spi",
    .of_match_table = mpc512x_psc_spi_of_match,
    },
    };
    module_platform_driver(mpc512x_psc_spi_of_driver);
    MODULE_AUTHOR("John Rigby");
    MODULE_DESCRIPTION("MPC512x PSC SPI Driver");
    MODULE_LICENSE("GPL");
