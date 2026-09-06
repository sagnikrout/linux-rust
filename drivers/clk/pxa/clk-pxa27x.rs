//! Automatically rewritten from C to Rust
//! Source: drivers/clk/pxa/clk-pxa27x.c
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
// Marvell PXA27x family clocks
//
// Copyright (C) 2014 Robert Jarzmik
//
// Heavily inspired from former arch/arm/mach-pxa/clock.c.
//

pub const KHz: c_int = 1000;

    enum {
    PXA_CORE_13Mhz = 0,
    PXA_CORE_RUN,
    PXA_CORE_TURBO,
    };
    enum {
    PXA_BUS_13Mhz = 0,
    PXA_BUS_RUN,
    };
    enum {
    PXA_LCD_13Mhz = 0,
    PXA_LCD_RUN,
    };
    enum {
    PXA_MEM_13Mhz = 0,
    PXA_MEM_SYSTEM_BUS,
    PXA_MEM_RUN,
    };

    (CLKCFG_FCS |				\
    ((B)  ? CLKCFG_FASTBUS : 0) |		\
    ((HT) ? CLKCFG_HALFTURBO : 0) |	\
    ((T)  ? CLKCFG_TURBO : 0))

// Define the refresh period in mSec for the SDRAM and the number of rows

    static void __iomem *clk_regs;
    static const char * const get_freq_khz[] = {
    "core", "run", "cpll", "memory",
    "system_bus"
    };
