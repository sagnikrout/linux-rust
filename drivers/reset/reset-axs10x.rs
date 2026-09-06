//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-axs10x.c
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
// Synopsys AXS10x reset driver.
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const AXS10X_MAX_RESETS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axs10x_rst {
    pub regs_rst: *mut void __iomem,
    pub lock: spinlock_t,
    pub rcdev: reset_controller_dev,
}

    static int axs10x_reset_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct axs10x_rst *rst = to_axs10x_rst(rcdev);
    unsigned long flags;
    spin_lock_irqsave(&rst.lock, flags);
    writel(BIT(id), rst.regs_rst);
    spin_unlock_irqrestore(&rst.lock, flags);
    return 0;
    }
    static const struct reset_control_ops axs10x_reset_ops = {
    .reset	= axs10x_reset_reset,
    };
#[no_mangle]
unsafe extern "C" fn axs10x_reset_probe(pdev: *mut platform_device) -> c_int {
    static int axs10x_reset_probe(struct platform_device *pdev)
    {
    struct axs10x_rst *rst;
    rst = devm_kzalloc(&pdev.dev, sizeof(*rst), GFP_KERNEL);
    if (!rst)
    return -ENOMEM;
    rst.regs_rst = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rst.regs_rst))
    return PTR_ERR(rst.regs_rst);
    spin_lock_init(&rst.lock);
    rst.rcdev.owner = THIS_MODULE;
    rst.rcdev.ops = &axs10x_reset_ops;
    rst.rcdev.of_node = pdev.dev.of_node;
    rst.rcdev.nr_resets = AXS10X_MAX_RESETS;
    return devm_reset_controller_register(&pdev.dev, &rst.rcdev);
    }
    static const struct of_device_id axs10x_reset_dt_match[] = {
    { .compatible = "snps,axs10x-reset" },
    { },
    };
    static struct platform_driver axs10x_reset_driver = {
    .probe	= axs10x_reset_probe,
    .driver	= {
    .name = "axs10x-reset",
    .of_match_table = axs10x_reset_dt_match,
    },
    };
    builtin_platform_driver(axs10x_reset_driver);
    MODULE_AUTHOR("Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>");
    MODULE_DESCRIPTION("Synopsys AXS10x reset driver");
    MODULE_LICENSE("GPL v2");
