//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/freescale/pinctrl-imx1-core.c
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
// Core driver for the imx pin controller in imx1/21/27
//
// Copyright (C) 2013 Pengutronix
// Author: Markus Pargmann <mpa@pengutronix.de>
//
// Based on pinctrl-imx.c:
// Author: Dong Aisheng <dong.aisheng@linaro.org>
// Copyright (C) 2012 Freescale Semiconductor, Inc.
// Copyright (C) 2012 Linaro Ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx1_pinctrl {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
    pub base: *mut void __iomem,
    pub info: *const imx1_pinctrl_soc_info,
}

//
// MX1 register offsets
//
pub const MX1_DDIR: c_uint = 0x00;
pub const MX1_OCR: c_uint = 0x04;
pub const MX1_ICONFA: c_uint = 0x0c;
pub const MX1_ICONFB: c_uint = 0x14;
pub const MX1_GIUS: c_uint = 0x20;
pub const MX1_GPR: c_uint = 0x38;
pub const MX1_PUEN: c_uint = 0x40;
pub const MX1_PORT_STRIDE: c_uint = 0x100;
//
// MUX_ID format defines
//

//
// IMX1 IOMUXC manages the pins based on ports. Each port has 32 pins. IOMUX
// control registers are separated into function, output configuration, input
// configuration A, input configuration B, GPIO in use and data direction.
//
// Those controls that are represented by 1 bit have a direct mapping between
// bit position and pin id. If they are represented by 2 bit, the lower 16 pins
// are in the first register and the upper 16 pins in the second (next)
// register. pin_id is stored in bit (pin_id%16)*2 and the bit above.
//
// Calculates the register offset from a pin_id
//
    static void __iomem *imx1_mem(struct imx1_pinctrl *ipctl, unsigned int pin_id)
    {
    let mut port: c_uint = pin_id / 32;
    return ipctl.base + port * MX1_PORT_STRIDE;
    }
//
// Write to a register with 2 bits per pin. The function will automatically
// use the next register if the pin is managed in the second register.
//
    static void imx1_write_2bit(struct imx1_pinctrl *ipctl, unsigned int pin_id,
    u32 value, u32 reg_offset)
    {
    void __iomem *reg = imx1_mem(ipctl, pin_id) + reg_offset;
    int offset = (pin_id % 16) * 2; /* offset, regardless of register used */
    int mask = ~(0x3 << offset); /* Mask for 2 bits at offset */
    u32 old_val;
    u32 new_val;
// Use the next register if the pin's port pin number is >=16
    if (pin_id % 32 >= 16)
    reg += 0x04;
    dev_dbg(ipctl.dev, "write: register 0x%p offset %d value 0x%x\n",
    reg, offset, value);
// Get current state of pins
    old_val = readl(reg);
    old_val &= mask;
    new_val = value & 0x3; /* Make sure value is really 2 bit */
    new_val <<= offset;
    new_val |= old_val;/* Set new state for pin_id */
    writel(new_val, reg);
    }
    static void imx1_write_bit(struct imx1_pinctrl *ipctl, unsigned int pin_id,
    u32 value, u32 reg_offset)
    {
    void __iomem *reg = imx1_mem(ipctl, pin_id) + reg_offset;
    let mut offset: c_int = pin_id % 32;
    let mut mask: c_int = ~BIT_MASK(offset);
    u32 old_val;
    u32 new_val;
// Get current state of pins
    old_val = readl(reg);
    old_val &= mask;
    new_val = value & 0x1; /* Make sure value is really 1 bit */
    new_val <<= offset;
    new_val |= old_val;/* Set new state for pin_id */
    writel(new_val, reg);
    }
    static int imx1_read_2bit(struct imx1_pinctrl *ipctl, unsigned int pin_id,
    u32 reg_offset)
    {
    void __iomem *reg = imx1_mem(ipctl, pin_id) + reg_offset;
    let mut offset: c_int = (pin_id % 16) * 2;
// Use the next register if the pin's port pin number is >=16
    if (pin_id % 32 >= 16)
    reg += 0x04;
    return (readl(reg) & (BIT(offset) | BIT(offset+1))) >> offset;
    }
    static int imx1_read_bit(struct imx1_pinctrl *ipctl, unsigned int pin_id,
    u32 reg_offset)
    {
    void __iomem *reg = imx1_mem(ipctl, pin_id) + reg_offset;
    let mut offset: c_int = pin_id % 32;
    return !!(readl(reg) & BIT(offset));
    }
    static inline const struct imx1_pin_group *imx1_pinctrl_find_group_by_name(
    const struct imx1_pinctrl_soc_info *info,
    const char *name)
    {
    const struct imx1_pin_group *grp = core::ptr::null_mut();
    int i;
    for (i = 0; i < info.ngroups; i++) {
    if (!strcmp(info.groups[i].name, name)) {
    grp = &info.groups[i];
    break;
    }
    }
    return grp;
    }
