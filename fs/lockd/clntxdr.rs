//! Automatically rewritten from C to Rust
//! Source: fs/lockd/clntxdr.c
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
// linux/fs/lockd/clntxdr.c
//
// XDR functions to encode/decode NLM version 1 and 3 RPC
// arguments and results. NLM version 2 is not specified
// by a standard, thus it is not implemented.
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
unsafe extern "C" fn loff_t_to_s32(offset: loff_t) -> i32 {
    static s32 loff_t_to_s32(loff_t offset)
    {
    s32 res;
    if (offset >= NLM_OFFSET_MAX)
    res = NLM_OFFSET_MAX;
#[no_mangle]
pub unsafe extern "C" fn if(-NLM_OFFSET_MAX: offset <=) -> else {
    else if (offset <= -NLM_OFFSET_MAX)
    res = -NLM_OFFSET_MAX;
    else
    res = offset;
    return res;
    }
    static void nlm_compute_offsets(const struct lockd_lock *lock,
    u32 *l_offset, u32 *l_len)
    {
    const struct file_lock *fl = &lock.fl;
// l_offset = loff_t_to_s32(fl->fl_start);
    if (fl.fl_end == OFFSET_MAX)
// l_len = 0;
    else
// l_len = loff_t_to_s32(fl->fl_end - fl->fl_start + 1);
    }
//
// Encode/decode NLMv3 basic data types
//
// Basic NLMv3 data types are not defined in an IETF standards
// document.  X/Open has a description of these data types that
// is useful.  See Chapter 10 of "Protocols for Interworking:
// XNFS, Version 3W".
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
    encode_netobj(xdr, (u8 *)&fh.data, NFS2_FHSIZE);
    }
//
// enum nlm_stats {
// LCK_GRANTED = 0,
// LCK_DENIED = 1,
// LCK_DENIED_NOLOCKS = 2,
// LCK_BLOCKED = 3,
// LCK_DENIED_GRACE_PERIOD = 4
// };
//
// struct nlm_stat {
// nlm_stats stat;
// };
//
// NB: we don't swap bytes for the NLM status values.  The upper
// layers deal directly with the status value in network byte
// order.
//
    static void encode_nlm_stat(struct xdr_stream *xdr,
    const __be32 stat)
    {
    __be32 *p;
    WARN_ON_ONCE(be32_to_cpu(stat) > NLM_LCK_DENIED_GRACE_PERIOD);
    p = xdr_reserve_space(xdr, 4);
// p = stat;
    }
    static int decode_nlm_stat(struct xdr_stream *xdr,
    __be32 *stat)
    {
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    goto out_overflow;
    if (unlikely(ntohl(*p) > ntohl(nlm_lck_denied_grace_period)))
    goto out_enum;
// stat = *p;
    return 0;
    out_enum:
    dprintk("%s: server returned invalid nlm_stats value: %u\n",
    __func__, be32_to_cpup(p));
    return -EIO;
    out_overflow:
    return -EIO;
    }
//
// struct nlm_holder {
// bool exclusive;
// int uppid;
// netobj oh;
// unsigned l_offset;
// unsigned l_len;
// };
//
    static void encode_nlm_holder(struct xdr_stream *xdr,
    const struct lockd_res *result)
    {
    const struct lockd_lock *lock = &result.lock;
    u32 l_offset, l_len;
    __be32 *p;
    encode_bool(xdr, lock.fl.c.flc_type == F_RDLCK);
    encode_int32(xdr, lock.svid);
    encode_netobj(xdr, lock.oh.data, lock.oh.len);
    p = xdr_reserve_space(xdr, 4 + 4);
    nlm_compute_offsets(lock, &l_offset, &l_len);
// p++ = cpu_to_be32(l_offset);
// p   = cpu_to_be32(l_len);
    }
#[no_mangle]
unsafe extern "C" fn decode_nlm_holder(xdr: *mut xdr_stream, result: *mut lockd_res) -> c_int {
    static int decode_nlm_holder(struct xdr_stream *xdr, struct lockd_res *result)
    {
    struct lockd_lock *lock = &result.lock;
    struct file_lock *fl = &lock.fl;
    u32 exclusive, l_offset, l_len;
    int error;
    __be32 *p;
    s32 end;
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
    p = xdr_inline_decode(xdr, 4 + 4);
    if (unlikely(p == core::ptr::null_mut()))
    goto out_overflow;
    fl.c.flc_flags = FL_POSIX;
    fl.c.flc_type  = exclusive != 0 ? F_WRLCK : F_RDLCK;
    l_offset = be32_to_cpup(p++);
    l_len = be32_to_cpup(p);
    end = l_offset + l_len - 1;
    fl.fl_start = (loff_t)l_offset;
    if (l_len == 0 || end < 0)
    fl.fl_end = OFFSET_MAX;
    else
    fl.fl_end = (loff_t)end;
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
// struct nlm_lock {
// string caller_name<LM_MAXSTRLEN>;
// netobj fh;
// netobj oh;
// int uppid;
// unsigned l_offset;
// unsigned l_len;
// };
//
    static void encode_nlm_lock(struct xdr_stream *xdr,
    const struct lockd_lock *lock)
    {
    u32 l_offset, l_len;
    __be32 *p;
    encode_caller_name(xdr, lock.caller);
    encode_fh(xdr, &lock.fh);
    encode_netobj(xdr, lock.oh.data, lock.oh.len);
    p = xdr_reserve_space(xdr, 4 + 4 + 4);
// p++ = cpu_to_be32(lock->svid);
    nlm_compute_offsets(lock, &l_offset, &l_len);
// p++ = cpu_to_be32(l_offset);
// p   = cpu_to_be32(l_len);
    }
//
// NLMv3 XDR encode functions
//
// NLMv3 argument types are defined in Chapter 10 of The Open Group's
// "Protocols for Interworking: XNFS, Version 3W".
//
// struct nlm_testargs {
// netobj cookie;
// bool exclusive;
// struct nlm_lock alock;
// };
//
    static void nlm_xdr_enc_testargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_bool(xdr, lock.fl.c.flc_type == F_WRLCK);
    encode_nlm_lock(xdr, lock);
    }
//
// struct nlm_lockargs {
// netobj cookie;
// bool block;
// bool exclusive;
// struct nlm_lock alock;
// bool reclaim;
// int state;
// };
//
    static void nlm_xdr_enc_lockargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_bool(xdr, args.block);
    encode_bool(xdr, lock.fl.c.flc_type == F_WRLCK);
    encode_nlm_lock(xdr, lock);
    encode_bool(xdr, args.reclaim);
    encode_int32(xdr, args.state);
    }
//
// struct nlm_cancargs {
// netobj cookie;
// bool block;
// bool exclusive;
// struct nlm_lock alock;
// };
//
    static void nlm_xdr_enc_cancargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_bool(xdr, args.block);
    encode_bool(xdr, lock.fl.c.flc_type == F_WRLCK);
    encode_nlm_lock(xdr, lock);
    }
