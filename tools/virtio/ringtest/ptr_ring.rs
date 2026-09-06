//! Automatically rewritten from C to Rust
//! Source: tools/virtio/ringtest/ptr_ring.c
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


// SPDX-License-Identifier: GPL-2.0
// Macro flag: #define _GNU_SOURCE

pub const SMP_CACHE_BYTES: c_int = 64;

    typedef pthread_spinlock_t  spinlock_t;
    typedef int gfp_t;
pub const __GFP_ZERO: c_uint = 0x1;
    static void *kmalloc(unsigned size, gfp_t gfp)
    {
    void *p = memalign(64, size);
    if (!p)
    return p;
    if (gfp & __GFP_ZERO)
    memset(p, 0, size);
    return p;
    }
    static inline void *kzalloc(unsigned size, gfp_t flags)
    {
    return kmalloc(size, flags | __GFP_ZERO);
    }
    static inline void *kmalloc_array(size_t n, size_t size, gfp_t flags)
    {
    if (size != 0 && n > SIZE_MAX / size)
    return core::ptr::null_mut();
    return kmalloc(n * size, flags);
    }
    static inline void *kcalloc(size_t n, size_t size, gfp_t flags)
    {
    return kmalloc_array(n, size, flags | __GFP_ZERO);
    }
#[no_mangle]
unsafe extern "C" fn kfree(p: *mut c_void) {
    static void kfree(void *p)
    {
    if (p)
    free(p);
    }

#[no_mangle]
unsafe extern "C" fn spin_lock_init(lock: *mut spinlock_t) {
    static void spin_lock_init(spinlock_t *lock)
    {
    let mut r: c_int = pthread_spin_init(lock, 0);
    assert(!r);
    }
#[no_mangle]
unsafe extern "C" fn spin_lock(lock: *mut spinlock_t) {
    static void spin_lock(spinlock_t *lock)
    {
    let mut ret: c_int = pthread_spin_lock(lock);
    assert(!ret);
    }
#[no_mangle]
unsafe extern "C" fn spin_unlock(lock: *mut spinlock_t) {
    static void spin_unlock(spinlock_t *lock)
    {
    let mut ret: c_int = pthread_spin_unlock(lock);
    assert(!ret);
    }
#[no_mangle]
unsafe extern "C" fn spin_lock_bh(lock: *mut spinlock_t) {
    static void spin_lock_bh(spinlock_t *lock)
    {
    spin_lock(lock);
    }
#[no_mangle]
unsafe extern "C" fn spin_unlock_bh(lock: *mut spinlock_t) {
    static void spin_unlock_bh(spinlock_t *lock)
    {
    spin_unlock(lock);
    }
#[no_mangle]
unsafe extern "C" fn spin_lock_irq(lock: *mut spinlock_t) {
    static void spin_lock_irq(spinlock_t *lock)
    {
    spin_lock(lock);
    }
#[no_mangle]
unsafe extern "C" fn spin_unlock_irq(lock: *mut spinlock_t) {
    static void spin_unlock_irq(spinlock_t *lock)
    {
    spin_unlock(lock);
    }
#[no_mangle]
unsafe extern "C" fn spin_lock_irqsave(lock: *mut spinlock_t, f: c_ulong) {
    static void spin_lock_irqsave(spinlock_t *lock, unsigned long f)
    {
    spin_lock(lock);
    }
#[no_mangle]
unsafe extern "C" fn spin_unlock_irqrestore(lock: *mut spinlock_t, f: c_ulong) {
    static void spin_unlock_irqrestore(spinlock_t *lock, unsigned long f)
    {
    spin_unlock(lock);
    }

    static unsigned long long headcnt, tailcnt;
    static struct ptr_ring array ____cacheline_aligned_in_smp;
// implemented by ring
#[no_mangle]
pub unsafe extern "C" fn alloc_ring() {
    void alloc_ring(void)
    {
    let mut ret: c_int = ptr_ring_init(&array, ring_size, 0);
    assert(!ret);
// Hacky way to poke at ring internals. Useful for testing though.
    if (param)
    array.batch = param;
    }
// guest side
#[no_mangle]
pub unsafe extern "C" fn add_inbuf(len: unsigned, buf: *mut c_void, datap: *mut c_void) -> c_int {
    int add_inbuf(unsigned len, void *buf, void *datap)
    {
    int ret;
    ret = __ptr_ring_produce(&array, buf);
    if (ret >= 0) {
    ret = 0;
    headcnt++;
    }
    return ret;
    }
//
// ptr_ring API provides no way for producer to find out whether a given
// buffer was consumed.  Our tests merely require that a successful get_buf
// implies that add_inbuf succeed in the past, and that add_inbuf will succeed,
// fake it accordingly.
//
    void *get_buf(unsigned *lenp, void **bufp)
    {
    void *datap;
    if (tailcnt == headcnt || __ptr_ring_full(&array))
    datap = core::ptr::null_mut();
    else {
    datap = "Buffer\n";
    ++tailcnt;
    }
    return datap;
    }
#[no_mangle]
pub unsafe extern "C" fn used_empty() -> bool {
    bool used_empty()
    {
    return (tailcnt == headcnt || __ptr_ring_full(&array));
    }
#[no_mangle]
pub unsafe extern "C" fn disable_call() {
    void disable_call()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_call() -> bool {
    bool enable_call()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn kick_available() {
    void kick_available(void)
    {
    assert(0);
    }
// host side
#[no_mangle]
pub unsafe extern "C" fn disable_kick() {
    void disable_kick()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_kick() -> bool {
    bool enable_kick()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn avail_empty() -> bool {
    bool avail_empty()
    {
    return __ptr_ring_empty(&array);
    }
#[no_mangle]
pub unsafe extern "C" fn use_buf(lenp: *mut unsigned, bufp: *mut c_void) -> bool {
    bool use_buf(unsigned *lenp, void **bufp)
    {
    void *ptr;
    ptr = __ptr_ring_consume(&array);
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn call_used() {
    void call_used(void)
    {
    assert(0);
    }
