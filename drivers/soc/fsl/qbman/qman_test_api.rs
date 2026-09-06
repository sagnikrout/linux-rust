//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qbman/qman_test_api.c
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


// Copyright 2008 - 2016 Freescale Semiconductor, Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

pub const CGR_ID: c_int = 27;
pub const POOL_ID: c_int = 2;

pub const NUM_ENQUEUES: c_int = 10;
pub const NUM_PARTIAL: c_int = 4;

    QM_SDQCR_TYPE_PRIO_QOS | \
    QM_SDQCR_TOKEN_SET(0x98) | \
    QM_SDQCR_CHANNELS_DEDICATED | \
    QM_SDQCR_CHANNELS_POOL(POOL_ID))

    static enum qman_cb_dqrr_result cb_dqrr(struct qman_portal *,
    struct qman_fq *,
    const struct qm_dqrr_entry *,
    bool sched_napi);
    static void cb_ern(struct qman_portal *, struct qman_fq *,
    const union qm_mr_entry *);
    static void cb_fqs(struct qman_portal *, struct qman_fq *,
    const union qm_mr_entry *);
    static struct qm_fd fd, fd_dq;
    static struct qman_fq fq_base = {
    .cb.dqrr = cb_dqrr,
    .cb.ern = cb_ern,
    .cb.fqs = cb_fqs
    };
    static DECLARE_WAIT_QUEUE_HEAD(waitqueue);
    static int retire_complete, sdqcr_complete;
