//! Automatically rewritten from C to Rust
//! Source: fs/nfs/mount_clnt.c
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
// In-kernel MOUNT protocol client
//
// Copyright (C) 1997, Olaf Kirch <okir@monad.swb.de>
//

//
// Defined by RFC 1094, section A.3; and RFC 1813, section 5.1.4
//

//
// XDR data type sizes
//

//
// XDR argument and result sizes
//

    MNT_authflav3_sz)
//
// Defined by RFC 1094, section A.5
//
    enum {
    MOUNTPROC_NULL		= 0,
    MOUNTPROC_MNT		= 1,
    MOUNTPROC_DUMP		= 2,
    MOUNTPROC_UMNT		= 3,
    MOUNTPROC_UMNTALL	= 4,
    MOUNTPROC_EXPORT	= 5,
    };
//
// Defined by RFC 1813, section 5.2
//
    enum {
    MOUNTPROC3_NULL		= 0,
    MOUNTPROC3_MNT		= 1,
    MOUNTPROC3_DUMP		= 2,
    MOUNTPROC3_UMNT		= 3,
    MOUNTPROC3_UMNTALL	= 4,
    MOUNTPROC3_EXPORT	= 5,
    };
    static const struct rpc_program mnt_program;
//
// Defined by OpenGroup XNFS Version 3W, chapter 8
//
    enum mountstat {
    MNT_OK			= 0,
    MNT_EPERM		= 1,
    MNT_ENOENT		= 2,
    MNT_EACCES		= 13,
    MNT_EINVAL		= 22,
    };
    static struct {
    u32 status;
    int errno;
    } mnt_errtbl[] = {
    { .status = MNT_OK,			.errno = 0,		},
    { .status = MNT_EPERM,			.errno = -EPERM,	},
    { .status = MNT_ENOENT,			.errno = -ENOENT,	},
    { .status = MNT_EACCES,			.errno = -EACCES,	},
    { .status = MNT_EINVAL,			.errno = -EINVAL,	},
    };
