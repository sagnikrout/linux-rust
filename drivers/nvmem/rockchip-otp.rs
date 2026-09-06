//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/rockchip-otp.c
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
// Rockchip OTP Driver
//
// Copyright (c) 2018 Rockchip Electronics Co. Ltd.
// Author: Finley Xiao <finley.xiao@rock-chips.com>
//

// OTP Register Offsets
pub const OTPC_SBPI_CTRL: c_uint = 0x0020;
pub const OTPC_SBPI_CMD_VALID_PRE: c_uint = 0x0024;
pub const OTPC_SBPI_CS_VALID_PRE: c_uint = 0x0028;
pub const OTPC_SBPI_STATUS: c_uint = 0x002C;
pub const OTPC_USER_CTRL: c_uint = 0x0100;
pub const OTPC_USER_ADDR: c_uint = 0x0104;
pub const OTPC_USER_ENABLE: c_uint = 0x0108;
pub const OTPC_USER_QP: c_uint = 0x0120;
pub const OTPC_USER_Q: c_uint = 0x0124;
pub const OTPC_INT_STATUS: c_uint = 0x0304;
pub const OTPC_SBPI_CMD0_OFFSET: c_uint = 0x1000;
pub const OTPC_SBPI_CMD1_OFFSET: c_uint = 0x1004;
// OTP Register bits and masks

pub const SBPI_DAP_ADDR: c_uint = 0x02;
pub const SBPI_DAP_ADDR_SHIFT: c_int = 8;

pub const SBPI_DAP_CMD_WRF: c_uint = 0xC0;
pub const SBPI_DAP_REG_ECC: c_uint = 0x3A;
pub const SBPI_ECC_ENABLE: c_uint = 0x00;
pub const SBPI_ECC_DISABLE: c_uint = 0x09;

