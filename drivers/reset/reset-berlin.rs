//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-berlin.c
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
// Copyright (C) 2014 Marvell Technology Group Ltd.
//
// Marvell Berlin reset driver
//
// Antoine Tenart <antoine.tenart@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const BERLIN_MAX_RESETS: c_int = 32;

    container_of((p), struct berlin_reset_priv, rcdev)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct berlin_reset_priv {
    pub regmap: *mut regmap,
    pub rcdev: reset_controller_dev,
}

    static int berlin_reset_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct berlin_reset_priv *priv = to_berlin_reset_priv(rcdev);
    let mut offset: c_int = id >> 8;
    let mut mask: c_int = BIT(id & 0x1f);
    regmap_write(priv.regmap, offset, mask);
// let the reset be effective
    udelay(10);
    return 0;
    }
    static const struct reset_control_ops berlin_reset_ops = {
    .reset	= berlin_reset_reset,
    };
    static int berlin_reset_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *reset_spec)
    {
    unsigned int offset, bit;
    offset = reset_spec.args[0];
    bit = reset_spec.args[1];
    if (bit >= BERLIN_MAX_RESETS)
    return -EINVAL;
    return (offset << 8) | bit;
    }
#[no_mangle]
unsafe extern "C" fn berlin2_reset_probe(pdev: *mut platform_device) -> c_int {
    static int berlin2_reset_probe(struct platform_device *pdev)
    {
    struct device_node *parent_np;
    struct berlin_reset_priv *priv;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    parent_np = of_get_parent(pdev.dev.of_node);
    priv.regmap = syscon_node_to_regmap(parent_np);
    of_node_put(parent_np);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    priv.rcdev.owner = THIS_MODULE;
    priv.rcdev.ops = &berlin_reset_ops;
    priv.rcdev.of_node = pdev.dev.of_node;
    priv.rcdev.of_reset_n_cells = 2;
    priv.rcdev.of_xlate = berlin_reset_xlate;
    return reset_controller_register(&priv.rcdev);
    }
    static const struct of_device_id berlin_reset_dt_match[] = {
    { .compatible = "marvell,berlin2-reset" },
    { },
    };
    MODULE_DEVICE_TABLE(of, berlin_reset_dt_match);
    static struct platform_driver berlin_reset_driver = {
    .probe	= berlin2_reset_probe,
    .driver	= {
    .name = "berlin2-reset",
    .of_match_table = berlin_reset_dt_match,
    },
    };
    module_platform_driver(berlin_reset_driver);
    MODULE_AUTHOR("Antoine Tenart <antoine.tenart@free-electrons.com>");
    MODULE_AUTHOR("Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>");
    MODULE_DESCRIPTION("Synaptics Berlin reset controller");
    MODULE_LICENSE("GPL");
