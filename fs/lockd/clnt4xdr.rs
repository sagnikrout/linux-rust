//! Automatically rewritten from C to Rust
//! Source: fs/lockd/clnt4xdr.c
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
// linux/fs/lockd/clnt4xdr.c
//
// XDR functions to encode/decode NLM version 4 RPC arguments and results.
//
// NLM client-side only.
//
// Copyright (C) 2010, Oracle.  All rights reserved.
//

//
// Declare the space requirements for NLM arguments and replies as
// number of 32bit-words
//

#[no_mangle]
unsafe extern "C" fn loff_t_to_s64(offset: loff_t) -> i64 {
    static s64 loff_t_to_s64(loff_t offset)
    {
    s64 res;
    if (offset >= NLM4_OFFSET_MAX)
    res = NLM4_OFFSET_MAX;
#[no_mangle]
pub unsafe extern "C" fn if(-NLM4_OFFSET_MAX: offset <=) -> else {
    else if (offset <= -NLM4_OFFSET_MAX)
    res = -NLM4_OFFSET_MAX;
    else
    res = offset;
    return res;
    }
    static void nlm4_compute_offsets(const struct lockd_lock *lock,
    u64 *l_offset, u64 *l_len)
    {
    const struct file_lock *fl = &lock.fl;
// l_offset = loff_t_to_s64(fl->fl_start);
    if (fl.fl_end == OFFSET_MAX)
// l_len = 0;
    else
// l_len = loff_t_to_s64(fl->fl_end - fl->fl_start + 1);
    }
//
// Encode/decode NLMv4 basic data types
//
// Basic NLMv4 data types are defined in Appendix II, section 6.1.4
// of RFC 1813: "NFS Version 3 Protocol Specification" and in Chapter
// 10 of X/Open's "Protocols for Interworking: XNFS, Version 3W".
//
// Not all basic data types have their own encoding and decoding
// functions.  For run-time efficiency, some data types are encoded
// or decoded inline.
//
#[no_mangle]
unsafe extern "C" fn encode_bool(xdr: *mut xdr_stream, value: c_int) {
    static void encode_bool(struct xdr_stream *xdr, const int value)
    {
    __be32 *p;
    p = xdr_reserve_space(xdr, 4);
// p = value ? xdr_one : xdr_zero;
    }
#[no_mangle]
unsafe extern "C" fn encode_int32(xdr: *mut xdr_stream, value: i32) {
    static void encode_int32(struct xdr_stream *xdr, const s32 value)
    {
    __be32 *p;
    p = xdr_reserve_space(xdr, 4);
// p = cpu_to_be32(value);
    }
//
// typedef opaque netobj<MAXNETOBJ_SZ>
//
    static void encode_netobj(struct xdr_stream *xdr,
    const u8 *data, const unsigned int length)
    {
    __be32 *p;
    p = xdr_reserve_space(xdr, 4 + length);
    xdr_encode_opaque(p, data, length);
    }
    static int decode_netobj(struct xdr_stream *xdr,
    struct xdr_netobj *obj)
    {
    ssize_t ret;
    ret = xdr_stream_decode_opaque_inline(xdr, (void *)&obj.data,
    XDR_MAX_NETOBJ);
    if (unlikely(ret < 0))
    return -EIO;
    obj.len = ret;
    return 0;
    }
//
// netobj cookie;
//
    static void encode_cookie(struct xdr_stream *xdr,
    const struct lockd_cookie *cookie)
    {
    encode_netobj(xdr, (u8 *)&cookie.data, cookie.len);
    }
    static int decode_cookie(struct xdr_stream *xdr,
    struct lockd_cookie *cookie)
    {
    u32 length;
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    goto out_overflow;
    length = be32_to_cpup(p++);
// apparently HPUX can return empty cookies
    if (length == 0)
    goto out_hpux;
    if (length > NLM_MAXCOOKIELEN)
    goto out_size;
    p = xdr_inline_decode(xdr, length);
    if (unlikely(p == core::ptr::null_mut()))
    goto out_overflow;
    cookie.len = length;
    memcpy(cookie.data, p, length);
    return 0;
    out_hpux:
    cookie.len = 4;
    memset(cookie.data, 0, 4);
    return 0;
    out_size:
    dprintk("NFS: returned cookie was too long: %u\n", length);
    return -EIO;
    out_overflow:
    return -EIO;
    }
//
// netobj fh;
//
#[no_mangle]
unsafe extern "C" fn encode_fh(xdr: *mut xdr_stream, fh: *const nfs_fh) {
    static void encode_fh(struct xdr_stream *xdr, const struct nfs_fh *fh)
    {
    encode_netobj(xdr, (u8 *)&fh.data, fh.size);
    }
//
// enum nlm4_stats {
// NLM4_GRANTED = 0,
// NLM4_DENIED = 1,
// NLM4_DENIED_NOLOCKS = 2,
// NLM4_BLOCKED = 3,
// NLM4_DENIED_GRACE_PERIOD = 4,
// NLM4_DEADLCK = 5,
// NLM4_ROFS = 6,
// NLM4_STALE_FH = 7,
// NLM4_FBIG = 8,
// NLM4_FAILED = 9
// };
//
// struct nlm4_stat {
// nlm4_stats stat;
// };
//
// NB: we don't swap bytes for the NLM status values.  The upper
// layers deal directly with the status value in network byte
// order.
//
    static void encode_nlm4_stat(struct xdr_stream *xdr,
    const __be32 stat)
    {
    __be32 *p;
    BUG_ON(be32_to_cpu(stat) > NLM_FAILED);
    p = xdr_reserve_space(xdr, 4);
// p = stat;
    }
#[no_mangle]
unsafe extern "C" fn decode_nlm4_stat(xdr: *mut xdr_stream, stat: *mut __be32) -> c_int {
    static int decode_nlm4_stat(struct xdr_stream *xdr, __be32 *stat)
    {
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    goto out_overflow;
    if (unlikely(ntohl(*p) > ntohl(nlm4_failed)))
    goto out_bad_xdr;
// stat = *p;
    return 0;
    out_bad_xdr:
    dprintk("%s: server returned invalid nlm4_stats value: %u\n",
    __func__, be32_to_cpup(p));
    return -EIO;
    out_overflow:
    return -EIO;
    }
//
// struct nlm4_holder {
// bool	exclusive;
// int32	svid;
// netobj	oh;
// uint64	l_offset;
// uint64	l_len;
// };
//
    static void encode_nlm4_holder(struct xdr_stream *xdr,
    const struct lockd_res *result)
    {
    const struct lockd_lock *lock = &result.lock;
    u64 l_offset, l_len;
    __be32 *p;
    encode_bool(xdr, lock.fl.c.flc_type == F_RDLCK);
    encode_int32(xdr, lock.svid);
    encode_netobj(xdr, lock.oh.data, lock.oh.len);
    p = xdr_reserve_space(xdr, 4 + 4);
    nlm4_compute_offsets(lock, &l_offset, &l_len);
    p = xdr_encode_hyper(p, l_offset);
    xdr_encode_hyper(p, l_len);
    }
#[no_mangle]
unsafe extern "C" fn decode_nlm4_holder(xdr: *mut xdr_stream, result: *mut lockd_res) -> c_int {
    static int decode_nlm4_holder(struct xdr_stream *xdr, struct lockd_res *result)
    {
    struct lockd_lock *lock = &result.lock;
    struct file_lock *fl = &lock.fl;
    u64 l_offset, l_len;
    u32 exclusive;
    int error;
    __be32 *p;
    memset(lock, 0, sizeof(*lock));
    locks_init_lock(fl);
    p = xdr_inline_decode(xdr, 4 + 4);
    if (unlikely(p == core::ptr::null_mut()))
    goto out_overflow;
    exclusive = be32_to_cpup(p++);
    lock.svid = be32_to_cpup(p);
    fl.c.flc_pid = (pid_t)lock.svid;
    error = decode_netobj(xdr, &lock.oh);
    if (unlikely(error))
    goto out;
    p = xdr_inline_decode(xdr, 8 + 8);
    if (unlikely(p == core::ptr::null_mut()))
    goto out_overflow;
    fl.c.flc_flags = FL_POSIX;
    fl.c.flc_type  = exclusive != 0 ? F_WRLCK : F_RDLCK;
    p = xdr_decode_hyper(p, &l_offset);
    xdr_decode_hyper(p, &l_len);
    lockd_set_file_lock_range4(fl, l_offset, l_len);
    error = 0;
    out:
    return error;
    out_overflow:
    return -EIO;
    }
//
// string caller_name<LM_MAXSTRLEN>;
//
#[no_mangle]
unsafe extern "C" fn encode_caller_name(xdr: *mut xdr_stream, name: *const c_char) {
    static void encode_caller_name(struct xdr_stream *xdr, const char *name)
    {
// NB: client-side does not set lock->len
    let mut length: u32 = strlen(name);
    __be32 *p;
    p = xdr_reserve_space(xdr, 4 + length);
    xdr_encode_opaque(p, name, length);
    }
//
// struct nlm4_lock {
// string	caller_name<LM_MAXSTRLEN>;
// netobj	fh;
// netobj	oh;
// int32	svid;
// uint64	l_offset;
// uint64	l_len;
// };
//
    static void encode_nlm4_lock(struct xdr_stream *xdr,
    const struct lockd_lock *lock)
    {
    u64 l_offset, l_len;
    __be32 *p;
    encode_caller_name(xdr, lock.caller);
    encode_fh(xdr, &lock.fh);
    encode_netobj(xdr, lock.oh.data, lock.oh.len);
    p = xdr_reserve_space(xdr, 4 + 8 + 8);
// p++ = cpu_to_be32(lock->svid);
    nlm4_compute_offsets(lock, &l_offset, &l_len);
    p = xdr_encode_hyper(p, l_offset);
    xdr_encode_hyper(p, l_len);
    }
//
// NLMv4 XDR encode functions
//
// NLMv4 argument types are defined in Appendix II of RFC 1813:
// "NFS Version 3 Protocol Specification" and Chapter 10 of X/Open's
// "Protocols for Interworking: XNFS, Version 3W".
//
// struct nlm4_testargs {
// netobj cookie;
// bool exclusive;
// struct nlm4_lock alock;
// };
//
    static void nlm4_xdr_enc_testargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_bool(xdr, lock.fl.c.flc_type == F_WRLCK);
    encode_nlm4_lock(xdr, lock);
    }
//
// struct nlm4_lockargs {
// netobj cookie;
// bool block;
// bool exclusive;
// struct nlm4_lock alock;
// bool reclaim;
// int state;
// };
//
    static void nlm4_xdr_enc_lockargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_bool(xdr, args.block);
    encode_bool(xdr, lock.fl.c.flc_type == F_WRLCK);
    encode_nlm4_lock(xdr, lock);
    encode_bool(xdr, args.reclaim);
    encode_int32(xdr, args.state);
    }
//
// struct nlm4_cancargs {
// netobj cookie;
// bool block;
// bool exclusive;
// struct nlm4_lock alock;
// };
//
    static void nlm4_xdr_enc_cancargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_bool(xdr, args.block);
    encode_bool(xdr, lock.fl.c.flc_type == F_WRLCK);
    encode_nlm4_lock(xdr, lock);
    }
//
// struct nlm4_unlockargs {
// netobj cookie;
// struct nlm4_lock alock;
// };
//
    static void nlm4_xdr_enc_unlockargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_nlm4_lock(xdr, lock);
    }
//
// struct nlm4_res {
// netobj cookie;
// nlm4_stat stat;
// };
//
    static void nlm4_xdr_enc_res(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_res *result = data;
    encode_cookie(xdr, &result.cookie);
    encode_nlm4_stat(xdr, result.status);
    }
//
// union nlm4_testrply switch (nlm4_stats stat) {
// case NLM4_DENIED:
// struct nlm4_holder holder;
// default:
// void;
// };
//
// struct nlm4_testres {
// netobj cookie;
// nlm4_testrply test_stat;
// };
//
    static void nlm4_xdr_enc_testres(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_res *result = data;
    encode_cookie(xdr, &result.cookie);
    encode_nlm4_stat(xdr, result.status);
    if (result.status == nlm_lck_denied)
    encode_nlm4_holder(xdr, result);
    }
//
// NLMv4 XDR decode functions
//
// NLMv4 argument types are defined in Appendix II of RFC 1813:
// "NFS Version 3 Protocol Specification" and Chapter 10 of X/Open's
// "Protocols for Interworking: XNFS, Version 3W".
//
// union nlm4_testrply switch (nlm4_stats stat) {
// case NLM4_DENIED:
// struct nlm4_holder holder;
// default:
// void;
// };
//
// struct nlm4_testres {
// netobj cookie;
// nlm4_testrply test_stat;
// };
//
    static int decode_nlm4_testrply(struct xdr_stream *xdr,
    struct lockd_res *result)
    {
    int error;
    error = decode_nlm4_stat(xdr, &result.status);
    if (unlikely(error))
    goto out;
    if (result.status == nlm_lck_denied)
    error = decode_nlm4_holder(xdr, result);
    out:
    return error;
    }
    static int nlm4_xdr_dec_testres(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    void *data)
    {
    struct lockd_res *result = data;
    int error;
    error = decode_cookie(xdr, &result.cookie);
    if (unlikely(error))
    goto out;
    error = decode_nlm4_testrply(xdr, result);
    out:
    return error;
    }
//
// struct nlm4_res {
// netobj cookie;
// nlm4_stat stat;
// };
//
    static int nlm4_xdr_dec_res(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    void *data)
    {
    struct lockd_res *result = data;
    int error;
    error = decode_cookie(xdr, &result.cookie);
    if (unlikely(error))
    goto out;
    error = decode_nlm4_stat(xdr, &result.status);
    out:
    return error;
    }
//
// For NLM, a void procedure really returns nothing
//

    [NLMPROC_##proc] = {							\
    .p_proc      = NLMPROC_##proc,					\
    .p_encode    = nlm4_xdr_enc_##argtype,				\
    .p_decode    = nlm4_xdr_dec_##restype,				\
    .p_arglen    = NLM4_##argtype##_sz,				\
    .p_replen    = NLM4_##restype##_sz,				\
    .p_statidx   = NLMPROC_##proc,					\
    .p_name      = #proc,						\
    }
    static const struct rpc_procinfo nlm4_procedures[] = {
    PROC(TEST,		testargs,	testres),
    PROC(LOCK,		lockargs,	res),
    PROC(CANCEL,		cancargs,	res),
    PROC(UNLOCK,		unlockargs,	res),
    PROC(GRANTED,		testargs,	res),
    PROC(TEST_MSG,		testargs,	norep),
    PROC(LOCK_MSG,		lockargs,	norep),
    PROC(CANCEL_MSG,	cancargs,	norep),
    PROC(UNLOCK_MSG,	unlockargs,	norep),
    PROC(GRANTED_MSG,	testargs,	norep),
    PROC(TEST_RES,		testres,	norep),
    PROC(LOCK_RES,		res,		norep),
    PROC(CANCEL_RES,	res,		norep),
    PROC(UNLOCK_RES,	res,		norep),
    PROC(GRANTED_RES,	res,		norep),
    };
    static unsigned int nlm_version4_counts[ARRAY_SIZE(nlm4_procedures)];
    const struct rpc_version nlm_version4 = {
    .number		= 4,
    .nrprocs	= ARRAY_SIZE(nlm4_procedures),
    .procs		= nlm4_procedures,
    .counts		= nlm_version4_counts,
    };
