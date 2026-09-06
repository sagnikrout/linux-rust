//! Automatically rewritten from C to Rust
//! Source: init/initramfs.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===





// SPDX-License-Identifier: GPL-2.0

    static __initdata bool csum_present;
    static __initdata u32 io_csum;
    static ssize_t __init xwrite(file *file, const unsigned char *p,
    size_t count, loff_t *pos)
    {
pub static mut out: isize = 0;
// sys_write only can write MAX_RW_COUNT aka 2G-4K bytes at most
    while (count) {
pub static mut rv: isize = 0;
    if (rv < 0) {
    if (rv == -EINTR || rv == -EAGAIN) {
    continue;
    }
    return out ? out : rv;
    } else if (rv == 0) {
    break;
    }
    if (csum_present) {
    let mut i = 0;
    while (i < rv) {
    io_csum += p[i];
    }
    }
    p += rv;
    out += rv;
    count -= rv;
    }
    return out;
    }
    static __initdata char *message;
#[no_mangle]
unsafe extern "C" fn error(x: *mut c_char)  {
    if (!message) {
    message = x;
    }
    }

    ({ show_mem(); panic(fmt, ##__VA_ARGS__); })
// link hash

    static __initdata struct hash {
    let mut ino = 0;
    let mut minor = 0;
    let mut major = 0;
    let mut mode;
pub static mut next: *mut c_void = core::ptr::null_mut();
    char name[N_ALIGN(PATH_MAX)];
    } *head[32];
    static __initdata bool hardlink_seen;
#[no_mangle]
pub unsafe extern "C" fn hash(major: c_int, minor: c_int, ino: c_int) -> c_int {
pub static mut tmp: c_ulong = 0;
    tmp += tmp >> 5;
    return tmp & 31;
    }
    static char __init *find_link(int major, int minor, int ino,
    umode_t mode, char *name)
    {
    let mut p = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
    for (p = head + hash(major, minor, ino); *p; p = &(*p).next) {
    if ((*p).ino != ino) {
    continue;
    }
    if ((*p).minor != minor) {
    continue;
    }
    if ((*p).major != major) {
    continue;
    }
    if (((*p).mode ^ mode) & S_IFMT) {
    continue;
    }
    return (*p).name;
    }
    q = kmalloc_obj(hash);
    if (!q) {
    panic_show_mem("can't allocate link hash entry");
    }
    q.major = major;
    q.minor = minor;
    q.ino = ino;
    q.mode = mode;
    strscpy(q.name, name);
    q.next = core::ptr::null_mut();
// p = q;
    hardlink_seen = true;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn free_hash()  {
    let mut p = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
    while (hardlink_seen && p < head + 32) {
    while (*p) {
    q = *p;
// p = q->next;
    kfree(q);
    }
    }
    hardlink_seen = false;
    }

#[no_mangle]
unsafe extern "C" fn do_utime(filename: *mut c_char, mtime: time64_t)  {
pub static mut timespec64: usize = 0;
    init_utimes(filename, t);
    }
#[no_mangle]
unsafe extern "C" fn do_utime_path(path: *const path, mtime: time64_t)  {
pub static mut timespec64: usize = 0;
    vfs_utimes(path, t);
    }
    static __initdata LIST_HEAD(dir_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dir_entry {
    pub list: list_head,
    pub mtime: time64_t,
    pub name: [c_char; 0],
}

#[no_mangle]
unsafe extern "C" fn dir_add(name: *const c_char, nlen: usize, mtime: time64_t)  {
pub static mut de: *mut c_void = core::ptr::null_mut();
    de = kmalloc_flex(*de, name, nlen);
    if (!de) {
    panic_show_mem("can't allocate dir_entry buffer");
    }
    INIT_LIST_HEAD(&de.list);
    strscpy(de.name, name, nlen);
    de.mtime = mtime;
    list_add(&de.list, &dir_list);
    }
#[no_mangle]
unsafe extern "C" fn dir_utime()  {
    let mut de = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(de, tmp, &dir_list, list) {
    list_del(&de.list);
    do_utime(de.name, de.mtime);
    kfree(de);
    }
    }

    static void __init do_utime(char *filename, time64_t mtime) {}
    static void __init do_utime_path(const struct path *path, time64_t mtime) {}
    static void __init dir_add(const char *name, size_t nlen, time64_t mtime) {}
    static void __init dir_utime(void) {}

    static __initdata time64_t mtime;
// cpio header parsing
    static __initdata unsigned long ino, major, minor, nlink;
    static __initdata umode_t mode;
    static __initdata unsigned long body_len, name_len;
    static __initdata uid_t uid;
    static __initdata gid_t gid;
    static __initdata unsigned rdev;
    static __initdata u32 hdr_csum;
#[no_mangle]
unsafe extern "C" fn parse_header(s: *mut c_char) -> c_int {
    __be32 header[13];
    let mut ret = 0;
    ret = hex2bin(header, s + 6, sizeof!(header));
    if (ret) {
    error("damaged header");
    return ret;
    }
    ino = be32_to_cpu(header[0]);
    mode = be32_to_cpu(header[1]);
    uid = be32_to_cpu(header[2]);
    gid = be32_to_cpu(header[3]);
    nlink = be32_to_cpu(header[4]);
    mtime = be32_to_cpu(header[5]); /* breaks in y2106 */
    body_len = be32_to_cpu(header[6]);
    major = be32_to_cpu(header[7]);
    minor = be32_to_cpu(header[8]);
    rdev = new_encode_dev(MKDEV!(be32_to_cpu(header[9]), be32_to_cpu(header[10])));
    name_len = be32_to_cpu(header[11]);
    hdr_csum = be32_to_cpu(header[12]);
    return 0;
    }
// Finite-state machine
    static __initdata enum state {
    Start,
    Collect,
    GotHeader,
    SkipIt,
    GotName,
    CopyFile,
    GotSymlink,
    Reset
    } state, next_state;
    static __initdata char *victim;
    static unsigned long byte_count __initdata;
    static __initdata loff_t this_header, next_header;
#[no_mangle]
pub unsafe extern "C" fn eat(n: unsigned)  {
    victim += n;
    this_header += n;
    byte_count -= n;
    }
    static __initdata char *collected;
    static long remains __initdata;
    static __initdata char *collect;
#[no_mangle]
unsafe extern "C" fn read_into(buf: *mut c_char, size: unsigned, next: state)  {
    if (byte_count >= size) {
    collected = victim;
    eat(size);
    state = next;
    } else {
    collect = collected = buf;
    remains = size;
    next_state = next;
    state = Collect;
    }
    }
    static __initdata char *header_buf, *symlink_buf, *name_buf;
#[no_mangle]
unsafe extern "C" fn do_start() -> c_int {
    read_into(header_buf, CPIO_HDRLEN, GotHeader);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_collect() -> c_int {
pub static mut n: c_ulong = 0;
    if (byte_count < n) {
    n = byte_count;
    }
    memcpy(collect, victim, n);
    eat(n);
    collect += n;
    if ((remains -= n) != 0) {
    return 1;
    }
    state = next_state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_header() -> c_int {
    if (!memcmp(collected, "070701", 6)) {
    csum_present = false;
    } else if (!memcmp(collected, "070702", 6)) {
    csum_present = true;
    } else {
    if (memcmp(collected, "070707", 6) == 0) {
    error("incorrect cpio method used: use -H newc option");
    }
    else {
    error("no cpio magic");
    }
    return 1;
    }
    if (parse_header(collected)) {
    return 1;
    }
    next_header = this_header + N_ALIGN(name_len) + body_len;
    next_header = (next_header + 3) & ~3;
    state = SkipIt;
    if (name_len <= 0 || name_len > PATH_MAX) {
    return 0;
    }
    if (S_ISLNK(mode)) {
    if (body_len > PATH_MAX) {
    return 0;
    }
    collect = collected = symlink_buf;
    remains = N_ALIGN(name_len) + body_len;
    next_state = GotSymlink;
    state = Collect;
    return 0;
    }
    if (S_ISREG(mode) || !body_len) {
    read_into(name_buf, N_ALIGN(name_len), GotName);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_skip() -> c_int {
    if (this_header + byte_count < next_header) {
    eat(byte_count);
    return 1;
    } else {
    eat(next_header - this_header);
    state = next_state;
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_reset() -> c_int {
    while (byte_count && *victim == '\0') {
    eat(1);
    }
    if (byte_count && (this_header & 3)) {
    error("broken padding");
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn clean_path(path: *mut c_char, fmode: umode_t)  {
pub static mut st: usize = 0;
    if (!init_stat(path, &st, AT_SYMLINK_NOFOLLOW) &&
    (st.mode ^ fmode) & S_IFMT) {
    if (S_ISDIR(st.mode)) {
    init_rmdir(path);
    }
    else {
    init_unlink(path);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn maybe_link() -> c_int {
    if (nlink >= 2) {
    let mut old = find_link(major, minor, ino, mode, collected);
    if (old) {
    clean_path(collected, 0);
    return (init_link(old, collected) < 0) ? -1 : 1;
    }
    }
    return 0;
    }
    static __initdata struct file *wfile;
    static __initdata loff_t wfile_pos;
#[no_mangle]
unsafe extern "C" fn do_name() -> c_int {
    state = SkipIt;
    next_state = Reset;
// name_len > 0 && name_len <= PATH_MAX checked in do_header
    if (collected[name_len - 1] != '\0') {
    pr_err!("initramfs name without nulterm: %.*s\n",
    (int)name_len, collected);
    error("malformed archive");
    return 1;
    }
    if (strcmp(collected, "TRAILER!!!") == 0) {
    free_hash();
    return 0;
    }
    clean_path(collected, mode);
    if (S_ISREG(mode)) {
pub static mut ml: c_int = 0;
    if (ml >= 0) {
pub static mut openflags: c_int = 0;
    if (ml != 1) {
    openflags |= O_TRUNC;
    }
    wfile = filp_open(collected, openflags, mode);
    if (IS_ERR(wfile)) {
    return 0;
    }
    wfile_pos = 0;
    io_csum = 0;
    vfs_fchown(wfile, uid, gid);
    vfs_fchmod(wfile, mode);
    if (body_len) {
    vfs_truncate(&wfile.f_path, body_len);
    }
    state = CopyFile;
    }
    } else if (S_ISDIR(mode)) {
    init_mkdir(collected, mode);
    init_chown(collected, uid, gid, 0);
    init_chmod(collected, mode);
    dir_add(collected, name_len, mtime);
    } else if (S_ISBLK(mode) || S_ISCHR(mode) ||
    S_ISFIFO(mode) || S_ISSOCK(mode)) {
    if (maybe_link() == 0) {
    init_mknod(collected, mode, rdev);
    init_chown(collected, uid, gid, 0);
    init_chmod(collected, mode);
    do_utime(collected, mtime);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_copy() -> c_int {
    if (byte_count >= body_len) {
    if (xwrite(wfile, victim, body_len, &wfile_pos) != body_len) {
    error("write error");
    }
    do_utime_path(&wfile.f_path, mtime);
    fput(wfile);
    if (csum_present && io_csum != hdr_csum) {
    error("bad data checksum");
    }
    eat(body_len);
    state = SkipIt;
    return 0;
    } else {
    if (xwrite(wfile, victim, byte_count, &wfile_pos) != byte_count) {
    error("write error");
    }
    body_len -= byte_count;
    eat(byte_count);
    return 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_symlink() -> c_int {
    if (collected[name_len - 1] != '\0') {
    pr_err!("initramfs symlink without nulterm: %.*s\n",
    (int)name_len, collected);
    error("malformed archive");
    return 1;
    }
    collected[N_ALIGN(name_len) + body_len] = '\0';
    clean_path(collected, 0);
    init_symlink(collected + N_ALIGN(name_len), collected);
    init_chown(collected, uid, gid, AT_SYMLINK_NOFOLLOW);
    do_utime(collected, mtime);
    state = SkipIt;
    next_state = Reset;
    return 0;
    }
    static __initdata int (*actions[])(void) = {
    [Start]		= do_start,
    [Collect]	= do_collect,
    [GotHeader]	= do_header,
    [SkipIt]	= do_skip,
    [GotName]	= do_name,
    [CopyFile]	= do_copy,
    [GotSymlink]	= do_symlink,
    [Reset]		= do_reset,
    };
#[no_mangle]
unsafe extern "C" fn write_buffer(buf: *mut c_char, len: c_ulong) -> long __init {
    byte_count = len;
    victim = buf;
    while (!actions[state]()) {
    ;
    }
    return len - byte_count;
    }
#[no_mangle]
unsafe extern "C" fn flush_buffer(bufv: *mut c_void, len: c_ulong) -> long __init {
    let mut buf = bufv;
    let mut written = 0;
pub static mut origLen: c_long = 0;
    if (message) {
    return -1;
    }
    while ((written = write_buffer(buf, len)) < len && !message) {
pub static mut c: c_char = 0;
    if (c == '0') {
    buf += written;
    len -= written;
    state = Start;
    } else if (c == 0) {
    buf += written;
    len -= written;
    state = Reset;
    } else {
    error("junk within compressed archive");
    }
    }
    return origLen;
    }
    static unsigned long my_inptr __initdata; /* index of next byte to be processed in inbuf */

//
// unpack_to_rootfs - decompress and extract an initramfs archive
// @buf: input initramfs archive to extract
// @len: length of initramfs data to process
//
// Returns: NULL for success or an error message string
//
// This symbol shouldn't be used externally. It's available for unit tests.
//
#[no_mangle]
pub unsafe extern "C" fn unpack_to_rootfs(buf: *mut c_char, len: c_ulong) -> *mut char  __init {
    let mut written = 0;
    let mut decompress;
pub static mut compress_name: *mut c_void = core::ptr::null_mut();
    struct {
    char header[CPIO_HDRLEN];
    char symlink[PATH_MAX + N_ALIGN(PATH_MAX) + 1];
    char name[N_ALIGN(PATH_MAX)];
    } *bufs = kmalloc_obj(*bufs);
    if (!bufs) {
    panic_show_mem("can't allocate buffers");
    }
    header_buf = bufs.header;
    symlink_buf = bufs.symlink;
    name_buf = bufs.name;
    state = Start;
    this_header = 0;
    message = core::ptr::null_mut();
    while (!message && len) {
pub static mut saved_offset: loff_t = 0;
    if (*buf == '0' && !(this_header & 3)) {
    state = Start;
    written = write_buffer(buf, len);
    buf += written;
    len -= written;
    continue;
    }
    if (!*buf) {
    buf += 1;
    len -= 1;
    this_header += 1;
    continue;
    }
    this_header = 0;
    decompress = decompress_method(buf, len, &compress_name);
    pr_debug!("Detected %s compressed data\n", compress_name);
    if (decompress) {
    let mut res = decompress(buf, len, core::ptr::null_mut(), flush_buffer, core::ptr::null_mut(),
    &my_inptr, error);
    if (res) {
    error("decompressor failed");
    }
    } else if (compress_name) {
    pr_err!("compression method %s not configured\n",
    compress_name);
    error("decompressor failed");
    } else {
    error("invalid magic at start of compressed archive");
    }
    if (state != Reset) {
    error("junk at the end of compressed archive");
    }
    this_header = saved_offset + my_inptr;
    buf += my_inptr;
    len -= my_inptr;
    }
    dir_utime();
// free any hardlink state collected without optional TRAILER!!!
    free_hash();
    kfree(bufs);
    return message;
    }
    static int __initdata do_retain_initrd;
#[no_mangle]
unsafe extern "C" fn retain_initrd_param(str: *mut c_char) -> c_int {
    if (*str) {
    return 0;
    }
    do_retain_initrd = 1;
    return 1;
    }
    __setup!("retain_initrd", retain_initrd_param);

#[no_mangle]
unsafe extern "C" fn keepinitrd_setup(__unused: *mut c_char) -> c_int {
    do_retain_initrd = 1;
    return 1;
    }
    __setup!("keepinitrd", keepinitrd_setup);

pub static mut initramfs_async: bool __initdata = true;
#[no_mangle]
unsafe extern "C" fn initramfs_async_setup(str: *mut c_char) -> c_int {
    return kstrtobool(str, &initramfs_async) == 0;
    }
    __setup!("initramfs_async=", initramfs_async_setup);
    extern char __initramfs_start[];
extern "C" { pub static mut __initramfs_size: usize; }

    static BIN_ATTR(initrd, 0440, sysfs_bin_attr_simple_read, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn reserve_initrd_mem()  {
    let mut start;
    let mut size = 0;
// Ignore the virtual address computed during device tree parsing
    initrd_start = initrd_end = 0;
    if (!phys_initrd_size) {
    return;
    }
//
// Round the memory region to page boundaries as per free_initrd_mem()
// This allows us to detect whether the pages overlapping the initrd
// are in use, but more importantly, reserves the entire set of pages
// as we don't want these pages allocated for other purposes.
//
    start = round_down(phys_initrd_start, PAGE_SIZE);
    size = phys_initrd_size + (phys_initrd_start - start);
    size = round_up(size, PAGE_SIZE);
    if (!memblock_is_region_memory(start, size)) {
    pr_err!("INITRD: 0x%08llx+0x%08lx is not a memory region",
    (u64)start, size);
// goto;
    }
    if (memblock_is_region_reserved(start, size)) {
    pr_err!("INITRD: 0x%08llx+0x%08lx overlaps in-use memory region\n",
    (u64)start, size);
// goto;
    }
    memblock_reserve(start, size);
// Now convert initrd to virtual addresses
    initrd_start = (unsigned long)__va(phys_initrd_start);
    initrd_end = initrd_start + phys_initrd_size;
    initrd_below_start_ok = 1;
    return;
// label;
    pr_cont(" - disabling initrd\n");
    initrd_start = 0;
    initrd_end = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_initrd_mem(start: c_ulong, end: c_ulong) -> void __weak __init {
    free_reserved_area(start, end, POISON_FREE_INITMEM,
    "initrd");
    }

#[no_mangle]
unsafe extern "C" fn kexec_free_initrd() -> bool __init {
pub static mut crashk_start: c_ulong = 0;
pub static mut crashk_end: c_ulong = 0;
//
// If the initrd region is overlapped with crashkernel reserved region,
// free only memory that is not part of crashkernel region.
//
    if (initrd_start >= crashk_end || initrd_end <= crashk_start) {
    return false;
    }
//
// Initialize initrd memory region since the kexec boot does not do.
//
    memset(initrd_start, 0, initrd_end - initrd_start);
    if (initrd_start < crashk_start) {
    free_initrd_mem(initrd_start, crashk_start);
    }
    if (initrd_end > crashk_end) {
    free_initrd_mem(crashk_end, initrd_end);
    }
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn kexec_free_initrd() -> bool {
    return false;
    }

#[no_mangle]
unsafe extern "C" fn populate_initrd_image(err: *mut c_char)  {
    let mut written = 0;
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut pos: loff_t = 0;
    printk!("rootfs image is not initramfs (%s); looks like an initrd\n",
    err);
    file = filp_open("/initrd.image", O_WRONLY|O_CREAT|O_LARGEFILE, 0700);
    if (IS_ERR(file)) {
    return;
    }
    written = xwrite(file, initrd_start, initrd_end - initrd_start,
    &pos);
    if (written != initrd_end - initrd_start) {
    pr_err!("/initrd.image: incomplete write (%zd != %ld)\n",
    written, initrd_end - initrd_start);
    }
    fput(file);
    }

#[no_mangle]
unsafe extern "C" fn unpack_initramfs(cookie: async_cookie_t)  {
// Load the built in initramfs
    let mut err = unpack_to_rootfs(__initramfs_start, __initramfs_size);
    if (err) {
    panic_show_mem("%s", err); /* Failed to decompress INTERNAL initramfs */
    }
    if (!initrd_start || IS_ENABLED!(CONFIG_INITRAMFS_FORCE)) {
    return;
    }
    if (IS_ENABLED!(CONFIG_BLK_DEV_RAM)) {
    printk!("Trying to unpack rootfs image as initramfs...\n");
    }
    else {
    printk!("Unpacking initramfs...\n");
    }
    err = unpack_to_rootfs(initrd_start, initrd_end - initrd_start);
    if (err) {

    populate_initrd_image(err);

    printk!("Initramfs unpacking failed: %s\n", err);

    }
    }
#[no_mangle]
unsafe extern "C" fn do_populate_rootfs(unused: *mut c_void, cookie: async_cookie_t)  {
    scoped_with_init_fs() {
    unpack_initramfs(cookie);
    security_initramfs_populated();
    }
//
// If the initrd region is overlapped with crashkernel reserved region,
// free only memory that is not part of crashkernel region.
//
    if (!do_retain_initrd && initrd_start && !kexec_free_initrd()) {
    free_initrd_mem(initrd_start, initrd_end);
    } else if (do_retain_initrd && initrd_start) {
    bin_attr_initrd.size = initrd_end - initrd_start;
    bin_attr_initrd.private = initrd_start;
    if (sysfs_create_bin_file(firmware_kobj, &bin_attr_initrd)) {
    pr_err!("Failed to create initrd sysfs file");
    }
    }
    initrd_start = 0;
    initrd_end = 0;
    init_flush_fput();
    }
    static ASYNC_DOMAIN_EXCLUSIVE(initramfs_domain);
    static async_cookie_t initramfs_cookie;
#[no_mangle]
pub unsafe extern "C" fn wait_for_initramfs() {
    if (!initramfs_cookie) {
//
// Something before rootfs_initcall wants to access
// the filesystem/initramfs. Probably a bug. Make a
// note, avoid deadlocking the machine, and let the
// caller's access fail as it used to.
//
    pr_warn_once("wait_for_initramfs() called before rootfs_initcalls\n");
    return;
    }
    async_synchronize_cookie_domain(initramfs_cookie + 1, &initramfs_domain);
    }
    EXPORT_SYMBOL_GPL(wait_for_initramfs);
#[no_mangle]
unsafe extern "C" fn populate_rootfs() -> c_int {
    initramfs_cookie = async_schedule_domain(do_populate_rootfs, core::ptr::null_mut(),
    &initramfs_domain);
    usermodehelper_enable();
    if (!initramfs_async) {
    wait_for_initramfs();
    }
    return 0;
    }
    rootfs_initcall!(populate_rootfs);