//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/imx/imx8mp-blk-ctrl.c
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
// Copyright 2022 Pengutronix, Lucas Stach <kernel@pengutronix.de>
//

pub const GPR_REG0: c_uint = 0x0;

pub const GPR_REG1: c_uint = 0x4;

pub const GPR_REG2: c_uint = 0x8;

pub const GPR_REG3: c_uint = 0xc;

    struct imx8mp_blk_ctrl_domain;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mp_blk_ctrl {
    pub dev: *mut device,
    pub power_nb: notifier_block,
    pub bus_power_dev: *mut device,
    pub regmap: *mut regmap,
    pub domains: *mut imx8mp_blk_ctrl_domain,
    pub onecell_data: genpd_onecell_data,
    pub domain): *mut *mut *mut void (power_off) (struct imx8mp_blk_ctrl bc, struct imx8mp_blk_ctrl_domain,
    pub domain): *mut *mut *mut void (power_on) (struct imx8mp_blk_ctrl bc, struct imx8mp_blk_ctrl_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mp_blk_ctrl_domain_data {
    pub name: *const c_char,
    pub clk_names: *const *const c_char,
    pub num_clks: c_int,
    pub path_names: *const *const c_char,
    pub num_paths: c_int,
    pub gpc_name: *const c_char,
    pub flags: c_uint,
}

pub const DOMAIN_MAX_CLKS: c_int = 3;
pub const DOMAIN_MAX_PATHS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mp_blk_ctrl_domain {
    pub genpd: generic_pm_domain,
    pub data: *const imx8mp_blk_ctrl_domain_data,
    pub clks: [clk_bulk_data; DOMAIN_MAX_CLKS],
    pub paths: [icc_bulk_data; DOMAIN_MAX_PATHS],
    pub power_dev: *mut device,
    pub bc: *mut imx8mp_blk_ctrl,
    pub power_nb: notifier_block,
    pub num_paths: c_int,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mp_blk_ctrl_data {
    pub max_reg: c_int,
    pub bc): *mut *mut int (probe) (struct imx8mp_blk_ctrl,
    pub power_notifier_fn: notifier_fn_t,
    pub domain): *mut *mut *mut void (power_off) (struct imx8mp_blk_ctrl bc, struct imx8mp_blk_ctrl_domain,
    pub domain): *mut *mut *mut void (power_on) (struct imx8mp_blk_ctrl bc, struct imx8mp_blk_ctrl_domain,
    pub domains: *const imx8mp_blk_ctrl_domain_data,
    pub num_domains: c_int,
}

    static inline struct imx8mp_blk_ctrl_domain *
    to_imx8mp_blk_ctrl_domain(struct generic_pm_domain *genpd)
    {
    return container_of(genpd, struct imx8mp_blk_ctrl_domain, genpd);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hsio_pll {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

    static inline struct clk_hsio_pll *to_clk_hsio_pll(struct clk_hw *hw)
    {
    return container_of(hw, struct clk_hsio_pll, hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_hsio_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_hsio_pll_prepare(struct clk_hw *hw)
    {
    struct clk_hsio_pll *clk = to_clk_hsio_pll(hw);
    u32 val;
// set the PLL configuration
    regmap_update_bits(clk.regmap, GPR_REG2,
    P_PLL_MASK | M_PLL_MASK | S_PLL_MASK,
    FIELD_PREP(P_PLL_MASK, 12) |
    FIELD_PREP(M_PLL_MASK, 800) |
    FIELD_PREP(S_PLL_MASK, 4));
// de-assert PLL reset
    regmap_update_bits(clk.regmap, GPR_REG3, PLL_RST, PLL_RST);
// enable PLL
    regmap_update_bits(clk.regmap, GPR_REG3, PLL_CKE, PLL_CKE);
    return regmap_read_poll_timeout(clk.regmap, GPR_REG1, val,
    val & PLL_LOCK, 10, 100);
    }
#[no_mangle]
unsafe extern "C" fn clk_hsio_pll_unprepare(hw: *mut clk_hw) {
    static void clk_hsio_pll_unprepare(struct clk_hw *hw)
    {
    struct clk_hsio_pll *clk = to_clk_hsio_pll(hw);
    regmap_update_bits(clk.regmap, GPR_REG3, PLL_RST | PLL_CKE, 0);
    }
#[no_mangle]
unsafe extern "C" fn clk_hsio_pll_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_hsio_pll_is_prepared(struct clk_hw *hw)
    {
    struct clk_hsio_pll *clk = to_clk_hsio_pll(hw);
    return regmap_test_bits(clk.regmap, GPR_REG1, PLL_LOCK);
    }
    static unsigned long clk_hsio_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return 100000000;
    }
    static const struct clk_ops clk_hsio_pll_ops = {
    .prepare = clk_hsio_pll_prepare,
    .unprepare = clk_hsio_pll_unprepare,
    .is_prepared = clk_hsio_pll_is_prepared,
    .recalc_rate = clk_hsio_pll_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn imx8mp_hsio_blk_ctrl_probe(bc: *mut imx8mp_blk_ctrl) -> c_int {
    static int imx8mp_hsio_blk_ctrl_probe(struct imx8mp_blk_ctrl *bc)
    {
    struct clk_hsio_pll *clk_hsio_pll;
    struct clk_hw *hw;
    let mut init: clk_init_data = {};
    int ret;
    clk_hsio_pll = devm_kzalloc(bc.dev, sizeof(*clk_hsio_pll), GFP_KERNEL);
    if (!clk_hsio_pll)
    return -ENOMEM;
    init.name = "hsio_pll";
    init.ops = &clk_hsio_pll_ops;
    init.parent_names = (const char *[]){"osc_24m"};
    init.num_parents = 1;
    clk_hsio_pll.regmap = bc.regmap;
    clk_hsio_pll.hw.init = &init;
    hw = &clk_hsio_pll.hw;
    ret = devm_clk_hw_register(bc.bus_power_dev, hw);
    if (ret)
    return ret;
    return devm_of_clk_add_hw_provider(bc.dev, of_clk_hw_simple_get, hw);
    }
    static void imx8mp_hsio_blk_ctrl_power_on(struct imx8mp_blk_ctrl *bc,
    struct imx8mp_blk_ctrl_domain *domain)
    {
    switch (domain.id) {
    case IMX8MP_HSIOBLK_PD_USB:
    regmap_set_bits(bc.regmap, GPR_REG0, USB_CLOCK_MODULE_EN);
    break;
    case IMX8MP_HSIOBLK_PD_PCIE:
    regmap_set_bits(bc.regmap, GPR_REG0, PCIE_CLOCK_MODULE_EN);
    break;
    case IMX8MP_HSIOBLK_PD_PCIE_PHY:
    regmap_set_bits(bc.regmap, GPR_REG0,
    PCIE_PHY_APB_RST | PCIE_PHY_INIT_RST);
    break;
    default:
    break;
    }
    }
    static void imx8mp_hsio_blk_ctrl_power_off(struct imx8mp_blk_ctrl *bc,
    struct imx8mp_blk_ctrl_domain *domain)
    {
    switch (domain.id) {
    case IMX8MP_HSIOBLK_PD_USB:
    regmap_clear_bits(bc.regmap, GPR_REG0, USB_CLOCK_MODULE_EN);
    break;
    case IMX8MP_HSIOBLK_PD_PCIE:
    regmap_clear_bits(bc.regmap, GPR_REG0, PCIE_CLOCK_MODULE_EN);
    break;
    case IMX8MP_HSIOBLK_PD_PCIE_PHY:
    regmap_clear_bits(bc.regmap, GPR_REG0,
    PCIE_PHY_APB_RST | PCIE_PHY_INIT_RST);
    break;
    default:
    break;
    }
    }
    static int imx8mp_hsio_power_notifier(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct imx8mp_blk_ctrl *bc = container_of(nb, struct imx8mp_blk_ctrl,
    power_nb);
    struct clk_bulk_data *usb_clk = bc.domains[IMX8MP_HSIOBLK_PD_USB].clks;
    let mut num_clks: c_int = bc.domains[IMX8MP_HSIOBLK_PD_USB].data.num_clks;
    int ret;
    switch (action) {
    case GENPD_NOTIFY_ON:
//
// enable USB clock for a moment for the power-on ADB handshake
// to proceed
//
    ret = clk_bulk_prepare_enable(num_clks, usb_clk);
    if (ret)
    return NOTIFY_BAD;
    regmap_set_bits(bc.regmap, GPR_REG0, USB_CLOCK_MODULE_EN);
    udelay(5);
    regmap_clear_bits(bc.regmap, GPR_REG0, USB_CLOCK_MODULE_EN);
    clk_bulk_disable_unprepare(num_clks, usb_clk);
    break;
    case GENPD_NOTIFY_PRE_OFF:
// enable USB clock for the power-down ADB handshake to work
    ret = clk_bulk_prepare_enable(num_clks, usb_clk);
    if (ret)
    return NOTIFY_BAD;
    regmap_set_bits(bc.regmap, GPR_REG0, USB_CLOCK_MODULE_EN);
    break;
    case GENPD_NOTIFY_OFF:
    clk_bulk_disable_unprepare(num_clks, usb_clk);
    break;
    default:
    break;
    }
    return NOTIFY_OK;
    }
    static const struct imx8mp_blk_ctrl_domain_data imx8mp_hsio_domain_data[] = {
    [IMX8MP_HSIOBLK_PD_USB] = {
    .name = "hsioblk-usb",
    .clk_names = (const char *[]){ "usb" },
    .num_clks = 1,
    .gpc_name = "usb",
    .path_names = (const char *[]){"usb1", "usb2"},
    .num_paths = 2,
    },
    [IMX8MP_HSIOBLK_PD_USB_PHY1] = {
    .name = "hsioblk-usb-phy1",
    .gpc_name = "usb-phy1",
    .flags = GENPD_FLAG_ACTIVE_WAKEUP,
    },
    [IMX8MP_HSIOBLK_PD_USB_PHY2] = {
    .name = "hsioblk-usb-phy2",
    .gpc_name = "usb-phy2",
    .flags = GENPD_FLAG_ACTIVE_WAKEUP,
    },
    [IMX8MP_HSIOBLK_PD_PCIE] = {
    .name = "hsioblk-pcie",
    .clk_names = (const char *[]){ "pcie" },
    .num_clks = 1,
    .gpc_name = "pcie",
    .path_names = (const char *[]){"noc-pcie", "pcie"},
    .num_paths = 2,
    },
    [IMX8MP_HSIOBLK_PD_PCIE_PHY] = {
    .name = "hsioblk-pcie-phy",
    .gpc_name = "pcie-phy",
    },
    };
    static const struct imx8mp_blk_ctrl_data imx8mp_hsio_blk_ctl_dev_data = {
    .max_reg = 0x24,
    .probe = imx8mp_hsio_blk_ctrl_probe,
    .power_on = imx8mp_hsio_blk_ctrl_power_on,
    .power_off = imx8mp_hsio_blk_ctrl_power_off,
    .power_notifier_fn = imx8mp_hsio_power_notifier,
    .domains = imx8mp_hsio_domain_data,
    .num_domains = ARRAY_SIZE(imx8mp_hsio_domain_data),
    };
pub const HDMI_RTX_RESET_CTL0: c_uint = 0x20;
pub const HDMI_RTX_CLK_CTL0: c_uint = 0x40;
pub const HDMI_RTX_CLK_CTL1: c_uint = 0x50;
pub const HDMI_RTX_CLK_CTL2: c_uint = 0x60;
pub const HDMI_RTX_CLK_CTL3: c_uint = 0x70;
pub const HDMI_RTX_CLK_CTL4: c_uint = 0x80;
pub const HDMI_TX_CONTROL0: c_uint = 0x200;

    static void imx8mp_hdmi_blk_ctrl_power_on(struct imx8mp_blk_ctrl *bc,
    struct imx8mp_blk_ctrl_domain *domain)
    {
    switch (domain.id) {
    case IMX8MP_HDMIBLK_PD_IRQSTEER:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL0, BIT(9));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(16));
    break;
    case IMX8MP_HDMIBLK_PD_LCDIF:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL0,
    BIT(16) | BIT(17) | BIT(18) |
    BIT(19) | BIT(20));
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(11));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0,
    BIT(4) | BIT(5) | BIT(6));
    regmap_set_bits(bc.regmap, HDMI_TX_CONTROL0,
    FIELD_PREP(HDMI_LCDIF_NOC_HURRY_MASK, 7));
    break;
    case IMX8MP_HDMIBLK_PD_PAI:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(17));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(18));
    break;
    case IMX8MP_HDMIBLK_PD_PVI:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(28));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(22));
    break;
    case IMX8MP_HDMIBLK_PD_TRNG:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(27) | BIT(30));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(20));
    break;
    case IMX8MP_HDMIBLK_PD_HDMI_TX:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL0,
    BIT(2) | BIT(4) | BIT(5));
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL1,
    BIT(12) | BIT(13) | BIT(14) | BIT(15) | BIT(16) |
    BIT(18) | BIT(19) | BIT(20) | BIT(21));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0,
    BIT(7) | BIT(10) | BIT(11));
    regmap_set_bits(bc.regmap, HDMI_TX_CONTROL0, BIT(1));
    break;
    case IMX8MP_HDMIBLK_PD_HDMI_TX_PHY:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL0, BIT(7));
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(22) | BIT(24));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(12));
    regmap_clear_bits(bc.regmap, HDMI_TX_CONTROL0, BIT(3));
    break;
    case IMX8MP_HDMIBLK_PD_HRV:
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(3) | BIT(4) | BIT(5));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(15));
    break;
    default:
    break;
    }
    }
    static void imx8mp_hdmi_blk_ctrl_power_off(struct imx8mp_blk_ctrl *bc,
    struct imx8mp_blk_ctrl_domain *domain)
    {
    switch (domain.id) {
    case IMX8MP_HDMIBLK_PD_IRQSTEER:
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL0, BIT(9));
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(16));
    break;
    case IMX8MP_HDMIBLK_PD_LCDIF:
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0,
    BIT(4) | BIT(5) | BIT(6));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(11));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL0,
    BIT(16) | BIT(17) | BIT(18) |
    BIT(19) | BIT(20));
    break;
    case IMX8MP_HDMIBLK_PD_PAI:
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(18));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(17));
    break;
    case IMX8MP_HDMIBLK_PD_PVI:
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(22));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(28));
    break;
    case IMX8MP_HDMIBLK_PD_TRNG:
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(20));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(27) | BIT(30));
    break;
    case IMX8MP_HDMIBLK_PD_HDMI_TX:
    regmap_clear_bits(bc.regmap, HDMI_TX_CONTROL0, BIT(1));
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0,
    BIT(7) | BIT(10) | BIT(11));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL1,
    BIT(12) | BIT(13) | BIT(14) | BIT(15) | BIT(16) |
    BIT(18) | BIT(19) | BIT(20) | BIT(21));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL0,
    BIT(2) | BIT(4) | BIT(5));
    break;
    case IMX8MP_HDMIBLK_PD_HDMI_TX_PHY:
    regmap_set_bits(bc.regmap, HDMI_TX_CONTROL0, BIT(3));
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(12));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL0, BIT(7));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(22) | BIT(24));
    break;
    case IMX8MP_HDMIBLK_PD_HRV:
    regmap_clear_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(15));
    regmap_clear_bits(bc.regmap, HDMI_RTX_CLK_CTL1, BIT(3) | BIT(4) | BIT(5));
    break;
    default:
    break;
    }
    }
    static int imx8mp_hdmi_power_notifier(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct imx8mp_blk_ctrl *bc = container_of(nb, struct imx8mp_blk_ctrl,
    power_nb);
    if (action != GENPD_NOTIFY_ON)
    return NOTIFY_OK;
//
// Contrary to other blk-ctrls the reset and clock don't clear when the
// power domain is powered down. To ensure the proper reset pulsing,
// first clear them all to asserted state, then enable the bus clocks
// and then release the ADB reset.
//
    regmap_write(bc.regmap, HDMI_RTX_RESET_CTL0, 0x0);
    regmap_write(bc.regmap, HDMI_RTX_CLK_CTL0, 0x0);
    regmap_write(bc.regmap, HDMI_RTX_CLK_CTL1, 0x0);
    regmap_set_bits(bc.regmap, HDMI_RTX_CLK_CTL0,
    BIT(0) | BIT(1) | BIT(10) | BIT(11));
    regmap_set_bits(bc.regmap, HDMI_RTX_RESET_CTL0, BIT(0));
//
// On power up we have no software backchannel to the GPC to
// wait for the ADB handshake to happen, so we just delay for a
// bit. On power down the GPC driver waits for the handshake.
//
    udelay(5);
    return NOTIFY_OK;
    }
    static const struct imx8mp_blk_ctrl_domain_data imx8mp_hdmi_domain_data[] = {
    [IMX8MP_HDMIBLK_PD_IRQSTEER] = {
    .name = "hdmiblk-irqsteer",
    .clk_names = (const char *[]){ "apb" },
    .num_clks = 1,
    .gpc_name = "irqsteer",
    },
    [IMX8MP_HDMIBLK_PD_LCDIF] = {
    .name = "hdmiblk-lcdif",
    .clk_names = (const char *[]){ "axi", "apb", "fdcc" },
    .num_clks = 3,
    .gpc_name = "lcdif",
    .path_names = (const char *[]){"lcdif-hdmi"},
    .num_paths = 1,
    },
    [IMX8MP_HDMIBLK_PD_PAI] = {
    .name = "hdmiblk-pai",
    .clk_names = (const char *[]){ "apb" },
    .num_clks = 1,
    .gpc_name = "pai",
    },
    [IMX8MP_HDMIBLK_PD_PVI] = {
    .name = "hdmiblk-pvi",
    .clk_names = (const char *[]){ "apb" },
    .num_clks = 1,
    .gpc_name = "pvi",
    },
    [IMX8MP_HDMIBLK_PD_TRNG] = {
    .name = "hdmiblk-trng",
    .clk_names = (const char *[]){ "apb" },
    .num_clks = 1,
    .gpc_name = "trng",
    },
    [IMX8MP_HDMIBLK_PD_HDMI_TX] = {
    .name = "hdmiblk-hdmi-tx",
    .clk_names = (const char *[]){ "apb", "ref_266m", "fdcc" },
    .num_clks = 3,
    .gpc_name = "hdmi-tx",
    },
    [IMX8MP_HDMIBLK_PD_HDMI_TX_PHY] = {
    .name = "hdmiblk-hdmi-tx-phy",
    .clk_names = (const char *[]){ "apb", "ref_24m" },
    .num_clks = 2,
    .gpc_name = "hdmi-tx-phy",
    },
    [IMX8MP_HDMIBLK_PD_HRV] = {
    .name = "hdmiblk-hrv",
    .clk_names = (const char *[]){ "axi", "apb" },
    .num_clks = 2,
    .gpc_name = "hrv",
    .path_names = (const char *[]){"hrv"},
    .num_paths = 1,
    },
    [IMX8MP_HDMIBLK_PD_HDCP] = {
    .name = "hdmiblk-hdcp",
    .clk_names = (const char *[]){ "axi", "apb" },
    .num_clks = 2,
    .gpc_name = "hdcp",
    .path_names = (const char *[]){"hdcp"},
    .num_paths = 1,
    },
    };
    static const struct imx8mp_blk_ctrl_data imx8mp_hdmi_blk_ctl_dev_data = {
    .max_reg = 0x23c,
    .power_on = imx8mp_hdmi_blk_ctrl_power_on,
    .power_off = imx8mp_hdmi_blk_ctrl_power_off,
    .power_notifier_fn = imx8mp_hdmi_power_notifier,
    .domains = imx8mp_hdmi_domain_data,
    .num_domains = ARRAY_SIZE(imx8mp_hdmi_domain_data),
    };
