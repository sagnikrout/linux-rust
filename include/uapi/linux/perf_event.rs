//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/perf_event.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Performance events:
//
// Copyright (C) 2008-2009, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright (C) 2008-2011, Red Hat, Inc., Ingo Molnar
// Copyright (C) 2008-2011, Red Hat, Inc., Peter Zijlstra
//
// Data type definitions, declarations, prototypes.
//
// Started by: Thomas Gleixner and Ingo Molnar
//
// For licencing details see kernel-base/COPYING
//

//
// User-space ABI bits:
//
// attr.type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_type_id {
    PERF_TYPE_HARDWARE			= 0,
    PERF_TYPE_SOFTWARE			= 1,
    PERF_TYPE_TRACEPOINT			= 2,
    PERF_TYPE_HW_CACHE			= 3,
    PERF_TYPE_RAW				= 4,
    PERF_TYPE_BREAKPOINT			= 5,

    PERF_TYPE_MAX,				/* non-ABI */
}

//
// attr.config layout for type PERF_TYPE_HARDWARE and PERF_TYPE_HW_CACHE
//
// PERF_TYPE_HARDWARE:			0xEEEEEEEE000000AA
// AA: hardware event ID
// EEEEEEEE: PMU type ID
//
// PERF_TYPE_HW_CACHE:			0xEEEEEEEE00DDCCBB
// BB: hardware cache ID
// CC: hardware cache op ID
// DD: hardware cache op result ID
// EEEEEEEE: PMU type ID
//
// If the PMU type ID is 0, PERF_TYPE_RAW will be applied.
//
pub const PERF_PMU_TYPE_SHIFT: c_int = 32;
pub const PERF_HW_EVENT_MASK: c_uint = 0xffffffff;
//
// Generalized performance event event_id types, used by the
// attr.event_id parameter of the sys_perf_event_open()
// syscall:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_hw_id {
//
// Common hardware events, generalized by the kernel:
//
    PERF_COUNT_HW_CPU_CYCLES		= 0,
    PERF_COUNT_HW_INSTRUCTIONS		= 1,
    PERF_COUNT_HW_CACHE_REFERENCES		= 2,
    PERF_COUNT_HW_CACHE_MISSES		= 3,
    PERF_COUNT_HW_BRANCH_INSTRUCTIONS	= 4,
    PERF_COUNT_HW_BRANCH_MISSES		= 5,
    PERF_COUNT_HW_BUS_CYCLES		= 6,
    PERF_COUNT_HW_STALLED_CYCLES_FRONTEND	= 7,
    PERF_COUNT_HW_STALLED_CYCLES_BACKEND	= 8,
    PERF_COUNT_HW_REF_CPU_CYCLES		= 9,

    PERF_COUNT_HW_MAX,			/* non-ABI */
}

//
// Generalized hardware cache events:
//
// { L1-D, L1-I, LLC, ITLB, DTLB, BPU, NODE } x
// { read, write, prefetch } x
// { accesses, misses }
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_hw_cache_id {
    PERF_COUNT_HW_CACHE_L1D			= 0,
    PERF_COUNT_HW_CACHE_L1I			= 1,
    PERF_COUNT_HW_CACHE_LL			= 2,
    PERF_COUNT_HW_CACHE_DTLB		= 3,
    PERF_COUNT_HW_CACHE_ITLB		= 4,
    PERF_COUNT_HW_CACHE_BPU			= 5,
    PERF_COUNT_HW_CACHE_NODE		= 6,

    PERF_COUNT_HW_CACHE_MAX,		/* non-ABI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_hw_cache_op_id {
    PERF_COUNT_HW_CACHE_OP_READ		= 0,
    PERF_COUNT_HW_CACHE_OP_WRITE		= 1,
    PERF_COUNT_HW_CACHE_OP_PREFETCH		= 2,

    PERF_COUNT_HW_CACHE_OP_MAX,		/* non-ABI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_hw_cache_op_result_id {
    PERF_COUNT_HW_CACHE_RESULT_ACCESS	= 0,
    PERF_COUNT_HW_CACHE_RESULT_MISS		= 1,

    PERF_COUNT_HW_CACHE_RESULT_MAX,		/* non-ABI */
}

//
// Special "software" events provided by the kernel, even if the hardware
// does not support performance events. These events measure various
// physical and SW events of the kernel (and allow the profiling of them as
// well):
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_sw_ids {
    PERF_COUNT_SW_CPU_CLOCK			= 0,
    PERF_COUNT_SW_TASK_CLOCK		= 1,
    PERF_COUNT_SW_PAGE_FAULTS		= 2,
    PERF_COUNT_SW_CONTEXT_SWITCHES		= 3,
    PERF_COUNT_SW_CPU_MIGRATIONS		= 4,
    PERF_COUNT_SW_PAGE_FAULTS_MIN		= 5,
    PERF_COUNT_SW_PAGE_FAULTS_MAJ		= 6,
    PERF_COUNT_SW_ALIGNMENT_FAULTS		= 7,
    PERF_COUNT_SW_EMULATION_FAULTS		= 8,
    PERF_COUNT_SW_DUMMY			= 9,
    PERF_COUNT_SW_BPF_OUTPUT		= 10,
    PERF_COUNT_SW_CGROUP_SWITCHES		= 11,

    PERF_COUNT_SW_MAX,			/* non-ABI */
}

