//! Automatically rewritten from C to Rust
//! Source: fs/nfs/nfs3acl.c
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
// nfs3_prepare_get_acl, nfs3_complete_get_acl, nfs3_abort_get_acl: Helpers for
// caching get_acl results in a race-free way.  See fs/posix_acl.c:get_acl()
// for explanations.
//
#[no_mangle]
unsafe extern "C" fn nfs3_prepare_get_acl(p: *mut posix_acl) {
    static void nfs3_prepare_get_acl(struct posix_acl **p)
    {
    struct posix_acl *sentinel = uncached_acl_sentinel(current);
// If the ACL isn't being read yet, set our sentinel.
    cmpxchg(p, ACL_NOT_CACHED, sentinel);
    }
#[no_mangle]
unsafe extern "C" fn nfs3_complete_get_acl(p: *mut posix_acl, acl: *mut posix_acl) {
    static void nfs3_complete_get_acl(struct posix_acl **p, struct posix_acl *acl)
    {
    struct posix_acl *sentinel = uncached_acl_sentinel(current);
// Only cache the ACL if our sentinel is still in place.
    posix_acl_dup(acl);
    if (cmpxchg(p, sentinel, acl) != sentinel)
    posix_acl_release(acl);
    }
#[no_mangle]
unsafe extern "C" fn nfs3_abort_get_acl(p: *mut posix_acl) {
    static void nfs3_abort_get_acl(struct posix_acl **p)
    {
    struct posix_acl *sentinel = uncached_acl_sentinel(current);
// Remove our sentinel upon failure.
    cmpxchg(p, sentinel, ACL_NOT_CACHED);
    }
    struct posix_acl *nfs3_get_acl(struct inode *inode, int type, bool rcu)
    {
    struct nfs_server *server = NFS_SERVER(inode);
    struct page *pages[NFSACL_MAXPAGES] = { };
    struct nfs3_getaclargs args = {
    .fh = NFS_FH(inode),
// The xdr layer may allocate pages here.
    .pages = pages,
    };
    struct nfs3_getaclres res = {
    core::ptr::null_mut(),
    };
    struct rpc_message msg = {
    .rpc_argp	= &args,
    .rpc_resp	= &res,
    };
    int status, count;
    if (rcu)
    return ERR_PTR(-ECHILD);
    if (!nfs_server_capable(inode, NFS_CAP_ACLS))
    return ERR_PTR(-EOPNOTSUPP);
    status = nfs_revalidate_inode(inode, NFS_INO_INVALID_CHANGE);
    if (status < 0)
    return ERR_PTR(status);
//
// Only get the access acl when explicitly requested: We don't
// need it for access decisions, and only some applications use
// it. Applications which request the access acl first are not
// penalized from this optimization.
//
    if (type == ACL_TYPE_ACCESS)
    args.mask |= NFS_ACLCNT|NFS_ACL;
    if (S_ISDIR(inode.i_mode))
    args.mask |= NFS_DFACLCNT|NFS_DFACL;
    if (args.mask == 0)
    return core::ptr::null_mut();
    dprintk("NFS call getacl\n");
    msg.rpc_proc = &server.client_acl.cl_procinfo[ACLPROC3_GETACL];
    res.fattr = nfs_alloc_fattr();
    if (res.fattr == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    if (args.mask & NFS_ACL)
    nfs3_prepare_get_acl(&inode.i_acl);
    if (args.mask & NFS_DFACL)
    nfs3_prepare_get_acl(&inode.i_default_acl);
    status = rpc_call_sync(server.client_acl, &msg, 0);
    dprintk("NFS reply getacl: %d\n", status);
// pages may have been allocated at the xdr layer.
    for (count = 0; count < NFSACL_MAXPAGES && args.pages[count]; count++)
    __free_page(args.pages[count]);
    switch (status) {
    case 0:
    nfs_refresh_inode(inode, res.fattr);
    break;
    case -EPFNOSUPPORT:
    case -EPROTONOSUPPORT:
    dprintk("NFS_V3_ACL extension not supported; disabling\n");
    server.caps &= ~NFS_CAP_ACLS;
    fallthrough;
    case -ENOTSUPP:
    status = -EOPNOTSUPP;
    goto getout;
    default:
    goto getout;
    }
    if ((args.mask & res.mask) != args.mask) {
    status = -EIO;
    goto getout;
    }
    if (res.acl_access != core::ptr::null_mut()) {
    if ((posix_acl_equiv_mode(res.acl_access, core::ptr::null_mut()) == 0) ||
    res.acl_access.a_count == 0) {
    posix_acl_release(res.acl_access);
    res.acl_access = core::ptr::null_mut();
    }
    }
    if (res.mask & NFS_ACL)
    nfs3_complete_get_acl(&inode.i_acl, res.acl_access);
    else
    forget_cached_acl(inode, ACL_TYPE_ACCESS);
    if (res.mask & NFS_DFACL)
    nfs3_complete_get_acl(&inode.i_default_acl, res.acl_default);
    else
    forget_cached_acl(inode, ACL_TYPE_DEFAULT);
    nfs_free_fattr(res.fattr);
    if (type == ACL_TYPE_ACCESS) {
    posix_acl_release(res.acl_default);
    return res.acl_access;
    } else {
    posix_acl_release(res.acl_access);
    return res.acl_default;
    }
    getout:
    nfs3_abort_get_acl(&inode.i_acl);
    nfs3_abort_get_acl(&inode.i_default_acl);
    posix_acl_release(res.acl_access);
    posix_acl_release(res.acl_default);
    nfs_free_fattr(res.fattr);
    return ERR_PTR(status);
    }
    static int __nfs3_proc_setacls(struct inode *inode, struct posix_acl *acl,
    struct posix_acl *dfacl)
    {
    struct nfs_server *server = NFS_SERVER(inode);
    struct nfs_fattr *fattr;
    struct page *pages[NFSACL_MAXPAGES];
    struct nfs3_setaclargs args = {
    .inode = inode,
    .mask = NFS_ACL,
    .acl_access = acl,
    .pages = pages,
    };
    struct rpc_message msg = {
    .rpc_argp	= &args,
    .rpc_resp	= &fattr,
    };
    let mut status: c_int = 0;
    if (acl == core::ptr::null_mut() && (!S_ISDIR(inode.i_mode) || dfacl == core::ptr::null_mut()))
    goto out;
    status = -EOPNOTSUPP;
    if (!nfs_server_capable(inode, NFS_CAP_ACLS))
    goto out;
// We are doing this here because XDR marshalling does not
// return any results, it BUGs.
    status = -ENOSPC;
    if (acl != core::ptr::null_mut() && acl.a_count > NFS_ACL_MAX_ENTRIES)
    goto out;
    if (dfacl != core::ptr::null_mut() && dfacl.a_count > NFS_ACL_MAX_ENTRIES)
    goto out;
    if (S_ISDIR(inode.i_mode)) {
    args.mask |= NFS_DFACL;
    args.acl_default = dfacl;
    args.len = nfsacl_size(acl, dfacl);
    } else
    args.len = nfsacl_size(acl, core::ptr::null_mut());
    if (args.len > NFS_ACL_INLINE_BUFSIZE) {
    let mut npages: c_uint = 1 + ((args.len - 1) >> PAGE_SHIFT);
    status = -ENOMEM;
    do {
    args.pages[args.npages] = alloc_page(GFP_KERNEL);
    if (args.pages[args.npages] == core::ptr::null_mut())
    goto out_freepages;
    args.npages++;
    } while (args.npages < npages);
    }
    dprintk("NFS call setacl\n");
    status = -ENOMEM;
    fattr = nfs_alloc_fattr();
    if (fattr == core::ptr::null_mut())
    goto out_freepages;
    msg.rpc_proc = &server.client_acl.cl_procinfo[ACLPROC3_SETACL];
    msg.rpc_resp = fattr;
    status = rpc_call_sync(server.client_acl, &msg, 0);
    nfs_access_zap_cache(inode);
    nfs_zap_acl_cache(inode);
    dprintk("NFS reply setacl: %d\n", status);
    switch (status) {
    case 0:
    status = nfs_refresh_inode(inode, fattr);
    break;
    case -EPFNOSUPPORT:
    case -EPROTONOSUPPORT:
    dprintk("NFS_V3_ACL SETACL RPC not supported"
    "(will not retry)\n");
    server.caps &= ~NFS_CAP_ACLS;
    fallthrough;
    case -ENOTSUPP:
    status = -EOPNOTSUPP;
    }
    nfs_free_fattr(fattr);
    out_freepages:
    while (args.npages != 0) {
    args.npages--;
    __free_page(args.pages[args.npages]);
    }
    out:
    return status;
    }
    int nfs3_proc_setacls(struct inode *inode, struct posix_acl *acl,
    struct posix_acl *dfacl)
    {
    int ret;
    ret = __nfs3_proc_setacls(inode, acl, dfacl);
    return (ret == -EOPNOTSUPP) ? 0 : ret;
    }
    int nfs3_set_acl(struct mnt_idmap *idmap, struct dentry *dentry,
    struct posix_acl *acl, int type)
    {
    struct posix_acl *orig = acl, *dfacl = core::ptr::null_mut(), *alloc;
    struct inode *inode = d_inode(dentry);
    int status;
    if (S_ISDIR(inode.i_mode)) {
    switch(type) {
    case ACL_TYPE_ACCESS:
    alloc = get_inode_acl(inode, ACL_TYPE_DEFAULT);
    if (IS_ERR(alloc))
    goto fail;
    dfacl = alloc;
    break;
    case ACL_TYPE_DEFAULT:
    alloc = get_inode_acl(inode, ACL_TYPE_ACCESS);
    if (IS_ERR(alloc))
    goto fail;
    dfacl = acl;
    acl = alloc;
    break;
    }
    }
    if (acl == core::ptr::null_mut()) {
    alloc = posix_acl_from_mode(inode.i_mode, GFP_KERNEL);
    if (IS_ERR(alloc))
    goto fail;
    acl = alloc;
    }
    status = __nfs3_proc_setacls(inode, acl, dfacl);
    out:
    if (acl != orig)
    posix_acl_release(acl);
    if (dfacl != orig)
    posix_acl_release(dfacl);
    return status;
    fail:
    status = PTR_ERR(alloc);
    goto out;
    }
    static int
    nfs3_list_one_acl(struct inode *inode, int type, const char *name, void *data,
    size_t size, ssize_t *result)
    {
    struct posix_acl *acl;
    char *p = data + *result;
    acl = get_inode_acl(inode, type);
    if (IS_ERR_OR_NULL(acl))
    return 0;
    posix_acl_release(acl);
// result += strlen(name);
// result += 1;
    if (!size)
    return 0;
    if (*result > size)
    return -ERANGE;
    strcpy(p, name);
    return 0;
    }
    ssize_t
    nfs3_listxattr(struct dentry *dentry, char *data, size_t size)
    {
    struct inode *inode = d_inode(dentry);
    let mut result: isize = 0;
    int error;
    error = nfs3_list_one_acl(inode, ACL_TYPE_ACCESS,
    XATTR_NAME_POSIX_ACL_ACCESS, data, size, &result);
    if (error)
    return error;
    error = nfs3_list_one_acl(inode, ACL_TYPE_DEFAULT,
    XATTR_NAME_POSIX_ACL_DEFAULT, data, size, &result);
    if (error)
    return error;
    return result;
    }
