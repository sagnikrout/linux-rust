//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/sysctl.c
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
// sysctls for configuring RxRPC operating parameters
//
// Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static struct ctl_table_header *rxrpc_sysctl_reg_table;
    let mut rxrpc_rx_mtu_min: static unsigned int = 500;
    let mut rxrpc_jumbo_max: static unsigned int = RXRPC_MAX_NR_JUMBO;
    let mut four: static unsigned int = 4;
    let mut max_backlog: static unsigned int = RXRPC_BACKLOG_MAX - 1;
    let mut n_65535: static unsigned int = 65535;
    let mut n_max_acks: static unsigned int = 255;
    let mut one_ms: static unsigned long = 1;
    let mut max_ms: static unsigned long = 1000;
    let mut one_jiffy: static unsigned long = 1;
    let mut max_jiffies: static unsigned long = MAX_JIFFY_OFFSET;

    let mut max_500: static unsigned long = 500;

//
// RxRPC operating parameters.
//
// See Documentation/networking/rxrpc.rst and the variable definitions for more
// information on the individual parameters.
//
    static struct ctl_table rxrpc_sysctl_table[] = {
// Values measured in milliseconds
    {
    .procname	= "soft_ack_delay",
    .data		= &rxrpc_soft_ack_delay,
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    .extra1		= (void *)&one_ms,
    .extra2		= (void *)&max_ms,
    },
    {
    .procname	= "idle_ack_delay",
    .data		= &rxrpc_idle_ack_delay,
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    .extra1		= (void *)&one_ms,
    .extra2		= (void *)&max_ms,
    },
    {
    .procname	= "idle_conn_expiry",
    .data		= &rxrpc_conn_idle_client_expiry,
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_ms_jiffies_minmax,
    .extra1		= (void *)&one_jiffy,
    .extra2		= (void *)&max_jiffies,
    },
    {
    .procname	= "idle_conn_fast_expiry",
    .data		= &rxrpc_conn_idle_client_fast_expiry,
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_ms_jiffies_minmax,
    .extra1		= (void *)&one_jiffy,
    .extra2		= (void *)&max_jiffies,
    },
// Values used in milliseconds

    {
    .procname	= "inject_rx_delay",
    .data		= &rxrpc_inject_rx_delay,
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    .extra1		= (void *)SYSCTL_LONG_ZERO,
    .extra2		= (void *)&max_500,
    },

// Non-time values
    {
    .procname	= "reap_client_conns",
    .data		= &rxrpc_reap_client_connections,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= (void *)SYSCTL_ONE,
    .extra2		= (void *)&n_65535,
    },
    {
    .procname	= "max_backlog",
    .data		= &rxrpc_max_backlog,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= (void *)&four,
    .extra2		= (void *)&max_backlog,
    },
    {
    .procname	= "rx_window_size",
    .data		= &rxrpc_rx_window_size,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= (void *)SYSCTL_ONE,
    .extra2		= (void *)&n_max_acks,
    },
    {
    .procname	= "rx_mtu",
    .data		= &rxrpc_rx_mtu,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= (void *)&rxrpc_rx_mtu_min,
    .extra2		= (void *)&n_65535,
    },
    {
    .procname	= "rx_jumbo_max",
    .data		= &rxrpc_rx_jumbo_max,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= (void *)SYSCTL_ONE,
    .extra2		= (void *)&rxrpc_jumbo_max,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn rxrpc_sysctl_init() -> int __init {
    int __init rxrpc_sysctl_init(void)
    {
    rxrpc_sysctl_reg_table = register_net_sysctl(&init_net, "net/rxrpc",
    rxrpc_sysctl_table);
    if (!rxrpc_sysctl_reg_table)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rxrpc_sysctl_exit() {
    void rxrpc_sysctl_exit(void)
    {
    if (rxrpc_sysctl_reg_table)
    unregister_net_sysctl_table(rxrpc_sysctl_reg_table);
    }
