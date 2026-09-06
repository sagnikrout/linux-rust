//! Automatically rewritten from C to Rust
//! Source: kernel/locking/test-ww_mutex.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Module-based API test facility for ww_mutexes
//

pub static mut wd_class: usize = 0;
pub static mut ww_class: usize = 0;
pub static mut wq: *mut c_void = core::ptr::null_mut();

    ww_acquire_init((a), (b)); 
    (a).deadlock_inject_countdown = ~0U; 
    } while (0)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_mutex {
    pub work: work_struct,
    pub mutex: ww_mutex,
    pub done: completion ready, go,,
    pub flags: c_uint,
}

#[no_mangle]
unsafe extern "C" fn test_mutex_work(work: *mut work_struct) {
    let mut mtx = container_of!(work, typeof(*mtx), work);
    complete(&mtx.ready);
    wait_for_completion(&mtx.go);
    if (mtx.flags & TEST_MTX_TRY) {
    while (!ww_mutex_trylock(&mtx.mutex, core::ptr::null_mut())) {
    cond_resched();
    }
    } else {
    ww_mutex_lock(&mtx.mutex, core::ptr::null_mut());
    }
    complete(&mtx.done);
    ww_mutex_unlock(&mtx.mutex);
    }
#[no_mangle]
unsafe extern "C" fn __test_mutex(class: *mut ww_class, flags: c_uint) -> c_int {

pub static mut mtx: usize = 0;
pub static mut ctx: usize = 0;
    let mut ret = 0;
    ww_mutex_init(&mtx.mutex, class);
    if (flags & TEST_MTX_CTX) {
    ww_acquire_init(&ctx, class);
    }
    INIT_WORK_ONSTACK(&mtx.work, test_mutex_work);
    init_completion(&mtx.ready);
    init_completion(&mtx.go);
    init_completion(&mtx.done);
    mtx.flags = flags;
    queue_work(wq, &mtx.work);
    wait_for_completion(&mtx.ready);
    ww_mutex_lock(&mtx.mutex, (flags & TEST_MTX_CTX) ? &ctx : core::ptr::null_mut());
    complete(&mtx.go);
    if (flags & TEST_MTX_SPIN) {
pub static mut timeout: c_ulong = 0;
    ret = 0;
    do {
    if (completion_done(&mtx.done)) {
    ret = -EINVAL;
    break;
    }
    cond_resched();
    } while (time_before(jiffies, timeout));
    } else {
    ret = wait_for_completion_timeout(&mtx.done, TIMEOUT);
    }
    ww_mutex_unlock(&mtx.mutex);
    if (flags & TEST_MTX_CTX) {
    ww_acquire_fini(&ctx);
    }
    if (ret) {
    pr_err!("%s(flags=%x): mutual exclusion failure\n",
    __func__, flags);
    ret = -EINVAL;
    }
    flush_work(&mtx.work);
    destroy_work_on_stack(&mtx.work);
    return ret;

    }
