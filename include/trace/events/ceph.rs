//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/ceph.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Ceph filesystem support module tracepoints
//
// Copyright (C) 2025 IONOS SE. All Rights Reserved.
// Written by Max Kellermann (max.kellermann@ionos.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_mdsc_suspend_reason {

//
// Export enum symbols via userspace.
//

    ceph_mdsc_suspend_reasons;

//
// Now redefine the EM() and E_() macros to map the enums to the strings that
// will be printed in the output.
//

    TRACE_EVENT(ceph_mdsc_submit_request,
    TP_PROTO(struct ceph_mds_client *mdsc,
    struct ceph_mds_request *req),

    TP_ARGS(mdsc, req),

    TP_STRUCT__entry(
    __field(u64,	tid)
    __field(int,	op)
    __field(u64,	ino)
    __field(u64,	snap)
    ),

    TP_fast_assign(
    struct inode *inode;

    __entry->tid = req->r_tid;
    __entry->op = req->r_op;

    inode = req->r_inode;
    if (inode == NULL && req->r_dentry)
    inode = d_inode(req->r_dentry);

    if (inode) {
    __entry->ino = ceph_ino(inode);
    __entry->snap = ceph_snap(inode);
    } else {
    __entry->ino = __entry->snap = 0;
    }
    ),

    TP_printk("R=%llu op=%s ino=%llx,%llx",
    __entry->tid,
    ceph_mds_op_name(__entry->op),
    __entry->ino, __entry->snap)
    );

    TRACE_EVENT(ceph_mdsc_suspend_request,
    TP_PROTO(struct ceph_mds_client *mdsc,
    struct ceph_mds_session *session,
    struct ceph_mds_request *req,
    enum ceph_mdsc_suspend_reason reason),

    TP_ARGS(mdsc, session, req, reason),

    TP_STRUCT__entry(
    __field(u64,				tid)
    __field(int,				op)
    __field(int,				mds)
    __field(enum ceph_mdsc_suspend_reason,	reason)
    ),

    TP_fast_assign(
    __entry->tid = req->r_tid;
    __entry->op = req->r_op;
    __entry->mds = session ? session->s_mds : -1;
    __entry->reason = reason;
    ),

    TP_printk("R=%llu op=%s reason=%s",
    __entry->tid,
    ceph_mds_op_name(__entry->op),
    __print_symbolic(__entry->reason, ceph_mdsc_suspend_reasons))
    );

    TRACE_EVENT(ceph_mdsc_resume_request,
    TP_PROTO(struct ceph_mds_client *mdsc,
    struct ceph_mds_request *req),

    TP_ARGS(mdsc, req),

    TP_STRUCT__entry(
    __field(u64,				tid)
    __field(int,				op)
    ),

    TP_fast_assign(
    __entry->tid = req->r_tid;
    __entry->op = req->r_op;
    ),

    TP_printk("R=%llu op=%s",
    __entry->tid,
    ceph_mds_op_name(__entry->op))
    );

    TRACE_EVENT(ceph_mdsc_send_request,
    TP_PROTO(struct ceph_mds_session *session,
    struct ceph_mds_request *req),

    TP_ARGS(session, req),

    TP_STRUCT__entry(
    __field(u64,		tid)
    __field(int,		op)
    __field(int,		mds)
    ),

    TP_fast_assign(
    __entry->tid = req->r_tid;
    __entry->op = req->r_op;
    __entry->mds = session->s_mds;
    ),

    TP_printk("R=%llu op=%s mds=%d",
    __entry->tid,
    ceph_mds_op_name(__entry->op),
    __entry->mds)
    );

    TRACE_EVENT(ceph_mdsc_complete_request,
    TP_PROTO(struct ceph_mds_client *mdsc,
    struct ceph_mds_request *req),

    TP_ARGS(mdsc, req),

    TP_STRUCT__entry(
    __field(u64,			tid)
    __field(int,			op)
    __field(int,			err)
    __field(unsigned long,		latency_ns)
    ),

    TP_fast_assign(
    __entry->tid = req->r_tid;
    __entry->op = req->r_op;
    __entry->err = req->r_err;
    __entry->latency_ns = req->r_end_latency - req->r_start_latency;
    ),

    TP_printk("R=%llu op=%s err=%d latency_ns=%lu",
    __entry->tid,
    ceph_mds_op_name(__entry->op),
    __entry->err,
    __entry->latency_ns)
    );

    TRACE_EVENT(ceph_handle_caps,
    TP_PROTO(struct ceph_mds_client *mdsc,
    struct ceph_mds_session *session,
    int op,
    const struct ceph_vino *vino,
    struct ceph_inode_info *inode,
    u32 seq, u32 mseq, u32 issue_seq),

    TP_ARGS(mdsc, session, op, vino, inode, seq, mseq, issue_seq),

    TP_STRUCT__entry(
    __field(int,	mds)
    __field(int,	op)
    __field(u64,	ino)
    __field(u64,	snap)
    __field(u32,	seq)
    __field(u32,	mseq)
    __field(u32,	issue_seq)
    ),

    TP_fast_assign(
    __entry->mds = session->s_mds;
    __entry->op = op;
    __entry->ino = vino->ino;
    __entry->snap = vino->snap;
    __entry->seq = seq;
    __entry->mseq = mseq;
    __entry->issue_seq = issue_seq;
    ),

    TP_printk("mds=%d op=%s vino=%llx.%llx seq=%u iseq=%u mseq=%u",
    __entry->mds,
    ceph_cap_op_name(__entry->op),
    __entry->ino,
    __entry->snap,
    __entry->seq,
    __entry->issue_seq,
    __entry->mseq)
    );

