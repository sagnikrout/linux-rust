//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/sti-cpufreq.c
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
// Match running platform with pre-defined OPP values for CPUFreq
//
// Author: Ajit Pal Singh <ajitpal.singh@st.com>
// Lee Jones <lee.jones@linaro.org>
//
// Copyright (C) 2015 STMicroelectronics (R&D) Limited
//

pub const VERSION_ELEMENTS: c_int = 3;
pub const MAX_PCODE_NAME_LEN: c_int = 16;
pub const VERSION_SHIFT: c_int = 28;
pub const HW_INFO_INDEX: c_int = 1;
pub const MAJOR_ID_INDEX: c_int = 1;
pub const MINOR_ID_INDEX: c_int = 2;
//
// Only match on "suitable for ALL versions" entries
//
// This will be used with the BIT() macro.  It sets the
// top bit of a 32bit value and is equal to 0x80000000.
//
pub const DEFAULT_VERSION: c_int = 31;
    enum {
    PCODE = 0,
    SUBSTRATE,
    DVFS_MAX_REGFIELDS,
    };
//
// struct sti_cpufreq_ddata - ST CPUFreq Driver Data
//
// @cpu:		CPU's OF node
// @syscfg_eng:		Engineering Syscon register map
// @syscfg:		Syscon register map
//
    static struct sti_cpufreq_ddata {
    struct device *cpu;
    struct regmap *syscfg_eng;
    struct regmap *syscfg;
    } ddata;
#[no_mangle]
unsafe extern "C" fn sti_cpufreq_fetch_major() -> c_int {
    struct device_node *np = ddata.cpu.of_node;
    struct device *dev = ddata.cpu;
    unsigned int major_offset;
    unsigned int socid;
    int ret;
    ret = of_property_read_u32_index(np, "st,syscfg",
    MAJOR_ID_INDEX, &major_offset);
    if (ret) {
    dev_err(dev, "No major number offset provided in %pOF [%d]\n",
    np, ret);
    return ret;
    }
    ret = regmap_read(ddata.syscfg, major_offset, &socid);
    if (ret) {
    dev_err(dev, "Failed to read major number from syscon [%d]\n",
    ret);
    return ret;
    }
    return ((socid >> VERSION_SHIFT) & 0xf) + 1;
    }
