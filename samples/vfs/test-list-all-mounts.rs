//! Automatically rewritten from C to Rust
//! Source: samples/vfs/test-list-all-mounts.c
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
// Copyright (c) 2024 Christian Brauner <brauner@kernel.org>
// Macro flag: #define _GNU_SOURCE

    static int __statmount(__u64 mnt_id, __u64 mnt_ns_id, __u64 mask,
    struct statmount *stmnt, size_t bufsize,
    unsigned int flags)
    {
    struct mnt_id_req req = {
    .size		= MNT_ID_REQ_SIZE_VER1,
    .mnt_id		= mnt_id,
    .param		= mask,
    .mnt_ns_id	= mnt_ns_id,
    };
    return syscall(__NR_statmount, &req, stmnt, bufsize, flags);
    }
    static struct statmount *sys_statmount(__u64 mnt_id, __u64 mnt_ns_id,
    __u64 mask, unsigned int flags)
    {
    let mut bufsize: usize = 1 << 15;
    struct statmount *stmnt = core::ptr::null_mut(), *tmp = core::ptr::null_mut();
    int ret;
    for (;;) {
    tmp = realloc(stmnt, bufsize);
    if (!tmp)
    goto out;
    stmnt = tmp;
    ret = __statmount(mnt_id, mnt_ns_id, mask, stmnt, bufsize, flags);
    if (!ret)
    return stmnt;
    if (errno != EOVERFLOW)
    goto out;
    bufsize <<= 1;
    if (bufsize >= UINT_MAX / 2)
    goto out;
    }
    out:
    free(stmnt);
    return core::ptr::null_mut();
    }
    static ssize_t sys_listmount(__u64 mnt_id, __u64 last_mnt_id, __u64 mnt_ns_id,
    __u64 list[], size_t num, unsigned int flags)
    {
    struct mnt_id_req req = {
    .size		= MNT_ID_REQ_SIZE_VER1,
    .mnt_id		= mnt_id,
    .param		= last_mnt_id,
    .mnt_ns_id	= mnt_ns_id,
    };
    return syscall(__NR_listmount, &req, list, num, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
pub const LISTMNT_BUFFER: c_int = 10;
    __u64 list[LISTMNT_BUFFER], last_mnt_id = 0;
    int ret, pidfd, fd_mntns;
    let mut info: mnt_ns_info = {};
    pidfd = sys_pidfd_open(getpid(), 0);
    if (pidfd < 0)
    die_errno("pidfd_open failed");
    fd_mntns = ioctl(pidfd, PIDFD_GET_MNT_NAMESPACE, 0);
    if (fd_mntns < 0)
    die_errno("ioctl(PIDFD_GET_MNT_NAMESPACE) failed");
    ret = ioctl(fd_mntns, NS_MNT_GET_INFO, &info);
    if (ret < 0)
    die_errno("ioctl(NS_GET_MNTNS_ID) failed");
    printf("Listing %u mounts for mount namespace %" PRIu64 "\n",
    info.nr_mounts, (uint64_t)info.mnt_ns_id);
    for (;;) {
    ssize_t nr_mounts;
    next:
    nr_mounts = sys_listmount(LSMT_ROOT, last_mnt_id,
    info.mnt_ns_id, list, LISTMNT_BUFFER,
    0);
    if (nr_mounts <= 0) {
    int fd_mntns_next;
    printf("Finished listing %u mounts for mount namespace %" PRIu64 "\n\n",
    info.nr_mounts, (uint64_t)info.mnt_ns_id);
    fd_mntns_next = ioctl(fd_mntns, NS_MNT_GET_NEXT, &info);
    if (fd_mntns_next < 0) {
    if (errno == ENOENT) {
    printf("Finished listing all mount namespaces\n");
    exit(0);
    }
    die_errno("ioctl(NS_MNT_GET_NEXT) failed");
    }
    close(fd_mntns);
    fd_mntns = fd_mntns_next;
    last_mnt_id = 0;
    printf("Listing %u mounts for mount namespace %" PRIu64 "\n",
    info.nr_mounts, (uint64_t)info.mnt_ns_id);
    goto next;
    }
    for (size_t cur = 0; cur < nr_mounts; cur++) {
    struct statmount *stmnt;
    last_mnt_id = list[cur];
    stmnt = sys_statmount(last_mnt_id, info.mnt_ns_id,
    STATMOUNT_SB_BASIC |
    STATMOUNT_MNT_BASIC |
    STATMOUNT_MNT_ROOT |
    STATMOUNT_MNT_POINT |
    STATMOUNT_MNT_NS_ID |
    STATMOUNT_MNT_OPTS |
    STATMOUNT_FS_TYPE |
    STATMOUNT_MNT_UIDMAP |
    STATMOUNT_MNT_GIDMAP, 0);
    if (!stmnt) {
    printf("Failed to statmount(%" PRIu64 ") in mount namespace(%" PRIu64 ")\n",
    (uint64_t)last_mnt_id, (uint64_t)info.mnt_ns_id);
    continue;
    }
    printf("mnt_id:\t\t%" PRIu64 "\nmnt_parent_id:\t%" PRIu64 "\nfs_type:\t%s\nmnt_root:\t%s\nmnt_point:\t%s\nmnt_opts:\t%s\n",
    (uint64_t)stmnt.mnt_id,
    (uint64_t)stmnt.mnt_parent_id,
    (stmnt.mask & STATMOUNT_FS_TYPE)   ? stmnt.str + stmnt.fs_type   : "",
    (stmnt.mask & STATMOUNT_MNT_ROOT)  ? stmnt.str + stmnt.mnt_root  : "",
    (stmnt.mask & STATMOUNT_MNT_POINT) ? stmnt.str + stmnt.mnt_point : "",
    (stmnt.mask & STATMOUNT_MNT_OPTS)  ? stmnt.str + stmnt.mnt_opts  : "");
    if (stmnt.mask & STATMOUNT_MNT_UIDMAP) {
    const char *idmap = stmnt.str + stmnt.mnt_uidmap;
    for (size_t idx = 0; idx < stmnt.mnt_uidmap_num; idx++) {
    printf("mnt_uidmap[%zu]:\t%s\n", idx, idmap);
    idmap += strlen(idmap) + 1;
    }
    }
    if (stmnt.mask & STATMOUNT_MNT_GIDMAP) {
    const char *idmap = stmnt.str + stmnt.mnt_gidmap;
    for (size_t idx = 0; idx < stmnt.mnt_gidmap_num; idx++) {
    printf("mnt_gidmap[%zu]:\t%s\n", idx, idmap);
    idmap += strlen(idmap) + 1;
    }
    }
    printf("\n");
    free(stmnt);
    }
    }
    exit(0);
    }
