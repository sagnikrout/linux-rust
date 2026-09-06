//! Automatically rewritten from C to Rust
//! Source: tools/lib/bpf/ringbuf.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Ring buffer operations.
//
// Copyright (C) 2020 Facebook, Inc.
//

// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring {
    pub sample_cb: ring_buffer_sample_fn,
    pub ctx: *mut c_void,
    pub data: *mut c_void,
    pub consumer_pos: *mut c_ulong,
    pub producer_pos: *mut c_ulong,
    pub mask: c_ulong,
    pub map_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer {
    pub events: *mut epoll_event,
    pub rings: *mut ring,
    pub page_size: usize,
    pub epoll_fd: c_int,
    pub ring_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_ring_buffer {
    pub event: epoll_event,
    pub consumer_pos: *mut c_ulong,
    pub producer_pos: *mut c_ulong,
    pub data: *mut c_void,
    pub mask: c_ulong,
    pub page_size: usize,
    pub map_fd: c_int,
    pub epoll_fd: c_int,
}

// 8-byte ring buffer header structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ringbuf_hdr {
    pub len: __u32,
    pub pad: __u32,
}

#[no_mangle]
unsafe extern "C" fn ringbuf_free_ring(rb: *mut ring_buffer, r: *mut ring) {
    static void ringbuf_free_ring(struct ring_buffer *rb, struct ring *r)
    {
    if (r.consumer_pos) {
    munmap(r.consumer_pos, rb.page_size);
    r.consumer_pos = core::ptr::null_mut();
    }
    if (r.producer_pos) {
    munmap(r.producer_pos, rb.page_size + 2 * (r.mask + 1));
    r.producer_pos = core::ptr::null_mut();
    }
    free(r);
    }
// Add extra RINGBUF maps to this ring buffer manager
    int ring_buffer__add(struct ring_buffer *rb, int map_fd,
    ring_buffer_sample_fn sample_cb, void *ctx)
    {
    struct bpf_map_info info;
    let mut len: __u32 = sizeof(info);
    struct epoll_event *e;
    struct ring *r;
    __u64 mmap_sz;
    void *tmp;
    int err;
    memset(&info, 0, sizeof(info));
    err = bpf_map_get_info_by_fd(map_fd, &info, &len);
    if (err) {
    err = -errno;
    pr_warn("ringbuf: failed to get map info for fd=%d: %s\n",
    map_fd, errstr(err));
    return libbpf_err(err);
    }
    if (info.type != BPF_MAP_TYPE_RINGBUF) {
    pr_warn("ringbuf: map fd=%d is not BPF_MAP_TYPE_RINGBUF\n",
    map_fd);
    return libbpf_err(-EINVAL);
    }
    tmp = libbpf_reallocarray(rb.rings, rb.ring_cnt + 1, sizeof(*rb.rings));
    if (!tmp)
    return libbpf_err(-ENOMEM);
    rb.rings = tmp;
    tmp = libbpf_reallocarray(rb.events, rb.ring_cnt + 1, sizeof(*rb.events));
    if (!tmp)
    return libbpf_err(-ENOMEM);
    rb.events = tmp;
    r = calloc(1, sizeof(*r));
    if (!r)
    return libbpf_err(-ENOMEM);
    rb.rings[rb.ring_cnt] = r;
    r.map_fd = map_fd;
    r.sample_cb = sample_cb;
    r.ctx = ctx;
    r.mask = info.max_entries - 1;
// Map writable consumer page
    tmp = mmap(core::ptr::null_mut(), rb.page_size, PROT_READ | PROT_WRITE, MAP_SHARED, map_fd, 0);
    if (tmp == MAP_FAILED) {
    err = -errno;
    pr_warn("ringbuf: failed to mmap consumer page for map fd=%d: %s\n",
    map_fd, errstr(err));
    goto err_out;
    }
    r.consumer_pos = tmp;
// Map read-only producer page and data pages. We map twice as big
// data size to allow simple reading of samples that wrap around the
// end of a ring buffer. See kernel implementation for details.
//
    mmap_sz = rb.page_size + 2 * (__u64)info.max_entries;
    if (mmap_sz != (__u64)(size_t)mmap_sz) {
    err = -E2BIG;
    pr_warn("ringbuf: ring buffer size (%u) is too big\n", info.max_entries);
    goto err_out;
    }
    tmp = mmap(core::ptr::null_mut(), (size_t)mmap_sz, PROT_READ, MAP_SHARED, map_fd, rb.page_size);
    if (tmp == MAP_FAILED) {
    err = -errno;
    pr_warn("ringbuf: failed to mmap data pages for map fd=%d: %s\n",
    map_fd, errstr(err));
    goto err_out;
    }
    r.producer_pos = tmp;
    r.data = tmp + rb.page_size;
    e = &rb.events[rb.ring_cnt];
    memset(e, 0, sizeof(*e));
    e.events = EPOLLIN;
    e.data.fd = rb.ring_cnt;
    if (epoll_ctl(rb.epoll_fd, EPOLL_CTL_ADD, map_fd, e) < 0) {
    err = -errno;
    pr_warn("ringbuf: failed to epoll add map fd=%d: %s\n",
    map_fd, errstr(err));
    goto err_out;
    }
    rb.ring_cnt++;
    return 0;
    err_out:
    ringbuf_free_ring(rb, r);
    return libbpf_err(err);
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__free(rb: *mut ring_buffer) {
    void ring_buffer__free(struct ring_buffer *rb)
    {
    int i;
    if (!rb)
    return;
    for (i = 0; i < rb.ring_cnt; ++i)
    ringbuf_free_ring(rb, rb.rings[i]);
    if (rb.epoll_fd >= 0)
    close(rb.epoll_fd);
    free(rb.events);
    free(rb.rings);
    free(rb);
    }
    struct ring_buffer *
    ring_buffer__new(int map_fd, ring_buffer_sample_fn sample_cb, void *ctx,
    const struct ring_buffer_opts *opts)
    {
    struct ring_buffer *rb;
    int err;
    if (!OPTS_VALID(opts, ring_buffer_opts))
    let mut errno: return = EINVAL, core::ptr::null_mut();
    rb = calloc(1, sizeof(*rb));
    if (!rb)
    let mut errno: return = ENOMEM, core::ptr::null_mut();
    rb.page_size = getpagesize();
    rb.epoll_fd = epoll_create1(EPOLL_CLOEXEC);
    if (rb.epoll_fd < 0) {
    err = -errno;
    pr_warn("ringbuf: failed to create epoll instance: %s\n", errstr(err));
    goto err_out;
    }
    err = ring_buffer__add(rb, map_fd, sample_cb, ctx);
    if (err)
    goto err_out;
    return rb;
    err_out:
    ring_buffer__free(rb);
    let mut errno: return = -err, core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn roundup_len(len: __u32) -> c_int {
    static inline int roundup_len(__u32 len)
    {
// clear out top 2 bits (discard and busy, if set)
    len <<= 2;
    len >>= 2;
// add length prefix
    len += BPF_RINGBUF_HDR_SZ;
// round up to 8 byte alignment
    return (len + 7) / 8 * 8;
    }
#[no_mangle]
unsafe extern "C" fn ringbuf_process_ring(r: *mut ring, n: usize) -> i64 {
    static int64_t ringbuf_process_ring(struct ring *r, size_t n)
    {
    int *len_ptr, len, err;
// 64-bit to avoid overflow in case of extreme application behavior
    let mut cnt: i64 = 0;
    unsigned long cons_pos, prod_pos;
    bool got_new_data;
    void *sample;
    cons_pos = smp_load_acquire(r.consumer_pos);
    do {
    got_new_data = false;
    prod_pos = smp_load_acquire(r.producer_pos);
    while (prod_pos - cons_pos > 0) {
    len_ptr = r.data + (cons_pos & r.mask);
    len = smp_load_acquire(len_ptr);
// sample not committed yet, bail out for now
    if (len & BPF_RINGBUF_BUSY_BIT)
    goto done;
    got_new_data = true;
    cons_pos += roundup_len(len);
    if ((len & BPF_RINGBUF_DISCARD_BIT) == 0) {
    sample = (void *)len_ptr + BPF_RINGBUF_HDR_SZ;
    err = r.sample_cb(r.ctx, sample, len);
    if (err < 0) {
// update consumer pos and bail out
    smp_store_release(r.consumer_pos,
    cons_pos);
    return err;
    }
    cnt++;
    }
    smp_store_release(r.consumer_pos, cons_pos);
    if (cnt >= n)
    goto done;
    }
    } while (got_new_data);
    done:
    return cnt;
    }
// Consume available ring buffer(s) data without event polling, up to n
// records.
//
// Returns number of records consumed across all registered ring buffers (or
// n, whichever is less), or negative number if any of the callbacks return
// error.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__consume_n(rb: *mut ring_buffer, n: usize) -> c_int {
    int ring_buffer__consume_n(struct ring_buffer *rb, size_t n)
    {
    int64_t err, res = 0;
    int i;
    for (i = 0; i < rb.ring_cnt; i++) {
    struct ring *ring = rb.rings[i];
    err = ringbuf_process_ring(ring, n);
    if (err < 0)
    return libbpf_err(err);
    res += err;
    n -= err;
    if (n == 0)
    break;
    }
    return res > INT_MAX ? INT_MAX : res;
    }
// Consume available ring buffer(s) data without event polling.
// Returns number of records consumed across all registered ring buffers (or
// INT_MAX, whichever is less), or negative number if any of the callbacks
// return error.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int {
    int ring_buffer__consume(struct ring_buffer *rb)
    {
    int64_t err, res = 0;
    int i;
    for (i = 0; i < rb.ring_cnt; i++) {
    struct ring *ring = rb.rings[i];
    err = ringbuf_process_ring(ring, INT_MAX);
    if (err < 0)
    return libbpf_err(err);
    res += err;
    if (res > INT_MAX) {
    res = INT_MAX;
    break;
    }
    }
    return res;
    }
// Poll for available data and consume records, if any are available.
// Returns number of records consumed (or INT_MAX, whichever is less), or
// negative number, if any of the registered callbacks returned error.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__poll(rb: *mut ring_buffer, timeout_ms: c_int) -> c_int {
    int ring_buffer__poll(struct ring_buffer *rb, int timeout_ms)
    {
    int i, cnt;
    int64_t err, res = 0;
    cnt = epoll_wait(rb.epoll_fd, rb.events, rb.ring_cnt, timeout_ms);
    if (cnt < 0)
    return libbpf_err(-errno);
    for (i = 0; i < cnt; i++) {
    let mut ring_id: __u32 = rb.events[i].data.fd;
    struct ring *ring = rb.rings[ring_id];
    err = ringbuf_process_ring(ring, INT_MAX);
    if (err < 0)
    return libbpf_err(err);
    res += err;
    }
    if (res > INT_MAX)
    res = INT_MAX;
    return res;
    }
// Get an fd that can be used to sleep until data is available in the ring(s)
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__epoll_fd(rb: *const ring_buffer) -> c_int {
    int ring_buffer__epoll_fd(const struct ring_buffer *rb)
    {
    return rb.epoll_fd;
    }
    struct ring *ring_buffer__ring(struct ring_buffer *rb, unsigned int idx)
    {
    if (idx >= rb.ring_cnt)
    let mut errno: return = ERANGE, core::ptr::null_mut();
    return rb.rings[idx];
    }
#[no_mangle]
pub unsafe extern "C" fn ring__consumer_pos(r: *const ring) -> c_ulong {
    unsigned long ring__consumer_pos(const struct ring *r)
    {
// Synchronizes with smp_store_release() in ringbuf_process_ring().
    return smp_load_acquire(r.consumer_pos);
    }
#[no_mangle]
pub unsafe extern "C" fn ring__producer_pos(r: *const ring) -> c_ulong {
    unsigned long ring__producer_pos(const struct ring *r)
    {
// Synchronizes with smp_store_release() in __bpf_ringbuf_reserve() in
// the kernel.
//
    return smp_load_acquire(r.producer_pos);
    }
#[no_mangle]
pub unsafe extern "C" fn ring__avail_data_size(r: *const ring) -> usize {
    size_t ring__avail_data_size(const struct ring *r)
    {
    unsigned long cons_pos, prod_pos;
    cons_pos = ring__consumer_pos(r);
    prod_pos = ring__producer_pos(r);
    return prod_pos - cons_pos;
    }
#[no_mangle]
pub unsafe extern "C" fn ring__size(r: *const ring) -> usize {
    size_t ring__size(const struct ring *r)
    {
    return r.mask + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn ring__map_fd(r: *const ring) -> c_int {
    int ring__map_fd(const struct ring *r)
    {
    return r.map_fd;
    }
#[no_mangle]
pub unsafe extern "C" fn ring__consume_n(r: *mut ring, n: usize) -> c_int {
    int ring__consume_n(struct ring *r, size_t n)
    {
    int64_t res;
    res = ringbuf_process_ring(r, n);
    if (res < 0)
    return libbpf_err(res);
    return res > INT_MAX ? INT_MAX : res;
    }
#[no_mangle]
pub unsafe extern "C" fn ring__consume(r: *mut ring) -> c_int {
    int ring__consume(struct ring *r)
    {
    return ring__consume_n(r, INT_MAX);
    }
#[no_mangle]
unsafe extern "C" fn user_ringbuf_unmap_ring(rb: *mut user_ring_buffer) {
    static void user_ringbuf_unmap_ring(struct user_ring_buffer *rb)
    {
    if (rb.consumer_pos) {
    munmap(rb.consumer_pos, rb.page_size);
    rb.consumer_pos = core::ptr::null_mut();
    }
    if (rb.producer_pos) {
    munmap(rb.producer_pos, rb.page_size + 2 * (rb.mask + 1));
    rb.producer_pos = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__free(rb: *mut user_ring_buffer) {
    void user_ring_buffer__free(struct user_ring_buffer *rb)
    {
    if (!rb)
    return;
    user_ringbuf_unmap_ring(rb);
    if (rb.epoll_fd >= 0)
    close(rb.epoll_fd);
    free(rb);
    }
#[no_mangle]
unsafe extern "C" fn user_ringbuf_map(rb: *mut user_ring_buffer, map_fd: c_int) -> c_int {
    static int user_ringbuf_map(struct user_ring_buffer *rb, int map_fd)
    {
    struct bpf_map_info info;
    let mut len: __u32 = sizeof(info);
    __u64 mmap_sz;
    void *tmp;
    struct epoll_event *rb_epoll;
    int err;
    memset(&info, 0, sizeof(info));
    err = bpf_map_get_info_by_fd(map_fd, &info, &len);
    if (err) {
    err = -errno;
    pr_warn("user ringbuf: failed to get map info for fd=%d: %s\n",
    map_fd, errstr(err));
    return err;
    }
    if (info.type != BPF_MAP_TYPE_USER_RINGBUF) {
    pr_warn("user ringbuf: map fd=%d is not BPF_MAP_TYPE_USER_RINGBUF\n", map_fd);
    return -EINVAL;
    }
    rb.map_fd = map_fd;
    rb.mask = info.max_entries - 1;
// Map read-only consumer page
    tmp = mmap(core::ptr::null_mut(), rb.page_size, PROT_READ, MAP_SHARED, map_fd, 0);
    if (tmp == MAP_FAILED) {
    err = -errno;
    pr_warn("user ringbuf: failed to mmap consumer page for map fd=%d: %s\n",
    map_fd, errstr(err));
    return err;
    }
    rb.consumer_pos = tmp;
// Map read-write the producer page and data pages. We map the data
// region as twice the total size of the ring buffer to allow the
// simple reading and writing of samples that wrap around the end of
// the buffer.  See the kernel implementation for details.
//
    mmap_sz = rb.page_size + 2 * (__u64)info.max_entries;
    if (mmap_sz != (__u64)(size_t)mmap_sz) {
    pr_warn("user ringbuf: ring buf size (%u) is too big\n", info.max_entries);
    return -E2BIG;
    }
    tmp = mmap(core::ptr::null_mut(), (size_t)mmap_sz, PROT_READ | PROT_WRITE, MAP_SHARED,
    map_fd, rb.page_size);
    if (tmp == MAP_FAILED) {
    err = -errno;
    pr_warn("user ringbuf: failed to mmap data pages for map fd=%d: %s\n",
    map_fd, errstr(err));
    return err;
    }
    rb.producer_pos = tmp;
    rb.data = tmp + rb.page_size;
    rb_epoll = &rb.event;
    rb_epoll.events = EPOLLOUT;
    if (epoll_ctl(rb.epoll_fd, EPOLL_CTL_ADD, map_fd, rb_epoll) < 0) {
    err = -errno;
    pr_warn("user ringbuf: failed to epoll add map fd=%d: %s\n", map_fd, errstr(err));
    return err;
    }
    return 0;
    }
    struct user_ring_buffer *
    user_ring_buffer__new(int map_fd, const struct user_ring_buffer_opts *opts)
    {
    struct user_ring_buffer *rb;
    int err;
    if (!OPTS_VALID(opts, user_ring_buffer_opts))
    let mut errno: return = EINVAL, core::ptr::null_mut();
    rb = calloc(1, sizeof(*rb));
    if (!rb)
    let mut errno: return = ENOMEM, core::ptr::null_mut();
    rb.page_size = getpagesize();
    rb.epoll_fd = epoll_create1(EPOLL_CLOEXEC);
    if (rb.epoll_fd < 0) {
    err = -errno;
    pr_warn("user ringbuf: failed to create epoll instance: %s\n", errstr(err));
    goto err_out;
    }
    err = user_ringbuf_map(rb, map_fd);
    if (err)
    goto err_out;
    return rb;
    err_out:
    user_ring_buffer__free(rb);
    let mut errno: return = -err, core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn user_ringbuf_commit(rb: *mut user_ring_buffer, sample: *mut c_void, discard: bool) {
    static void user_ringbuf_commit(struct user_ring_buffer *rb, void *sample, bool discard)
    {
    __u32 new_len;
    struct ringbuf_hdr *hdr;
    uintptr_t hdr_offset;
    hdr_offset = rb.mask + 1 + (sample - rb.data) - BPF_RINGBUF_HDR_SZ;
    hdr = rb.data + (hdr_offset & rb.mask);
    new_len = hdr.len & ~BPF_RINGBUF_BUSY_BIT;
    if (discard)
    new_len |= BPF_RINGBUF_DISCARD_BIT;
// Synchronizes with smp_load_acquire() in __bpf_user_ringbuf_peek() in
// the kernel.
//
    __atomic_exchange_n(&hdr.len, new_len, __ATOMIC_ACQ_REL);
    }
#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__discard(rb: *mut user_ring_buffer, sample: *mut c_void) {
    void user_ring_buffer__discard(struct user_ring_buffer *rb, void *sample)
    {
    user_ringbuf_commit(rb, sample, true);
    }
#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__submit(rb: *mut user_ring_buffer, sample: *mut c_void) {
    void user_ring_buffer__submit(struct user_ring_buffer *rb, void *sample)
    {
    user_ringbuf_commit(rb, sample, false);
    }
    void *user_ring_buffer__reserve(struct user_ring_buffer *rb, __u32 size)
    {
    __u32 avail_size, total_size, max_size;
// 64-bit to avoid overflow in case of extreme application behavior
    __u64 cons_pos, prod_pos;
    struct ringbuf_hdr *hdr;
// The top two bits are used as special flags
    if (size & (BPF_RINGBUF_BUSY_BIT | BPF_RINGBUF_DISCARD_BIT))
    let mut errno: return = E2BIG, core::ptr::null_mut();
// Synchronizes with smp_store_release() in __bpf_user_ringbuf_peek() in
// the kernel.
//
    cons_pos = smp_load_acquire(rb.consumer_pos);
// Synchronizes with smp_store_release() in user_ringbuf_commit()
    prod_pos = smp_load_acquire(rb.producer_pos);
    max_size = rb.mask + 1;
    avail_size = max_size - (prod_pos - cons_pos);
// Round up total size to a multiple of 8.
    total_size = (size + BPF_RINGBUF_HDR_SZ + 7) / 8 * 8;
    if (total_size > max_size)
    let mut errno: return = E2BIG, core::ptr::null_mut();
    if (avail_size < total_size)
    let mut errno: return = ENOSPC, core::ptr::null_mut();
    hdr = rb.data + (prod_pos & rb.mask);
    hdr.len = size | BPF_RINGBUF_BUSY_BIT;
    hdr.pad = 0;
// Synchronizes with smp_load_acquire() in __bpf_user_ringbuf_peek() in
// the kernel.
//
    smp_store_release(rb.producer_pos, prod_pos + total_size);
    return (void *)rb.data + ((prod_pos + BPF_RINGBUF_HDR_SZ) & rb.mask);
    }
#[no_mangle]
unsafe extern "C" fn ns_elapsed_timespec(start: *const timespec, end: *const timespec) -> __u64 {
    static __u64 ns_elapsed_timespec(const struct timespec *start, const struct timespec *end)
    {
    __u64 start_ns, end_ns, ns_per_s = 1000000000;
    start_ns = (__u64)start.tv_sec * ns_per_s + start.tv_nsec;
    end_ns = (__u64)end.tv_sec * ns_per_s + end.tv_nsec;
    return end_ns - start_ns;
    }
    void *user_ring_buffer__reserve_blocking(struct user_ring_buffer *rb, __u32 size, int timeout_ms)
    {
    void *sample;
    int err, ms_remaining = timeout_ms;
    struct timespec start;
    if (timeout_ms < 0 && timeout_ms != -1)
    let mut errno: return = EINVAL, core::ptr::null_mut();
    if (timeout_ms != -1) {
    err = clock_gettime(CLOCK_MONOTONIC, &start);
    if (err)
    return core::ptr::null_mut();
    }
    do {
    int cnt, ms_elapsed;
    struct timespec curr;
    let mut ns_per_ms: __u64 = 1000000;
    sample = user_ring_buffer__reserve(rb, size);
    if (sample)
    return sample;
#[no_mangle]
pub unsafe extern "C" fn if(ENOSPC: errno !=) -> else {
    else if (errno != ENOSPC)
    return core::ptr::null_mut();
// The kernel guarantees at least one event notification
// delivery whenever at least one sample is drained from the
// ring buffer in an invocation to bpf_ringbuf_drain(). Other
// additional events may be delivered at any time, but only one
// event is guaranteed per bpf_ringbuf_drain() invocation,
// provided that a sample is drained, and the BPF program did
// not pass BPF_RB_NO_WAKEUP to bpf_ringbuf_drain(). If
// BPF_RB_FORCE_WAKEUP is passed to bpf_ringbuf_drain(), a
// wakeup event will be delivered even if no samples are
// drained.
//
    cnt = epoll_wait(rb.epoll_fd, &rb.event, 1, ms_remaining);
    if (cnt < 0)
    return core::ptr::null_mut();
    if (timeout_ms == -1)
    continue;
    err = clock_gettime(CLOCK_MONOTONIC, &curr);
    if (err)
    return core::ptr::null_mut();
    ms_elapsed = ns_elapsed_timespec(&start, &curr) / ns_per_ms;
    ms_remaining = timeout_ms - ms_elapsed;
    } while (ms_remaining > 0);
// Try one more time to reserve a sample after the specified timeout has elapsed.
    return user_ring_buffer__reserve(rb, size);
    }
