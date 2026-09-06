//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-amlogic-spifc-a1.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Driver for Amlogic A1 SPI flash controller (SPIFC)
//
// Copyright (c) 2023, SberDevices. All Rights Reserved.
//
// Author: Martin Kurbanov <mmkurbanov@sberdevices.ru>
//

pub const SPIFC_A1_AHB_CTRL_REG: c_uint = 0x0;

pub const SPIFC_A1_USER_CTRL0_REG: c_uint = 0x200;

pub const SPIFC_A1_USER_CTRL1_REG: c_uint = 0x204;

pub const SPIFC_A1_USER_CTRL2_REG: c_uint = 0x208;

pub const SPIFC_A1_USER_CTRL3_REG: c_uint = 0x20c;

pub const SPIFC_A1_USER_ADDR_REG: c_uint = 0x210;
pub const SPIFC_A1_AHB_REQ_CTRL_REG: c_uint = 0x214;

pub const SPIFC_A1_DBUF_CTRL_REG: c_uint = 0x240;

pub const SPIFC_A1_DBUF_DATA_REG: c_uint = 0x244;
pub const SPIFC_A1_USER_DBUF_ADDR_REG: c_uint = 0x248;

pub const SPIFC_A1_MAX_HZ: c_int = 200000000;
pub const SPIFC_A1_MIN_HZ: c_int = 1000000;

    SPIFC_A1_USER_CMD_ENABLE | \
    FIELD_PREP(SPIFC_A1_USER_CMD_CODE, (op).cmd.opcode) | \
    FIELD_PREP(SPIFC_A1_USER_CMD_MODE, ilog2((op).cmd.buswidth)))

    SPIFC_A1_USER_ADDR_ENABLE | \
    FIELD_PREP(SPIFC_A1_USER_ADDR_MODE, ilog2((op).addr.buswidth)) | \
    FIELD_PREP(SPIFC_A1_USER_ADDR_BYTES, (op).addr.nbytes - 1))

    SPIFC_A1_USER_DUMMY_ENABLE | \
    FIELD_PREP(SPIFC_A1_USER_DUMMY_MODE, ilog2((op).dummy.buswidth)) | \
    FIELD_PREP(SPIFC_A1_USER_DUMMY_CLK_SYCLES, (op).dummy.nbytes << 3))

    SPIFC_A1_TSHWL_VAL | SPIFC_A1_TSHSL2_VAL | \
    SPIFC_A1_TSHSL1_VAL | SPIFC_A1_TWHSL_VAL)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amlogic_spifc_a1 {
    pub ctrl: *mut spi_controller,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub curr_speed_hz: u32,
}

