//! Automatically rewritten from C to Rust
//! Source: mm/util.c
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

//
// kfree_const - conditionally free memory
// @x: pointer to the memory
//
// Function calls kfree only if @x is not in .rodata section.
//
#[no_mangle]
pub unsafe extern "C" fn kfree_const(x: *const c_void) {
    if (!is_kernel_rodata((unsigned long)x)) {
    kfree(x);
    }
    }
    EXPORT_SYMBOL(kfree_const);
//
// __kmemdup_nul - Create a NUL-terminated string from @s, which might be unterminated.
// @s: The data to copy
// @len: The size of the data, not including the NUL terminator
// @gfp: the GFP mask used in the kmalloc() call when allocating memory
//
// Return: newly allocated copy of @s with NUL-termination or %NULL in
// case of error
//
    static __always_inline char *__kmemdup_nul(const char *s, size_t len, gfp_t gfp)
    {
pub static mut buf: *mut c_void = core::ptr::null_mut();
// '+1' for the NUL terminator
    buf = kmalloc_track_caller(len + 1, gfp);
    if (!buf) {
    return core::ptr::null_mut();
    }
    memcpy(buf, s, len);
// Ensure the buf is always NUL-terminated, regardless of @s.
    buf[len] = '\0';
    return buf;
    }
//
// kstrdup - allocate space for and copy an existing string
// @s: the string to duplicate
// @gfp: the GFP mask used in the kmalloc() call when allocating memory
//
// Return: newly allocated copy of @s or %NULL in case of error
//
    noinline
