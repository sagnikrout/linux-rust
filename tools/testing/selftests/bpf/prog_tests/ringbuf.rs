//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ringbuf.c
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
// Macro flag: #define _GNU_SOURCE

pub const EDONE: c_int = 7777;
    let mut duration: static int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample {
    pub pid: c_int,
    pub seq: c_int,
    pub value: c_long,
    pub comm: [c_char; 16],
}

    static int sample_cnt;
#[no_mangle]
unsafe extern "C" fn atomic_inc(cnt: *mut c_int) {
    static void atomic_inc(int *cnt)
    {
    __atomic_add_fetch(cnt, 1, __ATOMIC_SEQ_CST);
    }
#[no_mangle]
unsafe extern "C" fn atomic_xchg(cnt: *mut c_int, val: c_int) -> c_int {
    static int atomic_xchg(int *cnt, int val)
    {
    return __atomic_exchange_n(cnt, val, __ATOMIC_SEQ_CST);
    }
#[no_mangle]
unsafe extern "C" fn process_sample(ctx: *mut c_void, data: *mut c_void, len: usize) -> c_int {
    static int process_sample(void *ctx, void *data, size_t len)
    {
    struct sample *s = data;
    atomic_inc(&sample_cnt);
    switch (s.seq) {
    case 0:
    CHECK(s.value != 333, "sample1_value", "exp %ld, got %ld\n",
    333L, s.value);
    return 0;
    case 1:
    CHECK(s.value != 777, "sample2_value", "exp %ld, got %ld\n",
    777L, s.value);
    return -EDONE;
    default:
// we don't care about the rest
    return 0;
    }
    }
    static struct test_ringbuf_map_key_lskel *skel_map_key;
    static struct test_ringbuf_lskel *skel;
    static struct ring_buffer *ringbuf;
#[no_mangle]
unsafe extern "C" fn trigger_samples() {
    static void trigger_samples()
    {
    skel.bss.dropped = 0;
    skel.bss.total = 0;
    skel.bss.discarded = 0;
// trigger exactly two samples
    skel.bss.value = 333;
    syscall(__NR_getpgid);
    skel.bss.value = 777;
    syscall(__NR_getpgid);
    }
    static void *poll_thread(void *input)
    {
    let mut timeout: c_long = (long)input;
    return (void *)(long)ring_buffer__poll(ringbuf, timeout);
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_write_subtest() {
    static void ringbuf_write_subtest(void)
    {
    struct test_ringbuf_write_lskel *skel;
    let mut page_size: c_int = getpagesize();
    size_t *mmap_ptr;
    int err, rb_fd;
    skel = test_ringbuf_write_lskel__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.maps.ringbuf.max_entries = 0x40000;
    err = test_ringbuf_write_lskel__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    rb_fd = skel.maps.ringbuf.map_fd;
    mmap_ptr = mmap(core::ptr::null_mut(), page_size, PROT_READ | PROT_WRITE, MAP_SHARED, rb_fd, 0);
    if (!ASSERT_OK_PTR(mmap_ptr, "rw_cons_pos"))
    goto cleanup;
// mmap_ptr = 0x30000;
    ASSERT_OK(munmap(mmap_ptr, page_size), "unmap_rw");
    skel.bss.pid = getpid();
    ringbuf = ring_buffer__new(rb_fd, process_sample, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_OK_PTR(ringbuf, "ringbuf_new"))
    goto cleanup;
    err = test_ringbuf_write_lskel__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup_ringbuf;
    skel.bss.discarded = 0;
    skel.bss.passed = 0;
// trigger exactly two samples
    syscall(__NR_getpgid);
    syscall(__NR_getpgid);
    ASSERT_EQ(skel.bss.discarded, 2, "discarded");
    ASSERT_EQ(skel.bss.passed, 0, "passed");
    test_ringbuf_write_lskel__detach(skel);
    cleanup_ringbuf:
    ring_buffer__free(ringbuf);
    cleanup:
    test_ringbuf_write_lskel__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_subtest() {
    static void ringbuf_subtest(void)
    {
    let mut rec_sz: usize = BPF_RINGBUF_HDR_SZ + sizeof(struct sample);
    pthread_t thread;
    let mut bg_ret: c_long = -1;
    int err, cnt, rb_fd;
    let mut page_size: c_int = getpagesize();
    void *mmap_ptr, *tmp_ptr;
    struct ring *ring;
    int map_fd;
    unsigned long avail_data, ring_size, cons_pos, prod_pos;
    skel = test_ringbuf_lskel__open();
    if (CHECK(!skel, "skel_open", "skeleton open failed\n"))
    return;
    skel.maps.ringbuf.max_entries = page_size;
    err = test_ringbuf_lskel__load(skel);
    if (CHECK(err != 0, "skel_load", "skeleton load failed\n"))
    goto cleanup;
    rb_fd = skel.maps.ringbuf.map_fd;
// good read/write cons_pos
    mmap_ptr = mmap(core::ptr::null_mut(), page_size, PROT_READ | PROT_WRITE, MAP_SHARED, rb_fd, 0);
    ASSERT_OK_PTR(mmap_ptr, "rw_cons_pos");
    tmp_ptr = mremap(mmap_ptr, page_size, 2 * page_size, MREMAP_MAYMOVE);
    if (!ASSERT_ERR_PTR(tmp_ptr, "rw_extend"))
    goto cleanup;
    ASSERT_ERR(mprotect(mmap_ptr, page_size, PROT_EXEC), "exec_cons_pos_protect");
    ASSERT_OK(munmap(mmap_ptr, page_size), "unmap_rw");
// bad writeable prod_pos
    mmap_ptr = mmap(core::ptr::null_mut(), page_size, PROT_WRITE, MAP_SHARED, rb_fd, page_size);
    err = -errno;
    ASSERT_ERR_PTR(mmap_ptr, "wr_prod_pos");
    ASSERT_EQ(err, -EPERM, "wr_prod_pos_err");
// bad writeable data pages
    mmap_ptr = mmap(core::ptr::null_mut(), page_size, PROT_WRITE, MAP_SHARED, rb_fd, 2 * page_size);
    err = -errno;
    ASSERT_ERR_PTR(mmap_ptr, "wr_data_page_one");
    ASSERT_EQ(err, -EPERM, "wr_data_page_one_err");
    mmap_ptr = mmap(core::ptr::null_mut(), page_size, PROT_WRITE, MAP_SHARED, rb_fd, 3 * page_size);
    ASSERT_ERR_PTR(mmap_ptr, "wr_data_page_two");
    mmap_ptr = mmap(core::ptr::null_mut(), 2 * page_size, PROT_WRITE, MAP_SHARED, rb_fd, 2 * page_size);
    ASSERT_ERR_PTR(mmap_ptr, "wr_data_page_all");
// good read-only pages
    mmap_ptr = mmap(core::ptr::null_mut(), 4 * page_size, PROT_READ, MAP_SHARED, rb_fd, 0);
    if (!ASSERT_OK_PTR(mmap_ptr, "ro_prod_pos"))
    goto cleanup;
    ASSERT_ERR(mprotect(mmap_ptr, 4 * page_size, PROT_WRITE), "write_protect");
    ASSERT_ERR(mprotect(mmap_ptr, 4 * page_size, PROT_EXEC), "exec_protect");
    ASSERT_ERR_PTR(mremap(mmap_ptr, 0, 4 * page_size, MREMAP_MAYMOVE), "ro_remap");
    ASSERT_OK(munmap(mmap_ptr, 4 * page_size), "unmap_ro");
// good read-only pages with initial offset
    mmap_ptr = mmap(core::ptr::null_mut(), page_size, PROT_READ, MAP_SHARED, rb_fd, page_size);
    if (!ASSERT_OK_PTR(mmap_ptr, "ro_prod_pos"))
    goto cleanup;
    ASSERT_ERR(mprotect(mmap_ptr, page_size, PROT_WRITE), "write_protect");
    ASSERT_ERR(mprotect(mmap_ptr, page_size, PROT_EXEC), "exec_protect");
    ASSERT_ERR_PTR(mremap(mmap_ptr, 0, 3 * page_size, MREMAP_MAYMOVE), "ro_remap");
    ASSERT_OK(munmap(mmap_ptr, page_size), "unmap_ro");
// only trigger BPF program for current process
    skel.bss.pid = getpid();
    ringbuf = ring_buffer__new(skel.maps.ringbuf.map_fd,
    process_sample, core::ptr::null_mut(), core::ptr::null_mut());
    if (CHECK(!ringbuf, "ringbuf_create", "failed to create ringbuf\n"))
    goto cleanup;
    err = test_ringbuf_lskel__attach(skel);
    if (CHECK(err, "skel_attach", "skeleton attachment failed: %d\n", err))
    goto cleanup;
    trigger_samples();
    ring = ring_buffer__ring(ringbuf, 0);
    if (!ASSERT_OK_PTR(ring, "ring_buffer__ring_idx_0"))
    goto cleanup;
    map_fd = ring__map_fd(ring);
    ASSERT_EQ(map_fd, skel.maps.ringbuf.map_fd, "ring_map_fd");
// 2 submitted + 1 discarded records
    CHECK(skel.bss.avail_data != 3 * rec_sz,
    "err_avail_size", "exp %ld, got %ld\n",
    3L * rec_sz, skel.bss.avail_data);
    CHECK(skel.bss.ring_size != page_size,
    "err_ring_size", "exp %ld, got %ld\n",
    (long)page_size, skel.bss.ring_size);
    CHECK(skel.bss.cons_pos != 0,
    "err_cons_pos", "exp %ld, got %ld\n",
    0L, skel.bss.cons_pos);
    CHECK(skel.bss.prod_pos != 3 * rec_sz,
    "err_prod_pos", "exp %ld, got %ld\n",
    3L * rec_sz, skel.bss.prod_pos);
// verify getting this data directly via the ring object yields the same
// results
//
    avail_data = ring__avail_data_size(ring);
    ASSERT_EQ(avail_data, 3 * rec_sz, "ring_avail_size");
    ring_size = ring__size(ring);
    ASSERT_EQ(ring_size, page_size, "ring_ring_size");
    cons_pos = ring__consumer_pos(ring);
    ASSERT_EQ(cons_pos, 0, "ring_cons_pos");
    prod_pos = ring__producer_pos(ring);
    ASSERT_EQ(prod_pos, 3 * rec_sz, "ring_prod_pos");
// poll for samples
    err = ring_buffer__poll(ringbuf, -1);
// -EDONE is used as an indicator that we are done
    if (CHECK(err != -EDONE, "err_done", "done err: %d\n", err))
    goto cleanup;
    cnt = atomic_xchg(&sample_cnt, 0);
    CHECK(cnt != 2, "cnt", "exp %d samples, got %d\n", 2, cnt);
// we expect extra polling to return nothing
    err = ring_buffer__poll(ringbuf, 0);
    if (CHECK(err != 0, "extra_samples", "poll result: %d\n", err))
    goto cleanup;
    cnt = atomic_xchg(&sample_cnt, 0);
    CHECK(cnt != 0, "cnt", "exp %d samples, got %d\n", 0, cnt);
    CHECK(skel.bss.dropped != 0, "err_dropped", "exp %ld, got %ld\n",
    0L, skel.bss.dropped);
    CHECK(skel.bss.total != 2, "err_total", "exp %ld, got %ld\n",
    2L, skel.bss.total);
    CHECK(skel.bss.discarded != 1, "err_discarded", "exp %ld, got %ld\n",
    1L, skel.bss.discarded);
// now validate consumer position is updated and returned
    trigger_samples();
    CHECK(skel.bss.cons_pos != 3 * rec_sz,
    "err_cons_pos", "exp %ld, got %ld\n",
    3L * rec_sz, skel.bss.cons_pos);
    err = ring_buffer__poll(ringbuf, -1);
    CHECK(err <= 0, "poll_err", "err %d\n", err);
    cnt = atomic_xchg(&sample_cnt, 0);
    CHECK(cnt != 2, "cnt", "exp %d samples, got %d\n", 2, cnt);
// start poll in background w/ long timeout
    err = pthread_create(&thread, core::ptr::null_mut(), poll_thread, (void *)(long)10000);
    if (CHECK(err, "bg_poll", "pthread_create failed: %d\n", err))
    goto cleanup;
// turn off notifications now
    skel.bss.flags = BPF_RB_NO_WAKEUP;
// give background thread a bit of a time
    usleep(50000);
    trigger_samples();
// sleeping arbitrarily is bad, but no better way to know that
// epoll_wait() **DID NOT** unblock in background thread
//
    usleep(50000);
// background poll should still be blocked
    err = pthread_tryjoin_np(thread, (void **)&bg_ret);
    if (CHECK(err != EBUSY, "try_join", "err %d\n", err))
    goto cleanup;
// BPF side did everything right
    CHECK(skel.bss.dropped != 0, "err_dropped", "exp %ld, got %ld\n",
    0L, skel.bss.dropped);
    CHECK(skel.bss.total != 2, "err_total", "exp %ld, got %ld\n",
    2L, skel.bss.total);
    CHECK(skel.bss.discarded != 1, "err_discarded", "exp %ld, got %ld\n",
    1L, skel.bss.discarded);
    cnt = atomic_xchg(&sample_cnt, 0);
    CHECK(cnt != 0, "cnt", "exp %d samples, got %d\n", 0, cnt);
// clear flags to return to "adaptive" notification mode
    skel.bss.flags = 0;
// produce new samples, no notification should be triggered, because
// consumer is now behind
//
    trigger_samples();
// background poll should still be blocked
    err = pthread_tryjoin_np(thread, (void **)&bg_ret);
    if (CHECK(err != EBUSY, "try_join", "err %d\n", err))
    goto cleanup;
// still no samples, because consumer is behind
    cnt = atomic_xchg(&sample_cnt, 0);
    CHECK(cnt != 0, "cnt", "exp %d samples, got %d\n", 0, cnt);
    skel.bss.dropped = 0;
    skel.bss.total = 0;
    skel.bss.discarded = 0;
    skel.bss.value = 333;
    syscall(__NR_getpgid);
// now force notifications
    skel.bss.flags = BPF_RB_FORCE_WAKEUP;
    skel.bss.value = 777;
    syscall(__NR_getpgid);
// now we should get a pending notification
    usleep(50000);
    err = pthread_tryjoin_np(thread, (void **)&bg_ret);
    if (CHECK(err, "join_bg", "err %d\n", err))
    goto cleanup;
    if (CHECK(bg_ret <= 0, "bg_ret", "epoll_wait result: %ld", bg_ret))
    goto cleanup;
// due to timing variations, there could still be non-notified
// samples, so consume them here to collect all the samples
//
    err = ring_buffer__consume(ringbuf);
    CHECK(err < 0, "rb_consume", "failed: %d\b", err);
// also consume using ring__consume to make sure it works the same
    err = ring__consume(ring);
    ASSERT_GE(err, 0, "ring_consume");
// 3 rounds, 2 samples each
    cnt = atomic_xchg(&sample_cnt, 0);
    CHECK(cnt != 6, "cnt", "exp %d samples, got %d\n", 6, cnt);
// BPF side did everything right
    CHECK(skel.bss.dropped != 0, "err_dropped", "exp %ld, got %ld\n",
    0L, skel.bss.dropped);
    CHECK(skel.bss.total != 2, "err_total", "exp %ld, got %ld\n",
    2L, skel.bss.total);
    CHECK(skel.bss.discarded != 1, "err_discarded", "exp %ld, got %ld\n",
    1L, skel.bss.discarded);
    test_ringbuf_lskel__detach(skel);
    cleanup:
    ring_buffer__free(ringbuf);
    test_ringbuf_lskel__destroy(skel);
    }
//
// Test ring_buffer__consume_n() by producing N_TOT_SAMPLES samples in the ring
// buffer, via getpid(), and consuming them in chunks of N_SAMPLES.
//
pub const N_TOT_SAMPLES: c_int = 32;
pub const N_SAMPLES: c_int = 4;
// Sample value to verify the callback validity

#[no_mangle]
unsafe extern "C" fn process_n_sample(ctx: *mut c_void, data: *mut c_void, len: usize) -> c_int {
    static int process_n_sample(void *ctx, void *data, size_t len)
    {
    struct sample *s = data;
    ASSERT_EQ(s.value, SAMPLE_VALUE, "sample_value");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_n_subtest() {
    static void ringbuf_n_subtest(void)
    {
    struct test_ringbuf_n_lskel *skel_n;
    int err, i;
    skel_n = test_ringbuf_n_lskel__open();
    if (!ASSERT_OK_PTR(skel_n, "test_ringbuf_n_lskel__open"))
    return;
    skel_n.maps.ringbuf.max_entries = getpagesize();
    skel_n.bss.pid = getpid();
    err = test_ringbuf_n_lskel__load(skel_n);
    if (!ASSERT_OK(err, "test_ringbuf_n_lskel__load"))
    goto cleanup;
    ringbuf = ring_buffer__new(skel_n.maps.ringbuf.map_fd,
    process_n_sample, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_OK_PTR(ringbuf, "ring_buffer__new"))
    goto cleanup;
    err = test_ringbuf_n_lskel__attach(skel_n);
    if (!ASSERT_OK(err, "test_ringbuf_n_lskel__attach"))
    goto cleanup_ringbuf;
// Produce N_TOT_SAMPLES samples in the ring buffer by calling getpid()
    skel_n.bss.value = SAMPLE_VALUE;
    for (i = 0; i < N_TOT_SAMPLES; i++)
    syscall(__NR_getpgid);
// Consume all samples from the ring buffer in batches of N_SAMPLES
    for (i = 0; i < N_TOT_SAMPLES; i += err) {
    err = ring_buffer__consume_n(ringbuf, N_SAMPLES);
    if (!ASSERT_EQ(err, N_SAMPLES, "rb_consume"))
    goto cleanup_ringbuf;
    }
    cleanup_ringbuf:
    ring_buffer__free(ringbuf);
    cleanup:
    test_ringbuf_n_lskel__destroy(skel_n);
    }
#[no_mangle]
unsafe extern "C" fn process_map_key_sample(ctx: *mut c_void, data: *mut c_void, len: usize) -> c_int {
    static int process_map_key_sample(void *ctx, void *data, size_t len)
    {
    struct sample *s;
    int err, val;
    s = data;
    switch (s.seq) {
    case 1:
    ASSERT_EQ(s.value, 42, "sample_value");
    err = bpf_map_lookup_elem(skel_map_key.maps.hash_map.map_fd,
    s, &val);
    ASSERT_OK(err, "hash_map bpf_map_lookup_elem");
    ASSERT_EQ(val, 1, "hash_map val");
    return -EDONE;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_map_key_subtest() {
    static void ringbuf_map_key_subtest(void)
    {
    int err;
    skel_map_key = test_ringbuf_map_key_lskel__open();
    if (!ASSERT_OK_PTR(skel_map_key, "test_ringbuf_map_key_lskel__open"))
    return;
    skel_map_key.maps.ringbuf.max_entries = getpagesize();
    skel_map_key.bss.pid = getpid();
    err = test_ringbuf_map_key_lskel__load(skel_map_key);
    if (!ASSERT_OK(err, "test_ringbuf_map_key_lskel__load"))
    goto cleanup;
    ringbuf = ring_buffer__new(skel_map_key.maps.ringbuf.map_fd,
    process_map_key_sample, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_OK_PTR(ringbuf, "ring_buffer__new"))
    goto cleanup;
    err = test_ringbuf_map_key_lskel__attach(skel_map_key);
    if (!ASSERT_OK(err, "test_ringbuf_map_key_lskel__attach"))
    goto cleanup_ringbuf;
    syscall(__NR_getpgid);
    ASSERT_EQ(skel_map_key.bss.seq, 1, "skel_map_key.bss.seq");
    err = ring_buffer__poll(ringbuf, -1);
    ASSERT_EQ(err, -EDONE, "ring_buffer__poll");
    cleanup_ringbuf:
    ring_buffer__free(ringbuf);
    cleanup:
    test_ringbuf_map_key_lskel__destroy(skel_map_key);
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_overwrite_mode_subtest() {
    static void ringbuf_overwrite_mode_subtest(void)
    {
    unsigned long size, len1, len2, len3, len4, len5;
    unsigned long expect_avail_data, expect_prod_pos, expect_over_pos;
    struct test_ringbuf_overwrite_lskel *skel;
    let mut page_size: c_int = getpagesize();
    int err;
    skel = test_ringbuf_overwrite_lskel__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    size = page_size;
    len1 = page_size / 2;
    len2 = page_size / 4;
    len3 = size - len1 - len2 - BPF_RINGBUF_HDR_SZ * 3;
    len4 = len3 - 8;
    len5 = len3; /* retry with len3 */
    skel.maps.ringbuf.max_entries = size;
    skel.rodata.LEN1 = len1;
    skel.rodata.LEN2 = len2;
    skel.rodata.LEN3 = len3;
    skel.rodata.LEN4 = len4;
    skel.rodata.LEN5 = len5;
    skel.bss.pid = getpid();
    err = test_ringbuf_overwrite_lskel__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    err = test_ringbuf_overwrite_lskel__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
    syscall(__NR_getpgid);
    ASSERT_EQ(skel.bss.reserve1_fail, 0, "reserve 1");
    ASSERT_EQ(skel.bss.reserve2_fail, 0, "reserve 2");
    ASSERT_EQ(skel.bss.reserve3_fail, 1, "reserve 3");
    ASSERT_EQ(skel.bss.reserve4_fail, 0, "reserve 4");
    ASSERT_EQ(skel.bss.reserve5_fail, 0, "reserve 5");
    ASSERT_EQ(skel.bss.ring_size, size, "check_ring_size");
    expect_avail_data = len2 + len4 + len5 + 3 * BPF_RINGBUF_HDR_SZ;
    ASSERT_EQ(skel.bss.avail_data, expect_avail_data, "check_avail_size");
    ASSERT_EQ(skel.bss.cons_pos, 0, "check_cons_pos");
    expect_prod_pos = len1 + len2 + len4 + len5 + 4 * BPF_RINGBUF_HDR_SZ;
    ASSERT_EQ(skel.bss.prod_pos, expect_prod_pos, "check_prod_pos");
    expect_over_pos = len1 + BPF_RINGBUF_HDR_SZ;
    ASSERT_EQ(skel.bss.over_pos, expect_over_pos, "check_over_pos");
    test_ringbuf_overwrite_lskel__detach(skel);
    cleanup:
    test_ringbuf_overwrite_lskel__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_ringbuf() {
    void test_ringbuf(void)
    {
    if (test__start_subtest("ringbuf"))
    ringbuf_subtest();
    if (test__start_subtest("ringbuf_n"))
    ringbuf_n_subtest();
    if (test__start_subtest("ringbuf_map_key"))
    ringbuf_map_key_subtest();
    if (test__start_subtest("ringbuf_write"))
    ringbuf_write_subtest();
    if (test__start_subtest("ringbuf_overwrite_mode"))
    ringbuf_overwrite_mode_subtest();
    }
