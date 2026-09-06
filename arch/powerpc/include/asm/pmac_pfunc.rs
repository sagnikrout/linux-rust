//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pmac_pfunc.h
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

// Flags in command lists
pub const PMF_FLAGS_ON_INIT: c_uint = 0x80000000u;
pub const PMF_FLGAS_ON_TERM: c_uint = 0x40000000u;
pub const PMF_FLAGS_ON_SLEEP: c_uint = 0x20000000u;
pub const PMF_FLAGS_ON_WAKE: c_uint = 0x10000000u;
pub const PMF_FLAGS_ON_DEMAND: c_uint = 0x08000000u;
pub const PMF_FLAGS_INT_GEN: c_uint = 0x04000000u;
pub const PMF_FLAGS_HIGH_SPEED: c_uint = 0x02000000u;
pub const PMF_FLAGS_LOW_SPEED: c_uint = 0x01000000u;
pub const PMF_FLAGS_SIDE_EFFECTS: c_uint = 0x00800000u;
//
// Arguments to a platform function call.
//
// NOTE: By convention, pointer arguments point to an u32
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_args {
    pub v: u32,
    pub p: *mut u32,
    pub u: [}; 4],
    pub count: c_uint,
}

//
// A driver capable of interpreting commands provides a handlers
// structure filled with whatever handlers are implemented by this
// driver. Non implemented handlers are left NULL.
//
// PMF_STD_ARGS are the same arguments that are passed to the parser
// and that gets passed back to the various handlers.
//
// Interpreting a given function always start with a begin() call which
// returns an instance data to be passed around subsequent calls, and
// ends with an end() call. This allows the low level driver to implement
// locking policy or per-function instance data.
//
// For interrupt capable functions, irq_enable() is called when a client
// registers, and irq_disable() is called when the last client unregisters
// Note that irq_enable & irq_disable are called within a semaphore held
// by the core, thus you should not try to register yourself to some other
// pmf interrupt during those calls.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_handlers {
    pub args): *mut *mut *mut *mut void  (begin)(struct pmf_function func, struct pmf_args,
    pub instdata): *mut *mut *mut void (end)(struct pmf_function func, void,
    pub func): *mut *mut int (irq_enable)(struct pmf_function,
    pub func): *mut *mut int (irq_disable)(struct pmf_function,
    pub mask): *mut *mut int (write_gpio)(PMF_STD_ARGS, u8 value, u8,
    pub xor): *mut *mut int (read_gpio)(PMF_STD_ARGS, u8 mask, int rshift, u8,
    pub mask): *mut *mut int (write_reg32)(PMF_STD_ARGS, u32 offset, u32 value, u32,
    pub offset): *mut *mut int (read_reg32)(PMF_STD_ARGS, u32,
    pub mask): *mut *mut int (write_reg16)(PMF_STD_ARGS, u32 offset, u16 value, u16,
    pub offset): *mut *mut int (read_reg16)(PMF_STD_ARGS, u32,
    pub mask): *mut *mut int (write_reg8)(PMF_STD_ARGS, u32 offset, u8 value, u8,
    pub offset): *mut *mut int (read_reg8)(PMF_STD_ARGS, u32,
    pub duration): *mut *mut int (delay)(PMF_STD_ARGS, u32,
    pub mask): *mut *mut int (wait_reg32)(PMF_STD_ARGS, u32 offset, u32 value, u32,
    pub mask): *mut *mut int (wait_reg16)(PMF_STD_ARGS, u32 offset, u16 value, u16,
    pub mask): *mut *mut int (wait_reg8)(PMF_STD_ARGS, u32 offset, u8 value, u8,
    pub len): *mut *mut int (read_i2c)(PMF_STD_ARGS, u32,
    pub data): *const *const int (write_i2c)(PMF_STD_ARGS, u32 len, u8,
    pub valuedata): *const *const u8 maskdata, u8,
    pub len): *mut *mut int (read_cfg)(PMF_STD_ARGS, u32 offset, u32,
    pub data): *const *const int (write_cfg)(PMF_STD_ARGS, u32 offset, u32 len, u8,
    pub valuedata): *const *const u32 totallen, u8 maskdata, u8,
    pub len): *mut *mut int (read_i2c_sub)(PMF_STD_ARGS, u8 subaddr, u32,
    pub data): *const *const int (write_i2c_sub)(PMF_STD_ARGS, u8 subaddr, u32 len, u8,
    pub mode): *mut *mut int (set_i2c_mode)(PMF_STD_ARGS, int,
    pub valuedata): *const u8,
    pub xor): u32,
    pub xor): u32,
    pub xor): u32,
    pub mask): *mut *mut int (write_reg32_slm)(PMF_STD_ARGS, u32 offset, u32 shift, u32,
    pub mask): *mut *mut int (write_reg16_slm)(PMF_STD_ARGS, u32 offset, u32 shift, u32,
    pub mask): *mut *mut int (write_reg8_slm)(PMF_STD_ARGS, u32 offset, u32 shift, u32,
    pub valuedata): *const u8,
    pub owner: *mut module,
}