//
// Defined by RFC 1813, section 5.1.5
//
    enum mountstat3 {
    MNT3_OK			= 0,		/* no error */
    MNT3ERR_PERM		= 1,		/* Not owner */
    MNT3ERR_NOENT		= 2,		/* No such file or directory */
    MNT3ERR_IO		= 5,		/* I/O error */
    MNT3ERR_ACCES		= 13,		/* Permission denied */
    MNT3ERR_NOTDIR		= 20,		/* Not a directory */
    MNT3ERR_INVAL		= 22,		/* Invalid argument */
    MNT3ERR_NAMETOOLONG	= 63,		/* Filename too long */
    MNT3ERR_NOTSUPP		= 10004,	/* Operation not supported */
    MNT3ERR_SERVERFAULT	= 10006,	/* A failure on the server */
    };
    static struct {
    u32 status;
    int errno;
    } mnt3_errtbl[] = {
    { .status = MNT3_OK,			.errno = 0,		},
    { .status = MNT3ERR_PERM,		.errno = -EPERM,	},
    { .status = MNT3ERR_NOENT,		.errno = -ENOENT,	},
    { .status = MNT3ERR_IO,			.errno = -EIO,		},
    { .status = MNT3ERR_ACCES,		.errno = -EACCES,	},
    { .status = MNT3ERR_NOTDIR,		.errno = -ENOTDIR,	},
    { .status = MNT3ERR_INVAL,		.errno = -EINVAL,	},
    { .status = MNT3ERR_NAMETOOLONG,	.errno = -ENAMETOOLONG,	},
    { .status = MNT3ERR_NOTSUPP,		.errno = -ENOTSUPP,	},
    { .status = MNT3ERR_SERVERFAULT,	.errno = -EREMOTEIO,	},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mountres {
    pub errno: c_int,
    pub fh: *mut nfs_fh,
    pub auth_count: *mut c_uint,
    pub auth_flavors: *mut rpc_authflavor_t,
}

//
// nfs_mount - Obtain an NFS file handle for the given host and path
// @info: pointer to mount request arguments
// @timeo: deciseconds the mount waits for a response before it retries
// @retrans: number of times the mount retries a request
//
// Uses timeout parameters specified by caller. On successful return, the
// auth_flavs list and auth_flav_len will be populated with the list from the
// server or a faked-up list if the server didn't provide one.
//
#[no_mangle]
pub unsafe extern "C" fn nfs_mount(info: *mut nfs_mount_request, timeo: c_int, retrans: c_int) -> c_int {
    int nfs_mount(struct nfs_mount_request *info, int timeo, int retrans)
    {
    struct rpc_timeout mnt_timeout;
    struct mountres	result = {
    .fh		= info.fh,
    .auth_count	= info.auth_flav_len,
    .auth_flavors	= info.auth_flavs,
    };
    struct rpc_message msg	= {
    .rpc_argp	= info.dirpath,
    .rpc_resp	= &result,
    };
    struct rpc_create_args args = {
    .net		= info.net,
    .protocol	= info.protocol,
    .address	= (struct sockaddr *)info.sap,
    .addrsize	= info.salen,
    .timeout	= &mnt_timeout,
    .servername	= info.hostname,
    .program	= &mnt_program,
    .version	= info.version,
    .authflavor	= RPC_AUTH_UNIX,
    .cred		= current_cred(),
    };
    struct rpc_clnt		*mnt_clnt;
    int			status;
    dprintk("NFS: sending MNT request for %s:%s\n",
    (info.hostname ? info.hostname : "server"),
    info.dirpath);
    if (strlen(info.dirpath) > MNTPATHLEN)
    return -ENAMETOOLONG;
    if (info.noresvport)
    args.flags |= RPC_CLNT_CREATE_NONPRIVPORT;
    nfs_init_timeout_values(&mnt_timeout, info.protocol, timeo, retrans);
    mnt_clnt = rpc_create(&args);
    if (IS_ERR(mnt_clnt))
    goto out_clnt_err;
    if (info.version == NFS_MNT3_VERSION)
    msg.rpc_proc = &mnt_clnt.cl_procinfo[MOUNTPROC3_MNT];
    else
    msg.rpc_proc = &mnt_clnt.cl_procinfo[MOUNTPROC_MNT];
    status = rpc_call_sync(mnt_clnt, &msg, RPC_TASK_SOFT|RPC_TASK_TIMEOUT);
    rpc_shutdown_client(mnt_clnt);
    if (status < 0)
    goto out_call_err;
    if (result.errno != 0)
    goto out_mnt_err;
    dprintk("NFS: MNT request succeeded\n");
    status = 0;
//
// If the server didn't provide a flavor list, allow the
// client to try any flavor.
//
    if (info.version != NFS_MNT3_VERSION || *info.auth_flav_len == 0) {
    dprintk("NFS: Faking up auth_flavs list\n");
    info.auth_flavs[0] = RPC_AUTH_NULL;
// info->auth_flav_len = 1;
    }
    out:
    return status;
    out_clnt_err:
    status = PTR_ERR(mnt_clnt);
    dprintk("NFS: failed to create MNT RPC client, status=%d\n", status);
    goto out;
    out_call_err:
    dprintk("NFS: MNT request failed, status=%d\n", status);
    goto out;
    out_mnt_err:
    dprintk("NFS: MNT server returned result %d\n", result.errno);
    status = result.errno;
    goto out;
    }
//
// XDR encode/decode functions for MOUNT
//
#[no_mangle]
unsafe extern "C" fn encode_mntdirpath(xdr: *mut xdr_stream, pathname: *const c_char) {
    static void encode_mntdirpath(struct xdr_stream *xdr, const char *pathname)
    {
    let mut pathname_len: u32 = strlen(pathname);
    __be32 *p;
    p = xdr_reserve_space(xdr, 4 + pathname_len);
    xdr_encode_opaque(p, pathname, pathname_len);
    }
    static void mnt_xdr_enc_dirpath(struct rpc_rqst *req, struct xdr_stream *xdr,
    const void *dirpath)
    {
    encode_mntdirpath(xdr, dirpath);
    }
//
// RFC 1094: "A non-zero status indicates some sort of error.  In this
// case, the status is a UNIX error number."  This can be problematic
// if the server and client use different errno values for the same
// error.
//
// However, the OpenGroup XNFS spec provides a simple mapping that is
// independent of local errno values on the server and the client.
//
#[no_mangle]
unsafe extern "C" fn decode_status(xdr: *mut xdr_stream, res: *mut mountres) -> c_int {
    static int decode_status(struct xdr_stream *xdr, struct mountres *res)
    {
    unsigned int i;
    u32 status;
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return -EIO;
    status = be32_to_cpup(p);
    for (i = 0; i < ARRAY_SIZE(mnt_errtbl); i++) {
    if (mnt_errtbl[i].status == status) {
    res.errno = mnt_errtbl[i].errno;
    return 0;
    }
    }
    dprintk("NFS: unrecognized MNT status code: %u\n", status);
    res.errno = -EACCES;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_fhandle(xdr: *mut xdr_stream, res: *mut mountres) -> c_int {
    static int decode_fhandle(struct xdr_stream *xdr, struct mountres *res)
    {
    struct nfs_fh *fh = res.fh;
    __be32 *p;
    p = xdr_inline_decode(xdr, NFS2_FHSIZE);
    if (unlikely(p == core::ptr::null_mut()))
    return -EIO;
    fh.size = NFS2_FHSIZE;
    memcpy(fh.data, p, NFS2_FHSIZE);
    return 0;
    }
    static int mnt_xdr_dec_mountres(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    void *data)
    {
    struct mountres *res = data;
    int status;
    status = decode_status(xdr, res);
    if (unlikely(status != 0 || res.errno != 0))
    return status;
    return decode_fhandle(xdr, res);
    }
#[no_mangle]
unsafe extern "C" fn decode_fhs_status(xdr: *mut xdr_stream, res: *mut mountres) -> c_int {
    static int decode_fhs_status(struct xdr_stream *xdr, struct mountres *res)
    {
    unsigned int i;
    u32 status;
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return -EIO;
    status = be32_to_cpup(p);
    for (i = 0; i < ARRAY_SIZE(mnt3_errtbl); i++) {
    if (mnt3_errtbl[i].status == status) {
    res.errno = mnt3_errtbl[i].errno;
    return 0;
    }
    }
    dprintk("NFS: unrecognized MNT3 status code: %u\n", status);
    res.errno = -EACCES;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_fhandle3(xdr: *mut xdr_stream, res: *mut mountres) -> c_int {
    static int decode_fhandle3(struct xdr_stream *xdr, struct mountres *res)
    {
    struct nfs_fh *fh = res.fh;
    u32 size;
    __be32 *p;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return -EIO;
    size = be32_to_cpup(p);
    if (size > NFS3_FHSIZE || size == 0)
    return -EIO;
    p = xdr_inline_decode(xdr, size);
    if (unlikely(p == core::ptr::null_mut()))
    return -EIO;
    fh.size = size;
    memcpy(fh.data, p, size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn decode_auth_flavors(xdr: *mut xdr_stream, res: *mut mountres) -> c_int {
    static int decode_auth_flavors(struct xdr_stream *xdr, struct mountres *res)
    {
    rpc_authflavor_t *flavors = res.auth_flavors;
    unsigned int *count = res.auth_count;
    u32 entries, i;
    __be32 *p;
    if (*count == 0)
    return 0;
    p = xdr_inline_decode(xdr, 4);
    if (unlikely(p == core::ptr::null_mut()))
    return -EIO;
    entries = be32_to_cpup(p);
    dprintk("NFS: received %u auth flavors\n", entries);
    if (entries > NFS_MAX_SECFLAVORS)
    entries = NFS_MAX_SECFLAVORS;
    p = xdr_inline_decode(xdr, 4 * entries);
    if (unlikely(p == core::ptr::null_mut()))
    return -EIO;
    if (entries > *count)
    entries = *count;
    for (i = 0; i < entries; i++) {
    flavors[i] = be32_to_cpup(p++);
    dprintk("NFS:   auth flavor[%u]: %d\n", i, flavors[i]);
    }
// count = i;
    return 0;
    }
    static int mnt_xdr_dec_mountres3(struct rpc_rqst *req,
    struct xdr_stream *xdr,
    void *data)
    {
    struct mountres *res = data;
    int status;
    status = decode_fhs_status(xdr, res);
    if (unlikely(status != 0 || res.errno != 0))
    return status;
    status = decode_fhandle3(xdr, res);
    if (unlikely(status != 0)) {
    res.errno = -EBADHANDLE;
    return 0;
    }
    return decode_auth_flavors(xdr, res);
    }
    static const struct rpc_procinfo mnt_procedures[] = {
    [MOUNTPROC_MNT] = {
    .p_proc		= MOUNTPROC_MNT,
    .p_encode	= mnt_xdr_enc_dirpath,
    .p_decode	= mnt_xdr_dec_mountres,
    .p_arglen	= MNT_enc_dirpath_sz,
    .p_replen	= MNT_dec_mountres_sz,
    .p_statidx	= MOUNTPROC_MNT,
    .p_name		= "MOUNT",
    },
    [MOUNTPROC_UMNT] = {
    .p_proc		= MOUNTPROC_UMNT,
    .p_encode	= mnt_xdr_enc_dirpath,
    .p_arglen	= MNT_enc_dirpath_sz,
    .p_statidx	= MOUNTPROC_UMNT,
    .p_name		= "UMOUNT",
    },
    };
    static const struct rpc_procinfo mnt3_procedures[] = {
    [MOUNTPROC3_MNT] = {
    .p_proc		= MOUNTPROC3_MNT,
    .p_encode	= mnt_xdr_enc_dirpath,
    .p_decode	= mnt_xdr_dec_mountres3,
    .p_arglen	= MNT_enc_dirpath_sz,
    .p_replen	= MNT_dec_mountres3_sz,
    .p_statidx	= MOUNTPROC3_MNT,
    .p_name		= "MOUNT",
    },
    [MOUNTPROC3_UMNT] = {
    .p_proc		= MOUNTPROC3_UMNT,
    .p_encode	= mnt_xdr_enc_dirpath,
    .p_arglen	= MNT_enc_dirpath_sz,
    .p_statidx	= MOUNTPROC3_UMNT,
    .p_name		= "UMOUNT",
    },
    };
    static unsigned int mnt_counts[ARRAY_SIZE(mnt_procedures)];
    static const struct rpc_version mnt_version1 = {
    .number		= 1,
    .nrprocs	= ARRAY_SIZE(mnt_procedures),
    .procs		= mnt_procedures,
    .counts		= mnt_counts,
    };
    static unsigned int mnt3_counts[ARRAY_SIZE(mnt3_procedures)];
    static const struct rpc_version mnt_version3 = {
    .number		= 3,
    .nrprocs	= ARRAY_SIZE(mnt3_procedures),
    .procs		= mnt3_procedures,
    .counts		= mnt3_counts,
    };
    static const struct rpc_version *mnt_version[] = {
    core::ptr::null_mut(),
    &mnt_version1,
    core::ptr::null_mut(),
    &mnt_version3,
    };
    static struct rpc_stat mnt_stats;
    static const struct rpc_program mnt_program = {
    .name		= "mount",
    .number		= NFS_MNT_PROGRAM,
    .nrvers		= ARRAY_SIZE(mnt_version),
    .version	= mnt_version,
    .stats		= &mnt_stats,
    };
