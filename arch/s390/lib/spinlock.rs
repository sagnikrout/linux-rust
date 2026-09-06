//! Automatically rewritten from C to Rust
//! Source: arch/s390/lib/spinlock.c
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
// Out of line spinlock code.
//
// Copyright IBM Corp. 2004, 2006
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
//

    let mut spin_retry: c_int = -1;
#[no_mangle]
unsafe extern "C" fn spin_retry_init() -> int __init {
    static int __init spin_retry_init(void)
    {
    if (spin_retry < 0)
    spin_retry = 1000;
    return 0;
    }
    early_initcall(spin_retry_init);
//
// spin_retry= parameter
//
#[no_mangle]
unsafe extern "C" fn spin_retry_setup(str: *mut c_char) -> int __init {
    static int __init spin_retry_setup(char *str)
    {
    spin_retry = simple_strtoul(str, &str, 0);
    return 1;
    }
    __setup("spin_retry=", spin_retry_setup);
    static const struct ctl_table s390_spin_sysctl_table[] = {
    {
    .procname	= "spin_retry",
    .data		= &spin_retry,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    };
#[no_mangle]
unsafe extern "C" fn init_s390_spin_sysctls() -> int __init {
    static int __init init_s390_spin_sysctls(void)
    {
    register_sysctl_init("kernel", s390_spin_sysctl_table);
    return 0;
    }
    arch_initcall(init_s390_spin_sysctls);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spin_wait {
    pub prev: *mut *mut spin_wait next,,
    pub node_id: c_int,
    pub __aligned(32): },
    pub spin_wait[4]): static DEFINE_PER_CPU_ALIGNED(struct spin_wait,,
pub const _Q_LOCK_CPU_OFFSET: c_int = 0;
pub const _Q_LOCK_STEAL_OFFSET: c_int = 16;
pub const _Q_TAIL_IDX_OFFSET: c_int = 18;
pub const _Q_TAIL_CPU_OFFSET: c_int = 20;
pub const _Q_LOCK_CPU_MASK: c_uint = 0x0000ffff;
pub const _Q_LOCK_STEAL_ADD: c_uint = 0x00010000;
pub const _Q_LOCK_STEAL_MASK: c_uint = 0x00030000;
pub const _Q_TAIL_IDX_MASK: c_uint = 0x000c0000;
pub const _Q_TAIL_CPU_MASK: c_uint = 0xfff00000;

#[no_mangle]
pub unsafe extern "C" fn arch_spin_lock_setup(cpu: c_int) {
    void arch_spin_lock_setup(int cpu)
    {
    pub node: *mut spin_wait,
    pub ix: c_int,
    pub cpu): node = per_cpu_ptr(&spin_wait[0],,
    pub {: for (ix = 0; ix < 4; ix++, node++),
    pub sizeof(*node)): *mut memset(node, 0,,
    node.node_id = ((cpu + 1) << _Q_TAIL_CPU_OFFSET) +
    pub _Q_TAIL_IDX_OFFSET): (ix <<,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_load_niai4(lock: *mut c_int) -> c_int {
    static inline int arch_load_niai4(int *lock)
    {
    pub owner: c_int,
    asm_inline volatile(
    ALTERNATIVE("nop", ".insn rre,0xb2fa0000,4,0", ALT_FACILITY(49)) /* NIAI 4 */
    "	l	%[owner],%[lock]"
    pub "memory"): *mut *mut : [owner] "=d" (owner) : [lock] "R" (lock) :,
    pub owner: return,
    }

#[no_mangle]
pub unsafe extern "C" fn arch_try_cmpxchg_niai8(lock: *mut c_int, old: c_int, new: c_int) -> c_int {
    static inline int arch_try_cmpxchg_niai8(int *lock, int old, int new)
    {
    pub cc: c_int,
    asm_inline volatile(
    ALTERNATIVE("nop", ".insn rre,0xb2fa0000,8,0", ALT_FACILITY(49)) /* NIAI 8 */
    "	cs	%[old],%[new],%[lock]"
    : [old] "+d" (old), [lock] "+Q" (*lock), "=@cc" (cc)
    : [new] "d" (new)
    pub "memory"): :,
    pub 0: return cc ==,
    }

#[no_mangle]
pub unsafe extern "C" fn arch_try_cmpxchg_niai8(lock: *mut c_int, old: c_int, new: c_int) -> c_int {
    static inline int arch_try_cmpxchg_niai8(int *lock, int old, int new)
    {
    pub old: int expected =,
    asm_inline volatile(
    ALTERNATIVE("nop", ".insn rre,0xb2fa0000,8,0", ALT_FACILITY(49)) /* NIAI 8 */
    "	cs	%[old],%[new],%[lock]"
    : [old] "+d" (old), [lock] "+Q" (*lock)
    : [new] "d" (new)
    pub "memory"): : "cc",,
    pub old: return expected ==,
    }

    static inline struct spin_wait *arch_spin_decode_tail(int lock)
    {
    pub cpu: int ix,,
    pub _Q_TAIL_IDX_OFFSET: ix = (lock & _Q_TAIL_IDX_MASK) >>,
    pub _Q_TAIL_CPU_OFFSET: cpu = (lock & _Q_TAIL_CPU_MASK) >>,
    pub 1): return per_cpu_ptr(&spin_wait[ix], cpu -,
    }
#[no_mangle]
pub unsafe extern "C" fn arch_spin_yield_target(lock: c_int, node: *mut spin_wait) -> c_int {
    static inline int arch_spin_yield_target(int lock, struct spin_wait *node)
    {
    if (lock & _Q_LOCK_CPU_MASK)
    pub _Q_LOCK_CPU_MASK: return lock &,
    if (node == core::ptr::null_mut() || node.prev == core::ptr::null_mut())
    pub /: *mut *mut return 0; / 0 -> no target cpu,
    while (node.prev)
    pub node->prev: node =,
    pub _Q_TAIL_CPU_OFFSET: return node->node_id >>,
    }
#[no_mangle]
pub unsafe extern "C" fn arch_spin_lock_queued(lp: *mut arch_spinlock_t) {
    static inline void arch_spin_lock_queued(arch_spinlock_t *lp)
    {
    pub next: *mut *mut spin_wait node,,
    pub count: int lockval, ix, node_id, tail_id, old, new, owner,,
    pub get_lowcore()->spinlock_index++: ix =,
    pub /: *mut *mut lockval = spinlock_lockval(); / cpu + 1,
    pub this_cpu_ptr(&spin_wait[ix]): node =,
    pub NULL: node->prev = node->next =,
    pub node->node_id: node_id =,
// Enqueue the node for this CPU in the spinlock wait queue
    pub READ_ONCE(lp->lock): old =,
    while (1) {
    if ((old & _Q_LOCK_CPU_MASK) == 0 &&
    (old & _Q_LOCK_STEAL_MASK) != _Q_LOCK_STEAL_MASK) {
//
// The lock is free but there may be waiters.
// With no waiters simply take the lock, if there
// are waiters try to steal the lock. The lock may
// be stolen three times before the next queued
// waiter will get the lock.
//
    pub lockval: new = (old ? (old + _Q_LOCK_STEAL_ADD) : 0) |,
    if (arch_try_cmpxchg(&lp.lock, &old, new))
// Got the lock
    pub out: goto,
// lock passing in progress
    }
// Make the node of this CPU the new tail.
    pub _Q_LOCK_MASK): new = node_id | (old &,
    if (arch_try_cmpxchg(&lp.lock, &old, new))
    }
// Set the 'next' pointer of the tail node in the queue
    pub _Q_TAIL_MASK: tail_id = old &,
    if (tail_id != 0) {
    pub arch_spin_decode_tail(tail_id): node->prev =,
    pub node): WRITE_ONCE(node->prev->next,,
    }
// Pass the virtual CPU to the lock holder if it is not running
    pub node): owner = arch_spin_yield_target(old,,
    if (owner && arch_vcpu_is_preempted(owner - 1))
    pub 1): smp_yield_cpu(owner -,
// Spin on the CPU local node->prev pointer
    if (tail_id != 0) {
    pub spin_retry: count =,
    while (READ_ONCE(node.prev) != core::ptr::null_mut()) {
    if (count-- >= 0)
    pub spin_retry: count =,
// Query running state of lock holder again.
    pub node): owner = arch_spin_yield_target(old,,
    if (owner && arch_vcpu_is_preempted(owner - 1))
    pub 1): smp_yield_cpu(owner -,
    }
    }
// Spin on the lock value in the spinlock_t
    pub spin_retry: count =,
    while (1) {
    pub READ_ONCE(lp->lock): old =,
    pub _Q_LOCK_CPU_MASK: owner = old &,
    if (!owner) {
    pub _Q_TAIL_MASK: tail_id = old &,
    pub lockval: new = ((tail_id != node_id) ? tail_id : 0) |,
    if (arch_try_cmpxchg(&lp.lock, &old, new))
// Got the lock
    }
    if (count-- >= 0)
    pub spin_retry: count =,
    if (!machine_is_lpar() || arch_vcpu_is_preempted(owner - 1))
    pub 1): smp_yield_cpu(owner -,
    }
// Pass lock_spin job to next CPU in the queue
    if (node_id && tail_id != node_id) {
// Wait until the next CPU has set up the 'next' pointer
    while ((next = READ_ONCE(node.next)) == core::ptr::null_mut())
    pub NULL: next->prev =,
    }
    out:
    }
#[no_mangle]
pub unsafe extern "C" fn arch_spin_lock_classic(lp: *mut arch_spinlock_t) {
    static inline void arch_spin_lock_classic(arch_spinlock_t *lp)
    {
    pub count: int lockval, old, new, owner,,
    pub /: *mut *mut lockval = spinlock_lockval(); / cpu + 1,
// Pass the virtual CPU to the lock holder if it is not running
    pub NULL): owner = arch_spin_yield_target(READ_ONCE(lp->lock),,
    if (owner && arch_vcpu_is_preempted(owner - 1))
    pub 1): smp_yield_cpu(owner -,
    pub spin_retry: count =,
    while (1) {
    pub arch_load_niai4(&lp->lock): old =,
    pub _Q_LOCK_CPU_MASK: owner = old &,
// Try to get the lock if it is free.
    if (!owner) {
    pub lockval: new = (old & _Q_TAIL_MASK) |,
    if (arch_try_cmpxchg_niai8(&lp.lock, old, new)) {
// Got the lock
    }
    }
    if (count-- >= 0)
    pub spin_retry: count =,
    if (!machine_is_lpar() || arch_vcpu_is_preempted(owner - 1))
    pub 1): smp_yield_cpu(owner -,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_spin_lock_wait(lp: *mut arch_spinlock_t) {
    void arch_spin_lock_wait(arch_spinlock_t *lp)
    {
    pub LCB_F_SPIN): trace_contention_begin(lp,,
    if (test_cpu_flag(CIF_DEDICATED_CPU))
    else
    pub 0): trace_contention_end(lp,,
    }
#[no_mangle]
pub unsafe extern "C" fn arch_spin_trylock_retry(lp: *mut arch_spinlock_t) -> c_int {
    int arch_spin_trylock_retry(arch_spinlock_t *lp)
    {
    pub spinlock_lockval(): int cpu =,
    pub count: int owner,,
    pub {: for (count = spin_retry; count > 0; count--),
    pub READ_ONCE(lp->lock): owner =,
// Try to get the lock if it is free.
    if (!owner) {
    if (arch_try_cmpxchg(&lp.lock, &owner, cpu))
    pub 1: return,
    }
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn arch_read_lock_wait(rw: *mut arch_rwlock_t) {
    void arch_read_lock_wait(arch_rwlock_t *rw)
    {
    if (unlikely(in_interrupt())) {
    while (READ_ONCE(rw.cnts) & 0x10000)
    }
// Remove this reader again to allow recursive read locking
    pub &rw->cnts): __atomic_add_const(-1,,
// Put the reader into the wait queue
// Now add this reader to the count value again
    pub &rw->cnts): __atomic_add_const(1,,
// Loop until the writer is done
    while (READ_ONCE(rw.cnts) & 0x10000)
    }
#[no_mangle]
pub unsafe extern "C" fn arch_write_lock_wait(rw: *mut arch_rwlock_t) {
    void arch_write_lock_wait(arch_rwlock_t *rw)
    {
    pub old: c_int,
// Add this CPU to the write waiters
    pub &rw->cnts): __atomic_add(0x20000,,
// Put the writer into the wait queue
    while (1) {
    pub READ_ONCE(rw->cnts): old =,
    if ((old & 0x1ffff) == 0 &&
    arch_try_cmpxchg(&rw.cnts, &old, old | 0x10000))
// Got the lock
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_spin_relax(lp: *mut arch_spinlock_t) {
    void arch_spin_relax(arch_spinlock_t *lp)
    {
    pub cpu: c_int,
    pub _Q_LOCK_CPU_MASK: cpu = READ_ONCE(lp->lock) &,
    if (!cpu)
    if (machine_is_lpar() && !arch_vcpu_is_preempted(cpu - 1))
    pub 1): smp_yield_cpu(cpu -,
    }
