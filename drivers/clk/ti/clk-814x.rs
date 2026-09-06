//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/clk-814x.c
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

    static const struct omap_clkctrl_reg_data dm814_default_clkctrl_regs[] __initconst = {
    { DM814_USB_OTG_HS_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "pll260dcoclkldo" },
    { 0 },
    };
    static const struct omap_clkctrl_reg_data dm814_alwon_clkctrl_regs[] __initconst = {
    { DM814_UART1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM814_UART2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM814_UART3_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM814_GPIO1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM814_GPIO2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM814_I2C1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM814_I2C2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM814_WD_TIMER_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP | CLKF_NO_IDLEST, "sysclk18_ck" },
    { DM814_MCSPI1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM814_GPMC_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM814_MPU_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "mpu_ck" },
    { DM814_RTC_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP | CLKF_NO_IDLEST, "sysclk18_ck" },
    { DM814_TPCC_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM814_TPTC0_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM814_TPTC1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM814_TPTC2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM814_TPTC3_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM814_MMC1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk8_ck" },
    { DM814_MMC2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk8_ck" },
    { DM814_MMC3_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk8_ck" },
    { 0 },
    };
    static const struct
    omap_clkctrl_reg_data dm814_alwon_ethernet_clkctrl_regs[] __initconst = {
    { 0, core::ptr::null_mut(), CLKF_SW_SUP, "cpsw_125mhz_gclk" },
    };
    const struct omap_clkctrl_data dm814_clkctrl_data[] __initconst = {
    { 0x48180500, dm814_default_clkctrl_regs },
    { 0x48181400, dm814_alwon_clkctrl_regs },
    { 0x481815d4, dm814_alwon_ethernet_clkctrl_regs },
    { 0 },
    };
    static struct ti_dt_clk dm814_clks[] = {
    DT_CLK(core::ptr::null_mut(), "timer_sys_ck", "devosc_ck"),
    { .node_name = core::ptr::null_mut() },
    };
    static bool timer_clocks_initialized;
#[no_mangle]
unsafe extern "C" fn dm814x_adpll_early_init() -> int __init {
    static int __init dm814x_adpll_early_init(void)
    {
    struct device_node *np;
    if (!timer_clocks_initialized)
    return -ENODEV;
    np = of_find_node_by_name(core::ptr::null_mut(), "pllss");
    if (!np) {
    pr_err("Could not find node for plls\n");
    return -ENODEV;
    }
    of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    of_node_put(np);
    return 0;
    }
    core_initcall(dm814x_adpll_early_init);
    static const char * const init_clocks[] = {
    "pll040clkout",		/* MPU 481c5040.adpll.clkout */
    "pll290clkout",		/* DDR 481c5290.adpll.clkout */
    };
#[no_mangle]
unsafe extern "C" fn dm814x_adpll_enable_init_clocks() -> int __init {
    static int __init dm814x_adpll_enable_init_clocks(void)
    {
    int i, err;
    if (!timer_clocks_initialized)
    return -ENODEV;
    for (i = 0; i < ARRAY_SIZE(init_clocks); i++) {
    struct clk *clock;
    clock = clk_get(core::ptr::null_mut(), init_clocks[i]);
    if (WARN(IS_ERR(clock), "could not find init clock %s\n",
    init_clocks[i]))
    continue;
    err = clk_prepare_enable(clock);
    if (WARN(err, "could not enable init clock %s\n",
    init_clocks[i]))
    continue;
    }
    return 0;
    }
    postcore_initcall(dm814x_adpll_enable_init_clocks);
#[no_mangle]
pub unsafe extern "C" fn dm814x_dt_clk_init() -> int __init {
    int __init dm814x_dt_clk_init(void)
    {
    ti_dt_clocks_register(dm814_clks);
    omap2_clk_disable_autoidle_all();
    ti_clk_add_aliases();
    omap2_clk_enable_init_clocks(core::ptr::null_mut(), 0);
    timer_clocks_initialized = true;
    return 0;
    }
