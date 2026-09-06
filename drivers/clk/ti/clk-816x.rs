//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/clk-816x.c
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

    static const struct omap_clkctrl_reg_data dm816_default_clkctrl_regs[] __initconst = {
    { DM816_USB_OTG_HS_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { 0 },
    };
    static const struct omap_clkctrl_reg_data dm816_alwon_clkctrl_regs[] __initconst = {
    { DM816_UART1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM816_UART2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM816_UART3_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM816_GPIO1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM816_GPIO2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM816_I2C1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM816_I2C2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM816_TIMER1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "timer1_fck" },
    { DM816_TIMER2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "timer2_fck" },
    { DM816_TIMER3_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "timer3_fck" },
    { DM816_TIMER4_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "timer4_fck" },
    { DM816_TIMER5_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "timer5_fck" },
    { DM816_TIMER6_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "timer6_fck" },
    { DM816_TIMER7_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "timer7_fck" },
    { DM816_WD_TIMER_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP | CLKF_NO_IDLEST, "sysclk18_ck" },
    { DM816_MCSPI1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM816_MAILBOX_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM816_SPINBOX_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM816_MMC1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk10_ck" },
    { DM816_GPMC_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk6_ck" },
    { DM816_DAVINCI_MDIO_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP | CLKF_NO_IDLEST, "sysclk24_ck" },
    { DM816_EMAC1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP | CLKF_NO_IDLEST, "sysclk24_ck" },
    { DM816_MPU_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk2_ck" },
    { DM816_RTC_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP | CLKF_NO_IDLEST, "sysclk18_ck" },
    { DM816_TPCC_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM816_TPTC0_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM816_TPTC1_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM816_TPTC2_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { DM816_TPTC3_CLKCTRL, core::ptr::null_mut(), CLKF_SW_SUP, "sysclk4_ck" },
    { 0 },
    };
    const struct omap_clkctrl_data dm816_clkctrl_data[] __initconst = {
    { 0x48180500, dm816_default_clkctrl_regs },
    { 0x48181400, dm816_alwon_clkctrl_regs },
    { 0 },
    };
    static struct ti_dt_clk dm816x_clks[] = {
    DT_CLK(core::ptr::null_mut(), "sys_clkin", "sys_clkin_ck"),
    DT_CLK(core::ptr::null_mut(), "timer_sys_ck", "sys_clkin_ck"),
    DT_CLK(core::ptr::null_mut(), "timer_32k_ck", "sysclk18_ck"),
    DT_CLK(core::ptr::null_mut(), "timer_ext_ck", "tclkin_ck"),
    { .node_name = core::ptr::null_mut() },
    };
    static const char *enable_init_clks[] = {
    "ddr_pll_clk1",
    "ddr_pll_clk2",
    "ddr_pll_clk3",
    "sysclk6_ck",
    };
#[no_mangle]
pub unsafe extern "C" fn dm816x_dt_clk_init() -> int __init {
    int __init dm816x_dt_clk_init(void)
    {
    ti_dt_clocks_register(dm816x_clks);
    omap2_clk_disable_autoidle_all();
    ti_clk_add_aliases();
    omap2_clk_enable_init_clocks(enable_init_clks,
    ARRAY_SIZE(enable_init_clks));
    return 0;
    }
