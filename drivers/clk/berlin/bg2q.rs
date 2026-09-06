//! Automatically rewritten from C to Rust
//! Source: drivers/clk/berlin/bg2q.c
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
// Copyright (c) 2014 Marvell Technology Group Ltd.
//
// Alexandre Belloni <alexandre.belloni@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
//

pub const REG_PINMUX0: c_uint = 0x0018;
pub const REG_PINMUX5: c_uint = 0x002c;
pub const REG_SYSPLLCTL0: c_uint = 0x0030;
pub const REG_SYSPLLCTL4: c_uint = 0x0040;
pub const REG_CLKENABLE: c_uint = 0x00e8;
pub const REG_CLKSELECT0: c_uint = 0x00ec;
pub const REG_CLKSELECT1: c_uint = 0x00f0;
pub const REG_CLKSELECT2: c_uint = 0x00f4;
pub const REG_CLKSWITCH0: c_uint = 0x00f8;
pub const REG_CLKSWITCH1: c_uint = 0x00fc;
pub const REG_SW_GENERIC0: c_uint = 0x0110;
pub const REG_SW_GENERIC3: c_uint = 0x011c;
pub const REG_SDIO0XIN_CLKCTL: c_uint = 0x0158;
pub const REG_SDIO1XIN_CLKCTL: c_uint = 0x015c;
pub const MAX_CLKS: c_int = 28;
    static struct clk_hw_onecell_data *clk_data;
    static DEFINE_SPINLOCK(lock);
    static void __iomem *gbase;
    static void __iomem *cpupll_base;
    enum {
    REFCLK,
    SYSPLL, CPUPLL,
    AVPLL_B1, AVPLL_B2, AVPLL_B3, AVPLL_B4,
    AVPLL_B5, AVPLL_B6, AVPLL_B7, AVPLL_B8,
    };
    static const char *clk_names[] = {
    [REFCLK]		= "refclk",
    [SYSPLL]		= "syspll",
    [CPUPLL]		= "cpupll",
    [AVPLL_B1]		= "avpll_b1",
    [AVPLL_B2]		= "avpll_b2",
    [AVPLL_B3]		= "avpll_b3",
    [AVPLL_B4]		= "avpll_b4",
    [AVPLL_B5]		= "avpll_b5",
    [AVPLL_B6]		= "avpll_b6",
    [AVPLL_B7]		= "avpll_b7",
    [AVPLL_B8]		= "avpll_b8",
    };
    static const struct berlin2_pll_map bg2q_pll_map __initconst = {
    .vcodiv		= {1, 0, 2, 0, 3, 4, 0, 6, 8},
    .mult		= 1,
    .fbdiv_shift	= 7,
    .rfdiv_shift	= 2,
    .divsel_shift	= 9,
    };
    static const u8 default_parent_ids[] = {
    SYSPLL, AVPLL_B4, AVPLL_B5, AVPLL_B6, AVPLL_B7, SYSPLL
    };
    static const struct berlin2_div_data bg2q_divs[] __initconst = {
    {
    .name = "sys",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 0),
    BERLIN2_PLL_SELECT(REG_CLKSELECT0, 0),
    BERLIN2_DIV_SELECT(REG_CLKSELECT0, 3),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 3),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 4),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 5),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = CLK_IGNORE_UNUSED,
    },
    {
    .name = "drmfigo",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 17),
    BERLIN2_PLL_SELECT(REG_CLKSELECT0, 6),
    BERLIN2_DIV_SELECT(REG_CLKSELECT0, 9),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 6),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 7),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 8),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "cfg",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 1),
    BERLIN2_PLL_SELECT(REG_CLKSELECT0, 12),
    BERLIN2_DIV_SELECT(REG_CLKSELECT0, 15),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 9),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 10),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 11),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "gfx2d",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 4),
    BERLIN2_PLL_SELECT(REG_CLKSELECT0, 18),
    BERLIN2_DIV_SELECT(REG_CLKSELECT0, 21),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 12),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 13),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 14),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "zsp",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 6),
    BERLIN2_PLL_SELECT(REG_CLKSELECT0, 24),
    BERLIN2_DIV_SELECT(REG_CLKSELECT0, 27),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 15),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 16),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 17),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "perif",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 7),
    BERLIN2_PLL_SELECT(REG_CLKSELECT1, 0),
    BERLIN2_DIV_SELECT(REG_CLKSELECT1, 3),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 18),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 19),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 20),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = CLK_IGNORE_UNUSED,
    },
    {
    .name = "pcube",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 2),
    BERLIN2_PLL_SELECT(REG_CLKSELECT1, 6),
    BERLIN2_DIV_SELECT(REG_CLKSELECT1, 9),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 21),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 22),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 23),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "vscope",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 3),
    BERLIN2_PLL_SELECT(REG_CLKSELECT1, 12),
    BERLIN2_DIV_SELECT(REG_CLKSELECT1, 15),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 24),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 25),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 26),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "nfc_ecc",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 19),
    BERLIN2_PLL_SELECT(REG_CLKSELECT1, 18),
    BERLIN2_DIV_SELECT(REG_CLKSELECT1, 21),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 27),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 28),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH0, 29),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "vpp",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 21),
    BERLIN2_PLL_SELECT(REG_CLKSELECT1, 24),
    BERLIN2_DIV_SELECT(REG_CLKSELECT1, 27),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH0, 30),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH0, 31),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH1, 0),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "app",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_DIV_GATE(REG_CLKENABLE, 20),
    BERLIN2_PLL_SELECT(REG_CLKSELECT2, 0),
    BERLIN2_DIV_SELECT(REG_CLKSELECT2, 3),
    BERLIN2_PLL_SWITCH(REG_CLKSWITCH1, 1),
    BERLIN2_DIV_SWITCH(REG_CLKSWITCH1, 2),
    BERLIN2_DIV_D3SWITCH(REG_CLKSWITCH1, 3),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "sdio0xin",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_SINGLE_DIV(REG_SDIO0XIN_CLKCTL),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    {
    .name = "sdio1xin",
    .parent_ids = default_parent_ids,
    .num_parents = ARRAY_SIZE(default_parent_ids),
    .map = {
    BERLIN2_SINGLE_DIV(REG_SDIO1XIN_CLKCTL),
    },
    .div_flags = BERLIN2_DIV_HAS_GATE | BERLIN2_DIV_HAS_MUX,
    .flags = 0,
    },
    };
    static const struct berlin2_gate_data bg2q_gates[] __initconst = {
    { "gfx2daxi",	"perif",	5 },
    { "geth0",	"perif",	8 },
    { "sata",	"perif",	9 },
    { "ahbapb",	"perif",	10, CLK_IGNORE_UNUSED },
    { "usb0",	"perif",	11 },
    { "usb1",	"perif",	12 },
    { "usb2",	"perif",	13 },
    { "usb3",	"perif",	14 },
    { "pbridge",	"perif",	15, CLK_IGNORE_UNUSED },
    { "sdio",	"perif",	16 },
    { "nfc",	"perif",	18 },
    { "pcie",	"perif",	22 },
    };
