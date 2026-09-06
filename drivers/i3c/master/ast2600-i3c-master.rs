//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/ast2600-i3c-master.c
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
// Copyright (c) 2023 Code Construct
//
// Author: Jeremy Kerr <jk@codeconstruct.com.au>
//

// AST2600-specific global register set

pub const AST2600_DEFAULT_SDA_PULLUP_OHMS: c_int = 2000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast2600_i3c {
    pub dw: dw_i3c_master,
    pub global_regs: *mut regmap,
    pub global_idx: c_uint,
    pub sda_pullup: c_uint,
}

    static struct ast2600_i3c *to_ast2600_i3c(struct dw_i3c_master *dw)
    {
    return container_of(dw, struct ast2600_i3c, dw);
    }
#[no_mangle]
unsafe extern "C" fn ast2600_i3c_pullup_to_reg(ohms: c_uint, regp: *mut u32) -> c_int {
    static int ast2600_i3c_pullup_to_reg(unsigned int ohms, u32 *regp)
    {
    u32 reg;
    switch (ohms) {
    case 2000:
    reg = AST2600_I3CG_REG0_SDA_PULLUP_EN_2K;
    break;
    case 750:
    reg = AST2600_I3CG_REG0_SDA_PULLUP_EN_750;
    break;
    case 545:
    reg = AST2600_I3CG_REG0_SDA_PULLUP_EN_545;
    break;
    default:
    return -EINVAL;
    }
    if (regp)
// regp = reg;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ast2600_i3c_init(dw: *mut dw_i3c_master) -> c_int {
    static int ast2600_i3c_init(struct dw_i3c_master *dw)
    {
    struct ast2600_i3c *i3c = to_ast2600_i3c(dw);
    let mut reg: u32 = 0;
    int rc;
// reg0: set SDA pullup values
    rc = ast2600_i3c_pullup_to_reg(i3c.sda_pullup, &reg);
    if (rc)
    return rc;
    rc = regmap_write(i3c.global_regs,
    AST2600_I3CG_REG0(i3c.global_idx), reg);
    if (rc)
    return rc;
// reg1: set up the instance id, but leave everything else disabled,
// as it's all for client mode
//
    reg = AST2600_I3CG_REG1_INST_ID(i3c.global_idx);
    rc = regmap_write(i3c.global_regs,
    AST2600_I3CG_REG1(i3c.global_idx), reg);
    return rc;
    }
    static void ast2600_i3c_set_dat_ibi(struct dw_i3c_master *i3c,
    struct i3c_dev_desc *dev,
    bool enable, u32 *dat)
    {
//
// The ast2600 i3c controller will lock up on receiving 4n+1-byte IBIs
// if the PEC is disabled. We have no way to restrict the length of
// IBIs sent to the controller, so we need to unconditionally enable
// PEC checking, which means we drop a byte of payload data
//
    if (enable && dev.info.bcr & I3C_BCR_IBI_PAYLOAD) {
    dev_warn_once(&i3c.base.dev,
    "Enabling PEC workaround. IBI payloads will be truncated\n");
// dat |= DEV_ADDR_TABLE_IBI_PEC;
    }
    }
    static const struct dw_i3c_platform_ops ast2600_i3c_ops = {
    .init = ast2600_i3c_init,
    .set_dat_ibi = ast2600_i3c_set_dat_ibi,
    };
#[no_mangle]
unsafe extern "C" fn ast2600_i3c_probe(pdev: *mut platform_device) -> c_int {
    static int ast2600_i3c_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct of_phandle_args gspec;
    struct ast2600_i3c *i3c;
    int rc;
    i3c = devm_kzalloc(&pdev.dev, sizeof(*i3c), GFP_KERNEL);
    if (!i3c)
    return -ENOMEM;
    rc = of_parse_phandle_with_fixed_args(np, "aspeed,global-regs", 1, 0,
    &gspec);
    if (rc)
    return -ENODEV;
    i3c.global_regs = syscon_node_to_regmap(gspec.np);
    of_node_put(gspec.np);
    if (IS_ERR(i3c.global_regs))
    return PTR_ERR(i3c.global_regs);
    i3c.global_idx = gspec.args[0];
    rc = of_property_read_u32(np, "sda-pullup-ohms", &i3c.sda_pullup);
    if (rc)
    i3c.sda_pullup = AST2600_DEFAULT_SDA_PULLUP_OHMS;
    rc = ast2600_i3c_pullup_to_reg(i3c.sda_pullup, core::ptr::null_mut());
    if (rc)
    dev_err(&pdev.dev, "invalid sda-pullup value %d\n",
    i3c.sda_pullup);
    i3c.dw.platform_ops = &ast2600_i3c_ops;
    return dw_i3c_common_probe(&i3c.dw, pdev);
    }
#[no_mangle]
unsafe extern "C" fn ast2600_i3c_remove(pdev: *mut platform_device) {
    static void ast2600_i3c_remove(struct platform_device *pdev)
    {
    struct dw_i3c_master *dw_i3c = platform_get_drvdata(pdev);
    dw_i3c_common_remove(dw_i3c);
    }
    static const struct of_device_id ast2600_i3c_master_of_match[] = {
    { .compatible = "aspeed,ast2600-i3c", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ast2600_i3c_master_of_match);
    static struct platform_driver ast2600_i3c_driver = {
    .probe = ast2600_i3c_probe,
    .remove = ast2600_i3c_remove,
    .driver = {
    .name = "ast2600-i3c-master",
    .of_match_table = ast2600_i3c_master_of_match,
    },
    };
    module_platform_driver(ast2600_i3c_driver);
    MODULE_AUTHOR("Jeremy Kerr <jk@codeconstruct.com.au>");
    MODULE_DESCRIPTION("ASPEED AST2600 I3C driver");
    MODULE_LICENSE("GPL");