//
// Client reset tracepoints - identify the client by its monitor-
// assigned global_id so traces remain meaningful when kernel pointer
// hashing is enabled.
//
    TRACE_EVENT(ceph_client_reset_schedule,
    TP_PROTO(const struct ceph_mds_client *mdsc, const char *reason),
    TP_ARGS(mdsc, reason),
    TP_STRUCT__entry(
    __field(u64, client_id)
    __string(reason, reason ? reason : "")
    ),
    TP_fast_assign(
    __entry->client_id = mdsc->fsc->client->monc.auth ?
    mdsc->fsc->client->monc.auth->global_id : 0;
    __assign_str(reason);
    ),
    TP_printk("client_id=%llu reason=%s",
    __entry->client_id, __get_str(reason))
    );

    TRACE_EVENT(ceph_client_reset_complete,
    TP_PROTO(const struct ceph_mds_client *mdsc, int ret),
    TP_ARGS(mdsc, ret),
    TP_STRUCT__entry(
    __field(u64, client_id)
    __field(int, ret)
    ),
    TP_fast_assign(
    __entry->client_id = mdsc->fsc->client->monc.auth ?
    mdsc->fsc->client->monc.auth->global_id : 0;
    __entry->ret = ret;
    ),
    TP_printk("client_id=%llu ret=%d", __entry->client_id, __entry->ret)
    );

    TRACE_EVENT(ceph_client_reset_blocked,
    TP_PROTO(const struct ceph_mds_client *mdsc, int blocked_count),
    TP_ARGS(mdsc, blocked_count),
    TP_STRUCT__entry(
    __field(u64, client_id)
    __field(int, blocked_count)
    ),
    TP_fast_assign(
    __entry->client_id = mdsc->fsc->client->monc.auth ?
    mdsc->fsc->client->monc.auth->global_id : 0;
    __entry->blocked_count = blocked_count;
    ),
    TP_printk("client_id=%llu blocked_count=%d", __entry->client_id,
    __entry->blocked_count)
    );

    TRACE_EVENT(ceph_client_reset_unblocked,
    TP_PROTO(const struct ceph_mds_client *mdsc, int ret),
    TP_ARGS(mdsc, ret),
    TP_STRUCT__entry(
    __field(u64, client_id)
    __field(int, ret)
    ),
    TP_fast_assign(
    __entry->client_id = mdsc->fsc->client->monc.auth ?
    mdsc->fsc->client->monc.auth->global_id : 0;
    __entry->ret = ret;
    ),
    TP_printk("client_id=%llu ret=%d", __entry->client_id, __entry->ret)
    );

// This part must be outside protection
