//! Automatically rewritten from C to Rust
//! Source: mm/init-mm.c
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

// Macro flag: #define INIT_MM_CONTEXT(name)

    const struct vm_operations_struct vma_dummy_vm_ops;
//
// For dynamically allocated mm_structs, there is a dynamically sized cpumask
// at the end of the structure, the size of which depends on the maximum CPU
// number the system can see. That way we allocate only as much memory for
// mm_cpumask() as needed for the hundreds, or thousands of processes that
// a system typically runs.
//
// Since there is only one init_mm in the entire system, keep it simple
// and size this cpu_bitmask to NR_CPUS.
//
    struct mm_struct init_mm = {
    .mm_mt		= MTREE_INIT_EXT(mm_mt, MM_MT_FLAGS, init_mm.mmap_lock),
    .pgd		= swapper_pg_dir,
    .mm_users	= ATOMIC_INIT(2),
    .mm_count	= ATOMIC_INIT(1),
    .write_protect_seq = SEQCNT_ZERO(init_mm.write_protect_seq),
    MMAP_LOCK_INITIALIZER(init_mm)
    .page_table_lock =  __SPIN_LOCK_UNLOCKED(init_mm.page_table_lock),
    .arg_lock	=  __SPIN_LOCK_UNLOCKED(init_mm.arg_lock),
    .mmlist		= LIST_HEAD_INIT(init_mm.mmlist),

    .vma_writer_wait = __RCUWAIT_INITIALIZER(init_mm.vma_writer_wait),
    .mm_lock_seq	= SEQCNT_ZERO(init_mm.mm_lock_seq),

    .mm_cid.lock = __RAW_SPIN_LOCK_UNLOCKED(init_mm.mm_cid.lock),

    .flexible_array	= MM_STRUCT_FLEXIBLE_ARRAY_INIT,
    INIT_MM_CONTEXT(init_mm)
    };
    void setup_initial_init_mm(void *start_code, void *end_code,
    void *end_data, void *brk)
    {
    init_mm.start_code = (unsigned long)start_code;
    init_mm.end_code = (unsigned long)end_code;
    init_mm.end_data = (unsigned long)end_data;
    init_mm.brk = (unsigned long)brk;
    }
