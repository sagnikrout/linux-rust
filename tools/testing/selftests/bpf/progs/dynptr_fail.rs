//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/dynptr_fail.c
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
// Copyright (c) 2022 Facebook

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_info {
    pub x: c_int,
    pub ptr: bpf_dynptr,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct bpf_dynptr);
    } array_map1 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct test_info);
    } array_map2 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } array_map3 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } array_map4 SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample {
    pub pid: c_int,
    pub value: c_long,
    pub comm: [c_char; 16],
}

    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 4096);
    } ringbuf SEC(".maps");
    int err, val;
#[no_mangle]
unsafe extern "C" fn get_map_val_dynptr(ptr: *mut bpf_dynptr) -> c_int {
    static int get_map_val_dynptr(struct bpf_dynptr *ptr)
    {
    let mut key: __u32 = 0, *map_val;
    bpf_map_update_elem(&array_map3, &key, &val, 0);
    map_val = bpf_map_lookup_elem(&array_map3, &key);
    if (!map_val)
    return -ENOENT;
    bpf_dynptr_from_mem(map_val, sizeof(*map_val), 0, ptr);
    return 0;
    }
// Every bpf_ringbuf_reserve_dynptr call must have a corresponding
// bpf_ringbuf_submit/discard_dynptr call
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(id=1": "Unreleased reference) -> __failure {
    __failure __msg("Unreleased reference id=1")
#[no_mangle]
pub unsafe extern "C" fn ringbuf_missing_release1(ctx: *mut c_void) -> c_int {
    int ringbuf_missing_release1(void *ctx)
    {
    let mut ptr: bpf_dynptr = {};
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// missing a call to bpf_ringbuf_discard/submit_dynptr
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(id=3": "Unreleased reference) -> __failure {
    __failure __msg("Unreleased reference id=3")
#[no_mangle]
pub unsafe extern "C" fn ringbuf_missing_release2(ctx: *mut c_void) -> c_int {
    int ringbuf_missing_release2(void *ctx)
    {
    struct bpf_dynptr ptr1, ptr2;
    struct sample *sample;
    bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr1);
    bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr2);
    sample = bpf_dynptr_data(&ptr1, 0, sizeof(*sample));
    if (!sample) {
    bpf_ringbuf_discard_dynptr(&ptr1, 0);
    bpf_ringbuf_discard_dynptr(&ptr2, 0);
    return 0;
    }
    bpf_ringbuf_submit_dynptr(&ptr1, 0);
// missing a call to bpf_ringbuf_discard/submit_dynptr on ptr2
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn missing_release_callback_fn(index: __u32, data: *mut c_void) -> c_int {
    static int missing_release_callback_fn(__u32 index, void *data)
    {
    struct bpf_dynptr ptr;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// missing a call to bpf_ringbuf_discard/submit_dynptr
    return 0;
    }
// Any dynptr initialized within a callback must have bpf_dynptr_put called
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(id": "Unreleased reference) -> __failure {
    __failure __msg("Unreleased reference id")
#[no_mangle]
pub unsafe extern "C" fn ringbuf_missing_release_callback(ctx: *mut c_void) -> c_int {
    int ringbuf_missing_release_callback(void *ctx)
    {
    bpf_loop(10, missing_release_callback_fn, core::ptr::null_mut(), 0);
    return 0;
    }
// Can't call bpf_ringbuf_submit/discard_dynptr on a non-initialized dynptr
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn ringbuf_release_uninit_dynptr(ctx: *mut c_void) -> c_int {
    int ringbuf_release_uninit_dynptr(void *ctx)
    {
    struct bpf_dynptr ptr;
// this should fail
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
// A dynptr can't be used after it has been invalidated
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R3": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R3")
#[no_mangle]
pub unsafe extern "C" fn use_after_invalid(ctx: *mut c_void) -> c_int {
    int use_after_invalid(void *ctx)
    {
    struct bpf_dynptr ptr;
    char read_data[64];
    bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(read_data), 0, &ptr);
    bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
    bpf_ringbuf_submit_dynptr(&ptr, 0);
// this should fail
    bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
    return 0;
    }
// Can't call non-dynptr ringbuf APIs on a dynptr ringbuf sample
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=ringbuf_mem": "type=mem) -> __failure {
    __failure __msg("type=mem expected=ringbuf_mem")
#[no_mangle]
pub unsafe extern "C" fn ringbuf_invalid_api(ctx: *mut c_void) -> c_int {
    int ringbuf_invalid_api(void *ctx)
    {
    struct bpf_dynptr ptr;
    struct sample *sample;
    bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr);
    sample = bpf_dynptr_data(&ptr, 0, sizeof(*sample));
    if (!sample)
    goto done;
    sample.pid = 123;
// invalid API use. need to use dynptr API to submit/discard
    bpf_ringbuf_submit(sample, 0);
    done:
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// Can't add a dynptr to a map
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid read from) -> __failure {
    __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn add_dynptr_to_map1(ctx: *mut c_void) -> c_int {
    int add_dynptr_to_map1(void *ctx)
    {
    struct bpf_dynptr ptr;
    let mut key: c_int = 0;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
// this should fail
    bpf_map_update_elem(&array_map1, &key, &ptr, 0);
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
// Can't add a struct with an embedded dynptr to a map
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid read from) -> __failure {
    __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn add_dynptr_to_map2(ctx: *mut c_void) -> c_int {
    int add_dynptr_to_map2(void *ctx)
    {
    struct test_info x;
    let mut key: c_int = 0;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &x.ptr);
// this should fail
    bpf_map_update_elem(&array_map2, &key, &x, 0);
    bpf_ringbuf_submit_dynptr(&x.ptr, 0);
    return 0;
    }
// A data slice can't be accessed out of bounds
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "value is outside of the allowed memory) -> __failure {
    __failure __msg("value is outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn data_slice_out_of_bounds_ringbuf(ctx: *mut c_void) -> c_int {
    int data_slice_out_of_bounds_ringbuf(void *ctx)
    {
    struct bpf_dynptr ptr;
    void *data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 8, 0, &ptr);
    data  = bpf_dynptr_data(&ptr, 0, 8);
    if (!data)
    goto done;
// can't index out of bounds of the data slice
    val = *((char *)data + 8);
    done:
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
// A data slice can't be accessed out of bounds
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "value is outside of the allowed memory) -> __failure {
    __failure __msg("value is outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn data_slice_out_of_bounds_skb(skb: *mut __sk_buff) -> c_int {
    int data_slice_out_of_bounds_skb(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
    hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
// this should fail
// (__u8*)(hdr + 1) = 1;
    return SK_PASS;
    }
// A metadata slice can't be accessed out of bounds
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "value is outside of the allowed memory) -> __failure {
    __failure __msg("value is outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn data_slice_out_of_bounds_skb_meta(skb: *mut __sk_buff) -> c_int {
    int data_slice_out_of_bounds_skb_meta(struct __sk_buff *skb)
    {
    struct bpf_dynptr meta;
    __u8 *md;
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice_rdwr(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
// this should fail
// (md + 1) = 42;
    return SK_PASS;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "value is outside of the allowed memory) -> __failure {
    __failure __msg("value is outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn data_slice_out_of_bounds_map_value(ctx: *mut c_void) -> c_int {
    int data_slice_out_of_bounds_map_value(void *ctx)
    {
    __u32 map_val;
    struct bpf_dynptr ptr;
    void *data;
    get_map_val_dynptr(&ptr);
    data  = bpf_dynptr_data(&ptr, 0, sizeof(map_val));
    if (!data)
    return 0;
// can't index out of bounds of the data slice
    val = *((char *)data + (sizeof(map_val) + 1));
    return 0;
    }
// A data slice can't be used after it has been released
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn data_slice_use_after_release1(ctx: *mut c_void) -> c_int {
    int data_slice_use_after_release1(void *ctx)
    {
    struct bpf_dynptr ptr;
    struct sample *sample;
    bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr);
    sample = bpf_dynptr_data(&ptr, 0, sizeof(*sample));
    if (!sample)
    goto done;
    sample.pid = 123;
    bpf_ringbuf_submit_dynptr(&ptr, 0);
// this should fail
    val = sample.pid;
    return 0;
    done:
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// A data slice can't be used after it has been released.
//
// This tests the case where the data slice tracks a dynptr (ptr2)
// that is at a non-zero offset from the frame pointer (ptr1 is at fp,
// ptr2 is at fp - 16).
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn data_slice_use_after_release2(ctx: *mut c_void) -> c_int {
    int data_slice_use_after_release2(void *ctx)
    {
    struct bpf_dynptr ptr1, ptr2;
    struct sample *sample;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr1);
    bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(*sample), 0, &ptr2);
    sample = bpf_dynptr_data(&ptr2, 0, sizeof(*sample));
    if (!sample)
    goto done;
    sample.pid = 23;
    bpf_ringbuf_submit_dynptr(&ptr2, 0);
// this should fail
    sample.pid = 23;
    bpf_ringbuf_submit_dynptr(&ptr1, 0);
    return 0;
    done:
    bpf_ringbuf_discard_dynptr(&ptr2, 0);
    bpf_ringbuf_discard_dynptr(&ptr1, 0);
    return 0;
    }
// A data slice must be first checked for NULL
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('mem_or_null'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'mem_or_null'")
#[no_mangle]
pub unsafe extern "C" fn data_slice_missing_null_check1(ctx: *mut c_void) -> c_int {
    int data_slice_missing_null_check1(void *ctx)
    {
    struct bpf_dynptr ptr;
    void *data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 8, 0, &ptr);
    data  = bpf_dynptr_data(&ptr, 0, 8);
// missing if (!data) check
// this should fail
// (__u8 *)data = 3;
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
// A data slice can't be dereferenced if it wasn't checked for null
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('mem_or_null'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'mem_or_null'")
#[no_mangle]
pub unsafe extern "C" fn data_slice_missing_null_check2(ctx: *mut c_void) -> c_int {
    int data_slice_missing_null_check2(void *ctx)
    {
    struct bpf_dynptr ptr;
    __u64 *data1, *data2;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr);
    data1 = bpf_dynptr_data(&ptr, 0, 8);
    data2 = bpf_dynptr_data(&ptr, 0, 8);
    if (data1)
// this should fail
// data2 = 3;
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// Can't pass in a dynptr as an arg to a helper function that doesn't take in a
// dynptr argument
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid read from) -> __failure {
    __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn invalid_helper1(ctx: *mut c_void) -> c_int {
    int invalid_helper1(void *ctx)
    {
    struct bpf_dynptr ptr;
    get_map_val_dynptr(&ptr);
// this should fail
    bpf_strncmp((const char *)&ptr, sizeof(ptr), "hello!");
    return 0;
    }
// A dynptr can't be passed into a helper function at a non-zero offset
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset=-8": "cannot pass in dynptr at an) -> __failure {
    __failure __msg("cannot pass in dynptr at an offset=-8")
#[no_mangle]
pub unsafe extern "C" fn invalid_helper2(ctx: *mut c_void) -> c_int {
    int invalid_helper2(void *ctx)
    {
    struct bpf_dynptr ptr;
    char read_data[64];
    get_map_val_dynptr(&ptr);
// this should fail
    bpf_dynptr_read(read_data, sizeof(read_data), (void *)&ptr + 8, 0, 0);
    return 0;
    }
// A bpf_dynptr is invalidated if it's been written into
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn invalid_write1(ctx: *mut c_void) -> c_int {
    int invalid_write1(void *ctx)
    {
    struct bpf_dynptr ptr;
    void *data;
    let mut x: __u8 = 0;
    get_map_val_dynptr(&ptr);
    memcpy(&ptr, &x, sizeof(x));
// this should fail
    data = bpf_dynptr_data(&ptr, 0, 1);
    __sink(data);
    return 0;
    }
//
// A bpf_dynptr can't be used as a dynptr if it has been written into at a fixed
// offset
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn invalid_write2(ctx: *mut c_void) -> c_int {
    int invalid_write2(void *ctx)
    {
    struct bpf_dynptr ptr;
    char read_data[64];
    let mut x: __u8 = 0;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
    memcpy((void *)&ptr + 8, &x, sizeof(x));
// this should fail
    bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
//
// A bpf_dynptr can't be used as a dynptr if it has been written into at a
// non-const offset
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn invalid_write3(ctx: *mut c_void) -> c_int {
    int invalid_write3(void *ctx)
    {
    struct bpf_dynptr ptr;
    char stack_buf[16];
    unsigned long len;
    let mut x: __u8 = 0;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 8, 0, &ptr);
    memcpy(stack_buf, &val, sizeof(val));
    len = stack_buf[0] & 0xf;
    memcpy((void *)&ptr + len, &x, sizeof(x));
// this should fail
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn invalid_write4_callback(index: __u32, data: *mut c_void) -> c_int {
    static int invalid_write4_callback(__u32 index, void *data)
    {
// (__u32 *)data = 123;
    return 0;
    }
// If the dynptr is written into in a callback function, it should
// be invalidated as a dynptr
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn invalid_write4(ctx: *mut c_void) -> c_int {
    int invalid_write4(void *ctx)
    {
    struct bpf_dynptr ptr;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
    bpf_loop(10, invalid_write4_callback, &ptr, 0);
// this should fail
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
// A globally-defined bpf_dynptr can't be used (it must reside as a stack frame)
    struct bpf_dynptr global_dynptr;
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=fp": "type=map_value) -> __failure {
    __failure __msg("type=map_value expected=fp")
#[no_mangle]
pub unsafe extern "C" fn global(ctx: *mut c_void) -> c_int {
    int global(void *ctx)
    {
// this should fail
    bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &global_dynptr);
    bpf_ringbuf_discard_dynptr(&global_dynptr, 0);
    return 0;
    }
// A direct read should fail
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid read from) -> __failure {
    __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn invalid_read1(ctx: *mut c_void) -> c_int {
    int invalid_read1(void *ctx)
    {
    struct bpf_dynptr ptr;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// this should fail
    val = *(int *)&ptr;
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// A direct read at an offset should fail
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset": "cannot pass in dynptr at an) -> __failure {
    __failure __msg("cannot pass in dynptr at an offset")
#[no_mangle]
pub unsafe extern "C" fn invalid_read2(ctx: *mut c_void) -> c_int {
    int invalid_read2(void *ctx)
    {
    struct bpf_dynptr ptr;
    char read_data[64];
    get_map_val_dynptr(&ptr);
// this should fail
    bpf_dynptr_read(read_data, sizeof(read_data), (void *)&ptr + 1, 0, 0);
    return 0;
    }
// A direct read at an offset into the lower stack slot should fail
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid read from) -> __failure {
    __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn invalid_read3(ctx: *mut c_void) -> c_int {
    int invalid_read3(void *ctx)
    {
    struct bpf_dynptr ptr1, ptr2;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr1);
    bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr2);
// this should fail
    memcpy(&val, (void *)&ptr1 + 8, sizeof(val));
    bpf_ringbuf_discard_dynptr(&ptr1, 0);
    bpf_ringbuf_discard_dynptr(&ptr2, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn invalid_read4_callback(index: __u32, data: *mut c_void) -> c_int {
    static int invalid_read4_callback(__u32 index, void *data)
    {
// this should fail
    val = *(__u32 *)data;
    return 0;
    }
// A direct read within a callback function should fail
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid read from) -> __failure {
    __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn invalid_read4(ctx: *mut c_void) -> c_int {
    int invalid_read4(void *ctx)
    {
    struct bpf_dynptr ptr;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
    bpf_loop(10, invalid_read4_callback, &ptr, 0);
    bpf_ringbuf_submit_dynptr(&ptr, 0);
    return 0;
    }
// Initializing a dynptr on an offset should fail
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset=0": "cannot pass in dynptr at an) -> __failure {
    __failure __msg("cannot pass in dynptr at an offset=0")
#[no_mangle]
pub unsafe extern "C" fn invalid_offset(ctx: *mut c_void) -> c_int {
    int invalid_offset(void *ctx)
    {
    struct bpf_dynptr ptr;
// this should fail
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr + 1);
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// Can't release a dynptr twice
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn release_twice(ctx: *mut c_void) -> c_int {
    int release_twice(void *ctx)
    {
    struct bpf_dynptr ptr;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 16, 0, &ptr);
    bpf_ringbuf_discard_dynptr(&ptr, 0);
// this second release should fail
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn release_twice_callback_fn(index: __u32, data: *mut c_void) -> c_int {
    static int release_twice_callback_fn(__u32 index, void *data)
    {
// this should fail
    bpf_ringbuf_discard_dynptr(data, 0);
    return 0;
    }
// Test that releasing a dynptr twice, where one of the releases happens
// within a callback function, fails
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn release_twice_callback(ctx: *mut c_void) -> c_int {
    int release_twice_callback(void *ctx)
    {
    struct bpf_dynptr ptr;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 32, 0, &ptr);
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    bpf_loop(10, release_twice_callback_fn, &ptr, 0);
    return 0;
    }
// Reject unsupported local mem types for dynptr_from_mem API
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(data": "Unsupported reg type fp for bpf_dynptr_from_mem) -> __failure {
    __failure __msg("Unsupported reg type fp for bpf_dynptr_from_mem data")
#[no_mangle]
pub unsafe extern "C" fn dynptr_from_mem_invalid_api(ctx: *mut c_void) -> c_int {
    int dynptr_from_mem_invalid_api(void *ctx)
    {
    struct bpf_dynptr ptr;
    let mut x: c_int = 0;
// this should fail
    bpf_dynptr_from_mem(&x, sizeof(x), 0, &ptr);
    return 0;
    }
// Cannot create dynptr from dynptr data
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(data": "Unsupported reg type mem for bpf_dynptr_from_mem) -> __failure {
    __failure __msg("Unsupported reg type mem for bpf_dynptr_from_mem data")
#[no_mangle]
pub unsafe extern "C" fn dynptr_from_dynptr_data(ctx: *mut c_void) -> c_int {
    int dynptr_from_dynptr_data(void *ctx)
    {
    struct bpf_dynptr ptr, ptr2;
    __u8 *data;
    if (get_map_val_dynptr(&ptr))
    return 0;
    data = bpf_dynptr_data(&ptr, 0, sizeof(__u32));
    if (!data)
    return 0;
// this should fail
    bpf_dynptr_from_mem(data, sizeof(__u32), 0, &ptr2);
    return 0;
    }
// Cannot create dynptr from dynptr slice
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(data": "Unsupported reg type mem for bpf_dynptr_from_mem) -> __failure {
    __failure __msg("Unsupported reg type mem for bpf_dynptr_from_mem data")
#[no_mangle]
pub unsafe extern "C" fn dynptr_from_dynptr_slice(skb: *mut __sk_buff) -> c_int {
    int dynptr_from_dynptr_slice(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr, ptr2;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
    hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
// this should fail
    bpf_dynptr_from_mem(hdr, sizeof(*hdr), 0, &ptr2);
    return SK_PASS;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(__log_level(2: "cannot overwrite referenced dynptr")) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr") __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn dynptr_pruning_overwrite(ctx: *mut __sk_buff) -> c_int {
    int dynptr_pruning_overwrite(struct __sk_buff *ctx)
    {
    asm volatile (
    "r9 = 0xeB9F;				\
    r6 = %[ringbuf] ll;			\
    r1 = r6;				\
    r2 = 8;				\
    r3 = 0;				\
    r4 = r10;				\
    r4 += -16;				\
    call %[bpf_ringbuf_reserve_dynptr];	\
    if r0 == 0 goto pjmp1;			\
    goto pjmp2;				\
    pjmp1:						\
// (u64 *)(r10 - 16) = r9;		\
    pjmp2:						\
    r1 = r10;				\
    r1 += -16;				\
    r2 = 0;				\
    call %[bpf_ringbuf_discard_dynptr];	"
    :
    : __imm(bpf_ringbuf_reserve_dynptr),
    __imm(bpf_ringbuf_discard_dynptr),
    __imm_addr(ringbuf)
    : __clobber_all
    );
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(__log_level(2: "12: safe")) -> __success {
    __success __msg("12: safe") __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn dynptr_pruning_stacksafe(ctx: *mut __sk_buff) -> c_int {
    int dynptr_pruning_stacksafe(struct __sk_buff *ctx)
    {
    asm volatile (
    "r9 = 0xeB9F;				\
    r6 = %[ringbuf] ll;			\
    r1 = r6;				\
    r2 = 8;				\
    r3 = 0;				\
    r4 = r10;				\
    r4 += -16;				\
    call %[bpf_ringbuf_reserve_dynptr];	\
    if r0 == 0 goto stjmp1;		\
    goto stjmp2;				\
    stjmp1:						\
    r9 = r9;				\
    stjmp2:						\
    r1 = r10;				\
    r1 += -16;				\
    r2 = 0;				\
    call %[bpf_ringbuf_discard_dynptr];	"
    :
    : __imm(bpf_ringbuf_reserve_dynptr),
    __imm(bpf_ringbuf_discard_dynptr),
    __imm_addr(ringbuf)
    : __clobber_all
    );
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(__log_level(2: "cannot overwrite referenced dynptr")) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr") __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn dynptr_pruning_type_confusion(ctx: *mut __sk_buff) -> c_int {
    int dynptr_pruning_type_confusion(struct __sk_buff *ctx)
    {
    asm volatile (
    "r6 = %[array_map4] ll;			\
    r7 = %[ringbuf] ll;			\
    r1 = r6;				\
    r2 = r10;				\
    r2 += -8;				\
    r9 = 0;				\
// (u64 *)(r2 + 0) = r9;			\
    r3 = r10;				\
    r3 += -24;				\
    r9 = 0xeB9FeB9F;			\
// (u64 *)(r10 - 16) = r9;		\
// (u64 *)(r10 - 24) = r9;		\
    r9 = 0;				\
    r4 = 0;				\
    r8 = r2;				\
    call %[bpf_map_update_elem];		\
    r1 = r6;				\
    r2 = r8;				\
    call %[bpf_map_lookup_elem];		\
    if r0 != 0 goto tjmp1;			\
    exit;					\
    tjmp1:						\
    r8 = r0;				\
    r1 = r7;				\
    r2 = 8;				\
    r3 = 0;				\
    r4 = r10;				\
    r4 += -16;				\
    r0 = *(u64 *)(r0 + 0);			\
    call %[bpf_ringbuf_reserve_dynptr];	\
    if r0 == 0 goto tjmp2;			\
    r8 = r8;				\
    r8 = r8;				\
    r8 = r8;				\
    r8 = r8;				\
    r8 = r8;				\
    r8 = r8;				\
    r8 = r8;				\
    goto tjmp3;				\
    tjmp2:						\
// (u64 *)(r10 - 8) = r9;		\
// (u64 *)(r10 - 16) = r9;		\
    r1 = r8;				\
    r1 += 8;				\
    r2 = 0;				\
    r3 = 0;				\
    r4 = r10;				\
    r4 += -16;				\
    call %[bpf_dynptr_from_mem];		\
    tjmp3:						\
    r1 = r10;				\
    r1 += -16;				\
    r2 = 0;				\
    call %[bpf_ringbuf_discard_dynptr];	"
    :
    : __imm(bpf_map_update_elem),
    __imm(bpf_map_lookup_elem),
    __imm(bpf_ringbuf_reserve_dynptr),
    __imm(bpf_dynptr_from_mem),
    __imm(bpf_ringbuf_discard_dynptr),
    __imm_addr(array_map4),
    __imm_addr(ringbuf)
    : __clobber_all
    );
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(__log_level(2: "dynptr has to be at a constant offset")) -> __failure {
    __failure __msg("dynptr has to be at a constant offset") __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn dynptr_var_off_overwrite(ctx: *mut __sk_buff) -> c_int {
    int dynptr_var_off_overwrite(struct __sk_buff *ctx)
    {
    asm volatile (
    "r9 = 16;				\
// (u32 *)(r10 - 4) = r9;		\
    r8 = *(u32 *)(r10 - 4);		\
    if r8 >= 0 goto vjmp1;			\
    r0 = 1;				\
    exit;					\
    vjmp1:						\
    if r8 <= 16 goto vjmp2;		\
    r0 = 1;				\
    exit;					\
    vjmp2:						\
    r8 &= 16;				\
    r1 = %[ringbuf] ll;			\
    r2 = 8;				\
    r3 = 0;				\
    r4 = r10;				\
    r4 += -32;				\
    r4 += r8;				\
    call %[bpf_ringbuf_reserve_dynptr];	\
    r9 = 0xeB9F;				\
// (u64 *)(r10 - 16) = r9;		\
    r1 = r10;				\
    r1 += -32;				\
    r1 += r8;				\
    r2 = 0;				\
    call %[bpf_ringbuf_discard_dynptr];	"
    :
    : __imm(bpf_ringbuf_reserve_dynptr),
    __imm(bpf_ringbuf_discard_dynptr),
    __imm_addr(ringbuf)
    : __clobber_all
    );
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(__log_level(2: "cannot overwrite referenced dynptr")) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr") __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn dynptr_partial_slot_invalidate(ctx: *mut __sk_buff) -> c_int {
    int dynptr_partial_slot_invalidate(struct __sk_buff *ctx)
    {
    asm volatile (
    "r6 = %[ringbuf] ll;			\
    r7 = %[array_map4] ll;			\
    r1 = r7;				\
    r2 = r10;				\
    r2 += -8;				\
    r9 = 0;				\
// (u64 *)(r2 + 0) = r9;			\
    r3 = r2;				\
    r4 = 0;				\
    r8 = r2;				\
    call %[bpf_map_update_elem];		\
    r1 = r7;				\
    r2 = r8;				\
    call %[bpf_map_lookup_elem];		\
    if r0 != 0 goto sjmp1;			\
    exit;					\
    sjmp1:						\
    r7 = r0;				\
    r1 = r6;				\
    r2 = 8;				\
    r3 = 0;				\
    r4 = r10;				\
    r4 += -24;				\
    call %[bpf_ringbuf_reserve_dynptr];	\
// (u64 *)(r10 - 16) = r9;		\
    r1 = r7;				\
    r2 = 8;				\
    r3 = 0;				\
    r4 = r10;				\
    r4 += -16;				\
    call %[bpf_dynptr_from_mem];		\
    r1 = r10;				\
    r1 += -512;				\
    r2 = 488;				\
    r3 = r10;				\
    r3 += -24;				\
    r4 = 0;				\
    r5 = 0;				\
    call %[bpf_dynptr_read];		\
    r8 = 1;				\
    if r0 != 0 goto sjmp2;			\
    r8 = 0;				\
    sjmp2:						\
    r1 = r10;				\
    r1 += -24;				\
    r2 = 0;				\
    call %[bpf_ringbuf_discard_dynptr];	"
    :
    : __imm(bpf_map_update_elem),
    __imm(bpf_map_lookup_elem),
    __imm(bpf_ringbuf_reserve_dynptr),
    __imm(bpf_ringbuf_discard_dynptr),
    __imm(bpf_dynptr_from_mem),
    __imm(bpf_dynptr_read),
    __imm_addr(ringbuf),
    __imm_addr(array_map4)
    : __clobber_all
    );
    return 0;
    }
// Test that it is allowed to overwrite unreferenced dynptr.
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn dynptr_overwrite_unref(ctx: *mut c_void) -> c_int {
    int dynptr_overwrite_unref(void *ctx)
    {
    struct bpf_dynptr ptr;
    if (get_map_val_dynptr(&ptr))
    return 0;
    if (get_map_val_dynptr(&ptr))
    return 0;
    if (get_map_val_dynptr(&ptr))
    return 0;
    return 0;
    }
// Test that slices are invalidated on reinitializing a dynptr.
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn dynptr_invalidate_slice_reinit(ctx: *mut c_void) -> c_int {
    int dynptr_invalidate_slice_reinit(void *ctx)
    {
    struct bpf_dynptr ptr;
    __u8 *p;
    if (get_map_val_dynptr(&ptr))
    return 0;
    p = bpf_dynptr_data(&ptr, 0, 1);
    if (!p)
    return 0;
    if (get_map_val_dynptr(&ptr))
    return 0;
// this should fail
    return *p;
    }
// Invalidation of dynptr slices on destruction of dynptr should not miss
// mem_or_null pointers.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=percpu_ptr_": "R{{[0-9]+}} type=scalar) -> __failure {
    __failure __msg("R{{[0-9]+}} type=scalar expected=percpu_ptr_")
#[no_mangle]
pub unsafe extern "C" fn dynptr_invalidate_slice_or_null(ctx: *mut c_void) -> c_int {
    int dynptr_invalidate_slice_or_null(void *ctx)
    {
    struct bpf_dynptr ptr;
    __u8 *p;
    if (get_map_val_dynptr(&ptr))
    return 0;
    p = bpf_dynptr_data(&ptr, 0, 1);
// (__u8 *)&ptr = 0;
// this should fail
    bpf_this_cpu_ptr(p);
    return 0;
    }
// Destruction of dynptr should also any slices obtained from it
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R{{[0-9]+}} invalid mem access) -> __failure {
    __failure __msg("R{{[0-9]+}} invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn dynptr_invalidate_slice_failure(ctx: *mut c_void) -> c_int {
    int dynptr_invalidate_slice_failure(void *ctx)
    {
    struct bpf_dynptr ptr1;
    struct bpf_dynptr ptr2;
    __u8 *p1, *p2;
    if (get_map_val_dynptr(&ptr1))
    return 0;
    if (get_map_val_dynptr(&ptr2))
    return 0;
    p1 = bpf_dynptr_data(&ptr1, 0, 1);
    if (!p1)
    return 0;
    p2 = bpf_dynptr_data(&ptr2, 0, 1);
    if (!p2)
    return 0;
// (__u8 *)&ptr1 = 0;
// this should fail
    return *p1;
    }
// Invalidation of slices should be scoped and should not prevent dereferencing
// slices of another dynptr after destroying unrelated dynptr
//
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn dynptr_invalidate_slice_success(ctx: *mut c_void) -> c_int {
    int dynptr_invalidate_slice_success(void *ctx)
    {
    struct bpf_dynptr ptr1;
    struct bpf_dynptr ptr2;
    __u8 *p1, *p2;
    if (get_map_val_dynptr(&ptr1))
    return 1;
    if (get_map_val_dynptr(&ptr2))
    return 1;
    p1 = bpf_dynptr_data(&ptr1, 0, 1);
    if (!p1)
    return 1;
    p2 = bpf_dynptr_data(&ptr2, 0, 1);
    if (!p2)
    return 1;
// (__u8 *)&ptr1 = 0;
    return *p2;
    }
// Overwriting referenced dynptr should be rejected
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn dynptr_overwrite_ref(ctx: *mut c_void) -> c_int {
    int dynptr_overwrite_ref(void *ctx)
    {
    struct bpf_dynptr ptr;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
// this should fail
    if (get_map_val_dynptr(&ptr))
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// Reject writes to dynptr slot from bpf_dynptr_read
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn dynptr_read_into_slot(ctx: *mut c_void) -> c_int {
    int dynptr_read_into_slot(void *ctx)
    {
    union {
    struct {
    char _pad[48];
    struct bpf_dynptr ptr;
    };
    char buf[64];
    } data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &data.ptr);
// this should fail
    bpf_dynptr_read(data.buf, sizeof(data.buf), &data.ptr, 0, 0);
    return 0;
    }
// bpf_dynptr_slice()s are read-only and cannot be written to
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(rdonly_mem": "R{{[0-9]+}} cannot write into) -> __failure {
    __failure __msg("R{{[0-9]+}} cannot write into rdonly_mem")
#[no_mangle]
pub unsafe extern "C" fn skb_invalid_slice_write(skb: *mut __sk_buff) -> c_int {
    int skb_invalid_slice_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
    hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
// this should fail
    hdr.h_proto = 1;
    return SK_PASS;
    }
// bpf_dynptr_slice()s are read-only and cannot be written to
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(rdonly_mem": "R{{[0-9]+}} cannot write into) -> __failure {
    __failure __msg("R{{[0-9]+}} cannot write into rdonly_mem")
#[no_mangle]
pub unsafe extern "C" fn skb_meta_invalid_slice_write(skb: *mut __sk_buff) -> c_int {
    int skb_meta_invalid_slice_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr meta;
    __u8 *md;
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
// this should fail
// md = 42;
    return SK_PASS;
    }
// The read-only data slice is invalidated whenever a helper changes packet data
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn skb_invalid_data_slice1(skb: *mut __sk_buff) -> c_int {
    int skb_invalid_data_slice1(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
    hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
    val = hdr.h_proto;
    if (bpf_skb_pull_data(skb, skb.len))
    return SK_DROP;
// this should fail
    val = hdr.h_proto;
    return SK_PASS;
    }
// The read-write data slice is invalidated whenever a helper changes packet data
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn skb_invalid_data_slice2(skb: *mut __sk_buff) -> c_int {
    int skb_invalid_data_slice2(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
    hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
    hdr.h_proto = 123;
    if (bpf_skb_pull_data(skb, skb.len))
    return SK_DROP;
// this should fail
    hdr.h_proto = 1;
    return SK_PASS;
    }
// The read-only data slice is invalidated whenever bpf_dynptr_write() is called
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn skb_invalid_data_slice3(skb: *mut __sk_buff) -> c_int {
    int skb_invalid_data_slice3(struct __sk_buff *skb)
    {
    char write_data[64] = "hello there, world!!";
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
    hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
    val = hdr.h_proto;
    bpf_dynptr_write(&ptr, 0, write_data, sizeof(write_data), 0);
// this should fail
    val = hdr.h_proto;
    return SK_PASS;
    }
// The read-write data slice is invalidated whenever bpf_dynptr_write() is called
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn skb_invalid_data_slice4(skb: *mut __sk_buff) -> c_int {
    int skb_invalid_data_slice4(struct __sk_buff *skb)
    {
    char write_data[64] = "hello there, world!!";
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
    hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
    hdr.h_proto = 123;
    bpf_dynptr_write(&ptr, 0, write_data, sizeof(write_data), 0);
// this should fail
    hdr.h_proto = 1;
    return SK_PASS;
    }
// Read-only skb data slice is invalidated on write to skb metadata
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn ro_skb_slice_invalid_after_metadata_write(skb: *mut __sk_buff) -> c_int {
    int ro_skb_slice_invalid_after_metadata_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr data, meta;
    __u8 *d;
    bpf_dynptr_from_skb(skb, 0, &data);
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    d = bpf_dynptr_slice(&data, 0, core::ptr::null_mut(), sizeof(*d));
    if (!d)
    return SK_DROP;
    bpf_dynptr_write(&meta, 0, "x", 1, 0);
// this should fail
    val = *d;
    return SK_PASS;
    }
// Read-write skb data slice is invalidated on write to skb metadata
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn rw_skb_slice_invalid_after_metadata_write(skb: *mut __sk_buff) -> c_int {
    int rw_skb_slice_invalid_after_metadata_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr data, meta;
    __u8 *d;
    bpf_dynptr_from_skb(skb, 0, &data);
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    d = bpf_dynptr_slice_rdwr(&data, 0, core::ptr::null_mut(), sizeof(*d));
    if (!d)
    return SK_DROP;
    bpf_dynptr_write(&meta, 0, "x", 1, 0);
// this should fail
// d = 42;
    return SK_PASS;
    }
// Read-only skb metadata slice is invalidated on write to skb data
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn ro_skb_meta_slice_invalid_after_payload_write(skb: *mut __sk_buff) -> c_int {
    int ro_skb_meta_slice_invalid_after_payload_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr data, meta;
    __u8 *md;
    bpf_dynptr_from_skb(skb, 0, &data);
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
    bpf_dynptr_write(&data, 0, "x", 1, 0);
// this should fail
    val = *md;
    return SK_PASS;
    }
// Read-write skb metadata slice is invalidated on write to skb data slice
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn rw_skb_meta_slice_invalid_after_payload_write(skb: *mut __sk_buff) -> c_int {
    int rw_skb_meta_slice_invalid_after_payload_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr data, meta;
    __u8 *md;
    bpf_dynptr_from_skb(skb, 0, &data);
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice_rdwr(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
    bpf_dynptr_write(&data, 0, "x", 1, 0);
// this should fail
// md = 42;
    return SK_PASS;
    }
// Read-only skb metadata slice is invalidated whenever a helper changes packet data
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn ro_skb_meta_slice_invalid_after_payload_helper(skb: *mut __sk_buff) -> c_int {
    int ro_skb_meta_slice_invalid_after_payload_helper(struct __sk_buff *skb)
    {
    struct bpf_dynptr meta;
    __u8 *md;
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
    if (bpf_skb_pull_data(skb, skb.len))
    return SK_DROP;
// this should fail
    val = *md;
    return SK_PASS;
    }
// Read-write skb metadata slice is invalidated whenever a helper changes packet data
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn rw_skb_meta_slice_invalid_after_payload_helper(skb: *mut __sk_buff) -> c_int {
    int rw_skb_meta_slice_invalid_after_payload_helper(struct __sk_buff *skb)
    {
    struct bpf_dynptr meta;
    __u8 *md;
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice_rdwr(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
    if (bpf_skb_pull_data(skb, skb.len))
    return SK_DROP;
// this should fail
// md = 42;
    return SK_PASS;
    }
// Read-only skb metadata slice is invalidated on write to skb metadata
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn ro_skb_meta_slice_invalid_after_metadata_write(skb: *mut __sk_buff) -> c_int {
    int ro_skb_meta_slice_invalid_after_metadata_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr meta;
    __u8 *md;
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
    bpf_dynptr_write(&meta, 0, "x", 1, 0);
// this should fail
    val = *md;
    return SK_PASS;
    }
// Read-write skb metadata slice is invalidated on write to skb metadata
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn rw_skb_meta_slice_invalid_after_metadata_write(skb: *mut __sk_buff) -> c_int {
    int rw_skb_meta_slice_invalid_after_metadata_write(struct __sk_buff *skb)
    {
    struct bpf_dynptr meta;
    __u8 *md;
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    md = bpf_dynptr_slice_rdwr(&meta, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
    bpf_dynptr_write(&meta, 0, "x", 1, 0);
// this should fail
// md = 42;
    return SK_PASS;
    }
// The read-only data slice is invalidated whenever a helper changes packet data
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn xdp_invalid_data_slice1(xdp: *mut xdp_md) -> c_int {
    int xdp_invalid_data_slice1(struct xdp_md *xdp)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_xdp(xdp, 0, &ptr);
    hdr = bpf_dynptr_slice(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
    val = hdr.h_proto;
    if (bpf_xdp_adjust_head(xdp, 0 - (int)sizeof(*hdr)))
    return XDP_DROP;
// this should fail
    val = hdr.h_proto;
    return XDP_PASS;
    }
// The read-write data slice is invalidated whenever a helper changes packet data
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn xdp_invalid_data_slice2(xdp: *mut xdp_md) -> c_int {
    int xdp_invalid_data_slice2(struct xdp_md *xdp)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_xdp(xdp, 0, &ptr);
    hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
    if (!hdr)
    return SK_DROP;
    hdr.h_proto = 9;
    if (bpf_xdp_adjust_head(xdp, 0 - (int)sizeof(*hdr)))
    return XDP_DROP;
// this should fail
    hdr.h_proto = 1;
    return XDP_PASS;
    }
// Only supported prog type can create skb-type dynptrs
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_dynptr_from_skb is not) -> __failure {
    __failure __msg("calling kernel function bpf_dynptr_from_skb is not allowed")
#[no_mangle]
pub unsafe extern "C" fn skb_invalid_ctx(ctx: *mut c_void) -> c_int {
    int skb_invalid_ctx(void *ctx)
    {
    struct bpf_dynptr ptr;
// this should fail
    bpf_dynptr_from_skb(ctx, 0, &ptr);
    return 0;
    }
// Only supported prog type can create skb_meta-type dynptrs
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_dynptr_from_skb_meta is not) -> __failure {
    __failure __msg("calling kernel function bpf_dynptr_from_skb_meta is not allowed")
#[no_mangle]
pub unsafe extern "C" fn skb_meta_invalid_ctx(ctx: *mut c_void) -> c_int {
    int skb_meta_invalid_ctx(void *ctx)
    {
    struct bpf_dynptr meta;
// this should fail
    bpf_dynptr_from_skb_meta(ctx, 0, &meta);
    return 0;
    }
    SEC("fentry/skb_tx_error")
#[no_mangle]
pub unsafe extern "C" fn __msg(trusted": "must be referenced or) -> __failure {
    __failure __msg("must be referenced or trusted")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: skb_invalid_ctx_fentry, skb: *mut c_void) -> c_int {
    int BPF_PROG(skb_invalid_ctx_fentry, void *skb)
    {
    struct bpf_dynptr ptr;
// this should fail
    bpf_dynptr_from_skb(skb, 0, &ptr);
    return 0;
    }
    SEC("fexit/skb_tx_error")
#[no_mangle]
pub unsafe extern "C" fn __msg(trusted": "must be referenced or) -> __failure {
    __failure __msg("must be referenced or trusted")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: skb_invalid_ctx_fexit, skb: *mut c_void) -> c_int {
    int BPF_PROG(skb_invalid_ctx_fexit, void *skb)
    {
    struct bpf_dynptr ptr;
// this should fail
    bpf_dynptr_from_skb(skb, 0, &ptr);
    return 0;
    }
// Reject writes to dynptr slot for uninit arg
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn uninit_write_into_slot(ctx: *mut c_void) -> c_int {
    int uninit_write_into_slot(void *ctx)
    {
    struct {
    char buf[64];
    struct bpf_dynptr ptr;
    } data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 80, 0, &data.ptr);
// this should fail
    bpf_get_current_comm(data.buf, 80);
    return 0;
    }
// Only supported prog type can create xdp-type dynptrs
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_dynptr_from_xdp is not) -> __failure {
    __failure __msg("calling kernel function bpf_dynptr_from_xdp is not allowed")
#[no_mangle]
pub unsafe extern "C" fn xdp_invalid_ctx(ctx: *mut c_void) -> c_int {
    int xdp_invalid_ctx(void *ctx)
    {
    struct bpf_dynptr ptr;
// this should fail
    bpf_dynptr_from_xdp(ctx, 0, &ptr);
    return 0;
    }
    let mut hdr_size: __u32 = sizeof(struct ethhdr);
// Can't pass in variable-sized len to bpf_dynptr_slice
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(constant": "must be a known) -> __failure {
    __failure __msg("must be a known constant")
    __msg("requires this memory size to be a verifier-known constant")
#[no_mangle]
pub unsafe extern "C" fn dynptr_slice_var_len1(skb: *mut __sk_buff) -> c_int {
    int dynptr_slice_var_len1(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    char buffer[sizeof(*hdr)] = {};
    bpf_dynptr_from_skb(skb, 0, &ptr);
// this should fail
    hdr = bpf_dynptr_slice(&ptr, 0, buffer, hdr_size);
    if (!hdr)
    return SK_DROP;
    return SK_PASS;
    }
// Can't pass in variable-sized len to bpf_dynptr_slice
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(constant": "must be a known) -> __failure {
    __failure __msg("must be a known constant")
    __msg("requires this memory size to be a verifier-known constant")
#[no_mangle]
pub unsafe extern "C" fn dynptr_slice_var_len2(skb: *mut __sk_buff) -> c_int {
    int dynptr_slice_var_len2(struct __sk_buff *skb)
    {
    char buffer[sizeof(struct ethhdr)] = {};
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    bpf_dynptr_from_skb(skb, 0, &ptr);
    if (hdr_size <= sizeof(buffer)) {
// this should fail
    hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, hdr_size);
    if (!hdr)
    return SK_DROP;
    hdr.h_proto = 12;
    }
    return SK_PASS;
    }
#[no_mangle]
unsafe extern "C" fn callback(index: __u32, data: *mut c_void) -> c_int {
    static int callback(__u32 index, void *data)
    {
// (__u32 *)data = 123;
    return 0;
    }
// A commuted add should preserve the parent id of a dynptr data slice.
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn dynptr_slice_commuted_invalidate(ctx: *mut c_void) -> c_int {
    int dynptr_slice_commuted_invalidate(void *ctx)
    {
    struct bpf_dynptr ptr;
    __u32 *slice, *derived;
    bpf_ringbuf_reserve_dynptr(&ringbuf, sizeof(__u32), 0, &ptr);
    slice = bpf_dynptr_data(&ptr, 0, sizeof(__u32));
    if (!slice)
    goto done;
    asm volatile ("%[dst] = 0;"
    "%[dst] += %[src];"
    "%[src] = 0;"
    : [dst]"=&r"(derived), [src]"+r"(slice)
    :
    : "memory");
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    val = *derived;
    return 0;
    done:
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// If the dynptr is written into in a callback function, its data
// slices should be invalidated as well.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn invalid_data_slices(ctx: *mut c_void) -> c_int {
    int invalid_data_slices(void *ctx)
    {
    struct bpf_dynptr ptr;
    __u32 *slice;
    if (get_map_val_dynptr(&ptr))
    return 0;
    slice = bpf_dynptr_data(&ptr, 0, sizeof(__u32));
    if (!slice)
    return 0;
    bpf_loop(10, callback, &ptr, 0);
// this should fail
// slice = 1;
    return 0;
    }
// Program types that don't allow writes to packet data should fail if
// bpf_dynptr_slice_rdwr is called
//
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn __msg(data": "the prog does not allow writes to packet) -> __failure {
    __failure __msg("the prog does not allow writes to packet data")
#[no_mangle]
pub unsafe extern "C" fn invalid_slice_rdwr_rdonly(skb: *mut __sk_buff) -> c_int {
    int invalid_slice_rdwr_rdonly(struct __sk_buff *skb)
    {
    char buffer[sizeof(struct ethhdr)] = {};
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    bpf_dynptr_from_skb(skb, 0, &ptr);
// this should fail since cgroup_skb doesn't allow
// changing packet data
//
    hdr = bpf_dynptr_slice_rdwr(&ptr, 0, buffer, sizeof(buffer));
    __sink(hdr);
    return 0;
    }
// bpf_dynptr_adjust can only be called on initialized dynptrs
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn dynptr_adjust_invalid(ctx: *mut c_void) -> c_int {
    int dynptr_adjust_invalid(void *ctx)
    {
    let mut ptr: bpf_dynptr = {};
// this should fail
    bpf_dynptr_adjust(&ptr, 1, 2);
    return 0;
    }
// bpf_dynptr_is_null can only be called on initialized dynptrs
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn dynptr_is_null_invalid(ctx: *mut c_void) -> c_int {
    int dynptr_is_null_invalid(void *ctx)
    {
    let mut ptr: bpf_dynptr = {};
// this should fail
    bpf_dynptr_is_null(&ptr);
    return 0;
    }
// bpf_dynptr_is_rdonly can only be called on initialized dynptrs
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn dynptr_is_rdonly_invalid(ctx: *mut c_void) -> c_int {
    int dynptr_is_rdonly_invalid(void *ctx)
    {
    let mut ptr: bpf_dynptr = {};
// this should fail
    bpf_dynptr_is_rdonly(&ptr);
    return 0;
    }
// bpf_dynptr_size can only be called on initialized dynptrs
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn dynptr_size_invalid(ctx: *mut c_void) -> c_int {
    int dynptr_size_invalid(void *ctx)
    {
    let mut ptr: bpf_dynptr = {};
// this should fail
    bpf_dynptr_size(&ptr);
    return 0;
    }
// Only initialized dynptrs can be cloned
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn clone_invalid1(ctx: *mut c_void) -> c_int {
    int clone_invalid1(void *ctx)
    {
    let mut ptr1: bpf_dynptr = {};
    struct bpf_dynptr ptr2;
// this should fail
    bpf_dynptr_clone(&ptr1, &ptr2);
    return 0;
    }
// Can't overwrite an existing dynptr when cloning
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn clone_invalid2(xdp: *mut xdp_md) -> c_int {
    int clone_invalid2(struct xdp_md *xdp)
    {
    struct bpf_dynptr ptr1;
    struct bpf_dynptr clone;
    bpf_dynptr_from_xdp(xdp, 0, &ptr1);
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &clone);
// this should fail
    bpf_dynptr_clone(&ptr1, &clone);
    bpf_ringbuf_submit_dynptr(&clone, 0);
    return 0;
    }
// Invalidating a dynptr should invalidate its clones
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R3": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R3")
#[no_mangle]
pub unsafe extern "C" fn clone_invalidate1(ctx: *mut c_void) -> c_int {
    int clone_invalidate1(void *ctx)
    {
    struct bpf_dynptr clone;
    struct bpf_dynptr ptr;
    char read_data[64];
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
    bpf_ringbuf_submit_dynptr(&ptr, 0);
// this should fail
    bpf_dynptr_read(read_data, sizeof(read_data), &clone, 0, 0);
    return 0;
    }
// Invalidating a dynptr should invalidate its parent
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R3": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R3")
#[no_mangle]
pub unsafe extern "C" fn clone_invalidate2(ctx: *mut c_void) -> c_int {
    int clone_invalidate2(void *ctx)
    {
    struct bpf_dynptr ptr;
    struct bpf_dynptr clone;
    char read_data[64];
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
    bpf_ringbuf_submit_dynptr(&clone, 0);
// this should fail
    bpf_dynptr_read(read_data, sizeof(read_data), &ptr, 0, 0);
    return 0;
    }
// Invalidating a dynptr should invalidate its siblings
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R3": "Expected an initialized dynptr as) -> __failure {
    __failure __msg("Expected an initialized dynptr as R3")
#[no_mangle]
pub unsafe extern "C" fn clone_invalidate3(ctx: *mut c_void) -> c_int {
    int clone_invalidate3(void *ctx)
    {
    struct bpf_dynptr ptr;
    struct bpf_dynptr clone1;
    struct bpf_dynptr clone2;
    char read_data[64];
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone1);
    bpf_dynptr_clone(&ptr, &clone2);
    bpf_ringbuf_submit_dynptr(&clone2, 0);
// this should fail
    bpf_dynptr_read(read_data, sizeof(read_data), &clone1, 0, 0);
    return 0;
    }
// Invalidating a dynptr should invalidate any data slices
// of its clones
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn clone_invalidate4(ctx: *mut c_void) -> c_int {
    int clone_invalidate4(void *ctx)
    {
    struct bpf_dynptr ptr;
    struct bpf_dynptr clone;
    int *data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
    data = bpf_dynptr_data(&clone, 0, sizeof(val));
    if (!data)
    return 0;
    bpf_ringbuf_submit_dynptr(&ptr, 0);
// this should fail
// data = 123;
    return 0;
    }
// Invalidating a dynptr should invalidate any data slices
// of its parent
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn clone_invalidate5(ctx: *mut c_void) -> c_int {
    int clone_invalidate5(void *ctx)
    {
    struct bpf_dynptr ptr;
    struct bpf_dynptr clone;
    int *data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    data = bpf_dynptr_data(&ptr, 0, sizeof(val));
    if (!data)
    return 0;
    bpf_dynptr_clone(&ptr, &clone);
    bpf_ringbuf_submit_dynptr(&clone, 0);
// this should fail
// data = 123;
    return 0;
    }
// Invalidating a dynptr should invalidate any data slices
// of its sibling
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn clone_invalidate6(ctx: *mut c_void) -> c_int {
    int clone_invalidate6(void *ctx)
    {
    struct bpf_dynptr ptr;
    struct bpf_dynptr clone1;
    struct bpf_dynptr clone2;
    int *data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone1);
    bpf_dynptr_clone(&ptr, &clone2);
    data = bpf_dynptr_data(&clone1, 0, sizeof(val));
    if (!data)
    return 0;
    bpf_ringbuf_submit_dynptr(&clone2, 0);
// this should fail
// data = 123;
    return 0;
    }
// A skb clone's data slices should be invalid anytime packet data changes
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn clone_skb_packet_data(skb: *mut __sk_buff) -> c_int {
    int clone_skb_packet_data(struct __sk_buff *skb)
    {
    char buffer[sizeof(__u32)] = {};
    struct bpf_dynptr clone;
    struct bpf_dynptr ptr;
    __u32 *data;
    bpf_dynptr_from_skb(skb, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
    data = bpf_dynptr_slice_rdwr(&clone, 0, buffer, sizeof(buffer));
    if (!data)
    return XDP_DROP;
    if (bpf_skb_pull_data(skb, skb.len))
    return SK_DROP;
// this should fail
// data = 123;
    return 0;
    }
// A skb clone's metadata slice becomes invalid anytime packet data changes
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn clone_skb_packet_meta(skb: *mut __sk_buff) -> c_int {
    int clone_skb_packet_meta(struct __sk_buff *skb)
    {
    struct bpf_dynptr clone, meta;
    __u8 *md;
    bpf_dynptr_from_skb_meta(skb, 0, &meta);
    bpf_dynptr_clone(&meta, &clone);
    md = bpf_dynptr_slice_rdwr(&clone, 0, core::ptr::null_mut(), sizeof(*md));
    if (!md)
    return SK_DROP;
    if (bpf_skb_pull_data(skb, skb.len))
    return SK_DROP;
// this should fail
// md = 42;
    return 0;
    }
// A xdp clone's data slices should be invalid anytime packet data changes
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn clone_xdp_packet_data(xdp: *mut xdp_md) -> c_int {
    int clone_xdp_packet_data(struct xdp_md *xdp)
    {
    char buffer[sizeof(__u32)] = {};
    struct bpf_dynptr clone;
    struct bpf_dynptr ptr;
    struct ethhdr *hdr;
    __u32 *data;
    bpf_dynptr_from_xdp(xdp, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
    data = bpf_dynptr_slice_rdwr(&clone, 0, buffer, sizeof(buffer));
    if (!data)
    return XDP_DROP;
    if (bpf_xdp_adjust_head(xdp, 0 - (int)sizeof(*hdr)))
    return XDP_DROP;
// this should fail
// data = 123;
    return 0;
    }
// Buffers that are provided must be sufficiently long
    SEC("?cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn __msg(_arg: "memory, access": len pair leads to invalid memory) -> __failure {
    __failure __msg("memory, len pair leads to invalid memory access")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_skb_small_buff(skb: *mut __sk_buff) -> c_int {
    int test_dynptr_skb_small_buff(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    char buffer[8] = {};
    __u64 *data;
    if (bpf_dynptr_from_skb(skb, 0, &ptr)) {
    err = 1;
    return 1;
    }
// This may return NULL. SKB may require a buffer
    data = bpf_dynptr_slice(&ptr, 0, buffer, 9);
    return !!data;
    }
#[no_mangle]
pub unsafe extern "C" fn global_call_bpf_dynptr(dynptr: *const bpf_dynptr) -> __noinline long {
    __noinline long global_call_bpf_dynptr(const struct bpf_dynptr *dynptr)
    {
    let mut ret: c_long = 0;
// Avoid leaving this global function empty to avoid having the compiler
// optimize away the call to this global function.
//
    __sink(ret);
    return ret;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_dynptr": "R1 expected pointer to stack or struct) -> __failure {
    __failure __msg("R1 expected pointer to stack or const struct bpf_dynptr")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_reg_type(ctx: *mut c_void) -> c_int {
    int test_dynptr_reg_type(void *ctx)
    {
    struct task_struct *current = core::ptr::null_mut();
// R1 should be holding a PTR_TO_BTF_ID, so this shouldn't be a
// reg->type that can be passed to a function accepting a
// ARG_PTR_TO_DYNPTR | MEM_RDONLY. process_dynptr_func() should catch
// this.
//
    global_call_bpf_dynptr((const struct bpf_dynptr *)current);
    return 0;
    }
// Overwriting a referenced dynptr is allowed if a clone still holds the ref
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn dynptr_overwrite_ref_with_clone(ctx: *mut c_void) -> c_int {
    int dynptr_overwrite_ref_with_clone(void *ctx)
    {
    struct bpf_dynptr ptr, clone;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
// Overwrite the original - clone still holds the ref
// (volatile __u8 *)&ptr = 0;
    bpf_ringbuf_discard_dynptr(&clone, 0);
    return 0;
    }
// Overwriting the last referenced dynptr should still be rejected
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "cannot overwrite referenced) -> __failure {
    __failure __msg("cannot overwrite referenced dynptr")
#[no_mangle]
pub unsafe extern "C" fn dynptr_overwrite_ref_last_clone(ctx: *mut c_void) -> c_int {
    int dynptr_overwrite_ref_last_clone(void *ctx)
    {
    struct bpf_dynptr ptr, clone;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
// Overwrite the original - clone still holds the ref, OK
// (volatile __u8 *)&ptr = 0;
// Overwrite the last holder - this should fail
// (volatile __u8 *)&clone = 0;
    return 0;
    }
// Overwriting a clone should be allowed if the original still holds the ref
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn dynptr_overwrite_clone_with_original(ctx: *mut c_void) -> c_int {
    int dynptr_overwrite_clone_with_original(void *ctx)
    {
    struct bpf_dynptr ptr, clone;
    bpf_ringbuf_reserve_dynptr(&ringbuf, 64, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
// Overwrite the clone - original still holds the ref
// (volatile __u8 *)&clone = 0;
    bpf_ringbuf_discard_dynptr(&ptr, 0);
    return 0;
    }
// Data slices from the destroyed dynptr should be invalidated
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn dynptr_overwrite_ref_invalidate_slice(ctx: *mut c_void) -> c_int {
    int dynptr_overwrite_ref_invalidate_slice(void *ctx)
    {
    struct bpf_dynptr ptr, clone;
    int *data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    data = bpf_dynptr_data(&ptr, 0, sizeof(val));
    if (!data)
    return 0;
    bpf_dynptr_clone(&ptr, &clone);
// Overwrite the original - clone holds the ref
// (volatile __u8 *)&ptr = 0;
// data was from the original dynptr, should be invalid now
// data = 123;
    return 0;
    }
//
// Data slices from a dynptr clone should remain valid after
// overwriting the original dynptr
//
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn dynptr_overwrite_ref_clone_slice_valid(ctx: *mut c_void) -> c_int {
    int dynptr_overwrite_ref_clone_slice_valid(void *ctx)
    {
    struct bpf_dynptr ptr, clone;
    int *data;
    bpf_ringbuf_reserve_dynptr(&ringbuf, val, 0, &ptr);
    bpf_dynptr_clone(&ptr, &clone);
    data = bpf_dynptr_data(&clone, 0, sizeof(val));
    if (!data) {
    bpf_ringbuf_discard_dynptr(&clone, 0);
    return 0;
    }
// Overwrite the original - clone holds the ref
// (volatile __u8 *)&ptr = 0;
// data is from the clone, should still be valid
// data = 123;
    bpf_ringbuf_discard_dynptr(&clone, 0);
    return 0;
    }
