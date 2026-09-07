//! Automatically rewritten from C to Rust
//! Source: init/main.c
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
macro_rules! pure_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! min_t { ($($tt:tt)*) => { 0 }; }
macro_rules! max_t { ($($tt:tt)*) => { 0 }; }
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
macro_rules! pr_warn_once { ($($tt:tt)*) => {}; }
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
pub struct ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids { pub _opaque: [u8; 0] }

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
pub type compat_uptr_t = u32;
pub type compat_long_t = i32;
pub type compat_ulong_t = u32;
pub type compat_size_t = u32;
pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type compat_mode_t = u32;
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
pub const ENOSYS: c_int = 38;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;
pub const SHMLBA: usize = 4096;
pub const COMPAT_SHMLBA: usize = 4096;

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
macro_rules! kunit_test_init_section_suites { ($($tt:tt)*) => {}; }
macro_rules! kunit_test_suites { ($($tt:tt)*) => {}; }
macro_rules! do_trace_initcall_level { ($($tt:tt)*) => {}; }
macro_rules! do_one_initcall { ($($tt:tt)*) => {}; }
macro_rules! do_initcall_level { ($($tt:tt)*) => {}; }







// SPDX-License-Identifier: GPL-2.0-only
//
// linux/init/main.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// GK 2/5/95  -  Changed to support mounting root fs via NFS
// Added initrd & change_root: Werner Almesberger & Hans Lermen, Feb '96
// Moan early if gcc is old, avoiding bogus kernels - Paul Gortmaker, May '96
// Simplified starting of init:  Michael A. Griffith <grif@acm.org>
//

// Macro flag: #define CREATE_TRACE_POINTS

// forward_decl: kernel_init;
//
// Debug helper: via this flag we know that we are in 'early bootup code'
// where only the boot processor is running with IRQ disabled.  This means
// two things - IRQ must not be enabled before the flag is cleared and some
// operations which are not allowed with IRQ disabled are allowed while the
// flag is set.
//
    pub static mut early_boot_irqs_disabled: usize = 0;
pub static mut system_state: c_int = 0;
    EXPORT_SYMBOL!(system_state);
//
// Boot command-line arguments
//

// Default late time init is NULL. archs can override this later.
pub static mut late_time_init: usize = 0;
// Untouched command line saved by arch-specific code.
pub static mut boot_command_line: usize = 0;
pub static mut envp_init: usize = 0;
pub static mut panic_later: *mut c_char = core::ptr::null_mut();
pub static mut panic_param: *mut c_char = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn obsolete_checksetup(line: *mut c_char) -> bool  {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut had_early_param: bool = false;
    p = __setup_start;
    loop {
pub static mut n: c_int = 0;
    if (parameqn(line, p.str, n)) {
    if (p.early) {
// Already done in parse_early_param?
// (Needs exact match on param part).
// Keep iterating, as we can have early
// params and __setups of same names 8(
    if (line[n] == '\0' || line[n] == '=') {
    had_early_param = true;
    }
    } else if (!p.setup_func) {
    pr_warn!("Parameter %s is obsolete, ignored\n",
    p.str);
    return true;
    } else if (p.setup_func(line + n)) {
    return true;
    }
    }
    p += 1;
    break; }
    return had_early_param;
    }
//
// This should be approx 2 Bo*oMips to start (note initial shift), and will
// still work even if initially too large, it will just take slightly longer
//
pub static mut loops_per_jiffy: c_ulong = 0;
    EXPORT_SYMBOL!(loops_per_jiffy);
