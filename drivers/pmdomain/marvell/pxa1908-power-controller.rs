//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/marvell/pxa1908-power-controller.c
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
// Copyright 2025 Duje Mihanović <duje@dujemihanovic.xyz>
//

// VPU, GPU, ISP
pub const APMU_PWR_CTRL_REG: c_uint = 0xd8;
pub const APMU_PWR_BLK_TMR_REG: c_uint = 0xdc;
pub const APMU_PWR_STATUS_REG: c_uint = 0xf0;
// DSI
pub const APMU_DEBUG: c_uint = 0x88;

pub const APMU_AUDIO_CLK: c_uint = 0x80;
pub const AUDIO_ULCX_ENABLE: c_uint = 0x0d;
pub const POWER_ON_LATENCY_US: c_int = 300;
pub const POWER_OFF_LATENCY_US: c_int = 20;

pub const POWER_POLL_SLEEP_US: c_int = 6;
pub const NR_DOMAINS: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa1908_pd_ctrl {
    pub domains: [*mut generic_pm_domain; NR_DOMAINS],
    pub onecell_data: genpd_onecell_data,
    pub base: *mut regmap,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa1908_pd_data {
    pub reg_clk_res_ctrl: u32,
    pub pwr_state: u32,
    pub hw_mode: u32,
    pub keep_on: bool,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa1908_pd {
    pub data: pxa1908_pd_data,
    pub ctrl: *mut pxa1908_pd_ctrl,
    pub genpd: generic_pm_domain,
    pub initialized: bool,
}

#[no_mangle]
pub unsafe extern "C" fn pxa1908_pd_is_on(pd: *mut pxa1908_pd) -> bool {
    static inline bool pxa1908_pd_is_on(struct pxa1908_pd *pd)
    {
    struct pxa1908_pd_ctrl *ctrl = pd.ctrl;
    switch (pd.data.id) {
    case PXA1908_POWER_DOMAIN_AUDIO:
    return regmap_test_bits(ctrl.base, APMU_AUDIO_CLK, AUDIO_ULCX_ENABLE);
    case PXA1908_POWER_DOMAIN_DSI:
    return regmap_test_bits(ctrl.base, APMU_DEBUG, DSI_PHY_DVM_MASK);
    default:
    return regmap_test_bits(ctrl.base, APMU_PWR_STATUS_REG, pd.data.pwr_state);
    }
    }
#[no_mangle]
unsafe extern "C" fn pxa1908_pd_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static int pxa1908_pd_power_on(struct generic_pm_domain *genpd)
    {
    struct pxa1908_pd *pd = to_pxa1908_pd(genpd);
    const struct pxa1908_pd_data *data = &pd.data;
    struct pxa1908_pd_ctrl *ctrl = pd.ctrl;
    unsigned int status;
    let mut ret: c_int = 0;
    regmap_set_bits(ctrl.base, data.reg_clk_res_ctrl, data.hw_mode);
    if (data.id != PXA1908_POWER_DOMAIN_ISP)
    regmap_write(ctrl.base, APMU_PWR_BLK_TMR_REG, 0x20001fff);
    regmap_set_bits(ctrl.base, APMU_PWR_CTRL_REG, data.pwr_state);
    ret = regmap_read_poll_timeout(ctrl.base, APMU_PWR_STATUS_REG, status,
    status & data.pwr_state, POWER_POLL_SLEEP_US,
    POWER_ON_LATENCY_US + POWER_POLL_TIMEOUT_US);
    if (ret == -ETIMEDOUT)
    dev_err(ctrl.dev, "timed out powering on domain '%s'\n", pd.genpd.name);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pxa1908_pd_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static int pxa1908_pd_power_off(struct generic_pm_domain *genpd)
    {
    struct pxa1908_pd *pd = to_pxa1908_pd(genpd);
    const struct pxa1908_pd_data *data = &pd.data;
    struct pxa1908_pd_ctrl *ctrl = pd.ctrl;
    unsigned int status;
    int ret;
    regmap_clear_bits(ctrl.base, APMU_PWR_CTRL_REG, data.pwr_state);
    ret = regmap_read_poll_timeout(ctrl.base, APMU_PWR_STATUS_REG, status,
    !(status & data.pwr_state), POWER_POLL_SLEEP_US,
    POWER_OFF_LATENCY_US + POWER_POLL_TIMEOUT_US);
    if (ret == -ETIMEDOUT) {
    dev_err(ctrl.dev, "timed out powering off domain '%s'\n", pd.genpd.name);
    return ret;
    }
    return regmap_clear_bits(ctrl.base, data.reg_clk_res_ctrl, data.hw_mode);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa1908_dsi_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static inline int pxa1908_dsi_power_on(struct generic_pm_domain *genpd)
    {
    struct pxa1908_pd *pd = to_pxa1908_pd(genpd);
    struct pxa1908_pd_ctrl *ctrl = pd.ctrl;
    return regmap_set_bits(ctrl.base, APMU_DEBUG, DSI_PHY_DVM_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa1908_dsi_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static inline int pxa1908_dsi_power_off(struct generic_pm_domain *genpd)
    {
    struct pxa1908_pd *pd = to_pxa1908_pd(genpd);
    struct pxa1908_pd_ctrl *ctrl = pd.ctrl;
    return regmap_clear_bits(ctrl.base, APMU_DEBUG, DSI_PHY_DVM_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa1908_audio_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static inline int pxa1908_audio_power_on(struct generic_pm_domain *genpd)
    {
    struct pxa1908_pd *pd = to_pxa1908_pd(genpd);
    struct pxa1908_pd_ctrl *ctrl = pd.ctrl;
    return regmap_set_bits(ctrl.base, APMU_AUDIO_CLK, AUDIO_ULCX_ENABLE);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa1908_audio_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static inline int pxa1908_audio_power_off(struct generic_pm_domain *genpd)
    {
    struct pxa1908_pd *pd = to_pxa1908_pd(genpd);
    struct pxa1908_pd_ctrl *ctrl = pd.ctrl;
    return regmap_clear_bits(ctrl.base, APMU_AUDIO_CLK, AUDIO_ULCX_ENABLE);
    }

    [_id] = { \
    .data = { \
    .reg_clk_res_ctrl = ctrl, \
    .hw_mode = BIT(mode), \
    .pwr_state = BIT(state), \
    .id = _id, \
    }, \
    .genpd = { \
    .name = _name, \
    .power_on = pxa1908_pd_power_on, \
    .power_off = pxa1908_pd_power_off, \
    }, \
    }
    static struct pxa1908_pd domains[NR_DOMAINS] = {
    DOMAIN(PXA1908_POWER_DOMAIN_VPU, "vpu", 0xa4, 19, 2),
    DOMAIN(PXA1908_POWER_DOMAIN_GPU, "gpu", 0xcc, 11, 0),
    DOMAIN(PXA1908_POWER_DOMAIN_GPU2D, "gpu2d", 0xf4, 11, 6),
    DOMAIN(PXA1908_POWER_DOMAIN_ISP, "isp", 0x38, 15, 4),
    [PXA1908_POWER_DOMAIN_DSI] = {
    .genpd = {
    .name = "dsi",
    .power_on = pxa1908_dsi_power_on,
    .power_off = pxa1908_dsi_power_off,
//
// TODO: There is no DSI driver written yet and until then we probably
// don't want to power off the DSI PHY ever.
//
    .flags = GENPD_FLAG_ALWAYS_ON,
    },
    .data = {
// See above.
    .keep_on = true,
    },
    },
    [PXA1908_POWER_DOMAIN_AUDIO] = {
    .genpd = {
    .name = "audio",
    .power_on = pxa1908_audio_power_on,
    .power_off = pxa1908_audio_power_off,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn pxa1908_pd_remove(auxdev: *mut auxiliary_device) {
    static void pxa1908_pd_remove(struct auxiliary_device *auxdev)
    {
    struct pxa1908_pd *pd;
    int ret;
    for (int i = NR_DOMAINS - 1; i >= 0; i--) {
    pd = &domains[i];
    if (!pd.initialized)
    continue;
    if (pxa1908_pd_is_on(pd) && !pd.data.keep_on)
    pxa1908_pd_power_off(&pd.genpd);
    ret = pm_genpd_remove(&pd.genpd);
    if (ret)
    dev_err(&auxdev.dev, "failed to remove domain '%s': %d\n",
    pd.genpd.name, ret);
    }
    }
    static int
    pxa1908_pd_init(struct pxa1908_pd_ctrl *ctrl, int id, struct device *dev)
    {
    struct pxa1908_pd *pd = &domains[id];
    int ret;
    ctrl.domains[id] = &pd.genpd;
    pd.ctrl = ctrl;
// Make sure the state of the hardware is synced with the domain table above.
    if (pd.data.keep_on) {
    ret = pd.genpd.power_on(&pd.genpd);
    if (ret)
    return dev_err_probe(dev, ret, "failed to power on domain '%s'\n",
    pd.genpd.name);
    } else {
    if (pxa1908_pd_is_on(pd)) {
    dev_warn(dev,
    "domain '%s' is on despite being default off; powering off\n",
    pd.genpd.name);
    ret = pd.genpd.power_off(&pd.genpd);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to power off domain '%s'\n",
    pd.genpd.name);
    }
    }
    ret = pm_genpd_init(&pd.genpd, core::ptr::null_mut(), !pd.data.keep_on);
    if (ret)
    return dev_err_probe(dev, ret, "domain '%s' failed to initialize\n",
    pd.genpd.name);
    pd.initialized = true;
    return 0;
    }
    static int
    pxa1908_pd_probe(struct auxiliary_device *auxdev, const struct auxiliary_device_id *aux_id)
    {
    struct pxa1908_pd_ctrl *ctrl;
    struct device *dev = &auxdev.dev;
    int ret;
    ctrl = devm_kzalloc(dev, sizeof(*ctrl), GFP_KERNEL);
    if (!ctrl)
    return -ENOMEM;
    auxiliary_set_drvdata(auxdev, ctrl);
    ctrl.base = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(ctrl.base))
    return dev_err_probe(dev, PTR_ERR(ctrl.base), "no regmap available\n");
    ctrl.dev = dev;
    ctrl.onecell_data.domains = ctrl.domains;
    ctrl.onecell_data.num_domains = NR_DOMAINS;
    for (int i = 0; i < NR_DOMAINS; i++) {
    ret = pxa1908_pd_init(ctrl, i, dev);
    if (ret)
    goto err;
    }
    return of_genpd_add_provider_onecell(dev.parent.of_node, &ctrl.onecell_data);
    err:
    pxa1908_pd_remove(auxdev);
    return ret;
    }
    static const struct auxiliary_device_id pxa1908_pd_id[] = {
    { .name = "clk_pxa1908_apmu.power" },
    { }
    };
    MODULE_DEVICE_TABLE(auxiliary, pxa1908_pd_id);
    static struct auxiliary_driver pxa1908_pd_driver = {
    .probe = pxa1908_pd_probe,
    .remove = pxa1908_pd_remove,
    .id_table = pxa1908_pd_id,
    };
    module_auxiliary_driver(pxa1908_pd_driver);
    MODULE_AUTHOR("Duje Mihanović <duje@dujemihanovic.xyz>");
    MODULE_DESCRIPTION("Marvell PXA1908 power domain driver");
    MODULE_LICENSE("GPL");
