//! Automatically rewritten from C to Rust
//! Source: drivers/phy/phy-eyeq5-eth.c
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

pub const EQ5_PHY_COUNT: c_int = 2;
pub const EQ5_PHY0_GP: c_uint = 0x128;
pub const EQ5_PHY1_GP: c_uint = 0x12c;
pub const EQ5_PHY0_SGMII: c_uint = 0x134;
pub const EQ5_PHY1_SGMII: c_uint = 0x138;

//
// Instead of storing a phy_interface_t, we store this enum.
//
// We do not deal with RGMII timings in this generic PHY driver,
// it is all handled inside the net PHY.
//
    enum eq5_phy_submode {
    EQ5_PHY_SUBMODE_SGMII,
    EQ5_PHY_SUBMODE_RGMII,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq5_phy_inst {
    pub dev: *mut device,
    pub phy: *mut phy,
    pub sgmii: *mut *mut void __iomem gp,,
    pub submode: enum eq5_phy_submode,
    pub sgmii_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq5_phy_private {
    pub phys: [eq5_phy_inst; EQ5_PHY_COUNT],
}

#[no_mangle]
unsafe extern "C" fn eq5_phy_exit(phy: *mut phy) -> c_int {
    static int eq5_phy_exit(struct phy *phy)
    {
    struct eq5_phy_inst *inst = phy_get_drvdata(phy);
    writel(0, inst.gp);
    writel(0, inst.sgmii);
    udelay(5); /* settling time */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eq5_phy_init(phy: *mut phy) -> c_int {
    static int eq5_phy_init(struct phy *phy)
    {
    struct eq5_phy_inst *inst = phy_get_drvdata(phy);
    u32 reg;
//
// Hardware stops listening to our instructions once it is started.
// It must be reset to reconfigure it.
//
    eq5_phy_exit(phy);
    reg = EQ5_GP_TX_SWRST_DIS | EQ5_GP_TX_M_CLKE |
    EQ5_GP_SYS_SWRST_DIS | EQ5_GP_SYS_M_CLKE |
    FIELD_PREP(EQ5_GP_RGMII_DRV, 0x9);
    writel(reg, inst.gp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eq5_phy_power_on(phy: *mut phy) -> c_int {
    static int eq5_phy_power_on(struct phy *phy)
    {
    struct eq5_phy_inst *inst = phy_get_drvdata(phy);
    u32 reg;
    if (inst.submode == EQ5_PHY_SUBMODE_SGMII) {
    writel(readl(inst.gp) | EQ5_GP_SGMII_MODE, inst.gp);
    reg = EQ5_SGMII_PWR_EN | EQ5_SGMII_RST_DIS | EQ5_SGMII_PLL_EN;
    writel(reg, inst.sgmii);
    if (readl_poll_timeout(inst.sgmii, reg,
    reg & EQ5_SGMII_PLL_ACK, 1, 100)) {
    dev_err(inst.dev, "PLL timeout\n");
    return -ETIMEDOUT;
    }
    reg = readl(inst.sgmii);
    reg |= EQ5_SGMII_PWR_STATE | EQ5_SGMII_SIG_DET_SW;
    writel(reg, inst.sgmii);
    } else {
    writel(readl(inst.gp) & ~EQ5_GP_SGMII_MODE, inst.gp);
    writel(0, inst.sgmii);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eq5_phy_power_off(phy: *mut phy) -> c_int {
    static int eq5_phy_power_off(struct phy *phy)
    {
    struct eq5_phy_inst *inst = phy_get_drvdata(phy);
    writel(readl(inst.gp) & ~EQ5_GP_SGMII_MODE, inst.gp);
    writel(0, inst.sgmii);
    return 0;
    }
    static int eq5_phy_validate(struct phy *phy, enum phy_mode mode, int submode,
    union phy_configure_opts *opts)
    {
    struct eq5_phy_inst *inst = phy_get_drvdata(phy);
    if (mode != PHY_MODE_ETHERNET)
    return -EINVAL;
    if (phy_interface_mode_is_rgmii(submode))
    return 0;
    if (inst.sgmii_support && submode == PHY_INTERFACE_MODE_SGMII)
    return 0;
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn eq5_phy_set_mode(phy: *mut phy, mode: enum phy_mode, submode: c_int) -> c_int {
    static int eq5_phy_set_mode(struct phy *phy, enum phy_mode mode, int submode)
    {
    struct eq5_phy_inst *inst = phy_get_drvdata(phy);
    enum eq5_phy_submode target_submode;
    int ret;
    ret = eq5_phy_validate(phy, mode, submode, core::ptr::null_mut());
    if (ret)
    return ret;
    if (submode == PHY_INTERFACE_MODE_SGMII)
    target_submode = EQ5_PHY_SUBMODE_SGMII;
    else
    target_submode = EQ5_PHY_SUBMODE_RGMII;
    if (target_submode == inst.submode)
    return 0;
    inst.submode = target_submode;
    if (phy.power_count) {
    eq5_phy_init(phy);
    return eq5_phy_power_on(phy);
    }
    return 0;
    }
    static const struct phy_ops eq5_phy_ops = {
    .init		= eq5_phy_init,
    .exit		= eq5_phy_exit,
    .power_on	= eq5_phy_power_on,
    .power_off	= eq5_phy_power_off,
    .set_mode	= eq5_phy_set_mode,
    .validate	= eq5_phy_validate,
    };
    static struct phy *eq5_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct eq5_phy_private *priv = dev_get_drvdata(dev);
    if (args.args_count != 1 || args.args[0] >= EQ5_PHY_COUNT)
    return ERR_PTR(-EINVAL);
    return priv.phys[args.args[0]].phy;
    }
    static int eq5_phy_probe_phy(struct device *dev, struct eq5_phy_private *priv,
    unsigned int index, void __iomem *base,
    unsigned int gp, unsigned int sgmii,
    bool sgmii_support)
    {
    struct eq5_phy_inst *inst = &priv.phys[index];
    struct phy *phy;
    phy = devm_phy_create(dev, dev.of_node, &eq5_phy_ops);
    if (IS_ERR(phy))
    return dev_err_probe(dev, PTR_ERR(phy),
    "failed to create PHY %u\n", index);
    inst.dev = dev;
    inst.phy = phy;
    inst.gp = base + gp;
    inst.sgmii = base + sgmii;
    inst.sgmii_support = sgmii_support;
    phy_set_drvdata(phy, inst);
//
// Init inst->submode based on probe hardware state, allowing
// consumers to power us on without first setting the mode.
//
    if (sgmii_support && (readl(inst.gp) & EQ5_GP_SGMII_MODE))
    inst.submode = EQ5_PHY_SUBMODE_SGMII;
    else
    inst.submode = EQ5_PHY_SUBMODE_RGMII;
    return 0;
    }
    static int eq5_phy_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct device *dev = &adev.dev;
    struct phy_provider *provider;
    struct eq5_phy_private *priv;
    void __iomem *base;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dev_set_drvdata(dev, priv);
    base = (void __iomem *)dev_get_platdata(dev);
    ret = eq5_phy_probe_phy(dev, priv, 0, base, EQ5_PHY0_GP,
    EQ5_PHY0_SGMII, true);
    if (ret)
    return ret;
    ret = eq5_phy_probe_phy(dev, priv, 1, base, EQ5_PHY1_GP,
    EQ5_PHY1_SGMII, false);
    if (ret)
    return ret;
    provider = devm_of_phy_provider_register(dev, eq5_phy_xlate);
    if (IS_ERR(provider))
    return dev_err_probe(dev, PTR_ERR(provider),
    "registering provider failed\n");
    return 0;
    }
    static const struct auxiliary_device_id eq5_phy_id_table[] = {
    { .name = "clk_eyeq.phy" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, eq5_phy_id_table);
    static struct auxiliary_driver eq5_phy_driver = {
    .probe = eq5_phy_probe,
    .id_table = eq5_phy_id_table,
    };
    module_auxiliary_driver(eq5_phy_driver);
    MODULE_DESCRIPTION("EyeQ5 Ethernet PHY driver");
    MODULE_AUTHOR("Théo Lebrun <theo.lebrun@bootlin.com>");
    MODULE_LICENSE("GPL");