#[no_mangle]
unsafe extern "C" fn mdrefr_dri(freq_khz: c_uint) -> u32 {
    static u32 mdrefr_dri(unsigned int freq_khz)
    {
    let mut interval: u32 = freq_khz * SDRAM_TREF / pxa2xx_smemc_get_sdram_rows();
    return (interval - 31) / 32;
    }
//
// Get the clock frequency as reflected by CCSR and the turbo flag.
// We assume these values have been applied via a fcs.
// If info is not 0 we also display the current settings.
//
#[no_mangle]
pub unsafe extern "C" fn pxa27x_get_clk_frequency_khz(info: c_int) -> c_uint {
    unsigned int pxa27x_get_clk_frequency_khz(int info)
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
    pr_info("Run Mode clock: %ld.%02ldMHz\n",
    clks[1] / 1000000, (clks[1] % 1000000) / 10000);
    pr_info("Turbo Mode clock: %ld.%02ldMHz\n",
    clks[2] / 1000000, (clks[2] % 1000000) / 10000);
    pr_info("Memory clock: %ld.%02ldMHz\n",
    clks[3] / 1000000, (clks[3] % 1000000) / 10000);
    pr_info("System bus clock: %ld.%02ldMHz\n",
    clks[4] / 1000000, (clks[4] % 1000000) / 10000);
    }
    return (unsigned int)clks[0] / KHz;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_is_ppll_disabled() -> bool {
    static bool pxa27x_is_ppll_disabled(void)
    {
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    return ccsr & (1 << CCCR_PPDIS_BIT);
    }

    bit, is_lp, flags)					\
    PXA_CKEN(dev_id, con_id, bit, parents, 1, 1, mult_hp, div_hp,	\
    is_lp,  CKEN, CKEN_ ## bit, flags)

    PXA27X_CKEN(dev_id, con_id, pxa27x_pbus_parents, mult_hp,	\
    div_hp, bit, pxa27x_is_ppll_disabled, 0)
    PARENTS(pxa27x_pbus) = { "osc_13mhz", "ppll_312mhz" };
    PARENTS(pxa27x_sbus) = { "system_bus", "system_bus" };
    PARENTS(pxa27x_32Mhz_bus) = { "osc_32_768khz", "osc_32_768khz" };
    PARENTS(pxa27x_lcd_bus) = { "lcd_base", "lcd_base" };
    PARENTS(pxa27x_membus) = { "lcd_base", "lcd_base" };

    PXA_CKEN_1RATE(dev_id, con_id, bit, parents,			\
    CKEN, CKEN_ ## bit, 0)

    PXA_CKEN_1RATE(dev_id, con_id, bit, parents,			\
    CKEN, CKEN_ ## bit, CLK_IGNORE_UNUSED)
    static struct desc_clk_cken pxa27x_clocks[] __initdata = {
    PXA27X_PBUS_CKEN("pxa2xx-uart.0", core::ptr::null_mut(), FFUART, 2, 42, 1),
    PXA27X_PBUS_CKEN("pxa2xx-uart.1", core::ptr::null_mut(), BTUART, 2, 42, 1),
    PXA27X_PBUS_CKEN("pxa2xx-uart.2", core::ptr::null_mut(), STUART, 2, 42, 1),
    PXA27X_PBUS_CKEN("pxa2xx-i2s", core::ptr::null_mut(), I2S, 2, 51, 0),
    PXA27X_PBUS_CKEN("pxa2xx-i2c.0", core::ptr::null_mut(), I2C, 2, 19, 0),
    PXA27X_PBUS_CKEN("pxa27x-udc", core::ptr::null_mut(), USB, 2, 13, 5),
    PXA27X_PBUS_CKEN("pxa2xx-mci.0", core::ptr::null_mut(), MMC, 2, 32, 0),
    PXA27X_PBUS_CKEN("pxa2xx-ir", "FICPCLK", FICP, 2, 13, 0),
    PXA27X_PBUS_CKEN("pxa27x-ohci", core::ptr::null_mut(), USBHOST, 2, 13, 0),
    PXA27X_PBUS_CKEN("pxa2xx-i2c.1", core::ptr::null_mut(), PWRI2C, 1, 24, 0),
    PXA27X_PBUS_CKEN("pxa27x-ssp.0", core::ptr::null_mut(), SSP1, 1, 24, 0),
    PXA27X_PBUS_CKEN("pxa27x-ssp.1", core::ptr::null_mut(), SSP2, 1, 24, 0),
    PXA27X_PBUS_CKEN("pxa27x-ssp.2", core::ptr::null_mut(), SSP3, 1, 24, 0),
    PXA27X_PBUS_CKEN("pxa27x-pwm.0", core::ptr::null_mut(), PWM0, 1, 24, 0),
    PXA27X_PBUS_CKEN("pxa27x-pwm.1", core::ptr::null_mut(), PWM1, 1, 24, 0),
    PXA27X_PBUS_CKEN(core::ptr::null_mut(), "MSLCLK", MSL, 2, 13, 0),
    PXA27X_PBUS_CKEN(core::ptr::null_mut(), "USIMCLK", USIM, 2, 13, 0),
    PXA27X_PBUS_CKEN(core::ptr::null_mut(), "MSTKCLK", MEMSTK, 2, 32, 0),
    PXA27X_PBUS_CKEN(core::ptr::null_mut(), "AC97CLK", AC97, 1, 1, 0),
    PXA27X_PBUS_CKEN(core::ptr::null_mut(), "AC97CONFCLK", AC97CONF, 1, 1, 0),
    PXA27X_PBUS_CKEN(core::ptr::null_mut(), "OSTIMER0", OSTIMER, 1, 96, 0),
    PXA27X_CKEN_1RATE("pxa27x-keypad", core::ptr::null_mut(), KEYPAD,
    pxa27x_32Mhz_bus_parents, 0),
    PXA27X_CKEN_1RATE(core::ptr::null_mut(), "IMCLK", IM, pxa27x_sbus_parents, 0),
    PXA27X_CKEN_1RATE("pxa2xx-fb", core::ptr::null_mut(), LCD, pxa27x_lcd_bus_parents, 0),
    PXA27X_CKEN_1RATE("pxa27x-camera.0", core::ptr::null_mut(), CAMERA,
    pxa27x_lcd_bus_parents, 0),
    PXA27X_CKEN_1RATE_AO("pxa2xx-pcmcia", core::ptr::null_mut(), MEMC,
    pxa27x_membus_parents, 0),
    };
//
// PXA270 definitions
//
// For the PXA27x:
// Control variables are A, L, 2N for CCCR; B, HT, T for CLKCFG.
//
// A = 0 => memory controller clock from table 3-7,
// A = 1 => memory controller clock = system bus clock
// Run mode frequency	= 13 MHz * L
// Turbo mode frequency = 13 MHz * L * N
// System bus frequency = 13 MHz * L / (B + 1)
//
// In CCCR:
// A = 1
// L = 16	  oscillator to run mode ratio
// 2N = 6	  2 * (turbo mode to run mode ratio)
//
// In CCLKCFG:
// B = 1	  Fast bus mode
// HT = 0	  Half-Turbo mode
// T = 1	  Turbo mode
//
// For now, just support some of the combinations in table 3-7 of
// PXA27x Processor Family Developer's Manual to simplify frequency
// change sequences.
//
    static struct pxa2xx_freq pxa27x_freqs[] = {
    {104000000, 104000, PXA27x_CCCR(1,  8, 2), 0, PXA27x_CLKCFG(1, 0, 1) },
    {156000000, 104000, PXA27x_CCCR(1,  8, 3), 0, PXA27x_CLKCFG(1, 0, 1) },
    {208000000, 208000, PXA27x_CCCR(0, 16, 2), 1, PXA27x_CLKCFG(0, 0, 1) },
    {312000000, 208000, PXA27x_CCCR(1, 16, 3), 1, PXA27x_CLKCFG(1, 0, 1) },
    {416000000, 208000, PXA27x_CCCR(1, 16, 4), 1, PXA27x_CLKCFG(1, 0, 1) },
    {520000000, 208000, PXA27x_CCCR(1, 16, 5), 1, PXA27x_CLKCFG(1, 0, 1) },
    {624000000, 208000, PXA27x_CCCR(1, 16, 6), 1, PXA27x_CLKCFG(1, 0, 1) },
    };
    static unsigned long clk_pxa27x_cpll_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    unsigned long clkcfg;
    unsigned int t, ht;
    unsigned int l, L, n2, N;
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    asm("mrc\tp14, 0, %0, c6, c0, 0" : "=r" (clkcfg));
    t  = clkcfg & (1 << 0);
    ht = clkcfg & (1 << 2);
    l  = ccsr & CCSR_L_MASK;
    n2 = (ccsr & CCSR_N2_MASK) >> CCSR_N2_SHIFT;
    L  = l * parent_rate;
    N  = (L * n2) / 2;
    return N;
    }
    static int clk_pxa27x_cpll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    return pxa2xx_determine_rate(req, pxa27x_freqs,
    ARRAY_SIZE(pxa27x_freqs));
    }
    static int clk_pxa27x_cpll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    int i;
    pr_debug("%s(rate=%lu parent_rate=%lu)\n", __func__, rate, parent_rate);
    for (i = 0; i < ARRAY_SIZE(pxa27x_freqs); i++)
    if (pxa27x_freqs[i].cpll == rate)
    break;
    if (i >= ARRAY_SIZE(pxa27x_freqs))
    return -EINVAL;
    pxa2xx_cpll_change(&pxa27x_freqs[i], mdrefr_dri, clk_regs + CCCR);
    return 0;
    }
    PARENTS(clk_pxa27x_cpll) = { "osc_13mhz" };
    RATE_OPS(clk_pxa27x_cpll, "cpll");
    static unsigned long clk_pxa27x_lcd_base_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    unsigned int l, osc_forced;
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    let mut cccr: c_ulong = readl(clk_regs + CCCR);
    l  = ccsr & CCSR_L_MASK;
    osc_forced = ccsr & (1 << CCCR_CPDIS_BIT);
    if (osc_forced) {
    if (cccr & (1 << CCCR_LCD_26_BIT))
    return parent_rate * 2;
    else
    return parent_rate;
    }
    if (l <= 7)
    return parent_rate;
    if (l <= 16)
    return parent_rate / 2;
    return parent_rate / 4;
    }