#[no_mangle]
unsafe extern "C" fn imx1_get_groups_count(pctldev: *mut pinctrl_dev) -> c_int {
    static int imx1_get_groups_count(struct pinctrl_dev *pctldev)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    return info.ngroups;
    }
    static const char *imx1_get_group_name(struct pinctrl_dev *pctldev,
    unsigned selector)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    return info.groups[selector].name;
    }
    static int imx1_get_group_pins(struct pinctrl_dev *pctldev, unsigned selector,
    const unsigned int **pins,
    unsigned *npins)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    if (selector >= info.ngroups)
    return -EINVAL;
// pins = info->groups[selector].pin_ids;
// npins = info->groups[selector].npins;
    return 0;
    }
    static void imx1_pin_dbg_show(struct pinctrl_dev *pctldev, struct seq_file *s,
    unsigned offset)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    seq_printf(s, "GPIO %d, function %d, direction %d, oconf %d, iconfa %d, iconfb %d",
    imx1_read_bit(ipctl, offset, MX1_GIUS),
    imx1_read_bit(ipctl, offset, MX1_GPR),
    imx1_read_bit(ipctl, offset, MX1_DDIR),
    imx1_read_2bit(ipctl, offset, MX1_OCR),
    imx1_read_2bit(ipctl, offset, MX1_ICONFA),
    imx1_read_2bit(ipctl, offset, MX1_ICONFB));
    }
    static int imx1_dt_node_to_map(struct pinctrl_dev *pctldev,
    struct device_node *np,
    struct pinctrl_map **map, unsigned *num_maps)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    const struct imx1_pin_group *grp;
    struct pinctrl_map *new_map;
    struct device_node *parent;
    let mut map_num: c_int = 1;
    int i, j;
//
// first find the group of this node and check if we need create
// config maps for pins
//
    grp = imx1_pinctrl_find_group_by_name(info, np.name);
    if (!grp) {
    dev_err(info.dev, "unable to find group for node %pOFn\n",
    np);
    return -EINVAL;
    }
    for (i = 0; i < grp.npins; i++)
    map_num++;
    new_map = kmalloc_objs(struct pinctrl_map, map_num);
    if (!new_map)
    return -ENOMEM;
// map = new_map;
// num_maps = map_num;
// create mux map
    parent = of_get_parent(np);
    if (!parent) {
    kfree(new_map);
    return -EINVAL;
    }
    new_map[0].type = PIN_MAP_TYPE_MUX_GROUP;
    new_map[0].data.mux.function = parent.name;
    new_map[0].data.mux.group = np.name;
    of_node_put(parent);
// create config map
    new_map++;
    for (i = j = 0; i < grp.npins; i++) {
    new_map[j].type = PIN_MAP_TYPE_CONFIGS_PIN;
    new_map[j].data.configs.group_or_pin =
    pin_get_name(pctldev, grp.pins[i].pin_id);
    new_map[j].data.configs.configs = &grp.pins[i].config;
    new_map[j].data.configs.num_configs = 1;
    j++;
    }
    dev_dbg(pctldev.dev, "maps: function %s group %s num %d\n",
    (*map).data.mux.function, (*map).data.mux.group, map_num);
    return 0;
    }
    static void imx1_dt_free_map(struct pinctrl_dev *pctldev,
    struct pinctrl_map *map, unsigned num_maps)
    {
    kfree(map);
    }
    static const struct pinctrl_ops imx1_pctrl_ops = {
    .get_groups_count = imx1_get_groups_count,
    .get_group_name = imx1_get_group_name,
    .get_group_pins = imx1_get_group_pins,
    .pin_dbg_show = imx1_pin_dbg_show,
    .dt_node_to_map = imx1_dt_node_to_map,
    .dt_free_map = imx1_dt_free_map,
    };
    static int imx1_pmx_set(struct pinctrl_dev *pctldev, unsigned selector,
    unsigned group)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    const struct imx1_pin *pins;
    unsigned int npins;
    int i;
