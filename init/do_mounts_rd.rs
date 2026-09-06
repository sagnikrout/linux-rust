//! Automatically rewritten from C to Rust
//! Source: init/do_mounts_rd.c
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

    static struct file *in_file, *out_file;
    static loff_t in_pos, out_pos;
    int __initdata rd_image_start;		/* starting block # of image */
#[no_mangle]
unsafe extern "C" fn ramdisk_start_setup(str: *mut c_char) -> c_int {
    pr_warn!("ramdisk_start= option is deprecated and will be removed soon\n");
    return kstrtoint(str, 0, &rd_image_start) == 0;
    }
    __setup!("ramdisk_start=", ramdisk_start_setup);
    static int __init crd_load(decompress_fn deco);
//
// This routine tries to find a RAM disk image to load, and returns the
// number of blocks to read for a non-compressed image, 0 if the image
// is a compressed image, and -1 if an image with the right magic
// numbers could not be found.
//
// We currently check for the following magic numbers:
// minix
// ext2
// romfs
// cramfs
// squashfs
// gzip
// bzip2
// lzma
// xz
// lzo
// lz4
//
    static int __init
    identify_ramdisk_image(file *file, loff_t pos,
    decompress_fn *decompressor)
    {
pub static mut size: c_int = 512;
pub static mut minixsb: *mut c_void = core::ptr::null_mut();
pub static mut romfsb: *mut c_void = core::ptr::null_mut();
pub static mut cramfsb: *mut c_void = core::ptr::null_mut();
pub static mut squashfsb: *mut c_void = core::ptr::null_mut();
pub static mut nblocks: c_int = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut compress_name: *mut c_void = core::ptr::null_mut();
    let mut n = 0;
pub static mut start_block: c_int = 0;
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    minixsb =  buf;
    romfsb =  buf;
    cramfsb =  buf;
    squashfsb =  buf;
    memset(buf, 0xe5, size);
//
// Read block 0 to test for compressed kernel
//
    pos = start_block * BLOCK_SIZE;
    kernel_read(file, buf, size, &pos);
// decompressor = decompress_method(buf, size, &compress_name);
    if (compress_name) {
    printk!("RAMDISK: %s image found at block %d\n",
    compress_name, start_block);
    if (!*decompressor) {
    printk!("RAMDISK: %s decompressor not configured!\n",
    compress_name);
    }
    nblocks = 0;
// goto;
    }
// romfs is at block zero too
    if (romfsb.word0 == ROMSB_WORD0 &&
    romfsb.word1 == ROMSB_WORD1) {
    printk!("RAMDISK: romfs filesystem found at block %d\n",
    start_block);
    nblocks = (ntohl(romfsb.size)+BLOCK_SIZE-1)>>BLOCK_SIZE_BITS;
// goto;
    }
    if (cramfsb.magic == CRAMFS_MAGIC) {
    printk!("RAMDISK: cramfs filesystem found at block %d\n",
    start_block);
    nblocks = (cramfsb.size + BLOCK_SIZE - 1) >> BLOCK_SIZE_BITS;
// goto;
    }
// squashfs is at block zero too
    if (le32_to_cpu(squashfsb.s_magic) == SQUASHFS_MAGIC) {
    printk!("RAMDISK: squashfs filesystem found at block %d\n",
    start_block);
    nblocks = (le64_to_cpu(squashfsb.bytes_used) + BLOCK_SIZE - 1)
    >> BLOCK_SIZE_BITS;
// goto;
    }
//
// Read 512 bytes further to check if cramfs is padded
//
    pos = start_block * BLOCK_SIZE + 0x200;
    kernel_read(file, buf, size, &pos);
    if (cramfsb.magic == CRAMFS_MAGIC) {
    printk!("RAMDISK: cramfs filesystem found at block %d\n",
    start_block);
    nblocks = (cramfsb.size + BLOCK_SIZE - 1) >> BLOCK_SIZE_BITS;
// goto;
    }
//
// Read block 1 to test for minix and ext2 superblock
//
    pos = (start_block + 1) * BLOCK_SIZE;
    kernel_read(file, buf, size, &pos);
// Try minix
    if (minixsb.s_magic == MINIX_SUPER_MAGIC ||
    minixsb.s_magic == MINIX_SUPER_MAGIC2) {
    printk!("RAMDISK: Minix filesystem found at block %d\n",
    start_block);
    nblocks = minixsb.s_nzones << minixsb.s_log_zone_size;
// goto;
    }
// Try ext2
    n = ext2_image_size(buf);
    if (n) {
    printk!("RAMDISK: ext2 filesystem found at block %d\n",
    start_block);
    nblocks = n;
// goto;
    }
    printk!("RAMDISK: Couldn't find valid RAM disk image starting at %d.\n",
    start_block);
// label;
    kfree(buf);
    return nblocks;
    }