#[no_mangle]
unsafe extern "C" fn clk_pxa27x_lcd_base_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_pxa27x_lcd_base_get_parent(struct clk_hw *hw)
    {
    unsigned int osc_forced;
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    osc_forced = ccsr & (1 << CCCR_CPDIS_BIT);
    if (osc_forced)
    return PXA_LCD_13Mhz;
    else
    return PXA_LCD_RUN;
    }
    PARENTS(clk_pxa27x_lcd_base) = { "osc_13mhz", "run" };
    MUX_RO_RATE_RO_OPS(clk_pxa27x_lcd_base, "lcd_base");
#[no_mangle]
unsafe extern "C" fn pxa27x_register_plls() -> void __init {
    static void __init pxa27x_register_plls(void)
    {
    clk_register_fixed_rate(core::ptr::null_mut(), "osc_13mhz", core::ptr::null_mut(),
    CLK_GET_RATE_NOCACHE,
    13 * MHz);
    clkdev_pxa_register(CLK_OSC32k768, "osc_32_768khz", core::ptr::null_mut(),
    clk_register_fixed_rate(core::ptr::null_mut(), "osc_32_768khz", core::ptr::null_mut(),
    CLK_GET_RATE_NOCACHE,
    32768 * KHz));
    clk_register_fixed_rate(core::ptr::null_mut(), "clk_dummy", core::ptr::null_mut(), 0, 0);
    clk_register_fixed_factor(core::ptr::null_mut(), "ppll_312mhz", "osc_13mhz", 0, 24, 1);
    }
#[no_mangle]
unsafe extern "C" fn clk_pxa27x_core_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_pxa27x_core_get_parent(struct clk_hw *hw)
    {
    unsigned long clkcfg;
    unsigned int t, ht, osc_forced;
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    osc_forced = ccsr & (1 << CCCR_CPDIS_BIT);
    if (osc_forced)
    return PXA_CORE_13Mhz;
    asm("mrc\tp14, 0, %0, c6, c0, 0" : "=r" (clkcfg));
    t  = clkcfg & (1 << 0);
    ht = clkcfg & (1 << 2);
    if (ht || t)
    return PXA_CORE_TURBO;
    return PXA_CORE_RUN;
    }
#[no_mangle]
unsafe extern "C" fn clk_pxa27x_core_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_pxa27x_core_set_parent(struct clk_hw *hw, u8 index)
    {
    if (index > PXA_CORE_TURBO)
    return -EINVAL;
    pxa2xx_core_turbo_switch(index == PXA_CORE_TURBO);
    return 0;
    }
    static int clk_pxa27x_core_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    return __clk_mux_determine_rate(hw, req);
    }
    PARENTS(clk_pxa27x_core) = { "osc_13mhz", "run", "cpll" };
    MUX_OPS(clk_pxa27x_core, "core", CLK_SET_RATE_PARENT);
    static unsigned long clk_pxa27x_run_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    let mut n2: c_uint = (ccsr & CCSR_N2_MASK) >> CCSR_N2_SHIFT;
    return (parent_rate / n2) * 2;
    }
    PARENTS(clk_pxa27x_run) = { "cpll" };
    RATE_RO_OPS(clk_pxa27x_run, "run");
