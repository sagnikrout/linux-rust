//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/smccc/smccc.c
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
// Copyright (C) 2020 Arm Limited
//

    let mut smccc_version: static u32 = ARM_SMCCC_VERSION_1_0;
    let mut smccc_conduit: static enum arm_smccc_conduit = SMCCC_CONDUIT_NONE;
    let mut smccc_trng_available: bool __ro_after_init = false;
    let mut smccc_soc_id_version: s32 __ro_after_init = SMCCC_RET_NOT_SUPPORTED;
    let mut smccc_soc_id_revision: s32 __ro_after_init = SMCCC_RET_NOT_SUPPORTED;
#[no_mangle]
pub unsafe extern "C" fn arm_smccc_version_init(version: u32, conduit: enum arm_smccc_conduit) -> void __init {
    void __init arm_smccc_version_init(u32 version, enum arm_smccc_conduit conduit)
    {
    struct arm_smccc_res res;
    smccc_version = version;
    smccc_conduit = conduit;
    smccc_trng_available = smccc_probe_trng();
    if ((smccc_version >= ARM_SMCCC_VERSION_1_2) &&
    (smccc_conduit != SMCCC_CONDUIT_NONE)) {
    arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_FEATURES_FUNC_ID,
    ARM_SMCCC_ARCH_SOC_ID, &res);
    if ((s32)res.a0 >= 0) {
    arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_SOC_ID, 0, &res);
    smccc_soc_id_version = (s32)res.a0;
    arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_SOC_ID, 1, &res);
    smccc_soc_id_revision = (s32)res.a0;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arm_smccc_1_1_get_conduit() -> enum arm_smccc_conduit {
    enum arm_smccc_conduit arm_smccc_1_1_get_conduit(void)
    {
    if (smccc_version < ARM_SMCCC_VERSION_1_1)
    return SMCCC_CONDUIT_NONE;
    return smccc_conduit;
    }
    EXPORT_SYMBOL_GPL(arm_smccc_1_1_get_conduit);
#[no_mangle]
pub unsafe extern "C" fn arm_smccc_get_version() -> u32 {
    u32 arm_smccc_get_version(void)
    {
    return smccc_version;
    }
    EXPORT_SYMBOL_GPL(arm_smccc_get_version);
#[no_mangle]
pub unsafe extern "C" fn arm_smccc_get_soc_id_version() -> i32 {
    s32 arm_smccc_get_soc_id_version(void)
    {
    return smccc_soc_id_version;
    }
#[no_mangle]
pub unsafe extern "C" fn arm_smccc_get_soc_id_revision() -> i32 {
    s32 arm_smccc_get_soc_id_revision(void)
    {
    return smccc_soc_id_revision;
    }
    EXPORT_SYMBOL_GPL(arm_smccc_get_soc_id_revision);
#[no_mangle]
pub unsafe extern "C" fn arm_smccc_hypervisor_has_uuid(hyp_uuid: *const uuid_t) -> bool {
    bool arm_smccc_hypervisor_has_uuid(const uuid_t *hyp_uuid)
    {
    let mut res: arm_smccc_res = {};
    uuid_t uuid;
    arm_smccc_1_1_invoke(ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID, &res);
    if (res.a0 == SMCCC_RET_NOT_SUPPORTED)
    return false;
    uuid = smccc_res_to_uuid(res.a0, res.a1, res.a2, res.a3);
    return uuid_equal(&uuid, hyp_uuid);
    }
    EXPORT_SYMBOL_GPL(arm_smccc_hypervisor_has_uuid);
#[no_mangle]
unsafe extern "C" fn smccc_devices_init() -> int __init {
    static int __init smccc_devices_init(void)
    {
    struct platform_device *pdev;
    if (smccc_trng_available) {
    pdev = platform_device_register_simple("smccc_trng", -1,
    core::ptr::null_mut(), 0);
    if (IS_ERR(pdev))
    pr_err("smccc_trng: could not register device: %ld\n",
    PTR_ERR(pdev));
    }
    return 0;
    }
    device_initcall(smccc_devices_init);
