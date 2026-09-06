//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-ns2.c
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
    static const struct iproc_pll_ctrl genpll_scr = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL,
    .aon = AON_VAL(0x0, 1, 15, 12),
    .reset = RESET_VAL(0x4, 2, 1),
    .dig_filter = DF_VAL(0x0, 9, 3, 5, 4, 2, 3),
    .ndiv_int = REG_VAL(0x8, 4, 10),
    .pdiv = REG_VAL(0x8, 0, 4),
    .vco_ctrl = VCO_CTRL_VAL(0x10, 0xc),
    .status = REG_VAL(0x0, 27, 1),
    };
    static const struct iproc_clk_ctrl genpll_scr_clk[] = {
// bypass_shift, the last value passed into ENABLE_VAL(), is not defined
// in NS2.  However, it doesn't appear to be used anywhere, so setting
// it to 0.
//
    [BCM_NS2_GENPLL_SCR_SCR_CLK] = {
    .channel = BCM_NS2_GENPLL_SCR_SCR_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 18, 12, 0),
    .mdiv = REG_VAL(0x18, 0, 8),
    },
    [BCM_NS2_GENPLL_SCR_FS_CLK] = {
    .channel = BCM_NS2_GENPLL_SCR_FS_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 19, 13, 0),
    .mdiv = REG_VAL(0x18, 8, 8),
    },
    [BCM_NS2_GENPLL_SCR_AUDIO_CLK] = {
    .channel = BCM_NS2_GENPLL_SCR_AUDIO_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 20, 14, 0),
    .mdiv = REG_VAL(0x14, 0, 8),
    },
    [BCM_NS2_GENPLL_SCR_CH3_UNUSED] = {
    .channel = BCM_NS2_GENPLL_SCR_CH3_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 21, 15, 0),
    .mdiv = REG_VAL(0x14, 8, 8),
    },
    [BCM_NS2_GENPLL_SCR_CH4_UNUSED] = {
    .channel = BCM_NS2_GENPLL_SCR_CH4_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 22, 16, 0),
    .mdiv = REG_VAL(0x14, 16, 8),
    },
    [BCM_NS2_GENPLL_SCR_CH5_UNUSED] = {
    .channel = BCM_NS2_GENPLL_SCR_CH5_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 23, 17, 0),
    .mdiv = REG_VAL(0x14, 24, 8),
    },
    };
#[no_mangle]
unsafe extern "C" fn ns2_genpll_scr_clk_init(node: *mut device_node) -> void __init {
    static void __init ns2_genpll_scr_clk_init(struct device_node *node)
    {
    iproc_pll_clk_setup(node, &genpll_scr, core::ptr::null_mut(), 0, genpll_scr_clk,
    ARRAY_SIZE(genpll_scr_clk));
    }
    CLK_OF_DECLARE(ns2_genpll_src_clk, "brcm,ns2-genpll-scr",
    ns2_genpll_scr_clk_init);
    static const struct iproc_pll_ctrl genpll_sw = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL,
    .aon = AON_VAL(0x0, 1, 11, 10),
    .reset = RESET_VAL(0x4, 2, 1),
    .dig_filter = DF_VAL(0x0, 9, 3, 5, 4, 2, 3),
    .ndiv_int = REG_VAL(0x8, 4, 10),
    .pdiv = REG_VAL(0x8, 0, 4),
    .vco_ctrl = VCO_CTRL_VAL(0x10, 0xc),
    .status = REG_VAL(0x0, 13, 1),
    };
    static const struct iproc_clk_ctrl genpll_sw_clk[] = {
// bypass_shift, the last value passed into ENABLE_VAL(), is not defined
// in NS2.  However, it doesn't appear to be used anywhere, so setting
// it to 0.
//
    [BCM_NS2_GENPLL_SW_RPE_CLK] = {
    .channel = BCM_NS2_GENPLL_SW_RPE_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 18, 12, 0),
    .mdiv = REG_VAL(0x18, 0, 8),
    },
    [BCM_NS2_GENPLL_SW_250_CLK] = {
    .channel = BCM_NS2_GENPLL_SW_250_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 19, 13, 0),
    .mdiv = REG_VAL(0x18, 8, 8),
    },
    [BCM_NS2_GENPLL_SW_NIC_CLK] = {
    .channel = BCM_NS2_GENPLL_SW_NIC_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 20, 14, 0),
    .mdiv = REG_VAL(0x14, 0, 8),
    },
    [BCM_NS2_GENPLL_SW_CHIMP_CLK] = {
    .channel = BCM_NS2_GENPLL_SW_CHIMP_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 21, 15, 0),
    .mdiv = REG_VAL(0x14, 8, 8),
    },
    [BCM_NS2_GENPLL_SW_PORT_CLK] = {
    .channel = BCM_NS2_GENPLL_SW_PORT_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 22, 16, 0),
    .mdiv = REG_VAL(0x14, 16, 8),
    },
    [BCM_NS2_GENPLL_SW_SDIO_CLK] = {
    .channel = BCM_NS2_GENPLL_SW_SDIO_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 23, 17, 0),
    .mdiv = REG_VAL(0x14, 24, 8),
    },
    };