#[no_mangle]
unsafe extern "C" fn pxa27x_register_core() -> void __init {
    static void __init pxa27x_register_core(void)
    {
    clkdev_pxa_register(CLK_NONE, "cpll", core::ptr::null_mut(),
    clk_register_clk_pxa27x_cpll());
    clkdev_pxa_register(CLK_NONE, "run", core::ptr::null_mut(),
    clk_register_clk_pxa27x_run());
    clkdev_pxa_register(CLK_CORE, "core", core::ptr::null_mut(),
    clk_register_clk_pxa27x_core());
    }
    static unsigned long clk_pxa27x_system_bus_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    unsigned long clkcfg;
    unsigned int b, osc_forced;
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    osc_forced = ccsr & (1 << CCCR_CPDIS_BIT);
    asm("mrc\tp14, 0, %0, c6, c0, 0" : "=r" (clkcfg));
    b  = clkcfg & (1 << 3);
    if (osc_forced)
    return parent_rate;
    if (b)
    return parent_rate;
    else
    return parent_rate / 2;
    }
#[no_mangle]
unsafe extern "C" fn clk_pxa27x_system_bus_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_pxa27x_system_bus_get_parent(struct clk_hw *hw)
    {
    unsigned int osc_forced;
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    osc_forced = ccsr & (1 << CCCR_CPDIS_BIT);
    if (osc_forced)
    return PXA_BUS_13Mhz;
    else
    return PXA_BUS_RUN;
    }
    PARENTS(clk_pxa27x_system_bus) = { "osc_13mhz", "run" };
    MUX_RO_RATE_RO_OPS(clk_pxa27x_system_bus, "system_bus");
    static unsigned long clk_pxa27x_memory_get_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    unsigned int a, l, osc_forced;
    let mut cccr: c_ulong = readl(clk_regs + CCCR);
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    osc_forced = ccsr & (1 << CCCR_CPDIS_BIT);
    a = cccr & (1 << CCCR_A_BIT);
    l  = ccsr & CCSR_L_MASK;
    if (osc_forced || a)
    return parent_rate;
    if (l <= 10)
    return parent_rate;
    if (l <= 20)
    return parent_rate / 2;
    return parent_rate / 4;
    }
