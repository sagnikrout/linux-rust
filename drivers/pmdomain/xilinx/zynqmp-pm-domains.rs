//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/xilinx/zynqmp-pm-domains.c
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
// ZynqMP Generic PM domain support
//
// Copyright (C) 2015-2019 Xilinx, Inc.
//
// Davorin Mista <davorin.mista@aggios.com>
// Jolly Shah <jollys@xilinx.com>
// Rajan Vaja <rajan.vaja@xilinx.com>
//

    static int min_capability;
//
// struct zynqmp_pm_domain - Wrapper around struct generic_pm_domain
// @gpd:		Generic power domain
// @node_id:		PM node ID corresponding to device inside PM domain
// @requested:		The PM node mapped to the PM domain has been requested
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_pm_domain {
    pub gpd: generic_pm_domain,
    pub node_id: u32,
    pub requested: bool,
}

    container_of(pm_domain, struct zynqmp_pm_domain, gpd)
//
// zynqmp_gpd_is_active_wakeup_path() - Check if device is in wakeup source
// path
// @dev:	Device to check for wakeup source path
// @not_used:	Data member (not required)
//
// This function is checks device's child hierarchy and checks if any device is
// set as wakeup source.
//
// Return: 1 if device is in wakeup source path else 0
//
#[no_mangle]
unsafe extern "C" fn zynqmp_gpd_is_active_wakeup_path(dev: *mut device, not_used: *mut c_void) -> c_int {
    static int zynqmp_gpd_is_active_wakeup_path(struct device *dev, void *not_used)
    {
    int may_wakeup;
    may_wakeup = device_may_wakeup(dev);
    if (may_wakeup)
    return may_wakeup;
    return device_for_each_child(dev, core::ptr::null_mut(),
    zynqmp_gpd_is_active_wakeup_path);
    }
//
// zynqmp_gpd_power_on() - Power on PM domain
// @domain:	Generic PM domain
//
// This function is called before devices inside a PM domain are resumed, to
// power on PM domain.
//
// Return: 0 on success, error code otherwise
//
#[no_mangle]
unsafe extern "C" fn zynqmp_gpd_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int zynqmp_gpd_power_on(struct generic_pm_domain *domain)
    {
    struct zynqmp_pm_domain *pd = to_zynqmp_pm_domain(domain);
    int ret;
    ret = zynqmp_pm_set_requirement(pd.node_id,
    ZYNQMP_PM_CAPABILITY_ACCESS,
    ZYNQMP_PM_MAX_QOS,
    ZYNQMP_PM_REQUEST_ACK_BLOCKING);
    if (ret) {
    dev_err(&domain.dev,
    "failed to set requirement to 0x%x for PM node id %d: %d\n",
    ZYNQMP_PM_CAPABILITY_ACCESS, pd.node_id, ret);
    return ret;
    }
    dev_dbg(&domain.dev, "set requirement to 0x%x for PM node id %d\n",
    ZYNQMP_PM_CAPABILITY_ACCESS, pd.node_id);
    return 0;
    }
//
// zynqmp_gpd_power_off() - Power off PM domain
// @domain:	Generic PM domain
//
// This function is called after devices inside a PM domain are suspended, to
// power off PM domain.
//
// Return: 0 on success, error code otherwise
//
#[no_mangle]
unsafe extern "C" fn zynqmp_gpd_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int zynqmp_gpd_power_off(struct generic_pm_domain *domain)
    {
    struct zynqmp_pm_domain *pd = to_zynqmp_pm_domain(domain);
    int ret;
    struct pm_domain_data *pdd, *tmp;
    let mut capabilities: u32 = min_capability;
    bool may_wakeup;
// If domain is already released there is nothing to be done
    if (!pd.requested) {
    dev_dbg(&domain.dev, "PM node id %d is already released\n",
    pd.node_id);
    return 0;
    }
    list_for_each_entry_safe(pdd, tmp, &domain.dev_list, list_node) {
// If device is in wakeup path, set capability to WAKEUP
    may_wakeup = zynqmp_gpd_is_active_wakeup_path(pdd.dev, core::ptr::null_mut());
    if (may_wakeup) {
    dev_dbg(pdd.dev, "device is in wakeup path in %s\n",
    domain.name);
    capabilities = ZYNQMP_PM_CAPABILITY_WAKEUP;
    break;
    }
    }
    ret = zynqmp_pm_set_requirement(pd.node_id, capabilities, 0,
    ZYNQMP_PM_REQUEST_ACK_NO);
    if (ret) {
    dev_err(&domain.dev,
    "failed to set requirement to 0x%x for PM node id %d: %d\n",
    capabilities, pd.node_id, ret);
    return ret;
    }
    dev_dbg(&domain.dev, "set requirement to 0x%x for PM node id %d\n",
    capabilities, pd.node_id);
    return 0;
    }
//
// zynqmp_gpd_attach_dev() - Attach device to the PM domain
// @domain:	Generic PM domain
// @dev:	Device to attach
//
// Return: 0 on success, error code otherwise
//
    static int zynqmp_gpd_attach_dev(struct generic_pm_domain *domain,
    struct device *dev)
    {
    struct zynqmp_pm_domain *pd = to_zynqmp_pm_domain(domain);
    int ret;
// If this is not the first device to attach there is nothing to do
    if (domain.device_count)
    return 0;
    ret = zynqmp_pm_request_node(pd.node_id, 0, 0,
    ZYNQMP_PM_REQUEST_ACK_BLOCKING);
    if (ret) {
    dev_err(&domain.dev, "%s request failed for node %d: %d\n",
    domain.name, pd.node_id, ret);
    return ret;
    }
    pd.requested = true;
    dev_dbg(&domain.dev, "%s requested PM node id %d\n",
    dev_name(dev), pd.node_id);
    return 0;
    }
//
// zynqmp_gpd_detach_dev() - Detach device from the PM domain
// @domain:	Generic PM domain
// @dev:	Device to detach
//
    static void zynqmp_gpd_detach_dev(struct generic_pm_domain *domain,
    struct device *dev)
    {
    struct zynqmp_pm_domain *pd = to_zynqmp_pm_domain(domain);
    int ret;
// If this is not the last device to detach there is nothing to do
    if (domain.device_count)
    return;
    ret = zynqmp_pm_release_node(pd.node_id);
    if (ret) {
    dev_err(&domain.dev, "failed to release PM node id %d: %d\n",
    pd.node_id, ret);
    return;
    }
    pd.requested = false;
    dev_dbg(&domain.dev, "%s released PM node id %d\n",
    dev_name(dev), pd.node_id);
    }
    static struct generic_pm_domain *zynqmp_gpd_xlate
    (const struct of_phandle_args *genpdspec, void *data)
    {
    struct genpd_onecell_data *genpd_data = data;
    unsigned int i, idx = genpdspec.args[0];
    struct zynqmp_pm_domain *pd;
    pd = to_zynqmp_pm_domain(genpd_data.domains[0]);
    if (genpdspec.args_count != 1)
    return ERR_PTR(-EINVAL);
// Check for existing pm domains
    for (i = 0; i < ZYNQMP_NUM_DOMAINS; i++) {
    if (pd[i].node_id == idx)
    goto done;
    }
//
// Add index in empty node_id of power domain list as no existing
// power domain found for current index.
//
    for (i = 0; i < ZYNQMP_NUM_DOMAINS; i++) {
    if (pd[i].node_id == 0) {
    pd[i].node_id = idx;
    break;
    }
    }
    done:
    if (!genpd_data.domains[i] || i == ZYNQMP_NUM_DOMAINS)
    return ERR_PTR(-ENOENT);
    return genpd_data.domains[i];
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_gpd_probe(pdev: *mut platform_device) -> c_int {
    static int zynqmp_gpd_probe(struct platform_device *pdev)
    {
    int i;
    struct genpd_onecell_data *zynqmp_pd_data;
    struct generic_pm_domain **domains;
    struct zynqmp_pm_domain *pd;
    struct device *dev = &pdev.dev;
    pd = devm_kcalloc(dev, ZYNQMP_NUM_DOMAINS, sizeof(*pd), GFP_KERNEL);
    if (!pd)
    return -ENOMEM;
    zynqmp_pd_data = devm_kzalloc(dev, sizeof(*zynqmp_pd_data), GFP_KERNEL);
    if (!zynqmp_pd_data)
    return -ENOMEM;
    zynqmp_pd_data.xlate = zynqmp_gpd_xlate;
    domains = devm_kcalloc(dev, ZYNQMP_NUM_DOMAINS, sizeof(*domains),
    GFP_KERNEL);
    if (!domains)
    return -ENOMEM;
    if (!of_device_is_compatible(dev.parent.of_node,
    "xlnx,zynqmp-firmware"))
    min_capability = ZYNQMP_PM_CAPABILITY_UNUSABLE;
    for (i = 0; i < ZYNQMP_NUM_DOMAINS; i++, pd++) {
    pd.node_id = 0;
    pd.gpd.name = kasprintf(GFP_KERNEL, "domain%d", i);
    pd.gpd.power_off = zynqmp_gpd_power_off;
    pd.gpd.power_on = zynqmp_gpd_power_on;
    pd.gpd.attach_dev = zynqmp_gpd_attach_dev;
    pd.gpd.detach_dev = zynqmp_gpd_detach_dev;
    domains[i] = &pd.gpd;
// Mark all PM domains as initially powered off
    pm_genpd_init(&pd.gpd, core::ptr::null_mut(), true);
    }
    zynqmp_pd_data.domains = domains;
    zynqmp_pd_data.num_domains = ZYNQMP_NUM_DOMAINS;
    of_genpd_add_provider_onecell(dev.parent.of_node, zynqmp_pd_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_gpd_remove(pdev: *mut platform_device) {
    static void zynqmp_gpd_remove(struct platform_device *pdev)
    {
    of_genpd_del_provider(pdev.dev.parent.of_node);
    }
    static struct platform_driver zynqmp_power_domain_driver = {
    .driver	= {
    .name = "zynqmp_power_controller",
    },
    .probe = zynqmp_gpd_probe,
    .remove = zynqmp_gpd_remove,
    };
    module_platform_driver(zynqmp_power_domain_driver);
    MODULE_ALIAS("platform:zynqmp_power_controller");
