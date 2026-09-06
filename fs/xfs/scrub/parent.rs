//! Automatically rewritten from C to Rust
//! Source: fs/xfs/scrub/parent.c
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
//
// Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//

// Set us up to scrub parents.
    int
    xchk_setup_parent(
    struct xfs_scrub	*sc)
    {
    int			error;
    if (xchk_could_repair(sc)) {
    error = xrep_setup_parent(sc);
    if (error)
    return error;
    }
    return xchk_setup_inode_contents(sc, 0);
    }
// Parent pointers
// Look for an entry in a parent pointing to this inode.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_parent_ctx {
    pub sc: *mut xfs_scrub,
    pub nlink: xfs_nlink_t,
}

// Look for a single entry in a directory pointing to an inode.
    STATIC int
    xchk_parent_actor(
    struct xfs_scrub	*sc,
    struct xfs_inode	*dp,
    xfs_dir2_dataptr_t	dapos,
    const struct xfs_name	*name,
    xfs_ino_t		ino,
    void			*priv)
    {
    struct xchk_parent_ctx	*spc = priv;
    let mut error: c_int = 0;
// Does this name make sense?
    if (!xfs_dir2_namecheck(name.name, name.len))
    error = -EFSCORRUPTED;
    if (!xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &error))
    return error;
    if (I_INO(sc.ip) == ino)
    spc.nlink++;
    if (xchk_should_terminate(spc.sc, &error))
    return error;
    return 0;
    }
//
// Try to lock a parent directory for checking dirents.  Returns the inode
// flags for the locks we now hold, or zero if we failed.
//
    STATIC unsigned int
    xchk_parent_ilock_dir(
    struct xfs_inode	*dp)
    {
    if (!xfs_ilock_nowait(dp, XFS_ILOCK_SHARED))
    return 0;
    if (!xfs_need_iread_extents(&dp.i_df))
    return XFS_ILOCK_SHARED;
    xfs_iunlock(dp, XFS_ILOCK_SHARED);
    if (!xfs_ilock_nowait(dp, XFS_ILOCK_EXCL))
    return 0;
    return XFS_ILOCK_EXCL;
    }
//
// Given the inode number of the alleged parent of the inode being scrubbed,
// try to validate that the parent has exactly one directory entry pointing
// back to the inode being scrubbed.  Returns -EAGAIN if we need to revalidate
// the dotdot entry.
//
    STATIC int
    xchk_parent_validate(
    struct xfs_scrub	*sc,
    xfs_ino_t		parent_ino)
    {
    struct xchk_parent_ctx	spc = {
    .sc		= sc,
    .nlink		= 0,
    };
    struct xfs_mount	*mp = sc.mp;
    let mut ino: xfs_ino_t = I_INO(sc.ip);
    struct xfs_inode	*dp = core::ptr::null_mut();
    xfs_nlink_t		expected_nlink;
    unsigned int		lock_mode;
    let mut error: c_int = 0;
// Is this the root dir?  Then '..' must point to itself.
    if (sc.ip == mp.m_rootip) {
    if (ino != mp.m_sb.sb_rootino || ino != parent_ino)
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
// Is this the metadata root dir?  Then '..' must point to itself.
    if (sc.ip == mp.m_metadirip) {
    if (ino != mp.m_sb.sb_metadirino || ino != parent_ino)
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
// '..' must not point to ourselves.
    if (ino == parent_ino) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
//
// If we're an unlinked directory, the parent /won't/ have a link
// to us.  Otherwise, it should have one link.
//
    expected_nlink = VFS_I(sc.ip).i_nlink == 0 ? 0 : 1;
//
// Grab the parent directory inode.  This must be released before we
// cancel the scrub transaction.
//
// If _iget returns -EINVAL or -ENOENT then the parent inode number is
// garbage and the directory is corrupt.  If the _iget returns
// -EFSCORRUPTED or -EFSBADCRC then the parent is corrupt which is a
// cross referencing error.  Any other error is an operational error.
//
    error = xchk_iget(sc, parent_ino, &dp);
    if (error == -EINVAL || error == -ENOENT) {
    error = -EFSCORRUPTED;
    xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &error);
    return error;
    }
    if (!xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &error))
    return error;
    if (dp == sc.ip || xrep_is_tempfile(dp) ||
    !S_ISDIR(VFS_I(dp).i_mode)) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    goto out_rele;
    }
    lock_mode = xchk_parent_ilock_dir(dp);
    if (!lock_mode) {
    xchk_iunlock(sc, XFS_ILOCK_EXCL);
    xchk_ilock(sc, XFS_ILOCK_EXCL);
    error = -EAGAIN;
    goto out_rele;
    }
