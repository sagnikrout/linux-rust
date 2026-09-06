//! Automatically rewritten from C to Rust
//! Source: kernel/irq/matrix.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2017 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpumap {
    pub available: c_uint,
    pub allocated: c_uint,
    pub managed: c_uint,
    pub managed_allocated: c_uint,
    pub initialized: bool,
    pub online: bool,
    pub managed_map: *mut c_ulong,
    pub alloc_map: [c_ulong; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_matrix {
    pub matrix_bits: c_uint,
    pub alloc_start: c_uint,
    pub alloc_end: c_uint,
    pub alloc_size: c_uint,
    pub global_available: c_uint,
    pub global_reserved: c_uint,
    pub systembits_inalloc: c_uint,
    pub total_allocated: c_uint,
    pub online_maps: c_uint,
    pub maps: *mut cpumap ,
    pub system_map: *mut c_ulong,
    pub scratch_map: [c_ulong; 0],
}

// Macro flag: #define CREATE_TRACE_POINTS

//
// irq_alloc_matrix - Allocate a irq_matrix structure and initialize it
// @matrix_bits:	Number of matrix bits
// @alloc_start:	From which bit the allocation search starts
// @alloc_end:		At which bit the allocation search ends, i.e first
// invalid bit
//
#[no_mangle]
pub unsafe extern "C" fn irq_alloc_matrix(matrix_bits: c_uint, alloc_start: c_uint, alloc_end: c_uint) -> *mut c_void {
    unsigned int cpu, matrix_size = BITS_TO_LONGS(matrix_bits);
pub static mut m: *mut c_void = core::ptr::null_mut();
    m = kzalloc_flex(*m, scratch_map, matrix_size * 2);
    if (!m) {
    return core::ptr::null_mut();
    }
    m.system_map = &m.scratch_map[matrix_size];
    m.matrix_bits = matrix_bits;
    m.alloc_start = alloc_start;
    m.alloc_end = alloc_end;
    m.alloc_size = alloc_end - alloc_start;
    m.maps = __alloc_percpu(struct_size(m.maps, alloc_map, matrix_size * 2),
    __alignof__(*m.maps));
    if (!m.maps) {
    kfree(m);
    return core::ptr::null_mut();
    }
    for_each_possible_cpu(cpu) {
    let mut cm = per_cpu_ptr(m.maps, cpu);
    cm.managed_map = &cm.alloc_map[matrix_size];
    }
    return m;
    }
//
// irq_matrix_online - Bring the local CPU matrix online
// @m:		Matrix pointer
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_online(m: *mut irq_matrix) {
    let mut cm = this_cpu_ptr(m.maps);
    BUG_ON!(cm.online);
    if (!cm.initialized) {
    cm.available = m.alloc_size;
    cm.available -= cm.managed + m.systembits_inalloc;
    cm.initialized = true;
    }
    m.global_available += cm.available;
    cm.online = true;
    m.online_maps += 1;
    trace_irq_matrix_online(m);
    }
//
// irq_matrix_offline - Bring the local CPU matrix offline
// @m:		Matrix pointer
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_offline(m: *mut irq_matrix) {
    let mut cm = this_cpu_ptr(m.maps);
// Update the global available size
    m.global_available -= cm.available;
    cm.online = false;
    m.online_maps -= 1;
    trace_irq_matrix_offline(m);
    }
#[no_mangle]
pub unsafe extern "C" fn matrix_alloc_area(m: *mut irq_matrix, cm: *mut cpumap, num: c_uint, managed: bool) -> c_uint {
    unsigned int area, start = m.alloc_start;
pub static mut end: c_uint = 0;
    bitmap_or(m.scratch_map, cm.managed_map, m.system_map, end);
    bitmap_or(m.scratch_map, m.scratch_map, cm.alloc_map, end);
    area = bitmap_find_next_zero_area(m.scratch_map, end, start, num, 0);
    if (area >= end) {
    return area;
    }
    if (managed) {
    bitmap_set(cm.managed_map, area, num);
    }
    else {
    bitmap_set(cm.alloc_map, area, num);
    }
    return area;
    }
// Find the best CPU which has the lowest vector allocation count
#[no_mangle]
pub unsafe extern "C" fn matrix_find_best_cpu(m: *mut irq_matrix, msk: *mut cpumask) -> c_uint {
    unsigned int cpu, best_cpu, maxavl = 0;
pub static mut cm: *mut c_void = core::ptr::null_mut();
    best_cpu = UINT_MAX;
    for_each_cpu(cpu, msk) {
    cm = per_cpu_ptr(m.maps, cpu);
    if (!cm.online || cm.available <= maxavl) {
    continue;
    }
    best_cpu = cpu;
    maxavl = cm.available;
    }
    return best_cpu;
    }
// Find the best CPU which has the lowest number of managed IRQs allocated
#[no_mangle]
pub unsafe extern "C" fn matrix_find_best_cpu_managed(m: *mut irq_matrix, msk: *mut cpumask) -> c_uint {
    unsigned int cpu, best_cpu, allocated = UINT_MAX;
pub static mut cm: *mut c_void = core::ptr::null_mut();
    best_cpu = UINT_MAX;
    for_each_cpu(cpu, msk) {
    cm = per_cpu_ptr(m.maps, cpu);
    if (!cm.online || cm.managed_allocated > allocated) {
    continue;
    }
    best_cpu = cpu;
    allocated = cm.managed_allocated;
    }
    return best_cpu;
    }
//
// irq_matrix_assign_system - Assign system wide entry in the matrix
// @m:		Matrix pointer
// @bit:	Which bit to reserve
// @replace:	Replace an already allocated vector with a system
// vector at the same bit position.
//
// The BUG_ON!()s below are on purpose. If this goes wrong in the
// early boot process, then the chance to survive is about zero.
// If this happens when the system is life, it's not much better.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_assign_system(m: *mut irq_matrix, bit: c_uint, replace: bool) {
    let mut cm = this_cpu_ptr(m.maps);
    BUG_ON!(bit > m.matrix_bits);
    BUG_ON!(m.online_maps > 1 || (m.online_maps && !replace));
    set_bit(bit, m.system_map);
    if (replace) {
    BUG_ON!(!test_and_clear_bit(bit, cm.alloc_map));
    cm.allocated -= 1;
    m.total_allocated -= 1;
    }
    if (bit >= m.alloc_start && bit < m.alloc_end) {
    m.systembits_inalloc += 1;
    }
    trace_irq_matrix_assign_system(bit, m);
    }