#[no_mangle]
unsafe extern "C" fn berlin2q_clock_setup(np: *mut device_node) -> void __init {
    static void __init berlin2q_clock_setup(struct device_node *np)
    {
    struct device_node *parent_np = of_get_parent(np);
    const char *parent_names[9];
    struct clk *clk;
    struct clk_hw **hws;
    int n, ret;
    clk_data = kzalloc_flex(*clk_data, hws, MAX_CLKS);
    if (!clk_data) {
    of_node_put(parent_np);
    return;
    }
    clk_data.num = MAX_CLKS;
    hws = clk_data.hws;
    gbase = of_iomap(parent_np, 0);
    if (!gbase) {
    of_node_put(parent_np);
    pr_err("%pOF: Unable to map global base\n", np);
    return;
    }
// BG2Q CPU PLL is not part of global registers
    cpupll_base = of_iomap(parent_np, 1);
    of_node_put(parent_np);
    if (!cpupll_base) {
    pr_err("%pOF: Unable to map cpupll base\n", np);
    iounmap(gbase);
    return;
    }
// overwrite default clock names with DT provided ones
    clk = of_clk_get_by_name(np, clk_names[REFCLK]);
    if (!IS_ERR(clk)) {
    clk_names[REFCLK] = __clk_get_name(clk);
    clk_put(clk);
    }
// simple register PLLs
    ret = berlin2_pll_register(&bg2q_pll_map, gbase + REG_SYSPLLCTL0,
    clk_names[SYSPLL], clk_names[REFCLK], 0);
    if (ret)
    goto bg2q_fail;
    ret = berlin2_pll_register(&bg2q_pll_map, cpupll_base,
    clk_names[CPUPLL], clk_names[REFCLK], 0);
    if (ret)
    goto bg2q_fail;
// TODO: add BG2Q AVPLL
//
// TODO: add reference clock bypass switches:
// memPLLSWBypass, cpuPLLSWBypass, and sysPLLSWBypass
//
// clock divider cells
    for (n = 0; n < ARRAY_SIZE(bg2q_divs); n++) {
    const struct berlin2_div_data *dd = &bg2q_divs[n];
    int k;
    for (k = 0; k < dd.num_parents; k++)
    parent_names[k] = clk_names[dd.parent_ids[k]];
    hws[CLKID_SYS + n] = berlin2_div_register(&dd.map, gbase,
    dd.name, dd.div_flags, parent_names,
    dd.num_parents, dd.flags, &lock);
    }
// clock gate cells
    for (n = 0; n < ARRAY_SIZE(bg2q_gates); n++) {
    const struct berlin2_gate_data *gd = &bg2q_gates[n];
    hws[CLKID_GFX2DAXI + n] = clk_hw_register_gate(core::ptr::null_mut(), gd.name,
    gd.parent_name, gd.flags, gbase + REG_CLKENABLE,
    gd.bit_idx, 0, &lock);
    }
// cpuclk divider is fixed to 1
    hws[CLKID_CPU] =
    clk_hw_register_fixed_factor(core::ptr::null_mut(), "cpu", clk_names[CPUPLL],
    0, 1, 1);
// twdclk is derived from cpu/3
    hws[CLKID_TWD] =
    clk_hw_register_fixed_factor(core::ptr::null_mut(), "twd", "cpu", 0, 1, 3);
// check for errors on leaf clocks
    for (n = 0; n < MAX_CLKS; n++) {
    if (!IS_ERR(hws[n]))
    continue;
    pr_err("%pOF: Unable to register leaf clock %d\n", np, n);
    goto bg2q_fail;
    }
// register clk-provider
    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, clk_data);
    return;
    bg2q_fail:
    iounmap(cpupll_base);
    iounmap(gbase);
    }
    CLK_OF_DECLARE(berlin2q_clk, "marvell,berlin2q-clk",
    berlin2q_clock_setup);
