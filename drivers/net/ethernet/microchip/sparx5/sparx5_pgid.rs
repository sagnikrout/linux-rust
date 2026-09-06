//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_pgid.c
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


// SPDX-License-Identifier: GPL-2.0+

#[no_mangle]
pub unsafe extern "C" fn sparx5_pgid_init(spx5: *mut sparx5) {
    void sparx5_pgid_init(struct sparx5 *spx5)
    {
    int i;
    for (i = 0; i < spx5.data.consts.n_pgids; i++)
    spx5.pgid_map[i] = SPX5_PGID_FREE;
// Reserved for unicast, flood control, broadcast, and CPU.
// These cannot be freed.
//
    for (i = 0; i <= sparx5_get_pgid(spx5, PGID_CPU); i++)
    spx5.pgid_map[i] = SPX5_PGID_RESERVED;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_pgid_alloc_mcast(spx5: *mut sparx5, idx: *mut u16) -> c_int {
    int sparx5_pgid_alloc_mcast(struct sparx5 *spx5, u16 *idx)
    {
    int i;
// The multicast area starts at index 65, but the first 7
// are reserved for flood masks and CPU. Start alloc after that.
//
    for (i = sparx5_get_pgid(spx5, PGID_MCAST_START);
    i < spx5.data.consts.n_pgids; i++) {
    if (spx5.pgid_map[i] == SPX5_PGID_FREE) {
    spx5.pgid_map[i] = SPX5_PGID_MULTICAST;
// idx = i;
    return 0;
    }
    }
    return -EBUSY;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_pgid_free(spx5: *mut sparx5, idx: u16) -> c_int {
    int sparx5_pgid_free(struct sparx5 *spx5, u16 idx)
    {
    if (idx <= sparx5_get_pgid(spx5, PGID_CPU) ||
    idx >= spx5.data.consts.n_pgids)
    return -EINVAL;
    if (spx5.pgid_map[idx] == SPX5_PGID_FREE)
    return -EINVAL;
    spx5.pgid_map[idx] = SPX5_PGID_FREE;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_get_pgid(sparx5: *mut sparx5, pgid: c_int) -> c_int {
    int sparx5_get_pgid(struct sparx5 *sparx5, int pgid)
    {
    return sparx5.data.consts.n_ports + pgid;
    }