//
// struct nlm_unlockargs {
// netobj cookie;
// struct nlm_lock alock;
// };
//
    static void nlm_xdr_enc_unlockargs(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_args *args = data;
    const struct lockd_lock *lock = &args.lock;
    encode_cookie(xdr, &args.cookie);
    encode_nlm_lock(xdr, lock);
    }
//
// struct nlm_res {
// netobj cookie;
// nlm_stat stat;
// };
//
    static void nlm_xdr_enc_res(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_res *result = data;
    encode_cookie(xdr, &result.cookie);
    encode_nlm_stat(xdr, result.status);
    }
//
// union nlm_testrply switch (nlm_stats stat) {
// case LCK_DENIED:
// struct nlm_holder holder;
// default:
// void;
// };
//
// struct nlm_testres {
// netobj cookie;
// nlm_testrply test_stat;
// };
//
    static void encode_nlm_testrply(struct xdr_stream *xdr,
    const struct lockd_res *result)
    {
    if (result.status == nlm_lck_denied)
    encode_nlm_holder(xdr, result);
    }
    static void nlm_xdr_enc_testres(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    const void *data)
    {
    const struct lockd_res *result = data;
    encode_cookie(xdr, &result.cookie);
    encode_nlm_stat(xdr, result.status);
    encode_nlm_testrply(xdr, result);
    }
