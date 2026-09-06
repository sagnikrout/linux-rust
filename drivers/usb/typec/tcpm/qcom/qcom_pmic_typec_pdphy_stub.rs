//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/tcpm/qcom/qcom_pmic_typec_pdphy_stub.c
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
// Copyright (c) 2024, Linaro Ltd. All rights reserved.
//

    static int qcom_pmic_typec_pdphy_stub_pd_transmit(struct tcpc_dev *tcpc,
    enum tcpm_transmit_type type,
    const struct pd_message *msg,
    unsigned int negotiated_rev)
    {
    struct pmic_typec *tcpm = tcpc_to_tcpm(tcpc);
    struct device *dev = tcpm.dev;
    dev_dbg(dev, "pdphy_transmit: type=%d\n", type);
    tcpm_pd_transmit_complete(tcpm.tcpm_port,
    TCPC_TX_SUCCESS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pmic_typec_pdphy_stub_set_pd_rx(tcpc: *mut tcpc_dev, on: bool) -> c_int {
    static int qcom_pmic_typec_pdphy_stub_set_pd_rx(struct tcpc_dev *tcpc, bool on)
    {
    struct pmic_typec *tcpm = tcpc_to_tcpm(tcpc);
    struct device *dev = tcpm.dev;
    dev_dbg(dev, "set_pd_rx: %s\n", str_on_off(on));
    return 0;
    }
    static int qcom_pmic_typec_pdphy_stub_set_roles(struct tcpc_dev *tcpc, bool attached,
    enum typec_role power_role,
    enum typec_data_role data_role)
    {
    struct pmic_typec *tcpm = tcpc_to_tcpm(tcpc);
    struct device *dev = tcpm.dev;
    dev_dbg(dev, "pdphy_set_roles: data_role_host=%d power_role_src=%d\n",
    data_role, power_role);
    return 0;
    }
    static int qcom_pmic_typec_pdphy_stub_start(struct pmic_typec *tcpm,
    struct tcpm_port *tcpm_port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pmic_typec_pdphy_stub_stop(tcpm: *mut pmic_typec) {
    static void qcom_pmic_typec_pdphy_stub_stop(struct pmic_typec *tcpm)
    {
    }
    int qcom_pmic_typec_pdphy_stub_probe(struct platform_device *pdev,
    struct pmic_typec *tcpm)
    {
    tcpm.tcpc.set_pd_rx = qcom_pmic_typec_pdphy_stub_set_pd_rx;
    tcpm.tcpc.set_roles = qcom_pmic_typec_pdphy_stub_set_roles;
    tcpm.tcpc.pd_transmit = qcom_pmic_typec_pdphy_stub_pd_transmit;
    tcpm.pdphy_start = qcom_pmic_typec_pdphy_stub_start;
    tcpm.pdphy_stop = qcom_pmic_typec_pdphy_stub_stop;
    return 0;
    }