//
// irq_matrix_reserve_managed - Reserve a managed interrupt in a CPU map
// @m:		Matrix pointer
// @msk:	On which CPUs the bits should be reserved.
//
// Can be called for offline CPUs. Note, this will only reserve one bit
// on all CPUs in @msk, but it's not guaranteed that the bits are at the
// same offset on all CPUs
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_reserve_managed(m: *mut irq_matrix, msk: *const cpumask) -> c_int {
    let mut cpu = 0;
    let mut failed_cpu = 0;
    for_each_cpu(cpu, msk) {
    let mut cm = per_cpu_ptr(m.maps, cpu);
    let mut bit = 0;
    bit = matrix_alloc_area(m, cm, 1, true);
    if (bit >= m.alloc_end) {
// goto;
    }
    cm.managed += 1;
    if (cm.online) {
    cm.available -= 1;
    m.global_available -= 1;
    }
    trace_irq_matrix_reserve_managed(bit, cpu, m, cm);
    }
    return 0;
// label;
    failed_cpu = cpu;
    for_each_cpu(cpu, msk) {
    if (cpu == failed_cpu) {
    break;
    }
    irq_matrix_remove_managed(m, cpumask_of(cpu));
    }
    return -ENOSPC;
    }
//
// irq_matrix_remove_managed - Remove managed interrupts in a CPU map
// @m:		Matrix pointer
// @msk:	On which CPUs the bits should be removed
//
// Can be called for offline CPUs
//
// This removes not allocated managed interrupts from the map. It does
// not matter which one because the managed interrupts free their
// allocation when they shut down. If not, the accounting is screwed,
// but all what can be done at this point is warn about it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_remove_managed(m: *mut irq_matrix, msk: *const cpumask) {
    let mut cpu = 0;
    for_each_cpu(cpu, msk) {
    let mut cm = per_cpu_ptr(m.maps, cpu);
    unsigned int bit, end = m.alloc_end;
    if (WARN_ON_ONCE!(!cm.managed)) {
    continue;
    }
// Get managed bit which are not allocated
    bitmap_andnot(m.scratch_map, cm.managed_map, cm.alloc_map, end);
    bit = find_first_bit(m.scratch_map, end);
    if (WARN_ON_ONCE!(bit >= end)) {
    continue;
    }
    clear_bit(bit, cm.managed_map);
    cm.managed -= 1;
    if (cm.online) {
    cm.available += 1;
    m.global_available += 1;
    }
    trace_irq_matrix_remove_managed(bit, cpu, m, cm);
    }
    }