//
// Configure the mux mode for each pin in the group for a specific
// function.
//
    pins = info.groups[group].pins;
    npins = info.groups[group].npins;
    WARN_ON(!pins || !npins);
    dev_dbg(ipctl.dev, "enable function %s group %s\n",
    info.functions[selector].name, info.groups[group].name);
    for (i = 0; i < npins; i++) {
    let mut mux: c_uint = pins[i].mux_id;
    let mut pin_id: c_uint = pins[i].pin_id;
    let mut afunction: c_uint = MX1_MUX_FUNCTION(mux);
    let mut gpio_in_use: c_uint = MX1_MUX_GPIO(mux);
    let mut direction: c_uint = MX1_MUX_DIR(mux);
    let mut gpio_oconf: c_uint = MX1_MUX_OCONF(mux);
    let mut gpio_iconfa: c_uint = MX1_MUX_ICONFA(mux);
    let mut gpio_iconfb: c_uint = MX1_MUX_ICONFB(mux);
    dev_dbg(pctldev.dev, "%s, pin 0x%x, function %d, gpio %d, direction %d, oconf %d, iconfa %d, iconfb %d\n",
    __func__, pin_id, afunction, gpio_in_use,
    direction, gpio_oconf, gpio_iconfa,
    gpio_iconfb);
    imx1_write_bit(ipctl, pin_id, gpio_in_use, MX1_GIUS);
    imx1_write_bit(ipctl, pin_id, direction, MX1_DDIR);
    if (gpio_in_use) {
    imx1_write_2bit(ipctl, pin_id, gpio_oconf, MX1_OCR);
    imx1_write_2bit(ipctl, pin_id, gpio_iconfa,
    MX1_ICONFA);
    imx1_write_2bit(ipctl, pin_id, gpio_iconfb,
    MX1_ICONFB);
    } else {
    imx1_write_bit(ipctl, pin_id, afunction, MX1_GPR);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx1_pmx_get_funcs_count(pctldev: *mut pinctrl_dev) -> c_int {
    static int imx1_pmx_get_funcs_count(struct pinctrl_dev *pctldev)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    return info.nfunctions;
    }
    static const char *imx1_pmx_get_func_name(struct pinctrl_dev *pctldev,
    unsigned selector)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    return info.functions[selector].name;
    }
    static int imx1_pmx_get_groups(struct pinctrl_dev *pctldev, unsigned selector,
    const char * const **groups,
    unsigned * const num_groups)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
// groups = info->functions[selector].groups;
// num_groups = info->functions[selector].num_groups;
    return 0;
    }
    static const struct pinmux_ops imx1_pmx_ops = {
    .get_functions_count = imx1_pmx_get_funcs_count,
    .get_function_name = imx1_pmx_get_func_name,
    .get_function_groups = imx1_pmx_get_groups,
    .set_mux = imx1_pmx_set,
    };
    static int imx1_pinconf_get(struct pinctrl_dev *pctldev,
    unsigned pin_id, unsigned long *config)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
