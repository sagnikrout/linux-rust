//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ublk/fault_inject.c
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
// Fault injection ublk target. Hack this up however you like for
// testing specific behaviors of ublk_drv. Currently is a null target
// with a configurable delay before completing each I/O. This delay can
// be used to test ublk_drv's handling of I/O outstanding to the ublk
// server when it dies.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fi_opts {
    pub delay_ns: c_longlong,
    pub die_during_fetch: bool,
}

    static int ublk_fault_inject_tgt_init(const struct dev_ctx *ctx,
    struct ublk_dev *dev)
    {
    const struct ublksrv_ctrl_dev_info *info = &dev.dev_info;
    let mut dev_size: c_ulong = 250UL << 30;
    struct fi_opts *opts = core::ptr::null_mut();
    if (ctx.auto_zc_fallback) {
    ublk_err("%s: not support auto_zc_fallback\n", __func__);
    return -EINVAL;
    }
    dev.tgt.dev_size = dev_size;
    dev.tgt.params = (struct ublk_params) {
    .types = UBLK_PARAM_TYPE_BASIC,
    .basic = {
    .logical_bs_shift	= 9,
    .physical_bs_shift	= 12,
    .io_opt_shift		= 12,
    .io_min_shift		= 9,
    .max_sectors		= info.max_io_buf_bytes >> 9,
    .dev_sectors		= dev_size >> 9,
    },
    };
    ublk_set_integrity_params(ctx, &dev.tgt.params);
    opts = calloc(1, sizeof(*opts));
    if (!opts) {
    ublk_err("%s: couldn't allocate memory for opts\n", __func__);
    return -ENOMEM;
    }
    opts.delay_ns = ctx.fault_inject.delay_us * 1000;
    opts.die_during_fetch = ctx.fault_inject.die_during_fetch;
    dev.private_data = opts;
    return 0;
    }
    static void ublk_fault_inject_pre_fetch_io(struct ublk_thread *t,
    struct ublk_queue *q, int tag,
    bool batch)
    {
    struct fi_opts *opts = q.dev.private_data;
    if (!opts.die_during_fetch)
    return;
//
// Each queue fetches its IOs in increasing order of tags, so
// dying just before we're about to fetch tag 1 (regardless of
// what queue we're on) guarantees that we've fetched a nonempty
// proper subset of the tags on that queue.
//
    if (tag == 1) {
//
// Ensure our commands are actually live in the kernel
// before we die.
//
    io_uring_submit(&t.ring);
    raise(SIGKILL);
    }
    }
    static int ublk_fault_inject_queue_io(struct ublk_thread *t,
    struct ublk_queue *q, int tag)
    {
    const struct ublksrv_io_desc *iod = ublk_get_iod(q, tag);
    struct io_uring_sqe *sqe;
    struct fi_opts *opts = q.dev.private_data;
    struct __kernel_timespec ts = {
    .tv_nsec = opts.delay_ns,
    };
    ublk_io_alloc_sqes(t, &sqe, 1);
    io_uring_prep_timeout(sqe, &ts, 1, 0);
    sqe.user_data = build_user_data(tag, ublksrv_get_op(iod), 0, q.q_id, 1);
    ublk_queued_tgt_io(t, q, tag, 1);
    return 0;
    }
    static void ublk_fault_inject_tgt_io_done(struct ublk_thread *t,
    struct ublk_queue *q,
    const struct io_uring_cqe *cqe)
    {
    let mut tag: unsigned = user_data_to_tag(cqe.user_data);
    const struct ublksrv_io_desc *iod = ublk_get_iod(q, tag);
    if (cqe.res != -ETIME)
    ublk_err("%s: unexpected cqe res %d\n", __func__, cqe.res);
    if (ublk_completed_tgt_io(t, q, tag))
    ublk_complete_io(t, q, tag, iod.nr_sectors << 9);
    else
    ublk_err("%s: io not complete after 1 cqe\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn ublk_fault_inject_cmd_line(ctx: *mut dev_ctx, argc: c_int, argv[]: *mut c_char) {
    static void ublk_fault_inject_cmd_line(struct dev_ctx *ctx, int argc, char *argv[])
    {
    static const struct option longopts[] = {
    { "delay_us", 	1,	core::ptr::null_mut(),  0  },
    { "die_during_fetch", 1, core::ptr::null_mut(), 0  },
    { 0, 0, 0, 0 }
    };
    int option_idx, opt;
    ctx.fault_inject.delay_us = 0;
    ctx.fault_inject.die_during_fetch = false;
    while ((opt = getopt_long(argc, argv, "",
    longopts, &option_idx)) != -1) {
    switch (opt) {
    case 0:
    if (!strcmp(longopts[option_idx].name, "delay_us"))
    ctx.fault_inject.delay_us = strtoll(optarg, core::ptr::null_mut(), 10);
    if (!strcmp(longopts[option_idx].name, "die_during_fetch"))
    ctx.fault_inject.die_during_fetch = strtoll(optarg, core::ptr::null_mut(), 10);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ublk_fault_inject_usage(ops: *const ublk_tgt_ops) {
    static void ublk_fault_inject_usage(const struct ublk_tgt_ops *ops)
    {
    printf("\tfault_inject: [--delay_us us (default 0)] [--die_during_fetch 1]\n");
    }
    const struct ublk_tgt_ops fault_inject_tgt_ops = {
    .name = "fault_inject",
    .init_tgt = ublk_fault_inject_tgt_init,
    .pre_fetch_io = ublk_fault_inject_pre_fetch_io,
    .queue_io = ublk_fault_inject_queue_io,
    .tgt_io_done = ublk_fault_inject_tgt_io_done,
    .parse_cmd_line = ublk_fault_inject_cmd_line,
    .usage = ublk_fault_inject_usage,
    };
