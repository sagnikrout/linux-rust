//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/moduleparam.h
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
// (C) Copyright 2001, 2002 Rusty Russell IBM Corporation

//
// The maximum module name length, including the NUL byte.
// Chosen so that structs with an unsigned long line up, specifically
// modversion_info.
//

// You can override this manually, but generally this should match the

// We cannot use MODULE_PARAM_PREFIX because some modules override it.

// Generic info of form tag = "info"

// One for each parameter, describing how to use it.  Some files do

//
// Flags available for kernel_param_ops
//
// NOARG - the parameter allows for no argument (foo instead of foo=1)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_param_ops {
// How the ops should behave
    pub flags: c_uint,
// Returns 0, or -errno.  arg is in kp->arg.
    pub kp): *const *const *const int (set)(char val, struct kernel_param,
// Returns length written or -errno.  Buffer is 4k (ie. be short!)
    pub kp): *const *const *const int (get)(char buffer, struct kernel_param,
// Optional function to free kp->arg when module unloaded.
    pub arg): *mut *mut void (free)(void,
}

//
// Flags available for kernel_param
//
// UNSAFE - the parameter is dangerous and setting it will taint the kernel
// HWPARAM - Hardware param not permitted in lockdown mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_param {
    pub name: *const c_char,
    pub mod: *mut module,
    pub ops: *const kernel_param_ops,
    pub perm: u16,
    pub level: i8,
    pub flags: u8,
    pub arg: *mut c_void,
    pub str: *const kparam_string,
    pub arr: *const kparam_array,
}

// Special one for strings we want to copy into
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kparam_string {
    pub maxlen: c_uint,
    pub string: *mut c_char,
}

// Special one for arrays
//
// module_param - typesafe helper for a module/cmdline parameter
// @name: the variable to alter, and exposed parameter name.
// @type: the type of the parameter
// @perm: visibility in sysfs.
//
// @name becomes the module parameter, or (prefixed by KBUILD_MODNAME and a
// ".") the kernel commandline parameter.  Note that - is changed to _, so
// the user can use "foo-bar=1" even for variable "foo_bar".
//
// @perm is 0 if the variable is not to appear in sysfs, or 0444
// for world-readable, 0644 for root-writable, etc.  Note that if it
// is writable, you may need to use kernel_param_lock() around
// accesses (esp. charp, which can be kfreed when it changes).
//
// The @type is simply pasted to refer to a param_ops_##type and a
// param_check_##type: for convenience many standard types are provided but
// you can create your own by defining those variables.
//
// Standard types are:
// byte, hexint, short, ushort, int, uint, long, ulong
// charp: a character pointer
// bool: a bool, values 0/1, y/n, Y/N.
// invbool: the above, only sense-reversed (N = true).
//

//
// module_param_unsafe - same as module_param but taints kernel
// @name: the variable to alter, and exposed parameter name.
// @type: the type of the parameter
// @perm: visibility in sysfs.
//

//
// module_param_named - typesafe helper for a renamed module/cmdline parameter
// @name: a valid C identifier which is the parameter name.
// @value: the actual lvalue to alter.
// @type: the type of the parameter
// @perm: visibility in sysfs.
//
// Usually it's a good idea to have variable names and user-exposed names the
// same, but that's harder if the variable must be non-static or is inside a
// structure.  This allows exposure under a different name.
//

//
// module_param_named_unsafe - same as module_param_named but taints kernel
// @name: a valid C identifier which is the parameter name.
// @value: the actual lvalue to alter.
// @type: the type of the parameter
// @perm: visibility in sysfs.
//

//
// module_param_cb - general callback for a module/cmdline parameter
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

//
// core_param_cb - general callback for a module/cmdline parameter
// to be evaluated before core initcall level
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

//
// postcore_param_cb - general callback for a module/cmdline parameter
// to be evaluated before postcore initcall level
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

//
// arch_param_cb - general callback for a module/cmdline parameter
// to be evaluated before arch initcall level
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

//
// subsys_param_cb - general callback for a module/cmdline parameter
// to be evaluated before subsys initcall level
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

//
// fs_param_cb - general callback for a module/cmdline parameter
// to be evaluated before fs initcall level
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

//
// device_param_cb - general callback for a module/cmdline parameter
// to be evaluated before device initcall level
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

//
// late_param_cb - general callback for a module/cmdline parameter
// to be evaluated before late initcall level
// @name: a valid C identifier which is the parameter name.
// @ops: the set & get operations for this parameter.
// @arg: args for @ops
// @perm: visibility in sysfs.
//
// The ops can have NULL set or get functions.
//

// On alpha, ia64 and ppc64 relocations to global data cannot go into

// Macro flag: #define __moduleparam_const

// This is the fundamental function for registering boot/module parameters.

//
// Useful for describing a set/get pair used only once (i.e. for this
// parameter). For repeated set/get pairs (i.e. the same struct
// kernel_param_ops), use module_param_cb() instead.
//

extern "C" {
    pub fn kernel_param_lock(mod: *mut module);
}
extern "C" {
    pub fn kernel_param_unlock(mod: *mut module);
}

//
// core_param - define a historical core kernel parameter.
// @name: the name of the cmdline and sysfs parameter (often the same as var)
// @var: the variable
// @type: the type of the parameter
// @perm: visibility in sysfs
//
// core_param is just like module_param(), but cannot be modular and
// doesn't add a prefix (such as "printk.").  This is for compatibility
// with __setup(), and it makes sense as truly core parameters aren't
// tied to the particular file they're in.
//

