//! Automatically rewritten from C to Rust
//! Source: fs/ceph/util.c
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
// Some non-inline ceph helpers
//

//
// return true if @layout appears to be valid
//
#[no_mangle]
pub unsafe extern "C" fn ceph_file_layout_is_valid(layout: *const ceph_file_layout) -> c_int {
    int ceph_file_layout_is_valid(const struct ceph_file_layout *layout)
    {
    let mut su: __u32 = layout.stripe_unit;
    let mut sc: __u32 = layout.stripe_count;
    let mut os: __u32 = layout.object_size;
// stripe unit, object size must be non-zero, 64k increment
    if (!su || (su & (CEPH_MIN_STRIPE_UNIT-1)))
    return 0;
    if (!os || (os & (CEPH_MIN_STRIPE_UNIT-1)))
    return 0;
// object size must be a multiple of stripe unit
    if (os < su || os % su)
    return 0;
// stripe count must be non-zero
    if (!sc)
    return 0;
    return 1;
    }
    void ceph_file_layout_from_legacy(struct ceph_file_layout *fl,
    struct ceph_file_layout_legacy *legacy)
    {
    fl.stripe_unit = le32_to_cpu(legacy.fl_stripe_unit);
    fl.stripe_count = le32_to_cpu(legacy.fl_stripe_count);
    fl.object_size = le32_to_cpu(legacy.fl_object_size);
    fl.pool_id = le32_to_cpu(legacy.fl_pg_pool);
    if (fl.pool_id == 0 && fl.stripe_unit == 0 &&
    fl.stripe_count == 0 && fl.object_size == 0)
    fl.pool_id = -1;
    }
    void ceph_file_layout_to_legacy(struct ceph_file_layout *fl,
    struct ceph_file_layout_legacy *legacy)
    {
    legacy.fl_stripe_unit = cpu_to_le32(fl.stripe_unit);
    legacy.fl_stripe_count = cpu_to_le32(fl.stripe_count);
    legacy.fl_object_size = cpu_to_le32(fl.object_size);
    if (fl.pool_id >= 0)
    legacy.fl_pg_pool = cpu_to_le32(fl.pool_id);
    else
    legacy.fl_pg_pool = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_flags_to_mode(flags: c_int) -> c_int {
    int ceph_flags_to_mode(int flags)
    {
    int mode;

    if ((flags & O_DIRECTORY) == O_DIRECTORY)
    return CEPH_FILE_MODE_PIN;

    switch (flags & O_ACCMODE) {
    case O_WRONLY:
    mode = CEPH_FILE_MODE_WR;
    break;
    case O_RDONLY:
    mode = CEPH_FILE_MODE_RD;
    break;
    case O_RDWR:
    case O_ACCMODE: /* this is what the VFS does */
    mode = CEPH_FILE_MODE_RDWR;
    break;
    }

    if (flags & O_LAZY)
    mode |= CEPH_FILE_MODE_LAZY;

    return mode;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_caps_for_mode(mode: c_int) -> c_int {
    int ceph_caps_for_mode(int mode)
    {
    let mut caps: c_int = CEPH_CAP_PIN;
    if (mode & CEPH_FILE_MODE_RD)
    caps |= CEPH_CAP_FILE_SHARED |
    CEPH_CAP_FILE_RD | CEPH_CAP_FILE_CACHE;
    if (mode & CEPH_FILE_MODE_WR)
    caps |= CEPH_CAP_FILE_EXCL |
    CEPH_CAP_FILE_WR | CEPH_CAP_FILE_BUFFER |
    CEPH_CAP_AUTH_SHARED | CEPH_CAP_AUTH_EXCL |
    CEPH_CAP_XATTR_SHARED | CEPH_CAP_XATTR_EXCL;
    if (mode & CEPH_FILE_MODE_LAZY)
    caps |= CEPH_CAP_FILE_LAZYIO;
    return caps;
    }
