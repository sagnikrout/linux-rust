//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-bcmbca-hsspi.c
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
// Broadcom BCMBCA High Speed SPI Controller driver
//
// Copyright 2000-2010 Broadcom Corporation
// Copyright 2012-2013 Jonas Gorski <jonas.gorski@gmail.com>
// Copyright 2019-2022 Broadcom Ltd
//

pub const HSSPI_GLOBAL_CTRL_REG: c_uint = 0x0;
pub const GLOBAL_CTRL_CS_POLARITY_SHIFT: c_int = 0;
pub const GLOBAL_CTRL_CS_POLARITY_MASK: c_uint = 0x000000ff;
pub const GLOBAL_CTRL_PLL_CLK_CTRL_SHIFT: c_int = 8;
pub const GLOBAL_CTRL_PLL_CLK_CTRL_MASK: c_uint = 0x0000ff00;

pub const HSSPI_GLOBAL_EXT_TRIGGER_REG: c_uint = 0x4;
pub const HSSPI_INT_STATUS_REG: c_uint = 0x8;
pub const HSSPI_INT_STATUS_MASKED_REG: c_uint = 0xc;
pub const HSSPI_INT_MASK_REG: c_uint = 0x10;

pub const HSSPI_INT_CLEAR_ALL: c_uint = 0xff001f1f;

pub const PINGPONG_CMD_COMMAND_MASK: c_uint = 0xf;
pub const PINGPONG_COMMAND_NOOP: c_int = 0;
pub const PINGPONG_COMMAND_START_NOW: c_int = 1;
pub const PINGPONG_COMMAND_START_TRIGGER: c_int = 2;
pub const PINGPONG_COMMAND_HALT: c_int = 3;
pub const PINGPONG_COMMAND_FLUSH: c_int = 4;
pub const PINGPONG_CMD_PROFILE_SHIFT: c_int = 8;
pub const PINGPONG_CMD_SS_SHIFT: c_int = 12;

pub const CLK_CTRL_FREQ_CTRL_MASK: c_uint = 0x0000ffff;

pub const MODE_CTRL_MULTIDATA_RD_STRT_SHIFT: c_int = 8;
pub const MODE_CTRL_MULTIDATA_WR_STRT_SHIFT: c_int = 12;
pub const MODE_CTRL_MULTIDATA_RD_SIZE_SHIFT: c_int = 16;
pub const MODE_CTRL_MULTIDATA_WR_SIZE_SHIFT: c_int = 18;

pub const MODE_CTRL_PREPENDBYTE_CNT_SHIFT: c_int = 24;

pub const HSSPI_OP_CODE_SHIFT: c_int = 13;

pub const HSSPI_BUFFER_LEN: c_int = 512;
pub const HSSPI_OPCODE_LEN: c_int = 2;
pub const HSSPI_MAX_PREPEND_LEN: c_int = 15;
pub const HSSPI_MAX_SYNC_CLOCK: c_int = 30000000;
pub const HSSPI_SPI_MAX_CS: c_int = 8;

pub const HSSPI_POLL_STATUS_TIMEOUT_MS: c_int = 100;
pub const HSSPI_WAIT_MODE_POLLING: c_int = 0;
pub const HSSPI_WAIT_MODE_INTR: c_int = 1;

pub const SPIM_CTRL_CS_OVERRIDE_SEL_SHIFT: c_int = 0;
pub const SPIM_CTRL_CS_OVERRIDE_SEL_MASK: c_uint = 0xff;
pub const SPIM_CTRL_CS_OVERRIDE_VAL_SHIFT: c_int = 8;
pub const SPIM_CTRL_CS_OVERRIDE_VAL_MASK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmbca_hsspi {
    pub done: completion,
    pub bus_mutex: mutex,
    pub msg_mutex: mutex,
    pub pdev: *mut platform_device,
    pub clk: *mut clk,
    pub pll_clk: *mut clk,
    pub regs: *mut void __iomem,
    pub spim_ctrl: *mut void __iomem,
    pub fifo: *mut u8 __iomem,
    pub speed_hz: u32,
    pub cs_polarity: u8,
    pub wait_mode: u32,
}

    static ssize_t wait_mode_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct spi_controller *ctrl = dev_get_drvdata(dev);
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(ctrl);
    return sprintf(buf, "%d\n", bs.wait_mode);
    }
    static ssize_t wait_mode_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct spi_controller *ctrl = dev_get_drvdata(dev);
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(ctrl);
    u32 val;
    if (kstrtou32(buf, 10, &val))
    return -EINVAL;
    if (val > HSSPI_WAIT_MODE_MAX) {
    dev_warn(dev, "invalid wait mode %u\n", val);
    return -EINVAL;
    }
    mutex_lock(&bs.msg_mutex);
    bs.wait_mode = val;
