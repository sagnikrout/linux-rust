//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/seqlock.h
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
// seqcount_t / seqlock_t - a reader-writer consistency mechanism with
// lockless readers (read-only retry loops), and no writer starvation.
//
// See Documentation/locking/seqlock.rst
//
// Copyrights:
// - Based on x86_64 vsyscall gettimeofday: Keith Owens, Andrea Arcangeli
// - Sequence counters with associated locks, (C) 2020 Linutronix GmbH
//

//
// The seqlock seqcount_t interface does not prescribe a precise sequence of
// read begin/retry/end. For readers, typically there is a call to
// read_seqcount_begin() and read_seqcount_retry(), however, there are more
// esoteric cases which do not follow this pattern.
//
// As a consequence, we take the following best-effort approach for raw usage
// via seqcount_t under KCSAN: upon beginning a seq-reader critical section,
// pessimistically mark the next KCSAN_SEQLOCK_REGION_MAX memory accesses as
// atomics; if there is a matching read_seqcount_retry() call, no following
// memory operations are considered atomic. Usage of the seqlock_t interface
// is not affected.
//
pub const KCSAN_SEQLOCK_REGION_MAX: c_int = 1000;
//
// Make sure we are not reinitializing a held lock:
//

//
// seqcount_init() - runtime initializer for seqcount_t
// @s: Pointer to the seqcount_t instance
//

//
// SEQCNT_ZERO() - static initializer for seqcount_t
// @name: Name of the seqcount_t instance
//

//
// Sequence counters with associated locks (seqcount_LOCKNAME_t)
//
// A sequence counter which associates the lock used for writer
// serialization at initialization time. This enables lockdep to validate
// that the write side critical section is properly serialized.
//
// For associated locks which do not implicitly disable preemption,
// preemption protection is enforced in the write side function.
//
// Lockdep is never used in any for the raw write variants.
//
// See Documentation/locking/seqlock.rst
//
// typedef seqcount_LOCKNAME_t - sequence counter with LOCKNAME associated
// @seqcount:	The real sequence counter
// @lock:	Pointer to the associated lock
//
// A plain sequence counter with external writer synchronization by
// LOCKNAME @lock. The lock is associated to the sequence counter in the
// static initializer or init function. This enables lockdep to validate
// that the write side critical section is properly serialized.
//
// LOCKNAME:	raw_spinlock, spinlock, rwlock or mutex
//
// seqcount_LOCKNAME_init() - runtime initializer for seqcount_LOCKNAME_t
// @s:		Pointer to the seqcount_LOCKNAME_t instance
// @lock:	Pointer to the associated lock
//

//
// SEQCOUNT_LOCKNAME()	- Instantiate seqcount_LOCKNAME_t and helpers
// seqprop_LOCKNAME_*()	- Property accessors for seqcount_LOCKNAME_t
//
// @lockname:		"LOCKNAME" part of seqcount_LOCKNAME_t
// @locktype:		LOCKNAME canonical C data type
// @preemptible:	preemptibility of above locktype
// @lockbase:		prefix for associated lock/unlock
//

// \
// Re-read the sequence counter since the (possibly	\
// preempted) writer made progress.			\
// \
// PREEMPT_RT relies on the above LOCK+UNLOCK */		\
//
// __seqprop() for seqcount_t
//
extern "C" {
    pub fn smp_load_acquire(_arg: &s->sequence) -> return;
}

//
// SEQCNT_LOCKNAME_ZERO - static initializer for seqcount_LOCKNAME_t
// @name:	Name of the seqcount_LOCKNAME_t instance
// @lock:	Pointer to the associated LOCKNAME
//

//
// __read_seqcount_begin() - begin a seqcount_t read section
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// Return: count to be passed to read_seqcount_retry()
//

//
// raw_read_seqcount_begin() - begin a seqcount_t read section w/o lockdep
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// Return: count to be passed to read_seqcount_retry()
//

//
// read_seqcount_begin() - begin a seqcount_t read critical section
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// Return: count to be passed to read_seqcount_retry()
//

//
// raw_read_seqcount() - read the raw seqcount_t counter value
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// raw_read_seqcount opens a read critical section of the given
// seqcount_t, without any lockdep checking, and without checking or
// masking the sequence counter LSB. Calling code is responsible for
// handling that.
//
// Return: count to be passed to read_seqcount_retry()
//

