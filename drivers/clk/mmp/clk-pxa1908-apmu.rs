//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-pxa1908-apmu.c
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


// SPDX-License-Identifier: GPL-2.0-only

pub const APMU_CLK_GATE_CTRL: c_uint = 0x40;
pub const APMU_CCIC1: c_uint = 0x24;
pub const APMU_ISP: c_uint = 0x38;
pub const APMU_DSI1: c_uint = 0x44;
pub const APMU_DISP1: c_uint = 0x4c;
pub const APMU_CCIC0: c_uint = 0x50;
pub const APMU_SDH0: c_uint = 0x54;
pub const APMU_SDH1: c_uint = 0x58;
pub const APMU_USB: c_uint = 0x5c;
pub const APMU_NF: c_uint = 0x60;
pub const APMU_VPU: c_uint = 0xa4;
pub const APMU_GC: c_uint = 0xcc;
pub const APMU_SDH2: c_uint = 0xe0;
pub const APMU_GC2D: c_uint = 0xf4;
pub const APMU_TRACE: c_uint = 0x108;
pub const APMU_DVC_DFC_DEBUG: c_uint = 0x140;
pub const APMU_NR_CLKS: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa1908_clk_unit {
    pub unit: mmp_clk_unit,
    pub base: *mut void __iomem,
}

    static DEFINE_SPINLOCK(pll1_lock);
    static struct mmp_param_general_gate_clk pll1_gate_clks[] = {
    {PXA1908_CLK_PLL1_D2_GATE, "pll1_d2_gate", "pll1_d2", 0, APMU_CLK_GATE_CTRL, 29, 0, &pll1_lock},
    {PXA1908_CLK_PLL1_416_GATE, "pll1_416_gate", "pll1_416", 0, APMU_CLK_GATE_CTRL, 27, 0, &pll1_lock},
    {PXA1908_CLK_PLL1_624_GATE, "pll1_624_gate", "pll1_624", 0, APMU_CLK_GATE_CTRL, 26, 0, &pll1_lock},
    {PXA1908_CLK_PLL1_832_GATE, "pll1_832_gate", "pll1_832", 0, APMU_CLK_GATE_CTRL, 30, 0, &pll1_lock},
    {PXA1908_CLK_PLL1_1248_GATE, "pll1_1248_gate", "pll1_1248", 0, APMU_CLK_GATE_CTRL, 28, 0, &pll1_lock},
    };
    static DEFINE_SPINLOCK(sdh0_lock);
    static DEFINE_SPINLOCK(sdh1_lock);
    static DEFINE_SPINLOCK(sdh2_lock);
    static const char * const sdh_parent_names[] = {"pll1_416", "pll1_624"};
    static struct mmp_clk_mix_config sdh_mix_config = {
    .reg_info = DEFINE_MIX_REG_INFO(3, 8, 2, 6, 11),
    };
    static struct mmp_param_gate_clk apmu_gate_clks[] = {
    {PXA1908_CLK_USB, "usb_clk", core::ptr::null_mut(), 0, APMU_USB, 0x9, 0x9, 0x1, 0, core::ptr::null_mut()},
    {PXA1908_CLK_SDH0, "sdh0_clk", "sdh0_mix_clk", CLK_SET_RATE_PARENT | CLK_SET_RATE_UNGATE, APMU_SDH0, 0x12, 0x12, 0x0, 0, &sdh0_lock},
    {PXA1908_CLK_SDH1, "sdh1_clk", "sdh1_mix_clk", CLK_SET_RATE_PARENT | CLK_SET_RATE_UNGATE, APMU_SDH1, 0x12, 0x12, 0x0, 0, &sdh1_lock},
    {PXA1908_CLK_SDH2, "sdh2_clk", "sdh2_mix_clk", CLK_SET_RATE_PARENT | CLK_SET_RATE_UNGATE, APMU_SDH2, 0x12, 0x12, 0x0, 0, &sdh2_lock}
    };
#[no_mangle]
unsafe extern "C" fn pxa1908_axi_periph_clk_init(pxa_unit: *mut pxa1908_clk_unit) {
    static void pxa1908_axi_periph_clk_init(struct pxa1908_clk_unit *pxa_unit)
    {
    struct mmp_clk_unit *unit = &pxa_unit.unit;
    mmp_register_general_gate_clks(unit, pll1_gate_clks,
    pxa_unit.base, ARRAY_SIZE(pll1_gate_clks));
    sdh_mix_config.reg_info.reg_clk_ctrl = pxa_unit.base + APMU_SDH0;
    mmp_clk_register_mix(core::ptr::null_mut(), "sdh0_mix_clk", sdh_parent_names,
    ARRAY_SIZE(sdh_parent_names), CLK_SET_RATE_PARENT,
    &sdh_mix_config, &sdh0_lock);
    sdh_mix_config.reg_info.reg_clk_ctrl = pxa_unit.base + APMU_SDH1;
    mmp_clk_register_mix(core::ptr::null_mut(), "sdh1_mix_clk", sdh_parent_names,
    ARRAY_SIZE(sdh_parent_names), CLK_SET_RATE_PARENT,
    &sdh_mix_config, &sdh1_lock);
    sdh_mix_config.reg_info.reg_clk_ctrl = pxa_unit.base + APMU_SDH2;
    mmp_clk_register_mix(core::ptr::null_mut(), "sdh2_mix_clk", sdh_parent_names,
    ARRAY_SIZE(sdh_parent_names), CLK_SET_RATE_PARENT,
    &sdh_mix_config, &sdh2_lock);
    mmp_register_gate_clks(unit, apmu_gate_clks, pxa_unit.base,
    ARRAY_SIZE(apmu_gate_clks));
    }
#[no_mangle]
unsafe extern "C" fn pxa1908_apmu_probe(pdev: *mut platform_device) -> c_int {
    static int pxa1908_apmu_probe(struct platform_device *pdev)
    {
    struct pxa1908_clk_unit *pxa_unit;
    struct auxiliary_device *adev;
    pxa_unit = devm_kzalloc(&pdev.dev, sizeof(*pxa_unit), GFP_KERNEL);
    if (!pxa_unit)
    return -ENOMEM;
    pxa_unit.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pxa_unit.base))
    return PTR_ERR(pxa_unit.base);
    adev = devm_auxiliary_device_create(&pdev.dev, "power", core::ptr::null_mut());
    if (IS_ERR(adev))
    return dev_err_probe(&pdev.dev, PTR_ERR(adev),
    "Failed to register power controller\n");
    mmp_clk_init(pdev.dev.of_node, &pxa_unit.unit, APMU_NR_CLKS);
    pxa1908_axi_periph_clk_init(pxa_unit);
    return 0;
    }
    static const struct of_device_id pxa1908_apmu_match_table[] = {
    { .compatible = "marvell,pxa1908-apmu" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pxa1908_apmu_match_table);
    static struct platform_driver pxa1908_apmu_driver = {
    .probe = pxa1908_apmu_probe,
    .driver = {
    .name = "pxa1908-apmu",
    .of_match_table = pxa1908_apmu_match_table
    }
    };
    module_platform_driver(pxa1908_apmu_driver);
    MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>");
    MODULE_DESCRIPTION("Marvell PXA1908 APMU Clock Driver");
    MODULE_LICENSE("GPL");
