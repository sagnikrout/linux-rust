//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/moduleloader.h
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
// The stuff needed for archs to support modules.

// These may be implemented by architectures that need to hook into the
// module loader code.  Architectures that don't need to do anything special
// can just rely on the 'weak' default hooks defined in kernel/module.c.
// Note, however, that at least one of apply_relocate or apply_relocate_add
// must be implemented by each architecture.
//
// arch may override to do additional checking of ELF header architecture
extern "C" {
    pub fn module_elf_check_arch(hdr: *mut Elf_Ehdr) -> bool;
}
// Adjust arch-specific sections.  Return 0 on success.
// Additional bytes needed by arch in front of individual sections
extern "C" {
    pub fn arch_mod_section_prepend(mod: *mut module, section: c_uint) -> c_uint;
}
// Determines if the section name is an init section (that is only used during
// module loading).
//
extern "C" {
    pub fn module_init_section(name: *const c_char) -> bool;
}
// Determines if the section name is an exit section (that is only used during
// module unloading)
//
extern "C" {
    pub fn module_exit_section(name: *const c_char) -> bool;
}
// Describes whether within_module_init() will consider this an init section
// or not. This behaviour changes with CONFIG_MODULE_UNLOAD.
//
extern "C" {
    pub fn module_init_layout_section(sname: *const c_char) -> bool;
}
//
// Apply the given relocation to the (simplified) ELF.  Return -error
// or 0.
//

//
// Apply the given add relocation to the (simplified) ELF.  Return
// -error or 0
//

//
// Some architectures (namely x86_64 and ppc64) perform sanity checks when
// applying relocations.  If a patched module gets unloaded and then later
// reloaded (and re-patched), klp re-applies relocations to the replacement
// function(s).  Any leftover relocations from the previous loading of the
// patched module might trigger the sanity checks.
//
// To prevent that, when unloading a patched module, clear out any relocations
// that might trigger arch-specific sanity checks on a future module reload.
//

// Any final processing of module before access.  Return -error or 0.

extern "C" {
    pub fn flush_module_init_free_work();
}

// Any cleanup needed when module leaves.
extern "C" {
    pub fn module_arch_cleanup(mod: *mut module);
}
// Any cleanup before freeing mod->module_init
extern "C" {
    pub fn module_arch_freeing_init(mod: *mut module);
}
