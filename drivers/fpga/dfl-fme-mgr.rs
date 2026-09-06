//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/dfl-fme-mgr.c
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
// FPGA Manager Driver for FPGA Management Engine (FME)
//
// Copyright (C) 2017-2018 Intel Corporation, Inc.
//
// Authors:
// Kang Luwei <luwei.kang@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
// Wu Hao <hao.wu@intel.com>
// Joseph Grecco <joe.grecco@intel.com>
// Enno Luebbers <enno.luebbers@intel.com>
// Tim Whisonant <tim.whisonant@intel.com>
// Ananda Ravuri <ananda.ravuri@intel.com>
// Christopher Rauer <christopher.rauer@intel.com>
// Henry Mitchel <henry.mitchel@intel.com>
//

// FME Partial Reconfiguration Sub Feature Register Set
pub const FME_PR_DFH: c_uint = 0x0;
pub const FME_PR_CTRL: c_uint = 0x8;
pub const FME_PR_STS: c_uint = 0x10;
pub const FME_PR_DATA: c_uint = 0x18;
pub const FME_PR_ERR: c_uint = 0x20;
pub const FME_PR_INTFC_ID_L: c_uint = 0xA8;
pub const FME_PR_INTFC_ID_H: c_uint = 0xB0;
// FME PR Control Register Bitfield

// FME PR Status Register Bitfield
// Number of available entries in HW queue inside the PR engine.

pub const FME_PR_STS_PR_STS_IDLE: c_int = 0;

// FME PR Data Register Bitfield
// PR data from the raw-binary file.

// FME PR Error Register
// PR Operation errors detected.

// CRC error detected.

// Incompatible PR bitstream detected.

// PR data push protocol violated.

// PR data fifo overflow error detected

pub const PR_WAIT_TIMEOUT: c_int = 8000000;
pub const PR_HOST_STATUS_IDLE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fme_mgr_priv {
    pub ioaddr: *mut void __iomem,
    pub pr_error: u64,
}

