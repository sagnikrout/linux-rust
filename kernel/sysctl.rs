//! Automatically rewritten from C to Rust
//! Source: kernel/sysctl.c
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

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===

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
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }



// SPDX-License-Identifier: GPL-2.0-only
//
// sysctl.c: General linux system control interface
//

// shared constants to be used in various sysctls
    const int sysctl_vals[] = { 0, 1, 2, 3, 4, 100, 200, 1000, 3000, INT_MAX, 65535, -1 };
    EXPORT_SYMBOL(sysctl_vals);
    const unsigned long sysctl_long_vals[] = { 0, 1, LONG_MAX };
    EXPORT_SYMBOL_GPL(sysctl_long_vals);

// Constants used for minimum and maximum
pub static mut ngroups_max: int = 0;
pub static mut cap_last_cap: int = 0;

//
// enum sysctl_writes_mode - supported sysctl write modes
//
// @SYSCTL_WRITES_LEGACY: each write syscall must fully contain the sysctl value
// to be written, and multiple writes on the same sysctl file descriptor
// will rewrite the sysctl value, regardless of file position. No warning
// is issued when the initial position is not 0.
// @SYSCTL_WRITES_WARN: same as above but warn when the initial file position is
// not 0.
// @SYSCTL_WRITES_STRICT: writes to numeric sysctl entries must always be at
// file position 0 and the value must be fully contained in the buffer
// sent to the write syscall. If dealing with strings respect the file
// position, but restrict this to the max length of the buffer, anything
// passed the max length will be ignored. Multiple writes will append
// to the buffer.
//
// These write modes control how current file position affects the behavior of
// updating internal kernel (SYSCTL_USER_TO_KERN) sysctl values through the proc
// interface on each write.
//
    enum sysctl_writes_mode {
    SYSCTL_WRITES_LEGACY		= -1,
    SYSCTL_WRITES_WARN		= 0,
    SYSCTL_WRITES_STRICT		= 1,
    };
pub static mut sysctl_writes_strict: sysctl_writes_mode = 0;

//
// /proc/sys support
//

