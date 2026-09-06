//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-pxa1908-apbc.c
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

pub const APBC_UART0: c_uint = 0x0;
pub const APBC_UART1: c_uint = 0x4;
pub const APBC_GPIO: c_uint = 0x8;
pub const APBC_PWM0: c_uint = 0xc;
pub const APBC_PWM1: c_uint = 0x10;
pub const APBC_PWM2: c_uint = 0x14;
pub const APBC_PWM3: c_uint = 0x18;
pub const APBC_SSP0: c_uint = 0x1c;
pub const APBC_SSP1: c_uint = 0x20;
pub const APBC_IPC_RST: c_uint = 0x24;
pub const APBC_RTC: c_uint = 0x28;
pub const APBC_TWSI0: c_uint = 0x2c;
pub const APBC_KPC: c_uint = 0x30;
pub const APBC_SWJTAG: c_uint = 0x40;
pub const APBC_SSP2: c_uint = 0x4c;
pub const APBC_TWSI1: c_uint = 0x60;
pub const APBC_THERMAL: c_uint = 0x6c;
pub const APBC_TWSI3: c_uint = 0x70;
pub const APBC_NR_CLKS: c_int = 19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa1908_clk_unit {
    pub unit: mmp_clk_unit,
    pub base: *mut void __iomem,
}

    static DEFINE_SPINLOCK(pwm0_lock);
    static DEFINE_SPINLOCK(pwm2_lock);
    static DEFINE_SPINLOCK(uart0_lock);
    static DEFINE_SPINLOCK(uart1_lock);
    static const char * const uart_parent_names[] = {"pll1_117", "uart_pll"};
    static const char * const ssp_parent_names[] = {"pll1_d16", "pll1_d48", "pll1_d24", "pll1_d12"};
    static struct mmp_param_gate_clk apbc_gate_clks[] = {
    {PXA1908_CLK_TWSI0, "twsi0_clk", "pll1_32", CLK_SET_RATE_PARENT, APBC_TWSI0, 0x3, 3, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_TWSI1, "twsi1_clk", "pll1_32", CLK_SET_RATE_PARENT, APBC_TWSI1, 0x3, 3, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_TWSI3, "twsi3_clk", "pll1_32", CLK_SET_RATE_PARENT, APBC_TWSI3, 0x3, 3, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_GPIO, "gpio_clk", "vctcxo", CLK_SET_RATE_PARENT, APBC_GPIO, 0x3, 3, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_KPC, "kpc_clk", "clk32", CLK_SET_RATE_PARENT, APBC_KPC, 0x3, 3, 0, MMP_CLK_GATE_NEED_DELAY, core::ptr::null_mut()},
    {PXA1908_CLK_RTC, "rtc_clk", "clk32", CLK_SET_RATE_PARENT, APBC_RTC, 0x83, 0x83, 0, MMP_CLK_GATE_NEED_DELAY, core::ptr::null_mut()},
    {PXA1908_CLK_PWM1, "pwm1_clk", "pwm01_apb_share", CLK_SET_RATE_PARENT, APBC_PWM1, 0x2, 2, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_PWM3, "pwm3_clk", "pwm23_apb_share", CLK_SET_RATE_PARENT, APBC_PWM3, 0x2, 2, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_UART0, "uart0_clk", "uart0_mux", CLK_SET_RATE_PARENT, APBC_UART0, 0x3, 3, 0, 0, &uart0_lock},
    {PXA1908_CLK_UART1, "uart1_clk", "uart1_mux", CLK_SET_RATE_PARENT, APBC_UART1, 0x3, 3, 0, 0, &uart1_lock},
    {PXA1908_CLK_THERMAL, "thermal_clk", core::ptr::null_mut(), 0, APBC_THERMAL, 0x3, 3, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_IPC_RST, "ipc_clk", core::ptr::null_mut(), 0, APBC_IPC_RST, 0x3, 3, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_SSP0, "ssp0_clk", "ssp0_mux", 0, APBC_SSP0, 0x3, 3, 0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_SSP2, "ssp2_clk", "ssp2_mux", 0, APBC_SSP2, 0x3, 3, 0, 0, core::ptr::null_mut()},
    };
    static struct mmp_param_gate_clk apbc_gate_no_reset_clks[] = {
    {PXA1908_CLK_PWM0, "pwm0_clk", "pwm01_apb_share", CLK_SET_RATE_PARENT, APBC_PWM0, 0x2, 2, 0, 0, &pwm0_lock},
    {PXA1908_CLK_PWM2, "pwm2_clk", "pwm23_apb_share", CLK_SET_RATE_PARENT, APBC_PWM2, 0x2, 2, 0, 0, core::ptr::null_mut()},
    };
    static struct mmp_param_mux_clk apbc_mux_clks[] = {
    {0, "uart0_mux", uart_parent_names, ARRAY_SIZE(uart_parent_names), CLK_SET_RATE_PARENT, APBC_UART0, 4, 3, 0, &uart0_lock},
    {0, "uart1_mux", uart_parent_names, ARRAY_SIZE(uart_parent_names), CLK_SET_RATE_PARENT, APBC_UART1, 4, 3, 0, &uart1_lock},
    {0, "ssp0_mux", ssp_parent_names, ARRAY_SIZE(ssp_parent_names), 0, APBC_SSP0, 4, 3, 0, core::ptr::null_mut()},
    {0, "ssp2_mux", ssp_parent_names, ARRAY_SIZE(ssp_parent_names), 0, APBC_SSP2, 4, 3, 0, core::ptr::null_mut()},
    };
