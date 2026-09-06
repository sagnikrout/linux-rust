//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-hisi-kunpeng.c
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
// HiSilicon SPI Controller Driver for Kunpeng SoCs
//
// Copyright (c) 2021 HiSilicon Technologies Co., Ltd.
// Author: Jay Fang <f.fangjian@huawei.com>
//
// This code is based on spi-dw-core.c.

// Register offsets
pub const HISI_SPI_CSCR: c_uint = 0x00	/* cs control register */;
pub const HISI_SPI_CR: c_uint = 0x04	/* spi common control register */;
pub const HISI_SPI_ENR: c_uint = 0x08	/* spi enable register */;
pub const HISI_SPI_FIFOC: c_uint = 0x0c	/* fifo level control register */;
pub const HISI_SPI_IMR: c_uint = 0x10	/* interrupt mask register */;
pub const HISI_SPI_DIN: c_uint = 0x14	/* data in register */;
pub const HISI_SPI_DOUT: c_uint = 0x18	/* data out register */;
pub const HISI_SPI_SR: c_uint = 0x1c	/* status register */;
pub const HISI_SPI_RISR: c_uint = 0x20	/* raw interrupt status register */;
pub const HISI_SPI_ISR: c_uint = 0x24	/* interrupt status register */;
pub const HISI_SPI_ICR: c_uint = 0x28	/* interrupt clear register */;
pub const HISI_SPI_VERSION: c_uint = 0xe0	/* version register */;
// Bit fields in HISI_SPI_CR

// Bit fields in HISI_SPI_FIFOC

// Bit fields in HISI_SPI_IMR, 4 bits

// Bit fields in HISI_SPI_SR, 5 bits

// Bit fields in HISI_SPI_ISR, 4 bits

// Bit fields in HISI_SPI_ICR, 2 bits

pub const DIV_POST_MAX: c_uint = 0xFF;
pub const DIV_POST_MIN: c_uint = 0x00;
pub const DIV_PRE_MAX: c_uint = 0xFE;
pub const DIV_PRE_MIN: c_uint = 0x02;

pub const DEFAULT_NUM_CS: c_int = 1;

    enum hisi_spi_rx_level_trig {
    HISI_SPI_RX_1,
    HISI_SPI_RX_4,
    HISI_SPI_RX_8,
    HISI_SPI_RX_16,
    HISI_SPI_RX_32,
    HISI_SPI_RX_64,
    HISI_SPI_RX_128
    };
    enum hisi_spi_tx_level_trig {
    HISI_SPI_TX_1_OR_LESS,
    HISI_SPI_TX_4_OR_LESS,
    HISI_SPI_TX_8_OR_LESS,
    HISI_SPI_TX_16_OR_LESS,
    HISI_SPI_TX_32_OR_LESS,
    HISI_SPI_TX_64_OR_LESS,
    HISI_SPI_TX_128_OR_LESS
    };
    enum hisi_spi_frame_n_bytes {
    HISI_SPI_N_BYTES_NULL,
    HISI_SPI_N_BYTES_U8,
    HISI_SPI_N_BYTES_U16,
    HISI_SPI_N_BYTES_U32 = 4
    };