#[no_mangle]
unsafe extern "C" fn debug_kernel(str: *mut c_char) -> c_int {
    console_loglevel = CONSOLE_LOGLEVEL_DEBUG;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn quiet_kernel(str: *mut c_char) -> c_int {
    console_loglevel = CONSOLE_LOGLEVEL_QUIET;
    return 0;
    }
    early_param!("debug", debug_kernel);
    early_param!("quiet", quiet_kernel);
#[no_mangle]
unsafe extern "C" fn loglevel(str: *mut c_char) -> c_int {
    let mut newlevel = 0;
//
// Only update loglevel value when a correct setting was passed,
// to prevent blind crashes (when loglevel being set to 0) that
// are quite hard to debug
//
    if (get_option(&str, &newlevel)) {
    console_loglevel = newlevel;
    return 0;
    }
    return -EINVAL;
    }
    early_param!("loglevel", loglevel);

#[no_mangle]
unsafe extern "C" fn get_boot_config_from_initrd(_size: *mut usize) -> *mut c_void {
pub static mut size: usize = 0;
pub static mut csum: usize = 0;
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut hdr: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!initrd_end) {
    return core::ptr::null_mut();
    }
    data = initrd_end - BOOTCONFIG_MAGIC_LEN;
//
// Since Grub may align the size of initrd to 4, we must
// check the preceding 3 bytes as well.
//
    while (i < 4) {
    if (!memcmp(data, BOOTCONFIG_MAGIC, BOOTCONFIG_MAGIC_LEN)) {
// goto;
    }
    data -= 1;
    }
    return core::ptr::null_mut();
// label;
    hdr = (data - 8);
    size = get_unaligned_le32(hdr);
    csum = get_unaligned_le32(hdr + 4);
    data = (hdr) - size;
    if (data < initrd_start) {
    pr_err!("bootconfig size %d is greater than initrd size %ld\n",
    size, initrd_end - initrd_start);
    return core::ptr::null_mut();
    }
    if (xbc_calc_checksum(data, size) != csum) {
    pr_err!("bootconfig checksum failed\n");
    return core::ptr::null_mut();
    }
// Remove bootconfig from initramfs/initrd
    initrd_end = data;
    if (_size) {
// _size = size;
    }
    return data;
    }

#[no_mangle]
unsafe extern "C" fn get_boot_config_from_initrd(_size: *mut usize) -> *mut c_void {
    return core::ptr::null_mut();
    }

// Make an extra command line under given key word
#[no_mangle]
unsafe extern "C" fn xbc_make_cmdline(key: *const c_char) -> *mut char   {
pub static mut root: *mut c_void = core::ptr::null_mut();
pub static mut new_cmdline: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut len = 0;
    root = xbc_find_node(key);
    if (!root) {
    return core::ptr::null_mut();
    }
// Count required buffer size
    len = xbc_snprint_cmdline(core::ptr::null_mut(), 0, root);
    if (len <= 0) {
    return core::ptr::null_mut();
    }
    new_cmdline = memblock_alloc(len + 1, SMP_CACHE_BYTES);
    if (!new_cmdline) {
    pr_err!("Failed to allocate memory for extra kernel cmdline.\n");
    return core::ptr::null_mut();
    }
    ret = xbc_snprint_cmdline(new_cmdline, len + 1, root);
    if (ret < 0 || ret > len) {
    pr_err!("Failed to print extra kernel cmdline.\n");
    memblock_free(new_cmdline, len + 1);
    return core::ptr::null_mut();
    }
    return new_cmdline;
    }