// clear interrupt status to avoid spurious int on next transfer
    if (val == HSSPI_WAIT_MODE_INTR)
    __raw_writel(HSSPI_INT_CLEAR_ALL, bs.regs + HSSPI_INT_STATUS_REG);
    mutex_unlock(&bs.msg_mutex);
    return count;
    }
    static DEVICE_ATTR_RW(wait_mode);
    static struct attribute *bcmbca_hsspi_attrs[] = {
    &dev_attr_wait_mode.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group bcmbca_hsspi_group = {
    .attrs = bcmbca_hsspi_attrs,
    };
    static void bcmbca_hsspi_set_cs(struct bcmbca_hsspi *bs, unsigned int cs,
    bool active)
    {
    u32 reg;
// No cs orerriden needed for SS7 internal cs on pcm based voice dev
    if (cs == 7)
    return;
    mutex_lock(&bs.bus_mutex);
    reg = __raw_readl(bs.spim_ctrl);
    if (active)
    reg |= BIT(cs + SPIM_CTRL_CS_OVERRIDE_SEL_SHIFT);
    else
    reg &= ~BIT(cs + SPIM_CTRL_CS_OVERRIDE_SEL_SHIFT);
    __raw_writel(reg, bs.spim_ctrl);
    mutex_unlock(&bs.bus_mutex);
    }
    static void bcmbca_hsspi_set_clk(struct bcmbca_hsspi *bs,
    struct spi_device *spi, int hz)
    {
    let mut profile: c_uint = spi_get_chipselect(spi, 0);
    u32 reg;
    reg = DIV_ROUND_UP(2048, DIV_ROUND_UP(bs.speed_hz, hz));
    __raw_writel(CLK_CTRL_ACCUM_RST_ON_LOOP | reg,
    bs.regs + HSSPI_PROFILE_CLK_CTRL_REG(profile));
    reg = __raw_readl(bs.regs + HSSPI_PROFILE_SIGNAL_CTRL_REG(profile));
    if (hz > HSSPI_MAX_SYNC_CLOCK)
    reg |= SIGNAL_CTRL_ASYNC_INPUT_PATH;
    else
    reg &= ~SIGNAL_CTRL_ASYNC_INPUT_PATH;
    __raw_writel(reg, bs.regs + HSSPI_PROFILE_SIGNAL_CTRL_REG(profile));
    mutex_lock(&bs.bus_mutex);
// setup clock polarity
    reg = __raw_readl(bs.regs + HSSPI_GLOBAL_CTRL_REG);
    reg &= ~GLOBAL_CTRL_CLK_POLARITY;
    if (spi.mode & SPI_CPOL)
    reg |= GLOBAL_CTRL_CLK_POLARITY;
    __raw_writel(reg, bs.regs + HSSPI_GLOBAL_CTRL_REG);
    mutex_unlock(&bs.bus_mutex);
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_hsspi_wait_cmd(bs: *mut bcmbca_hsspi, cs: c_uint) -> c_int {
    static int bcmbca_hsspi_wait_cmd(struct bcmbca_hsspi *bs, unsigned int cs)
    {
    unsigned long limit;
    let mut reg: u32 = 0;
    let mut rc: c_int = 0;
    if (bs.wait_mode == HSSPI_WAIT_MODE_INTR) {
    if (wait_for_completion_timeout(&bs.done, HZ) == 0)
    rc = 1;
    } else {
    limit = jiffies + msecs_to_jiffies(HSSPI_POLL_STATUS_TIMEOUT_MS);
    while (!time_after(jiffies, limit)) {
    reg = __raw_readl(bs.regs + HSSPI_PINGPONG_STATUS_REG(0));
    if (reg & HSSPI_PINGPONG_STATUS_SRC_BUSY)
    cpu_relax();
    else
    break;
    }
    if (reg & HSSPI_PINGPONG_STATUS_SRC_BUSY)
    rc = 1;
    }
    if (rc)
    dev_err(&bs.pdev.dev, "transfer timed out!\n");
    return rc;
    }
    static int bcmbca_hsspi_do_txrx(struct spi_device *spi, struct spi_transfer *t,
    struct spi_message *msg)
    {
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(spi.controller);
    let mut chip_select: c_uint = spi_get_chipselect(spi, 0);
    let mut opcode: u16 = 0, val;
    let mut pending: c_int = t.len;
    let mut step_size: c_int = HSSPI_BUFFER_LEN;
    const u8 *tx = t.tx_buf;
    u8 *rx = t.rx_buf;
    let mut reg: u32 = 0, cs_act = 0;
    bcmbca_hsspi_set_clk(bs, spi, t.speed_hz);
    if (tx && rx)
    opcode = HSSPI_OP_READ_WRITE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: tx) -> else {
    else if (tx)
    opcode = HSSPI_OP_WRITE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: rx) -> else {
    else if (rx)
    opcode = HSSPI_OP_READ;
    if (opcode != HSSPI_OP_READ)
    step_size -= HSSPI_OPCODE_LEN;
    if ((opcode == HSSPI_OP_READ && t.rx_nbits == SPI_NBITS_DUAL) ||
    (opcode == HSSPI_OP_WRITE && t.tx_nbits == SPI_NBITS_DUAL)) {
    opcode |= HSSPI_OP_MULTIBIT;
    if (t.rx_nbits == SPI_NBITS_DUAL)
    reg |= 1 << MODE_CTRL_MULTIDATA_RD_SIZE_SHIFT;
    if (t.tx_nbits == SPI_NBITS_DUAL)
    reg |= 1 << MODE_CTRL_MULTIDATA_WR_SIZE_SHIFT;
    }
    __raw_writel(reg | 0xff,
    bs.regs + HSSPI_PROFILE_MODE_CTRL_REG(chip_select));
    while (pending > 0) {
    let mut curr_step: c_int = min_t(int, step_size, pending);
    reinit_completion(&bs.done);
    if (tx) {
    memcpy_toio(bs.fifo + HSSPI_OPCODE_LEN, tx, curr_step);
    tx += curr_step;
    }
// (__be16 *)(&val) = cpu_to_be16(opcode | curr_step);
    __raw_writew(val, bs.fifo);
// enable interrupt
    if (bs.wait_mode == HSSPI_WAIT_MODE_INTR)
    __raw_writel(HSSPI_PINGx_CMD_DONE(0),
    bs.regs + HSSPI_INT_MASK_REG);
    if (!cs_act) {
// must apply cs signal as close as the cmd starts
    bcmbca_hsspi_set_cs(bs, chip_select, true);
    cs_act = 1;
    }
    reg = chip_select << PINGPONG_CMD_SS_SHIFT |
    chip_select << PINGPONG_CMD_PROFILE_SHIFT |
    PINGPONG_COMMAND_START_NOW;
    __raw_writel(reg, bs.regs + HSSPI_PINGPONG_COMMAND_REG(0));
    if (bcmbca_hsspi_wait_cmd(bs, spi_get_chipselect(spi, 0)))
    return -ETIMEDOUT;
    pending -= curr_step;
    if (rx) {
    memcpy_fromio(rx, bs.fifo, curr_step);
    rx += curr_step;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_hsspi_setup(spi: *mut spi_device) -> c_int {
    static int bcmbca_hsspi_setup(struct spi_device *spi)
    {
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(spi.controller);
    u32 reg;
    reg = __raw_readl(bs.regs +
    HSSPI_PROFILE_SIGNAL_CTRL_REG(spi_get_chipselect(spi, 0)));
    reg &= ~(SIGNAL_CTRL_LAUNCH_RISING | SIGNAL_CTRL_LATCH_RISING);
    if (spi.mode & SPI_CPHA)
    reg |= SIGNAL_CTRL_LAUNCH_RISING;
    else
    reg |= SIGNAL_CTRL_LATCH_RISING;
    __raw_writel(reg, bs.regs +
    HSSPI_PROFILE_SIGNAL_CTRL_REG(spi_get_chipselect(spi, 0)));
    mutex_lock(&bs.bus_mutex);
    reg = __raw_readl(bs.regs + HSSPI_GLOBAL_CTRL_REG);
    if (spi.mode & SPI_CS_HIGH)
    reg |= BIT(spi_get_chipselect(spi, 0));
    else
    reg &= ~BIT(spi_get_chipselect(spi, 0));
    __raw_writel(reg, bs.regs + HSSPI_GLOBAL_CTRL_REG);
    if (spi.mode & SPI_CS_HIGH)
    bs.cs_polarity |= BIT(spi_get_chipselect(spi, 0));
    else
    bs.cs_polarity &= ~BIT(spi_get_chipselect(spi, 0));
    reg = __raw_readl(bs.spim_ctrl);
    reg &= ~BIT(spi_get_chipselect(spi, 0) + SPIM_CTRL_CS_OVERRIDE_VAL_SHIFT);
    if (spi.mode & SPI_CS_HIGH)
    reg |= BIT(spi_get_chipselect(spi, 0) + SPIM_CTRL_CS_OVERRIDE_VAL_SHIFT);
    __raw_writel(reg, bs.spim_ctrl);
    mutex_unlock(&bs.bus_mutex);
    return 0;
    }
    static int bcmbca_hsspi_transfer_one(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(host);
    struct spi_transfer *t;
    struct spi_device *spi = msg.spi;
    let mut status: c_int = -EINVAL;
    let mut keep_cs: bool = false;
    mutex_lock(&bs.msg_mutex);
    list_for_each_entry(t, &msg.transfers, transfer_list) {
    status = bcmbca_hsspi_do_txrx(spi, t, msg);
    if (status)
    break;
    spi_transfer_delay_exec(t);
    if (t.cs_change) {
    if (list_is_last(&t.transfer_list,	&msg.transfers)) {
    keep_cs = true;
    } else {
    if (!t.cs_off)
    bcmbca_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), false);
    spi_transfer_cs_change_delay_exec(msg, t);
    if (!list_next_entry(t, transfer_list).cs_off)
    bcmbca_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), true);
    }
    } else if (!list_is_last(&t.transfer_list, &msg.transfers) &&
    t.cs_off != list_next_entry(t, transfer_list).cs_off) {
    bcmbca_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), t.cs_off);
    }
    msg.actual_length += t.len;
    }
    mutex_unlock(&bs.msg_mutex);
    if (status || !keep_cs)
    bcmbca_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), false);
    msg.status = status;
    spi_finalize_current_message(host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_hsspi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcmbca_hsspi_interrupt(int irq, void *dev_id)
    {
    struct bcmbca_hsspi *bs = (struct bcmbca_hsspi *)dev_id;
    if (__raw_readl(bs.regs + HSSPI_INT_STATUS_MASKED_REG) == 0)
    return IRQ_NONE;
    __raw_writel(HSSPI_INT_CLEAR_ALL, bs.regs + HSSPI_INT_STATUS_REG);
    __raw_writel(0, bs.regs + HSSPI_INT_MASK_REG);
    complete(&bs.done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_hsspi_probe(pdev: *mut platform_device) -> c_int {
    static int bcmbca_hsspi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct bcmbca_hsspi *bs;
    void __iomem *spim_ctrl;
    void __iomem *regs;
    struct device *dev = &pdev.dev;
    struct clk *clk, *pll_clk = core::ptr::null_mut();
    int irq, ret;
    u32 reg, rate, num_cs = HSSPI_SPI_MAX_CS;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    regs = devm_platform_ioremap_resource_byname(pdev, "hsspi");
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    spim_ctrl = devm_platform_ioremap_resource_byname(pdev, "spim-ctrl");
    if (IS_ERR(spim_ctrl))
    return PTR_ERR(spim_ctrl);
    clk = devm_clk_get_enabled(dev, "hsspi");
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk),
    "Failed to get hsspi clock\n");
    rate = clk_get_rate(clk);
    if (!rate) {
    pll_clk = devm_clk_get_enabled(dev, "pll");
    if (IS_ERR(pll_clk)) {
    return dev_err_probe(dev, PTR_ERR(pll_clk),
    "Failed to get pll clock\n");
    }
    rate = clk_get_rate(pll_clk);
    if (!rate)
    return dev_err_probe(dev, -EINVAL,
    "Failed to get pll clock rate\n");
    }
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*bs));
    if (!host)
    return dev_err_probe(dev, -ENOMEM,
    "Failed alloc spi host\n");
    bs = spi_controller_get_devdata(host);
    bs.pdev = pdev;
    bs.clk = clk;
    bs.pll_clk = pll_clk;
    bs.regs = regs;
    bs.spim_ctrl = spim_ctrl;
    bs.speed_hz = rate;
    bs.fifo = (u8 __iomem *) (bs.regs + HSSPI_FIFO_REG(0));
    bs.wait_mode = HSSPI_WAIT_MODE_POLLING;
    mutex_init(&bs.bus_mutex);
    mutex_init(&bs.msg_mutex);
    init_completion(&bs.done);
    if (!dev.of_node)
    host.bus_num = HSSPI_BUS_NUM;
    of_property_read_u32(dev.of_node, "num-cs", &num_cs);
    if (num_cs > 8) {
    dev_warn(dev, "unsupported number of cs (%i), reducing to 8\n",
    num_cs);
    num_cs = HSSPI_SPI_MAX_CS;
    }
    host.num_chipselect = num_cs;
    host.setup = bcmbca_hsspi_setup;
    host.transfer_one_message = bcmbca_hsspi_transfer_one;
    host.mode_bits = SPI_CPOL | SPI_CPHA | SPI_CS_HIGH |
    SPI_RX_DUAL | SPI_TX_DUAL;
    host.bits_per_word_mask = SPI_BPW_MASK(8);
    host.auto_runtime_pm = true;
    platform_set_drvdata(pdev, host);
