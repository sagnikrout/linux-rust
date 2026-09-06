//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_tegra.c
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
// drivers/ata/ahci_tegra.c
//
// Copyright (c) 2014, NVIDIA CORPORATION.  All rights reserved.
//
// Author:
// Mikko Perttunen <mperttunen@nvidia.com>
//

pub const SATA_CONFIGURATION_0: c_uint = 0x180;

pub const SCFG_OFFSET: c_uint = 0x1000;
pub const T_SATA0_CFG_1: c_uint = 0x04;

pub const T_SATA0_CFG_9: c_uint = 0x24;
pub const T_SATA0_CFG_9_BASE_ADDRESS: c_uint = 0x40020000;
pub const SATA_FPCI_BAR5: c_uint = 0x94;

pub const SATA_INTR_MASK: c_uint = 0x188;

pub const T_SATA0_CFG_35: c_uint = 0x94;

pub const T_SATA0_AHCI_IDP1: c_uint = 0x98;

pub const T_SATA0_CFG_PHY_1: c_uint = 0x12c;

pub const T_SATA0_NVOOB: c_uint = 0x114;

pub const T_SATA_CFG_PHY_0: c_uint = 0x120;

pub const T_SATA0_CFG2NVOOB_2: c_uint = 0x134;

pub const T_SATA0_AHCI_HBA_CAP_BKDR: c_uint = 0x300;

pub const T_SATA0_BKDOOR_CC: c_uint = 0x4a4;

pub const T_SATA0_CFG_SATA: c_uint = 0x54c;

pub const T_SATA0_CFG_MISC: c_uint = 0x550;
pub const T_SATA0_INDEX: c_uint = 0x680;
pub const T_SATA0_CHX_PHY_CTRL1_GEN1: c_uint = 0x690;
pub const T_SATA0_CHX_PHY_CTRL1_GEN1_TX_AMP_MASK: c_uint = 0xff;
pub const T_SATA0_CHX_PHY_CTRL1_GEN1_TX_AMP_SHIFT: c_int = 0;

pub const T_SATA0_CHX_PHY_CTRL1_GEN1_TX_PEAK_SHIFT: c_int = 8;
pub const T_SATA0_CHX_PHY_CTRL1_GEN2: c_uint = 0x694;
pub const T_SATA0_CHX_PHY_CTRL1_GEN2_TX_AMP_MASK: c_uint = 0xff;
pub const T_SATA0_CHX_PHY_CTRL1_GEN2_TX_AMP_SHIFT: c_int = 0;

pub const T_SATA0_CHX_PHY_CTRL1_GEN2_TX_PEAK_SHIFT: c_int = 12;
pub const T_SATA0_CHX_PHY_CTRL2: c_uint = 0x69c;
pub const T_SATA0_CHX_PHY_CTRL2_CDR_CNTL_GEN1: c_uint = 0x23;
pub const T_SATA0_CHX_PHY_CTRL11: c_uint = 0x6d0;

pub const T_SATA0_CHX_PHY_CTRL17_0: c_uint = 0x6e8;
pub const T_SATA0_CHX_PHY_CTRL17_0_RX_EQ_CTRL_L_GEN1: c_uint = 0x55010000;
pub const T_SATA0_CHX_PHY_CTRL18_0: c_uint = 0x6ec;
pub const T_SATA0_CHX_PHY_CTRL18_0_RX_EQ_CTRL_L_GEN2: c_uint = 0x55010000;
pub const T_SATA0_CHX_PHY_CTRL20_0: c_uint = 0x6f4;
pub const T_SATA0_CHX_PHY_CTRL20_0_RX_EQ_CTRL_H_GEN1: c_uint = 0x1;
pub const T_SATA0_CHX_PHY_CTRL21_0: c_uint = 0x6f8;
pub const T_SATA0_CHX_PHY_CTRL21_0_RX_EQ_CTRL_H_GEN2: c_uint = 0x1;
// AUX Registers
pub const SATA_AUX_MISC_CNTL_1_0: c_uint = 0x8;

pub const SATA_AUX_RX_STAT_INT_0: c_uint = 0xc;

