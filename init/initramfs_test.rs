//! Automatically rewritten from C to Rust
//! Source: init/initramfs_test.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct initramfs_test_cpio {
    pub magic: *mut c_char,
    pub ino: c_uint,
    pub mode: c_uint,
    pub uid: c_uint,
    pub gid: c_uint,
    pub nlink: c_uint,
    pub mtime: c_uint,
    pub filesize: c_uint,
    pub devmajor: c_uint,
    pub devminor: c_uint,
    pub rdevmajor: c_uint,
    pub rdevminor: c_uint,
    pub namesize: c_uint,
    pub csum: c_uint,
    pub fname: *mut c_char,
    pub data: *mut c_char,
}

// regular newc header format

//
// Bogus newc header with "0x" prefixes on the uid, gid, and namesize values.
// parse_header()/simple_str[n]toul() accepted this, contrary to the initramfs
// specification. hex2bin() now fails.
//

    "%s%08x%08x0x%06x0X%06x%08x%08x%08x%08x%08x%08x%08x0x%06x%08x%s"
#[no_mangle]
pub unsafe extern "C" fn fill_cpio(cs: *mut initramfs_test_cpio, csz: size_t, inject_ox: bool, out: *mut c_char) -> size_t {
    let mut i = 0;
pub static mut off: usize = 0;
    while (i < csz) {
    let mut pos = &out[off];
    let mut c = &cs[i];
    let mut thislen = 0;
// +1 to account for nulterm
    thislen = sprintf(pos,
    inject_ox ? CPIO_HDR_OX_INJECT : CPIO_HDR_FMT,
    c.magic, c.ino, c.mode, c.uid, c.gid, c.nlink,
    c.mtime, c.filesize, c.devmajor, c.devminor,
    c.rdevmajor, c.rdevminor, c.namesize, c.csum,
    c.fname) + 1;
    pr_debug!("packing (%zu): %.*s\n", thislen, (int)thislen, pos);
    if (thislen != CPIO_HDRLEN + c.namesize) {
    pr_debug!("padded to: %u\n", CPIO_HDRLEN + c.namesize);
    }
    off += CPIO_HDRLEN + c.namesize;
    while (off & 3) {
    out[off++] = '\0';
    }
    memcpy(&out[off], c.data, c.filesize);
    off += c.filesize;
    while (off & 3) {
    out[off++] = '\0';
    }
    }
    return off;
    }
#[no_mangle]
unsafe extern "C" fn initramfs_test_extract(test: *mut kunit)  {
    let mut err = core::ptr::null_mut();
    let mut cpio_srcbuf = core::ptr::null_mut();
    let mut len = 0;
    struct timespec64 ts_before, ts_after;
pub static mut st: kstat = 0;
pub static mut initramfs_test_cpio: usize = 0;
// +3 to cater for any 4-byte end-alignment
    cpio_srcbuf = kzalloc(ARRAY_SIZE!(c) * (CPIO_HDRLEN + PATH_MAX + 3),
    GFP_KERNEL);
    len = fill_cpio(c, ARRAY_SIZE!(c), false, cpio_srcbuf);
    ktime_get_real_ts64(&ts_before);
    err = unpack_to_rootfs(cpio_srcbuf, len);
    ktime_get_real_ts64(&ts_after);
    if (err) {
    KUNIT_FAIL(test, "unpack failed %s", err);
// goto;
    }
    KUNIT_EXPECT_EQ(test, init_stat(c[0].fname, &st, 0), 0);
    KUNIT_EXPECT_TRUE(test, S_ISREG(st.mode));
    KUNIT_EXPECT_TRUE(test, uid_eq(st.uid, KUIDT_INIT(c[0].uid)));
    KUNIT_EXPECT_TRUE(test, gid_eq(st.gid, KGIDT_INIT(c[0].gid)));
    KUNIT_EXPECT_EQ(test, st.nlink, 1);
    if (IS_ENABLED!(CONFIG_INITRAMFS_PRESERVE_MTIME)) {
    KUNIT_EXPECT_EQ(test, st.mtime.tv_sec, c[0].mtime);
    } else {
    KUNIT_EXPECT_GE(test, st.mtime.tv_sec, ts_before.tv_sec);
    KUNIT_EXPECT_LE(test, st.mtime.tv_sec, ts_after.tv_sec);
    }
    KUNIT_EXPECT_EQ(test, st.blocks, c[0].filesize);
    KUNIT_EXPECT_EQ(test, init_stat(c[1].fname, &st, 0), 0);
    KUNIT_EXPECT_TRUE(test, S_ISDIR(st.mode));
    if (IS_ENABLED!(CONFIG_INITRAMFS_PRESERVE_MTIME)) {
    KUNIT_EXPECT_EQ(test, st.mtime.tv_sec, c[1].mtime);
    } else {
    KUNIT_EXPECT_GE(test, st.mtime.tv_sec, ts_before.tv_sec);
    KUNIT_EXPECT_LE(test, st.mtime.tv_sec, ts_after.tv_sec);
    }
    KUNIT_EXPECT_EQ(test, init_unlink(c[0].fname), 0);
    KUNIT_EXPECT_EQ(test, init_rmdir(c[1].fname), 0);
// label;
    kfree(cpio_srcbuf);
    }