#[no_mangle]
unsafe extern "C" fn pxa1908_apb_periph_clk_init(pxa_unit: *mut pxa1908_clk_unit) {
    static void pxa1908_apb_periph_clk_init(struct pxa1908_clk_unit *pxa_unit)
    {
    struct mmp_clk_unit *unit = &pxa_unit.unit;
    struct clk *clk;
    mmp_clk_register_gate(core::ptr::null_mut(), "pwm01_apb_share", "pll1_d48",
    CLK_SET_RATE_PARENT,
    pxa_unit.base + APBC_PWM0,
    0x5, 1, 0, 0, &pwm0_lock);
    mmp_clk_register_gate(core::ptr::null_mut(), "pwm23_apb_share", "pll1_d48",
    CLK_SET_RATE_PARENT,
    pxa_unit.base + APBC_PWM2,
    0x5, 1, 0, 0, &pwm2_lock);
    clk = mmp_clk_register_apbc("swjtag", core::ptr::null_mut(),
    pxa_unit.base + APBC_SWJTAG, 10, 0, core::ptr::null_mut());
    mmp_clk_add(unit, PXA1908_CLK_SWJTAG, clk);
    mmp_register_mux_clks(unit, apbc_mux_clks, pxa_unit.base,
    ARRAY_SIZE(apbc_mux_clks));
    mmp_register_gate_clks(unit, apbc_gate_clks, pxa_unit.base,
    ARRAY_SIZE(apbc_gate_clks));
    mmp_register_gate_clks(unit, apbc_gate_no_reset_clks, pxa_unit.base,
    ARRAY_SIZE(apbc_gate_no_reset_clks));
    }
// Taken from clk-of-pxa1928.c
    static void pxa1908_clk_reset_init(struct device_node *np,
    struct pxa1908_clk_unit *pxa_unit)
    {
    struct mmp_clk_reset_cell *cells;
    let mut nr_cells: c_int = ARRAY_SIZE(apbc_gate_clks);
    cells = kzalloc_objs(*cells, nr_cells);
    if (!cells)
    return;
    for (int i = 0; i < nr_cells; i++) {
    cells[i].clk_id = apbc_gate_clks[i].id;
    cells[i].reg = pxa_unit.base + apbc_gate_clks[i].offset;
    cells[i].bits = BIT(2);
    cells[i].flags = 0;
    cells[i].lock = apbc_gate_clks[i].lock;
    }
    mmp_clk_reset_register(np, cells, nr_cells);
    }
#[no_mangle]
unsafe extern "C" fn pxa1908_apbc_probe(pdev: *mut platform_device) -> c_int {
    static int pxa1908_apbc_probe(struct platform_device *pdev)
    {
    struct pxa1908_clk_unit *pxa_unit;
    pxa_unit = devm_kzalloc(&pdev.dev, sizeof(*pxa_unit), GFP_KERNEL);
    if (!pxa_unit)
    return -ENOMEM;
    pxa_unit.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pxa_unit.base))
    return PTR_ERR(pxa_unit.base);
    mmp_clk_init(pdev.dev.of_node, &pxa_unit.unit, APBC_NR_CLKS);
    pxa1908_apb_periph_clk_init(pxa_unit);
    pxa1908_clk_reset_init(pdev.dev.of_node, pxa_unit);
    return 0;
    }
    static const struct of_device_id pxa1908_apbc_match_table[] = {
    { .compatible = "marvell,pxa1908-apbc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pxa1908_apbc_match_table);
    static struct platform_driver pxa1908_apbc_driver = {
    .probe = pxa1908_apbc_probe,
    .driver = {
    .name = "pxa1908-apbc",
    .of_match_table = pxa1908_apbc_match_table
    }
    };
    module_platform_driver(pxa1908_apbc_driver);
    MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>");
    MODULE_DESCRIPTION("Marvell PXA1908 APBC Clock Driver");
    MODULE_LICENSE("GPL");
