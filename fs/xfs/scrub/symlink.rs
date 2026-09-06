//! Automatically rewritten from C to Rust
//! Source: fs/xfs/scrub/symlink.c
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

// Set us up to scrub a symbolic link.
    int
    xchk_setup_symlink(
    struct xfs_scrub	*sc)
    {
    let mut resblks: c_uint = 0;
    int			error;
// Allocate the buffer without the inode lock held.
    sc.buf = kvzalloc(XFS_SYMLINK_MAXLEN + 1, XCHK_GFP_FLAGS);
    if (!sc.buf)
    return -ENOMEM;
    if (xchk_could_repair(sc)) {
    error = xrep_setup_symlink(sc, &resblks);
    if (error)
    return error;
    }
    return xchk_setup_inode_contents(sc, resblks);
    }
// Symbolic links.
    int
    xchk_symlink(
    struct xfs_scrub	*sc)
    {
    struct xfs_inode	*ip = sc.ip;
    struct xfs_ifork	*ifp;
    loff_t			len;
    let mut error: c_int = 0;
    if (!S_ISLNK(VFS_I(ip).i_mode))
    return -ENOENT;
    if (xchk_file_looks_zapped(sc, XFS_SICK_INO_SYMLINK_ZAPPED)) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
    ifp = xfs_ifork_ptr(ip, XFS_DATA_FORK);
    len = ip.i_disk_size;
// Plausible size?
    if (len > XFS_SYMLINK_MAXLEN || len <= 0) {
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
// Inline symlink?
    if (ifp.if_format == XFS_DINODE_FMT_LOCAL) {
    if (len > xfs_inode_data_fork_size(ip) ||
    len > strnlen(ifp.if_data, xfs_inode_data_fork_size(ip)))
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    return 0;
    }
// Remote symlink; must read the contents.
    error = xfs_symlink_remote_read(sc.ip, sc.buf);
    if (!xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &error))
    return error;
    if (strnlen(sc.buf, XFS_SYMLINK_MAXLEN) < len)
    xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
// If a remote symlink is clean, it is clearly not zapped.
    xchk_mark_healthy_if_clean(sc, XFS_SICK_INO_SYMLINK_ZAPPED);
    return 0;
    }
