//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/common.bpf.h
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2022 Tejun Heo <tj@kernel.org>
// Copyright (c) 2022 David Vernet <dvernet@meta.com>
//
// The generated kfunc prototypes in vmlinux.h are missing address space
// attributes which cause build failures. For now, suppress the generated
// prototypes. See https://github.com/sched-ext/scx/issues/1111.
//
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

// Macro flag: #define __bpf__

pub const PF_IDLE: c_uint = 0x00000002	/* I am an IDLE thread */;
pub const PF_IO_WORKER: c_uint = 0x00000010	/* Task is an IO worker */;
pub const PF_WQ_WORKER: c_uint = 0x00000020	/* I'm a workqueue worker */;
pub const PF_KCOMPACTD: c_uint = 0x00010000      /* I am kcompactd */;
pub const PF_KSWAPD: c_uint = 0x00020000      /* I am kswapd */;
pub const PF_KTHREAD: c_uint = 0x00200000	/* I am a kernel thread */;
pub const PF_EXITING: c_uint = 0x00000004;
pub const CLOCK_MONOTONIC: c_int = 1;

pub const NR_CPUS: c_int = 1024;

//
// Earlier versions of clang/pahole lost upper 32bits in 64bit enums which can
// lead to really confusing misbehaviors. Let's trigger a build failure.
//
// sub-scheduler cap control, scx_bpf_sub_caps() cgroup_id 0 == self
//
// Use the following as @it__iter when calling scx_bpf_dsq_move[_vtime]() from
// within bpf_for_each() loops.
//

//
// Helper macro for initializing the fmt and variadic argument inputs to both
// bstr exit kfuncs. Callers to this function should use ___fmt and ___param to
// refer to the initialized list of inputs to the bstr kfunc.
//

// \
// Note that __param[] must have at least one				\
// element to keep the verifier happy.					\
// \
//
// scx_bpf_exit() wraps the scx_bpf_exit_bstr() kfunc with variadic arguments
// instead of an array of u64. Using this macro will cause the scheduler to
// exit cleanly with the specified exit code being passed to user space.
//

//
// scx_bpf_sub_kill() wraps the scx_bpf_sub_kill_bstr() kfunc with variadic
// arguments instead of an array of u64. It kills the direct child sub-scheduler
// @cgid, passing the formatted reason to its user space, and evaluates to the
// kfunc's return value. On a kernel without sub-scheduler support the kfunc is
// absent and it returns -EOPNOTSUPP.
//

//
// scx_bpf_error() wraps the scx_bpf_error_bstr() kfunc with variadic arguments
// instead of an array of u64. Invoking this macro will cause the scheduler to
// exit in an erroneous state, with diagnostic information being passed to the
// user. It appends the file and line number to aid debugging.
//

//
// scx_bpf_dump() wraps the scx_bpf_dump_bstr() kfunc with variadic arguments
// instead of an array of u64. To be used from ops.dump() and friends.
//

//
// scx_bpf_dump_header() is a wrapper around scx_bpf_dump that adds a header
// of system information for debugging.
//

//
// RESIZABLE_ARRAY - Generates annotations for an array that may be resized
// @elfsec: the data section of the BPF program in which to place the array
// @arr: the name of the array
//
// libbpf has an API for setting map value sizes. Since data sections (i.e.
// bss, data, rodata) themselves are maps, a data section can be resized. If
// a data section has an array as its last element, the BTF info for that
// array will be adjusted so that length of the array is extended to meet the
// new length of the data section. This macro annotates an array to have an
// element count of one with the assumption that this array can be resized
// within the userspace program. It also annotates the section specifier so
// this array exists in a custom sub data section which can be resized
// independently.
//
// See RESIZE_ARRAY() for the userspace convenience macro for resizing an
// array declared with RESIZABLE_ARRAY().
//

