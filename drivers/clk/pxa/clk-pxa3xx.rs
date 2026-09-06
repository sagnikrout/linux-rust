//! Automatically rewritten from C to Rust
//! Source: drivers/clk/pxa/clk-pxa3xx.c
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
// Marvell PXA3xxx family clocks
//
// Copyright (C) 2014 Robert Jarzmik
//
// Heavily inspired from former arch/arm/mach-pxa/pxa3xx.c
//
// For non-devicetree platforms. Once pxa is fully converted to devicetree, this
// should go away.
//

pub const KHz: c_int = 1000;

//
// Clock Enable Bit
//

// Note: GCU clock enable bit differs on PXA300/PXA310 and PXA320

    enum {
    PXA_CORE_60Mhz = 0,
    PXA_CORE_RUN,
    PXA_CORE_TURBO,
    };
    enum {
    PXA_BUS_60Mhz = 0,
    PXA_BUS_HSS,
    };
// crystal frequency to HSIO bus frequency multiplier (HSS)
    static unsigned char hss_mult[4] = { 8, 12, 16, 24 };
// crystal frequency to static memory controller multiplier (SMCFS)
    static unsigned int smcfs_mult[8] = { 6, 0, 8, 0, 0, 16, };
    static const char * const get_freq_khz[] = {
    "core", "ring_osc_60mhz", "run", "cpll", "system_bus"
    };
    static void __iomem *clk_regs;
