//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/imx/gpcv2.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2017 Impinj, Inc
// Author: Andrey Smirnov <andrew.smirnov@gmail.com>
//
// Based on the code of analogus driver:
//
// Copyright 2015-2017 Pengutronix, Lucas Stach <kernel@pengutronix.de>
//

pub const GPC_LPCR_A_CORE_BSC: c_uint = 0x000;
pub const GPC_PGC_CPU_MAPPING: c_uint = 0x0ec;
pub const IMX8MP_GPC_PGC_CPU_MAPPING: c_uint = 0x1cc;

pub const IMX8MP_GPC_PU_PGC_SW_PUP_REQ: c_uint = 0x0d8;
pub const IMX8MP_GPC_PU_PGC_SW_PDN_REQ: c_uint = 0x0e4;
pub const GPC_PU_PGC_SW_PUP_REQ: c_uint = 0x0f8;
pub const GPC_PU_PGC_SW_PDN_REQ: c_uint = 0x104;

pub const GPC_M4_PU_PDN_FLG: c_uint = 0x1bc;
pub const IMX8MP_GPC_PU_PWRHSK: c_uint = 0x190;
pub const GPC_PU_PWRHSK: c_uint = 0x1fc;

//
// The PGC offset values in Reference Manual
// (Rev. 1, 01/2018 and the older ones) GPC chapter's
// GPC_PGC memory map are incorrect, below offset
// values are from design RTL.
//
pub const IMX7_PGC_MIPI: c_int = 16;
pub const IMX7_PGC_PCIE: c_int = 17;
pub const IMX7_PGC_USB_HSIC: c_int = 20;
pub const IMX8M_PGC_MIPI: c_int = 16;
pub const IMX8M_PGC_PCIE1: c_int = 17;
pub const IMX8M_PGC_OTG1: c_int = 18;
pub const IMX8M_PGC_OTG2: c_int = 19;
pub const IMX8M_PGC_DDR1: c_int = 21;
pub const IMX8M_PGC_GPU: c_int = 23;
pub const IMX8M_PGC_VPU: c_int = 24;
pub const IMX8M_PGC_DISP: c_int = 26;
pub const IMX8M_PGC_MIPI_CSI1: c_int = 27;
pub const IMX8M_PGC_MIPI_CSI2: c_int = 28;
pub const IMX8M_PGC_PCIE2: c_int = 29;
pub const IMX8MM_PGC_MIPI: c_int = 16;
pub const IMX8MM_PGC_PCIE: c_int = 17;
pub const IMX8MM_PGC_OTG1: c_int = 18;
pub const IMX8MM_PGC_OTG2: c_int = 19;
pub const IMX8MM_PGC_DDR1: c_int = 21;
pub const IMX8MM_PGC_GPU2D: c_int = 22;
pub const IMX8MM_PGC_GPUMIX: c_int = 23;
pub const IMX8MM_PGC_VPUMIX: c_int = 24;
pub const IMX8MM_PGC_GPU3D: c_int = 25;
pub const IMX8MM_PGC_DISPMIX: c_int = 26;
pub const IMX8MM_PGC_VPUG1: c_int = 27;
pub const IMX8MM_PGC_VPUG2: c_int = 28;
pub const IMX8MM_PGC_VPUH1: c_int = 29;
pub const IMX8MN_PGC_MIPI: c_int = 16;
pub const IMX8MN_PGC_OTG1: c_int = 18;
pub const IMX8MN_PGC_DDR1: c_int = 21;
pub const IMX8MN_PGC_GPUMIX: c_int = 23;
pub const IMX8MN_PGC_DISPMIX: c_int = 26;
pub const IMX8MP_PGC_NOC: c_int = 9;
pub const IMX8MP_PGC_MIPI1: c_int = 12;
pub const IMX8MP_PGC_PCIE: c_int = 13;
pub const IMX8MP_PGC_USB1: c_int = 14;
pub const IMX8MP_PGC_USB2: c_int = 15;
pub const IMX8MP_PGC_MLMIX: c_int = 16;
pub const IMX8MP_PGC_AUDIOMIX: c_int = 17;
pub const IMX8MP_PGC_GPU2D: c_int = 18;
pub const IMX8MP_PGC_GPUMIX: c_int = 19;
pub const IMX8MP_PGC_VPUMIX: c_int = 20;
pub const IMX8MP_PGC_GPU3D: c_int = 21;
pub const IMX8MP_PGC_MEDIAMIX: c_int = 22;
pub const IMX8MP_PGC_VPU_G1: c_int = 23;
pub const IMX8MP_PGC_VPU_G2: c_int = 24;
pub const IMX8MP_PGC_VPU_VC8000E: c_int = 25;
pub const IMX8MP_PGC_HDMIMIX: c_int = 26;
pub const IMX8MP_PGC_HDMI: c_int = 27;
pub const IMX8MP_PGC_MIPI2: c_int = 28;
pub const IMX8MP_PGC_HSIOMIX: c_int = 29;
pub const IMX8MP_PGC_MEDIA_ISP_DWP: c_int = 30;
pub const IMX8MP_PGC_DDRMIX: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pgc_regs {
    pub map: u16,
    pub pup: u16,
    pub pdn: u16,
    pub hsk: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pgc_domain {
    pub genpd: generic_pm_domain,
    pub regmap: *mut regmap,
    pub regs: *const imx_pgc_regs,
    pub regulator: *mut regulator,
    pub reset: *mut reset_control,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub pgc: c_ulong,
    const struct {
    pub pxx: u32,
    pub map: u32,
    pub hskreq: u32,
    pub hskack: u32,
    pub bits: },
    pub voltage: c_int,
    pub keep_clocks: bool,
    pub dev: *mut device,
    pub pgc_sw_pup_reg: c_uint,
    pub pgc_sw_pdn_reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pgc_domain_data {
    pub domains: *const imx_pgc_domain,
    pub domains_num: usize,
    pub reg_access_table: *const regmap_access_table,
    pub pgc_regs: *const imx_pgc_regs,
}

    static inline struct imx_pgc_domain *
    to_imx_pgc_domain(struct generic_pm_domain *genpd)
    {
    return container_of(genpd, struct imx_pgc_domain, genpd);
    }
#[no_mangle]
unsafe extern "C" fn imx_pgc_power_up(genpd: *mut generic_pm_domain) -> c_int {
    static int imx_pgc_power_up(struct generic_pm_domain *genpd)
    {
    struct imx_pgc_domain *domain = to_imx_pgc_domain(genpd);
    u32 reg_val, pgc;
    int ret;
    ret = pm_runtime_get_sync(domain.dev);
    if (ret < 0) {
    pm_runtime_put_noidle(domain.dev);
    return ret;
    }
    if (!IS_ERR(domain.regulator)) {
    ret = regulator_enable(domain.regulator);
    if (ret) {
    dev_err(domain.dev,
    "failed to enable regulator: %pe\n",
    ERR_PTR(ret));
    goto out_put_pm;
    }
    }
    reset_control_assert(domain.reset);
// Enable reset clocks for all devices in the domain
    ret = clk_bulk_prepare_enable(domain.num_clks, domain.clks);
    if (ret) {
    dev_err(domain.dev, "failed to enable reset clocks\n");
    goto out_regulator_disable;
    }
// delays for reset to propagate
    udelay(5);
    if (domain.bits.pxx) {
// request the domain to power up
    regmap_update_bits(domain.regmap, domain.regs.pup,
    domain.bits.pxx, domain.bits.pxx);
//
// As per "5.5.9.4 Example Code 4" in IMX7DRM.pdf wait
// for PUP_REQ/PDN_REQ bit to be cleared
//
    ret = regmap_read_poll_timeout(domain.regmap,
    domain.regs.pup, reg_val,
    !(reg_val & domain.bits.pxx),
    0, USEC_PER_MSEC);
    if (ret) {
    dev_err(domain.dev, "failed to command PGC\n");
    goto out_clk_disable;
    }
// disable power control
    for_each_set_bit(pgc, &domain.pgc, 32) {
    regmap_clear_bits(domain.regmap, GPC_PGC_CTRL(pgc),
    GPC_PGC_CTRL_PCR);
    }
    }
// delay for reset to propagate
    udelay(5);
    reset_control_deassert(domain.reset);
// request the ADB400 to power up
    if (domain.bits.hskreq) {
    regmap_update_bits(domain.regmap, domain.regs.hsk,
    domain.bits.hskreq, domain.bits.hskreq);
//
// ret = regmap_read_poll_timeout(domain->regmap, domain->regs->hsk, reg_val,
// (reg_val & domain->bits.hskack), 0,
// USEC_PER_MSEC);
// Technically we need the commented code to wait handshake. But that needs
// the BLK-CTL module BUS clk-en bit being set.
//
// There is a separate BLK-CTL module and we will have such a driver for it,
// that driver will set the BUS clk-en bit and handshake will be triggered
// automatically there. Just add a delay and suppose the handshake finish
// after that.
//
// For some BLK-CTL module (eg. AudioMix on i.MX8MP) doesn't have BUS
// clk-en bit, it is better to add delay here, as the BLK-CTL module
// doesn't need to care about how it is powered up.
//
// regmap_read_bypassed() is to make sure the above write IO transaction
// already reaches target before udelay()
//
    regmap_read_bypassed(domain.regmap, domain.regs.hsk, &reg_val);
    udelay(10);
    }
// Disable reset clocks for all devices in the domain
    if (!domain.keep_clocks)
    clk_bulk_disable_unprepare(domain.num_clks, domain.clks);
    return 0;
    out_clk_disable:
    clk_bulk_disable_unprepare(domain.num_clks, domain.clks);
    out_regulator_disable:
    if (!IS_ERR(domain.regulator))
    regulator_disable(domain.regulator);
    out_put_pm:
    pm_runtime_put(domain.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_pgc_power_down(genpd: *mut generic_pm_domain) -> c_int {
    static int imx_pgc_power_down(struct generic_pm_domain *genpd)
    {
    struct imx_pgc_domain *domain = to_imx_pgc_domain(genpd);
    u32 reg_val, pgc;
    int ret;
// Enable reset clocks for all devices in the domain
    if (!domain.keep_clocks) {
    ret = clk_bulk_prepare_enable(domain.num_clks, domain.clks);
    if (ret) {
    dev_err(domain.dev, "failed to enable reset clocks\n");
    return ret;
    }
    }
// request the ADB400 to power down
    if (domain.bits.hskreq) {
    regmap_clear_bits(domain.regmap, domain.regs.hsk,
    domain.bits.hskreq);
    ret = regmap_read_poll_timeout(domain.regmap, domain.regs.hsk,
    reg_val,
    !(reg_val & domain.bits.hskack),
    0, USEC_PER_MSEC);
    if (ret) {
    dev_err(domain.dev, "failed to power down ADB400\n");
    goto out_clk_disable;
    }
    }
    if (domain.bits.pxx) {
// enable power control
    for_each_set_bit(pgc, &domain.pgc, 32) {
    regmap_update_bits(domain.regmap, GPC_PGC_CTRL(pgc),
    GPC_PGC_CTRL_PCR, GPC_PGC_CTRL_PCR);
    }
// request the domain to power down
    regmap_update_bits(domain.regmap, domain.regs.pdn,
    domain.bits.pxx, domain.bits.pxx);
//
// As per "5.5.9.4 Example Code 4" in IMX7DRM.pdf wait
// for PUP_REQ/PDN_REQ bit to be cleared
//
    ret = regmap_read_poll_timeout(domain.regmap,
    domain.regs.pdn, reg_val,
    !(reg_val & domain.bits.pxx),
    0, USEC_PER_MSEC);
    if (ret) {
    dev_err(domain.dev, "failed to command PGC\n");
    goto out_clk_disable;
    }
    }
// Disable reset clocks for all devices in the domain
    clk_bulk_disable_unprepare(domain.num_clks, domain.clks);
    if (!IS_ERR(domain.regulator)) {
    ret = regulator_disable(domain.regulator);
    if (ret) {
    dev_err(domain.dev,
    "failed to disable regulator: %pe\n",
    ERR_PTR(ret));
    return ret;
    }
    }
    pm_runtime_put_sync_suspend(domain.dev);
    return 0;
    out_clk_disable:
    if (!domain.keep_clocks)
    clk_bulk_disable_unprepare(domain.num_clks, domain.clks);
    return ret;
    }
    static const struct imx_pgc_domain imx7_pgc_domains[] = {
    [IMX7_POWER_DOMAIN_MIPI_PHY] = {
    .genpd = {
    .name      = "mipi-phy",
    },
    .bits  = {
    .pxx = IMX7_MIPI_PHY_SW_Pxx_REQ,
    .map = IMX7_MIPI_PHY_A_CORE_DOMAIN,
    },
    .voltage   = 1000000,
    .pgc	   = BIT(IMX7_PGC_MIPI),
    },
    [IMX7_POWER_DOMAIN_PCIE_PHY] = {
    .genpd = {
    .name      = "pcie-phy",
    },
    .bits  = {
    .pxx = IMX7_PCIE_PHY_SW_Pxx_REQ,
    .map = IMX7_PCIE_PHY_A_CORE_DOMAIN,
    },
    .voltage   = 1000000,
    .pgc	   = BIT(IMX7_PGC_PCIE),
    },
    [IMX7_POWER_DOMAIN_USB_HSIC_PHY] = {
    .genpd = {
    .name      = "usb-hsic-phy",
    },
    .bits  = {
    .pxx = IMX7_USB_HSIC_PHY_SW_Pxx_REQ,
    .map = IMX7_USB_HSIC_PHY_A_CORE_DOMAIN,
    },
    .voltage   = 1200000,
    .pgc	   = BIT(IMX7_PGC_USB_HSIC),
    },
    };
    static const struct regmap_range imx7_yes_ranges[] = {
    regmap_reg_range(GPC_LPCR_A_CORE_BSC,
    GPC_M4_PU_PDN_FLG),
    regmap_reg_range(GPC_PGC_CTRL(IMX7_PGC_MIPI),
    GPC_PGC_SR(IMX7_PGC_MIPI)),
    regmap_reg_range(GPC_PGC_CTRL(IMX7_PGC_PCIE),
    GPC_PGC_SR(IMX7_PGC_PCIE)),
    regmap_reg_range(GPC_PGC_CTRL(IMX7_PGC_USB_HSIC),
    GPC_PGC_SR(IMX7_PGC_USB_HSIC)),
    };
    static const struct regmap_access_table imx7_access_table = {
    .yes_ranges	= imx7_yes_ranges,
    .n_yes_ranges	= ARRAY_SIZE(imx7_yes_ranges),
    };
    static const struct imx_pgc_regs imx7_pgc_regs = {
    .map = GPC_PGC_CPU_MAPPING,
    .pup = GPC_PU_PGC_SW_PUP_REQ,
    .pdn = GPC_PU_PGC_SW_PDN_REQ,
    .hsk = GPC_PU_PWRHSK,
    };
    static const struct imx_pgc_domain_data imx7_pgc_domain_data = {
    .domains = imx7_pgc_domains,
    .domains_num = ARRAY_SIZE(imx7_pgc_domains),
    .reg_access_table = &imx7_access_table,
    .pgc_regs = &imx7_pgc_regs,
    };
    static const struct imx_pgc_domain imx8m_pgc_domains[] = {
    [IMX8M_POWER_DOMAIN_MIPI] = {
    .genpd = {
    .name      = "mipi",
    },
    .bits  = {
    .pxx = IMX8M_MIPI_SW_Pxx_REQ,
    .map = IMX8M_MIPI_A53_DOMAIN,
    },
    .pgc	   = BIT(IMX8M_PGC_MIPI),
    },
    [IMX8M_POWER_DOMAIN_PCIE1] = {
    .genpd = {
    .name = "pcie1",
    },
    .bits  = {
    .pxx = IMX8M_PCIE1_SW_Pxx_REQ,
    .map = IMX8M_PCIE1_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8M_PGC_PCIE1),
    },
    [IMX8M_POWER_DOMAIN_USB_OTG1] = {
    .genpd = {
    .name = "usb-otg1",
    },
    .bits  = {
    .pxx = IMX8M_OTG1_SW_Pxx_REQ,
    .map = IMX8M_OTG1_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8M_PGC_OTG1),
    },
    [IMX8M_POWER_DOMAIN_USB_OTG2] = {
    .genpd = {
    .name = "usb-otg2",
    },
    .bits  = {
    .pxx = IMX8M_OTG2_SW_Pxx_REQ,
    .map = IMX8M_OTG2_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8M_PGC_OTG2),
    },
    [IMX8M_POWER_DOMAIN_DDR1] = {
    .genpd = {
    .name = "ddr1",
    },
    .bits  = {
    .pxx = IMX8M_DDR1_SW_Pxx_REQ,
    .map = IMX8M_DDR2_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8M_PGC_DDR1),
    },
    [IMX8M_POWER_DOMAIN_GPU] = {
    .genpd = {
    .name = "gpu",
    },
    .bits  = {
    .pxx = IMX8M_GPU_SW_Pxx_REQ,
    .map = IMX8M_GPU_A53_DOMAIN,
    .hskreq = IMX8M_GPU_HSK_PWRDNREQN,
    .hskack = IMX8M_GPU_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8M_PGC_GPU),
    },
    [IMX8M_POWER_DOMAIN_VPU] = {
    .genpd = {
    .name = "vpu",
    },
    .bits  = {
    .pxx = IMX8M_VPU_SW_Pxx_REQ,
    .map = IMX8M_VPU_A53_DOMAIN,
    .hskreq = IMX8M_VPU_HSK_PWRDNREQN,
    .hskack = IMX8M_VPU_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8M_PGC_VPU),
    .keep_clocks = true,
    },
    [IMX8M_POWER_DOMAIN_DISP] = {
    .genpd = {
    .name = "disp",
    },
    .bits  = {
    .pxx = IMX8M_DISP_SW_Pxx_REQ,
    .map = IMX8M_DISP_A53_DOMAIN,
    .hskreq = IMX8M_DISP_HSK_PWRDNREQN,
    .hskack = IMX8M_DISP_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8M_PGC_DISP),
    },
    [IMX8M_POWER_DOMAIN_MIPI_CSI1] = {
    .genpd = {
    .name = "mipi-csi1",
    },
    .bits  = {
    .pxx = IMX8M_MIPI_CSI1_SW_Pxx_REQ,
    .map = IMX8M_MIPI_CSI1_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8M_PGC_MIPI_CSI1),
    },
    [IMX8M_POWER_DOMAIN_MIPI_CSI2] = {
    .genpd = {
    .name = "mipi-csi2",
    },
    .bits  = {
    .pxx = IMX8M_MIPI_CSI2_SW_Pxx_REQ,
    .map = IMX8M_MIPI_CSI2_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8M_PGC_MIPI_CSI2),
    },
    [IMX8M_POWER_DOMAIN_PCIE2] = {
    .genpd = {
    .name = "pcie2",
    },
    .bits  = {
    .pxx = IMX8M_PCIE2_SW_Pxx_REQ,
    .map = IMX8M_PCIE2_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8M_PGC_PCIE2),
    },
    };
    static const struct regmap_range imx8m_yes_ranges[] = {
    regmap_reg_range(GPC_LPCR_A_CORE_BSC,
    GPC_PU_PWRHSK),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_MIPI),
    GPC_PGC_SR(IMX8M_PGC_MIPI)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_PCIE1),
    GPC_PGC_SR(IMX8M_PGC_PCIE1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_OTG1),
    GPC_PGC_SR(IMX8M_PGC_OTG1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_OTG2),
    GPC_PGC_SR(IMX8M_PGC_OTG2)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_DDR1),
    GPC_PGC_SR(IMX8M_PGC_DDR1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_GPU),
    GPC_PGC_SR(IMX8M_PGC_GPU)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_VPU),
    GPC_PGC_SR(IMX8M_PGC_VPU)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_DISP),
    GPC_PGC_SR(IMX8M_PGC_DISP)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_MIPI_CSI1),
    GPC_PGC_SR(IMX8M_PGC_MIPI_CSI1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_MIPI_CSI2),
    GPC_PGC_SR(IMX8M_PGC_MIPI_CSI2)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8M_PGC_PCIE2),
    GPC_PGC_SR(IMX8M_PGC_PCIE2)),
    };
    static const struct regmap_access_table imx8m_access_table = {
    .yes_ranges	= imx8m_yes_ranges,
    .n_yes_ranges	= ARRAY_SIZE(imx8m_yes_ranges),
    };
    static const struct imx_pgc_domain_data imx8m_pgc_domain_data = {
    .domains = imx8m_pgc_domains,
    .domains_num = ARRAY_SIZE(imx8m_pgc_domains),
    .reg_access_table = &imx8m_access_table,
    .pgc_regs = &imx7_pgc_regs,
    };
    static const struct imx_pgc_domain imx8mm_pgc_domains[] = {
    [IMX8MM_POWER_DOMAIN_HSIOMIX] = {
    .genpd = {
    .name = "hsiomix",
    },
    .bits  = {
    .pxx = 0, /* no power sequence control */
    .map = 0, /* no power sequence control */
    .hskreq = IMX8MM_HSIO_HSK_PWRDNREQN,
    .hskack = IMX8MM_HSIO_HSK_PWRDNACKN,
    },
    .keep_clocks = true,
    },
    [IMX8MM_POWER_DOMAIN_PCIE] = {
    .genpd = {
    .name = "pcie",
    },
    .bits  = {
    .pxx = IMX8MM_PCIE_SW_Pxx_REQ,
    .map = IMX8MM_PCIE_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_PCIE),
    },
    [IMX8MM_POWER_DOMAIN_OTG1] = {
    .genpd = {
    .name = "usb-otg1",
    .flags = GENPD_FLAG_ACTIVE_WAKEUP,
    },
    .bits  = {
    .pxx = IMX8MM_OTG1_SW_Pxx_REQ,
    .map = IMX8MM_OTG1_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_OTG1),
    },
    [IMX8MM_POWER_DOMAIN_OTG2] = {
    .genpd = {
    .name = "usb-otg2",
    .flags = GENPD_FLAG_ACTIVE_WAKEUP,
    },
    .bits  = {
    .pxx = IMX8MM_OTG2_SW_Pxx_REQ,
    .map = IMX8MM_OTG2_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_OTG2),
    },
    [IMX8MM_POWER_DOMAIN_GPUMIX] = {
    .genpd = {
    .name = "gpumix",
    },
    .bits  = {
    .pxx = IMX8MM_GPUMIX_SW_Pxx_REQ,
    .map = IMX8MM_GPUMIX_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_GPUMIX),
    .keep_clocks = true,
    },
    [IMX8MM_POWER_DOMAIN_GPU] = {
    .genpd = {
    .name = "gpu",
    },
    .bits  = {
    .pxx = IMX8MM_GPU_SW_Pxx_REQ,
    .map = IMX8MM_GPU_A53_DOMAIN,
    .hskreq = IMX8MM_GPU_HSK_PWRDNREQN,
    .hskack = IMX8MM_GPU_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8MM_PGC_GPU2D) | BIT(IMX8MM_PGC_GPU3D),
    },
    [IMX8MM_POWER_DOMAIN_VPUMIX] = {
    .genpd = {
    .name = "vpumix",
    },
    .bits  = {
    .pxx = IMX8MM_VPUMIX_SW_Pxx_REQ,
    .map = IMX8MM_VPUMIX_A53_DOMAIN,
    .hskreq = IMX8MM_VPUMIX_HSK_PWRDNREQN,
    .hskack = IMX8MM_VPUMIX_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8MM_PGC_VPUMIX),
    .keep_clocks = true,
    },
    [IMX8MM_POWER_DOMAIN_VPUG1] = {
    .genpd = {
    .name = "vpu-g1",
    },
    .bits  = {
    .pxx = IMX8MM_VPUG1_SW_Pxx_REQ,
    .map = IMX8MM_VPUG1_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_VPUG1),
    },
    [IMX8MM_POWER_DOMAIN_VPUG2] = {
    .genpd = {
    .name = "vpu-g2",
    },
    .bits  = {
    .pxx = IMX8MM_VPUG2_SW_Pxx_REQ,
    .map = IMX8MM_VPUG2_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_VPUG2),
    },
    [IMX8MM_POWER_DOMAIN_VPUH1] = {
    .genpd = {
    .name = "vpu-h1",
    },
    .bits  = {
    .pxx = IMX8MM_VPUH1_SW_Pxx_REQ,
    .map = IMX8MM_VPUH1_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_VPUH1),
    .keep_clocks = true,
    },
    [IMX8MM_POWER_DOMAIN_DISPMIX] = {
    .genpd = {
    .name = "dispmix",
    },
    .bits  = {
    .pxx = IMX8MM_DISPMIX_SW_Pxx_REQ,
    .map = IMX8MM_DISPMIX_A53_DOMAIN,
    .hskreq = IMX8MM_DISPMIX_HSK_PWRDNREQN,
    .hskack = IMX8MM_DISPMIX_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8MM_PGC_DISPMIX),
    .keep_clocks = true,
    },
    [IMX8MM_POWER_DOMAIN_MIPI] = {
    .genpd = {
    .name = "mipi",
    },
    .bits  = {
    .pxx = IMX8MM_MIPI_SW_Pxx_REQ,
    .map = IMX8MM_MIPI_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MM_PGC_MIPI),
    },
    };
    static const struct regmap_range imx8mm_yes_ranges[] = {
    regmap_reg_range(GPC_LPCR_A_CORE_BSC,
    GPC_PU_PWRHSK),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_MIPI),
    GPC_PGC_SR(IMX8MM_PGC_MIPI)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_PCIE),
    GPC_PGC_SR(IMX8MM_PGC_PCIE)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_OTG1),
    GPC_PGC_SR(IMX8MM_PGC_OTG1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_OTG2),
    GPC_PGC_SR(IMX8MM_PGC_OTG2)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_DDR1),
    GPC_PGC_SR(IMX8MM_PGC_DDR1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_GPU2D),
    GPC_PGC_SR(IMX8MM_PGC_GPU2D)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_GPUMIX),
    GPC_PGC_SR(IMX8MM_PGC_GPUMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_VPUMIX),
    GPC_PGC_SR(IMX8MM_PGC_VPUMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_GPU3D),
    GPC_PGC_SR(IMX8MM_PGC_GPU3D)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_DISPMIX),
    GPC_PGC_SR(IMX8MM_PGC_DISPMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_VPUG1),
    GPC_PGC_SR(IMX8MM_PGC_VPUG1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_VPUG2),
    GPC_PGC_SR(IMX8MM_PGC_VPUG2)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MM_PGC_VPUH1),
    GPC_PGC_SR(IMX8MM_PGC_VPUH1)),
    };
    static const struct regmap_access_table imx8mm_access_table = {
    .yes_ranges	= imx8mm_yes_ranges,
    .n_yes_ranges	= ARRAY_SIZE(imx8mm_yes_ranges),
    };
    static const struct imx_pgc_domain_data imx8mm_pgc_domain_data = {
    .domains = imx8mm_pgc_domains,
    .domains_num = ARRAY_SIZE(imx8mm_pgc_domains),
    .reg_access_table = &imx8mm_access_table,
    .pgc_regs = &imx7_pgc_regs,
    };
    static const struct imx_pgc_domain imx8mp_pgc_domains[] = {
    [IMX8MP_POWER_DOMAIN_MIPI_PHY1] = {
    .genpd = {
    .name = "mipi-phy1",
    },
    .bits = {
    .pxx = IMX8MP_MIPI_PHY1_SW_Pxx_REQ,
    .map = IMX8MP_MIPI_PHY1_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_MIPI1),
    },
    [IMX8MP_POWER_DOMAIN_PCIE_PHY] = {
    .genpd = {
    .name = "pcie-phy1",
    },
    .bits = {
    .pxx = IMX8MP_PCIE_PHY_SW_Pxx_REQ,
    .map = IMX8MP_PCIE_PHY_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_PCIE),
    },
    [IMX8MP_POWER_DOMAIN_USB1_PHY] = {
    .genpd = {
    .name = "usb-otg1",
    },
    .bits = {
    .pxx = IMX8MP_USB1_PHY_Pxx_REQ,
    .map = IMX8MP_USB1_PHY_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_USB1),
    },
    [IMX8MP_POWER_DOMAIN_USB2_PHY] = {
    .genpd = {
    .name = "usb-otg2",
    },
    .bits = {
    .pxx = IMX8MP_USB2_PHY_Pxx_REQ,
    .map = IMX8MP_USB2_PHY_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_USB2),
    },
    [IMX8MP_POWER_DOMAIN_MLMIX] = {
    .genpd = {
    .name = "mlmix",
    },
    .bits = {
    .pxx = IMX8MP_MLMIX_Pxx_REQ,
    .map = IMX8MP_MLMIX_A53_DOMAIN,
    .hskreq = IMX8MP_MLMIX_PWRDNREQN,
    .hskack = IMX8MP_MLMIX_PWRDNACKN,
    },
    .pgc = BIT(IMX8MP_PGC_MLMIX),
    .keep_clocks = true,
    },
    [IMX8MP_POWER_DOMAIN_AUDIOMIX] = {
    .genpd = {
    .name = "audiomix",
    },
    .bits = {
    .pxx = IMX8MP_AUDIOMIX_Pxx_REQ,
    .map = IMX8MP_AUDIOMIX_A53_DOMAIN,
    .hskreq = IMX8MP_AUDIOMIX_PWRDNREQN,
    .hskack = IMX8MP_AUDIOMIX_PWRDNACKN,
    },
    .pgc = BIT(IMX8MP_PGC_AUDIOMIX),
    .keep_clocks = true,
    },
    [IMX8MP_POWER_DOMAIN_GPU2D] = {
    .genpd = {
    .name = "gpu2d",
    },
    .bits = {
    .pxx = IMX8MP_GPU_2D_Pxx_REQ,
    .map = IMX8MP_GPU2D_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_GPU2D),
    },
    [IMX8MP_POWER_DOMAIN_GPUMIX] = {
    .genpd = {
    .name = "gpumix",
    },
    .bits = {
    .pxx = IMX8MP_GPU_SHARE_LOGIC_Pxx_REQ,
    .map = IMX8MP_GPUMIX_A53_DOMAIN,
    .hskreq = IMX8MP_GPUMIX_PWRDNREQN,
    .hskack = IMX8MP_GPUMIX_PWRDNACKN,
    },
    .pgc = BIT(IMX8MP_PGC_GPUMIX),
    .keep_clocks = true,
    },
    [IMX8MP_POWER_DOMAIN_VPUMIX] = {
    .genpd = {
    .name = "vpumix",
    },
    .bits = {
    .pxx = IMX8MP_VPU_MIX_SHARE_LOGIC_Pxx_REQ,
    .map = IMX8MP_VPUMIX_A53_DOMAIN,
    .hskreq = IMX8MP_VPUMIX_PWRDNREQN,
    .hskack = IMX8MP_VPUMIX_PWRDNACKN,
    },
    .pgc = BIT(IMX8MP_PGC_VPUMIX),
    .keep_clocks = true,
    },
    [IMX8MP_POWER_DOMAIN_GPU3D] = {
    .genpd = {
    .name = "gpu3d",
    },
    .bits = {
    .pxx = IMX8MP_GPU_3D_Pxx_REQ,
    .map = IMX8MP_GPU3D_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_GPU3D),
    },
    [IMX8MP_POWER_DOMAIN_MEDIAMIX] = {
    .genpd = {
    .name = "mediamix",
    },
    .bits = {
    .pxx = IMX8MP_MEDIMIX_Pxx_REQ,
    .map = IMX8MP_MEDIAMIX_A53_DOMAIN,
    .hskreq = IMX8MP_MEDIAMIX_PWRDNREQN,
    .hskack = IMX8MP_MEDIAMIX_PWRDNACKN,
    },
    .pgc = BIT(IMX8MP_PGC_MEDIAMIX),
    .keep_clocks = true,
    },
    [IMX8MP_POWER_DOMAIN_VPU_G1] = {
    .genpd = {
    .name = "vpu-g1",
    },
    .bits = {
    .pxx = IMX8MP_VPU_G1_Pxx_REQ,
    .map = IMX8MP_VPU_G1_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_VPU_G1),
    },
    [IMX8MP_POWER_DOMAIN_VPU_G2] = {
    .genpd = {
    .name = "vpu-g2",
    },
    .bits = {
    .pxx = IMX8MP_VPU_G2_Pxx_REQ,
    .map = IMX8MP_VPU_G2_A53_DOMAIN
    },
    .pgc = BIT(IMX8MP_PGC_VPU_G2),
    },
    [IMX8MP_POWER_DOMAIN_VPU_VC8000E] = {
    .genpd = {
    .name = "vpu-h1",
    },
    .bits = {
    .pxx = IMX8MP_VPU_VC8K_Pxx_REQ,
    .map = IMX8MP_VPU_VC8000E_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_VPU_VC8000E),
    },
    [IMX8MP_POWER_DOMAIN_HDMIMIX] = {
    .genpd = {
    .name = "hdmimix",
    },
    .bits = {
    .pxx = IMX8MP_HDMIMIX_Pxx_REQ,
    .map = IMX8MP_HDMIMIX_A53_DOMAIN,
    .hskreq = IMX8MP_HDMIMIX_PWRDNREQN,
    .hskack = IMX8MP_HDMIMIX_PWRDNACKN,
    },
    .pgc = BIT(IMX8MP_PGC_HDMIMIX),
    .keep_clocks = true,
    },
    [IMX8MP_POWER_DOMAIN_HDMI_PHY] = {
    .genpd = {
    .name = "hdmi-phy",
    },
    .bits = {
    .pxx = IMX8MP_HDMI_PHY_Pxx_REQ,
    .map = IMX8MP_HDMI_PHY_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_HDMI),
    },
    [IMX8MP_POWER_DOMAIN_MIPI_PHY2] = {
    .genpd = {
    .name = "mipi-phy2",
    },
    .bits = {
    .pxx = IMX8MP_MIPI_PHY2_Pxx_REQ,
    .map = IMX8MP_MIPI_PHY2_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_MIPI2),
    },
    [IMX8MP_POWER_DOMAIN_HSIOMIX] = {
    .genpd = {
    .name = "hsiomix",
    },
    .bits = {
    .pxx = IMX8MP_HSIOMIX_Pxx_REQ,
    .map = IMX8MP_HSIOMIX_A53_DOMAIN,
    .hskreq = IMX8MP_HSIOMIX_PWRDNREQN,
    .hskack = IMX8MP_HSIOMIX_PWRDNACKN,
    },
    .pgc = BIT(IMX8MP_PGC_HSIOMIX),
    .keep_clocks = true,
    },
    [IMX8MP_POWER_DOMAIN_MEDIAMIX_ISPDWP] = {
    .genpd = {
    .name = "mediamix-isp-dwp",
    },
    .bits = {
    .pxx = IMX8MP_MEDIA_ISP_DWP_Pxx_REQ,
    .map = IMX8MP_MEDIA_ISPDWP_A53_DOMAIN,
    },
    .pgc = BIT(IMX8MP_PGC_MEDIA_ISP_DWP),
    },
    };
    static const struct regmap_range imx8mp_yes_ranges[] = {
    regmap_reg_range(GPC_LPCR_A_CORE_BSC,
    IMX8MP_GPC_PGC_CPU_MAPPING),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_NOC),
    GPC_PGC_SR(IMX8MP_PGC_NOC)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_MIPI1),
    GPC_PGC_SR(IMX8MP_PGC_MIPI1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_PCIE),
    GPC_PGC_SR(IMX8MP_PGC_PCIE)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_USB1),
    GPC_PGC_SR(IMX8MP_PGC_USB1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_USB2),
    GPC_PGC_SR(IMX8MP_PGC_USB2)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_MLMIX),
    GPC_PGC_SR(IMX8MP_PGC_MLMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_AUDIOMIX),
    GPC_PGC_SR(IMX8MP_PGC_AUDIOMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_GPU2D),
    GPC_PGC_SR(IMX8MP_PGC_GPU2D)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_GPUMIX),
    GPC_PGC_SR(IMX8MP_PGC_GPUMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_VPUMIX),
    GPC_PGC_SR(IMX8MP_PGC_VPUMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_GPU3D),
    GPC_PGC_SR(IMX8MP_PGC_GPU3D)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_MEDIAMIX),
    GPC_PGC_SR(IMX8MP_PGC_MEDIAMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_VPU_G1),
    GPC_PGC_SR(IMX8MP_PGC_VPU_G1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_VPU_G2),
    GPC_PGC_SR(IMX8MP_PGC_VPU_G2)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_VPU_VC8000E),
    GPC_PGC_SR(IMX8MP_PGC_VPU_VC8000E)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_HDMIMIX),
    GPC_PGC_SR(IMX8MP_PGC_HDMIMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_HDMI),
    GPC_PGC_SR(IMX8MP_PGC_HDMI)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_MIPI2),
    GPC_PGC_SR(IMX8MP_PGC_MIPI2)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_HSIOMIX),
    GPC_PGC_SR(IMX8MP_PGC_HSIOMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_MEDIA_ISP_DWP),
    GPC_PGC_SR(IMX8MP_PGC_MEDIA_ISP_DWP)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MP_PGC_DDRMIX),
    GPC_PGC_SR(IMX8MP_PGC_DDRMIX)),
    };
    static const struct regmap_access_table imx8mp_access_table = {
    .yes_ranges	= imx8mp_yes_ranges,
    .n_yes_ranges	= ARRAY_SIZE(imx8mp_yes_ranges),
    };
    static const struct imx_pgc_regs imx8mp_pgc_regs = {
    .map = IMX8MP_GPC_PGC_CPU_MAPPING,
    .pup = IMX8MP_GPC_PU_PGC_SW_PUP_REQ,
    .pdn = IMX8MP_GPC_PU_PGC_SW_PDN_REQ,
    .hsk = IMX8MP_GPC_PU_PWRHSK,
    };
    static const struct imx_pgc_domain_data imx8mp_pgc_domain_data = {
    .domains = imx8mp_pgc_domains,
    .domains_num = ARRAY_SIZE(imx8mp_pgc_domains),
    .reg_access_table = &imx8mp_access_table,
    .pgc_regs = &imx8mp_pgc_regs,
    };
    static const struct imx_pgc_domain imx8mn_pgc_domains[] = {
    [IMX8MN_POWER_DOMAIN_HSIOMIX] = {
    .genpd = {
    .name = "hsiomix",
    },
    .bits  = {
    .pxx = 0, /* no power sequence control */
    .map = 0, /* no power sequence control */
    .hskreq = IMX8MN_HSIO_HSK_PWRDNREQN,
    .hskack = IMX8MN_HSIO_HSK_PWRDNACKN,
    },
    .keep_clocks = true,
    },
    [IMX8MN_POWER_DOMAIN_OTG1] = {
    .genpd = {
    .name = "usb-otg1",
    .flags = GENPD_FLAG_ACTIVE_WAKEUP,
    },
    .bits  = {
    .pxx = IMX8MN_OTG1_SW_Pxx_REQ,
    .map = IMX8MN_OTG1_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MN_PGC_OTG1),
    },
    [IMX8MN_POWER_DOMAIN_GPUMIX] = {
    .genpd = {
    .name = "gpumix",
    },
    .bits  = {
    .pxx = IMX8MN_GPUMIX_SW_Pxx_REQ,
    .map = IMX8MN_GPUMIX_A53_DOMAIN,
    .hskreq = IMX8MN_GPUMIX_HSK_PWRDNREQN,
    .hskack = IMX8MN_GPUMIX_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8MN_PGC_GPUMIX),
    .keep_clocks = true,
    },
    [IMX8MN_POWER_DOMAIN_DISPMIX] = {
    .genpd = {
    .name = "dispmix",
    },
    .bits  = {
    .pxx = IMX8MN_DISPMIX_SW_Pxx_REQ,
    .map = IMX8MN_DISPMIX_A53_DOMAIN,
    .hskreq = IMX8MN_DISPMIX_HSK_PWRDNREQN,
    .hskack = IMX8MN_DISPMIX_HSK_PWRDNACKN,
    },
    .pgc   = BIT(IMX8MN_PGC_DISPMIX),
    .keep_clocks = true,
    },
    [IMX8MN_POWER_DOMAIN_MIPI] = {
    .genpd = {
    .name = "mipi",
    },
    .bits  = {
    .pxx = IMX8MN_MIPI_SW_Pxx_REQ,
    .map = IMX8MN_MIPI_A53_DOMAIN,
    },
    .pgc   = BIT(IMX8MN_PGC_MIPI),
    },
    };
    static const struct regmap_range imx8mn_yes_ranges[] = {
    regmap_reg_range(GPC_LPCR_A_CORE_BSC,
    GPC_PU_PWRHSK),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MN_PGC_MIPI),
    GPC_PGC_SR(IMX8MN_PGC_MIPI)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MN_PGC_OTG1),
    GPC_PGC_SR(IMX8MN_PGC_OTG1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MN_PGC_DDR1),
    GPC_PGC_SR(IMX8MN_PGC_DDR1)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MN_PGC_GPUMIX),
    GPC_PGC_SR(IMX8MN_PGC_GPUMIX)),
    regmap_reg_range(GPC_PGC_CTRL(IMX8MN_PGC_DISPMIX),
    GPC_PGC_SR(IMX8MN_PGC_DISPMIX)),
    };
    static const struct regmap_access_table imx8mn_access_table = {
    .yes_ranges	= imx8mn_yes_ranges,
    .n_yes_ranges	= ARRAY_SIZE(imx8mn_yes_ranges),
    };
    static const struct imx_pgc_domain_data imx8mn_pgc_domain_data = {
    .domains = imx8mn_pgc_domains,
    .domains_num = ARRAY_SIZE(imx8mn_pgc_domains),
    .reg_access_table = &imx8mn_access_table,
    .pgc_regs = &imx7_pgc_regs,
    };
