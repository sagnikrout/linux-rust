//! Automatically rewritten from C to Rust
//! Source: samples/vfs/mountinfo.c
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
// Use pidfds, nsfds, listmount() and statmount() mimic the
// contents of /proc/self/mountinfo.
//
// Macro flag: #define _GNU_SOURCE
// Macro flag: #define __SANE_USERSPACE_TYPES__

// max mounts per listmount call
pub const MAXMOUNTS: c_int = 1024;
// size of struct statmount (including trailing string buffer)
pub const STATMOUNT_BUFSIZE: c_int = 4096;
    static bool ext_format;

//
// There are no bindings in glibc for listmount() and statmount() (yet),
// make our own here.
//
    static int statmount(__u64 mnt_id, __u64 mnt_ns_id, __u64 mask,
    struct statmount *buf, size_t bufsize,
    unsigned int flags)
    {
    struct mnt_id_req req = {
    .size = MNT_ID_REQ_SIZE_VER0,
    .mnt_id = mnt_id,
    .param = mask,
    };
    if (mnt_ns_id) {
    req.size = MNT_ID_REQ_SIZE_VER1;
    req.mnt_ns_id = mnt_ns_id;
    }
    return syscall(__NR_statmount, &req, buf, bufsize, flags);
    }
    static ssize_t listmount(__u64 mnt_id, __u64 mnt_ns_id, __u64 last_mnt_id,
    __u64 list[], size_t num, unsigned int flags)
    {
    struct mnt_id_req req = {
    .size = MNT_ID_REQ_SIZE_VER0,
    .mnt_id = mnt_id,
    .param = last_mnt_id,
    };
    if (mnt_ns_id) {
    req.size = MNT_ID_REQ_SIZE_VER1;
    req.mnt_ns_id = mnt_ns_id;
    }
    return syscall(__NR_listmount, &req, list, num, flags);
    }
