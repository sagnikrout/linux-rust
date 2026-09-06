//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/vf610-ocotp.c
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
// Copyright (C) 2015 Toradex AG.
//
// Author: Sanchayan Maity <sanchayan.maity@toradex.com>
//
// Based on the barebox ocotp driver,
// Copyright (c) 2010 Baruch Siach <baruch@tkos.co.il>
// Orex Computed Radiography
//

// OCOTP Register Offsets
pub const OCOTP_CTRL_REG: c_uint = 0x00;
pub const OCOTP_CTRL_SET: c_uint = 0x04;
pub const OCOTP_CTRL_CLR: c_uint = 0x08;
pub const OCOTP_TIMING: c_uint = 0x10;
pub const OCOTP_DATA: c_uint = 0x20;
pub const OCOTP_READ_CTRL_REG: c_uint = 0x30;
pub const OCOTP_READ_FUSE_DATA: c_uint = 0x40;
// OCOTP Register bits and masks
pub const OCOTP_CTRL_WR_UNLOCK: c_int = 16;
pub const OCOTP_CTRL_WR_UNLOCK_KEY: c_uint = 0x3E77;

pub const OCOTP_CTRL_ADDR: c_int = 0;

pub const OCOTP_TIMING_STROBE_READ: c_int = 16;

pub const OCOTP_TIMING_RELAX: c_int = 12;

pub const OCOTP_TIMING_STROBE_PROG: c_int = 0;

pub const OCOTP_READ_CTRL_READ_FUSE: c_uint = 0x1;
pub const VF610_OCOTP_TIMEOUT: c_int = 100000;