//
// We cannot yet validate this parent pointer if the directory looks as
// though it has been zapped by the inode record repair code.
//
    if (xchk_dir_looks_zapped(dp)) {
    error = -EBUSY;
    xchk_set_incomplete(sc);
    goto out_unlock;
    }
// Metadata and regular inodes cannot cross trees.
    if (xfs_is_metadir_inode(dp) != xfs_is_metadir_inode(sc.ip)) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    goto out_unlock;
    }
// Look for a directory entry in the parent pointing to the child.
    error = xchk_dir_walk(sc, dp, xchk_parent_actor, &spc);
    if (!xchk_fblock_xref_process_error(sc, XFS_DATA_FORK, 0, &error))
    goto out_unlock;
//
// Ensure that the parent has as many links to the child as the child
// thinks it has to the parent.
//
    if (spc.nlink != expected_nlink)
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    out_unlock:
    xfs_iunlock(dp, lock_mode);
    out_rele:
    xchk_irele(sc, dp);
    return error;
    }
//
// Checking of Parent Pointers
// ===========================
//
// On filesystems with directory parent pointers, we check the referential
// integrity by visiting each parent pointer of a child file and checking that
// the directory referenced by the pointer actually has a dirent pointing
// forward to the child file.
//
// Deferred parent pointer entry that we saved for later.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_pptr {
// Cookie for retrieval of the pptr name.
    pub name_cookie: xfblob_cookie,
// Parent pointer record.
    pub pptr_rec: xfs_parent_rec,
// Length of the pptr name.
    pub namelen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_pptrs {
    pub sc: *mut xfs_scrub,
// How many parent pointers did we find at the end?
    pub pptrs_found: c_ulonglong,
// Parent of this directory.
    pub parent_ino: xfs_ino_t,
// Fixed-size array of xchk_pptr structures.
    pub pptr_entries: *mut xfarray,
// Blobs containing parent pointer names.
    pub pptr_names: *mut xfblob,
// Scratch buffer for scanning pptr xattrs
    pub pptr_args: xfs_da_args,
// If we've cycled the ILOCK, we must revalidate all deferred pptrs.
    pub need_revalidate: bool,
// Name buffer
    pub xname: xfs_name,
    pub namebuf: [c_char; MAXNAMELEN],
}

// Does this parent pointer match the dotdot entry?
    STATIC int
    xchk_parent_scan_dotdot(
    struct xfs_scrub		*sc,
    struct xfs_inode		*ip,
    unsigned int			attr_flags,
    const unsigned char		*name,
    unsigned int			namelen,
    const void			*value,
    unsigned int			valuelen,
    void				*priv)
    {
    struct xchk_pptrs		*pp = priv;
    xfs_ino_t			parent_ino;
    int				error;
    if (!(attr_flags & XFS_ATTR_PARENT))
    return 0;
    error = xfs_parent_from_attr(sc.mp, attr_flags, name, namelen, value,
    valuelen, &parent_ino, core::ptr::null_mut());
    if (error)
    return error;
    if (pp.parent_ino == parent_ino)
    return -ECANCELED;
    return 0;
    }
// Look up the dotdot entry so that we can check it as we walk the pptrs.
    STATIC int
    xchk_parent_pptr_and_dotdot(
    struct xchk_pptrs	*pp)
    {
    struct xfs_scrub	*sc = pp.sc;
    int			error;
// Look up '..'
    error = xchk_dir_lookup(sc, sc.ip, &xfs_name_dotdot, &pp.parent_ino);
    if (!xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &error))
    return error;
    if (!xfs_verify_dir_ino(sc.mp, pp.parent_ino)) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
// Is this the root dir?  Then '..' must point to itself.
    if (xchk_inode_is_dirtree_root(sc.ip)) {
    if (I_INO(sc.ip) != pp.parent_ino)
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
//
// If this is now an unlinked directory, the dotdot value is
// meaningless as long as it points to a valid inode.
//
    if (VFS_I(sc.ip).i_nlink == 0)
    return 0;
    if (pp.sc.sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT)
    return 0;
// Otherwise, walk the pptrs again, and check.
    error = xchk_xattr_walk(sc, sc.ip, xchk_parent_scan_dotdot, core::ptr::null_mut(), pp);
    if (error == -ECANCELED) {
// Found a parent pointer that matches dotdot.
    return 0;
    }
    if (!error || error == -EFSCORRUPTED) {
// Found a broken parent pointer or no match.
    xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return 0;
    }
    return error;
    }
