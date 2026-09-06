//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-sr.c
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
// Copyright 2017 Broadcom
//

    .pwr_shift = ps, .iso_shift = is }

    .p_reset_shift = prs }

    .ki_shift = kis, .ki_width = kiw, .kp_shift = kps, .kp_width = kpw, \
    .ka_shift = kas, .ka_width = kaw }

    .hold_shift = hs, .bypass_shift = bs }
    static const struct iproc_pll_ctrl sr_genpll0 = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC |
    IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 5, 1, 0),
    .reset = RESET_VAL(0x0, 12, 11),
    .dig_filter = DF_VAL(0x0, 4, 3, 0, 4, 7, 3),
    .sw_ctrl = SW_CTRL_VAL(0x10, 31),
    .ndiv_int = REG_VAL(0x10, 20, 10),
    .ndiv_frac = REG_VAL(0x10, 0, 20),
    .pdiv = REG_VAL(0x14, 0, 4),
    .status = REG_VAL(0x30, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_genpll0_clk[] = {
    [BCM_SR_GENPLL0_125M_CLK] = {
    .channel = BCM_SR_GENPLL0_125M_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 6, 0, 12),
    .mdiv = REG_VAL(0x18, 0, 9),
    },
    [BCM_SR_GENPLL0_SCR_CLK] = {
    .channel = BCM_SR_GENPLL0_SCR_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 7, 1, 13),
    .mdiv = REG_VAL(0x18, 10, 9),
    },
    [BCM_SR_GENPLL0_250M_CLK] = {
    .channel = BCM_SR_GENPLL0_250M_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 8, 2, 14),
    .mdiv = REG_VAL(0x18, 20, 9),
    },
    [BCM_SR_GENPLL0_PCIE_AXI_CLK] = {
    .channel = BCM_SR_GENPLL0_PCIE_AXI_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 9, 3, 15),
    .mdiv = REG_VAL(0x1c, 0, 9),
    },
    [BCM_SR_GENPLL0_PAXC_AXI_X2_CLK] = {
    .channel = BCM_SR_GENPLL0_PAXC_AXI_X2_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 10, 4, 16),
    .mdiv = REG_VAL(0x1c, 10, 9),
    },
    [BCM_SR_GENPLL0_PAXC_AXI_CLK] = {
    .channel = BCM_SR_GENPLL0_PAXC_AXI_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 11, 5, 17),
    .mdiv = REG_VAL(0x1c, 20, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_genpll0_clk_init(pdev: *mut platform_device) -> c_int {
    static int sr_genpll0_clk_init(struct platform_device *pdev)
    {
    iproc_pll_clk_setup(pdev.dev.of_node,
    &sr_genpll0, core::ptr::null_mut(), 0, sr_genpll0_clk,
    ARRAY_SIZE(sr_genpll0_clk));
    return 0;
    }
    static const struct iproc_pll_ctrl sr_genpll2 = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC |
    IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 1, 13, 12),
    .reset = RESET_VAL(0x0, 12, 11),
    .dig_filter = DF_VAL(0x0, 4, 3, 0, 4, 7, 3),
    .sw_ctrl = SW_CTRL_VAL(0x10, 31),
    .ndiv_int = REG_VAL(0x10, 20, 10),
    .ndiv_frac = REG_VAL(0x10, 0, 20),
    .pdiv = REG_VAL(0x14, 0, 4),
    .status = REG_VAL(0x30, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_genpll2_clk[] = {
    [BCM_SR_GENPLL2_NIC_CLK] = {
    .channel = BCM_SR_GENPLL2_NIC_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 6, 0, 12),
    .mdiv = REG_VAL(0x18, 0, 9),
    },
    [BCM_SR_GENPLL2_TS_500_CLK] = {
    .channel = BCM_SR_GENPLL2_TS_500_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 7, 1, 13),
    .mdiv = REG_VAL(0x18, 10, 9),
    },
    [BCM_SR_GENPLL2_125_NITRO_CLK] = {
    .channel = BCM_SR_GENPLL2_125_NITRO_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 8, 2, 14),
    .mdiv = REG_VAL(0x18, 20, 9),
    },
    [BCM_SR_GENPLL2_CHIMP_CLK] = {
    .channel = BCM_SR_GENPLL2_CHIMP_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 9, 3, 15),
    .mdiv = REG_VAL(0x1c, 0, 9),
    },
    [BCM_SR_GENPLL2_NIC_FLASH_CLK] = {
    .channel = BCM_SR_GENPLL2_NIC_FLASH_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 10, 4, 16),
    .mdiv = REG_VAL(0x1c, 10, 9),
    },
    [BCM_SR_GENPLL2_FS4_CLK] = {
    .channel = BCM_SR_GENPLL2_FS4_CLK,
    .enable = ENABLE_VAL(0x4, 11, 5, 17),
    .mdiv = REG_VAL(0x1c, 20, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_genpll2_clk_init(pdev: *mut platform_device) -> c_int {
    static int sr_genpll2_clk_init(struct platform_device *pdev)
    {
    iproc_pll_clk_setup(pdev.dev.of_node,
    &sr_genpll2, core::ptr::null_mut(), 0, sr_genpll2_clk,
    ARRAY_SIZE(sr_genpll2_clk));
    return 0;
    }
    static const struct iproc_pll_ctrl sr_genpll3 = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC |
    IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 1, 19, 18),
    .reset = RESET_VAL(0x0, 12, 11),
    .dig_filter = DF_VAL(0x0, 4, 3, 0, 4, 7, 3),
    .sw_ctrl = SW_CTRL_VAL(0x10, 31),
    .ndiv_int = REG_VAL(0x10, 20, 10),
    .ndiv_frac = REG_VAL(0x10, 0, 20),
    .pdiv = REG_VAL(0x14, 0, 4),
    .status = REG_VAL(0x30, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_genpll3_clk[] = {
    [BCM_SR_GENPLL3_HSLS_CLK] = {
    .channel = BCM_SR_GENPLL3_HSLS_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 6, 0, 12),
    .mdiv = REG_VAL(0x18, 0, 9),
    },
    [BCM_SR_GENPLL3_SDIO_CLK] = {
    .channel = BCM_SR_GENPLL3_SDIO_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 7, 1, 13),
    .mdiv = REG_VAL(0x18, 10, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_genpll3_clk_init(node: *mut device_node) {
    static void sr_genpll3_clk_init(struct device_node *node)
    {
    iproc_pll_clk_setup(node, &sr_genpll3, core::ptr::null_mut(), 0, sr_genpll3_clk,
    ARRAY_SIZE(sr_genpll3_clk));
    }
    CLK_OF_DECLARE(sr_genpll3_clk, "brcm,sr-genpll3", sr_genpll3_clk_init);
    static const struct iproc_pll_ctrl sr_genpll4 = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC |
    IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 1, 25, 24),
    .reset = RESET_VAL(0x0, 12, 11),
    .dig_filter = DF_VAL(0x0, 4, 3, 0, 4, 7, 3),
    .sw_ctrl = SW_CTRL_VAL(0x10, 31),
    .ndiv_int = REG_VAL(0x10, 20, 10),
    .ndiv_frac = REG_VAL(0x10, 0, 20),
    .pdiv = REG_VAL(0x14, 0, 4),
    .status = REG_VAL(0x30, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_genpll4_clk[] = {
    [BCM_SR_GENPLL4_CCN_CLK] = {
    .channel = BCM_SR_GENPLL4_CCN_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 6, 0, 12),
    .mdiv = REG_VAL(0x18, 0, 9),
    },
    [BCM_SR_GENPLL4_TPIU_PLL_CLK] = {
    .channel = BCM_SR_GENPLL4_TPIU_PLL_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 7, 1, 13),
    .mdiv = REG_VAL(0x18, 10, 9),
    },
    [BCM_SR_GENPLL4_NOC_CLK] = {
    .channel = BCM_SR_GENPLL4_NOC_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 8, 2, 14),
    .mdiv = REG_VAL(0x18, 20, 9),
    },
    [BCM_SR_GENPLL4_CHCLK_FS4_CLK] = {
    .channel = BCM_SR_GENPLL4_CHCLK_FS4_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 9, 3, 15),
    .mdiv = REG_VAL(0x1c, 0, 9),
    },
    [BCM_SR_GENPLL4_BRIDGE_FSCPU_CLK] = {
    .channel = BCM_SR_GENPLL4_BRIDGE_FSCPU_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x4, 10, 4, 16),
    .mdiv = REG_VAL(0x1c, 10, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_genpll4_clk_init(pdev: *mut platform_device) -> c_int {
    static int sr_genpll4_clk_init(struct platform_device *pdev)
    {
    iproc_pll_clk_setup(pdev.dev.of_node,
    &sr_genpll4, core::ptr::null_mut(), 0, sr_genpll4_clk,
    ARRAY_SIZE(sr_genpll4_clk));
    return 0;
    }
    static const struct iproc_pll_ctrl sr_genpll5 = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC |
    IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 1, 1, 0),
    .reset = RESET_VAL(0x0, 12, 11),
    .dig_filter = DF_VAL(0x0, 4, 3, 0, 4, 7, 3),
    .sw_ctrl = SW_CTRL_VAL(0x10, 31),
    .ndiv_int = REG_VAL(0x10, 20, 10),
    .ndiv_frac = REG_VAL(0x10, 0, 20),
    .pdiv = REG_VAL(0x14, 0, 4),
    .status = REG_VAL(0x30, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_genpll5_clk[] = {
    [BCM_SR_GENPLL5_FS4_HF_CLK] = {
    .channel = BCM_SR_GENPLL5_FS4_HF_CLK,
    .enable = ENABLE_VAL(0x4, 6, 0, 12),
    .mdiv = REG_VAL(0x18, 0, 9),
    },
    [BCM_SR_GENPLL5_CRYPTO_AE_CLK] = {
    .channel = BCM_SR_GENPLL5_CRYPTO_AE_CLK,
    .enable = ENABLE_VAL(0x4, 7, 1, 12),
    .mdiv = REG_VAL(0x18, 10, 9),
    },
    [BCM_SR_GENPLL5_RAID_AE_CLK] = {
    .channel = BCM_SR_GENPLL5_RAID_AE_CLK,
    .enable = ENABLE_VAL(0x4, 8, 2, 14),
    .mdiv = REG_VAL(0x18, 20, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_genpll5_clk_init(pdev: *mut platform_device) -> c_int {
    static int sr_genpll5_clk_init(struct platform_device *pdev)
    {
    iproc_pll_clk_setup(pdev.dev.of_node,
    &sr_genpll5, core::ptr::null_mut(), 0, sr_genpll5_clk,
    ARRAY_SIZE(sr_genpll5_clk));
    return 0;
    }
    static const struct iproc_pll_ctrl sr_lcpll0 = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 2, 19, 18),
    .reset = RESET_VAL(0x0, 31, 30),
    .sw_ctrl = SW_CTRL_VAL(0x4, 31),
    .ndiv_int = REG_VAL(0x4, 16, 10),
    .pdiv = REG_VAL(0x4, 26, 4),
    .status = REG_VAL(0x38, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_lcpll0_clk[] = {
    [BCM_SR_LCPLL0_SATA_REFP_CLK] = {
    .channel = BCM_SR_LCPLL0_SATA_REFP_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 7, 1, 13),
    .mdiv = REG_VAL(0x14, 0, 9),
    },
    [BCM_SR_LCPLL0_SATA_REFN_CLK] = {
    .channel = BCM_SR_LCPLL0_SATA_REFN_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 8, 2, 14),
    .mdiv = REG_VAL(0x14, 10, 9),
    },
    [BCM_SR_LCPLL0_SATA_350_CLK] = {
    .channel = BCM_SR_LCPLL0_SATA_350_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 9, 3, 15),
    .mdiv = REG_VAL(0x14, 20, 9),
    },
    [BCM_SR_LCPLL0_SATA_500_CLK] = {
    .channel = BCM_SR_LCPLL0_SATA_500_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 10, 4, 16),
    .mdiv = REG_VAL(0x18, 0, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_lcpll0_clk_init(pdev: *mut platform_device) -> c_int {
    static int sr_lcpll0_clk_init(struct platform_device *pdev)
    {
    iproc_pll_clk_setup(pdev.dev.of_node,
    &sr_lcpll0, core::ptr::null_mut(), 0, sr_lcpll0_clk,
    ARRAY_SIZE(sr_lcpll0_clk));
    return 0;
    }
    static const struct iproc_pll_ctrl sr_lcpll1 = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 2, 22, 21),
    .reset = RESET_VAL(0x0, 31, 30),
    .sw_ctrl = SW_CTRL_VAL(0x4, 31),
    .ndiv_int = REG_VAL(0x4, 16, 10),
    .pdiv = REG_VAL(0x4, 26, 4),
    .status = REG_VAL(0x38, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_lcpll1_clk[] = {
    [BCM_SR_LCPLL1_WAN_CLK] = {
    .channel = BCM_SR_LCPLL1_WAN_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 7, 1, 13),
    .mdiv = REG_VAL(0x14, 0, 9),
    },
    [BCM_SR_LCPLL1_USB_REF_CLK] = {
    .channel = BCM_SR_LCPLL1_USB_REF_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 8, 2, 14),
    .mdiv = REG_VAL(0x14, 10, 9),
    },
    [BCM_SR_LCPLL1_CRMU_TS_CLK] = {
    .channel = BCM_SR_LCPLL1_CRMU_TS_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 9, 3, 15),
    .mdiv = REG_VAL(0x14, 20, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_lcpll1_clk_init(pdev: *mut platform_device) -> c_int {
    static int sr_lcpll1_clk_init(struct platform_device *pdev)
    {
    iproc_pll_clk_setup(pdev.dev.of_node,
    &sr_lcpll1, core::ptr::null_mut(), 0, sr_lcpll1_clk,
    ARRAY_SIZE(sr_lcpll1_clk));
    return 0;
    }
    static const struct iproc_pll_ctrl sr_lcpll_pcie = {
    .flags = IPROC_CLK_AON | IPROC_CLK_PLL_NEEDS_SW_CFG,
    .aon = AON_VAL(0x0, 2, 25, 24),
    .reset = RESET_VAL(0x0, 31, 30),
    .sw_ctrl = SW_CTRL_VAL(0x4, 31),
    .ndiv_int = REG_VAL(0x4, 16, 10),
    .pdiv = REG_VAL(0x4, 26, 4),
    .status = REG_VAL(0x38, 12, 1),
    };
    static const struct iproc_clk_ctrl sr_lcpll_pcie_clk[] = {
    [BCM_SR_LCPLL_PCIE_PHY_REF_CLK] = {
    .channel = BCM_SR_LCPLL_PCIE_PHY_REF_CLK,
    .flags = IPROC_CLK_AON,
    .enable = ENABLE_VAL(0x0, 7, 1, 13),
    .mdiv = REG_VAL(0x14, 0, 9),
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_lcpll_pcie_clk_init(pdev: *mut platform_device) -> c_int {
    static int sr_lcpll_pcie_clk_init(struct platform_device *pdev)
    {
    iproc_pll_clk_setup(pdev.dev.of_node,
    &sr_lcpll_pcie, core::ptr::null_mut(), 0, sr_lcpll_pcie_clk,
    ARRAY_SIZE(sr_lcpll_pcie_clk));
    return 0;
    }
    static const struct of_device_id sr_clk_dt_ids[] = {
    { .compatible = "brcm,sr-genpll0", .data = sr_genpll0_clk_init },
    { .compatible = "brcm,sr-genpll2", .data = sr_genpll2_clk_init },
    { .compatible = "brcm,sr-genpll4", .data = sr_genpll4_clk_init },
    { .compatible = "brcm,sr-genpll5", .data = sr_genpll5_clk_init },
    { .compatible = "brcm,sr-lcpll0", .data = sr_lcpll0_clk_init },
    { .compatible = "brcm,sr-lcpll1", .data = sr_lcpll1_clk_init },
    { .compatible = "brcm,sr-lcpll-pcie", .data = sr_lcpll_pcie_clk_init },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn sr_clk_probe(pdev: *mut platform_device) -> c_int {
    static int sr_clk_probe(struct platform_device *pdev)
    {
    int (*probe_func)(struct platform_device *);
    probe_func = of_device_get_match_data(&pdev.dev);
    if (!probe_func)
    return -ENODEV;
    return probe_func(pdev);
    }
    static struct platform_driver sr_clk_driver = {
    .driver = {
    .name = "sr-clk",
    .of_match_table = sr_clk_dt_ids,
    },
    .probe = sr_clk_probe,
    };
    builtin_platform_driver(sr_clk_driver);
