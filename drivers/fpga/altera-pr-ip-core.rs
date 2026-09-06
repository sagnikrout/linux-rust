//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/altera-pr-ip-core.c
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
// Driver for Altera Partial Reconfiguration IP Core
//
// Copyright (C) 2016-2017 Intel Corporation
//
// Based on socfpga-a10.c Copyright (C) 2015-2016 Altera Corporation
// by Alan Tull <atull@opensource.altera.com>
//

pub const ALT_PR_DATA_OFST: c_uint = 0x00;
pub const ALT_PR_CSR_OFST: c_uint = 0x04;

pub const ALT_PR_CSR_STATUS_SFT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_pr_priv {
    pub reg_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn alt_pr_fpga_state(mgr: *mut fpga_manager) -> enum fpga_mgr_states {
    static enum fpga_mgr_states alt_pr_fpga_state(struct fpga_manager *mgr)
    {
    struct alt_pr_priv *priv = mgr.priv;
    const char *err = "unknown";
    let mut ret: enum fpga_mgr_states = FPGA_MGR_STATE_UNKNOWN;
    u32 val;
    val = readl(priv.reg_base + ALT_PR_CSR_OFST);
    val &= ALT_PR_CSR_STATUS_MSK;
    switch (val) {
    case ALT_PR_CSR_STATUS_NRESET:
    return FPGA_MGR_STATE_RESET;
    case ALT_PR_CSR_STATUS_PR_ERR:
    err = "pr error";
    ret = FPGA_MGR_STATE_WRITE_ERR;
    break;
    case ALT_PR_CSR_STATUS_CRC_ERR:
    err = "crc error";
    ret = FPGA_MGR_STATE_WRITE_ERR;
    break;
    case ALT_PR_CSR_STATUS_BAD_BITS:
    err = "bad bits";
    ret = FPGA_MGR_STATE_WRITE_ERR;
    break;
    case ALT_PR_CSR_STATUS_PR_IN_PROG:
    return FPGA_MGR_STATE_WRITE;
    case ALT_PR_CSR_STATUS_PR_SUCCESS:
    return FPGA_MGR_STATE_OPERATING;
    default:
    break;
    }
    dev_err(&mgr.dev, "encountered error code %d (%s) in %s()\n",
    val, err, __func__);
    return ret;
    }
    static int alt_pr_fpga_write_init(struct fpga_manager *mgr,
    struct fpga_image_info *info,
    const char *buf, size_t count)
    {
    struct alt_pr_priv *priv = mgr.priv;
    u32 val;
    if (!(info.flags & FPGA_MGR_PARTIAL_RECONFIG)) {
    dev_err(&mgr.dev, "%s Partial Reconfiguration flag not set\n",
    __func__);
    return -EINVAL;
    }
    val = readl(priv.reg_base + ALT_PR_CSR_OFST);
    if (val & ALT_PR_CSR_PR_START) {
    dev_err(&mgr.dev,
    "%s Partial Reconfiguration already started\n",
    __func__);
    return -EINVAL;
    }
    writel(val | ALT_PR_CSR_PR_START, priv.reg_base + ALT_PR_CSR_OFST);
    return 0;
    }
    static int alt_pr_fpga_write(struct fpga_manager *mgr, const char *buf,
    size_t count)
    {
    struct alt_pr_priv *priv = mgr.priv;
    u32 *buffer_32 = (u32 *)buf;
    let mut i: usize = 0;
    if (!count)
    return -EINVAL;
// Write out the complete 32-bit chunks
    while (count >= sizeof(u32)) {
    writel(buffer_32[i++], priv.reg_base);
    count -= sizeof(u32);
    }
// Write out remaining non 32-bit chunks
    switch (count) {
    case 3:
    writel(buffer_32[i++] & 0x00ffffff, priv.reg_base);
    break;
    case 2:
    writel(buffer_32[i++] & 0x0000ffff, priv.reg_base);
    break;
    case 1:
    writel(buffer_32[i++] & 0x000000ff, priv.reg_base);
    break;
    case 0:
    break;
    default:
// This will never happen
    return -EFAULT;
    }
    if (alt_pr_fpga_state(mgr) == FPGA_MGR_STATE_WRITE_ERR)
    return -EIO;
    return 0;
    }
    static int alt_pr_fpga_write_complete(struct fpga_manager *mgr,
    struct fpga_image_info *info)
    {
    let mut i: u32 = 0;
    do {
    switch (alt_pr_fpga_state(mgr)) {
    case FPGA_MGR_STATE_WRITE_ERR:
    return -EIO;
    case FPGA_MGR_STATE_OPERATING:
    dev_info(&mgr.dev,
    "successful partial reconfiguration\n");
    return 0;
    default:
    break;
    }
    udelay(1);
    } while (info.config_complete_timeout_us > i++);
    dev_err(&mgr.dev, "timed out waiting for write to complete\n");
    return -ETIMEDOUT;
    }
    static const struct fpga_manager_ops alt_pr_ops = {
    .state = alt_pr_fpga_state,
    .write_init = alt_pr_fpga_write_init,
    .write = alt_pr_fpga_write,
    .write_complete = alt_pr_fpga_write_complete,
    };
#[no_mangle]
pub unsafe extern "C" fn alt_pr_register(dev: *mut device, reg_base: *mut void __iomem) -> c_int {
    int alt_pr_register(struct device *dev, void __iomem *reg_base)
    {
    struct alt_pr_priv *priv;
    struct fpga_manager *mgr;
    u32 val;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.reg_base = reg_base;
    val = readl(priv.reg_base + ALT_PR_CSR_OFST);
    dev_dbg(dev, "%s status=%d start=%d\n", __func__,
    (val & ALT_PR_CSR_STATUS_MSK) >> ALT_PR_CSR_STATUS_SFT,
    (int)(val & ALT_PR_CSR_PR_START));
    mgr = devm_fpga_mgr_register(dev, dev_name(dev), &alt_pr_ops, priv);
    return PTR_ERR_OR_ZERO(mgr);
    }
    EXPORT_SYMBOL_GPL(alt_pr_register);
    MODULE_AUTHOR("Matthew Gerlach <matthew.gerlach@linux.intel.com>");
    MODULE_DESCRIPTION("Altera Partial Reconfiguration IP Core");
    MODULE_LICENSE("GPL v2");