//
// Drivers who expose platform functions register at init time, this
// causes the platform functions for that device node to be parsed in
// advance and associated with the device. The data structures are
// partially public so a driver can walk the list of platform functions
// and eventually inspect the flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_function {
// All functions for a given driver are linked
    pub link: list_head,
// Function node & driver data
    pub node: *mut device_node,
    pub driver_data: *mut c_void,
// For internal use by core
    pub dev: *mut pmf_device,
// The name is the "xxx" in "platform-do-xxx", this is how
// platform functions are identified by this code. Some functions
// only operate for a given target, in which case the phandle is
// here (or 0 if the filter doesn't apply)
//
    pub name: *const c_char,
    pub phandle: u32,
// The flags for that function. You can have several functions
// with the same name and different flag
//
    pub flags: u32,
// The actual tokenized function blob
    pub data: *const c_void,
    pub length: c_uint,
// Interrupt clients
    pub irq_clients: list_head,
// Refcounting
    pub ref: kref,
}

//
// For platform functions that are interrupts, one can register
// irq_client structures. You canNOT use the same structure twice
// as it contains a link member. Also, the callback is called with
// a spinlock held, you must not call back into any of the pmf_* functions
// from within that callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_irq_client {
    pub data): *mut *mut void (handler)(void,
    pub data: *mut c_void,
    pub owner: *mut module,
    pub link: list_head,
    pub func: *mut pmf_function,
}

//
// Register/Unregister a function-capable driver and its handlers
//
extern "C" {
    pub fn pmf_unregister_driver(np: *mut device_node);
}
//
// Register/Unregister interrupt clients
//
extern "C" {
    pub fn pmf_unregister_irq_client(client: *mut pmf_irq_client);
}
//
// Called by the handlers when an irq happens
//
extern "C" {
    pub fn pmf_do_irq(func: *mut pmf_function);
}
//
// Low level call to platform functions.
//
// The phandle can filter on the target object for functions that have
// multiple targets, the flags allow you to restrict the call to a given
// combination of flags.
//
// The args array contains as many arguments as is required by the function,
// this is dependent on the function you are calling, unfortunately Apple
// mechanism provides no way to encode that so you have to get it right at
// the call site. Some functions require no args, in which case, you can
// pass NULL.
//
// You can also pass NULL to the name. This will match any function that has
// the appropriate combination of flags & phandle or you can pass 0 to the
// phandle to match any
//
// High level call to a platform function.
//
// This one looks for the platform-xxx first so you should call it to the
// actual target if any. It will fallback to platform-do-xxx if it can't
// find one. It will also exclusively target functions that have
// the "OnDemand" flag.
//
// For low latency interrupt usage, you can lookup for on-demand functions
// using the functions below
//
extern "C" {
    pub fn pmf_get_function(func: *mut pmf_function) -> *mut pmf_function;
}
extern "C" {
    pub fn pmf_put_function(func: *mut pmf_function);
}
extern "C" {
    pub fn pmf_call_one(func: *mut pmf_function, args: *mut pmf_args) -> c_int;
}
extern "C" {
    pub fn pmac_pfunc_base_install() -> c_int;
}
// Suspend/resume code called by via-pmu directly for now
extern "C" {
    pub fn pmac_pfunc_base_suspend();
}
extern "C" {
    pub fn pmac_pfunc_base_resume();
}