pub const SATA_AUX_SPARE_CFG0_0: c_uint = 0x18;

pub const FUSE_SATA_CALIB: c_uint = 0x124;
pub const FUSE_SATA_CALIB_MASK: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_pad_calibration {
    pub gen1_tx_amp: u8,
    pub gen1_tx_peak: u8,
    pub gen2_tx_amp: u8,
    pub gen2_tx_peak: u8,
}

    static const struct sata_pad_calibration tegra124_pad_calibration[] = {
    {0x18, 0x04, 0x18, 0x0a},
    {0x0e, 0x04, 0x14, 0x0a},
    {0x0e, 0x07, 0x1a, 0x0e},
    {0x14, 0x0e, 0x1a, 0x0e},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ahci_ops {
    pub hpriv): *mut *mut int (init)(struct ahci_host_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ahci_regs {
    pub nvoob_comma_cnt_mask: c_uint,
    pub nvoob_comma_cnt_val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ahci_soc {
    pub supply_names: *const *const c_char,
    pub num_supplies: u32,
    pub supports_devslp: bool,
    pub has_sata_oob_rst: bool,
    pub ops: *const tegra_ahci_ops,
    pub regs: *const tegra_ahci_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ahci_priv {
    pub pdev: *mut platform_device,
    pub sata_regs: *mut void __iomem,
    pub sata_aux_regs: *mut void __iomem,
    pub sata_rst: *mut reset_control,
    pub sata_oob_rst: *mut reset_control,
    pub sata_cold_rst: *mut reset_control,
// Needs special handling, cannot use ahci_platform
    pub sata_clk: *mut clk,
    pub pmc: *mut tegra_pmc,
    pub supplies: *mut regulator_bulk_data,
    pub soc: *const tegra_ahci_soc,
}

#[no_mangle]
unsafe extern "C" fn tegra_ahci_handle_quirks(hpriv: *mut ahci_host_priv) {
    static void tegra_ahci_handle_quirks(struct ahci_host_priv *hpriv)
    {
    struct tegra_ahci_priv *tegra = hpriv.plat_data;
    u32 val;
    if (tegra.sata_aux_regs && !tegra.soc.supports_devslp) {
    val = readl(tegra.sata_aux_regs + SATA_AUX_MISC_CNTL_1_0);
    val &= ~SATA_AUX_MISC_CNTL_1_0_SDS_SUPPORT;
    writel(val, tegra.sata_aux_regs + SATA_AUX_MISC_CNTL_1_0);
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra124_ahci_init(hpriv: *mut ahci_host_priv) -> c_int {
    static int tegra124_ahci_init(struct ahci_host_priv *hpriv)
    {
    struct tegra_ahci_priv *tegra = hpriv.plat_data;
    struct sata_pad_calibration calib;
    int ret;
    u32 val;
// Pad calibration
    ret = tegra_fuse_readl(FUSE_SATA_CALIB, &val);
    if (ret)
    return ret;
    calib = tegra124_pad_calibration[val & FUSE_SATA_CALIB_MASK];
    writel(BIT(0), tegra.sata_regs + SCFG_OFFSET + T_SATA0_INDEX);
    val = readl(tegra.sata_regs +
    SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL1_GEN1);
    val &= ~T_SATA0_CHX_PHY_CTRL1_GEN1_TX_AMP_MASK;
    val &= ~T_SATA0_CHX_PHY_CTRL1_GEN1_TX_PEAK_MASK;
    val |= calib.gen1_tx_amp << T_SATA0_CHX_PHY_CTRL1_GEN1_TX_AMP_SHIFT;
    val |= calib.gen1_tx_peak << T_SATA0_CHX_PHY_CTRL1_GEN1_TX_PEAK_SHIFT;
    writel(val, tegra.sata_regs + SCFG_OFFSET +
    T_SATA0_CHX_PHY_CTRL1_GEN1);
    val = readl(tegra.sata_regs +
    SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL1_GEN2);
    val &= ~T_SATA0_CHX_PHY_CTRL1_GEN2_TX_AMP_MASK;
    val &= ~T_SATA0_CHX_PHY_CTRL1_GEN2_TX_PEAK_MASK;
    val |= calib.gen2_tx_amp << T_SATA0_CHX_PHY_CTRL1_GEN1_TX_AMP_SHIFT;
    val |= calib.gen2_tx_peak << T_SATA0_CHX_PHY_CTRL1_GEN1_TX_PEAK_SHIFT;
    writel(val, tegra.sata_regs + SCFG_OFFSET +
    T_SATA0_CHX_PHY_CTRL1_GEN2);
    writel(T_SATA0_CHX_PHY_CTRL11_GEN2_RX_EQ,
    tegra.sata_regs + SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL11);
    writel(T_SATA0_CHX_PHY_CTRL2_CDR_CNTL_GEN1,
    tegra.sata_regs + SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL2);
    writel(0, tegra.sata_regs + SCFG_OFFSET + T_SATA0_INDEX);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_ahci_power_on(hpriv: *mut ahci_host_priv) -> c_int {
    static int tegra_ahci_power_on(struct ahci_host_priv *hpriv)
    {
    struct tegra_ahci_priv *tegra = hpriv.plat_data;
    int ret;
    ret = regulator_bulk_enable(tegra.soc.num_supplies,
    tegra.supplies);
    if (ret)
    return ret;
    if (!tegra.pdev.dev.pm_domain) {
    ret = tegra_pmc_powergate_sequence_power_up(tegra.pmc,
    TEGRA_POWERGATE_SATA,
    tegra.sata_clk,
    tegra.sata_rst);
    if (ret)
    goto disable_regulators;
    }
    reset_control_assert(tegra.sata_oob_rst);
    reset_control_assert(tegra.sata_cold_rst);
    ret = ahci_platform_enable_resources(hpriv);
    if (ret)
    goto disable_power;
    reset_control_deassert(tegra.sata_cold_rst);
    reset_control_deassert(tegra.sata_oob_rst);
    return 0;
    disable_power:
    clk_disable_unprepare(tegra.sata_clk);
    if (!tegra.pdev.dev.pm_domain)
    tegra_pmc_powergate_power_off(tegra.pmc, TEGRA_POWERGATE_SATA);
    disable_regulators:
    regulator_bulk_disable(tegra.soc.num_supplies, tegra.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_ahci_power_off(hpriv: *mut ahci_host_priv) {
    static void tegra_ahci_power_off(struct ahci_host_priv *hpriv)
    {
    struct tegra_ahci_priv *tegra = hpriv.plat_data;
    ahci_platform_disable_resources(hpriv);
    reset_control_assert(tegra.sata_rst);
    reset_control_assert(tegra.sata_oob_rst);
    reset_control_assert(tegra.sata_cold_rst);
    clk_disable_unprepare(tegra.sata_clk);
    if (!tegra.pdev.dev.pm_domain)
    tegra_pmc_powergate_power_off(tegra.pmc, TEGRA_POWERGATE_SATA);
    regulator_bulk_disable(tegra.soc.num_supplies, tegra.supplies);
    }
#[no_mangle]
unsafe extern "C" fn tegra_ahci_controller_init(hpriv: *mut ahci_host_priv) -> c_int {
    static int tegra_ahci_controller_init(struct ahci_host_priv *hpriv)
    {
    struct tegra_ahci_priv *tegra = hpriv.plat_data;
    int ret;
    u32 val;
    ret = tegra_ahci_power_on(hpriv);
    if (ret) {
    dev_err(&tegra.pdev.dev,
    "failed to power on AHCI controller: %d\n", ret);
    return ret;
    }
//
// Program the following SATA IPFS registers to allow SW accesses to
// SATA's MMIO register range.
//
    val = readl(tegra.sata_regs + SATA_FPCI_BAR5);
    val &= ~(SATA_FPCI_BAR5_START_MASK | SATA_FPCI_BAR5_ACCESS_TYPE);
    val |= SATA_FPCI_BAR5_START | SATA_FPCI_BAR5_ACCESS_TYPE;
    writel(val, tegra.sata_regs + SATA_FPCI_BAR5);
// Program the following SATA IPFS register to enable the SATA
    val = readl(tegra.sata_regs + SATA_CONFIGURATION_0);
    val |= SATA_CONFIGURATION_0_EN_FPCI;
    writel(val, tegra.sata_regs + SATA_CONFIGURATION_0);
// Electrical settings for better link stability
    val = T_SATA0_CHX_PHY_CTRL17_0_RX_EQ_CTRL_L_GEN1;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL17_0);
    val = T_SATA0_CHX_PHY_CTRL18_0_RX_EQ_CTRL_L_GEN2;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL18_0);
    val = T_SATA0_CHX_PHY_CTRL20_0_RX_EQ_CTRL_H_GEN1;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL20_0);
    val = T_SATA0_CHX_PHY_CTRL21_0_RX_EQ_CTRL_H_GEN2;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CHX_PHY_CTRL21_0);
// For SQUELCH Filter & Gen3 drive getting detected as Gen1 drive
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA_CFG_PHY_0);
    val |= T_SATA_CFG_PHY_0_MASK_SQUELCH;
    val &= ~T_SATA_CFG_PHY_0_USE_7BIT_ALIGN_DET_FOR_SPD;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA_CFG_PHY_0);
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_NVOOB);
    val &= ~(tegra.soc.regs.nvoob_comma_cnt_mask |
    T_SATA0_NVOOB_SQUELCH_FILTER_LENGTH_MASK |
    T_SATA0_NVOOB_SQUELCH_FILTER_MODE_MASK);
    val |= (tegra.soc.regs.nvoob_comma_cnt_val |
    T_SATA0_NVOOB_SQUELCH_FILTER_LENGTH |
    T_SATA0_NVOOB_SQUELCH_FILTER_MODE);
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_NVOOB);
//
// Change CFG2NVOOB_2_COMWAKE_IDLE_CNT_LOW from 83.3 ns to 58.8ns
//
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG2NVOOB_2);
    val &= ~T_SATA0_CFG2NVOOB_2_COMWAKE_IDLE_CNT_LOW_MASK;
    val |= T_SATA0_CFG2NVOOB_2_COMWAKE_IDLE_CNT_LOW;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG2NVOOB_2);
    if (tegra.soc.ops && tegra.soc.ops.init)
    tegra.soc.ops.init(hpriv);
//
// Program the following SATA configuration registers to
// initialize SATA
//
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_1);
    val |= (T_SATA0_CFG_1_IO_SPACE | T_SATA0_CFG_1_MEMORY_SPACE |
    T_SATA0_CFG_1_BUS_MASTER | T_SATA0_CFG_1_SERR);
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_1);
    val = T_SATA0_CFG_9_BASE_ADDRESS;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_9);
// Program Class Code and Programming interface for SATA
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_SATA);
    val |= T_SATA0_CFG_SATA_BACKDOOR_PROG_IF_EN;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_SATA);
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_BKDOOR_CC);
    val &=
    ~(T_SATA0_BKDOOR_CC_CLASS_CODE_MASK |
    T_SATA0_BKDOOR_CC_PROG_IF_MASK);
    val |= T_SATA0_BKDOOR_CC_CLASS_CODE | T_SATA0_BKDOOR_CC_PROG_IF;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_BKDOOR_CC);
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_SATA);
    val &= ~T_SATA0_CFG_SATA_BACKDOOR_PROG_IF_EN;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_SATA);