//
// Bits that can be set in attr.sample_type to request information
// in the overflow packets.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_sample_format {
    PERF_SAMPLE_IP				= 1U << 0,
    PERF_SAMPLE_TID				= 1U << 1,
    PERF_SAMPLE_TIME			= 1U << 2,
    PERF_SAMPLE_ADDR			= 1U << 3,
    PERF_SAMPLE_READ			= 1U << 4,
    PERF_SAMPLE_CALLCHAIN			= 1U << 5,
    PERF_SAMPLE_ID				= 1U << 6,
    PERF_SAMPLE_CPU				= 1U << 7,
    PERF_SAMPLE_PERIOD			= 1U << 8,
    PERF_SAMPLE_STREAM_ID			= 1U << 9,
    PERF_SAMPLE_RAW				= 1U << 10,
    PERF_SAMPLE_BRANCH_STACK		= 1U << 11,
    PERF_SAMPLE_REGS_USER			= 1U << 12,
    PERF_SAMPLE_STACK_USER			= 1U << 13,
    PERF_SAMPLE_WEIGHT			= 1U << 14,
    PERF_SAMPLE_DATA_SRC			= 1U << 15,
    PERF_SAMPLE_IDENTIFIER			= 1U << 16,
    PERF_SAMPLE_TRANSACTION			= 1U << 17,
    PERF_SAMPLE_REGS_INTR			= 1U << 18,
    PERF_SAMPLE_PHYS_ADDR			= 1U << 19,
    PERF_SAMPLE_AUX				= 1U << 20,
    PERF_SAMPLE_CGROUP			= 1U << 21,
    PERF_SAMPLE_DATA_PAGE_SIZE		= 1U << 22,
    PERF_SAMPLE_CODE_PAGE_SIZE		= 1U << 23,
    PERF_SAMPLE_WEIGHT_STRUCT		= 1U << 24,

    PERF_SAMPLE_MAX = 1U << 25,		/* non-ABI */
}

//
// Values to program into branch_sample_type when PERF_SAMPLE_BRANCH is set.
//
// If the user does not pass priv level information via branch_sample_type,
// the kernel uses the event's priv level. Branch and event priv levels do
// not have to match. Branch priv level is checked for permissions.
//
// The branch types can be combined, however BRANCH_ANY covers all types
// of branches and therefore it supersedes all the other types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_branch_sample_type_shift {
    PERF_SAMPLE_BRANCH_USER_SHIFT		=  0, /* user branches */
    PERF_SAMPLE_BRANCH_KERNEL_SHIFT		=  1, /* kernel branches */
    PERF_SAMPLE_BRANCH_HV_SHIFT		=  2, /* hypervisor branches */

    PERF_SAMPLE_BRANCH_ANY_SHIFT		=  3, /* any branch types */
    PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT	=  4, /* any call branch */
    PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT	=  5, /* any return branch */
    PERF_SAMPLE_BRANCH_IND_CALL_SHIFT	=  6, /* indirect calls */
    PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT	=  7, /* transaction aborts */
    PERF_SAMPLE_BRANCH_IN_TX_SHIFT		=  8, /* in transaction */
    PERF_SAMPLE_BRANCH_NO_TX_SHIFT		=  9, /* not in transaction */
    PERF_SAMPLE_BRANCH_COND_SHIFT		= 10, /* conditional branches */

    PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT	= 11, /* CALL/RET stack */
    PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT	= 12, /* indirect jumps */
    PERF_SAMPLE_BRANCH_CALL_SHIFT		= 13, /* direct call */

    PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT	= 14, /* no flags */
    PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT	= 15, /* no cycles */

    PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT	= 16, /* save branch type */

    PERF_SAMPLE_BRANCH_HW_INDEX_SHIFT	= 17, /* save low level index of raw branch records */

    PERF_SAMPLE_BRANCH_PRIV_SAVE_SHIFT	= 18, /* save privilege mode */

    PERF_SAMPLE_BRANCH_COUNTERS_SHIFT	= 19, /* save occurrences of events on a branch */

    PERF_SAMPLE_BRANCH_MAX_SHIFT		/* non-ABI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_branch_sample_type {
    PERF_SAMPLE_BRANCH_USER			= 1U << PERF_SAMPLE_BRANCH_USER_SHIFT,
    PERF_SAMPLE_BRANCH_KERNEL		= 1U << PERF_SAMPLE_BRANCH_KERNEL_SHIFT,
    PERF_SAMPLE_BRANCH_HV			= 1U << PERF_SAMPLE_BRANCH_HV_SHIFT,

    PERF_SAMPLE_BRANCH_ANY			= 1U << PERF_SAMPLE_BRANCH_ANY_SHIFT,
    PERF_SAMPLE_BRANCH_ANY_CALL		= 1U << PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT,
    PERF_SAMPLE_BRANCH_ANY_RETURN		= 1U << PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT,
    PERF_SAMPLE_BRANCH_IND_CALL		= 1U << PERF_SAMPLE_BRANCH_IND_CALL_SHIFT,
    PERF_SAMPLE_BRANCH_ABORT_TX		= 1U << PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT,
    PERF_SAMPLE_BRANCH_IN_TX		= 1U << PERF_SAMPLE_BRANCH_IN_TX_SHIFT,
    PERF_SAMPLE_BRANCH_NO_TX		= 1U << PERF_SAMPLE_BRANCH_NO_TX_SHIFT,
    PERF_SAMPLE_BRANCH_COND			= 1U << PERF_SAMPLE_BRANCH_COND_SHIFT,

    PERF_SAMPLE_BRANCH_CALL_STACK		= 1U << PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT,
    PERF_SAMPLE_BRANCH_IND_JUMP		= 1U << PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT,
    PERF_SAMPLE_BRANCH_CALL			= 1U << PERF_SAMPLE_BRANCH_CALL_SHIFT,

    PERF_SAMPLE_BRANCH_NO_FLAGS		= 1U << PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT,
    PERF_SAMPLE_BRANCH_NO_CYCLES		= 1U << PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT,

    PERF_SAMPLE_BRANCH_TYPE_SAVE		= 1U << PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT,

    PERF_SAMPLE_BRANCH_HW_INDEX		= 1U << PERF_SAMPLE_BRANCH_HW_INDEX_SHIFT,

    PERF_SAMPLE_BRANCH_PRIV_SAVE		= 1U << PERF_SAMPLE_BRANCH_PRIV_SAVE_SHIFT,

    PERF_SAMPLE_BRANCH_COUNTERS		= 1U << PERF_SAMPLE_BRANCH_COUNTERS_SHIFT,

    PERF_SAMPLE_BRANCH_MAX			= 1U << PERF_SAMPLE_BRANCH_MAX_SHIFT,
}

//
// Common control flow change classifications:
//
// Common branch speculation outcome classifications:
//

//
// Values to determine ABI of the registers dump.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_sample_regs_abi {
    PERF_SAMPLE_REGS_ABI_NONE		= 0,
    PERF_SAMPLE_REGS_ABI_32			= 1,
    PERF_SAMPLE_REGS_ABI_64			= 2,
}

//
// Values for the memory transaction event qualifier, mostly for
// abort events. Multiple bits can be set.
//
// Bits 32..63 are reserved for the abort code
//
// The format of the data returned by read() on a perf event fd,
// as specified by attr.read_format:
//
// struct read_format {
// { u64		value;
// { u64		time_enabled; } && PERF_FORMAT_TOTAL_TIME_ENABLED
// { u64		time_running; } && PERF_FORMAT_TOTAL_TIME_RUNNING
// { u64		id;           } && PERF_FORMAT_ID
// { u64		lost;         } && PERF_FORMAT_LOST
// } && !PERF_FORMAT_GROUP
//
// { u64		nr;
// { u64		time_enabled; } && PERF_FORMAT_TOTAL_TIME_ENABLED
// { u64		time_running; } && PERF_FORMAT_TOTAL_TIME_RUNNING
// { u64		value;
// { u64	id;           } && PERF_FORMAT_ID
// { u64	lost;         } && PERF_FORMAT_LOST
// }		cntr[nr];
// } && PERF_FORMAT_GROUP
// };
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_read_format {
    PERF_FORMAT_TOTAL_TIME_ENABLED		= 1U << 0,
    PERF_FORMAT_TOTAL_TIME_RUNNING		= 1U << 1,
    PERF_FORMAT_ID				= 1U << 2,
    PERF_FORMAT_GROUP			= 1U << 3,
    PERF_FORMAT_LOST			= 1U << 4,

    PERF_FORMAT_MAX = 1U << 5,		/* non-ABI */
}