#[no_mangle]
unsafe extern "C" fn imx_pgc_domain_probe(pdev: *mut platform_device) -> c_int {
    static int imx_pgc_domain_probe(struct platform_device *pdev)
    {
    struct imx_pgc_domain *domain = pdev.dev.platform_data;
    int ret;
    domain.dev = &pdev.dev;
    domain.regulator = devm_regulator_get_optional(domain.dev, "power");
    if (IS_ERR(domain.regulator)) {
    if (PTR_ERR(domain.regulator) != -ENODEV)
    return dev_err_probe(domain.dev, PTR_ERR(domain.regulator),
    "Failed to get domain's regulator\n");
    } else if (domain.voltage) {
    regulator_set_voltage(domain.regulator,
    domain.voltage, domain.voltage);
    }
    domain.num_clks = devm_clk_bulk_get_all(domain.dev, &domain.clks);
    if (domain.num_clks < 0)
    return dev_err_probe(domain.dev, domain.num_clks,
    "Failed to get domain's clocks\n");
    domain.reset = devm_reset_control_array_get_optional_exclusive(domain.dev);
    if (IS_ERR(domain.reset))
    return dev_err_probe(domain.dev, PTR_ERR(domain.reset),
    "Failed to get domain's resets\n");
    pm_runtime_enable(domain.dev);
    if (domain.bits.map)
    regmap_update_bits(domain.regmap, domain.regs.map,
    domain.bits.map, domain.bits.map);
    ret = pm_genpd_init(&domain.genpd, core::ptr::null_mut(), true);
    if (ret) {
    dev_err_probe(domain.dev, ret, "Failed to init power domain\n");
    goto out_domain_unmap;
    }
    if (IS_ENABLED(CONFIG_LOCKDEP) &&
    of_property_present(domain.dev.of_node, "power-domains"))
    lockdep_set_subclass(&domain.genpd.mlock, 1);
    ret = of_genpd_add_provider_simple(domain.dev.of_node,
    &domain.genpd);
    if (ret) {
    dev_err_probe(domain.dev, ret, "Failed to add genpd provider\n");
    goto out_genpd_remove;
    }
    return 0;
    out_genpd_remove:
    pm_genpd_remove(&domain.genpd);
    out_domain_unmap:
    if (domain.bits.map)
    regmap_update_bits(domain.regmap, domain.regs.map,
    domain.bits.map, 0);
    pm_runtime_disable(domain.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_pgc_domain_remove(pdev: *mut platform_device) {
    static void imx_pgc_domain_remove(struct platform_device *pdev)
    {
    struct imx_pgc_domain *domain = pdev.dev.platform_data;
    of_genpd_del_provider(domain.dev.of_node);
    pm_genpd_remove(&domain.genpd);
    if (domain.bits.map)
    regmap_update_bits(domain.regmap, domain.regs.map,
    domain.bits.map, 0);
    pm_runtime_disable(domain.dev);
    }

#[no_mangle]
unsafe extern "C" fn imx_pgc_domain_suspend(dev: *mut device) -> c_int {
    static int imx_pgc_domain_suspend(struct device *dev)
    {
    int ret;
//
// This may look strange, but is done so the generic PM_SLEEP code
// can power down our domain and more importantly power it up again
// after resume, without tripping over our usage of runtime PM to
// power up/down the nested domains.
//
    ret = pm_runtime_get_sync(dev);
    if (ret < 0) {
    pm_runtime_put_noidle(dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_pgc_domain_resume(dev: *mut device) -> c_int {
    static int imx_pgc_domain_resume(struct device *dev)
    {
    pm_runtime_put(dev);
    return 0;
    }

    static const struct dev_pm_ops imx_pgc_domain_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(imx_pgc_domain_suspend, imx_pgc_domain_resume)
    };
    static const struct platform_device_id imx_pgc_domain_id[] = {
    { "imx-pgc-domain", },
    { },
    };
    static struct platform_driver imx_pgc_domain_driver = {
    .driver = {
    .name = "imx-pgc",
    .pm = &imx_pgc_domain_pm_ops,
    .suppress_bind_attrs = true,
    },
    .probe    = imx_pgc_domain_probe,
    .remove = imx_pgc_domain_remove,
    .id_table = imx_pgc_domain_id,
    };
    builtin_platform_driver(imx_pgc_domain_driver)
#[no_mangle]
unsafe extern "C" fn imx_gpcv2_probe(pdev: *mut platform_device) -> c_int {
    static int imx_gpcv2_probe(struct platform_device *pdev)
    {
    const struct imx_pgc_domain_data *domain_data =
    of_device_get_match_data(&pdev.dev);
    struct regmap_config regmap_config = {
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    .rd_table	= domain_data.reg_access_table,
    .wr_table	= domain_data.reg_access_table,
    .max_register   = SZ_4K,
    };
    struct device *dev = &pdev.dev;
    struct device_node *pgc_np __free(device_node) =
    of_get_child_by_name(dev.of_node, "pgc");
    struct regmap *regmap;
    void __iomem *base;
    int ret;
    if (!pgc_np) {
    dev_err(dev, "No power domains specified in DT\n");
    return -EINVAL;
    }
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(dev, base, &regmap_config);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(dev, "failed to init regmap (%d)\n", ret);
    return ret;
    }
    for_each_child_of_node_scoped(pgc_np, np) {
    struct platform_device *pd_pdev;
    struct imx_pgc_domain *domain;
    u32 domain_index;
    if (!of_device_is_available(np))
    continue;
    ret = of_property_read_u32(np, "reg", &domain_index);
    if (ret) {
    dev_err(dev, "Failed to read 'reg' property\n");
    return ret;
    }
    if (domain_index >= domain_data.domains_num) {
    dev_warn(dev,
    "Domain index %d is out of bounds\n",
    domain_index);
    continue;
    }
    pd_pdev = platform_device_alloc("imx-pgc-domain",
    domain_index);
    if (!pd_pdev) {
    dev_err(dev, "Failed to allocate platform device\n");
    return -ENOMEM;
    }
    ret = platform_device_add_data(pd_pdev,
    &domain_data.domains[domain_index],
    sizeof(domain_data.domains[domain_index]));
    if (ret) {
    platform_device_put(pd_pdev);
    return ret;
    }
    domain = pd_pdev.dev.platform_data;
    domain.regmap = regmap;
    domain.regs = domain_data.pgc_regs;
    domain.genpd.power_on  = imx_pgc_power_up;
    domain.genpd.power_off = imx_pgc_power_down;
    pd_pdev.dev.parent = dev;
    device_set_node(&pd_pdev.dev, of_fwnode_handle(np));
    ret = platform_device_add(pd_pdev);
    if (ret) {
    platform_device_put(pd_pdev);
    return ret;
    }
    }
    return 0;
    }
    static const struct of_device_id imx_gpcv2_dt_ids[] = {
    { .compatible = "fsl,imx7d-gpc", .data = &imx7_pgc_domain_data, },
    { .compatible = "fsl,imx8mm-gpc", .data = &imx8mm_pgc_domain_data, },
    { .compatible = "fsl,imx8mn-gpc", .data = &imx8mn_pgc_domain_data, },
    { .compatible = "fsl,imx8mp-gpc", .data = &imx8mp_pgc_domain_data, },
    { .compatible = "fsl,imx8mq-gpc", .data = &imx8m_pgc_domain_data, },
    { }
    };
    static struct platform_driver imx_gpc_driver = {
    .driver = {
    .name = "imx-gpcv2",
    .of_match_table = imx_gpcv2_dt_ids,
    .suppress_bind_attrs = true,
    },
    .probe = imx_gpcv2_probe,
    };
    builtin_platform_driver(imx_gpc_driver)
