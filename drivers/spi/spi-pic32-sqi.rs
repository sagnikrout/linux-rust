//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-pic32-sqi.c
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
// PIC32 Quad SPI controller driver.
//
// Purna Chandra Mandal <purna.mandal@microchip.com>
// Copyright (c) 2016, Microchip Technology Inc.
//

// SQI registers
pub const PESQI_XIP_CONF1_REG: c_uint = 0x00;
pub const PESQI_XIP_CONF2_REG: c_uint = 0x04;
pub const PESQI_CONF_REG: c_uint = 0x08;
pub const PESQI_CTRL_REG: c_uint = 0x0C;
pub const PESQI_CLK_CTRL_REG: c_uint = 0x10;
pub const PESQI_CMD_THRES_REG: c_uint = 0x14;
pub const PESQI_INT_THRES_REG: c_uint = 0x18;
pub const PESQI_INT_ENABLE_REG: c_uint = 0x1C;
pub const PESQI_INT_STAT_REG: c_uint = 0x20;
pub const PESQI_TX_DATA_REG: c_uint = 0x24;
pub const PESQI_RX_DATA_REG: c_uint = 0x28;
pub const PESQI_STAT1_REG: c_uint = 0x2C;
pub const PESQI_STAT2_REG: c_uint = 0x30;
pub const PESQI_BD_CTRL_REG: c_uint = 0x34;
pub const PESQI_BD_CUR_ADDR_REG: c_uint = 0x38;
pub const PESQI_BD_BASE_ADDR_REG: c_uint = 0x40;
pub const PESQI_BD_STAT_REG: c_uint = 0x44;
pub const PESQI_BD_POLL_CTRL_REG: c_uint = 0x48;
pub const PESQI_BD_TX_DMA_STAT_REG: c_uint = 0x4C;
pub const PESQI_BD_RX_DMA_STAT_REG: c_uint = 0x50;
pub const PESQI_THRES_REG: c_uint = 0x54;
pub const PESQI_INT_SIGEN_REG: c_uint = 0x58;
// PESQI_CONF_REG fields
pub const PESQI_MODE: c_uint = 0x7;
pub const PESQI_MODE_BOOT: c_int = 0;
pub const PESQI_MODE_PIO: c_int = 1;
pub const PESQI_MODE_DMA: c_int = 2;
pub const PESQI_MODE_XIP: c_int = 3;
pub const PESQI_MODE_SHIFT: c_int = 0;

pub const PESQI_LANES_SHIFT: c_int = 20;
pub const PESQI_SINGLE_LANE: c_int = 0;
pub const PESQI_DUAL_LANE: c_int = 1;
pub const PESQI_QUAD_LANE: c_int = 2;
pub const PESQI_CSEN_SHIFT: c_int = 24;

// PESQI_CLK_CTRL_REG fields

pub const PESQI_CLKDIV_SHIFT: c_int = 8;
pub const PESQI_CLKDIV: c_uint = 0xff;
// PESQI_INT_THR/CMD_THR_REG
pub const PESQI_TXTHR_MASK: c_uint = 0x1f;
pub const PESQI_TXTHR_SHIFT: c_int = 8;
pub const PESQI_RXTHR_MASK: c_uint = 0x1f;
pub const PESQI_RXTHR_SHIFT: c_int = 0;
// PESQI_INT_EN/INT_STAT/INT_SIG_EN_REG

// PESQI_BD_CTRL_REG

// PESQI controller buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buf_desc {
    pub /: *mut *mut u32 bd_ctrl; / control,
    pub /: *mut *mut u32 bd_status; / reserved,
    pub /: *mut *mut u32 bd_addr; / DMA buffer addr,
    pub /: *mut *mut u32 bd_nextp; / next item in chain,
}

// bd_ctrl
pub const BD_BUFLEN: c_uint = 0x1ff;

//
// struct ring_desc - Representation of SQI ring descriptor
// @list:	list element to add to free or used list.
// @bd:		PESQI controller buffer descriptor
// @bd_dma:	DMA address of PESQI controller buffer descriptor
// @xfer_len:	transfer length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_desc {
    pub list: list_head,
    pub bd: *mut buf_desc,
    pub bd_dma: dma_addr_t,
    pub xfer_len: u32,
}

