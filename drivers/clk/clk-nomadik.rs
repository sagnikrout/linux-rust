//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-nomadik.c
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
//
// Nomadik clock implementation
// Copyright (C) 2013 ST-Ericsson AB
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// The Nomadik clock tree is described in the STN8815A12 DB V4.2
// reference manual for the chip, page 94 ff.
// Clock IDs are in the STn8815 Reference Manual table 3, page 27.
//
pub const SRC_CR: c_uint = 0x00U;

pub const SRC_XTALCR: c_uint = 0x0CU;

pub const SRC_PLLCR: c_uint = 0x10U;

pub const SRC_PLLFR: c_uint = 0x14U;
pub const SRC_PCKEN0: c_uint = 0x24U;
pub const SRC_PCKDIS0: c_uint = 0x28U;
pub const SRC_PCKENSR0: c_uint = 0x2CU;
pub const SRC_PCKSR0: c_uint = 0x30U;
pub const SRC_PCKEN1: c_uint = 0x34U;
pub const SRC_PCKDIS1: c_uint = 0x38U;
pub const SRC_PCKENSR1: c_uint = 0x3CU;
pub const SRC_PCKSR1: c_uint = 0x40U;
// Lock protecting the SRC_CR register
    static DEFINE_SPINLOCK(src_lock);
// Base address of the SRC
    static void __iomem *src_base;
    static int nomadik_clk_reboot_handler(struct notifier_block *this,
    unsigned long code,
    void *unused)
    {
    u32 val;
// The main chrystal need to be enabled for reboot to work
    val = readl(src_base + SRC_XTALCR);
    val &= ~SRC_XTALCR_MXTALOVER;
    val |= SRC_XTALCR_MXTALEN;
    pr_crit("force-enabling MXTALO\n");
    writel(val, src_base + SRC_XTALCR);
    return NOTIFY_OK;
    }
    static struct notifier_block nomadik_clk_reboot_notifier = {
    .notifier_call = nomadik_clk_reboot_handler,
    };
    static const struct of_device_id nomadik_src_match[] __initconst = {
    { .compatible = "stericsson,nomadik-src" },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn nomadik_src_init() -> void __init {
    static void __init nomadik_src_init(void)
    {
    struct device_node *np;
    u32 val;
    np = of_find_matching_node(core::ptr::null_mut(), nomadik_src_match);
    if (!np) {
    pr_crit("no matching node for SRC, aborting clock init\n");
    return;
    }
    src_base = of_iomap(np, 0);
    if (!src_base) {
    pr_err("%s: must have src parent node with REGS (%pOFn)\n",
    __func__, np);
    goto out_put;
    }
// Set all timers to use the 2.4 MHz TIMCLK
    val = readl(src_base + SRC_CR);
    val |= SRC_CR_T0_ENSEL;
    val |= SRC_CR_T1_ENSEL;
    val |= SRC_CR_T2_ENSEL;
    val |= SRC_CR_T3_ENSEL;
    val |= SRC_CR_T4_ENSEL;
    val |= SRC_CR_T5_ENSEL;
    val |= SRC_CR_T6_ENSEL;
    val |= SRC_CR_T7_ENSEL;
    writel(val, src_base + SRC_CR);
    val = readl(src_base + SRC_XTALCR);
    pr_info("SXTALO is %s\n",
    str_disabled_enabled(val & SRC_XTALCR_SXTALDIS));
    pr_info("MXTAL is %s\n",
    str_enabled_disabled(val & SRC_XTALCR_MXTALSTAT));
    if (of_property_read_bool(np, "disable-sxtalo")) {
// The machine uses an external oscillator circuit
    val |= SRC_XTALCR_SXTALDIS;
    pr_info("disabling SXTALO\n");
    }
    if (of_property_read_bool(np, "disable-mxtalo")) {
// Disable this too: also run by external oscillator
    val |= SRC_XTALCR_MXTALOVER;
    val &= ~SRC_XTALCR_MXTALEN;
    pr_info("disabling MXTALO\n");
    }
    writel(val, src_base + SRC_XTALCR);
    register_reboot_notifier(&nomadik_clk_reboot_notifier);
    out_put:
    of_node_put(np);
    }
//
// struct clk_pll - Nomadik PLL clock
// @hw: corresponding clock hardware entry
// @id: PLL instance: 1 or 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub id: c_int,
}

//
// struct clk_src - Nomadik src clock
// @hw: corresponding clock hardware entry
// @id: the clock ID
// @group1: true if the clock is in group1, else it is in group0
// @clkbit: bit 0...31 corresponding to the clock in each clock register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_src {
    pub hw: clk_hw,
    pub id: c_int,
    pub group1: bool,
    pub clkbit: u32,
}