pub const OTPC_TIMEOUT: c_int = 10000;
// RK3588 Register
pub const RK3588_OTPC_AUTO_CTRL: c_uint = 0x04;
pub const RK3588_OTPC_AUTO_EN: c_uint = 0x08;
pub const RK3588_OTPC_INT_ST: c_uint = 0x84;
pub const RK3588_OTPC_DOUT0: c_uint = 0x20;
pub const RK3588_BURST_NUM: c_int = 1;
pub const RK3588_BURST_SHIFT: c_int = 8;
pub const RK3588_ADDR_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_data {
    pub size: c_int,
    pub read_offset: c_int,
    pub word_size: c_int,
    pub clks: *const *const c_char,
    pub num_clks: c_int,
    pub reg_read: nvmem_reg_read_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_otp {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub rst: *mut reset_control,
    pub data: *const rockchip_data,
    pub clks: [clk_bulk_data; ],
}

#[no_mangle]
unsafe extern "C" fn rockchip_otp_reset(otp: *mut rockchip_otp) -> c_int {
    static int rockchip_otp_reset(struct rockchip_otp *otp)
    {
    int ret;
    ret = reset_control_assert(otp.rst);
    if (ret) {
    dev_err(otp.dev, "failed to assert otp phy %d\n", ret);
    return ret;
    }
    udelay(2);
    ret = reset_control_deassert(otp.rst);
    if (ret) {
    dev_err(otp.dev, "failed to deassert otp phy %d\n", ret);
    return ret;
    }
    return 0;
    }
    static int rockchip_otp_wait_status(struct rockchip_otp *otp,
    unsigned int reg, u32 flag)
    {
    let mut status: u32 = 0;
    int ret;
    ret = readl_poll_timeout_atomic(otp.base + reg, status,
    (status & flag), 1, OTPC_TIMEOUT);
    if (ret)
    return ret;
// clean int status
    writel(flag, otp.base + reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_otp_ecc_enable(otp: *mut rockchip_otp, enable: bool) -> c_int {
    static int rockchip_otp_ecc_enable(struct rockchip_otp *otp, bool enable)
    {
    let mut ret: c_int = 0;
    writel(SBPI_DAP_ADDR_MASK | (SBPI_DAP_ADDR << SBPI_DAP_ADDR_SHIFT),
    otp.base + OTPC_SBPI_CTRL);
    writel(SBPI_CMD_VALID_MASK | 0x1, otp.base + OTPC_SBPI_CMD_VALID_PRE);
    writel(SBPI_DAP_CMD_WRF | SBPI_DAP_REG_ECC,
    otp.base + OTPC_SBPI_CMD0_OFFSET);
    if (enable)
    writel(SBPI_ECC_ENABLE, otp.base + OTPC_SBPI_CMD1_OFFSET);
    else
    writel(SBPI_ECC_DISABLE, otp.base + OTPC_SBPI_CMD1_OFFSET);
    writel(SBPI_ENABLE_MASK | SBPI_ENABLE, otp.base + OTPC_SBPI_CTRL);
    ret = rockchip_otp_wait_status(otp, OTPC_INT_STATUS, OTPC_SBPI_DONE);
    if (ret < 0)
    dev_err(otp.dev, "timeout during ecc_enable\n");
    return ret;
    }
    static int px30_otp_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct rockchip_otp *otp = context;
    u8 *buf = val;
    int ret;
    ret = rockchip_otp_reset(otp);
    if (ret) {
    dev_err(otp.dev, "failed to reset otp phy\n");
    return ret;
    }
    ret = rockchip_otp_ecc_enable(otp, false);
    if (ret < 0) {
    dev_err(otp.dev, "rockchip_otp_ecc_enable err\n");
    return ret;
    }
    writel(OTPC_USE_USER | OTPC_USE_USER_MASK, otp.base + OTPC_USER_CTRL);
    udelay(5);
    while (bytes--) {
    writel(offset++ | OTPC_USER_ADDR_MASK,
    otp.base + OTPC_USER_ADDR);
    writel(OTPC_USER_FSM_ENABLE | OTPC_USER_FSM_ENABLE_MASK,
    otp.base + OTPC_USER_ENABLE);
    ret = rockchip_otp_wait_status(otp, OTPC_INT_STATUS, OTPC_USER_DONE);
    if (ret < 0) {
    dev_err(otp.dev, "timeout during read setup\n");
    goto read_end;
    }
// buf++ = readb(otp->base + OTPC_USER_Q);
    }
    read_end:
    writel(0x0 | OTPC_USE_USER_MASK, otp.base + OTPC_USER_CTRL);
    return ret;
    }
    static int rk3568_otp_read(void *context, unsigned int offset, void *val,
    size_t count)
    {
    struct rockchip_otp *otp = context;
    u16 *buf = val;
    u32 otp_qp;
    int ret;
    ret = rockchip_otp_reset(otp);
    if (ret) {
    dev_err(otp.dev, "failed to reset otp phy\n");
    return ret;
    }
    ret = rockchip_otp_ecc_enable(otp, true);
    if (ret) {
    dev_err(otp.dev, "rockchip_otp_ecc_enable err\n");
    return ret;
    }
    writel(OTPC_USE_USER | OTPC_USE_USER_MASK, otp.base + OTPC_USER_CTRL);
    udelay(5);
    while (count--) {
    writel(offset++ | OTPC_USER_ADDR_MASK,
    otp.base + OTPC_USER_ADDR);
    writel(OTPC_USER_FSM_ENABLE | OTPC_USER_FSM_ENABLE_MASK,
    otp.base + OTPC_USER_ENABLE);
    ret = rockchip_otp_wait_status(otp, OTPC_INT_STATUS,
    OTPC_USER_DONE);
    if (ret) {
    dev_err(otp.dev, "timeout during read setup\n");
    goto read_end;
    }
    otp_qp = readl(otp.base + OTPC_USER_QP);
    if (((otp_qp & 0xc0) == 0xc0) || (otp_qp & 0x20)) {
    ret = -EIO;
    dev_err(otp.dev, "ecc check error during read setup\n");
    goto read_end;
    }
// buf++ = readl(otp->base + OTPC_USER_Q);
    }
    read_end:
    writel(0x0 | OTPC_USE_USER_MASK, otp.base + OTPC_USER_CTRL);
    return ret;
    }
    static int rk3588_otp_read(void *context, unsigned int offset,
    void *val, size_t count)
    {
    struct rockchip_otp *otp = context;
    u32 *buf = val;
    int ret;
    while (count--) {
    writel((offset++ << RK3588_ADDR_SHIFT) |
    (RK3588_BURST_NUM << RK3588_BURST_SHIFT),
    otp.base + RK3588_OTPC_AUTO_CTRL);
    writel(RK3588_AUTO_EN, otp.base + RK3588_OTPC_AUTO_EN);
    ret = rockchip_otp_wait_status(otp, RK3588_OTPC_INT_ST,
    RK3588_RD_DONE);
    if (ret) {
    dev_err(otp.dev, "timeout during read setup\n");
    return ret;
    }
// buf++ = readl(otp->base + RK3588_OTPC_DOUT0);
    }
    return ret;
    }
    static int rockchip_otp_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct rockchip_otp *otp = context;
    int ret, word_size;
    if (!otp.data || !otp.data.reg_read)
    return -EINVAL;
    ret = clk_bulk_prepare_enable(otp.data.num_clks, otp.clks);
    if (ret < 0) {
    dev_err(otp.dev, "failed to prepare/enable clks\n");
    return ret;
    }
    offset += otp.data.read_offset;
    word_size = otp.data.word_size;
    if (word_size > 1) {
    unsigned int addr_start, addr_end;
    size_t count;
    u8 *buf;
    addr_start = offset / word_size;
    addr_end = DIV_ROUND_UP(offset + bytes, word_size);
    count = addr_end - addr_start;
    buf = kzalloc(array_size(count, word_size), GFP_KERNEL);
    if (!buf) {
    ret = -ENOMEM;
    goto err;
    }
    ret = otp.data.reg_read(context, addr_start, buf, count);
    if (!ret)
    memcpy(val, buf + (offset % word_size), bytes);
    kfree(buf);
    } else {
    ret = otp.data.reg_read(context, offset, val, bytes);
    }
    err:
    clk_bulk_disable_unprepare(otp.data.num_clks, otp.clks);
    return ret;
    }
    static struct nvmem_config otp_config = {
    .name = "rockchip-otp",
    .owner = THIS_MODULE,
    .add_legacy_fixed_of_cells = true,
    .type = NVMEM_TYPE_OTP,
    .read_only = true,
    .stride = 1,
    .word_size = sizeof(u8),
    .reg_read = rockchip_otp_read,
    };
    static const char * const px30_otp_clocks[] = {
    "otp", "apb_pclk", "phy",
    };
    static const struct rockchip_data px30_data = {
    .size = 0x40,
    .clks = px30_otp_clocks,
    .num_clks = ARRAY_SIZE(px30_otp_clocks),
    .reg_read = px30_otp_read,
    };
    static const char * const rk3528_otp_clocks[] = {
    "otp", "apb_pclk", "sbpi",
    };
    static const struct rockchip_data rk3528_data = {
    .size = 0x80,
    .word_size = sizeof(u16),
    .clks = rk3528_otp_clocks,
    .num_clks = ARRAY_SIZE(rk3528_otp_clocks),
    .reg_read = rk3568_otp_read,
    };
    static const char * const rk3568_otp_clocks[] = {
    "otp", "apb_pclk", "phy", "sbpi",
    };
    static const struct rockchip_data rk3568_data = {
    .size = 0x80,
    .word_size = sizeof(u16),
    .clks = rk3568_otp_clocks,
    .num_clks = ARRAY_SIZE(rk3568_otp_clocks),
    .reg_read = rk3568_otp_read,
    };
    static const struct rockchip_data rk3576_data = {
    .size = 0x100,
    .read_offset = 0x700,
    .word_size = sizeof(u32),
    .clks = px30_otp_clocks,
    .num_clks = ARRAY_SIZE(px30_otp_clocks),
    .reg_read = rk3588_otp_read,
    };
    static const char * const rk3588_otp_clocks[] = {
    "otp", "apb_pclk", "phy", "arb",
    };
    static const struct rockchip_data rk3588_data = {
    .size = 0x400,
    .read_offset = 0xc00,
    .word_size = sizeof(u32),
    .clks = rk3588_otp_clocks,
    .num_clks = ARRAY_SIZE(rk3588_otp_clocks),
    .reg_read = rk3588_otp_read,
    };
    static const struct of_device_id rockchip_otp_match[] = {
    {
    .compatible = "rockchip,px30-otp",
    .data = &px30_data,
    },
    {
    .compatible = "rockchip,rk3308-otp",
    .data = &px30_data,
    },
    {
    .compatible = "rockchip,rk3528-otp",
    .data = &rk3528_data,
    },
    {
    .compatible = "rockchip,rk3562-otp",
    .data = &rk3568_data,
    },
    {
    .compatible = "rockchip,rk3568-otp",
    .data = &rk3568_data,
    },
    {
    .compatible = "rockchip,rk3576-otp",
    .data = &rk3576_data,
    },
    {
    .compatible = "rockchip,rk3588-otp",
    .data = &rk3588_data,
    },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, rockchip_otp_match);
#[no_mangle]
unsafe extern "C" fn rockchip_otp_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_otp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rockchip_otp *otp;
    const struct rockchip_data *data;
    struct nvmem_device *nvmem;
    int ret, i;
    data = of_device_get_match_data(dev);
    if (!data)
    return dev_err_probe(dev, -EINVAL, "failed to get match data\n");
    otp = devm_kzalloc(&pdev.dev, struct_size(otp, clks, data.num_clks),
    GFP_KERNEL);
    if (!otp)
    return -ENOMEM;
    otp.data = data;
    otp.dev = dev;
    otp.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(otp.base))
    return dev_err_probe(dev, PTR_ERR(otp.base),
    "failed to ioremap resource\n");
    for (i = 0; i < data.num_clks; ++i)
    otp.clks[i].id = data.clks[i];
    ret = devm_clk_bulk_get(dev, data.num_clks, otp.clks);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get clocks\n");
    otp.rst = devm_reset_control_array_get_exclusive(dev);
    if (IS_ERR(otp.rst))
    return dev_err_probe(dev, PTR_ERR(otp.rst),
    "failed to get resets\n");
    otp_config.size = data.size;
    otp_config.priv = otp;
    otp_config.dev = dev;
    nvmem = devm_nvmem_register(dev, &otp_config);
    if (IS_ERR(nvmem))
    return dev_err_probe(dev, PTR_ERR(nvmem),
    "failed to register nvmem device\n");
    return 0;
    }
    static struct platform_driver rockchip_otp_driver = {
    .probe = rockchip_otp_probe,
    .driver = {
    .name = "rockchip-otp",
    .of_match_table = rockchip_otp_match,
    },
    };
    module_platform_driver(rockchip_otp_driver);
    MODULE_DESCRIPTION("Rockchip OTP driver");
    MODULE_LICENSE("GPL v2");
