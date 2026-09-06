//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/riscv/include/uapi/asm/unistd.h
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
// Copyright (C) 2018 David Abdurachmanov <david.abdurachmanov@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

//
// Allows the instruction cache to be flushed from userspace.  Despite RISC-V
// having a direct 'fence.i' instruction available to userspace (which we
// can't trap!), that's not actually viable when running on Linux because the
// kernel might schedule a process on another hart.  There is no way for
// userspace to handle this without invoking the kernel (as it doesn't know the
// thread->hart mappings), so we've defined a RISC-V specific system call to
// flush the instruction cache.
//
// __NR_riscv_flush_icache is defined to flush the instruction cache over an
// address range, with the flush applying to either all threads or just the
// caller.  We don't currently do anything with the address range, that's just
// in there for forwards compatibility.
//

