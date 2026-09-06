//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/tegra/pinctrl-tegra-xusb.c
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
// Copyright (c) 2014, NVIDIA CORPORATION.  All rights reserved.
//

pub const XUSB_PADCTL_ELPG_PROGRAM: c_uint = 0x01c;

pub const XUSB_PADCTL_IOPHY_PLL_P0_CTL1: c_uint = 0x040;

pub const XUSB_PADCTL_IOPHY_PLL_P0_CTL2: c_uint = 0x044;

pub const XUSB_PADCTL_IOPHY_PLL_S0_CTL1: c_uint = 0x138;

pub const XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1: c_uint = 0x148;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_padctl_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub num_groups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_padctl_soc {
    pub pins: *const pinctrl_pin_desc,
    pub num_pins: c_uint,
    pub functions: *const tegra_xusb_padctl_function,
    pub num_functions: c_uint,
    pub lanes: *const tegra_xusb_padctl_lane,
    pub num_lanes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_padctl_lane {
    pub name: *const c_char,
    pub offset: c_uint,
    pub shift: c_uint,
    pub mask: c_uint,
    pub iddq: c_uint,
    pub funcs: *const c_uint,
    pub num_funcs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_padctl {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub lock: mutex,
    pub rst: *mut reset_control,
    pub soc: *const tegra_xusb_padctl_soc,
    pub pinctrl: *mut pinctrl_dev,
    pub desc: pinctrl_desc,
    pub provider: *mut phy_provider,
    pub phys: [*mut phy; 2],
    pub enable: c_uint,
}

    static inline void padctl_writel(struct tegra_xusb_padctl *padctl, u32 value,
    unsigned long offset)
    {
    writel(value, padctl.regs + offset);
    }
    static inline u32 padctl_readl(struct tegra_xusb_padctl *padctl,
    unsigned long offset)
    {
    return readl(padctl.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn tegra_xusb_padctl_get_groups_count(pinctrl: *mut pinctrl_dev) -> c_int {
    static int tegra_xusb_padctl_get_groups_count(struct pinctrl_dev *pinctrl)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    return padctl.soc.num_pins;
    }
    static const char *tegra_xusb_padctl_get_group_name(struct pinctrl_dev *pinctrl,
    unsigned int group)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    return padctl.soc.pins[group].name;
    }
    static int tegra_xusb_padctl_get_group_pins(struct pinctrl_dev *pinctrl,
    unsigned group,
    const unsigned **pins,
    unsigned *num_pins)
    {
//
// For the tegra-xusb pad controller groups are synonymous
// with lanes/pins and there is always one lane/pin per group.
//
// pins = &pinctrl->desc->pins[group].number;
// num_pins = 1;
    return 0;
    }
    enum tegra_xusb_padctl_param {
    TEGRA_XUSB_PADCTL_IDDQ,
    };
    static const struct tegra_xusb_padctl_property {
    const char *name;
    enum tegra_xusb_padctl_param param;
    } properties[] = {
    { "nvidia,iddq", TEGRA_XUSB_PADCTL_IDDQ },
    };

    static int tegra_xusb_padctl_parse_subnode(struct tegra_xusb_padctl *padctl,
    struct device_node *np,
    struct pinctrl_map **maps,
    unsigned int *reserved_maps,
    unsigned int *num_maps)
    {
    unsigned int i, reserve = 0, num_configs = 0;
    unsigned long config, *configs = core::ptr::null_mut();
    const char *function, *group;
    struct property *prop;
    let mut err: c_int = 0;
    u32 value;
    err = of_property_read_string(np, "nvidia,function", &function);
    if (err < 0) {
    if (err != -EINVAL)
    return err;
    function = core::ptr::null_mut();
    }
    for (i = 0; i < ARRAY_SIZE(properties); i++) {
    err = of_property_read_u32(np, properties[i].name, &value);
    if (err < 0) {
    if (err == -EINVAL)
    continue;
    goto out;
    }
    config = TEGRA_XUSB_PADCTL_PACK(properties[i].param, value);
    err = pinctrl_utils_add_config(padctl.pinctrl, &configs,
    &num_configs, config);
    if (err < 0)
    goto out;
    }
    if (function)
    reserve++;
    if (num_configs)
    reserve++;
    err = of_property_count_strings(np, "nvidia,lanes");
    if (err < 0)
    goto out;
    reserve *= err;
    err = pinctrl_utils_reserve_map(padctl.pinctrl, maps, reserved_maps,
    num_maps, reserve);
    if (err < 0)
    goto out;
    of_property_for_each_string(np, "nvidia,lanes", prop, group) {
    if (function) {
    err = pinctrl_utils_add_map_mux(padctl.pinctrl, maps,
    reserved_maps, num_maps, group,
    function);
    if (err < 0)
    goto out;
    }
    if (num_configs) {
    err = pinctrl_utils_add_map_configs(padctl.pinctrl,
    maps, reserved_maps, num_maps, group,
    configs, num_configs,
    PIN_MAP_TYPE_CONFIGS_GROUP);
    if (err < 0)
    goto out;
    }
    }
    err = 0;
    out:
    kfree(configs);
    return err;
    }
    static int tegra_xusb_padctl_dt_node_to_map(struct pinctrl_dev *pinctrl,
    struct device_node *parent,
    struct pinctrl_map **maps,
    unsigned int *num_maps)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    let mut reserved_maps: c_uint = 0;
    int err;
// num_maps = 0;
// maps = NULL;
    for_each_child_of_node_scoped(parent, np) {
    err = tegra_xusb_padctl_parse_subnode(padctl, np, maps,
    &reserved_maps,
    num_maps);
    if (err < 0)
    return err;
    }
    return 0;
    }
    static const struct pinctrl_ops tegra_xusb_padctl_pinctrl_ops = {
    .get_groups_count = tegra_xusb_padctl_get_groups_count,
    .get_group_name = tegra_xusb_padctl_get_group_name,
    .get_group_pins = tegra_xusb_padctl_get_group_pins,
    .dt_node_to_map = tegra_xusb_padctl_dt_node_to_map,
    .dt_free_map = pinctrl_utils_free_map,
    };
#[no_mangle]
unsafe extern "C" fn tegra_xusb_padctl_get_functions_count(pinctrl: *mut pinctrl_dev) -> c_int {
    static int tegra_xusb_padctl_get_functions_count(struct pinctrl_dev *pinctrl)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    return padctl.soc.num_functions;
    }
    static const char *
    tegra_xusb_padctl_get_function_name(struct pinctrl_dev *pinctrl,
    unsigned int function)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    return padctl.soc.functions[function].name;
    }
    static int tegra_xusb_padctl_get_function_groups(struct pinctrl_dev *pinctrl,
    unsigned int function,
    const char * const **groups,
    unsigned * const num_groups)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
// num_groups = padctl->soc->functions[function].num_groups;
// groups = padctl->soc->functions[function].groups;
    return 0;
    }
    static int tegra_xusb_padctl_pinmux_set(struct pinctrl_dev *pinctrl,
    unsigned int function,
    unsigned int group)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    const struct tegra_xusb_padctl_lane *lane;
    unsigned int i;
    u32 value;
    lane = &padctl.soc.lanes[group];
    for (i = 0; i < lane.num_funcs; i++)
    if (lane.funcs[i] == function)
    break;
    if (i >= lane.num_funcs)
    return -EINVAL;
    value = padctl_readl(padctl, lane.offset);
    value &= ~(lane.mask << lane.shift);
    value |= i << lane.shift;
    padctl_writel(padctl, value, lane.offset);
    return 0;
    }
    static const struct pinmux_ops tegra_xusb_padctl_pinmux_ops = {
    .get_functions_count = tegra_xusb_padctl_get_functions_count,
    .get_function_name = tegra_xusb_padctl_get_function_name,
    .get_function_groups = tegra_xusb_padctl_get_function_groups,
    .set_mux = tegra_xusb_padctl_pinmux_set,
    };
    static int tegra_xusb_padctl_pinconf_group_get(struct pinctrl_dev *pinctrl,
    unsigned int group,
    unsigned long *config)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    const struct tegra_xusb_padctl_lane *lane;
    enum tegra_xusb_padctl_param param;
    u32 value;
    param = TEGRA_XUSB_PADCTL_UNPACK_PARAM(*config);
    lane = &padctl.soc.lanes[group];
    switch (param) {
    case TEGRA_XUSB_PADCTL_IDDQ:
// lanes with iddq == 0 don't support this parameter
    if (lane.iddq == 0)
    return -EINVAL;
    value = padctl_readl(padctl, lane.offset);
    if (value & BIT(lane.iddq))
    value = 0;
    else
    value = 1;
// config = TEGRA_XUSB_PADCTL_PACK(param, value);
    break;
    default:
    dev_err(padctl.dev, "invalid configuration parameter: %04x\n",
    param);
    return -ENOTSUPP;
    }
    return 0;
    }
    static int tegra_xusb_padctl_pinconf_group_set(struct pinctrl_dev *pinctrl,
    unsigned int group,
    unsigned long *configs,
    unsigned int num_configs)
    {
    struct tegra_xusb_padctl *padctl = pinctrl_dev_get_drvdata(pinctrl);
    const struct tegra_xusb_padctl_lane *lane;
    enum tegra_xusb_padctl_param param;
    unsigned long value;
    unsigned int i;
    u32 regval;
    lane = &padctl.soc.lanes[group];
    for (i = 0; i < num_configs; i++) {
    param = TEGRA_XUSB_PADCTL_UNPACK_PARAM(configs[i]);
    value = TEGRA_XUSB_PADCTL_UNPACK_VALUE(configs[i]);
    switch (param) {
    case TEGRA_XUSB_PADCTL_IDDQ:
// lanes with iddq == 0 don't support this parameter
    if (lane.iddq == 0)
    return -EINVAL;
    regval = padctl_readl(padctl, lane.offset);
    if (value)
    regval &= ~BIT(lane.iddq);
    else
    regval |= BIT(lane.iddq);
    padctl_writel(padctl, regval, lane.offset);
    break;
    default:
    dev_err(padctl.dev,
    "invalid configuration parameter: %04x\n",
    param);
    return -ENOTSUPP;
    }
    }
    return 0;
    }

    static const char *strip_prefix(const char *s)
    {
    const char *comma = strchr(s, ',');
    if (!comma)
    return s;
    return comma + 1;
    }
    static void
    tegra_xusb_padctl_pinconf_group_dbg_show(struct pinctrl_dev *pinctrl,
    struct seq_file *s,
    unsigned int group)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(properties); i++) {
    unsigned long config, value;
    int err;
    config = TEGRA_XUSB_PADCTL_PACK(properties[i].param, 0);
    err = tegra_xusb_padctl_pinconf_group_get(pinctrl, group,
    &config);
    if (err < 0)
    continue;
    value = TEGRA_XUSB_PADCTL_UNPACK_VALUE(config);
    seq_printf(s, "\n\t%s=%lu\n", strip_prefix(properties[i].name),
    value);
    }
    }
    static void
    tegra_xusb_padctl_pinconf_config_dbg_show(struct pinctrl_dev *pinctrl,
    struct seq_file *s,
    unsigned long config)
    {
    enum tegra_xusb_padctl_param param;
    const char *name = "unknown";
    unsigned long value;
    unsigned int i;
    param = TEGRA_XUSB_PADCTL_UNPACK_PARAM(config);
    value = TEGRA_XUSB_PADCTL_UNPACK_VALUE(config);
    for (i = 0; i < ARRAY_SIZE(properties); i++) {
    if (properties[i].param == param) {
    name = properties[i].name;
    break;
    }
    }
    seq_printf(s, "%s=%lu", strip_prefix(name), value);
    }

    static const struct pinconf_ops tegra_xusb_padctl_pinconf_ops = {
    .pin_config_group_get = tegra_xusb_padctl_pinconf_group_get,
    .pin_config_group_set = tegra_xusb_padctl_pinconf_group_set,

    .pin_config_group_dbg_show = tegra_xusb_padctl_pinconf_group_dbg_show,
    .pin_config_config_dbg_show = tegra_xusb_padctl_pinconf_config_dbg_show,

    };
