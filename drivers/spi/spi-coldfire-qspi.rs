//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-coldfire-qspi.c
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
// Freescale/Motorola Coldfire Queued SPI driver
//
// Copyright 2010 Steven King <sfking@fdwdc.com>
//

pub const MCFQSPI_QMR: c_uint = 0x00;
pub const MCFQSPI_QMR_MSTR: c_uint = 0x8000;
pub const MCFQSPI_QMR_CPOL: c_uint = 0x0200;
pub const MCFQSPI_QMR_CPHA: c_uint = 0x0100;
pub const MCFQSPI_QDLYR: c_uint = 0x04;
pub const MCFQSPI_QDLYR_SPE: c_uint = 0x8000;
pub const MCFQSPI_QWR: c_uint = 0x08;
pub const MCFQSPI_QWR_HALT: c_uint = 0x8000;
pub const MCFQSPI_QWR_WREN: c_uint = 0x4000;
pub const MCFQSPI_QWR_CSIV: c_uint = 0x1000;
pub const MCFQSPI_QIR: c_uint = 0x0C;
pub const MCFQSPI_QIR_WCEFB: c_uint = 0x8000;
pub const MCFQSPI_QIR_ABRTB: c_uint = 0x4000;
pub const MCFQSPI_QIR_ABRTL: c_uint = 0x1000;
pub const MCFQSPI_QIR_WCEFE: c_uint = 0x0800;
pub const MCFQSPI_QIR_ABRTE: c_uint = 0x0400;
pub const MCFQSPI_QIR_SPIFE: c_uint = 0x0100;
pub const MCFQSPI_QIR_WCEF: c_uint = 0x0008;
pub const MCFQSPI_QIR_ABRT: c_uint = 0x0004;
pub const MCFQSPI_QIR_SPIF: c_uint = 0x0001;
pub const MCFQSPI_QAR: c_uint = 0x010;
pub const MCFQSPI_QAR_TXBUF: c_uint = 0x00;
pub const MCFQSPI_QAR_RXBUF: c_uint = 0x10;
pub const MCFQSPI_QAR_CMDBUF: c_uint = 0x20;
pub const MCFQSPI_QDR: c_uint = 0x014;
pub const MCFQSPI_QCR: c_uint = 0x014;
pub const MCFQSPI_QCR_CONT: c_uint = 0x8000;
pub const MCFQSPI_QCR_BITSE: c_uint = 0x4000;
pub const MCFQSPI_QCR_DT: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcfqspi {
    pub iobase: *mut void __iomem,
    pub irq: c_int,
    pub clk: *mut clk,
    pub cs_control: *mut mcfqspi_cs_control,
    pub waitq: wait_queue_head_t,
}

