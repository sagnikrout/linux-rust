//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/init.h
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

// These macros are used to mark some functions or
// initialized data (doesn't apply to uninitialized data)
// as `initialization' functions. The kernel can take this
// as hint that the function is used only during the initialization
// phase and free up used memory resources after
//
// Usage:
// For functions:
//
// You should add __init immediately before the function name, like:
//
// static void __init initme(int x, int y)
// {
// extern int z; z = x * y;
// }
//
// If the function has a prototype somewhere, you can also add
// __init between closing brace of the prototype and semicolon:
//
// extern int initialize_foobar_device(int, int, int) __init;
//
// For initialized data:
// You should insert __initdata or __initconst between the variable name
// and equal sign followed by value, e.g.:
//
// static int init_variable __initdata = 0;
// static const char linux_logo[] __initconst = { 0x32, 0x36, ... };
//
// Don't forget to initialize data not at file scope, i.e. within a function,
// as gcc otherwise puts the data into the bss section and not into the init
// section.
//
// These are for everybody (although not all archs will actually

//
// modpost check for section mismatches during the kernel build.
// A section mismatch happens when there are references from a
// code or data section to an init section (both code or data).
// The init sections are (for most archs) discarded by the kernel
// when early init has completed so all such references are potential bugs.
// For exit sections the same issue exists.
//
// The following markers are used for the cases where the reference to
// the *init / *exit section (code or data) is valid and will teach
// modpost not to issue a warning.  Intended semantics is that a code or
// data tagged __ref* can reference code or data from init section without
// producing a warning (of course, no warning does not mean code is
// correct, so optimally document why the __ref is needed and why it's OK).
//
// The markers follow same syntax rules as __init / __initdata.
//

// Macro flag: #define __exitused

// Macro flag: #define __meminit
// Macro flag: #define __meminitdata
// Macro flag: #define __meminitconst

// For assembly routines

// silence warnings when references are OK

//
// Used for initialization calls..
//
extern "C" {
    pub fn int(_arg: *mut initcall_t)(void) -> typedef;
}
extern "C" {
    pub fn void(_arg: *mut exitcall_t)(void) -> typedef;
}

pub type initcall_entry_t = c_int;
extern "C" {
    pub fn offset_to_ptr(_arg: entry) -> return;
}

pub type initcall_entry_t = initcall_t;

// Used for constructor calls.
extern "C" {
    pub fn void(_arg: *mut ctor_fn_t)(void) -> typedef;
}
// Defined in init/main.c
extern "C" {
    pub fn do_one_initcall(fn: initcall_t) -> c_int;
}
// used by init/main.c
extern "C" {
    pub fn setup_arch(: *mut c_char);
}
extern "C" {
    pub fn prepare_namespace();
}
extern "C" {
    pub fn init_rootfs() -> void __init;
}
extern "C" {
    pub fn init_IRQ();
}
extern "C" {
    pub fn time_init();
}
extern "C" {
    pub fn poking_init();
}
extern "C" {
    pub fn pgtable_cache_init();
}
extern "C" {
    pub fn mark_rodata_ro();
}
extern "C" {
    pub fn void(_arg: *mut late_time_init)(void) -> extern;
}

//
// initcalls are now grouped by functionality into separate
// subsections. Ordering inside the subsections is determined
// by link order.
// For backwards compatibility, initcall() puts the call in
// the device init subsection.
//
// The `id' arg to __define_initcall() is needed so that multiple initcalls
// can point at the same handler without causing duplicate-symbol build errors.
//
// Initcalls are run by placing pointers in initcall sections that the
// kernel iterates at runtime. The linker can do dead code / data elimination
// and remove that completely, so the initcall sections have to be marked
// as KEEP() in the linker script.
//
// Format: <modname>__<counter>_<line>_<fn>

// Format: __<prefix>__<iid><id>

//
// With LTO, the compiler doesn't necessarily obey link order for
// initcalls. In order to preserve the correct order, we add each
// variable into its own section and generate a linker script (in
// scripts/link-vmlinux.sh) to specify the order of the sections.
//

//
// With LTO, the compiler can rename static functions to avoid
// global naming collisions. We use a global stub function for
// initcalls to create a stable symbol name whose address can be
// taken in inline assembly when PREL32 relocations are used.
//

//
// Early initcalls run before initializing SMP.
//
// Only for built-in code, not modules.
//

//
// A "pure" initcall has no dependencies on anything else, and purely
// initializes variables that couldn't be statically initialized.
//
// This only exists for built-in code, not for modules.
// Keep main.c:initcall_level_names[] in sync.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct obs_kernel_param {
    pub str: *const c_char,
    pub ): *mut *mut int (setup_func)(char,
    pub early: c_int,
}

//
// Only for really core code.  See moduleparam.h for the normal way.
//
// Force the alignment so the compiler doesn't space elements of the
// obs_kernel_param "array" too far apart in .init.setup.
//

//
// NOTE: __setup functions return values:
// @fn returns 1 (or non-zero) if the option argument is "handled"
// and returns 0 if the option argument is "not handled".
//

//
// NOTE: @fn is as per module_param, not __setup!
// I.e., @fn returns 0 for no error or non-zero for error
// (possibly @fn returns a -errno value, but it does not matter).
// Emits warning if @fn returns non-zero.
//

// Relies on boot_command_line being set
extern "C" {
    pub fn parse_early_param() -> void __init;
}
extern "C" {
    pub fn parse_early_options(cmdline: *mut c_char) -> void __init;
}

// Data marked not to be saved by software suspend

