//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mxs/clk-imx23.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2012 Freescale Semiconductor, Inc.
//

    static void __iomem *clkctrl;
    static void __iomem *digctrl;

pub const BP_CPU_INTERRUPT_WAIT: c_int = 12;
pub const BP_CLKSEQ_BYPASS_SAIF: c_int = 0;
pub const BP_CLKSEQ_BYPASS_SSP: c_int = 5;
pub const BP_SAIF_DIV_FRAC_EN: c_int = 16;
pub const BP_FRAC_IOFRAC: c_int = 24;
#[no_mangle]
unsafe extern "C" fn clk_misc_init() -> void __init {
    static void __init clk_misc_init(void)
    {
    u32 val;
// Gate off cpu clock in WFI for power saving
    writel_relaxed(1 << BP_CPU_INTERRUPT_WAIT, CPU + SET);
// Clear BYPASS for SAIF
    writel_relaxed(1 << BP_CLKSEQ_BYPASS_SAIF, CLKSEQ + CLR);
// SAIF has to use frac div for functional operation
    val = readl_relaxed(SAIF);
    val |= 1 << BP_SAIF_DIV_FRAC_EN;
    writel_relaxed(val, SAIF);
//
// Source ssp clock from ref_io than ref_xtal,
// as ref_xtal only provides 24 MHz as maximum.
//
    writel_relaxed(1 << BP_CLKSEQ_BYPASS_SSP, CLKSEQ + CLR);
//
// 480 MHz seems too high to be ssp clock source directly,
// so set frac to get a 288 MHz ref_io.
//
    writel_relaxed(0x3f << BP_FRAC_IOFRAC, FRAC + CLR);
    writel_relaxed(30 << BP_FRAC_IOFRAC, FRAC + SET);
    }
    static const char *const sel_pll[]  __initconst = { "pll", "ref_xtal", };
    static const char *const sel_cpu[]  __initconst = { "ref_cpu", "ref_xtal", };
    static const char *const sel_pix[]  __initconst = { "ref_pix", "ref_xtal", };
    static const char *const sel_io[]   __initconst = { "ref_io", "ref_xtal", };
    static const char *const cpu_sels[] __initconst = { "cpu_pll", "cpu_xtal", };
    static const char *const emi_sels[] __initconst = { "emi_pll", "emi_xtal", };
    enum imx23_clk {
    ref_xtal, pll, ref_cpu, ref_emi, ref_pix, ref_io, saif_sel,
    lcdif_sel, gpmi_sel, ssp_sel, emi_sel, cpu, etm_sel, cpu_pll,
    cpu_xtal, hbus, xbus, lcdif_div, ssp_div, gpmi_div, emi_pll,
    emi_xtal, etm_div, saif_div, clk32k_div, rtc, adc, spdif_div,
    clk32k, dri, pwm, filt, uart, ssp, gpmi, spdif, emi, saif,
    lcdif, etm, usb, usb_phy,
    clk_max
    };
    static struct clk *clks[clk_max];
    static struct clk_onecell_data clk_data;
    static enum imx23_clk clks_init_on[] __initdata = {
    cpu, hbus, xbus, emi, uart,
    };
#[no_mangle]
unsafe extern "C" fn mx23_clocks_init(np: *mut device_node) -> void __init {
    static void __init mx23_clocks_init(struct device_node *np)
    {
    struct device_node *dcnp;
    u32 i;
    dcnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx23-digctl");
    digctrl = of_iomap(dcnp, 0);
    WARN_ON(!digctrl);
    of_node_put(dcnp);
    clkctrl = of_iomap(np, 0);
    WARN_ON(!clkctrl);
    clk_misc_init();
    clks[ref_xtal] = mxs_clk_fixed("ref_xtal", 24000000);
    clks[pll] = mxs_clk_pll("pll", "ref_xtal", PLLCTRL0, 16, 480000000);
    clks[ref_cpu] = mxs_clk_ref("ref_cpu", "pll", FRAC, 0);
    clks[ref_emi] = mxs_clk_ref("ref_emi", "pll", FRAC, 1);
    clks[ref_pix] = mxs_clk_ref("ref_pix", "pll", FRAC, 2);
    clks[ref_io] = mxs_clk_ref("ref_io", "pll", FRAC, 3);
    clks[saif_sel] = mxs_clk_mux("saif_sel", CLKSEQ, 0, 1, sel_pll, ARRAY_SIZE(sel_pll));
    clks[lcdif_sel] = mxs_clk_mux("lcdif_sel", CLKSEQ, 1, 1, sel_pix, ARRAY_SIZE(sel_pix));
    clks[gpmi_sel] = mxs_clk_mux("gpmi_sel", CLKSEQ, 4, 1, sel_io, ARRAY_SIZE(sel_io));
    clks[ssp_sel] = mxs_clk_mux("ssp_sel", CLKSEQ, 5, 1, sel_io, ARRAY_SIZE(sel_io));
    clks[emi_sel] = mxs_clk_mux("emi_sel", CLKSEQ, 6, 1, emi_sels, ARRAY_SIZE(emi_sels));
    clks[cpu] = mxs_clk_mux("cpu", CLKSEQ, 7, 1, cpu_sels, ARRAY_SIZE(cpu_sels));
    clks[etm_sel] = mxs_clk_mux("etm_sel", CLKSEQ, 8, 1, sel_cpu, ARRAY_SIZE(sel_cpu));
    clks[cpu_pll] = mxs_clk_div("cpu_pll", "ref_cpu", CPU, 0, 6, 28);
    clks[cpu_xtal] = mxs_clk_div("cpu_xtal", "ref_xtal", CPU, 16, 10, 29);
    clks[hbus] = mxs_clk_div("hbus", "cpu", HBUS, 0, 5, 29);
    clks[xbus] = mxs_clk_div("xbus", "ref_xtal", XBUS, 0, 10, 31);
    clks[lcdif_div] = mxs_clk_div("lcdif_div", "lcdif_sel", PIX, 0, 12, 29);
    clks[ssp_div] = mxs_clk_div("ssp_div", "ssp_sel", SSP, 0, 9, 29);
    clks[gpmi_div] = mxs_clk_div("gpmi_div", "gpmi_sel", GPMI, 0, 10, 29);
    clks[emi_pll] = mxs_clk_div("emi_pll", "ref_emi", EMI, 0, 6, 28);
    clks[emi_xtal] = mxs_clk_div("emi_xtal", "ref_xtal", EMI, 8, 4, 29);
    clks[etm_div] = mxs_clk_div("etm_div", "etm_sel", ETM, 0, 6, 29);
    clks[saif_div] = mxs_clk_frac("saif_div", "saif_sel", SAIF, 0, 16, 29);
    clks[clk32k_div] = mxs_clk_fixed_factor("clk32k_div", "ref_xtal", 1, 750);
    clks[rtc] = mxs_clk_fixed_factor("rtc", "ref_xtal", 1, 768);
    clks[adc] = mxs_clk_fixed_factor("adc", "clk32k", 1, 16);
    clks[spdif_div] = mxs_clk_fixed_factor("spdif_div", "pll", 1, 4);
    clks[clk32k] = mxs_clk_gate("clk32k", "clk32k_div", XTAL, 26);
    clks[dri] = mxs_clk_gate("dri", "ref_xtal", XTAL, 28);
    clks[pwm] = mxs_clk_gate("pwm", "ref_xtal", XTAL, 29);
    clks[filt] = mxs_clk_gate("filt", "ref_xtal", XTAL, 30);
    clks[uart] = mxs_clk_gate("uart", "ref_xtal", XTAL, 31);
    clks[ssp] = mxs_clk_gate("ssp", "ssp_div", SSP, 31);
    clks[gpmi] = mxs_clk_gate("gpmi", "gpmi_div", GPMI, 31);
    clks[spdif] = mxs_clk_gate("spdif", "spdif_div", SPDIF, 31);
    clks[emi] = mxs_clk_gate("emi", "emi_sel", EMI, 31);
    clks[saif] = mxs_clk_gate("saif", "saif_div", SAIF, 31);
    clks[lcdif] = mxs_clk_gate("lcdif", "lcdif_div", PIX, 31);
    clks[etm] = mxs_clk_gate("etm", "etm_div", ETM, 31);
    clks[usb] = mxs_clk_gate("usb", "usb_phy", DIGCTRL, 2);
    clks[usb_phy] = clk_register_gate(core::ptr::null_mut(), "usb_phy", "pll", 0, PLLCTRL0, 18, 0, &mxs_lock);
    for (i = 0; i < ARRAY_SIZE(clks); i++)
    if (IS_ERR(clks[i])) {
    pr_err("i.MX23 clk %d: register failed with %ld\n",
    i, PTR_ERR(clks[i]));
    return;
    }
    clk_data.clks = clks;
    clk_data.clk_num = ARRAY_SIZE(clks);
    of_clk_add_provider(np, of_clk_src_onecell_get, &clk_data);
    for (i = 0; i < ARRAY_SIZE(clks_init_on); i++)
    clk_prepare_enable(clks[clks_init_on[i]]);
    }
    CLK_OF_DECLARE(imx23_clkctrl, "fsl,imx23-clkctrl", mx23_clocks_init);
