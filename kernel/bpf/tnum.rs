//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/tnum.c
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


// SPDX-License-Identifier: GPL-2.0-only
// tnum: tracked (or tristate) numbers
//
// A tnum tracks knowledge about the bits of a value.  Each bit can be either
// known (0 or 1), or unknown (x).  Arithmetic operations on tnums will
// propagate the unknown bits such that the tnum result represents all the
// possible results for possible values of the operands.
//

// A completely unknown value
pub static mut tnum_unknown: tnum = 0;
#[no_mangle]
pub unsafe extern "C" fn tnum_const(value: u64) -> tnum {
    return TNUM(value, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_range(min: u64, max: u64) -> tnum {
pub static mut chi: u64 = 0;
pub static mut bits: u8 = 0;
// special case, needed because 1ULL << 64 is undefined
    if (bits > 63) {
    return tnum_unknown;
    }
// e.g. if chi = 4, bits = 3, delta = (1<<3) - 1 = 7.
// if chi = 0, bits = 0, delta = (1<<0) - 1 = 0, so we return
// constant min (since min == max).
//
    delta = (1ULL << bits) - 1;
    return TNUM(min & ~delta, delta);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_lshift(a: tnum, shift: u8) -> tnum {
    return TNUM(a.value << shift, a.mask << shift);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_rshift(a: tnum, shift: u8) -> tnum {
    return TNUM(a.value >> shift, a.mask >> shift);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_arshift(a: tnum, min_shift: u8, insn_bitness: u8) -> tnum {
// if a.value is negative, arithmetic shifting by minimum shift
// will have larger negative offset compared to more shifting.
// If a.value is nonnegative, arithmetic shifting by minimum shift
// will have larger positive offset compare to more shifting.
//
    if (insn_bitness == 32) {
    return TNUM((u32)(((s32)a.value) >> min_shift),
    (u32)(((s32)a.mask)  >> min_shift));
    }
    else {
    return TNUM((s64)a.value >> min_shift,
    (s64)a.mask  >> min_shift);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_add(a: tnum, b: tnum) -> tnum {
    u64 sm, sv, sigma, chi, mu;
    sm = a.mask + b.mask;
    sv = a.value + b.value;
    sigma = sm + sv;
    chi = sigma ^ sv;
    mu = chi | a.mask | b.mask;
    return TNUM(sv & ~mu, mu);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_sub(a: tnum, b: tnum) -> tnum {
    u64 dv, alpha, beta, chi, mu;
    dv = a.value - b.value;
    alpha = dv + a.mask;
    beta = dv - b.mask;
    chi = alpha ^ beta;
    mu = chi | a.mask | b.mask;
    return TNUM(dv & ~mu, mu);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_neg(a: tnum) -> tnum {
    return tnum_sub(TNUM(0, 0), a);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_and(a: tnum, b: tnum) -> tnum {
    u64 alpha, beta, v;
    alpha = a.value | a.mask;
    beta = b.value | b.mask;
    v = a.value & b.value;
    return TNUM(v, alpha & beta & ~v);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_or(a: tnum, b: tnum) -> tnum {
    u64 v, mu;
    v = a.value | b.value;
    mu = a.mask | b.mask;
    return TNUM(v, mu & ~v);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_xor(a: tnum, b: tnum) -> tnum {
    u64 v, mu;
    v = a.value ^ b.value;
    mu = a.mask | b.mask;
    return TNUM(v & ~mu, mu);
    }
// Perform long multiplication, iterating through the bits in a using rshift:
// - if LSB(a) is a known 0, keep current accumulator
// - if LSB(a) is a known 1, add b to current accumulator
// - if LSB(a) is unknown, take a union of the above cases.
//
// For example:
//
// acc_0:        acc_1:
//
// 11 *  ->      11 *  ->      11 *  -> union(0011, 1001) == x0x1
// x1            01            11
// ------        ------        ------
// 11            11            11
// xx            00            11
// ------        ------        ------
// ????          0011          1001
//
#[no_mangle]
pub unsafe extern "C" fn tnum_mul(a: tnum, b: tnum) -> tnum {
pub static mut acc: tnum = 0;
    while (a.value || a.mask) {
// LSB of tnum a is a certain 1
    if (a.value & 1) {
    acc = tnum_add(acc, b);
    }
// LSB of tnum a is uncertain
if true {
// acc = tnum_union(acc_0, acc_1), where acc_0 and
// acc_1 are partial accumulators for cases
// LSB(a) = certain 0 and LSB(a) = certain 1.
// acc_0 = acc + 0 * b = acc.
// acc_1 = acc + 1 * b = tnum_add(acc, b).
//
    acc = tnum_union(acc, tnum_add(acc, b));
    }
// Note: no case for LSB is certain 0
    a = tnum_rshift(a, 1);
    b = tnum_lshift(b, 1);
    }
    return acc;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_overlap(a: tnum, b: tnum) -> bool {
    let mut mu = 0;
    mu = ~a.mask & ~b.mask;
    return (a.value & mu) == (b.value & mu);
    }
// Note that if a and b disagree - i.e. one has a 'known 1' where the other has
// a 'known 0' - this will return a 'known 1' for that bit.
//
#[no_mangle]
pub unsafe extern "C" fn tnum_intersect(a: tnum, b: tnum) -> tnum {
    u64 v, mu;
    v = a.value | b.value;
    mu = a.mask & b.mask;
    return TNUM(v & ~mu, mu);
    }
// Returns a tnum with the uncertainty from both a and b, and in addition, new
// uncertainty at any position that a and b disagree. This represents a
// superset of the union of the concrete sets of both a and b. Despite the
// overapproximation, it is optimal.
//
#[no_mangle]
pub unsafe extern "C" fn tnum_union(a: tnum, b: tnum) -> tnum {
pub static mut v: u64 = 0;
pub static mut mu: u64 = 0;
    return TNUM(v & ~mu, mu);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_cast(a: tnum, size: u8) -> tnum {
    a.value &= (1ULL << (size * 8)) - 1;
    a.mask &= (1ULL << (size * 8)) - 1;
    return a;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_is_aligned(a: tnum, size: u64) -> bool {
    if (!size) {
    return true;
    }
    return !((a.value | a.mask) & (size - 1));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_in(a: tnum, b: tnum) -> bool {
    if (b.mask & ~a.mask) {
    return false;
    }
    b.value &= ~a.mask;
    return a.value == b.value;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_sbin(str: *mut c_char, size: usize, a: tnum) -> c_int {
    let mut n = 0;
    while (n) {
    if (n < size) {
    if (a.mask & 1) {
    str[n - 1] = 'x';
    }

    else if (a.value & 1) {
    str[n - 1] = '1';
    }
    else {
    str[n - 1] = '0';
    }
    }
    a.mask >>= 1;
    a.value >>= 1;
    }
    str[min(size - 1, (size_t)64)] = 0;
    return 64;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_subreg(a: tnum) -> tnum {
    return tnum_cast(a, 4);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_clear_subreg(a: tnum) -> tnum {
    return tnum_lshift(tnum_rshift(a, 32), 32);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_with_subreg(reg: tnum, subreg: tnum) -> tnum {
    return tnum_or(tnum_clear_subreg(reg), tnum_subreg(subreg));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_const_subreg(a: tnum, value: u32) -> tnum {
    return tnum_with_subreg(a, tnum_const(value));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_bswap16(a: tnum) -> tnum {
    return TNUM(swab16(a.value & 0xFFFF), swab16(a.mask & 0xFFFF));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_bswap32(a: tnum) -> tnum {
    return TNUM(swab32(a.value & 0xFFFFFFFF), swab32(a.mask & 0xFFFFFFFF));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_bswap64(a: tnum) -> tnum {
    return TNUM(swab64(a.value), swab64(a.mask));
    }
// Given tnum t, and a number z such that tmin <= z < tmax, where tmin
// is the smallest member of the t (= t.value) and tmax is the largest
// member of t (= t.value | t.mask), returns the smallest member of t
// larger than z.
//
// For example,
// t      = x11100x0
// z      = 11110001 (241)
// result = 11110010 (242)
//
// Note: if this function is called with z >= tmax, it just returns
// early with tmax; if this function is called with z < tmin, the
// algorithm already returns tmin.
//
#[no_mangle]
pub unsafe extern "C" fn tnum_step(t: tnum, z: u64) -> u64 {
    u64 tmax, d, carry_mask, filled, inc;
    tmax = t.value | t.mask;
// if z >= largest member of t, return largest member of t
    if (z >= tmax) {
    return tmax;
    }
// if z < smallest member of t, return smallest member of t
    if (z < t.value) {
    return t.value;
    }
//
// Let r be the result tnum member, z = t.value + d.
// Every tnum member is t.value | s for some submask s of t.mask,
// and since t.value & t.mask == 0, t.value | s == t.value + s.
// So r > z becomes s > d where d = z - t.value.
//
// Find the smallest submask s of t.mask greater than d by
// "incrementing d within the mask": fill every non-mask
// position with 1 (`filled`) so +1 ripples through the gaps,
// then keep only mask bits. `carry_mask` additionally fills
// positions below the highest non-mask 1 in d, preventing
// it from trapping the carry.
//
    d = z - t.value;
    carry_mask = (1ULL << fls64(d & ~t.mask)) - 1;
    filled = d | carry_mask | ~t.mask;
    inc = (filled + 1) & t.mask;
    return t.value | inc;
    }