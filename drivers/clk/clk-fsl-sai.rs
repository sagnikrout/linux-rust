//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-fsl-sai.c
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
// Freescale SAI BCLK as a generic clock driver
//
// Copyright 2020 Michael Walle <michael@walle.cc>
//

pub const I2S_CSR: c_uint = 0x00;
pub const I2S_CR2: c_uint = 0x08;
pub const I2S_MCR: c_uint = 0x100;
pub const CSR_BCE_BIT: c_int = 28;
pub const CSR_TE_BIT: c_int = 31;

pub const CR2_DIV_SHIFT: c_int = 0;
pub const CR2_DIV_WIDTH: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_sai_data {
    pub /: *mut *mut unsigned int offset; / Register offset,
    pub /: *mut *mut bool have_mclk; / Have MCLK control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_sai_clk {
    pub data: *const fsl_sai_data,
    pub bclk_div: clk_divider,
    pub mclk_div: clk_divider,
    pub bclk_gate: clk_gate,
    pub mclk_gate: clk_gate,
    pub bclk_hw: *mut clk_hw,
    pub mclk_hw: *mut clk_hw,
    pub lock: spinlock_t,
}

    static struct clk_hw *
    fsl_sai_of_clk_get(struct of_phandle_args *clkspec, void *data)
    {
    struct fsl_sai_clk *sai_clk = data;
    if (clkspec.args_count == 0)
    return sai_clk.bclk_hw;
    if (clkspec.args_count == 1) {
    if (clkspec.args[0] == 0)
    return sai_clk.bclk_hw;
    if (sai_clk.data.have_mclk && clkspec.args[0] == 1)
    return sai_clk.mclk_hw;
    }
    return ERR_PTR(-EINVAL);
    }
    static int fsl_sai_clk_register(struct device *dev, void __iomem *base,
    spinlock_t *lock, struct clk_divider *div,
    struct clk_gate *gate, struct clk_hw **hw,
    const int gate_bit, const int dir_bit,
    const int div_reg, char *name)
    {
    const struct fsl_sai_data *data = device_get_match_data(dev);
    let mut pdata: clk_parent_data = { .index = 0 };
    struct clk_hw *chw;
    char *cname;
    gate.reg = base + data.offset + I2S_CSR;
    gate.bit_idx = gate_bit;
    gate.lock = lock;
    div.reg = base + div_reg;
    div.shift = CR2_DIV_SHIFT;
    div.width = CR2_DIV_WIDTH;
    div.lock = lock;
    cname = devm_kasprintf(dev, GFP_KERNEL, "%s.%s",
    of_node_full_name(dev.of_node), name);
    if (!cname)
    return -ENOMEM;
// Set clock direction
    writel(dir_bit, base + div_reg);
    chw = devm_clk_hw_register_composite_pdata(dev, cname,
    &pdata, 1, core::ptr::null_mut(), core::ptr::null_mut(),
    &div.hw,
    &clk_divider_ops,
    &gate.hw,
    &clk_gate_ops,
    CLK_SET_RATE_GATE);
    if (IS_ERR(chw))
    return PTR_ERR(chw);
// hw = chw;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_sai_clk_probe(pdev: *mut platform_device) -> c_int {
    static int fsl_sai_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct fsl_sai_data *data = device_get_match_data(dev);
    struct fsl_sai_clk *sai_clk;
    struct clk *clk_bus;
    void __iomem *base;
    int ret;
    sai_clk = devm_kzalloc(dev, sizeof(*sai_clk), GFP_KERNEL);
    if (!sai_clk)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    clk_bus = devm_clk_get_optional_enabled(dev, "bus");
    if (IS_ERR(clk_bus))
    return PTR_ERR(clk_bus);
    sai_clk.data = data;
    spin_lock_init(&sai_clk.lock);
    ret = fsl_sai_clk_register(dev, base, &sai_clk.lock,
    &sai_clk.bclk_div, &sai_clk.bclk_gate,
    &sai_clk.bclk_hw, CSR_BCE_BIT, CR2_BCD,
    data.offset + I2S_CR2, "BCLK");
    if (ret)
    return ret;
    if (data.have_mclk) {
    ret = fsl_sai_clk_register(dev, base, &sai_clk.lock,
    &sai_clk.mclk_div,
    &sai_clk.mclk_gate,
    &sai_clk.mclk_hw,
    CSR_TE_BIT, MCR_MOE, I2S_MCR,
    "MCLK");
    if (ret)
    return ret;
    }
    return devm_of_clk_add_hw_provider(dev, fsl_sai_of_clk_get, sai_clk);
    }
    static const struct fsl_sai_data fsl_sai_vf610_data = {
    .offset	= 0,
    .have_mclk = false,
    };
    static const struct fsl_sai_data fsl_sai_imx8mq_data = {
    .offset	= 8,
    .have_mclk = true,
    };
    static const struct of_device_id of_fsl_sai_clk_ids[] = {
    { .compatible = "fsl,vf610-sai-clock", .data = &fsl_sai_vf610_data },
    { .compatible = "fsl,imx8mq-sai-clock", .data = &fsl_sai_imx8mq_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_fsl_sai_clk_ids);
    static struct platform_driver fsl_sai_clk_driver = {
    .probe = fsl_sai_clk_probe,
    .driver		= {
    .name	= "fsl-sai-clk",
    .of_match_table = of_fsl_sai_clk_ids,
    },
    };
    module_platform_driver(fsl_sai_clk_driver);
    MODULE_DESCRIPTION("Freescale SAI bitclock-as-a-clock driver");
    MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
    MODULE_ALIAS("platform:fsl-sai-clk");