//
// raw_seqcount_try_begin() - begin a seqcount_t read critical section
// w/o lockdep and w/o counter stabilization
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
// @start: count to be passed to read_seqcount_retry()
//
// Similar to raw_seqcount_begin(), except it enables eliding the critical
// section entirely if odd, instead of doing the speculation knowing it will
// fail.
//
// Useful when counter stabilization is more or less equivalent to taking
// the lock and there is a slowpath that does that.
//
// If true, start will be set to the (even) sequence count read.
//
// Return: true when a read critical section is started.
//

//
// raw_seqcount_begin() - begin a seqcount_t read critical section w/o
// lockdep and w/o counter stabilization
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// raw_seqcount_begin opens a read critical section of the given
// seqcount_t. Unlike read_seqcount_begin(), this function will not wait
// for the count to stabilize. If a writer is active when it begins, it
// will fail the read_seqcount_retry() at the end of the read critical
// section instead of stabilizing at the beginning of it.
//
// Use this only in special kernel hot paths where the read section is
// small and has a high probability of success through other external
// means. It will save a single branching instruction.
//
// Return: count to be passed to read_seqcount_retry()
//

// \
// If the counter is odd, let read_seqcount_retry() fail	\
// by decrementing the counter.					\
// \
//
// __read_seqcount_retry() - end a seqcount_t read section w/o barrier
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
// @start: count, from read_seqcount_begin()
//
// __read_seqcount_retry is like read_seqcount_retry, but has no smp_rmb()
// barrier. Callers should ensure that smp_rmb() or equivalent ordering is
// provided before actually loading any of the variables that are to be
// protected in this critical section.
//
// Use carefully, only in critical code, and comment how the barrier is
// provided.
//
// Return: true if a read section retry is required, else false
//

extern "C" {
    pub fn unlikely(start: READ_ONCE(s->sequence) !=) -> return;
}
//
// read_seqcount_retry() - end a seqcount_t read critical section
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
// @start: count, from read_seqcount_begin()
//
// read_seqcount_retry closes the read critical section of given
// seqcount_t.  If the critical section was invalid, it must be ignored
// (and typically retried).
//
// Return: true if a read section retry is required, else false
//

extern "C" {
    pub fn do___read_seqcount_retry(_arg: s, _arg: start) -> return;
}
//
// raw_write_seqcount_begin() - start a seqcount_t write section w/o lockdep
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// Context: check write_seqcount_begin()
//

//
// raw_write_seqcount_end() - end a seqcount_t write section w/o lockdep
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// Context: check write_seqcount_end()
//

//
// write_seqcount_begin_nested() - start a seqcount_t write section with
// custom lockdep nesting level
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
// @subclass: lockdep nesting level
//
// See Documentation/locking/lockdep-design.rst
// Context: check write_seqcount_begin()
//

//
// write_seqcount_begin() - start a seqcount_t write side critical section
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// Context: sequence counter write side sections must be serialized and
// non-preemptible. Preemption will be automatically disabled if and
// only if the seqcount write serialization lock is associated, and
// preemptible.  If readers can be invoked from hardirq or softirq
// context, interrupts or bottom halves must be respectively disabled.
//

//
// write_seqcount_end() - end a seqcount_t write side critical section
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// Context: Preemption will be automatically re-enabled if and only if
// the seqcount write serialization lock is associated, and preemptible.
//

//
// raw_write_seqcount_barrier() - do a seqcount_t write barrier
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// This can be used to provide an ordering guarantee instead of the usual
// consistency guarantee. It is one wmb cheaper, because it can collapse
// the two back-to-back wmb()s.
//
// Note that writes surrounding the barrier should be declared atomic (e.g.
// via WRITE_ONCE): a) to ensure the writes become visible to other threads
// atomically, avoiding compiler optimizations; b) to document which writes are
// meant to propagate to the reader critical section. This is necessary because
// neither writes before nor after the barrier are enclosed in a seq-writer
// critical section that would ensure readers are aware of ongoing writes::
//
// seqcount_t seq;
// bool X = true, Y = false;
//
// void read(void)
// {
// bool x, y;
//
// do {
// int s = read_seqcount_begin(&seq);
//
// x = X; y = Y;
//
// } while (read_seqcount_retry(&seq, s));
//
// BUG_ON(!x && !y);
// }
//
// void write(void)
// {
// WRITE_ONCE(Y, true);
//
// raw_write_seqcount_barrier(seq);
//
// WRITE_ONCE(X, false);
// }
//

