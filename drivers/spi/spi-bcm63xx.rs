//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-bcm63xx.c
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
// Broadcom BCM63xx SPI controller support
//
// Copyright (C) 2009-2012 Florian Fainelli <florian@openwrt.org>
// Copyright (C) 2010 Tanguy Bouzeloc <tanguy.bouzeloc@efixo.com>
//

// BCM 6338/6348 SPI core
pub const SPI_6348_RSET_SIZE: c_int = 64;
pub const SPI_6348_CMD: c_uint = 0x00	/* 16-bits register */;
pub const SPI_6348_INT_STATUS: c_uint = 0x02;
pub const SPI_6348_INT_MASK_ST: c_uint = 0x03;
pub const SPI_6348_INT_MASK: c_uint = 0x04;
pub const SPI_6348_ST: c_uint = 0x05;
pub const SPI_6348_CLK_CFG: c_uint = 0x06;
pub const SPI_6348_FILL_BYTE: c_uint = 0x07;
pub const SPI_6348_MSG_TAIL: c_uint = 0x09;
pub const SPI_6348_RX_TAIL: c_uint = 0x0b;
pub const SPI_6348_MSG_CTL: c_uint = 0x40	/* 8-bits register */;
pub const SPI_6348_MSG_CTL_WIDTH: c_int = 8;
pub const SPI_6348_MSG_DATA: c_uint = 0x41;
pub const SPI_6348_MSG_DATA_SIZE: c_uint = 0x3f;
pub const SPI_6348_RX_DATA: c_uint = 0x80;
pub const SPI_6348_RX_DATA_SIZE: c_uint = 0x3f;
// BCM 3368/6358/6262/6368 SPI core
pub const SPI_6358_RSET_SIZE: c_int = 1804;
pub const SPI_6358_MSG_CTL: c_uint = 0x00	/* 16-bits register */;
pub const SPI_6358_MSG_CTL_WIDTH: c_int = 16;
pub const SPI_6358_MSG_DATA: c_uint = 0x02;
pub const SPI_6358_MSG_DATA_SIZE: c_uint = 0x21e;
pub const SPI_6358_RX_DATA: c_uint = 0x400;
pub const SPI_6358_RX_DATA_SIZE: c_uint = 0x220;
pub const SPI_6358_CMD: c_uint = 0x700	/* 16-bits register */;
pub const SPI_6358_INT_STATUS: c_uint = 0x702;
pub const SPI_6358_INT_MASK_ST: c_uint = 0x703;
pub const SPI_6358_INT_MASK: c_uint = 0x704;
pub const SPI_6358_ST: c_uint = 0x705;
pub const SPI_6358_CLK_CFG: c_uint = 0x706;
pub const SPI_6358_FILL_BYTE: c_uint = 0x707;
pub const SPI_6358_MSG_TAIL: c_uint = 0x709;
pub const SPI_6358_RX_TAIL: c_uint = 0x70B;
// Shared SPI definitions
// Message configuration
pub const SPI_FD_RW: c_uint = 0x00;
pub const SPI_HD_W: c_uint = 0x01;
pub const SPI_HD_R: c_uint = 0x02;
pub const SPI_BYTE_CNT_SHIFT: c_int = 0;
pub const SPI_6348_MSG_TYPE_SHIFT: c_int = 6;
pub const SPI_6358_MSG_TYPE_SHIFT: c_int = 14;
// Command
pub const SPI_CMD_NOOP: c_uint = 0x00;
pub const SPI_CMD_SOFT_RESET: c_uint = 0x01;
pub const SPI_CMD_HARD_RESET: c_uint = 0x02;
pub const SPI_CMD_START_IMMEDIATE: c_uint = 0x03;
pub const SPI_CMD_COMMAND_SHIFT: c_int = 0;
pub const SPI_CMD_COMMAND_MASK: c_uint = 0x000f;
pub const SPI_CMD_DEVICE_ID_SHIFT: c_int = 4;
pub const SPI_CMD_PREPEND_BYTE_CNT_SHIFT: c_int = 8;
pub const SPI_CMD_ONE_BYTE_SHIFT: c_int = 11;
pub const SPI_CMD_ONE_WIRE_SHIFT: c_int = 12;
pub const SPI_DEV_ID_0: c_int = 0;
pub const SPI_DEV_ID_1: c_int = 1;
pub const SPI_DEV_ID_2: c_int = 2;
pub const SPI_DEV_ID_3: c_int = 3;
// Interrupt mask
pub const SPI_INTR_CMD_DONE: c_uint = 0x01;
pub const SPI_INTR_RX_OVERFLOW: c_uint = 0x02;
pub const SPI_INTR_TX_UNDERFLOW: c_uint = 0x04;
pub const SPI_INTR_TX_OVERFLOW: c_uint = 0x08;
pub const SPI_INTR_RX_UNDERFLOW: c_uint = 0x10;
pub const SPI_INTR_CLEAR_ALL: c_uint = 0x1f;
// Status
pub const SPI_RX_EMPTY: c_uint = 0x02;
pub const SPI_CMD_BUSY: c_uint = 0x04;
pub const SPI_SERIAL_BUSY: c_uint = 0x08;
// Clock configuration
pub const SPI_CLK_20MHZ: c_uint = 0x00;
pub const SPI_CLK_0_391MHZ: c_uint = 0x01;
pub const SPI_CLK_0_781MHZ: c_uint = 0x02	/* default */;
pub const SPI_CLK_1_563MHZ: c_uint = 0x03;
pub const SPI_CLK_3_125MHZ: c_uint = 0x04;
pub const SPI_CLK_6_250MHZ: c_uint = 0x05;
pub const SPI_CLK_12_50MHZ: c_uint = 0x06;
pub const SPI_CLK_MASK: c_uint = 0x07;
pub const SPI_SSOFFTIME_MASK: c_uint = 0x38;
pub const SPI_SSOFFTIME_SHIFT: c_int = 3;
pub const SPI_BYTE_SWAP: c_uint = 0x80;
    enum bcm63xx_regs_spi {
    SPI_CMD,
    SPI_INT_STATUS,
    SPI_INT_MASK_ST,
    SPI_INT_MASK,
    SPI_ST,
    SPI_CLK_CFG,
    SPI_FILL_BYTE,
    SPI_MSG_TAIL,
    SPI_RX_TAIL,
    SPI_MSG_CTL,
    SPI_MSG_DATA,
    SPI_RX_DATA,
    SPI_MSG_TYPE_SHIFT,
    SPI_MSG_CTL_WIDTH,
    SPI_MSG_DATA_SIZE,
    };