#[no_mangle]
pub unsafe extern "C" fn _proc_do_string(data: *mut c_char, maxlen: c_int, dir: c_int, buffer: *mut c_char, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut len = 0;
    char c, *p;
    if (!data || !maxlen || !*lenp) {
// lenp = 0;
    return 0;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    if (sysctl_writes_strict == SYSCTL_WRITES_STRICT) {
// Only continue writes not past the end of buffer.
    len = strlen(data);
    if (len > maxlen - 1) {
    len = maxlen - 1;
    }
    if (*ppos > len) {
    return 0;
    }
    len = *ppos;
    } else {
// Start writing from beginning of buffer.
    len = 0;
    }
// ppos += *lenp;
    p = buffer;
    while ((p - buffer) < *lenp && len < maxlen - 1) {
    c = *(p++);
    if (c == 0 || c == '\n') {
    break;
    }
    data[len++] = c;
    }
    data[len] = 0;
    } else {
    len = strlen(data);
    if (len > maxlen) {
    len = maxlen;
    }
    if (*ppos > len) {
// lenp = 0;
    return 0;
    }
    data += *ppos;
    len  -= *ppos;
    if (len > *lenp) {
    len = *lenp;
    }
    if (len) {
    memcpy(buffer, data, len);
    }
    if (len < *lenp) {
    buffer[len] = '\n';
    len += 1;
    }
// lenp = len;
// ppos += len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn warn_sysctl_write(table: *const ctl_table) {
    pr_warn_once("%s wrote to %s when file position was not 0!\n"
    "This will not be supported in the future. To silence this\n"
    "warning, set kernel.sysctl_writes_strict = -1\n",
    current.comm, table.procname);
    }
//
// proc_first_pos_non_zero_ignore - check if first position is allowed
// @ppos: file position
// @table: the sysctl table
//
// Returns: true if the first position is non-zero and the sysctl_writes_strict
// mode indicates this is not allowed for numeric input types. String proc
// handlers can ignore the return value.
//
#[no_mangle]
pub unsafe extern "C" fn proc_first_pos_non_zero_ignore(ppos: *mut loff_t, table: *mut ctl_table) -> bool {
    if (!*ppos) {
    return false;
    }
    match (sysctl_writes_strict) {
    SYSCTL_WRITES_STRICT => {
    return true;
    }
    SYSCTL_WRITES_WARN => {
    warn_sysctl_write(table);
    return false;
    }
    _ => {
    return false;
    }
    }
    }
//
// proc_dostring - read a string sysctl
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes a string from/to the user buffer. If the kernel
// buffer provided is not large enough to hold the string, the
// string is truncated. The copied string is %NULL-terminated.
// If the string is being read by the user process, it is copied
// and a newline '\n' is added. It is truncated if the buffer is
// not large enough.
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dostring(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    if (SYSCTL_USER_TO_KERN(dir)) {
    proc_first_pos_non_zero_ignore(ppos, table);
    }
    return _proc_do_string(table.data, table.maxlen, dir, buffer, lenp,
    ppos);
    }
#[no_mangle]
unsafe extern "C" fn proc_skip_spaces(buf: *mut c_char, size: *mut usize) {
    while (*size) {
    if (!isspace(**buf)) {
    break;
    }
    (*size)--;
    (*buf)++;
    }
    }
#[no_mangle]
unsafe extern "C" fn proc_skip_char(buf: *mut c_char, size: *mut usize, v: c_char) {
    while (*size) {
    if (**buf != v) {
    break;
    }
    (*size)--;
    (*buf)++;
    }
    }
//
// strtoul_lenient - parse an ASCII formatted integer from a buffer and only
// fail on overflow
//
// @cp: kernel buffer containing the string to parse
// @endp: pointer to store the trailing characters
// @base: the base to use
// @res: where the parsed integer will be stored
//
// This function will fail the parse on overflow. If there wasn't an overflow
// the function will defer the decision what characters count as invalid to the
// caller.
//
// Returns:
// * %0 on success and @res will contain the parsed integer,
// @endp will hold any trailing characters.
// * %-ERANGE on overflow.
//
#[no_mangle]
pub unsafe extern "C" fn strtoul_lenient(cp: *mut c_char, endp: *mut *mut c_char, base: c_uint, res: *mut c_ulong) -> c_int {
    unsigned long long result;
    let mut rv = 0;
    cp = _parse_integer_fixup_radix(cp, &base);
    rv = _parse_integer(cp, base, &result);
    if ((rv & KSTRTOX_OVERFLOW) || (result != (unsigned long)result)) {
    return -ERANGE;
    }
    cp += rv;
    if (endp) {
// endp = cp;
    }
// res = (unsigned long)result;
    return 0;
    }
pub const TMPBUFLEN: c_int = 22;
//
// proc_get_long - reads an ASCII formatted integer from a user buffer
//
// @buf: a kernel buffer
// @size: size of the kernel buffer
// @val: this is where the number will be stored
// @neg: set to %TRUE if number is negative
// @perm_tr: a vector which contains the allowed trailers
// @perm_tr_len: size of the perm_tr vector
// @tr: pointer to store the trailer character
//
// Returns:
// * %0 on success and @buf and @size are updated with
// the amount of bytes read. If @tr is non-NULL and a trailing
// character exists (size is non-zero after returning from this
// function), @tr is updated with the trailing character.
// * %-EINVAL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn proc_get_long(buf: *mut *mut c_char, size: *mut size_t, val: *mut c_ulong, neg: *mut bool, perm_tr: *mut c_char, perm_tr_len: c_uint, tr: *mut c_char) -> c_int {
    char *p, tmp[TMPBUFLEN];
pub static mut len: isize = 0;
    if (len <= 0) {
    return -EINVAL;
    }
    if (len > TMPBUFLEN - 1) {
    len = TMPBUFLEN - 1;
    }
    memcpy(tmp, *buf, len);
    tmp[len] = 0;
    p = tmp;
    if (*p == '-' && *size > 1) {
// neg = true;
    p += 1;
    } else {
// neg = false;
    }
    if (!isdigit(*p)) {
    return -EINVAL;
    }
    if (strtoul_lenient(p, &p, 0, val)) {
    return -EINVAL;
    }
    len = p - tmp;
// We don't know if the next char is whitespace thus we may accept
// invalid integers (e.g. 1234...a) or two integers instead of one
// (e.g. 123...1). So lets not allow such large numbers.
    if (len == TMPBUFLEN - 1) {
    return -EINVAL;
    }
    if (len < *size && perm_tr_len && !memchr(perm_tr, *p, perm_tr_len)) {
    return -EINVAL;
    }
    if (tr && (len < *size)) {
// tr = *p;
    }
// buf += len;
// size -= len;
    return 0;
    }
//
// proc_put_long - converts an integer to a decimal ASCII formatted string
//
// @buf: the user buffer
// @size: the size of the user buffer
// @val: the integer to be converted
// @neg: sign of the number, %TRUE for negative
//
// In case of success @buf and @size are updated with the amount of bytes
// written.
//
#[no_mangle]
unsafe extern "C" fn proc_put_long(buf: *mut c_void, size: *mut usize, val: c_ulong, neg: bool) {
    let mut len = 0;
    char tmp[TMPBUFLEN], *p = tmp;
    sprintf(p, "%s%lu", neg ? "-" : "", val);
    len = strlen(tmp);
    if (len > *size) {
    len = *size;
    }
    memcpy(*buf, tmp, len);
// size -= len;
// buf += len;
    }

