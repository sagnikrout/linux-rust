//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-bcm63xx-hsspi.c
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


//
// Broadcom BCM63XX High Speed SPI Controller driver
//
// Copyright 2000-2010 Broadcom Corporation
// Copyright 2012-2013 Jonas Gorski <jonas.gorski@gmail.com>
//
// Licensed under the GNU/GPL. See COPYING for details.
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
//
// Some chip require 30MHz but other require 25MHz. Use smaller value to cover
// both cases.
//
pub const HSSPI_MAX_SYNC_CLOCK: c_int = 25000000;
pub const HSSPI_SPI_MAX_CS: c_int = 8;

pub const HSSPI_POLL_STATUS_TIMEOUT_MS: c_int = 100;
pub const HSSPI_WAIT_MODE_POLLING: c_int = 0;
pub const HSSPI_WAIT_MODE_INTR: c_int = 1;

//
// Default transfer mode is auto. If the msg is prependable, use the prepend
// mode.  If not, falls back to use the dummy cs workaround mode but limit the
// clock to 25MHz to make sure it works in all board design.
//
pub const HSSPI_XFER_MODE_AUTO: c_int = 0;
pub const HSSPI_XFER_MODE_PREPEND: c_int = 1;
pub const HSSPI_XFER_MODE_DUMMYCS: c_int = 2;

    do {										\
    if (bs.xfer_mode == HSSPI_XFER_MODE_AUTO)				\
    dev_dbg(&bs.pdev.dev, fmt, ##__VA_ARGS__);		\
    else if (bs.xfer_mode == HSSPI_XFER_MODE_PREPEND)		\
    dev_err(&bs.pdev.dev, fmt, ##__VA_ARGS__);		\
    } while (0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63xx_hsspi {
    pub done: completion,
    pub bus_mutex: mutex,
    pub msg_mutex: mutex,
    pub pdev: *mut platform_device,
    pub clk: *mut clk,
    pub pll_clk: *mut clk,
    pub regs: *mut void __iomem,
    pub fifo: *mut u8 __iomem,
    pub speed_hz: u32,
    pub cs_polarity: u8,
    pub wait_mode: u32,
    pub xfer_mode: u32,
    pub prepend_cnt: u32,
    pub md_start: u32,
    pub prepend_buf: *mut u8,
}

    static ssize_t wait_mode_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct spi_controller *ctrl = dev_get_drvdata(dev);
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(ctrl);
    return sprintf(buf, "%d\n", bs.wait_mode);
    }
    static ssize_t wait_mode_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct spi_controller *ctrl = dev_get_drvdata(dev);
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(ctrl);
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
    static ssize_t xfer_mode_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct spi_controller *ctrl = dev_get_drvdata(dev);
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(ctrl);
    return sprintf(buf, "%d\n", bs.xfer_mode);
    }
    static ssize_t xfer_mode_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct spi_controller *ctrl = dev_get_drvdata(dev);
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(ctrl);
    u32 val;
    if (kstrtou32(buf, 10, &val))
    return -EINVAL;
    if (val > HSSPI_XFER_MODE_MAX) {
    dev_warn(dev, "invalid xfer mode %u\n", val);
    return -EINVAL;
    }
    mutex_lock(&bs.msg_mutex);
    bs.xfer_mode = val;
    mutex_unlock(&bs.msg_mutex);
    return count;
    }
    static DEVICE_ATTR_RW(xfer_mode);
    static struct attribute *bcm63xx_hsspi_attrs[] = {
    &dev_attr_wait_mode.attr,
    &dev_attr_xfer_mode.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group bcm63xx_hsspi_group = {
    .attrs = bcm63xx_hsspi_attrs,
    };
    static void bcm63xx_hsspi_set_clk(struct bcm63xx_hsspi *bs,
    struct spi_device *spi, int hz);
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_max_message_size(spi: *mut spi_device) -> usize {
    static size_t bcm63xx_hsspi_max_message_size(struct spi_device *spi)
    {
    return HSSPI_BUFFER_LEN - HSSPI_OPCODE_LEN;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_wait_cmd(bs: *mut bcm63xx_hsspi) -> c_int {
    static int bcm63xx_hsspi_wait_cmd(struct bcm63xx_hsspi *bs)
    {
    unsigned long limit;
    let mut reg: u32 = 0;
    let mut rc: c_int = 0;
    if (bs.wait_mode == HSSPI_WAIT_MODE_INTR) {
    if (wait_for_completion_timeout(&bs.done, HZ) == 0)
    rc = 1;
    } else {
// polling mode checks for status busy bit
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
    static bool bcm63xx_prepare_prepend_transfer(struct spi_controller *host,
    struct spi_message *msg,
    struct spi_transfer *t_prepend)
    {
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(host);
    let mut tx_only: bool = false, multidata = false;
    struct spi_transfer *t;
//
// Multiple transfers within a message may be combined into one transfer
// to the controller using its prepend feature. A SPI message is prependable
// only if the following are all true:
// 1. One or more half duplex write transfers at the start
// 2. Optional switch from single to dual bit within the write transfers
// 3. Optional full duplex read/write at the end if all single bit
// 4. No delay and cs_change between transfers
//
    bs.prepend_cnt = 0;
    bs.md_start = 0;
    list_for_each_entry(t, &msg.transfers, transfer_list) {
    if ((spi_delay_to_ns(&t.delay, t) > 0) || t.cs_change) {
    bcm63xx_prepend_printk_on_checkfail(bs,
    "Delay or cs change not supported in prepend mode!\n");
    return false;
    }
    tx_only = false;
    if (t.tx_buf && !t.rx_buf) {
    tx_only = true;
    if (bs.prepend_cnt + t.len >
    (HSSPI_BUFFER_LEN - HSSPI_OPCODE_LEN)) {
    bcm63xx_prepend_printk_on_checkfail(bs,
    "exceed max buf len, abort prepending transfers!\n");
    return false;
    }
    if (t.tx_nbits == SPI_NBITS_SINGLE &&
    !list_is_last(&t.transfer_list, &msg.transfers) &&
    multidata) {
    bcm63xx_prepend_printk_on_checkfail(bs,
    "single-bit after multi-bit not supported!\n");
    return false;
    }
    if (t.tx_nbits > SPI_NBITS_SINGLE)
    multidata = true;
    memcpy(bs.prepend_buf + bs.prepend_cnt, t.tx_buf, t.len);
    bs.prepend_cnt += t.len;
    if (t.tx_nbits == SPI_NBITS_SINGLE)
    bs.md_start += t.len;
    } else {
    if (!list_is_last(&t.transfer_list, &msg.transfers)) {
    bcm63xx_prepend_printk_on_checkfail(bs,
    "rx/tx_rx transfer not supported when it is not last one!\n");
    return false;
    }
    if (t.rx_buf && t.rx_nbits == SPI_NBITS_SINGLE &&
    multidata) {
    bcm63xx_prepend_printk_on_checkfail(bs,
    "single-bit after multi-bit not supported!\n");
    return false;
    }
    }
    if (list_is_last(&t.transfer_list, &msg.transfers)) {
    memcpy(t_prepend, t, sizeof(struct spi_transfer));
    if (tx_only) {
//
// if the last one is also a tx only transfer, merge
// all of them into one single tx transfer
//
    t_prepend.len = bs.prepend_cnt;
    t_prepend.tx_buf = bs.prepend_buf;
    bs.prepend_cnt = 0;
    } else {
//
// if the last one is not a tx only transfer, all
// the previous transfers are sent through prepend bytes and
// make sure it does not exceed the max prepend len
//
    if (bs.prepend_cnt > HSSPI_MAX_PREPEND_LEN) {
    bcm63xx_prepend_printk_on_checkfail(bs,
    "exceed max prepend len, abort prepending transfers!\n");
    return false;
    }
    }
//
// If switching from single-bit to multi-bit, make sure
// the start offset does not exceed the maximum
//
    if (multidata && bs.md_start > HSSPI_MAX_PREPEND_LEN) {
    bcm63xx_prepend_printk_on_checkfail(bs,
    "exceed max multi-bit offset, abort prepending transfers!\n");
    return false;
    }
    }
    }
    return true;
    }
    static int bcm63xx_hsspi_do_prepend_txrx(struct spi_device *spi,
    struct spi_transfer *t)
    {
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(spi.controller);
    let mut chip_select: c_uint = spi_get_chipselect(spi, 0);
    let mut opcode: u16 = 0, val;
    const u8 *tx = t.tx_buf;
    u8 *rx = t.rx_buf;
    let mut reg: u32 = 0;
//
// shouldn't happen as we set the max_message_size in the probe.
// but check it again in case some driver does not honor the max size
//
    if (t.len + bs.prepend_cnt > (HSSPI_BUFFER_LEN - HSSPI_OPCODE_LEN)) {
    dev_warn(&bs.pdev.dev,
    "Prepend message large than fifo size len %d prepend %d\n",
    t.len, bs.prepend_cnt);
    return -EINVAL;
    }
    bcm63xx_hsspi_set_clk(bs, spi, t.speed_hz);
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
    if ((opcode == HSSPI_OP_READ && t.rx_nbits == SPI_NBITS_DUAL) ||
    (opcode == HSSPI_OP_WRITE && t.tx_nbits == SPI_NBITS_DUAL)) {
    opcode |= HSSPI_OP_MULTIBIT;
    if (t.rx_nbits == SPI_NBITS_DUAL) {
    reg |= 1 << MODE_CTRL_MULTIDATA_RD_SIZE_SHIFT;
    reg |= bs.md_start << MODE_CTRL_MULTIDATA_RD_STRT_SHIFT;
    }
    if (t.tx_nbits == SPI_NBITS_DUAL) {
    reg |= 1 << MODE_CTRL_MULTIDATA_WR_SIZE_SHIFT;
    reg |= bs.md_start << MODE_CTRL_MULTIDATA_WR_STRT_SHIFT;
    }
    }
    reg |= bs.prepend_cnt << MODE_CTRL_PREPENDBYTE_CNT_SHIFT;
    __raw_writel(reg | 0xff,
    bs.regs + HSSPI_PROFILE_MODE_CTRL_REG(chip_select));
    reinit_completion(&bs.done);
    if (bs.prepend_cnt)
    memcpy_toio(bs.fifo + HSSPI_OPCODE_LEN, bs.prepend_buf,
    bs.prepend_cnt);
    if (tx)
    memcpy_toio(bs.fifo + HSSPI_OPCODE_LEN + bs.prepend_cnt, tx,
    t.len);
// (__be16 *)(&val) = cpu_to_be16(opcode | t->len);
    __raw_writew(val, bs.fifo);
// enable interrupt
    if (bs.wait_mode == HSSPI_WAIT_MODE_INTR)
    __raw_writel(HSSPI_PINGx_CMD_DONE(0), bs.regs + HSSPI_INT_MASK_REG);
// start the transfer
    reg = chip_select << PINGPONG_CMD_SS_SHIFT |
    chip_select << PINGPONG_CMD_PROFILE_SHIFT |
    PINGPONG_COMMAND_START_NOW;
    __raw_writel(reg, bs.regs + HSSPI_PINGPONG_COMMAND_REG(0));
    if (bcm63xx_hsspi_wait_cmd(bs))
    return -ETIMEDOUT;
    if (rx)
    memcpy_fromio(rx, bs.fifo, t.len);
    return 0;
    }
    static void bcm63xx_hsspi_set_cs(struct bcm63xx_hsspi *bs, unsigned int cs,
    bool active)
    {
    u32 reg;
    mutex_lock(&bs.bus_mutex);
    reg = __raw_readl(bs.regs + HSSPI_GLOBAL_CTRL_REG);
    reg &= ~BIT(cs);
    if (active == !(bs.cs_polarity & BIT(cs)))
    reg |= BIT(cs);
    __raw_writel(reg, bs.regs + HSSPI_GLOBAL_CTRL_REG);
    mutex_unlock(&bs.bus_mutex);
    }
    static void bcm63xx_hsspi_set_clk(struct bcm63xx_hsspi *bs,
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
unsafe extern "C" fn bcm63xx_hsspi_do_txrx(spi: *mut spi_device, t: *mut spi_transfer) -> c_int {
    static int bcm63xx_hsspi_do_txrx(struct spi_device *spi, struct spi_transfer *t)
    {
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(spi.controller);
    let mut chip_select: c_uint = spi_get_chipselect(spi, 0);
    let mut opcode: u16 = 0, val;
    let mut pending: c_int = t.len;
    let mut step_size: c_int = HSSPI_BUFFER_LEN;
    const u8 *tx = t.tx_buf;
    u8 *rx = t.rx_buf;
    let mut reg: u32 = 0;
    bcm63xx_hsspi_set_clk(bs, spi, t.speed_hz);
    if (!t.cs_off)
    bcm63xx_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), true);
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
    reg =  !chip_select << PINGPONG_CMD_SS_SHIFT |
    chip_select << PINGPONG_CMD_PROFILE_SHIFT |
    PINGPONG_COMMAND_START_NOW;
    __raw_writel(reg, bs.regs + HSSPI_PINGPONG_COMMAND_REG(0));
    if (bcm63xx_hsspi_wait_cmd(bs))
    return -ETIMEDOUT;
    if (rx) {
    memcpy_fromio(rx, bs.fifo, curr_step);
    rx += curr_step;
    }
    pending -= curr_step;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_setup(spi: *mut spi_device) -> c_int {
    static int bcm63xx_hsspi_setup(struct spi_device *spi)
    {
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(spi.controller);
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
// only change actual polarities if there is no transfer
    if ((reg & GLOBAL_CTRL_CS_POLARITY_MASK) == bs.cs_polarity) {
    if (spi.mode & SPI_CS_HIGH)
    reg |= BIT(spi_get_chipselect(spi, 0));
    else
    reg &= ~BIT(spi_get_chipselect(spi, 0));
    __raw_writel(reg, bs.regs + HSSPI_GLOBAL_CTRL_REG);
    }
    if (spi.mode & SPI_CS_HIGH)
    bs.cs_polarity |= BIT(spi_get_chipselect(spi, 0));
    else
    bs.cs_polarity &= ~BIT(spi_get_chipselect(spi, 0));
    mutex_unlock(&bs.bus_mutex);
    return 0;
    }
    static int bcm63xx_hsspi_do_dummy_cs_txrx(struct spi_device *spi,
    struct spi_message *msg)
    {
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(spi.controller);
    let mut status: c_int = -EINVAL;
    int dummy_cs;
    let mut keep_cs: bool = false;
    struct spi_transfer *t;
//
// This controller does not support keeping CS active during idle.
// To work around this, we use the following ugly hack:
//
// a. Invert the target chip select's polarity so it will be active.
// b. Select a "dummy" chip select to use as the hardware target.
// c. Invert the dummy chip select's polarity so it will be inactive
// during the actual transfers.
// d. Tell the hardware to send to the dummy chip select. Thanks to
// the multiplexed nature of SPI the actual target will receive
// the transfer and we see its response.
//
// e. At the end restore the polarities again to their default values.
//
    dummy_cs = !spi_get_chipselect(spi, 0);
    bcm63xx_hsspi_set_cs(bs, dummy_cs, true);
    list_for_each_entry(t, &msg.transfers, transfer_list) {
//
// We are here because one of reasons below:
// a. Message is not prependable and in default auto xfer mode. This mean
// we fallback to dummy cs mode at maximum 25MHz safe clock rate.
// b. User set to use the dummy cs mode.
//
    if (bs.xfer_mode == HSSPI_XFER_MODE_AUTO) {
    if (t.speed_hz > HSSPI_MAX_SYNC_CLOCK) {
    t.speed_hz = HSSPI_MAX_SYNC_CLOCK;
    dev_warn_once(&bs.pdev.dev,
    "Force to dummy cs mode. Reduce the speed to %dHz",
    t.speed_hz);
    }
    }
    status = bcm63xx_hsspi_do_txrx(spi, t);
    if (status)
    break;
    msg.actual_length += t.len;
    spi_transfer_delay_exec(t);
// use existing cs change logic from spi_transfer_one_message
    if (t.cs_change) {
    if (list_is_last(&t.transfer_list, &msg.transfers)) {
    keep_cs = true;
    } else {
    if (!t.cs_off)
    bcm63xx_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), false);
    spi_transfer_cs_change_delay_exec(msg, t);
    if (!list_next_entry(t, transfer_list).cs_off)
    bcm63xx_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), true);
    }
    } else if (!list_is_last(&t.transfer_list, &msg.transfers) &&
    t.cs_off != list_next_entry(t, transfer_list).cs_off) {
    bcm63xx_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), t.cs_off);
    }
    }
    bcm63xx_hsspi_set_cs(bs, dummy_cs, false);
    if (status || !keep_cs)
    bcm63xx_hsspi_set_cs(bs, spi_get_chipselect(spi, 0), false);
    return status;
    }
    static int bcm63xx_hsspi_transfer_one(struct spi_controller *host,
    struct spi_message *msg)
    {
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(host);
    struct spi_device *spi = msg.spi;
    let mut status: c_int = -EINVAL;
    let mut prependable: bool = false;
    struct spi_transfer t_prepend;
    mutex_lock(&bs.msg_mutex);
    if (bs.xfer_mode != HSSPI_XFER_MODE_DUMMYCS)
    prependable = bcm63xx_prepare_prepend_transfer(host, msg, &t_prepend);
    if (prependable) {
    status = bcm63xx_hsspi_do_prepend_txrx(spi, &t_prepend);
    msg.actual_length = (t_prepend.len + bs.prepend_cnt);
    } else {
    if (bs.xfer_mode == HSSPI_XFER_MODE_PREPEND) {
    dev_err(&bs.pdev.dev,
    "User sets prepend mode but msg not prependable! Abort transfer\n");
    status = -EINVAL;
    } else
    status = bcm63xx_hsspi_do_dummy_cs_txrx(spi, msg);
    }
    mutex_unlock(&bs.msg_mutex);
    msg.status = status;
    spi_finalize_current_message(host);
    return 0;
    }
    static bool bcm63xx_hsspi_mem_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    if (!spi_mem_default_supports_op(mem, op))
    return false;
    return true;
    }
    static const struct spi_controller_mem_ops bcm63xx_hsspi_mem_ops = {
    .supports_op = bcm63xx_hsspi_mem_supports_op,
    };
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcm63xx_hsspi_interrupt(int irq, void *dev_id)
    {
    struct bcm63xx_hsspi *bs = (struct bcm63xx_hsspi *)dev_id;
    if (__raw_readl(bs.regs + HSSPI_INT_STATUS_MASKED_REG) == 0)
    return IRQ_NONE;
    __raw_writel(HSSPI_INT_CLEAR_ALL, bs.regs + HSSPI_INT_STATUS_REG);
    __raw_writel(0, bs.regs + HSSPI_INT_MASK_REG);
    complete(&bs.done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_probe(pdev: *mut platform_device) -> c_int {
    static int bcm63xx_hsspi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct bcm63xx_hsspi *bs;
    void __iomem *regs;
    struct device *dev = &pdev.dev;
    struct clk *clk, *pll_clk = core::ptr::null_mut();
    int irq, ret;
    u32 reg, rate, num_cs = HSSPI_SPI_MAX_CS;
    struct reset_control *reset;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    clk = devm_clk_get_enabled(dev, "hsspi");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    reset = devm_reset_control_get_optional_shared(dev, core::ptr::null_mut());
    if (IS_ERR(reset))
    return PTR_ERR(reset);
    ret = reset_control_reset(reset);
    if (ret)
    return dev_err_probe(dev, ret, "unable to reset device: %d\n", ret);
    rate = clk_get_rate(clk);
    if (!rate) {
    pll_clk = devm_clk_get_enabled(dev, "pll");
    if (IS_ERR(pll_clk))
    return dev_err_probe(dev, PTR_ERR(pll_clk),
    "failed enable pll clk\n");
    rate = clk_get_rate(pll_clk);
    if (!rate)
    return dev_err_probe(dev, -EINVAL,
    "failed get pll clk rate\n");
    }
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*bs));
    if (!host)
    return dev_err_probe(dev, -ENOMEM, "alloc host no mem\n");
    bs = spi_controller_get_devdata(host);
    bs.pdev = pdev;
    bs.clk = clk;
    bs.pll_clk = pll_clk;
    bs.regs = regs;
    bs.speed_hz = rate;
    bs.fifo = (u8 __iomem *)(bs.regs + HSSPI_FIFO_REG(0));
    bs.wait_mode = HSSPI_WAIT_MODE_POLLING;
    bs.prepend_buf = devm_kzalloc(dev, HSSPI_BUFFER_LEN, GFP_KERNEL);
    if (!bs.prepend_buf)
    return -ENOMEM;
    mutex_init(&bs.bus_mutex);
    mutex_init(&bs.msg_mutex);
    init_completion(&bs.done);
    host.mem_ops = &bcm63xx_hsspi_mem_ops;
    if (!dev.of_node)
    host.bus_num = HSSPI_BUS_NUM;
    of_property_read_u32(dev.of_node, "num-cs", &num_cs);
    if (num_cs > 8) {
    dev_warn(dev, "unsupported number of cs (%i), reducing to 8\n",
    num_cs);
    num_cs = HSSPI_SPI_MAX_CS;
    }
    host.num_chipselect = num_cs;
    host.setup = bcm63xx_hsspi_setup;
    host.transfer_one_message = bcm63xx_hsspi_transfer_one;
    host.max_transfer_size = bcm63xx_hsspi_max_message_size;
    host.max_message_size = bcm63xx_hsspi_max_message_size;
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
    ret = devm_request_irq(dev, irq, bcm63xx_hsspi_interrupt, IRQF_SHARED,
    pdev.name, bs);
    if (ret)
    return ret;
    }
    pm_runtime_enable(&pdev.dev);
    ret = sysfs_create_group(&pdev.dev.kobj, &bcm63xx_hsspi_group);
    if (ret) {
    dev_err(&pdev.dev, "couldn't register sysfs group\n");
    goto out_pm_disable;
    }
// register and we are done
    ret = spi_register_controller(host);
    if (ret)
    goto out_sysgroup_disable;
    dev_info(dev, "Broadcom 63XX High Speed SPI Controller driver");
    return 0;
    out_sysgroup_disable:
    sysfs_remove_group(&pdev.dev.kobj, &bcm63xx_hsspi_group);
    out_pm_disable:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_remove(pdev: *mut platform_device) {
    static void bcm63xx_hsspi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
// reset the hardware and block queue progress
    __raw_writel(0, bs.regs + HSSPI_INT_MASK_REG);
    sysfs_remove_group(&pdev.dev.kobj, &bcm63xx_hsspi_group);
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_suspend(dev: *mut device) -> c_int {
    static int bcm63xx_hsspi_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(host);
    int ret;
    ret = spi_controller_suspend(host);
    if (ret)
    return ret;
    clk_disable_unprepare(bs.pll_clk);
    clk_disable_unprepare(bs.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_hsspi_resume(dev: *mut device) -> c_int {
    static int bcm63xx_hsspi_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct bcm63xx_hsspi *bs = spi_controller_get_devdata(host);
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
    static DEFINE_SIMPLE_DEV_PM_OPS(bcm63xx_hsspi_pm_ops, bcm63xx_hsspi_suspend,
    bcm63xx_hsspi_resume);
    static const struct of_device_id bcm63xx_hsspi_of_match[] = {
    { .compatible = "brcm,bcm6328-hsspi", },
    { .compatible = "brcm,bcmbca-hsspi-v1.0", },
    { },
    };
    MODULE_DEVICE_TABLE(of, bcm63xx_hsspi_of_match);
    static struct platform_driver bcm63xx_hsspi_driver = {
    .driver = {
    .name	= "bcm63xx-hsspi",
    .pm	= pm_sleep_ptr(&bcm63xx_hsspi_pm_ops),
    .of_match_table = bcm63xx_hsspi_of_match,
    },
    .probe		= bcm63xx_hsspi_probe,
    .remove		= bcm63xx_hsspi_remove,
    };
    module_platform_driver(bcm63xx_hsspi_driver);
    MODULE_ALIAS("platform:bcm63xx_hsspi");
    MODULE_DESCRIPTION("Broadcom BCM63xx High Speed SPI Controller driver");
    MODULE_AUTHOR("Jonas Gorski <jogo@openwrt.org>");
    MODULE_LICENSE("GPL");