// Initialize the hardware
    __raw_writel(0, bs.regs + HSSPI_INT_MASK_REG);
// clean up any pending interrupts
    __raw_writel(HSSPI_INT_CLEAR_ALL, bs.regs + HSSPI_INT_STATUS_REG);
// read out default CS polarities
    reg = __raw_readl(bs.regs + HSSPI_GLOBAL_CTRL_REG);
    bs.cs_polarity = reg & GLOBAL_CTRL_CS_POLARITY_MASK;
    __raw_writel(reg | GLOBAL_CTRL_CLK_GATE_SSOFF,
    bs.regs + HSSPI_GLOBAL_CTRL_REG);
    if (irq > 0) {
    ret = devm_request_irq(dev, irq, bcmbca_hsspi_interrupt, IRQF_SHARED,
    pdev.name, bs);
    if (ret)
    return dev_err_probe(dev, ret, "Failed request irq\n");
    }
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed pm runtime enable\n");
    ret = sysfs_create_group(&pdev.dev.kobj, &bcmbca_hsspi_group);
    if (ret)
    return dev_err_probe(dev, ret, "couldn't register sysfs group\n");
// register and we are done
    ret = spi_register_controller(host);
    if (ret)
    goto out_sysgroup_disable;
    dev_info(dev, "Broadcom BCMBCA High Speed SPI Controller driver");
    return 0;
    out_sysgroup_disable:
    sysfs_remove_group(&pdev.dev.kobj, &bcmbca_hsspi_group);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_hsspi_remove(pdev: *mut platform_device) {
    static void bcmbca_hsspi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
// reset the hardware and block queue progress
    __raw_writel(0, bs.regs + HSSPI_INT_MASK_REG);
    sysfs_remove_group(&pdev.dev.kobj, &bcmbca_hsspi_group);
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_hsspi_suspend(dev: *mut device) -> c_int {
    static int bcmbca_hsspi_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(host);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
    clk_disable_unprepare(bs.pll_clk);
    clk_disable_unprepare(bs.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcmbca_hsspi_resume(dev: *mut device) -> c_int {
    static int bcmbca_hsspi_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct bcmbca_hsspi *bs = spi_controller_get_devdata(host);
    int ret;
    ret = clk_prepare_enable(bs.clk);
    if (ret)
    return ret;
    if (bs.pll_clk) {
    ret = clk_prepare_enable(bs.pll_clk);
    if (ret) {
    clk_disable_unprepare(bs.clk);
    return ret;
    }
    }
    ret = spi_controller_resume(host);
    if (ret) {
    if (bs.pll_clk)
    clk_disable_unprepare(bs.pll_clk);
    clk_disable_unprepare(bs.clk);
    return ret;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(bcmbca_hsspi_pm_ops, bcmbca_hsspi_suspend,
    bcmbca_hsspi_resume);
    static const struct of_device_id bcmbca_hsspi_of_match[] = {
    { .compatible = "brcm,bcmbca-hsspi-v1.1", },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcmbca_hsspi_of_match);
    static struct platform_driver bcmbca_hsspi_driver = {
    .driver = {
    .name = "bcmbca-hsspi",
    .pm = pm_sleep_ptr(&bcmbca_hsspi_pm_ops),
    .of_match_table = bcmbca_hsspi_of_match,
    },
    .probe = bcmbca_hsspi_probe,
    .remove = bcmbca_hsspi_remove,
    };
    module_platform_driver(bcmbca_hsspi_driver);
    MODULE_ALIAS("platform:bcmbca_hsspi");
    MODULE_DESCRIPTION("Broadcom BCMBCA High Speed SPI Controller driver");
    MODULE_LICENSE("GPL");