#[no_mangle]
unsafe extern "C" fn clk_pxa27x_memory_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_pxa27x_memory_get_parent(struct clk_hw *hw)
    {
    unsigned int osc_forced, a;
    let mut cccr: c_ulong = readl(clk_regs + CCCR);
    let mut ccsr: c_ulong = readl(clk_regs + CCSR);
    osc_forced = ccsr & (1 << CCCR_CPDIS_BIT);
    a = cccr & (1 << CCCR_A_BIT);
    if (osc_forced)
    return PXA_MEM_13Mhz;
    if (a)
    return PXA_MEM_SYSTEM_BUS;
    else
    return PXA_MEM_RUN;
    }
    PARENTS(clk_pxa27x_memory) = { "osc_13mhz", "system_bus", "run" };
    MUX_RO_RATE_RO_OPS(clk_pxa27x_memory, "memory");

    { .con_id = _con_id, .dev_id = _dev_id, .parent = _parent }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy_clk {
    pub con_id: *const c_char,
    pub dev_id: *const c_char,
    pub parent: *const c_char,
}

    static struct dummy_clk dummy_clks[] __initdata = {
    DUMMY_CLK(core::ptr::null_mut(), "pxa27x-gpio", "osc_32_768khz"),
    DUMMY_CLK(core::ptr::null_mut(), "pxa-rtc", "osc_32_768khz"),
    DUMMY_CLK(core::ptr::null_mut(), "sa1100-rtc", "osc_32_768khz"),
    DUMMY_CLK("UARTCLK", "pxa2xx-ir", "STUART"),
    };
#[no_mangle]
unsafe extern "C" fn pxa27x_dummy_clocks_init() -> void __init {
    static void __init pxa27x_dummy_clocks_init(void)
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
unsafe extern "C" fn pxa27x_base_clocks_init() -> void __init {
    static void __init pxa27x_base_clocks_init(void)
    {
    pxa27x_register_plls();
    pxa27x_register_core();
    clkdev_pxa_register(CLK_NONE, "system_bus", core::ptr::null_mut(),
    clk_register_clk_pxa27x_system_bus());
    clkdev_pxa_register(CLK_NONE, "memory", core::ptr::null_mut(),
    clk_register_clk_pxa27x_memory());
    clk_register_clk_pxa27x_lcd_base();
    }
#[no_mangle]
pub unsafe extern "C" fn pxa27x_clocks_init(regs: *mut void __iomem) -> int __init {
    int __init pxa27x_clocks_init(void __iomem *regs)
    {
    clk_regs = regs;
    pxa27x_base_clocks_init();
    pxa27x_dummy_clocks_init();
    return clk_pxa_cken_init(pxa27x_clocks, ARRAY_SIZE(pxa27x_clocks), regs);
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_dt_clocks_init(np: *mut device_node) -> void __init {
    static void __init pxa27x_dt_clocks_init(struct device_node *np)
    {
    pxa27x_clocks_init(ioremap(0x41300000ul, 0x10));
    clk_pxa_dt_common_init(np);
    }
    CLK_OF_DECLARE(pxa_clks, "marvell,pxa270-clocks", pxa27x_dt_clocks_init);
