//! Automatically rewritten from C to Rust
//! Source: fs/nfs/callback_xdr.c
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
// linux/fs/nfs/callback_xdr.c
//
// Copyright (C) 2004 Trond Myklebust
//
// NFSv4 callback encode/decode procedures
//

    CB_OP_GETATTR_BITMAP_MAXSZ + \
// change, size, atime, ctime,
// mtime, deleg_atime, deleg_mtime */\
    (2 + 2 + 3 + 3 + 3 + 3 + 3) * 4)

    NFS4_MAX_SESSIONID_LEN + \
    (1 + 3) * 4) // seqid, 3 slotids

// Internal error code
pub const NFS4ERR_RESOURCE_HDR: c_int = 11050;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_op {
    pub ): *mut *mut *mut *mut __be32 (process_op)(void , void , struct cb_process_state,
    pub ): *mut *mut *mut *mut __be32 (decode_args)(struct svc_rqst , struct xdr_stream , void,
    __be32 (*encode_res)(struct svc_rqst *, struct xdr_stream *,
    pub ): *const c_void,
    pub res_maxsize: c_long,
}

    static struct callback_op callback_ops[];
#[no_mangle]
unsafe extern "C" fn nfs4_callback_null(rqstp: *mut svc_rqst) -> __be32 {
    static __be32 nfs4_callback_null(struct svc_rqst *rqstp)
    {
    return htonl(NFS4_OK);
    }
//
// svc_process_common() looks for an XDR encoder to know when
// not to drop a Reply.
//
#[no_mangle]
unsafe extern "C" fn nfs4_encode_void(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    static bool nfs4_encode_void(struct svc_rqst *rqstp, struct xdr_stream *xdr)
    {
    return true;
    }
    static __be32 decode_string(struct xdr_stream *xdr, unsigned int *len,
    const char **str, size_t maxlen)
    {
    ssize_t err;
    err = xdr_stream_decode_opaque_inline(xdr, (void **)str, maxlen);
    if (err < 0)
    return cpu_to_be32(NFS4ERR_RESOURCE);
// len = err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_fh(xdr: *mut xdr_stream, fh: *mut nfs_fh) -> __be32 {
    static __be32 decode_fh(struct xdr_stream *xdr, struct nfs_fh *fh)
    {
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    fh.size = ntohl(*p);
    if (fh.size > NFS4_FHSIZE)
    return htonl(NFS4ERR_BADHANDLE);
    p = xdr_inline_decode(xdr, fh.size);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    memcpy_and_pad(fh.data, sizeof(fh.data), p, fh.size, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_bitmap(xdr: *mut xdr_stream, bitmap: *mut u32) -> __be32 {
    static __be32 decode_bitmap(struct xdr_stream *xdr, uint32_t *bitmap)
    {
    __be32 *p;
    unsigned int attrlen;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    attrlen = ntohl(*p);
    p = xdr_inline_decode(xdr, attrlen << 2);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    if (likely(attrlen > 0))
    bitmap[0] = ntohl(*p++);
    if (attrlen > 1)
    bitmap[1] = ntohl(*p++);
    if (attrlen > 2)
    bitmap[2] = ntohl(*p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_stateid(xdr: *mut xdr_stream, stateid: *mut nfs4_stateid) -> __be32 {
    static __be32 decode_stateid(struct xdr_stream *xdr, nfs4_stateid *stateid)
    {
    __be32 *p;
    p = xdr_inline_decode(xdr, NFS4_STATEID_SIZE);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    memcpy(stateid.data, p, NFS4_STATEID_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_delegation_stateid(xdr: *mut xdr_stream, stateid: *mut nfs4_stateid) -> __be32 {
    static __be32 decode_delegation_stateid(struct xdr_stream *xdr, nfs4_stateid *stateid)
    {
    stateid.type = NFS4_DELEGATION_STATEID_TYPE;
    return decode_stateid(xdr, stateid);
    }
#[no_mangle]
unsafe extern "C" fn decode_compound_hdr_arg(xdr: *mut xdr_stream, hdr: *mut cb_compound_hdr_arg) -> __be32 {
    static __be32 decode_compound_hdr_arg(struct xdr_stream *xdr, struct cb_compound_hdr_arg *hdr)
    {
    __be32 *p;
    __be32 status;
    status = decode_string(xdr, &hdr.taglen, &hdr.tag, CB_OP_TAGLEN_MAXSZ);
    if (unlikely(status != 0))
    return status;
    p = xdr_inline_decode(xdr, 12);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    hdr.minorversion = ntohl(*p++);
// Check for minor version support
    if (hdr.minorversion <= NFS4_MAX_MINOR_VERSION) {
    hdr.cb_ident = ntohl(*p++); /* ignored by v4.1 and v4.2 */
    } else {
    pr_warn_ratelimited("NFS: %s: NFSv4 server callback with "
    "illegal minor version %u!\n",
    __func__, hdr.minorversion);
    return htonl(NFS4ERR_MINOR_VERS_MISMATCH);
    }
    hdr.nops = ntohl(*p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_op_hdr(xdr: *mut xdr_stream, op: *mut c_uint) -> __be32 {
    static __be32 decode_op_hdr(struct xdr_stream *xdr, unsigned int *op)
    {
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE_HDR);
// op = ntohl(*p);
    return 0;
    }
    static __be32 decode_getattr_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr, void *argp)
    {
    struct cb_getattrargs *args = argp;
    __be32 status;
    status = decode_fh(xdr, &args.fh);
    if (unlikely(status != 0))
    return status;
    return decode_bitmap(xdr, args.bitmap);
    }
    static __be32 decode_recall_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr, void *argp)
    {
    struct cb_recallargs *args = argp;
    __be32 *p;
    __be32 status;
    status = decode_delegation_stateid(xdr, &args.stateid);
    if (unlikely(status != 0))
    return status;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    args.truncate = ntohl(*p);
    return decode_fh(xdr, &args.fh);
    }
#[no_mangle]
unsafe extern "C" fn decode_layout_stateid(xdr: *mut xdr_stream, stateid: *mut nfs4_stateid) -> __be32 {
    static __be32 decode_layout_stateid(struct xdr_stream *xdr, nfs4_stateid *stateid)
    {
    stateid.type = NFS4_LAYOUT_STATEID_TYPE;
    return decode_stateid(xdr, stateid);
    }
    static __be32 decode_layoutrecall_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr, void *argp)
    {
    struct cb_layoutrecallargs *args = argp;
    __be32 *p;
    let mut status: __be32 = 0;
    uint32_t iomode;
    p = xdr_inline_decode(xdr, 4 * sizeof(uint32_t));
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_BADXDR);
    args.cbl_layout_type = ntohl(*p++);
// Depite the spec's xdr, iomode really belongs in the FILE switch,
// as it is unusable and ignored with the other types.
//
    iomode = ntohl(*p++);
    args.cbl_layoutchanged = ntohl(*p++);
    args.cbl_recall_type = ntohl(*p++);
    if (args.cbl_recall_type == RETURN_FILE) {
    args.cbl_range.iomode = iomode;
    status = decode_fh(xdr, &args.cbl_fh);
    if (unlikely(status != 0))
    return status;
    p = xdr_inline_decode(xdr, 2 * sizeof(uint64_t));
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_BADXDR);
    p = xdr_decode_hyper(p, &args.cbl_range.offset);
    p = xdr_decode_hyper(p, &args.cbl_range.length);
    return decode_layout_stateid(xdr, &args.cbl_stateid);
    } else if (args.cbl_recall_type == RETURN_FSID) {
    p = xdr_inline_decode(xdr, 2 * sizeof(uint64_t));
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_BADXDR);
    p = xdr_decode_hyper(p, &args.cbl_fsid.major);
    p = xdr_decode_hyper(p, &args.cbl_fsid.minor);
    } else if (args.cbl_recall_type != RETURN_ALL)
    return htonl(NFS4ERR_BADXDR);
    return 0;
    }
    static
    __be32 decode_devicenotify_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr,
    void *argp)
    {
    struct cb_devicenotifyargs *args = argp;
    uint32_t tmp, n, i;
    __be32 *p;
    let mut status: __be32 = 0;
// Num of device notifications
    p = xdr_inline_decode(xdr, sizeof(uint32_t));
    if (unlikely(p == core::ptr::null_mut())) {
    status = htonl(NFS4ERR_BADXDR);
    goto out;
    }
    n = ntohl(*p++);
    if (n == 0)
    goto out;
    args.devs = kmalloc_objs(*args.devs, n);
    if (!args.devs) {
    status = htonl(NFS4ERR_DELAY);
    goto out;
    }
// Decode each dev notification
    for (i = 0; i < n; i++) {
    struct cb_devicenotifyitem *dev = &args.devs[i];
    p = xdr_inline_decode(xdr, (4 * sizeof(uint32_t)) +
    NFS4_DEVICEID4_SIZE);
    if (unlikely(p == core::ptr::null_mut())) {
    status = htonl(NFS4ERR_BADXDR);
    goto err;
    }
    tmp = ntohl(*p++);	/* bitmap size */
    if (tmp != 1) {
    status = htonl(NFS4ERR_INVAL);
    goto err;
    }
    dev.cbd_notify_type = ntohl(*p++);
    if (dev.cbd_notify_type != NOTIFY_DEVICEID4_CHANGE &&
    dev.cbd_notify_type != NOTIFY_DEVICEID4_DELETE) {
    status = htonl(NFS4ERR_INVAL);
    goto err;
    }
    tmp = ntohl(*p++);	/* opaque size */
    if (((dev.cbd_notify_type == NOTIFY_DEVICEID4_CHANGE) &&
    (tmp != NFS4_DEVICEID4_SIZE + 8)) ||
    ((dev.cbd_notify_type == NOTIFY_DEVICEID4_DELETE) &&
    (tmp != NFS4_DEVICEID4_SIZE + 4))) {
    status = htonl(NFS4ERR_INVAL);
    goto err;
    }
    dev.cbd_layout_type = ntohl(*p++);
    memcpy(dev.cbd_dev_id.data, p, NFS4_DEVICEID4_SIZE);
    p += XDR_QUADLEN(NFS4_DEVICEID4_SIZE);
    if (dev.cbd_layout_type == NOTIFY_DEVICEID4_CHANGE) {
    p = xdr_inline_decode(xdr, sizeof(uint32_t));
    if (unlikely(p == core::ptr::null_mut())) {
    status = htonl(NFS4ERR_BADXDR);
    goto err;
    }
    dev.cbd_immediate = ntohl(*p++);
    } else {
    dev.cbd_immediate = 0;
    }
    dprintk("%s: type %d layout 0x%x immediate %d\n",
    __func__, dev.cbd_notify_type, dev.cbd_layout_type,
    dev.cbd_immediate);
    }
    args.ndevs = n;
    dprintk("%s: ndevs %d\n", __func__, args.ndevs);
    return 0;
    err:
    kfree(args.devs);
    out:
    args.devs = core::ptr::null_mut();
    args.ndevs = 0;
    dprintk("%s: status %d ndevs %d\n",
    __func__, ntohl(status), args.ndevs);
    return status;
    }
    static __be32 decode_sessionid(struct xdr_stream *xdr,
    struct nfs4_sessionid *sid)
    {
    __be32 *p;
    p = xdr_inline_decode(xdr, NFS4_MAX_SESSIONID_LEN);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    memcpy(sid.data, p, NFS4_MAX_SESSIONID_LEN);
    return 0;
    }
    static __be32 decode_rc_list(struct xdr_stream *xdr,
    struct referring_call_list *rc_list)
    {
    __be32 *p;
    int i;
    __be32 status;
    status = decode_sessionid(xdr, &rc_list.rcl_sessionid);
    if (status)
    goto out;
    status = htonl(NFS4ERR_RESOURCE);
    p = xdr_inline_decode(xdr, sizeof(uint32_t));
    if (unlikely(p == core::ptr::null_mut()))
    goto out;
    rc_list.rcl_nrefcalls = ntohl(*p++);
    if (rc_list.rcl_nrefcalls) {
    if (unlikely(rc_list.rcl_nrefcalls > xdr.buf.len))
    goto out;
    p = xdr_inline_decode(xdr,
    rc_list.rcl_nrefcalls * 2 * sizeof(uint32_t));
    if (unlikely(p == core::ptr::null_mut()))
    goto out;
    rc_list.rcl_refcalls = kmalloc_objs(*rc_list.rcl_refcalls,
    rc_list.rcl_nrefcalls);
    if (unlikely(rc_list.rcl_refcalls == core::ptr::null_mut()))
    goto out;
    for (i = 0; i < rc_list.rcl_nrefcalls; i++) {
    rc_list.rcl_refcalls[i].rc_sequenceid = ntohl(*p++);
    rc_list.rcl_refcalls[i].rc_slotid = ntohl(*p++);
    }
    }
    status = 0;
    out:
    return status;
    }
    static __be32 decode_cb_sequence_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr,
    void *argp)
    {
    struct cb_sequenceargs *args = argp;
    __be32 *p;
    int i;
    __be32 status;
    status = decode_sessionid(xdr, &args.csa_sessionid);
    if (status)
    return status;
    p = xdr_inline_decode(xdr, 5 * sizeof(uint32_t));
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    args.csa_addr = svc_addr(rqstp);
    args.csa_sequenceid = ntohl(*p++);
    args.csa_slotid = ntohl(*p++);
    args.csa_highestslotid = ntohl(*p++);
    args.csa_cachethis = ntohl(*p++);
    args.csa_nrclists = ntohl(*p++);
    args.csa_rclists = core::ptr::null_mut();
    if (args.csa_nrclists) {
    args.csa_rclists = kzalloc_objs(*args.csa_rclists,
    args.csa_nrclists);
    if (unlikely(args.csa_rclists == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    for (i = 0; i < args.csa_nrclists; i++) {
    status = decode_rc_list(xdr, &args.csa_rclists[i]);
    if (status) {
    args.csa_nrclists = i;
    goto out_free;
    }
    }
    }
    return 0;
    out_free:
    for (i = 0; i < args.csa_nrclists; i++)
    kfree(args.csa_rclists[i].rcl_refcalls);
    kfree(args.csa_rclists);
    return status;
    }
    static __be32 decode_recallany_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr,
    void *argp)
    {
    struct cb_recallanyargs *args = argp;
    uint32_t bitmap[3];
    __be32 *p, status;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_BADXDR);
    args.craa_objs_to_keep = ntohl(*p++);
    status = decode_bitmap(xdr, bitmap);
    if (unlikely(status))
    return status;
    args.craa_type_mask = bitmap[0];
    return 0;
    }
    static __be32 decode_recallslot_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr,
    void *argp)
    {
    struct cb_recallslotargs *args = argp;
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_BADXDR);
    args.crsa_target_highest_slotid = ntohl(*p++);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_lockowner(xdr: *mut xdr_stream, args: *mut cb_notify_lock_args) -> __be32 {
    static __be32 decode_lockowner(struct xdr_stream *xdr, struct cb_notify_lock_args *args)
    {
    __be32		*p;
    unsigned int	len;
    p = xdr_inline_decode(xdr, 12);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_BADXDR);
    p = xdr_decode_hyper(p, &args.cbnl_owner.clientid);
    len = be32_to_cpu(*p);
    p = xdr_inline_decode(xdr, len);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_BADXDR);
// Only try to decode if the length is right
    if (len == 20) {
    p += 2;	/* skip "lock id:" */
    args.cbnl_owner.s_dev = be32_to_cpu(*p++);
    xdr_decode_hyper(p, &args.cbnl_owner.id);
    args.cbnl_valid = true;
    } else {
    args.cbnl_owner.s_dev = 0;
    args.cbnl_owner.id = 0;
    args.cbnl_valid = false;
    }
    return 0;
    }
    static __be32 decode_notify_lock_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr, void *argp)
    {
    struct cb_notify_lock_args *args = argp;
    __be32 status;
    status = decode_fh(xdr, &args.cbnl_fh);
    if (unlikely(status != 0))
    return status;
    return decode_lockowner(xdr, args);
    }

    static __be32 decode_write_response(struct xdr_stream *xdr,
    struct cb_offloadargs *args)
    {
    __be32 *p;
// skip the always zero field
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(!p))
    goto out;
    p++;
// decode count, stable_how, verifier
    p = xdr_inline_decode(xdr, 8 + 4);
    if (unlikely(!p))
    goto out;
    p = xdr_decode_hyper(p, &args.wr_count);
    args.wr_writeverf.committed = be32_to_cpup(p);
    p = xdr_inline_decode(xdr, NFS4_VERIFIER_SIZE);
    if (likely(p)) {
    memcpy(&args.wr_writeverf.verifier.data[0], p,
    NFS4_VERIFIER_SIZE);
    return 0;
    }
    out:
    return htonl(NFS4ERR_RESOURCE);
    }
    static __be32 decode_offload_args(struct svc_rqst *rqstp,
    struct xdr_stream *xdr,
    void *data)
    {
    struct cb_offloadargs *args = data;
    __be32 *p;
    __be32 status;
// decode fh
    status = decode_fh(xdr, &args.coa_fh);
    if (unlikely(status != 0))
    return status;
// decode stateid
    status = decode_stateid(xdr, &args.coa_stateid);
    if (unlikely(status != 0))
    return status;
// decode status
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(!p))
    goto out;
    args.error = ntohl(*p++);
    if (!args.error) {
    status = decode_write_response(xdr, args);
    if (unlikely(status != 0))
    return status;
    } else {
    p = xdr_inline_decode(xdr, 8);
    if (unlikely(!p))
    goto out;
    p = xdr_decode_hyper(p, &args.wr_count);
    }
    return 0;
    out:
    return htonl(NFS4ERR_RESOURCE);
    }

#[no_mangle]
unsafe extern "C" fn encode_string(xdr: *mut xdr_stream, len: c_uint, str: *const c_char) -> __be32 {
    static __be32 encode_string(struct xdr_stream *xdr, unsigned int len, const char *str)
    {
    if (unlikely(xdr_stream_encode_opaque(xdr, str, len) < 0))
    return cpu_to_be32(NFS4ERR_RESOURCE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn encode_attr_bitmap(xdr: *mut xdr_stream, bitmap: *const u32, sz: usize) -> __be32 {
    static __be32 encode_attr_bitmap(struct xdr_stream *xdr, const uint32_t *bitmap, size_t sz)
    {
    if (xdr_stream_encode_uint32_array(xdr, bitmap, sz) < 0)
    return cpu_to_be32(NFS4ERR_RESOURCE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn encode_attr_change(xdr: *mut xdr_stream, bitmap: *const u32, change: u64) -> __be32 {
    static __be32 encode_attr_change(struct xdr_stream *xdr, const uint32_t *bitmap, uint64_t change)
    {
    __be32 *p;
    if (!(bitmap[0] & FATTR4_WORD0_CHANGE))
    return 0;
    p = xdr_reserve_space(xdr, 8);
    if (unlikely(!p))
    return htonl(NFS4ERR_RESOURCE);
    p = xdr_encode_hyper(p, change);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn encode_attr_size(xdr: *mut xdr_stream, bitmap: *const u32, size: u64) -> __be32 {
    static __be32 encode_attr_size(struct xdr_stream *xdr, const uint32_t *bitmap, uint64_t size)
    {
    __be32 *p;
    if (!(bitmap[0] & FATTR4_WORD0_SIZE))
    return 0;
    p = xdr_reserve_space(xdr, 8);
    if (unlikely(!p))
    return htonl(NFS4ERR_RESOURCE);
    p = xdr_encode_hyper(p, size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn encode_attr_time(xdr: *mut xdr_stream, time: *const timespec64) -> __be32 {
    static __be32 encode_attr_time(struct xdr_stream *xdr, const struct timespec64 *time)
    {
    __be32 *p;
    p = xdr_reserve_space(xdr, 12);
    if (unlikely(!p))
    return htonl(NFS4ERR_RESOURCE);
    p = xdr_encode_hyper(p, time.tv_sec);
// p = htonl(time->tv_nsec);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn encode_attr_atime(xdr: *mut xdr_stream, bitmap: *const u32, time: *const timespec64) -> __be32 {
    static __be32 encode_attr_atime(struct xdr_stream *xdr, const uint32_t *bitmap, const struct timespec64 *time)
    {
    if (!(bitmap[1] & FATTR4_WORD1_TIME_ACCESS))
    return 0;
    return encode_attr_time(xdr,time);
    }
#[no_mangle]
unsafe extern "C" fn encode_attr_ctime(xdr: *mut xdr_stream, bitmap: *const u32, time: *const timespec64) -> __be32 {
    static __be32 encode_attr_ctime(struct xdr_stream *xdr, const uint32_t *bitmap, const struct timespec64 *time)
    {
    if (!(bitmap[1] & FATTR4_WORD1_TIME_METADATA))
    return 0;
    return encode_attr_time(xdr,time);
    }
#[no_mangle]
unsafe extern "C" fn encode_attr_mtime(xdr: *mut xdr_stream, bitmap: *const u32, time: *const timespec64) -> __be32 {
    static __be32 encode_attr_mtime(struct xdr_stream *xdr, const uint32_t *bitmap, const struct timespec64 *time)
    {
    if (!(bitmap[1] & FATTR4_WORD1_TIME_MODIFY))
    return 0;
    return encode_attr_time(xdr,time);
    }
    static __be32 encode_attr_delegatime(struct xdr_stream *xdr,
    const uint32_t *bitmap,
    const struct timespec64 *time)
    {
    if (!(bitmap[2] & FATTR4_WORD2_TIME_DELEG_ACCESS))
    return 0;
    return encode_attr_time(xdr,time);
    }
    static __be32 encode_attr_delegmtime(struct xdr_stream *xdr,
    const uint32_t *bitmap,
    const struct timespec64 *time)
    {
    if (!(bitmap[2] & FATTR4_WORD2_TIME_DELEG_MODIFY))
    return 0;
    return encode_attr_time(xdr,time);
    }
#[no_mangle]
unsafe extern "C" fn encode_compound_hdr_res(xdr: *mut xdr_stream, hdr: *mut cb_compound_hdr_res) -> __be32 {
    static __be32 encode_compound_hdr_res(struct xdr_stream *xdr, struct cb_compound_hdr_res *hdr)
    {
    __be32 status;
    hdr.status = xdr_reserve_space(xdr, 4);
    if (unlikely(hdr.status == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    status = encode_string(xdr, hdr.taglen, hdr.tag);
    if (unlikely(status != 0))
    return status;
    hdr.nops = xdr_reserve_space(xdr, 4);
    if (unlikely(hdr.nops == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn encode_op_hdr(xdr: *mut xdr_stream, op: u32, res: __be32) -> __be32 {
    static __be32 encode_op_hdr(struct xdr_stream *xdr, uint32_t op, __be32 res)
    {
    __be32 *p;
    p = xdr_reserve_space(xdr, 8);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE_HDR);
// p++ = htonl(op);
// p = res;
    return 0;
    }
    static __be32 encode_getattr_res(struct svc_rqst *rqstp, struct xdr_stream *xdr,
    const void *resp)
    {
    const struct cb_getattrres *res = resp;
    __be32 *savep = core::ptr::null_mut();
    let mut status: __be32 = res.status;
    if (unlikely(status != 0))
    goto out;
    status = encode_attr_bitmap(xdr, res.bitmap, ARRAY_SIZE(res.bitmap));
    if (unlikely(status != 0))
    goto out;
    status = cpu_to_be32(NFS4ERR_RESOURCE);
    savep = xdr_reserve_space(xdr, sizeof(*savep));
    if (unlikely(!savep))
    goto out;
    status = encode_attr_change(xdr, res.bitmap, res.change_attr);
    if (unlikely(status != 0))
    goto out;
    status = encode_attr_size(xdr, res.bitmap, res.size);
    if (unlikely(status != 0))
    goto out;
    status = encode_attr_atime(xdr, res.bitmap, &res.atime);
    if (unlikely(status != 0))
    goto out;
    status = encode_attr_ctime(xdr, res.bitmap, &res.ctime);
    if (unlikely(status != 0))
    goto out;
    status = encode_attr_mtime(xdr, res.bitmap, &res.mtime);
    if (unlikely(status != 0))
    goto out;
    status = encode_attr_delegatime(xdr, res.bitmap, &res.atime);
    if (unlikely(status != 0))
    goto out;
    status = encode_attr_delegmtime(xdr, res.bitmap, &res.mtime);
// savep = htonl((unsigned int)((char *)xdr->p - (char *)(savep+1)));
    out:
    return status;
    }
    static __be32 encode_sessionid(struct xdr_stream *xdr,
    const struct nfs4_sessionid *sid)
    {
    __be32 *p;
    p = xdr_reserve_space(xdr, NFS4_MAX_SESSIONID_LEN);
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
    memcpy(p, sid, NFS4_MAX_SESSIONID_LEN);
    return 0;
    }
    static __be32 encode_cb_sequence_res(struct svc_rqst *rqstp,
    struct xdr_stream *xdr,
    const void *resp)
    {
    const struct cb_sequenceres *res = resp;
    __be32 *p;
    let mut status: __be32 = res.csr_status;
    if (unlikely(status != 0))
    return status;
    status = encode_sessionid(xdr, &res.csr_sessionid);
    if (status)
    return status;
    p = xdr_reserve_space(xdr, 4 * sizeof(uint32_t));
    if (unlikely(p == core::ptr::null_mut()))
    return htonl(NFS4ERR_RESOURCE);
// p++ = htonl(res->csr_sequenceid);
// p++ = htonl(res->csr_slotid);
// p++ = htonl(res->csr_highestslotid);
// p++ = htonl(res->csr_target_highestslotid);
    return 0;
    }
    static __be32
    preprocess_nfs41_op(int nop, unsigned int op_nr, struct callback_op **op)
    {
    if (op_nr == OP_CB_SEQUENCE) {
    if (nop != 0)
    return htonl(NFS4ERR_SEQUENCE_POS);
    } else {
    if (nop == 0)
    return htonl(NFS4ERR_OP_NOT_IN_SESSION);
    }
    switch (op_nr) {
    case OP_CB_GETATTR:
    case OP_CB_RECALL:
    case OP_CB_SEQUENCE:
    case OP_CB_RECALL_ANY:
    case OP_CB_RECALL_SLOT:
    case OP_CB_LAYOUTRECALL:
    case OP_CB_NOTIFY_DEVICEID:
    case OP_CB_NOTIFY_LOCK:
// op = &callback_ops[op_nr];
    break;
    case OP_CB_NOTIFY:
    case OP_CB_PUSH_DELEG:
    case OP_CB_RECALLABLE_OBJ_AVAIL:
    case OP_CB_WANTS_CANCELLED:
    return htonl(NFS4ERR_NOTSUPP);
    default:
    return htonl(NFS4ERR_OP_ILLEGAL);
    }
    return htonl(NFS_OK);
    }
    static void nfs4_callback_free_slot(struct nfs4_session *session,
    struct nfs4_slot *slot)
    {
    struct nfs4_slot_table *tbl = &session.bc_slot_table;
    spin_lock(&tbl.slot_tbl_lock);
//
// Let the state manager know callback processing done.
// A single slot, so highest used slotid is either 0 or -1
//
    nfs4_free_slot(tbl, slot);
    spin_unlock(&tbl.slot_tbl_lock);
    }
#[no_mangle]
unsafe extern "C" fn nfs4_cb_free_slot(cps: *mut cb_process_state) {
    static void nfs4_cb_free_slot(struct cb_process_state *cps)
    {
    if (cps.slot) {
    nfs4_callback_free_slot(cps.clp.cl_session, cps.slot);
    cps.slot = core::ptr::null_mut();
    }
    }

    static __be32
    preprocess_nfs42_op(int nop, unsigned int op_nr, struct callback_op **op)
    {
    let mut status: __be32 = preprocess_nfs41_op(nop, op_nr, op);
    if (status != htonl(NFS4ERR_OP_ILLEGAL))
    return status;
    if (op_nr == OP_CB_OFFLOAD) {
// op = &callback_ops[op_nr];
    return htonl(NFS_OK);
    } else
    return htonl(NFS4ERR_NOTSUPP);
    return htonl(NFS4ERR_OP_ILLEGAL);
    }

    static __be32
    preprocess_nfs42_op(int nop, unsigned int op_nr, struct callback_op **op)
    {
    return htonl(NFS4ERR_MINOR_VERS_MISMATCH);
    }

    static __be32
    preprocess_nfs4_op(unsigned int op_nr, struct callback_op **op)
    {
    switch (op_nr) {
    case OP_CB_GETATTR:
    case OP_CB_RECALL:
// op = &callback_ops[op_nr];
    break;
    default:
    return htonl(NFS4ERR_OP_ILLEGAL);
    }
    return htonl(NFS_OK);
    }
    static __be32 process_op(int nop, struct svc_rqst *rqstp,
    struct cb_process_state *cps)
    {
    struct xdr_stream *xdr_out = &rqstp.rq_res_stream;
    struct callback_op *op = &callback_ops[0];
    unsigned int op_nr;
    __be32 status;
    long maxlen;
    __be32 res;
    status = decode_op_hdr(&rqstp.rq_arg_stream, &op_nr);
    if (unlikely(status))
    return status;
    switch (cps.minorversion) {
    case 0:
    status = preprocess_nfs4_op(op_nr, &op);
    break;
    case 1:
    status = preprocess_nfs41_op(nop, op_nr, &op);
    break;
    case 2:
    status = preprocess_nfs42_op(nop, op_nr, &op);
    break;
    default:
    status = htonl(NFS4ERR_MINOR_VERS_MISMATCH);
    }
    if (status == htonl(NFS4ERR_OP_ILLEGAL))
    op_nr = OP_CB_ILLEGAL;
    if (status)
    goto encode_hdr;
    if (cps.drc_status) {
    status = cps.drc_status;
    goto encode_hdr;
    }
    maxlen = xdr_out.end - xdr_out.p;
    if (maxlen > 0 && maxlen < PAGE_SIZE) {
    status = op.decode_args(rqstp, &rqstp.rq_arg_stream,
    rqstp.rq_argp);
    if (likely(status == 0))
    status = op.process_op(rqstp.rq_argp, rqstp.rq_resp,
    cps);
    } else
    status = htonl(NFS4ERR_RESOURCE);
    encode_hdr:
    res = encode_op_hdr(xdr_out, op_nr, status);
    if (unlikely(res))
    return res;
    if (op.encode_res != core::ptr::null_mut() && status == 0)
    status = op.encode_res(rqstp, xdr_out, rqstp.rq_resp);
    return status;
    }
//
// Decode, process and encode a COMPOUND
//
#[no_mangle]
unsafe extern "C" fn nfs4_callback_compound(rqstp: *mut svc_rqst) -> __be32 {
    static __be32 nfs4_callback_compound(struct svc_rqst *rqstp)
    {
    let mut hdr_arg: cb_compound_hdr_arg = { 0 };
    let mut hdr_res: cb_compound_hdr_res = { core::ptr::null_mut() };
    struct cb_process_state cps = {
    .drc_status = 0,
    .clp = core::ptr::null_mut(),
    .net = SVC_NET(rqstp),
    };
    let mut nops: c_uint = 0;
    __be32 status;
    status = decode_compound_hdr_arg(&rqstp.rq_arg_stream, &hdr_arg);
    if (status == htonl(NFS4ERR_RESOURCE))
    return rpc_garbage_args;
    if (hdr_arg.minorversion == 0) {
    cps.clp = nfs4_find_client_ident(SVC_NET(rqstp), hdr_arg.cb_ident);
    if (!cps.clp) {
    trace_nfs_cb_no_clp(rqstp.rq_xid, hdr_arg.cb_ident);
    goto out_invalidcred;
    }
    if (!check_gss_callback_principal(cps.clp, rqstp)) {
    trace_nfs_cb_badprinc(rqstp.rq_xid, hdr_arg.cb_ident);
    nfs_put_client(cps.clp);
    goto out_invalidcred;
    }
    svc_xprt_set_valid(rqstp.rq_xprt);
    }
    cps.minorversion = hdr_arg.minorversion;
    hdr_res.taglen = hdr_arg.taglen;
    hdr_res.tag = hdr_arg.tag;
    if (encode_compound_hdr_res(&rqstp.rq_res_stream, &hdr_res) != 0) {
    if (cps.clp)
    nfs_put_client(cps.clp);
    return rpc_system_err;
    }
    while (status == 0 && nops != hdr_arg.nops) {
    status = process_op(nops, rqstp, &cps);
    nops++;
    }
// Buffer overflow in decode_ops_hdr or encode_ops_hdr. Return
// resource error in cb_compound status without returning op
    if (unlikely(status == htonl(NFS4ERR_RESOURCE_HDR))) {
    status = htonl(NFS4ERR_RESOURCE);
    nops--;
    }
    if (svc_is_backchannel(rqstp) && cps.clp) {
    rqstp.bc_to_initval = cps.clp.cl_rpcclient.cl_timeout.to_initval;
    rqstp.bc_to_retries = cps.clp.cl_rpcclient.cl_timeout.to_retries;
    }
// hdr_res.status = status;
// hdr_res.nops = htonl(nops);
    nfs4_cb_free_slot(&cps);
    nfs_put_client(cps.clp);
    return rpc_success;
    out_invalidcred:
    pr_warn_ratelimited("NFS: NFSv4 callback contains invalid cred\n");
    rqstp.rq_auth_stat = rpc_autherr_badcred;
    return rpc_success;
    }
    static int
    nfs_callback_dispatch(struct svc_rqst *rqstp)
    {
    const struct svc_procedure *procp = rqstp.rq_procinfo;
// rqstp->rq_accept_statp = procp->pc_func(rqstp);
    return 1;
    }
//
// Define NFS4 callback COMPOUND ops.
//
    static struct callback_op callback_ops[] = {
    [0] = {
    .res_maxsize = CB_OP_HDR_RES_MAXSZ,
    },
    [OP_CB_GETATTR] = {
    .process_op = nfs4_callback_getattr,
    .decode_args = decode_getattr_args,
    .encode_res = encode_getattr_res,
    .res_maxsize = CB_OP_GETATTR_RES_MAXSZ,
    },
    [OP_CB_RECALL] = {
    .process_op = nfs4_callback_recall,
    .decode_args = decode_recall_args,
    .res_maxsize = CB_OP_RECALL_RES_MAXSZ,
    },
    [OP_CB_LAYOUTRECALL] = {
    .process_op = nfs4_callback_layoutrecall,
    .decode_args = decode_layoutrecall_args,
    .res_maxsize = CB_OP_LAYOUTRECALL_RES_MAXSZ,
    },
    [OP_CB_NOTIFY_DEVICEID] = {
    .process_op = nfs4_callback_devicenotify,
    .decode_args = decode_devicenotify_args,
    .res_maxsize = CB_OP_DEVICENOTIFY_RES_MAXSZ,
    },
    [OP_CB_SEQUENCE] = {
    .process_op = nfs4_callback_sequence,
    .decode_args = decode_cb_sequence_args,
    .encode_res = encode_cb_sequence_res,
    .res_maxsize = CB_OP_SEQUENCE_RES_MAXSZ,
    },
    [OP_CB_RECALL_ANY] = {
    .process_op = nfs4_callback_recallany,
    .decode_args = decode_recallany_args,
    .res_maxsize = CB_OP_RECALLANY_RES_MAXSZ,
    },
    [OP_CB_RECALL_SLOT] = {
    .process_op = nfs4_callback_recallslot,
    .decode_args = decode_recallslot_args,
    .res_maxsize = CB_OP_RECALLSLOT_RES_MAXSZ,
    },
    [OP_CB_NOTIFY_LOCK] = {
    .process_op = nfs4_callback_notify_lock,
    .decode_args = decode_notify_lock_args,
    .res_maxsize = CB_OP_NOTIFY_LOCK_RES_MAXSZ,
    },

    [OP_CB_OFFLOAD] = {
    .process_op = nfs4_callback_offload,
    .decode_args = decode_offload_args,
    .res_maxsize = CB_OP_OFFLOAD_RES_MAXSZ,
    },

    };
//
// Define NFS4 callback procedures
//
    static const struct svc_procedure nfs4_callback_procedures1[] = {
    [CB_NULL] = {
    .pc_func = nfs4_callback_null,
    .pc_encode = nfs4_encode_void,
    .pc_xdrressize = 1,
    .pc_name = "core::ptr::null_mut()",
    },
    [CB_COMPOUND] = {
    .pc_func = nfs4_callback_compound,
    .pc_encode = nfs4_encode_void,
    .pc_argsize = 256,
    .pc_argzero = 256,
    .pc_ressize = 256,
    .pc_xdrressize = NFS4_CALLBACK_BUFSIZE,
    .pc_name = "COMPOUND",
    }
    };
    const struct svc_version nfs4_callback_version1 = {
    .vs_vers = 1,
    .vs_nproc = ARRAY_SIZE(nfs4_callback_procedures1),
    .vs_proc = nfs4_callback_procedures1,
    .vs_xdrsize = NFS4_CALLBACK_XDRSIZE,
    .vs_dispatch = nfs_callback_dispatch,
    .vs_hidden = true,
    .vs_need_cong_ctrl = true,
    };
    const struct svc_version nfs4_callback_version4 = {
    .vs_vers = 4,
    .vs_nproc = ARRAY_SIZE(nfs4_callback_procedures1),
    .vs_proc = nfs4_callback_procedures1,
    .vs_xdrsize = NFS4_CALLBACK_XDRSIZE,
    .vs_dispatch = nfs_callback_dispatch,
    .vs_hidden = true,
    .vs_need_cong_ctrl = true,
    };
