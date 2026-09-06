//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-imx1.c
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
// Copyright (C) 2008 Sascha Hauer <s.hauer@pengutronix.de>, Pengutronix
//

pub const MX1_CCM_BASE_ADDR: c_uint = 0x0021b000;
pub const MX1_TIM1_BASE_ADDR: c_uint = 0x00220000;

    static const char *prem_sel_clks[] = { "clk32_premult", "clk16m", };
    static const char *clko_sel_clks[] = { "per1", "hclk", "clk48m", "clk16m",
    "prem", "fclk", };
    static struct clk *clk[IMX1_CLK_MAX];
    static struct clk_onecell_data clk_data;
    static void __iomem *ccm __initdata;

#[no_mangle]
unsafe extern "C" fn mx1_clocks_init_dt(np: *mut device_node) -> void __init {
    static void __init mx1_clocks_init_dt(struct device_node *np)
    {
    ccm = of_iomap(np, 0);
    BUG_ON(!ccm);
    clk[IMX1_CLK_DUMMY] = imx_clk_fixed("dummy", 0);
    clk[IMX1_CLK_CLK32] = imx_obtain_fixed_clock("clk32", 32768);
    clk[IMX1_CLK_CLK16M_EXT] = imx_clk_fixed("clk16m_ext", 16000000);
    clk[IMX1_CLK_CLK16M] = imx_clk_gate("clk16m", "clk16m_ext", CCM_CSCR, 17);
    clk[IMX1_CLK_CLK32_PREMULT] = imx_clk_fixed_factor("clk32_premult", "clk32", 512, 1);
    clk[IMX1_CLK_PREM] = imx_clk_mux("prem", CCM_CSCR, 16, 1, prem_sel_clks, ARRAY_SIZE(prem_sel_clks));
    clk[IMX1_CLK_MPLL] = imx_clk_pllv1(IMX_PLLV1_IMX1, "mpll", "clk32_premult", CCM_MPCTL0);
    clk[IMX1_CLK_MPLL_GATE] = imx_clk_gate("mpll_gate", "mpll", CCM_CSCR, 0);
    clk[IMX1_CLK_SPLL] = imx_clk_pllv1(IMX_PLLV1_IMX1, "spll", "prem", CCM_SPCTL0);
    clk[IMX1_CLK_SPLL_GATE] = imx_clk_gate("spll_gate", "spll", CCM_CSCR, 1);
    clk[IMX1_CLK_MCU] = imx_clk_divider("mcu", "clk32_premult", CCM_CSCR, 15, 1);
    clk[IMX1_CLK_FCLK] = imx_clk_divider("fclk", "mpll_gate", CCM_CSCR, 15, 1);
    clk[IMX1_CLK_HCLK] = imx_clk_divider("hclk", "spll_gate", CCM_CSCR, 10, 4);
    clk[IMX1_CLK_CLK48M] = imx_clk_divider("clk48m", "spll_gate", CCM_CSCR, 26, 3);
    clk[IMX1_CLK_PER1] = imx_clk_divider("per1", "spll_gate", CCM_PCDR, 0, 4);
    clk[IMX1_CLK_PER2] = imx_clk_divider("per2", "spll_gate", CCM_PCDR, 4, 4);
    clk[IMX1_CLK_PER3] = imx_clk_divider("per3", "spll_gate", CCM_PCDR, 16, 7);
    clk[IMX1_CLK_CLKO] = imx_clk_mux("clko", CCM_CSCR, 29, 3, clko_sel_clks, ARRAY_SIZE(clko_sel_clks));
    clk[IMX1_CLK_UART3_GATE] = imx_clk_gate("uart3_gate", "hclk", SCM_GCCR, 6);
    clk[IMX1_CLK_SSI2_GATE] = imx_clk_gate("ssi2_gate", "hclk", SCM_GCCR, 5);
    clk[IMX1_CLK_BROM_GATE] = imx_clk_gate("brom_gate", "hclk", SCM_GCCR, 4);
    clk[IMX1_CLK_DMA_GATE] = imx_clk_gate("dma_gate", "hclk", SCM_GCCR, 3);
    clk[IMX1_CLK_CSI_GATE] = imx_clk_gate("csi_gate", "hclk", SCM_GCCR, 2);
    clk[IMX1_CLK_MMA_GATE] = imx_clk_gate("mma_gate", "hclk", SCM_GCCR, 1);
    clk[IMX1_CLK_USBD_GATE] = imx_clk_gate("usbd_gate", "clk48m", SCM_GCCR, 0);
    imx_check_clocks(clk, ARRAY_SIZE(clk));
    clk_data.clks = clk;
    clk_data.clk_num = ARRAY_SIZE(clk);
    of_clk_add_provider(np, of_clk_src_onecell_get, &clk_data);
    }
    CLK_OF_DECLARE(imx1_ccm, "fsl,imx1-ccm", mx1_clocks_init_dt);