// Helpers for initialising and "incrementing" a frame descriptor
#[no_mangle]
unsafe extern "C" fn fd_init(fd: *mut qm_fd) {
    static void fd_init(struct qm_fd *fd)
    {
    qm_fd_addr_set64(fd, 0xabdeadbeefLLU);
    qm_fd_set_contig_big(fd, 0x0000ffff);
    fd.cmd = cpu_to_be32(0xfeedf00d);
    }
#[no_mangle]
unsafe extern "C" fn fd_inc(fd: *mut qm_fd) {
    static void fd_inc(struct qm_fd *fd)
    {
    let mut t: u64 = qm_fd_addr_get64(fd);
    let mut z: c_int = t >> 40;
    unsigned int len, off;
    enum qm_fd_format fmt;
    t <<= 1;
    if (z)
    t |= 1;
    qm_fd_addr_set64(fd, t);
    fmt = qm_fd_get_format(fd);
    off = qm_fd_get_offset(fd);
    len = qm_fd_get_length(fd);
    len--;
    qm_fd_set_param(fd, fmt, off, len);
    be32_add_cpu(&fd.cmd, 1);
    }
// The only part of the 'fd' we can't memcmp() is the ppid
#[no_mangle]
unsafe extern "C" fn fd_neq(a: *const qm_fd, b: *const qm_fd) -> bool {
    static bool fd_neq(const struct qm_fd *a, const struct qm_fd *b)
    {
    let mut neq: bool = qm_fd_addr_get64(a) != qm_fd_addr_get64(b);
    neq |= qm_fd_get_format(a) != qm_fd_get_format(b);
    neq |= a.cfg != b.cfg;
    neq |= a.cmd != b.cmd;
    return neq;
    }
// test
#[no_mangle]
unsafe extern "C" fn do_enqueues(fq: *mut qman_fq) -> c_int {
    static int do_enqueues(struct qman_fq *fq)
    {
    unsigned int loop;
    let mut err: c_int = 0;
    for (loop = 0; loop < NUM_ENQUEUES; loop++) {
    if (qman_enqueue(fq, &fd)) {
    pr_crit("qman_enqueue() failed\n");
    err = -EIO;
    }
    fd_inc(&fd);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_test_api() -> c_int {
    int qman_test_api(void)
    {
    unsigned int flags, frmcnt;
    int err;
    struct qman_fq *fq = &fq_base;
    pr_info("%s(): Starting\n", __func__);
    fd_init(&fd);
    fd_init(&fd_dq);
// Initialise (parked) FQ
    err = qman_create_fq(0, FQ_FLAGS, fq);
    if (err) {
    pr_crit("qman_create_fq() failed\n");
    goto failed;
    }
    err = qman_init_fq(fq, QMAN_INITFQ_FLAG_LOCAL, core::ptr::null_mut());
    if (err) {
    pr_crit("qman_init_fq() failed\n");
    goto failed;
    }
// Do enqueues + VDQCR, twice. (Parked FQ)
    err = do_enqueues(fq);
    if (err)
    goto failed;
    pr_info("VDQCR (till-empty);\n");
    frmcnt = QM_VDQCR_NUMFRAMES_TILLEMPTY;
    err = qman_volatile_dequeue(fq, VDQCR_FLAGS, frmcnt);
    if (err) {
    pr_crit("qman_volatile_dequeue() failed\n");
    goto failed;
    }
    err = do_enqueues(fq);
    if (err)
    goto failed;
    pr_info("VDQCR (%d of %d);\n", NUM_PARTIAL, NUM_ENQUEUES);
    frmcnt = QM_VDQCR_NUMFRAMES_SET(NUM_PARTIAL);
    err = qman_volatile_dequeue(fq, VDQCR_FLAGS, frmcnt);
    if (err) {
    pr_crit("qman_volatile_dequeue() failed\n");
    goto failed;
    }
    pr_info("VDQCR (%d of %d);\n", NUM_ENQUEUES - NUM_PARTIAL,
    NUM_ENQUEUES);
    frmcnt = QM_VDQCR_NUMFRAMES_SET(NUM_ENQUEUES - NUM_PARTIAL);
    err = qman_volatile_dequeue(fq, VDQCR_FLAGS, frmcnt);
    if (err) {
    pr_err("qman_volatile_dequeue() failed\n");
    goto failed;
    }
    err = do_enqueues(fq);
    if (err)
    goto failed;
    pr_info("scheduled dequeue (till-empty)\n");
    err = qman_schedule_fq(fq);
    if (err) {
    pr_crit("qman_schedule_fq() failed\n");
    goto failed;
    }
    wait_event(waitqueue, sdqcr_complete);
// Retire and OOS the FQ
    err = qman_retire_fq(fq, &flags);
    if (err < 0) {
    pr_crit("qman_retire_fq() failed\n");
    goto failed;
    }
    wait_event(waitqueue, retire_complete);
    if (flags & QMAN_FQ_STATE_BLOCKOOS) {
    err = -EIO;
    pr_crit("leaking frames\n");
    goto failed;
    }
    err = qman_oos_fq(fq);
    if (err) {
    pr_crit("qman_oos_fq() failed\n");
    goto failed;
    }
    qman_destroy_fq(fq);
    pr_info("%s(): Finished\n", __func__);
    return 0;
    failed:
    WARN_ON(1);
    return err;
    }
    static enum qman_cb_dqrr_result cb_dqrr(struct qman_portal *p,
    struct qman_fq *fq,
    const struct qm_dqrr_entry *dq,
    bool sched_napi)
    {
    if (WARN_ON(fd_neq(&fd_dq, &dq.fd))) {
    pr_err("BADNESS: dequeued frame doesn't match;\n");
    return qman_cb_dqrr_consume;
    }
    fd_inc(&fd_dq);
    if (!(dq.stat & QM_DQRR_STAT_UNSCHEDULED) && !fd_neq(&fd_dq, &fd)) {
    sdqcr_complete = 1;
    wake_up(&waitqueue);
    }
    return qman_cb_dqrr_consume;
    }
    static void cb_ern(struct qman_portal *p, struct qman_fq *fq,
    const union qm_mr_entry *msg)
    {
    pr_crit("cb_ern() unimplemented");
    WARN_ON(1);
    }
    static void cb_fqs(struct qman_portal *p, struct qman_fq *fq,
    const union qm_mr_entry *msg)
    {
    let mut verb: u8 = (msg.verb & QM_MR_VERB_TYPE_MASK);
    if ((verb != QM_MR_VERB_FQRN) && (verb != QM_MR_VERB_FQRNI)) {
    pr_crit("unexpected FQS message");
    WARN_ON(1);
    return;
    }
    pr_info("Retirement message received\n");
    retire_complete = 1;
    wake_up(&waitqueue);
    }
