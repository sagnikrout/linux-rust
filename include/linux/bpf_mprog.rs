//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf_mprog.h
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
// Copyright (c) 2023 Isovalent

// bpf_mprog framework:
//
// bpf_mprog is a generic layer for multi-program attachment. In-kernel users
// of the bpf_mprog don't need to care about the dependency resolution
// internals, they can just consume it with few API calls. Currently available
// dependency directives are BPF_F_{BEFORE,AFTER} which enable insertion of
// a BPF program or BPF link relative to an existing BPF program or BPF link
// inside the multi-program array as well as prepend and append behavior if
// no relative object was specified, see corresponding selftests for concrete
// examples (e.g. tc_links and tc_opts test cases of test_progs).
//
// Usage of bpf_mprog_{attach,detach,query}() core APIs with pseudo code:
//
// Attach case:
//
// struct bpf_mprog_entry *entry, *entry_new;
// int ret;
//
// // bpf_mprog user-side lock
// // fetch active @entry from attach location
// [...]
// ret = bpf_mprog_attach(entry, &entry_new, [...]);
// if (!ret) {
// if (entry != entry_new) {
// // swap @entry to @entry_new at attach location
// // ensure there are no inflight users of @entry:
// synchronize_rcu();
// }
// bpf_mprog_commit(entry);
// } else {
// // error path, bail out, propagate @ret
// }
// // bpf_mprog user-side unlock
//
// Detach case:
//
// struct bpf_mprog_entry *entry, *entry_new;
// int ret;
//
// // bpf_mprog user-side lock
// // fetch active @entry from attach location
// [...]
// ret = bpf_mprog_detach(entry, &entry_new, [...]);
// if (!ret) {
// // all (*) marked is optional and depends on the use-case
// // whether bpf_mprog_bundle should be freed or not
// if (!bpf_mprog_total(entry_new))     (*)
// entry_new = NULL                 (*)
// // swap @entry to @entry_new at attach location
// // ensure there are no inflight users of @entry:
// synchronize_rcu();
// bpf_mprog_commit(entry);
// if (!entry_new)                      (*)
// // free bpf_mprog_bundle         (*)
// } else {
// // error path, bail out, propagate @ret
// }
// // bpf_mprog user-side unlock
//
// Query case:
//
// struct bpf_mprog_entry *entry;
// int ret;
//
// // bpf_mprog user-side lock
// // fetch active @entry from attach location
// [...]
// ret = bpf_mprog_query(attr, uattr, entry);
// // bpf_mprog user-side unlock
//
// Data/fast path:
//
// struct bpf_mprog_entry *entry;
// struct bpf_mprog_fp *fp;
// struct bpf_prog *prog;
// int ret = [...];
//
// rcu_read_lock();
// // fetch active @entry from attach location
// [...]
// bpf_mprog_foreach_prog(entry, fp, prog) {
// ret = bpf_prog_run(prog, [...]);
// // process @ret from program
// }
// [...]
// rcu_read_unlock();
//
// bpf_mprog locking considerations:
//
// bpf_mprog_{attach,detach,query}() must be protected by an external lock
// (like RTNL in case of tcx).
//
// bpf_mprog_entry pointer can be an __rcu annotated pointer (in case of tcx
// the netdevice has tcx_ingress and tcx_egress __rcu pointer) which gets
// updated via rcu_assign_pointer() pointing to the active bpf_mprog_entry of
// the bpf_mprog_bundle.
//
// Fast path accesses the active bpf_mprog_entry within RCU critical section
// (in case of tcx it runs in NAPI which provides RCU protection there,
// other users might need explicit rcu_read_lock()). The bpf_mprog_commit()
// assumes that for the old bpf_mprog_entry there are no inflight users
// anymore.
//
// The READ_ONCE()/WRITE_ONCE() pairing for bpf_mprog_fp's prog access is for
// the replacement case where we don't swap the bpf_mprog_entry.
//

pub const BPF_MPROG_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mprog_fp {
    pub prog: *mut bpf_prog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mprog_cp {
    pub link: *mut bpf_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mprog_entry {
    pub fp_items: [bpf_mprog_fp; BPF_MPROG_MAX],
    pub parent: *mut bpf_mprog_bundle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mprog_bundle {
    pub a: bpf_mprog_entry,
    pub b: bpf_mprog_entry,
    pub cp_items: [bpf_mprog_cp; BPF_MPROG_MAX],
    pub ref: *mut bpf_prog,
    pub revision: core::sync::atomic::AtomicI64,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tuple {
    pub prog: *mut bpf_prog,
    pub link: *mut bpf_link,
}

// In the non-link case prog deletions can only drop the reference
// to the prog after the bpf_mprog_entry got swapped and the
// bpf_mprog ensured that there are no inflight users anymore.
//
// Paired with bpf_mprog_mark_for_release().
//
extern "C" {
    pub fn atomic64_read(_arg: &entry->parent->revision) -> return;
}
// entry_new = peer;
// Total array size is needed in this case to enure the NULL
// entry is copied at the end.
//
// fp = &entry->fp_items[idx];
// cp = &entry->parent->cp_items[idx];
extern "C" {
    pub fn bpf_net_capable() -> return;
}