//
// MEMBER_VPTR - Obtain the verified pointer to a struct or array member
// @base: struct or array to index
// @member: dereferenced member (e.g. .field, [idx0][idx1], .field[idx0] ...)
//
// The verifier often gets confused by the instruction sequence the compiler
// generates for indexing struct fields or arrays. This macro forces the
// compiler to generate a code sequence which first calculates the byte offset,
// checks it against the struct or array size and add that byte offset to
// generate the pointer to the member to help the verifier.
//
// Ideally, we want to abort if the calculated offset is out-of-bounds. However,
// BPF currently doesn't support abort, so evaluate to %NULL instead. The caller
// must check for %NULL and take appropriate action to appease the verifier. To
// avoid confusing the verifier, it's best to check for %NULL and dereference
// immediately.
//
// vptr = MEMBER_VPTR(my_array, [i][j]);
// if (!vptr)
// return error;
// *vptr = new_value;
//
// sizeof(@base) should encompass the memory area to be accessed and thus can't
// be a pointer to the area. Use `MEMBER_VPTR(*ptr, .member)` instead of
// `MEMBER_VPTR(ptr, ->member)`.
//

//
// ARRAY_ELEM_PTR - Obtain the verified pointer to an array element
// @arr: array to index into
// @i: array index
// @n: number of elements in array
//
// Similar to MEMBER_VPTR() but is intended for use with arrays where the
// element count needs to be explicit.
// It can be used in cases where a global array is defined with an initial
// size but is intended to be be resized before loading the BPF program.
// Without this version of the macro, MEMBER_VPTR() will use the compile time
// size of the array to compute the max, which will result in rejection by
// the verifier.
//

//
// __sink - Hide @expr's value from the compiler and BPF verifier
// @expr: The expression whose value should be opacified
//
// No-op at runtime. The empty inline assembly with a read-write constraint
// ("+g") has two effects at compile/verify time:
//
// 1. Compiler: treats @expr as both read and written, preventing dead-code
// elimination and keeping @expr (and any side effects that produced it)
// alive.
//
// 2. BPF verifier: forgets the precise value/range of @expr ("makes it
// imprecise"). The verifier normally tracks exact ranges for every register
// and stack slot. While useful, precision means each distinct value creates a
// separate verifier state. Inside loops this leads to state explosion - each
// iteration carries different precise values so states never merge and the
// verifier explores every iteration individually.
//
// Example - preventing loop state explosion::
//
// u32 nr_intersects = 0, nr_covered = 0;
// __sink(nr_intersects);
// __sink(nr_covered);
// bpf_for(i, 0, nr_nodes) {
// if (intersects(cpumask, node_mask[i]))
// nr_intersects++;
// if (covers(cpumask, node_mask[i]))
// nr_covered++;
// }
//
// Without __sink(), the verifier tracks every possible (nr_intersects,
// nr_covered) pair across iterations, causing "BPF program is too large". With
// __sink(), the values become unknown scalars so all iterations collapse into
// one reusable state.
//
// Example - keeping a reference alive::
//
// struct task_struct *t = bpf_task_acquire(task);
// __sink(t);
//
// Follows the convention from BPF selftests (bpf_misc.h).
//

//
// BPF declarations and helpers
//
// list and rbtree

// task
// cgroup
// css iteration
// cpumask

// Provides iterator for possible and online cpus.
//
// # Example
//
// ```
// static inline void example_use() {
// int *cpu;
//
// for_each_possible_cpu(cpu){
// bpf_printk("CPU %d is possible", *cpu);
// }
//
// for_each_online_cpu(cpu){
// bpf_printk("CPU %d is online", *cpu);
// }
// ```