#[no_mangle]
unsafe extern "C" fn ns2_genpll_sw_clk_init(node: *mut device_node) -> void __init {
    static void __init ns2_genpll_sw_clk_init(struct device_node *node)
    {
    iproc_pll_clk_setup(node, &genpll_sw, core::ptr::null_mut(), 0, genpll_sw_clk,
    ARRAY_SIZE(genpll_sw_clk));
    }
    CLK_OF_DECLARE(ns2_genpll_sw_clk, "brcm,ns2-genpll-sw",
    ns2_genpll_sw_clk_init);
    static const struct iproc_pll_ctrl lcpll_ddr = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL,
    .aon = AON_VAL(0x0, 2, 1, 0),
    .reset = RESET_VAL(0x4, 2, 1),
    .dig_filter = DF_VAL(0x0, 9, 3, 5, 4, 1, 4),
    .ndiv_int = REG_VAL(0x8, 4, 10),
    .pdiv = REG_VAL(0x8, 0, 4),
    .vco_ctrl = VCO_CTRL_VAL(0x10, 0xc),
    .status = REG_VAL(0x0, 0, 1),
    };
    static const struct iproc_clk_ctrl lcpll_ddr_clk[] = {
// bypass_shift, the last value passed into ENABLE_VAL(), is not defined
// in NS2.  However, it doesn't appear to be used anywhere, so setting
// it to 0.
//
    [BCM_NS2_LCPLL_DDR_PCIE_SATA_USB_CLK] = {
    .channel = BCM_NS2_LCPLL_DDR_PCIE_SATA_USB_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 18, 12, 0),
    .mdiv = REG_VAL(0x14, 0, 8),
    },
    [BCM_NS2_LCPLL_DDR_DDR_CLK] = {
    .channel = BCM_NS2_LCPLL_DDR_DDR_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 19, 13, 0),
    .mdiv = REG_VAL(0x14, 8, 8),
    },
    [BCM_NS2_LCPLL_DDR_CH2_UNUSED] = {
    .channel = BCM_NS2_LCPLL_DDR_CH2_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 20, 14, 0),
    .mdiv = REG_VAL(0x10, 0, 8),
    },
    [BCM_NS2_LCPLL_DDR_CH3_UNUSED] = {
    .channel = BCM_NS2_LCPLL_DDR_CH3_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 21, 15, 0),
    .mdiv = REG_VAL(0x10, 8, 8),
    },
    [BCM_NS2_LCPLL_DDR_CH4_UNUSED] = {
    .channel = BCM_NS2_LCPLL_DDR_CH4_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 22, 16, 0),
    .mdiv = REG_VAL(0x10, 16, 8),
    },
    [BCM_NS2_LCPLL_DDR_CH5_UNUSED] = {
    .channel = BCM_NS2_LCPLL_DDR_CH5_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 23, 17, 0),
    .mdiv = REG_VAL(0x10, 24, 8),
    },
    };