#[no_mangle]
unsafe extern "C" fn warn_bootconfig(str: *mut c_char) -> c_int {
// The 'bootconfig' option is handled by setup_boot_config().
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_boot_config()  {
    let mut msg = core::ptr::null_mut();
    let mut data = core::ptr::null_mut();
    let mut pos = 0;
    let mut ret = 0;
    let mut offs = 0;
    let mut size = 0;
pub static mut from_embedded: bool = false;
// Cut out the bootconfig data even if we have no bootconfig option
    data = get_boot_config_from_initrd(&size);
// If there is no bootconfig in initrd, try embedded one.
    if (!data) {
    data = xbc_get_embedded_bootconfig(&size);
    from_embedded = true;
    }
    bootconfig_found = bootconfig_cmdline_requested(boot_command_line, &offs);
    if (!(bootconfig_found || IS_ENABLED!(CONFIG_BOOT_CONFIG_FORCE))) {
    return;
    }
// Offset of the init arguments after a "--", located by the helper.
    initargs_offs = offs;
    if (!data) {
// If user intended to use bootconfig, show an error level message
    if (bootconfig_found) {
    pr_err!("'bootconfig' found on command line, but no bootconfig found\n");
    }
    else {
    pr_info!("No bootconfig data provided, so skipping bootconfig");
    }
    return;
    }
    if (size >= XBC_DATA_MAX) {
    pr_err!("bootconfig size %ld greater than max size %d\n",
    size, XBC_DATA_MAX);
    return;
    }
    ret = xbc_init(data, size, &msg, &pos);
    if (ret < 0) {
    if (pos < 0) {
    pr_err!("Failed to init bootconfig: %s.\n", msg);
    }
    else {
    pr_err!("Failed to parse bootconfig: %s at %d.\n",
    msg, pos);
    }
    } else {
    xbc_get_info(&ret, core::ptr::null_mut());
    pr_info!("Load bootconfig: %ld bytes %d nodes\n", size, ret);
//
// keys starting with "kernel." are passed via cmdline. When
// this bootconfig came from the embedded source and
// setup_arch() already prepended the rendered "kernel" subtree
// to boot_command_line, rendering again here would duplicate
// the keys in saved_command_line and make accumulating handlers
// (console=, earlycon=, ...) re-register the same value. Skip
// only when the prepend really happened.
//
// On arches that do not select ARCH_SUPPORTS_CMDLINE_FROM_BOOTCONFIG,
// CONFIG_CMDLINE_FROM_BOOTCONFIG is unselectable and
// xbc_embedded_cmdline_applied() collapses to a stub returning
// false, so this path still runs and the embedded "kernel"
// keys reach the cmdline via the runtime parser exactly as
// before this series.
//
    if (!from_embedded || !xbc_embedded_cmdline_applied()) {
    extra_command_line = xbc_make_cmdline("kernel");
    }
// Also, "init." keys are init arguments
    extra_init_args = xbc_make_cmdline("init");
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn exit_boot_config()  {
    xbc_exit();
    }

#[no_mangle]
unsafe extern "C" fn setup_boot_config()  {
// Remove bootconfig data from initrd
    get_boot_config_from_initrd(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn warn_bootconfig(str: *mut c_char) -> c_int {
    pr_warn!("WARNING: 'bootconfig' found on the kernel command line but CONFIG_BOOT_CONFIG is not set.\n");
    return 0;
    }

    early_param!("bootconfig", warn_bootconfig);
#[no_mangle]
pub unsafe extern "C" fn cmdline_has_extra_options() -> bool  {
    return extra_command_line || extra_init_args;
    }
// Change NUL term back to "=", to make "param" the whole string.
#[no_mangle]
unsafe extern "C" fn repair_env_string(param: *mut c_char, val: *mut c_char)  {
    if (val) {
// param=val or param="val"?
    if (val == param+strlen(param)+1) {
    val[-1] = '=';
    }
if true {
    val[-2] = '=';
    memmove(val-1, val, strlen(val)+1);
    } else {
    BUG();
    }
    }
    }
// Anything after -- gets handed straight to init.
#[no_mangle]
pub unsafe extern "C" fn set_init_arg(param: *mut c_char, val: *mut c_char, unused: *mut c_char, arg: *mut c_void) -> c_int {
    let mut i = 0;
    if (panic_later) {
    return 0;
    }
    repair_env_string(param, val);
    while (argv_init[i]) {
    if (i == MAX_INIT_ARGS) {
    panic_later = "init";
    panic_param = param;
    return 0;
    }
    }
    argv_init[i] = param;
    return 0;
    }
//
// Unknown boot options get handed to init, unless they look like
// unused parameters (modprobe will find them in /proc/cmdline).
//
#[no_mangle]
pub unsafe extern "C" fn unknown_bootoption(param: *mut c_char, val: *mut c_char, unused: *mut c_char, arg: *mut c_void) -> c_int {
pub static mut len: usize = 0;
//
// Well-known bootloader identifiers:
// 1. LILO/Grub pass "BOOT_IMAGE=...";
// 2. kexec/kdump (kexec-tools) pass "kexec".
//
pub static mut bootloader: usize = 0;
// Handle params aliased to sysctls
    if (sysctl_is_alias(param)) {
    return 0;
    }
    repair_env_string(param, val);
// Handle bootloader identifier
    while (bootloader[i]) {
    if (strstarts(param, bootloader[i])) {
    return 0;
    }
    }
// Handle obsolete-style parameters
    if (obsolete_checksetup(param)) {
    return 0;
    }
// Unused module parameter.
    if (strnchr(param, len, '.')) {
    return 0;
    }
    if (panic_later) {
    return 0;
    }
    if (val) {
// Environment option
    let mut i = 0;
    while (envp_init[i]) {
    if (i == MAX_INIT_ENVS) {
    panic_later = "env";
    panic_param = param;
    }
    if (!strncmp(param, envp_init[i], len+1)) {
    break;
    }
    }
    envp_init[i] = param;
    } else {
// Command line option
    let mut i = 0;
    while (argv_init[i]) {
    if (i == MAX_INIT_ARGS) {
    panic_later = "init";
    panic_param = param;
    }
    }
    argv_init[i] = param;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_setup(str: *mut c_char) -> c_int {
    let mut i = 0;
    execute_command = str;
//
// In case LILO is going to boot us with default command line,
// it prepends "auto" before the whole cmdline which makes
// the shell think it should execute a script with such name.
// So we ignore all arguments entered _before_ init=... [MJ]
//
    while (i < MAX_INIT_ARGS) {
    argv_init[i] = core::ptr::null_mut();
    }
    return 1;
    }
    __setup!("init=", init_setup);
#[no_mangle]
unsafe extern "C" fn rdinit_setup(str: *mut c_char) -> c_int {
    let mut i = 0;
    ramdisk_execute_command = str;
    ramdisk_execute_command_set = true;
// See "auto" comment in init_setup
    while (i < MAX_INIT_ARGS) {
    argv_init[i] = core::ptr::null_mut();
    }
    return 1;
    }
    __setup!("rdinit=", rdinit_setup);

#[no_mangle]
pub unsafe extern "C" fn setup_nr_cpu_ids() { }
#[no_mangle]
pub unsafe extern "C" fn smp_prepare_cpus(maxcpus: c_uint) { }

//
// We need to store the untouched command line for future reference.
// We also need to store the touched command line since the parameter
// parsing is performed in place, and we should allow a component to
// store reference of name/value for future reference.
//
#[no_mangle]
unsafe extern "C" fn setup_command_line(command_line: *mut c_char)  {
    let mut len = 0;
    let mut xlen = 0;
    let mut ilen = 0;
    if (extra_command_line) {
    xlen = strlen(extra_command_line);
    }
    if (extra_init_args) {
    extra_init_args = strim(extra_init_args); /* remove trailing space */
    ilen = strlen(extra_init_args) + 4; /* for " -- " */
    }
    len = xlen + strlen(boot_command_line) + ilen + 1;
    saved_command_line = memblock_alloc_or_panic(len, SMP_CACHE_BYTES);
    len = xlen + strlen(command_line) + 1;
    static_command_line = memblock_alloc_or_panic(len, SMP_CACHE_BYTES);
    if (xlen) {
//
// We have to put extra_command_line before boot command
// lines because there could be dashes (separator of init
// command line) in the command lines.
//
    strcpy(saved_command_line, extra_command_line);
    strcpy(static_command_line, extra_command_line);
    }
    strcpy(saved_command_line + xlen, boot_command_line);
    strcpy(static_command_line + xlen, command_line);
    if (ilen) {
//
// Append supplemental init boot args to saved_command_line
// so that user can check what command line options passed
// to init.
// The order should always be
// " -- "[bootconfig init-param][cmdline init-param]
//
    if (initargs_offs) {
    len = xlen + initargs_offs;
    strcpy(saved_command_line + len, extra_init_args);
    len += ilen - 4;	/* strlen(extra_init_args) */
    strcpy(saved_command_line + len,
    boot_command_line + initargs_offs - 1);
    } else {
    len = strlen(saved_command_line);
    strcpy(saved_command_line + len, " -- ");
    len += 4;
    strcpy(saved_command_line + len, extra_init_args);
    }
    }
    saved_command_line_len = strlen(saved_command_line);
    }
//
// We need to finalize in a non- function or else race conditions
// between the root thread and the init thread may cause start_kernel to
// be reaped by free_initmem before the root thread has proceeded to
// cpu_idle.
//
// gcc-3.4 accidentally inlines this function, so use noinline.
//
pub static mut kthreadd_done: usize = 0;
#[no_mangle]
unsafe extern "C" fn rest_init()    {
pub static mut kernel_clone_args: usize = 0;
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    let mut pid = 0;
    rcu_scheduler_starting();
//
// We need to spawn init first so that it obtains pid 1, however
// the init task will end up wanting to create kthreads, which, if
// we schedule it before we create kthreadd, will OOPS.
//
    pid = kernel_clone(&init_args);
//
// Pin init on the boot CPU. Task migration is not properly working
// until sched_init_smp() has been run. It will set the allowed
// CPUs for init to the non isolated CPUs.
//
    rcu_read_lock();
    tsk = find_task_by_pid_ns(pid, &init_pid_ns);
    tsk.flags |= PF_NO_SETAFFINITY;
    set_cpus_allowed_ptr(tsk, cpumask_of(smp_processor_id()));
    rcu_read_unlock();
    numa_default_policy();
    pid = kernel_thread(kthreadd, core::ptr::null_mut(), core::ptr::null_mut(), CLONE_FS | CLONE_FILES);
    rcu_read_lock();
    kthreadd_task = find_task_by_pid_ns(pid, &init_pid_ns);
    rcu_read_unlock();
//
// Enable might_sleep() and smp_processor_id() checks.
// They cannot be enabled earlier because with CONFIG_PREEMPTION=y
// kernel_thread() would trigger might_sleep() splats. With
// CONFIG_PREEMPT_VOLUNTARY=y the init task might have scheduled
// already, but it's stuck on the kthreadd_done completion.
//
    system_state = SYSTEM_SCHEDULING;
    complete(&kthreadd_done);
//
// The boot idle thread must execute schedule()
// at least once to get things moving:
//
    schedule_preempt_disabled();
// Call into cpu_idle with preempt disabled
    cpu_startup_entry(CPUHP_ONLINE);
    }
// Check for early params.
#[no_mangle]
pub unsafe extern "C" fn do_early_param(param: *mut c_char, val: *mut c_char, unused: *mut c_char, arg: *mut c_void) -> c_int {
pub static mut p: *mut c_void = core::ptr::null_mut();
    while (p < __setup_end) {
    if (p.early && parameq(param, p.str)) {
    if (p.setup_func(val) != 0) {
    pr_warn!("Malformed early option '%s'\n", param);
    }
    }
    }
// We accept everything at this stage.
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_early_options(cmdline: *mut c_char)  {
    parse_args("early options", cmdline, core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(),
    do_early_param);
    }
// Arch code calls this early on, or if not, just before other parsing.
#[no_mangle]
pub unsafe extern "C" fn parse_early_param()  {
pub static mut done: usize = 0;
pub static mut tmp_cmdline: usize = 0;
// Keep these in sync with initcalls in include/linux/init.h
pub static mut initcall_level_names: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ignore_unknown_bootoption(param: *mut c_char, val: *mut c_char, unused: *mut c_char, arg: *mut c_void) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_initcall_level(level: c_int, command_line: *mut c_char)  {
pub static mut r#fn: *mut c_void = core::ptr::null_mut();
    parse_args(initcall_level_names[level],
    command_line, __start___param,
    __stop___param - __start___param,
    level, level,
    core::ptr::null_mut(), ignore_unknown_bootoption);
    do_trace_initcall_level(initcall_level_names[level]);
    while (r#fn < initcall_levels[level+1]) {
    do_one_initcall(initcall_from_entry(r#fn));
    }
    }
#[no_mangle]
unsafe extern "C" fn do_initcalls()  {
    let mut level = 0;
pub static mut len: usize = 0;
pub static mut command_line: *mut c_void = core::ptr::null_mut();
    command_line = kzalloc(len, GFP_KERNEL);
    if (!command_line) {
    panic("%s: Failed to allocate %zu bytes\n", __func__, len);
    }
    while (level < ARRAY_SIZE!(initcall_levels) - 1) {
// Parser modifies command_line, restore it each time
    strcpy(command_line, saved_command_line);
    do_initcall_level(level, command_line);
    }
    kfree(command_line);
    }
//
// Ok, the machine is now initialized. None of the devices
// have been touched yet, but the CPU subsystem is up and
// running, and memory and process management works.
//
// Now we can finally start doing some real work..
//
#[no_mangle]
unsafe extern "C" fn do_basic_setup()  {
    cpuset_init_smp();
    ksysfs_init();
    driver_init();
    init_irq_proc();
    do_ctors();
    do_initcalls();
    }
#[no_mangle]
unsafe extern "C" fn do_pre_smp_initcalls()  {
pub static mut r#fn: *mut c_void = core::ptr::null_mut();
    do_trace_initcall_level("early");
    while (r#fn < __initcall0_start) {
    do_one_initcall(initcall_from_entry(r#fn));
    }
    }
#[no_mangle]
unsafe extern "C" fn run_init_process(init_filename: *const c_char) -> c_int {
    let mut p = core::ptr::null_mut();
    argv_init[0] = init_filename;
    pr_info!("Run %s as init process\n", init_filename);
    pr_debug!("  with arguments:\n");
    while (*p) {
    pr_debug!("    %s\n", *p);
    }
    pr_debug!("  with environment:\n");
    while (*p) {
    pr_debug!("    %s\n", *p);
    }
    return kernel_execve(init_filename, argv_init, envp_init);
    }
#[no_mangle]
unsafe extern "C" fn try_to_run_init_process(init_filename: *const c_char) -> c_int {
    let mut ret = 0;
    ret = run_init_process(init_filename);
    if (ret && ret != -ENOENT) {
    pr_err!("Starting init: %s exists but couldn't execute it (error %d)\n",
    init_filename, ret);
    }
    return ret;
    }
// forward_decl: kernel_init_freeable;

pub static mut rodata_enabled: bool = true;

#[no_mangle]
pub unsafe extern "C" fn arch_parse_debug_rodata(str: *mut c_char) -> bool { return false; }

#[no_mangle]
unsafe extern "C" fn set_debug_rodata(str: *mut c_char) -> c_int {
    if (arch_parse_debug_rodata(str)) {
    return 0;
    }
    if (str && !strcmp(str, "on")) {
    rodata_enabled = true;
    }

    else if (str && !strcmp(str, "off")) {
    rodata_enabled = false;
    }
    else {
    pr_warn!("Invalid option string for rodata: '%s'\n", str);
    }
    return 0;
    }
    early_param!("rodata", set_debug_rodata);

#[no_mangle]
unsafe extern "C" fn mark_readonly() {
    if (IS_ENABLED!(CONFIG_STRICT_KERNEL_RWX) && rodata_enabled) {
//
// load_module() results in W+X mappings, which are cleaned
// up with init_free_wq. Let's make sure that queued work is
// flushed so that we don't hit false positives looking for
// insecure pages which are W+X.
//
    flush_module_init_free_work();
    jump_label_init_ro();
    mark_rodata_ro();
    debug_checkwx();
    rodata_test();
    } else if (IS_ENABLED!(CONFIG_STRICT_KERNEL_RWX)) {
    pr_info!("Kernel memory protection disabled.\n");
    } else if (IS_ENABLED!(CONFIG_ARCH_HAS_STRICT_KERNEL_RWX)) {
    pr_warn!("Kernel memory protection not selected by kernel config.\n");
    } else {
    pr_warn!("This architecture does not have kernel memory protection.\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn free_initmem()   {
    free_initmem_default(POISON_FREE_INITMEM);
    }
#[no_mangle]
unsafe extern "C" fn kernel_init(unused: *mut c_void) -> c_int  {
    let mut ret = 0;
    init_userspace_fs();
//
// Wait until kthreadd is all set-up.
//
    wait_for_completion(&kthreadd_done);
    kernel_init_freeable();
// need to finish all async  code before freeing the memory
    async_synchronize_full();
    system_state = SYSTEM_FREEING_INITMEM;
    kprobe_free_init_mem();
    ftrace_free_init_mem();
    kgdb_free_init_mem();
    exit_boot_config();
    free_initmem();
    mark_readonly();
//
// Kernel mappings are now finalized - update the userspace page-table
// to finalize PTI.
//
    pti_finalize();
    system_state = SYSTEM_RUNNING;
    numa_default_policy();
    rcu_end_inkernel_boot();
    do_sysctl_args();
    if (ramdisk_execute_command) {
    ret = run_init_process(ramdisk_execute_command);
    if (!ret) {
    return 0;
    }
    pr_err!("Failed to execute %s (error %d)\n",
    ramdisk_execute_command, ret);
    }
//
// We try each of these until one succeeds.
//
// The Bourne shell can be used instead of init if we are
// trying to recover a really broken machine.
//
    if (execute_command) {
    ret = run_init_process(execute_command);
    if (!ret) {
    return 0;
    }
    panic("Requested init %s failed (error %d).",
    execute_command, ret);
    }
    if (CONFIG_DEFAULT_INIT[0] != '\0') {
    ret = run_init_process(CONFIG_DEFAULT_INIT);
    if (ret) {
    pr_err!("Default init %s failed (error %d)\n",
    CONFIG_DEFAULT_INIT, ret);
    }
    else {
    return 0;
    }
    }
    if (!try_to_run_init_process("/sbin/init") ||
    !try_to_run_init_process("/etc/init") ||
    !try_to_run_init_process("/bin/init") ||
    !try_to_run_init_process("/bin/sh")) {
    return 0;
    }
    panic("No working init found.  Try passing init= option to kernel. See Linux Documentation/admin-guide/init.rst for guidance.");
    }
// Open /dev/console, for stdin/stdout/stderr, this should never fail
#[no_mangle]
pub unsafe extern "C" fn console_on_rootfs()  {
    let mut file = filp_open("/dev/console", O_RDWR, 0);
    if (IS_ERR(file)) {
    pr_err!("Warning: unable to open an initial console.\n");
    return;
    }
    init_dup(file);
    init_dup(file);
    init_dup(file);
    fput(file);
    }
#[no_mangle]
unsafe extern "C" fn kernel_init_freeable()   {
// Now the scheduler is fully set up and can do blocking allocations
    gfp_allowed_mask = __GFP_BITS_MASK;
//
// init can allocate pages on any node
//
    set_mems_allowed(node_states[N_MEMORY]);
    cad_pid = get_pid(task_pid(current));
    smp_prepare_cpus(setup_max_cpus);
    workqueue_init();
    init_mm_internals();
    do_pre_smp_initcalls();
    lockup_detector_init();
    smp_init();
    sched_init_smp();
    workqueue_init_topology();
    async_init();
    padata_init();
    page_alloc_init_late();
    do_basic_setup();
    kunit_run_all_tests();
    wait_for_initramfs();
    console_on_rootfs();
//
// check if there is an early userspace init.  If yes, let it do all
// the work
//
    let mut ramdisk_command_access = 0;
    ramdisk_command_access = init_eaccess(ramdisk_execute_command);
    if (ramdisk_command_access != 0) {
    if (ramdisk_execute_command_set) {
    pr_warn!("check access for rdinit=%s failed: %i, ignoring\n",
    ramdisk_execute_command, ramdisk_command_access);
    }
    ramdisk_execute_command = core::ptr::null_mut();
    prepare_namespace();
    }
//
// Ok, we have completed the initial bootup, and
// we're essentially up and running. Get rid of the
// initmem segments and start the user-mode stuff..
//
// rootfs is available now, try loading the public keys
// and default modules
//
// forward_decl: egrity_load_keys;
    }
}