// Enabling LPM capabilities through Backdoor Programming
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_AHCI_HBA_CAP_BKDR);
    val |= (T_SATA0_AHCI_HBA_CAP_BKDR_PARTIAL_ST_CAP |
    T_SATA0_AHCI_HBA_CAP_BKDR_SLUMBER_ST_CAP |
    T_SATA0_AHCI_HBA_CAP_BKDR_SALP |
    T_SATA0_AHCI_HBA_CAP_BKDR_SUPP_PM);
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_AHCI_HBA_CAP_BKDR);
// SATA Second Level Clock Gating configuration
// Enabling Gating of Tx/Rx clocks and driving Pad IDDQ and Lane
// IDDQ Signals
//
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_35);
    val &= ~T_SATA0_CFG_35_IDP_INDEX_MASK;
    val |= T_SATA0_CFG_35_IDP_INDEX;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_35);
    val = T_SATA0_AHCI_IDP1_DATA;
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_AHCI_IDP1);
    val = readl(tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_PHY_1);
    val |= (T_SATA0_CFG_PHY_1_PADS_IDDQ_EN |
    T_SATA0_CFG_PHY_1_PAD_PLL_IDDQ_EN);
    writel(val, tegra.sata_regs + SCFG_OFFSET + T_SATA0_CFG_PHY_1);
