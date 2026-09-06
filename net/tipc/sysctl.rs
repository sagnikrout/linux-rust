//! Automatically rewritten from C to Rust
//! Source: net/tipc/sysctl.c
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


//
// net/tipc/sysctl.c: sysctl interface to TIPC subsystem
//
// Copyright (c) 2013, Wind River Systems
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

    static struct ctl_table_header *tipc_ctl_hdr;
    static struct ctl_table tipc_table[] = {
    {
    .procname	= "tipc_rmem",
    .data		= &sysctl_tipc_rmem,
    .maxlen		= sizeof(sysctl_tipc_rmem),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1         = SYSCTL_ONE,
    },
    {
    .procname	= "named_timeout",
    .data		= &sysctl_tipc_named_timeout,
    .maxlen		= sizeof(sysctl_tipc_named_timeout),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1         = SYSCTL_ZERO,
    },
    {
    .procname       = "sk_filter",
    .data           = &sysctl_tipc_sk_filter,
    .maxlen         = sizeof(sysctl_tipc_sk_filter),
    .mode           = 0644,
    .proc_handler   = proc_doulongvec_minmax,
    },

    {
    .procname	= "max_tfms",
    .data		= &sysctl_tipc_max_tfms,
    .maxlen		= sizeof(sysctl_tipc_max_tfms),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1         = SYSCTL_ONE,
    },
    {
    .procname	= "key_exchange_enabled",
    .data		= &sysctl_tipc_key_exchange_enabled,
    .maxlen		= sizeof(sysctl_tipc_key_exchange_enabled),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1         = SYSCTL_ZERO,
    .extra2         = SYSCTL_ONE,
    },

    {
    .procname	= "bc_retruni",
    .data		= &sysctl_tipc_bc_retruni,
    .maxlen		= sizeof(sysctl_tipc_bc_retruni),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn tipc_register_sysctl() -> c_int {
    int tipc_register_sysctl(void)
    {
    tipc_ctl_hdr = register_net_sysctl(&init_net, "net/tipc", tipc_table);
    if (tipc_ctl_hdr == core::ptr::null_mut())
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tipc_unregister_sysctl() {
    void tipc_unregister_sysctl(void)
    {
    unregister_net_sysctl_table(tipc_ctl_hdr);
    }