#[no_mangle]
unsafe extern "C" fn sti_cpufreq_fetch_minor() -> c_int {
    static int sti_cpufreq_fetch_minor(void)
    {
    struct device *dev = ddata.cpu;
    struct device_node *np = dev.of_node;
    unsigned int minor_offset;
    unsigned int minid;
    int ret;
    ret = of_property_read_u32_index(np, "st,syscfg-eng",
    MINOR_ID_INDEX, &minor_offset);
    if (ret) {
    dev_err(dev,
    "No minor number offset provided %pOF [%d]\n",
    np, ret);
    return ret;
    }
    ret = regmap_read(ddata.syscfg_eng, minor_offset, &minid);
    if (ret) {
    dev_err(dev,
    "Failed to read the minor number from syscon [%d]\n",
    ret);
    return ret;
    }
    return minid & 0xf;
    }
    static int sti_cpufreq_fetch_regmap_field(const struct reg_field *reg_fields,
    int hw_info_offset, int field)
    {
    struct regmap_field *regmap_field;
    let mut reg_field: reg_field = reg_fields[field];
    struct device *dev = ddata.cpu;
    unsigned int value;
    int ret;
    reg_field.reg = hw_info_offset;
    regmap_field = devm_regmap_field_alloc(dev,
    ddata.syscfg_eng,
    reg_field);
    if (IS_ERR(regmap_field)) {
    dev_err(dev, "Failed to allocate reg field\n");
    return PTR_ERR(regmap_field);
    }
    ret = regmap_field_read(regmap_field, &value);
    if (ret) {
    dev_err(dev, "Failed to read %s code\n",
    field ? "SUBSTRATE" : "PCODE");
    return ret;
    }
    return value;
    }
    static const struct reg_field sti_stih407_dvfs_regfields[DVFS_MAX_REGFIELDS] = {
    [PCODE]		= REG_FIELD(0, 16, 19),
    [SUBSTRATE]	= REG_FIELD(0, 0, 2),
    };
    static const struct reg_field *sti_cpufreq_match(void)
    {
    if (of_machine_is_compatible("st,stih407") ||
    of_machine_is_compatible("st,stih410") ||
    of_machine_is_compatible("st,stih418"))
    return sti_stih407_dvfs_regfields;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sti_cpufreq_set_opp_info() -> c_int {
    static int sti_cpufreq_set_opp_info(void)
    {
    struct device *dev = ddata.cpu;
    struct device_node *np = dev.of_node;
    const struct reg_field *reg_fields;
    unsigned int hw_info_offset;
    unsigned int version[VERSION_ELEMENTS];
    int pcode, substrate, major, minor;
    int opp_token, ret;
    char name[MAX_PCODE_NAME_LEN];
    struct dev_pm_opp_config config = {
    .supported_hw = version,
    .supported_hw_count = ARRAY_SIZE(version),
    .prop_name = name,
    };
    reg_fields = sti_cpufreq_match();
    if (!reg_fields) {
    dev_err(dev, "This SoC doesn't support voltage scaling\n");
    return -ENODEV;
    }
    ret = of_property_read_u32_index(np, "st,syscfg-eng",
    HW_INFO_INDEX, &hw_info_offset);
    if (ret) {
    dev_warn(dev, "Failed to read HW info offset from DT\n");
    substrate = DEFAULT_VERSION;
    pcode = 0;
    goto use_defaults;
    }
    pcode = sti_cpufreq_fetch_regmap_field(reg_fields,
    hw_info_offset,
    PCODE);
    if (pcode < 0) {
    dev_warn(dev, "Failed to obtain process code\n");
// Use default pcode
    pcode = 0;
    }
    substrate = sti_cpufreq_fetch_regmap_field(reg_fields,
    hw_info_offset,
    SUBSTRATE);
    if (substrate) {
    dev_warn(dev, "Failed to obtain substrate code\n");
// Use default substrate
    substrate = DEFAULT_VERSION;
    }
    use_defaults:
    major = sti_cpufreq_fetch_major();
    if (major < 0) {
    dev_err(dev, "Failed to obtain major version\n");
// Use default major number
    major = DEFAULT_VERSION;
    }
    minor = sti_cpufreq_fetch_minor();
    if (minor < 0) {
    dev_err(dev, "Failed to obtain minor version\n");
// Use default minor number
    minor = DEFAULT_VERSION;
    }
    snprintf(name, MAX_PCODE_NAME_LEN, "pcode%d", pcode);
    version[0] = BIT(major);
    version[1] = BIT(minor);
    version[2] = BIT(substrate);
    opp_token = dev_pm_opp_set_config(dev, &config);
    if (opp_token < 0) {
    dev_err(dev, "Failed to set OPP config\n");
    return opp_token;
    }
    dev_dbg(dev, "pcode: %d major: %d minor: %d substrate: %d\n",
    pcode, major, minor, substrate);
    dev_dbg(dev, "version[0]: %x version[1]: %x version[2]: %x\n",
    version[0], version[1], version[2]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sti_cpufreq_fetch_syscon_registers() -> c_int {
    static int sti_cpufreq_fetch_syscon_registers(void)
    {
    struct device *dev = ddata.cpu;
    struct device_node *np = dev.of_node;
    ddata.syscfg = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(ddata.syscfg)) {
    dev_err(dev,  "\"st,syscfg\" not supplied\n");
    return PTR_ERR(ddata.syscfg);
    }
    ddata.syscfg_eng = syscon_regmap_lookup_by_phandle(np, "st,syscfg-eng");
    if (IS_ERR(ddata.syscfg_eng)) {
    dev_err(dev, "\"st,syscfg-eng\" not supplied\n");
    return PTR_ERR(ddata.syscfg_eng);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sti_cpufreq_init() -> int __init {
    static int __init sti_cpufreq_init(void)
    {
    int ret;
    if ((!of_machine_is_compatible("st,stih407")) &&
    (!of_machine_is_compatible("st,stih410")) &&
    (!of_machine_is_compatible("st,stih418")))
    return -ENODEV;
    ddata.cpu = get_cpu_device(0);
    if (!ddata.cpu) {
    dev_err(ddata.cpu, "Failed to get device for CPU0\n");
    goto skip_voltage_scaling;
    }
    if (!of_property_present(ddata.cpu.of_node, "operating-points-v2")) {
    dev_err(ddata.cpu, "OPP-v2 not supported\n");
    goto skip_voltage_scaling;
    }
    ret = sti_cpufreq_fetch_syscon_registers();
    if (ret)
    goto skip_voltage_scaling;
    ret = sti_cpufreq_set_opp_info();
    if (!ret)
    goto register_cpufreq_dt;
    skip_voltage_scaling:
    dev_err(ddata.cpu, "Not doing voltage scaling\n");
    register_cpufreq_dt:
    platform_device_register_simple("cpufreq-dt", -1, core::ptr::null_mut(), 0);
    return 0;
    }
    module_init(sti_cpufreq_init);
    static const struct of_device_id __maybe_unused sti_cpufreq_of_match[] = {
    { .compatible = "st,stih407" },
    { .compatible = "st,stih410" },
    { .compatible = "st,stih418" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sti_cpufreq_of_match);
    MODULE_DESCRIPTION("STMicroelectronics CPUFreq/OPP driver");
    MODULE_AUTHOR("Ajitpal Singh <ajitpal.singh@st.com>");
    MODULE_AUTHOR("Lee Jones <lee.jones@linaro.org>");
    MODULE_LICENSE("GPL v2");
