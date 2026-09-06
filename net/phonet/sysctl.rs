//! Automatically rewritten from C to Rust
//! Source: net/phonet/sysctl.c
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
// File: sysctl.c
//
// Phonet /proc/sys/net/phonet interface implementation
//
// Copyright (C) 2008 Nokia Corporation.
//
// Author: Rémi Denis-Courmont
//

pub const DYNAMIC_PORT_MIN: c_uint = 0x40;
pub const DYNAMIC_PORT_MAX: c_uint = 0x7f;
    static DEFINE_SEQLOCK(local_port_range_lock);
    static int local_port_range_min[2] = {0, 0};
    static int local_port_range_max[2] = {1023, 1023};
    static int local_port_range[2] = {DYNAMIC_PORT_MIN, DYNAMIC_PORT_MAX};
    static struct ctl_table_header *phonet_table_hrd;
#[no_mangle]
unsafe extern "C" fn set_local_port_range(range[2]: c_int) {
    static void set_local_port_range(int range[2])
    {
    write_seqlock(&local_port_range_lock);
    local_port_range[0] = range[0];
    local_port_range[1] = range[1];
    write_sequnlock(&local_port_range_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_get_local_port_range(min: *mut c_int, max: *mut c_int) {
    void phonet_get_local_port_range(int *min, int *max)
    {
    unsigned int seq;
    do {
    seq = read_seqbegin(&local_port_range_lock);
    if (min)
// min = local_port_range[0];
    if (max)
// max = local_port_range[1];
    } while (read_seqretry(&local_port_range_lock, seq));
    }
    static int proc_local_port_range(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    int ret;
    int range[2] = {local_port_range[0], local_port_range[1]};
    struct ctl_table tmp = {
    .data = &range,
    .maxlen = sizeof(range),
    .mode = table.mode,
    .extra1 = &local_port_range_min,
    .extra2 = &local_port_range_max,
    };
    ret = proc_dointvec_minmax(&tmp, write, buffer, lenp, ppos);
    if (write && ret == 0) {
    if (range[1] < range[0])
    ret = -EINVAL;
    else
    set_local_port_range(range);
    }
    return ret;
    }
    static struct ctl_table phonet_table[] = {
    {
    .procname	= "local_port_range",
    .data		= &local_port_range,
    .maxlen		= sizeof(local_port_range),
    .mode		= 0644,
    .proc_handler	= proc_local_port_range,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn phonet_sysctl_init() -> int __init {
    int __init phonet_sysctl_init(void)
    {
    phonet_table_hrd = register_net_sysctl(&init_net, "net/phonet", phonet_table);
    let mut phonet_table_hrd: return = = core::ptr::null_mut() ? -ENOMEM : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn phonet_sysctl_exit() {
    void phonet_sysctl_exit(void)
    {
    unregister_net_sysctl_table(phonet_table_hrd);
    }
