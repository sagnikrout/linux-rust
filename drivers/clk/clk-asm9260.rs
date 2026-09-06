//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-asm9260.c
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
// Copyright (c) 2014 Oleksij Rempel <linux@rempel-privat.de>.
//

pub const HW_AHBCLKCTRL0: c_uint = 0x0020;
pub const HW_AHBCLKCTRL1: c_uint = 0x0030;
pub const HW_SYSPLLCTRL: c_uint = 0x0100;
pub const HW_MAINCLKSEL: c_uint = 0x0120;
pub const HW_MAINCLKUEN: c_uint = 0x0124;
pub const HW_UARTCLKSEL: c_uint = 0x0128;
pub const HW_UARTCLKUEN: c_uint = 0x012c;
pub const HW_I2S0CLKSEL: c_uint = 0x0130;
pub const HW_I2S0CLKUEN: c_uint = 0x0134;
pub const HW_I2S1CLKSEL: c_uint = 0x0138;
pub const HW_I2S1CLKUEN: c_uint = 0x013c;
pub const HW_WDTCLKSEL: c_uint = 0x0160;
pub const HW_WDTCLKUEN: c_uint = 0x0164;
pub const HW_CLKOUTCLKSEL: c_uint = 0x0170;
pub const HW_CLKOUTCLKUEN: c_uint = 0x0174;
pub const HW_CPUCLKDIV: c_uint = 0x017c;
pub const HW_SYSAHBCLKDIV: c_uint = 0x0180;
pub const HW_I2S0MCLKDIV: c_uint = 0x0190;
pub const HW_I2S0SCLKDIV: c_uint = 0x0194;
pub const HW_I2S1MCLKDIV: c_uint = 0x0188;
pub const HW_I2S1SCLKDIV: c_uint = 0x018c;
pub const HW_UART0CLKDIV: c_uint = 0x0198;
pub const HW_UART1CLKDIV: c_uint = 0x019c;
pub const HW_UART2CLKDIV: c_uint = 0x01a0;
pub const HW_UART3CLKDIV: c_uint = 0x01a4;
pub const HW_UART4CLKDIV: c_uint = 0x01a8;
pub const HW_UART5CLKDIV: c_uint = 0x01ac;
pub const HW_UART6CLKDIV: c_uint = 0x01b0;
pub const HW_UART7CLKDIV: c_uint = 0x01b4;
pub const HW_UART8CLKDIV: c_uint = 0x01b8;
pub const HW_UART9CLKDIV: c_uint = 0x01bc;
pub const HW_SPI0CLKDIV: c_uint = 0x01c0;
pub const HW_SPI1CLKDIV: c_uint = 0x01c4;
pub const HW_QUADSPICLKDIV: c_uint = 0x01c8;
pub const HW_SSP0CLKDIV: c_uint = 0x01d0;
pub const HW_NANDCLKDIV: c_uint = 0x01d4;
pub const HW_TRACECLKDIV: c_uint = 0x01e0;
pub const HW_CAMMCLKDIV: c_uint = 0x01e8;
pub const HW_WDTCLKDIV: c_uint = 0x01ec;
pub const HW_CLKOUTCLKDIV: c_uint = 0x01f4;
pub const HW_MACCLKDIV: c_uint = 0x01f8;
pub const HW_LCDCLKDIV: c_uint = 0x01fc;
pub const HW_ADCANACLKDIV: c_uint = 0x0200;
    static struct clk_hw_onecell_data *clk_data;
    static DEFINE_SPINLOCK(asm9260_clk_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asm9260_div_clk {
    pub idx: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asm9260_gate_data {
    pub idx: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub reg: u32,
    pub bit_idx: u8,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asm9260_mux_clock {
    pub mask: u8,
    pub table: *mut u32,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub offset: c_ulong,
    pub flags: c_ulong,
}

    static void __iomem *base;
    static const struct asm9260_div_clk asm9260_div_clks[] __initconst = {
    { CLKID_SYS_CPU,	"cpu_div", "main_gate", HW_CPUCLKDIV },
    { CLKID_SYS_AHB,	"ahb_div", "cpu_div", HW_SYSAHBCLKDIV },
// i2s has two dividers: one for only external mclk and internal
// divider for all clks.
    { CLKID_SYS_I2S0M,	"i2s0m_div", "i2s0_mclk",  HW_I2S0MCLKDIV },
    { CLKID_SYS_I2S1M,	"i2s1m_div", "i2s1_mclk",  HW_I2S1MCLKDIV },
    { CLKID_SYS_I2S0S,	"i2s0s_div", "i2s0_gate",  HW_I2S0SCLKDIV },
    { CLKID_SYS_I2S1S,	"i2s1s_div", "i2s0_gate",  HW_I2S1SCLKDIV },
    { CLKID_SYS_UART0,	"uart0_div", "uart_gate", HW_UART0CLKDIV },
    { CLKID_SYS_UART1,	"uart1_div", "uart_gate", HW_UART1CLKDIV },
    { CLKID_SYS_UART2,	"uart2_div", "uart_gate", HW_UART2CLKDIV },
    { CLKID_SYS_UART3,	"uart3_div", "uart_gate", HW_UART3CLKDIV },
    { CLKID_SYS_UART4,	"uart4_div", "uart_gate", HW_UART4CLKDIV },
    { CLKID_SYS_UART5,	"uart5_div", "uart_gate", HW_UART5CLKDIV },
    { CLKID_SYS_UART6,	"uart6_div", "uart_gate", HW_UART6CLKDIV },
    { CLKID_SYS_UART7,	"uart7_div", "uart_gate", HW_UART7CLKDIV },
    { CLKID_SYS_UART8,	"uart8_div", "uart_gate", HW_UART8CLKDIV },
    { CLKID_SYS_UART9,	"uart9_div", "uart_gate", HW_UART9CLKDIV },
    { CLKID_SYS_SPI0,	"spi0_div",	"main_gate", HW_SPI0CLKDIV },
    { CLKID_SYS_SPI1,	"spi1_div",	"main_gate", HW_SPI1CLKDIV },
    { CLKID_SYS_QUADSPI,	"quadspi_div",	"main_gate", HW_QUADSPICLKDIV },
    { CLKID_SYS_SSP0,	"ssp0_div",	"main_gate", HW_SSP0CLKDIV },
    { CLKID_SYS_NAND,	"nand_div",	"main_gate", HW_NANDCLKDIV },
    { CLKID_SYS_TRACE,	"trace_div",	"main_gate", HW_TRACECLKDIV },
    { CLKID_SYS_CAMM,	"camm_div",	"main_gate", HW_CAMMCLKDIV },
    { CLKID_SYS_MAC,	"mac_div",	"main_gate", HW_MACCLKDIV },
    { CLKID_SYS_LCD,	"lcd_div",	"main_gate", HW_LCDCLKDIV },
    { CLKID_SYS_ADCANA,	"adcana_div",	"main_gate", HW_ADCANACLKDIV },
    { CLKID_SYS_WDT,	"wdt_div",	"wdt_gate",    HW_WDTCLKDIV },
    { CLKID_SYS_CLKOUT,	"clkout_div",	"clkout_gate", HW_CLKOUTCLKDIV },
    };
    static const struct asm9260_gate_data asm9260_mux_gates[] __initconst = {
    { 0, "main_gate",	"main_mux",	HW_MAINCLKUEN,	0 },
    { 0, "uart_gate",	"uart_mux",	HW_UARTCLKUEN,	0 },
    { 0, "i2s0_gate",	"i2s0_mux",	HW_I2S0CLKUEN,	0 },
    { 0, "i2s1_gate",	"i2s1_mux",	HW_I2S1CLKUEN,	0 },
    { 0, "wdt_gate",	"wdt_mux",	HW_WDTCLKUEN,	0 },
    { 0, "clkout_gate",	"clkout_mux",	HW_CLKOUTCLKUEN, 0 },
    };
    static const struct asm9260_gate_data asm9260_ahb_gates[] __initconst = {
// ahb gates
    { CLKID_AHB_ROM,	"rom",		"ahb_div",
    HW_AHBCLKCTRL0,	1, CLK_IGNORE_UNUSED},
    { CLKID_AHB_RAM,	"ram",		"ahb_div",
    HW_AHBCLKCTRL0,	2, CLK_IGNORE_UNUSED},
    { CLKID_AHB_GPIO,	"gpio",		"ahb_div",
    HW_AHBCLKCTRL0,	4 },
    { CLKID_AHB_MAC,	"mac",		"ahb_div",
    HW_AHBCLKCTRL0,	5 },
    { CLKID_AHB_EMI,	"emi",		"ahb_div",
    HW_AHBCLKCTRL0,	6, CLK_IGNORE_UNUSED},
    { CLKID_AHB_USB0,	"usb0",		"ahb_div",
    HW_AHBCLKCTRL0,	7 },
    { CLKID_AHB_USB1,	"usb1",		"ahb_div",
    HW_AHBCLKCTRL0,	8 },
    { CLKID_AHB_DMA0,	"dma0",		"ahb_div",
    HW_AHBCLKCTRL0,	9 },
    { CLKID_AHB_DMA1,	"dma1",		"ahb_div",
    HW_AHBCLKCTRL0,	10 },
    { CLKID_AHB_UART0,	"uart0",	"ahb_div",
    HW_AHBCLKCTRL0,	11 },
    { CLKID_AHB_UART1,	"uart1",	"ahb_div",
    HW_AHBCLKCTRL0,	12 },
    { CLKID_AHB_UART2,	"uart2",	"ahb_div",
    HW_AHBCLKCTRL0,	13 },
    { CLKID_AHB_UART3,	"uart3",	"ahb_div",
    HW_AHBCLKCTRL0,	14 },
    { CLKID_AHB_UART4,	"uart4",	"ahb_div",
    HW_AHBCLKCTRL0,	15 },
    { CLKID_AHB_UART5,	"uart5",	"ahb_div",
    HW_AHBCLKCTRL0,	16 },
    { CLKID_AHB_UART6,	"uart6",	"ahb_div",
    HW_AHBCLKCTRL0,	17 },
    { CLKID_AHB_UART7,	"uart7",	"ahb_div",
    HW_AHBCLKCTRL0,	18 },
    { CLKID_AHB_UART8,	"uart8",	"ahb_div",
    HW_AHBCLKCTRL0,	19 },
    { CLKID_AHB_UART9,	"uart9",	"ahb_div",
    HW_AHBCLKCTRL0,	20 },
    { CLKID_AHB_I2S0,	"i2s0",		"ahb_div",
    HW_AHBCLKCTRL0,	21 },
    { CLKID_AHB_I2C0,	"i2c0",		"ahb_div",
    HW_AHBCLKCTRL0,	22 },
    { CLKID_AHB_I2C1,	"i2c1",		"ahb_div",
    HW_AHBCLKCTRL0,	23 },
    { CLKID_AHB_SSP0,	"ssp0",		"ahb_div",
    HW_AHBCLKCTRL0,	24 },
    { CLKID_AHB_IOCONFIG,	"ioconf",	"ahb_div",
    HW_AHBCLKCTRL0,	25 },
    { CLKID_AHB_WDT,	"wdt",		"ahb_div",
    HW_AHBCLKCTRL0,	26 },
    { CLKID_AHB_CAN0,	"can0",		"ahb_div",
    HW_AHBCLKCTRL0,	27 },
    { CLKID_AHB_CAN1,	"can1",		"ahb_div",
    HW_AHBCLKCTRL0,	28 },
    { CLKID_AHB_MPWM,	"mpwm",		"ahb_div",
    HW_AHBCLKCTRL0,	29 },
    { CLKID_AHB_SPI0,	"spi0",		"ahb_div",
    HW_AHBCLKCTRL0,	30 },
    { CLKID_AHB_SPI1,	"spi1",		"ahb_div",
    HW_AHBCLKCTRL0,	31 },
    { CLKID_AHB_QEI,	"qei",		"ahb_div",
    HW_AHBCLKCTRL1,	0 },
    { CLKID_AHB_QUADSPI0,	"quadspi0",	"ahb_div",
    HW_AHBCLKCTRL1,	1 },
    { CLKID_AHB_CAMIF,	"capmif",	"ahb_div",
    HW_AHBCLKCTRL1,	2 },
    { CLKID_AHB_LCDIF,	"lcdif",	"ahb_div",
    HW_AHBCLKCTRL1,	3 },
    { CLKID_AHB_TIMER0,	"timer0",	"ahb_div",
    HW_AHBCLKCTRL1,	4 },
    { CLKID_AHB_TIMER1,	"timer1",	"ahb_div",
    HW_AHBCLKCTRL1,	5 },
    { CLKID_AHB_TIMER2,	"timer2",	"ahb_div",
    HW_AHBCLKCTRL1,	6 },
    { CLKID_AHB_TIMER3,	"timer3",	"ahb_div",
    HW_AHBCLKCTRL1,	7 },
    { CLKID_AHB_IRQ,	"irq",		"ahb_div",
    HW_AHBCLKCTRL1,	8, CLK_IGNORE_UNUSED},
    { CLKID_AHB_RTC,	"rtc",		"ahb_div",
    HW_AHBCLKCTRL1,	9 },
    { CLKID_AHB_NAND,	"nand",		"ahb_div",
    HW_AHBCLKCTRL1,	10 },
    { CLKID_AHB_ADC0,	"adc0",		"ahb_div",
    HW_AHBCLKCTRL1,	11 },
    { CLKID_AHB_LED,	"led",		"ahb_div",
    HW_AHBCLKCTRL1,	12 },
    { CLKID_AHB_DAC0,	"dac0",		"ahb_div",
    HW_AHBCLKCTRL1,	13 },
    { CLKID_AHB_LCD,	"lcd",		"ahb_div",
    HW_AHBCLKCTRL1,	14 },
    { CLKID_AHB_I2S1,	"i2s1",		"ahb_div",
    HW_AHBCLKCTRL1,	15 },
    { CLKID_AHB_MAC1,	"mac1",		"ahb_div",
    HW_AHBCLKCTRL1,	16 },
    };
    static struct clk_parent_data __initdata main_mux_p[] =   { { .index = 0, }, { .name = "pll" } };
    static struct clk_parent_data __initdata i2s0_mux_p[] =   { { .index = 0, }, { .name = "pll" }, { .name = "i2s0m_div"} };
    static struct clk_parent_data __initdata i2s1_mux_p[] =   { { .index = 0, }, { .name = "pll" }, { .name = "i2s1m_div"} };
    static struct clk_parent_data __initdata clkout_mux_p[] = { { .index = 0, }, { .name = "pll" }, { .name = "rtc"} };
    static u32 three_mux_table[] = {0, 1, 3};
    static struct asm9260_mux_clock asm9260_mux_clks[] __initdata = {
    { 1, three_mux_table, "main_mux",	main_mux_p,
    ARRAY_SIZE(main_mux_p), HW_MAINCLKSEL, },
    { 1, three_mux_table, "uart_mux",	main_mux_p,
    ARRAY_SIZE(main_mux_p), HW_UARTCLKSEL, },
    { 1, three_mux_table, "wdt_mux",	main_mux_p,
    ARRAY_SIZE(main_mux_p), HW_WDTCLKSEL, },
    { 3, three_mux_table, "i2s0_mux",	i2s0_mux_p,
    ARRAY_SIZE(i2s0_mux_p), HW_I2S0CLKSEL, },
    { 3, three_mux_table, "i2s1_mux",	i2s1_mux_p,
    ARRAY_SIZE(i2s1_mux_p), HW_I2S1CLKSEL, },
    { 3, three_mux_table, "clkout_mux",	clkout_mux_p,
    ARRAY_SIZE(clkout_mux_p), HW_CLKOUTCLKSEL, },
    };
#[no_mangle]
unsafe extern "C" fn asm9260_acc_init(np: *mut device_node) -> void __init {
    static void __init asm9260_acc_init(struct device_node *np)
    {
    struct clk_hw *pll_hw;
    struct clk_hw **hws;
    const char *pll_clk = "pll";
    let mut pll_parent_data: clk_parent_data = { .index = 0 };
    u32 rate;
    int n;
    clk_data = kzalloc_flex(*clk_data, hws, MAX_CLKS);
    if (!clk_data)
    return;
    clk_data.num = MAX_CLKS;
    hws = clk_data.hws;
    base = of_io_request_and_map(np, 0, np.name);
    if (IS_ERR(base))
    panic("%pOFn: unable to map resource", np);
// register pll
    rate = (ioread32(base + HW_SYSPLLCTRL) & 0xffff) * 1000000;
    pll_hw = clk_hw_register_fixed_rate_parent_accuracy(core::ptr::null_mut(), pll_clk, &pll_parent_data,
    0, rate);
    if (IS_ERR(pll_hw))
    panic("%pOFn: can't register REFCLK. Check DT!", np);
    for (n = 0; n < ARRAY_SIZE(asm9260_mux_clks); n++) {
    const struct asm9260_mux_clock *mc = &asm9260_mux_clks[n];
    clk_hw_register_mux_table_parent_data(core::ptr::null_mut(), mc.name, mc.parent_data,
    mc.num_parents, mc.flags, base + mc.offset,
    0, mc.mask, 0, mc.table, &asm9260_clk_lock);
    }
// clock mux gate cells
    for (n = 0; n < ARRAY_SIZE(asm9260_mux_gates); n++) {
    const struct asm9260_gate_data *gd = &asm9260_mux_gates[n];
    clk_hw_register_gate(core::ptr::null_mut(), gd.name,
    gd.parent_name, gd.flags | CLK_SET_RATE_PARENT,
    base + gd.reg, gd.bit_idx, 0, &asm9260_clk_lock);
    }
// clock div cells
    for (n = 0; n < ARRAY_SIZE(asm9260_div_clks); n++) {
    const struct asm9260_div_clk *dc = &asm9260_div_clks[n];
    hws[dc.idx] = clk_hw_register_divider(core::ptr::null_mut(), dc.name,
    dc.parent_name, CLK_SET_RATE_PARENT,
    base + dc.reg, 0, 8, CLK_DIVIDER_ONE_BASED,
    &asm9260_clk_lock);
    }
// clock ahb gate cells
    for (n = 0; n < ARRAY_SIZE(asm9260_ahb_gates); n++) {
    const struct asm9260_gate_data *gd = &asm9260_ahb_gates[n];
    hws[gd.idx] = clk_hw_register_gate(core::ptr::null_mut(), gd.name,
    gd.parent_name, gd.flags, base + gd.reg,
    gd.bit_idx, 0, &asm9260_clk_lock);
    }
// check for errors on leaf clocks
    for (n = 0; n < MAX_CLKS; n++) {
    if (!IS_ERR(hws[n]))
    continue;
    pr_err("%pOF: Unable to register leaf clock %d\n",
    np, n);
    goto fail;
    }
// register clk-provider
    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, clk_data);
    return;
    fail:
    iounmap(base);
    }
    CLK_OF_DECLARE(asm9260_acc, "alphascale,asm9260-clock-controller",
    asm9260_acc_init);
