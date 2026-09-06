//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kexec.h
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
// kexec system call -  It loads the new kernel to boot into.
// kexec does not sync, or unmount filesystems so if you need
// that to happen you need to do that yourself.
//

// kexec flags for different usage scenarios
pub const KEXEC_ON_CRASH: c_uint = 0x00000001;
pub const KEXEC_PRESERVE_CONTEXT: c_uint = 0x00000002;
pub const KEXEC_UPDATE_ELFCOREHDR: c_uint = 0x00000004;
pub const KEXEC_CRASH_HOTPLUG_SUPPORT: c_uint = 0x00000008;
pub const KEXEC_ARCH_MASK: c_uint = 0xffff0000;
//
// Kexec file load interface flags.
// KEXEC_FILE_UNLOAD : Unload already loaded kexec/kdump image.
// KEXEC_FILE_ON_CRASH : Load/unload operation belongs to kdump image.
// KEXEC_FILE_NO_INITRAMFS : No initramfs is being loaded. Ignore the initrd
// fd field.
// KEXEC_FILE_FORCE_DTB : Force carrying over the current boot's DTB to the new
// kernel on x86. This is already the default behavior on
// some other architectures, like ARM64 and PowerPC.
//
pub const KEXEC_FILE_UNLOAD: c_uint = 0x00000001;
pub const KEXEC_FILE_ON_CRASH: c_uint = 0x00000002;
pub const KEXEC_FILE_NO_INITRAMFS: c_uint = 0x00000004;
pub const KEXEC_FILE_DEBUG: c_uint = 0x00000008;
pub const KEXEC_FILE_NO_CMA: c_uint = 0x00000010;
pub const KEXEC_FILE_FORCE_DTB: c_uint = 0x00000020;
// These values match the ELF architecture values.
// Unless there is a good reason that should continue to be the case.
//

// The artificial cap on the number of segments passed to kexec_load.
pub const KEXEC_SEGMENT_MAX: c_int = 16;
//
// This structure is used to hold the arguments that are used when
// loading  kernel binaries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_segment {
    pub buf: *const c_void,
    pub bufsz: __kernel_size_t,
    pub mem: *const c_void,
    pub memsz: __kernel_size_t,
}