#[no_mangle]
unsafe extern "C" fn proc_put_char(buf: *mut c_void, size: *mut usize, c: c_char) {
    if (*size) {
    let mut buffer = buf;
// buffer = c;
    (*size)--;
    (*buffer)++;
// buf = *buffer;
    }
    }
//
// proc_uint_u2k_conv_uop - Assign user value to a kernel pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @u_ptr_op: execute this function before assigning to k_ptr
//
// Uses WRITE_ONCE to assign value to k_ptr. Executes u_ptr_op if
// not NULL. Check that the values are less than UINT_MAX to avoid
// having to support wrap around from userspace.
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_uint_u2k_conv_uop(u_ptr: *mut c_ulong, k_ptr: *mut c_uint) -> c_int {
pub static mut u: c_ulong = 0;
    if (u > UINT_MAX) {
    return -EINVAL;
    }
    WRITE_ONCE(*k_ptr, u);
    return 0;
    }
//
// proc_uint_k2u_conv - Assign kernel value to a user space pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
//
// Uses READ_ONCE to assign value to u_ptr.
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_uint_k2u_conv(u_ptr: *mut c_ulong, k_ptr: *const c_uint) -> c_int {
pub static mut val: c_uint = 0;
// u_ptr = (ulong)val;
    return 0;
    }
