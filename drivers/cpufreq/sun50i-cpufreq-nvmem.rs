//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/sun50i-cpufreq-nvmem.c
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
// Allwinner CPUFreq nvmem based driver
//
// The sun50i-cpufreq-nvmem driver reads the efuse value from the SoC to
// provide the OPP framework with required information.
//
// Copyright (C) 2019 Yangtao Li <tiny.windzz@gmail.com>
//

pub const NVMEM_MASK: c_uint = 0x7;
pub const NVMEM_SHIFT: c_int = 5;
pub const SUN50I_A100_NVMEM_MASK: c_uint = 0xf;
pub const SUN50I_A100_NVMEM_SHIFT: c_int = 12;
    static struct platform_device *cpufreq_dt_pdev, *sun50i_cpufreq_pdev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_cpufreq_data {
    pub speedbin): *mut *mut u32 (efuse_xlate)(u32,
}

#[no_mangle]
unsafe extern "C" fn sun50i_h6_efuse_xlate(speedbin: u32) -> u32 {
    static u32 sun50i_h6_efuse_xlate(u32 speedbin)
    {
    u32 efuse_value;
    efuse_value = (speedbin >> NVMEM_SHIFT) & NVMEM_MASK;
//
// We treat unexpected efuse values as if the SoC was from
// the slowest bin. Expected efuse values are 1-3, slowest
// to fastest.
//
    if (efuse_value >= 1 && efuse_value <= 3)
    return efuse_value - 1;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_a100_efuse_xlate(speedbin: u32) -> u32 {
    static u32 sun50i_a100_efuse_xlate(u32 speedbin)
    {
    u32 efuse_value;
    efuse_value = (speedbin >> SUN50I_A100_NVMEM_SHIFT) &
    SUN50I_A100_NVMEM_MASK;
    switch (efuse_value) {
    case 0b100:
    return 2;
    case 0b010:
    return 1;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn get_soc_id_revision() -> c_int {
    static int get_soc_id_revision(void)
    {

    return arm_smccc_get_soc_id_revision();

    return SMCCC_RET_NOT_SUPPORTED;

    }
//
// Judging by the OPP tables in the vendor BSP, the quality order of the
// returned speedbin index is 4 -> 0/2 -> 3 -> 1, from worst to best.
// 0 and 2 seem identical from the OPP tables' point of view.
//
#[no_mangle]
unsafe extern "C" fn sun50i_h616_efuse_xlate(speedbin: u32) -> u32 {
    static u32 sun50i_h616_efuse_xlate(u32 speedbin)
    {
    let mut ver_bits: c_int = get_soc_id_revision();
    let mut value: u32 = 0;
    switch (speedbin & 0xffff) {
    case 0x2000:
    value = 0;
    break;
    case 0x2400:
    case 0x7400:
    case 0x2c00:
    case 0x7c00:
    if (ver_bits != SMCCC_RET_NOT_SUPPORTED && ver_bits <= 1) {
// ic version A/B
    value = 1;
    } else {
// ic version C and later version
    value = 2;
    }
    break;
    case 0x5000:
    case 0x5400:
    case 0x6000:
    value = 3;
    break;
    case 0x5c00:
    value = 4;
    break;
    case 0x5d00:
    value = 0;
    break;
    case 0x6c00:
    value = 5;
    break;
    default:
    pr_warn("sun50i-cpufreq-nvmem: unknown speed bin 0x%x, using default bin 0\n",
    speedbin & 0xffff);
    value = 0;
    break;
    }
    return value;
    }
    static struct sunxi_cpufreq_data sun50i_h6_cpufreq_data = {
    .efuse_xlate = sun50i_h6_efuse_xlate,
    };
    static struct sunxi_cpufreq_data sun50i_a100_cpufreq_data = {
    .efuse_xlate = sun50i_a100_efuse_xlate,
    };
    static struct sunxi_cpufreq_data sun50i_h616_cpufreq_data = {
    .efuse_xlate = sun50i_h616_efuse_xlate,
    };
    static const struct of_device_id cpu_opp_match_list[] = {
    { .compatible = "allwinner,sun50i-h6-operating-points",
    .data = &sun50i_h6_cpufreq_data,
    },
    { .compatible = "allwinner,sun50i-a100-operating-points",
    .data = &sun50i_a100_cpufreq_data,
    },
    { .compatible = "allwinner,sun50i-h616-operating-points",
    .data = &sun50i_h616_cpufreq_data,
    },
    {}
    };
//
// dt_has_supported_hw() - Check if any OPPs use opp-supported-hw
//
// If we ask the cpufreq framework to use the opp-supported-hw feature, it
// will ignore every OPP node without that DT property. If none of the OPPs
// have it, the driver will fail probing, due to the lack of OPPs.
//
// Returns true if we have at least one OPP with the opp-supported-hw property.
//
#[no_mangle]
unsafe extern "C" fn dt_has_supported_hw() -> bool {
    static bool dt_has_supported_hw(void)
    {
    let mut has_opp_supported_hw: bool = false;
    struct device *cpu_dev;
    cpu_dev = get_cpu_device(0);
    if (!cpu_dev)
    return false;
    struct device_node *np __free(device_node) =
    dev_pm_opp_of_get_opp_desc_node(cpu_dev);
    if (!np)
    return false;
    for_each_child_of_node_scoped(np, opp) {
    if (of_property_present(opp, "opp-supported-hw")) {
    has_opp_supported_hw = true;
    break;
    }
    }
    return has_opp_supported_hw;
    }
//
// sun50i_cpufreq_get_efuse() - Determine speed grade from efuse value
//
// Returns non-negative speed bin index on success, a negative error
// value otherwise.
//
#[no_mangle]
unsafe extern "C" fn sun50i_cpufreq_get_efuse() -> c_int {
    static int sun50i_cpufreq_get_efuse(void)
    {
    const struct sunxi_cpufreq_data *opp_data;
    struct nvmem_cell *speedbin_nvmem;
    const struct of_device_id *match;
    struct device *cpu_dev;
    void *speedbin_ptr;
    let mut speedbin: u32 = 0;
    size_t len;
    int ret;
    cpu_dev = get_cpu_device(0);
    if (!cpu_dev)
    return -ENODEV;
    struct device_node *np __free(device_node) =
    dev_pm_opp_of_get_opp_desc_node(cpu_dev);
    if (!np)
    return -ENOENT;
    match = of_match_node(cpu_opp_match_list, np);
    if (!match)
    return -ENOENT;
    opp_data = match.data;
    speedbin_nvmem = of_nvmem_cell_get(np, core::ptr::null_mut());
    if (IS_ERR(speedbin_nvmem))
    return dev_err_probe(cpu_dev, PTR_ERR(speedbin_nvmem),
    "Could not get nvmem cell\n");
    speedbin_ptr = nvmem_cell_read(speedbin_nvmem, &len);
    nvmem_cell_put(speedbin_nvmem);
    if (IS_ERR(speedbin_ptr))
    return PTR_ERR(speedbin_ptr);
    if (len <= 4)
    memcpy(&speedbin, speedbin_ptr, len);
    speedbin = le32_to_cpu(speedbin);
    ret = opp_data.efuse_xlate(speedbin);
    kfree(speedbin_ptr);
    return ret;
    };
#[no_mangle]
unsafe extern "C" fn sun50i_cpufreq_nvmem_probe(pdev: *mut platform_device) -> c_int {
    static int sun50i_cpufreq_nvmem_probe(struct platform_device *pdev)
    {
    int *opp_tokens;
    char name[] = "speedXXXXXXXXXXX"; /* Integers can take 11 chars max */
    unsigned int cpu, supported_hw;
    let mut config: dev_pm_opp_config = {};
    int speed;
    int ret;
    opp_tokens = kzalloc_objs(*opp_tokens, num_possible_cpus());
    if (!opp_tokens)
    return -ENOMEM;
    speed = sun50i_cpufreq_get_efuse();
    if (speed < 0) {
    kfree(opp_tokens);
    return speed;
    }
//
// We need at least one OPP with the "opp-supported-hw" property,
// or else the upper layers will ignore every OPP and will bail out.
//
    if (dt_has_supported_hw()) {
    supported_hw = 1U << speed;
    config.supported_hw = &supported_hw;
    config.supported_hw_count = 1;
    }
    snprintf(name, sizeof(name), "speed%d", speed);
    config.prop_name = name;
    for_each_present_cpu(cpu) {
    struct device *cpu_dev = get_cpu_device(cpu);
    if (!cpu_dev) {
    ret = -ENODEV;
    goto free_opp;
    }
    ret = dev_pm_opp_set_config(cpu_dev, &config);
    if (ret < 0)
    goto free_opp;
    opp_tokens[cpu] = ret;
    }
    cpufreq_dt_pdev = platform_device_register_simple("cpufreq-dt", -1,
    core::ptr::null_mut(), 0);
    if (!IS_ERR(cpufreq_dt_pdev)) {
    platform_set_drvdata(pdev, opp_tokens);
    return 0;
    }
    ret = PTR_ERR(cpufreq_dt_pdev);
    pr_err("Failed to register platform device\n");
    free_opp:
    for_each_present_cpu(cpu)
    dev_pm_opp_clear_config(opp_tokens[cpu]);
    kfree(opp_tokens);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_cpufreq_nvmem_remove(pdev: *mut platform_device) {
    static void sun50i_cpufreq_nvmem_remove(struct platform_device *pdev)
    {
    int *opp_tokens = platform_get_drvdata(pdev);
    unsigned int cpu;
    platform_device_unregister(cpufreq_dt_pdev);
    for_each_present_cpu(cpu)
    dev_pm_opp_clear_config(opp_tokens[cpu]);
    kfree(opp_tokens);
    }
    static struct platform_driver sun50i_cpufreq_driver = {
    .probe = sun50i_cpufreq_nvmem_probe,
    .remove = sun50i_cpufreq_nvmem_remove,
    .driver = {
    .name = "sun50i-cpufreq-nvmem",
    },
    };
    static const struct of_device_id sun50i_cpufreq_match_list[] = {
    { .compatible = "allwinner,sun50i-h6" },
    { .compatible = "allwinner,sun50i-a100" },
    { .compatible = "allwinner,sun50i-h616" },
    { .compatible = "allwinner,sun50i-h618" },
    { .compatible = "allwinner,sun50i-h700" },
    {}
    };
    MODULE_DEVICE_TABLE(of, sun50i_cpufreq_match_list);
//
// Since the driver depends on nvmem drivers, which may return EPROBE_DEFER,
// all the real activity is done in the probe, which may be defered as well.
// The init here is only registering the driver and the platform device.
//
#[no_mangle]
unsafe extern "C" fn sun50i_cpufreq_init() -> int __init {
    static int __init sun50i_cpufreq_init(void)
    {
    int ret;
    if (!of_machine_device_match(sun50i_cpufreq_match_list))
    return -ENODEV;
    ret = platform_driver_register(&sun50i_cpufreq_driver);
    if (unlikely(ret < 0))
    return ret;
    sun50i_cpufreq_pdev =
    platform_device_register_simple("sun50i-cpufreq-nvmem",
    -1, core::ptr::null_mut(), 0);
    ret = PTR_ERR_OR_ZERO(sun50i_cpufreq_pdev);
    if (ret == 0)
    return 0;
    platform_driver_unregister(&sun50i_cpufreq_driver);
    return ret;
    }
    module_init(sun50i_cpufreq_init);
#[no_mangle]
unsafe extern "C" fn sun50i_cpufreq_exit() -> void __exit {
    static void __exit sun50i_cpufreq_exit(void)
    {
    platform_device_unregister(sun50i_cpufreq_pdev);
    platform_driver_unregister(&sun50i_cpufreq_driver);
    }
    module_exit(sun50i_cpufreq_exit);
    MODULE_DESCRIPTION("Sun50i-h6 cpufreq driver");
    MODULE_LICENSE("GPL v2");