//
// Access a cpumask in read-only mode (typically to check bits).
//
// True if the non-sleepable BPF trampoline prolog (__bpf_prog_enter) calls
// migrate_disable() for the current task. Recorded once by
// scx_lib_init_probe, an fentry program on bpf_scx_reg() that fires during
// the natural scheduler-attach call chain (auto-attached by scx_ops_attach!).
//
// Defaults to true (conservative). Over-reporting in is_migration_disabled()
// causes local-only dispatch, which is safe. Under-reporting can crash the
// scheduler, so we err high if the probe somehow fails to run.
//
// scx_lib_init_probe - non-sleepable prolog probe.
//
// Attached to bpf_scx_reg(), the .reg callback in bpf_sched_ext_ops
// (kernel/sched/ext.c). The kernel's struct_ops machinery invokes
// bpf_scx_reg when userspace creates the scheduler link, before
// ops.init() fires. Its address is taken in the vtable, so the symbol
// is non-inlinable and has been stable since introduction.
//
// Entering via fentry runs us through __bpf_prog_enter -- the
// non-sleepable prolog that consumers of is_migration_disabled() live
// under.
//
// Loud warning: the prolog adds at most 1 to migration_disabled.
// Reading > 1 means something upstream in the
// bpf_struct_ops_link_create -> bpf_scx_reg path disabled migration
// before the prolog ran, invalidating the probe; audit and adjust.
//
// Return true if task @p cannot migrate to a different CPU, false
// otherwise.
//
// IMPORTANT: designed for NON-SLEEPABLE BPF contexts only. Sleepable
// contexts (BPF_STRUCT_OPS_SLEEPABLE, SEC("syscall"),
// SEC("fentry.s/...")) enter via __bpf_prog_enter_sleepable() or
// __bpf_prog_enter_sleepable_recur(), both of which unconditionally
// call migrate_disable(); this helper can yield a false negative for
// p == current there, which can crash the scheduler.
//
// Testing p->migration_disabled in BPF is tricky because the BPF prolog
// (__bpf_prog_enter) may call migrate_disable() for the current task,
// making migration_disabled == 1 even for tasks that are not truly
// migration-disabled.
//
// Since commit 8e4f0b1ebcf2 ("bpf: use rcu_read_lock_dont_migrate() for
// trampoline.c"), the BPF prolog calls migrate_disable() only when
// CONFIG_PREEMPT_RCU is enabled. Two fast paths cover the common cases:
//
// 1) CONFIG_PREEMPT_RCU: prolog always calls migrate_disable(), so
// migration_disabled == 1 for the current task is ambiguous.
// Disambiguate by checking p == current.
//
// 2) v6.18+ without CONFIG_PREEMPT_RCU: prolog never calls
// migrate_disable(), so migration_disabled == 1 is unambiguously
// a real migrate_disable() call.
//
// A slow path handles pre-v6.18 kernels without CONFIG_PREEMPT_RCU,
// where the prolog historically called migrate_disable() unconditionally
// but a cherry-picked downstream kernel may not. The runtime-probed flag
// __scx_prolog_disables_migration (set by scx_lib_init_probe) distinguishes
// the two cases without relying on the kernel version alone.
//
// Fast path: prolog always disables migration
// Fast path: prolog never disables migration
// Slow path: pre-v6.18, !PREEMPT_RCU - use runtime flag
// rcu
// resilient qspinlock
//
// Time helpers, most of which are from jiffies.h.
//
// time_delta - Calculate the delta between new and old time stamp
// @after: first comparable as u64
// @before: second comparable as u64
//
// Return: the time difference, which is >= 0
//
// time_after - returns true if the time a is after time b.
// @a: first comparable as u64
// @b: second comparable as u64
//
// Do this with "<0" and ">=0" to only test the sign of the result. A
// good compiler would generate better code (and a really good compiler
// wouldn't care). Gcc is currently neither.
//
// Return: %true is time a is after time b, otherwise %false.
//
// time_before - returns true if the time a is before time b.
// @a: first comparable as u64
// @b: second comparable as u64
//
// Return: %true is time a is before time b, otherwise %false.
//
extern "C" {
    pub fn time_after(_arg: b, _arg: a) -> return;
}
//
// time_after_eq - returns true if the time a is after or the same as time b.
// @a: first comparable as u64
// @b: second comparable as u64
//
// Return: %true is time a is after or the same as time b, otherwise %false.
//
// time_before_eq - returns true if the time a is before or the same as time b.
// @a: first comparable as u64
// @b: second comparable as u64
//
// Return: %true is time a is before or the same as time b, otherwise %false.
//
extern "C" {
    pub fn time_after_eq(_arg: b, _arg: a) -> return;
}
//
// time_in_range - Calculate whether a is in the range of [b, c].
// @a: time to test
// @b: beginning of the range
// @c: end of the range
//
// Return: %true is time a is in the range [b, c], otherwise %false.
//
extern "C" {
    pub fn time_after_eq(_arg: a, time_before_eq(a: b) &&, _arg: c) -> return;
}
//
// time_in_range_open - Calculate whether a is in the range of [b, c).
// @a: time to test
// @b: beginning of the range
// @c: end of the range
//
// Return: %true is time a is in the range [b, c), otherwise %false.
//
extern "C" {
    pub fn time_after_eq(_arg: a, time_before(a: b) &&, _arg: c) -> return;
}
//
// Other helpers
//
// useful compiler attributes

