//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-pxa1908-apbcp.c
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

pub const APBCP_UART2: c_uint = 0x1c;
pub const APBCP_TWSI2: c_uint = 0x28;
pub const APBCP_AICER: c_uint = 0x38;
pub const APBCP_NR_CLKS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa1908_clk_unit {
    pub unit: mmp_clk_unit,
    pub base: *mut void __iomem,
}

    static DEFINE_SPINLOCK(uart2_lock);
    static const char * const uart_parent_names[] = {"pll1_117", "uart_pll"};
    static struct mmp_param_gate_clk apbcp_gate_clks[] = {
    {PXA1908_CLK_UART2, "uart2_clk", "uart2_mux", CLK_SET_RATE_PARENT, APBCP_UART2, 0x3, 0x3, 0x0, 0, &uart2_lock},
    {PXA1908_CLK_TWSI2, "twsi2_clk", "pll1_32", CLK_SET_RATE_PARENT, APBCP_TWSI2, 0x3, 0x3, 0x0, 0, core::ptr::null_mut()},
    {PXA1908_CLK_AICER, "ripc_clk", core::ptr::null_mut(), 0, APBCP_AICER, 0x3, 0x2, 0x0, 0, core::ptr::null_mut()},
    };
    static struct mmp_param_mux_clk apbcp_mux_clks[] = {
    {0, "uart2_mux", uart_parent_names, ARRAY_SIZE(uart_parent_names), CLK_SET_RATE_PARENT, APBCP_UART2, 4, 3, 0, &uart2_lock},
    };
#[no_mangle]
unsafe extern "C" fn pxa1908_apb_p_periph_clk_init(pxa_unit: *mut pxa1908_clk_unit) {
    static void pxa1908_apb_p_periph_clk_init(struct pxa1908_clk_unit *pxa_unit)
    {
    struct mmp_clk_unit *unit = &pxa_unit.unit;
    mmp_register_mux_clks(unit, apbcp_mux_clks, pxa_unit.base,
    ARRAY_SIZE(apbcp_mux_clks));
    mmp_register_gate_clks(unit, apbcp_gate_clks, pxa_unit.base,
    ARRAY_SIZE(apbcp_gate_clks));
    }
// Taken from clk-of-pxa1928.c
    static void pxa1908_clk_reset_init(struct device_node *np,
    struct pxa1908_clk_unit *pxa_unit)
    {
    struct mmp_clk_reset_cell *cells;
    let mut nr_cells: c_int = ARRAY_SIZE(apbcp_gate_clks);
    cells = kzalloc_objs(*cells, nr_cells);
    if (!cells)
    return;
    for (int i = 0; i < nr_cells; i++) {
    cells[i].clk_id = apbcp_gate_clks[i].id;
    cells[i].reg = pxa_unit.base + apbcp_gate_clks[i].offset;
    cells[i].bits = BIT(2);
    cells[i].flags = 0;
    cells[i].lock = apbcp_gate_clks[i].lock;
    }
    mmp_clk_reset_register(np, cells, nr_cells);
    }
#[no_mangle]
unsafe extern "C" fn pxa1908_apbcp_probe(pdev: *mut platform_device) -> c_int {
    static int pxa1908_apbcp_probe(struct platform_device *pdev)
    {
    struct pxa1908_clk_unit *pxa_unit;
    pxa_unit = devm_kzalloc(&pdev.dev, sizeof(*pxa_unit), GFP_KERNEL);
    if (!pxa_unit)
    return -ENOMEM;
    pxa_unit.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pxa_unit.base))
    return PTR_ERR(pxa_unit.base);
    mmp_clk_init(pdev.dev.of_node, &pxa_unit.unit, APBCP_NR_CLKS);
    pxa1908_apb_p_periph_clk_init(pxa_unit);
    pxa1908_clk_reset_init(pdev.dev.of_node, pxa_unit);
    return 0;
    }
    static const struct of_device_id pxa1908_apbcp_match_table[] = {
    { .compatible = "marvell,pxa1908-apbcp" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pxa1908_apbcp_match_table);
    static struct platform_driver pxa1908_apbcp_driver = {
    .probe = pxa1908_apbcp_probe,
    .driver = {
    .name = "pxa1908-apbcp",
    .of_match_table = pxa1908_apbcp_match_table
    }
    };
    module_platform_driver(pxa1908_apbcp_driver);
    MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>");
    MODULE_DESCRIPTION("Marvell PXA1908 APBCP Clock Driver");
    MODULE_LICENSE("GPL");