// Add: sample_stack_user

//
// 'struct perf_event_attr' contains various attributes that define
// a performance event - most of them hardware related configuration
// details, but also a lot of behavioral switches and values implemented
// by the kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
//
// Major type: hardware/software/tracepoint/etc.
//
    pub type: __u32,
//
// Size of the attr structure, for forward/backwards compatibility.
//
    pub size: __u32,
//
// Type specific configuration information.
//
    pub config: __u64,
    pub sample_period: __u64,
    pub sample_freq: __u64,
}

//
// precise_ip:
//
// 0 - SAMPLE_IP can have arbitrary skid
// 1 - SAMPLE_IP must have constant skid
// 2 - SAMPLE_IP requested to have 0 skid
// 3 - SAMPLE_IP must have 0 skid
//
// See also PERF_RECORD_MISC_EXACT_IP
//
// Defines set of user regs to dump on samples.
// See asm/perf_regs.h for details.
//
// Defines size of the user stack to dump on samples.
//
// Defines set of regs to dump for each sample
// state captured on:
// - precise = 0: PMU interrupt
// - precise > 0: sampled instruction
//
// See asm/perf_regs.h for details.
//
// Wakeup watermark for AUX area
//
// Max number of frame pointers in a callchain, should be
// lower than /proc/sys/kernel/perf_event_max_stack.
//
// Max number of entries of branch stack should be lower
// than the hardware limit.
//
// User provided data if sigtrap=1, passed back to user via
// siginfo_t::si_perf_data, e.g. to permit user to identify the event.
// Note, siginfo_t::si_perf_data is long-sized, and sig_data will be
// truncated accordingly on 32 bit architectures.
//
// Structure used by below PERF_EVENT_IOC_QUERY_BPF command
// to query BPF programs attached to the same perf tracepoint
// as the given perf event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_query_bpf {
//
// The below ids array length
//
    pub ids_len: __u32,
//
// Set by the kernel to indicate the number of
// available programs
//
    pub prog_cnt: __u32,
//
// User provided buffer to store program ids
//
    pub ids: [__u32; ],
}

//
// Ioctls that can be done on a perf event fd:
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_ioc_flags {
    PERF_IOC_FLAG_GROUP			= 1U << 0,
}

//
// Structure of the page that can be mapped via mmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_mmap_page {
    pub /: *mut *mut __u32 version; / version number of this structure,
    pub /: *mut *mut __u32 compat_version; / lowest version this is compat with,
