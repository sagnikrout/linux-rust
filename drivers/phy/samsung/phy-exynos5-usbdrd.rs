//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-exynos5-usbdrd.c
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
// Samsung Exynos5 SoC series USB DRD PHY driver
//
// Phy provider for USB 3.0 DRD controller on Exynos5 SoC series
//
// Copyright (C) 2014 Samsung Electronics Co., Ltd.
// Author: Vivek Gautam <gautam.vivek@samsung.com>
//

// Exynos USB PHY registers
pub const EXYNOS5_FSEL_9MHZ6: c_uint = 0x0;
pub const EXYNOS5_FSEL_10MHZ: c_uint = 0x1;
pub const EXYNOS5_FSEL_12MHZ: c_uint = 0x2;
pub const EXYNOS5_FSEL_19MHZ2: c_uint = 0x3;
pub const EXYNOS5_FSEL_20MHZ: c_uint = 0x4;
pub const EXYNOS5_FSEL_24MHZ: c_uint = 0x5;
pub const EXYNOS5_FSEL_26MHZ: c_uint = 0x6;
pub const EXYNOS5_FSEL_50MHZ: c_uint = 0x7;
// USB 3.2 DRD 4nm PHY link controller registers
pub const EXYNOS2200_DRD_CLKRST: c_uint = 0x0c;

pub const EXYNOS2200_DRD_UTMI: c_uint = 0x10;
// ExynosAutov920 bits

pub const EXYNOS2200_DRD_HSP_MISC: c_uint = 0x114;

pub const RES_TUNE_PHY1_PHY2: c_uint = 0x1;
pub const RES_TUNE_PHY1: c_uint = 0x2;
pub const RES_TUNE_PHY2: c_uint = 0x3;
// Exynos5: USB 3.0 DRD PHY registers
pub const EXYNOS5_DRD_LINKSYSTEM: c_uint = 0x04;

pub const EXYNOS5_DRD_PHYUTMI: c_uint = 0x08;

pub const EXYNOS5_DRD_PHYPIPE: c_uint = 0x0c;
pub const EXYNOS5_DRD_PHYCLKRST: c_uint = 0x10;

pub const PHYCLKRST_MPLL_MULTIPLIER_100MHZ_REF: c_uint = 0x19;
pub const PHYCLKRST_MPLL_MULTIPLIER_50M_REF: c_uint = 0x32;
pub const PHYCLKRST_MPLL_MULTIPLIER_24MHZ_REF: c_uint = 0x68;
pub const PHYCLKRST_MPLL_MULTIPLIER_20MHZ_REF: c_uint = 0x7d;
pub const PHYCLKRST_MPLL_MULTIPLIER_19200KHZ_REF: c_uint = 0x02;

pub const PHYCLKRST_FSEL_PAD_100MHZ: c_uint = 0x27;
pub const PHYCLKRST_FSEL_PAD_24MHZ: c_uint = 0x2a;
pub const PHYCLKRST_FSEL_PAD_20MHZ: c_uint = 0x31;
pub const PHYCLKRST_FSEL_PAD_19_2MHZ: c_uint = 0x38;

pub const PHYCLKRST_REFCLKSEL_PAD_REFCLK: c_uint = 0x2;
pub const PHYCLKRST_REFCLKSEL_EXT_REFCLK: c_uint = 0x3;

pub const EXYNOS5_DRD_PHYREG0: c_uint = 0x14;

pub const EXYNOS5_DRD_PHYREG1: c_uint = 0x18;

pub const EXYNOS5_DRD_PHYPARAM0: c_uint = 0x1c;

pub const PHYPARAM0_REF_LOSLEVEL_VAL: c_uint = 0x9;

pub const EXYNOS5_DRD_PHYPARAM1: c_uint = 0x20;

pub const PHYPARAM1_PCS_TXDEEMPH_VAL: c_uint = 0x1c;
pub const EXYNOS5_DRD_PHYTERM: c_uint = 0x24;
pub const EXYNOS5_DRD_PHYTEST: c_uint = 0x28;

pub const EXYNOS5_DRD_PHYADP: c_uint = 0x2c;
pub const EXYNOS5_DRD_PHYUTMICLKSEL: c_uint = 0x30;

pub const EXYNOS5_DRD_PHYRESUME: c_uint = 0x34;
pub const EXYNOS5_DRD_LINKPORT: c_uint = 0x44;

// USB 3.0 DRD PHY SS Function Control Reg; accessed by CR_PORT

// Exynos7870: USB DRD PHY registers
pub const EXYNOS7870_DRD_PHYPCSVAL: c_uint = 0x3C;

pub const EXYNOS7870_DRD_PHYPARAM2: c_uint = 0x50;

pub const EXYNOS7870_DRD_HSPHYCTRL: c_uint = 0x54;

pub const EXYNOS7870_DRD_HSPHYPLLTUNE: c_uint = 0x70;

// Exynos850: USB DRD PHY registers
pub const EXYNOS850_DRD_LINKCTRL: c_uint = 0x04;

pub const EXYNOS850_DRD_LINKPORT: c_uint = 0x08;

pub const EXYNOS850_DRD_CLKRST: c_uint = 0x20;
//
// On versions without SS ports (like E850), bit 3 is for the 2.0 phy (HS),
// while on versions with (like gs101), bits 2 and 3 are for the 3.0 phy (SS)
// and bits 12 & 13 for the 2.0 phy.
//

pub const EXYNOS850_DRD_SSPPLLCTL: c_uint = 0x30;

pub const EXYNOS850_DRD_UTMI: c_uint = 0x50;

pub const EXYNOS850_DRD_HSP: c_uint = 0x54;

pub const EXYNOS850_DRD_HSPPARACON: c_uint = 0x58;

pub const EXYNOS850_DRD_HSP_TEST: c_uint = 0x5c;

pub const EXYNOSAUTOV920_DRD_HSP_CLKRST: c_uint = 0x100;

pub const EXYNOSAUTOV920_DRD_HSPCTL: c_uint = 0x104;

pub const EXYNOSAUTOV920_DRD_HSP_TEST: c_uint = 0x10c;
pub const EXYNOSAUTOV920_DRD_HSPPLLTUNE: c_uint = 0x110;

// ExynosAutov920 phy usb31drd port reg
pub const EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL: c_uint = 0x000;

pub const EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0: c_uint = 0x0004;

pub const EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON1: c_uint = 0x0008;
pub const EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON2: c_uint = 0x000c;

pub const EXYNOSAUTOV920_USB31DRD_PHY_CONFIG0: c_uint = 0x100;

pub const EXYNOSAUTOV920_USB31DRD_PHY_CONFIG7: c_uint = 0x11c;

pub const EXYNOSAUTOV920_USB31DRD_PHY_CONFIG4: c_uint = 0x110;

// Exynos9 - GS101
pub const EXYNOS850_DRD_SECPMACTL: c_uint = 0x48;

// PMA registers
pub const EXYNOS9_PMA_USBDP_CMN_REG0008: c_uint = 0x0020;

pub const EXYNOS9_PMA_USBDP_CMN_REG00B8: c_uint = 0x02e0;

pub const EXYNOS9_PMA_USBDP_CMN_REG01C0: c_uint = 0x0700;

// these have similar register layout, for lanes 0 and 2
pub const EXYNOS9_PMA_USBDP_TRSV_REG03C3: c_uint = 0x0f0c;
pub const EXYNOS9_PMA_USBDP_TRSV_REG07C3: c_uint = 0x1f0c;

// TRSV_REG0413 and TRSV_REG0813 have similar register layout
pub const EXYNOS9_PMA_USBDP_TRSV_REG0413: c_uint = 0x104c;

pub const EXYNOS9_PMA_USBDP_TRSV_REG0813: c_uint = 0x204c;

// PCS registers
pub const EXYNOS9_PCS_NS_VEC_PS1_N1: c_uint = 0x010c;
pub const EXYNOS9_PCS_NS_VEC_PS2_N0: c_uint = 0x0110;
pub const EXYNOS9_PCS_NS_VEC_PS3_N0: c_uint = 0x0118;

pub const EXYNOS9_PCS_OUT_VEC_2: c_uint = 0x014c;
pub const EXYNOS9_PCS_OUT_VEC_3: c_uint = 0x0150;

pub const EXYNOS9_PCS_TIMEOUT_0: c_uint = 0x0170;
pub const EXYNOS9_PCS_TIMEOUT_3: c_uint = 0x017c;
pub const EXYNOS9_PCS_EBUF_PARAM: c_uint = 0x0304;

pub const EXYNOS9_PCS_BACK_END_MODE_VEC: c_uint = 0x030c;

pub const EXYNOS9_PCS_RX_CONTROL: c_uint = 0x03f0;

pub const EXYNOS9_PCS_RX_CONTROL_DEBUG: c_uint = 0x03f4;

pub const EXYNOS9_PCS_LOCAL_COEF: c_uint = 0x040c;

pub const EXYNOS9_PCS_HS_TX_COEF_MAP_0: c_uint = 0x0410;

pub const KHZ: c_int = 1000;

    .off = (o),		\
    .mask = (m),		\
    .val = (v),		\
    .region = PTR_PHY	\
    }

    .off = (o),		\
    .mask = (m),		\
    .val = (v),		\
    .region = PTR_PCS	\
    }

    .off = (o),		\
    .mask = (m),		\
    .val = (v),		\
    .region = PTR_PMA,	\
    }

    for (; (tune).region != PTR_INVALID; ++(tune))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos5_usbdrd_phy_tuning {
    pub off: u32,
    pub mask: u32,
    pub val: u32,
    pub region: c_char,
pub const PTR_INVALID: c_int = 0;
pub const PTR_PHY: c_int = 1;
pub const PTR_PCS: c_int = 2;
pub const PTR_PMA: c_int = 3;
}

    enum exynos5_usbdrd_phy_tuning_state {
    PTS_UTMI_POSTINIT,
    PTS_PIPE3_PREINIT,
    PTS_PIPE3_INIT,
    PTS_PIPE3_POSTINIT,
    PTS_PIPE3_POSTLOCK,
    PTS_MAX,
    };
    enum exynos5_usbdrd_phy_id {
    EXYNOS5_DRDPHY_UTMI,
    EXYNOS5_DRDPHY_PIPE3,
    EXYNOS5_DRDPHYS_NUM,
    };
    struct phy_usb_instance;
    struct exynos5_usbdrd_phy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos5_usbdrd_phy_config {
    pub id: u32,
    pub isolate): *mut *mut *mut void (phy_isol)(struct phy_usb_instance inst, bool,
    pub phy_drd): *mut *mut void (phy_init)(struct exynos5_usbdrd_phy,
    pub inst): *mut *mut unsigned int (set_refclk)(struct phy_usb_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos5_usbdrd_phy_drvdata {
    pub phy_cfg: *const exynos5_usbdrd_phy_config,
    pub phy_tunes: *const exynos5_usbdrd_phy_tuning,
    pub phy_ops: *const phy_ops,
    pub clk_names: *const *const c_char,
    pub n_clks: c_int,
    pub core_clk_names: *const *const c_char,
    pub n_core_clks: c_int,
    pub regulator_names: *const *const c_char,
    pub n_regulators: c_int,
    pub pmu_offset_usbdrd0_phy: u32,
    pub pmu_offset_usbdrd0_phy_ss: u32,
    pub pmu_offset_usbdrd1_phy: u32,
}

//
// struct exynos5_usbdrd_phy - driver data for USB 3.0 PHY
// @dev: pointer to device instance of this platform device
// @reg_phy: usb phy controller register memory base
// @reg_pcs: usb phy physical coding sublayer register memory base
// @reg_pma: usb phy physical media attachment register memory base
// @clks: clocks for register access
// @core_clks: core clocks for phy (ref, pipe3, utmi+, ITP, etc. as required)
// @drv_data: pointer to SoC level driver data structure
// @hs_phy: pointer to non-Samsung IP high-speed phy controller
// @phy_mutex: mutex protecting phy_init/exit & TCPC callbacks
// @phys: array for 'EXYNOS5_DRDPHYS_NUM' number of PHY
// instances each with its 'phy' and 'phy_cfg'.
// @extrefclk: frequency select settings when using 'separate
// reference clocks' for SS and HS operations
// @regulators: regulators for phy
// @sw: TypeC orientation switch handle
// @orientation: TypeC connector orientation - normal or flipped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos5_usbdrd_phy {
    pub dev: *mut device,
    pub reg_phy: *mut void __iomem,
    pub reg_pcs: *mut void __iomem,
    pub reg_pma: *mut void __iomem,
    pub clks: *mut clk_bulk_data,
    pub core_clks: *mut clk_bulk_data,
    pub drv_data: *const exynos5_usbdrd_phy_drvdata,
    pub hs_phy: *mut phy,
    pub phy_mutex: mutex,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_usb_instance {
    pub phy: *mut phy,
    pub index: u32,
    pub reg_pmu: *mut regmap,
    pub pmu_offset: u32,
    pub phy_cfg: *const exynos5_usbdrd_phy_config,
    pub phys: [}; EXYNOS5_DRDPHYS_NUM],
    pub extrefclk: u32,
    pub regulators: *mut regulator_bulk_data,
    pub sw: *mut typec_switch_dev,
    pub orientation: enum typec_orientation,
}

    static inline
    struct exynos5_usbdrd_phy *to_usbdrd_phy(struct phy_usb_instance *inst)
    {
    return container_of((inst), struct exynos5_usbdrd_phy,
    phys[(inst).index]);
    }