#[no_mangle]
unsafe extern "C" fn imx8mp_blk_ctrl_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static int imx8mp_blk_ctrl_power_on(struct generic_pm_domain *genpd)
    {
    struct imx8mp_blk_ctrl_domain *domain = to_imx8mp_blk_ctrl_domain(genpd);
    const struct imx8mp_blk_ctrl_domain_data *data = domain.data;
    struct imx8mp_blk_ctrl *bc = domain.bc;
    int ret;
// make sure bus domain is awake
    ret = pm_runtime_resume_and_get(bc.bus_power_dev);
    if (ret < 0) {
    dev_err(bc.dev, "failed to power up bus domain\n");
    return ret;
    }
// enable upstream clocks
    ret = clk_bulk_prepare_enable(data.num_clks, domain.clks);
    if (ret) {
    dev_err(bc.dev, "failed to enable clocks\n");
    goto bus_put;
    }
// domain specific blk-ctrl manipulation
    bc.power_on(bc, domain);
// power up upstream GPC domain
    ret = pm_runtime_resume_and_get(domain.power_dev);
    if (ret < 0) {
    dev_err(bc.dev, "failed to power up peripheral domain\n");
    goto clk_disable;
    }
    ret = icc_bulk_set_bw(domain.num_paths, domain.paths);
    if (ret)
    dev_err(bc.dev, "failed to set icc bw\n");
    clk_bulk_disable_unprepare(data.num_clks, domain.clks);
    return 0;
    clk_disable:
    clk_bulk_disable_unprepare(data.num_clks, domain.clks);
    bus_put:
    pm_runtime_put(bc.bus_power_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_blk_ctrl_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static int imx8mp_blk_ctrl_power_off(struct generic_pm_domain *genpd)
    {
    struct imx8mp_blk_ctrl_domain *domain = to_imx8mp_blk_ctrl_domain(genpd);
    const struct imx8mp_blk_ctrl_domain_data *data = domain.data;
    struct imx8mp_blk_ctrl *bc = domain.bc;
    int ret;
    ret = clk_bulk_prepare_enable(data.num_clks, domain.clks);
    if (ret) {
    dev_err(bc.dev, "failed to enable clocks\n");
    return ret;
    }
// domain specific blk-ctrl manipulation
    bc.power_off(bc, domain);
    clk_bulk_disable_unprepare(data.num_clks, domain.clks);
// power down upstream GPC domain
    pm_runtime_put(domain.power_dev);
// allow bus domain to suspend
    pm_runtime_put(bc.bus_power_dev);
    return 0;
    }
    static int imx8mp_blk_ctrl_gpc_notifier(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct imx8mp_blk_ctrl_domain *domain =
    container_of(nb, struct imx8mp_blk_ctrl_domain, power_nb);
    if (action == GENPD_NOTIFY_PRE_OFF) {
    if (domain.genpd.status == GENPD_STATE_ON)
    return NOTIFY_BAD;
    }
    return NOTIFY_OK;
    }
    static struct lock_class_key blk_ctrl_genpd_lock_class;
#[no_mangle]
unsafe extern "C" fn imx8mp_blk_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int imx8mp_blk_ctrl_probe(struct platform_device *pdev)
    {
    const struct imx8mp_blk_ctrl_data *bc_data;
    struct device *dev = &pdev.dev;
    struct imx8mp_blk_ctrl *bc;
    void __iomem *base;
    int num_domains, i, ret;
    struct regmap_config regmap_config = {
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    };
    bc = devm_kzalloc(dev, sizeof(*bc), GFP_KERNEL);
    if (!bc)
    return -ENOMEM;
    bc.dev = dev;
    bc_data = of_device_get_match_data(dev);
    num_domains = bc_data.num_domains;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap_config.max_register = bc_data.max_reg;
    bc.regmap = devm_regmap_init_mmio(dev, base, &regmap_config);
    if (IS_ERR(bc.regmap))
    return dev_err_probe(dev, PTR_ERR(bc.regmap),
    "failed to init regmap\n");
    bc.domains = devm_kcalloc(dev, num_domains,
    sizeof(struct imx8mp_blk_ctrl_domain),
    GFP_KERNEL);
    if (!bc.domains)
    return -ENOMEM;
    bc.onecell_data.num_domains = num_domains;
    bc.onecell_data.domains =
    devm_kcalloc(dev, num_domains,
    sizeof(struct generic_pm_domain *), GFP_KERNEL);
    if (!bc.onecell_data.domains)
    return -ENOMEM;
    bc.bus_power_dev = dev_pm_domain_attach_by_name(dev, "bus");
    if (IS_ERR(bc.bus_power_dev))
    return dev_err_probe(dev, PTR_ERR(bc.bus_power_dev),
    "failed to attach bus power domain\n");
    bc.power_off = bc_data.power_off;
    bc.power_on = bc_data.power_on;
    for (i = 0; i < num_domains; i++) {
    const struct imx8mp_blk_ctrl_domain_data *data = &bc_data.domains[i];
    struct imx8mp_blk_ctrl_domain *domain = &bc.domains[i];
    int j;
    domain.data = data;
    domain.num_paths = data.num_paths;
    for (j = 0; j < data.num_clks; j++)
    domain.clks[j].id = data.clk_names[j];
    for (j = 0; j < data.num_paths; j++) {
    domain.paths[j].name = data.path_names[j];
// Fake value for now, just let ICC could configure NoC mode/priority
    domain.paths[j].avg_bw = 1;
    domain.paths[j].peak_bw = 1;
    }
    ret = devm_of_icc_bulk_get(dev, data.num_paths, domain.paths);
    if (ret) {
    if (ret != -EPROBE_DEFER) {
    dev_warn_once(dev, "Could not get interconnect paths, NoC will stay unconfigured!\n");
    domain.num_paths = 0;
    } else {
    dev_err_probe(dev, ret, "failed to get noc entries\n");
    goto cleanup_pds;
    }
    }
    ret = devm_clk_bulk_get(dev, data.num_clks, domain.clks);
    if (ret) {
    dev_err_probe(dev, ret, "failed to get clock\n");
    goto cleanup_pds;
    }
    domain.power_dev =
    dev_pm_domain_attach_by_name(dev, data.gpc_name);
    if (IS_ERR_OR_NULL(domain.power_dev)) {
    if (!domain.power_dev)
    ret = -ENODEV;
    else
    ret = PTR_ERR(domain.power_dev);
    dev_err_probe(dev, ret,
    "failed to attach power domain %s\n",
    data.gpc_name);
    goto cleanup_pds;
    }
    domain.power_nb.notifier_call = imx8mp_blk_ctrl_gpc_notifier;
    ret = dev_pm_genpd_add_notifier(domain.power_dev, &domain.power_nb);
    if (ret) {
    dev_err_probe(dev, ret, "failed to add power notifier\n");
    dev_pm_domain_detach(domain.power_dev, true);
    goto cleanup_pds;
    }
    domain.genpd.name = data.name;
    domain.genpd.power_on = imx8mp_blk_ctrl_power_on;
    domain.genpd.power_off = imx8mp_blk_ctrl_power_off;
    domain.genpd.flags = data.flags;
    domain.bc = bc;
    domain.id = i;
    ret = pm_genpd_init(&domain.genpd, core::ptr::null_mut(), true);
    if (ret) {
    dev_err_probe(dev, ret, "failed to init power domain\n");
    dev_pm_genpd_remove_notifier(domain.power_dev);
    dev_pm_domain_detach(domain.power_dev, true);
    goto cleanup_pds;
    }
//
// We use runtime PM to trigger power on/off of the upstream GPC
// domain, as a strict hierarchical parent/child power domain
// setup doesn't allow us to meet the sequencing requirements.
// This means we have nested locking of genpd locks, without the
// nesting being visible at the genpd level, so we need a
// separate lock class to make lockdep aware of the fact that
// this are separate domain locks that can be nested without a
// self-deadlock.
//
    lockdep_set_class(&domain.genpd.mlock,
    &blk_ctrl_genpd_lock_class);
    bc.onecell_data.domains[i] = &domain.genpd;
    }
    ret = of_genpd_add_provider_onecell(dev.of_node, &bc.onecell_data);
    if (ret) {
    dev_err_probe(dev, ret, "failed to add power domain provider\n");
    goto cleanup_pds;
    }
    bc.power_nb.notifier_call = bc_data.power_notifier_fn;
    ret = dev_pm_genpd_add_notifier(bc.bus_power_dev, &bc.power_nb);
    if (ret) {
    dev_err_probe(dev, ret, "failed to add power notifier\n");
    goto cleanup_provider;
    }
    if (bc_data.probe) {
    ret = bc_data.probe(bc);
    if (ret)
    goto cleanup_provider;
    }
    dev_set_drvdata(dev, bc);
    return 0;
    cleanup_provider:
    of_genpd_del_provider(dev.of_node);
    cleanup_pds:
    for (i--; i >= 0; i--) {
    pm_genpd_remove(&bc.domains[i].genpd);
    dev_pm_genpd_remove_notifier(bc.domains[i].power_dev);
    dev_pm_domain_detach(bc.domains[i].power_dev, true);
    }
    dev_pm_domain_detach(bc.bus_power_dev, true);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_blk_ctrl_remove(pdev: *mut platform_device) {
    static void imx8mp_blk_ctrl_remove(struct platform_device *pdev)
    {
    struct imx8mp_blk_ctrl *bc = dev_get_drvdata(&pdev.dev);
    int i;
    of_genpd_del_provider(pdev.dev.of_node);
    for (i = 0; i < bc.onecell_data.num_domains; i++) {
    struct imx8mp_blk_ctrl_domain *domain = &bc.domains[i];
    pm_genpd_remove(&domain.genpd);
    dev_pm_genpd_remove_notifier(domain.power_dev);
    dev_pm_domain_detach(domain.power_dev, true);
    }
    dev_pm_genpd_remove_notifier(bc.bus_power_dev);
    dev_pm_domain_detach(bc.bus_power_dev, true);
    }

#[no_mangle]
unsafe extern "C" fn imx8mp_blk_ctrl_suspend(dev: *mut device) -> c_int {
    static int imx8mp_blk_ctrl_suspend(struct device *dev)
    {
    struct imx8mp_blk_ctrl *bc = dev_get_drvdata(dev);
    int ret, i;
//
// This may look strange, but is done so the generic PM_SLEEP code
// can power down our domains and more importantly power them up again
// after resume, without tripping over our usage of runtime PM to
// control the upstream GPC domains. Things happen in the right order
// in the system suspend/resume paths due to the device parent/child
// hierarchy.
//
    ret = pm_runtime_get_sync(bc.bus_power_dev);
    if (ret < 0) {
    pm_runtime_put_noidle(bc.bus_power_dev);
    return ret;
    }
    for (i = 0; i < bc.onecell_data.num_domains; i++) {
    struct imx8mp_blk_ctrl_domain *domain = &bc.domains[i];
    ret = pm_runtime_get_sync(domain.power_dev);
    if (ret < 0) {
    pm_runtime_put_noidle(domain.power_dev);
    goto out_fail;
    }
    }
    return 0;
    out_fail:
    for (i--; i >= 0; i--)
    pm_runtime_put(bc.domains[i].power_dev);
    pm_runtime_put(bc.bus_power_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_blk_ctrl_resume(dev: *mut device) -> c_int {
    static int imx8mp_blk_ctrl_resume(struct device *dev)
    {
    struct imx8mp_blk_ctrl *bc = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < bc.onecell_data.num_domains; i++)
    pm_runtime_put(bc.domains[i].power_dev);
    pm_runtime_put(bc.bus_power_dev);
    return 0;
    }

    static const struct dev_pm_ops imx8mp_blk_ctrl_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(imx8mp_blk_ctrl_suspend,
    imx8mp_blk_ctrl_resume)
    };
    static const struct of_device_id imx8mp_blk_ctrl_of_match[] = {
    {
    .compatible = "fsl,imx8mp-hsio-blk-ctrl",
    .data = &imx8mp_hsio_blk_ctl_dev_data,
    }, {
    .compatible = "fsl,imx8mp-hdmi-blk-ctrl",
    .data = &imx8mp_hdmi_blk_ctl_dev_data,
    }, {
// Sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, imx8mp_blk_ctrl_of_match);
    static struct platform_driver imx8mp_blk_ctrl_driver = {
    .probe = imx8mp_blk_ctrl_probe,
    .remove = imx8mp_blk_ctrl_remove,
    .driver = {
    .name = "imx8mp-blk-ctrl",
    .pm = &imx8mp_blk_ctrl_pm_ops,
    .of_match_table = imx8mp_blk_ctrl_of_match,
    .suppress_bind_attrs = true,
    },
    };
    module_platform_driver(imx8mp_blk_ctrl_driver);
    MODULE_DESCRIPTION("NXP i.MX8MP power domain driver");
    MODULE_LICENSE("GPL");
