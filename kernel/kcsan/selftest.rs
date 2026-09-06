//! Automatically rewritten from C to Rust
//! Source: kernel/kcsan/selftest.c
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
//
// KCSAN short boot-time selftests.
//
// Copyright (C) 2019, Google LLC.
//

pub const ITERS_PER_TEST: c_int = 2000;
//
// Test watchpoint encode and decode: check that encoding some access's info,
// and then subsequent decode preserves the access's info.
//
#[no_mangle]
unsafe extern "C" fn test_encode_decode() -> bool __init {
    static bool __init test_encode_decode(void)
    {
    int i;
    for (i = 0; i < ITERS_PER_TEST; ++i) {
    let mut size: usize = get_random_u32_inclusive(1, MAX_ENCODABLE_SIZE);
    let mut is_write: bool = !!get_random_u32_below(2);
    unsigned long verif_masked_addr;
    long encoded_watchpoint;
    bool verif_is_write;
    unsigned long addr;
    size_t verif_size;
    get_random_bytes(&addr, sizeof(addr));
    if (addr < PAGE_SIZE)
    addr = PAGE_SIZE;
    if (WARN_ON(!check_encodable(addr, size)))
    return false;
    encoded_watchpoint = encode_watchpoint(addr, size, is_write);
// Check special watchpoints
    if (WARN_ON(decode_watchpoint(INVALID_WATCHPOINT, &verif_masked_addr, &verif_size, &verif_is_write)))
    return false;
    if (WARN_ON(decode_watchpoint(CONSUMED_WATCHPOINT, &verif_masked_addr, &verif_size, &verif_is_write)))
    return false;
// Check decoding watchpoint returns same data
    if (WARN_ON(!decode_watchpoint(encoded_watchpoint, &verif_masked_addr, &verif_size, &verif_is_write)))
    return false;
    if (WARN_ON(verif_masked_addr != (addr & WATCHPOINT_ADDR_MASK)))
    goto fail;
    if (WARN_ON(verif_size != size))
    goto fail;
    if (WARN_ON(is_write != verif_is_write))
    goto fail;
    continue;
    fail:
    pr_err("%s fail: %s %zu bytes @ %lx . encoded: %lx . %s %zu bytes @ %lx\n",
    __func__, is_write ? "write" : "read", size, addr, encoded_watchpoint,
    verif_is_write ? "write" : "read", verif_size, verif_masked_addr);
    return false;
    }
    return true;
    }
// Test access matching function.
#[no_mangle]
unsafe extern "C" fn test_matching_access() -> bool __init {
    static bool __init test_matching_access(void)
    {
    if (WARN_ON(!matching_access(10, 1, 10, 1)))
    return false;
    if (WARN_ON(!matching_access(10, 2, 11, 1)))
    return false;
    if (WARN_ON(!matching_access(10, 1, 9, 2)))
    return false;
    if (WARN_ON(matching_access(10, 1, 11, 1)))
    return false;
    if (WARN_ON(matching_access(9, 1, 10, 1)))
    return false;
//
// An access of size 0 could match another access, as demonstrated here.
// Rather than add more comparisons to 'matching_access()', which would
// end up in the fast-path for *all* checks, check_access() simply
// returns for all accesses of size 0.
//
    if (WARN_ON(!matching_access(8, 8, 12, 0)))
    return false;
    return true;
    }
//
// Correct memory barrier instrumentation is critical to avoiding false
// positives: simple test to check at boot certain barriers are always properly
// instrumented. See kcsan_test for a more complete test.
//
    static DEFINE_SPINLOCK(test_spinlock);
#[no_mangle]
unsafe extern "C" fn test_barrier() -> bool __init {
    static bool __init test_barrier(void)
    {

    struct kcsan_scoped_access *reorder_access = &current.kcsan_ctx.reorder_access;

    struct kcsan_scoped_access *reorder_access = core::ptr::null_mut();

    let mut ret: bool = true;
    let mut arch_spinlock: arch_spinlock_t = __ARCH_SPIN_LOCK_UNLOCKED;
    atomic_t dummy;
    long test_var;
    if (!reorder_access || !IS_ENABLED(CONFIG_SMP))
    return true;

    do {											\
    reorder_access.type = (access_type) | KCSAN_ACCESS_SCOPED;			\
    reorder_access.size = 1;							\
    barrier;									\
    if (reorder_access.size != 0) {						\
    pr_err("improperly instrumented type=(" #access_type "): " name "\n");	\
    ret = false;								\
    }										\
    } while (0)

    kcsan_nestable_atomic_begin(); /* No watchpoints in called functions. */
    KCSAN_CHECK_READ_BARRIER(mb());
    KCSAN_CHECK_READ_BARRIER(rmb());
    KCSAN_CHECK_READ_BARRIER(smp_mb());
    KCSAN_CHECK_READ_BARRIER(smp_rmb());
    KCSAN_CHECK_READ_BARRIER(dma_rmb());
    KCSAN_CHECK_READ_BARRIER(smp_mb__before_atomic());
    KCSAN_CHECK_READ_BARRIER(smp_mb__after_atomic());
    KCSAN_CHECK_READ_BARRIER(smp_mb__after_spinlock());
    KCSAN_CHECK_READ_BARRIER(smp_store_mb(test_var, 0));
    KCSAN_CHECK_READ_BARRIER(smp_store_release(&test_var, 0));
    KCSAN_CHECK_READ_BARRIER(xchg(&test_var, 0));
    KCSAN_CHECK_READ_BARRIER(xchg_release(&test_var, 0));
    KCSAN_CHECK_READ_BARRIER(cmpxchg(&test_var, 0,  0));
    KCSAN_CHECK_READ_BARRIER(cmpxchg_release(&test_var, 0,  0));
    KCSAN_CHECK_READ_BARRIER(atomic_set_release(&dummy, 0));
    KCSAN_CHECK_READ_BARRIER(atomic_add_return(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(atomic_add_return_release(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(atomic_fetch_add(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(atomic_fetch_add_release(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(test_and_set_bit(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(test_and_clear_bit(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(test_and_change_bit(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(clear_bit_unlock(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(__clear_bit_unlock(0, &test_var));
    arch_spin_lock(&arch_spinlock);
    KCSAN_CHECK_READ_BARRIER(arch_spin_unlock(&arch_spinlock));
    spin_lock(&test_spinlock);
    KCSAN_CHECK_READ_BARRIER(spin_unlock(&test_spinlock));
    KCSAN_CHECK_WRITE_BARRIER(mb());
    KCSAN_CHECK_WRITE_BARRIER(wmb());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb());
    KCSAN_CHECK_WRITE_BARRIER(smp_wmb());
    KCSAN_CHECK_WRITE_BARRIER(dma_wmb());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb__before_atomic());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb__after_atomic());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb__after_spinlock());
    KCSAN_CHECK_WRITE_BARRIER(smp_store_mb(test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(smp_store_release(&test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(xchg(&test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(xchg_release(&test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(cmpxchg(&test_var, 0,  0));
    KCSAN_CHECK_WRITE_BARRIER(cmpxchg_release(&test_var, 0,  0));
    KCSAN_CHECK_WRITE_BARRIER(atomic_set_release(&dummy, 0));
    KCSAN_CHECK_WRITE_BARRIER(atomic_add_return(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(atomic_add_return_release(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(atomic_fetch_add(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(atomic_fetch_add_release(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(test_and_set_bit(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(test_and_clear_bit(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(test_and_change_bit(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(clear_bit_unlock(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(__clear_bit_unlock(0, &test_var));
    arch_spin_lock(&arch_spinlock);
    KCSAN_CHECK_WRITE_BARRIER(arch_spin_unlock(&arch_spinlock));
    spin_lock(&test_spinlock);
    KCSAN_CHECK_WRITE_BARRIER(spin_unlock(&test_spinlock));
    KCSAN_CHECK_RW_BARRIER(mb());
    KCSAN_CHECK_RW_BARRIER(wmb());
    KCSAN_CHECK_RW_BARRIER(rmb());
    KCSAN_CHECK_RW_BARRIER(smp_mb());
    KCSAN_CHECK_RW_BARRIER(smp_wmb());
    KCSAN_CHECK_RW_BARRIER(smp_rmb());
    KCSAN_CHECK_RW_BARRIER(dma_wmb());
    KCSAN_CHECK_RW_BARRIER(dma_rmb());
    KCSAN_CHECK_RW_BARRIER(smp_mb__before_atomic());
    KCSAN_CHECK_RW_BARRIER(smp_mb__after_atomic());
    KCSAN_CHECK_RW_BARRIER(smp_mb__after_spinlock());
    KCSAN_CHECK_RW_BARRIER(smp_store_mb(test_var, 0));
    KCSAN_CHECK_RW_BARRIER(smp_store_release(&test_var, 0));
    KCSAN_CHECK_RW_BARRIER(xchg(&test_var, 0));
    KCSAN_CHECK_RW_BARRIER(xchg_release(&test_var, 0));
    KCSAN_CHECK_RW_BARRIER(cmpxchg(&test_var, 0,  0));
    KCSAN_CHECK_RW_BARRIER(cmpxchg_release(&test_var, 0,  0));
    KCSAN_CHECK_RW_BARRIER(atomic_set_release(&dummy, 0));
    KCSAN_CHECK_RW_BARRIER(atomic_add_return(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(atomic_add_return_release(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(atomic_fetch_add(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(atomic_fetch_add_release(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(test_and_set_bit(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(test_and_clear_bit(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(test_and_change_bit(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(clear_bit_unlock(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(__clear_bit_unlock(0, &test_var));
    arch_spin_lock(&arch_spinlock);
    KCSAN_CHECK_RW_BARRIER(arch_spin_unlock(&arch_spinlock));
    spin_lock(&test_spinlock);
    KCSAN_CHECK_RW_BARRIER(spin_unlock(&test_spinlock));
    KCSAN_CHECK_RW_BARRIER(xor_unlock_is_negative_byte(1, &test_var));
    KCSAN_CHECK_READ_BARRIER(xor_unlock_is_negative_byte(1, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(xor_unlock_is_negative_byte(1, &test_var));
    kcsan_nestable_atomic_end();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kcsan_selftest() -> int __init {
    static int __init kcsan_selftest(void)
    {
    let mut passed: c_int = 0;
    let mut total: c_int = 0;

    do {                                                                   \
    ++total;                                                       \
    if (do_test())                                                 \
    ++passed;                                              \
    else                                                           \
    pr_err("selftest: " #do_test " failed");               \
    } while (0)
    RUN_TEST(test_encode_decode);
    RUN_TEST(test_matching_access);
    RUN_TEST(test_barrier);
    pr_info("selftest: %d/%d tests passed\n", passed, total);
    if (passed != total)
    panic("selftests failed");
    return 0;
    }
    postcore_initcall(kcsan_selftest);
