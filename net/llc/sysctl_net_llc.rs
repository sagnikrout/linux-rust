//! Automatically rewritten from C to Rust
//! Source: net/llc/sysctl_net_llc.c
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
// sysctl_net_llc.c: sysctl interface to LLC net subsystem.
//
// Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

    static struct ctl_table llc2_timeout_table[] = {
    {
    .procname	= "ack",
    .data		= &sysctl_llc2_ack_timeout,
    .maxlen		= sizeof(sysctl_llc2_ack_timeout),
    .mode		= 0644,
    .proc_handler   = proc_dointvec_jiffies,
    },
    {
    .procname	= "busy",
    .data		= &sysctl_llc2_busy_timeout,
    .maxlen		= sizeof(sysctl_llc2_busy_timeout),
    .mode		= 0644,
    .proc_handler   = proc_dointvec_jiffies,
    },
    {
    .procname	= "p",
    .data		= &sysctl_llc2_p_timeout,
    .maxlen		= sizeof(sysctl_llc2_p_timeout),
    .mode		= 0644,
    .proc_handler   = proc_dointvec_jiffies,
    },
    {
    .procname	= "rej",
    .data		= &sysctl_llc2_rej_timeout,
    .maxlen		= sizeof(sysctl_llc2_rej_timeout),
    .mode		= 0644,
    .proc_handler   = proc_dointvec_jiffies,
    },
    };
    static struct ctl_table_header *llc2_timeout_header;
    static struct ctl_table_header *llc_station_header;
#[no_mangle]
pub unsafe extern "C" fn llc_sysctl_init() -> int __init {
    int __init llc_sysctl_init(void)
    {
    static struct ctl_table empty[1] = {};
    llc2_timeout_header = register_net_sysctl(&init_net, "net/llc/llc2/timeout", llc2_timeout_table);
    llc_station_header = register_net_sysctl_sz(&init_net, "net/llc/station", empty, 0);
    if (!llc2_timeout_header || !llc_station_header) {
    llc_sysctl_exit();
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sysctl_exit() {
    void llc_sysctl_exit(void)
    {
    if (llc2_timeout_header) {
    unregister_net_sysctl_table(llc2_timeout_header);
    llc2_timeout_header = core::ptr::null_mut();
    }
    if (llc_station_header) {
    unregister_net_sysctl_table(llc_station_header);
    llc_station_header = core::ptr::null_mut();
    }
    }
