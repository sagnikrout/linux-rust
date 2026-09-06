//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/r8a78000-mdlc.c
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
// R-Car X5H Module Controller
//
// Copyright (C) 2026 Glider bv
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_map {
    pub /: *mut *mut int hw_id; / Hardware module ID or -1 sentinel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdlc_info {
    pub base: u32,
    pub mod_map: *const mod_map,
}

//
// struct r8a78000_mdlc_priv - Module Controller Private Data
//
// @link: Link into list of MDLC instances
// @genpd_data: PM domain provider data
// @rcdev: Reset controller entity
// @dev: MDLC device
// @np: Device node in DT representing the MDLC
// @mod_map: Mapping from hardware module IDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a78000_mdlc_priv {
    pub link: hlist_node,
    pub genpd_data: genpd_onecell_data,
    pub rcdev: reset_controller_dev,
    pub dev: *mut device,
    pub np: *mut device_node,
    pub mod_map: *const mod_map,
}

    static struct generic_pm_domain *r8a78000_genpd_always_on;
    static HLIST_HEAD(r8a78000_mdlc_list);
    static DEFINE_MUTEX(r8a78000_mdlc_lock);	/* protects the two above */
    static struct generic_pm_domain *r8a78000_genpd_xlate(
    const struct of_phandle_args *spec, void *data)
    {
    struct r8a78000_mdlc_priv *priv = container_of(data,
    struct r8a78000_mdlc_priv, genpd_data);
    struct device *dev = priv.dev;
    u32 id;
    if (spec.args_count != 2)
    return ERR_PTR(-EINVAL);
    id = spec.args[0];
    if (id >= R8A78000_MDLC_PD_AON) {
    dev_dbg(dev,
    "Mapping HW power domain 0x%x to always-on domain\n",
    id);
    return r8a78000_genpd_always_on;
    }
// For now only always-on domains are supported
    dev_err(dev, "Unknown power domain 0x%x\n", id);
    return ERR_PTR(-ENOENT);
    }

    container_of(_rcdev, struct r8a78000_mdlc_priv, rcdev)
    static const struct mod_map *mod_map_find(const struct mod_map *map, u32 id)
    {
    if (!map)
    return core::ptr::null_mut();
    for (; map.hw_id >= 0; map++) {
    if (map.hw_id == id)
    return map;
    }
    return core::ptr::null_mut();
    }
    static int r8a78000_mdlc_reset_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *spec)
    {
    struct r8a78000_mdlc_priv *priv = rcdev_to_priv(rcdev);
    struct device *dev = priv.dev;
    const struct mod_map *map;
    u32 id;
    if (spec.args_count != 1)
    return -EINVAL;
    id = spec.args[0];
    map = mod_map_find(priv.mod_map, id);
    if (!map) {
    dev_err(dev, "Unknown reset 0x%x\n", id);
    return -ENOENT;
    }
    dev_dbg(dev, "Ignoring HW reset 0x%x\n", id);
    return id;
    }

    static int r8a78000_mdlc_ ## op(struct reset_controller_dev *rcdev, \
    unsigned long id)		    \
    {								    \
    struct r8a78000_mdlc_priv *priv = rcdev_to_priv(rcdev);	    \
    \
    dev_dbg(priv.dev, "%s: Ignoring\n", __func__);		    \
    return 0;						    \
    }
    DEFINE_MDLC_RESET_WRAPPER(reset)
    DEFINE_MDLC_RESET_WRAPPER(assert)
    DEFINE_MDLC_RESET_WRAPPER(deassert)
    DEFINE_MDLC_RESET_WRAPPER(status)
    static const struct reset_control_ops r8a78000_mdlc_reset_ops = {
    .reset = r8a78000_mdlc_reset,
    .assert = r8a78000_mdlc_assert,
    .deassert = r8a78000_mdlc_deassert,
    .status = r8a78000_mdlc_status,
    };
    static int r8a78000_mdlc_attach_dev(struct generic_pm_domain *domain,
    struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct r8a78000_mdlc_priv *priv;
    struct of_phandle_args pd_spec;
    const struct mod_map *map;
    unsigned int id;
    int ret;
    ret = of_parse_phandle_with_args(np, "power-domains",
    "#power-domain-cells", 0, &pd_spec);
    if (ret < 0)
    return ret;
    scoped_guard(mutex, &r8a78000_mdlc_lock) {
    hlist_for_each_entry(priv, &r8a78000_mdlc_list, link) {
    if (priv.np == pd_spec.np)
    break;
    }
    }
    if (!priv) {
    dev_err(dev, "%s: MDLC %pOF not found\n", __func__, pd_spec.np);
    of_node_put(pd_spec.np);
    return -ENODEV;
    }
    id = pd_spec.args[1];
    of_node_put(pd_spec.np);
    map = mod_map_find(priv.mod_map, id);
    if (!map) {
    dev_err(dev, "Unknown module 0x%x\n", id);
    return -ENOENT;
    }
    dev_dbg(dev, "Ignoring HW module 0x%x\n", id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r8a78000_mdlc_unlink(data: *mut c_void) {
    static void r8a78000_mdlc_unlink(void *data)
    {
    struct r8a78000_mdlc_priv *priv = data;
    scoped_guard(mutex, &r8a78000_mdlc_lock) {
    hlist_del(&priv.link);
    }
    }
#[no_mangle]
unsafe extern "C" fn r8a78000_genpd_del_provider(data: *mut c_void) {
    static void r8a78000_genpd_del_provider(void *data)
    {
    of_genpd_del_provider(data);
    }
#[no_mangle]
unsafe extern "C" fn r8a78000_genpd_always_on_singleton(dev: *mut device) -> c_int {
    static int r8a78000_genpd_always_on_singleton(struct device *dev)
    {
    struct generic_pm_domain *genpd;
    int ret;
    guard(mutex)(&r8a78000_mdlc_lock);
    if (r8a78000_genpd_always_on)
    return 0;
    genpd = kzalloc_obj(*genpd);
    if (!genpd)
    return -ENOMEM;
    genpd.name = "always-on";
    genpd.attach_dev = r8a78000_mdlc_attach_dev;
    ret = pm_genpd_init(genpd, &pm_domain_always_on_gov, false);
    if (ret) {
    kfree(genpd);
    return dev_err_probe(dev, ret,
    "Failed to create always-on domain\n");
    }
    r8a78000_genpd_always_on = genpd;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r8a78000_mdlc_probe(pdev: *mut platform_device) -> c_int {
    static int r8a78000_mdlc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct r8a78000_mdlc_priv *priv;
    const struct mdlc_info *info;
    struct resource *res;
    int ret;
    ret = r8a78000_genpd_always_on_singleton(dev);
    if (ret)
    return ret;
    info = of_device_get_match_data(dev);
    if (!info)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.np = np;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    for (; info.base; info++) {
    if (info.base == res.start)
    break;
    }
    if (!info.base) {
    dev_dbg(dev, "Unsupported MDLC instance 0x%pa\n", &res.start);
    return -ENODEV;
    }
    priv.mod_map = info.mod_map;
    scoped_guard(mutex, &r8a78000_mdlc_lock) {
    hlist_add_head(&priv.link, &r8a78000_mdlc_list);
    }
    ret = devm_add_action_or_reset(dev, r8a78000_mdlc_unlink, priv);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add action\n");
// Note that no actual domains are registered, just need translation
    priv.genpd_data.xlate = r8a78000_genpd_xlate;
    ret = of_genpd_add_provider_onecell(np, &priv.genpd_data);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register genpd provider\n");
    ret = devm_add_action_or_reset(dev, r8a78000_genpd_del_provider, np);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to add unregister action\n");
    priv.rcdev.ops = &r8a78000_mdlc_reset_ops;
    priv.rcdev.of_node = np;
    priv.rcdev.of_reset_n_cells = 1;
    priv.rcdev.of_xlate = r8a78000_mdlc_reset_xlate;
    ret = devm_reset_controller_register(dev, &priv.rcdev);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register reset controller\n");
    return 0;
    }
    static const struct mod_map r8a78000_mdlc_perw_mod_default[] = {
    { 0x54 },	/* HSCIF0 */
    { -1 }
    };
    static const struct mdlc_info r8a78000_mdlc_default[] = {
    {
    .base = 0xc05d0000 /* mdlc_perw */,
    .mod_map = r8a78000_mdlc_perw_mod_default,
    },
    { /* sentinel */ }
    };
    static const struct of_device_id r8a78000_mdlc_match[] = {
    {
    .compatible = "renesas,r8a78000-mdlc",
    .data = &r8a78000_mdlc_default,
    },
    { /* sentinel */ }
    };
    static struct platform_driver r8a78000_mdlc_driver = {
    .probe = r8a78000_mdlc_probe,
    .driver = {
    .name = "r8a78000-mdlc",
    .of_match_table = r8a78000_mdlc_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(r8a78000_mdlc_driver)
    MODULE_DESCRIPTION("R-Car X5H MDLC Driver");
