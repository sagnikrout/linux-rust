//! Automatically rewritten from C to Rust
//! Source: drivers/soc/rockchip/dtpm.c
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
// Copyright 2021 Linaro Limited
//
// Author: Daniel Lezcano <daniel.lezcano@linaro.org>
//
// DTPM hierarchy description
//

    static struct dtpm_node __initdata rk3399_hierarchy[] = {
    [0] = { .name = "rk3399",
    .type = DTPM_NODE_VIRTUAL },
    [1] = { .name = "package",
    .type = DTPM_NODE_VIRTUAL,
    .parent = &rk3399_hierarchy[0] },
    [2] = { .name = "/cpus/cpu@0",
    .type = DTPM_NODE_DT,
    .parent = &rk3399_hierarchy[1] },
    [3] = { .name = "/cpus/cpu@1",
    .type = DTPM_NODE_DT,
    .parent = &rk3399_hierarchy[1] },
    [4] = { .name = "/cpus/cpu@2",
    .type = DTPM_NODE_DT,
    .parent = &rk3399_hierarchy[1] },
    [5] = { .name = "/cpus/cpu@3",
    .type = DTPM_NODE_DT,
    .parent = &rk3399_hierarchy[1] },
    [6] = { .name = "/cpus/cpu@100",
    .type = DTPM_NODE_DT,
    .parent = &rk3399_hierarchy[1] },
    [7] = { .name = "/cpus/cpu@101",
    .type = DTPM_NODE_DT,
    .parent = &rk3399_hierarchy[1] },
    [8] = { .name = "/gpu@ff9a0000",
    .type = DTPM_NODE_DT,
    .parent = &rk3399_hierarchy[1] },
    [9] = { /* sentinel */ }
    };
    static struct of_device_id __initdata rockchip_dtpm_match_table[] = {
    { .compatible = "rockchip,rk3399", .data = rk3399_hierarchy },
    {},
    };
#[no_mangle]
unsafe extern "C" fn rockchip_dtpm_init() -> int __init {
    static int __init rockchip_dtpm_init(void)
    {
    return dtpm_create_hierarchy(rockchip_dtpm_match_table);
    }
    module_init(rockchip_dtpm_init);
#[no_mangle]
unsafe extern "C" fn rockchip_dtpm_exit() -> void __exit {
    static void __exit rockchip_dtpm_exit(void)
    {
    return dtpm_destroy_hierarchy();
    }
    module_exit(rockchip_dtpm_exit);
    MODULE_SOFTDEP("pre: panfrost cpufreq-dt");
    MODULE_DESCRIPTION("Rockchip DTPM driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:dtpm");
    MODULE_AUTHOR("Daniel Lezcano <daniel.lezcano@kernel.org");