//
// exynos5_rate_to_clk() converts the supplied clock rate to the value that
// can be written to the phy register.
//
#[no_mangle]
unsafe extern "C" fn exynos5_rate_to_clk(rate: c_ulong, reg: *mut u32) -> c_uint {
    static unsigned int exynos5_rate_to_clk(unsigned long rate, u32 *reg)
    {
// EXYNOS5_FSEL_MASK
    switch (rate) {
    case 9600 * KHZ:
// reg = EXYNOS5_FSEL_9MHZ6;
    break;
    case 10 * MHZ:
// reg = EXYNOS5_FSEL_10MHZ;
    break;
    case 12 * MHZ:
// reg = EXYNOS5_FSEL_12MHZ;
    break;
    case 19200 * KHZ:
// reg = EXYNOS5_FSEL_19MHZ2;
    break;
    case 20 * MHZ:
// reg = EXYNOS5_FSEL_20MHZ;
    break;
    case 24 * MHZ:
// reg = EXYNOS5_FSEL_24MHZ;
    break;
    case 26 * MHZ:
// reg = EXYNOS5_FSEL_26MHZ;
    break;
    case 50 * MHZ:
// reg = EXYNOS5_FSEL_50MHZ;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static void exynos5_usbdrd_phy_isol(struct phy_usb_instance *inst,
    bool isolate)
    {
    unsigned int val;
    if (!inst.reg_pmu)
    return;
    val = isolate ? 0 : EXYNOS4_PHY_ENABLE;
    regmap_update_bits(inst.reg_pmu, inst.pmu_offset,
    EXYNOS4_PHY_ENABLE, val);
    }
//
// Sets the pipe3 phy's clk as EXTREFCLK (XXTI) which is internal clock
// from clock core. Further sets multiplier values and spread spectrum
// clock settings for SuperSpeed operations.
//
    static unsigned int
    exynos5_usbdrd_pipe3_set_refclk(struct phy_usb_instance *inst)
    {
    u32 reg;
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
// restore any previous reference clock settings
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
// Use EXTREFCLK as ref clock
    reg &= ~PHYCLKRST_REFCLKSEL;
    reg |= FIELD_PREP(PHYCLKRST_REFCLKSEL, PHYCLKRST_REFCLKSEL_EXT_REFCLK);
// FSEL settings corresponding to reference clock
    reg &= ~(PHYCLKRST_FSEL_PIPE |
    PHYCLKRST_MPLL_MULTIPLIER |
    PHYCLKRST_SSC_REFCLKSEL);
    switch (phy_drd.extrefclk) {
    case EXYNOS5_FSEL_50MHZ:
    reg |= (FIELD_PREP(PHYCLKRST_SSC_REFCLKSEL, 0x00) |
    FIELD_PREP(PHYCLKRST_MPLL_MULTIPLIER,
    PHYCLKRST_MPLL_MULTIPLIER_50M_REF));
    break;
    case EXYNOS5_FSEL_24MHZ:
    reg |= (FIELD_PREP(PHYCLKRST_SSC_REFCLKSEL, 0x88) |
    FIELD_PREP(PHYCLKRST_MPLL_MULTIPLIER,
    PHYCLKRST_MPLL_MULTIPLIER_24MHZ_REF));
    break;
    case EXYNOS5_FSEL_20MHZ:
    reg |= (FIELD_PREP(PHYCLKRST_SSC_REFCLKSEL, 0x00) |
    FIELD_PREP(PHYCLKRST_MPLL_MULTIPLIER,
    PHYCLKRST_MPLL_MULTIPLIER_20MHZ_REF));
    break;
    case EXYNOS5_FSEL_19MHZ2:
    reg |= (FIELD_PREP(PHYCLKRST_SSC_REFCLKSEL, 0x88) |
    FIELD_PREP(PHYCLKRST_MPLL_MULTIPLIER,
    PHYCLKRST_MPLL_MULTIPLIER_19200KHZ_REF));
    break;
    default:
    dev_dbg(phy_drd.dev, "unsupported ref clk\n");
    break;
    }
    return reg;
    }
//
// Sets the utmi phy's clk as EXTREFCLK (XXTI) which is internal clock
// from clock core. Further sets the FSEL values for HighSpeed operations.
//
    static unsigned int
    exynos5_usbdrd_utmi_set_refclk(struct phy_usb_instance *inst)
    {
    u32 reg;
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
// restore any previous reference clock settings
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
    reg &= ~PHYCLKRST_REFCLKSEL;
    reg |= FIELD_PREP(PHYCLKRST_REFCLKSEL, PHYCLKRST_REFCLKSEL_EXT_REFCLK);
    reg &= ~(PHYCLKRST_FSEL_UTMI |
    PHYCLKRST_MPLL_MULTIPLIER |
    PHYCLKRST_SSC_REFCLKSEL);
    reg |= FIELD_PREP(PHYCLKRST_FSEL_UTMI, phy_drd.extrefclk);
    return reg;
    }
    static void
    exynos5_usbdrd_apply_phy_tunes(struct exynos5_usbdrd_phy *phy_drd,
    enum exynos5_usbdrd_phy_tuning_state state)
    {
    const struct exynos5_usbdrd_phy_tuning *tune;
    tune = phy_drd.drv_data.phy_tunes[state];
    if (!tune)
    return;
    for_each_phy_tune(tune) {
    void __iomem *reg_base;
    let mut reg: u32 = 0;
    switch (tune.region) {
    case PTR_PHY:
    reg_base = phy_drd.reg_phy;
    break;
    case PTR_PCS:
    reg_base = phy_drd.reg_pcs;
    break;
    case PTR_PMA:
    reg_base = phy_drd.reg_pma;
    break;
    default:
    dev_warn_once(phy_drd.dev,
    "unknown phy region %d\n", tune.region);
    continue;
    }
    if (~tune.mask) {
    reg = readl(reg_base + tune.off);
    reg &= ~tune.mask;
    }
    reg |= tune.val;
    writel(reg, reg_base + tune.off);
    }
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_pipe3_init(phy_drd: *mut exynos5_usbdrd_phy) {
    static void exynos5_usbdrd_pipe3_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    u32 reg;
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM1);
// Set Tx De-Emphasis level
    reg &= ~PHYPARAM1_PCS_TXDEEMPH;
    reg |= FIELD_PREP(PHYPARAM1_PCS_TXDEEMPH, PHYPARAM1_PCS_TXDEEMPH_VAL);
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM1);
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYTEST);
    reg &= ~PHYTEST_POWERDOWN_SSP;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYTEST);
    }
    static void
    exynos5_usbdrd_usbdp_g2_v4_ctrl_pma_ready(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *regs_base = phy_drd.reg_phy;
    u32 reg;
// link pipe_clock selection to pclk of PMA
    reg = readl(regs_base + EXYNOS850_DRD_CLKRST);
    reg |= CLKRST_LINK_PCLK_SEL;
    writel(reg, regs_base + EXYNOS850_DRD_CLKRST);
    reg = readl(regs_base + EXYNOS850_DRD_SECPMACTL);
    reg &= ~SECPMACTL_PMA_REF_FREQ_SEL;
    reg |= FIELD_PREP(SECPMACTL_PMA_REF_FREQ_SEL, 1);
// SFR reset
    reg |= (SECPMACTL_PMA_LOW_PWR | SECPMACTL_PMA_APB_SW_RST);
    reg &= ~(SECPMACTL_PMA_ROPLL_REF_CLK_SEL |
    SECPMACTL_PMA_LCPLL_REF_CLK_SEL);
// PMA power off
    reg |= (SECPMACTL_PMA_TRSV_SW_RST | SECPMACTL_PMA_CMN_SW_RST |
    SECPMACTL_PMA_INIT_SW_RST);
    writel(reg, regs_base + EXYNOS850_DRD_SECPMACTL);
    udelay(1);
    reg = readl(regs_base + EXYNOS850_DRD_SECPMACTL);
    reg &= ~SECPMACTL_PMA_LOW_PWR;
    writel(reg, regs_base + EXYNOS850_DRD_SECPMACTL);
    udelay(1);
// release override
    reg = readl(regs_base + EXYNOS850_DRD_LINKCTRL);
    reg &= ~LINKCTRL_FORCE_PIPE_EN;
    writel(reg, regs_base + EXYNOS850_DRD_LINKCTRL);
    udelay(1);
// APB enable
    reg = readl(regs_base + EXYNOS850_DRD_SECPMACTL);
    reg &= ~SECPMACTL_PMA_APB_SW_RST;
    writel(reg, regs_base + EXYNOS850_DRD_SECPMACTL);
    }
    static void
    exynos5_usbdrd_usbdp_g2_v4_pma_lane_mux_sel(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *regs_base = phy_drd.reg_pma;
    u32 reg;
// lane configuration: USB on all lanes
    reg = readl(regs_base + EXYNOS9_PMA_USBDP_CMN_REG00B8);
    reg &= ~CMN_REG00B8_LANE_MUX_SEL_DP;
//
// USB on lanes 0 & 1 in normal mode, or 2 & 3 if reversed, DP on the
// other ones.
//
    reg |= FIELD_PREP(CMN_REG00B8_LANE_MUX_SEL_DP,
    ((phy_drd.orientation == TYPEC_ORIENTATION_NORMAL)
    ? (CMN_REG00B8_LANE_MUX_SEL_DP_LANE3
    | CMN_REG00B8_LANE_MUX_SEL_DP_LANE2)
    : (CMN_REG00B8_LANE_MUX_SEL_DP_LANE1
    | CMN_REG00B8_LANE_MUX_SEL_DP_LANE0)));
    writel(reg, regs_base + EXYNOS9_PMA_USBDP_CMN_REG00B8);
// override of TX receiver detector and comparator: lane 1
    reg = readl(regs_base + EXYNOS9_PMA_USBDP_TRSV_REG0413);
    if (phy_drd.orientation == TYPEC_ORIENTATION_NORMAL) {
    reg &= ~TRSV_REG0413_OVRD_LN1_TX_RXD_COMP_EN;
    reg &= ~TRSV_REG0413_OVRD_LN1_TX_RXD_EN;
    } else {
    reg |= TRSV_REG0413_OVRD_LN1_TX_RXD_COMP_EN;
    reg |= TRSV_REG0413_OVRD_LN1_TX_RXD_EN;
    }
    writel(reg, regs_base + EXYNOS9_PMA_USBDP_TRSV_REG0413);
// lane 3
    reg = readl(regs_base + EXYNOS9_PMA_USBDP_TRSV_REG0813);
    if (phy_drd.orientation == TYPEC_ORIENTATION_NORMAL) {
    reg |= TRSV_REG0813_OVRD_LN3_TX_RXD_COMP_EN;
    reg |= TRSV_REG0813_OVRD_LN3_TX_RXD_EN;
    } else {
    reg &= ~TRSV_REG0813_OVRD_LN3_TX_RXD_COMP_EN;
    reg &= ~TRSV_REG0813_OVRD_LN3_TX_RXD_EN;
    }
    writel(reg, regs_base + EXYNOS9_PMA_USBDP_TRSV_REG0813);
    }
    static int
    exynos5_usbdrd_usbdp_g2_v4_pma_check_pll_lock(struct exynos5_usbdrd_phy *phy_drd)
    {
    let mut timeout_us: static unsigned int = 40000;
    let mut sleep_us: static unsigned int = 40;
    static const u32 locked = (CMN_REG01C0_ANA_LCPLL_LOCK_DONE |
    CMN_REG01C0_ANA_LCPLL_AFC_DONE);
    u32 reg;
    int err;
    err = readl_poll_timeout(
    phy_drd.reg_pma + EXYNOS9_PMA_USBDP_CMN_REG01C0,
    reg, (reg & locked) == locked, sleep_us, timeout_us);
    if (err)
    dev_err(phy_drd.dev,
    "timed out waiting for PLL lock: %#.8x\n", reg);
    return err;
    }
    static void
    exynos5_usbdrd_usbdp_g2_v4_pma_check_cdr_lock(struct exynos5_usbdrd_phy *phy_drd)
    {
    let mut timeout_us: static unsigned int = 40000;
    let mut sleep_us: static unsigned int = 40;
    static const u32 locked =
    (TRSV_REG03C3_LN0_MON_RX_CDR_AFC_DONE
    | TRSV_REG03C3_LN0_MON_RX_CDR_CAL_DONE
    | TRSV_REG03C3_LN0_MON_RX_CDR_FLD_PLL_MODE_DONE
    | TRSV_REG03C3_LN0_MON_RX_CDR_LOCK_DONE);
    u32 reg;
    int err;
    err = readl_poll_timeout(
// lane depends on cable orientation
    (phy_drd.reg_pma
    + ((phy_drd.orientation == TYPEC_ORIENTATION_NORMAL)
    ? EXYNOS9_PMA_USBDP_TRSV_REG03C3
    : EXYNOS9_PMA_USBDP_TRSV_REG07C3)),
    reg, (reg & locked) == locked, sleep_us, timeout_us);
    if (err)
    dev_err(phy_drd.dev,
    "timed out waiting for CDR(l%d) lock: %#.8x\n",
    ((phy_drd.orientation == TYPEC_ORIENTATION_NORMAL)
    ? 0
    : 2), reg);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_utmi_init(phy_drd: *mut exynos5_usbdrd_phy) {
    static void exynos5_usbdrd_utmi_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    u32 reg;
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM0);
// Set Loss-of-Signal Detector sensitivity
    reg &= ~PHYPARAM0_REF_LOSLEVEL;
    reg |= FIELD_PREP(PHYPARAM0_REF_LOSLEVEL, PHYPARAM0_REF_LOSLEVEL_VAL);
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM0);
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM1);
// Set Tx De-Emphasis level
    reg &= ~PHYPARAM1_PCS_TXDEEMPH;
    reg |= FIELD_PREP(PHYPARAM1_PCS_TXDEEMPH, PHYPARAM1_PCS_TXDEEMPH_VAL);
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM1);
// UTMI Power Control
    writel(PHYUTMI_OTGDISABLE, phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMI);
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYTEST);
    reg &= ~PHYTEST_POWERDOWN_HSP;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYTEST);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_phy_init(phy: *mut phy) -> c_int {
    static int exynos5_usbdrd_phy_init(struct phy *phy)
    {
    int ret;
    u32 reg;
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
// Reset USB 3.0 PHY
    writel(0x0, phy_drd.reg_phy + EXYNOS5_DRD_PHYREG0);
    writel(0x0, phy_drd.reg_phy + EXYNOS5_DRD_PHYRESUME);
//
// Setting the Frame length Adj value[6:1] to default 0x20
// See xHCI 1.0 spec, 5.2.4
//
    reg =	LINKSYSTEM_XHCI_VERSION_CONTROL |
    FIELD_PREP(LINKSYSTEM_FLADJ, 0x20);
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_LINKSYSTEM);
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM0);
// Select PHY CLK source
    reg &= ~PHYPARAM0_REF_USE_PAD;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYPARAM0);
