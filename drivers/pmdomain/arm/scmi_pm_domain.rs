//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/arm/scmi_pm_domain.c
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
// SCMI Generic power domain support.
//
// Copyright (C) 2018-2021 ARM Ltd.
//

    static const struct scmi_power_proto_ops *power_ops;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_pm_domain {
    pub genpd: generic_pm_domain,
    pub ph: *const scmi_protocol_handle,
    pub name: *const c_char,
    pub domain: u32,
}

#[no_mangle]
unsafe extern "C" fn scmi_pd_power(domain: *mut generic_pm_domain, state: u32) -> c_int {
    static int scmi_pd_power(struct generic_pm_domain *domain, u32 state)
    {
    struct scmi_pm_domain *pd = to_scmi_pd(domain);
    return power_ops.state_set(pd.ph, pd.domain, state);
    }
#[no_mangle]
unsafe extern "C" fn scmi_pd_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int scmi_pd_power_on(struct generic_pm_domain *domain)
    {
    return scmi_pd_power(domain, SCMI_POWER_STATE_GENERIC_ON);
    }
#[no_mangle]
unsafe extern "C" fn scmi_pd_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int scmi_pd_power_off(struct generic_pm_domain *domain)
    {
    return scmi_pd_power(domain, SCMI_POWER_STATE_GENERIC_OFF);
    }
#[no_mangle]
unsafe extern "C" fn scmi_pm_domain_probe(sdev: *mut scmi_device) -> c_int {
    static int scmi_pm_domain_probe(struct scmi_device *sdev)
    {
    int num_domains, i, ret;
    struct device *dev = &sdev.dev;
    struct device_node *np = dev.of_node;
    struct scmi_pm_domain *scmi_pd;
    struct genpd_onecell_data *scmi_pd_data;
    struct generic_pm_domain **domains;
    const struct scmi_handle *handle = sdev.handle;
    struct scmi_protocol_handle *ph;
    if (!handle)
    return -ENODEV;
    power_ops = handle.devm_protocol_get(sdev, SCMI_PROTOCOL_POWER, &ph);
    if (IS_ERR(power_ops))
    return PTR_ERR(power_ops);
    num_domains = power_ops.num_domains_get(ph);
    if (num_domains < 0) {
    dev_err(dev, "number of domains not found\n");
    return num_domains;
    }
    scmi_pd = devm_kcalloc(dev, num_domains, sizeof(*scmi_pd), GFP_KERNEL);
    if (!scmi_pd)
    return -ENOMEM;
    scmi_pd_data = devm_kzalloc(dev, sizeof(*scmi_pd_data), GFP_KERNEL);
    if (!scmi_pd_data)
    return -ENOMEM;
    domains = devm_kcalloc(dev, num_domains, sizeof(*domains), GFP_KERNEL);
    if (!domains)
    return -ENOMEM;
    for (i = 0; i < num_domains; i++, scmi_pd++) {
    u32 state;
    if (power_ops.state_get(ph, i, &state)) {
    dev_warn(dev, "failed to get state for domain %d\n", i);
    continue;
    }
//
// Register the explicit power on request to the firmware so
// that it is tracked as used by OSPM agent and not
// accidentally turned off with OSPM's knowledge
//
    if (state == SCMI_POWER_STATE_GENERIC_ON)
    power_ops.state_set(ph, i, state);
    scmi_pd.domain = i;
    scmi_pd.ph = ph;
    scmi_pd.name = power_ops.name_get(ph, i);
    scmi_pd.genpd.name = scmi_pd.name;
    scmi_pd.genpd.power_off = scmi_pd_power_off;
    scmi_pd.genpd.power_on = scmi_pd_power_on;
    scmi_pd.genpd.flags = GENPD_FLAG_ACTIVE_WAKEUP;
    pm_genpd_init(&scmi_pd.genpd, core::ptr::null_mut(),
    state == SCMI_POWER_STATE_GENERIC_OFF);
    domains[i] = &scmi_pd.genpd;
    }
    scmi_pd_data.domains = domains;
    scmi_pd_data.num_domains = num_domains;
    ret = of_genpd_add_provider_onecell(np, scmi_pd_data);
    if (ret)
    goto err_rm_genpds;
    dev_set_drvdata(dev, scmi_pd_data);
//
// Parse (optional) power-domains-child-ids property to establish
// parent-child relationships.
//
    ret = of_genpd_add_child_ids(np, scmi_pd_data);
    if (ret < 0)
    dev_err(dev, "Failed to add child domain hierarchy: %d\n", ret);
    dev_info(dev, "Initialized %d power domains", num_domains);
    return 0;
    err_rm_genpds:
    for (i = num_domains - 1; i >= 0; i--)
    pm_genpd_remove(domains[i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scmi_pm_domain_remove(sdev: *mut scmi_device) {
    static void scmi_pm_domain_remove(struct scmi_device *sdev)
    {
    int i;
    struct genpd_onecell_data *scmi_pd_data;
    struct device *dev = &sdev.dev;
    struct device_node *np = dev.of_node;
    scmi_pd_data = dev_get_drvdata(dev);
// Remove any parent-child relationships established at probe time
    of_genpd_remove_child_ids(np, scmi_pd_data);
    of_genpd_del_provider(np);
    for (i = 0; i < scmi_pd_data.num_domains; i++) {
    if (!scmi_pd_data.domains[i])
    continue;
    pm_genpd_remove(scmi_pd_data.domains[i]);
    }
    }
    static const struct scmi_device_id scmi_id_table[] = {
    { SCMI_PROTOCOL_POWER, "genpd" },
    { },
    };
    MODULE_DEVICE_TABLE(scmi, scmi_id_table);
    static struct scmi_driver scmi_power_domain_driver = {
    .name = "scmi-power-domain",
    .probe = scmi_pm_domain_probe,
    .remove = scmi_pm_domain_remove,
    .id_table = scmi_id_table,
    };
    module_scmi_driver(scmi_power_domain_driver);
    MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
    MODULE_DESCRIPTION("ARM SCMI power domain driver");
    MODULE_LICENSE("GPL v2");