#[no_mangle]
unsafe extern "C" fn mcfqspi_wr_qmr(mcfqspi: *mut mcfqspi, val: u16) {
    static void mcfqspi_wr_qmr(struct mcfqspi *mcfqspi, u16 val)
    {
    writew(val, mcfqspi.iobase + MCFQSPI_QMR);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_wr_qdlyr(mcfqspi: *mut mcfqspi, val: u16) {
    static void mcfqspi_wr_qdlyr(struct mcfqspi *mcfqspi, u16 val)
    {
    writew(val, mcfqspi.iobase + MCFQSPI_QDLYR);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_rd_qdlyr(mcfqspi: *mut mcfqspi) -> u16 {
    static u16 mcfqspi_rd_qdlyr(struct mcfqspi *mcfqspi)
    {
    return readw(mcfqspi.iobase + MCFQSPI_QDLYR);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_wr_qwr(mcfqspi: *mut mcfqspi, val: u16) {
    static void mcfqspi_wr_qwr(struct mcfqspi *mcfqspi, u16 val)
    {
    writew(val, mcfqspi.iobase + MCFQSPI_QWR);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_wr_qir(mcfqspi: *mut mcfqspi, val: u16) {
    static void mcfqspi_wr_qir(struct mcfqspi *mcfqspi, u16 val)
    {
    writew(val, mcfqspi.iobase + MCFQSPI_QIR);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_wr_qar(mcfqspi: *mut mcfqspi, val: u16) {
    static void mcfqspi_wr_qar(struct mcfqspi *mcfqspi, u16 val)
    {
    writew(val, mcfqspi.iobase + MCFQSPI_QAR);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_wr_qdr(mcfqspi: *mut mcfqspi, val: u16) {
    static void mcfqspi_wr_qdr(struct mcfqspi *mcfqspi, u16 val)
    {
    writew(val, mcfqspi.iobase + MCFQSPI_QDR);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_rd_qdr(mcfqspi: *mut mcfqspi) -> u16 {
    static u16 mcfqspi_rd_qdr(struct mcfqspi *mcfqspi)
    {
    return readw(mcfqspi.iobase + MCFQSPI_QDR);
    }
    static void mcfqspi_cs_select(struct mcfqspi *mcfqspi, u8 chip_select,
    bool cs_high)
    {
    mcfqspi.cs_control.select(mcfqspi.cs_control, chip_select, cs_high);
    }
    static void mcfqspi_cs_deselect(struct mcfqspi *mcfqspi, u8 chip_select,
    bool cs_high)
    {
    mcfqspi.cs_control.deselect(mcfqspi.cs_control, chip_select, cs_high);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_cs_setup(mcfqspi: *mut mcfqspi) -> c_int {
    static int mcfqspi_cs_setup(struct mcfqspi *mcfqspi)
    {
    return (mcfqspi.cs_control.setup) ?
    mcfqspi.cs_control.setup(mcfqspi.cs_control) : 0;
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_cs_teardown(mcfqspi: *mut mcfqspi) {
    static void mcfqspi_cs_teardown(struct mcfqspi *mcfqspi)
    {
    if (mcfqspi.cs_control.teardown)
    mcfqspi.cs_control.teardown(mcfqspi.cs_control);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_qmr_baud(speed_hz: u32) -> u8 {
    static u8 mcfqspi_qmr_baud(u32 speed_hz)
    {
    return clamp((MCFQSPI_BUSCLK + speed_hz - 1) / speed_hz, 2u, 255u);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_qdlyr_spe(mcfqspi: *mut mcfqspi) -> bool {
    static bool mcfqspi_qdlyr_spe(struct mcfqspi *mcfqspi)
    {
    return mcfqspi_rd_qdlyr(mcfqspi) & MCFQSPI_QDLYR_SPE;
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_irq_handler(this_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mcfqspi_irq_handler(int this_irq, void *dev_id)
    {
    struct mcfqspi *mcfqspi = dev_id;
// clear interrupt
    mcfqspi_wr_qir(mcfqspi, MCFQSPI_QIR_SPIFE | MCFQSPI_QIR_SPIF);
    wake_up(&mcfqspi.waitq);
    return IRQ_HANDLED;
    }
    static void mcfqspi_transfer_msg8(struct mcfqspi *mcfqspi, unsigned count,
    const u8 *txbuf, u8 *rxbuf)
    {
    unsigned i, n, offset = 0;
    n = min(count, 16u);
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_CMDBUF);
    for (i = 0; i < n; ++i)
    mcfqspi_wr_qdr(mcfqspi, MCFQSPI_QCR_BITSE);
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_TXBUF);
    if (txbuf)
    for (i = 0; i < n; ++i)
    mcfqspi_wr_qdr(mcfqspi, *txbuf++);
    else
    for (i = 0; i < count; ++i)
    mcfqspi_wr_qdr(mcfqspi, 0);
    count -= n;
    if (count) {
    let mut qwr: u16 = 0xf08;
    mcfqspi_wr_qwr(mcfqspi, 0x700);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    do {
    wait_event(mcfqspi.waitq, !mcfqspi_qdlyr_spe(mcfqspi));
    mcfqspi_wr_qwr(mcfqspi, qwr);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    if (rxbuf) {
    mcfqspi_wr_qar(mcfqspi,
    MCFQSPI_QAR_RXBUF + offset);
    for (i = 0; i < 8; ++i)
// rxbuf++ = mcfqspi_rd_qdr(mcfqspi);
    }
    n = min(count, 8u);
    if (txbuf) {
    mcfqspi_wr_qar(mcfqspi,
    MCFQSPI_QAR_TXBUF + offset);
    for (i = 0; i < n; ++i)
    mcfqspi_wr_qdr(mcfqspi, *txbuf++);
    }
    qwr = (offset ? 0x808 : 0) + ((n - 1) << 8);
    offset ^= 8;
    count -= n;
    } while (count);
    wait_event(mcfqspi.waitq, !mcfqspi_qdlyr_spe(mcfqspi));
    mcfqspi_wr_qwr(mcfqspi, qwr);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    if (rxbuf) {
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_RXBUF + offset);
    for (i = 0; i < 8; ++i)
// rxbuf++ = mcfqspi_rd_qdr(mcfqspi);
    offset ^= 8;
    }
    } else {
    mcfqspi_wr_qwr(mcfqspi, (n - 1) << 8);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    }
    wait_event(mcfqspi.waitq, !mcfqspi_qdlyr_spe(mcfqspi));
    if (rxbuf) {
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_RXBUF + offset);
    for (i = 0; i < n; ++i)
// rxbuf++ = mcfqspi_rd_qdr(mcfqspi);
    }
    }
    static void mcfqspi_transfer_msg16(struct mcfqspi *mcfqspi, unsigned count,
    const u16 *txbuf, u16 *rxbuf)
    {
    unsigned i, n, offset = 0;
    n = min(count, 16u);
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_CMDBUF);
    for (i = 0; i < n; ++i)
    mcfqspi_wr_qdr(mcfqspi, MCFQSPI_QCR_BITSE);
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_TXBUF);
    if (txbuf)
    for (i = 0; i < n; ++i)
    mcfqspi_wr_qdr(mcfqspi, *txbuf++);
    else
    for (i = 0; i < count; ++i)
    mcfqspi_wr_qdr(mcfqspi, 0);
    count -= n;
    if (count) {
    let mut qwr: u16 = 0xf08;
    mcfqspi_wr_qwr(mcfqspi, 0x700);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    do {
    wait_event(mcfqspi.waitq, !mcfqspi_qdlyr_spe(mcfqspi));
    mcfqspi_wr_qwr(mcfqspi, qwr);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    if (rxbuf) {
    mcfqspi_wr_qar(mcfqspi,
    MCFQSPI_QAR_RXBUF + offset);
    for (i = 0; i < 8; ++i)
// rxbuf++ = mcfqspi_rd_qdr(mcfqspi);
    }
    n = min(count, 8u);
    if (txbuf) {
    mcfqspi_wr_qar(mcfqspi,
    MCFQSPI_QAR_TXBUF + offset);
    for (i = 0; i < n; ++i)
    mcfqspi_wr_qdr(mcfqspi, *txbuf++);
    }
    qwr = (offset ? 0x808 : 0x000) + ((n - 1) << 8);
    offset ^= 8;
    count -= n;
    } while (count);
    wait_event(mcfqspi.waitq, !mcfqspi_qdlyr_spe(mcfqspi));
    mcfqspi_wr_qwr(mcfqspi, qwr);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    if (rxbuf) {
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_RXBUF + offset);
    for (i = 0; i < 8; ++i)
// rxbuf++ = mcfqspi_rd_qdr(mcfqspi);
    offset ^= 8;
    }
    } else {
    mcfqspi_wr_qwr(mcfqspi, (n - 1) << 8);
    mcfqspi_wr_qdlyr(mcfqspi, MCFQSPI_QDLYR_SPE);
    }
    wait_event(mcfqspi.waitq, !mcfqspi_qdlyr_spe(mcfqspi));
    if (rxbuf) {
    mcfqspi_wr_qar(mcfqspi, MCFQSPI_QAR_RXBUF + offset);
    for (i = 0; i < n; ++i)
// rxbuf++ = mcfqspi_rd_qdr(mcfqspi);
    }
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_set_cs(spi: *mut spi_device, enable: bool) {
    static void mcfqspi_set_cs(struct spi_device *spi, bool enable)
    {
    struct mcfqspi *mcfqspi = spi_controller_get_devdata(spi.controller);
    let mut cs_high: bool = spi.mode & SPI_CS_HIGH;
    if (enable)
    mcfqspi_cs_select(mcfqspi, spi_get_chipselect(spi, 0), cs_high);
    else
    mcfqspi_cs_deselect(mcfqspi, spi_get_chipselect(spi, 0), cs_high);
    }
    static int mcfqspi_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct mcfqspi *mcfqspi = spi_controller_get_devdata(host);
    let mut qmr: u16 = MCFQSPI_QMR_MSTR;
    qmr |= t.bits_per_word << 10;
    if (spi.mode & SPI_CPHA)
    qmr |= MCFQSPI_QMR_CPHA;
    if (spi.mode & SPI_CPOL)
    qmr |= MCFQSPI_QMR_CPOL;
    qmr |= mcfqspi_qmr_baud(t.speed_hz);
    mcfqspi_wr_qmr(mcfqspi, qmr);
    mcfqspi_wr_qir(mcfqspi, MCFQSPI_QIR_SPIFE);
    if (t.bits_per_word == 8)
    mcfqspi_transfer_msg8(mcfqspi, t.len, t.tx_buf, t.rx_buf);
    else
    mcfqspi_transfer_msg16(mcfqspi, t.len / 2, t.tx_buf,
    t.rx_buf);
    mcfqspi_wr_qir(mcfqspi, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_setup(spi: *mut spi_device) -> c_int {
    static int mcfqspi_setup(struct spi_device *spi)
    {
    mcfqspi_cs_deselect(spi_controller_get_devdata(spi.controller),
    spi_get_chipselect(spi, 0), spi.mode & SPI_CS_HIGH);
    dev_dbg(&spi.dev,
    "bits per word %d, chip select %d, speed %d KHz\n",
    spi.bits_per_word, spi_get_chipselect(spi, 0),
    (MCFQSPI_BUSCLK / mcfqspi_qmr_baud(spi.max_speed_hz))
    / 1000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_probe(pdev: *mut platform_device) -> c_int {
    static int mcfqspi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct mcfqspi *mcfqspi;
    struct mcfqspi_platform_data *pdata;
    int status;
    pdata = dev_get_platdata(&pdev.dev);
    if (!pdata) {
    dev_dbg(&pdev.dev, "platform data is missing\n");
    return -ENOENT;
    }
    if (!pdata.cs_control) {
    dev_dbg(&pdev.dev, "pdata.cs_control is core::ptr::null_mut()\n");
    return -EINVAL;
    }
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*mcfqspi));
    if (host == core::ptr::null_mut())
    return -ENOMEM;
    mcfqspi = spi_controller_get_devdata(host);
    mcfqspi.iobase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mcfqspi.iobase))
    return PTR_ERR(mcfqspi.iobase);
    mcfqspi.irq = platform_get_irq(pdev, 0);
    if (mcfqspi.irq < 0) {
    dev_dbg(&pdev.dev, "platform_get_irq failed\n");
    return -ENXIO;
    }
    status = devm_request_irq(&pdev.dev, mcfqspi.irq, mcfqspi_irq_handler,
    0, pdev.name, mcfqspi);
    if (status) {
    dev_dbg(&pdev.dev, "request_irq failed\n");
    return status;
    }
    mcfqspi.clk = devm_clk_get_enabled(&pdev.dev, "qspi_clk");
    if (IS_ERR(mcfqspi.clk)) {
    dev_dbg(&pdev.dev, "clk_get failed\n");
    return PTR_ERR(mcfqspi.clk);
    }
    host.bus_num = pdata.bus_num;
    host.num_chipselect = pdata.num_chipselect;
    mcfqspi.cs_control = pdata.cs_control;
    status = mcfqspi_cs_setup(mcfqspi);
    if (status) {
    dev_dbg(&pdev.dev, "error initializing cs_control\n");
    return status;
    }
    init_waitqueue_head(&mcfqspi.waitq);
    host.mode_bits = SPI_CS_HIGH | SPI_CPOL | SPI_CPHA;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(8, 16);
    host.setup = mcfqspi_setup;
    host.set_cs = mcfqspi_set_cs;
    host.transfer_one = mcfqspi_transfer_one;
    host.auto_runtime_pm = true;
    platform_set_drvdata(pdev, host);
    pm_runtime_enable(&pdev.dev);
    status = spi_register_controller(host);
    if (status) {
    dev_dbg(&pdev.dev, "failed to register controller\n");
    goto fail1;
    }
    dev_info(&pdev.dev, "Coldfire QSPI bus driver\n");
    return 0;
    fail1:
    pm_runtime_disable(&pdev.dev);
    mcfqspi_cs_teardown(mcfqspi);
    dev_dbg(&pdev.dev, "Coldfire QSPI probe failed\n");
    return status;
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_remove(pdev: *mut platform_device) {
    static void mcfqspi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct mcfqspi *mcfqspi = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
    pm_runtime_disable(&pdev.dev);
// disable the hardware (set the baud rate to 0)
    mcfqspi_wr_qmr(mcfqspi, MCFQSPI_QMR_MSTR);
    mcfqspi_cs_teardown(mcfqspi);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_suspend(dev: *mut device) -> c_int {
    static int mcfqspi_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct mcfqspi *mcfqspi = spi_controller_get_devdata(host);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
    clk_disable(mcfqspi.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_resume(dev: *mut device) -> c_int {
    static int mcfqspi_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct mcfqspi *mcfqspi = spi_controller_get_devdata(host);
    clk_enable(mcfqspi.clk);
    return spi_controller_resume(host);
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_runtime_suspend(dev: *mut device) -> c_int {
    static int mcfqspi_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct mcfqspi *mcfqspi = spi_controller_get_devdata(host);
    clk_disable(mcfqspi.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcfqspi_runtime_resume(dev: *mut device) -> c_int {
    static int mcfqspi_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct mcfqspi *mcfqspi = spi_controller_get_devdata(host);
    clk_enable(mcfqspi.clk);
    return 0;
    }
    static const struct dev_pm_ops mcfqspi_pm = {
    SYSTEM_SLEEP_PM_OPS(mcfqspi_suspend, mcfqspi_resume)
    RUNTIME_PM_OPS(mcfqspi_runtime_suspend, mcfqspi_runtime_resume,
    core::ptr::null_mut())
    };
    static struct platform_driver mcfqspi_driver = {
    .driver.name	= DRIVER_NAME,
    .driver.pm	= pm_ptr(&mcfqspi_pm),
    .probe		= mcfqspi_probe,
    .remove		= mcfqspi_remove,
    };
    module_platform_driver(mcfqspi_driver);
    MODULE_AUTHOR("Steven King <sfking@fdwdc.com>");
    MODULE_DESCRIPTION("Coldfire QSPI Controller Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRIVER_NAME);