//
// Get the clock frequency as reflected by ACSR and the turbo flag.
// We assume these values have been applied via a fcs.
// If info is not 0 we also display the current settings.
//
#[no_mangle]
pub unsafe extern "C" fn pxa3xx_get_clk_frequency_khz(info: c_int) -> c_uint {
    unsigned int pxa3xx_get_clk_frequency_khz(int info)
    {
    struct clk *clk;
    unsigned long clks[5];
    int i;
    for (i = 0; i < 5; i++) {
    clk = clk_get(core::ptr::null_mut(), get_freq_khz[i]);
    if (IS_ERR(clk)) {
    clks[i] = 0;
    } else {
    clks[i] = clk_get_rate(clk);
    clk_put(clk);
    }
    }
    if (info) {
    pr_info("RO Mode clock: %ld.%02ldMHz\n",
    clks[1] / 1000000, (clks[0] % 1000000) / 10000);
    pr_info("Run Mode clock: %ld.%02ldMHz\n",
    clks[2] / 1000000, (clks[1] % 1000000) / 10000);
    pr_info("Turbo Mode clock: %ld.%02ldMHz\n",
    clks[3] / 1000000, (clks[2] % 1000000) / 10000);
    pr_info("System bus clock: %ld.%02ldMHz\n",
    clks[4] / 1000000, (clks[4] % 1000000) / 10000);
    }
    return (unsigned int)clks[0] / KHz;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa3xx_clk_update_accr(disable: u32, enable: u32, xclkcfg: u32, mask: u32) {
    void pxa3xx_clk_update_accr(u32 disable, u32 enable, u32 xclkcfg, u32 mask)
    {
    let mut accr: u32 = readl(clk_regs + ACCR);
    accr &= ~disable;
    accr |= enable;
    writel(accr, clk_regs + ACCR);
    if (xclkcfg)
    __asm__("mcr p14, 0, %0, c6, c0, 0\n" : : "r"(xclkcfg));
    while ((readl(clk_regs + ACSR) & mask) != (accr & mask))
    cpu_relax();
    }
    static unsigned long clk_pxa3xx_ac97_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    unsigned long ac97_div, rate;
    ac97_div = readl(clk_regs + AC97_DIV);
// This may loose precision for some rates but won't for the
// standard 24.576MHz.
//
    rate = parent_rate / 2;
    rate /= ((ac97_div >> 12) & 0x7fff);
    rate *= (ac97_div & 0xfff);
    return rate;
    }
    PARENTS(clk_pxa3xx_ac97) = { "spll_624mhz" };
    RATE_RO_OPS(clk_pxa3xx_ac97, "ac97");
    static unsigned long clk_pxa3xx_smemc_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    let mut acsr: c_ulong = readl(clk_regs + ACSR);
    return (parent_rate / 48)  * smcfs_mult[(acsr >> 23) & 0x7] /
    pxa3xx_smemc_get_memclkdiv();
    }
    PARENTS(clk_pxa3xx_smemc) = { "spll_624mhz" };
    RATE_RO_OPS(clk_pxa3xx_smemc, "smemc");
#[no_mangle]
unsafe extern "C" fn pxa3xx_is_ring_osc_forced() -> bool {
    static bool pxa3xx_is_ring_osc_forced(void)
    {
    let mut acsr: c_ulong = readl(clk_regs + ACSR);
    return acsr & ACCR_D0CS;
    }
    PARENTS(pxa3xx_pbus) = { "ring_osc_60mhz", "spll_624mhz" };
    PARENTS(pxa3xx_32Khz_bus) = { "osc_32_768khz", "osc_32_768khz" };
    PARENTS(pxa3xx_13MHz_bus) = { "osc_13mhz", "osc_13mhz" };
    PARENTS(pxa3xx_ac97_bus) = { "ring_osc_60mhz", "ac97" };
    PARENTS(pxa3xx_sbus) = { "ring_osc_60mhz", "system_bus" };
    PARENTS(pxa3xx_smemcbus) = { "ring_osc_60mhz", "smemc" };

    div_hp, bit, is_lp, flags)				\
    PXA_CKEN(dev_id, con_id, bit, parents, mult_lp, div_lp,		\
    mult_hp, div_hp, is_lp,  CKEN_AB(bit),			\
    (CKEN_ ## bit % 32), flags)

    mult_hp, div_hp, delay)			\
    PXA3XX_CKEN(dev_id, con_id, pxa3xx_pbus_parents, mult_lp,	\
    div_lp, mult_hp, div_hp, bit, pxa3xx_is_ring_osc_forced, 0)

    PXA_CKEN_1RATE(dev_id, con_id, bit, parents,			\
    CKEN_AB(bit), (CKEN_ ## bit % 32), 0)
    static struct desc_clk_cken pxa3xx_clocks[] __initdata = {
    PXA3XX_PBUS_CKEN("pxa2xx-uart.0", core::ptr::null_mut(), FFUART, 1, 4, 1, 42, 1),
    PXA3XX_PBUS_CKEN("pxa2xx-uart.1", core::ptr::null_mut(), BTUART, 1, 4, 1, 42, 1),
    PXA3XX_PBUS_CKEN("pxa2xx-uart.2", core::ptr::null_mut(), STUART, 1, 4, 1, 42, 1),
    PXA3XX_PBUS_CKEN("pxa2xx-i2c.0", core::ptr::null_mut(), I2C, 2, 5, 1, 19, 0),
    PXA3XX_PBUS_CKEN("pxa27x-udc", core::ptr::null_mut(), UDC, 1, 4, 1, 13, 5),
    PXA3XX_PBUS_CKEN("pxa27x-ohci", core::ptr::null_mut(), USBH, 1, 4, 1, 13, 0),
    PXA3XX_PBUS_CKEN("pxa3xx-u2d", core::ptr::null_mut(), USB2, 1, 4, 1, 13, 0),
    PXA3XX_PBUS_CKEN("pxa27x-pwm.0", core::ptr::null_mut(), PWM0, 1, 6, 1, 48, 0),
    PXA3XX_PBUS_CKEN("pxa27x-pwm.1", core::ptr::null_mut(), PWM1, 1, 6, 1, 48, 0),
    PXA3XX_PBUS_CKEN("pxa2xx-mci.0", core::ptr::null_mut(), MMC1, 1, 4, 1, 24, 0),
    PXA3XX_PBUS_CKEN("pxa2xx-mci.1", core::ptr::null_mut(), MMC2, 1, 4, 1, 24, 0),
    PXA3XX_PBUS_CKEN("pxa2xx-mci.2", core::ptr::null_mut(), MMC3, 1, 4, 1, 24, 0),
    PXA3XX_CKEN_1RATE("pxa27x-keypad", core::ptr::null_mut(), KEYPAD,
    pxa3xx_32Khz_bus_parents),
    PXA3XX_CKEN_1RATE("pxa3xx-ssp.0", core::ptr::null_mut(), SSP1, pxa3xx_13MHz_bus_parents),
    PXA3XX_CKEN_1RATE("pxa3xx-ssp.1", core::ptr::null_mut(), SSP2, pxa3xx_13MHz_bus_parents),
    PXA3XX_CKEN_1RATE("pxa3xx-ssp.2", core::ptr::null_mut(), SSP3, pxa3xx_13MHz_bus_parents),
    PXA3XX_CKEN_1RATE("pxa3xx-ssp.3", core::ptr::null_mut(), SSP4, pxa3xx_13MHz_bus_parents),
    PXA3XX_CKEN(core::ptr::null_mut(), "AC97CLK", pxa3xx_ac97_bus_parents, 1, 4, 1, 1, AC97,
    pxa3xx_is_ring_osc_forced, 0),
    PXA3XX_CKEN(core::ptr::null_mut(), "CAMCLK", pxa3xx_sbus_parents, 1, 2, 1, 1, CAMERA,
    pxa3xx_is_ring_osc_forced, 0),
    PXA3XX_CKEN("pxa2xx-fb", core::ptr::null_mut(), pxa3xx_sbus_parents, 1, 1, 1, 1, LCD,
    pxa3xx_is_ring_osc_forced, 0),
    PXA3XX_CKEN("pxa2xx-pcmcia", core::ptr::null_mut(), pxa3xx_smemcbus_parents, 1, 4,
    1, 1, SMC, pxa3xx_is_ring_osc_forced, CLK_IGNORE_UNUSED),
    };
    static struct desc_clk_cken pxa300_310_clocks[] __initdata = {
    PXA3XX_PBUS_CKEN("pxa3xx-gcu", core::ptr::null_mut(), PXA300_GCU, 1, 1, 1, 1, 0),
    PXA3XX_PBUS_CKEN("pxa3xx-nand", core::ptr::null_mut(), NAND, 1, 2, 1, 4, 0),
    PXA3XX_CKEN_1RATE("pxa3xx-gpio", core::ptr::null_mut(), GPIO, pxa3xx_13MHz_bus_parents),
    };
    static struct desc_clk_cken pxa320_clocks[] __initdata = {
    PXA3XX_PBUS_CKEN("pxa3xx-nand", core::ptr::null_mut(), NAND, 1, 2, 1, 6, 0),
    PXA3XX_PBUS_CKEN("pxa3xx-gcu", core::ptr::null_mut(), PXA320_GCU, 1, 1, 1, 1, 0),
    PXA3XX_CKEN_1RATE("pxa3xx-gpio", core::ptr::null_mut(), GPIO, pxa3xx_13MHz_bus_parents),
    };
    static struct desc_clk_cken pxa93x_clocks[] __initdata = {
    PXA3XX_PBUS_CKEN("pxa3xx-gcu", core::ptr::null_mut(), PXA300_GCU, 1, 1, 1, 1, 0),
    PXA3XX_PBUS_CKEN("pxa3xx-nand", core::ptr::null_mut(), NAND, 1, 2, 1, 4, 0),
    PXA3XX_CKEN_1RATE("pxa93x-gpio", core::ptr::null_mut(), GPIO, pxa3xx_13MHz_bus_parents),
    };
    static unsigned long clk_pxa3xx_system_bus_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    let mut acsr: c_ulong = readl(clk_regs + ACSR);
    let mut hss: c_uint = (acsr >> 14) & 0x3;
    if (pxa3xx_is_ring_osc_forced())
    return parent_rate;
    return parent_rate / 48 * hss_mult[hss];
    }
#[no_mangle]
unsafe extern "C" fn clk_pxa3xx_system_bus_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_pxa3xx_system_bus_get_parent(struct clk_hw *hw)
    {
    if (pxa3xx_is_ring_osc_forced())
    return PXA_BUS_60Mhz;
    else
    return PXA_BUS_HSS;
    }
    PARENTS(clk_pxa3xx_system_bus) = { "ring_osc_60mhz", "spll_624mhz" };
    MUX_RO_RATE_RO_OPS(clk_pxa3xx_system_bus, "system_bus");
    static unsigned long clk_pxa3xx_core_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return parent_rate;
    }
#[no_mangle]
unsafe extern "C" fn clk_pxa3xx_core_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_pxa3xx_core_get_parent(struct clk_hw *hw)
    {
    unsigned long xclkcfg;
    unsigned int t;
    if (pxa3xx_is_ring_osc_forced())
    return PXA_CORE_60Mhz;
// Read XCLKCFG register turbo bit
    __asm__ __volatile__("mrc\tp14, 0, %0, c6, c0, 0" : "=r"(xclkcfg));
    t = xclkcfg & 0x1;
    if (t)
    return PXA_CORE_TURBO;
    return PXA_CORE_RUN;
    }
    PARENTS(clk_pxa3xx_core) = { "ring_osc_60mhz", "run", "cpll" };
    MUX_RO_RATE_RO_OPS(clk_pxa3xx_core, "core");
    static unsigned long clk_pxa3xx_run_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    let mut acsr: c_ulong = readl(clk_regs + ACSR);
    let mut xn: c_uint = (acsr & ACCR_XN_MASK) >> 8;
    unsigned int t, xclkcfg;
// Read XCLKCFG register turbo bit
    __asm__ __volatile__("mrc\tp14, 0, %0, c6, c0, 0" : "=r"(xclkcfg));
    t = xclkcfg & 0x1;
    return t ? (parent_rate / xn) * 2 : parent_rate;
    }
    PARENTS(clk_pxa3xx_run) = { "cpll" };
    RATE_RO_OPS(clk_pxa3xx_run, "run");
    static unsigned long clk_pxa3xx_cpll_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    let mut acsr: c_ulong = readl(clk_regs + ACSR);
    let mut xn: c_uint = (acsr & ACCR_XN_MASK) >> 8;
    let mut xl: c_uint = acsr & ACCR_XL_MASK;
    unsigned int t, xclkcfg;
// Read XCLKCFG register turbo bit
    __asm__ __volatile__("mrc\tp14, 0, %0, c6, c0, 0" : "=r"(xclkcfg));
    t = xclkcfg & 0x1;
    pr_info("RJK: parent_rate=%lu, xl=%u, xn=%u\n", parent_rate, xl, xn);
    return t ? parent_rate * xl * xn : parent_rate * xl;
    }
    PARENTS(clk_pxa3xx_cpll) = { "osc_13mhz" };
    RATE_RO_OPS(clk_pxa3xx_cpll, "cpll");