#[no_mangle]
pub unsafe extern "C" fn kstrdup(s: *mut c_char, gfp: gfp_t) -> *mut c_void {
    return s ? __kmemdup_nul(s, strlen(s), gfp) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL(kstrdup);
//
// kstrdup_const - conditionally duplicate an existing const string
// @s: the string to duplicate
// @gfp: the GFP mask used in the kmalloc() call when allocating memory
//
// Note: Strings allocated by kstrdup_const should be freed by kfree_const and
// must not be passed to krealloc().
//
// Return: source string if it is in .rodata section otherwise
// fallback to kstrdup.
//
    const char *kstrdup_const(const char *s, gfp_t gfp)
    {
    if (is_kernel_rodata((unsigned long)s)) {
    return s;
    }
    return kstrdup(s, gfp);
    }
    EXPORT_SYMBOL(kstrdup_const);
//
// kstrndup - allocate space for and copy an existing string
// @s: the string to duplicate
// @max: read at most @max chars from @s
// @gfp: the GFP mask used in the kmalloc() call when allocating memory
//
// Note: Use kmemdup_nul() instead if the size is known exactly.
//
// Return: newly allocated copy of @s or %NULL in case of error
//
#[no_mangle]
pub unsafe extern "C" fn kstrndup(s: *mut c_char, max: size_t, gfp: gfp_t) -> *mut c_void {
    return s ? __kmemdup_nul(s, strnlen(s, max), gfp) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL(kstrndup);
//
// kmemdup - duplicate region of memory
//
// @src: memory region to duplicate
// @len: memory region length
// @gfp: GFP mask to use
//
// Return: newly allocated copy of @src or %NULL in case of error,
// result is physically contiguous. Use kfree() to free.
//
#[no_mangle]
pub unsafe extern "C" fn kmemdup_noprof(src: *mut c_void, len: size_t, gfp: gfp_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kmalloc_node_track_caller_noprof(len, gfp, NUMA_NO_NODE, _RET_IP_);
    if (p) {
    memcpy(p, src, len);
    }
    return p;
    }
    EXPORT_SYMBOL(kmemdup_noprof);
//
// kmemdup_array - duplicate a given array.
//
// @src: array to duplicate.
// @count: number of elements to duplicate from array.
// @element_size: size of each element of array.
// @gfp: GFP mask to use.
//
// Return: duplicated array of @src or %NULL in case of error,
// result is physically contiguous. Use kfree() to free.
//
#[no_mangle]
pub unsafe extern "C" fn kmemdup_array(src: *mut c_void, count: size_t, element_size: size_t, gfp: gfp_t) -> *mut c_void {
    return kmemdup(src, size_mul(element_size, count), gfp);
    }
    EXPORT_SYMBOL(kmemdup_array);
//
// kvmemdup - duplicate region of memory
//
// @src: memory region to duplicate
// @len: memory region length
// @gfp: GFP mask to use
//
// Return: newly allocated copy of @src or %NULL in case of error,
// result may be not physically contiguous. Use kvfree() to free.
//
#[no_mangle]
pub unsafe extern "C" fn kvmemdup(src: *mut c_void, len: size_t, gfp: gfp_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kvmalloc(len, gfp);
    if (p) {
    memcpy(p, src, len);
    }
    return p;
    }
    EXPORT_SYMBOL(kvmemdup);
//
// kmemdup_nul - Create a NUL-terminated string from unterminated data
// @s: The data to stringify
// @len: The size of the data
// @gfp: the GFP mask used in the kmalloc() call when allocating memory
//
// Return: newly allocated copy of @s with NUL-termination or %NULL in
// case of error
//
#[no_mangle]
pub unsafe extern "C" fn kmemdup_nul(s: *mut c_char, len: size_t, gfp: gfp_t) -> *mut c_void {
    return s ? __kmemdup_nul(s, len, gfp) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL(kmemdup_nul);
pub static mut user_buckets: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn init_user_buckets() -> c_int {
    user_buckets = kmem_buckets_create("memdup_user", 0, 0, INT_MAX, core::ptr::null_mut());
    return 0;
    }
    subsys_initcall!(init_user_buckets);
//
// memdup_user - duplicate memory region from user space
//
// @src: source address in user space
// @len: number of bytes to copy
//
// Return: an ERR_PTR() on failure.  Result is physically
// contiguous, to be freed by kfree().
//
#[no_mangle]
pub unsafe extern "C" fn memdup_user(src: *mut c_void, len: size_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kmem_buckets_alloc_track_caller(user_buckets, len, GFP_USER | __GFP_NOWARN);
    if (!p) {
    return ERR_PTR(-ENOMEM);
    }
    if (copy_from_user(p, src, len)) {
    kfree(p);
    return ERR_PTR(-EFAULT);
    }
    return p;
    }
    EXPORT_SYMBOL(memdup_user);
//
// vmemdup_user - duplicate memory region from user space
//
// @src: source address in user space
// @len: number of bytes to copy
//
// Return: an ERR_PTR() on failure.  Result may be not
// physically contiguous.  Use kvfree() to free.
//
#[no_mangle]
pub unsafe extern "C" fn vmemdup_user(src: *mut c_void, len: size_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kmem_buckets_valloc(user_buckets, len, GFP_USER);
    if (!p) {
    return ERR_PTR(-ENOMEM);
    }
    if (copy_from_user(p, src, len)) {
    kvfree(p);
    return ERR_PTR(-EFAULT);
    }
    return p;
    }
    EXPORT_SYMBOL(vmemdup_user);
//
// strndup_user - duplicate an existing string from user space
// @s: The string to duplicate
// @n: Maximum number of bytes to copy, including the trailing NUL.
//
// Return: newly allocated copy of @s or an ERR_PTR() in case of error
//
#[no_mangle]
pub unsafe extern "C" fn strndup_user(s: *mut c_char, n: c_long) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut length = 0;
    length = strnlen_user(s, n);
    if (!length) {
    return ERR_PTR(-EFAULT);
    }
    if (length > n) {
    return ERR_PTR(-EINVAL);
    }
    p = memdup_user(s, length);
    if (IS_ERR(p)) {
    return p;
    }
    p[length - 1] = '\0';
    return p;
    }
    EXPORT_SYMBOL(strndup_user);
//
// memdup_user_nul - duplicate memory region from user space and NUL-terminate
//
// @src: source address in user space
// @len: number of bytes to copy
//
// Return: an ERR_PTR() on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memdup_user_nul(src: *mut c_void, len: size_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kmem_buckets_alloc_track_caller(user_buckets, len + 1, GFP_USER | __GFP_NOWARN);
    if (!p) {
    return ERR_PTR(-ENOMEM);
    }
    if (copy_from_user(p, src, len)) {
    kfree(p);
    return ERR_PTR(-EFAULT);
    }
    p[len] = '\0';
    return p;
    }
    EXPORT_SYMBOL(memdup_user_nul);
// Check if the vma is being used as a stack by this task
#[no_mangle]
pub unsafe extern "C" fn vma_is_stack_for_current(vma: *const vm_area_struct) -> c_int {
pub static mut t: *mut task_ __maybe_unused = core::ptr::null_mut();
    return (vma.vm_start <= KSTK_ESP(t) && vma.vm_end >= KSTK_ESP(t));
    }
//
// Change backing file, only valid to use during initial VMA setup.
//
#[no_mangle]
pub unsafe extern "C" fn vma_set_file(vma: *mut vm_area_struct, file: *mut file) {
// Changing an anonymous vma with this is illegal
    get_file(file);
    swap(vma.vm_file, file);
    fput(file);
    }
    EXPORT_SYMBOL(vma_set_file);

#[no_mangle]
pub unsafe extern "C" fn randomize_stack_top(stack_top: c_ulong) -> c_ulong {
pub static mut random_variable: c_ulong = 0;
    if (current.flags & PF_RANDOMIZE) {
    random_variable = get_random_long();
    random_variable &= STACK_RND_MASK;
    random_variable <<= PAGE_SHIFT;
    }

    return PAGE_ALIGN(stack_top) + random_variable;

    return PAGE_ALIGN(stack_top) - random_variable;

    }
//
// randomize_page - Generate a random, page aligned address
// @start:	The smallest acceptable address the caller will take.
// @range:	The size of the area, starting at @start, within which the
// random address must fall.
//
// If @start + @range would overflow, @range is capped.
//
// NOTE: Historical use of randomize_range, which this replaces, presumed that
// @start was already page aligned.  We now align it regardless.
//
// Return: A page aligned address within [start, start + range).  On error,
// @start is returned.
//
#[no_mangle]
pub unsafe extern "C" fn randomize_page(start: c_ulong, range: c_ulong) -> c_ulong {
    if (!PAGE_ALIGNED(start)) {
    range -= PAGE_ALIGN(start) - start;
    start = PAGE_ALIGN(start);
    }
    if (start > ULONG_MAX - range) {
    range = ULONG_MAX - start;
    }
    range >>= PAGE_SHIFT;
    if (range == 0) {
    return start;
    }
    return start + (get_random_long() % range << PAGE_SHIFT);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_randomize_brk(mm: *mut mm_struct) -> unsigned long __weak {
// Is the current task 32bit ?
    if (!IS_ENABLED!(CONFIG_64BIT) || is_compat_task()) {
    return randomize_page(mm.brk, SZ_32M);
    }
    return randomize_page(mm.brk, SZ_1G);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_mmap_rnd() -> c_ulong {
    let mut rnd = 0;

    if (is_compat_task()) {
    rnd = get_random_long() & ((1UL << mmap_rnd_compat_bits) - 1);
    }
    else {

    rnd = get_random_long() & ((1UL << mmap_rnd_bits) - 1);
    }
    return rnd << PAGE_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn mmap_is_legacy(rlim_stack: *const rlimit) -> c_int {
    if (current.personality & ADDR_COMPAT_LAYOUT) {
    return 1;
    }
// On parisc the stack always grows up - so a unlimited stack should
// not be an indicator to use the legacy memory layout.
    if (rlim_stack.rlim_cur == RLIM_INFINITY &&
    !IS_ENABLED!(CONFIG_STACK_GROWSUP)) {
    return 1;
    }
    return sysctl_legacy_va_layout;
    }
//
// Leave enough space between the mmap area and the stack to honour ulimit in
// the face of randomisation.
//

#[no_mangle]
unsafe extern "C" fn mmap_base(rnd: c_ulong, rlim_stack: *const rlimit) -> c_ulong {

//
// For an upwards growing stack the calculation is much simpler.
// Memory for the maximum stack size is reserved at the top of the
// task. mmap_base starts directly below the stack and grows
// downwards.
//
    return PAGE_ALIGN_DOWN(mmap_upper_limit(rlim_stack) - rnd);

pub static mut gap: c_ulong = 0;
pub static mut pad: c_ulong = 0;
// Account for stack randomization if necessary
    if (current.flags & PF_RANDOMIZE) {
    pad += (STACK_RND_MASK << PAGE_SHIFT);
    }
// Values close to RLIM_INFINITY can overflow.
    if (gap + pad > gap) {
    gap += pad;
    }
    if (gap < MIN_GAP && MIN_GAP < MAX_GAP) {
    gap = MIN_GAP;
    }

    else if (gap > MAX_GAP) {
    gap = MAX_GAP;
    }
    return PAGE_ALIGN(STACK_TOP - gap - rnd);

    }
#[no_mangle]
pub unsafe extern "C" fn arch_pick_mmap_layout(mm: *mut mm_struct, rlim_stack: *const rlimit) {
pub static mut random_factor: c_ulong = 0;
    if (current.flags & PF_RANDOMIZE) {
    random_factor = arch_mmap_rnd();
    }
    if (mmap_is_legacy(rlim_stack)) {
    mm.mmap_base = TASK_UNMAPPED_BASE + random_factor;
    mm_flags_clear(MMF_TOPDOWN, mm);
    } else {
    mm.mmap_base = mmap_base(random_factor, rlim_stack);
    mm_flags_set(MMF_TOPDOWN, mm);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: arch_pick_mmap_layout
pub unsafe extern "C" fn arch_pick_mmap_layout_dup(mm: *mut mm_struct, rlim_stack: *const rlimit) {
    mm.mmap_base = TASK_UNMAPPED_BASE;
    mm_flags_clear(MMF_TOPDOWN, mm);
    }

    EXPORT_SYMBOL_IF_KUNIT(arch_pick_mmap_layout);

//
// __account_locked_vm - account locked pages to an mm's locked_vm
// @mm:          mm to account against
// @pages:       number of pages to account
// @inc:         %true if @pages should be considered positive, %false if not
// @task:        task used to check RLIMIT_MEMLOCK
// @bypass_rlim: %true if checking RLIMIT_MEMLOCK should be skipped
//
// Assumes @task and @mm are valid (i.e. at least one reference on each), and
// that mmap_lock is held as writer.
//
// Return:
// * 0       on success
// * -ENOMEM if RLIMIT_MEMLOCK would be exceeded.
//
#[no_mangle]
pub unsafe extern "C" fn __account_locked_vm(mm: *mut mm_struct, pages: c_ulong, inc: bool, task: *mut task_struct, bypass_rlim: bool) -> c_int {
    unsigned long locked_vm, limit;
pub static mut ret: c_int = 0;
    mmap_assert_write_locked(mm);
    locked_vm = mm.locked_vm;
    if (inc) {
    if (!bypass_rlim) {
    limit = task_rlimit(task, RLIMIT_MEMLOCK) >> PAGE_SHIFT;
    if (locked_vm + pages > limit) {
    ret = -ENOMEM;
    }
    }
    if (!ret) {
    mm.locked_vm = locked_vm + pages;
    }
    } else {
    WARN_ON_ONCE!(pages > locked_vm);
    mm.locked_vm = locked_vm - pages;
    }
    pr_debug!("%s: [%d] caller %ps %c%lu %lu/%lu%s\n", __func__, task.pid,
    _RET_IP_, (inc) ? '+' : '-', pages << PAGE_SHIFT,
    locked_vm << PAGE_SHIFT, task_rlimit(task, RLIMIT_MEMLOCK),
    ret ? " - exceeded" : "");
    return ret;
    }
    EXPORT_SYMBOL_GPL(__account_locked_vm);
//
// account_locked_vm - account locked pages to an mm's locked_vm
// @mm:          mm to account against, may be NULL
// @pages:       number of pages to account
// @inc:         %true if @pages should be considered positive, %false if not
//
// Assumes a non-NULL @mm is valid (i.e. at least one reference on it).
//
// Return:
// * 0       on success, or if mm is NULL
// * -ENOMEM if RLIMIT_MEMLOCK would be exceeded.
//
#[no_mangle]
pub unsafe extern "C" fn account_locked_vm(mm: *mut mm_struct, pages: c_ulong, inc: bool) -> c_int {
    let mut ret = 0;
    if (pages == 0 || !mm) {
    return 0;
    }
    mmap_write_lock(mm);
    ret = __account_locked_vm(mm, pages, inc, current,
    capable(CAP_IPC_LOCK));
    mmap_write_unlock(mm);
    return ret;
    }
    EXPORT_SYMBOL_GPL(account_locked_vm);
#[no_mangle]
pub unsafe extern "C" fn vm_mmap_pgoff(file: *mut file, addr: c_ulong, len: c_ulong, prot: c_ulong, flag: c_ulong, pgoff: c_ulong) -> c_ulong {
pub static mut off: loff_t = 0;
    let mut ret = 0;
    let mut mm = current.mm;
    let mut populate = 0;
pub static mut uf: usize = 0;
    ret = security_mmap_file(file, prot, flag);
    if (!ret) {
    ret = fsnotify_mmap_perm(file, prot, off, len);
    }
    if (!ret) {
    if (mmap_write_lock_killable(mm)) {
    return -EINTR;
    }
    ret = do_mmap(file, addr, len, prot, flag, EMPTY_VMA_FLAGS, pgoff,
    &populate, &uf);
    mmap_write_unlock(mm);
    userfaultfd_unmap_complete(mm, &uf);
    if (populate) {
    mm_populate(ret, populate);
    }
    }
    return ret;
    }
//
// Perform a userland memory mapping into the current process address space. See
// the comment for do_mmap() for more details on this operation in general.
//
// This differs from do_mmap() in that:
//
// a. An offset parameter is provided rather than pgoff, which is both checked
// for overflow and page alignment.
// b. mmap locking is performed on the caller's behalf.
// c. Userfaultfd unmap events and memory population are handled.
//
// This means that this function performs essentially the same work as if
// userland were invoking mmap (2).
//
// Returns either an error, or the address at which the requested mapping has
// been performed.
//
#[no_mangle]
pub unsafe extern "C" fn vm_mmap(file: *mut file, addr: c_ulong, len: c_ulong, prot: c_ulong, flag: c_ulong, offset: c_ulong) -> c_ulong {
    if (unlikely(offset + PAGE_ALIGN(len) < offset)) {
    return -EINVAL;
    }
    if (unlikely(offset_in_page(offset))) {
    return -EINVAL;
    }
    return vm_mmap_pgoff(file, addr, len, prot, flag, offset >> PAGE_SHIFT);
    }
    EXPORT_SYMBOL(vm_mmap);

//
// Perform a userland memory mapping for a shadow stack into the current
// process address space. This is intended to be used by architectures that
// support user shadow stacks.
//
#[no_mangle]
pub unsafe extern "C" fn vm_mmap_shadow_stack(addr: c_ulong, len: c_ulong, flags: c_ulong) -> c_ulong {
pub static mut vma_flags: vma_flags_t = 0;
    let mut mm = current.mm;
    unsigned long ret, unused;
    flags |= MAP_ANONYMOUS | MAP_PRIVATE;
    if (addr) {
    flags |= MAP_FIXED_NOREPLACE;
    }
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE)) {
    vma_flags_set(&vma_flags, VMA_NOHUGEPAGE_BIT);
    }
    mmap_write_lock(mm);
    ret = do_mmap(core::ptr::null_mut(), addr, len, PROT_READ | PROT_WRITE, flags,
    vma_flags, 0, &unused, core::ptr::null_mut());
    mmap_write_unlock(mm);
    return ret;
    }

//
// __vmalloc_array - allocate memory for a virtually contiguous array.
// @n: number of elements.
// @size: element size.
// @flags: the type of memory to allocate (see kmalloc).
//
#[no_mangle]
pub unsafe extern "C" fn __vmalloc_array_noprof(n: size_t, size: size_t, flags: gfp_t) -> *mut c_void {
    let mut bytes = 0;
    if (unlikely(check_mul_overflow(n, size, &bytes))) {
    return core::ptr::null_mut();
    }
    return __vmalloc_noprof(bytes, flags);
    }
    EXPORT_SYMBOL(__vmalloc_array_noprof);
//
// vmalloc_array - allocate memory for a virtually contiguous array.
// @n: number of elements.
// @size: element size.
//
#[no_mangle]
pub unsafe extern "C" fn vmalloc_array_noprof(n: size_t, size: size_t) -> *mut c_void {
    return __vmalloc_array_noprof(n, size, GFP_KERNEL);
    }
    EXPORT_SYMBOL(vmalloc_array_noprof);
//
// __vcalloc - allocate and zero memory for a virtually contiguous array.
// @n: number of elements.
// @size: element size.
// @flags: the type of memory to allocate (see kmalloc).
//
#[no_mangle]
pub unsafe extern "C" fn __vcalloc_noprof(n: size_t, size: size_t, flags: gfp_t) -> *mut c_void {
    return __vmalloc_array_noprof(n, size, flags | __GFP_ZERO);
    }
    EXPORT_SYMBOL(__vcalloc_noprof);
//
// vcalloc - allocate and zero memory for a virtually contiguous array.
// @n: number of elements.
// @size: element size.
//
#[no_mangle]
pub unsafe extern "C" fn vcalloc_noprof(n: size_t, size: size_t) -> *mut c_void {
    return __vmalloc_array_noprof(n, size, GFP_KERNEL | __GFP_ZERO);
    }
    EXPORT_SYMBOL(vcalloc_noprof);
#[no_mangle]
pub unsafe extern "C" fn folio_anon_vma(folio: *mut folio) -> *mut c_void {
pub static mut mapping: c_ulong = 0;
    if ((mapping & FOLIO_MAPPING_FLAGS) != FOLIO_MAPPING_ANON) {
    return core::ptr::null_mut();
    }
    return (mapping - FOLIO_MAPPING_ANON);
    }
//
// folio_mapping - Find the mapping where this folio is stored.
// @folio: The folio.
//
// For folios which are in the page cache, return the mapping that this
// page belongs to.  Folios in the swap cache return the swap mapping
// this page is stored in (which is different from the mapping for the
// swap file or swap device where the data is stored).
//
// You can call this for folios which aren't in the swap cache or page
// cache and it will return NULL.
//
#[no_mangle]
pub unsafe extern "C" fn folio_mapping(folio: *mut folio) -> *mut c_void {
pub static mut mapping: *mut c_void = core::ptr::null_mut();
// This happens if someone calls flush_dcache_page on slab page
    if (unlikely(folio_test_slab(folio))) {
    return core::ptr::null_mut();
    }
    if (unlikely(folio_test_swapcache(folio))) {
    return swap_address_space(folio.swap);
    }
    mapping = folio.mapping;
    if ((unsigned long)mapping & FOLIO_MAPPING_FLAGS) {
    return core::ptr::null_mut();
    }
    return mapping;
    }
    EXPORT_SYMBOL(folio_mapping);
//
// folio_copy - Copy the contents of one folio to another.
// @dst: Folio to copy to.
// @src: Folio to copy from.
//
// The bytes in the folio represented by @src are copied to @dst.
// Assumes the caller has validated that @dst is at least as large as @src.
// Can be called in atomic context for order-0 folios, but if the folio is
// larger, it may sleep.
//
#[no_mangle]
pub unsafe extern "C" fn folio_copy(dst: *mut folio, src: *mut folio) {
pub static mut i: c_long = 0;
pub static mut nr: c_long = 0;
    for (;;) {
    copy_highpage(folio_page(dst, i), folio_page(src, i));
    if (++i == nr) {
    break;
    }
    cond_resched();
    }
    }
    EXPORT_SYMBOL(folio_copy);
#[no_mangle]
pub unsafe extern "C" fn folio_mc_copy(dst: *mut folio, src: *mut folio) -> c_int {
pub static mut nr: c_long = 0;
pub static mut i: c_long = 0;
    for (;;) {
    if (copy_mc_highpage(folio_page(dst, i), folio_page(src, i))) {
    return -EHWPOISON;
    }
    if (++i == nr) {
    break;
    }
    cond_resched();
    }
    return 0;
    }
    EXPORT_SYMBOL(folio_mc_copy);
pub static mut : int sysctl_overcommit_memory = 0;
pub static mut : int sysctl_overcommit_ratio = 50;
    static unsigned long sysctl_overcommit_kbytes ;
pub static mut : int sysctl_max_map_count = 0;
    let mut sysctl_user_reserve_kbytes = 1UL << 17; /* 128MB */
    let mut sysctl_admin_reserve_kbytes = 1UL << 13; /* 8MB */

#[no_mangle]
pub unsafe extern "C" fn overcommit_ratio_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut ret = 0;
    ret = proc_dointvec(table, write, buffer, lenp, ppos);
    if (ret == 0 && write) {
    sysctl_overcommit_kbytes = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sync_overcommit_as(dummy: *mut work_struct) {
    percpu_counter_sync(&vm_committed_as);
    }
#[no_mangle]
pub unsafe extern "C" fn overcommit_policy_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
pub static mut t: usize = 0;
pub static mut new_policy: c_int = 0;
    let mut ret = 0;
//
// The deviation of sync_overcommit_as could be big with loose policy
// like OVERCOMMIT_ALWAYS/OVERCOMMIT_GUESS. When changing policy to
// strict OVERCOMMIT_NEVER, we need to reduce the deviation to comply
// with the strict "NEVER", and to avoid possible race condition (even
// though user usually won't too frequently do the switching to policy
// OVERCOMMIT_NEVER), the switch is done in the following order:
// 1. changing the batch
// 2. sync percpu count on each CPU
// 3. switch the policy
//
    if (write) {
    t = *table;
    t.data = &new_policy;
    ret = proc_dointvec_minmax(&t, write, buffer, lenp, ppos);
    if (ret || new_policy == -1) {
    return ret;
    }
    mm_compute_batch(new_policy);
    if (new_policy == OVERCOMMIT_NEVER) {
    schedule_on_each_cpu(sync_overcommit_as);
    }
    sysctl_overcommit_memory = new_policy;
    } else {
    ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn overcommit_kbytes_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut ret = 0;
    ret = proc_doulongvec_minmax(table, write, buffer, lenp, ppos);
    if (ret == 0 && write) {
    sysctl_overcommit_ratio = 0;
    }
    return ret;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_vm_util_sysctls() -> c_int {
    register_sysctl_init("vm", util_sysctl_table);
    return 0;
    }
    subsys_initcall!(init_vm_util_sysctls);

//
// Committed memory limit enforced when OVERCOMMIT_NEVER policy is used
//
#[no_mangle]
pub unsafe extern "C" fn vm_commit_limit() -> c_ulong {
    let mut allowed = 0;
    if (sysctl_overcommit_kbytes) {
    allowed = sysctl_overcommit_kbytes >> (PAGE_SHIFT - 10);
    }
    else {
    allowed = ((totalram_pages() - hugetlb_total_pages())
// sysctl_overcommit_ratio / 100);
    }
    allowed += total_swap_pages;
    return allowed;
    }
//
// Make sure vm_committed_as in one cacheline and not cacheline shared with
// other variables. It can be updated by several CPUs frequently.
//
    struct percpu_counter vm_committed_as ____cacheline_aligned_in_smp;
//
// The global memory commitment made in the system can be a metric
// that can be used to drive ballooning decisions when Linux is hosted
// as a guest. On Hyper-V, the host implements a policy engine for dynamically
// balancing memory across competing virtual machines that are hosted.
// Several metrics drive this policy engine including the guest reported
// memory commitment.
//
// The time cost of this is very low for small platforms, and for big
// platform like a 2S/36C/72T Skylake server, in worst case where
// vm_committed_as's spinlock is under severe contention, the time cost
// could be about 30~40 microseconds.
//
#[no_mangle]
pub unsafe extern "C" fn vm_memory_committed() -> c_ulong {
    return percpu_counter_sum_positive(&vm_committed_as);
    }
    EXPORT_SYMBOL_GPL(vm_memory_committed);
//
// Check that a process has enough memory to allocate a new virtual
// mapping. 0 means there is enough memory for the allocation to
// succeed and -ENOMEM implies there is not.
//
// We currently support three overcommit policies, which are set via the
// vm.overcommit_memory sysctl.  See Documentation/mm/overcommit-accounting.rst
//
// Strict overcommit modes added 2002 Feb 26 by Alan Cox.
// Additional code 2002 Jul 20 by Robert Love.
//
// cap_sys_admin is 1 if the process has admin privileges, 0 otherwise.
//
// Note this is a helper function intended to be used by LSMs which
// wish to use this logic.
//
#[no_mangle]
pub unsafe extern "C" fn __vm_enough_memory(mm: *const mm_struct, pages: c_long, cap_sys_admin: c_int) -> c_int {
    let mut allowed = 0;
    let mut bytes_failed = 0;
    vm_acct_memory(pages);
//
// Sometimes we want to use more memory than we have
//
    if (sysctl_overcommit_memory == OVERCOMMIT_ALWAYS) {
    return 0;
    }
    if (sysctl_overcommit_memory == OVERCOMMIT_GUESS) {
    if (pages > totalram_pages() + total_swap_pages) {
// goto;
    }
    return 0;
    }
    allowed = vm_commit_limit();
//
// Reserve some for root
//
    if (!cap_sys_admin) {
    allowed -= sysctl_admin_reserve_kbytes >> (PAGE_SHIFT - 10);
    }
//
// Don't let a single process grow so big a user can't recover
//
    if (mm) {
pub static mut reserve: c_long = 0;
    allowed -= min_t(long, mm.total_vm / 32, reserve);
    }
    if (percpu_counter_read_positive(&vm_committed_as) < allowed) {
    return 0;
    }
// label;
    bytes_failed = pages << PAGE_SHIFT;
    pr_warn_ratelimited("%s: pid: %d, comm: %s, bytes: %lu not enough memory for the allocation\n",
    __func__, current.pid, current.comm, bytes_failed);
    vm_unacct_memory(pages);
    return -ENOMEM;
    }
//
// get_cmdline() - copy the cmdline value to a buffer.
// @task:     the task whose cmdline value to copy.
// @buffer:   the buffer to copy to.
// @buflen:   the length of the buffer. Larger cmdline values are truncated
// to this length.
//
// Return: the size of the cmdline field copied. Note that the copy does
// not guarantee an ending NULL byte.
//
#[no_mangle]
pub unsafe extern "C" fn get_cmdline(task: *mut task_struct, buffer: *mut c_char, buflen: c_int) -> c_int {
pub static mut res: c_int = 0;
    let mut len = 0;
    let mut mm = get_task_mm(task);
    unsigned long arg_start, arg_end, env_start, env_end;
    if (!mm) {
// goto;
    }
    if (!mm.arg_end) {
// goto;	/* Shh! No looking before we're done */
    }
    spin_lock(&mm.arg_lock);
    arg_start = mm.arg_start;
    arg_end = mm.arg_end;
    env_start = mm.env_start;
    env_end = mm.env_end;
    spin_unlock(&mm.arg_lock);
    len = arg_end - arg_start;
    if (len > buflen) {
    len = buflen;
    }
    res = access_process_vm(task, arg_start, buffer, len, FOLL_FORCE);
//
// If the nul at the end of args has been overwritten, then
// assume application is using setproctitle(3).
//
    if (res > 0 && buffer[res-1] != '\0' && len < buflen) {
    len = strnlen(buffer, res);
    if (len < res) {
    res = len;
    } else {
    len = env_end - env_start;
    if (len > buflen - res) {
    len = buflen - res;
    }
    res += access_process_vm(task, env_start,
    buffer+res, len,
    FOLL_FORCE);
    res = strnlen(buffer, res);
    }
    }
// label;
    mmput(mm);
// label;
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn memcmp_pages(page1: *mut page, page2: *mut page) -> int __weak {
    let mut addr1 = core::ptr::null_mut();
    let mut addr2 = core::ptr::null_mut();
    let mut ret = 0;
    addr1 = kmap_local_page(page1);
    addr2 = kmap_local_page(page2);
    ret = memcmp(addr1, addr2, PAGE_SIZE);
    kunmap_local(addr2);
    kunmap_local(addr1);
    return ret;
    }

//
// mem_dump_obj - Print available provenance information
// @object: object for which to find provenance information.
//
// This function uses pr_cont(), so that the caller is expected to have
// printed out whatever preamble is appropriate.  The provenance information
// depends on the type of object and on how much debugging is enabled.
// For example, for a slab-cache object, the slab name is printed, and,
// if available, the return address and stack trace from the allocation
// and last free path of that object.
//
#[no_mangle]
pub unsafe extern "C" fn mem_dump_obj(object: *mut c_void) {
pub static mut type: *mut c_void = core::ptr::null_mut();
    if (kmem_dump_obj(object)) {
    return;
    }
    if (vmalloc_dump_obj(object)) {
    return;
    }
    if (is_vmalloc_addr(object)) {
    type = "vmalloc memory";
    }

    else if (virt_addr_valid(object)) {
    type = "non-slab/vmalloc memory";
    }

    else if (object == core::ptr::null_mut()) {
    type = "core::ptr::null_mut() pointer";
    }

    else if (object == ZERO_SIZE_PTR) {
    type = "zero-size pointer";
    }
    else {
    type = "non-paged memory";
    }
    pr_cont(" %s\n", type);
    }
    EXPORT_SYMBOL_GPL(mem_dump_obj);

//
// A driver might set a page logically offline -- PageOffline() -- and
// turn the page inaccessible in the hypervisor; after that, access to page
// content can be fatal.
//
// Some special PFN walkers -- i.e., /proc/kcore -- read content of random
// pages after checking PageOffline(); however, these PFN walkers can race
// with drivers that set PageOffline().
//
// page_offline_freeze()/page_offline_thaw() allows for a subsystem to
// synchronize with such drivers, achieving that a page cannot be set
// PageOffline() while frozen.
//
// page_offline_begin()/page_offline_end() is used by drivers that care about
// such races when setting a page PageOffline().
//
pub static mut page_offline_rwsem: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn page_offline_freeze() {
    down_read(&page_offline_rwsem);
    }
#[no_mangle]
pub unsafe extern "C" fn page_offline_thaw() {
    up_read(&page_offline_rwsem);
    }
#[no_mangle]
pub unsafe extern "C" fn page_offline_begin() {
    down_write(&page_offline_rwsem);
    }
    EXPORT_SYMBOL(page_offline_begin);
#[no_mangle]
pub unsafe extern "C" fn page_offline_end() {
    up_write(&page_offline_rwsem);
    }
    EXPORT_SYMBOL(page_offline_end);

#[no_mangle]
pub unsafe extern "C" fn flush_dcache_folio(folio: *mut folio) {
    long i, nr = folio_nr_pages(folio);
    for (i = 0; i < nr; i++) {
    flush_dcache_page(folio_page(folio, i));
    }
    }
    EXPORT_SYMBOL(flush_dcache_folio);

//
// compat_set_desc_from_vma() - assigns VMA descriptor @desc fields from a VMA.
// @desc: A VMA descriptor whose fields need to be set.
// @file: The file object describing the file being mmap()'d.
// @vma: The VMA whose fields we wish to assign to @desc.
//
// This is a compatibility function to allow an mmap() hook to call
// mmap_prepare() hooks when drivers nest these. This function specifically
// allows the construction of a vm_area_desc value, @desc, from a VMA @vma for
// the purposes of doing this.
//
// Once the conversion of drivers is complete this function will no longer be
// required and will be removed.
//
#[no_mangle]
pub unsafe extern "C" fn compat_set_desc_from_vma(desc: *mut vm_area_desc, file: *mut file, vma: *mut vm_area_struct) {
    memset(desc, 0, sizeof!(*desc));
    desc.mm = vma.vm_mm;
    desc.file = file;
    desc.start = vma.vm_start;
    desc.end = vma.vm_end;
    desc.pgoff = vma_start_pgoff(vma);
    desc.vm_file = vma.vm_file;
    desc.vma_flags = vma.flags;
    desc.page_prot = vma.vm_page_prot;
    desc.vm_ops = vma.vm_ops;
// Default.
    desc.action.type = MMAP_NOTHING;
    }
    EXPORT_SYMBOL(compat_set_desc_from_vma);
//
// __compat_vma_mmap() - Similar to compat_vma_mmap(), only it allows
// flexibility as to how the mmap_prepare callback is invoked, which is useful
// for drivers which invoke nested mmap_prepare callbacks in an mmap() hook.
// @desc: A VMA descriptor upon which an mmap_prepare() hook has already been
// executed.
// @vma: The VMA to which @desc should be applied.
//
// The function assumes that you have obtained a VMA descriptor @desc from
// compat_set_desc_from_vma(), and already executed the mmap_prepare() hook upon
// it.
//
// It then performs any specified mmap actions, and invokes the vm_ops->mapped()
// hook if one is present.
//
// See the description of compat_vma_mmap() for more details.
//
// Once the conversion of drivers is complete this function will no longer be
// required and will be removed.
//
// Returns: 0 on success or error.
//
#[no_mangle]
pub unsafe extern "C" fn __compat_vma_mmap(desc: *mut vm_area_desc, vma: *mut vm_area_struct) -> c_int {
    let mut err = 0;
// Perform any preparatory tasks for mmap action.
    err = mmap_action_prepare(desc);
    if (err) {
    return err;
    }
// Update the VMA from the descriptor.
    compat_set_vma_from_desc(vma, desc);
// Complete any specified mmap actions.
    return mmap_action_complete(vma, &desc.action, /*is_compat=*/true);
    }
    EXPORT_SYMBOL(__compat_vma_mmap);
//
// compat_vma_mmap() - Apply the file's .mmap_prepare() hook to an
// existing VMA and execute any requested actions.
// @file: The file which possesss an f_op->mmap_prepare() hook.
// @vma: The VMA to apply the .mmap_prepare() hook to.
//
// Ordinarily, .mmap_prepare() is invoked directly upon mmap(). However, certain
// stacked drivers invoke a nested mmap hook of an underlying file.
//
// Until all drivers are converted to use .mmap_prepare(), we must be
// conservative and continue to invoke these stacked drivers using the
// deprecated .mmap() hook.
//
// However we have a problem if the underlying file system possesses an
// .mmap_prepare() hook, as we are in a different context when we invoke the
// .mmap() hook, already having a VMA to deal with.
//
// compat_vma_mmap() is a compatibility function that takes VMA state,
// establishes a struct vm_area_desc descriptor, passes to the underlying
// .mmap_prepare() hook and applies any changes performed by it.
//
// Once the conversion of drivers is complete this function will no longer be
// required and will be removed.
//
// Returns: 0 on success or error.
//
#[no_mangle]
pub unsafe extern "C" fn compat_vma_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
pub static mut desc: usize = 0;
pub static mut action: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    compat_set_desc_from_vma(&desc, file, vma);
    err = vfs_mmap_prepare(file, &desc);
    if (err) {
    return err;
    }
    action = &desc.action;
// being invoked from .mmmap means we don't have to enforce this.
    action.hide_from_rmap_until_complete = false;
    return __compat_vma_mmap(&desc, vma);
    }
    EXPORT_SYMBOL(compat_vma_mmap);
#[no_mangle]
pub unsafe extern "C" fn set_ps_flags(ps: *mut page_snapshot, folio: *mut folio, page: *mut page) {
//
// Only the first page of a high-order buddy page has PageBuddy() set.
// So we have to check manually whether this page is part of a high-
// order buddy page.
//
    if (PageBuddy(page)) {
    ps.flags |= PAGE_SNAPSHOT_PG_BUDDY;
    }

    else if (page_count(page) == 0 && is_free_buddy_page(page)) {
    ps.flags |= PAGE_SNAPSHOT_PG_BUDDY;
    }
    if (folio_test_idle(folio)) {
    ps.flags |= PAGE_SNAPSHOT_PG_IDLE;
    }
    }
//
// snapshot_page() - Create a snapshot of a struct page
// @ps: Pointer to a struct page_snapshot to store the page snapshot
// @page: The page to snapshot
//
// Create a snapshot of the page and store both its struct page and struct
// folio representations in @ps.
//
// A snapshot is marked as "faithful" if the compound state of @page was
// stable and allowed safe reconstruction of the folio representation. In
// rare cases where this is not possible (e.g. due to folio splitting),
// snapshot_page() falls back to treating @page as a single page and the
// snapshot is marked as "unfaithful". The snapshot_page_is_faithful()
// helper can be used to check for this condition.
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_page(ps: *mut page_snapshot, page: *const page) {
    unsigned long info, nr_pages = 1;
pub static mut foliop: *mut c_void = core::ptr::null_mut();
pub static mut loops: c_int = 5;
    ps.pfn = page_to_pfn(page);
    ps.flags = PAGE_SNAPSHOT_FAITHFUL;
// label;
    memset(&ps.folio_snapshot, 0, sizeof!(folio));
    memcpy(&ps.page_snapshot, page, sizeof!(*page));
    info = ps.page_snapshot.compound_info;
    if (!(info & 1)) {
    ps.idx = 0;
    foliop = &ps.page_snapshot;
    if (!folio_test_large(foliop)) {
    set_ps_flags(ps, page_folio(page), page);
    memcpy(&ps.folio_snapshot, foliop,
    sizeof!(page));
    return;
    }
    foliop = page;
    } else {
// See compound_head()
    if (compound_info_has_mask()) {
pub static mut p: c_ulong = 0;
    foliop = (p & info);
    } else {
    foliop = (info - 1);
    }
    ps.idx = folio_page_idx(foliop, page);
    }
    if (ps.idx < MAX_FOLIO_NR_PAGES) {
    memcpy(&ps.folio_snapshot, foliop, 2 * sizeof!(page));
    nr_pages = folio_nr_pages(&ps.folio_snapshot);
    if (nr_pages > 2) {
    memcpy(&ps.folio_snapshot.__page_2, &foliop.__page_2,
    sizeof!(page));
    }
    set_ps_flags(ps, foliop, page);
    }
    if (ps.idx > nr_pages) {
    if (loops-- > 0) {
// goto;
    }
    clear_compound_head(&ps.page_snapshot);
    foliop = &ps.page_snapshot;
    memcpy(&ps.folio_snapshot, foliop, sizeof!(page));
    ps.flags = 0;
    ps.idx = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn call_vma_mapped(vma: *mut vm_area_struct) -> c_int {
    let mut vm_ops = vma.vm_ops;
    let mut vm_private_data = vma.vm_private_data;
    let mut err = 0;
    if (!vm_ops || !vm_ops.mapped) {
    return 0;
    }
    err = vm_ops.mapped(vma.vm_start, vma.vm_end, vma_start_pgoff(vma),
    vma.vm_file, &vm_private_data);
    if (err) {
    return err;
    }
    if (vm_private_data != vma.vm_private_data) {
    vma.vm_private_data = vm_private_data;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mmap_action_finish(vma: *mut vm_area_struct, action: *mut mmap_action, err: c_int, is_compat: bool) -> c_int {
    let mut len = 0;
    if (!err) {
    err = call_vma_mapped(vma);
    }
// do_munmap() might take rmap lock, so release if held.
    maybe_rmap_unlock_action(vma, action);
//
// If this is invoked from the compatibility layer, post-mmap() hook
// logic will handle cleanup for us.
//
    if (!err || is_compat) {
    return err;
    }
//
// If an error occurs, unmap the VMA altogether and return an error. We
// only clear the newly allocated VMA, since this function is only
// invoked if we do NOT merge, so we only clean up the VMA we created.
//
    len = vma_pages(vma) << PAGE_SHIFT;
    do_munmap(current.mm, vma.vm_start, len, core::ptr::null_mut());
    return action.error_override ?: err;
    }

#[no_mangle]
unsafe extern "C" fn check_mmap_action(action: *mut mmap_action) -> c_int {
pub static mut override: c_ulong = 0;
    if (WARN_ON_ONCE!(override && !IS_ERR_VALUE(override))) {
    return -EINVAL;
    }
    return 0;
    }
//
// mmap_action_prepare - Perform preparatory setup for an VMA descriptor
// action which need to be performed.
// @desc: The VMA descriptor to prepare for its @desc->action.
//
// Returns: %0 on success, otherwise error.
//
#[no_mangle]
pub unsafe extern "C" fn mmap_action_prepare(desc: *mut vm_area_desc) -> c_int {
    let mut action = &desc.action;
    let mut err = 0;
    err = check_mmap_action(action);
    if (err) {
    return err;
    }
    match (action.type) {
    MMAP_NOTHING => {
    return 0;
    }
    MMAP_REMAP_PFN => {
    return remap_pfn_range_prepare(desc);
    }
    MMAP_IO_REMAP_PFN => {
    return io_remap_pfn_range_prepare(desc);
    }
    MMAP_SIMPLE_IO_REMAP => {
    return simple_ioremap_prepare(desc);
    }
    MMAP_MAP_KERNEL_PAGES => {
    return map_kernel_pages_prepare(desc);
    }
    }
    WARN_ON_ONCE!(1);
    return -EINVAL;
    }
    EXPORT_SYMBOL(mmap_action_prepare);
//
// mmap_action_complete - Execute VMA descriptor action.
// @vma: The VMA to perform the action upon.
// @action: The action to perform.
// @is_compat: Is this being invoked from the compatibility layer?
//
// Similar to mmap_action_prepare().
//
// Return: 0 on success, or error, at which point the VMA will be unmapped if
// !@is_compat.
//
#[no_mangle]
pub unsafe extern "C" fn mmap_action_complete(vma: *mut vm_area_struct, action: *mut mmap_action, is_compat: bool) -> c_int {
pub static mut err: c_int = 0;
    match (action.type) {
    MMAP_NOTHING => {
    // break;
    }
    MMAP_REMAP_PFN => {
    err = remap_pfn_range_complete(vma, action);
    // break;
    }
    MMAP_MAP_KERNEL_PAGES => {
    err = map_kernel_pages_complete(vma, action);
    // break;
    }
    MMAP_IO_REMAP_PFN => {
    }
    MMAP_SIMPLE_IO_REMAP => {
// Should have been delegated.
    WARN_ON_ONCE!(1);
    err = -EINVAL;
    // break;
    }
    }
    return mmap_action_finish(vma, action, err, is_compat);
    }
    EXPORT_SYMBOL(mmap_action_complete);

#[no_mangle]
#[no_mangle]
// duplicate fn: mmap_action_prepare
pub unsafe extern "C" fn mmap_action_prepare_dup(desc: *mut vm_area_desc) -> c_int {
    match (desc.action.type) {
    MMAP_NOTHING => {
    // break;
    }
    MMAP_REMAP_PFN => {
    }
    MMAP_IO_REMAP_PFN => {
    }
    MMAP_SIMPLE_IO_REMAP => {
    }
    MMAP_MAP_KERNEL_PAGES => {
    WARN_ON_ONCE!(1); /* nommu cannot handle these. */
    // break;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL(mmap_action_prepare);
#[no_mangle]
#[no_mangle]
// duplicate fn: mmap_action_complete
pub unsafe extern "C" fn mmap_action_complete_dup(vma: *mut vm_area_struct, action: *mut mmap_action, is_compat: bool) -> c_int {
pub static mut err: c_int = 0;
    match (action.type) {
    MMAP_NOTHING => {
    // break;
    }
    MMAP_REMAP_PFN => {
    }
    MMAP_IO_REMAP_PFN => {
    }
    MMAP_SIMPLE_IO_REMAP => {
    }
    MMAP_MAP_KERNEL_PAGES => {
    WARN_ON_ONCE!(1); /* nommu cannot handle this. */
    err = -EINVAL;
    // break;
    }
    }
    return mmap_action_finish(vma, action, err, is_compat);
    }
    EXPORT_SYMBOL(mmap_action_complete);

//
// folio_pte_batch - detect a PTE batch for a large folio
// @folio: The large folio to detect a PTE batch for.
// @ptep: Page table pointer for the first entry.
// @pte: Page table entry for the first page.
// @max_nr: The maximum number of table entries to consider.
//
// This is a simplified variant of folio_pte_batch_flags().
//
// Detect a PTE batch: consecutive (present) PTEs that map consecutive
// pages of the same large folio in a single VMA and a single page table.
//
// All PTEs inside a PTE batch have the same PTE bits set, excluding the PFN,
// the accessed bit, writable bit, dirt-bit and soft-dirty bit.
//
// ptep must map any page of the folio. max_nr must be at least one and
// must be limited by the caller so scanning cannot exceed a single VMA and
// a single page table.
//
// Return: the number of table entries in the batch.
//
#[no_mangle]
pub unsafe extern "C" fn folio_pte_batch(folio: *mut folio, ptep: *mut pte_t, pte: pte_t, max_nr: c_uint) -> c_uint {
    return folio_pte_batch_flags(folio, core::ptr::null_mut(), ptep, &pte, max_nr, 0);
    }

//
// page_range_contiguous - test whether the page range is contiguous
// @page: the start of the page range.
// @nr_pages: the number of pages in the range.
//
// Test whether the page range is contiguous, such that they can be iterated
// naively, corresponding to iterating a contiguous PFN range.
//
// This function should primarily only be used for debug checks, or when
// working with page ranges that are not naturally contiguous (e.g., pages
// within a folio are).
//
// Returns true if contiguous, otherwise false.
//
#[no_mangle]
pub unsafe extern "C" fn page_range_contiguous(page: *const page, nr_pages: c_ulong) -> bool {
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
    let mut pfn = 0;
//
// The memmap is allocated per memory section, so no need to check
// within the first section. However, we need to check each other
// spanned memory section once, making sure the first page in a
// section could similarly be reached by just iterating pages.
//
    for (pfn = ALIGN(start_pfn, PAGES_PER_SECTION);
    pfn < end_pfn; pfn += PAGES_PER_SECTION) {
    if (unlikely(page + (pfn - start_pfn) != pfn_to_page(pfn)))
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL(page_range_contiguous);