// config = imx1_read_bit(ipctl, pin_id, MX1_PUEN);
    return 0;
    }
    static int imx1_pinconf_set(struct pinctrl_dev *pctldev,
    unsigned pin_id, unsigned long *configs,
    unsigned num_configs)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    int i;
    for (i = 0; i != num_configs; ++i) {
    imx1_write_bit(ipctl, pin_id, configs[i] & 0x01, MX1_PUEN);
    dev_dbg(ipctl.dev, "pinconf set pullup pin %s\n",
    pin_desc_get(pctldev, pin_id).name);
    }
    return 0;
    }
    static void imx1_pinconf_dbg_show(struct pinctrl_dev *pctldev,
    struct seq_file *s, unsigned pin_id)
    {
    unsigned long config;
    imx1_pinconf_get(pctldev, pin_id, &config);
    seq_printf(s, "0x%lx", config);
    }
    static void imx1_pinconf_group_dbg_show(struct pinctrl_dev *pctldev,
    struct seq_file *s, unsigned group)
    {
    struct imx1_pinctrl *ipctl = pinctrl_dev_get_drvdata(pctldev);
    const struct imx1_pinctrl_soc_info *info = ipctl.info;
    struct imx1_pin_group *grp;
    unsigned long config;
    const char *name;
    int i, ret;
    if (group >= info.ngroups)
    return;
    seq_puts(s, "\n");
    grp = &info.groups[group];
    for (i = 0; i < grp.npins; i++) {
    name = pin_get_name(pctldev, grp.pins[i].pin_id);
    ret = imx1_pinconf_get(pctldev, grp.pins[i].pin_id, &config);
    if (ret)
    return;
    seq_printf(s, "%s: 0x%lx", name, config);
    }
    }
    static const struct pinconf_ops imx1_pinconf_ops = {
    .pin_config_get = imx1_pinconf_get,
    .pin_config_set = imx1_pinconf_set,
    .pin_config_dbg_show = imx1_pinconf_dbg_show,
    .pin_config_group_dbg_show = imx1_pinconf_group_dbg_show,
    };
    static struct pinctrl_desc imx1_pinctrl_desc = {
    .pctlops = &imx1_pctrl_ops,
    .pmxops = &imx1_pmx_ops,
    .confops = &imx1_pinconf_ops,
    .owner = THIS_MODULE,
    };
    static int imx1_pinctrl_parse_groups(struct device_node *np,
    struct imx1_pin_group *grp,
    struct imx1_pinctrl_soc_info *info,
    u32 index)
    {
    int size;
    const __be32 *list;
    int i;
    dev_dbg(info.dev, "group(%d): %pOFn\n", index, np);
// Initialise group
    grp.name = np.name;
//
// the binding format is fsl,pins = <PIN MUX_ID CONFIG>
//
    list = of_get_property(np, "fsl,pins", &size);
// we do not check return since it's safe node passed down
    if (!size || size % 12) {
    dev_notice(info.dev, "Not a valid fsl,pins property (%pOFn)\n",
    np);
    return -EINVAL;
    }
    grp.npins = size / 12;
    grp.pins = devm_kcalloc(info.dev,
    grp.npins, sizeof(struct imx1_pin), GFP_KERNEL);
    grp.pin_ids = devm_kcalloc(info.dev,
    grp.npins, sizeof(unsigned int), GFP_KERNEL);
    if (!grp.pins || !grp.pin_ids)
    return -ENOMEM;
    for (i = 0; i < grp.npins; i++) {
    grp.pins[i].pin_id = be32_to_cpu(*list++);
    grp.pins[i].mux_id = be32_to_cpu(*list++);
    grp.pins[i].config = be32_to_cpu(*list++);
    grp.pin_ids[i] = grp.pins[i].pin_id;
    }
    return 0;
    }
    static int imx1_pinctrl_parse_functions(struct device_node *np,
    struct imx1_pinctrl_soc_info *info,
    u32 index)
    {
    struct imx1_pmx_func *func;
    struct imx1_pin_group *grp;
    int ret;
    static u32 grp_index;
    let mut i: u32 = 0;
    dev_dbg(info.dev, "parse function(%d): %pOFn\n", index, np);
    func = &info.functions[index];
// Initialise function
    func.name = np.name;
    func.num_groups = of_get_child_count(np);
    if (func.num_groups == 0)
    return -EINVAL;
    func.groups = devm_kcalloc(info.dev,
    func.num_groups, sizeof(char *), GFP_KERNEL);
    if (!func.groups)
    return -ENOMEM;
    for_each_child_of_node_scoped(np, child) {
    func.groups[i] = child.name;
    grp = &info.groups[grp_index++];
    ret = imx1_pinctrl_parse_groups(child, grp, info, i++);
    if (ret == -ENOMEM)
    return ret;
    }
    return 0;
    }