#[no_mangle]
unsafe extern "C" fn pxa3xx_register_core() -> void __init {
    static void __init pxa3xx_register_core(void)
    {
    clk_register_clk_pxa3xx_cpll();
    clk_register_clk_pxa3xx_run();
    clkdev_pxa_register(CLK_CORE, "core", core::ptr::null_mut(),
    clk_register_clk_pxa3xx_core());
    }
#[no_mangle]
unsafe extern "C" fn pxa3xx_register_plls() -> void __init {
    static void __init pxa3xx_register_plls(void)
    {
    clk_register_fixed_rate(core::ptr::null_mut(), "osc_13mhz", core::ptr::null_mut(),
    CLK_GET_RATE_NOCACHE,
    13 * MHz);
    clkdev_pxa_register(CLK_OSC32k768, "osc_32_768khz", core::ptr::null_mut(),
    clk_register_fixed_rate(core::ptr::null_mut(), "osc_32_768khz", core::ptr::null_mut(),
    CLK_GET_RATE_NOCACHE,
    32768));
    clk_register_fixed_rate(core::ptr::null_mut(), "ring_osc_120mhz", core::ptr::null_mut(),
    CLK_GET_RATE_NOCACHE,
    120 * MHz);
    clk_register_fixed_rate(core::ptr::null_mut(), "clk_dummy", core::ptr::null_mut(), 0, 0);
    clk_register_fixed_factor(core::ptr::null_mut(), "spll_624mhz", "osc_13mhz", 0, 48, 1);
    clk_register_fixed_factor(core::ptr::null_mut(), "ring_osc_60mhz", "ring_osc_120mhz",
    0, 1, 2);
    }

    { .con_id = _con_id, .dev_id = _dev_id, .parent = _parent }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy_clk {
    pub con_id: *const c_char,
    pub dev_id: *const c_char,
    pub parent: *const c_char,
}

    static struct dummy_clk dummy_clks[] __initdata = {
    DUMMY_CLK(core::ptr::null_mut(), "pxa93x-gpio", "osc_13mhz"),
    DUMMY_CLK(core::ptr::null_mut(), "sa1100-rtc", "osc_32_768khz"),
    DUMMY_CLK("UARTCLK", "pxa2xx-ir", "STUART"),
    DUMMY_CLK(core::ptr::null_mut(), "pxa3xx-pwri2c.1", "osc_13mhz"),
    };
#[no_mangle]
unsafe extern "C" fn pxa3xx_dummy_clocks_init() -> void __init {
    static void __init pxa3xx_dummy_clocks_init(void)
    {
    struct clk *clk;
    struct dummy_clk *d;
    const char *name;
    int i;
    for (i = 0; i < ARRAY_SIZE(dummy_clks); i++) {
    d = &dummy_clks[i];
    name = d.dev_id ? d.dev_id : d.con_id;
    clk = clk_register_fixed_factor(core::ptr::null_mut(), name, d.parent, 0, 1, 1);
    clk_register_clkdev(clk, d.con_id, d.dev_id);
    }
    }
#[no_mangle]
unsafe extern "C" fn pxa3xx_base_clocks_init(oscc_reg: *mut void __iomem) -> void __init {
    static void __init pxa3xx_base_clocks_init(void __iomem *oscc_reg)
    {
    struct clk *clk;
    pxa3xx_register_plls();
    pxa3xx_register_core();
    clk_register_clk_pxa3xx_system_bus();
    clk_register_clk_pxa3xx_ac97();
    clk_register_clk_pxa3xx_smemc();
    clk = clk_register_gate(core::ptr::null_mut(), "CLK_POUT",
    "osc_13mhz", 0, oscc_reg, 11, 0, core::ptr::null_mut());
    clk_register_clkdev(clk, "CLK_POUT", core::ptr::null_mut());
    clkdev_pxa_register(CLK_OSTIMER, "OSTIMER0", core::ptr::null_mut(),
    clk_register_fixed_factor(core::ptr::null_mut(), "os-timer0",
    "osc_13mhz", 0, 1, 4));
    }
#[no_mangle]
pub unsafe extern "C" fn pxa3xx_clocks_init(regs: *mut void __iomem, oscc_reg: *mut void __iomem) -> int __init {
    int __init pxa3xx_clocks_init(void __iomem *regs, void __iomem *oscc_reg)
    {
    int ret;
    clk_regs = regs;
    pxa3xx_base_clocks_init(oscc_reg);
    pxa3xx_dummy_clocks_init();
    ret = clk_pxa_cken_init(pxa3xx_clocks, ARRAY_SIZE(pxa3xx_clocks), regs);
    if (ret)
    return ret;
    if (cpu_is_pxa320())
    return clk_pxa_cken_init(pxa320_clocks,
    ARRAY_SIZE(pxa320_clocks), regs);
    if (cpu_is_pxa300() || cpu_is_pxa310())
    return clk_pxa_cken_init(pxa300_310_clocks,
    ARRAY_SIZE(pxa300_310_clocks), regs);
    return clk_pxa_cken_init(pxa93x_clocks, ARRAY_SIZE(pxa93x_clocks), regs);
    }
#[no_mangle]
unsafe extern "C" fn pxa3xx_dt_clocks_init(np: *mut device_node) -> void __init {
    static void __init pxa3xx_dt_clocks_init(struct device_node *np)
    {
    pxa3xx_clocks_init(ioremap(0x41340000, 0x10), ioremap(0x41350000, 4));
    clk_pxa_dt_common_init(np);
    }
    CLK_OF_DECLARE(pxa_clks, "marvell,pxa300-clocks", pxa3xx_dt_clocks_init);
