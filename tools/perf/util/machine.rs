//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/machine.h
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

// Native host kernel uses -1 as pid index in machine

#[repr(C)]
#[derive(Copy, Clone)]
pub struct machine {
    pub rb_node: rb_node,
    pub pid: pid_t,
    pub id_hdr_size: u16,
    pub comm_exec: bool,
    pub kptr_restrict_warned: bool,
    pub single_address_space: bool,
    pub root_dir: *mut c_char,
    pub mmap_name: *mut c_char,
    pub kallsyms_filename: *mut c_char,
    pub threads: threads,
    pub vdso_info: *mut vdso_info,
    pub env: *mut perf_env,
    pub dsos: dsos,
    pub kmaps: *mut maps,
    pub vmlinux_map: *mut map,
    pub kernel_start: u64,
    pub text_start: u64,
    pub text_end: u64,
    pub trace: } sched, lock, traceiter,,
//
// The current parallelism level (number of threads that run on CPUs).
// This value can be less than 1, or larger than the total number
// of CPUs, if events are poorly ordered.
//
    pub parallelism: c_int,
    pub current_tid: *mut pid_t,
    pub current_tid_sz: usize,
    pub priv: *mut c_void,
    pub db_id: u64,
}

//
// The main kernel (vmlinux) map
//
// kernel (the one returned by machine__kernel_map()) plus kernel modules maps
//
extern "C" {
    pub fn machine__get_kernel_start(machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn machine__addr_cpumode(machine: *mut machine, cpumode: u8, addr: u64) -> u8;
}
extern "C" {
    pub fn void(machine: *mut *mut machine__process_t)(struct machine, data: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct machines {
    pub host: machine,
    pub guests: rb_root_cached,
}

extern "C" {
    pub fn machines__init(machines: *mut machines) -> c_int;
}
extern "C" {
    pub fn machines__exit(machines: *mut machines);
}
extern "C" {
    pub fn machines__set_id_hdr_size(machines: *mut machines, id_hdr_size: u16);
}
extern "C" {
    pub fn machines__set_comm_exec(machines: *mut machines, comm_exec: bool);
}
extern "C" {
    pub fn machine__init(machine: *mut machine, root_dir: *const c_char, pid: pid_t) -> c_int;
}
extern "C" {
    pub fn machine__exit(machine: *mut machine);
}
extern "C" {
    pub fn machine__delete_threads(machine: *mut machine);
}
extern "C" {
    pub fn machine__delete(machine: *mut machine);
}
extern "C" {
    pub fn machine__remove_thread(machine: *mut machine, th: *mut thread);
}
// symbols=*/true);
//
// Default guest kernel is defined by parameter --guestkallsyms
// and --guestmodules
//
extern "C" {
    pub fn machine__is_lock_function(machine: *mut machine, addr: u64) -> bool;
}
extern "C" {
    pub fn machine__nr_cpus_avail(machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn machine__fprintf(machine: *mut machine, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn maps__find_symbol(_arg: machine->kmaps, _arg: addr, _arg: mapp) -> return;
}
extern "C" {
    pub fn maps__find_symbol_by_name(_arg: machine->kmaps, _arg: name, _arg: mapp) -> return;
}
extern "C" {
    pub fn arch__fix_module_text_start(start: *mut u64, size: *mut u64, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn machine__load_kallsyms(machine: *mut machine, filename: *const c_char) -> c_int;
}
extern "C" {
    pub fn machine__load_vmlinux_path(machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn machines__fprintf_dsos(machines: *mut machines, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn machine__destroy_kernel_maps(machine: *mut machine);
}
extern "C" {
    pub fn machine__create_kernel_maps(machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn machines__create_kernel_maps(machines: *mut machines, pid: pid_t) -> c_int;
}
extern "C" {
    pub fn machines__create_guest_kernel_maps(machines: *mut machines) -> c_int;
}
extern "C" {
    pub fn machines__destroy_kernel_maps(machines: *mut machines);
}
extern "C" {
    pub fn int(dso: *mut *mut machine__dso_t)(struct dso, machine: *mut machine, priv: *mut c_void) -> typedef;
}
extern "C" {
    pub fn int(map: *mut *mut machine__map_t)(struct map, priv: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_list {
    pub list: list_head,
    pub thread: *mut thread,
}

// Make a list of struct thread_list based on threads in the machine.
extern "C" {
    pub fn machine__thread_list(machine: *mut machine, list: *mut list_head) -> c_int;
}
// Free up the nodes within the thread_list list.
extern "C" {
    pub fn thread_list__delete(list: *mut list_head);
}
extern "C" {
    pub fn machine__get_current_tid(machine: *mut machine, cpu: c_int) -> pid_t;
}
//
// For use with libtraceevent's tep_set_function_resolver()
//
// Kernel-space maps for symbols that are outside the main kernel map and module maps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extra_kernel_map {
    pub start: u64,
    pub end: u64,
    pub pgoff: u64,
    pub name: [c_char; KMAP_NAME_LEN],
}

extern "C" {
    pub fn machine__hit_all_dsos(machine: *mut machine) -> c_int;
}