//
// write_seqcount_invalidate() - invalidate in-progress seqcount_t read
// side operations
// @s: Pointer to seqcount_t or any of the seqcount_LOCKNAME_t variants
//
// After write_seqcount_invalidate, no seqcount_t read side operations
// will complete successfully and see data older than this.
//

//
// Latch sequence counters (seqcount_latch_t)
//
// A sequence counter variant where the counter even/odd value is used to
// switch between two copies of protected data. This allows the read path,
// typically NMIs, to safely interrupt the write side critical section.
//
// As the write sections are fully preemptible, no special handling for
// PREEMPT_RT is needed.
//
// SEQCNT_LATCH_ZERO() - static initializer for seqcount_latch_t
// @seq_name: Name of the seqcount_latch_t instance
//

//
// seqcount_latch_init() - runtime initializer for seqcount_latch_t
// @s: Pointer to the seqcount_latch_t instance
//

//
// raw_read_seqcount_latch() - pick even/odd latch data copy
// @s: Pointer to seqcount_latch_t
//
// See raw_write_seqcount_latch() for details and a full reader/writer
// usage example.
//
// Return: sequence counter raw value. Use the lowest bit as an index for
// picking which data copy to read. The full counter must then be checked
// with raw_read_seqcount_latch_retry().
//
// Pairs with the first smp_wmb() in raw_write_seqcount_latch().
// Due to the dependent load, a full smp_rmb() is not needed.
//
extern "C" {
    pub fn READ_ONCE(_arg: s->seqcount.sequence) -> return;
}
//
// read_seqcount_latch() - pick even/odd latch data copy
// @s: Pointer to seqcount_latch_t
//
// See write_seqcount_latch() for details and a full reader/writer usage
// example.
//
// Return: sequence counter raw value. Use the lowest bit as an index for
// picking which data copy to read. The full counter must then be checked
// with read_seqcount_latch_retry().
//
extern "C" {
    pub fn raw_read_seqcount_latch(_arg: s) -> return;
}
//
// raw_read_seqcount_latch_retry() - end a seqcount_latch_t read section
// @s:		Pointer to seqcount_latch_t
// @start:	count, from raw_read_seqcount_latch()
//
// Return: true if a read section retry is required, else false
//
extern "C" {
    pub fn unlikely(start: READ_ONCE(s->seqcount.sequence) !=) -> return;
}
//
// read_seqcount_latch_retry() - end a seqcount_latch_t read section
// @s:		Pointer to seqcount_latch_t
// @start:	count, from read_seqcount_latch()
//
// Return: true if a read section retry is required, else false
//
extern "C" {
    pub fn raw_read_seqcount_latch_retry(_arg: s, _arg: start) -> return;
}
//
// raw_write_seqcount_latch() - redirect latch readers to even/odd copy
// @s: Pointer to seqcount_latch_t
//
// write_seqcount_latch_begin() - redirect latch readers to odd copy
// @s: Pointer to seqcount_latch_t
//
// The latch technique is a multiversion concurrency control method that allows
// queries during non-atomic modifications. If you can guarantee queries never
// interrupt the modification -- e.g. the concurrency is strictly between CPUs
// -- you most likely do not need this.
//
// Where the traditional RCU/lockless data structures rely on atomic
// modifications to ensure queries observe either the old or the new state the
// latch allows the same for non-atomic updates. The trade-off is doubling the
// cost of storage; we have to maintain two copies of the entire data
// structure.
//
// Very simply put: we first modify one copy and then the other. This ensures
// there is always one copy in a stable state, ready to give us an answer.
//
// The basic form is a data structure like::
//
// struct latch_struct {
// seqcount_latch_t	seq;
// struct data_struct	data[2];
// };
//
// Where a modification, which is assumed to be externally serialized, does the
// following::
//
// void latch_modify(struct latch_struct *latch, ...)
// {
// write_seqcount_latch_begin(&latch->seq);
// modify(latch->data[0], ...);
// write_seqcount_latch(&latch->seq);
// modify(latch->data[1], ...);
// write_seqcount_latch_end(&latch->seq);
// }
//
// The query will have a form like::
//
// struct entry *latch_query(struct latch_struct *latch, ...)
// {
// struct entry *entry;
// unsigned seq, idx;
//
// do {
// seq = read_seqcount_latch(&latch->seq);
//
// idx = seq & 0x01;
// entry = data_query(latch->data[idx], ...);
//
// // This includes needed smp_rmb()
// } while (read_seqcount_latch_retry(&latch->seq, seq));
//
// return entry;
// }
//
// So during the modification, queries are first redirected to data[1]. Then we
// modify data[0]. When that is complete, we redirect queries back to data[0]
// and we can modify data[1].
//
// NOTE:
//
// The non-requirement for atomic modifications does _NOT_ include
// the publishing of new entries in the case where data is a dynamic
// data structure.
//
// An iteration might start in data[0] and get suspended long enough
// to miss an entire modification sequence, once it resumes it might
// observe the new entry.
//
// NOTE2:
//
// When data is a dynamic data structure; one should use regular RCU
// patterns to manage the lifetimes of the objects within.
//
// write_seqcount_latch() - redirect latch readers to even copy
// @s: Pointer to seqcount_latch_t
//
// write_seqcount_latch_end() - end a seqcount_latch_t write section
// @s:		Pointer to seqcount_latch_t
//
// Marks the end of a seqcount_latch_t writer section, after all copies of the
// latch-protected data have been updated.
//

