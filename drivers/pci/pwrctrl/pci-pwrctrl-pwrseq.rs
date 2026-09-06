//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pwrctrl/pci-pwrctrl-pwrseq.c
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
// Copyright (C) 2024 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrseq_pwrctrl {
    pub pwrctrl: pci_pwrctrl,
    pub pwrseq: *mut pwrseq_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrseq_pwrctrl_pdata {
    pub target: *const c_char,
//
// Called before doing anything else to perform device-specific
// verification between requesting the power sequencing handle.
//
    pub dev): *mut *mut int (validate_device)(struct device,
}

#[no_mangle]
unsafe extern "C" fn pwrseq_pwrctrl_qcm_wcn_validate_device(dev: *mut device) -> c_int {
    static int pwrseq_pwrctrl_qcm_wcn_validate_device(struct device *dev)
    {
//
// Old device trees for some platforms already define wifi nodes for
// the WCN family of chips since before power sequencing was added
// upstream.
//
// These nodes don't consume the regulator outputs from the PMU, and
// if we allow this driver to bind to one of such "incomplete" nodes,
// we'll see a kernel log error about the indefinite probe deferral.
//
// Check the existence of the regulator supply that exists on all
// WCN models before moving forward.
//
    if (!device_property_present(dev, "vddaon-supply"))
    return -ENODEV;
    return 0;
    }
    static const struct pwrseq_pwrctrl_pdata pwrseq_pwrctrl_qcom_wcn_pdata = {
    .target = "wlan",
    .validate_device = pwrseq_pwrctrl_qcm_wcn_validate_device,
    };
#[no_mangle]
unsafe extern "C" fn pwrseq_pwrctrl_power_on(pwrctrl: *mut pci_pwrctrl) -> c_int {
    static int pwrseq_pwrctrl_power_on(struct pci_pwrctrl *pwrctrl)
    {
    struct pwrseq_pwrctrl *pwrseq = container_of(pwrctrl,
    struct pwrseq_pwrctrl, pwrctrl);
    return pwrseq_enable(pwrseq.pwrseq);
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_pwrctrl_power_off(pwrctrl: *mut pci_pwrctrl) -> c_int {
    static int pwrseq_pwrctrl_power_off(struct pci_pwrctrl *pwrctrl)
    {
    struct pwrseq_pwrctrl *pwrseq = container_of(pwrctrl,
    struct pwrseq_pwrctrl, pwrctrl);
    return pwrseq_disable(pwrseq.pwrseq);
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_pwrctrl_probe(pdev: *mut platform_device) -> c_int {
    static int pwrseq_pwrctrl_probe(struct platform_device *pdev)
    {
    const struct pwrseq_pwrctrl_pdata *pdata;
    struct pwrseq_pwrctrl *pwrseq;
    struct device *dev = &pdev.dev;
    int ret;
    pdata = device_get_match_data(dev);
    if (!pdata || !pdata.target)
    return -EINVAL;
    if (pdata.validate_device) {
    ret = pdata.validate_device(dev);
    if (ret)
    return ret;
    }
    pwrseq = devm_kzalloc(dev, sizeof(*pwrseq), GFP_KERNEL);
    if (!pwrseq)
    return -ENOMEM;
    pwrseq.pwrseq = devm_pwrseq_get(dev, pdata.target);
    if (IS_ERR(pwrseq.pwrseq))
    return dev_err_probe(dev, PTR_ERR(pwrseq.pwrseq),
    "Failed to get the power sequencer\n");
    pwrseq.pwrctrl.power_on = pwrseq_pwrctrl_power_on;
    pwrseq.pwrctrl.power_off = pwrseq_pwrctrl_power_off;
    pci_pwrctrl_init(&pwrseq.pwrctrl, dev);
    ret = devm_pci_pwrctrl_device_set_ready(dev, &pwrseq.pwrctrl);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register the pwrctrl wrapper\n");
    return 0;
    }
    static const struct of_device_id pwrseq_pwrctrl_of_match[] = {
    {
// ATH11K in QCA6390 package.
    .compatible = "pci17cb,1101",
    .data = &pwrseq_pwrctrl_qcom_wcn_pdata,
    },
    {
// ATH11K in WCN6855 package.
    .compatible = "pci17cb,1103",
    .data = &pwrseq_pwrctrl_qcom_wcn_pdata,
    },
    {
// ATH12K in WCN7850 package.
    .compatible = "pci17cb,1107",
    .data = &pwrseq_pwrctrl_qcom_wcn_pdata,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, pwrseq_pwrctrl_of_match);
    static struct platform_driver pwrseq_pwrctrl_driver = {
    .driver = {
    .name = "pci-pwrctrl-pwrseq",
    .of_match_table = pwrseq_pwrctrl_of_match,
    },
    .probe = pwrseq_pwrctrl_probe,
    };
    module_platform_driver(pwrseq_pwrctrl_driver);
    MODULE_AUTHOR("Bartosz Golaszewski <bartosz.golaszewski@linaro.org>");
    MODULE_DESCRIPTION("Generic PCI Power Control module for power sequenced devices");
    MODULE_LICENSE("GPL");
