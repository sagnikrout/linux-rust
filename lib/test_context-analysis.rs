//! Automatically rewritten from C to Rust
//! Source: lib/test_context-analysis.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Compile-only tests for common patterns that should not generate false
// positive errors when compiled with Clang's context analysis.
//

//
// Test that helper macros work as expected.
//
#[no_mangle]
unsafe extern "C" fn test_common_helpers() -> void __used {
    static void __used test_common_helpers(void)
    {
    BUILD_BUG_ON(context_unsafe(3) != 3); /* plain expression */
    BUILD_BUG_ON(context_unsafe((void)2; 3) != 3); /* does not swallow semi-colon */
    BUILD_BUG_ON(context_unsafe((void)2, 3) != 3); /* does not swallow commas */
    context_unsafe(do { } while (0)); /* works with void statements */
    }

    struct test_##class##_data {								\
    type lock;									\
    int counter __guarded_by(&lock);						\
    int *pointer __pt_guarded_by(&lock);						\
    };											\
    static void __used test_##class##_init(struct test_##class##_data *d)			\
    {											\
    guard(type_init)(&d.lock);							\
    d.counter = 0;									\
    }											\
    static void __used test_##class(struct test_##class##_data *d)				\
    {											\
    unsigned long flags;								\
    d.pointer++;									\
    type_lock(&d.lock);								\
    op(d.counter);									\
    op(*d.pointer);								\
    type_unlock(&d.lock);								\
    type_lock##_irq(&d.lock);							\
    op(d.counter);									\
    op(*d.pointer);								\
    type_unlock##_irq(&d.lock);							\
    type_lock##_bh(&d.lock);							\
    op(d.counter);									\
    op(*d.pointer);								\
    type_unlock##_bh(&d.lock);							\
    type_lock##_irqsave(&d.lock, flags);						\
    op(d.counter);									\
    op(*d.pointer);								\
    type_unlock##_irqrestore(&d.lock, flags);					\
    }											\
    static void __used test_##class##_trylock(struct test_##class##_data *d)		\
    {											\
    if (type_trylock(&d.lock)) {							\
    op(d.counter);								\
    type_unlock(&d.lock);							\
    }										\
    }											\
    static void __used test_##class##_assert(struct test_##class##_data *d)			\
    {											\
    lockdep_assert_held(&d.lock);							\
    op(d.counter);									\
    }											\
    static void __used test_##class##_guard(struct test_##class##_data *d)			\
    {											\
    { guard(class)(&d.lock);		op(d.counter); }			\
    { guard(class##_irq)(&d.lock);		op(d.counter); }			\
    { guard(class##_irqsave)(&d.lock);	op(d.counter); }			\
    }

    TEST_SPINLOCK_COMMON(raw_spinlock,
    raw_spinlock_t,
    raw_spinlock_init,
    raw_spin_lock,
    raw_spin_unlock,
    raw_spin_trylock,
    TEST_OP_RW);
#[no_mangle]
unsafe extern "C" fn test_raw_spinlock_trylock_extra(d: *mut test_raw_spinlock_data) -> void __used {
    static void __used test_raw_spinlock_trylock_extra(struct test_raw_spinlock_data *d)
    {
    unsigned long flags;
    data_race(d.counter++); /* no warning */
    if (raw_spin_trylock_irq(&d.lock)) {
    d.counter++;
    raw_spin_unlock_irq(&d.lock);
    }
    if (raw_spin_trylock_irqsave(&d.lock, flags)) {
    d.counter++;
    raw_spin_unlock_irqrestore(&d.lock, flags);
    }
    scoped_cond_guard(raw_spinlock_try, return, &d.lock) {
    d.counter++;
    }
    }
    TEST_SPINLOCK_COMMON(spinlock,
    spinlock_t,
    spinlock_init,
    spin_lock,
    spin_unlock,
    spin_trylock,
    TEST_OP_RW);
#[no_mangle]
unsafe extern "C" fn test_spinlock_trylock_extra(d: *mut test_spinlock_data) -> void __used {
    static void __used test_spinlock_trylock_extra(struct test_spinlock_data *d)
    {
    unsigned long flags;
    if (spin_trylock_irq(&d.lock)) {
    d.counter++;
    spin_unlock_irq(&d.lock);
    }
    if (spin_trylock_irqsave(&d.lock, flags)) {
    d.counter++;
    spin_unlock_irqrestore(&d.lock, flags);
    }
    scoped_cond_guard(spinlock_try, return, &d.lock) {
    d.counter++;
    }
    }
    TEST_SPINLOCK_COMMON(write_lock,
    rwlock_t,
    rwlock_init,
    write_lock,
    write_unlock,
    write_trylock,
    TEST_OP_RW);
#[no_mangle]
unsafe extern "C" fn test_write_trylock_extra(d: *mut test_write_lock_data) -> void __used {
    static void __used test_write_trylock_extra(struct test_write_lock_data *d)
    {
    unsigned long flags;
    if (write_trylock_irqsave(&d.lock, flags)) {
    d.counter++;
    write_unlock_irqrestore(&d.lock, flags);
    }
    }
    TEST_SPINLOCK_COMMON(read_lock,
    rwlock_t,
    rwlock_init,
    read_lock,
    read_unlock,
    read_trylock,
    TEST_OP_RO);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_mutex_data {
    pub mtx: mutex,
    pub __guarded_by(&mtx): int counter,
    pub mtx2: mutex,
    pub &mtx2): int anyread __guarded_by(&mtx,,
    pub &mtx2): *mut *mut int anyptr __pt_guarded_by(&mtx,,
}

#[no_mangle]
unsafe extern "C" fn test_mutex_init(d: *mut test_mutex_data) -> void __used {
    static void __used test_mutex_init(struct test_mutex_data *d)
    {
    guard(mutex_init)(&d.mtx);
    d.counter = 0;
    }
#[no_mangle]
unsafe extern "C" fn test_mutex_lock(d: *mut test_mutex_data) -> void __used {
    static void __used test_mutex_lock(struct test_mutex_data *d)
    {
    mutex_lock(&d.mtx);
    d.counter++;
    mutex_unlock(&d.mtx);
    mutex_lock_io(&d.mtx);
    d.counter++;
    mutex_unlock(&d.mtx);
    }
#[no_mangle]
unsafe extern "C" fn test_mutex_trylock(d: *mut test_mutex_data, a: *mut core::sync::atomic::AtomicI32) -> void __used {
    static void __used test_mutex_trylock(struct test_mutex_data *d, atomic_t *a)
    {
    if (!mutex_lock_interruptible(&d.mtx)) {
    d.counter++;
    mutex_unlock(&d.mtx);
    }
    if (!mutex_lock_killable(&d.mtx)) {
    d.counter++;
    mutex_unlock(&d.mtx);
    }
    if (mutex_trylock(&d.mtx)) {
    d.counter++;
    mutex_unlock(&d.mtx);
    }
    if (atomic_dec_and_mutex_lock(a, &d.mtx)) {
    d.counter++;
    mutex_unlock(&d.mtx);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_mutex_assert(d: *mut test_mutex_data) -> void __used {
    static void __used test_mutex_assert(struct test_mutex_data *d)
    {
    lockdep_assert_held(&d.mtx);
    d.counter++;
    }
#[no_mangle]
unsafe extern "C" fn test_mutex_guard(d: *mut test_mutex_data) -> void __used {
    static void __used test_mutex_guard(struct test_mutex_data *d)
    {
    guard(mutex)(&d.mtx);
    d.counter++;
    }
#[no_mangle]
unsafe extern "C" fn test_mutex_cond_guard(d: *mut test_mutex_data) -> void __used {
    static void __used test_mutex_cond_guard(struct test_mutex_data *d)
    {
    scoped_cond_guard(mutex_try, return, &d.mtx) {
    d.counter++;
    }
    scoped_cond_guard(mutex_intr, return, &d.mtx) {
    d.counter++;
    }
    }
#[no_mangle]
unsafe extern "C" fn test_mutex_multiguard(d: *mut test_mutex_data) -> void __used {
    static void __used test_mutex_multiguard(struct test_mutex_data *d)
    {
    mutex_lock(&d.mtx);
    (void)d.anyread;
    (void)*d.anyptr;
    mutex_unlock(&d.mtx);
    mutex_lock(&d.mtx2);
    (void)d.anyread;
    (void)*d.anyptr;
    mutex_unlock(&d.mtx2);
    mutex_lock(&d.mtx);
    mutex_lock(&d.mtx2);
    d.anyread++;
    (*d.anyptr)++;
    mutex_unlock(&d.mtx2);
    mutex_unlock(&d.mtx);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_seqlock_data {
    pub sl: seqlock_t,
    pub __guarded_by(&sl): int counter,
}

#[no_mangle]
unsafe extern "C" fn test_seqlock_init(d: *mut test_seqlock_data) -> void __used {
    static void __used test_seqlock_init(struct test_seqlock_data *d)
    {
    guard(seqlock_init)(&d.sl);
    d.counter = 0;
    }
#[no_mangle]
unsafe extern "C" fn test_seqlock_reader(d: *mut test_seqlock_data) -> void __used {
    static void __used test_seqlock_reader(struct test_seqlock_data *d)
    {
    unsigned int seq;
    do {
    seq = read_seqbegin(&d.sl);
    (void)d.counter;
    } while (read_seqretry(&d.sl, seq));
    }
#[no_mangle]
unsafe extern "C" fn test_seqlock_writer(d: *mut test_seqlock_data) -> void __used {
    static void __used test_seqlock_writer(struct test_seqlock_data *d)
    {
    unsigned long flags;
    write_seqlock(&d.sl);
    d.counter++;
    write_sequnlock(&d.sl);
    write_seqlock_irq(&d.sl);
    d.counter++;
    write_sequnlock_irq(&d.sl);
    write_seqlock_bh(&d.sl);
    d.counter++;
    write_sequnlock_bh(&d.sl);
    write_seqlock_irqsave(&d.sl, flags);
    d.counter++;
    write_sequnlock_irqrestore(&d.sl, flags);
    }
#[no_mangle]
unsafe extern "C" fn test_seqlock_scoped(d: *mut test_seqlock_data) -> void __used {
    static void __used test_seqlock_scoped(struct test_seqlock_data *d)
    {
    scoped_seqlock_read (&d.sl, ss_lockless) {
    (void)d.counter;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_rwsem_data {
    pub sem: rw_semaphore,
    pub __guarded_by(&sem): int counter,
}

#[no_mangle]
unsafe extern "C" fn test_rwsem_init(d: *mut test_rwsem_data) -> void __used {
    static void __used test_rwsem_init(struct test_rwsem_data *d)
    {
    guard(rwsem_init)(&d.sem);
    d.counter = 0;
    }
#[no_mangle]
unsafe extern "C" fn test_rwsem_reader(d: *mut test_rwsem_data) -> void __used {
    static void __used test_rwsem_reader(struct test_rwsem_data *d)
    {
    down_read(&d.sem);
    (void)d.counter;
    up_read(&d.sem);
    if (down_read_trylock(&d.sem)) {
    (void)d.counter;
    up_read(&d.sem);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_rwsem_writer(d: *mut test_rwsem_data) -> void __used {
    static void __used test_rwsem_writer(struct test_rwsem_data *d)
    {
    down_write(&d.sem);
    d.counter++;
    up_write(&d.sem);
    down_write(&d.sem);
    d.counter++;
    downgrade_write(&d.sem);
    (void)d.counter;
    up_read(&d.sem);
    if (down_write_trylock(&d.sem)) {
    d.counter++;
    up_write(&d.sem);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_rwsem_assert(d: *mut test_rwsem_data) -> void __used {
    static void __used test_rwsem_assert(struct test_rwsem_data *d)
    {
    rwsem_assert_held_nolockdep(&d.sem);
    d.counter++;
    }
#[no_mangle]
unsafe extern "C" fn test_rwsem_guard(d: *mut test_rwsem_data) -> void __used {
    static void __used test_rwsem_guard(struct test_rwsem_data *d)
    {
    { guard(rwsem_read)(&d.sem); (void)d.counter; }
    { guard(rwsem_write)(&d.sem); d.counter++; }
    }
#[no_mangle]
unsafe extern "C" fn test_rwsem_cond_guard(d: *mut test_rwsem_data) -> void __used {
    static void __used test_rwsem_cond_guard(struct test_rwsem_data *d)
    {
    scoped_cond_guard(rwsem_read_try, return, &d.sem) {
    (void)d.counter;
    }
    scoped_cond_guard(rwsem_write_try, return, &d.sem) {
    d.counter++;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_bit_spinlock_data {
    pub bits: c_ulong,
    pub &bits)): int counter __guarded_by(__bitlock(3,,
}

#[no_mangle]
unsafe extern "C" fn test_bit_spin_lock(d: *mut test_bit_spinlock_data) -> void __used {
    static void __used test_bit_spin_lock(struct test_bit_spinlock_data *d)
    {
//
// Note, the analysis seems to have false negatives, because it won't
// precisely recognize the bit of the fake __bitlock() token.
//
    bit_spin_lock(3, &d.bits);
    d.counter++;
    bit_spin_unlock(3, &d.bits);
    bit_spin_lock(3, &d.bits);
    d.counter++;
    __bit_spin_unlock(3, &d.bits);
    if (bit_spin_trylock(3, &d.bits)) {
    d.counter++;
    bit_spin_unlock(3, &d.bits);
    }
    }
//
// Test that we can mark a variable guarded by RCU, and we can dereference and
// write to the pointer with RCU's primitives.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_rcu_data {
    pub data: *mut long __rcu_guarded,
}

#[no_mangle]
unsafe extern "C" fn test_rcu_guarded_reader(d: *mut test_rcu_data) -> void __used {
    static void __used test_rcu_guarded_reader(struct test_rcu_data *d)
    {
    rcu_read_lock();
    (void)rcu_dereference(d.data);
    rcu_read_unlock();
    rcu_read_lock_bh();
    (void)rcu_dereference(d.data);
    rcu_read_unlock_bh();
    rcu_read_lock_sched();
    (void)rcu_dereference(d.data);
    rcu_read_unlock_sched();
    }
#[no_mangle]
unsafe extern "C" fn test_rcu_guard(d: *mut test_rcu_data) -> void __used {
    static void __used test_rcu_guard(struct test_rcu_data *d)
    {
    guard(rcu)();
    (void)rcu_dereference(d.data);
    }
#[no_mangle]
unsafe extern "C" fn test_rcu_guarded_updater(d: *mut test_rcu_data) -> void __used {
    static void __used test_rcu_guarded_updater(struct test_rcu_data *d)
    {
    rcu_assign_pointer(d.data, core::ptr::null_mut());
    RCU_INIT_POINTER(d.data, core::ptr::null_mut());
    (void)unrcu_pointer(d.data);
    }
    static void wants_rcu_held(void)	__must_hold_shared(RCU)       { }
    static void wants_rcu_held_bh(void)	__must_hold_shared(RCU_BH)    { }
    static void wants_rcu_held_sched(void)	__must_hold_shared(RCU_SCHED) { }
#[no_mangle]
unsafe extern "C" fn test_rcu_lock_variants() -> void __used {
    static void __used test_rcu_lock_variants(void)
    {
    rcu_read_lock();
    wants_rcu_held();
    rcu_read_unlock();
    rcu_read_lock_bh();
    wants_rcu_held_bh();
    rcu_read_unlock_bh();
    rcu_read_lock_sched();
    wants_rcu_held_sched();
    rcu_read_unlock_sched();
    }
#[no_mangle]
unsafe extern "C" fn test_rcu_lock_reentrant() -> void __used {
    static void __used test_rcu_lock_reentrant(void)
    {
    rcu_read_lock();
    rcu_read_lock();
    rcu_read_lock_bh();
    rcu_read_lock_bh();
    rcu_read_lock_sched();
    rcu_read_lock_sched();
    rcu_read_unlock_sched();
    rcu_read_unlock_sched();
    rcu_read_unlock_bh();
    rcu_read_unlock_bh();
    rcu_read_unlock();
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn test_rcu_assert_variants() -> void __used {
    static void __used test_rcu_assert_variants(void)
    {
    lockdep_assert_in_rcu_read_lock();
    wants_rcu_held();
    lockdep_assert_in_rcu_read_lock_bh();
    wants_rcu_held_bh();
    lockdep_assert_in_rcu_read_lock_sched();
    wants_rcu_held_sched();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_srcu_data {
    pub srcu: srcu_struct,
    pub data: *mut long __rcu_guarded,
}

#[no_mangle]
unsafe extern "C" fn test_srcu(d: *mut test_srcu_data) -> void __used {
    static void __used test_srcu(struct test_srcu_data *d)
    {
    init_srcu_struct(&d.srcu);
    let mut idx: c_int = srcu_read_lock(&d.srcu);
    long *data = srcu_dereference(d.data, &d.srcu);
    (void)data;
    srcu_read_unlock(&d.srcu, idx);
    rcu_assign_pointer(d.data, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_srcu_guard(d: *mut test_srcu_data) -> void __used {
    static void __used test_srcu_guard(struct test_srcu_data *d)
    {
    { guard(srcu)(&d.srcu); (void)srcu_dereference(d.data, &d.srcu); }
    { guard(srcu_fast)(&d.srcu); (void)srcu_dereference(d.data, &d.srcu); }
    { guard(srcu_fast_notrace)(&d.srcu); (void)srcu_dereference(d.data, &d.srcu); }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_local_lock_data {
    pub lock: local_lock_t,
    pub __guarded_by(&lock): int counter,
}

    static DEFINE_PER_CPU(struct test_local_lock_data, test_local_lock_data) = {
    .lock = INIT_LOCAL_LOCK(lock),
    };
#[no_mangle]
unsafe extern "C" fn test_local_lock_init(d: *mut test_local_lock_data) -> void __used {
    static void __used test_local_lock_init(struct test_local_lock_data *d)
    {
    guard(local_lock_init)(&d.lock);
    d.counter = 0;
    }
#[no_mangle]
unsafe extern "C" fn test_local_lock() -> void __used {
    static void __used test_local_lock(void)
    {
    unsigned long flags;
    local_lock(&test_local_lock_data.lock);
    this_cpu_add(test_local_lock_data.counter, 1);
    local_unlock(&test_local_lock_data.lock);
    local_lock_irq(&test_local_lock_data.lock);
    this_cpu_add(test_local_lock_data.counter, 1);
    local_unlock_irq(&test_local_lock_data.lock);
    local_lock_irqsave(&test_local_lock_data.lock, flags);
    this_cpu_add(test_local_lock_data.counter, 1);
    local_unlock_irqrestore(&test_local_lock_data.lock, flags);
    local_lock_nested_bh(&test_local_lock_data.lock);
    this_cpu_add(test_local_lock_data.counter, 1);
    local_unlock_nested_bh(&test_local_lock_data.lock);
    }
#[no_mangle]
unsafe extern "C" fn test_local_lock_guard() -> void __used {
    static void __used test_local_lock_guard(void)
    {
    { guard(local_lock)(&test_local_lock_data.lock); this_cpu_add(test_local_lock_data.counter, 1); }
    { guard(local_lock_irq)(&test_local_lock_data.lock); this_cpu_add(test_local_lock_data.counter, 1); }
    { guard(local_lock_irqsave)(&test_local_lock_data.lock); this_cpu_add(test_local_lock_data.counter, 1); }
    { guard(local_lock_nested_bh)(&test_local_lock_data.lock); this_cpu_add(test_local_lock_data.counter, 1); }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_local_trylock_data {
    pub lock: local_trylock_t,
    pub __guarded_by(&lock): int counter,
}

    static DEFINE_PER_CPU(struct test_local_trylock_data, test_local_trylock_data) = {
    .lock = INIT_LOCAL_TRYLOCK(lock),
    };
#[no_mangle]
unsafe extern "C" fn test_local_trylock_init(d: *mut test_local_trylock_data) -> void __used {
    static void __used test_local_trylock_init(struct test_local_trylock_data *d)
    {
    guard(local_trylock_init)(&d.lock);
    d.counter = 0;
    }
#[no_mangle]
unsafe extern "C" fn test_local_trylock() -> void __used {
    static void __used test_local_trylock(void)
    {
    local_lock(&test_local_trylock_data.lock);
    this_cpu_add(test_local_trylock_data.counter, 1);
    local_unlock(&test_local_trylock_data.lock);
    if (local_trylock(&test_local_trylock_data.lock)) {
    this_cpu_add(test_local_trylock_data.counter, 1);
    local_unlock(&test_local_trylock_data.lock);
    }
    }
    static DEFINE_WD_CLASS(ww_class);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_ww_mutex_data {
    pub mtx: ww_mutex,
    pub __guarded_by(&mtx): int counter,
}

#[no_mangle]
unsafe extern "C" fn test_ww_mutex_lock_noctx(d: *mut test_ww_mutex_data) -> void __used {
    static void __used test_ww_mutex_lock_noctx(struct test_ww_mutex_data *d)
    {
    if (!ww_mutex_lock(&d.mtx, core::ptr::null_mut())) {
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    }
    if (!ww_mutex_lock_interruptible(&d.mtx, core::ptr::null_mut())) {
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    }
    if (ww_mutex_trylock(&d.mtx, core::ptr::null_mut())) {
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    }
    ww_mutex_lock_slow(&d.mtx, core::ptr::null_mut());
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    ww_mutex_destroy(&d.mtx);
    }
#[no_mangle]
unsafe extern "C" fn test_ww_mutex_lock_ctx(d: *mut test_ww_mutex_data) -> void __used {
    static void __used test_ww_mutex_lock_ctx(struct test_ww_mutex_data *d)
    {
    struct ww_acquire_ctx ctx;
    ww_acquire_init(&ctx, &ww_class);
    if (!ww_mutex_lock(&d.mtx, &ctx)) {
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    }
    if (!ww_mutex_lock_interruptible(&d.mtx, &ctx)) {
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    }
    if (ww_mutex_trylock(&d.mtx, &ctx)) {
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    }
    ww_mutex_lock_slow(&d.mtx, &ctx);
    d.counter++;
    ww_mutex_unlock(&d.mtx);
    ww_acquire_done(&ctx);
    ww_acquire_fini(&ctx);
    ww_mutex_destroy(&d.mtx);
    }
    static DEFINE_PER_CPU(raw_spinlock_t, test_per_cpu_lock);
#[no_mangle]
unsafe extern "C" fn test_per_cpu(cpu: c_int) -> void __used {
    static void __used test_per_cpu(int cpu)
    {
    raw_spin_lock(&per_cpu(test_per_cpu_lock, cpu));
    raw_spin_unlock(&per_cpu(test_per_cpu_lock, cpu));
    raw_spin_lock(per_cpu_ptr(&test_per_cpu_lock, cpu));
    raw_spin_unlock(per_cpu_ptr(&test_per_cpu_lock, cpu));
    }