#[no_mangle]
unsafe extern "C" fn nr_blocks(file: *mut file) -> c_ulong {
    let mut inode = file.f_mapping.host;
    if (!S_ISBLK(inode.i_mode)) {
    return 0;
    }
    return i_size_read(inode) >> 10;
    }
#[no_mangle]
pub unsafe extern "C" fn rd_load_image() -> c_int {
pub static mut res: c_int = 0;
    unsigned long rd_blocks, devblocks, nr_disks;
    let mut nblocks = 0;
    let mut i = 0;
    let mut buf = core::ptr::null_mut();
pub static mut rotate: c_ushort = 0;
pub static mut decompressor: decompress_fn = 0;
    char rotator[4] = { '|' , '/' , '-' , '\\' };
    out_file = filp_open("/dev/ram", O_RDWR, 0);
    if (IS_ERR(out_file)) {
// goto;
    }
    in_file = filp_open("/initrd.image", O_RDONLY, 0);
    if (IS_ERR(in_file)) {
// goto;
    }
    in_pos = rd_image_start * BLOCK_SIZE;
    nblocks = identify_ramdisk_image(in_file, in_pos, &decompressor);
    if (nblocks < 0) {
// goto;
    }
    if (nblocks == 0) {
    if (crd_load(decompressor) == 0) {
// goto;
    }
// goto;
    }
//
// NOTE NOTE: nblocks is not actually blocks but
// the number of kibibytes of data to load into a ramdisk.
//
    rd_blocks = nr_blocks(out_file);
    if (nblocks > rd_blocks) {
    printk!("RAMDISK: image too big! (%dKiB/%ldKiB)\n",
    nblocks, rd_blocks);
// goto;
    }
//
// OK, time to copy in the data
//
    devblocks = nblocks;
    if (devblocks == 0) {
    printk!("RAMDISK: could not determine device size\n");
// goto;
    }
    buf = kmalloc(BLOCK_SIZE, GFP_KERNEL);
    if (!buf) {
    printk!("RAMDISK: could not allocate buffer\n");
// goto;
    }
    nr_disks = (nblocks - 1) / devblocks + 1;
    pr_notice("RAMDISK: Loading %dKiB [%ld disk%s] into ram disk... ",
    nblocks, nr_disks, str_plural(nr_disks));
    while (i < nblocks) {
    if (i && (i % devblocks == 0)) {
    pr_cont("done disk #1.\n");
    rotate = 0;
    fput(in_file);
    break;
    }
    kernel_read(in_file, buf, BLOCK_SIZE, &in_pos);
    kernel_write(out_file, buf, BLOCK_SIZE, &out_pos);
    if (!IS_ENABLED!(CONFIG_S390) && !(i % 16)) {
    pr_cont("%c\b", rotator[rotate & 0x3]);
    rotate += 1;
    }
    }
    pr_cont("done.\n");
// label;
    res = 1;
// label;
    fput(in_file);
// label;
    fput(out_file);
// label;
    kfree(buf);
    init_unlink("/dev/ram");
    return res;
    }
    static int exit_code;
    static int decompress_error;
#[no_mangle]
unsafe extern "C" fn compr_fill(buf: *mut c_void, len: c_ulong) -> long __init {
pub static mut r: c_long = 0;
    if (r < 0) {
    printk!("RAMDISK: error while reading compressed data");
    }

    else if (r == 0) {
    printk!("RAMDISK: EOF while reading compressed data");
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn compr_flush(window: *mut c_void, outcnt: c_ulong) -> long __init {
pub static mut written: c_long = 0;
    if (written != outcnt) {
    if (decompress_error == 0) {
    printk!("RAMDISK: incomplete write (%ld != %ld)\n",
    written, outcnt);
    }
    decompress_error = 1;
    return -1;
    }
    return outcnt;
    }
#[no_mangle]
unsafe extern "C" fn error(x: *mut c_char)  {
    printk!("%s\n", x);
    exit_code = 1;
    decompress_error = 1;
    }
#[no_mangle]
unsafe extern "C" fn crd_load(deco: decompress_fn) -> c_int {
    let mut result = 0;
    if (!deco) {
    pr_emerg("Invalid ramdisk decompression routine.  "
    "Select appropriate config option.\n");
    panic("Could not decompress initial ramdisk image.");
    }
    result = deco(core::ptr::null_mut(), 0, compr_fill, compr_flush, core::ptr::null_mut(), core::ptr::null_mut(), error);
    if (decompress_error) {
    result = 1;
    }
    return result;
    }