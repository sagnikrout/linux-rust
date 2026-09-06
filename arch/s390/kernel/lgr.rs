//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/lgr.c
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
// Linux Guest Relocation (LGR) detection
//
// Copyright IBM Corp. 2012
// Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
//

//
// LGR info: Contains stfle and stsi data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgr_info {
// Bit field with facility information: 4 DWORDs are stored
    pub stfle_fac_list: [u64; 4],
// Level of system (1 = CEC, 2 = LPAR, 3 = z/VM
    pub level: u32,
// Level 1: CEC info (stsi 1.1.1)
    pub manufacturer: [c_char; 16],
    pub type: [c_char; 4],
    pub sequence: [c_char; 16],
    pub plant: [c_char; 4],
    pub model: [c_char; 16],
// Level 2: LPAR info (stsi 2.2.2)
    pub lpar_number: u16,
    pub name: [c_char; 8],
// Level 3: VM info (stsi 3.2.2)
    pub vm_count: u8,
    struct {
    pub name: [c_char; 8],
    pub cpi: [c_char; 16],
    pub vm: [}; VM_LEVEL_MAX],
    pub __aligned(8): } __packed,
//
// LGR globals
//
    pub __aligned(PAGE_SIZE): static char lgr_page[PAGE_SIZE],
    pub lgr_info_last: static struct lgr_info,
    pub lgr_info_cur: static struct lgr_info,
    pub lgr_dbf: *mut static struct debug_info,
//
// Copy buffer and then convert it to ASCII
//
#[no_mangle]
unsafe extern "C" fn cpascii(dst: *mut c_char, src: *mut c_char, size: c_int) {
    static void cpascii(char *dst, char *src, int size)
    {
    pub size): memcpy(dst, src,,
    pub size): EBCASC(dst,,
    }
//
// Fill LGR info with 1.1.1 stsi data
//
#[no_mangle]
unsafe extern "C" fn lgr_stsi_1_1_1(lgr_info: *mut lgr_info) {
    static void lgr_stsi_1_1_1(struct lgr_info *lgr_info)
    {
    pub lgr_page: *mut *mut *mut sysinfo_1_1_1 si = (void ),
    if (stsi(si, 1, 1, 1))
    cpascii(lgr_info.manufacturer, si.manufacturer,
    pub sizeof(si->type)): cpascii(lgr_info->type, si->type,,
    pub sizeof(si->model)): cpascii(lgr_info->model, si->model,,
    pub sizeof(si->sequence)): cpascii(lgr_info->sequence, si->sequence,,
    pub sizeof(si->plant)): cpascii(lgr_info->plant, si->plant,,
    }
//
// Fill LGR info with 2.2.2 stsi data
//
#[no_mangle]
unsafe extern "C" fn lgr_stsi_2_2_2(lgr_info: *mut lgr_info) {
    static void lgr_stsi_2_2_2(struct lgr_info *lgr_info)
    {
    pub lgr_page: *mut *mut *mut sysinfo_2_2_2 si = (void ),
    if (stsi(si, 2, 2, 2))
    pub sizeof(si->name)): cpascii(lgr_info->name, si->name,,
    pub si->lpar_number: lgr_info->lpar_number =,
    }
//
// Fill LGR info with 3.2.2 stsi data
//
#[no_mangle]
unsafe extern "C" fn lgr_stsi_3_2_2(lgr_info: *mut lgr_info) {
    static void lgr_stsi_3_2_2(struct lgr_info *lgr_info)
    {
    pub lgr_page: *mut *mut *mut sysinfo_3_2_2 si = (void ),
    pub i: c_int,
    if (stsi(si, 3, 2, 2))
    pub {: for (i = 0; i < min_t(u8, si->count, VM_LEVEL_MAX); i++),
    cpascii(lgr_info.vm[i].name, si.vm[i].name,
    cpascii(lgr_info.vm[i].cpi, si.vm[i].cpi,
    }
    pub si->count: lgr_info->vm_count =,
    }
//
// Fill LGR info with current data
//
#[no_mangle]
unsafe extern "C" fn lgr_info_get(lgr_info: *mut lgr_info) {
    static void lgr_info_get(struct lgr_info *lgr_info)
    {
    pub level: c_int,
    pub sizeof(*lgr_info)): *mut memset(lgr_info, 0,,
    pub ARRAY_SIZE(lgr_info->stfle_fac_list)): stfle(lgr_info->stfle_fac_list,,
    pub 0): level = stsi(NULL, 0, 0,,
    pub level: lgr_info->level =,
    if (level >= 1)
    if (level >= 2)
    if (level >= 3)
    }
//
// Check if LGR info has changed and if yes log new LGR info to s390dbf
//
#[no_mangle]
pub unsafe extern "C" fn lgr_info_log() {
    void lgr_info_log(void)
    {
    pub DEFINE_SPINLOCK(lgr_info_lock): static,
    pub flags: c_ulong,
    if (!spin_trylock_irqsave(&lgr_info_lock, flags))
    if (memcmp(&lgr_info_last, &lgr_info_cur, sizeof(lgr_info_cur)) != 0) {
    pub sizeof(lgr_info_cur)): debug_event(lgr_dbf, 1, &lgr_info_cur,,
    pub lgr_info_cur: lgr_info_last =,
    }
    pub flags): spin_unlock_irqrestore(&lgr_info_lock,,
    }
    pub lgr_timer_set(void): static void,
//
// LGR timer callback
//
#[no_mangle]
unsafe extern "C" fn lgr_timer_fn(unused: *mut timer_list) {
    static void lgr_timer_fn(struct timer_list *unused)
    {
    }
    pub lgr_timer: static struct timer_list,
//
// Setup next LGR timer
//
#[no_mangle]
unsafe extern "C" fn lgr_timer_set() {
    static void lgr_timer_set(void)
    {
    pub secs_to_jiffies(LGR_TIMER_INTERVAL_SECS)): mod_timer(&lgr_timer, jiffies +,
    }
//
// Initialize LGR: Add s390dbf, write initial lgr_info and setup timer
//
#[no_mangle]
unsafe extern "C" fn lgr_init() -> int __init {
    static int __init lgr_init(void)
    {
    pub lgr_info)): lgr_dbf = debug_register("lgr", 1, 1, sizeof(struct,
    if (!lgr_dbf)
    pub -ENOMEM: return,
    pub &debug_hex_ascii_view): debug_register_view(lgr_dbf,,
    pub sizeof(lgr_info_last)): debug_event(lgr_dbf, 1, &lgr_info_last,,
    pub TIMER_DEFERRABLE): timer_setup(&lgr_timer, lgr_timer_fn,,
    pub 0: return,
    }