#[no_mangle]
unsafe extern "C" fn pll_clk_enable(hw: *mut clk_hw) -> c_int {
    static int pll_clk_enable(struct clk_hw *hw)
    {
    struct clk_pll *pll = to_pll(hw);
    u32 val;
    spin_lock(&src_lock);
    val = readl(src_base + SRC_PLLCR);
    if (pll.id == 1) {
    if (val & SRC_PLLCR_PLL1OVER) {
    val |= SRC_PLLCR_PLL1EN;
    writel(val, src_base + SRC_PLLCR);
    }
    } else if (pll.id == 2) {
    val |= SRC_PLLCR_PLL2EN;
    writel(val, src_base + SRC_PLLCR);
    }
    spin_unlock(&src_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pll_clk_disable(hw: *mut clk_hw) {
    static void pll_clk_disable(struct clk_hw *hw)
    {
    struct clk_pll *pll = to_pll(hw);
    u32 val;
    spin_lock(&src_lock);
    val = readl(src_base + SRC_PLLCR);
    if (pll.id == 1) {
    if (val & SRC_PLLCR_PLL1OVER) {
    val &= ~SRC_PLLCR_PLL1EN;
    writel(val, src_base + SRC_PLLCR);
    }
    } else if (pll.id == 2) {
    val &= ~SRC_PLLCR_PLL2EN;
    writel(val, src_base + SRC_PLLCR);
    }
    spin_unlock(&src_lock);
    }
#[no_mangle]
unsafe extern "C" fn pll_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int pll_clk_is_enabled(struct clk_hw *hw)
    {
    struct clk_pll *pll = to_pll(hw);
    u32 val;
    val = readl(src_base + SRC_PLLCR);
    if (pll.id == 1) {
    if (val & SRC_PLLCR_PLL1OVER)
    return !!(val & SRC_PLLCR_PLL1EN);
    } else if (pll.id == 2) {
    return !!(val & SRC_PLLCR_PLL2EN);
    }
    return 1;
    }
    static unsigned long pll_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pll *pll = to_pll(hw);
    u32 val;
    val = readl(src_base + SRC_PLLFR);
    if (pll.id == 1) {
    u8 mul;
    u8 div;
    mul = (val >> 8) & 0x3FU;
    mul += 2;
    div = val & 0x07U;
    return (parent_rate * mul) >> div;
    }
    if (pll.id == 2) {
    u8 mul;
    mul = (val >> 24) & 0x3FU;
    mul += 2;
    return (parent_rate * mul);
    }
// Unknown PLL
    return 0;
    }
    static const struct clk_ops pll_clk_ops = {
    .enable = pll_clk_enable,
    .disable = pll_clk_disable,
    .is_enabled = pll_clk_is_enabled,
    .recalc_rate = pll_clk_recalc_rate,
    };
    static struct clk_hw * __init
    pll_clk_register(struct device *dev, const char *name,
    const char *parent_name, u32 id)
    {
    int ret;
    struct clk_pll *pll;
    struct clk_init_data init;
    if (id != 1 && id != 2) {
    pr_err("%s: the Nomadik has only PLL 1 & 2\n", __func__);
    return ERR_PTR(-EINVAL);
    }
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &pll_clk_ops;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    pll.hw.init = &init;
    pll.id = id;
    pr_debug("register PLL1 clock \"%s\"\n", name);
    ret = clk_hw_register(dev, &pll.hw);
    if (ret) {
    kfree(pll);
    return ERR_PTR(ret);
    }
    return &pll.hw;
    }
//
// The Nomadik SRC clocks are gated, but not in the sense that
// you read-modify-write a register. Instead there are separate
// clock enable and clock disable registers. Writing a '1' bit in
// the enable register for a certain clock ungates that clock without
// affecting the other clocks. The disable register works the opposite
// way.
//
#[no_mangle]
unsafe extern "C" fn src_clk_enable(hw: *mut clk_hw) -> c_int {
    static int src_clk_enable(struct clk_hw *hw)
    {
    struct clk_src *sclk = to_src(hw);
    let mut enreg: u32 = sclk.group1 ? SRC_PCKEN1 : SRC_PCKEN0;
    let mut sreg: u32 = sclk.group1 ? SRC_PCKSR1 : SRC_PCKSR0;
    writel(sclk.clkbit, src_base + enreg);
// spin until enabled
    while (!(readl(src_base + sreg) & sclk.clkbit))
    cpu_relax();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn src_clk_disable(hw: *mut clk_hw) {
    static void src_clk_disable(struct clk_hw *hw)
    {
    struct clk_src *sclk = to_src(hw);
    let mut disreg: u32 = sclk.group1 ? SRC_PCKDIS1 : SRC_PCKDIS0;
    let mut sreg: u32 = sclk.group1 ? SRC_PCKSR1 : SRC_PCKSR0;
    writel(sclk.clkbit, src_base + disreg);
// spin until disabled
    while (readl(src_base + sreg) & sclk.clkbit)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn src_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int src_clk_is_enabled(struct clk_hw *hw)
    {
    struct clk_src *sclk = to_src(hw);
    let mut sreg: u32 = sclk.group1 ? SRC_PCKSR1 : SRC_PCKSR0;
    let mut val: u32 = readl(src_base + sreg);
    return !!(val & sclk.clkbit);
    }
    static unsigned long
    src_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return parent_rate;
    }
    static const struct clk_ops src_clk_ops = {
    .enable = src_clk_enable,
    .disable = src_clk_disable,
    .is_enabled = src_clk_is_enabled,
    .recalc_rate = src_clk_recalc_rate,
    };
    static struct clk_hw * __init
    src_clk_register(struct device *dev, const char *name,
    const char *parent_name, u8 id)
    {
    int ret;
    struct clk_src *sclk;
    struct clk_init_data init;
    sclk = kzalloc_obj(*sclk);
    if (!sclk)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &src_clk_ops;
// Do not force-disable the static SDRAM controller
    if (id == 2)
    init.flags = CLK_IGNORE_UNUSED;
    else
    init.flags = 0;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    sclk.hw.init = &init;
    sclk.id = id;
    sclk.group1 = (id > 31);
    sclk.clkbit = BIT(id & 0x1f);
    pr_debug("register clock \"%s\" ID: %d group: %d bits: %08x\n",
    name, id, sclk.group1, sclk.clkbit);
    ret = clk_hw_register(dev, &sclk.hw);
    if (ret) {
    kfree(sclk);
    return ERR_PTR(ret);
    }
    return &sclk.hw;
    }

    static u32 src_pcksr0_boot;
    static u32 src_pcksr1_boot;
    static const char * const src_clk_names[] = {
    "HCLKDMA0  ",
    "HCLKSMC   ",
    "HCLKSDRAM ",
    "HCLKDMA1  ",
    "HCLKCLCD  ",
    "PCLKIRDA  ",
    "PCLKSSP   ",
    "PCLKUART0 ",
    "PCLKSDI   ",
    "PCLKI2C0  ",
    "PCLKI2C1  ",
    "PCLKUART1 ",
    "PCLMSP0   ",
    "HCLKUSB   ",
    "HCLKDIF   ",
    "HCLKSAA   ",
    "HCLKSVA   ",
    "PCLKHSI   ",
    "PCLKXTI   ",
    "PCLKUART2 ",
    "PCLKMSP1  ",
    "PCLKMSP2  ",
    "PCLKOWM   ",
    "HCLKHPI   ",
    "PCLKSKE   ",
    "PCLKHSEM  ",
    "HCLK3D    ",
    "HCLKHASH  ",
    "HCLKCRYP  ",
    "PCLKMSHC  ",
    "HCLKUSBM  ",
    "HCLKRNG   ",
    "RESERVED  ",
    "RESERVED  ",
    "RESERVED  ",
    "RESERVED  ",
    "CLDCLK    ",
    "IRDACLK   ",
    "SSPICLK   ",
    "UART0CLK  ",
    "SDICLK    ",
    "I2C0CLK   ",
    "I2C1CLK   ",
    "UART1CLK  ",
    "MSPCLK0   ",
    "USBCLK    ",
    "DIFCLK    ",
    "IPI2CCLK  ",
    "IPBMCCLK  ",
    "HSICLKRX  ",
    "HSICLKTX  ",
    "UART2CLK  ",
    "MSPCLK1   ",
    "MSPCLK2   ",
    "OWMCLK    ",
    "RESERVED  ",
    "SKECLK    ",
    "RESERVED  ",
    "3DCLK     ",
    "PCLKMSP3  ",
    "MSPCLK3   ",
    "MSHCCLK   ",
    "USBMCLK   ",
    "RNGCCLK   ",
    };
#[no_mangle]
unsafe extern "C" fn nomadik_src_clk_debugfs_show(s: *mut seq_file, what: *mut c_void) -> c_int {
    static int nomadik_src_clk_debugfs_show(struct seq_file *s, void *what)
    {
    int i;
    let mut src_pcksr0: u32 = readl(src_base + SRC_PCKSR0);
    let mut src_pcksr1: u32 = readl(src_base + SRC_PCKSR1);
    let mut src_pckensr0: u32 = readl(src_base + SRC_PCKENSR0);
    let mut src_pckensr1: u32 = readl(src_base + SRC_PCKENSR1);
    seq_puts(s, "Clock:      Boot:   Now:    Request: ASKED:\n");
    for (i = 0; i < ARRAY_SIZE(src_clk_names); i++) {
    let mut pcksrb: u32 = (i < 0x20) ? src_pcksr0_boot : src_pcksr1_boot;
    let mut pcksr: u32 = (i < 0x20) ? src_pcksr0 : src_pcksr1;
    let mut pckreq: u32 = (i < 0x20) ? src_pckensr0 : src_pckensr1;
    let mut mask: u32 = BIT(i & 0x1f);
    seq_printf(s, "%s  %s     %s     %s\n",
    src_clk_names[i],
    (pcksrb & mask) ? "on " : "off",
    (pcksr & mask) ? "on " : "off",
    (pckreq & mask) ? "on " : "off");
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(nomadik_src_clk_debugfs);
#[no_mangle]
unsafe extern "C" fn nomadik_src_clk_init_debugfs() -> int __init {
    static int __init nomadik_src_clk_init_debugfs(void)
    {
// Vital for multiplatform
    if (!src_base)
    return -ENODEV;
    src_pcksr0_boot = readl(src_base + SRC_PCKSR0);
    src_pcksr1_boot = readl(src_base + SRC_PCKSR1);
    debugfs_create_file("nomadik-src-clk", S_IFREG | S_IRUGO,
    core::ptr::null_mut(), core::ptr::null_mut(), &nomadik_src_clk_debugfs_fops);
    return 0;
    }
    device_initcall(nomadik_src_clk_init_debugfs);

#[no_mangle]
unsafe extern "C" fn of_nomadik_pll_setup(np: *mut device_node) -> void __init {
    static void __init of_nomadik_pll_setup(struct device_node *np)
    {
    struct clk_hw *hw;
    const char *clk_name = np.name;
    const char *parent_name;
    u32 pll_id;
    if (!src_base)
    nomadik_src_init();
    if (of_property_read_u32(np, "pll-id", &pll_id)) {
    pr_err("%s: PLL \"%s\" missing pll-id property\n",
    __func__, clk_name);
    return;
    }
    parent_name = of_clk_get_parent_name(np, 0);
    hw = pll_clk_register(core::ptr::null_mut(), clk_name, parent_name, pll_id);
    if (!IS_ERR(hw))
    of_clk_add_hw_provider(np, of_clk_hw_simple_get, hw);
    }
    CLK_OF_DECLARE(nomadik_pll_clk,
    "st,nomadik-pll-clock", of_nomadik_pll_setup);
#[no_mangle]
unsafe extern "C" fn of_nomadik_hclk_setup(np: *mut device_node) -> void __init {
    static void __init of_nomadik_hclk_setup(struct device_node *np)
    {
    struct clk_hw *hw;
    const char *clk_name = np.name;
    const char *parent_name;
    if (!src_base)
    nomadik_src_init();
    parent_name = of_clk_get_parent_name(np, 0);
//
// The HCLK divides PLL1 with 1 (passthru), 2, 3 or 4.
//
    hw = clk_hw_register_divider(core::ptr::null_mut(), clk_name, parent_name,
    0, src_base + SRC_CR,
    13, 2,
    CLK_DIVIDER_ONE_BASED | CLK_DIVIDER_ALLOW_ZERO,
    &src_lock);
    if (!IS_ERR(hw))
    of_clk_add_hw_provider(np, of_clk_hw_simple_get, hw);
    }
    CLK_OF_DECLARE(nomadik_hclk_clk,
    "st,nomadik-hclk-clock", of_nomadik_hclk_setup);
#[no_mangle]
unsafe extern "C" fn of_nomadik_src_clk_setup(np: *mut device_node) -> void __init {
    static void __init of_nomadik_src_clk_setup(struct device_node *np)
    {
    struct clk_hw *hw;
    const char *clk_name = np.name;
    const char *parent_name;
    u32 clk_id;
    if (!src_base)
    nomadik_src_init();
    if (of_property_read_u32(np, "clock-id", &clk_id)) {
    pr_err("%s: SRC clock \"%s\" missing clock-id property\n",
    __func__, clk_name);
    return;
    }
    parent_name = of_clk_get_parent_name(np, 0);
    hw = src_clk_register(core::ptr::null_mut(), clk_name, parent_name, clk_id);
    if (!IS_ERR(hw))
    of_clk_add_hw_provider(np, of_clk_hw_simple_get, hw);
    }
    CLK_OF_DECLARE(nomadik_src_clk,
    "st,nomadik-src-clock", of_nomadik_src_clk_setup);