//
// Don't terminate filename. Previously, the cpio filename field was passed
// directly to filp_open(collected, O_CREAT|..) without nulterm checks. See
// https://lore.kernel.org/linux-fsdevel/20241030035509.20194-2-ddiss@suse.de
//
#[no_mangle]
unsafe extern "C" fn initramfs_test_fname_overrun(test: *mut kunit)  {
    let mut err = core::ptr::null_mut();
    let mut cpio_srcbuf = core::ptr::null_mut();
    size_t len, suffix_off;
pub static mut initramfs_test_cpio: usize = 0;
//
// poison cpio source buffer, so we can detect overrun. source
// buffer is used by read_into() when hdr or fname
// are already available (e.g. no compression).
//
    cpio_srcbuf = kmalloc(CPIO_HDRLEN + PATH_MAX + 3, GFP_KERNEL);
    memset(cpio_srcbuf, 'B', CPIO_HDRLEN + PATH_MAX + 3);
// limit overrun to avoid crashes / filp_open() ENAMETOOLONG
    cpio_srcbuf[CPIO_HDRLEN + strlen(c[0].fname) + 20] = '\0';
    len = fill_cpio(c, ARRAY_SIZE!(c), false, cpio_srcbuf);
// overwrite trailing fname terminator and padding
    suffix_off = len - 1;
    while (cpio_srcbuf[suffix_off] == '\0') {
    cpio_srcbuf[suffix_off] = 'P';
    suffix_off -= 1;
    }
    err = unpack_to_rootfs(cpio_srcbuf, len);
    KUNIT_EXPECT_NOT_NULL(test, err);
    kfree(cpio_srcbuf);
    }
#[no_mangle]
unsafe extern "C" fn initramfs_test_data(test: *mut kunit)  {
    let mut err = core::ptr::null_mut();
    let mut cpio_srcbuf = core::ptr::null_mut();
    let mut len = 0;
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut initramfs_test_cpio: usize = 0;
// +6 for max name and data 4-byte padding
    cpio_srcbuf = kmalloc(CPIO_HDRLEN + c[0].namesize + c[0].filesize + 6,
    GFP_KERNEL);
    len = fill_cpio(c, ARRAY_SIZE!(c), false, cpio_srcbuf);
    err = unpack_to_rootfs(cpio_srcbuf, len);
    KUNIT_EXPECT_NULL(test, err);
    file = filp_open(c[0].fname, O_RDONLY, 0);
    if (IS_ERR(file)) {
    KUNIT_FAIL(test, "open failed");
// goto;
    }
// read back file contents into @cpio_srcbuf and confirm match
    len = kernel_read(file, cpio_srcbuf, c[0].filesize, core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, len, c[0].filesize);
    KUNIT_EXPECT_MEMEQ(test, cpio_srcbuf, c[0].data, len);
    fput(file);
    KUNIT_EXPECT_EQ(test, init_unlink(c[0].fname), 0);
// label;
    kfree(cpio_srcbuf);
    }
#[no_mangle]
unsafe extern "C" fn initramfs_test_csum(test: *mut kunit)  {
    let mut err = core::ptr::null_mut();
    let mut cpio_srcbuf = core::ptr::null_mut();
    let mut len = 0;
pub static mut initramfs_test_cpio: usize = 0;
    cpio_srcbuf = kmalloc(8192, GFP_KERNEL);
    len = fill_cpio(c, ARRAY_SIZE!(c), false, cpio_srcbuf);
    err = unpack_to_rootfs(cpio_srcbuf, len);
    KUNIT_EXPECT_NULL(test, err);
    KUNIT_EXPECT_EQ(test, init_unlink(c[0].fname), 0);
    KUNIT_EXPECT_EQ(test, init_unlink(c[1].fname), 0);
// mess up the csum and confirm that unpack fails
    c[0].csum -= 1;
    len = fill_cpio(c, ARRAY_SIZE!(c), false, cpio_srcbuf);
    err = unpack_to_rootfs(cpio_srcbuf, len);
    KUNIT_EXPECT_NOT_NULL(test, err);
//
// file (with content) is still retained in case of bad-csum abort.
// Perhaps we should change this.
//
    KUNIT_EXPECT_EQ(test, init_unlink(c[0].fname), 0);
    KUNIT_EXPECT_EQ(test, init_unlink(c[1].fname), -ENOENT);
    kfree(cpio_srcbuf);
    }
