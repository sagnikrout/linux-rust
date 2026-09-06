//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/hypercall.h
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


//
// hypercall.h
//
// Linux-specific hypervisor handling.
//
// Copyright (c) 2002-2004, K A Fraser
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// The hypercall asms have to meet several constraints:
// - Work on 32- and 64-bit.
// The two architectures put their arguments in different sets of
// registers.
//
// - Work around asm syntax quirks
// It isn't possible to specify one of the rNN registers in a
// constraint, so we use explicit register variables to get the
// args into the right place.
//
// - Mark all registers as potentially clobbered
// Even unused parameters can be clobbered by the hypervisor, so we
// need to make sure gcc knows it.
//
// - Avoid compiler bugs.
// This is the tricky part.  Because x86_32 has such a constrained
// register set, gcc versions below 4.3 have trouble generating
// code when all the arg registers and memory are trashed by the
// asm.  There are syntactically simpler ways of achieving the
// semantics below, but they cause the compiler to crash.
//
// The only combination I found which works is:
// - assign the __argX variables first
// - list all actually used parameters as "+r" (__argX)
// - clobber the rest
//
// The result certainly isn't pretty, and it really shows up cpp's
// weakness as a macro language.  Sorry.  (But let's just give thanks
// there aren't more than 5 arguments...)
//
extern "C" {
    pub fn xen_hypercall_func();
}

// Macro flag: #define __ADDRESSABLE_xen_hypercall

// Macro flag: #define __HYPERCALL_0ARG()

//
// Suppress objtool seeing the STAC/CLAC and getting confused about it
// calling random code with AC=1.
//
extern "C" {
    pub fn volatile("memory": ASM_STAC_UNSAFE :::, _arg: "flags") -> asm;
}
extern "C" {
    pub fn volatile("memory": ASM_CLAC_UNSAFE :::, _arg: "flags") -> asm;
}

extern "C" {
    pub fn _hypercall1(_arg: c_int, _arg: set_trap_table, _arg: table) -> return;
}
extern "C" {
    pub fn _hypercall4(_arg: c_int, _arg: mmu_update, _arg: req, _arg: count, _arg: success_count, _arg: domid) -> return;
}
extern "C" {
    pub fn _hypercall4(_arg: c_int, _arg: mmuext_op, _arg: op, _arg: count, _arg: success_count, _arg: domid) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: set_gdt, _arg: frame_list, _arg: entries) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: callback_op, _arg: cmd, _arg: arg) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: set_debugreg, _arg: reg, _arg: value) -> return;
}
extern "C" {
    pub fn _hypercall1(long: unsigned, _arg: get_debugreg, _arg: reg) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: update_descriptor, _arg: ma, _arg: desc) -> return;
}
extern "C" {
    pub fn _hypercall3(_arg: c_int, _arg: update_va_mapping, _arg: va, _arg: new_val.pte, _arg: flags) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: set_segment_base, _arg: reg, _arg: value) -> return;
}

extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: sched_op, _arg: cmd, _arg: arg) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_long, _arg: set_timer_op, _arg: timeout_lo, _arg: timeout_hi) -> return;
}
extern "C" {
    pub fn _hypercall1(_arg: c_int, _arg: mca, _arg: mc_op) -> return;
}
extern "C" {
    pub fn _hypercall1(_arg: c_int, _arg: platform_op, _arg: op) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_long, _arg: memory_op, _arg: cmd, _arg: arg) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: multicall, _arg: call_list, _arg: nr_calls) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: event_channel_op, _arg: cmd, _arg: arg) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: xen_version, _arg: cmd, _arg: arg) -> return;
}
extern "C" {
    pub fn _hypercall3(_arg: c_int, _arg: console_io, _arg: cmd, _arg: count, _arg: str) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: physdev_op, _arg: cmd, _arg: arg) -> return;
}
extern "C" {
    pub fn _hypercall3(_arg: c_int, _arg: grant_table_op, _arg: cmd, _arg: uop, _arg: count) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: vm_assist, _arg: cmd, _arg: type) -> return;
}
extern "C" {
    pub fn _hypercall3(_arg: c_int, _arg: vcpu_op, _arg: cmd, _arg: vcpuid, _arg: extra_args) -> return;
}
//
// For a PV guest the tools require that the start_info mfn be
// present in rdx/edx when the hypercall is made. Per the
// hypercall calling convention this is the third hypercall
// argument, which is start_info_mfn here.
//
extern "C" {
    pub fn _hypercall3(_arg: c_int, _arg: sched_op, _arg: SCHEDOP_shutdown, _arg: &r, _arg: start_info_mfn) -> return;
}
extern "C" {
    pub fn _hypercall2(long: unsigned, _arg: hvm_op, _arg: op, _arg: arg) -> return;
}
extern "C" {
    pub fn _hypercall2(_arg: c_int, _arg: xenpmu_op, _arg: op, _arg: arg) -> return;
}
