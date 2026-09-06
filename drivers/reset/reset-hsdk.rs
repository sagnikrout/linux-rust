//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-hsdk.c
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
// Copyright (C) 2017 Synopsys.
//
// Synopsys HSDK Development platform reset driver.
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsdk_rst {
    pub regs_ctl: *mut void __iomem,
    pub regs_rst: *mut void __iomem,
    pub lock: spinlock_t,
    pub rcdev: reset_controller_dev,
}

    static const u32 rst_map[] = {
    BIT(16), /* APB_RST  */
    BIT(17), /* AXI_RST  */
    BIT(18), /* ETH_RST  */
    BIT(19), /* USB_RST  */
    BIT(20), /* SDIO_RST */
    BIT(21), /* HDMI_RST */
    BIT(22), /* GFX_RST  */
    BIT(25), /* DMAC_RST */
    BIT(31), /* EBI_RST  */
    };

pub const CGU_SYS_RST_CTRL: c_uint = 0x0;
pub const CGU_IP_SW_RESET: c_uint = 0x0;
pub const CGU_IP_SW_RESET_DELAY_SHIFT: c_int = 16;

pub const CGU_IP_SW_RESET_DELAY: c_int = 0;

pub const SW_RESET_TIMEOUT: c_int = 10000;
#[no_mangle]
unsafe extern "C" fn hsdk_reset_config(rst: *mut hsdk_rst, id: c_ulong) {
    static void hsdk_reset_config(struct hsdk_rst *rst, unsigned long id)
    {
    writel(rst_map[id], rst.regs_ctl + CGU_SYS_RST_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hsdk_reset_do(rst: *mut hsdk_rst) -> c_int {
    static int hsdk_reset_do(struct hsdk_rst *rst)
    {
    u32 reg;
    reg = readl(rst.regs_rst + CGU_IP_SW_RESET);
    reg &= ~CGU_IP_SW_RESET_DELAY_MASK;
    reg |= CGU_IP_SW_RESET_DELAY << CGU_IP_SW_RESET_DELAY_SHIFT;
    reg |= CGU_IP_SW_RESET_RESET;
    writel(reg, rst.regs_rst + CGU_IP_SW_RESET);
// wait till reset bit is back to 0
    return readl_poll_timeout_atomic(rst.regs_rst + CGU_IP_SW_RESET, reg,
    !(reg & CGU_IP_SW_RESET_RESET), 5, SW_RESET_TIMEOUT);
    }
    static int hsdk_reset_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct hsdk_rst *rst = to_hsdk_rst(rcdev);
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&rst.lock, flags);
    hsdk_reset_config(rst, id);
    ret = hsdk_reset_do(rst);
    spin_unlock_irqrestore(&rst.lock, flags);
    return ret;
    }
    static const struct reset_control_ops hsdk_reset_ops = {
    .reset	= hsdk_reset_reset,
    .deassert = hsdk_reset_reset,
    };
#[no_mangle]
unsafe extern "C" fn hsdk_reset_probe(pdev: *mut platform_device) -> c_int {
    static int hsdk_reset_probe(struct platform_device *pdev)
    {
    struct hsdk_rst *rst;
    rst = devm_kzalloc(&pdev.dev, sizeof(*rst), GFP_KERNEL);
    if (!rst)
    return -ENOMEM;
    rst.regs_ctl = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rst.regs_ctl))
    return PTR_ERR(rst.regs_ctl);
    rst.regs_rst = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(rst.regs_rst))
    return PTR_ERR(rst.regs_rst);
    spin_lock_init(&rst.lock);
    rst.rcdev.owner = THIS_MODULE;
    rst.rcdev.ops = &hsdk_reset_ops;
    rst.rcdev.of_node = pdev.dev.of_node;
    rst.rcdev.nr_resets = HSDK_MAX_RESETS;
    rst.rcdev.of_reset_n_cells = 1;
    return reset_controller_register(&rst.rcdev);
    }
    static const struct of_device_id hsdk_reset_dt_match[] = {
    { .compatible = "snps,hsdk-reset" },
    { },
    };
    static struct platform_driver hsdk_reset_driver = {
    .probe	= hsdk_reset_probe,
    .driver	= {
    .name = "hsdk-reset",
    .of_match_table = hsdk_reset_dt_match,
    },
    };
    builtin_platform_driver(hsdk_reset_driver);
    MODULE_AUTHOR("Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>");
    MODULE_DESCRIPTION("Synopsys HSDK SDP reset driver");
    MODULE_LICENSE("GPL v2");
