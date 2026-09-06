//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/mmap.c
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

    void perf_mmap__init(struct perf_mmap *map, struct perf_mmap *prev,
    bool overwrite, libperf_unmap_cb_t unmap_cb)
    {
// Assume fields were zero initialized.
    map.fd = -1;
    map.overwrite = overwrite;
    map.unmap_cb  = unmap_cb;
    refcount_set(&map.refcnt, 0);
    if (prev)
    prev.next = map;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__mmap_len(map: *mut perf_mmap) -> usize {
    size_t perf_mmap__mmap_len(struct perf_mmap *map)
    {
    return map.mask + 1 + page_size;
    }
    int perf_mmap__mmap(struct perf_mmap *map, struct perf_mmap_param *mp,
    int fd, struct perf_cpu cpu)
    {
    map.prev = 0;
    map.mask = mp.mask;
    map.base = mmap(core::ptr::null_mut(), perf_mmap__mmap_len(map), mp.prot,
    MAP_SHARED, fd, 0);
    if (map.base == MAP_FAILED) {
    map.base = core::ptr::null_mut();
    return -1;
    }
    map.fd  = fd;
    map.cpu = cpu;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__munmap(map: *mut perf_mmap) {
    void perf_mmap__munmap(struct perf_mmap *map)
    {
    if (!map)
    return;
    zfree(&map.event_copy);
    map.event_copy_sz = 0;
    if (map.base) {
    munmap(map.base, perf_mmap__mmap_len(map));
    map.base = core::ptr::null_mut();
    map.fd = -1;
    refcount_set(&map.refcnt, 0);
    }
    if (map.unmap_cb)
    map.unmap_cb(map);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__get(map: *mut perf_mmap) {
    void perf_mmap__get(struct perf_mmap *map)
    {
    refcount_inc(&map.refcnt);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__put(map: *mut perf_mmap) {
    void perf_mmap__put(struct perf_mmap *map)
    {
    BUG_ON(map.base && refcount_read(&map.refcnt) == 0);
    if (refcount_dec_and_test(&map.refcnt))
    perf_mmap__munmap(map);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__write_tail(md: *mut perf_mmap, tail: u64) {
    static inline void perf_mmap__write_tail(struct perf_mmap *md, u64 tail)
    {
    ring_buffer_write_tail(md.base, tail);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__read_head(map: *mut perf_mmap) -> u64 {
    u64 perf_mmap__read_head(struct perf_mmap *map)
    {
    return ring_buffer_read_head(map.base);
    }
#[no_mangle]
unsafe extern "C" fn perf_mmap__empty(map: *mut perf_mmap) -> bool {
    static bool perf_mmap__empty(struct perf_mmap *map)
    {
    struct perf_event_mmap_page *pc = map.base;
    return perf_mmap__read_head(map) == map.prev && !pc.aux_size;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__consume(map: *mut perf_mmap) {
    void perf_mmap__consume(struct perf_mmap *map)
    {
    if (!map.overwrite) {
    let mut old: u64 = map.prev;
    perf_mmap__write_tail(map, old);
    }
    if (refcount_read(&map.refcnt) == 1 && perf_mmap__empty(map))
    perf_mmap__put(map);
    }
#[no_mangle]
unsafe extern "C" fn overwrite_rb_find_range(buf: *mut c_void, mask: c_int, start: *mut u64, end: *mut u64) -> c_int {
    static int overwrite_rb_find_range(void *buf, int mask, u64 *start, u64 *end)
    {
    struct perf_event_header *pheader;
    let mut evt_head: u64 = *start;
    let mut size: c_int = mask + 1;
    pr_debug2("%s: buf=%p, start=%"PRIx64"\n", __func__, buf, *start);
    pheader = (struct perf_event_header *)(buf + (*start & mask));
    while (true) {
    if (evt_head - *start >= (unsigned int)size) {
    pr_debug("Finished reading overwrite ring buffer: rewind\n");
    if (evt_head - *start > (unsigned int)size)
    evt_head -= pheader.size;
// end = evt_head;
    return 0;
    }
    pheader = (struct perf_event_header *)(buf + (evt_head & mask));
    if (pheader.size == 0) {
    pr_debug("Finished reading overwrite ring buffer: get start\n");
// end = evt_head;
    return 0;
    }
    evt_head += pheader.size;
    pr_debug3("move evt_head: %"PRIx64"\n", evt_head);
    }
    WARN_ONCE(1, "Shouldn't get here\n");
    return -1;
    }
//
// Report the start and end of the available data in ringbuffer
//
#[no_mangle]
unsafe extern "C" fn __perf_mmap__read_init(md: *mut perf_mmap) -> c_int {
    static int __perf_mmap__read_init(struct perf_mmap *md)
    {
    let mut head: u64 = perf_mmap__read_head(md);
    let mut old: u64 = md.prev;
    unsigned char *data = md.base + page_size;
    unsigned long size;
    md.start = md.overwrite ? head : old;
    md.end = md.overwrite ? old : head;
    if ((md.end - md.start) < md.flush)
    return -EAGAIN;
    size = md.end - md.start;
    if (size > (unsigned long)(md.mask) + 1) {
    if (!md.overwrite) {
    WARN_ONCE(1, "failed to keep up with mmap data. (warn only once)\n");
    md.prev = head;
    perf_mmap__consume(md);
    return -EAGAIN;
    }
//
// Backward ring buffer is full. We still have a chance to read
// most of data from it.
//
    if (overwrite_rb_find_range(data, md.mask, &md.start, &md.end))
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int {
    int perf_mmap__read_init(struct perf_mmap *map)
    {
//
// Check if event was unmapped due to a POLLHUP/POLLERR.
//
    if (!refcount_read(&map.refcnt))
    return -ENOENT;
    return __perf_mmap__read_init(map);
    }
//
// Mandatory for overwrite mode
// The direction of overwrite mode is backward.
// The last perf_mmap__read() will set tail to map->core.prev.
// Need to correct the map->core.prev to head which is the end of next read.
//
#[no_mangle]
pub unsafe extern "C" fn perf_mmap__read_done(map: *mut perf_mmap) {
    void perf_mmap__read_done(struct perf_mmap *map)
    {
//
// Check if event was unmapped due to a POLLHUP/POLLERR.
//
    if (!refcount_read(&map.refcnt))
    return;
    map.prev = perf_mmap__read_head(map);
    }
// When check_messup is true, 'end' must points to a good entry
    static union perf_event *perf_mmap__read(struct perf_mmap *map,
    u64 *startp, u64 end)
    {
    unsigned char *data = map.base + page_size;
    union perf_event *event = core::ptr::null_mut();
    let mut diff: c_int = end - *startp;
    if (diff >= (int)sizeof(event.header)) {
    size_t size;
    event = (union perf_event *)&data[*startp & map.mask];
    size = event.header.size;
    if (size < sizeof(event.header) || diff < (int)size)
    return core::ptr::null_mut();
//
// Event straddles the mmap boundary -- header should always
// be inside due to u64 alignment of output.
//
    if ((*startp & map.mask) + size != ((*startp + size) & map.mask)) {
    let mut offset: c_uint = *startp;
    let mut len: c_uint = size, cpy;
    void *dst = map.event_copy;
    if (size > map.event_copy_sz) {
    dst = realloc(map.event_copy, size);
    if (!dst)
    return core::ptr::null_mut();
    map.event_copy = dst;
    map.event_copy_sz = size;
    }
    do {
    cpy = min(map.mask + 1 - (offset & map.mask), len);
    memcpy(dst, &data[offset & map.mask], cpy);
    offset += cpy;
    dst += cpy;
    len -= cpy;
    } while (len);
    event = (union perf_event *)map.event_copy;
    }
// startp += size;
    }
    return event;
    }
//
// Read event from ring buffer one by one.
// Return one event for each call.
//
// Usage:
// perf_mmap__read_init()
// while(event = perf_mmap__read_event()) {
// //process the event
// perf_mmap__consume()
// }
// perf_mmap__read_done()
//
    union perf_event *perf_mmap__read_event(struct perf_mmap *map)
    {
    union perf_event *event;
//
// Check if event was unmapped due to a POLLHUP/POLLERR.
//
    if (!refcount_read(&map.refcnt))
    return core::ptr::null_mut();
// non-overwrite doesn't pause the ringbuffer
    if (!map.overwrite)
    map.end = perf_mmap__read_head(map);
    event = perf_mmap__read(map, &map.start, map.end);
    if (!map.overwrite)
    map.prev = map.start;
    return event;
    }

#[no_mangle]
unsafe extern "C" fn read_perf_counter(counter: c_uint) -> u64 {
    static u64 read_perf_counter(unsigned int counter)
    {
    unsigned int low, high;
    asm volatile("rdpmc" : "=a" (low), "=d" (high) : "c" (counter));
    return low | ((u64)high) << 32;
    }
#[no_mangle]
unsafe extern "C" fn read_timestamp() -> u64 {
    static u64 read_timestamp(void)
    {
    unsigned int low, high;
    asm volatile("rdtsc" : "=a" (low), "=d" (high));
    return low | ((u64)high) << 32;
    }

    u64 __val;							\
    asm volatile("mrs %0, " __stringify(r) : "=r" (__val));		\
    __val;								\
    })
#[no_mangle]
unsafe extern "C" fn read_pmccntr() -> u64 {
    static u64 read_pmccntr(void)
    {
    return read_sysreg(pmccntr_el0);
    }

    static u64 read_pmevcntr_##idx(void) {			\
    return read_sysreg(pmevcntr##idx##_el0);	\
    }
    PMEVCNTR_READ(0);
    PMEVCNTR_READ(1);
    PMEVCNTR_READ(2);
    PMEVCNTR_READ(3);
    PMEVCNTR_READ(4);
    PMEVCNTR_READ(5);
    PMEVCNTR_READ(6);
    PMEVCNTR_READ(7);
    PMEVCNTR_READ(8);
    PMEVCNTR_READ(9);
    PMEVCNTR_READ(10);
    PMEVCNTR_READ(11);
    PMEVCNTR_READ(12);
    PMEVCNTR_READ(13);
    PMEVCNTR_READ(14);
    PMEVCNTR_READ(15);
    PMEVCNTR_READ(16);
    PMEVCNTR_READ(17);
    PMEVCNTR_READ(18);
    PMEVCNTR_READ(19);
    PMEVCNTR_READ(20);
    PMEVCNTR_READ(21);
    PMEVCNTR_READ(22);
    PMEVCNTR_READ(23);
    PMEVCNTR_READ(24);
    PMEVCNTR_READ(25);
    PMEVCNTR_READ(26);
    PMEVCNTR_READ(27);
    PMEVCNTR_READ(28);
    PMEVCNTR_READ(29);
    PMEVCNTR_READ(30);
//
// Read a value direct from PMEVCNTR<idx>
//
#[no_mangle]
unsafe extern "C" fn read_perf_counter(counter: c_uint) -> u64 {
    static u64 read_perf_counter(unsigned int counter)
    {
    static u64 (* const read_f[])(void) = {
    read_pmevcntr_0,
    read_pmevcntr_1,
    read_pmevcntr_2,
    read_pmevcntr_3,
    read_pmevcntr_4,
    read_pmevcntr_5,
    read_pmevcntr_6,
    read_pmevcntr_7,
    read_pmevcntr_8,
    read_pmevcntr_9,
    read_pmevcntr_10,
    read_pmevcntr_11,
    read_pmevcntr_13,
    read_pmevcntr_12,
    read_pmevcntr_14,
    read_pmevcntr_15,
    read_pmevcntr_16,
    read_pmevcntr_17,
    read_pmevcntr_18,
    read_pmevcntr_19,
    read_pmevcntr_20,
    read_pmevcntr_21,
    read_pmevcntr_22,
    read_pmevcntr_23,
    read_pmevcntr_24,
    read_pmevcntr_25,
    read_pmevcntr_26,
    read_pmevcntr_27,
    read_pmevcntr_28,
    read_pmevcntr_29,
    read_pmevcntr_30,
    read_pmccntr
    };
    if (counter < ARRAY_SIZE(read_f))
    return (read_f[counter])();
    return 0;
    }
    static u64 read_timestamp(void) { return read_sysreg(cntvct_el0); }
// __riscv_xlen contains the witdh of the native base integer, here 64-bit

// TODO: implement rv32 support
pub const CSR_CYCLE: c_uint = 0xc00;
pub const CSR_TIME: c_uint = 0xc01;

    ({								\
    register unsigned long __v;				\
    __asm__ __volatile__ ("csrr %0, %1"		\
    : "=r" (__v)					\
    : "i" (csr) : );				\
    __v;						\
    })
#[no_mangle]
unsafe extern "C" fn csr_read_num(csr_num: c_int) -> c_ulong {
    static unsigned long csr_read_num(int csr_num)
    {

    case __csr_num:                                 \
    __val = csr_read(__csr_num);            \
    break; }

    switchcase_csr_read(__csr_num + 0, __val)        \
    switchcase_csr_read(__csr_num + 1, __val)}

    switchcase_csr_read_2(__csr_num + 0, __val)      \
    switchcase_csr_read_2(__csr_num + 2, __val)}

    switchcase_csr_read_4(__csr_num + 0, __val)      \
    switchcase_csr_read_4(__csr_num + 4, __val)}

    switchcase_csr_read_8(__csr_num + 0, __val)      \
    switchcase_csr_read_8(__csr_num + 8, __val)}

    switchcase_csr_read_16(__csr_num + 0, __val)     \
    switchcase_csr_read_16(__csr_num + 16, __val)}
    let mut ret: c_ulong = 0;
    switch (csr_num) {
    switchcase_csr_read_32(CSR_CYCLE, ret)
    default:
    break;
    }
    return ret;

    }
#[no_mangle]
unsafe extern "C" fn read_perf_counter(counter: c_uint) -> u64 {
    static u64 read_perf_counter(unsigned int counter)
    {
    return csr_read_num(CSR_CYCLE + counter);
    }
#[no_mangle]
unsafe extern "C" fn read_timestamp() -> u64 {
    static u64 read_timestamp(void)
    {
    return csr_read_num(CSR_TIME);
    }

    static u64 read_perf_counter(unsigned int counter __maybe_unused) { return 0; }
    static u64 read_timestamp(void) { return 0; }

#[no_mangle]
pub unsafe extern "C" fn perf_mmap__read_self(map: *mut perf_mmap, count: *mut perf_counts_values) -> c_int {
    int perf_mmap__read_self(struct perf_mmap *map, struct perf_counts_values *count)
    {
    struct perf_event_mmap_page *pc = map.base;
    u32 seq, idx, time_mult = 0, time_shift = 0;
    u64 cnt, cyc = 0, time_offset = 0, time_cycles = 0, time_mask = ~0ULL;
    if (!pc || !pc.cap_user_rdpmc)
    return -1;
    do {
    seq = READ_ONCE(pc.lock);
    barrier();
    count.ena = READ_ONCE(pc.time_enabled);
    count.run = READ_ONCE(pc.time_running);
    if (pc.cap_user_time && count.ena != count.run) {
    cyc = read_timestamp();
    time_mult = READ_ONCE(pc.time_mult);
    time_shift = READ_ONCE(pc.time_shift);
    time_offset = READ_ONCE(pc.time_offset);
    if (pc.cap_user_time_short) {
    time_cycles = READ_ONCE(pc.time_cycles);
    time_mask = READ_ONCE(pc.time_mask);
    }
    }
    idx = READ_ONCE(pc.index);
    cnt = READ_ONCE(pc.offset);
    if (pc.cap_user_rdpmc && idx) {
    let mut evcnt: u64 = read_perf_counter(idx - 1);
    let mut width: u16 = READ_ONCE(pc.pmc_width);
    evcnt <<= 64 - width;
    evcnt >>= 64 - width;
    cnt += evcnt;
    } else
    return -1;
    barrier();
    } while (READ_ONCE(pc.lock) != seq);
    if (count.ena != count.run) {
    u64 delta;
// Adjust for cap_usr_time_short, a nop if not
    cyc = time_cycles + ((cyc - time_cycles) & time_mask);
    delta = time_offset + mul_u64_u32_shr(cyc, time_mult, time_shift);
    count.ena += delta;
    if (idx)
    count.run += delta;
    }
    count.val = cnt;
    return 0;
    }