// This bit must be set for both HS and SS operations
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMICLKSEL);
    reg |= PHYUTMICLKSEL_UTMI_CLKSEL;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMICLKSEL);
// UTMI or PIPE3 specific init
    inst.phy_cfg.phy_init(phy_drd);
// reference clock settings
    reg = inst.phy_cfg.set_refclk(inst);
// Digital power supply in normal operating mode
    reg |=	PHYCLKRST_RETENABLEN |
// Enable ref clock for SS function
    PHYCLKRST_REF_SSP_EN |
// Enable spread spectrum
    PHYCLKRST_SSC_EN |
// Power down HS Bias and PLL blocks in suspend mode
    PHYCLKRST_COMMONONN |
// Reset the port
    PHYCLKRST_PORTRESET;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
    fsleep(10);
    reg &= ~PHYCLKRST_PORTRESET;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_phy_exit(phy: *mut phy) -> c_int {
    static int exynos5_usbdrd_phy_exit(struct phy *phy)
    {
    int ret;
    u32 reg;
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
    reg =	PHYUTMI_OTGDISABLE |
    PHYUTMI_FORCESUSPEND |
    PHYUTMI_FORCESLEEP;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMI);
// Resetting the PHYCLKRST enable bits to reduce leakage current
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
    reg &= ~(PHYCLKRST_REF_SSP_EN |
    PHYCLKRST_SSC_EN |
    PHYCLKRST_COMMONONN);
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
// Control PHYTEST to remove leakage current
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYTEST);
    reg |=	PHYTEST_POWERDOWN_SSP |
    PHYTEST_POWERDOWN_HSP;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYTEST);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_phy_power_on(phy: *mut phy) -> c_int {
    static int exynos5_usbdrd_phy_power_on(struct phy *phy)
    {
    int ret;
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    dev_dbg(phy_drd.dev, "Request to power_on usbdrd_phy phy\n");
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_core_clks,
    phy_drd.core_clks);
    if (ret)
    return ret;
// Enable VBUS supply
    ret = regulator_bulk_enable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    if (ret) {
    dev_err(phy_drd.dev, "Failed to enable PHY regulator(s)\n");
    goto fail_vbus;
    }
// Power-on PHY
    inst.phy_cfg.phy_isol(inst, false);
    return 0;
    fail_vbus:
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_core_clks,
    phy_drd.core_clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_phy_power_off(phy: *mut phy) -> c_int {
    static int exynos5_usbdrd_phy_power_off(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    dev_dbg(phy_drd.dev, "Request to power_off usbdrd_phy phy\n");
// Power-off the PHY
    inst.phy_cfg.phy_isol(inst, true);
// Disable VBUS supply
    regulator_bulk_disable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_core_clks,
    phy_drd.core_clks);
    return 0;
    }
    static int crport_handshake(struct exynos5_usbdrd_phy *phy_drd,
    u32 val, u32 cmd)
    {
    unsigned int result;
    int err;
    writel(val | cmd, phy_drd.reg_phy + EXYNOS5_DRD_PHYREG0);
    err = readl_poll_timeout(phy_drd.reg_phy + EXYNOS5_DRD_PHYREG1,
    result, (result & PHYREG1_CR_ACK), 1, 100);
    if (err == -ETIMEDOUT) {
    dev_err(phy_drd.dev, "CRPORT handshake timeout1 (0x%08x)\n", val);
    return err;
    }
    writel(val, phy_drd.reg_phy + EXYNOS5_DRD_PHYREG0);
    err = readl_poll_timeout(phy_drd.reg_phy + EXYNOS5_DRD_PHYREG1,
    result, !(result & PHYREG1_CR_ACK), 1, 100);
    if (err == -ETIMEDOUT) {
    dev_err(phy_drd.dev, "CRPORT handshake timeout2 (0x%08x)\n", val);
    return err;
    }
    return 0;
    }
    static int crport_ctrl_write(struct exynos5_usbdrd_phy *phy_drd,
    u32 addr, u32 data)
    {
    u32 val;
    int ret;
// Write Address
    val = FIELD_PREP(PHYREG0_CR_DATA_IN, addr);
    writel(val, phy_drd.reg_phy + EXYNOS5_DRD_PHYREG0);
    ret = crport_handshake(phy_drd, val, PHYREG0_CR_CAP_ADDR);
    if (ret)
    return ret;
// Write Data
    val = FIELD_PREP(PHYREG0_CR_DATA_IN, data);
    writel(val, phy_drd.reg_phy + EXYNOS5_DRD_PHYREG0);
    ret = crport_handshake(phy_drd, val, PHYREG0_CR_CAP_DATA);
    if (ret)
    return ret;
    ret = crport_handshake(phy_drd, val, PHYREG0_CR_WRITE);
    return ret;
    }
//
// Calibrate few PHY parameters using CR_PORT register to meet
// SuperSpeed requirements on Exynos5420 and Exynos5800 systems,
// which have 28nm USB 3.0 DRD PHY.
//
#[no_mangle]
unsafe extern "C" fn exynos5420_usbdrd_phy_calibrate(phy_drd: *mut exynos5_usbdrd_phy) -> c_int {
    static int exynos5420_usbdrd_phy_calibrate(struct exynos5_usbdrd_phy *phy_drd)
    {
    unsigned int temp;
    let mut ret: c_int = 0;
//
// Change los_bias to (0x5) for 28nm PHY from a
// default value (0x0); los_level is set as default
// (0x9) as also reflected in los_level[30:26] bits
// of PHYPARAM0 register.
//
    temp = LOSLEVEL_OVRD_IN_LOS_BIAS_5420 |
    LOSLEVEL_OVRD_IN_EN |
    LOSLEVEL_OVRD_IN_LOS_LEVEL_DEFAULT;
    ret = crport_ctrl_write(phy_drd,
    EXYNOS5_DRD_PHYSS_LOSLEVEL_OVRD_IN,
    temp);
    if (ret) {
    dev_err(phy_drd.dev,
    "Failed setting Loss-of-Signal level for SuperSpeed\n");
    return ret;
    }
//
// Set tx_vboost_lvl to (0x5) for 28nm PHY Tuning,
// to raise Tx signal level from its default value of (0x4)
//
    temp = TX_VBOOSTLEVEL_OVRD_IN_VBOOST_5420;
    ret = crport_ctrl_write(phy_drd,
    EXYNOS5_DRD_PHYSS_TX_VBOOSTLEVEL_OVRD_IN,
    temp);
    if (ret) {
    dev_err(phy_drd.dev,
    "Failed setting Tx-Vboost-Level for SuperSpeed\n");
    return ret;
    }
//
// Set proper time to wait for RxDetect measurement, for
// desired reference clock of PHY, by tuning the CR_PORT
// register LANE0.TX_DEBUG which is internal to PHY.
// This fixes issue with few USB 3.0 devices, which are
// not detected (not even generate interrupts on the bus
// on insertion) without this change.
// e.g. Samsung SUM-TSB16S 3.0 USB drive.
//
    switch (phy_drd.extrefclk) {
    case EXYNOS5_FSEL_50MHZ:
    temp = LANE0_TX_DEBUG_RXDET_MEAS_TIME_48M_50M_52M;
    break;
    case EXYNOS5_FSEL_20MHZ:
    case EXYNOS5_FSEL_19MHZ2:
    temp = LANE0_TX_DEBUG_RXDET_MEAS_TIME_19M2_20M;
    break;
    case EXYNOS5_FSEL_24MHZ:
    default:
    temp = LANE0_TX_DEBUG_RXDET_MEAS_TIME_24M;
    break;
    }
    ret = crport_ctrl_write(phy_drd,
    EXYNOS5_DRD_PHYSS_LANE0_TX_DEBUG,
    temp);
    if (ret)
    dev_err(phy_drd.dev,
    "Fail to set RxDet measurement time for SuperSpeed\n");
    return ret;
    }
    static struct phy *exynos5_usbdrd_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct exynos5_usbdrd_phy *phy_drd = dev_get_drvdata(dev);
    if (WARN_ON(args.args[0] >= EXYNOS5_DRDPHYS_NUM))
    return ERR_PTR(-ENODEV);
    return phy_drd.phys[args.args[0]].phy;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_phy_calibrate(phy: *mut phy) -> c_int {
    static int exynos5_usbdrd_phy_calibrate(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI)
    return exynos5420_usbdrd_phy_calibrate(phy_drd);
    return 0;
    }
    static const struct phy_ops exynos5_usbdrd_phy_ops = {
    .init		= exynos5_usbdrd_phy_init,
    .exit		= exynos5_usbdrd_phy_exit,
    .power_on	= exynos5_usbdrd_phy_power_on,
    .power_off	= exynos5_usbdrd_phy_power_off,
    .calibrate	= exynos5_usbdrd_phy_calibrate,
    .owner		= THIS_MODULE,
    };
    static void exynos7870_usbdrd_phy_isol(struct phy_usb_instance *inst,
    bool isolate)
    {
    unsigned int val;
    if (!inst.reg_pmu)
    return;
    val = isolate ? 0 : EXYNOS7870_USB2PHY_ENABLE;
    regmap_update_bits(inst.reg_pmu, inst.pmu_offset,
    EXYNOS7870_USB2PHY_ENABLE, val);
    }
#[no_mangle]
unsafe extern "C" fn exynos7870_usbdrd_utmi_init(phy_drd: *mut exynos5_usbdrd_phy) {
    static void exynos7870_usbdrd_utmi_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    u32 reg;
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
// Use PADREFCLK as ref clock
    reg &= ~PHYCLKRST_REFCLKSEL;
    reg |= FIELD_PREP(PHYCLKRST_REFCLKSEL, PHYCLKRST_REFCLKSEL_PAD_REFCLK);
// Select ref clock rate
    reg &= ~PHYCLKRST_FSEL_UTMI;
    reg &= ~PHYCLKRST_FSEL_PIPE;
    reg |= FIELD_PREP(PHYCLKRST_FSEL_UTMI, phy_drd.extrefclk);
// Enable suspend and reset the port
    reg |= PHYCLKRST_EN_UTMISUSPEND;
    reg |= PHYCLKRST_COMMONONN;
    reg |= PHYCLKRST_PORTRESET;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
    udelay(10);
// Clear the port reset bit
    reg &= ~PHYCLKRST_PORTRESET;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYCLKRST);
// Change PHY PLL tune value
    reg = readl(phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYPLLTUNE);
    if (phy_drd.extrefclk == EXYNOS5_FSEL_24MHZ)
    reg |= HSPHYPLLTUNE_PLL_B_TUNE;
    else
    reg &= ~HSPHYPLLTUNE_PLL_B_TUNE;
    reg &= ~HSPHYPLLTUNE_PLL_P_TUNE;
    reg |= FIELD_PREP(HSPHYPLLTUNE_PLL_P_TUNE, 14);
    writel(reg, phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYPLLTUNE);
// High-Speed PHY control
    reg = readl(phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
    reg &= ~HSPHYCTRL_SIDDQ;
    reg &= ~HSPHYCTRL_PHYSWRST;
    reg &= ~HSPHYCTRL_PHYSWRSTALL;
    writel(reg, phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
    udelay(500);
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_LINKSYSTEM);
//
// Setting the Frame length Adj value[6:1] to default 0x20
// See xHCI 1.0 spec, 5.2.4
//
    reg |= LINKSYSTEM_XHCI_VERSION_CONTROL;
    reg &= ~LINKSYSTEM_FLADJ;
    reg |= FIELD_PREP(LINKSYSTEM_FLADJ, 0x20);
// Set VBUSVALID signal as the VBUS pad is not used
    reg |= LINKSYSTEM_FORCE_BVALID;
    reg |= LINKSYSTEM_FORCE_VBUSVALID;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_LINKSYSTEM);
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMI);
// Release force_sleep & force_suspend
    reg &= ~PHYUTMI_FORCESLEEP;
    reg &= ~PHYUTMI_FORCESUSPEND;
// DP/DM pull down control
    reg &= ~PHYUTMI_DMPULLDOWN;
    reg &= ~PHYUTMI_DPPULLDOWN;
    reg &= ~PHYUTMI_DRVVBUS;
// Set DP-pull up as the VBUS pad is not used
    reg |= PHYUTMI_VBUSVLDEXTSEL;
    reg |= PHYUTMI_VBUSVLDEXT;
// Disable OTG block and VBUS valid comparator
    reg |= PHYUTMI_OTGDISABLE;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMI);
// Configure OVC IO usage
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_LINKPORT);
    reg |= LINKPORT_HOST_PORT_OVCR_U3_SEL | LINKPORT_HOST_PORT_OVCR_U2_SEL;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_LINKPORT);
