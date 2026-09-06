//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-power.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PowerNV OPAL power control for graceful shutdown handling
//
// Copyright 2015 IBM Corp.
//

pub const SOFT_OFF: c_uint = 0x00;
pub const SOFT_REBOOT: c_uint = 0x01;
// Detect EPOW event
#[no_mangle]
unsafe extern "C" fn detect_epow() -> bool {
    static bool detect_epow(void)
    {
    u16 epow;
    int i, rc;
    __be16 epow_classes;
    __be16 opal_epow_status[OPAL_SYSEPOW_MAX] = {0};
//
// Check for EPOW event. Kernel sends supported EPOW classes info
// to OPAL. OPAL returns EPOW info along with classes present.
//
    epow_classes = cpu_to_be16(OPAL_SYSEPOW_MAX);
    rc = opal_get_epow_status(opal_epow_status, &epow_classes);
    if (rc != OPAL_SUCCESS) {
    pr_err("Failed to get EPOW event information\n");
    return false;
    }
// Look for EPOW events present
    for (i = 0; i < be16_to_cpu(epow_classes); i++) {
    epow = be16_to_cpu(opal_epow_status[i]);
// Filter events which do not need shutdown.
    if (i == OPAL_SYSEPOW_POWER)
    epow &= ~(OPAL_SYSPOWER_CHNG | OPAL_SYSPOWER_FAIL |
    OPAL_SYSPOWER_INCL);
    if (epow)
    return true;
    }
    return false;
    }
// Check for existing EPOW, DPO events
#[no_mangle]
unsafe extern "C" fn poweroff_pending() -> bool __init {
    static bool __init poweroff_pending(void)
    {
    int rc;
    __be64 opal_dpo_timeout;
// Check for DPO event
    rc = opal_get_dpo_status(&opal_dpo_timeout);
    if (rc == OPAL_SUCCESS) {
    pr_info("Existing DPO event detected.\n");
    return true;
    }
// Check for EPOW event
    if (detect_epow()) {
    pr_info("Existing EPOW event detected.\n");
    return true;
    }
    return false;
    }
// OPAL power-control events notifier
    static int opal_power_control_event(struct notifier_block *nb,
    unsigned long msg_type, void *msg)
    {
    uint64_t type;
    switch (msg_type) {
    case OPAL_MSG_EPOW:
    if (detect_epow()) {
    pr_info("EPOW msg received. Powering off system\n");
    orderly_poweroff(true);
    }
    break;
    case OPAL_MSG_DPO:
    pr_info("DPO msg received. Powering off system\n");
    orderly_poweroff(true);
    break;
    case OPAL_MSG_SHUTDOWN:
    type = be64_to_cpu(((struct opal_msg *)msg).params[0]);
    switch (type) {
    case SOFT_REBOOT:
    pr_info("Reboot requested\n");
    orderly_reboot();
    break;
    case SOFT_OFF:
    pr_info("Poweroff requested\n");
    orderly_poweroff(true);
    break;
    default:
    pr_err("Unknown power-control type %llu\n", type);
    }
    break;
    default:
    pr_err("Unknown OPAL message type %lu\n", msg_type);
    }
    return 0;
    }
// OPAL EPOW event notifier block
    static struct notifier_block opal_epow_nb = {
    .notifier_call	= opal_power_control_event,
    .next		= core::ptr::null_mut(),
    .priority	= 0,
    };
// OPAL DPO event notifier block
    static struct notifier_block opal_dpo_nb = {
    .notifier_call	= opal_power_control_event,
    .next		= core::ptr::null_mut(),
    .priority	= 0,
    };
// OPAL power-control event notifier block
    static struct notifier_block opal_power_control_nb = {
    .notifier_call	= opal_power_control_event,
    .next		= core::ptr::null_mut(),
    .priority	= 0,
    };
#[no_mangle]
pub unsafe extern "C" fn opal_power_control_init() -> int __init {
    int __init opal_power_control_init(void)
    {
    int ret, supported = 0;
    struct device_node *np;
// Register OPAL power-control events notifier
    ret = opal_message_notifier_register(OPAL_MSG_SHUTDOWN,
    &opal_power_control_nb);
    if (ret)
    pr_err("Failed to register SHUTDOWN notifier, ret = %d\n", ret);
// Determine OPAL EPOW, DPO support
    np = of_find_node_by_path("/ibm,opal/epow");
    if (np) {
    supported = of_device_is_compatible(np, "ibm,opal-v3-epow");
    of_node_put(np);
    }
    if (!supported)
    return 0;
    pr_info("OPAL EPOW, DPO support detected.\n");
// Register EPOW event notifier
    ret = opal_message_notifier_register(OPAL_MSG_EPOW, &opal_epow_nb);
    if (ret)
    pr_err("Failed to register EPOW notifier, ret = %d\n", ret);
// Register DPO event notifier
    ret = opal_message_notifier_register(OPAL_MSG_DPO, &opal_dpo_nb);
    if (ret)
    pr_err("Failed to register DPO notifier, ret = %d\n", ret);
// Check for any pending EPOW or DPO events.
    if (poweroff_pending())
    orderly_poweroff(true);
    return 0;
    }