//
// seqlock_init() - dynamic initializer for seqlock_t
// @sl: Pointer to the seqlock_t instance
//

//
// DEFINE_SEQLOCK(sl) - Define a statically allocated seqlock_t
// @sl: Name of the seqlock_t instance
//

//
// read_seqbegin() - start a seqlock_t read side critical section
// @sl: Pointer to seqlock_t
//
// Return: count, to be passed to read_seqretry()
//
extern "C" {
    pub fn read_seqcount_begin(_arg: &sl->seqcount) -> return;
}
//
// read_seqretry() - end a seqlock_t read side section
// @sl: Pointer to seqlock_t
// @start: count, from read_seqbegin()
//
// read_seqretry closes the read side critical section of given seqlock_t.
// If the critical section was invalid, it must be ignored (and typically
// retried).
//
// Return: true if a read section retry is required, else false
//
extern "C" {
    pub fn read_seqcount_retry(_arg: &sl->seqcount, _arg: start) -> return;
}
//
// For all seqlock_t write side functions, use the internal
// do_write_seqcount_begin() instead of generic write_seqcount_begin().
// This way, no redundant lockdep_assert_held() checks are added.
//
// write_seqlock() - start a seqlock_t write side critical section
// @sl: Pointer to seqlock_t
//
// write_seqlock opens a write side critical section for the given
// seqlock_t.  It also implicitly acquires the spinlock_t embedded inside
// that sequential lock. All seqlock_t write side sections are thus
// automatically serialized and non-preemptible.
//
// Context: if the seqlock_t read section, or other write side critical
// sections, can be invoked from hardirq or softirq contexts, use the
// _irqsave or _bh variants of this function instead.
//
// write_sequnlock() - end a seqlock_t write side critical section
// @sl: Pointer to seqlock_t
//
// write_sequnlock closes the (serialized and non-preemptible) write side
// critical section of given seqlock_t.
//
// write_seqlock_bh() - start a softirqs-disabled seqlock_t write section
// @sl: Pointer to seqlock_t
//
// _bh variant of write_seqlock(). Use only if the read side section, or
// other write side sections, can be invoked from softirq contexts.
//
// write_sequnlock_bh() - end a softirqs-disabled seqlock_t write section
// @sl: Pointer to seqlock_t
//
// write_sequnlock_bh closes the serialized, non-preemptible, and
// softirqs-disabled, seqlock_t write side critical section opened with
// write_seqlock_bh().
//
// write_seqlock_irq() - start a non-interruptible seqlock_t write section
// @sl: Pointer to seqlock_t
//
// _irq variant of write_seqlock(). Use only if the read side section, or
// other write sections, can be invoked from hardirq contexts.
//
// write_sequnlock_irq() - end a non-interruptible seqlock_t write section
// @sl: Pointer to seqlock_t
//
// write_sequnlock_irq closes the serialized and non-interruptible
// seqlock_t write side section opened with write_seqlock_irq().
//
// write_seqlock_irqsave() - start a non-interruptible seqlock_t write
// section
// @lock:  Pointer to seqlock_t
// @flags: Stack-allocated storage for saving caller's local interrupt
// state, to be passed to write_sequnlock_irqrestore().
//
// _irqsave variant of write_seqlock(). Use it only if the read side
// section, or other write sections, can be invoked from hardirq context.
//