//
// READ/WRITE_ONCE() are from kernel (include/asm-generic/rwonce.h). They
// prevent compiler from caching, redoing or reordering reads or writes.
//
// __unqual_typeof(x) - Declare an unqualified scalar type, leaving
// non-scalar types unchanged,
//
// Prefer C11 _Generic for better compile-times and simpler code. Note: 'char'
// is not type-compatible with 'signed char', and we define a separate case.
//
// This is copied verbatim from kernel's include/linux/compiler_types.h, but
// with default expression (for pointers) changed from (x) to (typeof(x)0).
//
// This is because LLVM has a bug where for lvalue (x), it does not get rid of
// an extra address_space qualifier, but does in case of rvalue (typeof(x)0).
// Hence, for pointers, we need to create an rvalue expression to get the
// desired type. See https://github.com/llvm/llvm-project/issues/53400.
//

//
// __calc_avg - Calculate exponential weighted moving average (EWMA) with
// @old and @new values. @decay represents how large the @old value remains.
// With a larger @decay value, the moving average changes slowly, exhibiting
// fewer fluctuations.
//

//
// log2_u32 - Compute the base 2 logarithm of a 32-bit exponential value.
// @v: The value for which we're computing the base 2 logarithm.
//
// log2_u64 - Compute the base 2 logarithm of a 64-bit exponential value.
// @v: The value for which we're computing the base 2 logarithm.
//
// sqrt_u64 - Calculate the square root of value @x using Newton's method.
//
// ctzll -- Counts trailing zeros in an unsigned long long. If the input value
// is zero, the return value is undefined.
//

//
// Use the ctz builtin when: (1) building for native x86, or
// (2) building for BPF with clang >= 19 (BPF backend supports
// the intrinsic from clang 19 onward; earlier versions hit
// "unimplemented opcode" in the backend).
//
extern "C" {
    pub fn __builtin_ctzll(_arg: v) -> return;
}

//
// If neither the target architecture nor the toolchains support ctzll,
// use software-based emulation. Let's use the De Bruijn sequence-based
// approach to find LSB fastly. See the details of De Bruijn sequence:
//
// https://en.wikipedia.org/wiki/De_Bruijn_sequence
// https://www.chessprogramming.org/BitScan#De_Bruijn_Multiplication
//
// Isolate the least significant bit (LSB).
// For example, if v = 0b...10100, then v & -v = 0b...00100
//
// Each isolated bit produces a unique 6-bit value, guaranteed by the
// De Bruijn property. Calculate a unique index into the lookup table
// using the magic constant and a right shift.
//
// Multiplying by the 64-bit constant "spreads out" that 1-bit into a
// unique pattern in the top 6 bits. This uniqueness property is
// exactly what a De Bruijn sequence guarantees: Every possible 6-bit
// pattern (in top bits) occurs exactly once for each LSB position. So,
// the constant 0x03f79d71b4cb0a89ULL is carefully chosen to be a
// De Bruijn sequence, ensuring no collisions in the table index.
//
// Lookup in a precomputed table. No collision is guaranteed by the
// De Bruijn property.
//

