//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/apple-soc-cpufreq.c
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
// Apple SoC CPU cluster performance state driver
//
// Copyright The Asahi Linux Contributors
//
// Based on scpi-cpufreq.c
//

pub const APPLE_DVFS_CMD: c_uint = 0x20;

pub const APPLE_DVFS_CMD_PS1_S5L8960X_SHIFT: c_int = 22;

pub const APPLE_DVFS_CMD_PS1_SHIFT: c_int = 0;
// Same timebase as CPU counter (24MHz)
pub const APPLE_DVFS_LAST_CHG_TIME: c_uint = 0x38;
//
// Apple ran out of bits and had to shift this in T8112...
//
pub const APPLE_DVFS_STATUS: c_uint = 0x50;

pub const APPLE_DVFS_STATUS_CUR_PS_SHIFT_S5L8960X: c_int = 3;

pub const APPLE_DVFS_STATUS_CUR_PS_SHIFT_T8103: c_int = 4;

pub const APPLE_DVFS_STATUS_CUR_PS_SHIFT_T8112: c_int = 5;

//
// Div is +1, base clock is 12MHz on existing SoCs.
// For documentation purposes. We use the OPP table to
// get the frequency.
//
pub const APPLE_DVFS_PLL_STATUS: c_uint = 0xc0;
pub const APPLE_DVFS_PLL_FACTOR: c_uint = 0xc8;

