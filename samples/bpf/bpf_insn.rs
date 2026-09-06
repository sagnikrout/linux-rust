//! Automatically rewritten from C Header to Rust Module
//! Source: samples/bpf/bpf_insn.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// eBPF instruction mini library
// ALU ops on registers, bpf_add|sub|...: dst_reg += src_reg

// ALU ops on immediates, bpf_add|sub|...: dst_reg += imm32

// Short form of mov, dst_reg = src_reg

// Short form of mov, dst_reg = imm32

// BPF_LD_IMM64 macro encodes single 'load 64-bit immediate' insn

// pseudo BPF_LD_IMM64 insn used to refer to process-local map_fd

// Direct packet access, R0 = *(uint *) (skb->data + imm32)

// Memory load, dst_reg = *(uint *) (src_reg + off16)

// Memory store, *(uint *) (dst_reg + off16) = src_reg

//
// Atomic operations:
//
// BPF_ADD                  *(uint *) (dst_reg + off16) += src_reg
// BPF_AND                  *(uint *) (dst_reg + off16) &= src_reg
// BPF_OR                   *(uint *) (dst_reg + off16) |= src_reg
// BPF_XOR                  *(uint *) (dst_reg + off16) ^= src_reg
// BPF_ADD | BPF_FETCH      src_reg = atomic_fetch_add(dst_reg + off16, src_reg);
// BPF_AND | BPF_FETCH      src_reg = atomic_fetch_and(dst_reg + off16, src_reg);
// BPF_OR | BPF_FETCH       src_reg = atomic_fetch_or(dst_reg + off16, src_reg);
// BPF_XOR | BPF_FETCH      src_reg = atomic_fetch_xor(dst_reg + off16, src_reg);
// BPF_XCHG                 src_reg = atomic_xchg(dst_reg + off16, src_reg)
// BPF_CMPXCHG              r0 = atomic_cmpxchg(dst_reg + off16, r0, src_reg)
//

// Legacy alias

// Memory store, *(uint *) (dst_reg + off16) = imm32

// Conditional jumps against registers, if (dst_reg 'op' src_reg) goto pc + off16

// Like BPF_JMP_REG, but with 32-bit wide operands for comparison.

// Conditional jumps against immediates, if (dst_reg 'op' imm32) goto pc + off16

// Like BPF_JMP_IMM, but with 32-bit wide operands for comparison.

// Raw code statement block

// Program exit

