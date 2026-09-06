//! Automatically rewritten from C to Rust
//! Source: drivers/phy/freescale/phy-fsl-imx8mq-usb.c
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
// Copyright 2017-2026 NXP.

pub const PHY_CTRL0: c_uint = 0x0;

pub const PHY_CTRL0_FSEL_24M: c_uint = 0x2a;
pub const PHY_CTRL0_FSEL_100M: c_uint = 0x27;

pub const PHY_CTRL0_SSC_RANGE_4003PPM: c_uint = 0x2;
pub const PHY_CTRL0_SSC_RANGE_4492PPM: c_uint = 0x1;
pub const PHY_CTRL0_SSC_RANGE_4980PPM: c_uint = 0x0;
pub const PHY_CTRL1: c_uint = 0x4;

pub const PHY_CTRL2: c_uint = 0x8;

pub const PHY_CTRL3: c_uint = 0xc;

pub const PHY_CTRL4: c_uint = 0x10;

pub const PHY_CTRL5: c_uint = 0x14;

pub const PHY_CTRL6: c_uint = 0x18;

pub const PHY_CRCTL: c_uint = 0x30;
pub const PHY_TUNE_DEFAULT: c_uint = 0xffffffff;
pub const TCA_CLK_RST: c_uint = 0x00;

pub const TCA_INTR_EN: c_uint = 0x04;
pub const TCA_INTR_STS: c_uint = 0x08;
pub const TCA_GCFG: c_uint = 0x10;

pub const TCA_GCFG_OP_MODE_SYSMODE: c_int = 0;
pub const TCA_GCFG_OP_MODE_SYNCMODE: c_int = 1;
pub const TCA_TCPC: c_uint = 0x14;

pub const TCA_TCPC_MUX_CONTRL_NO_CONN: c_int = 0;
pub const TCA_TCPC_MUX_CONTRL_USB_CONN: c_int = 1;
pub const TCA_SYSMODE_CFG: c_uint = 0x18;

pub const TCA_CTRLSYNCMODE_CFG0: c_uint = 0x20;
pub const TCA_CTRLSYNCMODE_CFG1: c_uint = 0x20;
pub const TCA_PSTATE: c_uint = 0x30;

pub const TCA_GEN_STATUS: c_uint = 0x34;