#[no_mangle]
unsafe extern "C" fn ns2_lcpll_ddr_clk_init(node: *mut device_node) -> void __init {
    static void __init ns2_lcpll_ddr_clk_init(struct device_node *node)
    {
    iproc_pll_clk_setup(node, &lcpll_ddr, core::ptr::null_mut(), 0, lcpll_ddr_clk,
    ARRAY_SIZE(lcpll_ddr_clk));
    }
    CLK_OF_DECLARE(ns2_lcpll_ddr_clk, "brcm,ns2-lcpll-ddr",
    ns2_lcpll_ddr_clk_init);
    static const struct iproc_pll_ctrl lcpll_ports = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL,
    .aon = AON_VAL(0x0, 2, 5, 4),
    .reset = RESET_VAL(0x4, 2, 1),
    .dig_filter = DF_VAL(0x0, 9, 3, 5, 4, 1, 4),
    .ndiv_int = REG_VAL(0x8, 4, 10),
    .pdiv = REG_VAL(0x8, 0, 4),
    .vco_ctrl = VCO_CTRL_VAL(0x10, 0xc),
    .status = REG_VAL(0x0, 0, 1),
    };
    static const struct iproc_clk_ctrl lcpll_ports_clk[] = {
// bypass_shift, the last value passed into ENABLE_VAL(), is not defined
// in NS2.  However, it doesn't appear to be used anywhere, so setting
// it to 0.
//
    [BCM_NS2_LCPLL_PORTS_WAN_CLK] = {
    .channel = BCM_NS2_LCPLL_PORTS_WAN_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 18, 12, 0),
    .mdiv = REG_VAL(0x14, 0, 8),
    },
    [BCM_NS2_LCPLL_PORTS_RGMII_CLK] = {
    .channel = BCM_NS2_LCPLL_PORTS_RGMII_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 19, 13, 0),
    .mdiv = REG_VAL(0x14, 8, 8),
    },
    [BCM_NS2_LCPLL_PORTS_CH2_UNUSED] = {
    .channel = BCM_NS2_LCPLL_PORTS_CH2_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 20, 14, 0),
    .mdiv = REG_VAL(0x10, 0, 8),
    },
    [BCM_NS2_LCPLL_PORTS_CH3_UNUSED] = {
    .channel = BCM_NS2_LCPLL_PORTS_CH3_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 21, 15, 0),
    .mdiv = REG_VAL(0x10, 8, 8),
    },
    [BCM_NS2_LCPLL_PORTS_CH4_UNUSED] = {
    .channel = BCM_NS2_LCPLL_PORTS_CH4_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 22, 16, 0),
    .mdiv = REG_VAL(0x10, 16, 8),
    },
    [BCM_NS2_LCPLL_PORTS_CH5_UNUSED] = {
    .channel = BCM_NS2_LCPLL_PORTS_CH5_UNUSED,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 23, 17, 0),
    .mdiv = REG_VAL(0x10, 24, 8),
    },
    };
#[no_mangle]
unsafe extern "C" fn ns2_lcpll_ports_clk_init(node: *mut device_node) -> void __init {
    static void __init ns2_lcpll_ports_clk_init(struct device_node *node)
    {
    iproc_pll_clk_setup(node, &lcpll_ports, core::ptr::null_mut(), 0, lcpll_ports_clk,
    ARRAY_SIZE(lcpll_ports_clk));
    }
    CLK_OF_DECLARE(ns2_lcpll_ports_clk, "brcm,ns2-lcpll-ports",
    ns2_lcpll_ports_clk_init);
