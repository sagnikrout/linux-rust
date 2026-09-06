//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-ppc4xx.c
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
// SPI_PPC4XX SPI controller driver.
//
// Copyright (C) 2007 Gary Jennejohn <garyj@denx.de>
// Copyright 2008 Stefan Roese <sr@denx.de>, DENX Software Engineering
// Copyright 2009 Harris Corporation, Steven A. Falco <sfalco@harris.com>
//
// Based in part on drivers/spi/spi_s3c24xx.c
//
// Copyright (c) 2006 Ben Dooks
// Copyright (c) 2006 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// The PPC4xx SPI controller has no FIFO so each sent/received byte will
// generate an interrupt to the CPU. This can cause high CPU utilization.
// This driver allows platforms to reduce the interrupt load on the CPU
// during SPI transfers by setting max_speed_hz via the device tree.
//

// bits in mode register - bit 0 is MSb
//
// SPI_PPC4XX_MODE_SCP = 0 means "data latched on trailing edge of clock"
// SPI_PPC4XX_MODE_SCP = 1 means "data latched on leading edge of clock"
// Note: This is the inverse of CPHA.
//

// SPI_PPC4XX_MODE_SPE = 1 means "port enabled"

//
// SPI_PPC4XX_MODE_RD = 0 means "MSB first" - this is the normal mode
// SPI_PPC4XX_MODE_RD = 1 means "LSB first" - this is bit-reversed mode
// Note: This is identical to SPI_LSB_FIRST.
//

//
// SPI_PPC4XX_MODE_CI = 0 means "clock idles low"
// SPI_PPC4XX_MODE_CI = 1 means "clock idles high"
// Note: This is identical to CPOL.
//

//
// SPI_PPC4XX_MODE_IL = 0 means "loopback disable"
// SPI_PPC4XX_MODE_IL = 1 means "loopback enable"
//

// bits in control register
// starts a transfer when set

// bits in status register
// port is busy with a transfer

// RxD ready

// clock settings (SCP and CI) for various SPI modes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_ppc4xx_regs {
    pub mode: u8,
    pub rxd: u8,
    pub txd: u8,
    pub cr: u8,
    pub sr: u8,
    pub dummy: u8,
//
// Clock divisor modulus register
// This uses the following formula:
// SCPClkOut = OPBCLK/(4(CDM + 1))
// or
// CDM = (OPBCLK/4*SCPClkOut) - 1
// bit 0 is the MSb!
//
    pub cdm: u8,
}

// SPI Controller driver's private data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc4xx_spi {
// bitbang has to be first
    pub bitbang: spi_bitbang,
    pub done: completion,
// need this to set the SPI clock
    pub opb_freq: c_uint,
// for transfers
    pub len: c_int,
    pub count: c_int,
// data buffers
    pub tx: *const c_uchar,
    pub rx: *mut c_uchar,
    pub /: *mut *mut *mut spi_ppc4xx_regs __iomem regs; / pointer to the registers,
    pub host: *mut spi_controller,
    pub dev: *mut device,
}

// need this so we can set the clock in the chipselect routine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_ppc4xx_cs {
    pub mode: u8,
}

#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_txrx(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int spi_ppc4xx_txrx(struct spi_device *spi, struct spi_transfer *t)
    {
    struct ppc4xx_spi *hw;
    u8 data;
    dev_dbg(&spi.dev, "txrx: tx %p, rx %p, len %d\n",
    t.tx_buf, t.rx_buf, t.len);
    hw = spi_controller_get_devdata(spi.controller);
    hw.tx = t.tx_buf;
    hw.rx = t.rx_buf;
    hw.len = t.len;
    hw.count = 0;
// send the first byte
    data = hw.tx ? hw.tx[0] : 0;
    out_8(&hw.regs.txd, data);
    out_8(&hw.regs.cr, SPI_PPC4XX_CR_STR);
    wait_for_completion(&hw.done);
    return hw.count;
    }