//
// write_sequnlock_irqrestore() - end non-interruptible seqlock_t write
// section
// @sl:    Pointer to seqlock_t
// @flags: Caller's saved interrupt state, from write_seqlock_irqsave()
//
// write_sequnlock_irqrestore closes the serialized and non-interruptible
// seqlock_t write section previously opened with write_seqlock_irqsave().
//
// read_seqlock_excl() - begin a seqlock_t locking reader section
// @sl:	Pointer to seqlock_t
//
// read_seqlock_excl opens a seqlock_t locking reader critical section.  A
// locking reader exclusively locks out *both* other writers *and* other
// locking readers, but it does not update the embedded sequence number.
//
// Locking readers act like a normal spin_lock()/spin_unlock().
//
// Context: if the seqlock_t write section, *or other read sections*, can
// be invoked from hardirq or softirq contexts, use the _irqsave or _bh
// variant of this function instead.
//
// The opened read section must be closed with read_sequnlock_excl().
//
// read_sequnlock_excl() - end a seqlock_t locking reader critical section
// @sl: Pointer to seqlock_t
//
// read_seqlock_excl_bh() - start a seqlock_t locking reader section with
// softirqs disabled
// @sl: Pointer to seqlock_t
//
// _bh variant of read_seqlock_excl(). Use this variant only if the
// seqlock_t write side section, *or other read sections*, can be invoked
// from softirq contexts.
//
// read_sequnlock_excl_bh() - stop a seqlock_t softirq-disabled locking
// reader section
// @sl: Pointer to seqlock_t
//
// read_seqlock_excl_irq() - start a non-interruptible seqlock_t locking
// reader section
// @sl: Pointer to seqlock_t
//
// _irq variant of read_seqlock_excl(). Use this only if the seqlock_t
// write side section, *or other read sections*, can be invoked from a
// hardirq context.
//
// read_sequnlock_excl_irq() - end an interrupts-disabled seqlock_t
// locking reader section
// @sl: Pointer to seqlock_t
//
// read_seqlock_excl_irqsave() - start a non-interruptible seqlock_t
// locking reader section
// @lock:  Pointer to seqlock_t
// @flags: Stack-allocated storage for saving caller's local interrupt
// state, to be passed to read_sequnlock_excl_irqrestore().
//
// _irqsave variant of read_seqlock_excl(). Use this only if the seqlock_t
// write side section, *or other read sections*, can be invoked from a
// hardirq context.
//

