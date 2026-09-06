//! Automatically rewritten from C to Rust
//! Source: drivers/base/firmware_loader/fallback_table.c
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
// firmware fallback configuration table
//
    struct firmware_fallback_config fw_fallback_config = {
    .force_sysfs_fallback = IS_ENABLED(CONFIG_FW_LOADER_USER_HELPER_FALLBACK),
    .loading_timeout = 60,
    .old_timeout = 60,
    };
    EXPORT_SYMBOL_NS_GPL(fw_fallback_config, "FIRMWARE_LOADER_PRIVATE");

    static const struct ctl_table firmware_config_table[] = {
    {
    .procname	= "force_sysfs_fallback",
    .data		= &fw_fallback_config.force_sysfs_fallback,
    .maxlen         = sizeof(unsigned int),
    .mode           = 0644,
    .proc_handler   = proc_douintvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_ONE,
    },
    {
    .procname	= "ignore_sysfs_fallback",
    .data		= &fw_fallback_config.ignore_sysfs_fallback,
    .maxlen         = sizeof(unsigned int),
    .mode           = 0644,
    .proc_handler   = proc_douintvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_ONE,
    },
    };
    static struct ctl_table_header *firmware_config_sysct_table_header;
#[no_mangle]
pub unsafe extern "C" fn register_firmware_config_sysctl() -> c_int {
    int register_firmware_config_sysctl(void)
    {
    firmware_config_sysct_table_header =
    register_sysctl("kernel/firmware_config",
    firmware_config_table);
    if (!firmware_config_sysct_table_header)
    return -ENOMEM;
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(register_firmware_config_sysctl, "FIRMWARE_LOADER_PRIVATE");
#[no_mangle]
pub unsafe extern "C" fn unregister_firmware_config_sysctl() {
    void unregister_firmware_config_sysctl(void)
    {
    unregister_sysctl_table(firmware_config_sysct_table_header);
    firmware_config_sysct_table_header = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_NS_GPL(unregister_firmware_config_sysctl, "FIRMWARE_LOADER_PRIVATE");
