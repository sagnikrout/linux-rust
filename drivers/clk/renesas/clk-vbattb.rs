//! Automatically rewritten from C to Rust
//! Source: drivers/clk/renesas/clk-vbattb.c
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
// VBATTB clock driver
//
// Copyright (C) 2024 Renesas Electronics Corp.
//

pub const VBATTB_BKSCCR: c_uint = 0x1c;
pub const VBATTB_BKSCCR_SOSEL: c_int = 6;
pub const VBATTB_SOSCCR2: c_uint = 0x24;
pub const VBATTB_SOSCCR2_SOSTP2: c_int = 0;
pub const VBATTB_XOSCCR: c_uint = 0x30;
pub const VBATTB_XOSCCR_OUTEN: c_int = 16;

pub const VBATTB_XOSCCR_XSEL_4_PF: c_uint = 0x0;
pub const VBATTB_XOSCCR_XSEL_7_PF: c_uint = 0x1;
pub const VBATTB_XOSCCR_XSEL_9_PF: c_uint = 0x2;
pub const VBATTB_XOSCCR_XSEL_12_5_PF: c_uint = 0x3;
//
// struct vbattb_clk - VBATTB clock data structure
// @base: base address
// @lock: lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbattb_clk {
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn vbattb_clk_validate_load_capacitance(reg_lc: *mut u32, of_lc: u32) -> c_int {
    static int vbattb_clk_validate_load_capacitance(u32 *reg_lc, u32 of_lc)
    {
    switch (of_lc) {
    case 4000:
// reg_lc = VBATTB_XOSCCR_XSEL_4_PF;
    break;
    case 7000:
// reg_lc = VBATTB_XOSCCR_XSEL_7_PF;
    break;
    case 9000:
// reg_lc = VBATTB_XOSCCR_XSEL_9_PF;
    break;
    case 12500:
// reg_lc = VBATTB_XOSCCR_XSEL_12_5_PF;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vbattb_clk_action(data: *mut c_void) {
    static void vbattb_clk_action(void *data)
    {
    struct device *dev = data;
    struct reset_control *rstc = dev_get_drvdata(dev);
    int ret;
    ret = reset_control_assert(rstc);
    if (ret)
    dev_err(dev, "Failed to de-assert reset!\n");
    ret = pm_runtime_put_sync(dev);
    if (ret < 0)
    dev_err(dev, "Failed to runtime suspend!\n");
    of_clk_del_provider(dev.of_node);
    }
#[no_mangle]
unsafe extern "C" fn vbattb_clk_probe(pdev: *mut platform_device) -> c_int {
    static int vbattb_clk_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    let mut parent_data: clk_parent_data = {};
    struct clk_hw_onecell_data *clk_data;
    const struct clk_hw *parent_hws[2];
    struct device *dev = &pdev.dev;
    struct reset_control *rstc;
    struct vbattb_clk *vbclk;
    u32 of_lc, reg_lc;
    struct clk_hw *hw;
// 4 clocks are exported: VBATTB_XC, VBATTB_XBYP, VBATTB_MUX, VBATTB_VBATTCLK.
    let mut num_clks: u8 = 4;
    int ret;
// Default to 4pF as this is not needed if external clock device is connected.
    of_lc = 4000;
    of_property_read_u32(np, "quartz-load-femtofarads", &of_lc);
    ret = vbattb_clk_validate_load_capacitance(&reg_lc, of_lc);
    if (ret)
    return ret;
    vbclk = devm_kzalloc(dev, sizeof(*vbclk), GFP_KERNEL);
    if (!vbclk)
    return -ENOMEM;
    clk_data = devm_kzalloc(dev, struct_size(clk_data, hws, num_clks), GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    clk_data.num = num_clks;
    vbclk.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(vbclk.base))
    return PTR_ERR(vbclk.base);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    rstc = devm_reset_control_get_shared(dev, core::ptr::null_mut());
    if (IS_ERR(rstc))
    return PTR_ERR(rstc);
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
    ret = reset_control_deassert(rstc);
    if (ret) {
    pm_runtime_put_sync(dev);
    return ret;
    }
    dev_set_drvdata(dev, rstc);
    ret = devm_add_action_or_reset(dev, vbattb_clk_action, dev);
    if (ret)
    return ret;
    spin_lock_init(&vbclk.lock);
    parent_data.fw_name = "rtx";
    hw = devm_clk_hw_register_gate_parent_data(dev, "xc", &parent_data, 0,
    vbclk.base + VBATTB_SOSCCR2,
    VBATTB_SOSCCR2_SOSTP2,
    CLK_GATE_SET_TO_DISABLE, &vbclk.lock);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    clk_data.hws[VBATTB_XC] = hw;
    hw = devm_clk_hw_register_fixed_factor_fwname(dev, np, "xbyp", "rtx", 0, 1, 1);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    clk_data.hws[VBATTB_XBYP] = hw;
    parent_hws[0] = clk_data.hws[VBATTB_XC];
    parent_hws[1] = clk_data.hws[VBATTB_XBYP];
    hw = devm_clk_hw_register_mux_parent_hws(dev, "mux", parent_hws, 2, 0,
    vbclk.base + VBATTB_BKSCCR,
    VBATTB_BKSCCR_SOSEL,
    1, 0, &vbclk.lock);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    clk_data.hws[VBATTB_MUX] = hw;
// Set load capacitance before registering the VBATTCLK clock.
    scoped_guard(spinlock, &vbclk.lock) {
    let mut val: u32 = readl_relaxed(vbclk.base + VBATTB_XOSCCR);
    val &= ~VBATTB_XOSCCR_XSEL;
    val |= reg_lc;
    writel_relaxed(val, vbclk.base + VBATTB_XOSCCR);
    }
// This feeds the RTC counter clock and it needs to stay on.
    hw = devm_clk_hw_register_gate_parent_hw(dev, "vbattclk", hw, CLK_IS_CRITICAL,
    vbclk.base + VBATTB_XOSCCR,
    VBATTB_XOSCCR_OUTEN, 0,
    &vbclk.lock);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    clk_data.hws[VBATTB_VBATTCLK] = hw;
    return of_clk_add_hw_provider(np, of_clk_hw_onecell_get, clk_data);
    }
    static const struct of_device_id vbattb_clk_match[] = {
    { .compatible = "renesas,r9a08g045-vbattb" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, vbattb_clk_match);
    static struct platform_driver vbattb_clk_driver = {
    .driver		= {
    .name	= "renesas-vbattb-clk",
    .of_match_table = vbattb_clk_match,
    },
    .probe = vbattb_clk_probe,
    };
    module_platform_driver(vbattb_clk_driver);
    MODULE_DESCRIPTION("Renesas VBATTB Clock Driver");
    MODULE_AUTHOR("Claudiu Beznea <claudiu.beznea.uj@bp.renesas.com>");
    MODULE_LICENSE("GPL");
