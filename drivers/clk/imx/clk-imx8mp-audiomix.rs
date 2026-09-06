//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-imx8mp-audiomix.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for i.MX8M Plus Audio BLK_CTRL
//
// Copyright (C) 2022 Marek Vasut <marex@denx.de>
//

pub const CLKEN0: c_uint = 0x000;
pub const CLKEN1: c_uint = 0x004;
pub const EARC: c_uint = 0x200;
pub const SAI1_MCLK_SEL: c_uint = 0x300;
pub const SAI2_MCLK_SEL: c_uint = 0x304;
pub const SAI3_MCLK_SEL: c_uint = 0x308;
pub const SAI5_MCLK_SEL: c_uint = 0x30C;
pub const SAI6_MCLK_SEL: c_uint = 0x310;
pub const SAI7_MCLK_SEL: c_uint = 0x314;
pub const PDM_SEL: c_uint = 0x318;
pub const SAI_PLL_GNRL_CTL: c_uint = 0x400;
pub const SAI_PLL_FDIVL_CTL0: c_uint = 0x404;
pub const SAI_PLL_FDIVL_CTL1: c_uint = 0x408;
pub const SAI_PLL_SSCG_CTL: c_uint = 0x40C;
pub const SAI_PLL_MNIT_CTL: c_uint = 0x410;
pub const IPG_LP_CTRL: c_uint = 0x504;

    static const struct clk_parent_data					\
    clk_imx8mp_audiomix_sai##n##_mclk1_parents[] = {			\
    {								\
    .fw_name = "sai"__stringify(n),				\
    .name = "sai"__stringify(n)				\
    }, {								\
    .fw_name = "sai"__stringify(n)"_mclk",			\
    .name = "sai"__stringify(n)"_mclk"			\
    },								\
    }
    SAIn_MCLK1_PARENT(1);
    SAIn_MCLK1_PARENT(2);
    SAIn_MCLK1_PARENT(3);
    SAIn_MCLK1_PARENT(5);
    SAIn_MCLK1_PARENT(6);
    SAIn_MCLK1_PARENT(7);
    static const struct clk_parent_data clk_imx8mp_audiomix_sai_mclk2_parents[] = {
    { .fw_name = "sai1", .name = "sai1" },
    { .fw_name = "sai2", .name = "sai2" },
    { .fw_name = "sai3", .name = "sai3" },
    { .name = "dummy" },
    { .fw_name = "sai5", .name = "sai5" },
    { .fw_name = "sai6", .name = "sai6" },
    { .fw_name = "sai7", .name = "sai7" },
    { .fw_name = "sai1_mclk", .name = "sai1_mclk" },
    { .fw_name = "sai2_mclk", .name = "sai2_mclk" },
    { .fw_name = "sai3_mclk", .name = "sai3_mclk" },
    { .name = "dummy" },
    { .fw_name = "sai5_mclk", .name = "sai5_mclk" },
    { .fw_name = "sai6_mclk", .name = "sai6_mclk" },
    { .fw_name = "sai7_mclk", .name = "sai7_mclk" },
    { .fw_name = "spdif_extclk", .name = "spdif_extclk" },
    { .name = "dummy" },
    };
    static const struct clk_parent_data clk_imx8mp_audiomix_pdm_parents[] = {
    { .fw_name = "pdm", .name = "pdm" },
    { .name = "sai_pll_out_div2" },
    { .fw_name = "sai1_mclk", .name = "sai1_mclk" },
    { .name = "dummy" },
    };
    static const struct clk_parent_data clk_imx8mp_audiomix_pll_parents[] = {
    { .fw_name = "osc_24m", .name = "osc_24m" },
    { .name = "dummy" },
    { .name = "dummy" },
    { .name = "dummy" },
    };
    static const struct clk_parent_data clk_imx8mp_audiomix_pll_bypass_sels[] = {
    { .fw_name = "sai_pll", .name = "sai_pll" },
    { .fw_name = "sai_pll_ref_sel", .name = "sai_pll_ref_sel" },
    };

    {								\
    gname"_cg",						\
    IMX8MP_CLK_AUDIOMIX_##cname,				\
    { .fw_name = "ahb", .name = "ahb" }, core::ptr::null_mut(), 1,		\
    CLKEN0 + 4 * !!(IMX8MP_CLK_AUDIOMIX_##cname / 32),	\
    1, IMX8MP_CLK_AUDIOMIX_##cname % 32			\
    }

    {								\
    "sai"__stringify(n)"_mclk1_sel",			\
    IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK1_SEL, {},		\
    clk_imx8mp_audiomix_sai##n##_mclk1_parents,		\
    ARRAY_SIZE(clk_imx8mp_audiomix_sai##n##_mclk1_parents), \
    SAI##n##_MCLK_SEL, 1, 0					\
    }, {								\
    "sai"__stringify(n)"_mclk2_sel",			\
    IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK2_SEL, {},		\
    clk_imx8mp_audiomix_sai_mclk2_parents,			\
    ARRAY_SIZE(clk_imx8mp_audiomix_sai_mclk2_parents),	\
    SAI##n##_MCLK_SEL, 4, 1					\
    }, {								\
    "sai"__stringify(n)"_ipg_cg",				\
    IMX8MP_CLK_AUDIOMIX_SAI##n##_IPG,			\
    { .fw_name = "ahb", .name = "ahb" }, core::ptr::null_mut(), 1,		\
    CLKEN0, 1, IMX8MP_CLK_AUDIOMIX_SAI##n##_IPG		\
    }, {								\
    "sai"__stringify(n)"_mclk1_cg",				\
    IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK1,			\
    {							\
    .fw_name = "sai"__stringify(n)"_mclk1_sel",	\
    .name = "sai"__stringify(n)"_mclk1_sel"		\
    }, core::ptr::null_mut(), 1,						\
    CLKEN0, 1, IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK1		\
    }, {								\
    "sai"__stringify(n)"_mclk2_cg",				\
    IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK2,			\
    {							\
    .fw_name = "sai"__stringify(n)"_mclk2_sel",	\
    .name = "sai"__stringify(n)"_mclk2_sel"		\
    }, core::ptr::null_mut(), 1,						\
    CLKEN0, 1, IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK2		\
    }, {								\
    "sai"__stringify(n)"_mclk3_cg",				\
    IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK3,			\
    {							\
    .fw_name = "sai_pll_out_div2",			\
    .name = "sai_pll_out_div2"			\
    }, core::ptr::null_mut(), 1,						\
    CLKEN0, 1, IMX8MP_CLK_AUDIOMIX_SAI##n##_MCLK3		\
    }

    {								\
    "pdm_sel", IMX8MP_CLK_AUDIOMIX_PDM_SEL, {},		\
    clk_imx8mp_audiomix_pdm_parents,			\
    ARRAY_SIZE(clk_imx8mp_audiomix_pdm_parents),		\
    PDM_SEL, 2, 0						\
    }

    {								\
    gname"_cg",						\
    IMX8MP_CLK_AUDIOMIX_##cname,				\
    { .fw_name = pname, .name = pname }, core::ptr::null_mut(), 1,		\
    CLKEN0 + 4 * !!(IMX8MP_CLK_AUDIOMIX_##cname / 32),	\
    1, IMX8MP_CLK_AUDIOMIX_##cname % 32			\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_imx8mp_audiomix_sel {
    pub name: *const c_char,
    pub clkid: c_int,
    pub /: *const *const clk_parent_data parent; / For gate,
    pub /: *const *const *const clk_parent_data parents; / For mux,
    pub num_parents: c_int,
    pub reg: u16,
    pub width: u8,
    pub shift: u8,
}

    static struct clk_imx8mp_audiomix_sel sels[] = {
    CLK_GATE("asrc", ASRC_IPG),
    CLK_GATE("pdm", PDM_IPG),
    CLK_GATE("earc", EARC_IPG),
    CLK_GATE_PARENT("ocrama", OCRAMA_IPG, "axi"),
    CLK_GATE("aud2htx", AUD2HTX_IPG),
    CLK_GATE_PARENT("earc_phy", EARC_PHY, "sai_pll_out_div2"),
    CLK_GATE("sdma2", SDMA2_ROOT),
    CLK_GATE("sdma3", SDMA3_ROOT),
    CLK_GATE("spba2", SPBA2_ROOT),
    CLK_GATE_PARENT("dsp", DSP_ROOT, "axi"),
    CLK_GATE_PARENT("dspdbg", DSPDBG_ROOT, "axi"),
    CLK_GATE("edma", EDMA_ROOT),
    CLK_GATE_PARENT("audpll", AUDPLL_ROOT, "osc_24m"),
    CLK_GATE("mu2", MU2_ROOT),
    CLK_GATE("mu3", MU3_ROOT),
    CLK_PDM,
    CLK_SAIn(1),
    CLK_SAIn(2),
    CLK_SAIn(3),
    CLK_SAIn(5),
    CLK_SAIn(6),
    CLK_SAIn(7)
    };
    static const u16 audiomix_regs[] = {
    CLKEN0,
    CLKEN1,
    EARC,
    SAI1_MCLK_SEL,
    SAI2_MCLK_SEL,
    SAI3_MCLK_SEL,
    SAI5_MCLK_SEL,
    SAI6_MCLK_SEL,
    SAI7_MCLK_SEL,
    PDM_SEL,
    SAI_PLL_GNRL_CTL,
    SAI_PLL_FDIVL_CTL0,
    SAI_PLL_FDIVL_CTL1,
    SAI_PLL_SSCG_CTL,
    SAI_PLL_MNIT_CTL,
    IPG_LP_CTRL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_imx8mp_audiomix_priv {
    pub base: *mut void __iomem,
    pub regs_save: [u32; ARRAY_SIZE(audiomix_regs)],
// Must be last
    pub clk_data: clk_hw_onecell_data,
}

    static int clk_imx8mp_audiomix_reset_controller_register(struct device *dev,
    struct clk_imx8mp_audiomix_priv *priv)
    {
    struct auxiliary_device *adev;
    if (!of_property_present(dev.of_node, "#reset-cells"))
    return 0;
    adev = devm_auxiliary_device_create(dev, "reset", core::ptr::null_mut());
    if (!adev)
    return -ENODEV;
    return 0;
    }

    static int clk_imx8mp_audiomix_reset_controller_register(struct device *dev,
    struct clk_imx8mp_audiomix_priv *priv)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn clk_imx8mp_audiomix_save_restore(dev: *mut device, save: bool) {
    static void clk_imx8mp_audiomix_save_restore(struct device *dev, bool save)
    {
    struct clk_imx8mp_audiomix_priv *priv = dev_get_drvdata(dev);
    void __iomem *base = priv.base;
    int i;
    if (save) {
    for (i = 0; i < ARRAY_SIZE(audiomix_regs); i++)
    priv.regs_save[i] = readl(base + audiomix_regs[i]);
    } else {
    for (i = 0; i < ARRAY_SIZE(audiomix_regs); i++)
    writel(priv.regs_save[i], base + audiomix_regs[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn clk_imx8mp_audiomix_probe(pdev: *mut platform_device) -> c_int {
    static int clk_imx8mp_audiomix_probe(struct platform_device *pdev)
    {
    struct clk_imx8mp_audiomix_priv *priv;
    struct clk_hw_onecell_data *clk_hw_data;
    struct device *dev = &pdev.dev;
    void __iomem *base;
    struct clk_hw *hw;
    int i, ret;
    priv = devm_kzalloc(dev,
    struct_size(priv, clk_data.hws, IMX8MP_CLK_AUDIOMIX_END),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    clk_hw_data = &priv.clk_data;
    clk_hw_data.num = IMX8MP_CLK_AUDIOMIX_END;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.base = base;
    dev_set_drvdata(dev, priv);
//
// pm_runtime_enable needs to be called before clk register.
// That is to make core->rpm_enabled to be true for clock
// usage.
//
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    for (i = 0; i < ARRAY_SIZE(sels); i++) {
    if (sels[i].num_parents == 1) {
    hw = devm_clk_hw_register_gate_parent_data(dev,
    sels[i].name, &sels[i].parent, CLK_SET_RATE_PARENT,
    base + sels[i].reg, sels[i].shift, 0, core::ptr::null_mut());
    } else {
    hw = devm_clk_hw_register_mux_parent_data_table(dev,
    sels[i].name, sels[i].parents,
    sels[i].num_parents, CLK_SET_RATE_PARENT,
    base + sels[i].reg,
    sels[i].shift, sels[i].width,
    0, core::ptr::null_mut(), core::ptr::null_mut());
    }
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto err_clk_register;
    }
    clk_hw_data.hws[sels[i].clkid] = hw;
    }
// SAI PLL
    hw = devm_clk_hw_register_mux_parent_data_table(dev,
    "sai_pll_ref_sel", clk_imx8mp_audiomix_pll_parents,
    ARRAY_SIZE(clk_imx8mp_audiomix_pll_parents),
    CLK_SET_RATE_NO_REPARENT, base + SAI_PLL_GNRL_CTL,
    0, 2, 0, core::ptr::null_mut(), core::ptr::null_mut());
    clk_hw_data.hws[IMX8MP_CLK_AUDIOMIX_SAI_PLL_REF_SEL] = hw;
    hw = imx_dev_clk_hw_pll14xx(dev, "sai_pll", "sai_pll_ref_sel",
    base + 0x400, &imx_1443x_pll);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto err_clk_register;
    }
    clk_hw_data.hws[IMX8MP_CLK_AUDIOMIX_SAI_PLL] = hw;
    hw = devm_clk_hw_register_mux_parent_data_table(dev,
    "sai_pll_bypass", clk_imx8mp_audiomix_pll_bypass_sels,
    ARRAY_SIZE(clk_imx8mp_audiomix_pll_bypass_sels),
    CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT,
    base + SAI_PLL_GNRL_CTL, 16, 1, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto err_clk_register;
    }
    clk_hw_data.hws[IMX8MP_CLK_AUDIOMIX_SAI_PLL_BYPASS] = hw;
    hw = devm_clk_hw_register_gate(dev, "sai_pll_out", "sai_pll_bypass",
    CLK_SET_RATE_PARENT,
    base + SAI_PLL_GNRL_CTL, 13,
    0, core::ptr::null_mut());
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto err_clk_register;
    }
    clk_hw_data.hws[IMX8MP_CLK_AUDIOMIX_SAI_PLL_OUT] = hw;
    hw = devm_clk_hw_register_fixed_factor(dev, "sai_pll_out_div2",
    "sai_pll_out",
    CLK_SET_RATE_PARENT, 1, 2);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto err_clk_register;
    }
    ret = devm_of_clk_add_hw_provider(&pdev.dev, of_clk_hw_onecell_get,
    clk_hw_data);
    if (ret)
    goto err_clk_register;
    ret = clk_imx8mp_audiomix_reset_controller_register(dev, priv);
    if (ret)
    goto err_clk_register;
    pm_runtime_put_sync(dev);
    return 0;
    err_clk_register:
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_imx8mp_audiomix_remove(pdev: *mut platform_device) {
    static void clk_imx8mp_audiomix_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn clk_imx8mp_audiomix_runtime_suspend(dev: *mut device) -> c_int {
    static int clk_imx8mp_audiomix_runtime_suspend(struct device *dev)
    {
    clk_imx8mp_audiomix_save_restore(dev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_imx8mp_audiomix_runtime_resume(dev: *mut device) -> c_int {
    static int clk_imx8mp_audiomix_runtime_resume(struct device *dev)
    {
    clk_imx8mp_audiomix_save_restore(dev, false);
    return 0;
    }
    static const struct dev_pm_ops clk_imx8mp_audiomix_pm_ops = {
    RUNTIME_PM_OPS(clk_imx8mp_audiomix_runtime_suspend,
    clk_imx8mp_audiomix_runtime_resume, core::ptr::null_mut())
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct of_device_id clk_imx8mp_audiomix_of_match[] = {
    { .compatible = "fsl,imx8mp-audio-blk-ctrl" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, clk_imx8mp_audiomix_of_match);
    static struct platform_driver clk_imx8mp_audiomix_driver = {
    .probe	= clk_imx8mp_audiomix_probe,
    .remove = clk_imx8mp_audiomix_remove,
    .driver = {
    .name = "imx8mp-audio-blk-ctrl",
    .of_match_table = clk_imx8mp_audiomix_of_match,
    .pm = pm_ptr(&clk_imx8mp_audiomix_pm_ops),
    },
    };
    module_platform_driver(clk_imx8mp_audiomix_driver);
    MODULE_AUTHOR("Marek Vasut <marex@denx.de>");
    MODULE_DESCRIPTION("Freescale i.MX8MP Audio Block Controller driver");
    MODULE_LICENSE("GPL");