//
// Try to lock a parent directory for checking dirents.  Returns the inode
// flags for the locks we now hold, or zero if we failed.
//
    STATIC unsigned int
    xchk_parent_lock_dir(
    struct xfs_scrub	*sc,
    struct xfs_inode	*dp)
    {
    if (!xfs_ilock_nowait(dp, XFS_IOLOCK_SHARED))
    return 0;
    if (!xfs_ilock_nowait(dp, XFS_ILOCK_SHARED)) {
    xfs_iunlock(dp, XFS_IOLOCK_SHARED);
    return 0;
    }
    if (!xfs_need_iread_extents(&dp.i_df))
    return XFS_IOLOCK_SHARED | XFS_ILOCK_SHARED;
    xfs_iunlock(dp, XFS_ILOCK_SHARED);
    if (!xfs_ilock_nowait(dp, XFS_ILOCK_EXCL)) {
    xfs_iunlock(dp, XFS_IOLOCK_SHARED);
    return 0;
    }
    return XFS_IOLOCK_SHARED | XFS_ILOCK_EXCL;
    }
// Check the forward link (dirent) associated with this parent pointer.
    STATIC int
    xchk_parent_dirent(
    struct xchk_pptrs	*pp,
    const struct xfs_name	*xname,
    struct xfs_inode	*dp)
    {
    struct xfs_scrub	*sc = pp.sc;
    xfs_ino_t		child_ino;
    int			error;
//
// Use the name attached to this parent pointer to look up the
// directory entry in the alleged parent.
//
    error = xchk_dir_lookup(sc, dp, xname, &child_ino);
    if (error == -ENOENT) {
    xchk_fblock_xref_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return 0;
    }
    if (!xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0, &error))
    return error;
// Does the inode number match?
    if (child_ino != I_INO(sc.ip)) {
    xchk_fblock_xref_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return 0;
    }
    return 0;
    }
// Try to grab a parent directory.
    STATIC int
    xchk_parent_iget(
    struct xchk_pptrs	*pp,
    const struct xfs_parent_rec	*pptr,
    struct xfs_inode	**dpp)
    {
    struct xfs_scrub	*sc = pp.sc;
    struct xfs_inode	*ip;
    let mut parent_ino: xfs_ino_t = be64_to_cpu(pptr.p_ino);
    int			error;
// Validate inode number.
    error = xfs_dir_ino_validate(sc.mp, parent_ino);
    if (error) {
    xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return -ECANCELED;
    }
    error = xchk_iget(sc, parent_ino, &ip);
    if (error == -EINVAL || error == -ENOENT) {
    xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return -ECANCELED;
    }
    if (!xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0, &error))
    return error;
// The parent must be a directory.
    if (!S_ISDIR(VFS_I(ip).i_mode)) {
    xchk_fblock_xref_set_corrupt(sc, XFS_ATTR_FORK, 0);
    goto out_rele;
    }
// Validate generation number.
    if (VFS_I(ip).i_generation != be32_to_cpu(pptr.p_gen)) {
    xchk_fblock_xref_set_corrupt(sc, XFS_ATTR_FORK, 0);
    goto out_rele;
    }
// dpp = ip;
    return 0;
    out_rele:
    xchk_irele(sc, ip);
    return 0;
    }