//
// Bits needed to read the HW events in user-space.
//
// u32 seq, time_mult, time_shift, index, width;
// u64 count, enabled, running;
// u64 cyc, time_offset;
// s64 pmc = 0;
//
// do {
// seq = pc->lock;
// barrier()
//
// enabled = pc->time_enabled;
// running = pc->time_running;
//
// if (pc->cap_usr_time && enabled != running) {
// cyc = rdtsc();
// time_offset = pc->time_offset;
// time_mult   = pc->time_mult;
// time_shift  = pc->time_shift;
// }
//
// index = pc->index;
// count = pc->offset;
// if (pc->cap_user_rdpmc && index) {
// width = pc->pmc_width;
// pmc = rdpmc(index - 1);
// }
//
// barrier();
// } while (pc->lock != seq);
//
// NOTE: for obvious reason this only works on self-monitoring
// processes.
//
    pub /: *mut *mut __u32 lock; / seqlock for synchronization,
    pub /: *mut *mut __u32 index; / hardware event identifier,
    pub /: *mut *mut __s64 offset; / add to hardware event value,
    pub /: *mut *mut __u64 time_enabled; / time event active,
    pub /: *mut *mut __u64 time_running; / time event on CPU,
    pub capabilities: __u64,
    pub 58: cap_____res :,
}

//
// If cap_user_rdpmc this field provides the bit-width of the value
// read using the rdpmc() or equivalent instruction. This can be used
// to sign extend the result like:
//
// pmc <<= 64 - width;
// pmc >>= 64 - width; // signed shift right
// count += pmc;
//
// If cap_usr_time the below fields can be used to compute the time
// delta since time_enabled (in ns) using RDTSC or similar.
//
// u64 quot, rem;
// u64 delta;
//
// quot = (cyc >> time_shift);
// rem = cyc & (((u64)1 << time_shift) - 1);
// delta = time_offset + quot * time_mult +
// ((rem * time_mult) >> time_shift);
//
// Where time_offset,time_mult,time_shift and cyc are read in the
// seqcount loop described above. This delta can then be added to
// enabled and possible running (if index), improving the scaling:
//
// enabled += delta;
// if (index)
// running += delta;
//
// quot = count / running;
// rem  = count % running;
// count = quot * enabled + (rem * enabled) / running;
//
// If cap_usr_time_zero, the hardware clock (e.g. TSC) can be calculated
// from sample timestamps.
//
// time = timestamp - time_zero;
// quot = time / time_mult;
// rem  = time % time_mult;
// cyc = (quot << time_shift) + (rem << time_shift) / time_mult;
//
// And vice versa:
//
// quot = cyc >> time_shift;
// rem  = cyc & (((u64)1 << time_shift) - 1);
// timestamp = time_zero + quot * time_mult +
// ((rem * time_mult) >> time_shift);
//
// If cap_usr_time_short, the hardware clock is less than 64bit wide
// and we must compute the 'cyc' value, as used by cap_usr_time, as:
//
// cyc = time_cycles + ((cyc - time_cycles) & time_mask)
//
// NOTE: this form is explicitly chosen such that cap_usr_time_short
// is a correction on top of cap_usr_time, and code that doesn't
// know about cap_usr_time_short still works under the assumption
// the counter doesn't wrap.
//
// Hole for extension of the self monitor capabilities
//
// Control data for the mmap() data buffer.
//
// User-space reading the @data_head value should issue an smp_rmb(),
// after reading this value.
//
// When the mapping is PROT_WRITE the @data_tail value should be
// written by user-space to reflect the last read data, after issuing
// an smp_mb() to separate the data read from the ->data_tail store.
// In this case the kernel will not over-write unread data.
//
// See perf_output_put_handle() for the data ordering.
//
// data_{offset,size} indicate the location and size of the perf record
// buffer within the mmapped area.
//
// AUX area is defined by aux_{offset,size} fields that should be set
// by the user-space, so that
//
// aux_offset >= data_offset + data_size
//
// prior to mmap()ing it. Size of the mmap()ed area should be aux_size.
//
// Ring buffer pointers aux_{head,tail} have the same semantics as
// data_{head,tail} and same ordering rules apply.
//
// The current state of perf_event_header::misc bits usage:
// ('|' used bit, '-' unused bit)
//
// 012         CDEF
// |||---------||||
//
// Where:
// 0-2     CPUMODE_MASK
//
// C       PROC_MAP_PARSE_TIMEOUT
// D       MMAP_DATA / COMM_EXEC / FORK_EXEC / SWITCH_OUT
// E       MMAP_BUILD_ID / EXACT_IP / SCHED_OUT_PREEMPT
// F       (reserved)
//

//
// Indicates that /proc/PID/maps parsing are truncated by time out.
//

//
// Following PERF_RECORD_MISC_* are used on different
// events, so can reuse the same bit position:
//
// PERF_RECORD_MISC_MMAP_DATA  - PERF_RECORD_MMAP* events
// PERF_RECORD_MISC_COMM_EXEC  - PERF_RECORD_COMM event
// PERF_RECORD_MISC_FORK_EXEC  - PERF_RECORD_FORK event (perf internal)
// PERF_RECORD_MISC_SWITCH_OUT - PERF_RECORD_SWITCH* events
//

//
// These PERF_RECORD_MISC_* flags below are safely reused
// for the following events:
//
// PERF_RECORD_MISC_EXACT_IP           - PERF_RECORD_SAMPLE of precise events
// PERF_RECORD_MISC_SWITCH_OUT_PREEMPT - PERF_RECORD_SWITCH* events
// PERF_RECORD_MISC_MMAP_BUILD_ID      - PERF_RECORD_MMAP2 event
//
// PERF_RECORD_MISC_EXACT_IP:
// Indicates that the content of PERF_SAMPLE_IP points to
// the actual instruction that triggered the event. See also
// perf_event_attr::precise_ip.
//
// PERF_RECORD_MISC_SWITCH_OUT_PREEMPT:
// Indicates that thread was preempted in TASK_RUNNING state.
//
// PERF_RECORD_MISC_MMAP_BUILD_ID:
// Indicates that mmap2 event carries build ID data.
//

