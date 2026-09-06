//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/tegra/powergate-bpmp.c
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
// Copyright (c) 2016-2017, NVIDIA CORPORATION. All rights reserved
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_powergate_info {
    pub id: c_uint,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_powergate {
    pub genpd: generic_pm_domain,
    pub bpmp: *mut tegra_bpmp,
    pub id: c_uint,
}

    static inline struct tegra_powergate *
    to_tegra_powergate(struct generic_pm_domain *genpd)
    {
    return container_of(genpd, struct tegra_powergate, genpd);
    }
    static int tegra_bpmp_powergate_set_state(struct tegra_bpmp *bpmp,
    unsigned int id, u32 state)
    {
    struct mrq_pg_request request;
    struct tegra_bpmp_message msg;
    int err;
    memset(&request, 0, sizeof(request));
    request.cmd = CMD_PG_SET_STATE;
    request.id = id;
    request.set_state.state = state;
    memset(&msg, 0, sizeof(msg));
    msg.mrq = MRQ_PG;
    msg.tx.data = &request;
    msg.tx.size = sizeof(request);
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0)
    return err;
#[no_mangle]
pub unsafe extern "C" fn if(0: msg.rx.ret <) -> else {
    else if (msg.rx.ret < 0)
    return -EINVAL;
    return 0;
    }
    static int tegra_bpmp_powergate_get_state(struct tegra_bpmp *bpmp,
    unsigned int id)
    {
    struct mrq_pg_response response;
    struct mrq_pg_request request;
    struct tegra_bpmp_message msg;
    int err;
    memset(&request, 0, sizeof(request));
    request.cmd = CMD_PG_GET_STATE;
    request.id = id;
    memset(&response, 0, sizeof(response));
    memset(&msg, 0, sizeof(msg));
    msg.mrq = MRQ_PG;
    msg.tx.data = &request;
    msg.tx.size = sizeof(request);
    msg.rx.data = &response;
    msg.rx.size = sizeof(response);
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0)
    return PG_STATE_OFF;
#[no_mangle]
pub unsafe extern "C" fn if(0: msg.rx.ret <) -> else {
    else if (msg.rx.ret < 0)
    return -EINVAL;
    return response.get_state.state;
    }
#[no_mangle]
unsafe extern "C" fn tegra_bpmp_powergate_get_max_id(bpmp: *mut tegra_bpmp) -> c_int {
    static int tegra_bpmp_powergate_get_max_id(struct tegra_bpmp *bpmp)
    {
    struct mrq_pg_response response;
    struct mrq_pg_request request;
    struct tegra_bpmp_message msg;
    int err;
    memset(&request, 0, sizeof(request));
    request.cmd = CMD_PG_GET_MAX_ID;
    memset(&response, 0, sizeof(response));
    memset(&msg, 0, sizeof(msg));
    msg.mrq = MRQ_PG;
    msg.tx.data = &request;
    msg.tx.size = sizeof(request);
    msg.rx.data = &response;
    msg.rx.size = sizeof(response);
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0)
    return err;
#[no_mangle]
pub unsafe extern "C" fn if(0: msg.rx.ret <) -> else {
    else if (msg.rx.ret < 0)
    return -EINVAL;
    return response.get_max_id.max_id;
    }
    static char *tegra_bpmp_powergate_get_name(struct tegra_bpmp *bpmp,
    unsigned int id)
    {
    struct mrq_pg_response response;
    struct mrq_pg_request request;
    struct tegra_bpmp_message msg;
    int err;
    memset(&request, 0, sizeof(request));
    request.cmd = CMD_PG_GET_NAME;
    request.id = id;
    memset(&response, 0, sizeof(response));
    memset(&msg, 0, sizeof(msg));
    msg.mrq = MRQ_PG;
    msg.tx.data = &request;
    msg.tx.size = sizeof(request);
    msg.rx.data = &response;
    msg.rx.size = sizeof(response);
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0 || msg.rx.ret < 0)
    return core::ptr::null_mut();
    if (response.get_name.name[0] == '\0')
    return core::ptr::null_mut();
    if (dev_to_node(bpmp.dev) != NUMA_NO_NODE)
    return kasprintf(GFP_KERNEL, "%d-%s", dev_to_node(bpmp.dev),
    response.get_name.name);
    return kstrdup(response.get_name.name, GFP_KERNEL);
    }
    static inline bool tegra_bpmp_powergate_is_powered(struct tegra_bpmp *bpmp,
    unsigned int id)
    {
    return tegra_bpmp_powergate_get_state(bpmp, id) != PG_STATE_OFF;
    }
#[no_mangle]
unsafe extern "C" fn tegra_powergate_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int tegra_powergate_power_on(struct generic_pm_domain *domain)
    {
    struct tegra_powergate *powergate = to_tegra_powergate(domain);
    struct tegra_bpmp *bpmp = powergate.bpmp;
    return tegra_bpmp_powergate_set_state(bpmp, powergate.id,
    PG_STATE_ON);
    }
#[no_mangle]
unsafe extern "C" fn tegra_powergate_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int tegra_powergate_power_off(struct generic_pm_domain *domain)
    {
    struct tegra_powergate *powergate = to_tegra_powergate(domain);
    struct tegra_bpmp *bpmp = powergate.bpmp;
    return tegra_bpmp_powergate_set_state(bpmp, powergate.id,
    PG_STATE_OFF);
    }
    static struct tegra_powergate *
    tegra_powergate_add(struct tegra_bpmp *bpmp,
    const struct tegra_powergate_info *info)
    {
    struct tegra_powergate *powergate;
    bool off;
    int err;
    off = !tegra_bpmp_powergate_is_powered(bpmp, info.id);
    powergate = devm_kzalloc(bpmp.dev, sizeof(*powergate), GFP_KERNEL);
    if (!powergate)
    return ERR_PTR(-ENOMEM);
    powergate.id = info.id;
    powergate.bpmp = bpmp;
    powergate.genpd.name = kstrdup(info.name, GFP_KERNEL);
    powergate.genpd.power_on = tegra_powergate_power_on;
    powergate.genpd.power_off = tegra_powergate_power_off;
    powergate.genpd.flags = GENPD_FLAG_NO_STAY_ON;
    err = pm_genpd_init(&powergate.genpd, core::ptr::null_mut(), off);
    if (err < 0) {
    kfree(powergate.genpd.name);
    return ERR_PTR(err);
    }
    return powergate;
    }
#[no_mangle]
unsafe extern "C" fn tegra_powergate_remove(powergate: *mut tegra_powergate) {
    static void tegra_powergate_remove(struct tegra_powergate *powergate)
    {
    struct generic_pm_domain *genpd = &powergate.genpd;
    struct tegra_bpmp *bpmp = powergate.bpmp;
    int err;
    err = pm_genpd_remove(genpd);
    if (err < 0)
    dev_err(bpmp.dev, "failed to remove power domain %s: %d\n",
    genpd.name, err);
    kfree(genpd.name);
    }
    static int
    tegra_bpmp_probe_powergates(struct tegra_bpmp *bpmp,
    struct tegra_powergate_info **powergatesp)
    {
    struct tegra_powergate_info *powergates;
    unsigned int max_id, id, count = 0;
    let mut num_holes: c_uint = 0;
    int err;
    err = tegra_bpmp_powergate_get_max_id(bpmp);
    if (err < 0)
    return err;
    max_id = err;
    dev_dbg(bpmp.dev, "maximum powergate ID: %u\n", max_id);
    powergates = kzalloc_objs(*powergates, max_id + 1);
    if (!powergates)
    return -ENOMEM;
    for (id = 0; id <= max_id; id++) {
    struct tegra_powergate_info *info = &powergates[count];
    info.name = tegra_bpmp_powergate_get_name(bpmp, id);
    if (!info.name) {
    num_holes++;
    continue;
    }
    info.id = id;
    count++;
    }
    dev_dbg(bpmp.dev, "holes: %u\n", num_holes);
// powergatesp = powergates;
    return count;
    }
    static int tegra_bpmp_add_powergates(struct tegra_bpmp *bpmp,
    struct tegra_powergate_info *powergates,
    unsigned int count)
    {
    struct genpd_onecell_data *genpd = &bpmp.genpd;
    struct generic_pm_domain **domains;
    struct tegra_powergate *powergate;
    unsigned int i;
    int err;
    domains = kzalloc_objs(*domains, count);
    if (!domains)
    return -ENOMEM;
    for (i = 0; i < count; i++) {
    powergate = tegra_powergate_add(bpmp, &powergates[i]);
    if (IS_ERR(powergate)) {
    err = PTR_ERR(powergate);
    goto remove;
    }
    dev_dbg(bpmp.dev, "added power domain %s\n",
    powergate.genpd.name);
    domains[i] = &powergate.genpd;
    }
    genpd.num_domains = count;
    genpd.domains = domains;
    return 0;
    remove:
    while (i--) {
    powergate = to_tegra_powergate(domains[i]);
    tegra_powergate_remove(powergate);
    }
    kfree(domains);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra_bpmp_remove_powergates(bpmp: *mut tegra_bpmp) {
    static void tegra_bpmp_remove_powergates(struct tegra_bpmp *bpmp)
    {
    struct genpd_onecell_data *genpd = &bpmp.genpd;
    let mut i: c_uint = genpd.num_domains;
    struct tegra_powergate *powergate;
    while (i--) {
    dev_dbg(bpmp.dev, "removing power domain %s\n",
    genpd.domains[i].name);
    powergate = to_tegra_powergate(genpd.domains[i]);
    tegra_powergate_remove(powergate);
    }
    }
    static struct generic_pm_domain *
    tegra_powergate_xlate(const struct of_phandle_args *spec, void *data)
    {
    struct generic_pm_domain *domain = ERR_PTR(-ENOENT);
    struct genpd_onecell_data *genpd = data;
    unsigned int i;
    for (i = 0; i < genpd.num_domains; i++) {
    struct tegra_powergate *powergate;
    powergate = to_tegra_powergate(genpd.domains[i]);
    if (powergate.id == spec.args[0]) {
    domain = &powergate.genpd;
    break;
    }
    }
    return domain;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_bpmp_init_powergates(bpmp: *mut tegra_bpmp) -> c_int {
    int tegra_bpmp_init_powergates(struct tegra_bpmp *bpmp)
    {
    struct device_node *np = bpmp.dev.of_node;
    struct tegra_powergate_info *powergates;
    struct device *dev = bpmp.dev;
    unsigned int count, i;
    int err;
    err = tegra_bpmp_probe_powergates(bpmp, &powergates);
    if (err < 0)
    return err;
    count = err;
    dev_dbg(dev, "%u power domains probed\n", count);
    err = tegra_bpmp_add_powergates(bpmp, powergates, count);
    if (err < 0)
    goto free;
    bpmp.genpd.xlate = tegra_powergate_xlate;
    err = of_genpd_add_provider_onecell(np, &bpmp.genpd);
    if (err < 0) {
    dev_err(dev, "failed to add power domain provider: %d\n", err);
    tegra_bpmp_remove_powergates(bpmp);
    }
    free:
    for (i = 0; i < count; i++)
    kfree(powergates[i].name);
    kfree(powergates);
    return err;
    }