//
// Walk an xattr of a file.  If this xattr is a parent pointer, follow it up
// to a parent directory and check that the parent has a dirent pointing back
// to us.
//
    STATIC int
    xchk_parent_scan_attr(
    struct xfs_scrub	*sc,
    struct xfs_inode	*ip,
    unsigned int		attr_flags,
    const unsigned char	*name,
    unsigned int		namelen,
    const void		*value,
    unsigned int		valuelen,
    void			*priv)
    {
    struct xfs_name		xname = {
    .name		= name,
    .len		= namelen,
    };
    struct xchk_pptrs	*pp = priv;
    struct xfs_inode	*dp = core::ptr::null_mut();
    const struct xfs_parent_rec *pptr_rec = value;
    xfs_ino_t		parent_ino;
    unsigned int		lockmode;
    int			error;
    if (!(attr_flags & XFS_ATTR_PARENT))
    return 0;
    error = xfs_parent_from_attr(sc.mp, attr_flags, name, namelen, value,
    valuelen, &parent_ino, core::ptr::null_mut());
    if (error) {
    xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return -ECANCELED;
    }
// No self-referential parent pointers.
    if (parent_ino == I_INO(sc.ip)) {
    xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return -ECANCELED;
    }
    pp.pptrs_found++;
    error = xchk_parent_iget(pp, pptr_rec, &dp);
    if (error)
    return error;
    if (!dp)
    return 0;
// Try to lock the inode.
    lockmode = xchk_parent_lock_dir(sc, dp);
    if (!lockmode) {
    struct xchk_pptr	save_pp = {
    .pptr_rec	= *pptr_rec, /* struct copy */
    .namelen	= namelen,
    };
// Couldn't lock the inode, so save the pptr for later.
    trace_xchk_parent_defer(sc.ip, &xname, I_INO(dp));
    error = xfblob_storename(pp.pptr_names, &save_pp.name_cookie,
    &xname);
    if (!xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0,
    &error))
    goto out_rele;
    error = xfarray_append(pp.pptr_entries, &save_pp);
    if (!xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0,
    &error))
    goto out_rele;
    goto out_rele;
    }
    error = xchk_parent_dirent(pp, &xname, dp);
    if (error)
    goto out_unlock;
    out_unlock:
    xfs_iunlock(dp, lockmode);
    out_rele:
    xchk_irele(sc, dp);
    return error;
    }
//
// Revalidate a parent pointer that we collected in the past but couldn't check
// because of lock contention.  Returns 0 if the parent pointer is still valid,
// -ENOENT if it has gone away on us, or a negative errno.
//
    STATIC int
    xchk_parent_revalidate_pptr(
    struct xchk_pptrs		*pp,
    const struct xfs_name		*xname,
    struct xfs_parent_rec		*pptr)
    {
    struct xfs_scrub		*sc = pp.sc;
    int				error;
    error = xfs_parent_lookup(sc.tp, sc.ip, xname, pptr, &pp.pptr_args);
    if (error == -ENOATTR) {
// Parent pointer went away, nothing to revalidate.
    return -ENOENT;
    }
    return error;
    }
//
// Check a parent pointer the slow way, which means we cycle locks a bunch
// and put up with revalidation until we get it done.
//
    STATIC int
    xchk_parent_slow_pptr(
    struct xchk_pptrs	*pp,
    const struct xfs_name	*xname,
    struct xfs_parent_rec	*pptr)
    {
    struct xfs_scrub	*sc = pp.sc;
    struct xfs_inode	*dp = core::ptr::null_mut();
    unsigned int		lockmode;
    int			error;
// Check that the deferred parent pointer still exists.
    if (pp.need_revalidate) {
    error = xchk_parent_revalidate_pptr(pp, xname, pptr);
    if (error == -ENOENT)
    return 0;
    if (!xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0,
    &error))
    return error;
    }
    error = xchk_parent_iget(pp, pptr, &dp);
    if (error)
    return error;
    if (!dp)
    return 0;
//
// If we can grab both IOLOCK and ILOCK of the alleged parent, we
// can proceed with the validation.
//
    lockmode = xchk_parent_lock_dir(sc, dp);
    if (lockmode) {
    trace_xchk_parent_slowpath(sc.ip, xname, I_INO(dp));
    goto check_dirent;
    }
//
// We couldn't lock the parent dir.  Drop all the locks and try to
// get them again, one at a time.
//
    xchk_iunlock(sc, sc.ilock_flags);
    pp.need_revalidate = true;
    trace_xchk_parent_ultraslowpath(sc.ip, xname, I_INO(dp));
    error = xchk_dir_trylock_for_pptrs(sc, dp, &lockmode);
    if (error)
    goto out_rele;
// Revalidate the parent pointer now that we cycled locks.
    error = xchk_parent_revalidate_pptr(pp, xname, pptr);
    if (error == -ENOENT) {
    error = 0;
    goto out_unlock;
    }
    if (!xchk_fblock_xref_process_error(sc, XFS_ATTR_FORK, 0, &error))
    goto out_unlock;
    check_dirent:
    error = xchk_parent_dirent(pp, xname, dp);
    out_unlock:
    xfs_iunlock(dp, lockmode);
    out_rele:
    xchk_irele(sc, dp);
    return error;
    }
