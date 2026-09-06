//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/airoha-cpufreq.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_cpufreq_priv {
    pub opp_token: c_int,
    pub pd_list: *mut dev_pm_domain_list,
    pub cpufreq_dt: *mut platform_device,
}

    static struct platform_device *cpufreq_pdev;
// NOP function to disable OPP from setting clock
    static int airoha_cpufreq_config_clks_nop(struct device *dev,
    struct opp_table *opp_table,
    struct dev_pm_opp *opp,
    void *data, bool scaling_down)
    {
    return 0;
    }
    static const char * const airoha_cpufreq_clk_names[] = { "cpu", core::ptr::null_mut() };
    static const char * const airoha_cpufreq_pd_names[] = { "perf" };
#[no_mangle]
unsafe extern "C" fn airoha_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    static int airoha_cpufreq_probe(struct platform_device *pdev)
    {
    const struct dev_pm_domain_attach_data attach_data = {
    .pd_names = airoha_cpufreq_pd_names,
    .num_pd_names = ARRAY_SIZE(airoha_cpufreq_pd_names),
    .pd_flags = PD_FLAG_DEV_LINK_ON | PD_FLAG_REQUIRED_OPP,
    };
    struct dev_pm_opp_config config = {
    .clk_names = airoha_cpufreq_clk_names,
    .config_clks = airoha_cpufreq_config_clks_nop,
    };
    struct platform_device *cpufreq_dt;
    struct airoha_cpufreq_priv *priv;
    struct device *dev = &pdev.dev;
    struct device *cpu_dev;
    int ret;
// CPUs refer to the same OPP table
    cpu_dev = get_cpu_device(0);
    if (!cpu_dev)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// Set OPP table conf with NOP config_clks
    priv.opp_token = dev_pm_opp_set_config(cpu_dev, &config);
    if (priv.opp_token < 0)
    return dev_err_probe(dev, priv.opp_token, "Failed to set OPP config\n");
// Attach PM for OPP
    ret = dev_pm_domain_attach_list(cpu_dev, &attach_data,
    &priv.pd_list);
    if (ret)
    goto clear_opp_config;
    cpufreq_dt = platform_device_register_simple("cpufreq-dt", -1, core::ptr::null_mut(), 0);
    ret = PTR_ERR_OR_ZERO(cpufreq_dt);
    if (ret) {
    dev_err(dev, "failed to create cpufreq-dt device: %d\n", ret);
    goto detach_pm;
    }
    priv.cpufreq_dt = cpufreq_dt;
    platform_set_drvdata(pdev, priv);
    return 0;
    detach_pm:
    dev_pm_domain_detach_list(priv.pd_list);
    clear_opp_config:
    dev_pm_opp_clear_config(priv.opp_token);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn airoha_cpufreq_remove(pdev: *mut platform_device) {
    static void airoha_cpufreq_remove(struct platform_device *pdev)
    {
    struct airoha_cpufreq_priv *priv = platform_get_drvdata(pdev);
    platform_device_unregister(priv.cpufreq_dt);
    dev_pm_domain_detach_list(priv.pd_list);
    dev_pm_opp_clear_config(priv.opp_token);
    }
    static struct platform_driver airoha_cpufreq_driver = {
    .probe = airoha_cpufreq_probe,
    .remove = airoha_cpufreq_remove,
    .driver = {
    .name = "airoha-cpufreq",
    },
    };
    static const struct of_device_id airoha_cpufreq_match_list[] __initconst = {
    { .compatible = "airoha,an7583" },
    { .compatible = "airoha,en7581" },
    {},
    };
    MODULE_DEVICE_TABLE(of, airoha_cpufreq_match_list);
#[no_mangle]
unsafe extern "C" fn airoha_cpufreq_init() -> int __init {
    static int __init airoha_cpufreq_init(void)
    {
    const struct of_device_id *match;
    int ret;
    match = of_machine_get_match(airoha_cpufreq_match_list);
    if (!match)
    return -ENODEV;
    ret = platform_driver_register(&airoha_cpufreq_driver);
    if (unlikely(ret < 0))
    return ret;
    cpufreq_pdev = platform_device_register_data(core::ptr::null_mut(), "airoha-cpufreq",
    -1, match, sizeof(*match));
    ret = PTR_ERR_OR_ZERO(cpufreq_pdev);
    if (ret)
    platform_driver_unregister(&airoha_cpufreq_driver);
    return ret;
    }
    module_init(airoha_cpufreq_init);
#[no_mangle]
unsafe extern "C" fn airoha_cpufreq_exit() -> void __exit {
    static void __exit airoha_cpufreq_exit(void)
    {
    platform_device_unregister(cpufreq_pdev);
    platform_driver_unregister(&airoha_cpufreq_driver);
    }
    module_exit(airoha_cpufreq_exit);
    MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
    MODULE_DESCRIPTION("CPUfreq driver for Airoha SoCs");
    MODULE_LICENSE("GPL");