// Global constants
pub const PESQI_BD_BUF_LEN_MAX: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_sqi {
    pub regs: *mut void __iomem,
    pub sys_clk: *mut clk,
    pub /: *mut *mut *mut clk base_clk; / drives spi clock,
    pub host: *mut spi_controller,
    pub irq: c_int,
    pub xfer_done: completion,
    pub ring: *mut ring_desc,
    pub bd: *mut c_void,
    pub bd_dma: dma_addr_t,
    pub /: *mut *mut list_head bd_list_free; / free,
    pub /: *mut *mut list_head bd_list_used; / allocated,
    pub cur_spi: *mut spi_device,
    pub cur_speed: u32,
    pub cur_mode: u8,
}

#[no_mangle]
pub unsafe extern "C" fn pic32_setbits(reg: *mut void __iomem, set: u32) {
    static inline void pic32_setbits(void __iomem *reg, u32 set)
    {
    writel(readl(reg) | set, reg);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_clrbits(reg: *mut void __iomem, clr: u32) {
    static inline void pic32_clrbits(void __iomem *reg, u32 clr)
    {
    writel(readl(reg) & ~clr, reg);
    }
#[no_mangle]
unsafe extern "C" fn pic32_sqi_set_clk_rate(sqi: *mut pic32_sqi, sck: u32) -> c_int {
    static int pic32_sqi_set_clk_rate(struct pic32_sqi *sqi, u32 sck)
    {
    u32 val, div;
// div = base_clk / (2 * spi_clk)
    div = clk_get_rate(sqi.base_clk) / (2 * sck);
    div &= PESQI_CLKDIV;
    val = readl(sqi.regs + PESQI_CLK_CTRL_REG);
// apply new divider
    val &= ~(PESQI_CLK_STABLE | (PESQI_CLKDIV << PESQI_CLKDIV_SHIFT));
    val |= div << PESQI_CLKDIV_SHIFT;
    writel(val, sqi.regs + PESQI_CLK_CTRL_REG);
// wait for stability
    return readl_poll_timeout(sqi.regs + PESQI_CLK_CTRL_REG, val,
    val & PESQI_CLK_STABLE, 1, 5000);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_sqi_enable_int(sqi: *mut pic32_sqi) {
    static inline void pic32_sqi_enable_int(struct pic32_sqi *sqi)
    {
    let mut mask: u32 = PESQI_DMAERR | PESQI_BDDONE | PESQI_PKTCOMP;
    writel(mask, sqi.regs + PESQI_INT_ENABLE_REG);
// INT_SIGEN works as interrupt-gate to INTR line
    writel(mask, sqi.regs + PESQI_INT_SIGEN_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_sqi_disable_int(sqi: *mut pic32_sqi) {
    static inline void pic32_sqi_disable_int(struct pic32_sqi *sqi)
    {
    writel(0, sqi.regs + PESQI_INT_ENABLE_REG);
    writel(0, sqi.regs + PESQI_INT_SIGEN_REG);
    }
#[no_mangle]
unsafe extern "C" fn pic32_sqi_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_sqi_isr(int irq, void *dev_id)
    {
    struct pic32_sqi *sqi = dev_id;
    u32 enable, status;
    enable = readl(sqi.regs + PESQI_INT_ENABLE_REG);
    status = readl(sqi.regs + PESQI_INT_STAT_REG);
// check spurious interrupt
    if (!status)
    return IRQ_NONE;
    if (status & PESQI_DMAERR) {
    enable = 0;
    goto irq_done;
    }
    if (status & PESQI_TXTHR)
    enable &= ~(PESQI_TXTHR | PESQI_TXFULL | PESQI_TXEMPTY);
    if (status & PESQI_RXTHR)
    enable &= ~(PESQI_RXTHR | PESQI_RXFULL | PESQI_RXEMPTY);
    if (status & PESQI_BDDONE)
    enable &= ~PESQI_BDDONE;
// packet processing completed
    if (status & PESQI_PKTCOMP) {
// mask all interrupts
    enable = 0;
// complete transaction
    complete(&sqi.xfer_done);
    }
    irq_done:
// interrupts are sticky, so mask when handled
    writel(enable, sqi.regs + PESQI_INT_ENABLE_REG);
    return IRQ_HANDLED;
    }
    static struct ring_desc *ring_desc_get(struct pic32_sqi *sqi)
    {
    struct ring_desc *rdesc;
    if (list_empty(&sqi.bd_list_free))
    return core::ptr::null_mut();
    rdesc = list_first_entry(&sqi.bd_list_free, struct ring_desc, list);
    list_move_tail(&rdesc.list, &sqi.bd_list_used);
    return rdesc;
    }
#[no_mangle]
unsafe extern "C" fn ring_desc_put(sqi: *mut pic32_sqi, rdesc: *mut ring_desc) {
    static void ring_desc_put(struct pic32_sqi *sqi, struct ring_desc *rdesc)
    {
    list_move(&rdesc.list, &sqi.bd_list_free);
    }
    static int pic32_sqi_one_transfer(struct pic32_sqi *sqi,
    struct spi_message *mesg,
    struct spi_transfer *xfer)
    {
    struct spi_device *spi = mesg.spi;
    struct scatterlist *sg, *sgl;
    struct ring_desc *rdesc;
    struct buf_desc *bd;
    int nents, i;
    u32 bd_ctrl;
    u32 nbits;
// Device selection
    bd_ctrl = spi_get_chipselect(spi, 0) << BD_DEVSEL_SHIFT;
// half-duplex: select transfer buffer, direction and lane
    if (xfer.rx_buf) {
    bd_ctrl |= BD_DATA_RECV;
    nbits = xfer.rx_nbits;
    sgl = xfer.rx_sg.sgl;
    nents = xfer.rx_sg.nents;
    } else {
    nbits = xfer.tx_nbits;
    sgl = xfer.tx_sg.sgl;
    nents = xfer.tx_sg.nents;
    }
    if (nbits & SPI_NBITS_QUAD)
    bd_ctrl |= BD_QUAD;
#[no_mangle]
pub unsafe extern "C" fn if(SPI_NBITS_DUAL: nbits &) -> else {
    else if (nbits & SPI_NBITS_DUAL)
    bd_ctrl |= BD_DUAL;
// LSB first
    if (spi.mode & SPI_LSB_FIRST)
    bd_ctrl |= BD_LSBF;
// ownership to hardware
    bd_ctrl |= BD_EN;
    for_each_sg(sgl, sg, nents, i) {
// get ring descriptor
    rdesc = ring_desc_get(sqi);
    if (!rdesc)
    break;
    bd = rdesc.bd;
// BD CTRL: length
    rdesc.xfer_len = sg_dma_len(sg);
    bd.bd_ctrl = bd_ctrl;
    bd.bd_ctrl |= rdesc.xfer_len;
// BD STAT
    bd.bd_status = 0;
// BD BUFFER ADDRESS
    bd.bd_addr = sg.dma_address;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_sqi_prepare_hardware(host: *mut spi_controller) -> c_int {
    static int pic32_sqi_prepare_hardware(struct spi_controller *host)
    {
    struct pic32_sqi *sqi = spi_controller_get_devdata(host);
// enable spi interface
    pic32_setbits(sqi.regs + PESQI_CONF_REG, PESQI_EN);
// enable spi clk
    pic32_setbits(sqi.regs + PESQI_CLK_CTRL_REG, PESQI_CLK_EN);
    return 0;
    }
    static bool pic32_sqi_can_dma(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *x)
    {
// Do DMA irrespective of transfer size
    return true;
    }
    static int pic32_sqi_one_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct spi_device *spi = msg.spi;
    struct ring_desc *rdesc, *next;
    struct spi_transfer *xfer;
    struct pic32_sqi *sqi;
    let mut ret: c_int = 0, mode;
    unsigned long time_left;
    u32 val;
    sqi = spi_controller_get_devdata(host);
    reinit_completion(&sqi.xfer_done);
    msg.actual_length = 0;
// We can't handle spi_transfer specific "speed_hz", "bits_per_word"
// and "delay_usecs". But spi_device specific speed and mode change
// can be handled at best during spi chip-select switch.
//
    if (sqi.cur_spi != spi) {
// set spi speed
    if (sqi.cur_speed != spi.max_speed_hz) {
    sqi.cur_speed = spi.max_speed_hz;
    ret = pic32_sqi_set_clk_rate(sqi, spi.max_speed_hz);
    if (ret)
    dev_warn(&spi.dev, "set_clk, %d\n", ret);
    }
// set spi mode
    mode = spi.mode & (SPI_MODE_3 | SPI_LSB_FIRST);
    if (sqi.cur_mode != mode) {
    val = readl(sqi.regs + PESQI_CONF_REG);
    val &= ~(PESQI_CPOL | PESQI_CPHA | PESQI_LSBF);
    if (mode & SPI_CPOL)
    val |= PESQI_CPOL;
    if (mode & SPI_LSB_FIRST)
    val |= PESQI_LSBF;
    val |= PESQI_CPHA;
    writel(val, sqi.regs + PESQI_CONF_REG);
    sqi.cur_mode = mode;
    }
    sqi.cur_spi = spi;
    }
// prepare hardware desc-list(BD) for transfer(s)
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
    ret = pic32_sqi_one_transfer(sqi, msg, xfer);
    if (ret) {
    dev_err(&spi.dev, "xfer %p err\n", xfer);
    goto xfer_out;
    }
    }
// BDs are prepared and chained. Now mark LAST_BD, CS_DEASSERT at last
// element of the list.
//
    rdesc = list_last_entry(&sqi.bd_list_used, struct ring_desc, list);
    rdesc.bd.bd_ctrl |= BD_LAST | BD_CS_DEASSERT |
    BD_LIFM | BD_PKT_INT_EN;
// set base address BD list for DMA engine
    rdesc = list_first_entry(&sqi.bd_list_used, struct ring_desc, list);
    writel(rdesc.bd_dma, sqi.regs + PESQI_BD_BASE_ADDR_REG);
// enable interrupt
    pic32_sqi_enable_int(sqi);
// enable DMA engine
    val = PESQI_DMA_EN | PESQI_POLL_EN | PESQI_BDP_START;
    writel(val, sqi.regs + PESQI_BD_CTRL_REG);
// wait for xfer completion
    time_left = wait_for_completion_timeout(&sqi.xfer_done, 5 * HZ);
    if (time_left == 0) {
    dev_err(&sqi.host.dev, "wait timedout/interrupted\n");
    ret = -ETIMEDOUT;
    msg.status = ret;
    } else {
// success
    msg.status = 0;
    ret = 0;
    }
// disable DMA
    writel(0, sqi.regs + PESQI_BD_CTRL_REG);
    pic32_sqi_disable_int(sqi);
    xfer_out:
    list_for_each_entry_safe_reverse(rdesc, next,
    &sqi.bd_list_used, list) {
// Update total byte transferred
    msg.actual_length += rdesc.xfer_len;
// release ring descr
    ring_desc_put(sqi, rdesc);
    }
    spi_finalize_current_message(spi.controller);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_sqi_unprepare_hardware(host: *mut spi_controller) -> c_int {
    static int pic32_sqi_unprepare_hardware(struct spi_controller *host)
    {
    struct pic32_sqi *sqi = spi_controller_get_devdata(host);
// disable clk
    pic32_clrbits(sqi.regs + PESQI_CLK_CTRL_REG, PESQI_CLK_EN);
// disable spi
    pic32_clrbits(sqi.regs + PESQI_CONF_REG, PESQI_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ring_desc_ring_alloc(sqi: *mut pic32_sqi) -> c_int {
    static int ring_desc_ring_alloc(struct pic32_sqi *sqi)
    {
    struct ring_desc *rdesc;
    struct buf_desc *bd;
    int i;
// allocate coherent DMAable memory for hardware buffer descriptors.
    sqi.bd = dma_alloc_coherent(&sqi.host.dev,
    sizeof(*bd) * PESQI_BD_COUNT,
    &sqi.bd_dma, GFP_KERNEL);
    if (!sqi.bd) {
    dev_err(&sqi.host.dev, "failed allocating dma buffer\n");
    return -ENOMEM;
    }
// allocate software ring descriptors
    sqi.ring = kzalloc_objs(*rdesc, PESQI_BD_COUNT);
    if (!sqi.ring) {
    dma_free_coherent(&sqi.host.dev,
    sizeof(*bd) * PESQI_BD_COUNT,
    sqi.bd, sqi.bd_dma);
    return -ENOMEM;
    }
    bd = (struct buf_desc *)sqi.bd;
    INIT_LIST_HEAD(&sqi.bd_list_free);
    INIT_LIST_HEAD(&sqi.bd_list_used);
// initialize ring-desc
    for (i = 0, rdesc = sqi.ring; i < PESQI_BD_COUNT; i++, rdesc++) {
    INIT_LIST_HEAD(&rdesc.list);
    rdesc.bd = &bd[i];
    rdesc.bd_dma = sqi.bd_dma + (void *)&bd[i] - (void *)bd;
    list_add_tail(&rdesc.list, &sqi.bd_list_free);
    }
// Prepare BD: chain to next BD(s)
    for (i = 0, rdesc = sqi.ring; i < PESQI_BD_COUNT - 1; i++)
    bd[i].bd_nextp = rdesc[i + 1].bd_dma;
    bd[PESQI_BD_COUNT - 1].bd_nextp = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ring_desc_ring_free(sqi: *mut pic32_sqi) {
    static void ring_desc_ring_free(struct pic32_sqi *sqi)
    {
    dma_free_coherent(&sqi.host.dev,
    sizeof(struct buf_desc) * PESQI_BD_COUNT,
    sqi.bd, sqi.bd_dma);
    kfree(sqi.ring);
    }
#[no_mangle]
unsafe extern "C" fn pic32_sqi_hw_init(sqi: *mut pic32_sqi) {
    static void pic32_sqi_hw_init(struct pic32_sqi *sqi)
    {
    unsigned long flags;
    u32 val;
// Soft-reset of PESQI controller triggers interrupt.
// We are not yet ready to handle them so disable CPU
// interrupt for the time being.
//
    local_irq_save(flags);
// assert soft-reset
    writel(PESQI_SOFT_RESET, sqi.regs + PESQI_CONF_REG);
// wait until clear
    readl_poll_timeout_atomic(sqi.regs + PESQI_CONF_REG, val,
    !(val & PESQI_SOFT_RESET), 1, 5000);
// disable all interrupts
    pic32_sqi_disable_int(sqi);
// Now it is safe to enable back CPU interrupt
    local_irq_restore(flags);
// tx and rx fifo interrupt threshold
    val = readl(sqi.regs + PESQI_CMD_THRES_REG);
    val &= ~(PESQI_TXTHR_MASK << PESQI_TXTHR_SHIFT);
    val &= ~(PESQI_RXTHR_MASK << PESQI_RXTHR_SHIFT);
    val |= (1U << PESQI_TXTHR_SHIFT) | (1U << PESQI_RXTHR_SHIFT);
    writel(val, sqi.regs + PESQI_CMD_THRES_REG);
    val = readl(sqi.regs + PESQI_INT_THRES_REG);
    val &= ~(PESQI_TXTHR_MASK << PESQI_TXTHR_SHIFT);
    val &= ~(PESQI_RXTHR_MASK << PESQI_RXTHR_SHIFT);
    val |= (1U << PESQI_TXTHR_SHIFT) | (1U << PESQI_RXTHR_SHIFT);
    writel(val, sqi.regs + PESQI_INT_THRES_REG);
// default configuration
    val = readl(sqi.regs + PESQI_CONF_REG);
// set mode: DMA
    val &= ~PESQI_MODE;
    val |= PESQI_MODE_DMA << PESQI_MODE_SHIFT;
    writel(val, sqi.regs + PESQI_CONF_REG);
// DATAEN - SQIID0-ID3
    val |= PESQI_QUAD_LANE << PESQI_LANES_SHIFT;
// burst/INCR4 enable
    val |= PESQI_BURST_EN;
// CSEN - all CS
    val |= 3U << PESQI_CSEN_SHIFT;
    writel(val, sqi.regs + PESQI_CONF_REG);
// write poll count
    writel(0, sqi.regs + PESQI_BD_POLL_CTRL_REG);
    sqi.cur_speed = 0;
    sqi.cur_mode = -1;
    }
#[no_mangle]
unsafe extern "C" fn pic32_sqi_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_sqi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct pic32_sqi *sqi;
    int ret;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*sqi));
    if (!host)
    return -ENOMEM;
    sqi = spi_controller_get_devdata(host);
    sqi.host = host;
    sqi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sqi.regs))
    return PTR_ERR(sqi.regs);
// irq
    sqi.irq = platform_get_irq(pdev, 0);
    if (sqi.irq < 0)
    return sqi.irq;
// clocks
    sqi.sys_clk = devm_clk_get_enabled(&pdev.dev, "reg_ck");
    if (IS_ERR(sqi.sys_clk)) {
    dev_err(&pdev.dev, "no sys_clk ?\n");
    return PTR_ERR(sqi.sys_clk);
    }
    sqi.base_clk = devm_clk_get_enabled(&pdev.dev, "spi_ck");
    if (IS_ERR(sqi.base_clk)) {
    dev_err(&pdev.dev, "no base clk ?\n");
    return PTR_ERR(sqi.base_clk);
    }
    init_completion(&sqi.xfer_done);
// initialize hardware
    pic32_sqi_hw_init(sqi);
// allocate buffers & descriptors
    ret = ring_desc_ring_alloc(sqi);
    if (ret) {
    dev_err(&pdev.dev, "ring alloc failed\n");
    return ret;
    }
// install irq handlers
    ret = request_irq(sqi.irq, pic32_sqi_isr, 0,
    dev_name(&pdev.dev), sqi);
    if (ret < 0) {
    dev_err(&pdev.dev, "request_irq(%d), failed\n", sqi.irq);
    goto err_free_ring;
    }
// register host
    host.num_chipselect	= 2;
    host.max_speed_hz	= clk_get_rate(sqi.base_clk);
    host.dma_alignment	= 32;
    host.max_dma_len	= PESQI_BD_BUF_LEN_MAX;
    host.dev.of_node	= pdev.dev.of_node;
    host.mode_bits		= SPI_MODE_3 | SPI_MODE_0 | SPI_TX_DUAL |
    SPI_RX_DUAL | SPI_TX_QUAD | SPI_RX_QUAD;
    host.flags		= SPI_CONTROLLER_HALF_DUPLEX;
    host.can_dma		= pic32_sqi_can_dma;
    host.bits_per_word_mask	= SPI_BPW_RANGE_MASK(8, 32);
    host.transfer_one_message	= pic32_sqi_one_message;
    host.prepare_transfer_hardware	= pic32_sqi_prepare_hardware;
    host.unprepare_transfer_hardware	= pic32_sqi_unprepare_hardware;
    ret = spi_register_controller(host);
    if (ret) {
    dev_err(&host.dev, "failed registering spi host\n");
    free_irq(sqi.irq, sqi);
    goto err_free_ring;
    }
    platform_set_drvdata(pdev, sqi);
    return 0;
    err_free_ring:
    ring_desc_ring_free(sqi);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_sqi_remove(pdev: *mut platform_device) {
    static void pic32_sqi_remove(struct platform_device *pdev)
    {
    struct pic32_sqi *sqi = platform_get_drvdata(pdev);
    spi_unregister_controller(sqi.host);
// release resources
    free_irq(sqi.irq, sqi);
    ring_desc_ring_free(sqi);
    }
    static const struct of_device_id pic32_sqi_of_ids[] = {
    {.compatible = "microchip,pic32mzda-sqi",},
    {},
    };
    MODULE_DEVICE_TABLE(of, pic32_sqi_of_ids);
    static struct platform_driver pic32_sqi_driver = {
    .driver = {
    .name = "sqi-pic32",
    .of_match_table = of_match_ptr(pic32_sqi_of_ids),
    },
    .probe = pic32_sqi_probe,
    .remove = pic32_sqi_remove,
    };
    module_platform_driver(pic32_sqi_driver);
    MODULE_AUTHOR("Purna Chandra Mandal <purna.mandal@microchip.com>");
    MODULE_DESCRIPTION("Microchip SPI driver for PIC32 SQI controller.");
    MODULE_LICENSE("GPL v2");