//
// hardlink hashtable may leak when the archive omits a trailer:
// https://lore.kernel.org/r/20241107002044.16477-10-ddiss@suse.de
//
#[no_mangle]
unsafe extern "C" fn initramfs_test_hardlink(test: *mut kunit)  {
    let mut err = core::ptr::null_mut();
    let mut cpio_srcbuf = core::ptr::null_mut();
    let mut len = 0;
pub static mut st0: kstat = 0;
pub static mut initramfs_test_cpio: usize = 0;
    cpio_srcbuf = kmalloc(8192, GFP_KERNEL);
    len = fill_cpio(c, ARRAY_SIZE!(c), false, cpio_srcbuf);
    err = unpack_to_rootfs(cpio_srcbuf, len);
    KUNIT_EXPECT_NULL(test, err);
    KUNIT_EXPECT_EQ(test, init_stat(c[0].fname, &st0, 0), 0);
    KUNIT_EXPECT_EQ(test, init_stat(c[1].fname, &st1, 0), 0);
    KUNIT_EXPECT_EQ(test, st0.ino, st1.ino);
    KUNIT_EXPECT_EQ(test, st0.nlink, 2);
    KUNIT_EXPECT_EQ(test, st1.nlink, 2);
    KUNIT_EXPECT_EQ(test, init_unlink(c[0].fname), 0);
    KUNIT_EXPECT_EQ(test, init_unlink(c[1].fname), 0);
    kfree(cpio_srcbuf);
    }
pub const INITRAMFS_TEST_MANY_LIMIT: c_int = 1000;

    + sizeof!(__stringify(INITRAMFS_TEST_MANY_LIMIT)))
#[no_mangle]
unsafe extern "C" fn initramfs_test_many(test: *mut kunit)  {
    let mut err = core::ptr::null_mut();
    let mut cpio_srcbuf = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    size_t len = INITRAMFS_TEST_MANY_LIMIT *
    (CPIO_HDRLEN + INITRAMFS_TEST_MANY_PATH_MAX + 3);
    char thispath[INITRAMFS_TEST_MANY_PATH_MAX];
    let mut i = 0;
    p = cpio_srcbuf = kmalloc(len, GFP_KERNEL);
    while (i < INITRAMFS_TEST_MANY_LIMIT) {
pub static mut initramfs_test_cpio: usize = 0;
    c.namesize = 1 + sprintf(thispath, "initramfs_test_many-%d", i);
    p += fill_cpio(&c, 1, false, p);
    }
    len = p - cpio_srcbuf;
    err = unpack_to_rootfs(cpio_srcbuf, len);
    KUNIT_EXPECT_NULL(test, err);
    while (i < INITRAMFS_TEST_MANY_LIMIT) {
    sprintf(thispath, "initramfs_test_many-%d", i);
    KUNIT_EXPECT_EQ(test, init_unlink(thispath), 0);
    }
    kfree(cpio_srcbuf);
    }
//
// An initramfs filename is namesize in length, including the zero-terminator.
// A filename can be zero-terminated prior to namesize, with the remainder used
// as padding. This can be useful for e.g. alignment of file data segments with
// a 4KB filesystem block, allowing for extent sharing (reflinks) between cpio
// source and destination. This hack works with both GNU cpio and initramfs, as
// long as PATH_MAX isn't exceeded.
//
#[no_mangle]
unsafe extern "C" fn initramfs_test_fname_pad(test: *mut kunit)  {
pub static mut err: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
pub static mut file: *mut c_void = core::ptr::null_mut();
    char fdata[] = "this file data is aligned at 4K in the archive";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_fname_pad {
    pub CPIO_HDRLEN]: char padded_fname[4096 -,
    pub sizeof!(fdata)]: char cpio_srcbuf[CPIO_HDRLEN + PATH_MAX + 3 +,
    pub test_fname_pad): *mut *mut } tbufs = kzalloc_obj(struct,
pub static mut initramfs_test_cpio: usize = 0;
    kunit_test_init_section_suites(&initramfs_test_suite);
    MODULE_DESCRIPTION("Initramfs KUnit test suite");
    MODULE_LICENSE("GPL v2");
}