//
// irq_matrix_alloc_managed - Allocate a managed interrupt in a CPU map
// @m:		Matrix pointer
// @msk:	Which CPUs to search in
// @mapped_cpu:	Pointer to store the CPU for which the irq was allocated
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_alloc_managed(m: *mut irq_matrix, msk: *mut cpumask, mapped_cpu: *mut c_uint) -> c_int {
    let mut bit = 0;
    let mut cpu = 0;
    let mut end = 0;
pub static mut cm: *mut c_void = core::ptr::null_mut();
    if (cpumask_empty(msk)) {
    return -EINVAL;
    }
    cpu = matrix_find_best_cpu_managed(m, msk);
    if (cpu == UINT_MAX) {
    return -ENOSPC;
    }
    cm = per_cpu_ptr(m.maps, cpu);
    end = m.alloc_end;
// Get managed bit which are not allocated
    bitmap_andnot(m.scratch_map, cm.managed_map, cm.alloc_map, end);
    bit = find_first_bit(m.scratch_map, end);
    if (bit >= end) {
    return -ENOSPC;
    }
    set_bit(bit, cm.alloc_map);
    cm.allocated += 1;
    cm.managed_allocated += 1;
    m.total_allocated += 1;
// mapped_cpu = cpu;
    trace_irq_matrix_alloc_managed(bit, cpu, m, cm);
    return bit;
    }
//
// irq_matrix_assign - Assign a preallocated interrupt in the local CPU map
// @m:		Matrix pointer
// @bit:	Which bit to mark
//
// This should only be used to mark preallocated vectors
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_assign(m: *mut irq_matrix, bit: c_uint) {
    let mut cm = this_cpu_ptr(m.maps);
    if (WARN_ON_ONCE!(bit < m.alloc_start || bit >= m.alloc_end)) {
    return;
    }
    if (WARN_ON_ONCE!(test_and_set_bit(bit, cm.alloc_map))) {
    return;
    }
    cm.allocated += 1;
    m.total_allocated += 1;
    cm.available -= 1;
    m.global_available -= 1;
    trace_irq_matrix_assign(bit, smp_processor_id(), m, cm);
    }
//
// irq_matrix_reserve - Reserve interrupts
// @m:		Matrix pointer
//
// This is merely a book keeping call. It increments the number of globally
// reserved interrupt bits w/o actually allocating them. This allows to
// setup interrupt descriptors w/o assigning low level resources to it.
// The actual allocation happens when the interrupt gets activated.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_reserve(m: *mut irq_matrix) {
    if (m.global_reserved == m.global_available) {
    pr_warn!("Interrupt reservation exceeds available resources\n");
    }
    m.global_reserved += 1;
    trace_irq_matrix_reserve(m);
    }
//
// irq_matrix_remove_reserved - Remove interrupt reservation
// @m:		Matrix pointer
//
// This is merely a book keeping call. It decrements the number of globally
// reserved interrupt bits. This is used to undo irq_matrix_reserve() when the
// interrupt was never in use and a real vector allocated, which undid the
// reservation.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_remove_reserved(m: *mut irq_matrix) {
    m.global_reserved -= 1;
    trace_irq_matrix_remove_reserved(m);
    }
