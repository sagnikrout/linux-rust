//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kgdb.h
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
//
// This provides the callbacks and functions that KGDB needs to share between
// the core, I/O and arch-specific portions.
//
// Author: Amit Kale <amitkale@linsyssoft.com> and
// Tom Rini <trini@kernel.crashing.org>
//
// 2001-2004 (c) Amit S. Kale and 2003-2005 (c) MontaVista Software, Inc.
//

//
// kgdb_skipexception - (optional) exit kgdb_handle_exception early
// @exception: Exception vector number
// @regs: Current &struct pt_regs.
//
// On some architectures it is required to skip a breakpoint
// exception when it occurs after a breakpoint has been removed.
// This can be implemented in the architecture specific portion of kgdb.
//
extern "C" {
    pub fn kgdb_skipexception(exception: c_int, regs: *mut pt_regs) -> c_int;
}
//
// kgdb_breakpoint - compiled in breakpoint
//
// This will be implemented as a static inline per architecture.  This
// function is called by the kgdb core to execute an architecture
// specific trap to cause kgdb to enter the exception processing.
//
extern "C" {
    pub fn kgdb_breakpoint();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kgdb_bptype {
    BP_BREAKPOINT = 0,
    BP_HARDWARE_BREAKPOINT,
    BP_WRITE_WATCHPOINT,
    BP_READ_WATCHPOINT,
    BP_ACCESS_WATCHPOINT,
    BP_POKE_BREAKPOINT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kgdb_bpstate {
    BP_UNDEFINED = 0,
    BP_REMOVED,
    BP_SET,
    BP_ACTIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgdb_bkpt {
    pub bpt_addr: c_ulong,
    pub saved_instr: [c_uchar; BREAK_INSTR_SIZE],
    pub type: kgdb_bptype,
    pub state: kgdb_bpstate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_reg_def_t {
    pub name: *mut c_char,
    pub size: c_int,
    pub offset: c_int,
}

pub const DBG_MAX_REG_NUM: c_int = 0;

extern "C" {
    pub fn dbg_set_reg(regno: c_int, mem: *mut c_void, regs: *mut pt_regs) -> c_int;
}

pub const KGDB_HW_BREAKPOINT: c_int = 1;
//
// Functions each KGDB-supporting architecture must provide:
//
// kgdb_arch_init - Perform any architecture specific initialization.
//
// This function will handle the initialization of any architecture
// specific callbacks.
//
extern "C" {
    pub fn kgdb_arch_init() -> c_int;
}
//
// kgdb_arch_exit - Perform any architecture specific uninitalization.
//
// This function will handle the uninitalization of any architecture
// specific callbacks, for dynamic registration and unregistration.
//
extern "C" {
    pub fn kgdb_arch_exit();
}
//
// pt_regs_to_gdb_regs - Convert ptrace regs to GDB regs
// @gdb_regs: A pointer to hold the registers in the order GDB wants.
// @regs: The &struct pt_regs of the current process.
//
// Convert the pt_regs in @regs into the format for registers that
// GDB expects, stored in @gdb_regs.
//
extern "C" {
    pub fn pt_regs_to_gdb_regs(gdb_regs: *mut c_ulong, regs: *mut pt_regs);
}
//
// sleeping_thread_to_gdb_regs - Convert ptrace regs to GDB regs
// @gdb_regs: A pointer to hold the registers in the order GDB wants.
// @p: The &struct task_struct of the desired process.
//
// Convert the register values of the sleeping process in @p to
// the format that GDB expects.
// This function is called when kgdb does not have access to the
// &struct pt_regs and therefore it should fill the gdb registers
// @gdb_regs with what has	been saved in &struct thread_struct
// thread field during switch_to.
//
// gdb_regs_to_pt_regs - Convert GDB regs to ptrace regs.
// @gdb_regs: A pointer to hold the registers we've received from GDB.
// @regs: A pointer to a &struct pt_regs to hold these values in.
//
// Convert the GDB regs in @gdb_regs into the pt_regs, and store them
// in @regs.
//
extern "C" {
    pub fn gdb_regs_to_pt_regs(gdb_regs: *mut c_ulong, regs: *mut pt_regs);
}
//
// kgdb_arch_handle_exception - Handle architecture specific GDB packets.
// @vector: The error vector of the exception that happened.
// @signo: The signal number of the exception that happened.
// @err_code: The error code of the exception that happened.
// @remcom_in_buffer: The buffer of the packet we have read.
// @remcom_out_buffer: The buffer of %BUFMAX bytes to write a packet into.
// @regs: The &struct pt_regs of the current process.
//
// This function MUST handle the 'c' and 's' command packets,
// as well packets to set / remove a hardware breakpoint, if used.
// If there are additional packets which the hardware needs to handle,
// they are handled here.  The code should return -1 if it wants to
// process more packets, and a %0 or %1 if it wants to exit from the
// kgdb callback.
//
// kgdb_arch_handle_qxfer_pkt - Handle architecture specific GDB XML
// packets.
// @remcom_in_buffer: The buffer of the packet we have read.
// @remcom_out_buffer: The buffer of %BUFMAX bytes to write a packet into.
//
// kgdb_call_nmi_hook - Call kgdb_nmicallback() on the current CPU
// @ignored: This parameter is only here to match the prototype.
//
// If you're using the default implementation of kgdb_roundup_cpus()
// this function will be called per CPU.  If you don't implement
// kgdb_call_nmi_hook() a default will be used.
//
extern "C" {
    pub fn kgdb_call_nmi_hook(ignored: *mut c_void);
}
//
// kgdb_roundup_cpus - Get other CPUs into a holding pattern
//
// On SMP systems, we need to get the attention of the other CPUs
// and get them into a known state.  This should do what is needed
// to get the other CPUs to call kgdb_handle_exception().  Note that
// on some arches, the NMI approach is not used for rounding up all
// the CPUs.  Normally those architectures can just not implement
// this and get the default.
//
// On non-SMP systems, this is not called.
//
extern "C" {
    pub fn kgdb_roundup_cpus();
}
//
// kgdb_arch_set_pc - Generic call back to the program counter
// @regs: Current &struct pt_regs.
// @pc: The new value for the program counter
//
// This function handles updating the program counter and requires an
// architecture specific implementation.
//
extern "C" {
    pub fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: c_ulong);
}
// Optional functions.
extern "C" {
    pub fn kgdb_validate_break_address(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kgdb_arch_set_breakpoint(bpt: *mut kgdb_bkpt) -> c_int;
}
extern "C" {
    pub fn kgdb_arch_remove_breakpoint(bpt: *mut kgdb_bkpt) -> c_int;
}
//
// kgdb_arch_late - Perform any architecture specific initialization.
//
// This function will handle the late initialization of any
// architecture specific callbacks.  This is an optional function for
// handling things like late initialization of hw breakpoints.  The
// default implementation does nothing.
//
extern "C" {
    pub fn kgdb_arch_late();
}
//
// struct kgdb_arch - Describe architecture specific values.
// @gdb_bpt_instr: The instruction to trigger a breakpoint.
// @flags: Flags for the breakpoint, currently just %KGDB_HW_BREAKPOINT.
// @set_breakpoint: Allow an architecture to specify how to set a software
// breakpoint.
// @remove_breakpoint: Allow an architecture to specify how to remove a
// software breakpoint.
// @set_hw_breakpoint: Allow an architecture to specify how to set a hardware
// breakpoint.
// @remove_hw_breakpoint: Allow an architecture to specify how to remove a
// hardware breakpoint.
// @disable_hw_break: Allow an architecture to specify how to disable
// hardware breakpoints for a single cpu.
// @remove_all_hw_break: Allow an architecture to specify how to remove all
// hardware breakpoints.
// @correct_hw_break: Allow an architecture to specify how to correct the
// hardware debug registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgdb_arch {
    pub gdb_bpt_instr: [c_uchar; BREAK_INSTR_SIZE],
    pub flags: c_ulong,
    pub ): *mut *mut int (set_breakpoint)(unsigned long, char,
    pub ): *mut *mut int (remove_breakpoint)(unsigned long, char,
    pub kgdb_bptype): *mut *mut int (set_hw_breakpoint)(unsigned long, int, enum,
    pub kgdb_bptype): *mut *mut int (remove_hw_breakpoint)(unsigned long, int, enum,
    pub regs): *mut *mut void (disable_hw_break)(struct pt_regs,
    pub (*remove_all_hw_break)(void): *mut c_void,
    pub (*correct_hw_break)(void): *mut c_void,
}

//
// struct kgdb_io - Describe the interface for an I/O driver to talk with KGDB.
// @name: Name of the I/O driver.
// @read_char: Pointer to a function that will return one char.
// @write_char: Pointer to a function that will write one char.
// @flush: Pointer to a function that will flush any pending writes.
// @init: Pointer to a function that will initialize the device.
// @deinit: Pointer to a function that will deinit the device. Implies that
// this I/O driver is temporary and expects to be replaced. Called when
// an I/O driver is replaced or explicitly unregistered.
// @pre_exception: Pointer to a function that will do any prep work for
// the I/O driver.
// @post_exception: Pointer to a function that will do any cleanup work
// for the I/O driver.
// @cons: valid if the I/O device is a console; else NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgdb_io {
    pub name: *const c_char,
    pub (void): *mut *mut int (read_char),
    pub (u8): *mut *mut void (write_char),
    pub (void): *mut *mut void (flush),
    pub (void): *mut *mut int (init),
    pub (void): *mut *mut void (deinit),
    pub (void): *mut *mut void (pre_exception),
    pub (void): *mut *mut void (post_exception),
    pub cons: *mut console,
}

extern "C" {
    pub fn kgdb_arch_pc(exception: c_int, regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn kgdb_register_io_module(local_kgdb_io_ops: *mut kgdb_io) -> c_int;
}
extern "C" {
    pub fn kgdb_unregister_io_module(local_kgdb_io_ops: *mut kgdb_io);
}
extern "C" {
    pub fn kgdb_hex2long(ptr: *mut c_char, long_val: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn kgdb_hex2mem(buf: *mut c_char, mem: *mut c_char, count: c_int) -> c_int;
}
extern "C" {
    pub fn kgdb_isremovedbreak(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kgdb_has_hit_break(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kgdb_nmicallback(cpu: c_int, regs: *mut c_void) -> c_int;
}
extern "C" {
    pub fn gdbstub_exit(status: c_int);
}
//
// kgdb and kprobes both use the same (kprobe) blocklist (which makes sense
// given they are both typically hooked up to the same trap meaning on most
// architectures one cannot be used to debug the other)
//
// However on architectures where kprobes is not (yet) implemented we permit
// breakpoints everywhere rather than blocking everything by default.
//

extern "C" {
    pub fn within_kprobe_blacklist(_arg: addr) -> return;
}

extern "C" {
    pub fn dbg_late_init() -> void __init;
}
extern "C" {
    pub fn kgdb_panic(msg: *const c_char);
}
extern "C" {
    pub fn kgdb_free_init_mem();
}

// Macro flag: #define dbg_late_init()