#[no_mangle]
unsafe extern "C" fn tegra_xusb_padctl_enable(padctl: *mut tegra_xusb_padctl) {
    static void tegra_xusb_padctl_enable(struct tegra_xusb_padctl *padctl)
    {
    u32 value;
    guard(mutex)(&padctl.lock);
    if (padctl.enable++ > 0)
    return;
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~XUSB_PADCTL_ELPG_PROGRAM_AUX_MUX_LP0_CLAMP_EN;
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~XUSB_PADCTL_ELPG_PROGRAM_AUX_MUX_LP0_CLAMP_EN_EARLY;
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~XUSB_PADCTL_ELPG_PROGRAM_AUX_MUX_LP0_VCORE_DOWN;
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    }
#[no_mangle]
unsafe extern "C" fn tegra_xusb_padctl_disable(padctl: *mut tegra_xusb_padctl) {
    static void tegra_xusb_padctl_disable(struct tegra_xusb_padctl *padctl)
    {
    u32 value;
    guard(mutex)(&padctl.lock);
    if (WARN_ON(padctl.enable == 0))
    return;
    if (--padctl.enable > 0)
    return;
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value |= XUSB_PADCTL_ELPG_PROGRAM_AUX_MUX_LP0_VCORE_DOWN;
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value |= XUSB_PADCTL_ELPG_PROGRAM_AUX_MUX_LP0_CLAMP_EN_EARLY;
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value |= XUSB_PADCTL_ELPG_PROGRAM_AUX_MUX_LP0_CLAMP_EN;
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    }
#[no_mangle]
unsafe extern "C" fn tegra_xusb_phy_init(phy: *mut phy) -> c_int {
    static int tegra_xusb_phy_init(struct phy *phy)
    {
    struct tegra_xusb_padctl *padctl = phy_get_drvdata(phy);
    tegra_xusb_padctl_enable(padctl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_xusb_phy_exit(phy: *mut phy) -> c_int {
    static int tegra_xusb_phy_exit(struct phy *phy)
    {
    struct tegra_xusb_padctl *padctl = phy_get_drvdata(phy);
    tegra_xusb_padctl_disable(padctl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcie_phy_power_on(phy: *mut phy) -> c_int {
    static int pcie_phy_power_on(struct phy *phy)
    {
    struct tegra_xusb_padctl *padctl = phy_get_drvdata(phy);
    unsigned long timeout;
    let mut err: c_int = -ETIMEDOUT;
    u32 value;
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_P0_CTL1);
    value &= ~XUSB_PADCTL_IOPHY_PLL_P0_CTL1_REFCLK_SEL_MASK;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_P0_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_P0_CTL2);
    value |= XUSB_PADCTL_IOPHY_PLL_P0_CTL2_REFCLKBUF_EN |
    XUSB_PADCTL_IOPHY_PLL_P0_CTL2_TXCLKREF_EN |
    XUSB_PADCTL_IOPHY_PLL_P0_CTL2_TXCLKREF_SEL;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_P0_CTL2);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_P0_CTL1);
    value |= XUSB_PADCTL_IOPHY_PLL_P0_CTL1_PLL_RST;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_P0_CTL1);
    timeout = jiffies + msecs_to_jiffies(50);
    while (time_before(jiffies, timeout)) {
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_P0_CTL1);
    if (value & XUSB_PADCTL_IOPHY_PLL_P0_CTL1_PLL0_LOCKDET) {
    err = 0;
    break;
    }
    usleep_range(100, 200);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcie_phy_power_off(phy: *mut phy) -> c_int {
    static int pcie_phy_power_off(struct phy *phy)
    {
    struct tegra_xusb_padctl *padctl = phy_get_drvdata(phy);
    u32 value;
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_P0_CTL1);
    value &= ~XUSB_PADCTL_IOPHY_PLL_P0_CTL1_PLL_RST;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_P0_CTL1);
    return 0;
    }
    static const struct phy_ops pcie_phy_ops = {
    .init = tegra_xusb_phy_init,
    .exit = tegra_xusb_phy_exit,
    .power_on = pcie_phy_power_on,
    .power_off = pcie_phy_power_off,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn sata_phy_power_on(phy: *mut phy) -> c_int {
    static int sata_phy_power_on(struct phy *phy)
    {
    struct tegra_xusb_padctl *padctl = phy_get_drvdata(phy);
    unsigned long timeout;
    let mut err: c_int = -ETIMEDOUT;
    u32 value;
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1);
    value &= ~XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1_IDDQ_OVRD;
    value &= ~XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1_IDDQ;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value &= ~XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL_PWR_OVRD;
    value &= ~XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL_IDDQ;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value |= XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL1_MODE;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value |= XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL_RST;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    timeout = jiffies + msecs_to_jiffies(50);
    while (time_before(jiffies, timeout)) {
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    if (value & XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL1_LOCKDET) {
    err = 0;
    break;
    }
    usleep_range(100, 200);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sata_phy_power_off(phy: *mut phy) -> c_int {
    static int sata_phy_power_off(struct phy *phy)
    {
    struct tegra_xusb_padctl *padctl = phy_get_drvdata(phy);
    u32 value;
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value &= ~XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL_RST;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value &= ~XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL1_MODE;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value |= XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL_PWR_OVRD;
    value |= XUSB_PADCTL_IOPHY_PLL_S0_CTL1_PLL_IDDQ;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_PLL_S0_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1);
    value |= ~XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1_IDDQ_OVRD;
    value |= ~XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1_IDDQ;
    padctl_writel(padctl, value, XUSB_PADCTL_IOPHY_MISC_PAD_S0_CTL1);
    return 0;
    }
    static const struct phy_ops sata_phy_ops = {
    .init = tegra_xusb_phy_init,
    .exit = tegra_xusb_phy_exit,
    .power_on = sata_phy_power_on,
    .power_off = sata_phy_power_off,
    .owner = THIS_MODULE,
    };
    static struct phy *tegra_xusb_padctl_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct tegra_xusb_padctl *padctl = dev_get_drvdata(dev);
    let mut index: c_uint = args.args[0];
    if (args.args_count <= 0)
    return ERR_PTR(-EINVAL);
    if (index >= ARRAY_SIZE(padctl.phys))
    return ERR_PTR(-EINVAL);
    return padctl.phys[index];
    }
pub const PIN_OTG_0: c_int = 0;
pub const PIN_OTG_1: c_int = 1;
pub const PIN_OTG_2: c_int = 2;
pub const PIN_ULPI_0: c_int = 3;
pub const PIN_HSIC_0: c_int = 4;
pub const PIN_HSIC_1: c_int = 5;
pub const PIN_PCIE_0: c_int = 6;
pub const PIN_PCIE_1: c_int = 7;
pub const PIN_PCIE_2: c_int = 8;
pub const PIN_PCIE_3: c_int = 9;
pub const PIN_PCIE_4: c_int = 10;
pub const PIN_SATA_0: c_int = 11;
    static const struct pinctrl_pin_desc tegra124_pins[] = {
    PINCTRL_PIN(PIN_OTG_0,  "otg-0"),
    PINCTRL_PIN(PIN_OTG_1,  "otg-1"),
    PINCTRL_PIN(PIN_OTG_2,  "otg-2"),
    PINCTRL_PIN(PIN_ULPI_0, "ulpi-0"),
    PINCTRL_PIN(PIN_HSIC_0, "hsic-0"),
    PINCTRL_PIN(PIN_HSIC_1, "hsic-1"),
    PINCTRL_PIN(PIN_PCIE_0, "pcie-0"),
    PINCTRL_PIN(PIN_PCIE_1, "pcie-1"),
    PINCTRL_PIN(PIN_PCIE_2, "pcie-2"),
    PINCTRL_PIN(PIN_PCIE_3, "pcie-3"),
    PINCTRL_PIN(PIN_PCIE_4, "pcie-4"),
    PINCTRL_PIN(PIN_SATA_0, "sata-0"),
    };
    static const char * const tegra124_snps_groups[] = {
    "otg-0",
    "otg-1",
    "otg-2",
    "ulpi-0",
    "hsic-0",
    "hsic-1",
    };
    static const char * const tegra124_xusb_groups[] = {
    "otg-0",
    "otg-1",
    "otg-2",
    "ulpi-0",
    "hsic-0",
    "hsic-1",
    };
    static const char * const tegra124_uart_groups[] = {
    "otg-0",
    "otg-1",
    "otg-2",
    };
    static const char * const tegra124_pcie_groups[] = {
    "pcie-0",
    "pcie-1",
    "pcie-2",
    "pcie-3",
    "pcie-4",
    };
    static const char * const tegra124_usb3_groups[] = {
    "pcie-0",
    "pcie-1",
    "sata-0",
    };
    static const char * const tegra124_sata_groups[] = {
    "sata-0",
    };
    static const char * const tegra124_rsvd_groups[] = {
    "otg-0",
    "otg-1",
    "otg-2",
    "pcie-0",
    "pcie-1",
    "pcie-2",
    "pcie-3",
    "pcie-4",
    "sata-0",
    };

    {								\
    .name = #_name,						\
    .num_groups = ARRAY_SIZE(tegra124_##_name##_groups),	\
    .groups = tegra124_##_name##_groups,			\
    }
    static struct tegra_xusb_padctl_function tegra124_functions[] = {
    TEGRA124_FUNCTION(snps),
    TEGRA124_FUNCTION(xusb),
    TEGRA124_FUNCTION(uart),
    TEGRA124_FUNCTION(pcie),
    TEGRA124_FUNCTION(usb3),
    TEGRA124_FUNCTION(sata),
    TEGRA124_FUNCTION(rsvd),
    };
    enum tegra124_function {
    TEGRA124_FUNC_SNPS,
    TEGRA124_FUNC_XUSB,
    TEGRA124_FUNC_UART,
    TEGRA124_FUNC_PCIE,
    TEGRA124_FUNC_USB3,
    TEGRA124_FUNC_SATA,
    TEGRA124_FUNC_RSVD,
    };
    static const unsigned int tegra124_otg_functions[] = {
    TEGRA124_FUNC_SNPS,
    TEGRA124_FUNC_XUSB,
    TEGRA124_FUNC_UART,
    TEGRA124_FUNC_RSVD,
    };
    static const unsigned int tegra124_usb_functions[] = {
    TEGRA124_FUNC_SNPS,
    TEGRA124_FUNC_XUSB,
    };
    static const unsigned int tegra124_pci_functions[] = {
    TEGRA124_FUNC_PCIE,
    TEGRA124_FUNC_USB3,
    TEGRA124_FUNC_SATA,
    TEGRA124_FUNC_RSVD,
    };

    {								\
    .name = _name,						\
    .offset = _offset,					\
    .shift = _shift,					\
    .mask = _mask,						\
    .iddq = _iddq,						\
    .num_funcs = ARRAY_SIZE(tegra124_##_funcs##_functions),	\
    .funcs = tegra124_##_funcs##_functions,			\
    }
    static const struct tegra_xusb_padctl_lane tegra124_lanes[] = {
    TEGRA124_LANE("otg-0",  0x004,  0, 0x3, 0, otg),
    TEGRA124_LANE("otg-1",  0x004,  2, 0x3, 0, otg),
    TEGRA124_LANE("otg-2",  0x004,  4, 0x3, 0, otg),
    TEGRA124_LANE("ulpi-0", 0x004, 12, 0x1, 0, usb),
    TEGRA124_LANE("hsic-0", 0x004, 14, 0x1, 0, usb),
    TEGRA124_LANE("hsic-1", 0x004, 15, 0x1, 0, usb),
    TEGRA124_LANE("pcie-0", 0x134, 16, 0x3, 1, pci),
    TEGRA124_LANE("pcie-1", 0x134, 18, 0x3, 2, pci),
    TEGRA124_LANE("pcie-2", 0x134, 20, 0x3, 3, pci),
    TEGRA124_LANE("pcie-3", 0x134, 22, 0x3, 4, pci),
    TEGRA124_LANE("pcie-4", 0x134, 24, 0x3, 5, pci),
    TEGRA124_LANE("sata-0", 0x134, 26, 0x3, 6, pci),
    };
    static const struct tegra_xusb_padctl_soc tegra124_soc = {
    .num_pins = ARRAY_SIZE(tegra124_pins),
    .pins = tegra124_pins,
    .num_functions = ARRAY_SIZE(tegra124_functions),
    .functions = tegra124_functions,
    .num_lanes = ARRAY_SIZE(tegra124_lanes),
    .lanes = tegra124_lanes,
    };
    static const struct of_device_id tegra_xusb_padctl_of_match[] = {
    { .compatible = "nvidia,tegra124-xusb-padctl", .data = &tegra124_soc },
    { }
    };
    MODULE_DEVICE_TABLE(of, tegra_xusb_padctl_of_match);
// predeclare these in order to silence sparse
    int tegra_xusb_padctl_legacy_probe(struct platform_device *pdev);
    int tegra_xusb_padctl_legacy_remove(struct platform_device *pdev);
#[no_mangle]
pub unsafe extern "C" fn tegra_xusb_padctl_legacy_probe(pdev: *mut platform_device) -> c_int {
    int tegra_xusb_padctl_legacy_probe(struct platform_device *pdev)
    {
    struct tegra_xusb_padctl *padctl;
    const struct of_device_id *match;
    struct phy *phy;
    int err;
    padctl = devm_kzalloc(&pdev.dev, sizeof(*padctl), GFP_KERNEL);
    if (!padctl)
    return -ENOMEM;
    platform_set_drvdata(pdev, padctl);
    mutex_init(&padctl.lock);
    padctl.dev = &pdev.dev;
//
// Note that we can't replace this by of_device_get_match_data()
// because we need the separate matching table for this legacy code on
// Tegra124. of_device_get_match_data() would attempt to use the table
// from the updated driver and fail.
//
    match = of_match_node(tegra_xusb_padctl_of_match, pdev.dev.of_node);
    padctl.soc = match.data;
    padctl.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(padctl.regs))
    return PTR_ERR(padctl.regs);
    padctl.rst = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(padctl.rst))
    return PTR_ERR(padctl.rst);
    err = reset_control_deassert(padctl.rst);
    if (err < 0)
    return err;
    memset(&padctl.desc, 0, sizeof(padctl.desc));
    padctl.desc.name = dev_name(padctl.dev);
    padctl.desc.pins = tegra124_pins;
    padctl.desc.npins = ARRAY_SIZE(tegra124_pins);
    padctl.desc.pctlops = &tegra_xusb_padctl_pinctrl_ops;
    padctl.desc.pmxops = &tegra_xusb_padctl_pinmux_ops;
    padctl.desc.confops = &tegra_xusb_padctl_pinconf_ops;
    padctl.desc.owner = THIS_MODULE;
    padctl.pinctrl = devm_pinctrl_register(&pdev.dev, &padctl.desc,
    padctl);
    if (IS_ERR(padctl.pinctrl)) {
    dev_err(&pdev.dev, "failed to register pincontrol\n");
    err = PTR_ERR(padctl.pinctrl);
    goto reset;
    }
    phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &pcie_phy_ops);
    if (IS_ERR(phy)) {
    err = PTR_ERR(phy);
    goto reset;
    }
    padctl.phys[TEGRA_XUSB_PADCTL_PCIE] = phy;
    phy_set_drvdata(phy, padctl);
    phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &sata_phy_ops);
    if (IS_ERR(phy)) {
    err = PTR_ERR(phy);
    goto reset;
    }
    padctl.phys[TEGRA_XUSB_PADCTL_SATA] = phy;
    phy_set_drvdata(phy, padctl);
    padctl.provider = devm_of_phy_provider_register(&pdev.dev,
    tegra_xusb_padctl_xlate);
    if (IS_ERR(padctl.provider)) {
    err = PTR_ERR(padctl.provider);
    dev_err(&pdev.dev, "failed to register PHYs: %d\n", err);
    goto reset;
    }
    return 0;
    reset:
    reset_control_assert(padctl.rst);
    return err;
    }
    EXPORT_SYMBOL_GPL(tegra_xusb_padctl_legacy_probe);
#[no_mangle]
pub unsafe extern "C" fn tegra_xusb_padctl_legacy_remove(pdev: *mut platform_device) -> c_int {
    int tegra_xusb_padctl_legacy_remove(struct platform_device *pdev)
    {
    struct tegra_xusb_padctl *padctl = platform_get_drvdata(pdev);
    int err;
    err = reset_control_assert(padctl.rst);
    if (err < 0)
    dev_err(&pdev.dev, "failed to assert reset: %d\n", err);
    return err;
    }
    EXPORT_SYMBOL_GPL(tegra_xusb_padctl_legacy_remove);
