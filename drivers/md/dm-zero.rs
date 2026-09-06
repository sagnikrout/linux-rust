//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-zero.c
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
// Copyright (C) 2003 Jana Saout <jana@saout.de>
//
// This file is released under the GPL.
//

//
// Construct a dummy mapping that only returns zeros
//
#[no_mangle]
unsafe extern "C" fn zero_ctr(ti: *mut dm_target, argc: c_uint, argv: *mut c_char) -> c_int {
    static int zero_ctr(struct dm_target *ti, unsigned int argc, char **argv)
    {
    if (argc != 0) {
    ti.error = "No arguments required";
    return -EINVAL;
    }
//
// Silently drop discards, avoiding -EOPNOTSUPP.
//
    ti.num_discard_bios = 1;
    ti.discards_supported = true;
    return 0;
    }
//
// Return zeros only on reads
//
#[no_mangle]
unsafe extern "C" fn zero_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    static int zero_map(struct dm_target *ti, struct bio *bio)
    {
    switch (bio_op(bio)) {
    case REQ_OP_READ:
    if (bio.bi_opf & REQ_RAHEAD) {
// readahead of null bytes only wastes buffer cache
    return DM_MAPIO_KILL;
    }
    zero_fill_bio(bio);
    break;
    case REQ_OP_WRITE:
    case REQ_OP_DISCARD:
// writes get silently dropped
    break;
    default:
    return DM_MAPIO_KILL;
    }
    bio_endio(bio);
// accepted bio, don't make new request
    return DM_MAPIO_SUBMITTED;
    }
#[no_mangle]
unsafe extern "C" fn zero_io_hints(ti: *mut dm_target, limits: *mut queue_limits) {
    static void zero_io_hints(struct dm_target *ti, struct queue_limits *limits)
    {
    limits.max_hw_discard_sectors = UINT_MAX;
    limits.discard_granularity = 512;
    }
    static struct target_type zero_target = {
    .name   = "zero",
    .version = {1, 2, 0},
    .features = DM_TARGET_NOWAIT,
    .module = THIS_MODULE,
    .ctr    = zero_ctr,
    .map    = zero_map,
    .io_hints = zero_io_hints,
    };
    module_dm(zero);
    MODULE_AUTHOR("Jana Saout <jana@saout.de>");
    MODULE_DESCRIPTION(DM_NAME " dummy target returning zeros");
    MODULE_LICENSE("GPL");
