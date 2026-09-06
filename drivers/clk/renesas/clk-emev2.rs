//! Automatically rewritten from C to Rust
//! Source: drivers/clk/renesas/clk-emev2.c
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
// EMMA Mobile EV2 common clock framework support
//
// Copyright (C) 2013 Takashi Yoshii <takashi.yoshii.ze@renesas.com>
// Copyright (C) 2012 Magnus Damm
//

// EMEV2 SMU registers
pub const USIAU0_RSTCTRL: c_uint = 0x094;
pub const USIBU1_RSTCTRL: c_uint = 0x0ac;
pub const USIBU2_RSTCTRL: c_uint = 0x0b0;
pub const USIBU3_RSTCTRL: c_uint = 0x0b4;
pub const IIC0_RSTCTRL: c_uint = 0x0dc;
pub const IIC1_RSTCTRL: c_uint = 0x0e0;
pub const STI_RSTCTRL: c_uint = 0x124;
pub const STI_CLKSEL: c_uint = 0x688;
    static DEFINE_SPINLOCK(lock);
// not pretty, but hey
    static void __iomem *smu_base;
#[no_mangle]
unsafe extern "C" fn emev2_smu_write(value: c_ulong, offs: c_int) -> void __init {
    static void __init emev2_smu_write(unsigned long value, int offs)
    {
    BUG_ON(!smu_base || (offs >= PAGE_SIZE));
    writel_relaxed(value, smu_base + offs);
    }
    static const struct of_device_id smu_id[] __initconst = {
    { .compatible = "renesas,emev2-smu", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn emev2_smu_init() -> void __init {
    static void __init emev2_smu_init(void)
    {
    struct device_node *np;
    np = of_find_matching_node(core::ptr::null_mut(), smu_id);
    BUG_ON(!np);
    smu_base = of_iomap(np, 0);
    BUG_ON(!smu_base);
    of_node_put(np);
// setup STI timer to run on 32.768 kHz and deassert reset
    emev2_smu_write(0, STI_CLKSEL);
    emev2_smu_write(1, STI_RSTCTRL);
// deassert reset for UART0->UART3
    emev2_smu_write(2, USIAU0_RSTCTRL);
    emev2_smu_write(2, USIBU1_RSTCTRL);
    emev2_smu_write(2, USIBU2_RSTCTRL);
    emev2_smu_write(2, USIBU3_RSTCTRL);
// deassert reset for IIC0->IIC1
    emev2_smu_write(1, IIC0_RSTCTRL);
    emev2_smu_write(1, IIC1_RSTCTRL);
    }
#[no_mangle]
unsafe extern "C" fn emev2_smu_clkdiv_init(np: *mut device_node) -> void __init {
    static void __init emev2_smu_clkdiv_init(struct device_node *np)
    {
    u32 reg[2];
    struct clk *clk;
    const char *parent_name = of_clk_get_parent_name(np, 0);
    if (WARN_ON(of_property_read_u32_array(np, "reg", reg, 2)))
    return;
    if (!smu_base)
    emev2_smu_init();
    clk = clk_register_divider(core::ptr::null_mut(), np.name, parent_name, 0,
    smu_base + reg[0], reg[1], 8, 0, &lock);
    of_clk_add_provider(np, of_clk_src_simple_get, clk);
    pr_debug("## %s %pOFn %p\n", __func__, np, clk);
    }
    CLK_OF_DECLARE(emev2_smu_clkdiv, "renesas,emev2-smu-clkdiv",
    emev2_smu_clkdiv_init);
#[no_mangle]
unsafe extern "C" fn emev2_smu_gclk_init(np: *mut device_node) -> void __init {
    static void __init emev2_smu_gclk_init(struct device_node *np)
    {
    u32 reg[2];
    struct clk *clk;
    const char *parent_name = of_clk_get_parent_name(np, 0);
    if (WARN_ON(of_property_read_u32_array(np, "reg", reg, 2)))
    return;
    if (!smu_base)
    emev2_smu_init();
    clk = clk_register_gate(core::ptr::null_mut(), np.name, parent_name, 0,
    smu_base + reg[0], reg[1], 0, &lock);
    of_clk_add_provider(np, of_clk_src_simple_get, clk);
    pr_debug("## %s %pOFn %p\n", __func__, np, clk);
    }
    CLK_OF_DECLARE(emev2_smu_gclk, "renesas,emev2-smu-gclk", emev2_smu_gclk_init);