//
// Check if the DT contains pins in the direct child nodes. This indicates the
// newer DT format to store pins. This function returns true if the first found
// fsl,pins property is in a child of np. Otherwise false is returned.
//
#[no_mangle]
unsafe extern "C" fn imx1_pinctrl_dt_is_flat_functions(np: *mut device_node) -> bool {
    static bool imx1_pinctrl_dt_is_flat_functions(struct device_node *np)
    {
    for_each_child_of_node_scoped(np, function_np) {
    if (of_property_present(function_np, "fsl,pins"))
    return true;
    for_each_child_of_node_scoped(function_np, pinctrl_np) {
    if (of_property_present(pinctrl_np, "fsl,pins"))
    return false;
    }
    }
    return true;
    }
    static int imx1_pinctrl_parse_dt(struct platform_device *pdev,
    struct imx1_pinctrl *pctl, struct imx1_pinctrl_soc_info *info)
    {
    struct device_node *np = pdev.dev.of_node;
    bool flat_funcs;
    int ret;
    let mut nfuncs: u32 = 0;
    let mut ngroups: u32 = 0;
    let mut ifunc: u32 = 0;
    if (!np)
    return -ENODEV;
    flat_funcs = imx1_pinctrl_dt_is_flat_functions(np);
    if (flat_funcs) {
    nfuncs = 1;
    ngroups = of_get_child_count(np);
    } else {
    for_each_child_of_node_scoped(np, child) {
    ++nfuncs;
    ngroups += of_get_child_count(child);
    }
    }
    if (!nfuncs) {
    dev_err(&pdev.dev, "No pin functions defined\n");
    return -EINVAL;
    }
    info.nfunctions = nfuncs;
    info.functions = devm_kcalloc(&pdev.dev,
    nfuncs, sizeof(struct imx1_pmx_func), GFP_KERNEL);
    info.ngroups = ngroups;
    info.groups = devm_kcalloc(&pdev.dev,
    ngroups, sizeof(struct imx1_pin_group), GFP_KERNEL);
    if (!info.functions || !info.groups)
    return -ENOMEM;
    if (flat_funcs) {
    imx1_pinctrl_parse_functions(np, info, 0);
    } else {
    for_each_child_of_node_scoped(np, child) {
    ret = imx1_pinctrl_parse_functions(child, info, ifunc++);
    if (ret == -ENOMEM)
    return -ENOMEM;
    }
    }
    return 0;
    }
    int imx1_pinctrl_core_probe(struct platform_device *pdev,
    struct imx1_pinctrl_soc_info *info)
    {
    struct imx1_pinctrl *ipctl;
    struct resource *res;
    struct pinctrl_desc *pctl_desc;
    int ret;
    if (!info || !info.pins || !info.npins) {
    dev_err(&pdev.dev, "wrong pinctrl info\n");
    return -EINVAL;
    }
    info.dev = &pdev.dev;
// Create state holders etc for this driver
    ipctl = devm_kzalloc(&pdev.dev, sizeof(*ipctl), GFP_KERNEL);
    if (!ipctl)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENOENT;
    ipctl.base = devm_ioremap(&pdev.dev, res.start,
    resource_size(res));
    if (!ipctl.base)
    return -ENOMEM;
    pctl_desc = &imx1_pinctrl_desc;
    pctl_desc.name = dev_name(&pdev.dev);
    pctl_desc.pins = info.pins;
    pctl_desc.npins = info.npins;
    ret = imx1_pinctrl_parse_dt(pdev, ipctl, info);
    if (ret) {
    dev_err(&pdev.dev, "fail to probe dt properties\n");
    return ret;
    }
    ipctl.info = info;
    ipctl.dev = info.dev;
    platform_set_drvdata(pdev, ipctl);
    ipctl.pctl = devm_pinctrl_register(&pdev.dev, pctl_desc, ipctl);
    if (IS_ERR(ipctl.pctl)) {
    dev_err(&pdev.dev, "could not register IMX pinctrl driver\n");
    return PTR_ERR(ipctl.pctl);
    }
    ret = of_platform_populate(pdev.dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "Failed to populate subdevices\n");
    return ret;
    }
    dev_info(&pdev.dev, "initialized IMX pinctrl driver\n");
    return 0;
    }
