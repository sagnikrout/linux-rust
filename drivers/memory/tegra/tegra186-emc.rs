//! Automatically rewritten from C to Rust
//! Source: drivers/memory/tegra/tegra186-emc.c
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
// Copyright (C) 2019-2025 NVIDIA CORPORATION.  All rights reserved.
//

    static DEFINE_MUTEX(tegra_emc_debugfs_root_lock);
    static struct dentry *tegra_emc_debugfs_root;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_emc_dvfs {
    pub latency: c_ulong,
    pub rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_emc {
    pub bpmp: *mut tegra_bpmp,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub clk_dbb: *mut clk,
    pub dvfs: *mut tegra186_emc_dvfs,
    pub num_dvfs: c_uint,
    struct {
    pub root: *mut dentry,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub debugfs: },
    pub provider: icc_provider,
}

//
// debugfs interface
//
// The memory controller driver exposes some files in debugfs that can be used
// to control the EMC frequency. The top-level directory can be found here:
//
// /sys/kernel/debug/emc
//
// It contains the following files:
//
// - available_rates: This file contains a list of valid, space-separated
// EMC frequencies.
//
// - min_rate: Writing a value to this file sets the given frequency as the
// floor of the permitted range. If this is higher than the currently
// configured EMC frequency, this will cause the frequency to be
// increased so that it stays within the valid range.
//
// - max_rate: Similarily to the min_rate file, writing a value to this file
// sets the given frequency as the ceiling of the permitted range. If
// the value is lower than the currently configured EMC frequency, this
// will cause the frequency to be decreased so that it stays within the
// valid range.
//
    static bool tegra186_emc_validate_rate(struct tegra186_emc *emc,
    unsigned long rate)
    {
    unsigned int i;
    for (i = 0; i < emc.num_dvfs; i++)
    if (rate == emc.dvfs[i].rate)
    return true;
    return false;
    }
    static int tegra186_emc_debug_available_rates_show(struct seq_file *s,
    void *data)
    {
    struct tegra186_emc *emc = s.private;
    const char *prefix = "";
    unsigned int i;
    for (i = 0; i < emc.num_dvfs; i++) {
    seq_printf(s, "%s%lu", prefix, emc.dvfs[i].rate);
    prefix = " ";
    }
    seq_puts(s, "\n");
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tegra186_emc_debug_available_rates);
#[no_mangle]
unsafe extern "C" fn tegra186_emc_debug_min_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra186_emc_debug_min_rate_get(void *data, u64 *rate)
    {
    struct tegra186_emc *emc = data;
// rate = emc->debugfs.min_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_emc_debug_min_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra186_emc_debug_min_rate_set(void *data, u64 rate)
    {
    struct tegra186_emc *emc = data;
    int err;
    if (!tegra186_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = clk_set_min_rate(emc.clk, rate);
    if (err < 0)
    return err;
    emc.debugfs.min_rate = rate;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(tegra186_emc_debug_min_rate_fops,
    tegra186_emc_debug_min_rate_get,
    tegra186_emc_debug_min_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn tegra186_emc_debug_max_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra186_emc_debug_max_rate_get(void *data, u64 *rate)
    {
    struct tegra186_emc *emc = data;
// rate = emc->debugfs.max_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_emc_debug_max_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra186_emc_debug_max_rate_set(void *data, u64 rate)
    {
    struct tegra186_emc *emc = data;
    int err;
    if (!tegra186_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = clk_set_max_rate(emc.clk, rate);
    if (err < 0)
    return err;
    emc.debugfs.max_rate = rate;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(tegra186_emc_debug_max_rate_fops,
    tegra186_emc_debug_max_rate_get,
    tegra186_emc_debug_max_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn tegra186_emc_get_emc_dvfs_latency(emc: *mut tegra186_emc) -> c_int {
    static int tegra186_emc_get_emc_dvfs_latency(struct tegra186_emc *emc)
    {
    struct mrq_emc_dvfs_latency_response response;
    struct tegra_bpmp_message msg;
    unsigned int i;
    int err;
    memset(&msg, 0, sizeof(msg));
    msg.mrq = MRQ_EMC_DVFS_LATENCY;
    msg.tx.data = core::ptr::null_mut();
    msg.tx.size = 0;
    msg.rx.data = &response;
    msg.rx.size = sizeof(response);
    err = tegra_bpmp_transfer(emc.bpmp, &msg);
    if (err < 0) {
    dev_err(emc.dev, "failed to EMC DVFS pairs: %d\n", err);
    return err;
    }
    if (msg.rx.ret < 0) {
    dev_err(emc.dev, "EMC DVFS MRQ failed: %d (BPMP error code)\n", msg.rx.ret);
    return -EINVAL;
    }
    emc.debugfs.min_rate = ULONG_MAX;
    emc.debugfs.max_rate = 0;
    emc.num_dvfs = response.num_pairs;
    emc.dvfs = devm_kmalloc_array(emc.dev, emc.num_dvfs, sizeof(*emc.dvfs), GFP_KERNEL);
    if (!emc.dvfs)
    return -ENOMEM;
    dev_dbg(emc.dev, "%u DVFS pairs:\n", emc.num_dvfs);
    for (i = 0; i < emc.num_dvfs; i++) {
    emc.dvfs[i].rate = response.pairs[i].freq * 1000;
    emc.dvfs[i].latency = response.pairs[i].latency;
    if (emc.dvfs[i].rate < emc.debugfs.min_rate)
    emc.debugfs.min_rate = emc.dvfs[i].rate;
    if (emc.dvfs[i].rate > emc.debugfs.max_rate)
    emc.debugfs.max_rate = emc.dvfs[i].rate;
    dev_dbg(emc.dev, "  %2u: %lu Hz . %lu us\n", i,
    emc.dvfs[i].rate, emc.dvfs[i].latency);
    }
    err = clk_set_rate_range(emc.clk, emc.debugfs.min_rate, emc.debugfs.max_rate);
    if (err < 0) {
    dev_err(emc.dev, "failed to set rate range [%lu-%lu] for %pC\n",
    emc.debugfs.min_rate, emc.debugfs.max_rate, emc.clk);
    return err;
    }
    scoped_guard(mutex, &tegra_emc_debugfs_root_lock) {
    if (!tegra_emc_debugfs_root)
    tegra_emc_debugfs_root = debugfs_create_dir("emc", core::ptr::null_mut());
    if (dev_to_node(emc.dev) == NUMA_NO_NODE)
    emc.debugfs.root = tegra_emc_debugfs_root;
    else
    emc.debugfs.root = debugfs_create_dir(dev_name(emc.dev),
    tegra_emc_debugfs_root);
    }
    debugfs_create_file("available_rates", 0444, emc.debugfs.root, emc,
    &tegra186_emc_debug_available_rates_fops);
    debugfs_create_file("min_rate", 0644, emc.debugfs.root, emc,
    &tegra186_emc_debug_min_rate_fops);
    debugfs_create_file("max_rate", 0644, emc.debugfs.root, emc,
    &tegra186_emc_debug_max_rate_fops);
    return 0;
    }
//
// tegra186_emc_icc_set_bw() - Set BW api for EMC provider
// @src: ICC node for External Memory Controller (EMC)
// @dst: ICC node for External Memory (DRAM)
//
// Do nothing here as info to BPMP-FW is now passed in the BW set function
// of the MC driver. BPMP-FW sets the final Freq based on the passed values.
//
#[no_mangle]
unsafe extern "C" fn tegra186_emc_icc_set_bw(src: *mut icc_node, dst: *mut icc_node) -> c_int {
    static int tegra186_emc_icc_set_bw(struct icc_node *src, struct icc_node *dst)
    {
    return 0;
    }
    static struct icc_node *
    tegra186_emc_of_icc_xlate(const struct of_phandle_args *spec, void *data)
    {
    struct icc_provider *provider = data;
    struct icc_node *node;
// External Memory is the only possible ICC route
    list_for_each_entry(node, &provider.nodes, node_list) {
    if (tegra_mc_client_id_from_node(node) != TEGRA_ICC_EMEM)
    continue;
    return node;
    }
    return ERR_PTR(-EPROBE_DEFER);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_emc_icc_get_init_bw(node: *mut icc_node, avg: *mut u32, peak: *mut u32) -> c_int {
    static int tegra186_emc_icc_get_init_bw(struct icc_node *node, u32 *avg, u32 *peak)
    {
// avg = 0;
// peak = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_emc_interconnect_init(emc: *mut tegra186_emc) -> c_int {
    static int tegra186_emc_interconnect_init(struct tegra186_emc *emc)
    {
    let mut node_id: c_int = dev_to_node(emc.dev.parent);
    struct icc_node *node;
    int err;
    emc.provider.dev = emc.dev;
    emc.provider.set = tegra186_emc_icc_set_bw;
    emc.provider.data = &emc.provider;
    emc.provider.aggregate = icc_std_aggregate;
    emc.provider.xlate = tegra186_emc_of_icc_xlate;
    emc.provider.get_bw = tegra186_emc_icc_get_init_bw;
    icc_provider_init(&emc.provider);
// create External Memory Controller node
    node = tegra_mc_icc_node_create(node_id, TEGRA_ICC_EMC);
    if (IS_ERR(node)) {
    err = PTR_ERR(node);
    goto remove_nodes;
    }
    if (node_id == NUMA_NO_NODE)
    node.name = "External Memory Controller";
    else
    node.name = dev_name(emc.dev);
    icc_node_add(node, &emc.provider);
// link External Memory Controller to External Memory (DRAM)
    err = tegra_mc_icc_link_create(node, node_id, TEGRA_ICC_EMEM);
    if (err)
    goto remove_nodes;
// create External Memory node
    node = tegra_mc_icc_node_create(node_id, TEGRA_ICC_EMEM);
    if (IS_ERR(node)) {
    err = PTR_ERR(node);
    goto remove_nodes;
    }
    if (node_id == NUMA_NO_NODE)
    node.name = "External Memory (DRAM)";
    else
    node.name = devm_kasprintf(emc.dev, GFP_KERNEL, "%d-dram", node_id);
    icc_node_add(node, &emc.provider);
    err = icc_provider_register(&emc.provider);
    if (err)
    goto remove_nodes;
    return 0;
    remove_nodes:
    icc_nodes_remove(&emc.provider);
    return dev_err_probe(emc.dev, err, "failed to initialize ICC\n");
    }
#[no_mangle]
unsafe extern "C" fn tegra186_emc_probe(pdev: *mut platform_device) -> c_int {
    static int tegra186_emc_probe(struct platform_device *pdev)
    {
    struct tegra_mc *mc = dev_get_drvdata(pdev.dev.parent);
    struct tegra186_emc *emc;
    int err;
    emc = devm_kzalloc(&pdev.dev, sizeof(*emc), GFP_KERNEL);
    if (!emc)
    return -ENOMEM;
    emc.bpmp = tegra_bpmp_get(&pdev.dev);
    if (IS_ERR(emc.bpmp))
    return dev_err_probe(&pdev.dev, PTR_ERR(emc.bpmp),
    "failed to get BPMP\n");
    emc.clk = devm_clk_get(&pdev.dev, "emc");
    if (IS_ERR(emc.clk)) {
    err = dev_err_probe(&pdev.dev, PTR_ERR(emc.clk),
    "failed to get EMC clock\n");
    goto put_bpmp;
    }
    emc.clk_dbb = devm_clk_get_optional_enabled(&pdev.dev, "dbb");
    if (IS_ERR(emc.clk_dbb)) {
    err = dev_err_probe(&pdev.dev, PTR_ERR(emc.clk_dbb),
    "failed to get DBB clock\n");
    goto put_bpmp;
    }
    platform_set_drvdata(pdev, emc);
    emc.dev = &pdev.dev;
    if (tegra_bpmp_mrq_is_supported(emc.bpmp, MRQ_EMC_DVFS_LATENCY)) {
    err = tegra186_emc_get_emc_dvfs_latency(emc);
    if (err)
    goto put_bpmp;
    }
    if (mc && mc.soc.icc_ops) {
    if (tegra_bpmp_mrq_is_supported(emc.bpmp, MRQ_BWMGR_INT)) {
    mc.bwmgr_mrq_supported = true;
//
// MC driver probe can't get BPMP reference as it gets probed
// earlier than BPMP. So, save the BPMP ref got from the EMC
// DT node in the mc->bpmp and use it in MC's icc_set hook.
//
    mc.bpmp = emc.bpmp;
    barrier();
    }
//
// Initialize the ICC even if BPMP-FW doesn't support 'MRQ_BWMGR_INT'.
// Use the flag 'mc->bwmgr_mrq_supported' within MC driver and return
// EINVAL instead of passing the request to BPMP-FW later when the BW
// request is made by client with 'icc_set_bw()' call.
//
    err = tegra186_emc_interconnect_init(emc);
    if (err) {
    mc.bpmp = core::ptr::null_mut();
    goto put_bpmp;
    }
    }
    return 0;
    put_bpmp:
    tegra_bpmp_put(emc.bpmp);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_emc_remove(pdev: *mut platform_device) {
    static void tegra186_emc_remove(struct platform_device *pdev)
    {
    struct tegra_mc *mc = dev_get_drvdata(pdev.dev.parent);
    struct tegra186_emc *emc = platform_get_drvdata(pdev);
    debugfs_remove_recursive(emc.debugfs.root);
    scoped_guard(mutex, &tegra_emc_debugfs_root_lock) {
    if (emc.debugfs.root == tegra_emc_debugfs_root) {
    tegra_emc_debugfs_root = core::ptr::null_mut();
    } else if (tegra_emc_debugfs_root &&
    simple_empty(tegra_emc_debugfs_root)) {
    debugfs_remove(tegra_emc_debugfs_root);
    tegra_emc_debugfs_root = core::ptr::null_mut();
    }
    }
    mc.bpmp = core::ptr::null_mut();
    tegra_bpmp_put(emc.bpmp);
    }
    static const struct of_device_id tegra186_emc_of_match[] = {

    { .compatible = "nvidia,tegra186-emc" },

    { .compatible = "nvidia,tegra194-emc" },

    { .compatible = "nvidia,tegra234-emc" },

    { .compatible = "nvidia,tegra264-emc" },

    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tegra186_emc_of_match);
    static struct platform_driver tegra186_emc_driver = {
    .driver = {
    .name = "tegra186-emc",
    .of_match_table = tegra186_emc_of_match,
    .suppress_bind_attrs = true,
    .sync_state = icc_sync_state,
    },
    .probe = tegra186_emc_probe,
    .remove = tegra186_emc_remove,
    };
    module_platform_driver(tegra186_emc_driver);
    MODULE_AUTHOR("Thierry Reding <treding@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra186 External Memory Controller driver");