//
// NLMv3 XDR decode functions
//
// NLMv3 result types are defined in Chapter 10 of The Open Group's
// "Protocols for Interworking: XNFS, Version 3W".
//
// union nlm_testrply switch (nlm_stats stat) {
// case LCK_DENIED:
// struct nlm_holder holder;
// default:
// void;
// };
//
// struct nlm_testres {
// netobj cookie;
// nlm_testrply test_stat;
// };
//
    static int decode_nlm_testrply(struct xdr_stream *xdr,
    struct lockd_res *result)
    {
    int error;
    error = decode_nlm_stat(xdr, &result.status);
    if (unlikely(error))
    goto out;
    if (result.status == nlm_lck_denied)
    error = decode_nlm_holder(xdr, result);
    out:
    return error;
    }
    static int nlm_xdr_dec_testres(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    void *data)
    {
    struct lockd_res *result = data;
    int error;
    error = decode_cookie(xdr, &result.cookie);
    if (unlikely(error))
    goto out;
    error = decode_nlm_testrply(xdr, result);
    out:
    return error;
    }
//
// struct nlm_res {
// netobj cookie;
// nlm_stat stat;
// };
//
    static int nlm_xdr_dec_res(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    void *data)
    {
    struct lockd_res *result = data;
    int error;
    error = decode_cookie(xdr, &result.cookie);
    if (unlikely(error))
    goto out;
    error = decode_nlm_stat(xdr, &result.status);
    out:
    return error;
    }
//
// For NLM, a void procedure really returns nothing
//

    [NLMPROC_##proc] = {							\
    .p_proc      = NLMPROC_##proc,					\
    .p_encode    = nlm_xdr_enc_##argtype,		\
    .p_decode    = nlm_xdr_dec_##restype,				\
    .p_arglen    = NLM_##argtype##_sz,				\
    .p_replen    = NLM_##restype##_sz,				\
    .p_statidx   = NLMPROC_##proc,					\
    .p_name      = #proc,						\
    }
    static const struct rpc_procinfo nlm_procedures[] = {
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
    static unsigned int nlm_version1_counts[ARRAY_SIZE(nlm_procedures)];
    static const struct rpc_version	nlm_version1 = {
    .number		= 1,
    .nrprocs	= ARRAY_SIZE(nlm_procedures),
    .procs		= nlm_procedures,
    .counts		= nlm_version1_counts,
    };
    static unsigned int nlm_version3_counts[ARRAY_SIZE(nlm_procedures)];
    static const struct rpc_version	nlm_version3 = {
    .number		= 3,
    .nrprocs	= ARRAY_SIZE(nlm_procedures),
    .procs		= nlm_procedures,
    .counts		= nlm_version3_counts,
    };
    static const struct rpc_version	*nlm_versions[] = {
    [1] = &nlm_version1,
    [3] = &nlm_version3,

    [4] = &nlm_version4,

    };
    static struct rpc_stat		nlm_rpc_stats;
    const struct rpc_program	nlm_program = {
    .name		= "lockd",
    .number		= NLM_PROGRAM,
    .nrvers		= ARRAY_SIZE(nlm_versions),
    .version	= nlm_versions,
    .stats		= &nlm_rpc_stats,
    };