//
// Return a value proportionally scaled to the task's weight.
//
// Return a value inversely proportional to the task's weight.
//
// Get a random u64 from the kernel's pseudo-random generator.
//
// Define the shadow structure to avoid a compilation error when
// vmlinux.h does not enable necessary kernel configs. The ___local
// suffix is a CO-RE convention that tells the loader to match this
// against the base struct rq in the kernel. The attribute
// preserve_access_index tells the compiler to generate a CO-RE
// relocation for these fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq___local {
//
// A monotonically increasing clock per CPU. It is rq->clock minus
// cumulative IRQ time and hypervisor steal time. Unlike rq->clock,
// it does not advance during IRQ processing or hypervisor preemption.
// It does advance during idle (the idle task counts as a running task
// for this purpose).
//
    pub clock_task: u64,
//
// Invariant version of clock_task scaled by CPU capacity and
// frequency. For example, clock_pelt advances 2x slower on a CPU
// with half the capacity.
//
// At idle exit, rq->clock_pelt jumps forward to resync with
// clock_task. The kernel's rq_clock_pelt() corrects for this jump
// by subtracting lost_idle_time, yielding a clock that appears
// continuous across idle transitions. scx_clock_pelt() mirrors
// rq_clock_pelt() by performing the same subtraction.
//
    pub clock_pelt: u64,
//
// Accumulates the magnitude of each clock_pelt jump at idle exit.
// Subtracting this from clock_pelt gives rq_clock_pelt(): a
// continuous, capacity-invariant clock suitable for both task
// execution time stamping and cross-idle measurements.
//
    pub lost_idle_time: c_ulong,
//
// Shadow of paravirt_steal_clock() (the hypervisor's cumulative
// stolen time counter). Stays frozen while the hypervisor preempts
// the vCPU; catches up the next time update_rq_clock_task() is
// called. The delta is the stolen time not yet subtracted from
// clock_task.
//
// Unlike irqtime->total (a plain kernel-side field), the live stolen
// time counter lives in hypervisor-specific shared memory and has no
// kernel-side equivalent readable from BPF in a hypervisor-agnostic
// way. This field is therefore the only portable BPF-accessible
// approximation of cumulative steal time.
//
// Available only when CONFIG_PARAVIRT_TIME_ACCOUNTING is on.
//
    pub prev_steal_time_rq: u64,
    pub __attribute__((preserve_access_index)): },
    pub __ksym: extern struct rq runqueues,
//
// Define the shadow structure to avoid a compilation error when
// vmlinux.h does not enable necessary kernel configs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irqtime___local {
//
// Cumulative IRQ time counter for this CPU, in nanoseconds. Advances
// immediately at the exit of every hardirq and non-ksoftirqd softirq
// via irqtime_account_irq(). ksoftirqd time is counted as normal
// task time and is NOT included. NMI time is also NOT included.
//
// The companion field irqtime->sync (struct u64_stats_sync) protects
// against 64-bit tearing on 32-bit architectures. On 64-bit kernels,
// u64_stats_sync is an empty struct and all seqcount operations are
// no-ops, so a plain BPF_CORE_READ of this field is safe.
//
// Available only when CONFIG_IRQ_TIME_ACCOUNTING is on.
//
    pub total: u64,
    pub __attribute__((preserve_access_index)): },
//
// cpu_irqtime is a per-CPU variable defined only when
// CONFIG_IRQ_TIME_ACCOUNTING is on. Declare it as __weak so the BPF
// loader sets its address to 0 (rather than failing) when the symbol
// is absent from the running kernel.
//
    pub __weak: extern struct irqtime___local cpu_irqtime __ksym,
