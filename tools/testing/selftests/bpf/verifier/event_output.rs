//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/verifier/event_output.c
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


// instructions used to output a skb based software event, produced
// from code snippet:
// struct TMP {
// uint64_t tmp;
// } tt;
// tt.tmp = 5;
// bpf_perf_event_output(skb, &connection_tracking_event_map, 0,
// &tt, sizeof(tt));
// return 1;
//
// the bpf assembly from llvm is:
// 0:       b7 02 00 00 05 00 00 00         r2 = 5
// 1:       7b 2a f8 ff 00 00 00 00         *(u64 *)(r10 - 8) = r2
// 2:       bf a4 00 00 00 00 00 00         r4 = r10
// 3:       07 04 00 00 f8 ff ff ff         r4 += -8
// 4:       18 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00    r2 = 0ll
// 6:       b7 03 00 00 00 00 00 00         r3 = 0
// 7:       b7 05 00 00 08 00 00 00         r5 = 8
// 8:       85 00 00 00 19 00 00 00         call 25
// 9:       b7 00 00 00 01 00 00 00         r0 = 1
// 10:       95 00 00 00 00 00 00 00         exit
//
// The reason I put the code here instead of fill_helpers is that map fixup
// is against the insns, instead of filled prog.
//

    BPF_MOV64_IMM(BPF_REG_2, 5),				\
    BPF_STX_MEM(BPF_DW, BPF_REG_10, BPF_REG_2, -8),		\
    BPF_MOV64_REG(BPF_REG_4, BPF_REG_10),			\
    BPF_ALU64_IMM(BPF_ADD, BPF_REG_4, -8),			\
    BPF_LD_MAP_FD(BPF_REG_2, 0),				\
    BPF_MOV64_IMM(BPF_REG_3, 0),				\
    BPF_MOV64_IMM(BPF_REG_5, 8),				\
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0,		\
    BPF_FUNC_perf_event_output),		\
    BPF_MOV64_IMM(BPF_REG_0, 1),				\
    BPF_EXIT_INSN(),
    {
    "perfevent for sockops",
    .insns = { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_SOCK_OPS,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for tc",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_SCHED_CLS,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for lwt out",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_LWT_OUT,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for xdp",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_XDP,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for socket filter",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_SOCKET_FILTER,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for sk_skb",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_SK_SKB,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for cgroup skb",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_CGROUP_SKB,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for cgroup dev",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_CGROUP_DEVICE,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for cgroup sysctl",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_CGROUP_SYSCTL,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
    {
    "perfevent for cgroup sockopt",
    .insns =  { __PERF_EVENT_INSNS__ },
    .prog_type = BPF_PROG_TYPE_CGROUP_SOCKOPT,
    .expected_attach_type = BPF_CGROUP_SETSOCKOPT,
    .fixup_map_event_output = { 4 },
    .result = ACCEPT,
    .retval = 1,
    },
