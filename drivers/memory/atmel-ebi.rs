//! Automatically rewritten from C to Rust
//! Source: drivers/memory/atmel-ebi.c
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
// EBI driver for Atmel chips
// inspired by the fsl weim bus driver
//
// Copyright (C) 2013 Jean-Jacques Hiblot <jjhiblot@traphandler.com>
//

pub const AT91_EBI_NUM_CS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ebi_dev_config {
    pub cs: c_int,
    pub smcconf: atmel_smc_cs_conf,
}

    struct atmel_ebi;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ebi_dev {
    pub node: list_head,
    pub ebi: *mut atmel_ebi,
    pub mode: u32,
    pub numcs: c_int,
    pub __counted_by(numcs): atmel_ebi_dev_config configs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ebi_caps {
    pub available_cs: c_uint,
    pub ebi_csa_offs: c_uint,
    pub regmap_name: *const c_char,
    void (*get_config)(struct atmel_ebi_dev *ebid,
    pub conf): *mut atmel_ebi_dev_config,
    int (*xlate_config)(struct atmel_ebi_dev *ebid,
    struct device_node *configs_np,
    pub conf): *mut atmel_ebi_dev_config,
    void (*apply_config)(struct atmel_ebi_dev *ebid,
    pub conf): *mut atmel_ebi_dev_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ebi {
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    struct  {
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub layout: *const atmel_hsmc_reg_layout,
    pub smc: },
    pub dev: *mut device,
    pub caps: *const atmel_ebi_caps,
    pub devs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_smc_timing_xlate {
    pub name: *const c_char,
    int (*converter)(struct atmel_smc_cs_conf *conf,
    pub nycles): unsigned int shift, unsigned int,
    pub shift: c_uint,
}

    { .name = nm, .converter = atmel_smc_cs_conf_set_setup, .shift = pos}

    { .name = nm, .converter = atmel_smc_cs_conf_set_pulse, .shift = pos}

    { .name = nm, .converter = atmel_smc_cs_conf_set_cycle, .shift = pos}
    static void at91sam9_ebi_get_config(struct atmel_ebi_dev *ebid,
    struct atmel_ebi_dev_config *conf)
    {
    atmel_smc_cs_conf_get(ebid.ebi.smc.regmap, conf.cs,
    &conf.smcconf);
    }
    static void sama5_ebi_get_config(struct atmel_ebi_dev *ebid,
    struct atmel_ebi_dev_config *conf)
    {
    atmel_hsmc_cs_conf_get(ebid.ebi.smc.regmap, ebid.ebi.smc.layout,
    conf.cs, &conf.smcconf);
    }
    static const struct atmel_smc_timing_xlate timings_xlate_table[] = {
    ATMEL_SMC_SETUP_XLATE("atmel,smc-ncs-rd-setup-ns",
    ATMEL_SMC_NCS_RD_SHIFT),
    ATMEL_SMC_SETUP_XLATE("atmel,smc-ncs-wr-setup-ns",
    ATMEL_SMC_NCS_WR_SHIFT),
    ATMEL_SMC_SETUP_XLATE("atmel,smc-nrd-setup-ns", ATMEL_SMC_NRD_SHIFT),
    ATMEL_SMC_SETUP_XLATE("atmel,smc-nwe-setup-ns", ATMEL_SMC_NWE_SHIFT),
    ATMEL_SMC_PULSE_XLATE("atmel,smc-ncs-rd-pulse-ns",
    ATMEL_SMC_NCS_RD_SHIFT),
    ATMEL_SMC_PULSE_XLATE("atmel,smc-ncs-wr-pulse-ns",
    ATMEL_SMC_NCS_WR_SHIFT),
    ATMEL_SMC_PULSE_XLATE("atmel,smc-nrd-pulse-ns", ATMEL_SMC_NRD_SHIFT),
    ATMEL_SMC_PULSE_XLATE("atmel,smc-nwe-pulse-ns", ATMEL_SMC_NWE_SHIFT),
    ATMEL_SMC_CYCLE_XLATE("atmel,smc-nrd-cycle-ns", ATMEL_SMC_NRD_SHIFT),
    ATMEL_SMC_CYCLE_XLATE("atmel,smc-nwe-cycle-ns", ATMEL_SMC_NWE_SHIFT),
    };
    static int atmel_ebi_xslate_smc_timings(struct atmel_ebi_dev *ebid,
    struct device_node *np,
    struct atmel_smc_cs_conf *smcconf)
    {
    let mut clk_rate: c_uint = clk_get_rate(ebid.ebi.clk);
    let mut clk_period_ns: c_uint = NSEC_PER_SEC / clk_rate;
    let mut required: bool = false;
    unsigned int ncycles;
    int ret, i;
    u32 val;
    ret = of_property_read_u32(np, "atmel,smc-tdf-ns", &val);
    if (!ret) {
    required = true;
    ncycles = DIV_ROUND_UP(val, clk_period_ns);
    if (ncycles > ATMEL_SMC_MODE_TDF_MAX) {
    ret = -EINVAL;
    goto out;
    }
    if (ncycles < ATMEL_SMC_MODE_TDF_MIN)
    ncycles = ATMEL_SMC_MODE_TDF_MIN;
    smcconf.mode |= ATMEL_SMC_MODE_TDF(ncycles);
    }
    for (i = 0; i < ARRAY_SIZE(timings_xlate_table); i++) {
    const struct atmel_smc_timing_xlate *xlate;
    xlate = &timings_xlate_table[i];
    ret = of_property_read_u32(np, xlate.name, &val);
    if (ret) {
    if (!required)
    continue;
    else
    break;
    }
    if (!required) {
    ret = -EINVAL;
    break;
    }
    ncycles = DIV_ROUND_UP(val, clk_period_ns);
    ret = xlate.converter(smcconf, xlate.shift, ncycles);
    if (ret)
    goto out;
    }
    out:
    if (ret) {
    dev_err(ebid.ebi.dev,
    "missing or invalid timings definition in %pOF",
    np);
    return ret;
    }
    return required;
    }
    static int atmel_ebi_xslate_smc_config(struct atmel_ebi_dev *ebid,
    struct device_node *np,
    struct atmel_ebi_dev_config *conf)
    {
    struct atmel_smc_cs_conf *smcconf = &conf.smcconf;
    let mut required: bool = false;
    const char *tmp_str;
    u32 tmp;
    int ret;
    ret = of_property_read_u32(np, "atmel,smc-bus-width", &tmp);
    if (!ret) {
    switch (tmp) {
    case 8:
    smcconf.mode |= ATMEL_SMC_MODE_DBW_8;
    break;
    case 16:
    smcconf.mode |= ATMEL_SMC_MODE_DBW_16;
    break;
    case 32:
    smcconf.mode |= ATMEL_SMC_MODE_DBW_32;
    break;
    default:
    return -EINVAL;
    }
    required = true;
    }
    if (of_property_read_bool(np, "atmel,smc-tdf-optimized")) {
    smcconf.mode |= ATMEL_SMC_MODE_TDFMODE_OPTIMIZED;
    required = true;
    }
    tmp_str = core::ptr::null_mut();
    of_property_read_string(np, "atmel,smc-byte-access-type", &tmp_str);
    if (tmp_str && !strcmp(tmp_str, "write")) {
    smcconf.mode |= ATMEL_SMC_MODE_BAT_WRITE;
    required = true;
    }
    tmp_str = core::ptr::null_mut();
    of_property_read_string(np, "atmel,smc-read-mode", &tmp_str);
    if (tmp_str && !strcmp(tmp_str, "nrd")) {
    smcconf.mode |= ATMEL_SMC_MODE_READMODE_NRD;
    required = true;
    }
    tmp_str = core::ptr::null_mut();
    of_property_read_string(np, "atmel,smc-write-mode", &tmp_str);
    if (tmp_str && !strcmp(tmp_str, "nwe")) {
    smcconf.mode |= ATMEL_SMC_MODE_WRITEMODE_NWE;
    required = true;
    }
    tmp_str = core::ptr::null_mut();
    of_property_read_string(np, "atmel,smc-exnw-mode", &tmp_str);
    if (tmp_str) {
    if (!strcmp(tmp_str, "frozen"))
    smcconf.mode |= ATMEL_SMC_MODE_EXNWMODE_FROZEN;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tmp_str, _arg: "ready")) -> else {
    else if (!strcmp(tmp_str, "ready"))
    smcconf.mode |= ATMEL_SMC_MODE_EXNWMODE_READY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(tmp_str, _arg: "disabled")) -> else {
    else if (strcmp(tmp_str, "disabled"))
    return -EINVAL;
    required = true;
    }
    ret = of_property_read_u32(np, "atmel,smc-page-mode", &tmp);
    if (!ret) {
    switch (tmp) {
    case 4:
    smcconf.mode |= ATMEL_SMC_MODE_PS_4;
    break;
    case 8:
    smcconf.mode |= ATMEL_SMC_MODE_PS_8;
    break;
    case 16:
    smcconf.mode |= ATMEL_SMC_MODE_PS_16;
    break;
    case 32:
    smcconf.mode |= ATMEL_SMC_MODE_PS_32;
    break;
    default:
    return -EINVAL;
    }
    smcconf.mode |= ATMEL_SMC_MODE_PMEN;
    required = true;
    }
    ret = atmel_ebi_xslate_smc_timings(ebid, np, &conf.smcconf);
    if (ret < 0)
    return -EINVAL;
    if ((ret > 0 && !required) || (!ret && required)) {
    dev_err(ebid.ebi.dev, "missing atmel,smc- properties in %pOF",
    np);
    return -EINVAL;
    }
    return required;
    }
    static void at91sam9_ebi_apply_config(struct atmel_ebi_dev *ebid,
    struct atmel_ebi_dev_config *conf)
    {
    atmel_smc_cs_conf_apply(ebid.ebi.smc.regmap, conf.cs,
    &conf.smcconf);
    }
    static void sama5_ebi_apply_config(struct atmel_ebi_dev *ebid,
    struct atmel_ebi_dev_config *conf)
    {
    atmel_hsmc_cs_conf_apply(ebid.ebi.smc.regmap, ebid.ebi.smc.layout,
    conf.cs, &conf.smcconf);
    }
    static int atmel_ebi_dev_setup(struct atmel_ebi *ebi, struct device_node *np,
    int reg_cells)
    {
    const struct atmel_ebi_caps *caps = ebi.caps;
    let mut conf: atmel_ebi_dev_config = { };
    struct device *dev = ebi.dev;
    struct atmel_ebi_dev *ebid;
    let mut cslines: c_ulong = 0;
    int ret, numcs = 0, nentries, i;
    let mut apply: bool = false;
    u32 cs;
    nentries = of_property_count_elems_of_size(np, "reg",
    reg_cells * sizeof(u32));
    for (i = 0; i < nentries; i++) {
    ret = of_property_read_u32_index(np, "reg", i * reg_cells,
    &cs);
    if (ret)
    return ret;
    if (cs >= AT91_EBI_NUM_CS ||
    !(ebi.caps.available_cs & BIT(cs))) {
    dev_err(dev, "invalid reg property in %pOF\n", np);
    return -EINVAL;
    }
    if (!test_and_set_bit(cs, &cslines))
    numcs++;
    }
    if (!numcs) {
    dev_err(dev, "invalid reg property in %pOF\n", np);
    return -EINVAL;
    }
    ebid = devm_kzalloc(ebi.dev, struct_size(ebid, configs, numcs),
    GFP_KERNEL);
    if (!ebid)
    return -ENOMEM;
    ebid.ebi = ebi;
    ebid.numcs = numcs;
    ret = caps.xlate_config(ebid, np, &conf);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    apply = true;
    i = 0;
    for_each_set_bit(cs, &cslines, AT91_EBI_NUM_CS) {
    ebid.configs[i].cs = cs;
    if (apply) {
    conf.cs = cs;
    caps.apply_config(ebid, &conf);
    }
    caps.get_config(ebid, &ebid.configs[i]);
//
// Attach the EBI device to the generic SMC logic if at least
// one "atmel,smc-" property is present.
//
    if (ebi.caps.ebi_csa_offs && apply)
    regmap_update_bits(ebi.regmap,
    ebi.caps.ebi_csa_offs,
    BIT(cs), 0);
    i++;
    }
    list_add_tail(&ebid.node, &ebi.devs);
    return 0;
    }
    static const struct atmel_ebi_caps at91sam9260_ebi_caps = {
    .available_cs = 0xff,
    .ebi_csa_offs = AT91SAM9260_MATRIX_EBICSA,
    .regmap_name = "atmel,matrix",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct atmel_ebi_caps at91sam9261_ebi_caps = {
    .available_cs = 0xff,
    .ebi_csa_offs = AT91SAM9261_MATRIX_EBICSA,
    .regmap_name = "atmel,matrix",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct atmel_ebi_caps at91sam9263_ebi0_caps = {
    .available_cs = 0x3f,
    .ebi_csa_offs = AT91SAM9263_MATRIX_EBI0CSA,
    .regmap_name = "atmel,matrix",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct atmel_ebi_caps at91sam9263_ebi1_caps = {
    .available_cs = 0x7,
    .ebi_csa_offs = AT91SAM9263_MATRIX_EBI1CSA,
    .regmap_name = "atmel,matrix",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct atmel_ebi_caps at91sam9rl_ebi_caps = {
    .available_cs = 0x3f,
    .ebi_csa_offs = AT91SAM9RL_MATRIX_EBICSA,
    .regmap_name = "atmel,matrix",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct atmel_ebi_caps at91sam9g45_ebi_caps = {
    .available_cs = 0x3f,
    .ebi_csa_offs = AT91SAM9G45_MATRIX_EBICSA,
    .regmap_name = "atmel,matrix",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct atmel_ebi_caps at91sam9x5_ebi_caps = {
    .available_cs = 0x3f,
    .ebi_csa_offs = AT91SAM9X5_MATRIX_EBICSA,
    .regmap_name = "atmel,matrix",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct atmel_ebi_caps sama5d3_ebi_caps = {
    .available_cs = 0xf,
    .get_config = sama5_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = sama5_ebi_apply_config,
    };
    static const struct atmel_ebi_caps sam9x60_ebi_caps = {
    .available_cs = 0x3f,
    .ebi_csa_offs = AT91_SFR_CCFG_EBICSA,
    .regmap_name = "microchip,sfr",
    .get_config = at91sam9_ebi_get_config,
    .xlate_config = atmel_ebi_xslate_smc_config,
    .apply_config = at91sam9_ebi_apply_config,
    };
    static const struct of_device_id atmel_ebi_id_table[] = {
    {
    .compatible = "atmel,at91sam9260-ebi",
    .data = &at91sam9260_ebi_caps,
    },
    {
    .compatible = "atmel,at91sam9261-ebi",
    .data = &at91sam9261_ebi_caps,
    },
    {
    .compatible = "atmel,at91sam9263-ebi0",
    .data = &at91sam9263_ebi0_caps,
    },
    {
    .compatible = "atmel,at91sam9263-ebi1",
    .data = &at91sam9263_ebi1_caps,
    },
    {
    .compatible = "atmel,at91sam9rl-ebi",
    .data = &at91sam9rl_ebi_caps,
    },
    {
    .compatible = "atmel,at91sam9g45-ebi",
    .data = &at91sam9g45_ebi_caps,
    },
    {
    .compatible = "atmel,at91sam9x5-ebi",
    .data = &at91sam9x5_ebi_caps,
    },
    {
    .compatible = "atmel,sama5d3-ebi",
    .data = &sama5d3_ebi_caps,
    },
    {
    .compatible = "microchip,sam9x60-ebi",
    .data = &sam9x60_ebi_caps,
    },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn atmel_ebi_dev_disable(ebi: *mut atmel_ebi, np: *mut device_node) -> c_int {
    static int atmel_ebi_dev_disable(struct atmel_ebi *ebi, struct device_node *np)
    {
    struct device *dev = ebi.dev;
    struct property *newprop;
    newprop = devm_kzalloc(dev, sizeof(*newprop), GFP_KERNEL);
    if (!newprop)
    return -ENOMEM;
    newprop.name = devm_kstrdup(dev, "status", GFP_KERNEL);
    if (!newprop.name)
    return -ENOMEM;
    newprop.value = devm_kstrdup(dev, "disabled", GFP_KERNEL);
    if (!newprop.value)
    return -ENOMEM;
    newprop.length = sizeof("disabled");
    return of_update_property(np, newprop);
    }
#[no_mangle]
unsafe extern "C" fn atmel_ebi_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_ebi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct atmel_ebi *ebi;
    int ret, reg_cells;
    struct clk *clk;
    u32 val;
    ebi = devm_kzalloc(dev, sizeof(*ebi), GFP_KERNEL);
    if (!ebi)
    return -ENOMEM;
    platform_set_drvdata(pdev, ebi);
    INIT_LIST_HEAD(&ebi.devs);
    ebi.caps = device_get_match_data(dev);
    if (!ebi.caps)
    return -EINVAL;
    ebi.dev = dev;
    clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    ebi.clk = clk;
    struct device_node *smc_np __free(device_node) =
    of_parse_phandle(dev.of_node, "atmel,smc", 0);
    ebi.smc.regmap = syscon_node_to_regmap(smc_np);
    if (IS_ERR(ebi.smc.regmap))
    return PTR_ERR(ebi.smc.regmap);
    ebi.smc.layout = atmel_hsmc_get_reg_layout(smc_np);
    if (IS_ERR(ebi.smc.layout))
    return PTR_ERR(ebi.smc.layout);
    ebi.smc.clk = of_clk_get(smc_np, 0);
    if (IS_ERR(ebi.smc.clk)) {
    if (PTR_ERR(ebi.smc.clk) != -ENOENT)
    return PTR_ERR(ebi.smc.clk);
    ebi.smc.clk = core::ptr::null_mut();
    }
    ret = clk_prepare_enable(ebi.smc.clk);
    if (ret)
    return ret;
//
// The sama5d3 does not provide an EBICSA register and thus does need
// to access it.
//
    if (ebi.caps.ebi_csa_offs) {
    ebi.regmap =
    syscon_regmap_lookup_by_phandle(np,
    ebi.caps.regmap_name);
    if (IS_ERR(ebi.regmap))
    return PTR_ERR(ebi.regmap);
    }
    ret = of_property_read_u32(np, "#address-cells", &val);
    if (ret) {
    dev_err(dev, "missing #address-cells property\n");
    return ret;
    }
    reg_cells = val;
    ret = of_property_read_u32(np, "#size-cells", &val);
    if (ret) {
    dev_err(dev, "missing #address-cells property\n");
    return ret;
    }
    reg_cells += val;
    for_each_available_child_of_node_scoped(np, child) {
    if (!of_property_present(child, "reg"))
    continue;
    ret = atmel_ebi_dev_setup(ebi, child, reg_cells);
    if (ret) {
    dev_err(dev, "failed to configure EBI bus for %pOF, disabling the device",
    child);
    ret = atmel_ebi_dev_disable(ebi, child);
    if (ret)
    return ret;
    }
    }
    return of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    }
#[no_mangle]
unsafe extern "C" fn atmel_ebi_resume(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int atmel_ebi_resume(struct device *dev)
    {
    struct atmel_ebi *ebi = dev_get_drvdata(dev);
    struct atmel_ebi_dev *ebid;
    list_for_each_entry(ebid, &ebi.devs, node) {
    int i;
    for (i = 0; i < ebid.numcs; i++)
    ebid.ebi.caps.apply_config(ebid, &ebid.configs[i]);
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(atmel_ebi_pm_ops, core::ptr::null_mut(), atmel_ebi_resume);
    static struct platform_driver atmel_ebi_driver = {
    .probe = atmel_ebi_probe,
    .driver = {
    .name = "atmel-ebi",
    .of_match_table	= atmel_ebi_id_table,
    .pm = &atmel_ebi_pm_ops,
    },
    };
    builtin_platform_driver(atmel_ebi_driver);