#[no_mangle]
unsafe extern "C" fn show_mnt_attrs(flags: __u64) {
    static void show_mnt_attrs(__u64 flags)
    {
    printf("%s", flags & MOUNT_ATTR_RDONLY ? "ro" : "rw");
    if (flags & MOUNT_ATTR_NOSUID)
    printf(",nosuid");
    if (flags & MOUNT_ATTR_NODEV)
    printf(",nodev");
    if (flags & MOUNT_ATTR_NOEXEC)
    printf(",noexec");
    switch (flags & MOUNT_ATTR__ATIME) {
    case MOUNT_ATTR_RELATIME:
    printf(",relatime");
    break;
    case MOUNT_ATTR_NOATIME:
    printf(",noatime");
    break;
    case MOUNT_ATTR_STRICTATIME:
// print nothing
    break;
    }
    if (flags & MOUNT_ATTR_NODIRATIME)
    printf(",nodiratime");
    if (flags & MOUNT_ATTR_NOSYMFOLLOW)
    printf(",nosymfollow");
    if (flags & MOUNT_ATTR_IDMAP)
    printf(",idmapped");
    }
#[no_mangle]
unsafe extern "C" fn show_propagation(sm: *mut statmount) {
    static void show_propagation(struct statmount *sm)
    {
    if (sm.mnt_propagation & MS_SHARED)
    printf(" shared:%llu", sm.mnt_peer_group);
    if (sm.mnt_propagation & MS_SLAVE) {
    printf(" master:%llu", sm.mnt_master);
    if (sm.propagate_from && sm.propagate_from != sm.mnt_master)
    printf(" propagate_from:%llu", sm.propagate_from);
    }
    if (sm.mnt_propagation & MS_UNBINDABLE)
    printf(" unbindable");
    }
#[no_mangle]
unsafe extern "C" fn show_sb_flags(flags: __u64) {
    static void show_sb_flags(__u64 flags)
    {
    printf("%s", flags & MS_RDONLY ? "ro" : "rw");
    if (flags & MS_SYNCHRONOUS)
    printf(",sync");
    if (flags & MS_DIRSYNC)
    printf(",dirsync");
    if (flags & MS_MANDLOCK)
    printf(",mand");
    if (flags & MS_LAZYTIME)
    printf(",lazytime");
    }
#[no_mangle]
unsafe extern "C" fn dump_mountinfo(mnt_id: __u64, mnt_ns_id: __u64) -> c_int {
    static int dump_mountinfo(__u64 mnt_id, __u64 mnt_ns_id)
    {
    int ret;
    struct statmount *buf = alloca(STATMOUNT_BUFSIZE);
    const __u64 mask = STATMOUNT_SB_BASIC | STATMOUNT_MNT_BASIC |
    STATMOUNT_PROPAGATE_FROM | STATMOUNT_FS_TYPE |
    STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT |
    STATMOUNT_MNT_OPTS | STATMOUNT_FS_SUBTYPE |
    STATMOUNT_SB_SOURCE;
    ret = statmount(mnt_id, mnt_ns_id, mask, buf, STATMOUNT_BUFSIZE, 0);
    if (ret < 0) {
    perror("statmount");
    return 1;
    }
    if (ext_format)
    printf("0x%llx 0x%llx 0x%llx ", mnt_ns_id, mnt_id, buf.mnt_parent_id);
    printf("%u %u %u:%u %s %s ", buf.mnt_id_old, buf.mnt_parent_id_old,
    buf.sb_dev_major, buf.sb_dev_minor,
    &buf.str[buf.mnt_root],
    &buf.str[buf.mnt_point]);
    show_mnt_attrs(buf.mnt_attr);
    show_propagation(buf);
    printf(" - %s", &buf.str[buf.fs_type]);
    if (buf.mask & STATMOUNT_FS_SUBTYPE)
    printf(".%s", &buf.str[buf.fs_subtype]);
    if (buf.mask & STATMOUNT_SB_SOURCE)
    printf(" %s ", &buf.str[buf.sb_source]);
    else
    printf(" :none ");
    show_sb_flags(buf.sb_flags);
    if (buf.mask & STATMOUNT_MNT_OPTS)
    printf(",%s", &buf.str[buf.mnt_opts]);
    printf("\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dump_mounts(mnt_ns_id: __u64) -> c_int {
    static int dump_mounts(__u64 mnt_ns_id)
    {
    __u64 mntid[MAXMOUNTS];
    let mut last_mnt_id: __u64 = 0;
    ssize_t count;
    int i;
//
// Get a list of all mntids in mnt_ns_id. If it returns MAXMOUNTS
// mounts, then go again until we get everything.
//
    do {
    count = listmount(LSMT_ROOT, mnt_ns_id, last_mnt_id, mntid, MAXMOUNTS, 0);
    if (count < 0 || count > MAXMOUNTS) {
    errno = count < 0 ? errno : count;
    perror("listmount");
    return 1;
    }
// Walk the returned mntids and print info about each
    for (i = 0; i < count; ++i) {
    let mut ret: c_int = dump_mountinfo(mntid[i], mnt_ns_id);
    if (ret != 0)
    return ret;
    }
// Set up last_mnt_id to pick up where we left off
    last_mnt_id = mntid[count - 1];
    } while (count == MAXMOUNTS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usage(prog: *const *const c_char) {
    static void usage(const char * const prog)
    {
    printf("Usage:\n");
    printf("%s [-e] [-p pid] [-r] [-h]\n", prog);
    printf("    -e: extended format\n");
    printf("    -h: print usage message\n");
    printf("    -p: get mount namespace from given pid\n");
    printf("    -r: recursively print all mounts in all child namespaces\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    int main(int argc, char * const *argv)
    {
    let mut mni: mnt_ns_info = { .size = MNT_NS_INFO_SIZE_VER0 };
    int pidfd, mntns, ret, opt;
    let mut pid: pid_t = getpid();
    let mut recursive: bool = false;
    while ((opt = getopt(argc, argv, "ehp:r")) != -1) {
    switch (opt) {
    case 'e':
    ext_format = true;
    break;
    case 'h':
    usage(argv[0]);
    return 0;
    case 'p':
    pid = atoi(optarg);
    break;
    case 'r':
    recursive = true;
    break;
    }
    }
// Get a pidfd for pid
    pidfd = syscall(__NR_pidfd_open, pid, 0);
    if (pidfd < 0) {
    perror("pidfd_open");
    return 1;
    }
// Get the mnt namespace for pidfd
    mntns = ioctl(pidfd, PIDFD_GET_MNT_NAMESPACE, core::ptr::null_mut());
    if (mntns < 0) {
    perror("PIDFD_GET_MNT_NAMESPACE");
    return 1;
    }
    close(pidfd);
// get info about mntns. In particular, the mnt_ns_id
    ret = ioctl(mntns, NS_MNT_GET_INFO, &mni);
    if (ret < 0) {
    perror("NS_MNT_GET_INFO");
    return 1;
    }
    do {
    int ret;
    ret = dump_mounts(mni.mnt_ns_id);
    if (ret)
    return ret;
    if (!recursive)
    break;
// get the next mntns (and overwrite the old mount ns info)
    ret = ioctl(mntns, NS_MNT_GET_NEXT, &mni);
    close(mntns);
    mntns = ret;
    } while (mntns >= 0);
    return 0;
    }