//
// proc_uint_conv - Change user or kernel pointer based on direction
//
// @u_ptr: pointer to user variable
// @k_ptr: pointer to kernel variable
// @dir: %TRUE if this is a write to the sysctl file
// @tbl: the sysctl table
// @k_ptr_range_check: Check range for k_ptr when %TRUE
// @user_to_kern: Callback used to assign value from user to kernel var
// @kern_to_user: Callback used to assign value from kernel to user var
//
// When direction is kernel to user, then the u_ptr is modified.
// When direction is user to kernel, then the k_ptr is modified.
//
// Returns: %0 on success
//
#[no_mangle]
pub unsafe extern "C" fn proc_uint_conv(u_ptr: *mut c_ulong, k_ptr: *mut c_uint, dir: c_int, tbl: *mut ctl_table, k_ptr_range_check: bool, u_ptr: *mut *mut int (user_to_kern)( ulong, u_ptr: *mut *mut int (kern_to_user)(ulong) -> c_int {
    if (SYSCTL_KERN_TO_USER(dir)) {
    return kern_to_user(u_ptr, k_ptr);
    }
    if (k_ptr_range_check) {
    let mut tmp_k;
    let mut ret = 0;
    if (!tbl) {
    return -EINVAL;
    }
    ret = user_to_kern(u_ptr, &tmp_k);
    if (ret) {
    return ret;
    }
    if ((tbl.extra1 &&
// tbl->extra1 > tmp_k) ||
    (tbl.extra2 &&
tbl.extra2 < tmp_k)) {
    return -ERANGE;
    }
    WRITE_ONCE(*k_ptr, tmp_k);
    } else {
    return user_to_kern(u_ptr, k_ptr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_uint_u2k_conv(u_ptr: *const c_ulong, k_ptr: *mut c_uint) -> c_int {
    return proc_uint_u2k_conv_uop(u_ptr, k_ptr, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_uint_conv(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_uint, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_uint_conv(u_ptr, k_ptr, dir, tbl, false,
    proc_uint_u2k_conv, proc_uint_k2u_conv);
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_uint_conv_minmax(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_uint, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_uint_conv(u_ptr, k_ptr, dir, tbl, true,
    proc_uint_u2k_conv, proc_uint_k2u_conv);
    }
//
// proc_int_k2u_conv_kop - Assign kernel value to a user space pointer
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @negp: assigned %TRUE if the converted kernel value is negative;
// %FALSE otherweise
// @k_ptr_op: execute this function before assigning to u_ptr
//
// Uses READ_ONCE to get value from k_ptr. Executes k_ptr_op before assigning
// to u_ptr if not NULL. Does **not** check for overflow.
//
// Returns: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_int_k2u_conv_kop(u_ptr: *mut c_ulong, k_ptr: *mut c_int, negp: *mut bool) -> c_int {
pub static mut val: c_int = 0;
    if (val < 0) {
// negp = true;
// u_ptr = k_ptr_op ? -k_ptr_op((ulong)val) : -(ulong)val;
    } else {
// negp = false;
// u_ptr = k_ptr_op ? k_ptr_op((ulong)val) : (ulong) val;
    }
    return 0;
    }
//
// proc_int_u2k_conv_uop - Assign user value to a kernel pointer
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @negp: If %TRUE, the converted user value is made negative.
// @u_ptr_op: execute this function before assigning to k_ptr
//
// Uses WRITE_ONCE to assign value to k_ptr. Executes u_ptr_op if
// not NULL. Check for overflow with UINT_MAX.
//
// Returns: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_int_u2k_conv_uop(u_ptr: *mut c_ulong, k_ptr: *mut c_int, negp: *mut bool) -> c_int {
pub static mut u: c_ulong = 0;
    if (*negp) {
    if (u > (ulong) INT_MAX + 1) {
    return -EINVAL;
    }
    WRITE_ONCE(*k_ptr, -u);
    } else {
    if (u > (ulong) INT_MAX) {
    return -EINVAL;
    }
    WRITE_ONCE(*k_ptr, u);
    }
    return 0;
    }
//
// proc_int_conv - Change user or kernel pointer based on direction
//
// @negp: will be passed to uni-directional converters
// @u_ptr: pointer to user variable
// @k_ptr: pointer to kernel variable
// @dir: %TRUE if this is a write to the sysctl file
// @tbl: the sysctl table
// @k_ptr_range_check: Check range for k_ptr when %TRUE
// @user_to_kern: Callback used to assign value from user to kernel var
// @kern_to_user: Callback used to assign value from kernel to user var
//
// When direction is kernel to user, then the u_ptr is modified.
// When direction is user to kernel, then the k_ptr is modified.
//
// Returns: 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn proc_int_conv(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table, k_ptr_range_check: bool, negp: *mut *mut int (user_to_kern)( bool, u_ptr: *mut c_ulong, negp: *mut *mut int (kern_to_user)(bool, u_ptr: *mut c_ulong) -> c_int {
    if (SYSCTL_KERN_TO_USER(dir)) {
    return kern_to_user(negp, u_ptr, k_ptr);
    }
    if (k_ptr_range_check) {
    let mut tmp_k = 0;
    let mut ret = 0;
    if (!tbl) {
    return -EINVAL;
    }
    ret = user_to_kern(negp, u_ptr, &tmp_k);
    if (ret) {
    return ret;
    }
    if ((tbl.extra1 && *tbl.extra1 > tmp_k) ||
    (tbl.extra2 && *tbl.extra2 < tmp_k)) {
    return -EINVAL;
    }
    WRITE_ONCE(*k_ptr, tmp_k);
    } else {
    return user_to_kern(negp, u_ptr, k_ptr);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sysctl_user_to_kern_int_conv(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int) -> c_int {
    return proc_int_u2k_conv_uop(u_ptr, k_ptr, negp, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sysctl_kern_to_user_int_conv(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *const c_int) -> c_int {
    return proc_int_k2u_conv_kop(u_ptr, k_ptr, negp, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_int_conv(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, false,
    sysctl_user_to_kern_int_conv,
    sysctl_kern_to_user_int_conv);
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_int_conv_minmax(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, true,
    sysctl_user_to_kern_int_conv,
    sysctl_kern_to_user_int_conv);
    }
    static const char proc_wspace_sep[] = { ' ', '\t', '\n' };
//
// Element type processed by do_proc_vec(). The tag selects the element size
// and signedness, and it selects which member of union proc_vec_conv is live.
//
    enum proc_vec_type {
    PROC_VEC_INT,
    PROC_VEC_UINT,
    PROC_VEC_ULONG,
    };
//
// Converter passed to do_proc_vec(). Only the member matching the
// enum proc_vec_type tag is ever read, so every dispatch stays fully typed and
// no void * converter pointer is needed.
//
    union proc_vec_conv {
    int (*int_conv)(bool *negp, ulong *u_ptr, int *k_ptr,
    int dir, const struct ctl_table *table);
    int (*uint_conv)(bool *negp, ulong *u_ptr, uint *k_ptr,
    int dir, const struct ctl_table *table);
    int (*ulong_conv)(bool *negp, ulong *u_ptr, ulong *k_ptr,
    int dir, const struct ctl_table *table);
    };
//
// Dispatch to the converter member selected by @type. @k_ptr walks
// table->data as raw bytes and is cast back to the element type here.
//
#[no_mangle]
pub unsafe extern "C" fn proc_vec_conv(type: proc_vec_type, conv: union proc_vec_conv, negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_char, dir: c_int, table: *mut ctl_table) -> c_int {
    match (type) {
    PROC_VEC_INT => {
    return conv.int_conv(negp, u_ptr, k_ptr, dir, table);
    }
    PROC_VEC_UINT => {
    return conv.uint_conv(negp, u_ptr, k_ptr, dir, table);
    }
    PROC_VEC_ULONG => {
    return conv.ulong_conv(negp, u_ptr, k_ptr, dir, table);
    }
    }
    return -EINVAL;
    }
//
// Read/write a vector of @type elements. The element size and signedness are
// derived from @type, so a single runtime function replaces the per-type
// variants. table->data is walked as raw bytes (@i) advanced by @size; the
// converter performs the actual typed load/store.
//
#[no_mangle]
pub unsafe extern "C" fn do_proc_vec(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t, type: proc_vec_type, conv: union proc_vec_conv) -> c_int {
    int vleft, first = 1, err = 0;
    size_t left, size;
    let mut is_unsigned = 0;
    let mut i = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    match (type) {
    PROC_VEC_INT => {
    size = sizeof!(int);
    is_unsigned = false;
    // break;
    }
    PROC_VEC_UINT => {
    size = sizeof!(uint);
    is_unsigned = true;
    // break;
    }
    PROC_VEC_ULONG => {
    size = sizeof!(ulong);
    is_unsigned = true;
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    if (!table.data || !table.maxlen || !*lenp ||
    (*ppos && SYSCTL_KERN_TO_USER(dir))) {
// lenp = 0;
    return 0;
    }
    i = table.data;
    vleft = table.maxlen / size;
    left = *lenp;
// uint arrays are not supported, *Do not* add support for them.
    if (type == PROC_VEC_UINT && vleft != 1) {
    return -EINVAL;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    if (proc_first_pos_non_zero_ignore(ppos, table)) {
// goto;
    }
    if (left > PAGE_SIZE - 1) {
    left = PAGE_SIZE - 1;
    }
    p = buffer;
    }
    while (left && vleft--) {
    let mut lval = 0;
pub static mut neg: bool = false;
    if (SYSCTL_USER_TO_KERN(dir)) {
    proc_skip_spaces(&p, &left);
    if (!left) {
    break;
    }
    err = proc_get_long(&p, &left, &lval, &neg,
    proc_wspace_sep,
    sizeof!(proc_wspace_sep), core::ptr::null_mut());
    if (!err && neg && is_unsigned) {
    err = -EINVAL;
    }
    if (err) {
    break;
    }
    if (proc_vec_conv(type, conv, &neg, &lval, i, dir, table)) {
    err = -EINVAL;
    break;
    }
    } else {
    if (proc_vec_conv(type, conv, &neg, &lval, i, dir, table)) {
    err = -EINVAL;
    break;
    }
    if (!first) {
    proc_put_char(&buffer, &left, '\t');
    }
    proc_put_long(&buffer, &left, lval, neg);
    }
    }
    if (SYSCTL_KERN_TO_USER(dir) && !first && left && !err) {
    proc_put_char(&buffer, &left, '\n');
    }
    if (SYSCTL_USER_TO_KERN(dir) && !err && left) {
    proc_skip_spaces(&p, &left);
    }
    if (SYSCTL_USER_TO_KERN(dir) && first) {
    return err ? : -EINVAL;
    }
// lenp -= left;
// label;
// ppos += *lenp;
    return err;
    }
//
// proc_douintvec_conv - read a vector of unsigned ints with a custom converter
//
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
// @conv: Custom converter call back
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) unsigned integer
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// Returns: %0 on success
//
#[no_mangle]
pub unsafe extern "C" fn proc_douintvec_conv(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t, negp: *mut *mut int (conv)(bool, u_ptr: *mut c_ulong, k_ptr: *mut c_uint, dir: c_int) -> c_int {
    if (!conv) {
    conv = do_proc_uint_conv;
    }
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = conv });
    }
//
// proc_dobool - read/write a bool
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes one integer value from/to the user buffer,
// treated as an ASCII string.
//
// table->data must point to a bool variable and table->maxlen must
// be sizeof!(bool).
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dobool(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
pub static mut tmp: usize = 0;
    let mut data = table.data;
    let mut res = 0;
    let mut val = 0;
// Do not support arrays yet.
    if (table.maxlen != sizeof!(bool)) {
    return -EINVAL;
    }
    tmp = *table;
    tmp.maxlen = sizeof!(val);
    tmp.data = &val;
    val = READ_ONCE(*data);
    res = proc_dointvec(&tmp, dir, buffer, lenp, ppos);
    if (res) {
    return res;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    WRITE_ONCE(*data, val);
    }
    return 0;
    }
//
// proc_dointvec - read a vector of integers
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) integer
// values from/to the user buffer, treated as an ASCII string.
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dointvec(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_INT,
    (union proc_vec_conv){ .int_conv = do_proc_int_conv });
    }
//
// proc_douintvec - read a vector of unsigned integers
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) unsigned integer
// values from/to the user buffer, treated as an ASCII string.
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_douintvec(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = do_proc_uint_conv });
    }
//
// proc_dointvec_minmax - read a vector of integers with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) integer
// values from/to the user buffer, treated as an ASCII string.
//
// This routine will ensure the values are within the range specified by
// table->extra1 (min) and table->extra2 (max).
//
// Returns: %0 on success or -EINVAL when the range check fails and
// SYSCTL_USER_TO_KERN(dir) == true
//
#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_minmax(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_INT,
    (union proc_vec_conv){ .int_conv = do_proc_int_conv_minmax });
    }
//
// proc_douintvec_minmax - read a vector of unsigned ints with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) unsigned integer
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// When changing the kernel variable, this routine will ensure the values
// are within the range specified by table->extra1 (min) and table->extra2
// (max). And Check that the values are less than UINT_MAX to avoid having to
// support wrap around uses from userspace.
//
// Returns: %0 on success or -ERANGE when range check failes and
// SYSCTL_USER_TO_KERN(dir) == true
//
#[no_mangle]
pub unsafe extern "C" fn proc_douintvec_minmax(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = do_proc_uint_conv_minmax });
    }
//
// proc_dou8vec_minmax - read a vector of unsigned chars with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(u8) unsigned chars
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// This routine will ensure the values are within the range specified by
// table->extra1 (min) and table->extra2 (max).
//
// Returns: %0 on success or an error on SYSCTL_USER_TO_KERN(dir) == true
// and the range check fails.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dou8vec_minmax(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
pub static mut tmp: usize = 0;
pub static mut min: c_uint = 0;
    let mut data = table.data;
    let mut res = 0;
// Do not support arrays yet.
    if (table.maxlen != sizeof!(u8)) {
    return -EINVAL;
    }
    tmp = *table;
    tmp.maxlen = sizeof!(val);
    tmp.data = &val;
    if (!tmp.extra1) {
    tmp.extra1 =  &min;
    }
    if (!tmp.extra2) {
    tmp.extra2 =  &max;
    }
    val = READ_ONCE(*data);
    res = do_proc_vec(&tmp, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = do_proc_uint_conv_minmax });
    if (res) {
    return res;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    WRITE_ONCE(*data, val);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(proc_dou8vec_minmax);
//
// proc_ulong_conv - Change user or kernel pointer based on direction
//
// @u_ptr: pointer to user variable
// @k_ptr: pointer to kernel variable
// @dir: %TRUE if this is a write to the sysctl file
// @tbl: the sysctl table
// @k_ptr_range_check: Check range for k_ptr when %TRUE
// @user_to_kern: Callback used to assign value from user to kernel var
// @kern_to_user: Callback used to assign value from kernel to user var
//
// When direction is kernel to user, then the u_ptr is modified.
// When direction is user to kernel, then the k_ptr is modified.
//
// Returns: 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn proc_ulong_conv(u_ptr: *mut c_ulong, k_ptr: *mut c_ulong, dir: c_int, tbl: *mut ctl_table, k_ptr_range_check: bool, u_ptr: *mut *mut int (user_to_kern)( ulong, u_ptr: *mut *mut int (kern_to_user)(ulong) -> c_int {
    if (SYSCTL_KERN_TO_USER(dir)) {
    return kern_to_user(u_ptr, k_ptr);
    }
    if (k_ptr_range_check) {
    let mut tmp_k;
    let mut ret = 0;
    if (!tbl) {
    return -EINVAL;
    }
    ret = user_to_kern(u_ptr, &tmp_k);
    if (ret) {
    return ret;
    }
    if ((tbl.extra1 && *tbl.extra1 > tmp_k) ||
    (tbl.extra2 && *tbl.extra2 < tmp_k)) {
    return -ERANGE;
    }
    WRITE_ONCE(*k_ptr, tmp_k);
    } else {
    return user_to_kern(u_ptr, k_ptr);
    }
    return 0;
    }
//
// proc_ulong_u2k_conv_uop - Assign user value to a kernel pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @u_ptr_op: execute this function before assigning to k_ptr
//
// Uses WRITE_ONCE to assign value to k_ptr. Executes u_ptr_op if
// not NULL.
//
// Returns: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_ulong_u2k_conv_uop(u_ptr: *mut c_ulong, k_ptr: *mut c_ulong) -> c_int {
pub static mut u: c_ulong = 0;
    WRITE_ONCE(*k_ptr, u);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_ulong_u2k_conv(u_ptr: *const c_ulong, k_ptr: *mut c_ulong) -> c_int {
    return proc_ulong_u2k_conv_uop(u_ptr, k_ptr, core::ptr::null_mut());
    }
//
// proc_ulong_k2u_conv_kop - Assign kernel value to a user space pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @k_ptr_op: Operation applied to k_ptr before assignment
//
// Uses READ_ONCE to assign value to u_ptr. Executes k_ptr_op if
// not NULL.
//
// Returns: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_ulong_k2u_conv_kop(u_ptr: *mut c_ulong, k_ptr: *mut c_ulong) -> c_int {
pub static mut val: c_ulong = 0;
// u_ptr = (ulong)val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_ulong_k2u_conv(u_ptr: *mut c_ulong, k_ptr: *const c_ulong) -> c_int {
    return proc_ulong_k2u_conv_kop(u_ptr, k_ptr, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_ulong_conv(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_ulong, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_ulong_conv(u_ptr, k_ptr, dir, tbl, true,
    proc_ulong_u2k_conv, proc_ulong_k2u_conv);
    }
//
// proc_doulongvec_conv - read a vector of unsigned longs with a custom converter
//
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
// @conv: Custom converter call back
//
// Reads/writes up to table->maxlen/sizeof!(unsigned long) unsigned long
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// Returns: 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn proc_doulongvec_conv(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t, negp: *mut *mut int (conv)(bool, u_ptr: *mut c_ulong, k_ptr: *mut c_ulong, dir: c_int) -> c_int {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_ULONG,
    (union proc_vec_conv){ .ulong_conv = conv });
    }
