//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-nsp.c
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
// Copyright (C) 2015 Broadcom Corporation

    .pwr_shift = ps, .iso_shift = is }

    .p_reset_shift = prs }

    .ki_width = kiw, .kp_shift = kps, .kp_width = kpw, .ka_shift = kas,    \
    .ka_width = kaw }

    .hold_shift = hs, .bypass_shift = bs }
#[no_mangle]
unsafe extern "C" fn nsp_armpll_init(node: *mut device_node) -> void __init {
    static void __init nsp_armpll_init(struct device_node *node)
    {
    iproc_armpll_setup(node);
    }
    CLK_OF_DECLARE(nsp_armpll, "brcm,nsp-armpll", nsp_armpll_init);
    static const struct iproc_pll_ctrl genpll = {
    .flags = IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_EMBED_PWRCTRL,
    .aon = AON_VAL(0x0, 1, 12, 0),
    .reset = RESET_VAL(0x0, 11, 10),
    .dig_filter = DF_VAL(0x0, 4, 3, 0, 4, 7, 3),
    .ndiv_int = REG_VAL(0x14, 20, 10),
    .ndiv_frac = REG_VAL(0x14, 0, 20),
    .pdiv = REG_VAL(0x18, 24, 3),
    .status = REG_VAL(0x20, 12, 1),
    };
    static const struct iproc_clk_ctrl genpll_clk[] = {
    [BCM_NSP_GENPLL_PHY_CLK] = {
    .channel = BCM_NSP_GENPLL_PHY_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 12, 6, 18),
    .mdiv = REG_VAL(0x18, 16, 8),
    },
    [BCM_NSP_GENPLL_ENET_SW_CLK] = {
    .channel = BCM_NSP_GENPLL_ENET_SW_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 13, 7, 19),
    .mdiv = REG_VAL(0x18, 8, 8),
    },
    [BCM_NSP_GENPLL_USB_PHY_REF_CLK] = {
    .channel = BCM_NSP_GENPLL_USB_PHY_REF_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 14, 8, 20),
    .mdiv = REG_VAL(0x18, 0, 8),
    },
    [BCM_NSP_GENPLL_IPROCFAST_CLK] = {
    .channel = BCM_NSP_GENPLL_IPROCFAST_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 15, 9, 21),
    .mdiv = REG_VAL(0x1c, 16, 8),
    },
    [BCM_NSP_GENPLL_SATA1_CLK] = {
    .channel = BCM_NSP_GENPLL_SATA1_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 16, 10, 22),
    .mdiv = REG_VAL(0x1c, 8, 8),
    },
    [BCM_NSP_GENPLL_SATA2_CLK] = {
    .channel = BCM_NSP_GENPLL_SATA2_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 17, 11, 23),
    .mdiv = REG_VAL(0x1c, 0, 8),
    },
    };
#[no_mangle]
unsafe extern "C" fn nsp_genpll_clk_init(node: *mut device_node) -> void __init {
    static void __init nsp_genpll_clk_init(struct device_node *node)
    {
    iproc_pll_clk_setup(node, &genpll, core::ptr::null_mut(), 0, genpll_clk,
    ARRAY_SIZE(genpll_clk));
    }
    CLK_OF_DECLARE(nsp_genpll_clk, "brcm,nsp-genpll", nsp_genpll_clk_init);
    static const struct iproc_pll_ctrl lcpll0 = {
    .flags = IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_EMBED_PWRCTRL,
    .aon = AON_VAL(0x0, 1, 24, 0),
    .reset = RESET_VAL(0x0, 23, 22),
    .dig_filter = DF_VAL(0x0, 16, 3, 12, 4, 19, 4),
    .ndiv_int = REG_VAL(0x4, 20, 8),
    .ndiv_frac = REG_VAL(0x4, 0, 20),
    .pdiv = REG_VAL(0x4, 28, 3),
    .status = REG_VAL(0x10, 12, 1),
    };
    static const struct iproc_clk_ctrl lcpll0_clk[] = {
    [BCM_NSP_LCPLL0_PCIE_PHY_REF_CLK] = {
    .channel = BCM_NSP_LCPLL0_PCIE_PHY_REF_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 6, 3, 9),
    .mdiv = REG_VAL(0x8, 24, 8),
    },
    [BCM_NSP_LCPLL0_SDIO_CLK] = {
    .channel = BCM_NSP_LCPLL0_SDIO_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 7, 4, 10),
    .mdiv = REG_VAL(0x8, 16, 8),
    },
    [BCM_NSP_LCPLL0_DDR_PHY_CLK] = {
    .channel = BCM_NSP_LCPLL0_DDR_PHY_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 8, 5, 11),
    .mdiv = REG_VAL(0x8, 8, 8),
    },
    };
#[no_mangle]
unsafe extern "C" fn nsp_lcpll0_clk_init(node: *mut device_node) -> void __init {
    static void __init nsp_lcpll0_clk_init(struct device_node *node)
    {
    iproc_pll_clk_setup(node, &lcpll0, core::ptr::null_mut(), 0, lcpll0_clk,
    ARRAY_SIZE(lcpll0_clk));
    }
    CLK_OF_DECLARE(nsp_lcpll0_clk, "brcm,nsp-lcpll0", nsp_lcpll0_clk_init);