#[no_mangle]
unsafe extern "C" fn pr_error_to_mgr_status(err: u64) -> u64 {
    static u64 pr_error_to_mgr_status(u64 err)
    {
    let mut status: u64 = 0;
    if (err & FME_PR_ERR_OPERATION_ERR)
    status |= FPGA_MGR_STATUS_OPERATION_ERR;
    if (err & FME_PR_ERR_CRC_ERR)
    status |= FPGA_MGR_STATUS_CRC_ERR;
    if (err & FME_PR_ERR_INCOMPATIBLE_BS)
    status |= FPGA_MGR_STATUS_INCOMPATIBLE_IMAGE_ERR;
    if (err & FME_PR_ERR_PROTOCOL_ERR)
    status |= FPGA_MGR_STATUS_IP_PROTOCOL_ERR;
    if (err & FME_PR_ERR_FIFO_OVERFLOW)
    status |= FPGA_MGR_STATUS_FIFO_OVERFLOW_ERR;
    return status;
    }
#[no_mangle]
unsafe extern "C" fn fme_mgr_pr_error_handle(fme_pr: *mut void __iomem) -> u64 {
    static u64 fme_mgr_pr_error_handle(void __iomem *fme_pr)
    {
    u64 pr_status, pr_error;
    pr_status = readq(fme_pr + FME_PR_STS);
    if (!(pr_status & FME_PR_STS_PR_STS))
    return 0;
    pr_error = readq(fme_pr + FME_PR_ERR);
    writeq(pr_error, fme_pr + FME_PR_ERR);
    return pr_error;
    }
    static int fme_mgr_write_init(struct fpga_manager *mgr,
    struct fpga_image_info *info,
    const char *buf, size_t count)
    {
    struct device *dev = &mgr.dev;
    struct fme_mgr_priv *priv = mgr.priv;
    void __iomem *fme_pr = priv.ioaddr;
    u64 pr_ctrl, pr_status;
    if (!(info.flags & FPGA_MGR_PARTIAL_RECONFIG)) {
    dev_err(dev, "only supports partial reconfiguration.\n");
    return -EINVAL;
    }
    dev_dbg(dev, "resetting PR before initiated PR\n");
    pr_ctrl = readq(fme_pr + FME_PR_CTRL);
    pr_ctrl |= FME_PR_CTRL_PR_RST;
    writeq(pr_ctrl, fme_pr + FME_PR_CTRL);
    if (readq_poll_timeout(fme_pr + FME_PR_CTRL, pr_ctrl,
    pr_ctrl & FME_PR_CTRL_PR_RSTACK, 1,
    PR_WAIT_TIMEOUT)) {
    dev_err(dev, "PR Reset ACK timeout\n");
    return -ETIMEDOUT;
    }
    pr_ctrl = readq(fme_pr + FME_PR_CTRL);
    pr_ctrl &= ~FME_PR_CTRL_PR_RST;
    writeq(pr_ctrl, fme_pr + FME_PR_CTRL);
    dev_dbg(dev,
    "waiting for PR resource in HW to be initialized and ready\n");
    if (readq_poll_timeout(fme_pr + FME_PR_STS, pr_status,
    (pr_status & FME_PR_STS_PR_STS) ==
    FME_PR_STS_PR_STS_IDLE, 1, PR_WAIT_TIMEOUT)) {
    dev_err(dev, "PR Status timeout\n");
    priv.pr_error = fme_mgr_pr_error_handle(fme_pr);
    return -ETIMEDOUT;
    }
    dev_dbg(dev, "check and clear previous PR error\n");
    priv.pr_error = fme_mgr_pr_error_handle(fme_pr);
    if (priv.pr_error)
    dev_dbg(dev, "previous PR error detected %llx\n",
    (unsigned long long)priv.pr_error);
    dev_dbg(dev, "set PR port ID\n");
    pr_ctrl = readq(fme_pr + FME_PR_CTRL);
    pr_ctrl &= ~FME_PR_CTRL_PR_RGN_ID;
    pr_ctrl |= FIELD_PREP(FME_PR_CTRL_PR_RGN_ID, info.region_id);
    writeq(pr_ctrl, fme_pr + FME_PR_CTRL);
    return 0;
    }
    static int fme_mgr_write(struct fpga_manager *mgr,
    const char *buf, size_t count)
    {
    struct device *dev = &mgr.dev;
    struct fme_mgr_priv *priv = mgr.priv;
    void __iomem *fme_pr = priv.ioaddr;
    u64 pr_ctrl, pr_status, pr_data;
    let mut delay: c_int = 0, pr_credit, i = 0;
    dev_dbg(dev, "start request\n");
    pr_ctrl = readq(fme_pr + FME_PR_CTRL);
    pr_ctrl |= FME_PR_CTRL_PR_START;
    writeq(pr_ctrl, fme_pr + FME_PR_CTRL);
    dev_dbg(dev, "pushing data from bitstream to HW\n");
//
// driver can push data to PR hardware using PR_DATA register once HW
// has enough pr_credit (> 1), pr_credit reduces one for every 32bit
// pr data write to PR_DATA register. If pr_credit <= 1, driver needs
// to wait for enough pr_credit from hardware by polling.
//
    pr_status = readq(fme_pr + FME_PR_STS);
    pr_credit = FIELD_GET(FME_PR_STS_PR_CREDIT, pr_status);
    while (count > 0) {
    while (pr_credit <= 1) {
    if (delay++ > PR_WAIT_TIMEOUT) {
    dev_err(dev, "PR_CREDIT timeout\n");
    return -ETIMEDOUT;
    }
    udelay(1);
    pr_status = readq(fme_pr + FME_PR_STS);
    pr_credit = FIELD_GET(FME_PR_STS_PR_CREDIT, pr_status);
    }
    if (count < 4) {
    dev_err(dev, "Invalid PR bitstream size\n");
    return -EINVAL;
    }
    pr_data = 0;
    pr_data |= FIELD_PREP(FME_PR_DATA_PR_DATA_RAW,
// (((u32 *)buf) + i));
    writeq(pr_data, fme_pr + FME_PR_DATA);
    count -= 4;
    pr_credit--;
    i++;
    }
    return 0;
    }
    static int fme_mgr_write_complete(struct fpga_manager *mgr,
    struct fpga_image_info *info)
    {
    struct device *dev = &mgr.dev;
    struct fme_mgr_priv *priv = mgr.priv;
    void __iomem *fme_pr = priv.ioaddr;
    u64 pr_ctrl;
    pr_ctrl = readq(fme_pr + FME_PR_CTRL);
    pr_ctrl |= FME_PR_CTRL_PR_COMPLETE;
    writeq(pr_ctrl, fme_pr + FME_PR_CTRL);
    dev_dbg(dev, "green bitstream push complete\n");
    dev_dbg(dev, "waiting for HW to release PR resource\n");
    if (readq_poll_timeout(fme_pr + FME_PR_CTRL, pr_ctrl,
    !(pr_ctrl & FME_PR_CTRL_PR_START), 1,
    PR_WAIT_TIMEOUT)) {
    dev_err(dev, "PR Completion ACK timeout.\n");
    return -ETIMEDOUT;
    }
    dev_dbg(dev, "PR operation complete, checking status\n");
    priv.pr_error = fme_mgr_pr_error_handle(fme_pr);
    if (priv.pr_error) {
    dev_dbg(dev, "PR error detected %llx\n",
    (unsigned long long)priv.pr_error);
    return -EIO;
    }
    dev_dbg(dev, "PR done successfully\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fme_mgr_status(mgr: *mut fpga_manager) -> u64 {
    static u64 fme_mgr_status(struct fpga_manager *mgr)
    {
    struct fme_mgr_priv *priv = mgr.priv;
    return pr_error_to_mgr_status(priv.pr_error);
    }
    static const struct fpga_manager_ops fme_mgr_ops = {
    .write_init = fme_mgr_write_init,
    .write = fme_mgr_write,
    .write_complete = fme_mgr_write_complete,
    .status = fme_mgr_status,
    };
    static void fme_mgr_get_compat_id(void __iomem *fme_pr,
    struct fpga_compat_id *id)
    {
    id.id_l = readq(fme_pr + FME_PR_INTFC_ID_L);
    id.id_h = readq(fme_pr + FME_PR_INTFC_ID_H);
    }
#[no_mangle]
unsafe extern "C" fn fme_mgr_probe(pdev: *mut platform_device) -> c_int {
    static int fme_mgr_probe(struct platform_device *pdev)
    {
    struct dfl_fme_mgr_pdata *pdata = dev_get_platdata(&pdev.dev);
    let mut info: fpga_manager_info = { 0 };
    struct device *dev = &pdev.dev;
    struct fme_mgr_priv *priv;
    struct fpga_manager *mgr;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    if (pdata.ioaddr)
    priv.ioaddr = pdata.ioaddr;
    if (!priv.ioaddr) {
    priv.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.ioaddr))
    return PTR_ERR(priv.ioaddr);
    }
    info.name = "DFL FME FPGA Manager";
    info.mops = &fme_mgr_ops;
    info.priv = priv;
    info.compat_id = devm_kzalloc(dev, sizeof(*info.compat_id), GFP_KERNEL);
    if (!info.compat_id)
    return -ENOMEM;
    fme_mgr_get_compat_id(priv.ioaddr, info.compat_id);
    mgr = devm_fpga_mgr_register_full(dev, &info);
    return PTR_ERR_OR_ZERO(mgr);
    }
    static struct platform_driver fme_mgr_driver = {
    .driver	= {
    .name    = DFL_FPGA_FME_MGR,
    },
    .probe   = fme_mgr_probe,
    };
    module_platform_driver(fme_mgr_driver);
    MODULE_DESCRIPTION("FPGA Manager for DFL FPGA Management Engine");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:dfl-fme-mgr");
