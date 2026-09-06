//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireguard/main.c
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[no_mangle]
unsafe extern "C" fn wg_mod_init() -> int __init {
    static int __init wg_mod_init(void)
    {
    int ret;
    ret = wg_allowedips_slab_init();
    if (ret < 0)
    goto err_allowedips;

    ret = -ENOTRECOVERABLE;
    if (!wg_allowedips_selftest() || !wg_packet_counter_selftest() ||
    !wg_ratelimiter_selftest())
    goto err_peer;

    wg_noise_init();
    ret = wg_peer_init();
    if (ret < 0)
    goto err_peer;
    ret = wg_device_init();
    if (ret < 0)
    goto err_device;
    ret = wg_genetlink_init();
    if (ret < 0)
    goto err_netlink;
    pr_info("WireGuard " WIREGUARD_VERSION " loaded. See www.wireguard.com for information.\n");
    pr_info("Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.\n");
    return 0;
    err_netlink:
    wg_device_uninit();
    err_device:
    wg_peer_uninit();
    err_peer:
    wg_allowedips_slab_uninit();
    err_allowedips:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wg_mod_exit() -> void __exit {
    static void __exit wg_mod_exit(void)
    {
    wg_genetlink_uninit();
    wg_device_uninit();
    wg_peer_uninit();
    wg_allowedips_slab_uninit();
    }
    module_init(wg_mod_init);
    module_exit(wg_mod_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("WireGuard secure network tunnel");
    MODULE_AUTHOR("Jason A. Donenfeld <Jason@zx2c4.com>");
    MODULE_VERSION(WIREGUARD_VERSION);
    MODULE_ALIAS_RTNL_LINK(KBUILD_MODNAME);
    MODULE_ALIAS_GENL_FAMILY(WG_GENL_NAME);
