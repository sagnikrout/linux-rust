//! Automatically rewritten from C to Rust
//! Source: block/blk-timeout.c
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
// Functions related to generic timeout handling of requests.
//

    static DECLARE_FAULT_ATTR(fail_io_timeout);
#[no_mangle]
unsafe extern "C" fn setup_fail_io_timeout(str: *mut c_char) -> int __init {
    static int __init setup_fail_io_timeout(char *str)
    {
    return setup_fault_attr(&fail_io_timeout, str);
    }
    __setup("fail_io_timeout=", setup_fail_io_timeout);
#[no_mangle]
pub unsafe extern "C" fn __blk_should_fake_timeout(q: *mut request_queue) -> bool {
    bool __blk_should_fake_timeout(struct request_queue *q)
    {
    return should_fail(&fail_io_timeout, 1);
    }
    EXPORT_SYMBOL_GPL(__blk_should_fake_timeout);
#[no_mangle]
unsafe extern "C" fn fail_io_timeout_debugfs() -> int __init {
    static int __init fail_io_timeout_debugfs(void)
    {
    struct dentry *dir = fault_create_debugfs_attr("fail_io_timeout",
    core::ptr::null_mut(), &fail_io_timeout);
    return PTR_ERR_OR_ZERO(dir);
    }
    late_initcall(fail_io_timeout_debugfs);
    ssize_t part_timeout_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct gendisk *disk = dev_to_disk(dev);
    let mut set: c_int = test_bit(QUEUE_FLAG_FAIL_IO, &disk.queue.queue_flags);
    return sprintf(buf, "%d\n", set != 0);
    }
    ssize_t part_timeout_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct gendisk *disk = dev_to_disk(dev);
    int val;
    if (count) {
    struct request_queue *q = disk.queue;
    char *p = (char *) buf;
    val = simple_strtoul(p, &p, 10);
    if (val)
    blk_queue_flag_set(QUEUE_FLAG_FAIL_IO, q);
    else
    blk_queue_flag_clear(QUEUE_FLAG_FAIL_IO, q);
    }
    return count;
    }

//
// blk_abort_request - Request recovery for the specified command
// @req:	pointer to the request of interest
//
// This function requests that the block layer start recovery for the
// request by deleting the timer and calling the q's timeout function.
// LLDDs who implement their own error recovery MAY ignore the timeout
// event if they generated blk_abort_request.
//
#[no_mangle]
pub unsafe extern "C" fn blk_abort_request(req: *mut request) {
    void blk_abort_request(struct request *req)
    {
//
// All we need to ensure is that timeout scan takes place
// immediately and that scan sees the new timeout value.
// No need for fancy synchronizations.
//
    WRITE_ONCE(req.deadline, jiffies);
    kblockd_schedule_work(&req.q.timeout_work);
    }
    EXPORT_SYMBOL_GPL(blk_abort_request);
    static unsigned long blk_timeout_mask __read_mostly;
#[no_mangle]
unsafe extern "C" fn blk_timeout_init() -> int __init {
    static int __init blk_timeout_init(void)
    {
    blk_timeout_mask = roundup_pow_of_two(HZ) - 1;
    return 0;
    }
    late_initcall(blk_timeout_init);
//
// Just a rough estimate, we don't care about specific values for timeouts.
//
#[no_mangle]
pub unsafe extern "C" fn blk_round_jiffies(j: c_ulong) -> c_ulong {
    static inline unsigned long blk_round_jiffies(unsigned long j)
    {
    return (j + blk_timeout_mask) + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_rq_timeout(timeout: c_ulong) -> c_ulong {
    unsigned long blk_rq_timeout(unsigned long timeout)
    {
    unsigned long maxt;
    maxt = blk_round_jiffies(jiffies + BLK_MAX_TIMEOUT);
    if (time_after(timeout, maxt))
    timeout = maxt;
    return timeout;
    }
//
// blk_add_timer - Start timeout timer for a single request
// @req:	request that is about to start running.
//
// Notes:
// Each request has its own timer, and as it is added to the queue, we
// set up the timer. When the request completes, we cancel the timer.
//
#[no_mangle]
pub unsafe extern "C" fn blk_add_timer(req: *mut request) {
    void blk_add_timer(struct request *req)
    {
    struct request_queue *q = req.q;
    unsigned long expiry;
//
// Some LLDs, like scsi, peek at the timeout to prevent a
// command from being retried forever.
//
    if (!req.timeout)
    req.timeout = q.rq_timeout;
    req.rq_flags &= ~RQF_TIMED_OUT;
    expiry = jiffies + req.timeout;
    WRITE_ONCE(req.deadline, expiry);
//
// If the timer isn't already pending or this timeout is earlier
// than an existing one, modify the timer. Round up to next nearest
// second.
//
    expiry = blk_rq_timeout(blk_round_jiffies(expiry));
    if (!timer_pending(&q.timeout) ||
    time_before(expiry, q.timeout.expires)) {
    let mut diff: c_ulong = q.timeout.expires - expiry;
//
// Due to added timer slack to group timers, the timer
// will often be a little in front of what we asked for.
// So apply some tolerance here too, otherwise we keep
// modifying the timer because expires for value X
// will be X + something.
//
    if (!timer_pending(&q.timeout) || (diff >= HZ / 2))
    mod_timer(&q.timeout, expiry);
    }
    }