pub const DEF_RELAX: c_int = 20;
    static const int base_to_fuse_addr_mappings[][2] = {
    {0x400, 0x00},
    {0x410, 0x01},
    {0x420, 0x02},
    {0x450, 0x05},
    {0x4F0, 0x0F},
    {0x600, 0x20},
    {0x610, 0x21},
    {0x620, 0x22},
    {0x630, 0x23},
    {0x640, 0x24},
    {0x650, 0x25},
    {0x660, 0x26},
    {0x670, 0x27},
    {0x6F0, 0x2F},
    {0x880, 0x38},
    {0x890, 0x39},
    {0x8A0, 0x3A},
    {0x8B0, 0x3B},
    {0x8C0, 0x3C},
    {0x8D0, 0x3D},
    {0x8E0, 0x3E},
    {0x8F0, 0x3F},
    {0xC80, 0x78},
    {0xC90, 0x79},
    {0xCA0, 0x7A},
    {0xCB0, 0x7B},
    {0xCC0, 0x7C},
    {0xCD0, 0x7D},
    {0xCE0, 0x7E},
    {0xCF0, 0x7F},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf610_ocotp {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub nvmem: *mut nvmem_device,
    pub timing: c_int,
}

#[no_mangle]
unsafe extern "C" fn vf610_ocotp_wait_busy(base: *mut void __iomem) -> c_int {
    static int vf610_ocotp_wait_busy(void __iomem *base)
    {
    let mut timeout: c_int = VF610_OCOTP_TIMEOUT;
    while ((readl(base) & OCOTP_CTRL_BUSY) && --timeout)
    udelay(10);
    if (!timeout) {
    writel(OCOTP_CTRL_ERR, base + OCOTP_CTRL_CLR);
    return -ETIMEDOUT;
    }
    udelay(10);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vf610_ocotp_calculate_timing(ocotp_dev: *mut vf610_ocotp) -> c_int {
    static int vf610_ocotp_calculate_timing(struct vf610_ocotp *ocotp_dev)
    {
    u32 clk_rate;
    u32 relax, strobe_read, strobe_prog;
    u32 timing;
    clk_rate = clk_get_rate(ocotp_dev.clk);
// Refer section OTP read/write timing parameters in TRM
    relax = clk_rate / (1000000000 / DEF_RELAX) - 1;
    strobe_prog = clk_rate / (1000000000 / 10000) + 2 * (DEF_RELAX + 1) - 1;
    strobe_read = clk_rate / (1000000000 / 40) + 2 * (DEF_RELAX + 1) - 1;
    timing = BF(relax, OCOTP_TIMING_RELAX);
    timing |= BF(strobe_read, OCOTP_TIMING_STROBE_READ);
    timing |= BF(strobe_prog, OCOTP_TIMING_STROBE_PROG);
    return timing;
    }
#[no_mangle]
unsafe extern "C" fn vf610_get_fuse_address(base_addr_offset: c_int) -> c_int {
    static int vf610_get_fuse_address(int base_addr_offset)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(base_to_fuse_addr_mappings); i++) {
    if (base_to_fuse_addr_mappings[i][0] == base_addr_offset)
    return base_to_fuse_addr_mappings[i][1];
    }
    return -EINVAL;
    }
    static int vf610_ocotp_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct vf610_ocotp *ocotp = context;
    void __iomem *base = ocotp.base;
    u32 reg, *buf = val;
    int fuse_addr;
    int ret;
    while (bytes > 0) {
    fuse_addr = vf610_get_fuse_address(offset);
    if (fuse_addr > 0) {
    writel(ocotp.timing, base + OCOTP_TIMING);
    ret = vf610_ocotp_wait_busy(base + OCOTP_CTRL_REG);
    if (ret)
    return ret;
    reg = readl(base + OCOTP_CTRL_REG);
    reg &= ~OCOTP_CTRL_ADDR_MASK;
    reg &= ~OCOTP_CTRL_WR_UNLOCK_MASK;
    reg |= BF(fuse_addr, OCOTP_CTRL_ADDR);
    writel(reg, base + OCOTP_CTRL_REG);
    writel(OCOTP_READ_CTRL_READ_FUSE,
    base + OCOTP_READ_CTRL_REG);
    ret = vf610_ocotp_wait_busy(base + OCOTP_CTRL_REG);
    if (ret)
    return ret;
    if (readl(base) & OCOTP_CTRL_ERR) {
    dev_dbg(ocotp.dev, "Error reading from fuse address %x\n",
    fuse_addr);
    writel(OCOTP_CTRL_ERR, base + OCOTP_CTRL_CLR);
    }
//
// In case of error, we do not abort and expect to read
// 0xBADABADA as mentioned by the TRM. We just read this
// value and return.
//
// buf = readl(base + OCOTP_READ_FUSE_DATA);
    } else {
// buf = 0;
    }
    buf++;
    bytes -= 4;
    offset += 4;
    }
    return 0;
    }
    static struct nvmem_config ocotp_config = {
    .name = "ocotp",
    .stride = 4,
    .word_size = 4,
    .reg_read = vf610_ocotp_read,
    };
    static const struct of_device_id ocotp_of_match[] = {
    { .compatible = "fsl,vf610-ocotp", },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, ocotp_of_match);
#[no_mangle]
unsafe extern "C" fn vf610_ocotp_probe(pdev: *mut platform_device) -> c_int {
    static int vf610_ocotp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource *res;
    struct vf610_ocotp *ocotp_dev;
    ocotp_dev = devm_kzalloc(dev, sizeof(struct vf610_ocotp), GFP_KERNEL);
    if (!ocotp_dev)
    return -ENOMEM;
    ocotp_dev.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(ocotp_dev.base))
    return PTR_ERR(ocotp_dev.base);
    ocotp_dev.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(ocotp_dev.clk)) {
    dev_err(dev, "failed getting clock, err = %ld\n",
    PTR_ERR(ocotp_dev.clk));
    return PTR_ERR(ocotp_dev.clk);
    }
    ocotp_dev.dev = dev;
    ocotp_dev.timing = vf610_ocotp_calculate_timing(ocotp_dev);
    ocotp_config.size = resource_size(res);
    ocotp_config.priv = ocotp_dev;
    ocotp_config.dev = dev;
    ocotp_dev.nvmem = devm_nvmem_register(dev, &ocotp_config);
    return PTR_ERR_OR_ZERO(ocotp_dev.nvmem);
    }
    static struct platform_driver vf610_ocotp_driver = {
    .probe = vf610_ocotp_probe,
    .driver = {
    .name = "vf610-ocotp",
    .of_match_table = ocotp_of_match,
    },
    };
    module_platform_driver(vf610_ocotp_driver);
    MODULE_AUTHOR("Sanchayan Maity <sanchayan.maity@toradex.com>");
    MODULE_DESCRIPTION("Vybrid OCOTP driver");
    MODULE_LICENSE("GPL v2");