// Slave spi_dev related
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_chip_data {
    pub cr: u32,
    pub /: *mut *mut u32 speed_hz; / baud rate,
    pub /: *mut *mut u16 clk_div; / baud rate divider,
// clk_div = (1 + div_post) * div_pre
    pub /: *mut *mut u8 div_post; / value from 0 to 255,
    pub /: *mut *mut u8 div_pre; / value from 2 to 254 (even only!),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_spi {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub /: *mut *mut u32 fifo_len; / depth of the FIFO buffer,
// Current message transfer state info
    pub tx: *const c_void,
    pub tx_len: c_uint,
    pub rx: *mut c_void,
    pub rx_len: c_uint,
    pub /: *mut *mut u8 n_bytes; / current is a 1/2/4 bytes op,
    pub debugfs: *mut dentry,
    pub regset: debugfs_regset32,
}

    {					\
    .name = _name,			\
    .offset = _off,			\
    }
    static const struct debugfs_reg32 hisi_spi_regs[] = {
    HISI_SPI_DBGFS_REG("CSCR", HISI_SPI_CSCR),
    HISI_SPI_DBGFS_REG("CR", HISI_SPI_CR),
    HISI_SPI_DBGFS_REG("ENR", HISI_SPI_ENR),
    HISI_SPI_DBGFS_REG("FIFOC", HISI_SPI_FIFOC),
    HISI_SPI_DBGFS_REG("IMR", HISI_SPI_IMR),
    HISI_SPI_DBGFS_REG("SR", HISI_SPI_SR),
    HISI_SPI_DBGFS_REG("RISR", HISI_SPI_RISR),
    HISI_SPI_DBGFS_REG("ISR", HISI_SPI_ISR),
    HISI_SPI_DBGFS_REG("ICR", HISI_SPI_ICR),
    HISI_SPI_DBGFS_REG("VERSION", HISI_SPI_VERSION),
    };
#[no_mangle]
unsafe extern "C" fn hisi_spi_debugfs_init(hs: *mut hisi_spi) -> c_int {
    static int hisi_spi_debugfs_init(struct hisi_spi *hs)
    {
    char name[32];
    struct spi_controller *host = dev_get_drvdata(hs.dev);
    snprintf(name, 32, "hisi_spi%d", host.bus_num);
    hs.debugfs = debugfs_create_dir(name, core::ptr::null_mut());
    if (IS_ERR(hs.debugfs))
    return -ENOMEM;
    hs.regset.regs = hisi_spi_regs;
    hs.regset.nregs = ARRAY_SIZE(hisi_spi_regs);
    hs.regset.base = hs.regs;
    debugfs_create_regset32("registers", 0400, hs.debugfs, &hs.regset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_busy(hs: *mut hisi_spi) -> u32 {
    static u32 hisi_spi_busy(struct hisi_spi *hs)
    {
    return readl(hs.regs + HISI_SPI_SR) & SR_BUSY;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_rx_not_empty(hs: *mut hisi_spi) -> u32 {
    static u32 hisi_spi_rx_not_empty(struct hisi_spi *hs)
    {
    return readl(hs.regs + HISI_SPI_SR) & SR_RXNE;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_tx_not_full(hs: *mut hisi_spi) -> u32 {
    static u32 hisi_spi_tx_not_full(struct hisi_spi *hs)
    {
    return readl(hs.regs + HISI_SPI_SR) & SR_TXNF;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_flush_fifo(hs: *mut hisi_spi) {
    static void hisi_spi_flush_fifo(struct hisi_spi *hs)
    {
    let mut limit: c_ulong = loops_per_jiffy << 1;
    do {
    let mut inner_limit: c_ulong = loops_per_jiffy;
    while (hisi_spi_rx_not_empty(hs) && --inner_limit) {
    readl(hs.regs + HISI_SPI_DOUT);
    cpu_relax();
    }
    if (!inner_limit) {
    dev_warn_ratelimited(hs.dev, "RX FIFO flush timeout\n");
    break;
    }
    } while (hisi_spi_busy(hs) && --limit);
    if (!limit)
    dev_warn_ratelimited(hs.dev, "SPI busy timeout\n");
    }
// Disable the controller and all interrupts
#[no_mangle]
unsafe extern "C" fn hisi_spi_disable(hs: *mut hisi_spi) {
    static void hisi_spi_disable(struct hisi_spi *hs)
    {
    writel(0, hs.regs + HISI_SPI_ENR);
    writel(IMR_MASK, hs.regs + HISI_SPI_IMR);
    writel(ICR_MASK, hs.regs + HISI_SPI_ICR);
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_n_bytes(transfer: *mut spi_transfer) -> u8 {
    static u8 hisi_spi_n_bytes(struct spi_transfer *transfer)
    {
    if (transfer.bits_per_word <= 8)
    return HISI_SPI_N_BYTES_U8;
#[no_mangle]
pub unsafe extern "C" fn if(16: transfer->bits_per_word <=) -> else {
    else if (transfer.bits_per_word <= 16)
    return HISI_SPI_N_BYTES_U16;
    else
    return HISI_SPI_N_BYTES_U32;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_reader(hs: *mut hisi_spi) {
    static void hisi_spi_reader(struct hisi_spi *hs)
    {
    let mut max: u32 = min_t(u32, hs.rx_len, hs.fifo_len);
    u32 rxw;
    while (hisi_spi_rx_not_empty(hs) && max--) {
    rxw = readl(hs.regs + HISI_SPI_DOUT);
// Check the transfer's original "rx" is not null
    if (hs.rx) {
    switch (hs.n_bytes) {
    case HISI_SPI_N_BYTES_U8:
// (u8 *)(hs->rx) = rxw;
    break;
    case HISI_SPI_N_BYTES_U16:
// (u16 *)(hs->rx) = rxw;
    break;
    case HISI_SPI_N_BYTES_U32:
// (u32 *)(hs->rx) = rxw;
    break;
    }
    hs.rx += hs.n_bytes;
    }
    --hs.rx_len;
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_writer(hs: *mut hisi_spi) {
    static void hisi_spi_writer(struct hisi_spi *hs)
    {
    let mut max: u32 = min_t(u32, hs.tx_len, hs.fifo_len);
    let mut txw: u32 = 0;
    while (hisi_spi_tx_not_full(hs) && max--) {
// Check the transfer's original "tx" is not null
    if (hs.tx) {
    switch (hs.n_bytes) {
    case HISI_SPI_N_BYTES_U8:
    txw = *(u8 *)(hs.tx);
    break;
    case HISI_SPI_N_BYTES_U16:
    txw = *(u16 *)(hs.tx);
    break;
    case HISI_SPI_N_BYTES_U32:
    txw = *(u32 *)(hs.tx);
    break;
    }
    hs.tx += hs.n_bytes;
    }
    writel(txw, hs.regs + HISI_SPI_DIN);
    --hs.tx_len;
    }
    }
#[no_mangle]
unsafe extern "C" fn __hisi_calc_div_reg(chip: *mut hisi_chip_data) {
    static void __hisi_calc_div_reg(struct hisi_chip_data *chip)
    {
    chip.div_pre = DIV_PRE_MAX;
    while (chip.div_pre >= DIV_PRE_MIN) {
    if (chip.clk_div % chip.div_pre == 0)
    break;
    chip.div_pre -= 2;
    }
    if (chip.div_pre > chip.clk_div)
    chip.div_pre = chip.clk_div;
    chip.div_post = (chip.clk_div / chip.div_pre) - 1;
    }
    static u32 hisi_calc_effective_speed(struct spi_controller *host,
    struct hisi_chip_data *chip, u32 speed_hz)
    {
    u32 effective_speed;
// Note clock divider doesn't support odd numbers
    chip.clk_div = DIV_ROUND_UP(host.max_speed_hz, speed_hz) + 1;
    chip.clk_div &= 0xfffe;
    if (chip.clk_div > CLK_DIV_MAX)
    chip.clk_div = CLK_DIV_MAX;
    effective_speed = host.max_speed_hz / chip.clk_div;
    if (chip.speed_hz != effective_speed) {
    __hisi_calc_div_reg(chip);
    chip.speed_hz = effective_speed;
    }
    return effective_speed;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_prepare_cr(spi: *mut spi_device) -> u32 {
    static u32 hisi_spi_prepare_cr(struct spi_device *spi)
    {
    let mut cr: u32 = FIELD_PREP(CR_SPD_MODE_MASK, 1);
    cr |= FIELD_PREP(CR_CPHA_MASK, (spi.mode & SPI_CPHA) ? 1 : 0);
    cr |= FIELD_PREP(CR_CPOL_MASK, (spi.mode & SPI_CPOL) ? 1 : 0);
    cr |= FIELD_PREP(CR_LOOP_MASK, (spi.mode & SPI_LOOP) ? 1 : 0);
    return cr;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_hw_init(hs: *mut hisi_spi) {
    static void hisi_spi_hw_init(struct hisi_spi *hs)
    {
    hisi_spi_disable(hs);
// FIFO default config
    writel(FIELD_PREP(FIFOC_TX_MASK, HISI_SPI_TX_64_OR_LESS) |
    FIELD_PREP(FIFOC_RX_MASK, HISI_SPI_RX_16),
    hs.regs + HISI_SPI_FIFOC);
    hs.fifo_len = 256;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hisi_spi_irq(int irq, void *dev_id)
    {
    struct spi_controller *host = dev_id;
    struct hisi_spi *hs = spi_controller_get_devdata(host);
    let mut irq_status: u32 = readl(hs.regs + HISI_SPI_ISR) & ISR_MASK;
    if (!irq_status)
    return IRQ_NONE;
    if (!host.cur_msg)
    return IRQ_HANDLED;
// Error handling
    if (irq_status & ISR_RXOF) {
    dev_err(hs.dev, "interrupt_transfer: fifo overflow\n");
    host.cur_msg.status = -EIO;
    goto finalize_transfer;
    }
//
// Read data from the Rx FIFO every time. If there is
// nothing left to receive, finalize the transfer.
//
    hisi_spi_reader(hs);
    if (!hs.rx_len)
    goto finalize_transfer;
// Send data out when Tx FIFO IRQ triggered
    if (irq_status & ISR_TX)
    hisi_spi_writer(hs);
    return IRQ_HANDLED;
    finalize_transfer:
    hisi_spi_disable(hs);
    spi_finalize_current_transfer(host);
    return IRQ_HANDLED;
    }
    static int hisi_spi_transfer_one(struct spi_controller *host,
    struct spi_device *spi, struct spi_transfer *transfer)
    {
    struct hisi_spi *hs = spi_controller_get_devdata(host);
    struct hisi_chip_data *chip = spi_get_ctldata(spi);
    let mut cr: u32 = chip.cr;
// Update per transfer options for speed and bpw
    transfer.effective_speed_hz =
    hisi_calc_effective_speed(host, chip, transfer.speed_hz);
    cr |= FIELD_PREP(CR_DIV_PRE_MASK, chip.div_pre);
    cr |= FIELD_PREP(CR_DIV_POST_MASK, chip.div_post);
    cr |= FIELD_PREP(CR_BPW_MASK, transfer.bits_per_word - 1);
    writel(cr, hs.regs + HISI_SPI_CR);
    hisi_spi_flush_fifo(hs);
    hs.n_bytes = hisi_spi_n_bytes(transfer);
    hs.tx = transfer.tx_buf;
    hs.tx_len = transfer.len / hs.n_bytes;
    hs.rx = transfer.rx_buf;
    hs.rx_len = hs.tx_len;
//
// Ensure that the transfer data above has been updated
// before the interrupt to start.
//
    smp_mb();
// Enable all interrupts and the controller
    writel(~(u32)IMR_MASK, hs.regs + HISI_SPI_IMR);
    writel(1, hs.regs + HISI_SPI_ENR);
    return 1;
    }
    static void hisi_spi_handle_err(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct hisi_spi *hs = spi_controller_get_devdata(host);
    hisi_spi_disable(hs);
//
// Wait for interrupt handler that is
// already in timeout to complete.
//
    msleep(HISI_SPI_WAIT_TIMEOUT_MS);
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_setup(spi: *mut spi_device) -> c_int {
    static int hisi_spi_setup(struct spi_device *spi)
    {
    struct hisi_chip_data *chip;
// Only alloc on first setup
    chip = spi_get_ctldata(spi);
    if (!chip) {
    chip = kzalloc_obj(*chip);
    if (!chip)
    return -ENOMEM;
    spi_set_ctldata(spi, chip);
    }
    chip.cr = hisi_spi_prepare_cr(spi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_cleanup(spi: *mut spi_device) {
    static void hisi_spi_cleanup(struct spi_device *spi)
    {
    struct hisi_chip_data *chip = spi_get_ctldata(spi);
    kfree(chip);
    spi_set_ctldata(spi, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_spi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *host;
    struct hisi_spi *hs;
    u32 num_cs;
    int ret, irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    host = devm_spi_alloc_host(dev, sizeof(*hs));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    hs = spi_controller_get_devdata(host);
    hs.dev = dev;
    hs.irq = irq;
    hs.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hs.regs))
    return PTR_ERR(hs.regs);
// Specify maximum SPI clocking speed (host only) by firmware
    ret = device_property_read_u32(dev, "spi-max-frequency",
    &host.max_speed_hz);
    if (ret) {
    dev_err(dev, "failed to get max SPI clocking speed, ret=%d\n",
    ret);
    return -EINVAL;
    }
    if (host.max_speed_hz == 0)
    return dev_err_probe(dev, -EINVAL, "spi-max-frequency can't be 0\n");
    ret = device_property_read_u32(dev, "num-cs", &num_cs);
    if (ret)
    host.num_chipselect = DEFAULT_NUM_CS;
    else
    host.num_chipselect = num_cs;
    host.use_gpio_descriptors = true;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH | SPI_LOOP;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(4, 32);
    host.bus_num = pdev.id;
    host.setup = hisi_spi_setup;
    host.cleanup = hisi_spi_cleanup;
    host.transfer_one = hisi_spi_transfer_one;
    host.handle_err = hisi_spi_handle_err;
    host.min_speed_hz = DIV_ROUND_UP(host.max_speed_hz, CLK_DIV_MAX);
    hisi_spi_hw_init(hs);
    ret = devm_request_irq(dev, hs.irq, hisi_spi_irq, 0, dev_name(dev),
    host);
    if (ret < 0) {
    dev_err(dev, "failed to get IRQ=%d, ret=%d\n", hs.irq, ret);
    return ret;
    }
    ret = spi_register_controller(host);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register spi host\n");
    if (hisi_spi_debugfs_init(hs))
    dev_info(dev, "failed to create debugfs dir\n");
    dev_info(dev, "hw version:0x%x max-freq:%u kHz\n",
    readl(hs.regs + HISI_SPI_VERSION),
    host.max_speed_hz / 1000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_remove(pdev: *mut platform_device) {
    static void hisi_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct hisi_spi *hs = spi_controller_get_devdata(host);
    debugfs_remove_recursive(hs.debugfs);
    spi_unregister_controller(host);
    }
    static const struct acpi_device_id hisi_spi_acpi_match[] = {
    {"HISI03E1", 0},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, hisi_spi_acpi_match);
    static struct platform_driver hisi_spi_driver = {
    .probe		= hisi_spi_probe,
    .remove		= hisi_spi_remove,
    .driver		= {
    .name	= "hisi-kunpeng-spi",
    .acpi_match_table = hisi_spi_acpi_match,
    },
    };
    module_platform_driver(hisi_spi_driver);
    MODULE_AUTHOR("Jay Fang <f.fangjian@huawei.com>");
    MODULE_DESCRIPTION("HiSilicon SPI Controller Driver for Kunpeng SoCs");
    MODULE_LICENSE("GPL v2");
