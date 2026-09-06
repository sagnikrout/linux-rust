//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/bpf_misc.h
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

// Expand a macro and then stringize the expansion

// This set of attributes controls behavior of the
// test_loader.c:test_loader__run_subtests().
//
// The test_loader sequentially loads each program in a skeleton.
// Programs could be loaded in privileged and unprivileged modes.
// - __success, __failure, __msg, __regex imply privileged mode;
// - __success_unpriv, __failure_unpriv, __msg_unpriv, __regex_unpriv
// imply unprivileged mode.
// If combination of privileged and unprivileged attributes is present
// both modes are used. If none are present privileged mode is implied.
//
// See test_loader.c:drop_capabilities() for exact set of capabilities
// that differ between privileged and unprivileged modes.
//
// For test filtering purposes the name of the program loaded in
// unprivileged mode is derived from the usual program name by adding
// `@unpriv' suffix.
//
// __msg             Message expected to be found in the verifier log.
// Multiple __msg attributes could be specified.
// To match a regular expression use "{{" "}}" brackets,
// e.g. "foo{{[0-9]+}}"  matches strings like "foo007".
// Extended POSIX regular expression syntax is allowed
// inside the brackets.
// __not_msg         Message not expected to be found in verifier log.
// If __msg_not is situated between __msg tags
// framework matches __msg tags first, and then
// checks that __msg_not is not present in a portion of
// a log between bracketing __msg tags.
// Same regex syntax as for __msg is supported.
// __msg_unpriv      Same as __msg but for unprivileged mode.
// __not_msg_unpriv  Same as __not_msg but for unprivileged mode.
//
// __stderr          Message expected to be found in bpf stderr stream. The
// same regex rules apply like __msg.
// __stderr_unpriv   Same as __stderr but for unpriveleged mode.
// __stdout          Same as __stderr but for stdout stream.
// __stdout_unpriv   Same as __stdout but for unpriveleged mode.
//
// __xlated          Expect a line in a disassembly log after verifier applies rewrites.
// Multiple __xlated attributes could be specified.
// Regular expressions could be specified same way as in __msg.
// __xlated_unpriv   Same as __xlated but for unprivileged mode.
//
// __jited           Match a line in a disassembly of the jited BPF program.
// Has to be used after __arch_* macro.
// For example:
//
// __arch_x86_64
// __jited("   endbr64")
// __jited("   nopl    (%rax,%rax)")
// __jited("   xorq    %rax, %rax")
// ...
// __naked void some_test(void)
// {
// asm volatile (... ::: __clobber_all);
// }
//
// Regular expressions could be included in patterns same way
// as in __msg.
//
// By default assume that each pattern has to be matched on the
// next consecutive line of disassembly, e.g.:
//
// __jited("   endbr64")             # matched on line N
// __jited("   nopl    (%rax,%rax)") # matched on line N+1
//
// If match occurs on a wrong line an error is reported.
// To override this behaviour use literal "...", e.g.:
//
// __jited("   endbr64")             # matched on line N
// __jited("...")                    # not matched
// __jited("   nopl    (%rax,%rax)") # matched on any line >= N
//
// __jited_unpriv    Same as __jited but for unprivileged mode.
//
// __success         Expect program load success in privileged mode.
// __success_unpriv  Expect program load success in unprivileged mode.
//
// __failure         Expect program load failure in privileged mode.
// __failure_unpriv  Expect program load failure in unprivileged mode.
//
// __retval          Execute the program using BPF_PROG_TEST_RUN command,
// expect return value to match passed parameter:
// - a decimal number
// - a hexadecimal number, when starts from 0x
// - a macro which expands to one of the above
// - literal _INT_MIN (expands to INT_MIN)
// In addition, two special macros are defined below:
// - POINTER_VALUE
// - TEST_DATA_LEN
// __retval_unpriv   Same, but load program in unprivileged mode.
//
// __description     Text to be used for display and as an additional filter
// alias, while the original program name stays matchable.
//
// __log_level       Log level to use for the program, numeric value expected.
//
// __flag            Adds one flag use for the program, the following values are valid:
// - BPF_F_STRICT_ALIGNMENT;
// - BPF_F_TEST_RND_HI32;
// - BPF_F_TEST_STATE_FREQ;
// - BPF_F_SLEEPABLE;
// - BPF_F_XDP_HAS_FRAGS;
// - A numeric value.
// Multiple __flag attributes could be specified, the final flags
// value is derived by applying binary "or" to all specified values.
//
// __auxiliary         Annotated program is not a separate test, but used as auxiliary
// for some other test cases and should always be loaded.
// __auxiliary_unpriv  Same, but load program in unprivileged mode.
//
// __arch_*          Specify on which architecture the test case should be tested.
// Several __arch_* annotations could be specified at once.
// When test case is not run on current arch it is marked as skipped.
// __caps_unpriv     Specify the capabilities that should be set when running the test.
//
// __linear_size     Specify the size of the linear area of non-linear skbs, or
// 0 for linear skbs.
//

// Define common capabilities tested using __caps_unpriv
pub const CAP_NET_ADMIN: c_int = 12;
pub const CAP_SYS_ADMIN: c_int = 21;
pub const CAP_PERFMON: c_int = 38;
pub const CAP_BPF: c_int = 39;
// Convenience macro for use with 'asm volatile' blocks

// Magic constants used with __retval()
pub const POINTER_VALUE: c_uint = 0xbadcafe;
pub const TEST_DATA_LEN: c_int = 64;

pub const SYSCALL_WRAPPER: c_int = 1;

pub const SYSCALL_WRAPPER: c_int = 1;

pub const SYSCALL_WRAPPER: c_int = 1;

pub const SYSCALL_WRAPPER: c_int = 1;

pub const SYSCALL_WRAPPER: c_int = 1;

pub const SYSCALL_WRAPPER: c_int = 0;

// How many arguments are passed to function in register

pub const FUNC_REG_ARG_CNT: c_int = 6;

pub const FUNC_REG_ARG_CNT: c_int = 3;

pub const FUNC_REG_ARG_CNT: c_int = 5;

pub const FUNC_REG_ARG_CNT: c_int = 4;

pub const FUNC_REG_ARG_CNT: c_int = 8;

pub const FUNC_REG_ARG_CNT: c_int = 8;

pub const FUNC_REG_ARG_CNT: c_int = 8;

pub const FUNC_REG_ARG_CNT: c_int = 6;

pub const FUNC_REG_ARG_CNT: c_int = 8;

// default to 5 for others
pub const FUNC_REG_ARG_CNT: c_int = 5;

// make it look to compiler like value is read and written

// Macro flag: #define CAN_USE_GOTOL

// Macro flag: #define CAN_USE_BPF_ST

// Macro flag: #define CAN_USE_LOAD_ACQ_STORE_REL

// Macro flag: #define SPEC_V1

// Macro flag: #define SPEC_V4

