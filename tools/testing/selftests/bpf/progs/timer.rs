//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/timer.c
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
// Copyright (c) 2021 Facebook

pub const CLOCK_MONOTONIC: c_int = 1;
pub const CLOCK_BOOTTIME: c_int = 7;
    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmap_elem {
    pub counter: c_int,
    pub timer: bpf_timer,
    pub /: *mut *mut bpf_spin_lock lock; / unused,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1000);
    __type(key, int);
    __type(value, struct hmap_elem);
    } hmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __uint(max_entries, 1000);
    __type(key, int);
    __type(value, struct hmap_elem);
    } hmap_malloc SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub t: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, int);
    __type(value, struct elem);
    } array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 4);
    __type(key, int);
    __type(value, struct elem);
    } lru SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } abs_timer SEC(".maps"), soft_timer_pinned SEC(".maps"), abs_timer_pinned SEC(".maps"),
    race_array SEC(".maps");
    __u64 bss_data;
    __u64 abs_data;
    __u64 err;
    __u64 ok;
    __u64 test_hits;
    __u64 update_hits;
    __u64 cancel_hits;
    let mut callback_check: __u64 = 52;
    let mut callback2_check: __u64 = 52;
    __u64 pinned_callback_check;
    __s32 pinned_cpu;
    let mut async_cancel: bool = 0;