// Check all the parent pointers that we deferred the first time around.
    STATIC int
    xchk_parent_finish_slow_pptrs(
    struct xchk_pptrs	*pp)
    {
    xfarray_idx_t		array_cur;
    int			error;
    foreach_xfarray_idx(pp.pptr_entries, array_cur) {
    struct xchk_pptr	pptr;
    if (pp.sc.sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT)
    return 0;
    error = xfarray_load(pp.pptr_entries, array_cur, &pptr);
    if (error)
    return error;
    error = xfblob_loadname(pp.pptr_names, pptr.name_cookie,
    &pp.xname, pptr.namelen);
    if (error)
    return error;
    error = xchk_parent_slow_pptr(pp, &pp.xname, &pptr.pptr_rec);
    if (error)
    return error;
    }
// Empty out both xfiles now that we've checked everything.
    xfarray_truncate(pp.pptr_entries);
    xfblob_truncate(pp.pptr_names);
    return 0;
    }
// Count the number of parent pointers.
    STATIC int
    xchk_parent_count_pptr(
    struct xfs_scrub		*sc,
    struct xfs_inode		*ip,
    unsigned int			attr_flags,
    const unsigned char		*name,
    unsigned int			namelen,
    const void			*value,
    unsigned int			valuelen,
    void				*priv)
    {
    struct xchk_pptrs		*pp = priv;
    int				error;
    if (!(attr_flags & XFS_ATTR_PARENT))
    return 0;
    error = xfs_parent_from_attr(sc.mp, attr_flags, name, namelen, value,
    valuelen, core::ptr::null_mut(), core::ptr::null_mut());
    if (error)
    return error;
    pp.pptrs_found++;
    return 0;
    }
//
// Compare the number of parent pointers to the link count.  For
// non-directories these should be the same.  For unlinked directories the
// count should be zero; for linked directories, it should be nonzero.
//
    STATIC int
    xchk_parent_count_pptrs(
    struct xchk_pptrs	*pp)
    {
    struct xfs_scrub	*sc = pp.sc;
    int			error;
//
// If we cycled the ILOCK while cross-checking parent pointers with
// dirents, then we need to recalculate the number of parent pointers.
//
    if (pp.need_revalidate) {
    pp.pptrs_found = 0;
    error = xchk_xattr_walk(sc, sc.ip, xchk_parent_count_pptr,
    core::ptr::null_mut(), pp);
    if (error == -EFSCORRUPTED) {
// Found a bad parent pointer
    xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0);
    return 0;
    }
    if (error)
    return error;
    }
    if (S_ISDIR(VFS_I(sc.ip).i_mode)) {
    if (xchk_inode_is_dirtree_root(sc.ip))
    pp.pptrs_found++;
    if (VFS_I(sc.ip).i_nlink == 0 && pp.pptrs_found > 0)
    xchk_ip_set_corrupt(sc, sc.ip);
    else if (VFS_I(sc.ip).i_nlink > 0 &&
    pp.pptrs_found == 0)
    xchk_ip_set_corrupt(sc, sc.ip);
    } else {
//
// Starting with metadir, we allow checking of parent pointers
// of non-directory files that are children of the superblock.
// Pretend that we found a parent pointer attr.
//
    if (xfs_has_metadir(sc.mp) && xchk_inode_is_sb_rooted(sc.ip))
    pp.pptrs_found++;
    if (VFS_I(sc.ip).i_nlink != pp.pptrs_found)
    xchk_ip_set_corrupt(sc, sc.ip);
    }
    return 0;
    }
// Check parent pointers of a file.
    STATIC int
    xchk_parent_pptr(
    struct xfs_scrub	*sc)
    {
    struct xchk_pptrs	*pp;
    int			error;
    pp = kvzalloc_obj(struct xchk_pptrs, XCHK_GFP_FLAGS);
    if (!pp)
    return -ENOMEM;
    pp.sc = sc;
    pp.xname.name = pp.namebuf;
//
// Set up some staging memory for parent pointers that we can't check
// due to locking contention.
//
    error = xfarray_create("slow parent pointer entries", 0,
    sizeof(struct xchk_pptr), &pp.pptr_entries);
    if (error)
    goto out_pp;
    error = xfblob_create("slow parent pointer names", &pp.pptr_names);
    if (error)
    goto out_entries;
    error = xchk_xattr_walk(sc, sc.ip, xchk_parent_scan_attr, core::ptr::null_mut(), pp);
    if (error == -ECANCELED) {
    error = 0;
    goto out_names;
    }
    if (error)
    goto out_names;
    error = xchk_parent_finish_slow_pptrs(pp);
    if (error == -ETIMEDOUT) {
// Couldn't grab a lock, scrub was marked incomplete
    error = 0;
    goto out_names;
    }
    if (error)
    goto out_names;
    if (pp.sc.sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT)
    goto out_names;
//
// For subdirectories, make sure the dotdot entry references the same
// inode as the parent pointers.
//
// If we're scanning a /consistent/ directory, there should only be
// one parent pointer, and it should point to the same directory as
// the dotdot entry.
//
// However, a corrupt directory tree might feature a subdirectory with
// multiple parents.  The directory loop scanner is responsible for
// correcting that kind of problem, so for now we only validate that
// the dotdot entry matches /one/ of the parents.
//
    if (S_ISDIR(VFS_I(sc.ip).i_mode)) {
    error = xchk_parent_pptr_and_dotdot(pp);
    if (error)
    goto out_names;
    }
    if (pp.sc.sm.sm_flags & XFS_SCRUB_OFLAG_CORRUPT)
    goto out_names;
//
// Complain if the number of parent pointers doesn't match the link
// count.  This could be a sign of missing parent pointers (or an
// incorrect link count).
//
    error = xchk_parent_count_pptrs(pp);
    if (error)
    goto out_names;
    out_names:
    xfblob_destroy(pp.pptr_names);
    out_entries:
    xfarray_destroy(pp.pptr_entries);
    out_pp:
    kvfree(pp);
    return error;
    }