//
// read_sequnlock_excl_irqrestore() - end non-interruptible seqlock_t
// locking reader section
// @sl:    Pointer to seqlock_t
// @flags: Caller saved interrupt state, from read_seqlock_excl_irqsave()
//
// read_seqbegin_or_lock() - begin a seqlock_t lockless or locking reader
// @lock: Pointer to seqlock_t
// @seq : Marker and return parameter. If the passed value is even, the
// reader will become a *lockless* seqlock_t reader as in read_seqbegin().
// If the passed value is odd, the reader will become a *locking* reader
// as in read_seqlock_excl().  In the first call to this function, the
// caller *must* initialize and pass an even value to @seq; this way, a
// lockless read can be optimistically tried first.
//
// read_seqbegin_or_lock is an API designed to optimistically try a normal
// lockless seqlock_t read section first.  If an odd counter is found, the
// lockless read trial has failed, and the next read iteration transforms
// itself into a full seqlock_t locking reader.
//
// This is typically used to avoid seqlock_t lockless readers starvation
// (too much retry loops) in the case of a sharp spike in write side
// activity.
//
// Context: if the seqlock_t write section, *or other read sections*, can
// be invoked from hardirq or softirq contexts, use the _irqsave or _bh
// variant of this function instead.
//
// Check Documentation/locking/seqlock.rst for template example code.
//
// Return: the encountered sequence counter value, through the @seq
// parameter, which is overloaded as a return parameter. This returned
// value must be checked with need_seqretry(). If the read section need to
// be retried, this returned value must also be passed as the @seq
// parameter of the next read_seqbegin_or_lock() iteration.
//
// seq = read_seqbegin(lock);
//
// need_seqretry() - validate seqlock_t "locking or lockless" read section
// @lock: Pointer to seqlock_t
// @seq: sequence count, from read_seqbegin_or_lock()
//
// Return: true if a read section retry is required, false otherwise
//
// done_seqretry() - end seqlock_t "locking or lockless" reader section
// @lock: Pointer to seqlock_t
// @seq: count, from read_seqbegin_or_lock()
//
// done_seqretry finishes the seqlock_t read side critical section started
// with read_seqbegin_or_lock() and validated by need_seqretry().
//
// read_seqbegin_or_lock_irqsave() - begin a seqlock_t lockless reader, or
// a non-interruptible locking reader
// @lock: Pointer to seqlock_t
// @seq:  Marker and return parameter. Check read_seqbegin_or_lock().
//
// This is the _irqsave variant of read_seqbegin_or_lock(). Use it only if
// the seqlock_t write section, *or other read sections*, can be invoked
// from hardirq context.
//
// Note: Interrupts will be disabled only for "locking reader" mode.
//
// Return:
//
// 1. The saved local interrupts state in case of a locking reader, to
// be passed to done_seqretry_irqrestore().
//
// 2. The encountered sequence counter value, returned through @seq
// overloaded as a return parameter. Check read_seqbegin_or_lock().
//
// seq = read_seqbegin(lock);
//
// done_seqretry_irqrestore() - end a seqlock_t lockless reader, or a
// non-interruptible locking reader section
// @lock:  Pointer to seqlock_t
// @seq:   Count, from read_seqbegin_or_lock_irqsave()
// @flags: Caller's saved local interrupt state in case of a locking
// reader, also from read_seqbegin_or_lock_irqsave()
//
// This is the _irqrestore variant of done_seqretry(). The read section
// must've been opened with read_seqbegin_or_lock_irqsave(), and validated
// by need_seqretry().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ss_state {
    ss_done = 0,
    ss_lock,
    ss_lock_irqsave,
    ss_lockless,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ss_tmp {
    pub state: ss_state,
    pub data: c_ulong,
    pub lock: *mut spinlock_t,
    pub lock_irqsave: *mut spinlock_t,
}

extern "C" {
    pub fn __scoped_seqlock_invalid_target();
}

//
// For some reason some GCC-8 architectures (nios2, alpha) have trouble
// determining that the ss_done state is impossible in __scoped_seqlock_next()
// below.
//
// Similarly KASAN and UBSAN_ALIGNMENT are known to confuse compilers enough
// to break this. But we don't care about code quality for such builds anyway.
//

//
// Canary for compiler optimization -- if the compiler doesn't realize this is
// an impossible state, it very likely generates sub-optimal code here.
//
extern "C" {
    pub fn __scoped_seqlock_bug();
}

//
// Context analysis no-op helper to release seqlock at the end of the for-scope;
// the alias analysis of the compiler will recognize that the pointer @s is an
// alias to @_seqlock passed to read_seqbegin(_seqlock) below.
//

// __UNIQUE_ID(ctx) __cleanup(__scoped_seqlock_cleanup_ctx) =\
//
// scoped_seqlock_read() - execute the read-side critical section
// without manual sequence counter handling
// or calls to other helpers
// @_seqlock: pointer to seqlock_t protecting the data
// @_target: an enum ss_state: one of {ss_lock, ss_lock_irqsave, ss_lockless}
// indicating the type of critical read section
//
// Example::
//
// scoped_seqlock_read (&lock, ss_lock) {
// // read-side critical section
// }
//
// Starts with a lockess pass first. If it fails, restarts the critical
// section with the lock held.
//

