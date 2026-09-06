//! Automatically rewritten from C to Rust
//! Source: drivers/phy/mediatek/phy-mtk-xsphy.c
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


// SPDX-License-Identifier: GPL-2.0
//
// MediaTek USB3.1 gen2 xsphy Driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Chunfeng Yun <chunfeng.yun@mediatek.com>
//

// u2 phy banks
pub const SSUSB_SIFSLV_MISC: c_uint = 0x000;
pub const SSUSB_SIFSLV_U2FREQ: c_uint = 0x100;
pub const SSUSB_SIFSLV_U2PHY_COM: c_uint = 0x300;
// u3 phy shared banks
pub const SSPXTP_SIFSLV_DIG_GLB: c_uint = 0x000;
pub const SSPXTP_SIFSLV_PHYA_GLB: c_uint = 0x100;
// u3 phy banks
pub const SSPXTP_SIFSLV_DIG_LN_TOP: c_uint = 0x000;
pub const SSPXTP_SIFSLV_DIG_LN_TX0: c_uint = 0x100;
pub const SSPXTP_SIFSLV_DIG_LN_RX0: c_uint = 0x200;
pub const SSPXTP_SIFSLV_DIG_LN_DAIF: c_uint = 0x300;
pub const SSPXTP_SIFSLV_PHYA_LN: c_uint = 0x400;

pub const XSP_SLEW_RATE_COEF: c_int = 17;
pub const XSP_SR_COEF_DIVISOR: c_int = 1000;
pub const XSP_FM_DET_CYCLE_CNT: c_int = 1024;
// PHY switch between pcie/usb3/sgmii
pub const USB_PHY_SWITCH_CTRL: c_uint = 0x0;