pub const BCM63XX_SPI_MAX_PREPEND: c_int = 7;
pub const BCM63XX_SPI_MAX_CS: c_int = 8;
pub const BCM63XX_SPI_BUS_NUM: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63xx_spi {
    pub done: completion,
    pub regs: *mut void __iomem,
    pub irq: c_int,
// Platform data
    pub reg_offsets: *const c_ulong,
    pub fifo_size: c_uint,
    pub msg_type_shift: c_uint,
    pub msg_ctl_width: c_uint,
// data iomem
    pub tx_io: *mut u8 __iomem,
    pub rx_io: *const u8 __iomem,
    pub clk: *mut clk,
    pub pdev: *mut platform_device,
}

    static inline u8 bcm_spi_readb(struct bcm63xx_spi *bs,
    unsigned int offset)
    {
    return readb(bs.regs + bs.reg_offsets[offset]);
    }
    static inline void bcm_spi_writeb(struct bcm63xx_spi *bs,
    u8 value, unsigned int offset)
    {
    writeb(value, bs.regs + bs.reg_offsets[offset]);
    }
    static inline void bcm_spi_writew(struct bcm63xx_spi *bs,
    u16 value, unsigned int offset)
    {

    iowrite16be(value, bs.regs + bs.reg_offsets[offset]);

    writew(value, bs.regs + bs.reg_offsets[offset]);

    }
    static const unsigned int bcm63xx_spi_freq_table[SPI_CLK_MASK][2] = {
    { 20000000, SPI_CLK_20MHZ },
    { 12500000, SPI_CLK_12_50MHZ },
    {  6250000, SPI_CLK_6_250MHZ },
    {  3125000, SPI_CLK_3_125MHZ },
    {  1563000, SPI_CLK_1_563MHZ },
    {   781000, SPI_CLK_0_781MHZ },
    {   391000, SPI_CLK_0_391MHZ }
    };
    static void bcm63xx_spi_setup_transfer(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct bcm63xx_spi *bs = spi_controller_get_devdata(spi.controller);
    u8 clk_cfg, reg;
    int i;
// Default to lowest clock configuration
    clk_cfg = SPI_CLK_0_391MHZ;
// Find the closest clock configuration
    for (i = 0; i < SPI_CLK_MASK; i++) {
    if (t.speed_hz >= bcm63xx_spi_freq_table[i][0]) {
    clk_cfg = bcm63xx_spi_freq_table[i][1];
    break;
    }
    }
// clear existing clock configuration bits of the register
    reg = bcm_spi_readb(bs, SPI_CLK_CFG);
    reg &= ~SPI_CLK_MASK;
    reg |= clk_cfg;
    bcm_spi_writeb(bs, reg, SPI_CLK_CFG);
    dev_dbg(&spi.dev, "Setting clock register to %02x (hz %d)\n",
    clk_cfg, t.speed_hz);
    }
