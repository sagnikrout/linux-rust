//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/module.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Dynamic loading of modules into the kernel.
//
// Rewritten by Richard Henderson <rth@tamu.edu> Dec 1996
// Rewritten again by Rusty Russell, 2002
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct modversion_info {
    pub crc: c_ulong,
    pub name: [c_char; MODULE_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_kobject {
    pub kobj: kobject,
    pub mod: *mut module,
    pub drivers_dir: *mut kobject,
    pub mp: *mut module_param_attrs,
    pub kobj_completion: *mut completion,
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_attribute {
    pub attr: attribute,
    pub ): *mut c_char,
    pub count): *const *const char , size_t,
    pub ): *const *const *const void (setup)(struct module , char,
    pub ): *mut *mut int (test)(struct module,
    pub ): *mut *mut void (free)(struct module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_version_attribute {
    pub mattr: module_attribute,
    pub module_name: *const c_char,
    pub version: *const c_char,
}

// These are either module local, or the kernel's dummy ones.
extern "C" {
    pub fn init_module() -> c_int;
}
extern "C" {
    pub fn cleanup_module();
}

//
// module_init() - driver initialization entry point
// @x: function to be run at kernel boot time or module insertion
//
// module_init() will either be called during do_initcalls() (if
// builtin) or at module insertion time (if a module).  There can only
// be one per module.
//

//
// module_exit() - driver exit entry point
// @x: function to be run when driver is removed
//
// module_exit() will wrap the driver clean-up code
// with cleanup_module() when used with rmmod when
// the driver is a module.  If the driver is statically
// compiled into the kernel, module_exit() has no effect.
// There can only be one per module.
//

//
// In most cases loadable modules do not need custom
// initcall levels. There are still some valid cases where
// a driver may be needed early if built in, and does not
// matter when built as a loadable module. Like bus
// snooping debug drivers.
//

// Each module must use one module_init().

// This is only required if you want to be unloadable.

// This means "can be init if no module support, otherwise module load

// Macro flag: #define __init_or_module
// Macro flag: #define __initdata_or_module
// Macro flag: #define __initconst_or_module

// For userspace: you can also call me...

// Soft module dependencies. See man modprobe.d for details.
// Example: MODULE_SOFTDEP("pre: module-foo module-bar post: module-baz")
//

//
// Weak module dependencies. See man modprobe.d for details.
// Example: MODULE_WEAKDEP("module-foo")
//

//
// MODULE_FILE is used for generating modules.builtin
// So, make it no-op when this is being built as a module
//

// Macro flag: #define MODULE_FILE

//
// The following license idents are currently accepted as indicating free
// software modules
//
// "GPL"				[GNU Public License v2]
// "GPL v2"			[GNU Public License v2]
// "GPL and additional rights"	[GNU Public License v2 rights and more]
// "Dual BSD/GPL"			[GNU Public License v2
// or BSD license choice]
// "Dual MIT/GPL"			[GNU Public License v2
// or MIT license choice]
// "Dual MPL/GPL"			[GNU Public License v2
// or Mozilla license choice]
//
// The following other idents are available
//
// "Proprietary"			[Non free products]
//
// Both "GPL v2" and "GPL" (the latter also in dual licensed strings) are
// merely stating that the module is licensed under the GPL v2, but are not
// telling whether "GPL v2 only" or "GPL v2 or later". The reason why there
// are two variants is a historic and failed attempt to convey more
// information in the MODULE_LICENSE string. For module loading the
// "only/or later" distinction is completely irrelevant and does neither
// replace the proper license identifiers in the corresponding source file
// nor amends them in any way. The sole purpose is to make the
// 'Proprietary' flagging work and to refuse to bind symbols which are
// exported with EXPORT_SYMBOL_GPL when a non free module is loaded.
//
// In the same way "BSD" is not a clear license information. It merely
// states, that the module is licensed under one of the compatible BSD
// license variants. The detailed and correct license information is again
// to be found in the corresponding source files.
//
// There are dual licensed components, but when running with Linux it is the
// GPL that is relevant so this is a non issue. Similarly LGPL linked with GPL
// is a GPL combined work.
//
// This exists for several reasons
// 1.	So modinfo can show license info for users wanting to vet their setup
// is free
// 2.	So the community can ignore bug reports including proprietary modules
// 3.	So vendors can do likewise based on their own policies
//

//
// Author(s), use "Name <email>" or just "Name", for multiple
// authors use multiple MODULE_AUTHOR() statements/lines.
//

// What your module does.

//
// Format: __mod_device_table__kmod_<modname>__<type>__<name>
// Parts of the string `__kmod_` and `__` are used as delimiters when parsing
// a symbol in file2alias.c
//

// Creates an alias so file2alias.c can find device table.

// Version of form [<epoch>:]<version>[-<extra-version>].
// Or for CVS/RCS ID version, everything but the number is stripped.
// <epoch>: A (small) unsigned integer which allows you to start versions
// anew. If not mentioned, it's zero.  eg. "2:1.0" is after
// "1:2.0".
// <version>: The <version> may contain only alphanumerics and the
// character `.'.  Ordered by numeric sort for numeric parts,
// ascii sort for ascii parts (as per RPM or DEB algorithm).
// <extraversion>: Like <version>, but inserted for local
// customizations, eg "rh3" or "rusty1".
// Using this automatically adds a checksum of the .c files and the
// local headers in "srcversion".
//

// Optional firmware file (or files) needed by the module
// format is simply firmware file name.  Multiple firmware
// files require multiple MODULE_FIRMWARE() specifiers

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum module_state {
    MODULE_STATE_LIVE,	/* Normal state. */
    MODULE_STATE_COMING,	/* Full formed, running module_init. */
    MODULE_STATE_GOING,	/* Going away. */
    MODULE_STATE_UNFORMED,	/* Still setting it up. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_tree_node {
    pub mod: *mut module,
    pub node: latch_tree_node,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_mem_type {
    MOD_TEXT = 0,
    MOD_DATA,
    MOD_RODATA,
    MOD_RO_AFTER_INIT,
    MOD_INIT_TEXT,
    MOD_INIT_DATA,
    MOD_INIT_RODATA,

    MOD_MEM_NUM_TYPES,
    MOD_INVALID = -1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_memory {
    pub base: *mut c_void,
    pub is_rox: bool,
    pub size: c_uint,

    pub mtn: mod_tree_node,

}

// Only touch one cacheline for common rbtree-for-core-layout case.

// Macro flag: #define __module_memory_align

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_kallsyms {
    pub symtab: *mut Elf_Sym,
    pub num_symtab: c_uint,
    pub strtab: *mut c_char,
    pub typetab: *mut c_char,
}

//
// struct klp_modinfo - ELF information preserved from the livepatch module
//
// @hdr: ELF header
// @sechdrs: Section header table
// @secstrings: String table for the section headers
// @symndx: The symbol table section index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klp_modinfo {
    pub hdr: Elf_Ehdr,
    pub sechdrs: *mut Elf_Shdr,
    pub secstrings: *mut c_char,
    pub symndx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module {
    pub state: module_state,
// Member of list of modules
    pub list: list_head,
// Unique handle for this module
    pub name: [c_char; MODULE_NAME_LEN],
// Module build ID
    pub build_id: [c_uchar; BUILD_ID_SIZE_MAX],
// Sysfs stuff.
    pub mkobj: module_kobject,
    pub modinfo_attrs: *mut module_attribute,
    pub version: *const c_char,
    pub srcversion: *const c_char,
    pub imported_namespaces: *const c_char,
    pub holders_dir: *mut kobject,
// Exported symbols
    pub syms: *const kernel_symbol,
    pub crcs: *const u32,
    pub flagstab: *const u8,
    pub num_syms: c_uint,

    pub kcfi_traps: *mut i32,
    pub kcfi_traps_end: *mut i32,

// Kernel parameters.

    pub param_lock: mutex,

    pub kp: *mut kernel_param,
    pub num_kp: c_uint,
// GPL-only exported symbols.
    pub using_gplonly_symbols: bool,

// Signature was verified.
    pub sig_ok: bool,

    pub async_probe_requested: bool,
// Exception table
    pub num_exentries: c_uint,
    pub extable: *mut exception_table_entry,
// Startup function.
    pub (*init)(void): *mut c_int,
    pub __module_memory_align: module_memory mem[MOD_MEM_NUM_TYPES],
// Arch-specific module values
    pub arch: mod_arch_specific,
    pub /: *mut *mut unsigned long taints; / same bits as kernel:taint_flags,

// Support for BUG
    pub num_bugs: unsigned,
    pub bug_list: list_head,
    pub bug_table: *mut bug_entry,

// Protected by RCU and/or module_mutex: use rcu_dereference()
    pub kallsyms: *mut mod_kallsyms __rcu,
    pub core_kallsyms: mod_kallsyms,
// Section attributes
    pub sect_attrs: *mut module_sect_attrs,
// Notes attributes
    pub notes_attrs: *mut module_notes_attrs,

// Per-cpu data.
    pub percpu: *mut void __percpu,
    pub percpu_size: c_uint,

    pub noinstr_text_start: *mut c_void,
    pub noinstr_text_size: c_uint,

    pub num_tracepoints: c_uint,
    pub tracepoints_ptrs: *mut tracepoint_ptr_t,

    pub num_srcu_structs: c_uint,
    pub srcu_struct_ptrs: *mut srcu_struct,

    pub num_bpf_raw_events: c_uint,
    pub bpf_raw_events: *mut bpf_raw_event_map,

    pub btf_data_size: c_uint,
    pub btf_base_data_size: c_uint,
    pub btf_data: *mut c_void,
    pub btf_base_data: *mut c_void,

    pub jump_entries: *mut jump_entry,
    pub num_jump_entries: c_uint,

    pub num_trace_bprintk_fmt: c_uint,
    pub trace_bprintk_fmt_start: *const c_char,

    pub trace_events: *mut trace_event_call,
    pub num_trace_events: c_uint,
    pub trace_evals: *mut trace_eval_map,
    pub num_trace_evals: c_uint,

    pub num_ftrace_callsites: c_uint,
    pub ftrace_callsites: *mut c_ulong,

    pub kprobes_text_start: *mut c_void,
    pub kprobes_text_size: c_uint,
    pub kprobe_blacklist: *mut c_ulong,
    pub num_kprobe_blacklist: c_uint,

    pub num_static_call_sites: c_int,
    pub static_call_sites: *mut static_call_site,

    pub num_kunit_init_suites: c_int,
    pub kunit_init_suites: *mut kunit_suite,
    pub num_kunit_suites: c_int,
    pub kunit_suites: *mut kunit_suite,

    pub /: *mut *mut bool klp; / Is this a livepatch module?,
    pub klp_alive: bool,
// ELF information
    pub klp_info: *mut klp_modinfo,

    pub printk_index_size: c_uint,
    pub printk_index_start: *mut pi_entry,

// What modules depend on me?
    pub source_list: list_head,
// What modules do I depend on?
    pub target_list: list_head,
// Destruction function.
    pub (*exit)(void): *mut c_void,
    pub refcnt: core::sync::atomic::AtomicI32,

// Constructor functions.
    pub ctors: *mut ctor_fn_t,
    pub num_ctors: c_uint,

    pub ei_funcs: *mut error_injection_entry,
    pub num_ei_funcs: c_uint,

    pub dyndbg_info: _ddebug_info,

    pub __randomize_layout: } ____cacheline_aligned,

// Get/put a kernel symbol (calls must be symmetric)
    pub symbol): *const *const void __symbol_get(char,
    pub symbol): *const *const void __symbol_get_gpl(char,

    pub \: __used __section(".no_trim_symbol") = __stringify(x);,
    pub }): (typeof(&x))(__symbol_get(__stringify(x)));,

    pub sym->st_value: return,

// FIXME: It'd be nice to isolate modules during init, too, so they
    pub MODULE_STATE_GOING: return mod->state !=,
    pub MODULE_STATE_COMING: return mod->state ==,
    pub addr): *mut *mut module __module_text_address(unsigned long,
    pub addr): *mut *mut module __module_address(unsigned long,
    pub addr): bool is_module_address(unsigned long,
    pub can_addr): *mut bool __is_module_percpu_address(unsigned long addr, unsigned long,
    pub addr): bool is_module_percpu_address(unsigned long,
    pub addr): bool is_module_text_address(unsigned long,
    pub size: unsigned long base,,
    pub long)mod->mem[type].base: base = (unsigned,
    pub mod->mem[type].size: size =,
    pub size: return addr - base <,
    pub true: return,
    pub false: return,
    pub true: return,
    pub false: return,
    pub mod): return within_module_init(addr, mod) || within_module_core(addr,,
// Search for module by name: must be in a RCU critical section.
    pub name): *const *const module find_module(char,
    pub code): c_long,

    pub mod): *mut int module_refcount(struct module,
    pub symbol): *const void __symbol_put(char,

    pub addr): *mut void symbol_put_addr(void,
// Sometimes we know we already have a refcount, and it's easier not
    pub module): *mut extern void __module_get(struct module,
//
// try_module_get() - take module refcount unless module is being removed
// @module: the module we should check for
//
// Only try to get a module reference count if the module is not being removed.
// This call will fail if the module is in the process of being removed.
//
// Care must also be taken to ensure the module exists and is alive prior to
// usage of this call. This can be gauranteed through two means:
//
// 1) Direct protection: you know an earlier caller must have increased the
// module reference through __module_get(). This can typically be achieved
// by having another entity other than the module itself increment the
// module reference count.
//
// 2) Implied protection: there is an implied protection against module
// removal. An example of this is the implied protection used by kernfs
// sysfs. The sysfs store / read file operations are guaranteed to exist
// through the use of kernfs's active reference (see kernfs_active()) and a
// sysfs / kernfs file removal cannot happen unless the same file is not
// active. Therefore, if a sysfs file is being read or written to the module
// which created it must still exist. It is therefore safe to use
// try_module_get() on module sysfs store / read ops.
//
// One of the real values to try_module_get() is the module_is_live() check
// which ensures that the caller of try_module_get() can yield to userspace
// module removal requests and gracefully fail if the module is on its way out.
//
// Returns true if the reference count was successfully incremented.
//
    pub module): *mut extern bool try_module_get(struct module,
//
// module_put() - release a reference count to a module
// @module: the module we should release a reference count for
//
// If you successfully bump a reference count to a module with try_module_get(),
// when you are finished you must call module_put() to release that reference
// count.
//
    pub module): *mut extern void module_put(struct module,

    pub module_is_live(module): return !module ||,

// This is a #define so the string doesn't get put in every .o file

    pub \: *mut *mut module __mod = (mod);,
    pub \: __mod ? __mod->name : "kernel";,

    pub mod->build_id: return,

    pub NULL: return,

// Dereference module function descriptor
    pub ptr): *mut *mut *mut void dereference_module_function_descriptor(struct module mod, void,
    pub nb): *mut int register_module_notifier(struct notifier_block,
    pub nb): *mut int unregister_module_notifier(struct notifier_block,
    pub print_modules(void): extern void,
    pub module->async_probe_requested: return module &&,

    pub mod->klp: return,

    pub false: return,

    pub data): *mut *mut *mut *mut void module_for_each_mod(int(func)(struct module mod, void data), void,

    pub NULL: return,
    pub NULL: return,
    pub false: return,
    pub false: return,
    pub false: return,
    pub false: return,
    pub false: return,
    pub false: return,
    pub false: return,
// Get/put a kernel symbol (calls should be symmetric)

    pub true: return,

// no events will happen anyway, so this can always succeed
    pub 0: return,
    pub 0: return,

    pub false: return,
// Dereference module function descriptor
    pub ptr: return,
    pub false: return,

    pub module_kset: *mut extern struct kset,
    pub module_ktype: extern struct kobj_type,

// BELOW HERE ALL THESE ARE OBSOLETE AND WILL VANISH

    pub ): *mut module,
    pub ): *mut void module_bug_cleanup(struct module,

    pub has_retpoline): extern bool retpoline_module_ok(bool,

    pub true: return,

    pub is_module_sig_enforced(void): bool,
    pub set_module_sig_enforced(void): c_void,
    pub module->sig_ok: return,

    pub false: return,
    pub true: return,

    pub data): *mut c_void,
// For kallsyms to ask for address resolution.  namebuf should be at
// least KSYM_NAME_LEN long: a pointer to namebuf is returned if
// found, otherwise NULL.
//
    pub namebuf): *mut c_char,
    pub symname): *mut int lookup_module_symbol_name(unsigned long addr, char,
    pub name): *mut c_char,
// Returns 0 and fills in value, defined and namebuf, or -ERANGE if
// symnum out of range.
//
    pub exported): *mut *mut *mut char name, char module_name, int,
// Look for this name: can be of form module:name.
    pub name): *const unsigned long module_kallsyms_lookup_name(char,
    pub name): *const *const unsigned long find_kallsyms_symbol_value(struct module mod, char,

    pub -EOPNOTSUPP: return,
// For kallsyms to ask for address resolution.  NULL means not found.
    pub 0: return,
    pub -ERANGE: return,
    pub -ERANGE: return,
    pub 0: return,
    pub 0: return,

// Define __free(module_put) macro for struct module *.