// Scrub a parent pointer.
    int
    xchk_parent(
    struct xfs_scrub	*sc)
    {
    struct xfs_mount	*mp = sc.mp;
    xfs_ino_t		parent_ino;
    let mut error: c_int = 0;
    if (xfs_has_parent(mp))
    return xchk_parent_pptr(sc);
//
// If we're a directory, check that the '..' link points up to
// a directory that has one entry pointing to us.
//
    if (!S_ISDIR(VFS_I(sc.ip).i_mode))
    return -ENOENT;
// We're not a special inode, are we?
    if (!xfs_verify_dir_ino(mp, I_INO(sc.ip))) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
    do {
    if (xchk_should_terminate(sc, &error))
    break;
// Look up '..'
    error = xchk_dir_lookup(sc, sc.ip, &xfs_name_dotdot,
    &parent_ino);
    if (!xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &error))
    return error;
    if (!xfs_verify_dir_ino(mp, parent_ino)) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
//
// Check that the dotdot entry points to a parent directory
// containing a dirent pointing to this subdirectory.
//
    error = xchk_parent_validate(sc, parent_ino);
    } while (error == -EAGAIN);
    if (error == -EBUSY) {
//
// We could not scan a directory, so we marked the check
// incomplete.  No further error return is necessary.
//
    return 0;
    }
    return error;
    }
//
// Decide if this file's extended attributes (and therefore its parent
// pointers) have been zapped to satisfy the inode and ifork verifiers.
// Checking and repairing should be postponed until the extended attribute
// structure is fixed.
//
    bool
    xchk_pptr_looks_zapped(
    struct xfs_inode	*ip)
    {
    struct inode		*inode = VFS_I(ip);
    ASSERT(xfs_has_parent(ip.i_mount));
//
// Temporary files that cannot be linked into the directory tree do not
// have attr forks because they cannot ever have parents.
//
    if (inode.i_nlink == 0 && !(inode_state_read_once(inode) & I_LINKABLE))
    return false;
//
// Directory tree roots do not have parents, so the expected outcome
// of a parent pointer scan is always the empty set.  It's safe to scan
// them even if the attr fork was zapped.
//
    if (xchk_inode_is_dirtree_root(ip))
    return false;
//
// Metadata inodes that are rooted in the superblock do not have any
// parents.  Hence the attr fork will not be initialized, but there are
// no parent pointers that might have been zapped.
//
    if (xchk_inode_is_sb_rooted(ip))
    return false;
//
// Linked and linkable non-rootdir files should always have an
// attribute fork because that is where parent pointers are
// stored.  If the fork is absent, something is amiss.
//
    if (!xfs_inode_has_attr_fork(ip))
    return true;
// Repair zapped this file's attr fork a short time ago
    if (xfs_ifork_zapped(ip, XFS_ATTR_FORK))
    return true;
//
// If the dinode repair found a bad attr fork, it will reset the fork
// to extents format with zero records and wait for the bmapbta
// scrubber to reconstruct the block mappings.  The extended attribute
// structure always contain some content when parent pointers are
// enabled, so this is a clear sign of a zapped attr fork.
//
    return ip.i_af.if_format == XFS_DINODE_FMT_EXTENTS &&
    ip.i_af.if_nextents == 0;
    }