// the spi->mode bits understood by this driver:

    static int bcm63xx_txrx_bufs(struct spi_device *spi, struct spi_transfer *first,
    unsigned int num_transfers)
    {
    struct bcm63xx_spi *bs = spi_controller_get_devdata(spi.controller);
    u16 msg_ctl;
    u16 cmd;
    unsigned int i, timeout = 0, prepend_len = 0, len = 0;
    struct spi_transfer *t = first;
    let mut do_rx: bool = false;
    let mut do_tx: bool = false;
// Disable the CMD_DONE interrupt
    bcm_spi_writeb(bs, 0, SPI_INT_MASK);
    dev_dbg(&spi.dev, "txrx: tx %p, rx %p, len %d\n",
    t.tx_buf, t.rx_buf, t.len);
    if (num_transfers > 1 && t.tx_buf && t.len <= BCM63XX_SPI_MAX_PREPEND)
    prepend_len = t.len;
// prepare the buffer
    for (i = 0; i < num_transfers; i++) {
    if (t.tx_buf) {
    do_tx = true;
    memcpy_toio(bs.tx_io + len, t.tx_buf, t.len);
// don't prepend more than one tx
    if (t != first)
    prepend_len = 0;
    }
    if (t.rx_buf) {
    do_rx = true;
//
// In certain hardware implementations, there appears to be a
// hidden accumulator that tracks the number of bytes written into
// the hardware FIFO, and this accumulator overrides the length in
// the SPI_MSG_CTL register.
//
// Therefore, for read-only transfers, we need to write some dummy
// value into the FIFO to keep the accumulator tracking the correct
// length.
//
    if (!t.tx_buf)
    memset_io(bs.tx_io + len, 0xFF, t.len);
// prepend is half-duplex write only
    if (t == first)
    prepend_len = 0;
    }
    len += t.len;
    t = list_entry(t.transfer_list.next, struct spi_transfer,
    transfer_list);
    }
    reinit_completion(&bs.done);
// Fill in the Message control register
    msg_ctl = (len << SPI_BYTE_CNT_SHIFT);
    if (do_rx && do_tx && prepend_len == 0)
    msg_ctl |= (SPI_FD_RW << bs.msg_type_shift);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: do_rx) -> else {
    else if (do_rx)
    msg_ctl |= (SPI_HD_R << bs.msg_type_shift);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: do_tx) -> else {
    else if (do_tx)
    msg_ctl |= (SPI_HD_W << bs.msg_type_shift);
    switch (bs.msg_ctl_width) {
    case 8:
    bcm_spi_writeb(bs, msg_ctl, SPI_MSG_CTL);
    break;
    case 16:
    bcm_spi_writew(bs, msg_ctl, SPI_MSG_CTL);
    break;
    }
// Issue the transfer
    cmd = SPI_CMD_START_IMMEDIATE;
    cmd |= (prepend_len << SPI_CMD_PREPEND_BYTE_CNT_SHIFT);
    cmd |= (spi_get_chipselect(spi, 0) << SPI_CMD_DEVICE_ID_SHIFT);
    bcm_spi_writew(bs, cmd, SPI_CMD);
// Enable the CMD_DONE interrupt
    bcm_spi_writeb(bs, SPI_INTR_CMD_DONE, SPI_INT_MASK);
    timeout = wait_for_completion_timeout(&bs.done, HZ);
    if (!timeout)
    return -ETIMEDOUT;
    if (!do_rx)
    return 0;
    len = 0;
    t = first;
// Read out all the data
    for (i = 0; i < num_transfers; i++) {
    if (t.rx_buf)
    memcpy_fromio(t.rx_buf, bs.rx_io + len, t.len);
    if (t != first || prepend_len == 0)
    len += t.len;
    t = list_entry(t.transfer_list.next, struct spi_transfer,
    transfer_list);
    }
    return 0;
    }
    static int bcm63xx_spi_transfer_one(struct spi_controller *host,
    struct spi_message *m)
    {
    struct bcm63xx_spi *bs = spi_controller_get_devdata(host);
    struct spi_transfer *t, *first = core::ptr::null_mut();
    struct spi_device *spi = m.spi;
    let mut status: c_int = 0;
    let mut n_transfers: c_uint = 0, total_len = 0;
    let mut can_use_prepend: bool = false;
//
// This SPI controller does not support keeping CS active after a
// transfer.
// Work around this by merging as many transfers we can into one big
// full-duplex transfers.
//
    list_for_each_entry(t, &m.transfers, transfer_list) {
    if (!first)
    first = t;
    n_transfers++;
    total_len += t.len;
    if (n_transfers == 2 && !first.rx_buf && !t.tx_buf &&
    first.len <= BCM63XX_SPI_MAX_PREPEND)
    can_use_prepend = true;
#[no_mangle]
pub unsafe extern "C" fn if(t->tx_buf: can_use_prepend &&) -> else {
    else if (can_use_prepend && t.tx_buf)
    can_use_prepend = false;
// we can only transfer one fifo worth of data
    if ((can_use_prepend &&
    total_len > (bs.fifo_size + BCM63XX_SPI_MAX_PREPEND)) ||
    (!can_use_prepend && total_len > bs.fifo_size)) {
    dev_err(&spi.dev, "unable to do transfers larger than FIFO size (%i > %i)\n",
    total_len, bs.fifo_size);
    status = -EINVAL;
    goto exit;
    }
// all combined transfers have to have the same speed
    if (t.speed_hz != first.speed_hz) {
    dev_err(&spi.dev, "unable to change speed between transfers\n");
    status = -EINVAL;
    goto exit;
    }
// CS will be deasserted directly after transfer
    if (t.delay.value) {
    dev_err(&spi.dev, "unable to keep CS asserted after transfer\n");
    status = -EINVAL;
    goto exit;
    }
    if (t.cs_change ||
    list_is_last(&t.transfer_list, &m.transfers)) {
// configure adapter for a new transfer
    bcm63xx_spi_setup_transfer(spi, first);
// send the data
    status = bcm63xx_txrx_bufs(spi, first, n_transfers);
    if (status)
    goto exit;
    m.actual_length += total_len;
    first = core::ptr::null_mut();
    n_transfers = 0;
    total_len = 0;
    can_use_prepend = false;
    }
    }
    exit:
    m.status = status;
    spi_finalize_current_message(host);
    return 0;
    }
// This driver supports single host mode only. Hence
// CMD_DONE is the only interrupt we care about
//
#[no_mangle]
unsafe extern "C" fn bcm63xx_spi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcm63xx_spi_interrupt(int irq, void *dev_id)
    {
    struct spi_controller *host = (struct spi_controller *)dev_id;
    struct bcm63xx_spi *bs = spi_controller_get_devdata(host);
    u8 intr;
// Read interupts and clear them immediately
    intr = bcm_spi_readb(bs, SPI_INT_STATUS);
    bcm_spi_writeb(bs, SPI_INTR_CLEAR_ALL, SPI_INT_STATUS);
    bcm_spi_writeb(bs, 0, SPI_INT_MASK);
// A transfer completed
    if (intr & SPI_INTR_CMD_DONE)
    complete(&bs.done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_spi_max_length(spi: *mut spi_device) -> usize {
    static size_t bcm63xx_spi_max_length(struct spi_device *spi)
    {
    struct bcm63xx_spi *bs = spi_controller_get_devdata(spi.controller);
    return bs.fifo_size;
    }
    static const unsigned long bcm6348_spi_reg_offsets[] = {
    [SPI_CMD]		= SPI_6348_CMD,
    [SPI_INT_STATUS]	= SPI_6348_INT_STATUS,
    [SPI_INT_MASK_ST]	= SPI_6348_INT_MASK_ST,
    [SPI_INT_MASK]		= SPI_6348_INT_MASK,
    [SPI_ST]		= SPI_6348_ST,
    [SPI_CLK_CFG]		= SPI_6348_CLK_CFG,
    [SPI_FILL_BYTE]		= SPI_6348_FILL_BYTE,
    [SPI_MSG_TAIL]		= SPI_6348_MSG_TAIL,
    [SPI_RX_TAIL]		= SPI_6348_RX_TAIL,
    [SPI_MSG_CTL]		= SPI_6348_MSG_CTL,
    [SPI_MSG_DATA]		= SPI_6348_MSG_DATA,
    [SPI_RX_DATA]		= SPI_6348_RX_DATA,
    [SPI_MSG_TYPE_SHIFT]	= SPI_6348_MSG_TYPE_SHIFT,
    [SPI_MSG_CTL_WIDTH]	= SPI_6348_MSG_CTL_WIDTH,
    [SPI_MSG_DATA_SIZE]	= SPI_6348_MSG_DATA_SIZE,
    };
    static const unsigned long bcm6358_spi_reg_offsets[] = {
    [SPI_CMD]		= SPI_6358_CMD,
    [SPI_INT_STATUS]	= SPI_6358_INT_STATUS,
    [SPI_INT_MASK_ST]	= SPI_6358_INT_MASK_ST,
    [SPI_INT_MASK]		= SPI_6358_INT_MASK,
    [SPI_ST]		= SPI_6358_ST,
    [SPI_CLK_CFG]		= SPI_6358_CLK_CFG,
    [SPI_FILL_BYTE]		= SPI_6358_FILL_BYTE,
    [SPI_MSG_TAIL]		= SPI_6358_MSG_TAIL,
    [SPI_RX_TAIL]		= SPI_6358_RX_TAIL,
    [SPI_MSG_CTL]		= SPI_6358_MSG_CTL,
    [SPI_MSG_DATA]		= SPI_6358_MSG_DATA,
    [SPI_RX_DATA]		= SPI_6358_RX_DATA,
    [SPI_MSG_TYPE_SHIFT]	= SPI_6358_MSG_TYPE_SHIFT,
    [SPI_MSG_CTL_WIDTH]	= SPI_6358_MSG_CTL_WIDTH,
    [SPI_MSG_DATA_SIZE]	= SPI_6358_MSG_DATA_SIZE,
    };
    static const struct platform_device_id bcm63xx_spi_dev_match[] = {
    {
    .name = "bcm6348-spi",
    .driver_data = (unsigned long)bcm6348_spi_reg_offsets,
    },
    {
    .name = "bcm6358-spi",
    .driver_data = (unsigned long)bcm6358_spi_reg_offsets,
    },
    { }
    };
    MODULE_DEVICE_TABLE(platform, bcm63xx_spi_dev_match);
    static const struct of_device_id bcm63xx_spi_of_match[] = {
    { .compatible = "brcm,bcm6348-spi", .data = &bcm6348_spi_reg_offsets },
    { .compatible = "brcm,bcm6358-spi", .data = &bcm6358_spi_reg_offsets },
    { },
    };
    MODULE_DEVICE_TABLE(of, bcm63xx_spi_of_match);
#[no_mangle]
unsafe extern "C" fn bcm63xx_spi_probe(pdev: *mut platform_device) -> c_int {
    static int bcm63xx_spi_probe(struct platform_device *pdev)
    {
    struct resource *r;
    const unsigned long *bcm63xx_spireg;
    struct device *dev = &pdev.dev;
    int irq, bus_num;
    struct spi_controller *host;
    struct clk *clk;
    struct bcm63xx_spi *bs;
    int ret;
    let mut num_cs: u32 = BCM63XX_SPI_MAX_CS;
    struct reset_control *reset;
    if (dev.of_node) {
    const struct of_device_id *match;
    match = of_match_node(bcm63xx_spi_of_match, dev.of_node);
    if (!match)
    return -EINVAL;
    bcm63xx_spireg = match.data;
    of_property_read_u32(dev.of_node, "num-cs", &num_cs);
    if (num_cs > BCM63XX_SPI_MAX_CS) {
    dev_warn(dev, "unsupported number of cs (%i), reducing to 8\n",
    num_cs);
    num_cs = BCM63XX_SPI_MAX_CS;
    }
    bus_num = -1;
    } else if (pdev.id_entry.driver_data) {
    const struct platform_device_id *match = pdev.id_entry;
    bcm63xx_spireg = (const unsigned long *)match.driver_data;
    bus_num = BCM63XX_SPI_BUS_NUM;
    } else {
    return -EINVAL;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    clk = devm_clk_get(dev, "spi");
    if (IS_ERR(clk)) {
    dev_err(dev, "no clock for device\n");
    return PTR_ERR(clk);
    }
    reset = devm_reset_control_get_optional_shared(dev, core::ptr::null_mut());
    if (IS_ERR(reset))
    return PTR_ERR(reset);
    host = devm_spi_alloc_host(dev, sizeof(*bs));
    if (!host)
    return -ENOMEM;
    bs = spi_controller_get_devdata(host);
    init_completion(&bs.done);
    platform_set_drvdata(pdev, host);
    bs.pdev = pdev;
    bs.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &r);
    if (IS_ERR(bs.regs))
    return PTR_ERR(bs.regs);
    bs.irq = irq;
    bs.clk = clk;
    bs.reg_offsets = bcm63xx_spireg;
    bs.fifo_size = bs.reg_offsets[SPI_MSG_DATA_SIZE];
    ret = devm_request_irq(&pdev.dev, irq, bcm63xx_spi_interrupt, 0,
    pdev.name, host);
    if (ret) {
    dev_err(dev, "unable to request irq\n");
    return ret;
    }
    host.bus_num = bus_num;
    host.num_chipselect = num_cs;
    host.transfer_one_message = bcm63xx_spi_transfer_one;
    host.mode_bits = MODEBITS;
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    host.max_transfer_size = bcm63xx_spi_max_length;
    host.max_message_size = bcm63xx_spi_max_length;
    host.auto_runtime_pm = true;
    bs.msg_type_shift = bs.reg_offsets[SPI_MSG_TYPE_SHIFT];
    bs.msg_ctl_width = bs.reg_offsets[SPI_MSG_CTL_WIDTH];
    bs.tx_io = bs.regs + bs.reg_offsets[SPI_MSG_DATA];
    bs.rx_io = bs.regs + bs.reg_offsets[SPI_RX_DATA];
// Initialize hardware
    ret = clk_prepare_enable(bs.clk);
    if (ret)
    return ret;
    ret = reset_control_reset(reset);
    if (ret) {
    dev_err(dev, "unable to reset device: %d\n", ret);
    goto out_clk_disable;
    }
    bcm_spi_writeb(bs, SPI_INTR_CLEAR_ALL, SPI_INT_STATUS);
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret)
    goto out_clk_disable;
// register and we are done
    ret = spi_register_controller(host);
    if (ret) {
    dev_err(dev, "spi register failed\n");
    goto out_clk_disable;
    }
    dev_info(dev, "at %pr (irq %d, FIFOs size %d)\n",
    r, irq, bs.fifo_size);
    return 0;
    out_clk_disable:
    clk_disable_unprepare(clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_spi_remove(pdev: *mut platform_device) {
    static void bcm63xx_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct bcm63xx_spi *bs = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
// reset spi block
    bcm_spi_writeb(bs, 0, SPI_INT_MASK);
// HW shutdown
    clk_disable_unprepare(bs.clk);
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_spi_suspend(dev: *mut device) -> c_int {
    static int bcm63xx_spi_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct bcm63xx_spi *bs = spi_controller_get_devdata(host);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
    clk_disable_unprepare(bs.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_spi_resume(dev: *mut device) -> c_int {
    static int bcm63xx_spi_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct bcm63xx_spi *bs = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(bs.clk);
    if (ret)
    return ret;
    ret = spi_controller_resume(host);
    if (ret) {
    clk_disable_unprepare(bs.clk);
    return ret;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(bcm63xx_spi_pm_ops, bcm63xx_spi_suspend, bcm63xx_spi_resume);
    static struct platform_driver bcm63xx_spi_driver = {
    .driver = {
    .name	= "bcm63xx-spi",
    .pm	= &bcm63xx_spi_pm_ops,
    .of_match_table = bcm63xx_spi_of_match,
    },
    .id_table	= bcm63xx_spi_dev_match,
    .probe		= bcm63xx_spi_probe,
    .remove		= bcm63xx_spi_remove,
    };
    module_platform_driver(bcm63xx_spi_driver);
    MODULE_ALIAS("platform:bcm63xx_spi");
    MODULE_AUTHOR("Florian Fainelli <florian@openwrt.org>");
    MODULE_AUTHOR("Tanguy Bouzeloc <tanguy.bouzeloc@efixo.com>");
    MODULE_DESCRIPTION("Broadcom BCM63xx SPI Controller driver");
    MODULE_LICENSE("GPL");