pub const APPLE_DVFS_TRANSITION_TIMEOUT: c_int = 400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_soc_cpufreq_info {
    pub has_ps2: bool,
    pub max_pstate: u64,
    pub cur_pstate_mask: u64,
    pub cur_pstate_shift: u64,
    pub ps1_mask: u64,
    pub ps1_shift: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_cpu_priv {
    pub cpu_dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub info: *const apple_soc_cpufreq_info,
}

    static struct cpufreq_driver apple_soc_cpufreq_driver;
    static const struct apple_soc_cpufreq_info soc_s5l8960x_info = {
    .has_ps2 = false,
    .max_pstate = 7,
    .cur_pstate_mask = APPLE_DVFS_STATUS_CUR_PS_S5L8960X,
    .cur_pstate_shift = APPLE_DVFS_STATUS_CUR_PS_SHIFT_S5L8960X,
    .ps1_mask = APPLE_DVFS_CMD_PS1_S5L8960X,
    .ps1_shift = APPLE_DVFS_CMD_PS1_S5L8960X_SHIFT,
    };
    static const struct apple_soc_cpufreq_info soc_t8103_info = {
    .has_ps2 = true,
    .max_pstate = 15,
    .cur_pstate_mask = APPLE_DVFS_STATUS_CUR_PS_T8103,
    .cur_pstate_shift = APPLE_DVFS_STATUS_CUR_PS_SHIFT_T8103,
    .ps1_mask = APPLE_DVFS_CMD_PS1,
    .ps1_shift = APPLE_DVFS_CMD_PS1_SHIFT,
    };
    static const struct apple_soc_cpufreq_info soc_t8112_info = {
    .has_ps2 = false,
    .max_pstate = 31,
    .cur_pstate_mask = APPLE_DVFS_STATUS_CUR_PS_T8112,
    .cur_pstate_shift = APPLE_DVFS_STATUS_CUR_PS_SHIFT_T8112,
    .ps1_mask = APPLE_DVFS_CMD_PS1,
    .ps1_shift = APPLE_DVFS_CMD_PS1_SHIFT,
    };
    static const struct apple_soc_cpufreq_info soc_default_info = {
    .has_ps2 = false,
    .max_pstate = 15,
    .cur_pstate_mask = 0, /* fallback */
    .ps1_mask = APPLE_DVFS_CMD_PS1,
    .ps1_shift = APPLE_DVFS_CMD_PS1_SHIFT,
    };
    static const struct of_device_id apple_soc_cpufreq_of_match[] __maybe_unused = {
    {
    .compatible = "apple,s5l8960x-cluster-cpufreq",
    .data = &soc_s5l8960x_info,
    },
    {
    .compatible = "apple,t8103-cluster-cpufreq",
    .data = &soc_t8103_info,
    },
    {
    .compatible = "apple,t8112-cluster-cpufreq",
    .data = &soc_t8112_info,
    },
    {
    .compatible = "apple,cluster-cpufreq",
    .data = &soc_default_info,
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn apple_soc_cpufreq_get_rate(cpu: c_uint) -> c_uint {
    static unsigned int apple_soc_cpufreq_get_rate(unsigned int cpu)
    {
    struct cpufreq_policy *policy;
    struct apple_cpu_priv *priv;
    struct cpufreq_frequency_table *p;
    unsigned int pstate;
    policy = cpufreq_cpu_get_raw(cpu);
    if (unlikely(!policy))
    return 0;
    priv = policy.driver_data;
    if (priv.info.cur_pstate_mask) {
    let mut reg: u32 = readl_relaxed(priv.reg_base + APPLE_DVFS_STATUS);
    pstate = (reg & priv.info.cur_pstate_mask) >>  priv.info.cur_pstate_shift;
    } else {
//
// For the fallback case we might not know the layout of DVFS_STATUS,
// so just use the command register value (which ignores boost limitations).
//
    let mut reg: u64 = readq_relaxed(priv.reg_base + APPLE_DVFS_CMD);
    pstate = FIELD_GET(APPLE_DVFS_CMD_PS1, reg);
    }
    cpufreq_for_each_valid_entry(p, policy.freq_table)
    if (p.driver_data == pstate)
    return p.frequency;
    dev_err(priv.cpu_dev, "could not find frequency for pstate %d\n",
    pstate);
    return 0;
    }
    static int apple_soc_cpufreq_set_target(struct cpufreq_policy *policy,
    unsigned int index)
    {
    struct apple_cpu_priv *priv = policy.driver_data;
    let mut pstate: c_uint = policy.freq_table[index].driver_data;
    u64 reg;
// Fallback for newer SoCs
    if (index > priv.info.max_pstate)
    index = priv.info.max_pstate;
    if (readq_poll_timeout_atomic(priv.reg_base + APPLE_DVFS_CMD, reg,
    !(reg & APPLE_DVFS_CMD_BUSY), 2,
    APPLE_DVFS_TRANSITION_TIMEOUT)) {
    return -EIO;
    }
    reg &= ~priv.info.ps1_mask;
    reg |= pstate << priv.info.ps1_shift;
    if (priv.info.has_ps2)
    FIELD_MODIFY(APPLE_DVFS_CMD_PS2, &reg, pstate);
    reg |= APPLE_DVFS_CMD_SET;
    writeq_relaxed(reg, priv.reg_base + APPLE_DVFS_CMD);
    return 0;
    }
    static unsigned int apple_soc_cpufreq_fast_switch(struct cpufreq_policy *policy,
    unsigned int target_freq)
    {
    if (apple_soc_cpufreq_set_target(policy, policy.cached_resolved_idx) < 0)
    return 0;
    return policy.freq_table[policy.cached_resolved_idx].frequency;
    }
    static int apple_soc_cpufreq_find_cluster(struct cpufreq_policy *policy,
    void __iomem **reg_base,
    const struct apple_soc_cpufreq_info **info)
    {
    struct of_phandle_args args;
    const struct of_device_id *match;
    let mut ret: c_int = 0;
    ret = of_perf_domain_get_sharing_cpumask(policy.cpu, "performance-domains",
    "#performance-domain-cells",
    policy.cpus, &args);
    if (ret < 0)
    return ret;
    match = of_match_node(apple_soc_cpufreq_of_match, args.np);
    of_node_put(args.np);
    if (!match)
    return -ENODEV;
// info = match->data;
// reg_base = of_iomap(args.np, 0);
    if (!*reg_base)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_soc_cpufreq_init(policy: *mut cpufreq_policy) -> c_int {
    static int apple_soc_cpufreq_init(struct cpufreq_policy *policy)
    {
    int ret, i;
    unsigned int transition_latency;
    void __iomem *reg_base;
    struct device *cpu_dev;
    struct apple_cpu_priv *priv;
    const struct apple_soc_cpufreq_info *info;
    struct cpufreq_frequency_table *freq_table;
    cpu_dev = get_cpu_device(policy.cpu);
    if (!cpu_dev) {
    pr_err("failed to get cpu%d device\n", policy.cpu);
    return -ENODEV;
    }
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    ret = apple_soc_cpufreq_find_cluster(policy, &reg_base, &info);
    if (ret) {
    dev_err(cpu_dev, "%s: failed to get cluster info: %d\n", __func__, ret);
    goto out_free_priv;
    }
    ret = dev_pm_opp_of_cpumask_add_table(policy.cpus);
    if (ret < 0) {
    dev_err(cpu_dev, "%s: failed to add OPP table: %d\n", __func__, ret);
    goto out_iounmap;
    }
    ret = dev_pm_opp_get_opp_count(cpu_dev);
    if (ret <= 0) {
    dev_dbg(cpu_dev, "OPP table is not ready, deferring probe\n");
    ret = -EPROBE_DEFER;
    goto out_free_table;
    }
    ret = dev_pm_opp_init_cpufreq_table(cpu_dev, &freq_table);
    if (ret) {
    dev_err(cpu_dev, "failed to init cpufreq table: %d\n", ret);
    goto out_free_table;
    }
// Get OPP levels (p-state indexes) and stash them in driver_data
    for (i = 0; freq_table[i].frequency != CPUFREQ_TABLE_END; i++) {
    let mut rate: c_ulong = freq_table[i].frequency * 1000UL + 999;
    struct dev_pm_opp *opp = dev_pm_opp_find_freq_floor(cpu_dev, &rate);
    if (IS_ERR(opp)) {
    ret = PTR_ERR(opp);
    goto out_free_cpufreq_table;
    }
    freq_table[i].driver_data = dev_pm_opp_get_level(opp);
    dev_pm_opp_put(opp);
    }
    priv.cpu_dev = cpu_dev;
    priv.reg_base = reg_base;
    priv.info = info;
    policy.driver_data = priv;
    policy.freq_table = freq_table;
    transition_latency = dev_pm_opp_get_max_transition_latency(cpu_dev);
    if (!transition_latency)
    transition_latency = APPLE_DVFS_TRANSITION_TIMEOUT * NSEC_PER_USEC;
    policy.cpuinfo.transition_latency = transition_latency;
    policy.dvfs_possible_from_any_cpu = true;
    policy.fast_switch_possible = true;
    policy.suspend_freq = freq_table[0].frequency;
    return 0;
    out_free_cpufreq_table:
    dev_pm_opp_free_cpufreq_table(cpu_dev, &freq_table);
    out_free_table:
    dev_pm_opp_of_cpumask_remove_table(policy.cpus);
    out_iounmap:
    iounmap(reg_base);
    out_free_priv:
    kfree(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apple_soc_cpufreq_exit(policy: *mut cpufreq_policy) {
    static void apple_soc_cpufreq_exit(struct cpufreq_policy *policy)
    {
    struct apple_cpu_priv *priv = policy.driver_data;
    dev_pm_opp_free_cpufreq_table(priv.cpu_dev, &policy.freq_table);
    dev_pm_opp_of_cpumask_remove_table(policy.cpus);
    iounmap(priv.reg_base);
    kfree(priv);
    }
    static struct cpufreq_driver apple_soc_cpufreq_driver = {
    .name		= "apple-cpufreq",
    .flags		= CPUFREQ_HAVE_GOVERNOR_PER_POLICY |
    CPUFREQ_NEED_INITIAL_FREQ_CHECK | CPUFREQ_IS_COOLING_DEV,
    .verify		= cpufreq_generic_frequency_table_verify,
    .get		= apple_soc_cpufreq_get_rate,
    .init		= apple_soc_cpufreq_init,
    .exit		= apple_soc_cpufreq_exit,
    .target_index	= apple_soc_cpufreq_set_target,
    .fast_switch	= apple_soc_cpufreq_fast_switch,
    .register_em	= cpufreq_register_em_with_opp,
    .set_boost	= cpufreq_boost_set_sw,
    .suspend	= cpufreq_generic_suspend,
    };
#[no_mangle]
unsafe extern "C" fn apple_soc_cpufreq_module_init() -> int __init {
    static int __init apple_soc_cpufreq_module_init(void)
    {
    if (!of_machine_is_compatible("apple,arm-platform"))
    return -ENODEV;
    return cpufreq_register_driver(&apple_soc_cpufreq_driver);
    }
    module_init(apple_soc_cpufreq_module_init);
#[no_mangle]
unsafe extern "C" fn apple_soc_cpufreq_module_exit() -> void __exit {
    static void __exit apple_soc_cpufreq_module_exit(void)
    {
    cpufreq_unregister_driver(&apple_soc_cpufreq_driver);
    }
    module_exit(apple_soc_cpufreq_module_exit);
    MODULE_DEVICE_TABLE(of, apple_soc_cpufreq_of_match);
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
    MODULE_DESCRIPTION("Apple SoC CPU cluster DVFS driver");
    MODULE_LICENSE("GPL");