#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_request(spifc: *mut amlogic_spifc_a1, read: bool) -> c_int {
    static int amlogic_spifc_a1_request(struct amlogic_spifc_a1 *spifc, bool read)
    {
    u32 mask = SPIFC_A1_USER_REQUEST_FINISH |
    (read ? SPIFC_A1_USER_DATA_UPDATED : 0);
    u32 val;
    writel(SPIFC_A1_USER_REQUEST_ENABLE,
    spifc.base + SPIFC_A1_USER_CTRL0_REG);
    return readl_poll_timeout(spifc.base + SPIFC_A1_USER_CTRL0_REG,
    val, (val & mask) == mask, 0,
    200 * USEC_PER_MSEC);
    }
    static void amlogic_spifc_a1_drain_buffer(struct amlogic_spifc_a1 *spifc,
    char *buf, u32 len)
    {
    u32 data;
    let mut count: u32 = len / sizeof(data);
    let mut pad: u32 = len % sizeof(data);
    writel(SPIFC_A1_DBUF_AUTO_UPDATE_ADDR,
    spifc.base + SPIFC_A1_DBUF_CTRL_REG);
    ioread32_rep(spifc.base + SPIFC_A1_DBUF_DATA_REG, buf, count);
    if (pad) {
    data = readl(spifc.base + SPIFC_A1_DBUF_DATA_REG);
    memcpy(buf + len - pad, &data, pad);
    }
    }
    static void amlogic_spifc_a1_fill_buffer(struct amlogic_spifc_a1 *spifc,
    const char *buf, u32 len)
    {
    u32 data;
    let mut count: u32 = len / sizeof(data);
    let mut pad: u32 = len % sizeof(data);
    writel(SPIFC_A1_DBUF_DIR | SPIFC_A1_DBUF_AUTO_UPDATE_ADDR,
    spifc.base + SPIFC_A1_DBUF_CTRL_REG);
    iowrite32_rep(spifc.base + SPIFC_A1_DBUF_DATA_REG, buf, count);
    if (pad) {
    memcpy(&data, buf + len - pad, pad);
    writel(data, spifc.base + SPIFC_A1_DBUF_DATA_REG);
    }
    }
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_user_init(spifc: *mut amlogic_spifc_a1) {
    static void amlogic_spifc_a1_user_init(struct amlogic_spifc_a1 *spifc)
    {
    writel(0, spifc.base + SPIFC_A1_USER_CTRL0_REG);
    writel(0, spifc.base + SPIFC_A1_USER_CTRL1_REG);
    writel(0, spifc.base + SPIFC_A1_USER_CTRL2_REG);
    writel(0, spifc.base + SPIFC_A1_USER_CTRL3_REG);
    }
    static void amlogic_spifc_a1_set_cmd(struct amlogic_spifc_a1 *spifc,
    u32 cmd_cfg)
    {
    u32 val;
    val = readl(spifc.base + SPIFC_A1_USER_CTRL1_REG);
    val &= ~(SPIFC_A1_USER_CMD_MODE | SPIFC_A1_USER_CMD_CODE);
    val |= cmd_cfg;
    writel(val, spifc.base + SPIFC_A1_USER_CTRL1_REG);
    }
    static void amlogic_spifc_a1_set_addr(struct amlogic_spifc_a1 *spifc, u32 addr,
    u32 addr_cfg)
    {
    u32 val;
    writel(addr, spifc.base + SPIFC_A1_USER_ADDR_REG);
    val = readl(spifc.base + SPIFC_A1_USER_CTRL1_REG);
    val &= ~(SPIFC_A1_USER_ADDR_MODE | SPIFC_A1_USER_ADDR_BYTES);
    val |= addr_cfg;
    writel(val, spifc.base + SPIFC_A1_USER_CTRL1_REG);
    }
    static void amlogic_spifc_a1_set_dummy(struct amlogic_spifc_a1 *spifc,
    u32 dummy_cfg)
    {
    let mut val: u32 = readl(spifc.base + SPIFC_A1_USER_CTRL2_REG);
    val &= ~(SPIFC_A1_USER_DUMMY_MODE | SPIFC_A1_USER_DUMMY_CLK_SYCLES);
    val |= dummy_cfg;
    writel(val, spifc.base + SPIFC_A1_USER_CTRL2_REG);
    }
    static int amlogic_spifc_a1_read(struct amlogic_spifc_a1 *spifc, void *buf,
    u32 size, u32 mode)
    {
    let mut val: u32 = readl(spifc.base + SPIFC_A1_USER_CTRL3_REG);
    int ret;
    val |= SPIFC_A1_USER_DIN_ENABLE;
    FIELD_MODIFY(SPIFC_A1_USER_DIN_MODE, &val, mode);
    FIELD_MODIFY(SPIFC_A1_USER_DIN_BYTES, &val, size);
    writel(val, spifc.base + SPIFC_A1_USER_CTRL3_REG);
    ret = amlogic_spifc_a1_request(spifc, true);
    if (!ret)
    amlogic_spifc_a1_drain_buffer(spifc, buf, size);
    return ret;
    }
    static int amlogic_spifc_a1_write(struct amlogic_spifc_a1 *spifc,
    const void *buf, u32 size, u32 mode)
    {
    u32 val;
    amlogic_spifc_a1_fill_buffer(spifc, buf, size);
    val = readl(spifc.base + SPIFC_A1_USER_CTRL1_REG);
    val &= ~(SPIFC_A1_USER_DOUT_MODE | SPIFC_A1_USER_DOUT_BYTES);
    val |= FIELD_PREP(SPIFC_A1_USER_DOUT_MODE, mode);
    val |= FIELD_PREP(SPIFC_A1_USER_DOUT_BYTES, size);
    val |= SPIFC_A1_USER_DOUT_ENABLE;
    writel(val, spifc.base + SPIFC_A1_USER_CTRL1_REG);
    return amlogic_spifc_a1_request(spifc, false);
    }
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_set_freq(spifc: *mut amlogic_spifc_a1, freq: u32) -> c_int {
    static int amlogic_spifc_a1_set_freq(struct amlogic_spifc_a1 *spifc, u32 freq)
    {
    int ret;
    if (freq == spifc.curr_speed_hz)
    return 0;
    ret = clk_set_rate(spifc.clk, freq);
    if (ret)
    return ret;
    spifc.curr_speed_hz = freq;
    return 0;
    }
    static int amlogic_spifc_a1_exec_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct amlogic_spifc_a1 *spifc =
    spi_controller_get_devdata(mem.spi.controller);
    let mut data_size: usize = op.data.nbytes;
    int ret;
    ret = amlogic_spifc_a1_set_freq(spifc, op.max_freq);
    if (ret)
    return ret;
    amlogic_spifc_a1_user_init(spifc);
    amlogic_spifc_a1_set_cmd(spifc, SPIFC_A1_USER_CMD(op));
    if (op.addr.nbytes)
    amlogic_spifc_a1_set_addr(spifc, op.addr.val,
    SPIFC_A1_USER_ADDR(op));
    if (op.dummy.nbytes)
    amlogic_spifc_a1_set_dummy(spifc, SPIFC_A1_USER_DUMMY(op));
    if (data_size) {
    let mut mode: u32 = ilog2(op.data.buswidth);
    writel(0, spifc.base + SPIFC_A1_USER_DBUF_ADDR_REG);
    if (op.data.dir == SPI_MEM_DATA_IN)
    ret = amlogic_spifc_a1_read(spifc, op.data.buf.in,
    data_size, mode);
    else
    ret = amlogic_spifc_a1_write(spifc, op.data.buf.out,
    data_size, mode);
    } else {
    ret = amlogic_spifc_a1_request(spifc, false);
    }
    return ret;
    }
    static int amlogic_spifc_a1_adjust_op_size(struct spi_mem *mem,
    struct spi_mem_op *op)
    {
    op.data.nbytes = min(op.data.nbytes, SPIFC_A1_BUFFER_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_hw_init(spifc: *mut amlogic_spifc_a1) {
    static void amlogic_spifc_a1_hw_init(struct amlogic_spifc_a1 *spifc)
    {
    u32 regv;
    regv = readl(spifc.base + SPIFC_A1_AHB_REQ_CTRL_REG);
    regv &= ~(SPIFC_A1_AHB_REQ_ENABLE);
    writel(regv, spifc.base + SPIFC_A1_AHB_REQ_CTRL_REG);
    regv = readl(spifc.base + SPIFC_A1_AHB_CTRL_REG);
    regv &= ~(SPIFC_A1_AHB_BUS_EN);
    writel(regv, spifc.base + SPIFC_A1_AHB_CTRL_REG);
    writel(SPIFC_A1_ACTIMING0_VAL, spifc.base + SPIFC_A1_ACTIMING0_REG);
    writel(0, spifc.base + SPIFC_A1_USER_DBUF_ADDR_REG);
    }
    static const struct spi_controller_mem_ops amlogic_spifc_a1_mem_ops = {
    .exec_op = amlogic_spifc_a1_exec_op,
    .adjust_op_size = amlogic_spifc_a1_adjust_op_size,
    };
    static const struct spi_controller_mem_caps amlogic_spifc_a1_mem_caps = {
    .per_op_freq = true,
    };
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_probe(pdev: *mut platform_device) -> c_int {
    static int amlogic_spifc_a1_probe(struct platform_device *pdev)
    {
    struct spi_controller *ctrl;
    struct amlogic_spifc_a1 *spifc;
    int ret;
    ctrl = devm_spi_alloc_host(&pdev.dev, sizeof(*spifc));
    if (!ctrl)
    return -ENOMEM;
    spifc = spi_controller_get_devdata(ctrl);
    platform_set_drvdata(pdev, spifc);
    spifc.dev = &pdev.dev;
    spifc.ctrl = ctrl;
    spifc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spifc.base))
    return PTR_ERR(spifc.base);
    spifc.clk = devm_clk_get_enabled(spifc.dev, core::ptr::null_mut());
    if (IS_ERR(spifc.clk))
    return dev_err_probe(spifc.dev, PTR_ERR(spifc.clk),
    "unable to get clock\n");
    amlogic_spifc_a1_hw_init(spifc);
    pm_runtime_set_autosuspend_delay(spifc.dev, 500);
    pm_runtime_use_autosuspend(spifc.dev);
    ret = devm_pm_runtime_enable(spifc.dev);
    if (ret)
    return ret;
    ctrl.num_chipselect = 1;
    ctrl.bits_per_word_mask = SPI_BPW_MASK(8);
    ctrl.auto_runtime_pm = true;
    ctrl.mem_ops = &amlogic_spifc_a1_mem_ops;
    ctrl.mem_caps = &amlogic_spifc_a1_mem_caps;
    ctrl.min_speed_hz = SPIFC_A1_MIN_HZ;
    ctrl.max_speed_hz = SPIFC_A1_MAX_HZ;
    ctrl.mode_bits = (SPI_RX_DUAL | SPI_TX_DUAL |
    SPI_RX_QUAD | SPI_TX_QUAD);
    ret = devm_spi_register_controller(spifc.dev, ctrl);
    if (ret)
    return dev_err_probe(spifc.dev, ret,
    "failed to register spi controller\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_suspend(dev: *mut device) -> c_int {
    static int amlogic_spifc_a1_suspend(struct device *dev)
    {
    struct amlogic_spifc_a1 *spifc = dev_get_drvdata(dev);
    int ret;
    ret = spi_controller_suspend(spifc.ctrl);
    if (ret)
    return ret;
    if (!pm_runtime_suspended(dev))
    clk_disable_unprepare(spifc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_resume(dev: *mut device) -> c_int {
    static int amlogic_spifc_a1_resume(struct device *dev)
    {
    struct amlogic_spifc_a1 *spifc = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (!pm_runtime_suspended(dev)) {
    ret = clk_prepare_enable(spifc.clk);
    if (ret)
    return ret;
    }
    amlogic_spifc_a1_hw_init(spifc);
    ret = spi_controller_resume(spifc.ctrl);
    if (ret)
    clk_disable_unprepare(spifc.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_runtime_suspend(dev: *mut device) -> c_int {
    static int amlogic_spifc_a1_runtime_suspend(struct device *dev)
    {
    struct amlogic_spifc_a1 *spifc = dev_get_drvdata(dev);
    clk_disable_unprepare(spifc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amlogic_spifc_a1_runtime_resume(dev: *mut device) -> c_int {
    static int amlogic_spifc_a1_runtime_resume(struct device *dev)
    {
    struct amlogic_spifc_a1 *spifc = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(spifc.clk);
    if (!ret)
    amlogic_spifc_a1_hw_init(spifc);
    return ret;
    }
    static const struct dev_pm_ops amlogic_spifc_a1_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(amlogic_spifc_a1_suspend, amlogic_spifc_a1_resume)
    RUNTIME_PM_OPS(amlogic_spifc_a1_runtime_suspend,
    amlogic_spifc_a1_runtime_resume,
    core::ptr::null_mut())
    };

    static const struct of_device_id amlogic_spifc_a1_dt_match[] = {
    { .compatible = "amlogic,a1-spifc", },
    { },
    };
    MODULE_DEVICE_TABLE(of, amlogic_spifc_a1_dt_match);

    static struct platform_driver amlogic_spifc_a1_driver = {
    .probe	= amlogic_spifc_a1_probe,
    .driver	= {
    .name		= "amlogic-spifc-a1",
    .of_match_table	= of_match_ptr(amlogic_spifc_a1_dt_match),
    .pm		= pm_ptr(&amlogic_spifc_a1_pm_ops),
    },
    };
    module_platform_driver(amlogic_spifc_a1_driver);
    MODULE_AUTHOR("Martin Kurbanov <mmkurbanov@sberdevices.ru>");
    MODULE_DESCRIPTION("Amlogic A1 SPIFC driver");
    MODULE_LICENSE("GPL");