// High-Speed PHY swrst
    reg = readl(phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
    reg |= HSPHYCTRL_PHYSWRST;
    writel(reg, phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
    udelay(20);
// Clear the PHY swrst bit
    reg = readl(phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
    reg &= ~HSPHYCTRL_PHYSWRST;
    writel(reg, phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
    if (phy_drd.drv_data.phy_tunes)
    exynos5_usbdrd_apply_phy_tunes(phy_drd,
    PTS_UTMI_POSTINIT);
    }
#[no_mangle]
unsafe extern "C" fn exynos7870_usbdrd_phy_init(phy: *mut phy) -> c_int {
    static int exynos7870_usbdrd_phy_init(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
// UTMI or PIPE3 specific init
    inst.phy_cfg.phy_init(phy_drd);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos7870_usbdrd_phy_exit(phy: *mut phy) -> c_int {
    static int exynos7870_usbdrd_phy_exit(struct phy *phy)
    {
    int ret;
    u32 reg;
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
//
// Disable the VBUS signal and the ID pull-up resistor.
// Enable force-suspend and force-sleep modes.
//
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMI);
    reg &= ~(PHYUTMI_DRVVBUS | PHYUTMI_VBUSVLDEXT | PHYUTMI_VBUSVLDEXTSEL);
    reg &= ~PHYUTMI_IDPULLUP;
    reg |= PHYUTMI_FORCESUSPEND | PHYUTMI_FORCESLEEP;
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_PHYUTMI);
// Power down PHY analog blocks
    reg = readl(phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
    reg |= HSPHYCTRL_SIDDQ;
    writel(reg, phy_drd.reg_phy + EXYNOS7870_DRD_HSPHYCTRL);
// Clear VBUSVALID signal as the VBUS pad is not used
    reg = readl(phy_drd.reg_phy + EXYNOS5_DRD_LINKSYSTEM);
    reg &= ~(LINKSYSTEM_FORCE_BVALID | LINKSYSTEM_FORCE_VBUSVALID);
    writel(reg, phy_drd.reg_phy + EXYNOS5_DRD_LINKSYSTEM);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
    static const struct phy_ops exynos7870_usbdrd_phy_ops = {
    .init		= exynos7870_usbdrd_phy_init,
    .exit		= exynos7870_usbdrd_phy_exit,
    .power_on	= exynos5_usbdrd_phy_power_on,
    .power_off	= exynos5_usbdrd_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn exynos2200_usbdrd_utmi_init(phy_drd: *mut exynos5_usbdrd_phy) {
    static void exynos2200_usbdrd_utmi_init(struct exynos5_usbdrd_phy *phy_drd)
    {
// Configure non-Samsung IP PHY, responsible for UTMI
    phy_init(phy_drd.hs_phy);
    }
#[no_mangle]
unsafe extern "C" fn exynos2200_usbdrd_link_init(phy_drd: *mut exynos5_usbdrd_phy) {
    static void exynos2200_usbdrd_link_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *regs_base = phy_drd.reg_phy;
    u32 reg;
//
// Disable HWACG (hardware auto clock gating control). This will force
// QACTIVE signal in Q-Channel interface to HIGH level, to make sure
// the PHY clock is not gated by the hardware.
//
    reg = readl(regs_base + EXYNOS850_DRD_LINKCTRL);
    reg |= LINKCTRL_FORCE_QACT;
    writel(reg, regs_base + EXYNOS850_DRD_LINKCTRL);
// De-assert link reset
    reg = readl(regs_base + EXYNOS2200_DRD_CLKRST);
    reg &= ~CLKRST_LINK_SW_RST;
    writel(reg, regs_base + EXYNOS2200_DRD_CLKRST);
// Set link VBUS Valid
    reg = readl(regs_base + EXYNOS2200_DRD_UTMI);
    reg |= EXYNOS2200_UTMI_FORCE_BVALID | EXYNOS2200_UTMI_FORCE_VBUSVALID;
    writel(reg, regs_base + EXYNOS2200_DRD_UTMI);
    }
    static void
    exynos2200_usbdrd_link_attach_detach_pipe3_phy(struct phy_usb_instance *inst)
    {
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    void __iomem *regs_base = phy_drd.reg_phy;
    u32 reg;
    reg = readl(regs_base + EXYNOS850_DRD_LINKCTRL);
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI) {
// force pipe3 signal for link
    reg &= ~LINKCTRL_FORCE_PHYSTATUS;
    reg |= LINKCTRL_FORCE_PIPE_EN | LINKCTRL_FORCE_RXELECIDLE;
    } else {
// disable forcing pipe interface
    reg &= ~LINKCTRL_FORCE_PIPE_EN;
    }
    writel(reg, regs_base + EXYNOS850_DRD_LINKCTRL);
    reg = readl(regs_base + EXYNOS2200_DRD_HSP_MISC);
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI) {
// calibrate only eUSB phy
    reg |= FIELD_PREP(HSP_MISC_RES_TUNE, RES_TUNE_PHY1);
    reg |= HSP_MISC_SET_REQ_IN2;
    } else {
// calibrate for dual phy
    reg |= FIELD_PREP(HSP_MISC_RES_TUNE, RES_TUNE_PHY1_PHY2);
    reg &= ~HSP_MISC_SET_REQ_IN2;
    }
    writel(reg, regs_base + EXYNOS2200_DRD_HSP_MISC);
    reg = readl(regs_base + EXYNOS2200_DRD_CLKRST);
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI)
    reg &= ~EXYNOS2200_CLKRST_LINK_PCLK_SEL;
    else
    reg |= EXYNOS2200_CLKRST_LINK_PCLK_SEL;
    writel(reg, regs_base + EXYNOS2200_DRD_CLKRST);
    }
#[no_mangle]
unsafe extern "C" fn exynos2200_usbdrd_phy_init(phy: *mut phy) -> c_int {
    static int exynos2200_usbdrd_phy_init(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI) {
// Power-on PHY ...
    ret = regulator_bulk_enable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    if (ret) {
    dev_err(phy_drd.dev,
    "Failed to enable PHY regulator(s)\n");
    return ret;
    }
    }
//
// ... and ungate power via PMU. Without this here, we get an SError
// trying to access PMA registers
//
    exynos5_usbdrd_phy_isol(inst, false);
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
// Set up the link controller
    exynos2200_usbdrd_link_init(phy_drd);
// UTMI or PIPE3 link preparation
    exynos2200_usbdrd_link_attach_detach_pipe3_phy(inst);
// UTMI or PIPE3 specific init
    inst.phy_cfg.phy_init(phy_drd);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos2200_usbdrd_phy_exit(phy: *mut phy) -> c_int {
    static int exynos2200_usbdrd_phy_exit(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    void __iomem *regs_base = phy_drd.reg_phy;
    u32 reg;
    int ret;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
    reg = readl(regs_base + EXYNOS2200_DRD_UTMI);
    reg &= ~(EXYNOS2200_UTMI_FORCE_BVALID | EXYNOS2200_UTMI_FORCE_VBUSVALID);
    writel(reg, regs_base + EXYNOS2200_DRD_UTMI);
    reg = readl(regs_base + EXYNOS2200_DRD_CLKRST);
    reg |= CLKRST_LINK_SW_RST;
    writel(reg, regs_base + EXYNOS2200_DRD_CLKRST);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    exynos5_usbdrd_phy_isol(inst, true);
    return regulator_bulk_disable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    }
    static const struct phy_ops exynos2200_usbdrd_phy_ops = {
    .init		= exynos2200_usbdrd_phy_init,
    .exit		= exynos2200_usbdrd_phy_exit,
    .owner		= THIS_MODULE,
    };
    static void
    exynos5_usbdrd_usb_v3p1_pipe_override(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *regs_base = phy_drd.reg_phy;
    u32 reg;
// force pipe3 signal for link
    reg = readl(regs_base + EXYNOS850_DRD_LINKCTRL);
    reg &= ~LINKCTRL_FORCE_PHYSTATUS;
    reg |= LINKCTRL_FORCE_PIPE_EN | LINKCTRL_FORCE_RXELECIDLE;
    writel(reg, regs_base + EXYNOS850_DRD_LINKCTRL);
// PMA disable
    reg = readl(regs_base + EXYNOS850_DRD_SECPMACTL);
    reg |= SECPMACTL_PMA_LOW_PWR;
    writel(reg, regs_base + EXYNOS850_DRD_SECPMACTL);
    }
#[no_mangle]
unsafe extern "C" fn exynos850_usbdrd_utmi_init(phy_drd: *mut exynos5_usbdrd_phy) {
    static void exynos850_usbdrd_utmi_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *regs_base = phy_drd.reg_phy;
    u32 reg;
    u32 ss_ports;
//
// Disable HWACG (hardware auto clock gating control). This will force
// QACTIVE signal in Q-Channel interface to HIGH level, to make sure
// the PHY clock is not gated by the hardware.
//
    reg = readl(regs_base + EXYNOS850_DRD_LINKCTRL);
    reg |= LINKCTRL_FORCE_QACT;
    writel(reg, regs_base + EXYNOS850_DRD_LINKCTRL);
    reg = readl(regs_base + EXYNOS850_DRD_LINKPORT);
    ss_ports = FIELD_GET(LINKPORT_HOST_NUM_U3, reg);
// Start PHY Reset (POR=high)
    reg = readl(regs_base + EXYNOS850_DRD_CLKRST);
    if (ss_ports) {
    reg |= CLKRST_PHY20_SW_POR;
    reg |= CLKRST_PHY20_SW_POR_SEL;
    reg |= CLKRST_PHY_RESET_SEL;
    }
    reg |= CLKRST_PHY_SW_RST;
    writel(reg, regs_base + EXYNOS850_DRD_CLKRST);
// Enable UTMI+
    reg = readl(regs_base + EXYNOS850_DRD_UTMI);
    reg &= ~(UTMI_FORCE_SUSPEND | UTMI_FORCE_SLEEP | UTMI_DP_PULLDOWN |
    UTMI_DM_PULLDOWN);
    writel(reg, regs_base + EXYNOS850_DRD_UTMI);
// Set PHY clock and control HS PHY
    reg = readl(regs_base + EXYNOS850_DRD_HSP);
    reg |= HSP_EN_UTMISUSPEND | HSP_COMMONONN;
    writel(reg, regs_base + EXYNOS850_DRD_HSP);
// Set VBUS Valid and D+ pull-up control by VBUS pad usage
    reg = readl(regs_base + EXYNOS850_DRD_LINKCTRL);
    reg |= FIELD_PREP(LINKCTRL_BUS_FILTER_BYPASS, 0xf);
    writel(reg, regs_base + EXYNOS850_DRD_LINKCTRL);
    if (!phy_drd.sw) {
    reg = readl(regs_base + EXYNOS850_DRD_UTMI);
    reg |= UTMI_FORCE_BVALID | UTMI_FORCE_VBUSVALID;
    writel(reg, regs_base + EXYNOS850_DRD_UTMI);
    reg = readl(regs_base + EXYNOS850_DRD_HSP);
    reg |= HSP_VBUSVLDEXT | HSP_VBUSVLDEXTSEL;
    writel(reg, regs_base + EXYNOS850_DRD_HSP);
    }
    reg = readl(regs_base + EXYNOS850_DRD_SSPPLLCTL);
    reg &= ~SSPPLLCTL_FSEL;
    switch (phy_drd.extrefclk) {
    case EXYNOS5_FSEL_50MHZ:
    reg |= FIELD_PREP(SSPPLLCTL_FSEL, 7);
    break;
    case EXYNOS5_FSEL_26MHZ:
    reg |= FIELD_PREP(SSPPLLCTL_FSEL, 6);
    break;
    case EXYNOS5_FSEL_24MHZ:
    reg |= FIELD_PREP(SSPPLLCTL_FSEL, 2);
    break;
    case EXYNOS5_FSEL_20MHZ:
    reg |= FIELD_PREP(SSPPLLCTL_FSEL, 1);
    break;
    case EXYNOS5_FSEL_19MHZ2:
    reg |= FIELD_PREP(SSPPLLCTL_FSEL, 0);
    break;
    default:
    dev_warn(phy_drd.dev, "unsupported ref clk: %#.2x\n",
    phy_drd.extrefclk);
    break;
    }
    writel(reg, regs_base + EXYNOS850_DRD_SSPPLLCTL);
    if (phy_drd.drv_data.phy_tunes)
    exynos5_usbdrd_apply_phy_tunes(phy_drd,
    PTS_UTMI_POSTINIT);
// Power up PHY analog blocks
    reg = readl(regs_base + EXYNOS850_DRD_HSP_TEST);
    reg &= ~HSP_TEST_SIDDQ;
    writel(reg, regs_base + EXYNOS850_DRD_HSP_TEST);
// Finish PHY reset (POR=low)
    fsleep(10); /* required before doing POR=low */
    reg = readl(regs_base + EXYNOS850_DRD_CLKRST);
    if (ss_ports) {
    reg |= CLKRST_PHY20_SW_POR_SEL;
    reg &= ~CLKRST_PHY20_SW_POR;
    }
    reg &= ~(CLKRST_PHY_SW_RST | CLKRST_PORT_RST);
    writel(reg, regs_base + EXYNOS850_DRD_CLKRST);
    fsleep(75); /* required after POR=low for guaranteed PHY clock */
// Disable single ended signal out
    reg = readl(regs_base + EXYNOS850_DRD_HSP);
    reg &= ~HSP_FSV_OUT_EN;
    writel(reg, regs_base + EXYNOS850_DRD_HSP);
    if (ss_ports)
    exynos5_usbdrd_usb_v3p1_pipe_override(phy_drd);
    }
#[no_mangle]
unsafe extern "C" fn exynos850_usbdrd_phy_init(phy: *mut phy) -> c_int {
    static int exynos850_usbdrd_phy_init(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
// UTMI or PIPE3 specific init
    scoped_guard(mutex, &phy_drd.phy_mutex)
    inst.phy_cfg.phy_init(phy_drd);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos850_usbdrd_phy_exit(phy: *mut phy) -> c_int {
    static int exynos850_usbdrd_phy_exit(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    void __iomem *regs_base = phy_drd.reg_phy;
    u32 reg;
    int ret;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
    guard(mutex)(&phy_drd.phy_mutex);
// Set PHY clock and control HS PHY
    reg = readl(regs_base + EXYNOS850_DRD_UTMI);
    reg &= ~(UTMI_DP_PULLDOWN | UTMI_DM_PULLDOWN);
    reg |= UTMI_FORCE_SUSPEND | UTMI_FORCE_SLEEP;
    writel(reg, regs_base + EXYNOS850_DRD_UTMI);
// Power down PHY analog blocks
    reg = readl(regs_base + EXYNOS850_DRD_HSP_TEST);
    reg |= HSP_TEST_SIDDQ;
    writel(reg, regs_base + EXYNOS850_DRD_HSP_TEST);
// Link reset
    reg = readl(regs_base + EXYNOS850_DRD_CLKRST);
    reg |= CLKRST_LINK_SW_RST;
    writel(reg, regs_base + EXYNOS850_DRD_CLKRST);
    fsleep(10); /* required before doing POR=low */
    reg &= ~CLKRST_LINK_SW_RST;
    writel(reg, regs_base + EXYNOS850_DRD_CLKRST);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
    static const struct phy_ops exynos850_usbdrd_phy_ops = {
    .init		= exynos850_usbdrd_phy_init,
    .exit		= exynos850_usbdrd_phy_exit,
    .power_on	= exynos5_usbdrd_phy_power_on,
    .power_off	= exynos5_usbdrd_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_gs101_pipe3_init(phy_drd: *mut exynos5_usbdrd_phy) {
    static void exynos5_usbdrd_gs101_pipe3_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *regs_pma = phy_drd.reg_pma;
    void __iomem *regs_phy = phy_drd.reg_phy;
    u32 reg;
    exynos5_usbdrd_usbdp_g2_v4_ctrl_pma_ready(phy_drd);
// force aux off
    reg = readl(regs_pma + EXYNOS9_PMA_USBDP_CMN_REG0008);
    reg &= ~CMN_REG0008_AUX_EN;
    reg |= CMN_REG0008_OVRD_AUX_EN;
    writel(reg, regs_pma + EXYNOS9_PMA_USBDP_CMN_REG0008);
    exynos5_usbdrd_apply_phy_tunes(phy_drd, PTS_PIPE3_PREINIT);
    exynos5_usbdrd_apply_phy_tunes(phy_drd, PTS_PIPE3_INIT);
    exynos5_usbdrd_apply_phy_tunes(phy_drd, PTS_PIPE3_POSTINIT);
    exynos5_usbdrd_usbdp_g2_v4_pma_lane_mux_sel(phy_drd);
// reset release from port
    reg = readl(regs_phy + EXYNOS850_DRD_SECPMACTL);
    reg &= ~(SECPMACTL_PMA_TRSV_SW_RST | SECPMACTL_PMA_CMN_SW_RST |
    SECPMACTL_PMA_INIT_SW_RST);
    writel(reg, regs_phy + EXYNOS850_DRD_SECPMACTL);
    if (!exynos5_usbdrd_usbdp_g2_v4_pma_check_pll_lock(phy_drd))
    exynos5_usbdrd_usbdp_g2_v4_pma_check_cdr_lock(phy_drd);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_gs101_phy_init(phy: *mut phy) -> c_int {
    static int exynos5_usbdrd_gs101_phy_init(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI) {
// Power-on PHY ...
    ret = regulator_bulk_enable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    if (ret) {
    dev_err(phy_drd.dev,
    "Failed to enable PHY regulator(s)\n");
    return ret;
    }
    }
//
// ... and ungate power via PMU. Without this here, we get an SError
// trying to access PMA registers
//
    exynos5_usbdrd_phy_isol(inst, false);
    return exynos850_usbdrd_phy_init(phy);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_gs101_phy_exit(phy: *mut phy) -> c_int {
    static int exynos5_usbdrd_gs101_phy_exit(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI) {
    ret = exynos850_usbdrd_phy_exit(phy);
    if (ret)
    return ret;
    }
    exynos5_usbdrd_phy_isol(inst, true);
    if (inst.phy_cfg.id != EXYNOS5_DRDPHY_UTMI)
    return 0;
    return regulator_bulk_disable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    }
    static const struct phy_ops gs101_usbdrd_phy_ops = {
    .init		= exynos5_usbdrd_gs101_phy_init,
    .exit		= exynos5_usbdrd_gs101_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_phy_clk_handle(phy_drd: *mut exynos5_usbdrd_phy) -> c_int {
    static int exynos5_usbdrd_phy_clk_handle(struct exynos5_usbdrd_phy *phy_drd)
    {
    int ret;
    struct clk *ref_clk;
    unsigned long ref_rate;
    phy_drd.clks = devm_kcalloc(phy_drd.dev, phy_drd.drv_data.n_clks,
    sizeof(*phy_drd.clks), GFP_KERNEL);
    if (!phy_drd.clks)
    return -ENOMEM;
    for (int i = 0; i < phy_drd.drv_data.n_clks; ++i)
    phy_drd.clks[i].id = phy_drd.drv_data.clk_names[i];
    ret = devm_clk_bulk_get(phy_drd.dev, phy_drd.drv_data.n_clks,
    phy_drd.clks);
    if (ret)
    return dev_err_probe(phy_drd.dev, ret,
    "failed to get phy clock(s)\n");
    phy_drd.core_clks = devm_kcalloc(phy_drd.dev,
    phy_drd.drv_data.n_core_clks,
    sizeof(*phy_drd.core_clks),
    GFP_KERNEL);
    if (!phy_drd.core_clks)
    return -ENOMEM;
    for (int i = 0; i < phy_drd.drv_data.n_core_clks; ++i)
    phy_drd.core_clks[i].id = phy_drd.drv_data.core_clk_names[i];
    ret = devm_clk_bulk_get(phy_drd.dev, phy_drd.drv_data.n_core_clks,
    phy_drd.core_clks);
    if (ret)
    return dev_err_probe(phy_drd.dev, ret,
    "failed to get phy core clock(s)\n");
    if (phy_drd.drv_data.n_core_clks) {
    ref_clk = core::ptr::null_mut();
    for (int i = 0; i < phy_drd.drv_data.n_core_clks; ++i) {
    if (!strcmp(phy_drd.core_clks[i].id, "ref")) {
    ref_clk = phy_drd.core_clks[i].clk;
    break;
    }
    }
    if (!ref_clk)
    return dev_err_probe(phy_drd.dev, -ENODEV,
    "failed to find phy reference clock\n");
    ref_rate = clk_get_rate(ref_clk);
    ret = exynos5_rate_to_clk(ref_rate, &phy_drd.extrefclk);
    if (ret)
    return dev_err_probe(phy_drd.dev, ret,
    "clock rate (%ld) not supported\n",
    ref_rate);
    }
    return 0;
    }
    static const struct exynos5_usbdrd_phy_config phy_cfg_exynos2200[] = {
    {
    .id		= EXYNOS5_DRDPHY_UTMI,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynos2200_usbdrd_utmi_init,
    },
    };
    static int exynos5_usbdrd_orien_sw_set(struct typec_switch_dev *sw,
    enum typec_orientation orientation)
    {
    struct exynos5_usbdrd_phy *phy_drd = typec_switch_get_drvdata(sw);
    int ret;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret) {
    dev_err(phy_drd.dev, "Failed to enable PHY clocks(s)\n");
    return ret;
    }
    scoped_guard(mutex, &phy_drd.phy_mutex) {
    let mut regs_base: *mut void __iomem  const = phy_drd.reg_phy;
    unsigned int reg;
    if (orientation == TYPEC_ORIENTATION_NONE) {
    reg = readl(regs_base + EXYNOS850_DRD_UTMI);
    reg &= ~(UTMI_FORCE_VBUSVALID | UTMI_FORCE_BVALID);
    writel(reg, regs_base +  EXYNOS850_DRD_UTMI);
    reg = readl(regs_base + EXYNOS850_DRD_HSP);
    reg |= HSP_VBUSVLDEXTSEL;
    reg &= ~HSP_VBUSVLDEXT;
    writel(reg, regs_base + EXYNOS850_DRD_HSP);
    } else {
    reg = readl(regs_base + EXYNOS850_DRD_UTMI);
    reg |= UTMI_FORCE_VBUSVALID | UTMI_FORCE_BVALID;
    writel(reg, regs_base +  EXYNOS850_DRD_UTMI);
    reg = readl(regs_base + EXYNOS850_DRD_HSP);
    reg |= HSP_VBUSVLDEXTSEL | HSP_VBUSVLDEXT;
    writel(reg, regs_base + EXYNOS850_DRD_HSP);
    }
    phy_drd.orientation = orientation;
    }
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_orien_switch_unregister(data: *mut c_void) {
    static void exynos5_usbdrd_orien_switch_unregister(void *data)
    {
    struct exynos5_usbdrd_phy *phy_drd = data;
    typec_switch_unregister(phy_drd.sw);
    }
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_setup_notifiers(phy_drd: *mut exynos5_usbdrd_phy) -> c_int {
    static int exynos5_usbdrd_setup_notifiers(struct exynos5_usbdrd_phy *phy_drd)
    {
    int ret;
    if (!IS_ENABLED(CONFIG_TYPEC))
    return 0;
    if (device_property_present(phy_drd.dev, "orientation-switch")) {
    let mut sw_desc: typec_switch_desc = { };
    sw_desc.drvdata = phy_drd;
    sw_desc.fwnode = dev_fwnode(phy_drd.dev);
    sw_desc.set = exynos5_usbdrd_orien_sw_set;
    phy_drd.sw = typec_switch_register(phy_drd.dev, &sw_desc);
    if (IS_ERR(phy_drd.sw))
    return dev_err_probe(phy_drd.dev,
    PTR_ERR(phy_drd.sw),
    "Failed to register TypeC orientation switch\n");
    ret = devm_add_action_or_reset(phy_drd.dev,
    exynos5_usbdrd_orien_switch_unregister,
    phy_drd);
    if (ret)
    return dev_err_probe(phy_drd.dev, ret,
    "Failed to register TypeC orientation devm action\n");
    }
    return 0;
    }
    static const struct exynos5_usbdrd_phy_config phy_cfg_exynos5[] = {
    {
    .id		= EXYNOS5_DRDPHY_UTMI,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynos5_usbdrd_utmi_init,
    .set_refclk	= exynos5_usbdrd_utmi_set_refclk,
    },
    {
    .id		= EXYNOS5_DRDPHY_PIPE3,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynos5_usbdrd_pipe3_init,
    .set_refclk	= exynos5_usbdrd_pipe3_set_refclk,
    },
    };
    static const struct exynos5_usbdrd_phy_config phy_cfg_exynos7870[] = {
    {
    .id		= EXYNOS5_DRDPHY_UTMI,
    .phy_isol	= exynos7870_usbdrd_phy_isol,
    .phy_init	= exynos7870_usbdrd_utmi_init,
    },
    };
    static const struct exynos5_usbdrd_phy_config phy_cfg_exynos850[] = {
    {
    .id		= EXYNOS5_DRDPHY_UTMI,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynos850_usbdrd_utmi_init,
    },
    };
    static
    const struct exynos5_usbdrd_phy_tuning exynos7870_tunes_utmi_postinit[] = {
    PHY_TUNING_ENTRY_PHY(EXYNOS5_DRD_PHYPARAM0,
    (PHYPARAM0_TXVREFTUNE | PHYPARAM0_TXRISETUNE |
    PHYPARAM0_TXRESTUNE | PHYPARAM0_TXPREEMPPULSETUNE |
    PHYPARAM0_TXPREEMPAMPTUNE | PHYPARAM0_TXHSXVTUNE |
    PHYPARAM0_TXFSLSTUNE | PHYPARAM0_SQRXTUNE |
    PHYPARAM0_OTGTUNE | PHYPARAM0_COMPDISTUNE),
    (FIELD_PREP_CONST(PHYPARAM0_TXVREFTUNE, 3) |
    FIELD_PREP_CONST(PHYPARAM0_TXRISETUNE, 1) |
    FIELD_PREP_CONST(PHYPARAM0_TXRESTUNE, 2) |
    FIELD_PREP_CONST(PHYPARAM0_TXPREEMPPULSETUNE, 0) |
    FIELD_PREP_CONST(PHYPARAM0_TXPREEMPAMPTUNE, 0) |
    FIELD_PREP_CONST(PHYPARAM0_TXHSXVTUNE, 0) |
    FIELD_PREP_CONST(PHYPARAM0_TXFSLSTUNE, 3) |
    FIELD_PREP_CONST(PHYPARAM0_SQRXTUNE, 5) |
    FIELD_PREP_CONST(PHYPARAM0_OTGTUNE, 2) |
    FIELD_PREP_CONST(PHYPARAM0_COMPDISTUNE, 3))),
    PHY_TUNING_ENTRY_LAST
    };
    static const struct exynos5_usbdrd_phy_tuning *exynos7870_tunes[PTS_MAX] = {
    [PTS_UTMI_POSTINIT] = exynos7870_tunes_utmi_postinit,
    };
    static const char * const exynos5_clk_names[] = {
    "phy",
    };
    static const char * const exynos5_core_clk_names[] = {
    "ref",
    };
    static const char * const exynos5433_core_clk_names[] = {
    "ref", "phy_pipe", "phy_utmi", "itp",
    };
    static const char * const exynos5_regulator_names[] = {
    "vbus", "vbus-boost",
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos2200_usb32drd_phy = {
    .phy_cfg		= phy_cfg_exynos2200,
    .phy_ops		= &exynos2200_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOS2200_PHY_CTRL_USB20,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
// clocks and regulators are specific to the underlying PHY blocks
    .core_clk_names		= core::ptr::null_mut(),
    .n_core_clks		= 0,
    .regulator_names	= core::ptr::null_mut(),
    .n_regulators		= 0,
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos5420_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynos5,
    .phy_ops		= &exynos5_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOS5_USBDRD_PHY_CONTROL,
    .pmu_offset_usbdrd1_phy	= EXYNOS5420_USBDRD1_PHY_CONTROL,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynos5_regulator_names,
    .n_regulators		= ARRAY_SIZE(exynos5_regulator_names),
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos5250_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynos5,
    .phy_ops		= &exynos5_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOS5_USBDRD_PHY_CONTROL,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynos5_regulator_names,
    .n_regulators		= ARRAY_SIZE(exynos5_regulator_names),
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos5433_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynos5,
    .phy_ops		= &exynos5_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOS5_USBDRD_PHY_CONTROL,
    .pmu_offset_usbdrd1_phy	= EXYNOS5433_USBHOST30_PHY_CONTROL,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5433_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5433_core_clk_names),
    .regulator_names	= exynos5_regulator_names,
    .n_regulators		= ARRAY_SIZE(exynos5_regulator_names),
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos7_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynos5,
    .phy_ops		= &exynos5_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOS5_USBDRD_PHY_CONTROL,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5433_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5433_core_clk_names),
    .regulator_names	= exynos5_regulator_names,
    .n_regulators		= ARRAY_SIZE(exynos5_regulator_names),
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos7870_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynos7870,
    .phy_tunes		= exynos7870_tunes,
    .phy_ops		= &exynos7870_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOS5_USBDRD_PHY_CONTROL,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynos5_regulator_names,
    .n_regulators		= ARRAY_SIZE(exynos5_regulator_names),
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos850_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynos850,
    .phy_ops		= &exynos850_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOS5_USBDRD_PHY_CONTROL,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynos5_regulator_names,
    .n_regulators		= ARRAY_SIZE(exynos5_regulator_names),
    };
    static const struct exynos5_usbdrd_phy_tuning exynos990_tunes_utmi_postinit[] = {
    PHY_TUNING_ENTRY_PHY(EXYNOS850_DRD_HSPPARACON,
    (HSPPARACON_TXVREF |
    HSPPARACON_TXPREEMPAMP | HSPPARACON_SQRX |
    HSPPARACON_COMPDIS),
    (FIELD_PREP_CONST(HSPPARACON_TXVREF, 7) |
    FIELD_PREP_CONST(HSPPARACON_TXPREEMPAMP, 3) |
    FIELD_PREP_CONST(HSPPARACON_SQRX, 5) |
    FIELD_PREP_CONST(HSPPARACON_COMPDIS, 7))),
    PHY_TUNING_ENTRY_LAST
    };
    static const struct exynos5_usbdrd_phy_tuning *exynos990_tunes[PTS_MAX] = {
    [PTS_UTMI_POSTINIT] = exynos990_tunes_utmi_postinit,
    };
    static const struct exynos5_usbdrd_phy_drvdata exynos990_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynos850,
    .phy_ops		= &exynos850_usbdrd_phy_ops,
    .phy_tunes		= exynos990_tunes,
    .pmu_offset_usbdrd0_phy	= EXYNOS990_PHY_CTRL_USB20,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynos5_regulator_names,
    .n_regulators		= ARRAY_SIZE(exynos5_regulator_names),
    };
    static void
    exynosautov920_usb31drd_cr_clk(struct exynos5_usbdrd_phy *phy_drd, bool high)
    {
    void __iomem *reg_phy = phy_drd.reg_phy;
    u32 reg;
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    if (high)
    reg |= PHY_CR_PARA_CON0_PHY0_CR_PARA_CLK;
    else
    reg &= ~PHY_CR_PARA_CON0_PHY0_CR_PARA_CLK;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    fsleep(1);
    }
    static void
    exynosautov920_usb31drd_port_phy_ready(struct exynos5_usbdrd_phy *phy_drd)
    {
    struct device *dev = phy_drd.dev;
    void __iomem *reg_phy = phy_drd.reg_phy;
    let mut timeout_us: static unsigned int = 20000;
    let mut sleep_us: static unsigned int = 40;
    u32 reg;
    int err;
// Clear cr_para_con
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    reg &= ~(PHY_CR_PARA_CON0_PHY0_CR_PARA_CLK |
    PHY_CR_PARA_CON0_PHY0_CR_PARA_ADDR);
    reg |= PHY_CR_PARA_CON0_PHY0_CR_PARA_SEL;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    writel(0x0, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON1);
    writel(0x0, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON2);
    exynosautov920_usb31drd_cr_clk(phy_drd, true);
    exynosautov920_usb31drd_cr_clk(phy_drd, false);
//
// The maximum time from phy reset de-assertion to de-assertion of
// tx/rx_ack can be as high as 5ms in fast simulation mode.
// Time to phy ready is < 20ms
//
    err = readl_poll_timeout(reg_phy +
    EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0,
    reg, !(reg & PHY_CR_PARA_CON0_PHY0_CR_PARA_ACK),
    sleep_us, timeout_us);
    if (err)
    dev_err(dev, "timed out waiting for rx/tx_ack: %#.8x\n", reg);
    reg &= ~PHY_CR_PARA_CON0_PHY0_CR_PARA_CLK;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    }
    static void
    exynosautov920_usb31drd_cr_write(struct exynos5_usbdrd_phy *phy_drd,
    u16 addr, u16 data)
    {
    void __iomem *reg_phy = phy_drd.reg_phy;
    let mut cnt: u32 = 0;
    u32 reg;
// Pre Clocking
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    reg |= PHY_CR_PARA_CON0_PHY0_CR_PARA_SEL;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
//
// tx clks must be available prior to assertion of tx req.
// tx pstate p2 to p0 transition directly is not permitted.
// tx clk ready must be asserted synchronously on tx clk prior
// to internal transmit clk alignment sequence in the phy
// when entering from p2 to p1 to p0.
//
    do {
    exynosautov920_usb31drd_cr_clk(phy_drd, true);
    exynosautov920_usb31drd_cr_clk(phy_drd, false);
    cnt++;
    } while (cnt < 15);
    reg &= ~PHY_CR_PARA_CON0_PHY0_CR_PARA_SEL;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
//
// tx data path is active when tx lane is in p0 state
// and tx data en asserted. enable cr_para_wr_en.
//
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON2);
    reg &= ~PHY_CR_PARA_CON2_PHY0_CR_PARA_WR_DATA;
    reg |= FIELD_PREP(PHY_CR_PARA_CON2_PHY0_CR_PARA_WR_DATA, data) |
    PHY_CR_PARA_CON2_PHY0_CR_PARA_WR_EN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON2);
// write addr
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    reg &= ~PHY_CR_PARA_CON0_PHY0_CR_PARA_ADDR;
    reg |= FIELD_PREP(PHY_CR_PARA_CON0_PHY0_CR_PARA_ADDR, addr) |
    PHY_CR_PARA_CON0_PHY0_CR_PARA_CLK |
    PHY_CR_PARA_CON0_PHY0_CR_PARA_SEL;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
// check cr_para_ack
    cnt = 0;
    do {
//
// data symbols are captured by phy on rising edge of the
// tx_clk when tx data enabled.
// completion of the write cycle is acknowledged by assertion
// of the cr_para_ack.
//
    exynosautov920_usb31drd_cr_clk(phy_drd, true);
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CR_PARA_CON0);
    if ((reg & PHY_CR_PARA_CON0_PHY0_CR_PARA_ACK))
    break;
    exynosautov920_usb31drd_cr_clk(phy_drd, false);
//
// wait for minimum of 10 cr_para_clk cycles after phy reset
// is negated, before accessing control regs to allow for
// internal resets.
//
    cnt++;
    } while (cnt < 10);
    if (cnt < 10)
    exynosautov920_usb31drd_cr_clk(phy_drd, false);
    }
    static void
    exynosautov920_usb31drd_phy_reset(struct exynos5_usbdrd_phy *phy_drd, int val)
    {
    void __iomem *reg_phy = phy_drd.reg_phy;
    u32 reg;
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    reg &= ~PHY_RST_CTRL_PHY_RESET_OVRD_EN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    if (val)
    reg |= PHY_RST_CTRL_PHY_RESET;
    else
    reg &= ~PHY_RST_CTRL_PHY_RESET;
    reg |= PHY_RST_CTRL_PHY_RESET_OVRD_EN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    }
    static void
    exynosautov920_usb31drd_lane0_reset(struct exynos5_usbdrd_phy *phy_drd, int val)
    {
    void __iomem *reg_phy = phy_drd.reg_phy;
    u32 reg;
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    reg |= PHY_RST_CTRL_PIPE_LANE0_RESET_N_OVRD_EN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    if (val)
    reg &= ~PHY_RST_CTRL_PIPE_LANE0_RESET_N;
    else
    reg |= PHY_RST_CTRL_PIPE_LANE0_RESET_N;
    reg &= ~PHY_RST_CTRL_PIPE_LANE0_RESET_N_OVRD_EN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_RST_CTRL);
    }
    static void
    exynosautov920_usb31drd_pipe3_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *reg_phy = phy_drd.reg_phy;
    u32 reg;
//
// Phy and Pipe Lane reset assert.
// assert reset (phy_reset = 1).
// The lane-ack outputs are asserted during reset (tx_ack = rx_ack = 1)
//
    exynosautov920_usb31drd_phy_reset(phy_drd, 1);
    exynosautov920_usb31drd_lane0_reset(phy_drd, 1);
//
// ANA Power En, PCS & PMA PWR Stable Set
// ramp-up power suppiles
//
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG0);
    reg |= PHY_CONFIG0_PHY0_ANA_PWR_EN | PHY_CONFIG0_PHY0_PCS_PWR_STABLE |
    PHY_CONFIG0_PHY0_PMA_PWR_STABLE;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG0);
    fsleep(10);
//
// phy is not functional in test_powerdown mode, test_powerdown to be
// de-asserted for normal operation
//
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG7);
    reg &= ~PHY_CONFIG7_PHY_TEST_POWERDOWN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG7);
//
// phy reset signal be asserted for minimum 10us after power
// supplies are ramped-up
//
    fsleep(10);
//
// Phy and Pipe Lane reset assert de-assert
//
    exynosautov920_usb31drd_phy_reset(phy_drd, 0);
    exynosautov920_usb31drd_lane0_reset(phy_drd, 0);
// Pipe_rx0_sris_mode_en  = 1
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG4);
    reg |= PHY_CONFIG4_PIPE_RX0_SRIS_MODE_EN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG4);
//
// wait for lane ack outputs to de-assert (tx_ack = rx_ack = 0)
// Exit from the reset state is indicated by de-assertion of *_ack
//
    exynosautov920_usb31drd_port_phy_ready(phy_drd);
// override values for level settings
    exynosautov920_usb31drd_cr_write(phy_drd, 0x22, 0x00F5);
    }
    static void
    exynosautov920_usb31drd_ssphy_disable(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *reg_phy = phy_drd.reg_phy;
    u32 reg;
// 1. Assert reset (phy_reset = 1)
    exynosautov920_usb31drd_lane0_reset(phy_drd, 1);
    exynosautov920_usb31drd_phy_reset(phy_drd, 1);
// phy test power down
    reg = readl(reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG7);
    reg |= PHY_CONFIG7_PHY_TEST_POWERDOWN;
    writel(reg, reg_phy + EXYNOSAUTOV920_USB31DRD_PHY_CONFIG7);
    }
    static void
    exynosautov920_usbdrd_utmi_init(struct exynos5_usbdrd_phy *phy_drd)
    {
    void __iomem *reg_phy = phy_drd.reg_phy;
    u32 reg;
//
// Disable HWACG (hardware auto clock gating control). This
// forces QACTIVE signal in Q-Channel interface to HIGH level,
// to make sure the PHY clock is not gated by the hardware.
//
    reg = readl(reg_phy + EXYNOS850_DRD_LINKCTRL);
    reg |= LINKCTRL_FORCE_QACT;
    writel(reg, reg_phy + EXYNOS850_DRD_LINKCTRL);
// De-assert link reset
    reg = readl(reg_phy + EXYNOS2200_DRD_CLKRST);
    reg &= ~CLKRST_LINK_SW_RST;
    writel(reg, reg_phy + EXYNOS2200_DRD_CLKRST);
// Set PHY POR High
    reg = readl(reg_phy + EXYNOSAUTOV920_DRD_HSP_CLKRST);
    reg |= HSPCLKRST_PHY20_SW_POR | HSPCLKRST_PHY20_SW_POR_SEL;
    writel(reg, reg_phy + EXYNOSAUTOV920_DRD_HSP_CLKRST);
// Enable UTMI+
    reg = readl(reg_phy + EXYNOS2200_DRD_UTMI);
    reg &= ~(UTMICTL_FORCE_UTMI_SUSPEND | UTMICTL_FORCE_UTMI_SLEEP |
    UTMICTL_FORCE_DPPULLDOWN | UTMICTL_FORCE_DMPULLDOWN);
    writel(reg, reg_phy + EXYNOS2200_DRD_UTMI);
// set phy clock & control HS phy
    reg = readl(reg_phy + EXYNOSAUTOV920_DRD_HSPCTL);
    reg |= HSPCTRL_EN_UTMISUSPEND | HSPCTRL_COMMONONN;
    writel(reg, reg_phy + EXYNOSAUTOV920_DRD_HSPCTL);
    fsleep(100);
// Set VBUS Valid and DP-Pull up control by VBUS pad usage
    reg = readl(reg_phy + EXYNOS850_DRD_LINKCTRL);
    reg |= FIELD_PREP_CONST(LINKCTRL_BUS_FILTER_BYPASS, 0xf);
    writel(reg, reg_phy + EXYNOS850_DRD_LINKCTRL);
    reg = readl(reg_phy + EXYNOS2200_DRD_UTMI);
    reg |= EXYNOS2200_UTMI_FORCE_VBUSVALID | EXYNOS2200_UTMI_FORCE_BVALID;
    writel(reg, reg_phy + EXYNOS2200_DRD_UTMI);
    reg = readl(reg_phy + EXYNOSAUTOV920_DRD_HSPCTL);
    reg |= HSPCTRL_VBUSVLDEXTSEL | HSPCTRL_VBUSVLDEXT;
    writel(reg, reg_phy + EXYNOSAUTOV920_DRD_HSPCTL);
// Setting FSEL for refference clock
    reg = readl(reg_phy + EXYNOSAUTOV920_DRD_HSPPLLTUNE);
    reg &= ~HSPPLLTUNE_FSEL;
    switch (phy_drd.extrefclk) {
    case EXYNOS5_FSEL_50MHZ:
    reg |= FIELD_PREP(HSPPLLTUNE_FSEL, 7);
    break;
    case EXYNOS5_FSEL_26MHZ:
    reg |= FIELD_PREP(HSPPLLTUNE_FSEL, 6);
    break;
    case EXYNOS5_FSEL_24MHZ:
    reg |= FIELD_PREP(HSPPLLTUNE_FSEL, 2);
    break;
    case EXYNOS5_FSEL_20MHZ:
    reg |= FIELD_PREP(HSPPLLTUNE_FSEL, 1);
    break;
    case EXYNOS5_FSEL_19MHZ2:
    reg |= FIELD_PREP(HSPPLLTUNE_FSEL, 0);
    break;
    default:
    dev_warn(phy_drd.dev, "unsupported ref clk: %#.2x\n",
    phy_drd.extrefclk);
    break;
    }
    writel(reg, reg_phy + EXYNOSAUTOV920_DRD_HSPPLLTUNE);
// Enable PHY Power Mode
    reg = readl(reg_phy + EXYNOSAUTOV920_DRD_HSP_TEST);
    reg &= ~HSP_TEST_SIDDQ;
    writel(reg, reg_phy + EXYNOSAUTOV920_DRD_HSP_TEST);
// before POR low, 10us delay is needed to Finish PHY reset
    fsleep(10);
// Set PHY POR Low
    reg = readl(reg_phy + EXYNOSAUTOV920_DRD_HSP_CLKRST);
    reg |= HSPCLKRST_PHY20_SW_POR_SEL;
    reg &= ~(HSPCLKRST_PHY20_SW_POR | HSPCLKRST_PHY20_SW_PORTRESET);
    writel(reg, reg_phy + EXYNOSAUTOV920_DRD_HSP_CLKRST);
// after POR low and delay 75us, PHYCLOCK is guaranteed.
    fsleep(75);
// Disable forcing pipe interface
    reg = readl(reg_phy + EXYNOS850_DRD_LINKCTRL);
    reg &= ~LINKCTRL_FORCE_PIPE_EN;
    writel(reg, reg_phy + EXYNOS850_DRD_LINKCTRL);
// Pclk to pipe_clk
    reg = readl(reg_phy + EXYNOS2200_DRD_CLKRST);
    reg |= EXYNOS2200_CLKRST_LINK_PCLK_SEL;
    writel(reg, reg_phy + EXYNOS2200_DRD_CLKRST);
    }
    static void
    exynosautov920_usbdrd_hsphy_disable(struct exynos5_usbdrd_phy *phy_drd)
    {
    u32 reg;
    void __iomem *reg_phy = phy_drd.reg_phy;
// set phy clock & control HS phy
    reg = readl(reg_phy + EXYNOS2200_DRD_UTMI);
    reg |= UTMICTL_FORCE_UTMI_SUSPEND | UTMICTL_FORCE_UTMI_SLEEP;
    reg &= ~(UTMICTL_FORCE_DPPULLDOWN | UTMICTL_FORCE_DMPULLDOWN);
    writel(reg, reg_phy + EXYNOS2200_DRD_UTMI);
// Disable PHY Power Mode
    reg = readl(reg_phy + EXYNOSAUTOV920_DRD_HSP_TEST);
    reg |= HSP_TEST_SIDDQ;
    writel(reg, reg_phy + EXYNOSAUTOV920_DRD_HSP_TEST);
// clear force q-channel
    reg = readl(reg_phy + EXYNOS850_DRD_LINKCTRL);
    reg &= ~LINKCTRL_FORCE_QACT;
    writel(reg, reg_phy + EXYNOS850_DRD_LINKCTRL);
// link sw reset is need for USB_DP/DM high-z in host mode
    reg = readl(reg_phy + EXYNOS2200_DRD_CLKRST);
    reg |= CLKRST_LINK_SW_RST;
    writel(reg, reg_phy + EXYNOS2200_DRD_CLKRST);
    fsleep(10);
    reg &= ~CLKRST_LINK_SW_RST;
    writel(reg, reg_phy + EXYNOS2200_DRD_CLKRST);
    }
#[no_mangle]
unsafe extern "C" fn exynosautov920_usbdrd_phy_init(phy: *mut phy) -> c_int {
    static int exynosautov920_usbdrd_phy_init(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
// Bypass PHY isol
    inst.phy_cfg.phy_isol(inst, false);
// UTMI or PIPE3 specific init
    inst.phy_cfg.phy_init(phy_drd);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynosautov920_usbdrd_phy_exit(phy: *mut phy) -> c_int {
    static int exynosautov920_usbdrd_phy_exit(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
    exynos850_usbdrd_phy_exit(phy);
// enable PHY isol
    inst.phy_cfg.phy_isol(inst, true);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynosautov920_usbdrd_combo_phy_exit(phy: *mut phy) -> c_int {
    static int exynosautov920_usbdrd_combo_phy_exit(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    let mut ret: c_int = 0;
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_clks, phy_drd.clks);
    if (ret)
    return ret;
    if (inst.phy_cfg.id == EXYNOS5_DRDPHY_UTMI)
    exynosautov920_usbdrd_hsphy_disable(phy_drd);
#[no_mangle]
pub unsafe extern "C" fn if(EXYNOS5_DRDPHY_PIPE3: inst->phy_cfg->id ==) -> else {
    else if (inst.phy_cfg.id == EXYNOS5_DRDPHY_PIPE3)
    exynosautov920_usb31drd_ssphy_disable(phy_drd);
// enable PHY isol
    inst.phy_cfg.phy_isol(inst, true);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_clks, phy_drd.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynosautov920_usbdrd_phy_power_on(phy: *mut phy) -> c_int {
    static int exynosautov920_usbdrd_phy_power_on(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    int ret;
    dev_dbg(phy_drd.dev, "Request to power_on usbdrd_phy phy\n");
    ret = clk_bulk_prepare_enable(phy_drd.drv_data.n_core_clks,
    phy_drd.core_clks);
    if (ret)
    return ret;
// Enable supply
    ret = regulator_bulk_enable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    if (ret) {
    dev_err(phy_drd.dev, "Failed to enable PHY regulator(s)\n");
    goto fail_supply;
    }
    return 0;
    fail_supply:
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_core_clks,
    phy_drd.core_clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynosautov920_usbdrd_phy_power_off(phy: *mut phy) -> c_int {
    static int exynosautov920_usbdrd_phy_power_off(struct phy *phy)
    {
    struct phy_usb_instance *inst = phy_get_drvdata(phy);
    struct exynos5_usbdrd_phy *phy_drd = to_usbdrd_phy(inst);
    dev_dbg(phy_drd.dev, "Request to power_off usbdrd_phy phy\n");
// Disable supply
    regulator_bulk_disable(phy_drd.drv_data.n_regulators,
    phy_drd.regulators);
    clk_bulk_disable_unprepare(phy_drd.drv_data.n_core_clks,
    phy_drd.core_clks);
    return 0;
    }
    static const char * const exynosautov920_usb30_regulators[] = {
    "dvdd", "vdd18",
    };
    static const char * const exynosautov920_usb20_regulators[] = {
    "dvdd", "vdd18", "vdd33",
    };
    static const struct
    exynos5_usbdrd_phy_config usb31drd_phy_cfg_exynosautov920[] = {
    {
    .id		= EXYNOS5_DRDPHY_PIPE3,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynosautov920_usb31drd_pipe3_init,
    },
    };
    static const struct phy_ops exynosautov920_usb31drd_combo_ssphy_ops = {
    .init		= exynosautov920_usbdrd_phy_init,
    .exit		= exynosautov920_usbdrd_combo_phy_exit,
    .power_on	= exynosautov920_usbdrd_phy_power_on,
    .power_off	= exynosautov920_usbdrd_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const
    struct exynos5_usbdrd_phy_drvdata exynosautov920_usb31drd_combo_ssphy = {
    .phy_cfg		= usb31drd_phy_cfg_exynosautov920,
    .phy_ops		= &exynosautov920_usb31drd_combo_ssphy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOSAUTOV920_PHY_CTRL_USB31,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynosautov920_usb30_regulators,
    .n_regulators		= ARRAY_SIZE(exynosautov920_usb30_regulators),
    };
    static const struct phy_ops exynosautov920_usbdrd_combo_hsphy_ops = {
    .init		= exynosautov920_usbdrd_phy_init,
    .exit		= exynosautov920_usbdrd_combo_phy_exit,
    .power_on	= exynosautov920_usbdrd_phy_power_on,
    .power_off	= exynosautov920_usbdrd_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct
    exynos5_usbdrd_phy_config usbdrd_hsphy_cfg_exynosautov920[] = {
    {
    .id		= EXYNOS5_DRDPHY_UTMI,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynosautov920_usbdrd_utmi_init,
    },
    };
    static const
    struct exynos5_usbdrd_phy_drvdata exynosautov920_usbdrd_combo_hsphy = {
    .phy_cfg		= usbdrd_hsphy_cfg_exynosautov920,
    .phy_ops		= &exynosautov920_usbdrd_combo_hsphy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOSAUTOV920_PHY_CTRL_USB20,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynosautov920_usb20_regulators,
    .n_regulators		= ARRAY_SIZE(exynosautov920_usb20_regulators),
    };
    static const struct phy_ops exynosautov920_usbdrd_phy_ops = {
    .init		= exynosautov920_usbdrd_phy_init,
    .exit		= exynosautov920_usbdrd_phy_exit,
    .power_on       = exynosautov920_usbdrd_phy_power_on,
    .power_off      = exynosautov920_usbdrd_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct exynos5_usbdrd_phy_config phy_cfg_exynosautov920[] = {
    {
    .id		= EXYNOS5_DRDPHY_UTMI,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynos850_usbdrd_utmi_init,
    },
    };
    static const struct exynos5_usbdrd_phy_drvdata exynosautov920_usbdrd_phy = {
    .phy_cfg		= phy_cfg_exynosautov920,
    .phy_ops		= &exynosautov920_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy	= EXYNOSAUTOV920_PHY_CTRL_USB20,
    .clk_names		= exynos5_clk_names,
    .n_clks			= ARRAY_SIZE(exynos5_clk_names),
    .core_clk_names		= exynos5_core_clk_names,
    .n_core_clks		= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names	= exynosautov920_usb20_regulators,
    .n_regulators		= ARRAY_SIZE(exynosautov920_usb20_regulators),
    };
    static const struct exynos5_usbdrd_phy_config phy_cfg_gs101[] = {
    {
    .id		= EXYNOS5_DRDPHY_UTMI,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynos850_usbdrd_utmi_init,
    },
    {
    .id		= EXYNOS5_DRDPHY_PIPE3,
    .phy_isol	= exynos5_usbdrd_phy_isol,
    .phy_init	= exynos5_usbdrd_gs101_pipe3_init,
    },
    };
    static const struct exynos5_usbdrd_phy_tuning gs101_tunes_utmi_postinit[] = {
    PHY_TUNING_ENTRY_PHY(EXYNOS850_DRD_HSPPARACON,
    (HSPPARACON_TXVREF | HSPPARACON_TXRES |
    HSPPARACON_TXPREEMPAMP | HSPPARACON_SQRX |
    HSPPARACON_COMPDIS),
    (FIELD_PREP_CONST(HSPPARACON_TXVREF, 6) |
    FIELD_PREP_CONST(HSPPARACON_TXRES, 1) |
    FIELD_PREP_CONST(HSPPARACON_TXPREEMPAMP, 3) |
    FIELD_PREP_CONST(HSPPARACON_SQRX, 5) |
    FIELD_PREP_CONST(HSPPARACON_COMPDIS, 7))),
    PHY_TUNING_ENTRY_LAST
    };
    static const struct exynos5_usbdrd_phy_tuning gs101_tunes_pipe3_preinit[] = {
// preinit
// CDR data mode exit GEN1 ON / GEN2 OFF
    PHY_TUNING_ENTRY_PMA(0x0c8c, -1, 0xff),
    PHY_TUNING_ENTRY_PMA(0x1c8c, -1, 0xff),
    PHY_TUNING_ENTRY_PMA(0x0c9c, -1, 0x7d),
    PHY_TUNING_ENTRY_PMA(0x1c9c, -1, 0x7d),
// improve EDS distribution
    PHY_TUNING_ENTRY_PMA(0x0e7c, -1, 0x06),
    PHY_TUNING_ENTRY_PMA(0x09e0, -1, 0x00),
    PHY_TUNING_ENTRY_PMA(0x09e4, -1, 0x36),
    PHY_TUNING_ENTRY_PMA(0x1e7c, -1, 0x06),
    PHY_TUNING_ENTRY_PMA(0x19e0, -1, 0x00),
    PHY_TUNING_ENTRY_PMA(0x19e4, -1, 0x36),
// fix bootloader bug
    PHY_TUNING_ENTRY_PMA(0x1e90, -1, 0x02),
    PHY_TUNING_ENTRY_PMA(0x1e94, -1, 0x0b),
// improve LVCC
    PHY_TUNING_ENTRY_PMA(0x08f0, -1, 0x30),
    PHY_TUNING_ENTRY_PMA(0x18f0, -1, 0x30),
// LFPS RX VIH shmoo hole
    PHY_TUNING_ENTRY_PMA(0x0a08, -1, 0x0c),
    PHY_TUNING_ENTRY_PMA(0x1a08, -1, 0x0c),
// remove unrelated option for v4 phy
    PHY_TUNING_ENTRY_PMA(0x0a0c, -1, 0x05),
    PHY_TUNING_ENTRY_PMA(0x1a0c, -1, 0x05),
// improve Gen2 LVCC
    PHY_TUNING_ENTRY_PMA(0x00f8, -1, 0x1c),
    PHY_TUNING_ENTRY_PMA(0x00fc, -1, 0x54),
// Change Vth of RCV_DET because of TD 7.40 Polling Retry Test
    PHY_TUNING_ENTRY_PMA(0x104c, -1, 0x07),
    PHY_TUNING_ENTRY_PMA(0x204c, -1, 0x07),
// reduce Ux Exit time, assuming 26MHz clock
// Gen1
    PHY_TUNING_ENTRY_PMA(0x0ca8, -1, 0x00),
    PHY_TUNING_ENTRY_PMA(0x0cac, -1, 0x04),
    PHY_TUNING_ENTRY_PMA(0x1ca8, -1, 0x00),
    PHY_TUNING_ENTRY_PMA(0x1cac, -1, 0x04),
// Gen2
    PHY_TUNING_ENTRY_PMA(0x0cb8, -1, 0x00),
    PHY_TUNING_ENTRY_PMA(0x0cbc, -1, 0x04),
    PHY_TUNING_ENTRY_PMA(0x1cb8, -1, 0x00),
    PHY_TUNING_ENTRY_PMA(0x1cbc, -1, 0x04),
// RX impedance setting
    PHY_TUNING_ENTRY_PMA(0x0bb0, 0x03, 0x01),
    PHY_TUNING_ENTRY_PMA(0x0bb4, 0xf0, 0xa0),
    PHY_TUNING_ENTRY_PMA(0x1bb0, 0x03, 0x01),
    PHY_TUNING_ENTRY_PMA(0x1bb4, 0xf0, 0xa0),
    PHY_TUNING_ENTRY_LAST
    };
    static const struct exynos5_usbdrd_phy_tuning gs101_tunes_pipe3_init[] = {
// init
// abnormal common pattern mask
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_BACK_END_MODE_VEC,
    BACK_END_MODE_VEC_DISABLE_DATA_MASK, 0),
// de-serializer enabled when U2
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_OUT_VEC_2, PCS_OUT_VEC_B4_DYNAMIC,
    PCS_OUT_VEC_B4_SEL_OUT),
// TX Keeper Disable, Squelch on when U3
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_OUT_VEC_3, PCS_OUT_VEC_B7_DYNAMIC,
    PCS_OUT_VEC_B7_SEL_OUT | PCS_OUT_VEC_B2_SEL_OUT),
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_NS_VEC_PS1_N1, -1,
    (FIELD_PREP_CONST(NS_VEC_NS_REQ, 5) |
    NS_VEC_ENABLE_TIMER |
    FIELD_PREP_CONST(NS_VEC_SEL_TIMEOUT, 3))),
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_NS_VEC_PS2_N0, -1,
    (FIELD_PREP_CONST(NS_VEC_NS_REQ, 1) |
    NS_VEC_ENABLE_TIMER |
    FIELD_PREP_CONST(NS_VEC_SEL_TIMEOUT, 3) |
    FIELD_PREP_CONST(NS_VEC_COND_MASK, 2) |
    FIELD_PREP_CONST(NS_VEC_EXP_COND, 2))),
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_NS_VEC_PS3_N0, -1,
    (FIELD_PREP_CONST(NS_VEC_NS_REQ, 1) |
    NS_VEC_ENABLE_TIMER |
    FIELD_PREP_CONST(NS_VEC_SEL_TIMEOUT, 3) |
    FIELD_PREP_CONST(NS_VEC_COND_MASK, 7) |
    FIELD_PREP_CONST(NS_VEC_EXP_COND, 7))),
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_TIMEOUT_0, -1, 112),
// Block Aligner Type B
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_RX_CONTROL, 0,
    RX_CONTROL_EN_BLOCK_ALIGNER_TYPE_B),
// Block align at TS1/TS2 for Gen2 stability (Gen2 only)
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_RX_CONTROL_DEBUG,
    RX_CONTROL_DEBUG_NUM_COM_FOUND,
    (RX_CONTROL_DEBUG_EN_TS_CHECK |
//
// increase pcs ts1 adding packet-cnt 1 --> 4
// lnx_rx_valid_rstn_delay_rise_sp/ssp :
// 19.6us(0x200) -> 15.3us(0x4)
//
    FIELD_PREP_CONST(RX_CONTROL_DEBUG_NUM_COM_FOUND, 4))),
// Gen1 Tx DRIVER pre-shoot, de-emphasis, level ctrl
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_HS_TX_COEF_MAP_0,
    (HS_TX_COEF_MAP_0_SSTX_DEEMP | HS_TX_COEF_MAP_0_SSTX_LEVEL |
    HS_TX_COEF_MAP_0_SSTX_PRE_SHOOT),
    (FIELD_PREP_CONST(HS_TX_COEF_MAP_0_SSTX_DEEMP, 8) |
    FIELD_PREP_CONST(HS_TX_COEF_MAP_0_SSTX_LEVEL, 0xb) |
    FIELD_PREP_CONST(HS_TX_COEF_MAP_0_SSTX_PRE_SHOOT, 0))),
// Gen2 Tx DRIVER level ctrl
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_LOCAL_COEF,
    LOCAL_COEF_PMA_CENTER_COEF,
    FIELD_PREP_CONST(LOCAL_COEF_PMA_CENTER_COEF, 0xb)),
// Gen2 U1 exit LFPS duration : 900ns ~ 1.2us
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_TIMEOUT_3, -1, 4096),
// set skp_remove_th 0x2 -> 0x7 for avoiding retry problem.
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_EBUF_PARAM,
    EBUF_PARAM_SKP_REMOVE_TH_EMPTY_MODE,
    FIELD_PREP_CONST(EBUF_PARAM_SKP_REMOVE_TH_EMPTY_MODE, 0x7)),
    PHY_TUNING_ENTRY_LAST
    };
    static const struct exynos5_usbdrd_phy_tuning gs101_tunes_pipe3_postlock[] = {
// Squelch off when U3
    PHY_TUNING_ENTRY_PCS(EXYNOS9_PCS_OUT_VEC_3, PCS_OUT_VEC_B2_SEL_OUT, 0),
    PHY_TUNING_ENTRY_LAST
    };
    static const struct exynos5_usbdrd_phy_tuning *gs101_tunes[PTS_MAX] = {
    [PTS_UTMI_POSTINIT] = gs101_tunes_utmi_postinit,
    [PTS_PIPE3_PREINIT] = gs101_tunes_pipe3_preinit,
    [PTS_PIPE3_INIT] = gs101_tunes_pipe3_init,
    [PTS_PIPE3_POSTLOCK] = gs101_tunes_pipe3_postlock,
    };
    static const char * const gs101_clk_names[] = {
    "phy", "ctrl_aclk", "ctrl_pclk", "scl_pclk",
    };
    static const char * const gs101_regulator_names[] = {
    "pll",
    "dvdd-usb20", "vddh-usb20", "vdd33-usb20",
    "vdda-usbdp", "vddh-usbdp",
    };
    static const struct exynos5_usbdrd_phy_drvdata gs101_usbd31rd_phy = {
    .phy_cfg			= phy_cfg_gs101,
    .phy_tunes			= gs101_tunes,
    .phy_ops			= &gs101_usbdrd_phy_ops,
    .pmu_offset_usbdrd0_phy		= GS101_PHY_CTRL_USB20,
    .pmu_offset_usbdrd0_phy_ss	= GS101_PHY_CTRL_USBDP,
    .clk_names			= gs101_clk_names,
    .n_clks				= ARRAY_SIZE(gs101_clk_names),
    .core_clk_names			= exynos5_core_clk_names,
    .n_core_clks			= ARRAY_SIZE(exynos5_core_clk_names),
    .regulator_names		= gs101_regulator_names,
    .n_regulators			= ARRAY_SIZE(gs101_regulator_names),
    };
    static const struct of_device_id exynos5_usbdrd_phy_of_match[] = {
    {
    .compatible = "google,gs101-usb31drd-phy",
    .data = &gs101_usbd31rd_phy
    }, {
    .compatible = "samsung,exynos2200-usb32drd-phy",
    .data = &exynos2200_usb32drd_phy,
    }, {
    .compatible = "samsung,exynos5250-usbdrd-phy",
    .data = &exynos5250_usbdrd_phy
    }, {
    .compatible = "samsung,exynos5420-usbdrd-phy",
    .data = &exynos5420_usbdrd_phy
    }, {
    .compatible = "samsung,exynos5433-usbdrd-phy",
    .data = &exynos5433_usbdrd_phy
    }, {
    .compatible = "samsung,exynos7-usbdrd-phy",
    .data = &exynos7_usbdrd_phy
    }, {
    .compatible = "samsung,exynos7870-usbdrd-phy",
    .data = &exynos7870_usbdrd_phy
    }, {
    .compatible = "samsung,exynos850-usbdrd-phy",
    .data = &exynos850_usbdrd_phy
    }, {
    .compatible = "samsung,exynos990-usbdrd-phy",
    .data = &exynos990_usbdrd_phy
    }, {
    .compatible = "samsung,exynosautov920-usb31drd-combo-ssphy",
    .data = &exynosautov920_usb31drd_combo_ssphy
    }, {
    .compatible = "samsung,exynosautov920-usbdrd-combo-hsphy",
    .data = &exynosautov920_usbdrd_combo_hsphy
    }, {
    .compatible = "samsung,exynosautov920-usbdrd-phy",
    .data = &exynosautov920_usbdrd_phy
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, exynos5_usbdrd_phy_of_match);
#[no_mangle]
unsafe extern "C" fn exynos5_usbdrd_phy_probe(pdev: *mut platform_device) -> c_int {
    static int exynos5_usbdrd_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct exynos5_usbdrd_phy *phy_drd;
    struct phy_provider *phy_provider;
    const struct exynos5_usbdrd_phy_drvdata *drv_data;
    struct regmap *reg_pmu;
    u32 pmu_offset;
    int i, ret;
    int channel;
    phy_drd = devm_kzalloc(dev, sizeof(*phy_drd), GFP_KERNEL);
    if (!phy_drd)
    return -ENOMEM;
    dev_set_drvdata(dev, phy_drd);
    phy_drd.dev = dev;
    drv_data = of_device_get_match_data(dev);
    if (!drv_data)
    return -EINVAL;
    phy_drd.drv_data = drv_data;
    ret = devm_mutex_init(dev, &phy_drd.phy_mutex);
    if (ret)
    return ret;
    if (of_property_present(dev.of_node, "reg-names")) {
    void __iomem *reg;
    reg = devm_platform_ioremap_resource_byname(pdev, "phy");
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    phy_drd.reg_phy = reg;
    reg = devm_platform_ioremap_resource_byname(pdev, "pcs");
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    phy_drd.reg_pcs = reg;
    reg = devm_platform_ioremap_resource_byname(pdev, "pma");
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    phy_drd.reg_pma = reg;
    } else {
// DTB with just a single region
    phy_drd.reg_phy = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy_drd.reg_phy))
    return PTR_ERR(phy_drd.reg_phy);
    }
//
// USB32DRD 4nm controller implements Synopsys eUSB2.0 PHY
// and Synopsys SS/USBDP COMBOPHY, managed by external code.
//
    if (of_property_present(dev.of_node, "phy-names")) {
    phy_drd.hs_phy = devm_of_phy_get(dev, dev.of_node, "hs");
    if (IS_ERR(phy_drd.hs_phy))
    return dev_err_probe(dev, PTR_ERR(phy_drd.hs_phy),
    "failed to get hs_phy\n");
    }
    ret = exynos5_usbdrd_phy_clk_handle(phy_drd);
    if (ret)
    return ret;
    reg_pmu = syscon_regmap_lookup_by_phandle(dev.of_node,
    "samsung,pmu-syscon");
    if (IS_ERR(reg_pmu))
    return dev_err_probe(dev, PTR_ERR(reg_pmu),
    "Failed to lookup PMU regmap\n");
//
// Exynos5420 SoC has multiple channels for USB 3.0 PHY, with
// each having separate power control registers.
// 'channel' facilitates to set such registers.
//
    channel = of_alias_get_id(node, "usbdrdphy");
    if (channel < 0)
    dev_dbg(dev, "Not a multi-controller usbdrd phy\n");
// Get regulators
    phy_drd.regulators = devm_kcalloc(dev,
    drv_data.n_regulators,
    sizeof(*phy_drd.regulators),
    GFP_KERNEL);
    if (!phy_drd.regulators)
    return -ENOMEM;
    regulator_bulk_set_supply_names(phy_drd.regulators,
    drv_data.regulator_names,
    drv_data.n_regulators);
    ret = devm_regulator_bulk_get(dev, drv_data.n_regulators,
    phy_drd.regulators);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get regulators\n");
    ret = exynos5_usbdrd_setup_notifiers(phy_drd);
    if (ret)
    return ret;
    dev_vdbg(dev, "Creating usbdrd_phy phy\n");
    for (i = 0; i < EXYNOS5_DRDPHYS_NUM; i++) {
    struct phy *phy = devm_phy_create(dev, core::ptr::null_mut(), drv_data.phy_ops);
    if (IS_ERR(phy))
    return dev_err_probe(dev, PTR_ERR(phy),
    "Failed to create usbdrd_phy phy\n");
    phy_drd.phys[i].phy = phy;
    phy_drd.phys[i].index = i;
    phy_drd.phys[i].reg_pmu = reg_pmu;
    switch (channel) {
    case 1:
    pmu_offset = drv_data.pmu_offset_usbdrd1_phy;
    break;
    case 0:
    default:
    pmu_offset = drv_data.pmu_offset_usbdrd0_phy;
    if (i == EXYNOS5_DRDPHY_PIPE3 && drv_data
    .pmu_offset_usbdrd0_phy_ss)
    pmu_offset = drv_data.pmu_offset_usbdrd0_phy_ss;
    break;
    }
    phy_drd.phys[i].pmu_offset = pmu_offset;
    phy_drd.phys[i].phy_cfg = &drv_data.phy_cfg[i];
    phy_set_drvdata(phy, &phy_drd.phys[i]);
    }
    phy_provider = devm_of_phy_provider_register(dev,
    exynos5_usbdrd_phy_xlate);
    if (IS_ERR(phy_provider))
    return dev_err_probe(phy_drd.dev, PTR_ERR(phy_provider),
    "Failed to register phy provider\n");
    return 0;
    }
    static struct platform_driver exynos5_usb3drd_phy = {
    .probe	= exynos5_usbdrd_phy_probe,
    .driver = {
    .of_match_table	= exynos5_usbdrd_phy_of_match,
    .name		= "exynos5_usb3drd_phy",
    .suppress_bind_attrs = true,
    }
    };
    module_platform_driver(exynos5_usb3drd_phy);
    MODULE_DESCRIPTION("Samsung Exynos5 SoCs USB 3.0 DRD controller PHY driver");
    MODULE_AUTHOR("Vivek Gautam <gautam.vivek@samsung.com>");
    MODULE_LICENSE("GPL v2");
