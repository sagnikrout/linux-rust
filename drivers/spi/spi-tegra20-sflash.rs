//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-tegra20-sflash.c
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
// SPI driver for Nvidia's Tegra20 Serial Flash Controller.
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//

pub const SPI_COMMAND: c_uint = 0x000;

pub const SPI_CS_VAL_LOW: c_uint = 0x0;

pub const SPI_CS_HW: c_uint = 0x0;

    SPI_CS1_EN | SPI_CS0_EN)

pub const SPI_STATUS: c_uint = 0x004;

pub const SPI_RX_CMP: c_uint = 0x8;
pub const SPI_DMA_CTL: c_uint = 0x0C;

pub const SPI_TX_FIFO: c_uint = 0x10;
pub const SPI_RX_FIFO: c_uint = 0x20;

pub const MAX_CHIP_SELECT: c_int = 4;
pub const SPI_FIFO_DEPTH: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_sflash_data {
    pub dev: *mut device,
    pub host: *mut spi_controller,
    pub lock: spinlock_t,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub base: *mut void __iomem,
    pub irq: unsigned,
    pub cur_speed: u32,
    pub cur_spi: *mut spi_device,
    pub cur_pos: unsigned,
    pub cur_len: unsigned,
    pub bytes_per_word: unsigned,
    pub cur_direction: unsigned,
    pub curr_xfer_words: unsigned,
    pub cur_rx_pos: unsigned,
    pub cur_tx_pos: unsigned,
    pub tx_status: u32,
    pub rx_status: u32,
    pub status_reg: u32,
    pub def_command_reg: u32,
    pub command_reg: u32,
    pub dma_control_reg: u32,
    pub xfer_completion: completion,
    pub curr_xfer: *mut spi_transfer,
}

    static int tegra_sflash_runtime_suspend(struct device *dev);
    static int tegra_sflash_runtime_resume(struct device *dev);
    static inline u32 tegra_sflash_readl(struct tegra_sflash_data *tsd,
    unsigned long reg)
    {
    return readl(tsd.base + reg);
    }
    static inline void tegra_sflash_writel(struct tegra_sflash_data *tsd,
    u32 val, unsigned long reg)
    {
    writel(val, tsd.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn tegra_sflash_clear_status(tsd: *mut tegra_sflash_data) {
    static void tegra_sflash_clear_status(struct tegra_sflash_data *tsd)
    {
// Write 1 to clear status register
    tegra_sflash_writel(tsd, SPI_RDY | SPI_FIFO_ERROR, SPI_STATUS);
    }
    static unsigned tegra_sflash_calculate_curr_xfer_param(
    struct spi_device *spi, struct tegra_sflash_data *tsd,
    struct spi_transfer *t)
    {
    let mut remain_len: unsigned = t.len - tsd.cur_pos;
    unsigned max_word;
    tsd.bytes_per_word = DIV_ROUND_UP(t.bits_per_word, 8);
    max_word = remain_len / tsd.bytes_per_word;
    if (max_word > SPI_FIFO_DEPTH)
    max_word = SPI_FIFO_DEPTH;
    tsd.curr_xfer_words = max_word;
    return max_word;
    }
    static unsigned tegra_sflash_fill_tx_fifo_from_client_txbuf(
    struct tegra_sflash_data *tsd, struct spi_transfer *t)
    {
    unsigned nbytes;
    u32 status;
    let mut max_n_32bit: unsigned = tsd.curr_xfer_words;
    u8 *tx_buf = (u8 *)t.tx_buf + tsd.cur_tx_pos;
    if (max_n_32bit > SPI_FIFO_DEPTH)
    max_n_32bit = SPI_FIFO_DEPTH;
    nbytes = max_n_32bit * tsd.bytes_per_word;
    status = tegra_sflash_readl(tsd, SPI_STATUS);
    while (!(status & SPI_TXF_FULL)) {
    int i;
    let mut x: u32 = 0;
    for (i = 0; nbytes && (i < tsd.bytes_per_word);
    i++, nbytes--)
    x |= (u32)(*tx_buf++) << (i * 8);
    tegra_sflash_writel(tsd, x, SPI_TX_FIFO);
    if (!nbytes)
    break;
    status = tegra_sflash_readl(tsd, SPI_STATUS);
    }
    tsd.cur_tx_pos += max_n_32bit * tsd.bytes_per_word;
    return max_n_32bit;
    }
    static int tegra_sflash_read_rx_fifo_to_client_rxbuf(
    struct tegra_sflash_data *tsd, struct spi_transfer *t)
    {
    u32 status;
    let mut read_words: c_uint = 0;
    u8 *rx_buf = (u8 *)t.rx_buf + tsd.cur_rx_pos;
    status = tegra_sflash_readl(tsd, SPI_STATUS);
    while (!(status & SPI_RXF_EMPTY)) {
    int i;
    let mut x: u32 = tegra_sflash_readl(tsd, SPI_RX_FIFO);
    for (i = 0; (i < tsd.bytes_per_word); i++)
// rx_buf++ = (x >> (i*8)) & 0xFF;
    read_words++;
    status = tegra_sflash_readl(tsd, SPI_STATUS);
    }
    tsd.cur_rx_pos += read_words * tsd.bytes_per_word;
    return 0;
    }
    static int tegra_sflash_start_cpu_based_transfer(
    struct tegra_sflash_data *tsd, struct spi_transfer *t)
    {
    let mut val: u32 = 0;
    unsigned cur_words;
    if (tsd.cur_direction & DATA_DIR_TX)
    val |= SPI_IE_TXC;
    if (tsd.cur_direction & DATA_DIR_RX)
    val |= SPI_IE_RXC;
    tegra_sflash_writel(tsd, val, SPI_DMA_CTL);
    tsd.dma_control_reg = val;
    if (tsd.cur_direction & DATA_DIR_TX)
    cur_words = tegra_sflash_fill_tx_fifo_from_client_txbuf(tsd, t);
    else
    cur_words = tsd.curr_xfer_words;
    val |= SPI_DMA_BLK_COUNT(cur_words);
    tegra_sflash_writel(tsd, val, SPI_DMA_CTL);
    tsd.dma_control_reg = val;
    val |= SPI_DMA_EN;
    tegra_sflash_writel(tsd, val, SPI_DMA_CTL);
    return 0;
    }
    static int tegra_sflash_start_transfer_one(struct spi_device *spi,
    struct spi_transfer *t, bool is_first_of_msg,
    bool is_single_xfer)
    {
    struct tegra_sflash_data *tsd = spi_controller_get_devdata(spi.controller);
    u32 speed;
    u32 command;
    speed = t.speed_hz;
    if (speed != tsd.cur_speed) {
    clk_set_rate(tsd.clk, speed);
    tsd.cur_speed = speed;
    }
    tsd.cur_spi = spi;
    tsd.cur_pos = 0;
    tsd.cur_rx_pos = 0;
    tsd.cur_tx_pos = 0;
    tsd.curr_xfer = t;
    tegra_sflash_calculate_curr_xfer_param(spi, tsd, t);
    if (is_first_of_msg) {
    command = tsd.def_command_reg;
    command |= SPI_BIT_LENGTH(t.bits_per_word - 1);
    command |= SPI_CS_VAL_HIGH;
    command &= ~SPI_MODES;
    if (spi.mode & SPI_CPHA)
    command |= SPI_CK_SDA_FALLING;
    if (spi.mode & SPI_CPOL)
    command |= SPI_ACTIVE_SCLK_DRIVE_HIGH;
    else
    command |= SPI_ACTIVE_SCLK_DRIVE_LOW;
    command |= SPI_CS0_EN << spi_get_chipselect(spi, 0);
    } else {
    command = tsd.command_reg;
    command &= ~SPI_BIT_LENGTH(~0);
    command |= SPI_BIT_LENGTH(t.bits_per_word - 1);
    command &= ~(SPI_RX_EN | SPI_TX_EN);
    }
    tsd.cur_direction = 0;
    if (t.rx_buf) {
    command |= SPI_RX_EN;
    tsd.cur_direction |= DATA_DIR_RX;
    }
    if (t.tx_buf) {
    command |= SPI_TX_EN;
    tsd.cur_direction |= DATA_DIR_TX;
    }
    tegra_sflash_writel(tsd, command, SPI_COMMAND);
    tsd.command_reg = command;
    return tegra_sflash_start_cpu_based_transfer(tsd, t);
    }
    static int tegra_sflash_transfer_one_message(struct spi_controller *host,
    struct spi_message *msg)
    {
    let mut is_first_msg: bool = true;
    int single_xfer;
    struct tegra_sflash_data *tsd = spi_controller_get_devdata(host);
    struct spi_transfer *xfer;
    struct spi_device *spi = msg.spi;
    int ret;
    msg.status = 0;
    msg.actual_length = 0;
    single_xfer = list_is_singular(&msg.transfers);
    list_for_each_entry(xfer, &msg.transfers, transfer_list) {
    reinit_completion(&tsd.xfer_completion);
    ret = tegra_sflash_start_transfer_one(spi, xfer,
    is_first_msg, single_xfer);
    if (ret < 0) {
    dev_err(tsd.dev,
    "spi can not start transfer, err %d\n", ret);
    goto exit;
    }
    is_first_msg = false;
    ret = wait_for_completion_timeout(&tsd.xfer_completion,
    SPI_DMA_TIMEOUT);
    if (WARN_ON(ret == 0)) {
    dev_err(tsd.dev,
    "spi transfer timeout, err %d\n", ret);
    ret = -EIO;
    goto exit;
    }
    if (tsd.tx_status ||  tsd.rx_status) {
    dev_err(tsd.dev, "Error in Transfer\n");
    ret = -EIO;
    goto exit;
    }
    msg.actual_length += xfer.len;
    if (xfer.cs_change && xfer.delay.value) {
    tegra_sflash_writel(tsd, tsd.def_command_reg,
    SPI_COMMAND);
    spi_transfer_delay_exec(xfer);
    }
    }
    ret = 0;
    exit:
    tegra_sflash_writel(tsd, tsd.def_command_reg, SPI_COMMAND);
    msg.status = ret;
    spi_finalize_current_message(host);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn handle_cpu_based_xfer(tsd: *mut tegra_sflash_data) -> irqreturn_t {
    static irqreturn_t handle_cpu_based_xfer(struct tegra_sflash_data *tsd)
    {
    struct spi_transfer *t = tsd.curr_xfer;
    spin_lock(&tsd.lock);
    if (tsd.tx_status || tsd.rx_status || (tsd.status_reg & SPI_BSY)) {
    dev_err(tsd.dev,
    "CpuXfer ERROR bit set 0x%x\n", tsd.status_reg);
    dev_err(tsd.dev,
    "CpuXfer 0x%08x:0x%08x\n", tsd.command_reg,
    tsd.dma_control_reg);
    reset_control_assert(tsd.rst);
    udelay(2);
    reset_control_deassert(tsd.rst);
    complete(&tsd.xfer_completion);
    goto exit;
    }
    if (tsd.cur_direction & DATA_DIR_RX)
    tegra_sflash_read_rx_fifo_to_client_rxbuf(tsd, t);
    if (tsd.cur_direction & DATA_DIR_TX)
    tsd.cur_pos = tsd.cur_tx_pos;
    else
    tsd.cur_pos = tsd.cur_rx_pos;
    if (tsd.cur_pos == t.len) {
    complete(&tsd.xfer_completion);
    goto exit;
    }
    tegra_sflash_calculate_curr_xfer_param(tsd.cur_spi, tsd, t);
    tegra_sflash_start_cpu_based_transfer(tsd, t);
    exit:
    spin_unlock(&tsd.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tegra_sflash_isr(irq: c_int, context_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_sflash_isr(int irq, void *context_data)
    {
    struct tegra_sflash_data *tsd = context_data;
    tsd.status_reg = tegra_sflash_readl(tsd, SPI_STATUS);
    if (tsd.cur_direction & DATA_DIR_TX)
    tsd.tx_status = tsd.status_reg & SPI_TX_OVF;
    if (tsd.cur_direction & DATA_DIR_RX)
    tsd.rx_status = tsd.status_reg & SPI_RX_UNF;
    tegra_sflash_clear_status(tsd);
    return handle_cpu_based_xfer(tsd);
    }
    static const struct of_device_id tegra_sflash_of_match[] = {
    { .compatible = "nvidia,tegra20-sflash", },
    {}
    };
    MODULE_DEVICE_TABLE(of, tegra_sflash_of_match);
#[no_mangle]
unsafe extern "C" fn tegra_sflash_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_sflash_probe(struct platform_device *pdev)
    {
    struct spi_controller	*host;
    struct tegra_sflash_data	*tsd;
    int ret;
    const struct of_device_id *match;
    match = of_match_device(tegra_sflash_of_match, &pdev.dev);
    if (!match) {
    dev_err(&pdev.dev, "Error: No device match found\n");
    return -ENODEV;
    }
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*tsd));
    if (!host)
    return -ENOMEM;
// the spi->mode bits understood by this driver:
    host.mode_bits = SPI_CPOL | SPI_CPHA;
    host.transfer_one_message = tegra_sflash_transfer_one_message;
    host.auto_runtime_pm = true;
    host.num_chipselect = MAX_CHIP_SELECT;
    platform_set_drvdata(pdev, host);
    tsd = spi_controller_get_devdata(host);
    tsd.host = host;
    tsd.dev = &pdev.dev;
    spin_lock_init(&tsd.lock);
    if (of_property_read_u32(tsd.dev.of_node, "spi-max-frequency",
    &host.max_speed_hz))
    host.max_speed_hz = 25000000; /* 25MHz */
    tsd.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(tsd.base))
    return PTR_ERR(tsd.base);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    tsd.irq = ret;
    ret = request_irq(tsd.irq, tegra_sflash_isr, 0,
    dev_name(&pdev.dev), tsd);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to register ISR for IRQ %d\n",
    tsd.irq);
    return ret;
    }
    tsd.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(tsd.clk)) {
    dev_err(&pdev.dev, "can not get clock\n");
    ret = PTR_ERR(tsd.clk);
    goto exit_free_irq;
    }
    tsd.rst = devm_reset_control_get_exclusive(&pdev.dev, "spi");
    if (IS_ERR(tsd.rst)) {
    dev_err(&pdev.dev, "can not get reset\n");
    ret = PTR_ERR(tsd.rst);
    goto exit_free_irq;
    }
    init_completion(&tsd.xfer_completion);
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = tegra_sflash_runtime_resume(&pdev.dev);
    if (ret)
    goto exit_pm_disable;
    }
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret < 0) {
    dev_err(&pdev.dev, "pm runtime get failed, e = %d\n", ret);
    goto exit_pm_disable;
    }
// Reset controller
    reset_control_assert(tsd.rst);
    udelay(2);
    reset_control_deassert(tsd.rst);
    tsd.def_command_reg  = SPI_M_S | SPI_CS_SW;
    tegra_sflash_writel(tsd, tsd.def_command_reg, SPI_COMMAND);
    pm_runtime_put(&pdev.dev);
    ret = spi_register_controller(host);
    if (ret < 0) {
    dev_err(&pdev.dev, "can not register to host err %d\n", ret);
    goto exit_pm_disable;
    }
    return ret;
    exit_pm_disable:
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    tegra_sflash_runtime_suspend(&pdev.dev);
    exit_free_irq:
    free_irq(tsd.irq, tsd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_sflash_remove(pdev: *mut platform_device) {
    static void tegra_sflash_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct tegra_sflash_data	*tsd = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
    free_irq(tsd.irq, tsd);
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    tegra_sflash_runtime_suspend(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_sflash_suspend(dev: *mut device) -> c_int {
    static int tegra_sflash_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    return spi_controller_suspend(host);
    }
#[no_mangle]
unsafe extern "C" fn tegra_sflash_resume(dev: *mut device) -> c_int {
    static int tegra_sflash_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct tegra_sflash_data *tsd = spi_controller_get_devdata(host);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0) {
    dev_err(dev, "pm runtime failed, e = %d\n", ret);
    return ret;
    }
    tegra_sflash_writel(tsd, tsd.command_reg, SPI_COMMAND);
    pm_runtime_put(dev);
    return spi_controller_resume(host);
    }
#[no_mangle]
unsafe extern "C" fn tegra_sflash_runtime_suspend(dev: *mut device) -> c_int {
    static int tegra_sflash_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct tegra_sflash_data *tsd = spi_controller_get_devdata(host);
// Flush all write which are in PPSB queue by reading back
    tegra_sflash_readl(tsd, SPI_COMMAND);
    clk_disable_unprepare(tsd.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_sflash_runtime_resume(dev: *mut device) -> c_int {
    static int tegra_sflash_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct tegra_sflash_data *tsd = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(tsd.clk);
    if (ret < 0) {
    dev_err(tsd.dev, "clk_prepare failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct dev_pm_ops slink_pm_ops = {
    RUNTIME_PM_OPS(tegra_sflash_runtime_suspend,
    tegra_sflash_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(tegra_sflash_suspend, tegra_sflash_resume)
    };
    static struct platform_driver tegra_sflash_driver = {
    .driver = {
    .name		= "spi-tegra-sflash",
    .pm		= pm_ptr(&slink_pm_ops),
    .of_match_table	= tegra_sflash_of_match,
    },
    .probe =	tegra_sflash_probe,
    .remove =	tegra_sflash_remove,
    };
    module_platform_driver(tegra_sflash_driver);
    MODULE_ALIAS("platform:spi-tegra-sflash");
    MODULE_DESCRIPTION("NVIDIA Tegra20 Serial Flash Controller Driver");
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
    MODULE_LICENSE("GPL v2");