pub const RG_PHY_SW_PCIE: c_uint = 0x0;
pub const RG_PHY_SW_USB3: c_uint = 0x1;
pub const RG_PHY_SW_SGMII: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsphy_instance {
    pub phy: *mut phy,
    pub port_base: *mut void __iomem,
    pub /: *mut *mut *mut clk ref_clk; / reference clock of anolog phy,
    pub index: u32,
    pub type: u32,
    pub type_sw: *mut regmap,
    pub type_sw_reg: u32,
    pub type_sw_index: u32,
// only for HQA test
    pub efuse_intr: c_int,
    pub efuse_tx_imp: c_int,
    pub efuse_rx_imp: c_int,
// u2 eye diagram
    pub eye_src: c_int,
    pub eye_vrt: c_int,
    pub eye_term: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_xsphy {
    pub dev: *mut device,
    pub /: *mut *mut *mut void __iomem glb_base; / only shared u3 sif,
    pub /: *mut *mut int src_ref_clk; / MHZ, reference clock for slew rate calibrate,
    pub /: *mut *mut int src_coef; / coefficient for slew rate calibrate,
    pub nphys: c_int,
    pub __counted_by(nphys): *mut *mut xsphy_instance phys[],
}

    static void u2_phy_slew_rate_calibrate(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst)
    {
    void __iomem *pbase = inst.port_base;
    int calib_val;
    int fm_out;
    u32 tmp;
// use force value
    if (inst.eye_src)
    return;
// enable USB ring oscillator
    mtk_phy_set_bits(pbase + XSP_USBPHYACR5, P2A5_RG_HSTX_SRCAL_EN);
    udelay(1);	/* wait clock stable */
// enable free run clock
    mtk_phy_set_bits(pbase + XSP_U2FREQ_FMMONR1, P2F_RG_FRCK_EN);
// set cycle count as 1024
    mtk_phy_update_field(pbase + XSP_U2FREQ_FMCR0, P2F_RG_CYCLECNT,
    XSP_FM_DET_CYCLE_CNT);
// enable frequency meter
    mtk_phy_set_bits(pbase + XSP_U2FREQ_FMCR0, P2F_RG_FREQDET_EN);
// ignore return value
    readl_poll_timeout(pbase + XSP_U2FREQ_FMMONR1, tmp,
    (tmp & P2F_USB_FM_VALID), 10, 200);
    fm_out = readl(pbase + XSP_U2FREQ_MMONR0);
// disable frequency meter
    mtk_phy_clear_bits(pbase + XSP_U2FREQ_FMCR0, P2F_RG_FREQDET_EN);
// disable free run clock
    mtk_phy_clear_bits(pbase + XSP_U2FREQ_FMMONR1, P2F_RG_FRCK_EN);
    if (fm_out) {
// (1024 / FM_OUT) x reference clock frequency x coefficient
    tmp = xsphy.src_ref_clk * xsphy.src_coef;
    tmp = (tmp * XSP_FM_DET_CYCLE_CNT) / fm_out;
    calib_val = DIV_ROUND_CLOSEST(tmp, XSP_SR_COEF_DIVISOR);
    } else {
// if FM detection fail, set default value
    calib_val = 3;
    }
    dev_dbg(xsphy.dev, "phy.%d, fm_out:%d, calib:%d (clk:%d, coef:%d)\n",
    inst.index, fm_out, calib_val,
    xsphy.src_ref_clk, xsphy.src_coef);
// set HS slew rate
    mtk_phy_update_field(pbase + XSP_USBPHYACR5, P2A5_RG_HSTX_SRCTRL, calib_val);
// disable USB ring oscillator
    mtk_phy_clear_bits(pbase + XSP_USBPHYACR5, P2A5_RG_HSTX_SRCAL_EN);
    }
    static void u2_phy_instance_init(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst)
    {
    void __iomem *pbase = inst.port_base;
// DP/DM BC1.1 path Disable
    mtk_phy_clear_bits(pbase + XSP_USBPHYACR6, P2A6_RG_BC11_SW_EN);
    mtk_phy_set_bits(pbase + XSP_USBPHYACR0, P2A0_RG_INTR_EN);
    }
    static void u2_phy_instance_power_on(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst)
    {
    void __iomem *pbase = inst.port_base;
    let mut index: u32 = inst.index;
    mtk_phy_set_bits(pbase + XSP_USBPHYACR6, P2A6_RG_OTG_VBUSCMP_EN);
    mtk_phy_update_bits(pbase + XSP_U2PHYDTM1,
    P2D_RG_VBUSVALID | P2D_RG_AVALID | P2D_RG_SESSEND,
    P2D_RG_VBUSVALID | P2D_RG_AVALID);
    dev_dbg(xsphy.dev, "%s(%d)\n", __func__, index);
    }
    static void u2_phy_instance_power_off(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst)
    {
    void __iomem *pbase = inst.port_base;
    let mut index: u32 = inst.index;
    mtk_phy_clear_bits(pbase + XSP_USBPHYACR6, P2A6_RG_OTG_VBUSCMP_EN);
    mtk_phy_update_bits(pbase + XSP_U2PHYDTM1,
    P2D_RG_VBUSVALID | P2D_RG_AVALID | P2D_RG_SESSEND,
    P2D_RG_SESSEND);
    dev_dbg(xsphy.dev, "%s(%d)\n", __func__, index);
    }
    static void u2_phy_instance_set_mode(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst,
    enum phy_mode mode)
    {
    u32 tmp;
    tmp = readl(inst.port_base + XSP_U2PHYDTM1);
    switch (mode) {
    case PHY_MODE_USB_DEVICE:
    tmp |= P2D_FORCE_IDDIG | P2D_RG_IDDIG;
    break;
    case PHY_MODE_USB_HOST:
    tmp |= P2D_FORCE_IDDIG;
    tmp &= ~P2D_RG_IDDIG;
    break;
    case PHY_MODE_USB_OTG:
    tmp &= ~(P2D_FORCE_IDDIG | P2D_RG_IDDIG);
    break;
    default:
    return;
    }
    writel(tmp, inst.port_base + XSP_U2PHYDTM1);
    }
    static void phy_parse_property(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst)
    {
    struct device *dev = &inst.phy.dev;
    switch (inst.type) {
    case PHY_TYPE_USB2:
    device_property_read_u32(dev, "mediatek,efuse-intr",
    &inst.efuse_intr);
    device_property_read_u32(dev, "mediatek,eye-src",
    &inst.eye_src);
    device_property_read_u32(dev, "mediatek,eye-vrt",
    &inst.eye_vrt);
    device_property_read_u32(dev, "mediatek,eye-term",
    &inst.eye_term);
    dev_dbg(dev, "intr:%d, src:%d, vrt:%d, term:%d\n",
    inst.efuse_intr, inst.eye_src,
    inst.eye_vrt, inst.eye_term);
    break;
    case PHY_TYPE_USB3:
    device_property_read_u32(dev, "mediatek,efuse-intr",
    &inst.efuse_intr);
    device_property_read_u32(dev, "mediatek,efuse-tx-imp",
    &inst.efuse_tx_imp);
    device_property_read_u32(dev, "mediatek,efuse-rx-imp",
    &inst.efuse_rx_imp);
    dev_dbg(dev, "intr:%d, tx-imp:%d, rx-imp:%d\n",
    inst.efuse_intr, inst.efuse_tx_imp,
    inst.efuse_rx_imp);
    break;
    case PHY_TYPE_PCIE:
    case PHY_TYPE_SGMII:
// nothing to do
    break;
    default:
    dev_err(xsphy.dev, "incompatible phy type\n");
    return;
    }
    }
    static void u2_phy_props_set(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst)
    {
    void __iomem *pbase = inst.port_base;
    if (inst.efuse_intr)
    mtk_phy_update_field(pbase + XSP_USBPHYACR1, P2A1_RG_INTR_CAL,
    inst.efuse_intr);
    if (inst.eye_src)
    mtk_phy_update_field(pbase + XSP_USBPHYACR5, P2A5_RG_HSTX_SRCTRL,
    inst.eye_src);
    if (inst.eye_vrt)
    mtk_phy_update_field(pbase + XSP_USBPHYACR1, P2A1_RG_VRT_SEL,
    inst.eye_vrt);
    if (inst.eye_term)
    mtk_phy_update_field(pbase + XSP_USBPHYACR1, P2A1_RG_TERM_SEL,
    inst.eye_term);
    }
    static void u3_phy_props_set(struct mtk_xsphy *xsphy,
    struct xsphy_instance *inst)
    {
    void __iomem *pbase = inst.port_base;
    if (inst.efuse_intr)
    mtk_phy_update_field(xsphy.glb_base + SSPXTP_PHYA_GLB_00,
    RG_XTP_GLB_BIAS_INTR_CTRL, inst.efuse_intr);
    if (inst.efuse_tx_imp)
    mtk_phy_update_field(pbase + SSPXTP_PHYA_LN_04,
    RG_XTP_LN0_TX_IMPSEL, inst.efuse_tx_imp);
    if (inst.efuse_rx_imp)
    mtk_phy_update_field(pbase + SSPXTP_PHYA_LN_14,
    RG_XTP_LN0_RX_IMPSEL, inst.efuse_rx_imp);
    }
// type switch for usb3/pcie/sgmii
    static int phy_type_syscon_get(struct xsphy_instance *instance,
    struct device_node *dn)
    {
    struct of_phandle_args args;
    int ret;
// type switch function is optional
    if (!of_property_present(dn, "mediatek,syscon-type"))
    return 0;
    ret = of_parse_phandle_with_fixed_args(dn, "mediatek,syscon-type",
    2, 0, &args);
    if (ret)
    return ret;
    instance.type_sw_reg = args.args[0];
    instance.type_sw_index = args.args[1] & 0x3; /* <=3 */
    instance.type_sw = syscon_node_to_regmap(args.np);
    of_node_put(args.np);
    dev_info(&instance.phy.dev, "type_sw - reg %#x, index %d\n",
    instance.type_sw_reg, instance.type_sw_index);
    return PTR_ERR_OR_ZERO(instance.type_sw);
    }
#[no_mangle]
unsafe extern "C" fn phy_type_set(instance: *mut xsphy_instance) -> c_int {
    static int phy_type_set(struct xsphy_instance *instance)
    {
    int type;
    u32 offset;
    if (!instance.type_sw)
    return 0;
    switch (instance.type) {
    case PHY_TYPE_USB3:
    type = RG_PHY_SW_USB3;
    break;
    case PHY_TYPE_PCIE:
    type = RG_PHY_SW_PCIE;
    break;
    case PHY_TYPE_SGMII:
    type = RG_PHY_SW_SGMII;
    break;
    case PHY_TYPE_USB2:
    default:
    return 0;
    }
    offset = instance.type_sw_index * BITS_PER_BYTE;
    regmap_update_bits(instance.type_sw, instance.type_sw_reg,
    RG_PHY_SW_TYPE << offset, type << offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_phy_init(phy: *mut phy) -> c_int {
    static int mtk_phy_init(struct phy *phy)
    {
    struct xsphy_instance *inst = phy_get_drvdata(phy);
    struct mtk_xsphy *xsphy = dev_get_drvdata(phy.dev.parent);
    int ret;
    ret = clk_prepare_enable(inst.ref_clk);
    if (ret) {
    dev_err(xsphy.dev, "failed to enable ref_clk\n");
    return ret;
    }
    switch (inst.type) {
    case PHY_TYPE_USB2:
    u2_phy_instance_init(xsphy, inst);
    u2_phy_props_set(xsphy, inst);
    break;
    case PHY_TYPE_USB3:
    u3_phy_props_set(xsphy, inst);
    break;
    case PHY_TYPE_PCIE:
    case PHY_TYPE_SGMII:
// nothing to do, only used to set type
    break;
    default:
    dev_err(xsphy.dev, "incompatible phy type\n");
    clk_disable_unprepare(inst.ref_clk);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_phy_power_on(phy: *mut phy) -> c_int {
    static int mtk_phy_power_on(struct phy *phy)
    {
    struct xsphy_instance *inst = phy_get_drvdata(phy);
    struct mtk_xsphy *xsphy = dev_get_drvdata(phy.dev.parent);
    if (inst.type == PHY_TYPE_USB2) {
    u2_phy_instance_power_on(xsphy, inst);
    u2_phy_slew_rate_calibrate(xsphy, inst);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_phy_power_off(phy: *mut phy) -> c_int {
    static int mtk_phy_power_off(struct phy *phy)
    {
    struct xsphy_instance *inst = phy_get_drvdata(phy);
    struct mtk_xsphy *xsphy = dev_get_drvdata(phy.dev.parent);
    if (inst.type == PHY_TYPE_USB2)
    u2_phy_instance_power_off(xsphy, inst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_phy_exit(phy: *mut phy) -> c_int {
    static int mtk_phy_exit(struct phy *phy)
    {
    struct xsphy_instance *inst = phy_get_drvdata(phy);
    clk_disable_unprepare(inst.ref_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_phy_set_mode(phy: *mut phy, mode: enum phy_mode, submode: c_int) -> c_int {
    static int mtk_phy_set_mode(struct phy *phy, enum phy_mode mode, int submode)
    {
    struct xsphy_instance *inst = phy_get_drvdata(phy);
    struct mtk_xsphy *xsphy = dev_get_drvdata(phy.dev.parent);
    if (inst.type == PHY_TYPE_USB2)
    u2_phy_instance_set_mode(xsphy, inst, mode);
    return 0;
    }
    static struct phy *mtk_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct mtk_xsphy *xsphy = dev_get_drvdata(dev);
    struct xsphy_instance *inst = core::ptr::null_mut();
    struct device_node *phy_np = args.np;
    int index;
    if (args.args_count != 1) {
    dev_err(dev, "invalid number of cells in 'phy' property\n");
    return ERR_PTR(-EINVAL);
    }
    for (index = 0; index < xsphy.nphys; index++)
    if (phy_np == xsphy.phys[index].phy.dev.of_node) {
    inst = xsphy.phys[index];
    break;
    }
    if (!inst) {
    dev_err(dev, "failed to find appropriate phy\n");
    return ERR_PTR(-EINVAL);
    }
    inst.type = args.args[0];
    if (!(inst.type == PHY_TYPE_USB2 ||
    inst.type == PHY_TYPE_USB3 ||
    inst.type == PHY_TYPE_PCIE ||
    inst.type == PHY_TYPE_SGMII)) {
    dev_err(dev, "unsupported phy type: %d\n", inst.type);
    return ERR_PTR(-EINVAL);
    }
    phy_parse_property(xsphy, inst);
    phy_type_set(inst);
    return inst.phy;
    }
    static const struct phy_ops mtk_xsphy_ops = {
    .init		= mtk_phy_init,
    .exit		= mtk_phy_exit,
    .power_on	= mtk_phy_power_on,
    .power_off	= mtk_phy_power_off,
    .set_mode	= mtk_phy_set_mode,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id mtk_xsphy_id_table[] = {
    { .compatible = "mediatek,xsphy", },
    { },
    };
    MODULE_DEVICE_TABLE(of, mtk_xsphy_id_table);
#[no_mangle]
unsafe extern "C" fn mtk_xsphy_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_xsphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct phy_provider *provider;
    struct resource *glb_res;
    struct mtk_xsphy *xsphy;
    struct resource res;
    size_t nphys;
    int port;
    nphys = of_get_child_count(np);
    xsphy = devm_kzalloc(dev, struct_size(xsphy, phys, nphys), GFP_KERNEL);
    if (!xsphy)
    return -ENOMEM;
    xsphy.nphys = nphys;
    xsphy.dev = dev;
    platform_set_drvdata(pdev, xsphy);
    glb_res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
// optional, may not exist if no u3 phys
    if (glb_res) {
// get banks shared by multiple u3 phys
    xsphy.glb_base = devm_ioremap_resource(dev, glb_res);
    if (IS_ERR(xsphy.glb_base)) {
    dev_err(dev, "failed to remap glb regs\n");
    return PTR_ERR(xsphy.glb_base);
    }
    }
    xsphy.src_ref_clk = XSP_REF_CLK;
    xsphy.src_coef = XSP_SLEW_RATE_COEF;
// update parameters of slew rate calibrate if exist
    device_property_read_u32(dev, "mediatek,src-ref-clk-mhz",
    &xsphy.src_ref_clk);
    device_property_read_u32(dev, "mediatek,src-coef", &xsphy.src_coef);
    port = 0;
    for_each_child_of_node_scoped(np, child_np) {
    struct xsphy_instance *inst;
    struct phy *phy;
    int retval;
    inst = devm_kzalloc(dev, sizeof(*inst), GFP_KERNEL);
    if (!inst)
    return -ENOMEM;
    xsphy.phys[port] = inst;
    phy = devm_phy_create(dev, child_np, &mtk_xsphy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create phy\n");
    return PTR_ERR(phy);
    }
    retval = of_address_to_resource(child_np, 0, &res);
    if (retval) {
    dev_err(dev, "failed to get address resource(id-%d)\n",
    port);
    return retval;
    }
    inst.port_base = devm_ioremap_resource(&phy.dev, &res);
    if (IS_ERR(inst.port_base)) {
    dev_err(dev, "failed to remap phy regs\n");
    return PTR_ERR(inst.port_base);
    }
    inst.phy = phy;
    inst.index = port;
    phy_set_drvdata(phy, inst);
    port++;
    inst.ref_clk = devm_clk_get(&phy.dev, "ref");
    if (IS_ERR(inst.ref_clk)) {
    dev_err(dev, "failed to get ref_clk(id-%d)\n", port);
    return PTR_ERR(inst.ref_clk);
    }
    retval = phy_type_syscon_get(inst, child_np);
    if (retval)
    return retval;
    }
    provider = devm_of_phy_provider_register(dev, mtk_phy_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static struct platform_driver mtk_xsphy_driver = {
    .probe		= mtk_xsphy_probe,
    .driver		= {
    .name	= "mtk-xsphy",
    .of_match_table = mtk_xsphy_id_table,
    },
    };
    module_platform_driver(mtk_xsphy_driver);
    MODULE_AUTHOR("Chunfeng Yun <chunfeng.yun@mediatek.com>");
    MODULE_DESCRIPTION("MediaTek USB XS-PHY driver");
    MODULE_LICENSE("GPL v2");
