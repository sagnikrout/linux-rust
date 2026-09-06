//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_bits_iter.c
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
// Copyright (c) 2024 Yafang Shao <laoar.shao@gmail.com>

    char _license[] SEC("license") = "GPL";
    int bpf_iter_bits_new(struct bpf_iter_bits *it, const u64 *unsafe_ptr__ign,
    u32 nr_bits) __ksym __weak;
    int *bpf_iter_bits_next(struct bpf_iter_bits *it) __ksym __weak;
    void bpf_iter_bits_destroy(struct bpf_iter_bits *it) __ksym __weak;
    u64 bits_array[511] = {};
    SEC("iter.s/cgroup")
    __description("bits iter without destroy")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: no_destroy, meta: *mut bpf_iter_meta, cgrp: *mut cgroup) -> c_int {
    int BPF_PROG(no_destroy, struct bpf_iter_meta *meta, struct cgroup *cgrp)
    {
    struct bpf_iter_bits it;
    let mut data: u64 = 1;
    bpf_iter_bits_new(&it, &data, 1);
    bpf_iter_bits_next(&it);
    return 0;
    }
    SEC("iter/cgroup")
    __description("uninitialized iter in .next()")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_bits as) -> __failure {
    __failure __msg("expected an initialized iter_bits as R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: next_uninit, meta: *mut bpf_iter_meta, cgrp: *mut cgroup) -> c_int {
    int BPF_PROG(next_uninit, struct bpf_iter_meta *meta, struct cgroup *cgrp)
    {
    let mut it: bpf_iter_bits = {};
    bpf_iter_bits_next(&it);
    return 0;
    }
    SEC("iter/cgroup")
    __description("uninitialized iter in .destroy()")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_bits as) -> __failure {
    __failure __msg("expected an initialized iter_bits as R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: destroy_uninit, meta: *mut bpf_iter_meta, cgrp: *mut cgroup) -> c_int {
    int BPF_PROG(destroy_uninit, struct bpf_iter_meta *meta, struct cgroup *cgrp)
    {
    let mut it: bpf_iter_bits = {};
    bpf_iter_bits_destroy(&it);
    return 0;
    }
    SEC("syscall")
    __description("null pointer")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn null_pointer() -> c_int {
    int null_pointer(void)
    {
    struct bpf_iter_bits iter;
    int err, nr = 0;
    int *bit;
    err = bpf_iter_bits_new(&iter, core::ptr::null_mut(), 1);
    bpf_iter_bits_destroy(&iter);
    if (err != -EINVAL)
    return 1;
    bpf_for_each(bits, bit, core::ptr::null_mut(), 1)
    nr++;
    return nr;
    }
    SEC("syscall")
    __description("bits copy")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 10) -> __success {
    __success __retval(10)
#[no_mangle]
pub unsafe extern "C" fn bits_copy() -> c_int {
    int bits_copy(void)
    {
    u64 data = 0xf7310UL; /* 4 + 3 + 2 + 1 + 0*/
    let mut nr: c_int = 0;
    int *bit;
    bpf_for_each(bits, bit, &data, 1)
    nr++;
    return nr;
    }
    SEC("syscall")
    __description("bits memalloc")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 64) -> __success {
    __success __retval(64)
#[no_mangle]
pub unsafe extern "C" fn bits_memalloc() -> c_int {
    int bits_memalloc(void)
    {
    u64 data[2];
    let mut nr: c_int = 0;
    int *bit;
    __builtin_memset(&data, 0xf0, sizeof(data)); /* 4 * 16 */
    bpf_for_each(bits, bit, &data[0], ARRAY_SIZE(data))
    nr++;
    return nr;
    }
    SEC("syscall")
    __description("bit index")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 8) -> __success {
    __success __retval(8)
#[no_mangle]
pub unsafe extern "C" fn bit_index() -> c_int {
    int bit_index(void)
    {
    let mut data: u64 = 0x100;
    let mut bit_idx: c_int = 0;
    int *bit;
    bpf_for_each(bits, bit, &data, 1) {
    if (*bit == 0)
    continue;
    bit_idx = *bit;
    }
    return bit_idx;
    }
    SEC("syscall")
    __description("bits too big")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn bits_too_big() -> c_int {
    int bits_too_big(void)
    {
    u64 data[4];
    let mut nr: c_int = 0;
    int *bit;
    __builtin_memset(&data, 0xff, sizeof(data));
    bpf_for_each(bits, bit, &data[0], 512) /* Be greater than 511 */
    nr++;
    return nr;
    }
    SEC("syscall")
    __description("fewer words")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn fewer_words() -> c_int {
    int fewer_words(void)
    {
    u64 data[2] = {0x1, 0xff};
    let mut nr: c_int = 0;
    int *bit;
    bpf_for_each(bits, bit, &data[0], 1)
    nr++;
    return nr;
    }
    SEC("syscall")
    __description("zero words")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn zero_words() -> c_int {
    int zero_words(void)
    {
    u64 data[2] = {0x1, 0xff};
    let mut nr: c_int = 0;
    int *bit;
    bpf_for_each(bits, bit, &data[0], 0)
    nr++;
    return nr;
    }
    SEC("syscall")
    __description("huge words")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn huge_words() -> c_int {
    int huge_words(void)
    {
    u64 data[8] = {0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1};
    let mut nr: c_int = 0;
    int *bit;
    bpf_for_each(bits, bit, &data[0], 67108865)
    nr++;
    return nr;
    }
    SEC("syscall")
    __description("max words")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 4) -> __success {
    __success __retval(4)
#[no_mangle]
pub unsafe extern "C" fn max_words() -> c_int {
    int max_words(void)
    {
    let mut nr: volatile int = 0;
    int *bit;
    bits_array[0] = (1ULL << 63) | 1U;
    bits_array[510] = (1ULL << 33) | (1ULL << 32);
    bpf_for_each(bits, bit, bits_array, 511) {
    if (nr == 0 && *bit != 0)
    break;
    if (nr == 2 && *bit != 32672)
    break;
    nr++;
    }
    return nr;
    }
    SEC("syscall")
    __description("bad words")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn bad_words() -> c_int {
    int bad_words(void)
    {
    void *bad_addr = (void *)-4095;
    struct bpf_iter_bits iter;
    volatile int nr;
    int *bit;
    int err;
    err = bpf_iter_bits_new(&iter, bad_addr, 1);
    bpf_iter_bits_destroy(&iter);
    if (err != -EFAULT)
    return 1;
    nr = 0;
    bpf_for_each(bits, bit, bad_addr, 1)
    nr++;
    if (nr != 0)
    return 2;
    err = bpf_iter_bits_new(&iter, bad_addr, 4);
    bpf_iter_bits_destroy(&iter);
    if (err != -EFAULT)
    return 3;
    nr = 0;
    bpf_for_each(bits, bit, bad_addr, 4)
    nr++;
    if (nr != 0)
    return 4;
    return 0;
    }
