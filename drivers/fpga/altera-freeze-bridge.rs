//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/altera-freeze-bridge.c
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
// FPGA Freeze Bridge Controller
//
// Copyright (C) 2016 Altera Corporation. All rights reserved.
//

pub const FREEZE_CSR_STATUS_OFFSET: c_int = 0;
pub const FREEZE_CSR_CTRL_OFFSET: c_int = 4;
pub const FREEZE_CSR_ILLEGAL_REQ_OFFSET: c_int = 8;
pub const FREEZE_CSR_REG_VERSION: c_int = 12;
pub const FREEZE_CSR_SUPPORTED_VERSION: c_int = 2;
pub const FREEZE_CSR_OFFICIAL_VERSION: c_uint = 0xad000003;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_freeze_br_data {
    pub dev: *mut device,
    pub base_addr: *mut void __iomem,
    pub enable: bool,
}

//
// Poll status until status bit is set or we have a timeout.
//
    static int altera_freeze_br_req_ack(struct altera_freeze_br_data *priv,
    u32 timeout, u32 req_ack)
    {
    struct device *dev = priv.dev;
    void __iomem *csr_illegal_req_addr = priv.base_addr +
    FREEZE_CSR_ILLEGAL_REQ_OFFSET;
    u32 status, illegal, ctrl;
    let mut ret: c_int = -ETIMEDOUT;
    do {
    illegal = readl(csr_illegal_req_addr);
    if (illegal) {
    dev_err(dev, "illegal request detected 0x%x", illegal);
    writel(1, csr_illegal_req_addr);
    illegal = readl(csr_illegal_req_addr);
    if (illegal)
    dev_err(dev, "illegal request not cleared 0x%x",
    illegal);
    ret = -EINVAL;
    break;
    }
    status = readl(priv.base_addr + FREEZE_CSR_STATUS_OFFSET);
    dev_dbg(dev, "%s %x %x\n", __func__, status, req_ack);
    status &= req_ack;
    if (status) {
    ctrl = readl(priv.base_addr + FREEZE_CSR_CTRL_OFFSET);
    dev_dbg(dev, "%s request %x acknowledged %x %x\n",
    __func__, req_ack, status, ctrl);
    ret = 0;
    break;
    }
    udelay(1);
    } while (timeout--);
    if (ret == -ETIMEDOUT)
    dev_err(dev, "%s timeout waiting for 0x%x\n",
    __func__, req_ack);
    return ret;
    }
    static int altera_freeze_br_do_freeze(struct altera_freeze_br_data *priv,
    u32 timeout)
    {
    struct device *dev = priv.dev;
    void __iomem *csr_ctrl_addr = priv.base_addr +
    FREEZE_CSR_CTRL_OFFSET;
    u32 status;
    int ret;
    status = readl(priv.base_addr + FREEZE_CSR_STATUS_OFFSET);
    dev_dbg(dev, "%s %d %d\n", __func__, status, readl(csr_ctrl_addr));
    if (status & FREEZE_CSR_STATUS_FREEZE_REQ_DONE) {
    dev_dbg(dev, "%s bridge already disabled %d\n",
    __func__, status);
    return 0;
    } else if (!(status & FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE)) {
    dev_err(dev, "%s bridge not enabled %d\n", __func__, status);
    return -EINVAL;
    }
    writel(FREEZE_CSR_CTRL_FREEZE_REQ, csr_ctrl_addr);
    ret = altera_freeze_br_req_ack(priv, timeout,
    FREEZE_CSR_STATUS_FREEZE_REQ_DONE);
    if (ret)
    writel(0, csr_ctrl_addr);
    else
    writel(FREEZE_CSR_CTRL_RESET_REQ, csr_ctrl_addr);
    return ret;
    }
    static int altera_freeze_br_do_unfreeze(struct altera_freeze_br_data *priv,
    u32 timeout)
    {
    struct device *dev = priv.dev;
    void __iomem *csr_ctrl_addr = priv.base_addr +
    FREEZE_CSR_CTRL_OFFSET;
    u32 status;
    int ret;
    writel(0, csr_ctrl_addr);
    status = readl(priv.base_addr + FREEZE_CSR_STATUS_OFFSET);
    dev_dbg(dev, "%s %d %d\n", __func__, status, readl(csr_ctrl_addr));
    if (status & FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE) {
    dev_dbg(dev, "%s bridge already enabled %d\n",
    __func__, status);
    return 0;
    } else if (!(status & FREEZE_CSR_STATUS_FREEZE_REQ_DONE)) {
    dev_err(dev, "%s bridge not frozen %d\n", __func__, status);
    return -EINVAL;
    }
    writel(FREEZE_CSR_CTRL_UNFREEZE_REQ, csr_ctrl_addr);
    ret = altera_freeze_br_req_ack(priv, timeout,
    FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE);
    status = readl(priv.base_addr + FREEZE_CSR_STATUS_OFFSET);
    dev_dbg(dev, "%s %d %d\n", __func__, status, readl(csr_ctrl_addr));
    writel(0, csr_ctrl_addr);
    return ret;
    }
