//! Automatically rewritten from C to Rust
//! Source: lib/bucket_locks.c
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


// Allocate an array of spinlocks to be accessed by a hash. Two arguments
// indicate the number of elements to allocate in the array. max_size
// gives the maximum number of elements to allocate. cpu_mult gives
// the number of locks per CPU to allocate. The size is rounded up
// to a power of 2 to be suitable as a hash table.
//
    int __alloc_bucket_spinlocks(spinlock_t **locks, unsigned int *locks_mask,
    size_t max_size, unsigned int cpu_mult, gfp_t gfp,
    const char *name, struct lock_class_key *key)
    {
    spinlock_t *tlocks = core::ptr::null_mut();
    unsigned int i, size;

    let mut nr_pcpus: c_uint = 2;

    let mut nr_pcpus: c_uint = num_possible_cpus();

    if (cpu_mult) {
    nr_pcpus = min_t(unsigned int, nr_pcpus, 64UL);
    size = min_t(unsigned int, nr_pcpus * cpu_mult, max_size);
    } else {
    size = max_size;
    }
    if (sizeof(spinlock_t) != 0) {
    tlocks = kvmalloc_objs(spinlock_t, size, gfp);
    if (!tlocks)
    return -ENOMEM;
    for (i = 0; i < size; i++) {
    spin_lock_init(&tlocks[i]);
    lockdep_init_map(&tlocks[i].dep_map, name, key, 0);
    }
    }
// locks = tlocks;
// locks_mask = size - 1;
    return 0;
    }
    EXPORT_SYMBOL(__alloc_bucket_spinlocks);
#[no_mangle]
pub unsafe extern "C" fn free_bucket_spinlocks(locks: *mut spinlock_t) {
    void free_bucket_spinlocks(spinlock_t *locks)
    {
    kvfree(locks);
    }
    EXPORT_SYMBOL(free_bucket_spinlocks);