// Enabling IPFS Clock Gating
    val = readl(tegra.sata_regs + SATA_CONFIGURATION_0);
    val &= ~SATA_CONFIGURATION_0_CLK_OVERRIDE;
    writel(val, tegra.sata_regs + SATA_CONFIGURATION_0);
    tegra_ahci_handle_quirks(hpriv);
// Unmask SATA interrupts
    val = readl(tegra.sata_regs + SATA_INTR_MASK);
    val |= SATA_INTR_MASK_IP_INT_MASK;
    writel(val, tegra.sata_regs + SATA_INTR_MASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_ahci_controller_deinit(hpriv: *mut ahci_host_priv) {
    static void tegra_ahci_controller_deinit(struct ahci_host_priv *hpriv)
    {
    tegra_ahci_power_off(hpriv);
    }
#[no_mangle]
unsafe extern "C" fn tegra_ahci_host_stop(host: *mut ata_host) {
    static void tegra_ahci_host_stop(struct ata_host *host)
    {
    struct ahci_host_priv *hpriv = host.private_data;
    tegra_ahci_controller_deinit(hpriv);
    }
    static struct ata_port_operations ahci_tegra_port_ops = {
    .inherits	= &ahci_ops,
    .host_stop	= tegra_ahci_host_stop,
    };
    static const struct ata_port_info ahci_tegra_port_info = {
    .flags		= AHCI_FLAG_COMMON | ATA_FLAG_NO_DIPM,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &ahci_tegra_port_ops,
    };
    static const char *const tegra124_supply_names[] = {
    "avdd", "hvdd", "vddio", "target-5v", "target-12v"
    };
    static const struct tegra_ahci_ops tegra124_ahci_ops = {
    .init = tegra124_ahci_init,
    };
    static const struct tegra_ahci_regs tegra124_ahci_regs = {
    .nvoob_comma_cnt_mask = GENMASK(30, 28),
    .nvoob_comma_cnt_val = (7 << 28),
    };
    static const struct tegra_ahci_soc tegra124_ahci_soc = {
    .supply_names = tegra124_supply_names,
    .num_supplies = ARRAY_SIZE(tegra124_supply_names),
    .supports_devslp = false,
    .has_sata_oob_rst = true,
    .ops = &tegra124_ahci_ops,
    .regs = &tegra124_ahci_regs,
    };
    static const struct tegra_ahci_soc tegra210_ahci_soc = {
    .supports_devslp = false,
    .has_sata_oob_rst = true,
    .regs = &tegra124_ahci_regs,
    };
    static const struct tegra_ahci_regs tegra186_ahci_regs = {
    .nvoob_comma_cnt_mask = GENMASK(23, 16),
    .nvoob_comma_cnt_val = (7 << 16),
    };
    static const struct tegra_ahci_soc tegra186_ahci_soc = {
    .supports_devslp = false,
    .has_sata_oob_rst = false,
    .regs = &tegra186_ahci_regs,
    };
    static const struct of_device_id tegra_ahci_of_match[] = {
    {
    .compatible = "nvidia,tegra124-ahci",
    .data = &tegra124_ahci_soc
    },
    {
    .compatible = "nvidia,tegra210-ahci",
    .data = &tegra210_ahci_soc
    },
    {
    .compatible = "nvidia,tegra186-ahci",
    .data = &tegra186_ahci_soc
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, tegra_ahci_of_match);
    static const struct scsi_host_template ahci_platform_sht = {
    AHCI_SHT(DRV_NAME),
    };
#[no_mangle]
unsafe extern "C" fn tegra_ahci_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_ahci_probe(struct platform_device *pdev)
    {
    struct ahci_host_priv *hpriv;
    struct tegra_ahci_priv *tegra;
    struct resource *res;
    int ret;
    hpriv = ahci_platform_get_resources(pdev, 0);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    tegra = devm_kzalloc(&pdev.dev, sizeof(*tegra), GFP_KERNEL);
    if (!tegra)
    return -ENOMEM;
    hpriv.plat_data = tegra;
    tegra.pdev = pdev;
    tegra.soc = of_device_get_match_data(&pdev.dev);
    tegra.sata_regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(tegra.sata_regs))
    return PTR_ERR(tegra.sata_regs);
//
// AUX registers is optional.
//
    res = platform_get_resource(pdev, IORESOURCE_MEM, 2);
    if (res) {
    tegra.sata_aux_regs = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(tegra.sata_aux_regs))
    return PTR_ERR(tegra.sata_aux_regs);
    }
    tegra.sata_rst = devm_reset_control_get(&pdev.dev, "sata");
    if (IS_ERR(tegra.sata_rst)) {
    dev_err(&pdev.dev, "Failed to get sata reset\n");
    return PTR_ERR(tegra.sata_rst);
    }
    if (tegra.soc.has_sata_oob_rst) {
    tegra.sata_oob_rst = devm_reset_control_get(&pdev.dev,
    "sata-oob");
    if (IS_ERR(tegra.sata_oob_rst)) {
    dev_err(&pdev.dev, "Failed to get sata-oob reset\n");
    return PTR_ERR(tegra.sata_oob_rst);
    }
    }
    tegra.sata_cold_rst = devm_reset_control_get(&pdev.dev, "sata-cold");
    if (IS_ERR(tegra.sata_cold_rst)) {
    dev_err(&pdev.dev, "Failed to get sata-cold reset\n");
    return PTR_ERR(tegra.sata_cold_rst);
    }
    tegra.sata_clk = devm_clk_get(&pdev.dev, "sata");
    if (IS_ERR(tegra.sata_clk)) {
    dev_err(&pdev.dev, "Failed to get sata clock\n");
    return PTR_ERR(tegra.sata_clk);
    }
    tegra.pmc = devm_tegra_pmc_get(&pdev.dev);
    if (IS_ERR(tegra.pmc))
    return dev_err_probe(&pdev.dev, PTR_ERR(tegra.pmc),
    "failed to get PMC\n");
    tegra.supplies = devm_kcalloc(&pdev.dev,
    tegra.soc.num_supplies,
    sizeof(*tegra.supplies), GFP_KERNEL);
    if (!tegra.supplies)
    return -ENOMEM;
    regulator_bulk_set_supply_names(tegra.supplies,
    tegra.soc.supply_names,
    tegra.soc.num_supplies);
    ret = devm_regulator_bulk_get(&pdev.dev,
    tegra.soc.num_supplies,
    tegra.supplies);
    if (ret) {
    dev_err(&pdev.dev, "Failed to get regulators\n");
    return ret;
    }
    ret = tegra_ahci_controller_init(hpriv);
    if (ret)
    return ret;
    ret = ahci_platform_init_host(pdev, hpriv, &ahci_tegra_port_info,
    &ahci_platform_sht);
    if (ret)
    goto deinit_controller;
    return 0;
    deinit_controller:
    tegra_ahci_controller_deinit(hpriv);
    return ret;
    };
    static struct platform_driver tegra_ahci_driver = {
    .probe = tegra_ahci_probe,
    .remove = ata_platform_remove_one,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = tegra_ahci_of_match,
    },
// LP0 suspend support not implemented
    };
    module_platform_driver(tegra_ahci_driver);
    MODULE_AUTHOR("Mikko Perttunen <mperttunen@nvidia.com>");
    MODULE_DESCRIPTION("Tegra AHCI SATA driver");
    MODULE_LICENSE("GPL v2");