//
// This is a workaround to get an rq pointer now that
// scx_bpf_cpu_rq() has been removed.
//
// WARNING: The caller must hold the rq lock for @cpu. This is
// guaranteed when called from scheduling callbacks (ops.running,
// ops.stopping, ops.enqueue, ops.dequeue, ops.dispatch, etc.).
// There is no runtime check available in BPF for kernel spinlock
// state — correctness is enforced by calling context only.
//
    pub cpu): *mut *mut return (void )bpf_per_cpu_ptr(&runqueues,,
    pub get_current_rq(cpu): *mut *mut rq___local rq =,
//
// Equivalent to the kernel's rq_clock_task(): wall-clock time minus
// cumulative IRQ time (CONFIG_IRQ_TIME_ACCOUNTING) and hypervisor
// steal time (CONFIG_PARAVIRT_TIME_ACCOUNTING). Without those configs,
// it equals rq->clock.
//
// Conceptually this clock advances during idle (the idle task counts
// as a running task), but rq->clock_task is only updated on scheduling
// events. With NO_HZ_IDLE (the default), the periodic tick is stopped
// on idle CPUs, so rq->clock_task is not refreshed while a CPU is
// idle. Reading this clock for a remote idle CPU from a BPF timer
// callback returns the value from when the CPU last went idle, making
// the delta over an idle interval effectively zero.
//
    pub 0: return rq ? rq->clock_task :,
    pub get_current_rq(cpu): *mut *mut rq___local rq =,
//
// Equivalent to the kernel's rq_clock_pelt(): subtracts
// lost_idle_time from clock_pelt to absorb the jump that occurs
// when clock_pelt resyncs with clock_task at idle exit. The intent
// is a continuous, capacity- and frequency-invariant clock that is
// frozen during idle, IRQ, and hypervisor steal.
//
// However, like scx_clock_task(), this clock has a stale-read issue
// for remote idle CPUs with NO_HZ_IDLE (the default). clock_pelt
// itself advances at wall-clock rate (hardware-clock based), but
// lost_idle_time is only updated via update_rq_clock_pelt(), which
// requires update_rq_clock() to be called. With NO_HZ_IDLE, the
// periodic tick is stopped on idle CPUs, so lost_idle_time is not
// refreshed during idle. Reading this clock for a remote idle CPU
// from a BPF timer callback therefore returns a value that drifts
// at wall-clock rate -- the same stale behaviour as scx_clock_task().
//
// Without NO_HZ_IDLE, periodic ticks keep lost_idle_time nearly in
// sync (stale by at most one tick period, ~1 ms), so the result is
// accurate.
//
    pub 0: return rq ? (rq->clock_pelt - rq->lost_idle_time) :,
    pub rq: *mut rq___local,
//
// Check field existence before calling get_current_rq() so we avoid
// the per_cpu lookup entirely on kernels built without
// CONFIG_PARAVIRT_TIME_ACCOUNTING.
//
    pub 0: return,
// Lagging shadow of the kernel's paravirt_steal_clock().
    pub get_current_rq(cpu): rq =,
    pub 0: return rq ? BPF_CORE_READ(rq, prev_steal_time_rq) :,
    pub irqt: *mut irqtime___local,
//
// bpf_core_type_exists() resolves at load time: if struct irqtime is
// absent from kernel BTF (CONFIG_IRQ_TIME_ACCOUNTING off), the loader
// patches this into an unconditional return 0, making the
// bpf_per_cpu_ptr() call below dead code that the verifier never sees.
//
    pub 0: return,
// Equivalent to the kernel's irq_time_read().
    pub cpu): irqt = bpf_per_cpu_ptr(&cpu_irqtime,,
    pub 0: return irqt ? BPF_CORE_READ(irqt, total) :,
// Abbreviated forms of <linux/overflow.h>'s struct_size() family.

