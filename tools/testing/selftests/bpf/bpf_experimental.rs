//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bpf_experimental.h
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


// Convenience macro to wrap over bpf_obj_new

// Convenience macro to wrap over bpf_percpu_obj_new

// Description
// Throw a BPF exception from the program, immediately terminating its
// execution and unwinding the stack. The supplied 'cookie' parameter
// will be the return value of the program when an exception is thrown,
// and the default exception callback is used. Otherwise, if an exception
// callback is set using the '__exception_cb(callback)' declaration tag
// on the main program, the 'cookie' parameter will be the callback's only
// input argument.
//
// Thus, in case of default exception callback, 'cookie' is subjected to
// constraints on the program's return value (as with R0 on exit).
// Otherwise, the return value of the marked exception callback will be
// subjected to the same checks.
//
// Note that throwing an exception with lingering resources (locks,
// references, etc.) will lead to a verification error.
//
// Note that callbacks *cannot* call this helper.
// Returns
// Never.
// Throws
// An exception with the specified 'cookie' value.
//
// Description
// Acquire a reference on the exe_file member field belonging to the
// mm_struct that is nested within the supplied task_struct. The supplied
// task_struct must be trusted/referenced.
// Returns
// A referenced file pointer pointing to the exe_file member field of the
// mm_struct nested in the supplied task_struct, or NULL.
//
// Description
// Release a reference on the supplied file. The supplied file must be
// acquired.
//
// Description
// Resolve a pathname for the supplied path and store it in the supplied
// buffer. The supplied path must be trusted/referenced.
// Returns
// A positive integer corresponding to the length of the resolved pathname,
// including the NULL termination character, stored in the supplied
// buffer. On error, a negative integer is returned.
//
// This macro must be used to mark the exception callback corresponding to the
// main program. For example:
//
// int exception_cb(u64 cookie) {
// return cookie;
// }
//
// SEC("tc")
// __exception_cb(exception_cb)
// int main_prog(struct __sk_buff *ctx) {
// ...
// return TC_ACT_OK;
// }
//
// Here, exception callback for the main program will be 'exception_cb'. Note
// that this attribute can only be used once, and multiple exception callbacks
// specified for the main program will lead to verification error.
//

// C type conversions coupled with comparison operator are tricky.
// Make sure BPF program is compiled with -Wsign-compare then
// __lhs OP __rhs below will catch the mistake.
// Be aware that we check only __lhs to figure out the sign of compare.
//

// "i" will truncate 64-bit constant into s32,			\
// so we have to use extra register via "r".			\
// \

// emit instruction:
// rX = rX .off = BPF_ADDR_SPACE_CAST .imm32 = (dst_as << 16) | src_as
//

// Description
// Assert that a conditional expression is true.
// Returns
// Void.
// Throws
// An exception with the value zero when the assertion fails.
//

// Description
// Assert that a conditional expression is true.
// Returns
// Void.
// Throws
// An exception with the specified value when the assertion fails.
//

// Description
// Assert that LHS is in the range [BEG, END] (inclusive of both). This
// statement updates the known bounds of LHS during verification. Note
// that both BEG and END must be constant values, and must fit within the
// data type of LHS.
// Returns
// Void.
// Throws
// An exception with the value zero when the assertion fails.
//

// Description
// Assert that LHS is in the range [BEG, END] (inclusive of both). This
// statement updates the known bounds of LHS during verification. Note
// that both BEG and END must be constant values, and must fit within the
// data type of LHS.
// Returns
// Void.
// Throws
// An exception with the specified value when the assertion fails.
//

pub const PREEMPT_BITS: c_int = 8;
pub const SOFTIRQ_BITS: c_int = 8;
pub const HARDIRQ_DISABLE_BITS: c_int = 8;
pub const HARDIRQ_BITS: c_int = 4;
pub const NMI_BITS: c_int = 1;
pub const PREEMPT_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_hot___local {
    pub preempt_count: c_int,
    pub __attribute__((preserve_access_index)): },
    pub __weak: extern struct pcpu_hot___local pcpu_hot __ksym,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct___preempt_rt {
    pub softirq_disable_cnt: c_int,
    pub __attribute__((preserve_access_index)): },

    pub __ksym: *mut *mut extern struct lowcore bpf_get_lowcore(void) __weak,

// By default, read the per-CPU __preempt_count.
    pub bpf_this_cpu_ptr(&__preempt_count): *mut *mut *mut return (int ),
//
// If __preempt_count does not exist, try to read preempt_count under
// struct pcpu_hot. Between v6.1 and v6.14 -- more specifically,
// [64701838bf057, 46e8fff6d45fe), preempt_count had been managed
// under struct pcpu_hot.
//

    pub bpf_get_current_task_btf()->thread_info.preempt.count: return,

    pub bpf_get_current_task_btf()->thread_info.preempt_count: return,

    pub bpf_get_lowcore()->preempt_count: return,

    pub bpf_get_current_task_btf()->thread_info.preempt_count: return,

    pub bpf_get_current_task_btf()->thread_info.preempt_count: return,

    pub 0: return,
// Description
// Report whether it is in interrupt context. Only works on the following archs:
// * x86
// * arm64
// * powerpc64
// * s390x
// * loongarch
// * riscv
//
    pub tsk: *mut task_struct___preempt_rt,
    pub pcnt: c_int,
    pub get_preempt_count(): pcnt =,
    pub SOFTIRQ_MASK): return pcnt & (NMI_MASK | HARDIRQ_MASK |,
    pub bpf_get_current_task_btf(): *mut *mut tsk = (void ),
    pub SOFTIRQ_MASK): (tsk->softirq_disable_cnt &,
// Description
// Report whether it is in NMI context. Only works on the following archs:
// * x86
// * arm64
// * powerpc64
// * s390x
// * loongarch
// * riscv
//
    pub NMI_MASK: return get_preempt_count() &,
// Description
// Report whether it is in hard IRQ context. Only works on the following archs:
// * x86
// * arm64
// * powerpc64
// * s390x
// * loongarch
// * riscv
//
    pub HARDIRQ_MASK: return get_preempt_count() &,
// Description
// Report whether it is in softirq context. Only works on the following archs:
// * x86
// * arm64
// * powerpc64
// * s390x
// * loongarch
// * riscv
//
    pub tsk: *mut task_struct___preempt_rt,
    pub pcnt: c_int,
    pub get_preempt_count(): pcnt =,
    pub SOFTIRQ_OFFSET: return (pcnt & SOFTIRQ_MASK) &,
    pub bpf_get_current_task_btf(): *mut *mut tsk = (void ),
    pub SOFTIRQ_OFFSET: return (tsk->softirq_disable_cnt & SOFTIRQ_MASK) &,
// Description
// Report whether it is in task context. Only works on the following archs:
// * x86
// * arm64
// * powerpc64
// * s390x
// * loongarch
// * riscv
//
    pub tsk: *mut task_struct___preempt_rt,
    pub pcnt: c_int,
    pub get_preempt_count(): pcnt =,
    pub SOFTIRQ_OFFSET)): return !(pcnt & (NMI_MASK | HARDIRQ_MASK |,
    pub bpf_get_current_task_btf(): *mut *mut tsk = (void ),
    pub SOFTIRQ_OFFSET)): ((tsk->softirq_disable_cnt & SOFTIRQ_MASK) &,
