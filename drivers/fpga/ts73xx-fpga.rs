//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/ts73xx-fpga.c
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
// Technologic Systems TS-73xx SBC FPGA loader
//
// Copyright (C) 2016 Florian Fainelli <f.fainelli@gmail.com>
//
// FPGA Manager Driver for the on-board Altera Cyclone II FPGA found on
// TS-7300, heavily based on load_fpga.c in their vendor tree.
//

pub const TS73XX_FPGA_DATA_REG: c_int = 0;
pub const TS73XX_FPGA_CONFIG_REG: c_int = 1;
pub const TS73XX_FPGA_WRITE_DONE: c_uint = 0x1;

pub const TS73XX_FPGA_RESET: c_uint = 0x2;

pub const TS73XX_FPGA_LOAD_OK: c_uint = 0x4;
pub const TS73XX_FPGA_CONFIG_LOAD: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts73xx_fpga_priv {
    pub io_base: *mut void __iomem,
    pub dev: *mut device,
}

    static int ts73xx_fpga_write_init(struct fpga_manager *mgr,
    struct fpga_image_info *info,
    const char *buf, size_t count)
    {
    struct ts73xx_fpga_priv *priv = mgr.priv;
// Reset the FPGA
    writeb(0, priv.io_base + TS73XX_FPGA_CONFIG_REG);
    udelay(TS73XX_FPGA_RESET_LOW_DELAY);
    writeb(TS73XX_FPGA_RESET, priv.io_base + TS73XX_FPGA_CONFIG_REG);
    udelay(TS73XX_FPGA_RESET_HIGH_DELAY);
    return 0;
    }
    static int ts73xx_fpga_write(struct fpga_manager *mgr, const char *buf,
    size_t count)
    {
    struct ts73xx_fpga_priv *priv = mgr.priv;
    let mut i: usize = 0;
    int ret;
    u8 reg;
    while (count--) {
    ret = readb_poll_timeout(priv.io_base + TS73XX_FPGA_CONFIG_REG,
    reg, !(reg & TS73XX_FPGA_WRITE_DONE),
    1, TS73XX_FPGA_WRITE_DONE_TIMEOUT);
    if (ret < 0)
    return ret;
    writeb(buf[i], priv.io_base + TS73XX_FPGA_DATA_REG);
    i++;
    }
    return 0;
    }
    static int ts73xx_fpga_write_complete(struct fpga_manager *mgr,
    struct fpga_image_info *info)
    {
    struct ts73xx_fpga_priv *priv = mgr.priv;
    u8 reg;
    usleep_range(1000, 2000);
    reg = readb(priv.io_base + TS73XX_FPGA_CONFIG_REG);
    reg |= TS73XX_FPGA_CONFIG_LOAD;
    writeb(reg, priv.io_base + TS73XX_FPGA_CONFIG_REG);
    usleep_range(1000, 2000);
    reg = readb(priv.io_base + TS73XX_FPGA_CONFIG_REG);
    reg &= ~TS73XX_FPGA_CONFIG_LOAD;
    writeb(reg, priv.io_base + TS73XX_FPGA_CONFIG_REG);
    reg = readb(priv.io_base + TS73XX_FPGA_CONFIG_REG);
    if ((reg & TS73XX_FPGA_LOAD_OK) != TS73XX_FPGA_LOAD_OK)
    return -ETIMEDOUT;
    return 0;
    }
    static const struct fpga_manager_ops ts73xx_fpga_ops = {
    .write_init	= ts73xx_fpga_write_init,
    .write		= ts73xx_fpga_write,
    .write_complete	= ts73xx_fpga_write_complete,
    };
#[no_mangle]
unsafe extern "C" fn ts73xx_fpga_probe(pdev: *mut platform_device) -> c_int {
    static int ts73xx_fpga_probe(struct platform_device *pdev)
    {
    struct device *kdev = &pdev.dev;
    struct ts73xx_fpga_priv *priv;
    struct fpga_manager *mgr;
    priv = devm_kzalloc(kdev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = kdev;
    priv.io_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.io_base))
    return PTR_ERR(priv.io_base);
    mgr = devm_fpga_mgr_register(kdev, "TS-73xx FPGA Manager",
    &ts73xx_fpga_ops, priv);
    return PTR_ERR_OR_ZERO(mgr);
    }
    static const struct of_device_id ts73xx_fpga_of_match[] = {
    { .compatible = "technologic,ts7300-fpga" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ts73xx_fpga_of_match);
    static struct platform_driver ts73xx_fpga_driver = {
    .driver	= {
    .name	= "ts73xx-fpga-mgr",
    .of_match_table = ts73xx_fpga_of_match,
    },
    .probe	= ts73xx_fpga_probe,
    };
    module_platform_driver(ts73xx_fpga_driver);
    MODULE_AUTHOR("Florian Fainelli <f.fainelli@gmail.com>");
    MODULE_DESCRIPTION("TS-73xx FPGA Manager driver");
    MODULE_LICENSE("GPL v2");