//
// core_param_unsafe - same as core_param but taints kernel
// @name: the name of the cmdline and sysfs parameter (often the same as var)
// @var: the variable
// @type: the type of the parameter
// @perm: visibility in sysfs
//

//
// __core_param_cb - similar like core_param, with a set/get ops instead of type.
// @name: the name of the cmdline and sysfs parameter (often the same as var)
// @ops: the set & get operations for this parameter.
// @arg: the variable
// @perm: visibility in sysfs
//
// Ideally this should be called 'core_param_cb', but the name has been
// used for module core parameter, so add the '__' prefix
//

//
// module_param_string - a char array parameter
// @name: the name of the parameter
// @string: the string variable
// @len: the maximum length of the string, incl. terminator
// @perm: visibility in sysfs.
//
// This actually copies the string when it's set (unlike type charp).
// @len is usually just sizeof(string).
//

//
// parameq - checks if two parameter names match
// @name1: parameter name 1
// @name2: parameter name 2
//
// Returns: true if the two parameter names are equal.
// Dashes (-) are considered equal to underscores (_).
//
extern "C" {
    pub fn parameq(name1: *const c_char, name2: *const c_char) -> bool;
}
//
// parameqn - checks if two parameter names match
// @name1: parameter name 1
// @name2: parameter name 2
// @n: the length to compare
//
// Similar to parameq(), except it compares @n characters.
//
// Returns: true if the first @n characters of the two parameter names
// are equal.
// Dashes (-) are considered equal to underscores (_).
//
extern "C" {
    pub fn parameqn(name1: *const c_char, name2: *const c_char, n: usize) -> bool;
}
extern "C" {
    pub fn int(param: *mut *mut parse_unknown_fn)(char, val: *mut c_char, doing: *const c_char, arg: *mut c_void) -> typedef;
}
// Called on module insert or kernel boot
// Called by module remove.

extern "C" {
    pub fn module_destroy_params(params: *const kernel_param, num: c_uint);
}

// All the helper functions
// The macros to do compile-time type checking stolen from Jakub

extern "C" {
    pub fn param_set_byte(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_byte(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_short(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_short(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_ushort(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_ushort(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_int(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_int(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_uint(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_uint(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_long(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_long(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_ulong(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_ulong(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_ullong(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_ullong(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_hexint(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_hexint(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_charp(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_charp(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_free_charp(arg: *mut c_void);
}

// We used to allow int as well as bool.  We're taking that away!
extern "C" {
    pub fn param_set_bool(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_bool(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

extern "C" {
    pub fn param_set_bool_enable_only(val: *const c_char, kp: *const kernel_param) -> c_int;
}
// getter is the same as for the regular bool

extern "C" {
    pub fn param_set_invbool(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_invbool(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

// An int, which can only be set like a bool (though it shows as an int).
extern "C" {
    pub fn param_set_bint(val: *const c_char, kp: *const kernel_param) -> c_int;
}

//
// module_param_array - a parameter which is an array of some type
// @name: the name of the array variable
// @type: the type, as per module_param()
// @nump: optional pointer filled in with the number written
// @perm: visibility in sysfs
//
// Input and output are as comma-separated values.  Commas inside values
// don't work properly (eg. an array of charp).
//
// ARRAY_SIZE(@name) is used to determine the number of elements in the
// array, so the definition must be visible.
//

//
// module_param_array_named - renamed parameter which is an array of some type
// @name: a valid C identifier which is the parameter name
// @array: the name of the array variable
// @type: the type, as per module_param()
// @nump: optional pointer filled in with the number written
// @perm: visibility in sysfs
//
// This exposes a different name than the actual variable name.  See
// module_param_named() for why this might be necessary.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwparam_type {
    hwparam_ioport,		/* Module parameter configures an I/O port */
    hwparam_iomem,		/* Module parameter configures an I/O mem address */
    hwparam_ioport_or_iomem, /* Module parameter could be either, depending on other option */
    hwparam_irq,		/* Module parameter configures an IRQ */
    hwparam_dma,		/* Module parameter configures a DMA channel */
    hwparam_dma_addr,	/* Module parameter configures a DMA buffer address */
    hwparam_other,		/* Module parameter configures some other value */
}

//
// module_param_hw_named - A parameter representing a hw parameters
// @name: a valid C identifier which is the parameter name.
// @value: the actual lvalue to alter.
// @type: the type of the parameter
// @hwtype: what the value represents (enum hwparam_type)
// @perm: visibility in sysfs.
//
// Usually it's a good idea to have variable names and user-exposed names the
// same, but that's harder if the variable must be non-static or is inside a
// structure.  This allows exposure under a different name.
//

//
// module_param_hw_array - A parameter representing an array of hw parameters
// @name: the name of the array variable
// @type: the type, as per module_param()
// @hwtype: what the value represents (enum hwparam_type)
// @nump: optional pointer filled in with the number written
// @perm: visibility in sysfs
//
// Input and output are as comma-separated values.  Commas inside values
// don't work properly (eg. an array of charp).
//
// ARRAY_SIZE(@name) is used to determine the number of elements in the
// array, so the definition must be visible.
//

extern "C" {
    pub fn param_set_copystring(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn param_get_string(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}
// for exporting parameters in /sys/module/.../parameters

extern "C" {
    pub fn module_param_sysfs_remove(mod: *mut module);
}

