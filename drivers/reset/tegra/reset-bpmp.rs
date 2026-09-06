//! Automatically rewritten from C to Rust
//! Source: drivers/reset/tegra/reset-bpmp.c
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
// Copyright (C) 2016 NVIDIA Corporation
//

    static struct tegra_bpmp *to_tegra_bpmp(struct reset_controller_dev *rstc)
    {
    return container_of(rstc, struct tegra_bpmp, rstc);
    }
    static int tegra_bpmp_reset_common(struct reset_controller_dev *rstc,
    enum mrq_reset_commands command,
    unsigned int id)
    {
    struct tegra_bpmp *bpmp = to_tegra_bpmp(rstc);
    struct mrq_reset_request request;
    struct tegra_bpmp_message msg;
    int err;
    memset(&request, 0, sizeof(request));
    request.cmd = command;
    request.reset_id = id;
    memset(&msg, 0, sizeof(msg));
    msg.mrq = MRQ_RESET;
    msg.tx.data = &request;
    msg.tx.size = sizeof(request);
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err)
    return err;
    if (msg.rx.ret)
    return -EINVAL;
    return 0;
    }
    static int tegra_bpmp_reset_module(struct reset_controller_dev *rstc,
    unsigned long id)
    {
    return tegra_bpmp_reset_common(rstc, CMD_RESET_MODULE, id);
    }
    static int tegra_bpmp_reset_assert(struct reset_controller_dev *rstc,
    unsigned long id)
    {
    return tegra_bpmp_reset_common(rstc, CMD_RESET_ASSERT, id);
    }
    static int tegra_bpmp_reset_deassert(struct reset_controller_dev *rstc,
    unsigned long id)
    {
    return tegra_bpmp_reset_common(rstc, CMD_RESET_DEASSERT, id);
    }
    static const struct reset_control_ops tegra_bpmp_reset_ops = {
    .reset = tegra_bpmp_reset_module,
    .assert = tegra_bpmp_reset_assert,
    .deassert = tegra_bpmp_reset_deassert,
    };
#[no_mangle]
pub unsafe extern "C" fn tegra_bpmp_init_resets(bpmp: *mut tegra_bpmp) -> c_int {
    int tegra_bpmp_init_resets(struct tegra_bpmp *bpmp)
    {
    bpmp.rstc.ops = &tegra_bpmp_reset_ops;
    bpmp.rstc.owner = THIS_MODULE;
    bpmp.rstc.of_node = bpmp.dev.of_node;
    bpmp.rstc.nr_resets = bpmp.soc.num_resets;
    return devm_reset_controller_register(bpmp.dev, &bpmp.rstc);
    }