//
// enable = 1 : allow traffic through the bridge
// enable = 0 : disable traffic through the bridge
//
    static int altera_freeze_br_enable_set(struct fpga_bridge *bridge,
    bool enable)
    {
    struct altera_freeze_br_data *priv = bridge.priv;
    struct fpga_image_info *info = bridge.info;
    let mut timeout: u32 = 0;
    int ret;
    if (enable) {
    if (info)
    timeout = info.enable_timeout_us;
    ret = altera_freeze_br_do_unfreeze(bridge.priv, timeout);
    } else {
    if (info)
    timeout = info.disable_timeout_us;
    ret = altera_freeze_br_do_freeze(bridge.priv, timeout);
    }
    if (!ret)
    priv.enable = enable;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn altera_freeze_br_enable_show(bridge: *mut fpga_bridge) -> c_int {
    static int altera_freeze_br_enable_show(struct fpga_bridge *bridge)
    {
    struct altera_freeze_br_data *priv = bridge.priv;
    return priv.enable;
    }
    static const struct fpga_bridge_ops altera_freeze_br_br_ops = {
    .enable_set = altera_freeze_br_enable_set,
    .enable_show = altera_freeze_br_enable_show,
    };
    static const struct of_device_id altera_freeze_br_of_match[] = {
    { .compatible = "altr,freeze-bridge-controller", },
    {},
    };
    MODULE_DEVICE_TABLE(of, altera_freeze_br_of_match);
#[no_mangle]
unsafe extern "C" fn altera_freeze_br_probe(pdev: *mut platform_device) -> c_int {
    static int altera_freeze_br_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = pdev.dev.of_node;
    void __iomem *base_addr;
    struct altera_freeze_br_data *priv;
    struct fpga_bridge *br;
    u32 status, revision;
    if (!np)
    return -ENODEV;
    base_addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base_addr))
    return PTR_ERR(base_addr);
    revision = readl(base_addr + FREEZE_CSR_REG_VERSION);
    if ((revision != FREEZE_CSR_SUPPORTED_VERSION) &&
    (revision != FREEZE_CSR_OFFICIAL_VERSION)) {
    dev_err(dev,
    "%s unexpected revision 0x%x != 0x%x != 0x%x\n",
    __func__, revision, FREEZE_CSR_SUPPORTED_VERSION,
    FREEZE_CSR_OFFICIAL_VERSION);
    return -EINVAL;
    }
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    status = readl(base_addr + FREEZE_CSR_STATUS_OFFSET);
    if (status & FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE)
    priv.enable = 1;
    priv.base_addr = base_addr;
    br = fpga_bridge_register(dev, FREEZE_BRIDGE_NAME,
    &altera_freeze_br_br_ops, priv);
    if (IS_ERR(br))
    return PTR_ERR(br);
    platform_set_drvdata(pdev, br);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn altera_freeze_br_remove(pdev: *mut platform_device) {
    static void altera_freeze_br_remove(struct platform_device *pdev)
    {
    struct fpga_bridge *br = platform_get_drvdata(pdev);
    fpga_bridge_unregister(br);
    }
    static struct platform_driver altera_freeze_br_driver = {
    .probe = altera_freeze_br_probe,
    .remove = altera_freeze_br_remove,
    .driver = {
    .name	= "altera_freeze_br",
    .of_match_table = altera_freeze_br_of_match,
    },
    };
    module_platform_driver(altera_freeze_br_driver);
    MODULE_DESCRIPTION("Altera Freeze Bridge");
    MODULE_AUTHOR("Alan Tull <atull@opensource.altera.com>");
    MODULE_LICENSE("GPL v2");