//
// irq_matrix_alloc - Allocate a regular interrupt in a CPU map
// @m:		Matrix pointer
// @msk:	Which CPUs to search in
// @reserved:	Allocate previously reserved interrupts
// @mapped_cpu: Pointer to store the CPU for which the irq was allocated
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_alloc(m: *mut irq_matrix, msk: *mut cpumask, reserved: bool, mapped_cpu: *mut c_uint) -> c_int {
    let mut cpu = 0;
    let mut bit = 0;
pub static mut cm: *mut c_void = core::ptr::null_mut();
//
// Not required in theory, but matrix_find_best_cpu() uses
// for_each_cpu() which ignores the cpumask on UP .
//
    if (cpumask_empty(msk)) {
    return -EINVAL;
    }
    cpu = matrix_find_best_cpu(m, msk);
    if (cpu == UINT_MAX) {
    return -ENOSPC;
    }
    cm = per_cpu_ptr(m.maps, cpu);
    bit = matrix_alloc_area(m, cm, 1, false);
    if (bit >= m.alloc_end) {
    return -ENOSPC;
    }
    cm.allocated += 1;
    cm.available -= 1;
    m.total_allocated += 1;
    m.global_available -= 1;
    if (reserved) {
    m.global_reserved -= 1;
    }
// mapped_cpu = cpu;
    trace_irq_matrix_alloc(bit, cpu, m, cm);
    return bit;
    }
//
// irq_matrix_free - Free allocated interrupt in the matrix
// @m:		Matrix pointer
// @cpu:	Which CPU map needs be updated
// @bit:	The bit to remove
// @managed:	If true, the interrupt is managed and not accounted
// as available.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_free(m: *mut irq_matrix, cpu: c_uint, bit: c_uint, managed: bool) {
    let mut cm = per_cpu_ptr(m.maps, cpu);
    if (WARN_ON_ONCE!(bit < m.alloc_start || bit >= m.alloc_end)) {
    return;
    }
    if (WARN_ON_ONCE!(!test_and_clear_bit(bit, cm.alloc_map))) {
    return;
    }
    cm.allocated -= 1;
    if(managed) {
    cm.managed_allocated -= 1;
    }
    if (cm.online) {
    m.total_allocated -= 1;
    }
    if (!managed) {
    cm.available += 1;
    if (cm.online) {
    m.global_available += 1;
    }
    }
    trace_irq_matrix_free(bit, cpu, m, cm);
    }
//
// irq_matrix_available - Get the number of globally available irqs
// @m:		Pointer to the matrix to query
// @cpudown:	If true, the local CPU is about to go down, adjust
// the number of available irqs accordingly
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_available(m: *mut irq_matrix, cpudown: bool) -> c_uint {
    let mut cm = this_cpu_ptr(m.maps);
    if (!cpudown) {
    return m.global_available;
    }
    return m.global_available - cm.available;
    }
//
// irq_matrix_reserved - Get the number of globally reserved irqs
// @m:		Pointer to the matrix to query
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_reserved(m: *mut irq_matrix) -> c_uint {
    return m.global_reserved;
    }
//
// irq_matrix_allocated - Get the number of allocated non-managed irqs on the local CPU
// @m:		Pointer to the matrix to search
//
// This returns number of allocated non-managed interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_allocated(m: *mut irq_matrix) -> c_uint {
    let mut cm = this_cpu_ptr(m.maps);
    return cm.allocated - cm.managed_allocated;
    }

//
// irq_matrix_debug_show - Show detailed allocation information
// @sf:		Pointer to the seq_file to print to
// @m:		Pointer to the matrix allocator
// @ind:	Indentation for the print format
//
// Note, this is a lockless snapshot.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_debug_show(sf: *mut seq_file, m: *mut irq_matrix, ind: c_int) {
pub static mut nsys: c_uint = 0;
    let mut cpu = 0;
    seq_printf(sf, "Online bitmaps:   %6u\n", m.online_maps);
    seq_printf(sf, "Global available: %6u\n", m.global_available);
    seq_printf(sf, "Global reserved:  %6u\n", m.global_reserved);
    seq_printf(sf, "Total allocated:  %6u\n", m.total_allocated);
    seq_printf(sf, "System: %u: %*pbl\n", nsys, m.matrix_bits,
    m.system_map);
    seq_printf(sf, "%*s| CPU | avl | man | mac | act | vectors\n", ind, " ");
    cpus_read_lock();
    for_each_online_cpu(cpu) {
    let mut cm = per_cpu_ptr(m.maps, cpu);
    seq_printf(sf, "%*s %4d  %4u  %4u  %4u %4u  %*pbl\n", ind, " ",
    cpu, cm.available, cm.managed,
    cm.managed_allocated, cm.allocated,
    m.matrix_bits, cm.alloc_map);
    }
    cpus_read_unlock();
    }