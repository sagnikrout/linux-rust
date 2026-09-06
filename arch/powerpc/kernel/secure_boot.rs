//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/secure_boot.c
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
// Copyright (C) 2019 IBM Corporation
// Author: Nayna Jain
//

    static struct device_node *get_ppc_fw_sb_node(void)
    {
    static const struct of_device_id ids[] = {
    { .compatible = "ibm,secureboot", },
    { .compatible = "ibm,secureboot-v1", },
    { .compatible = "ibm,secureboot-v2", },
    {},
    };
    return of_find_matching_node(core::ptr::null_mut(), ids);
    }
#[no_mangle]
pub unsafe extern "C" fn is_ppc_secureboot_enabled() -> bool {
    bool is_ppc_secureboot_enabled(void)
    {
    struct device_node *node;
    let mut enabled: bool = false;
    u32 secureboot;
    node = get_ppc_fw_sb_node();
    enabled = of_property_read_bool(node, "os-secureboot-enforcing");
    of_node_put(node);
    if (enabled)
    goto out;
    node = of_find_node_by_path("/");
    if (!of_property_read_u32(node, "ibm,secure-boot", &secureboot))
    enabled = (secureboot > 1);
    of_node_put(node);
    out:
    pr_info("Secure boot mode %s\n", str_enabled_disabled(enabled));
    return enabled;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_get_secureboot() -> bool {
    bool arch_get_secureboot(void)
    {
    return is_ppc_secureboot_enabled();
    }
#[no_mangle]
pub unsafe extern "C" fn is_ppc_trustedboot_enabled() -> bool {
    bool is_ppc_trustedboot_enabled(void)
    {
    struct device_node *node;
    let mut enabled: bool = false;
    u32 trustedboot;
    node = get_ppc_fw_sb_node();
    enabled = of_property_read_bool(node, "trusted-enabled");
    of_node_put(node);
    if (enabled)
    goto out;
    node = of_find_node_by_path("/");
    if (!of_property_read_u32(node, "ibm,trusted-boot", &trustedboot))
    enabled = (trustedboot > 0);
    of_node_put(node);
    out:
    pr_info("Trusted boot mode %s\n", str_enabled_disabled(enabled));
    return enabled;
    }