pub const ARRAY: c_int = 1;
pub const HTAB: c_int = 2;
pub const HTAB_MALLOC: c_int = 3;
pub const LRU: c_int = 4;
// callback for array and lru timers
#[no_mangle]
unsafe extern "C" fn timer_cb1(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_cb1(void *map, int *key, struct bpf_timer *timer)
    {
// increment bss variable twice.
// Once via array timer callback and once via lru timer callback
//
    bss_data += 5;
// *key == 0 - the callback was called for array timer.
// *key == 4 - the callback was called from lru timer.
//
    if (*key == ARRAY) {
    struct bpf_timer *lru_timer;
    let mut lru_key: c_int = LRU;
// rearm array timer to be called again in ~35 seconds
    if (bpf_timer_start(timer, 1ull << 35, 0) != 0)
    err |= 1;
    lru_timer = bpf_map_lookup_elem(&lru, &lru_key);
    if (!lru_timer)
    return 0;
    bpf_timer_set_callback(lru_timer, timer_cb1);
    if (bpf_timer_start(lru_timer, 0, 0) != 0)
    err |= 2;
    } else if (*key == LRU) {
    int lru_key, i;
    for (i = LRU + 1;
    i <= 100  /* for current LRU eviction algorithm this number
// should be larger than ~ lru->max_entries * 2
// ;
    i++) {
    let mut init: elem = {};
// lru_key cannot be used as loop induction variable
// otherwise the loop will be unbounded.
//
    lru_key = i;
// add more elements into lru map to push out current
// element and force deletion of this timer
//
    bpf_map_update_elem(map, &lru_key, &init, 0);
// look it up to bump it into active list
    bpf_map_lookup_elem(map, &lru_key);
// keep adding until *key changes underneath,
// which means that key/timer memory was reused
//
    if (*key != LRU)
    break;
    }
// check that the timer was removed
    if (bpf_timer_cancel(timer) != -EINVAL)
    err |= 4;
    ok |= 1;
    }
    return 0;
    }
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test1, _arg: c_int, _arg: a) -> c_int {
    int BPF_PROG2(test1, int, a)
    {
    struct bpf_timer *arr_timer, *lru_timer;
    let mut init: elem = {};
    let mut lru_key: c_int = LRU;
    let mut array_key: c_int = ARRAY;
    arr_timer = bpf_map_lookup_elem(&array, &array_key);
    if (!arr_timer)
    return 0;
    bpf_timer_init(arr_timer, &array, CLOCK_MONOTONIC);
    bpf_map_update_elem(&lru, &lru_key, &init, 0);
    lru_timer = bpf_map_lookup_elem(&lru, &lru_key);
    if (!lru_timer)
    return 0;
    bpf_timer_init(lru_timer, &lru, CLOCK_MONOTONIC);
    bpf_timer_set_callback(arr_timer, timer_cb1);
    bpf_timer_start(arr_timer, 0 /* call timer_cb1 asap */, 0);
// init more timers to check that array destruction
// doesn't leak timer memory.
//
    array_key = 0;
    arr_timer = bpf_map_lookup_elem(&array, &array_key);
    if (!arr_timer)
    return 0;
    bpf_timer_init(arr_timer, &array, CLOCK_MONOTONIC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timer_error(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_error(void *map, int *key, struct bpf_timer *timer)
    {
    err = 42;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_async_cancel_succeed(ctx: *mut c_void) -> c_int {
    int test_async_cancel_succeed(void *ctx)
    {
    struct bpf_timer *arr_timer;
    let mut array_key: c_int = ARRAY;
    arr_timer = bpf_map_lookup_elem(&array, &array_key);
    if (!arr_timer)
    return 0;
    bpf_timer_init(arr_timer, &array, CLOCK_MONOTONIC);
    bpf_timer_set_callback(arr_timer, timer_error);
    bpf_timer_start(arr_timer, 100000 /* 100us */, 0);
    bpf_timer_cancel_async(arr_timer);
    ok = 7;
    return 0;
    }
// callback for prealloc and non-prealloca hashtab timers
#[no_mangle]
unsafe extern "C" fn timer_cb2(map: *mut c_void, key: *mut c_int, val: *mut hmap_elem) -> c_int {
    static int timer_cb2(void *map, int *key, struct hmap_elem *val)
    {
    if (*key == HTAB)
    callback_check--;
    else
    callback2_check--;
    if (val.counter > 0 && --val.counter) {
// re-arm the timer again to execute after 1 usec
    bpf_timer_start(&val.timer, 1000, 0);
    } else if (*key == HTAB) {
    struct bpf_timer *arr_timer;
    let mut array_key: c_int = ARRAY;
// cancel arr_timer otherwise bpf_fentry_test1 prog
// will stay alive forever.
//
    arr_timer = bpf_map_lookup_elem(&array, &array_key);
    if (!arr_timer)
    return 0;
    if (bpf_timer_cancel(arr_timer) != 1)
// bpf_timer_cancel should return 1 to indicate
// that arr_timer was active at this time
//
    err |= 8;
// try to cancel ourself. It shouldn't deadlock.
    if (bpf_timer_cancel(&val.timer) != -EDEADLK)
    err |= 16;
// delete this key and this timer anyway.
// It shouldn't deadlock either.
//
    bpf_map_delete_elem(map, key);
// in preallocated hashmap both 'key' and 'val' could have been
// reused to store another map element (like in LRU above),
// but in controlled test environment the below test works.
// It's not a use-after-free. The memory is owned by the map.
//
    if (bpf_timer_start(&val.timer, 1000, 0) != -EINVAL)
    err |= 32;
    ok |= 2;
    } else {
    if (*key != HTAB_MALLOC)
    err |= 64;
// try to cancel ourself. It shouldn't deadlock.
    if (bpf_timer_cancel(&val.timer) != -EDEADLK)
    err |= 128;
// delete this key and this timer anyway.
// It shouldn't deadlock either.
//
    bpf_map_delete_elem(map, key);
    ok |= 4;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_timer_test() -> c_int {
    int bpf_timer_test(void)
    {
    struct hmap_elem *val;
    let mut key: c_int = HTAB, key_malloc = HTAB_MALLOC;
    val = bpf_map_lookup_elem(&hmap, &key);
    if (val) {
    if (bpf_timer_init(&val.timer, &hmap, CLOCK_BOOTTIME) != 0)
    err |= 512;
    bpf_timer_set_callback(&val.timer, timer_cb2);
    bpf_timer_start(&val.timer, 1000, 0);
    }
    val = bpf_map_lookup_elem(&hmap_malloc, &key_malloc);
    if (val) {
    if (bpf_timer_init(&val.timer, &hmap_malloc, CLOCK_BOOTTIME) != 0)
    err |= 1024;
    bpf_timer_set_callback(&val.timer, timer_cb2);
    bpf_timer_start(&val.timer, 1000, 0);
    }
    return 0;
    }
    SEC("fentry/bpf_fentry_test2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test2, _arg: c_int, _arg: a, _arg: c_int, _arg: b) -> c_int {
    int BPF_PROG2(test2, int, a, int, b)
    {
    let mut init: hmap_elem = {}, *val;
    let mut key: c_int = HTAB, key_malloc = HTAB_MALLOC;
    init.counter = 10; /* number of times to trigger timer_cb2 */
    bpf_map_update_elem(&hmap, &key, &init, 0);
    val = bpf_map_lookup_elem(&hmap, &key);
    if (val)
    bpf_timer_init(&val.timer, &hmap, CLOCK_BOOTTIME);
// update the same key to free the timer
    bpf_map_update_elem(&hmap, &key, &init, 0);
    bpf_map_update_elem(&hmap_malloc, &key_malloc, &init, 0);
    val = bpf_map_lookup_elem(&hmap_malloc, &key_malloc);
    if (val)
    bpf_timer_init(&val.timer, &hmap_malloc, CLOCK_BOOTTIME);
// update the same key to free the timer
    bpf_map_update_elem(&hmap_malloc, &key_malloc, &init, 0);
// init more timers to check that htab operations
// don't leak timer memory.
//
    key = 0;
    bpf_map_update_elem(&hmap, &key, &init, 0);
    val = bpf_map_lookup_elem(&hmap, &key);
    if (val)
    bpf_timer_init(&val.timer, &hmap, CLOCK_BOOTTIME);
    bpf_map_delete_elem(&hmap, &key);
    bpf_map_update_elem(&hmap, &key, &init, 0);
    val = bpf_map_lookup_elem(&hmap, &key);
    if (val)
    bpf_timer_init(&val.timer, &hmap, CLOCK_BOOTTIME);
// and with non-prealloc htab
    key_malloc = 0;
    bpf_map_update_elem(&hmap_malloc, &key_malloc, &init, 0);
    val = bpf_map_lookup_elem(&hmap_malloc, &key_malloc);
    if (val)
    bpf_timer_init(&val.timer, &hmap_malloc, CLOCK_BOOTTIME);
    bpf_map_delete_elem(&hmap_malloc, &key_malloc);
    bpf_map_update_elem(&hmap_malloc, &key_malloc, &init, 0);
    val = bpf_map_lookup_elem(&hmap_malloc, &key_malloc);
    if (val)
    bpf_timer_init(&val.timer, &hmap_malloc, CLOCK_BOOTTIME);
    return bpf_timer_test();
    }
// callback for absolute timer
#[no_mangle]
unsafe extern "C" fn timer_cb3(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_cb3(void *map, int *key, struct bpf_timer *timer)
    {
    abs_data += 6;
    if (abs_data < 12) {
    bpf_timer_start(timer, bpf_ktime_get_boot_ns() + 1000,
    BPF_F_TIMER_ABS);
    } else {
// Re-arm timer ~35 seconds in future
    bpf_timer_start(timer, bpf_ktime_get_boot_ns() + (1ull << 35),
    BPF_F_TIMER_ABS);
    }
    return 0;
    }
    SEC("fentry/bpf_fentry_test3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test3, _arg: c_int, _arg: a) -> c_int {
    int BPF_PROG2(test3, int, a)
    {
    let mut key: c_int = 0;
    struct bpf_timer *timer;
    bpf_printk("test3");
    timer = bpf_map_lookup_elem(&abs_timer, &key);
    if (timer) {
    if (bpf_timer_init(timer, &abs_timer, CLOCK_BOOTTIME) != 0)
    err |= 2048;
    bpf_timer_set_callback(timer, timer_cb3);
    bpf_timer_start(timer, bpf_ktime_get_boot_ns() + 1000,
    BPF_F_TIMER_ABS);
    }
    return 0;
    }
// callback for pinned timer
#[no_mangle]
unsafe extern "C" fn timer_cb_pinned(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_cb_pinned(void *map, int *key, struct bpf_timer *timer)
    {
    let mut cpu: __s32 = bpf_get_smp_processor_id();
    if (cpu != pinned_cpu)
    err |= 16384;
    pinned_callback_check++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_pinned_timer(soft: bool) {
    static void test_pinned_timer(bool soft)
    {
    let mut key: c_int = 0;
    void *map;
    struct bpf_timer *timer;
    let mut flags: __u64 = BPF_F_TIMER_CPU_PIN;
    __u64 start_time;
    if (soft) {
    map = &soft_timer_pinned;
    start_time = 0;
    } else {
    map = &abs_timer_pinned;
    start_time = bpf_ktime_get_boot_ns();
    flags |= BPF_F_TIMER_ABS;
    }
    timer = bpf_map_lookup_elem(map, &key);
    if (timer) {
    if (bpf_timer_init(timer, map, CLOCK_BOOTTIME) != 0)
    err |= 4096;
    bpf_timer_set_callback(timer, timer_cb_pinned);
    pinned_cpu = bpf_get_smp_processor_id();
    bpf_timer_start(timer, start_time + 1000, flags);
    } else {
    err |= 8192;
    }
    }
    SEC("fentry/bpf_fentry_test4")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test4, _arg: c_int, _arg: a) -> c_int {
    int BPF_PROG2(test4, int, a)
    {
    bpf_printk("test4");
    test_pinned_timer(true);
    return 0;
    }
    SEC("fentry/bpf_fentry_test5")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test5, _arg: c_int, _arg: a) -> c_int {
    int BPF_PROG2(test5, int, a)
    {
    bpf_printk("test5");
    test_pinned_timer(false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn race_timer_callback(race_array: *mut c_void, race_key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int race_timer_callback(void *race_array, int *race_key, struct bpf_timer *timer)
    {
    bpf_timer_start(timer, 1000000, 0);
    return 0;
    }
// Callback that updates its own map element
#[no_mangle]
unsafe extern "C" fn update_self_callback(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int update_self_callback(void *map, int *key, struct bpf_timer *timer)
    {
    let mut init: elem = {};
    bpf_map_update_elem(map, key, &init, BPF_ANY);
    __sync_fetch_and_add(&update_hits, 1);
    return 0;
    }
// Callback that cancels itself using async cancel
#[no_mangle]
unsafe extern "C" fn cancel_self_callback(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int cancel_self_callback(void *map, int *key, struct bpf_timer *timer)
    {
    bpf_timer_cancel_async(timer);
    __sync_fetch_and_add(&cancel_hits, 1);
    return 0;
    }
    enum test_mode {
    TEST_RACE_SYNC,
    TEST_RACE_ASYNC,
    TEST_UPDATE,
    TEST_CANCEL,
    };
#[no_mangle]
unsafe extern "C" fn test_common(mode: enum test_mode) -> __always_inline int {
    static __always_inline int test_common(enum test_mode mode)
    {
    struct bpf_timer *timer;
    struct elem init;
    int ret, key = 0;
    __builtin_memset(&init, 0, sizeof(struct elem));
    bpf_map_update_elem(&race_array, &key, &init, BPF_ANY);
    timer = bpf_map_lookup_elem(&race_array, &key);
    if (!timer)
    return 0;
    ret = bpf_timer_init(timer, &race_array, CLOCK_MONOTONIC);
    if (ret && ret != -EBUSY)
    return 0;
    if (mode == TEST_RACE_SYNC || mode == TEST_RACE_ASYNC)
    bpf_timer_set_callback(timer, race_timer_callback);
#[no_mangle]
pub unsafe extern "C" fn if(TEST_UPDATE: mode ==) -> else {
    else if (mode == TEST_UPDATE)
    bpf_timer_set_callback(timer, update_self_callback);
    else
    bpf_timer_set_callback(timer, cancel_self_callback);
    bpf_timer_start(timer, 0, 0);
    if (mode == TEST_RACE_ASYNC)
    bpf_timer_cancel_async(timer);
#[no_mangle]
pub unsafe extern "C" fn if(TEST_RACE_SYNC: mode ==) -> else {
    else if (mode == TEST_RACE_SYNC)
    bpf_timer_cancel(timer);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn race(ctx: *mut c_void) -> c_int {
    int race(void *ctx)
    {
    return test_common(async_cancel ? TEST_RACE_ASYNC : TEST_RACE_SYNC);
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn nmi_race(ctx: *mut c_void) -> c_int {
    int nmi_race(void *ctx)
    {
    __sync_fetch_and_add(&test_hits, 1);
    return test_common(TEST_RACE_ASYNC);
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn nmi_update(ctx: *mut c_void) -> c_int {
    int nmi_update(void *ctx)
    {
    __sync_fetch_and_add(&test_hits, 1);
    return test_common(TEST_UPDATE);
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn nmi_cancel(ctx: *mut c_void) -> c_int {
    int nmi_cancel(void *ctx)
    {
    __sync_fetch_and_add(&test_hits, 1);
    return test_common(TEST_CANCEL);
    }