//
// proc_doulongvec_minmax - read a vector of long integers with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned long) unsigned long
// values from/to the user buffer, treated as an ASCII string.
//
// This routine will ensure the values are within the range specified by
// table->extra1 (min) and table->extra2 (max).
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_doulongvec_minmax(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_ULONG,
    (union proc_vec_conv){ .ulong_conv = do_proc_ulong_conv });
    }
//
// proc_dointvec_conv - read a vector of ints with a custom converter
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
// @conv: Custom converter call back. Defaults to do_proc_int_conv
//
// Reads/writes up to table->maxlen/sizeof!(int) integer values from/to the
// user buffer, treated as an ASCII string.
//
// Returns: 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_conv(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t, negp: *mut *mut int (conv)(bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int) -> c_int {
    if (!conv) {
    conv = do_proc_int_conv;
    }
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_INT,
    (union proc_vec_conv){ .int_conv = conv });
    }
//
// proc_do_large_bitmap - read/write from/to a large bitmap
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// The bitmap is stored at table->data and the bitmap length (in bits)
// in table->maxlen.
//
// We use a range comma separated format (e.g. 1,3-4,10-10) so that
// large bitmaps may be represented in a compact manner. Writing into
// the file will clear the bitmap then update it with the given input.
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_do_large_bitmap(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
pub static mut err: c_int = 0;
pub static mut left: usize = 0;
pub static mut bitmap_len: c_ulong = 0;
    let mut bitmap = * table.data;
    let mut tmp_bitmap = core::ptr::null_mut();
    char tr_a[] = { '-', ',', '\n' }, tr_b[] = { ',', '\n', 0 }, c = 0;
    if (!bitmap || !bitmap_len || !left || (*ppos && SYSCTL_KERN_TO_USER(dir))) {
// lenp = 0;
    return 0;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    let mut p = buffer;
pub static mut skipped: usize = 0;
    if (left > PAGE_SIZE - 1) {
    left = PAGE_SIZE - 1;
// How much of the buffer we'll skip this pass
    skipped = *lenp - left;
    }
    tmp_bitmap = bitmap_zalloc(bitmap_len, GFP_KERNEL);
    if (!tmp_bitmap) {
    return -ENOMEM;
    }
    proc_skip_char(&p, &left, '\n');
    while (!err && left) {
    unsigned long val_a, val_b;
    let mut neg = 0;
    let mut saved_left = 0;
// In case we stop parsing mid-number, we can reset
    saved_left = left;
    err = proc_get_long(&p, &left, &val_a, &neg, tr_a,
    sizeof!(tr_a), &c);
//
// If we consumed the entirety of a truncated buffer or
// only one char is left (may be a "-"), then stop here,
// reset, & come back for more.
//
    if ((left <= 1) && skipped) {
    left = saved_left;
    break;
    }
    if (err) {
    break;
    }
    if (val_a >= bitmap_len || neg) {
    err = -EINVAL;
    break;
    }
    val_b = val_a;
    if (left) {
    p += 1;
    left -= 1;
    }
    if (c == '-') {
    err = proc_get_long(&p, &left, &val_b,
    &neg, tr_b, sizeof!(tr_b),
    &c);
//
// If we consumed all of a truncated buffer or
// then stop here, reset, & come back for more.
//
    if (!left && skipped) {
    left = saved_left;
    break;
    }
    if (err) {
    break;
    }
    if (val_b >= bitmap_len || neg ||
    val_a > val_b) {
    err = -EINVAL;
    break;
    }
    if (left) {
    p += 1;
    left -= 1;
    }
    }
    bitmap_set(tmp_bitmap, val_a, val_b - val_a + 1);
    proc_skip_char(&p, &left, '\n');
    }
    left += skipped;
    } else {
    unsigned long bit_a, bit_b = 0;
pub static mut first: bool = 1;
    while (left) {
    bit_a = find_next_bit(bitmap, bitmap_len, bit_b);
    if (bit_a >= bitmap_len) {
    break;
    }
    bit_b = find_next_zero_bit(bitmap, bitmap_len,
    bit_a + 1) - 1;
    if (!first) {
    proc_put_char(&buffer, &left, ',');
    }
    proc_put_long(&buffer, &left, bit_a, false);
    if (bit_a != bit_b) {
    proc_put_char(&buffer, &left, '-');
    proc_put_long(&buffer, &left, bit_b, false);
    }
    first = 0; bit_b += 1;
    }
    proc_put_char(&buffer, &left, '\n');
    }
    if (!err) {
    if (SYSCTL_USER_TO_KERN(dir)) {
    if (*ppos) {
    bitmap_or(bitmap, bitmap, tmp_bitmap, bitmap_len);
    }
    else {
    bitmap_copy(bitmap, tmp_bitmap, bitmap_len);
    }
    }
// lenp -= left;
// ppos += *lenp;
    }
    bitmap_free(tmp_bitmap);
    return err;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: proc_dostring
pub unsafe extern "C" fn proc_dostring_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_dobool
pub unsafe extern "C" fn proc_dobool_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_dointvec
pub unsafe extern "C" fn proc_dointvec_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_douintvec
pub unsafe extern "C" fn proc_douintvec_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_dointvec_minmax
pub unsafe extern "C" fn proc_dointvec_minmax_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_douintvec_minmax
pub unsafe extern "C" fn proc_douintvec_minmax_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_douintvec_conv
pub unsafe extern "C" fn proc_douintvec_conv_dup(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t, negp: *mut *mut int (conv)(bool, lvalp: *mut c_ulong, valp: *mut c_uint, write: c_int) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_uint_k2u_conv
pub unsafe extern "C" fn proc_uint_k2u_conv_dup(u_ptr: *mut c_ulong, k_ptr: *const c_uint) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_uint_u2k_conv_uop
pub unsafe extern "C" fn proc_uint_u2k_conv_uop_dup(u_ptr: *mut c_ulong, k_ptr: *mut c_uint) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_uint_conv
pub unsafe extern "C" fn proc_uint_conv_dup(u_ptr: *mut c_ulong, k_ptr: *mut c_uint, dir: c_int, tbl: *mut ctl_table, k_ptr_range_check: bool, u_ptr: *mut *mut int (user_to_kern)( ulong, u_ptr: *mut *mut int (kern_to_user)(ulong) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_dou8vec_minmax
pub unsafe extern "C" fn proc_dou8vec_minmax_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_doulongvec_minmax
pub unsafe extern "C" fn proc_doulongvec_minmax_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_doulongvec_conv
pub unsafe extern "C" fn proc_doulongvec_conv_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t, negp: *mut *mut int (conv)(bool, u_ptr: *mut c_ulong, k_ptr: *mut c_ulong, dir: c_int) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_dointvec_conv
pub unsafe extern "C" fn proc_dointvec_conv_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t, negp: *mut *mut int (conv)(bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: proc_do_large_bitmap
pub unsafe extern "C" fn proc_do_large_bitmap_dup(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return -ENOSYS;
    }

#[no_mangle]
pub unsafe extern "C" fn proc_do_static_key(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut key = table.data;
// static DEFINE_MUTEX(static_key_mutex);
    let mut val = 0;
    let mut ret = 0;
pub static mut ctl_table: usize = 0;
    if (SYSCTL_USER_TO_KERN(dir) && !capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    mutex_lock(&static_key_mutex);
    val = static_key_enabled(key);
    ret = proc_dointvec_minmax(&tmp, dir, buffer, lenp, ppos);
    if (SYSCTL_USER_TO_KERN(dir) && !ret) {
    if (val) {
    static_key_enable(key);
    }
    else {
    static_key_disable(key);
    }
    }
    mutex_unlock(&static_key_mutex);
    return ret;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn sysctl_init_bases() -> c_int {
    register_sysctl_init("kernel", sysctl_subsys_table);
    return 0;
    }

//
// No sense putting this after each symbol definition, twice,
exception granted :-)
//
    EXPORT_SYMBOL(proc_dobool);
    EXPORT_SYMBOL(proc_dointvec);
    EXPORT_SYMBOL(proc_douintvec);
    EXPORT_SYMBOL(proc_dointvec_minmax);
    EXPORT_SYMBOL_GPL(proc_douintvec_minmax);
    EXPORT_SYMBOL(proc_dostring);
    EXPORT_SYMBOL(proc_doulongvec_minmax);
    EXPORT_SYMBOL(proc_do_large_bitmap);