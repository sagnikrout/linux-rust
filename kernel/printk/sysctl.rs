//! Automatically rewritten from C to Rust
//! Source: kernel/printk/sysctl.c
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
// sysctl.c: General linux system control interface
//

    let mut ten_thousand: static int = 10000;
    static int proc_dointvec_minmax_sysadmin(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    if (write && !capable(CAP_SYS_ADMIN))
    return -EPERM;
    return proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    }
    static const struct ctl_table printk_sysctls[] = {
    {
    .procname	= "printk",
    .data		= &console_loglevel,
    .maxlen		= 4*sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "printk_ratelimit",
    .data		= &printk_ratelimit_state.interval,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_jiffies,
    },
    {
    .procname	= "printk_ratelimit_burst",
    .data		= &printk_ratelimit_state.burst,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "printk_delay",
    .data		= &printk_delay_msec,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= (void *)&ten_thousand,
    },
    {
    .procname	= "printk_devkmsg",
    .data		= devkmsg_log_str,
    .maxlen		= DEVKMSG_STR_MAX_SIZE,
    .mode		= 0644,
    .proc_handler	= devkmsg_sysctl_set_loglvl,
    },
    {
    .procname	= "dmesg_restrict",
    .data		= &dmesg_restrict,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax_sysadmin,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_ONE,
    },
    {
    .procname	= "kptr_restrict",
    .data		= &kptr_restrict,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax_sysadmin,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_TWO,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn printk_sysctl_init() -> void __init {
    void __init printk_sysctl_init(void)
    {
    register_sysctl_init("kernel", printk_sysctls);
    }