#[no_mangle]
unsafe extern "C" fn test_mutex(class: *mut ww_class) -> c_int {
    let mut ret = 0;
    let mut i = 0;
    while (i < __TEST_MTX_LAST) {
    ret = __test_mutex(class, i);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_aa(class: *mut ww_class, trylock: bool) -> c_int {
pub static mut mutex: usize = 0;
pub static mut ctx: usize = 0;
    let mut ret = 0;
    let mut from = trylock ? "trylock" : "lock";
    ww_mutex_init(&mutex, class);
    ww_acquire_init(&ctx, class);
    if (!trylock) {
    ret = ww_mutex_lock(&mutex, &ctx);
    if (ret) {
    pr_err!("%s: initial lock failed!\n", __func__);
// goto;
    }
    } else {
    ret = !ww_mutex_trylock(&mutex, &ctx);
    if (ret) {
    pr_err!("%s: initial trylock failed!\n", __func__);
// goto;
    }
    }
    if (ww_mutex_trylock(&mutex, core::ptr::null_mut()))  {
    pr_err!("%s: trylocked itself without context from %s!\n", __func__, from);
    ww_mutex_unlock(&mutex);
    ret = -EINVAL;
// goto;
    }
    if (ww_mutex_trylock(&mutex, &ctx))  {
    pr_err!("%s: trylocked itself with context from %s!\n", __func__, from);
    ww_mutex_unlock(&mutex);
    ret = -EINVAL;
// goto;
    }
    ret = ww_mutex_lock(&mutex, &ctx);
    if (ret != -EALREADY) {
    pr_err!("%s: missed deadlock for recursing, ret=%d from %s\n",
    __func__, ret, from);
    if (!ret) {
    ww_mutex_unlock(&mutex);
    }
    ret = -EINVAL;
// goto;
    }
    ww_mutex_unlock(&mutex);
    ret = 0;
// label;
    ww_acquire_fini(&ctx);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_abba {
    pub work: work_struct,
    pub class: *mut ww_class,
    pub a_mutex: ww_mutex,
    pub b_mutex: ww_mutex,
    pub a_ready: completion,
    pub b_ready: completion,
    pub trylock: bool resolve,,
    pub result: c_int,
}

#[no_mangle]
unsafe extern "C" fn test_abba_work(work: *mut work_struct) {
    let mut abba = container_of!(work, typeof(*abba), work);
pub static mut ctx: usize = 0;
    let mut err = 0;
    ww_acquire_init_noinject(&ctx, abba.class);
    if (!abba.trylock) {
    ww_mutex_lock(&abba.b_mutex, &ctx);
    }
    else {
    WARN_ON!(!ww_mutex_trylock(&abba.b_mutex, &ctx));
    }
    WARN_ON!(READ_ONCE(abba.b_mutex.ctx) != &ctx);
    complete(&abba.b_ready);
    wait_for_completion(&abba.a_ready);
    err = ww_mutex_lock(&abba.a_mutex, &ctx);
    if (abba.resolve && err == -EDEADLK) {
    ww_mutex_unlock(&abba.b_mutex);
    ww_mutex_lock_slow(&abba.a_mutex, &ctx);
    err = ww_mutex_lock(&abba.b_mutex, &ctx);
    }
    if (!err) {
    ww_mutex_unlock(&abba.a_mutex);
    }
    ww_mutex_unlock(&abba.b_mutex);
    ww_acquire_fini(&ctx);
    abba.result = err;
    }
#[no_mangle]
unsafe extern "C" fn test_abba(class: *mut ww_class, trylock: bool, resolve: bool) -> c_int {
pub static mut abba: usize = 0;
pub static mut ctx: usize = 0;
    let mut err = 0;
    let mut ret = 0;
    ww_mutex_init(&abba.a_mutex, class);
    ww_mutex_init(&abba.b_mutex, class);
    INIT_WORK_ONSTACK(&abba.work, test_abba_work);
    init_completion(&abba.a_ready);
    init_completion(&abba.b_ready);
    abba.class = class;
    abba.trylock = trylock;
    abba.resolve = resolve;
    queue_work(wq, &abba.work);
    ww_acquire_init_noinject(&ctx, class);
    if (!trylock) {
    ww_mutex_lock(&abba.a_mutex, &ctx);
    }
    else {
    WARN_ON!(!ww_mutex_trylock(&abba.a_mutex, &ctx));
    }
    WARN_ON!(READ_ONCE(abba.a_mutex.ctx) != &ctx);
    complete(&abba.a_ready);
    wait_for_completion(&abba.b_ready);
    err = ww_mutex_lock(&abba.b_mutex, &ctx);
    if (resolve && err == -EDEADLK) {
    ww_mutex_unlock(&abba.a_mutex);
    ww_mutex_lock_slow(&abba.b_mutex, &ctx);
    err = ww_mutex_lock(&abba.a_mutex, &ctx);
    }
    if (!err) {
    ww_mutex_unlock(&abba.b_mutex);
    }
    ww_mutex_unlock(&abba.a_mutex);
    ww_acquire_fini(&ctx);
    flush_work(&abba.work);
    destroy_work_on_stack(&abba.work);
    ret = 0;
    if (resolve) {
    if (err || abba.result) {
    pr_err!("%s: failed to resolve ABBA deadlock, A err=%d, B err=%d\n",
    __func__, err, abba.result);
    ret = -EINVAL;
    }
    } else {
    if (err != -EDEADLK && abba.result != -EDEADLK) {
    pr_err!("%s: missed ABBA deadlock, A err=%d, B err=%d\n",
    __func__, err, abba.result);
    ret = -EINVAL;
    }
    }
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_cycle {
    pub work: work_struct,
    pub class: *mut ww_class,
    pub a_mutex: ww_mutex,
    pub b_mutex: *mut ww_mutex,
    pub a_signal: *mut completion,
    pub b_signal: completion,
    pub result: c_int,
}

#[no_mangle]
unsafe extern "C" fn test_cycle_work(work: *mut work_struct) {
    let mut cycle = container_of!(work, typeof(*cycle), work);
pub static mut ctx: usize = 0;
    int err, erra = 0;
    ww_acquire_init_noinject(&ctx, cycle.class);
    ww_mutex_lock(&cycle.a_mutex, &ctx);
    complete(cycle.a_signal);
    wait_for_completion(&cycle.b_signal);
    err = ww_mutex_lock(cycle.b_mutex, &ctx);
    if (err == -EDEADLK) {
    err = 0;
    ww_mutex_unlock(&cycle.a_mutex);
    ww_mutex_lock_slow(cycle.b_mutex, &ctx);
    erra = ww_mutex_lock(&cycle.a_mutex, &ctx);
    }
    if (!err) {
    ww_mutex_unlock(cycle.b_mutex);
    }
    if (!erra) {
    ww_mutex_unlock(&cycle.a_mutex);
    }
    ww_acquire_fini(&ctx);
    cycle.result = err ?: erra;
    }
#[no_mangle]
unsafe extern "C" fn __test_cycle(class: *mut ww_class, nthreads: c_uint) -> c_int {
pub static mut cycles: *mut c_void = core::ptr::null_mut();
    unsigned int n, last = nthreads - 1;
    let mut ret = 0;
    cycles = kmalloc_objs(*cycles, nthreads);
    if (!cycles) {
    return -ENOMEM;
    }
    while (n < nthreads) {
    let mut cycle = &cycles[n];
    cycle.class = class;
    ww_mutex_init(&cycle.a_mutex, class);
    if (n == last) {
    cycle.b_mutex = &cycles[0].a_mutex;
    }
    else {
    cycle.b_mutex = &cycles[n + 1].a_mutex;
    }
    if (n == 0) {
    cycle.a_signal = &cycles[last].b_signal;
    }
    else {
    cycle.a_signal = &cycles[n - 1].b_signal;
    }
    init_completion(&cycle.b_signal);
    INIT_WORK(&cycle.work, test_cycle_work);
    cycle.result = 0;
    }
    for (n = 0; n < nthreads; n++) {
    queue_work(wq, &cycles[n].work);
    }
    flush_workqueue(wq);
    ret = 0;
    while (n < nthreads) {
    let mut cycle = &cycles[n];
    if (!cycle.result) {
    continue;
    }
    pr_err!("cyclic deadlock not resolved, ret[%d/%d] = %d\n",
    n, nthreads, cycle.result);
    ret = -EINVAL;
    break;
    }
    for (n = 0; n < nthreads; n++) {
    ww_mutex_destroy(&cycles[n].a_mutex);
    }
    kfree(cycles);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_cycle(class: *mut ww_class, ncpus: c_uint) -> c_int {
    let mut n = 0;
    let mut ret = 0;
    while (n <= ncpus + 1) {
    ret = __test_cycle(class, n);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stress {
    pub work: work_struct,
    pub locks: *mut ww_mutex,
    pub class: *mut ww_class,
    pub timeout: c_ulong,
    pub nlocks: c_int,
}

pub static mut rng: usize = 0;
pub static mut rng_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn prandom_u32_below(ceil: u32) -> u32 {
    let mut ret = 0;
    spin_lock(&rng_lock);
    ret = prandom_u32_state(&rng) % ceil;
    spin_unlock(&rng_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn get_random_order(count: c_int) -> *mut c_void {
pub static mut order: *mut c_void = core::ptr::null_mut();
    let mut n = 0;
    let mut r = 0;
    order = kmalloc_objs(*order, count);
    if (!order) {
    return order;
    }
    for (n = 0; n < count; n++) {
    order[n] = n;
    }
    while (n > 1) {
    r = prandom_u32_below(n + 1);
    if (r != n) {
    swap(order[n], order[r]);
    }
    }
    return order;
    }
#[no_mangle]
unsafe extern "C" fn dummy_load(stress: *mut stress) {
    usleep_range(1000, 2000);
    }
#[no_mangle]
unsafe extern "C" fn stress_inorder_work(work: *mut work_struct) {
    let mut stress = container_of!(work, typeof(*stress), work);
pub static mut nlocks: c_int = 0;
    let mut locks = stress.locks;
pub static mut ctx: usize = 0;
pub static mut order: *mut c_void = core::ptr::null_mut();
    order = get_random_order(nlocks);
    if (!order) {
    return;
    }
    do {
pub static mut contended: c_int = 0;
    let mut n = 0;
    let mut err = 0;
    ww_acquire_init(&ctx, stress.class);
// label;
    err = 0;
    while (n < nlocks) {
    if (n == contended) {
    continue;
    }
    err = ww_mutex_lock(&locks[order[n]], &ctx);
    if (err < 0) {
    break;
    }
    }
    if (!err) {
    dummy_load(stress);
    }
    if (contended > n) {
    ww_mutex_unlock(&locks[order[contended]]);
    }
    contended = n;
    while (n--) {
    ww_mutex_unlock(&locks[order[n]]);
    }
    if (err == -EDEADLK) {
    if (!time_after(jiffies, stress.timeout)) {
    ww_mutex_lock_slow(&locks[order[contended]], &ctx);
// goto;
    }
    }
    ww_acquire_fini(&ctx);
    if (err) {
    pr_err_once("stress (%s) failed with %d\n",
    __func__, err);
    break;
    }
    } while (!time_after(jiffies, stress.timeout));
    kfree(order);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reorder_lock {
    pub link: list_head,
    pub lock: *mut ww_mutex,
}

#[no_mangle]
unsafe extern "C" fn stress_reorder_work(work: *mut work_struct) {
    let mut stress = container_of!(work, typeof(*stress), work);
pub static mut locks: usize = 0;
pub static mut ctx: usize = 0;
    let mut ll = core::ptr::null_mut();
    let mut ln = core::ptr::null_mut();
pub static mut order: *mut c_void = core::ptr::null_mut();
    let mut n = 0;
    let mut err = 0;
    order = get_random_order(stress.nlocks);
    if (!order) {
    return;
    }
    while (n < stress.nlocks) {
    ll = kmalloc_obj(*ll);
    if (!ll) {
// goto;
    }
    ll.lock = &stress.locks[order[n]];
    list_add(&ll.link, &locks);
    }
    kfree(order);
    order = core::ptr::null_mut();
    do {
    ww_acquire_init(&ctx, stress.class);
    list_for_each_entry(ll, &locks, link) {
    err = ww_mutex_lock(ll.lock, &ctx);
    if (!err) {
    continue;
    }
    ln = ll;
    list_for_each_entry_continue_reverse(ln, &locks, link) {
    ww_mutex_unlock(ln.lock);
    }
    if (err != -EDEADLK) {
    pr_err_once("stress (%s) failed with %d\n",
    __func__, err);
    break;
    }
    ww_mutex_lock_slow(ll.lock, &ctx);
    list_move(&ll.link, &locks); /* restarts iteration */
    }
    dummy_load(stress);
    list_for_each_entry(ll, &locks, link) {
    ww_mutex_unlock(ll.lock);
    }
    ww_acquire_fini(&ctx);
    } while (!time_after(jiffies, stress.timeout));
// label;
    list_for_each_entry_safe(ll, ln, &locks, link) {
    kfree(ll);
    }
    kfree(order);
    }
#[no_mangle]
unsafe extern "C" fn stress_one_work(work: *mut work_struct) {
    let mut stress = container_of!(work, typeof(*stress), work);
pub static mut nlocks: c_int = 0;
    let mut lock = stress.locks + get_random_u32_below(nlocks);
    let mut err = 0;
    do {
    err = ww_mutex_lock(lock, core::ptr::null_mut());
    if (!err) {
    dummy_load(stress);
    ww_mutex_unlock(lock);
    } else {
    pr_err_once("stress (%s) failed with %d\n",
    __func__, err);
    break;
    }
    } while (!time_after(jiffies, stress.timeout));
    }

#[no_mangle]
unsafe extern "C" fn stress(class: *mut ww_class, nlocks: c_int, nthreads: c_int, flags: c_uint) -> c_int {
pub static mut locks: *mut c_void = core::ptr::null_mut();
pub static mut stress_array: *mut c_void = core::ptr::null_mut();
    let mut n = 0;
    let mut count = 0;
    locks = kmalloc_objs(*locks, nlocks);
    if (!locks) {
    return -ENOMEM;
    }
    stress_array = kmalloc_objs(*stress_array, nthreads);
    if (!stress_array) {
    kfree(locks);
    return -ENOMEM;
    }
    for (n = 0; n < nlocks; n++) {
    ww_mutex_init(&locks[n], class);
    }
    count = 0;
    while (nthreads) {
pub static mut stress: *mut c_void = core::ptr::null_mut();
    void (*fn)(work_struct *work);
    fn = core::ptr::null_mut();
    match (n & 3) {
    0 => {
    if (flags & STRESS_INORDER) {
    fn = stress_inorder_work;
    }
    // break;
    }
    1 => {
    if (flags & STRESS_REORDER) {
    fn = stress_reorder_work;
    }
    // break;
    }
    2 => {
    if (flags & STRESS_ONE) {
    fn = stress_one_work;
    }
    // break;
    }
    }
    if (!fn) {
    continue;
    }
    stress = &stress_array[count++];
    INIT_WORK(&stress.work, fn);
    stress.class = class;
    stress.locks = locks;
    stress.nlocks = nlocks;
    stress.timeout = jiffies + 2*HZ;
    queue_work(wq, &stress.work);
    nthreads -= 1;
    }
    flush_workqueue(wq);
    for (n = 0; n < nlocks; n++) {
    ww_mutex_destroy(&locks[n]);
    }
    kfree(stress_array);
    kfree(locks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_tests(class: *mut ww_class) -> c_int {
pub static mut ncpus: c_int = 0;
    let mut ret = 0;
    let mut i = 0;
    ret = test_mutex(class);
    if (ret) {
    return ret;
    }
    ret = test_aa(class, false);
    if (ret) {
    return ret;
    }
    ret = test_aa(class, true);
    if (ret) {
    return ret;
    }
    while (i < 4) {
    ret = test_abba(class, i & 1, i & 2);
    if (ret) {
    return ret;
    }
    }
    ret = test_cycle(class, ncpus);
    if (ret) {
    return ret;
    }
    ret = stress(class, 16, 2 * ncpus, STRESS_INORDER);
    if (ret) {
    return ret;
    }
    ret = stress(class, 16, 2 * ncpus, STRESS_REORDER);
    if (ret) {
    return ret;
    }
    ret = stress(class, 2046, hweight32(STRESS_ALL) * ncpus, STRESS_ALL);
    if (ret) {
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_test_classes() -> c_int {
    let mut ret = 0;
    pr_info!("Beginning ww (wound) mutex selftests\n");
    ret = run_tests(&ww_class);
    if (ret) {
    return ret;
    }
    pr_info!("Beginning ww (die) mutex selftests\n");
    ret = run_tests(&wd_class);
    if (ret) {
    return ret;
    }
    pr_info!("All ww mutex selftests passed\n");
    return 0;
    }
pub static mut run_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn run_tests_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    if (!mutex_trylock(&run_lock)) {
    pr_err!("Test already running\n");
    return count;
    }
    run_test_classes();
    mutex_unlock(&run_lock);
    return count;
    }
    static struct kobj_attribute run_tests_attribute =
    __ATTR(run_tests, 0664, core::ptr::null_mut(), run_tests_store);
    static struct attribute *attrs[] = {
    &run_tests_attribute.attr,
    core::ptr::null_mut(),   /* need to core::ptr::null_mut() terminate the list of attributes */
    };
pub static mut attribute_group: usize = 0;
pub static mut test_ww_mutex_kobj: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn test_ww_mutex_init() -> c_int {
    let mut ret = 0;
    prandom_seed_state(&rng, get_random_u64());
    wq = alloc_workqueue("test-ww_mutex", WQ_UNBOUND, 0);
    if (!wq) {
    return -ENOMEM;
    }
    test_ww_mutex_kobj = kobject_create_and_add("test_ww_mutex", kernel_kobj);
    if (!test_ww_mutex_kobj) {
    destroy_workqueue(wq);
    return -ENOMEM;
    }
// Create the files associated with this kobject
    ret = sysfs_create_group(test_ww_mutex_kobj, &attr_group);
    if (ret) {
    kobject_put(test_ww_mutex_kobj);
    destroy_workqueue(wq);
    return ret;
    }
    mutex_lock(&run_lock);
    ret = run_test_classes();
    mutex_unlock(&run_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_ww_mutex_exit()  {
    kobject_put(test_ww_mutex_kobj);
    destroy_workqueue(wq);
    }
    module_init!(test_ww_mutex_init);
    module_exit!(test_ww_mutex_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_DESCRIPTION("API test facility for ww_mutexes");