#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_setupxfer(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int spi_ppc4xx_setupxfer(struct spi_device *spi, struct spi_transfer *t)
    {
    struct ppc4xx_spi *hw = spi_controller_get_devdata(spi.controller);
    struct spi_ppc4xx_cs *cs = spi.controller_state;
    int scr;
    let mut cdm: u8 = 0;
    u32 speed;
// Start with the generic configuration for this device.
    speed = spi.max_speed_hz;
//
// Modify the configuration if the transfer overrides it.  Do not allow
// the transfer to overwrite the generic configuration with zeros.
//
    if (t) {
    if (t.speed_hz)
    speed = min(t.speed_hz, spi.max_speed_hz);
    }
    if (!speed || (speed > spi.max_speed_hz)) {
    dev_err(&spi.dev, "invalid speed_hz (%d)\n", speed);
    return -EINVAL;
    }
// Write new configuration
    out_8(&hw.regs.mode, cs.mode);
// Set the clock
// opb_freq was already divided by 4
    scr = (hw.opb_freq / speed) - 1;
    if (scr > 0)
    cdm = min(scr, 0xff);
    dev_dbg(&spi.dev, "setting pre-scaler to %d (hz %d)\n", cdm, speed);
    if (in_8(&hw.regs.cdm) != cdm)
    out_8(&hw.regs.cdm, cdm);
    mutex_lock(&hw.bitbang.lock);
    if (!hw.bitbang.busy) {
    hw.bitbang.chipselect(spi, BITBANG_CS_INACTIVE);
// Need to ndelay here?
    }
    mutex_unlock(&hw.bitbang.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_setup(spi: *mut spi_device) -> c_int {
    static int spi_ppc4xx_setup(struct spi_device *spi)
    {
    struct spi_ppc4xx_cs *cs = spi.controller_state;
    if (!spi.max_speed_hz) {
    dev_err(&spi.dev, "invalid max_speed_hz (must be non-zero)\n");
    return -EINVAL;
    }
    if (cs == core::ptr::null_mut()) {
    cs = kzalloc_obj(*cs);
    if (!cs)
    return -ENOMEM;
    spi.controller_state = cs;
    }
//
// We set all bits of the SPI0_MODE register, so,
// no need to read-modify-write
//
    cs.mode = SPI_PPC4XX_MODE_SPE;
    switch (spi.mode & SPI_MODE_X_MASK) {
    case SPI_MODE_0:
    cs.mode |= SPI_CLK_MODE0;
    break;
    case SPI_MODE_1:
    cs.mode |= SPI_CLK_MODE1;
    break;
    case SPI_MODE_2:
    cs.mode |= SPI_CLK_MODE2;
    break;
    case SPI_MODE_3:
    cs.mode |= SPI_CLK_MODE3;
    break;
    }
    if (spi.mode & SPI_LSB_FIRST)
    cs.mode |= SPI_PPC4XX_MODE_RD;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t spi_ppc4xx_int(int irq, void *dev_id)
    {
    struct ppc4xx_spi *hw;
    u8 status;
    u8 data;
    unsigned int count;
    hw = (struct ppc4xx_spi *)dev_id;
    status = in_8(&hw.regs.sr);
    if (!status)
    return IRQ_NONE;
//
// BSY de-asserts one cycle after the transfer is complete.  The
// interrupt is asserted after the transfer is complete.  The exact
// relationship is not documented, hence this code.
//
    if (unlikely(status & SPI_PPC4XX_SR_BSY)) {
    u8 lstatus;
    let mut cnt: c_int = 0;
    dev_dbg(hw.dev, "got interrupt but spi still busy?\n");
    do {
    ndelay(10);
    lstatus = in_8(&hw.regs.sr);
    } while (++cnt < 100 && lstatus & SPI_PPC4XX_SR_BSY);
    if (cnt >= 100) {
    dev_err(hw.dev, "busywait: too many loops!\n");
    complete(&hw.done);
    return IRQ_HANDLED;
    } else {
// status is always 1 (RBR) here
    status = in_8(&hw.regs.sr);
    dev_dbg(hw.dev, "loops %d status %x\n", cnt, status);
    }
    }
    count = hw.count;
    hw.count++;
// RBR triggered this interrupt.  Therefore, data must be ready.
    data = in_8(&hw.regs.rxd);
    if (hw.rx)
    hw.rx[count] = data;
    count++;
    if (count < hw.len) {
    data = hw.tx ? hw.tx[count] : 0;
    out_8(&hw.regs.txd, data);
    out_8(&hw.regs.cr, SPI_PPC4XX_CR_STR);
    } else {
    complete(&hw.done);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_cleanup(spi: *mut spi_device) {
    static void spi_ppc4xx_cleanup(struct spi_device *spi)
    {
    kfree(spi.controller_state);
    }
#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_enable(hw: *mut ppc4xx_spi) {
    static void spi_ppc4xx_enable(struct ppc4xx_spi *hw)
    {
//
// On all 4xx PPC's the SPI bus is shared/multiplexed with
// the 2nd I2C bus. We need to enable the SPI bus before
// using it.
//
// need to clear bit 14 to enable SPC
    dcri_clrset(SDR0, SDR0_PFC1, 0x80000000 >> 14, 0);
    }
//
// platform_device layer stuff...
//
#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_of_probe(op: *mut platform_device) -> c_int {
    static int spi_ppc4xx_of_probe(struct platform_device *op)
    {
    struct ppc4xx_spi *hw;
    struct spi_controller *host;
    struct spi_bitbang *bbp;
    struct device_node *np = op.dev.of_node;
    struct device *dev = &op.dev;
    struct device_node *opbnp;
    int ret;
    unsigned int opb_freq;
    void __iomem *regs;
    int irqnum;
    regs = devm_platform_ioremap_resource(op, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
// Request IRQ
    irqnum = platform_get_irq(op, 0);
    if (irqnum < 0)
    return irqnum;
    host = devm_spi_alloc_host(dev, sizeof(*hw));
    if (host == core::ptr::null_mut())
    return -ENOMEM;
    host.dev.of_node = np;
    platform_set_drvdata(op, host);
    hw = spi_controller_get_devdata(host);
    hw.host = host;
    hw.dev = dev;
    init_completion(&hw.done);
// Setup the state for the bitbang driver
    bbp = &hw.bitbang;
    bbp.ctlr = hw.host;
    bbp.setup_transfer = spi_ppc4xx_setupxfer;
    bbp.txrx_bufs = spi_ppc4xx_txrx;
    bbp.use_dma = 0;
    bbp.ctlr.setup = spi_ppc4xx_setup;
    bbp.ctlr.cleanup = spi_ppc4xx_cleanup;
    bbp.ctlr.bits_per_word_mask = SPI_BPW_MASK(8);
    bbp.ctlr.use_gpio_descriptors = true;
//
// The SPI core will count the number of GPIO descriptors to figure
// out the number of chip selects available on the platform.
//
    bbp.ctlr.num_chipselect = 0;
// the spi->mode bits understood by this driver:
    bbp.ctlr.mode_bits =
    SPI_CPHA | SPI_CPOL | SPI_CS_HIGH | SPI_LSB_FIRST;
// Get the clock for the OPB
    opbnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,opb");
    if (opbnp == core::ptr::null_mut()) {
    dev_err(dev, "OPB: cannot find node\n");
    return -ENODEV;
    }
// Get the clock (Hz) for the OPB
    ret = of_property_read_u32(opbnp, "clock-frequency", &opb_freq);
    of_node_put(opbnp);
    if (ret) {
    dev_err(dev, "OPB: no clock-frequency property set\n");
    return -ENODEV;
    }
    hw.opb_freq = opb_freq;
    hw.opb_freq >>= 2;
    hw.regs = regs;
    ret = devm_request_irq(&op.dev, irqnum, spi_ppc4xx_int,
    0, "spi_ppc4xx_of", hw);
    if (ret)
    return ret;
    spi_ppc4xx_enable(hw);
// Finally register our spi controller
    dev.dma_mask = 0;
    return spi_bitbang_start(bbp);
    }
#[no_mangle]
unsafe extern "C" fn spi_ppc4xx_of_remove(op: *mut platform_device) {
    static void spi_ppc4xx_of_remove(struct platform_device *op)
    {
    struct spi_controller *host = platform_get_drvdata(op);
    struct ppc4xx_spi *hw = spi_controller_get_devdata(host);
    spi_bitbang_stop(&hw.bitbang);
    }
    static const struct of_device_id spi_ppc4xx_of_match[] = {
    { .compatible = "ibm,ppc4xx-spi", },
    {},
    };
    MODULE_DEVICE_TABLE(of, spi_ppc4xx_of_match);
    static struct platform_driver spi_ppc4xx_of_driver = {
    .probe = spi_ppc4xx_of_probe,
    .remove = spi_ppc4xx_of_remove,
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = spi_ppc4xx_of_match,
    },
    };
    module_platform_driver(spi_ppc4xx_of_driver);
    MODULE_AUTHOR("Gary Jennejohn & Stefan Roese");
    MODULE_DESCRIPTION("Simple PPC4xx SPI Driver");
    MODULE_LICENSE("GPL");
