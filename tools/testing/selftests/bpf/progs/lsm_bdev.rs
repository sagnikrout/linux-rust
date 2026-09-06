//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/lsm_bdev.c
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
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>
//
// BPF LSM block device integrity tracker for dm-verity.
//
// Tracks block devices in a hashmap keyed by bd_dev.  When dm-verity
// calls security_bdev_setintegrity() during verity_preresume(), the
// setintegrity hook records the roothash and signature-validity data.
// The free hook cleans up when the device goes away.  The alloc hook
// counts allocations for test validation.
//
// The sleepable hooks exercise bpf_copy_from_user() to verify that
// the sleepable classification actually permits sleepable helpers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct verity_info {
    pub /: *mut *mut __u8 has_roothash; / LSM_INT_DMVERITY_ROOTHASH seen,
    pub /: *mut *mut __u8 sig_valid; / LSM_INT_DMVERITY_SIG_VALID value (non-NULL = valid),
    pub /: *mut *mut __u32 setintegrity_cnt; / total setintegrity calls for this dev,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, __u32);		/* dev_t from bdev.bd_dev */
    __type(value, struct verity_info);
    } verity_devices SEC(".maps");
// Global counters exposed to userspace via skeleton bss.
    int alloc_count;
    char _license[] SEC("license") = "GPL";
    SEC("lsm.s/bdev_setintegrity")
    int BPF_PROG(bdev_setintegrity, struct block_device *bdev,
    enum lsm_integrity_type type, const void *value, size_t size)
    {
    let mut zero: verity_info = {};
    struct verity_info *info;
    __u32 dev;
    char buf;
//
// Exercise a sleepable helper to confirm the verifier
// allows it in this sleepable hook.
//
    (void)bpf_copy_from_user(&buf, sizeof(buf), core::ptr::null_mut());
    dev = bdev.bd_dev;
    info = bpf_map_lookup_elem(&verity_devices, &dev);
    if (!info) {
    bpf_map_update_elem(&verity_devices, &dev, &zero, BPF_NOEXIST);
    info = bpf_map_lookup_elem(&verity_devices, &dev);
    if (!info)
    return 0;
    }
    if (type == LSM_INT_DMVERITY_ROOTHASH)
    info.has_roothash = 1;
#[no_mangle]
pub unsafe extern "C" fn if(LSM_INT_DMVERITY_SIG_VALID: type ==) -> else {
    else if (type == LSM_INT_DMVERITY_SIG_VALID)
    info.sig_valid = (value != core::ptr::null_mut());
    __sync_fetch_and_add(&info.setintegrity_cnt, 1);
    return 0;
    }
    SEC("lsm/bdev_free_security")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bdev_free_security, bdev: *mut block_device) {
    void BPF_PROG(bdev_free_security, struct block_device *bdev)
    {
    let mut dev: __u32 = bdev.bd_dev;
    bpf_map_delete_elem(&verity_devices, &dev);
    }
    SEC("lsm.s/bdev_alloc_security")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bdev_alloc_security, bdev: *mut block_device) -> c_int {
    int BPF_PROG(bdev_alloc_security, struct block_device *bdev)
    {
    char buf;
//
// Exercise a sleepable helper to confirm the verifier
// allows it in this sleepable hook.
//
    (void)bpf_copy_from_user(&buf, sizeof(buf), core::ptr::null_mut());
    __sync_fetch_and_add(&alloc_count, 1);
    return 0;
    }