pub const TCA_VBUS_CTRL: c_uint = 0x40;
pub const TCA_VBUS_STATUS: c_uint = 0x44;
pub const TCA_INFO: c_uint = 0xfc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tca_blk {
    pub sw: *mut typec_switch_dev,
    pub base: *mut void __iomem,
    pub mutex: mutex,
    pub orientation: enum typec_orientation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mq_usb_phy {
    pub phy: *mut phy,
    pub clk: *mut clk,
    pub alt_clk: *mut clk,
    pub base: *mut void __iomem,
    pub vbus: *mut regulator,
    pub tca: *mut tca_blk,
    pub cr_regmap: *mut regmap,
    pub pcs_tx_swing_full: u32,
    pub pcs_tx_deemph_3p5db: u32,
    pub tx_vref_tune: u32,
    pub tx_rise_tune: u32,
    pub tx_preemp_amp_tune: u32,
    pub tx_vboost_level: u32,
    pub comp_dis_tune: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mq_usb_phy_drvdata {
    pub ops: *const phy_ops,
    pub need_genpd_rpm_on: bool,
}

    static void tca_blk_orientation_set(struct tca_blk *tca,
    enum typec_orientation orientation);
    static int tca_blk_typec_switch_set(struct typec_switch_dev *sw,
    enum typec_orientation orientation)
    {
    struct imx8mq_usb_phy *imx_phy = typec_switch_get_drvdata(sw);
    struct tca_blk *tca = imx_phy.tca;
    if (tca.orientation == orientation)
    return 0;
    PM_RUNTIME_ACQUIRE_IF_ENABLED(&imx_phy.phy.dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    tca_blk_orientation_set(tca, orientation);
    return 0;
    }
    static struct typec_switch_dev *tca_blk_get_typec_switch(struct platform_device *pdev,
    struct imx8mq_usb_phy *imx_phy)
    {
    struct device *dev = &pdev.dev;
    struct typec_switch_dev *sw;
    let mut sw_desc: typec_switch_desc = { };
    sw_desc.drvdata = imx_phy;
    sw_desc.fwnode = dev.fwnode;
    sw_desc.set = tca_blk_typec_switch_set;
    sw_desc.name = core::ptr::null_mut();
    sw = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(sw)) {
    dev_err(dev, "Error register tca orientation switch: %ld",
    PTR_ERR(sw));
    return core::ptr::null_mut();
    }
    return sw;
    }
#[no_mangle]
unsafe extern "C" fn tca_blk_put_typec_switch(data: *mut c_void) {
    static void tca_blk_put_typec_switch(void *data)
    {
    typec_switch_unregister(data);
    }
    static void tca_blk_orientation_set(struct tca_blk *tca,
    enum typec_orientation orientation)
    {
    u32 val;
    mutex_lock(&tca.mutex);
    if (orientation == TYPEC_ORIENTATION_NONE) {
//
// use Controller Synced Mode for TCA low power enable and
// put PHY to USB safe state.
//
    val = FIELD_PREP(TCA_GCFG_OP_MODE, TCA_GCFG_OP_MODE_SYNCMODE);
    writel(val, tca.base + TCA_GCFG);
    val = TCA_TCPC_VALID | TCA_TCPC_LOW_POWER_EN;
    writel(val, tca.base + TCA_TCPC);
    goto out;
    }
// use System Configuration Mode for TCA mux control.
    val = FIELD_PREP(TCA_GCFG_OP_MODE, TCA_GCFG_OP_MODE_SYSMODE);
    writel(val, tca.base + TCA_GCFG);
// Disable TCA module
    val = readl(tca.base + TCA_SYSMODE_CFG);
    val |= TCA_SYSMODE_TCPC_DISABLE;
    writel(val, tca.base + TCA_SYSMODE_CFG);
    if (orientation == TYPEC_ORIENTATION_REVERSE)
    val |= TCA_SYSMODE_TCPC_FLIP;
#[no_mangle]
pub unsafe extern "C" fn if(TYPEC_ORIENTATION_NORMAL: orientation ==) -> else {
    else if (orientation == TYPEC_ORIENTATION_NORMAL)
    val &= ~TCA_SYSMODE_TCPC_FLIP;
    writel(val, tca.base + TCA_SYSMODE_CFG);
// Enable TCA module
    val &= ~TCA_SYSMODE_TCPC_DISABLE;
    writel(val, tca.base + TCA_SYSMODE_CFG);
    out:
    tca.orientation = orientation;
    mutex_unlock(&tca.mutex);
    }
#[no_mangle]
unsafe extern "C" fn tca_blk_init(tca: *mut tca_blk) {
    static void tca_blk_init(struct tca_blk *tca)
    {
    u32 val;
// reset XBar block
    val = readl(tca.base + TCA_CLK_RST);
    val &= ~TCA_CLK_RST_SW;
    writel(val, tca.base + TCA_CLK_RST);
    udelay(100);
// clear reset
    val |= TCA_CLK_RST_SW;
    writel(val, tca.base + TCA_CLK_RST);
    tca_blk_orientation_set(tca, tca.orientation);
    }
    static struct tca_blk *imx95_usb_phy_get_tca(struct platform_device *pdev,
    struct imx8mq_usb_phy *imx_phy)
    {
    struct device *dev = &pdev.dev;
    struct resource *res;
    struct tca_blk *tca;
    int ret;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!res)
    return core::ptr::null_mut();
    tca = devm_kzalloc(dev, sizeof(*tca), GFP_KERNEL);
    if (!tca)
    return ERR_PTR(-ENOMEM);
    tca.base = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(tca.base))
    return ERR_CAST(tca.base);
    mutex_init(&tca.mutex);
    tca.orientation = TYPEC_ORIENTATION_NORMAL;
    tca.sw = tca_blk_get_typec_switch(pdev, imx_phy);
    ret = devm_add_action_or_reset(&pdev.dev, tca_blk_put_typec_switch, tca.sw);
    if (ret)
    return ERR_PTR(ret);
    return tca;
    }
#[no_mangle]
unsafe extern "C" fn phy_tx_vref_tune_from_property(percent: u32) -> u32 {
    static u32 phy_tx_vref_tune_from_property(u32 percent)
    {
    percent = clamp(percent, 94U, 124U);
    return DIV_ROUND_CLOSEST(percent - 94U, 2);
    }
#[no_mangle]
unsafe extern "C" fn imx95_phy_tx_vref_tune_from_property(percent: u32) -> u32 {
    static u32 imx95_phy_tx_vref_tune_from_property(u32 percent)
    {
    percent = clamp(percent, 90U, 108U);
    switch (percent) {
    case 90 ... 91:
    percent = 0;
    break;
    case 92 ... 96:
    percent -= 91;
    break;
    case 97 ... 104:
    percent -= 92;
    break;
    case 105 ... 108:
    percent -= 93;
    break;
    }
    return percent;
    }
#[no_mangle]
unsafe extern "C" fn phy_tx_rise_tune_from_property(percent: u32) -> u32 {
    static u32 phy_tx_rise_tune_from_property(u32 percent)
    {
    switch (percent) {
    case 0 ... 98:
    return 3;
    case 99:
    return 2;
    case 100 ... 101:
    return 1;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn imx95_phy_tx_rise_tune_from_property(percent: u32) -> u32 {
    static u32 imx95_phy_tx_rise_tune_from_property(u32 percent)
    {
    percent = clamp(percent, 90U, 120U);
    switch (percent) {
    case 90 ... 99:
    return 3;
    case 101 ... 115:
    return 1;
    case 116 ... 120:
    return 0;
    default:
    return 2;
    }
    }
#[no_mangle]
unsafe extern "C" fn phy_tx_preemp_amp_tune_from_property(microamp: u32) -> u32 {
    static u32 phy_tx_preemp_amp_tune_from_property(u32 microamp)
    {
    microamp = min(microamp, 1800U);
    return microamp / 600;
    }
#[no_mangle]
unsafe extern "C" fn phy_tx_vboost_level_from_property(microvolt: u32) -> u32 {
    static u32 phy_tx_vboost_level_from_property(u32 microvolt)
    {
    switch (microvolt) {
    case 1156:
    return 5;
    case 844:
    return 3;
    default:
    return 4;
    }
    }
#[no_mangle]
unsafe extern "C" fn phy_pcs_tx_deemph_3p5db_from_property(decibel: u32) -> u32 {
    static u32 phy_pcs_tx_deemph_3p5db_from_property(u32 decibel)
    {
    return min(decibel, 36U);
    }
#[no_mangle]
unsafe extern "C" fn phy_comp_dis_tune_from_property(percent: u32) -> u32 {
    static u32 phy_comp_dis_tune_from_property(u32 percent)
    {
    switch (percent) {
    case 0 ... 92:
    return 0;
    case 93 ... 95:
    return 1;
    case 96 ... 97:
    return 2;
    case 98 ... 102:
    return 3;
    case 103 ... 105:
    return 4;
    case 106 ... 109:
    return 5;
    case 110 ... 113:
    return 6;
    default:
    return 7;
    }
    }
#[no_mangle]
unsafe extern "C" fn imx95_phy_comp_dis_tune_from_property(percent: u32) -> u32 {
    static u32 imx95_phy_comp_dis_tune_from_property(u32 percent)
    {
    percent = clamp(percent, 94, 104);
    switch (percent) {
    case 94 ... 95:
    percent = 0;
    break;
    case 96 ... 98:
    percent -= 95;
    break;
    case 99 ... 102:
    percent -= 96;
    break;
    case 103 ... 104:
    percent -= 97;
    break;
    }
    return percent;
    }
#[no_mangle]
unsafe extern "C" fn phy_pcs_tx_swing_full_from_property(percent: u32) -> u32 {
    static u32 phy_pcs_tx_swing_full_from_property(u32 percent)
    {
    percent = min(percent, 100U);
    return (percent * 127) / 100;
    }
#[no_mangle]
unsafe extern "C" fn imx8m_get_phy_tuning_data(imx_phy: *mut imx8mq_usb_phy) {
    static void imx8m_get_phy_tuning_data(struct imx8mq_usb_phy *imx_phy)
    {
    struct device *dev = imx_phy.phy.dev.parent;
    let mut is_imx95: bool = false;
    if (device_is_compatible(dev, "fsl,imx95-usb-phy"))
    is_imx95 = true;
    if (device_property_read_u32(dev, "fsl,phy-tx-vref-tune-percent",
    &imx_phy.tx_vref_tune))
    imx_phy.tx_vref_tune = PHY_TUNE_DEFAULT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_imx95) -> else {
    else if (is_imx95)
    imx_phy.tx_vref_tune =
    imx95_phy_tx_vref_tune_from_property(imx_phy.tx_vref_tune);
    else
    imx_phy.tx_vref_tune =
    phy_tx_vref_tune_from_property(imx_phy.tx_vref_tune);
    if (device_property_read_u32(dev, "fsl,phy-tx-rise-tune-percent",
    &imx_phy.tx_rise_tune))
    imx_phy.tx_rise_tune = PHY_TUNE_DEFAULT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_imx95) -> else {
    else if (is_imx95)
    imx_phy.tx_rise_tune =
    imx95_phy_tx_rise_tune_from_property(imx_phy.tx_rise_tune);
    else
    imx_phy.tx_rise_tune =
    phy_tx_rise_tune_from_property(imx_phy.tx_rise_tune);
    if (device_property_read_u32(dev, "fsl,phy-tx-preemp-amp-tune-microamp",
    &imx_phy.tx_preemp_amp_tune))
    imx_phy.tx_preemp_amp_tune = PHY_TUNE_DEFAULT;
    else
    imx_phy.tx_preemp_amp_tune =
    phy_tx_preemp_amp_tune_from_property(imx_phy.tx_preemp_amp_tune);
    if (device_property_read_u32(dev, "fsl,phy-tx-vboost-level-microvolt",
    &imx_phy.tx_vboost_level))
    imx_phy.tx_vboost_level = PHY_TUNE_DEFAULT;
    else
    imx_phy.tx_vboost_level =
    phy_tx_vboost_level_from_property(imx_phy.tx_vboost_level);
    if (device_property_read_u32(dev, "fsl,phy-comp-dis-tune-percent",
    &imx_phy.comp_dis_tune))
    imx_phy.comp_dis_tune = PHY_TUNE_DEFAULT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_imx95) -> else {
    else if (is_imx95)
    imx_phy.comp_dis_tune =
    imx95_phy_comp_dis_tune_from_property(imx_phy.comp_dis_tune);
    else
    imx_phy.comp_dis_tune =
    phy_comp_dis_tune_from_property(imx_phy.comp_dis_tune);
    if (device_property_read_u32(dev, "fsl,phy-pcs-tx-deemph-3p5db-attenuation-db",
    &imx_phy.pcs_tx_deemph_3p5db))
    imx_phy.pcs_tx_deemph_3p5db = PHY_TUNE_DEFAULT;
    else
    imx_phy.pcs_tx_deemph_3p5db =
    phy_pcs_tx_deemph_3p5db_from_property(imx_phy.pcs_tx_deemph_3p5db);
    if (device_property_read_u32(dev, "fsl,phy-pcs-tx-swing-full-percent",
    &imx_phy.pcs_tx_swing_full))
    imx_phy.pcs_tx_swing_full = PHY_TUNE_DEFAULT;
    else
    imx_phy.pcs_tx_swing_full =
    phy_pcs_tx_swing_full_from_property(imx_phy.pcs_tx_swing_full);
    }
#[no_mangle]
unsafe extern "C" fn imx8m_phy_tune(imx_phy: *mut imx8mq_usb_phy) {
    static void imx8m_phy_tune(struct imx8mq_usb_phy *imx_phy)
    {
    u32 value;
// PHY tuning
    if (imx_phy.pcs_tx_deemph_3p5db != PHY_TUNE_DEFAULT) {
    value = readl(imx_phy.base + PHY_CTRL4);
    value &= ~PHY_CTRL4_PCS_TX_DEEMPH_3P5DB_MASK;
    value |= FIELD_PREP(PHY_CTRL4_PCS_TX_DEEMPH_3P5DB_MASK,
    imx_phy.pcs_tx_deemph_3p5db);
    writel(value, imx_phy.base + PHY_CTRL4);
    }
    if (imx_phy.pcs_tx_swing_full != PHY_TUNE_DEFAULT) {
    value = readl(imx_phy.base + PHY_CTRL5);
    value &= ~PHY_CTRL5_PCS_TX_SWING_FULL_MASK;
    value |= FIELD_PREP(PHY_CTRL5_PCS_TX_SWING_FULL_MASK,
    imx_phy.pcs_tx_swing_full);
    writel(value, imx_phy.base + PHY_CTRL5);
    }
    if ((imx_phy.tx_vref_tune & imx_phy.tx_rise_tune &
    imx_phy.tx_preemp_amp_tune & imx_phy.comp_dis_tune &
    imx_phy.tx_vboost_level) == PHY_TUNE_DEFAULT)
// If all are the default values, no need update.
    return;
    value = readl(imx_phy.base + PHY_CTRL3);
    if (imx_phy.tx_vref_tune != PHY_TUNE_DEFAULT) {
    value &= ~PHY_CTRL3_TXVREF_TUNE_MASK;
    value |= FIELD_PREP(PHY_CTRL3_TXVREF_TUNE_MASK,
    imx_phy.tx_vref_tune);
    }
    if (imx_phy.tx_rise_tune != PHY_TUNE_DEFAULT) {
    value &= ~PHY_CTRL3_TXRISE_TUNE_MASK;
    value |= FIELD_PREP(PHY_CTRL3_TXRISE_TUNE_MASK,
    imx_phy.tx_rise_tune);
    }
    if (imx_phy.tx_preemp_amp_tune != PHY_TUNE_DEFAULT) {
    value &= ~PHY_CTRL3_TXPREEMP_TUNE_MASK;
    value |= FIELD_PREP(PHY_CTRL3_TXPREEMP_TUNE_MASK,
    imx_phy.tx_preemp_amp_tune);
    }
    if (imx_phy.comp_dis_tune != PHY_TUNE_DEFAULT) {
    value &= ~PHY_CTRL3_COMPDISTUNE_MASK;
    value |= FIELD_PREP(PHY_CTRL3_COMPDISTUNE_MASK,
    imx_phy.comp_dis_tune);
    }
    if (imx_phy.tx_vboost_level != PHY_TUNE_DEFAULT) {
    value &= ~PHY_CTRL3_TX_VBOOST_LEVEL_MASK;
    value |= FIELD_PREP(PHY_CTRL3_TX_VBOOST_LEVEL_MASK,
    imx_phy.tx_vboost_level);
    }
    writel(value, imx_phy.base + PHY_CTRL3);
    }
#[no_mangle]
unsafe extern "C" fn imx8mq_usb_phy_init(phy: *mut phy) -> c_int {
    static int imx8mq_usb_phy_init(struct phy *phy)
    {
    struct imx8mq_usb_phy *imx_phy = phy_get_drvdata(phy);
    u32 value;
    value = readl(imx_phy.base + PHY_CTRL1);
    value &= ~(PHY_CTRL1_VDATSRCENB0 | PHY_CTRL1_VDATDETENB0 |
    PHY_CTRL1_COMMONONN);
    value |= PHY_CTRL1_RESET | PHY_CTRL1_ATERESET;
    writel(value, imx_phy.base + PHY_CTRL1);
    value = readl(imx_phy.base + PHY_CTRL0);
    value |= PHY_CTRL0_REF_SSP_EN;
    writel(value, imx_phy.base + PHY_CTRL0);
    value = readl(imx_phy.base + PHY_CTRL2);
    value |= PHY_CTRL2_TXENABLEN0;
    writel(value, imx_phy.base + PHY_CTRL2);
    value = readl(imx_phy.base + PHY_CTRL1);
    value &= ~(PHY_CTRL1_RESET | PHY_CTRL1_ATERESET);
    writel(value, imx_phy.base + PHY_CTRL1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_usb_phy_init(phy: *mut phy) -> c_int {
    static int imx8mp_usb_phy_init(struct phy *phy)
    {
    struct imx8mq_usb_phy *imx_phy = phy_get_drvdata(phy);
    u32 value;
// USB3.0 PHY signal fsel for 24M ref
    value = readl(imx_phy.base + PHY_CTRL0);
    value &= ~PHY_CTRL0_FSEL_MASK;
    value |= FIELD_PREP(PHY_CTRL0_FSEL_MASK, imx_phy.alt_clk ?
    PHY_CTRL0_FSEL_100M : PHY_CTRL0_FSEL_24M);
    writel(value, imx_phy.base + PHY_CTRL0);
// Disable alt_clk_en and use internal MPLL clocks
    value = readl(imx_phy.base + PHY_CTRL6);
    value &= ~(PHY_CTRL6_ALT_CLK_SEL | PHY_CTRL6_ALT_CLK_EN);
    writel(value, imx_phy.base + PHY_CTRL6);
    value = readl(imx_phy.base + PHY_CTRL1);
    value &= ~(PHY_CTRL1_VDATSRCENB0 | PHY_CTRL1_VDATDETENB0);
    value |= PHY_CTRL1_RESET | PHY_CTRL1_ATERESET;
    writel(value, imx_phy.base + PHY_CTRL1);
    value = readl(imx_phy.base + PHY_CTRL0);
    value |= PHY_CTRL0_REF_SSP_EN;
    value &= ~PHY_CTRL0_SSC_RANGE_MASK;
    value |= FIELD_PREP(PHY_CTRL0_SSC_RANGE_MASK,
    PHY_CTRL0_SSC_RANGE_4003PPM);
    writel(value, imx_phy.base + PHY_CTRL0);
    value = readl(imx_phy.base + PHY_CTRL2);
    value |= PHY_CTRL2_TXENABLEN0 | PHY_CTRL2_OTG_DISABLE;
    writel(value, imx_phy.base + PHY_CTRL2);
    udelay(10);
    value = readl(imx_phy.base + PHY_CTRL1);
    value &= ~(PHY_CTRL1_RESET | PHY_CTRL1_ATERESET);
    writel(value, imx_phy.base + PHY_CTRL1);
    imx8m_phy_tune(imx_phy);
    if (imx_phy.tca)
    tca_blk_init(imx_phy.tca);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mq_phy_power_on(phy: *mut phy) -> c_int {
    static int imx8mq_phy_power_on(struct phy *phy)
    {
    struct imx8mq_usb_phy *imx_phy = phy_get_drvdata(phy);
    u32 value;
    int ret;
    ret = regulator_enable(imx_phy.vbus);
    if (ret)
    return ret;
// Disable rx term override
    value = readl(imx_phy.base + PHY_CTRL6);
    value &= ~PHY_CTRL6_RXTERM_OVERRIDE_SEL;
    writel(value, imx_phy.base + PHY_CTRL6);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mq_phy_power_off(phy: *mut phy) -> c_int {
    static int imx8mq_phy_power_off(struct phy *phy)
    {
    struct imx8mq_usb_phy *imx_phy = phy_get_drvdata(phy);
    u32 value;
// Override rx term to be 0
    value = readl(imx_phy.base + PHY_CTRL6);
    value |= PHY_CTRL6_RXTERM_OVERRIDE_SEL;
    writel(value, imx_phy.base + PHY_CTRL6);
    regulator_disable(imx_phy.vbus);
    return 0;
    }
    static const struct phy_ops imx8mq_usb_phy_ops = {
    .init		= imx8mq_usb_phy_init,
    .power_on	= imx8mq_phy_power_on,
    .power_off	= imx8mq_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct phy_ops imx8mp_usb_phy_ops = {
    .init		= imx8mp_usb_phy_init,
    .power_on	= imx8mq_phy_power_on,
    .power_off	= imx8mq_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct imx8mq_usb_phy_drvdata imx8mq_usb_phy_data = {
    .ops = &imx8mq_usb_phy_ops,
    };
    static const struct imx8mq_usb_phy_drvdata imx8mp_usb_phy_data = {
    .ops = &imx8mp_usb_phy_ops,
    .need_genpd_rpm_on = true,
    };
    static const struct imx8mq_usb_phy_drvdata imx95_usb_phy_data = {
    .ops = &imx8mp_usb_phy_ops,
    };
    static const struct of_device_id imx8mq_usb_phy_of_match[] = {
    {.compatible = "fsl,imx8mq-usb-phy",
    .data = &imx8mq_usb_phy_data,},
    {.compatible = "fsl,imx8mp-usb-phy",
    .data = &imx8mp_usb_phy_data,},
    {.compatible = "fsl,imx95-usb-phy",
    .data = &imx95_usb_phy_data,},
    { }
    };
    MODULE_DEVICE_TABLE(of, imx8mq_usb_phy_of_match);
    static const struct regmap_config imx_cr_regmap_config = {
    .name = "cr",
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = 0x7,
    };
#[no_mangle]
unsafe extern "C" fn imx8mq_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int imx8mq_usb_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct imx8mq_usb_phy *imx_phy;
    const struct imx8mq_usb_phy_drvdata *phy_data;
    int ret;
    imx_phy = devm_kzalloc(dev, sizeof(*imx_phy), GFP_KERNEL);
    if (!imx_phy)
    return -ENOMEM;
    platform_set_drvdata(pdev, imx_phy);
    imx_phy.clk = devm_clk_get_enabled(dev, "phy");
    if (IS_ERR(imx_phy.clk)) {
    dev_err(dev, "failed to get imx8mq usb phy clock\n");
    return PTR_ERR(imx_phy.clk);
    }
    imx_phy.alt_clk = devm_clk_get_optional_enabled(dev, "alt");
    if (IS_ERR(imx_phy.alt_clk))
    return dev_err_probe(dev, PTR_ERR(imx_phy.alt_clk),
    "Failed to get alt clk\n");
    imx_phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(imx_phy.base))
    return PTR_ERR(imx_phy.base);
    imx_phy.cr_regmap = devm_regmap_init_mmio(dev, imx_phy.base + PHY_CRCTL,
    &imx_cr_regmap_config);
    if (IS_ERR(imx_phy.cr_regmap)) {
    dev_warn(dev, "Fail to init debug register regmap\n");
    imx_phy.cr_regmap = core::ptr::null_mut();
    }
    imx_phy.vbus = devm_regulator_get(dev, "vbus");
    if (IS_ERR(imx_phy.vbus))
    return dev_err_probe(dev, PTR_ERR(imx_phy.vbus), "failed to get vbus\n");
    phy_data = of_device_get_match_data(dev);
    if (!phy_data)
    return -EINVAL;
    if (phy_data.need_genpd_rpm_on) {
    ret = dev_pm_genpd_rpm_always_on(dev, true);
    if (ret && ret != -EOPNOTSUPP)
    dev_warn(dev, "failed to set genpd rpm always on\n");
    }
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    imx_phy.phy = devm_phy_create(dev, core::ptr::null_mut(), phy_data.ops);
    if (IS_ERR(imx_phy.phy)) {
    ret = dev_err_probe(dev, PTR_ERR(imx_phy.phy),
    "failed to create PHY\n");
    goto disable_rpm;
    }
    phy_set_drvdata(imx_phy.phy, imx_phy);
    imx_phy.tca = imx95_usb_phy_get_tca(pdev, imx_phy);
    if (IS_ERR(imx_phy.tca)) {
    ret = dev_err_probe(dev, PTR_ERR(imx_phy.tca),
    "failed to get tca\n");
    goto disable_rpm;
    }
    imx8m_get_phy_tuning_data(imx_phy);
    device_set_wakeup_capable(dev, true);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    ret = dev_err_probe(dev, PTR_ERR(phy_provider),
    "failed to register phy provider\n");
    goto disable_rpm;
    }
    return 0;
    disable_rpm:
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8mq_usb_phy_remove(pdev: *mut platform_device) {
    static void imx8mq_usb_phy_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    ret = pm_runtime_get_sync(dev);
    if (ret < 0)
    dev_warn(dev, "failed to resume on remove: %d\n", ret);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    }
#[no_mangle]
unsafe extern "C" fn imx8mq_usb_phy_runtime_suspend(dev: *mut device) -> c_int {
    static int imx8mq_usb_phy_runtime_suspend(struct device *dev)
    {
    struct imx8mq_usb_phy *imx_phy = dev_get_drvdata(dev);
    if (imx_phy.cr_regmap)
    regcache_cache_only(imx_phy.cr_regmap, true);
    clk_disable_unprepare(imx_phy.alt_clk);
    clk_disable_unprepare(imx_phy.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mq_usb_phy_runtime_resume(dev: *mut device) -> c_int {
    static int imx8mq_usb_phy_runtime_resume(struct device *dev)
    {
    struct imx8mq_usb_phy *imx_phy = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(imx_phy.clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(imx_phy.alt_clk);
    if (ret) {
    clk_disable_unprepare(imx_phy.clk);
    return ret;
    }
    if (imx_phy.cr_regmap)
    regcache_cache_only(imx_phy.cr_regmap, false);
    return 0;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(imx8mq_usb_phy_pm_ops, imx8mq_usb_phy_runtime_suspend,
    imx8mq_usb_phy_runtime_resume, core::ptr::null_mut());
    static struct platform_driver imx8mq_usb_phy_driver = {
    .probe	= imx8mq_usb_phy_probe,
    .remove = imx8mq_usb_phy_remove,
    .driver = {
    .name	= "imx8mq-usb-phy",
    .of_match_table	= imx8mq_usb_phy_of_match,
    .pm = pm_ptr(&imx8mq_usb_phy_pm_ops),
    .suppress_bind_attrs = true,
    }
    };
    module_platform_driver(imx8mq_usb_phy_driver);
    MODULE_DESCRIPTION("FSL IMX8MQ USB PHY driver");
    MODULE_LICENSE("GPL");
