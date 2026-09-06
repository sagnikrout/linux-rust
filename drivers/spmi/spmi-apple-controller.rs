//! Automatically rewritten from C to Rust
//! Source: drivers/spmi/spmi-apple-controller.c
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
// Apple SoC SPMI device driver
//
// Copyright The Asahi Linux Contributors
//
// Inspired by:
// OpenBSD support Copyright (c) 2021 Mark Kettenis <kettenis@openbsd.org>
// Correllium support Copyright (C) 2021 Corellium LLC
// hisi-spmi-controller.c
// spmi-pmic-arb.c Copyright (c) 2021, The Linux Foundation.
//

// SPMI Controller Registers
pub const SPMI_STATUS_REG: c_int = 0;
pub const SPMI_CMD_REG: c_uint = 0x4;
pub const SPMI_RSP_REG: c_uint = 0x8;

pub const REG_POLL_INTERVAL_US: c_int = 10000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_spmi {
    pub regs: *mut void __iomem,
}

    readl_poll_timeout((spmi).regs + (reg), (val), (cond), \
    REG_POLL_INTERVAL_US, REG_POLL_TIMEOUT_US)
#[no_mangle]
pub unsafe extern "C" fn apple_spmi_pack_cmd(opc: u8, sid: u8, saddr: u16, len: usize) -> u32 {
    static inline u32 apple_spmi_pack_cmd(u8 opc, u8 sid, u16 saddr, size_t len)
    {
    return opc | sid << 8 | saddr << 16 | (len - 1) | (1 << 15);
    }
// Wait for Rx FIFO to have something
#[no_mangle]
unsafe extern "C" fn apple_spmi_wait_rx_not_empty(ctrl: *mut spmi_controller) -> c_int {
    static int apple_spmi_wait_rx_not_empty(struct spmi_controller *ctrl)
    {
    struct apple_spmi *spmi = spmi_controller_get_drvdata(ctrl);
    int ret;
    u32 status;
    ret = poll_reg(spmi, SPMI_STATUS_REG, status, !(status & SPMI_RX_FIFO_EMPTY));
    if (ret) {
    dev_err(&ctrl.dev,
    "failed to wait for RX FIFO not empty\n");
    return ret;
    }
    return 0;
    }
    static int spmi_read_cmd(struct spmi_controller *ctrl, u8 opc, u8 sid,
    u16 saddr, u8 *buf, size_t len)
    {
    struct apple_spmi *spmi = spmi_controller_get_drvdata(ctrl);
    let mut spmi_cmd: u32 = apple_spmi_pack_cmd(opc, sid, saddr, len);
    u32 rsp;
    let mut len_read: usize = 0;
    u8 i;
    int ret;
    writel(spmi_cmd, spmi.regs + SPMI_CMD_REG);
    ret = apple_spmi_wait_rx_not_empty(ctrl);
    if (ret)
    return ret;
// Discard SPMI reply status
    readl(spmi.regs + SPMI_RSP_REG);
// Read SPMI data reply
    while (len_read < len) {
    rsp = readl(spmi.regs + SPMI_RSP_REG);
    i = 0;
    while ((len_read < len) && (i < 4)) {
    buf[len_read++] = ((0xff << (8 * i)) & rsp) >> (8 * i);
    i += 1;
    }
    }
    return 0;
    }
    static int spmi_write_cmd(struct spmi_controller *ctrl, u8 opc, u8 sid,
    u16 saddr, const u8 *buf, size_t len)
    {
    struct apple_spmi *spmi = spmi_controller_get_drvdata(ctrl);
    let mut spmi_cmd: u32 = apple_spmi_pack_cmd(opc, sid, saddr, len);
    let mut i: usize = 0, j;
    int ret;
    writel(spmi_cmd, spmi.regs + SPMI_CMD_REG);
    while (i < len) {
    j = 0;
    spmi_cmd = 0;
    while ((j < 4) & (i < len))
    spmi_cmd |= buf[i++] << (j++ * 8);
    writel(spmi_cmd, spmi.regs + SPMI_CMD_REG);
    }
    ret = apple_spmi_wait_rx_not_empty(ctrl);
    if (ret)
    return ret;
// Discard
    readl(spmi.regs + SPMI_RSP_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_spmi_probe(pdev: *mut platform_device) -> c_int {
    static int apple_spmi_probe(struct platform_device *pdev)
    {
    struct apple_spmi *spmi;
    struct spmi_controller *ctrl;
    int ret;
    ctrl = devm_spmi_controller_alloc(&pdev.dev, sizeof(*spmi));
    if (IS_ERR(ctrl))
    return -ENOMEM;
    spmi = spmi_controller_get_drvdata(ctrl);
    spmi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spmi.regs))
    return PTR_ERR(spmi.regs);
    ctrl.dev.of_node = pdev.dev.of_node;
    ctrl.read_cmd = spmi_read_cmd;
    ctrl.write_cmd = spmi_write_cmd;
    ret = devm_spmi_controller_add(&pdev.dev, ctrl);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "spmi_controller_add failed\n");
    return 0;
    }
    static const struct of_device_id apple_spmi_match_table[] = {
    { .compatible = "apple,t8103-spmi", },
    { .compatible = "apple,spmi", },
    {}
    };
    MODULE_DEVICE_TABLE(of, apple_spmi_match_table);
    static struct platform_driver apple_spmi_driver = {
    .probe		= apple_spmi_probe,
    .driver		= {
    .name	= "apple-spmi",
    .of_match_table = apple_spmi_match_table,
    },
    };
    module_platform_driver(apple_spmi_driver);
    MODULE_AUTHOR("Jean-Francois Bortolotti <jeff@borto.fr>");
    MODULE_DESCRIPTION("Apple SoC SPMI driver");
    MODULE_LICENSE("GPL");