//
// Reserve the last bit to indicate some extended misc field
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_header {
    pub type: __u32,
    pub misc: __u16,
    pub size: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_ns_link_info {
    pub dev: __u64,
    pub ino: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_type {

//
// If perf_event_attr.sample_id_all is set then all event types will
// have the sample_type selected fields related to where/when
// (identity) an event took place (TID, TIME, ID, STREAM_ID, CPU,
// IDENTIFIER) described in PERF_RECORD_SAMPLE below, it will be stashed
// just after the perf_event_header and the fields already present for
// the existing fields, i.e. at the end of the payload. That way a newer
// perf.data file will be supported by older perf tools, with these new
// optional fields being ignored.
//
// struct sample_id {
// { u32			pid, tid; } && PERF_SAMPLE_TID
// { u64			time;     } && PERF_SAMPLE_TIME
// { u64			id;       } && PERF_SAMPLE_ID
// { u64			stream_id;} && PERF_SAMPLE_STREAM_ID
// { u32			cpu, res; } && PERF_SAMPLE_CPU
// { u64			id;	  } && PERF_SAMPLE_IDENTIFIER
// } && perf_event_attr::sample_id_all
//
// Note that PERF_SAMPLE_IDENTIFIER duplicates PERF_SAMPLE_ID.  The
// advantage of PERF_SAMPLE_IDENTIFIER is that its position is fixed
// relative to header.size.
//

//
// The MMAP events record the PROT_EXEC mappings so that we can
// correlate user-space IPs to code. They have the following structure:
//
// struct {
// struct perf_event_header	header;
//
// u32				pid, tid;
// u64				addr;
// u64				len;
// u64				pgoff;
// char				filename[];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_MMAP			= 1,

//
// struct {
// struct perf_event_header	header;
// u64				id;
// u64				lost;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_LOST			= 2,

//
// struct {
// struct perf_event_header	header;
//
// u32				pid, tid;
// char				comm[];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_COMM			= 3,

//
// struct {
// struct perf_event_header	header;
// u32				pid, ppid;
// u32				tid, ptid;
// u64				time;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_EXIT			= 4,

//
// struct {
// struct perf_event_header	header;
// u64				time;
// u64				id;
// u64				stream_id;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_THROTTLE			= 5,
    PERF_RECORD_UNTHROTTLE			= 6,

//
// struct {
// struct perf_event_header	header;
// u32				pid, ppid;
// u32				tid, ptid;
// u64				time;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_FORK			= 7,

//
// struct {
// struct perf_event_header	header;
// u32				pid, tid;
//
// struct read_format		values;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_READ			= 8,

//
// struct {
// struct perf_event_header	header;
//
// #
// # Note that PERF_SAMPLE_IDENTIFIER duplicates PERF_SAMPLE_ID.
// # The advantage of PERF_SAMPLE_IDENTIFIER is that its position
// # is fixed relative to header.
// #
//
// { u64			id;	  } && PERF_SAMPLE_IDENTIFIER
// { u64			ip;	  } && PERF_SAMPLE_IP
// { u32			pid, tid; } && PERF_SAMPLE_TID
// { u64			time;     } && PERF_SAMPLE_TIME
// { u64			addr;     } && PERF_SAMPLE_ADDR
// { u64			id;	  } && PERF_SAMPLE_ID
// { u64			stream_id;} && PERF_SAMPLE_STREAM_ID
// { u32			cpu, res; } && PERF_SAMPLE_CPU
// { u64			period;   } && PERF_SAMPLE_PERIOD
//
// { struct read_format	values;	  } && PERF_SAMPLE_READ
//
// { u64			nr,
// u64			ips[nr];  } && PERF_SAMPLE_CALLCHAIN
//
// #
// # The RAW record below is opaque data wrt the ABI
// #
// # That is, the ABI doesn't make any promises wrt to
// # the stability of its content, it may vary depending
// # on event, hardware, kernel version and phase of
// # the moon.
// #
// # In other words, PERF_SAMPLE_RAW contents are not an ABI.
// #
//
// { u32			size;
// char                  data[size];}&& PERF_SAMPLE_RAW
//
// { u64                   nr;
// { u64	hw_idx; } && PERF_SAMPLE_BRANCH_HW_INDEX
// { u64 from, to, flags } lbr[nr];
// #
// # The format of the counters is decided by the
// # "branch_counter_nr" and "branch_counter_width",
// # which are defined in the ABI.
// #
// { u64 counters; } cntr[nr] && PERF_SAMPLE_BRANCH_COUNTERS
// } && PERF_SAMPLE_BRANCH_STACK
//
// { u64			abi; # enum perf_sample_regs_abi
// u64			regs[weight(mask)]; } && PERF_SAMPLE_REGS_USER
//
// { u64			size;
// char			data[size];
// u64			dyn_size; } && PERF_SAMPLE_STACK_USER
//
// { union perf_sample_weight
// {
// u64		full; && PERF_SAMPLE_WEIGHT
// #if defined(__LITTLE_ENDIAN_BITFIELD)
// struct {
// u32	var1_dw;
// u16	var2_w;
// u16	var3_w;
// } && PERF_SAMPLE_WEIGHT_STRUCT
// #elif defined(__BIG_ENDIAN_BITFIELD)
// struct {
// u16	var3_w;
// u16	var2_w;
// u32	var1_dw;
// } && PERF_SAMPLE_WEIGHT_STRUCT
// #endif
// }
// { u64			data_src; } && PERF_SAMPLE_DATA_SRC
// { u64			transaction; } && PERF_SAMPLE_TRANSACTION
// { u64			abi; # enum perf_sample_regs_abi
// u64			regs[weight(mask)]; } && PERF_SAMPLE_REGS_INTR
// { u64			phys_addr;} && PERF_SAMPLE_PHYS_ADDR
// { u64			cgroup;} && PERF_SAMPLE_CGROUP
// { u64			data_page_size;} && PERF_SAMPLE_DATA_PAGE_SIZE
// { u64			code_page_size;} && PERF_SAMPLE_CODE_PAGE_SIZE
// { u64			size;
// char			data[size]; } && PERF_SAMPLE_AUX
// };
//
    PERF_RECORD_SAMPLE			= 9,

//
// The MMAP2 records are an augmented version of MMAP, they add
// maj, min, ino numbers to be used to uniquely identify each mapping
//
// struct {
// struct perf_event_header	header;
//
// u32				pid, tid;
// u64				addr;
// u64				len;
// u64				pgoff;
// union {
// struct {
// u32		maj;
// u32		min;
// u64		ino;
// u64		ino_generation;
// };
// struct {
// u8		build_id_size;
// u8		__reserved_1;
// u16		__reserved_2;
// u8		build_id[20];
// };
// u32				prot, flags;
// char				filename[];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_MMAP2			= 10,

//
// Records that new data landed in the AUX buffer part.
//
// struct {
// struct perf_event_header	header;
//
// u64				aux_offset;
// u64				aux_size;
// u64				flags;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_AUX				= 11,

//
// Indicates that instruction trace has started
//
// struct {
// struct perf_event_header	header;
// u32				pid;
// u32				tid;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_ITRACE_START		= 12,

//
// Records the dropped/lost sample number.
//
// struct {
// struct perf_event_header	header;
//
// u64				lost;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_LOST_SAMPLES		= 13,

//
// Records a context switch in or out (flagged by
// PERF_RECORD_MISC_SWITCH_OUT). See also
// PERF_RECORD_SWITCH_CPU_WIDE.
//
// struct {
// struct perf_event_header	header;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_SWITCH			= 14,

//
// CPU-wide version of PERF_RECORD_SWITCH with next_prev_pid and
// next_prev_tid that are the next (switching out) or previous
// (switching in) pid/tid.
//
// struct {
// struct perf_event_header	header;
// u32				next_prev_pid;
// u32				next_prev_tid;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_SWITCH_CPU_WIDE		= 15,

//
// struct {
// struct perf_event_header	header;
// u32				pid;
// u32				tid;
// u64				nr_namespaces;
// { u64				dev, inode; } [nr_namespaces];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_NAMESPACES			= 16,

//
// Record ksymbol register/unregister events:
//
// struct {
// struct perf_event_header	header;
// u64				addr;
// u32				len;
// u16				ksym_type;
// u16				flags;
// char				name[];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_KSYMBOL			= 17,

//
// Record BPF events:
// enum perf_bpf_event_type {
// PERF_BPF_EVENT_UNKNOWN		= 0,
// PERF_BPF_EVENT_PROG_LOAD	= 1,
// PERF_BPF_EVENT_PROG_UNLOAD	= 2,
// };
//
// struct {
// struct perf_event_header	header;
// u16				type;
// u16				flags;
// u32				id;
// u8				tag[BPF_TAG_SIZE];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_BPF_EVENT			= 18,

//
// struct {
// struct perf_event_header	header;
// u64				id;
// char				path[];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_CGROUP			= 19,

//
// Records changes to kernel text i.e. self-modified code. 'old_len' is
// the number of old bytes, 'new_len' is the number of new bytes. Either
// 'old_len' or 'new_len' may be zero to indicate, for example, the
// addition or removal of a trampoline. 'bytes' contains the old bytes
// followed immediately by the new bytes.
//
// struct {
// struct perf_event_header	header;
// u64				addr;
// u16				old_len;
// u16				new_len;
// u8				bytes[];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_TEXT_POKE			= 20,

//
// Data written to the AUX area by hardware due to aux_output, may need
// to be matched to the event by an architecture-specific hardware ID.
// This records the hardware ID, but requires sample_id to provide the
// event ID. e.g. Intel PT uses this record to disambiguate PEBS-via-PT
// records from multiple events.
//
// struct {
// struct perf_event_header	header;
// u64				hw_id;
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_AUX_OUTPUT_HW_ID		= 21,

//
// This user callchain capture was deferred until shortly before
// returning to user space.  Previous samples would have kernel
// callchains only and they need to be stitched with this to make full
// callchains.
//
// struct {
// struct perf_event_header	header;
// u64				cookie;
// u64				nr;
// u64				ips[nr];
// struct sample_id		sample_id;
// };
//
    PERF_RECORD_CALLCHAIN_DEFERRED		= 22,

    PERF_RECORD_MAX,			/* non-ABI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_record_ksymbol_type {
    PERF_RECORD_KSYMBOL_TYPE_UNKNOWN	= 0,
    PERF_RECORD_KSYMBOL_TYPE_BPF		= 1,
//
// Out of line code such as kprobe-replaced instructions or optimized
// kprobes or ftrace trampolines.
//
    PERF_RECORD_KSYMBOL_TYPE_OOL		= 2,
    PERF_RECORD_KSYMBOL_TYPE_MAX		/* non-ABI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_bpf_event_type {
    PERF_BPF_EVENT_UNKNOWN			= 0,
    PERF_BPF_EVENT_PROG_LOAD		= 1,
    PERF_BPF_EVENT_PROG_UNLOAD		= 2,
    PERF_BPF_EVENT_MAX,			/* non-ABI */
}

pub const PERF_MAX_STACK_DEPTH: c_int = 127;
pub const PERF_MAX_CONTEXTS_PER_STACK: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_callchain_context {
    PERF_CONTEXT_HV				= (__u64)-32,
    PERF_CONTEXT_KERNEL			= (__u64)-128,
    PERF_CONTEXT_USER			= (__u64)-512,
    PERF_CONTEXT_USER_DEFERRED		= (__u64)-640,

    PERF_CONTEXT_GUEST			= (__u64)-2048,
    PERF_CONTEXT_GUEST_KERNEL		= (__u64)-2176,
    PERF_CONTEXT_GUEST_USER			= (__u64)-2560,

    PERF_CONTEXT_MAX			= (__u64)-4095,
}

//
// PERF_RECORD_AUX::flags bits
//
pub const PERF_AUX_FLAG_TRUNCATED: c_uint = 0x0001	/* Record was truncated to fit */;
pub const PERF_AUX_FLAG_OVERWRITE: c_uint = 0x0002	/* Snapshot from overwrite mode */;
pub const PERF_AUX_FLAG_PARTIAL: c_uint = 0x0004	/* Record contains gaps */;
pub const PERF_AUX_FLAG_COLLISION: c_uint = 0x0008	/* Sample collided with another */;
pub const PERF_AUX_FLAG_PMU_FORMAT_TYPE_MASK: c_uint = 0xff00	/* PMU specific trace format type */;
// CoreSight PMU AUX buffer formats
pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_CORESIGHT: c_uint = 0x0000 /* Default for backward compatibility */;
pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_RAW: c_uint = 0x0100 /* Raw format of the source */;

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_mem_data_src {
    pub val: __u64,
    pub 13: mem_rsvd :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_mem_data_src {
    pub val: __u64,
    pub /: *mut *mut mem_op : 5; / Type of opcode,
}

// Type of memory opcode:
pub const PERF_MEM_OP_NA: c_uint = 0x0001 /* Not available */;
pub const PERF_MEM_OP_LOAD: c_uint = 0x0002 /* Load instruction */;
pub const PERF_MEM_OP_STORE: c_uint = 0x0004 /* Store instruction */;
pub const PERF_MEM_OP_PFETCH: c_uint = 0x0008 /* Prefetch */;
pub const PERF_MEM_OP_EXEC: c_uint = 0x0010 /* Code (execution) */;
pub const PERF_MEM_OP_SHIFT: c_int = 0;
//
// The PERF_MEM_LVL_* namespace is being deprecated to some extent in
// favour of newer composite PERF_MEM_{LVLNUM_,REMOTE_,SNOOPX_} fields.
// We support this namespace in order to not break defined ABIs.
//
// Memory hierarchy (memory level, hit or miss)
//
pub const PERF_MEM_LVL_NA: c_uint = 0x0001 /* Not available */;
pub const PERF_MEM_LVL_HIT: c_uint = 0x0002 /* Hit level */;
pub const PERF_MEM_LVL_MISS: c_uint = 0x0004 /* Miss level  */;
pub const PERF_MEM_LVL_L1: c_uint = 0x0008 /* L1 */;
pub const PERF_MEM_LVL_LFB: c_uint = 0x0010 /* Line Fill Buffer */;
pub const PERF_MEM_LVL_L2: c_uint = 0x0020 /* L2 */;
pub const PERF_MEM_LVL_L3: c_uint = 0x0040 /* L3 */;
pub const PERF_MEM_LVL_LOC_RAM: c_uint = 0x0080 /* Local DRAM */;
pub const PERF_MEM_LVL_REM_RAM1: c_uint = 0x0100 /* Remote DRAM (1 hop) */;
pub const PERF_MEM_LVL_REM_RAM2: c_uint = 0x0200 /* Remote DRAM (2 hops) */;
pub const PERF_MEM_LVL_REM_CCE1: c_uint = 0x0400 /* Remote Cache (1 hop) */;
pub const PERF_MEM_LVL_REM_CCE2: c_uint = 0x0800 /* Remote Cache (2 hops) */;
pub const PERF_MEM_LVL_IO: c_uint = 0x1000 /* I/O memory */;
pub const PERF_MEM_LVL_UNC: c_uint = 0x2000 /* Uncached memory */;
pub const PERF_MEM_LVL_SHIFT: c_int = 5;
pub const PERF_MEM_REMOTE_REMOTE: c_uint = 0x0001 /* Remote */;
pub const PERF_MEM_REMOTE_SHIFT: c_int = 37;
pub const PERF_MEM_LVLNUM_L1: c_uint = 0x0001 /* L1 */;
pub const PERF_MEM_LVLNUM_L2: c_uint = 0x0002 /* L2 */;
pub const PERF_MEM_LVLNUM_L3: c_uint = 0x0003 /* L3 */;
pub const PERF_MEM_LVLNUM_L4: c_uint = 0x0004 /* L4 */;
pub const PERF_MEM_LVLNUM_L2_MHB: c_uint = 0x0005 /* L2 Miss Handling Buffer */;
pub const PERF_MEM_LVLNUM_MSC: c_uint = 0x0006 /* Memory-side Cache */;
pub const PERF_MEM_LVLNUM_L0: c_uint = 0x0007 /* L0 */;
pub const PERF_MEM_LVLNUM_UNC: c_uint = 0x0008 /* Uncached */;
pub const PERF_MEM_LVLNUM_CXL: c_uint = 0x0009 /* CXL */;
pub const PERF_MEM_LVLNUM_IO: c_uint = 0x000a /* I/O */;
pub const PERF_MEM_LVLNUM_ANY_CACHE: c_uint = 0x000b /* Any cache */;
pub const PERF_MEM_LVLNUM_LFB: c_uint = 0x000c /* LFB / L1 Miss Handling Buffer */;
pub const PERF_MEM_LVLNUM_RAM: c_uint = 0x000d /* RAM */;
pub const PERF_MEM_LVLNUM_PMEM: c_uint = 0x000e /* PMEM */;
pub const PERF_MEM_LVLNUM_NA: c_uint = 0x000f /* N/A */;
pub const PERF_MEM_LVLNUM_SHIFT: c_int = 33;
// Snoop mode
pub const PERF_MEM_SNOOP_NA: c_uint = 0x0001 /* Not available */;
pub const PERF_MEM_SNOOP_NONE: c_uint = 0x0002 /* No snoop */;
pub const PERF_MEM_SNOOP_HIT: c_uint = 0x0004 /* Snoop hit */;
pub const PERF_MEM_SNOOP_MISS: c_uint = 0x0008 /* Snoop miss */;
pub const PERF_MEM_SNOOP_HITM: c_uint = 0x0010 /* Snoop hit modified */;
pub const PERF_MEM_SNOOP_SHIFT: c_int = 19;
pub const PERF_MEM_SNOOPX_FWD: c_uint = 0x0001 /* Forward */;
pub const PERF_MEM_SNOOPX_PEER: c_uint = 0x0002 /* Transfer from peer */;
pub const PERF_MEM_SNOOPX_SHIFT: c_int = 38;
// Locked instruction
pub const PERF_MEM_LOCK_NA: c_uint = 0x0001 /* Not available */;
pub const PERF_MEM_LOCK_LOCKED: c_uint = 0x0002 /* Locked transaction */;
pub const PERF_MEM_LOCK_SHIFT: c_int = 24;
// TLB access
pub const PERF_MEM_TLB_NA: c_uint = 0x0001 /* Not available */;
pub const PERF_MEM_TLB_HIT: c_uint = 0x0002 /* Hit level */;
pub const PERF_MEM_TLB_MISS: c_uint = 0x0004 /* Miss level */;
pub const PERF_MEM_TLB_L1: c_uint = 0x0008 /* L1 */;
pub const PERF_MEM_TLB_L2: c_uint = 0x0010 /* L2 */;
pub const PERF_MEM_TLB_WK: c_uint = 0x0020 /* Hardware Walker*/;
pub const PERF_MEM_TLB_OS: c_uint = 0x0040 /* OS fault handler */;
pub const PERF_MEM_TLB_SHIFT: c_int = 26;
// Access blocked
pub const PERF_MEM_BLK_NA: c_uint = 0x0001 /* Not available */;
pub const PERF_MEM_BLK_DATA: c_uint = 0x0002 /* Data could not be forwarded */;
pub const PERF_MEM_BLK_ADDR: c_uint = 0x0004 /* Address conflict */;
pub const PERF_MEM_BLK_SHIFT: c_int = 40;
// Hop level
pub const PERF_MEM_HOPS_0: c_uint = 0x0001 /* Remote core, same node */;
pub const PERF_MEM_HOPS_1: c_uint = 0x0002 /* Remote node, same socket */;
pub const PERF_MEM_HOPS_2: c_uint = 0x0003 /* Remote socket, same board */;
pub const PERF_MEM_HOPS_3: c_uint = 0x0004 /* Remote board */;
// 5-7 available
pub const PERF_MEM_HOPS_SHIFT: c_int = 43;
// Cache/Memory region
pub const PERF_MEM_REGION_NA: c_uint = 0x0  /* Invalid */;
pub const PERF_MEM_REGION_RSVD: c_uint = 0x01 /* Reserved */;
pub const PERF_MEM_REGION_L_SHARE: c_uint = 0x02 /* Local CA shared cache */;
pub const PERF_MEM_REGION_L_NON_SHARE: c_uint = 0x03 /* Local CA non-shared cache */;
pub const PERF_MEM_REGION_O_IO: c_uint = 0x04 /* Other CA IO agent */;
pub const PERF_MEM_REGION_O_SHARE: c_uint = 0x05 /* Other CA shared cache */;
pub const PERF_MEM_REGION_O_NON_SHARE: c_uint = 0x06 /* Other CA non-shared cache */;
pub const PERF_MEM_REGION_MMIO: c_uint = 0x07 /* MMIO */;
pub const PERF_MEM_REGION_MEM0: c_uint = 0x08 /* Memory region 0 */;
pub const PERF_MEM_REGION_MEM1: c_uint = 0x09 /* Memory region 1 */;
pub const PERF_MEM_REGION_MEM2: c_uint = 0x0a /* Memory region 2 */;
pub const PERF_MEM_REGION_MEM3: c_uint = 0x0b /* Memory region 3 */;
pub const PERF_MEM_REGION_MEM4: c_uint = 0x0c /* Memory region 4 */;
pub const PERF_MEM_REGION_MEM5: c_uint = 0x0d /* Memory region 5 */;
pub const PERF_MEM_REGION_MEM6: c_uint = 0x0e /* Memory region 6 */;
pub const PERF_MEM_REGION_MEM7: c_uint = 0x0f /* Memory region 7 */;
pub const PERF_MEM_REGION_SHIFT: c_int = 46;

//
// Layout of single taken branch records:
//
// from: source instruction (may not always be a branch insn)
// to: branch target
// mispred: branch target was mispredicted
// predicted: branch target was predicted
//
// support for mispred, predicted is optional. In case it
// is not supported mispred = predicted = 0.
//
// in_tx: running in a hardware transaction
// abort: aborting a hardware transaction
// cycles: cycles from last branch (or 0 if not supported)
// type: branch type
// spec: branch speculation info (or 0 if not supported)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_branch_entry {
    pub from: __u64,
    pub to: __u64,
    pub 31: reserved :,
}

// Size of used info bits in struct perf_branch_entry
pub const PERF_BRANCH_ENTRY_INFO_BITS_MAX: c_int = 33;
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_sample_weight {
    pub full: __u64,

    pub var1_dw: __u32,
    pub var2_w: __u16,
    pub var3_w: __u16,
}

