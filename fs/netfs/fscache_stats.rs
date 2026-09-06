//! Automatically rewritten from C to Rust
//! Source: fs/netfs/fscache_stats.c
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
// FS-Cache statistics
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// operation counters
//
    atomic_t fscache_n_volumes;
    atomic_t fscache_n_volumes_collision;
    atomic_t fscache_n_volumes_nomem;
    atomic_t fscache_n_cookies;
    atomic_t fscache_n_cookies_lru;
    atomic_t fscache_n_cookies_lru_expired;
    atomic_t fscache_n_cookies_lru_removed;
    atomic_t fscache_n_cookies_lru_dropped;
    atomic_t fscache_n_acquires;
    atomic_t fscache_n_acquires_ok;
    atomic_t fscache_n_acquires_oom;
    atomic_t fscache_n_invalidates;
    atomic_t fscache_n_updates;
    EXPORT_SYMBOL(fscache_n_updates);
    atomic_t fscache_n_relinquishes;
    atomic_t fscache_n_relinquishes_retire;
    atomic_t fscache_n_relinquishes_dropped;
    atomic_t fscache_n_resizes;
    atomic_t fscache_n_resizes_null;
    atomic_t fscache_n_read;
    EXPORT_SYMBOL(fscache_n_read);
    atomic_t fscache_n_write;
    EXPORT_SYMBOL(fscache_n_write);
    atomic_t fscache_n_no_write_space;
    EXPORT_SYMBOL(fscache_n_no_write_space);
    atomic_t fscache_n_no_create_space;
    EXPORT_SYMBOL(fscache_n_no_create_space);
    atomic_t fscache_n_culled;
    EXPORT_SYMBOL(fscache_n_culled);
    atomic_t fscache_n_dio_misfit;
    EXPORT_SYMBOL(fscache_n_dio_misfit);
//
// display the general statistics
//
#[no_mangle]
pub unsafe extern "C" fn fscache_stats_show(m: *mut seq_file) -> c_int {
    int fscache_stats_show(struct seq_file *m)
    {
    seq_puts(m, "-- FS-Cache statistics --\n");
    seq_printf(m, "Cookies: n=%d v=%d vcol=%u voom=%u\n",
    atomic_read(&fscache_n_cookies),
    atomic_read(&fscache_n_volumes),
    atomic_read(&fscache_n_volumes_collision),
    atomic_read(&fscache_n_volumes_nomem)
    );
    seq_printf(m, "Acquire: n=%u ok=%u oom=%u\n",
    atomic_read(&fscache_n_acquires),
    atomic_read(&fscache_n_acquires_ok),
    atomic_read(&fscache_n_acquires_oom));
    seq_printf(m, "LRU    : n=%u exp=%u rmv=%u drp=%u at=%ld\n",
    atomic_read(&fscache_n_cookies_lru),
    atomic_read(&fscache_n_cookies_lru_expired),
    atomic_read(&fscache_n_cookies_lru_removed),
    atomic_read(&fscache_n_cookies_lru_dropped),
    timer_pending(&fscache_cookie_lru_timer) ?
    fscache_cookie_lru_timer.expires - jiffies : 0);
    seq_printf(m, "Invals : n=%u\n",
    atomic_read(&fscache_n_invalidates));
    seq_printf(m, "Updates: n=%u rsz=%u rsn=%u\n",
    atomic_read(&fscache_n_updates),
    atomic_read(&fscache_n_resizes),
    atomic_read(&fscache_n_resizes_null));
    seq_printf(m, "Relinqs: n=%u rtr=%u drop=%u\n",
    atomic_read(&fscache_n_relinquishes),
    atomic_read(&fscache_n_relinquishes_retire),
    atomic_read(&fscache_n_relinquishes_dropped));
    seq_printf(m, "NoSpace: nwr=%u ncr=%u cull=%u\n",
    atomic_read(&fscache_n_no_write_space),
    atomic_read(&fscache_n_no_create_space),
    atomic_read(&fscache_n_culled));
    seq_printf(m, "IO     : rd=%u wr=%u mis=%u\n",
    atomic_read(&fscache_n_read),
    atomic_read(&fscache_n_write),
    atomic_read(&fscache_n_dio_misfit));
    return 0;
    }
