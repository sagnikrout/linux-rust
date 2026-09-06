//! Automatically rewritten from C to Rust
//! Source: fs/nfsd/nfs3acl.c
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
// Process version 3 NFSACL requests.
//
// Copyright (C) 2002-2003 Andreas Gruenbacher <agruen@suse.de>
//

// FIXME: nfsacl.h is a broken header

//
// NULL call.
//
    static __be32
    nfsd3_proc_null(struct svc_rqst *rqstp)
    {
    return rpc_success;
    }
//
// Get the Access and/or Default ACL of a file.
//
#[no_mangle]
unsafe extern "C" fn nfsd3_proc_getacl(rqstp: *mut svc_rqst) -> __be32 {
    static __be32 nfsd3_proc_getacl(struct svc_rqst *rqstp)
    {
    struct nfsd3_getaclargs *argp = rqstp.rq_argp;
    struct nfsd3_getaclres *resp = rqstp.rq_resp;
    struct posix_acl *acl;
    struct inode *inode;
    svc_fh *fh;
    fh = fh_copy(&resp.fh, &argp.fh);
    resp.status = fh_verify(rqstp, &resp.fh, 0, NFSD_MAY_NOP);
    if (resp.status != nfs_ok)
    goto out;
    inode = d_inode(fh.fh_dentry);
    if (argp.mask & ~NFS_ACL_MASK) {
    resp.status = nfserr_inval;
    goto out;
    }
    resp.mask = argp.mask;
    if (resp.mask & (NFS_ACL|NFS_ACLCNT)) {
    acl = get_inode_acl(inode, ACL_TYPE_ACCESS);
    if (acl == core::ptr::null_mut()) {
// Solaris returns the inode's minimum ACL.
    acl = posix_acl_from_mode(inode.i_mode, GFP_KERNEL);
    }
    if (IS_ERR(acl)) {
    resp.status = nfserrno(PTR_ERR(acl));
    goto fail;
    }
    resp.acl_access = acl;
    }
    if (resp.mask & (NFS_DFACL|NFS_DFACLCNT)) {
// Check how Solaris handles requests for the Default ACL
    of a non-directory! */
    acl = get_inode_acl(inode, ACL_TYPE_DEFAULT);
    if (IS_ERR(acl)) {
    resp.status = nfserrno(PTR_ERR(acl));
    goto fail;
    }
    resp.acl_default = acl;
    }
// resp->acl_{access,default} are released in nfs3svc_release_getacl.
    out:
    return rpc_success;
    fail:
    posix_acl_release(resp.acl_access);
    posix_acl_release(resp.acl_default);
    resp.acl_access = core::ptr::null_mut();
    resp.acl_default = core::ptr::null_mut();
    goto out;
    }
//
// Set the Access and/or Default ACL of a file.
//
#[no_mangle]
unsafe extern "C" fn nfsd3_proc_setacl(rqstp: *mut svc_rqst) -> __be32 {
    static __be32 nfsd3_proc_setacl(struct svc_rqst *rqstp)
    {
    struct nfsd3_setaclargs *argp = rqstp.rq_argp;
    struct nfsd3_attrstat *resp = rqstp.rq_resp;
    struct inode *inode;
    svc_fh *fh;
    int error;
    fh = fh_copy(&resp.fh, &argp.fh);
    resp.status = fh_verify(rqstp, &resp.fh, 0, NFSD_MAY_SATTR);
    if (resp.status != nfs_ok)
    goto out;
    inode = d_inode(fh.fh_dentry);
    error = fh_want_write(fh);
    if (error)
    goto out_errno;
    inode_lock(inode);
    error = 0;
    if (argp.mask & NFS_ACL) {
    error = set_posix_acl(&nop_mnt_idmap, fh.fh_dentry,
    ACL_TYPE_ACCESS, argp.acl_access);
    if (error)
    goto out_drop_lock;
    }
    if (argp.mask & NFS_DFACL) {
    error = set_posix_acl(&nop_mnt_idmap, fh.fh_dentry,
    ACL_TYPE_DEFAULT, argp.acl_default);
    }
    out_drop_lock:
    inode_unlock(inode);
    fh_drop_write(fh);
    out_errno:
    resp.status = nfserrno(error);
    out:
// argp->acl_{access,default} are released in nfs3svc_release_setacl.
    return rpc_success;
    }
//
// XDR decode functions
//
    static bool
    nfs3svc_decode_getaclargs(struct svc_rqst *rqstp, struct xdr_stream *xdr)
    {
    struct nfsd3_getaclargs *args = rqstp.rq_argp;
    if (!svcxdr_decode_nfs_fh3(xdr, &args.fh))
    return false;
    if (xdr_stream_decode_u32(xdr, &args.mask) < 0)
    return false;
    return true;
    }
    static bool
    nfs3svc_decode_setaclargs(struct svc_rqst *rqstp, struct xdr_stream *xdr)
    {
    struct nfsd3_setaclargs *argp = rqstp.rq_argp;
    if (!svcxdr_decode_nfs_fh3(xdr, &argp.fh))
    return false;
    if (xdr_stream_decode_u32(xdr, &argp.mask) < 0)
    return false;
    if (argp.mask & ~NFS_ACL_MASK)
    return false;
    if (!nfs_stream_decode_acl(xdr, core::ptr::null_mut(), (argp.mask & NFS_ACL) ?
    &argp.acl_access : core::ptr::null_mut()))
    return false;
    if (!nfs_stream_decode_acl(xdr, core::ptr::null_mut(), (argp.mask & NFS_DFACL) ?
    &argp.acl_default : core::ptr::null_mut()))
    return false;
    return true;
    }
//
// XDR encode functions
//
// GETACL
    static bool
    nfs3svc_encode_getaclres(struct svc_rqst *rqstp, struct xdr_stream *xdr)
    {
    struct nfsd3_getaclres *resp = rqstp.rq_resp;
    struct dentry *dentry = resp.fh.fh_dentry;
    struct inode *inode;
    if (!svcxdr_encode_nfsstat3(xdr, resp.status))
    return false;
    switch (resp.status) {
    case nfs_ok:
    inode = d_inode(dentry);
    if (!svcxdr_encode_post_op_attr(rqstp, xdr, &resp.fh))
    return false;
    if (xdr_stream_encode_u32(xdr, resp.mask) < 0)
    return false;
    if (!nfs_stream_encode_acl(xdr, inode, resp.acl_access,
    resp.mask & NFS_ACL, 0))
    return false;
    if (!nfs_stream_encode_acl(xdr, inode, resp.acl_default,
    resp.mask & NFS_DFACL,
    NFS_ACL_DEFAULT))
    return false;
    break;
    default:
    if (!svcxdr_encode_post_op_attr(rqstp, xdr, &resp.fh))
    return false;
    }
    return true;
    }
// SETACL
    static bool
    nfs3svc_encode_setaclres(struct svc_rqst *rqstp, struct xdr_stream *xdr)
    {
    struct nfsd3_attrstat *resp = rqstp.rq_resp;
    return svcxdr_encode_nfsstat3(xdr, resp.status) &&
    svcxdr_encode_post_op_attr(rqstp, xdr, &resp.fh);
    }
//
// XDR release functions
//
#[no_mangle]
unsafe extern "C" fn nfs3svc_release_getacl(rqstp: *mut svc_rqst) {
    static void nfs3svc_release_getacl(struct svc_rqst *rqstp)
    {
    struct nfsd3_getaclres *resp = rqstp.rq_resp;
    fh_put(&resp.fh);
    posix_acl_release(resp.acl_access);
    posix_acl_release(resp.acl_default);
    }
#[no_mangle]
unsafe extern "C" fn nfs3svc_release_setacl(rqstp: *mut svc_rqst) {
    static void nfs3svc_release_setacl(struct svc_rqst *rqstp)
    {
    struct nfsd3_setaclargs *argp = rqstp.rq_argp;
    struct nfsd3_attrstat *resp = rqstp.rq_resp;
    fh_put(&resp.fh);
    posix_acl_release(argp.acl_access);
    posix_acl_release(argp.acl_default);
    }

    static const struct svc_procedure nfsd_acl_procedures3[3] = {
    [ACLPROC3_NULL] = {
    .pc_func = nfsd3_proc_null,
    .pc_decode = nfssvc_decode_voidarg,
    .pc_encode = nfssvc_encode_voidres,
    .pc_argsize = sizeof(struct nfsd_voidargs),
    .pc_argzero = sizeof(struct nfsd_voidargs),
    .pc_ressize = sizeof(struct nfsd_voidres),
    .pc_cachetype = RC_NOCACHE,
    .pc_xdrressize = ST,
    .pc_name = "core::ptr::null_mut()",
    },
    [ACLPROC3_GETACL] = {
    .pc_func = nfsd3_proc_getacl,
    .pc_decode = nfs3svc_decode_getaclargs,
    .pc_encode = nfs3svc_encode_getaclres,
    .pc_release = nfs3svc_release_getacl,
    .pc_argsize = sizeof(struct nfsd3_getaclargs),
    .pc_argzero = sizeof(struct nfsd3_getaclargs),
    .pc_ressize = sizeof(struct nfsd3_getaclres),
    .pc_cachetype = RC_NOCACHE,
    .pc_xdrressize = ST+1+2*(1+ACL),
    .pc_name = "GETACL",
    },
    [ACLPROC3_SETACL] = {
    .pc_func = nfsd3_proc_setacl,
    .pc_decode = nfs3svc_decode_setaclargs,
    .pc_encode = nfs3svc_encode_setaclres,
    .pc_release = nfs3svc_release_setacl,
    .pc_argsize = sizeof(struct nfsd3_setaclargs),
    .pc_argzero = sizeof(struct nfsd3_setaclargs),
    .pc_ressize = sizeof(struct nfsd3_attrstat),
    .pc_cachetype = RC_NOCACHE,
    .pc_xdrressize = ST+pAT,
    .pc_name = "SETACL",
    },
    };
    const struct svc_version nfsd_acl_version3 = {
    .vs_vers	= 3,
    .vs_nproc	= ARRAY_SIZE(nfsd_acl_procedures3),
    .vs_proc	= nfsd_acl_procedures3,
    .vs_dispatch	= nfsd_dispatch,
    .vs_xdrsize	= NFS3_SVC_XDRSIZE,
    };
