//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/kvm.c
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
// Copyright (C) 2010 SUSE Linux Products GmbH. All rights reserved.
// Copyright 2010-2011 Freescale Semiconductor, Inc.
//
// Authors:
// Alexander Graf <agraf@suse.de>
//

pub const KVM_INST_LWZ: c_uint = 0x80000000;
pub const KVM_INST_STW: c_uint = 0x90000000;
pub const KVM_INST_LD: c_uint = 0xe8000000;
pub const KVM_INST_STD: c_uint = 0xf8000000;
pub const KVM_INST_NOP: c_uint = 0x60000000;
pub const KVM_INST_B: c_uint = 0x48000000;
pub const KVM_INST_B_MASK: c_uint = 0x03ffffff;
pub const KVM_INST_B_MAX: c_uint = 0x01ffffff;
pub const KVM_INST_LI: c_uint = 0x38000000;
pub const KVM_MASK_RT: c_uint = 0x03e00000;
pub const KVM_RT_30: c_uint = 0x03c00000;
pub const KVM_MASK_RB: c_uint = 0x0000f800;
pub const KVM_INST_MFMSR: c_uint = 0x7c0000a6;
pub const SPR_FROM: c_int = 0;
pub const SPR_TO: c_uint = 0x100;

    (((sprn) & 0x1f) << 16) | \
    (((sprn) & 0x3e0) << 6) | \
    (moveto))

pub const KVM_INST_TLBSYNC: c_uint = 0x7c00046c;
pub const KVM_INST_MTMSRD_L0: c_uint = 0x7c000164;
pub const KVM_INST_MTMSRD_L1: c_uint = 0x7c010164;
pub const KVM_INST_MTMSR: c_uint = 0x7c000124;
pub const KVM_INST_WRTEE: c_uint = 0x7c000106;
pub const KVM_INST_WRTEEI_0: c_uint = 0x7c000146;
pub const KVM_INST_WRTEEI_1: c_uint = 0x7c008146;
pub const KVM_INST_MTSRIN: c_uint = 0x7c0001e4;
    let mut kvm_patching_worked: static bool = true;
    extern char kvm_tmp[];
    extern char kvm_tmp_end[];
    static int kvm_tmp_index;
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins(inst: *mut u32, new_inst: u32) -> void __init {
    static void __init kvm_patch_ins(u32 *inst, u32 new_inst)
    {
// inst = new_inst;
    flush_icache_range((ulong)inst, (ulong)inst + 4);
    }
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_ll(inst: *mut u32, addr: c_long, rt: u32) -> void __init {
    static void __init kvm_patch_ins_ll(u32 *inst, long addr, u32 rt)
    {

    kvm_patch_ins(inst, KVM_INST_LD | rt | (addr & 0x0000fffc));

    kvm_patch_ins(inst, KVM_INST_LWZ | rt | (addr & 0x0000fffc));

    }
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_ld(inst: *mut u32, addr: c_long, rt: u32) -> void __init {
    static void __init kvm_patch_ins_ld(u32 *inst, long addr, u32 rt)
    {

    kvm_patch_ins(inst, KVM_INST_LD | rt | (addr & 0x0000fffc));

    kvm_patch_ins(inst, KVM_INST_LWZ | rt | ((addr + 4) & 0x0000fffc));

    }
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_lwz(inst: *mut u32, addr: c_long, rt: u32) -> void __init {
    static void __init kvm_patch_ins_lwz(u32 *inst, long addr, u32 rt)
    {
    kvm_patch_ins(inst, KVM_INST_LWZ | rt | (addr & 0x0000ffff));
    }
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_std(inst: *mut u32, addr: c_long, rt: u32) -> void __init {
    static void __init kvm_patch_ins_std(u32 *inst, long addr, u32 rt)
    {

    kvm_patch_ins(inst, KVM_INST_STD | rt | (addr & 0x0000fffc));

    kvm_patch_ins(inst, KVM_INST_STW | rt | ((addr + 4) & 0x0000fffc));

    }
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_stw(inst: *mut u32, addr: c_long, rt: u32) -> void __init {
    static void __init kvm_patch_ins_stw(u32 *inst, long addr, u32 rt)
    {
    kvm_patch_ins(inst, KVM_INST_STW | rt | (addr & 0x0000fffc));
    }
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_nop(inst: *mut u32) -> void __init {
    static void __init kvm_patch_ins_nop(u32 *inst)
    {
    kvm_patch_ins(inst, KVM_INST_NOP);
    }
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_b(inst: *mut u32, addr: c_int) -> void __init {
    static void __init kvm_patch_ins_b(u32 *inst, int addr)
    {

// On relocatable kernels interrupts handlers and our code
    can be in different regions, so we don't patch them */
    if ((ulong)inst < (ulong)&__end_interrupts)
    return;

    kvm_patch_ins(inst, KVM_INST_B | (addr & KVM_INST_B_MASK));
    }
#[no_mangle]
unsafe extern "C" fn kvm_alloc(len: c_int) -> *mut u32  __init {
    static u32 * __init kvm_alloc(int len)
    {
    u32 *p;
    if ((kvm_tmp_index + len) > (kvm_tmp_end - kvm_tmp)) {
    printk(KERN_ERR "KVM: No more space (%d + %d)\n",
    kvm_tmp_index, len);
    kvm_patching_worked = false;
    return core::ptr::null_mut();
    }
    p = (void*)&kvm_tmp[kvm_tmp_index];
    kvm_tmp_index += len;
    return p;
    }
    extern u32 kvm_emulate_mtmsrd_branch_offs;
    extern u32 kvm_emulate_mtmsrd_reg_offs;
    extern u32 kvm_emulate_mtmsrd_orig_ins_offs;
    extern u32 kvm_emulate_mtmsrd_len;
    extern u32 kvm_emulate_mtmsrd[];
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_mtmsrd(inst: *mut u32, rt: u32) -> void __init {
    static void __init kvm_patch_ins_mtmsrd(u32 *inst, u32 rt)
    {
    u32 *p;
    int distance_start;
    int distance_end;
    ulong next_inst;
    p = kvm_alloc(kvm_emulate_mtmsrd_len * 4);
    if (!p)
    return;
// Find out where we are and put everything there
    distance_start = (ulong)p - (ulong)inst;
    next_inst = ((ulong)inst + 4);
    distance_end = next_inst - (ulong)&p[kvm_emulate_mtmsrd_branch_offs];
// Make sure we only write valid b instructions
    if (distance_start > KVM_INST_B_MAX) {
    kvm_patching_worked = false;
    return;
    }
// Modify the chunk to fit the invocation
    memcpy(p, kvm_emulate_mtmsrd, kvm_emulate_mtmsrd_len * 4);
    p[kvm_emulate_mtmsrd_branch_offs] |= distance_end & KVM_INST_B_MASK;
    switch (get_rt(rt)) {
    case 30:
    kvm_patch_ins_ll(&p[kvm_emulate_mtmsrd_reg_offs],
    magic_var(scratch2), KVM_RT_30);
    break;
    case 31:
    kvm_patch_ins_ll(&p[kvm_emulate_mtmsrd_reg_offs],
    magic_var(scratch1), KVM_RT_30);
    break;
    default:
    p[kvm_emulate_mtmsrd_reg_offs] |= rt;
    break;
    }
    p[kvm_emulate_mtmsrd_orig_ins_offs] = *inst;
    flush_icache_range((ulong)p, (ulong)p + kvm_emulate_mtmsrd_len * 4);
// Patch the invocation
    kvm_patch_ins_b(inst, distance_start);
    }
    extern u32 kvm_emulate_mtmsr_branch_offs;
    extern u32 kvm_emulate_mtmsr_reg1_offs;
    extern u32 kvm_emulate_mtmsr_reg2_offs;
    extern u32 kvm_emulate_mtmsr_orig_ins_offs;
    extern u32 kvm_emulate_mtmsr_len;
    extern u32 kvm_emulate_mtmsr[];
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_mtmsr(inst: *mut u32, rt: u32) -> void __init {
    static void __init kvm_patch_ins_mtmsr(u32 *inst, u32 rt)
    {
    u32 *p;
    int distance_start;
    int distance_end;
    ulong next_inst;
    p = kvm_alloc(kvm_emulate_mtmsr_len * 4);
    if (!p)
    return;
// Find out where we are and put everything there
    distance_start = (ulong)p - (ulong)inst;
    next_inst = ((ulong)inst + 4);
    distance_end = next_inst - (ulong)&p[kvm_emulate_mtmsr_branch_offs];
// Make sure we only write valid b instructions
    if (distance_start > KVM_INST_B_MAX) {
    kvm_patching_worked = false;
    return;
    }
// Modify the chunk to fit the invocation
    memcpy(p, kvm_emulate_mtmsr, kvm_emulate_mtmsr_len * 4);
    p[kvm_emulate_mtmsr_branch_offs] |= distance_end & KVM_INST_B_MASK;
// Make clobbered registers work too
    switch (get_rt(rt)) {
    case 30:
    kvm_patch_ins_ll(&p[kvm_emulate_mtmsr_reg1_offs],
    magic_var(scratch2), KVM_RT_30);
    kvm_patch_ins_ll(&p[kvm_emulate_mtmsr_reg2_offs],
    magic_var(scratch2), KVM_RT_30);
    break;
    case 31:
    kvm_patch_ins_ll(&p[kvm_emulate_mtmsr_reg1_offs],
    magic_var(scratch1), KVM_RT_30);
    kvm_patch_ins_ll(&p[kvm_emulate_mtmsr_reg2_offs],
    magic_var(scratch1), KVM_RT_30);
    break;
    default:
    p[kvm_emulate_mtmsr_reg1_offs] |= rt;
    p[kvm_emulate_mtmsr_reg2_offs] |= rt;
    break;
    }
    p[kvm_emulate_mtmsr_orig_ins_offs] = *inst;
    flush_icache_range((ulong)p, (ulong)p + kvm_emulate_mtmsr_len * 4);
// Patch the invocation
    kvm_patch_ins_b(inst, distance_start);
    }

    extern u32 kvm_emulate_wrtee_branch_offs;
    extern u32 kvm_emulate_wrtee_reg_offs;
    extern u32 kvm_emulate_wrtee_orig_ins_offs;
    extern u32 kvm_emulate_wrtee_len;
    extern u32 kvm_emulate_wrtee[];
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_wrtee(inst: *mut u32, rt: u32, imm_one: c_int) -> void __init {
    static void __init kvm_patch_ins_wrtee(u32 *inst, u32 rt, int imm_one)
    {
    u32 *p;
    int distance_start;
    int distance_end;
    ulong next_inst;
    p = kvm_alloc(kvm_emulate_wrtee_len * 4);
    if (!p)
    return;
// Find out where we are and put everything there
    distance_start = (ulong)p - (ulong)inst;
    next_inst = ((ulong)inst + 4);
    distance_end = next_inst - (ulong)&p[kvm_emulate_wrtee_branch_offs];
// Make sure we only write valid b instructions
    if (distance_start > KVM_INST_B_MAX) {
    kvm_patching_worked = false;
    return;
    }
// Modify the chunk to fit the invocation
    memcpy(p, kvm_emulate_wrtee, kvm_emulate_wrtee_len * 4);
    p[kvm_emulate_wrtee_branch_offs] |= distance_end & KVM_INST_B_MASK;
    if (imm_one) {
    p[kvm_emulate_wrtee_reg_offs] =
    KVM_INST_LI | __PPC_RT(R30) | MSR_EE;
    } else {
// Make clobbered registers work too
    switch (get_rt(rt)) {
    case 30:
    kvm_patch_ins_ll(&p[kvm_emulate_wrtee_reg_offs],
    magic_var(scratch2), KVM_RT_30);
    break;
    case 31:
    kvm_patch_ins_ll(&p[kvm_emulate_wrtee_reg_offs],
    magic_var(scratch1), KVM_RT_30);
    break;
    default:
    p[kvm_emulate_wrtee_reg_offs] |= rt;
    break;
    }
    }
    p[kvm_emulate_wrtee_orig_ins_offs] = *inst;
    flush_icache_range((ulong)p, (ulong)p + kvm_emulate_wrtee_len * 4);
// Patch the invocation
    kvm_patch_ins_b(inst, distance_start);
    }
    extern u32 kvm_emulate_wrteei_0_branch_offs;
    extern u32 kvm_emulate_wrteei_0_len;
    extern u32 kvm_emulate_wrteei_0[];
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_wrteei_0(inst: *mut u32) -> void __init {
    static void __init kvm_patch_ins_wrteei_0(u32 *inst)
    {
    u32 *p;
    int distance_start;
    int distance_end;
    ulong next_inst;
    p = kvm_alloc(kvm_emulate_wrteei_0_len * 4);
    if (!p)
    return;
// Find out where we are and put everything there
    distance_start = (ulong)p - (ulong)inst;
    next_inst = ((ulong)inst + 4);
    distance_end = next_inst - (ulong)&p[kvm_emulate_wrteei_0_branch_offs];
// Make sure we only write valid b instructions
    if (distance_start > KVM_INST_B_MAX) {
    kvm_patching_worked = false;
    return;
    }
    memcpy(p, kvm_emulate_wrteei_0, kvm_emulate_wrteei_0_len * 4);
    p[kvm_emulate_wrteei_0_branch_offs] |= distance_end & KVM_INST_B_MASK;
    flush_icache_range((ulong)p, (ulong)p + kvm_emulate_wrteei_0_len * 4);
// Patch the invocation
    kvm_patch_ins_b(inst, distance_start);
    }

    extern u32 kvm_emulate_mtsrin_branch_offs;
    extern u32 kvm_emulate_mtsrin_reg1_offs;
    extern u32 kvm_emulate_mtsrin_reg2_offs;
    extern u32 kvm_emulate_mtsrin_orig_ins_offs;
    extern u32 kvm_emulate_mtsrin_len;
    extern u32 kvm_emulate_mtsrin[];
#[no_mangle]
unsafe extern "C" fn kvm_patch_ins_mtsrin(inst: *mut u32, rt: u32, rb: u32) -> void __init {
    static void __init kvm_patch_ins_mtsrin(u32 *inst, u32 rt, u32 rb)
    {
    u32 *p;
    int distance_start;
    int distance_end;
    ulong next_inst;
    p = kvm_alloc(kvm_emulate_mtsrin_len * 4);
    if (!p)
    return;
// Find out where we are and put everything there
    distance_start = (ulong)p - (ulong)inst;
    next_inst = ((ulong)inst + 4);
    distance_end = next_inst - (ulong)&p[kvm_emulate_mtsrin_branch_offs];
// Make sure we only write valid b instructions
    if (distance_start > KVM_INST_B_MAX) {
    kvm_patching_worked = false;
    return;
    }
// Modify the chunk to fit the invocation
    memcpy(p, kvm_emulate_mtsrin, kvm_emulate_mtsrin_len * 4);
    p[kvm_emulate_mtsrin_branch_offs] |= distance_end & KVM_INST_B_MASK;
    p[kvm_emulate_mtsrin_reg1_offs] |= (rb << 10);
    p[kvm_emulate_mtsrin_reg2_offs] |= rt;
    p[kvm_emulate_mtsrin_orig_ins_offs] = *inst;
    flush_icache_range((ulong)p, (ulong)p + kvm_emulate_mtsrin_len * 4);
// Patch the invocation
    kvm_patch_ins_b(inst, distance_start);
    }

#[no_mangle]
unsafe extern "C" fn kvm_map_magic_page(data: *mut c_void) -> void __init {
    static void __init kvm_map_magic_page(void *data)
    {
    u32 *features = data;
    ulong in[8] = {0};
    ulong out[8];
    in[0] = KVM_MAGIC_PAGE;
    in[1] = KVM_MAGIC_PAGE | MAGIC_PAGE_FLAG_NOT_MAPPED_NX;
    epapr_hypercall(in, out, KVM_HCALL_TOKEN(KVM_HC_PPC_MAP_MAGIC_PAGE));
// features = out[0];
    }
#[no_mangle]
unsafe extern "C" fn kvm_check_ins(inst: *mut u32, features: u32) -> void __init {
    static void __init kvm_check_ins(u32 *inst, u32 features)
    {
    let mut _inst: u32 = *inst;
    let mut inst_no_rt: u32 = _inst & ~KVM_MASK_RT;
    let mut inst_rt: u32 = _inst & KVM_MASK_RT;
    switch (inst_no_rt) {
// Loads
    case KVM_INST_MFMSR:
    kvm_patch_ins_ld(inst, magic_var(msr), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SPRG0):
    kvm_patch_ins_ld(inst, magic_var(sprg0), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SPRG1):
    kvm_patch_ins_ld(inst, magic_var(sprg1), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SPRG2):
    kvm_patch_ins_ld(inst, magic_var(sprg2), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SPRG3):
    kvm_patch_ins_ld(inst, magic_var(sprg3), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SRR0):
    kvm_patch_ins_ld(inst, magic_var(srr0), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SRR1):
    kvm_patch_ins_ld(inst, magic_var(srr1), inst_rt);
    break;

    case KVM_INST_MFSPR(SPRN_DEAR):

    case KVM_INST_MFSPR(SPRN_DAR):

    kvm_patch_ins_ld(inst, magic_var(dar), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_DSISR):
    kvm_patch_ins_lwz(inst, magic_var(dsisr), inst_rt);
    break;

    case KVM_INST_MFSPR(SPRN_MAS0):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(mas0), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_MAS1):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(mas1), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_MAS2):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_ld(inst, magic_var(mas2), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_MAS3):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(mas7_3) + 4, inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_MAS4):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(mas4), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_MAS6):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(mas6), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_MAS7):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(mas7_3), inst_rt);
    break;

    case KVM_INST_MFSPR(SPRN_SPRG4):

    case KVM_INST_MFSPR(SPRN_SPRG4R):

    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_ld(inst, magic_var(sprg4), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SPRG5):

    case KVM_INST_MFSPR(SPRN_SPRG5R):

    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_ld(inst, magic_var(sprg5), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SPRG6):

    case KVM_INST_MFSPR(SPRN_SPRG6R):

    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_ld(inst, magic_var(sprg6), inst_rt);
    break;
    case KVM_INST_MFSPR(SPRN_SPRG7):

    case KVM_INST_MFSPR(SPRN_SPRG7R):

    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_ld(inst, magic_var(sprg7), inst_rt);
    break;

    case KVM_INST_MFSPR(SPRN_ESR):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(esr), inst_rt);
    break;

    case KVM_INST_MFSPR(SPRN_PIR):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_lwz(inst, magic_var(pir), inst_rt);
    break;
// Stores
    case KVM_INST_MTSPR(SPRN_SPRG0):
    kvm_patch_ins_std(inst, magic_var(sprg0), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SPRG1):
    kvm_patch_ins_std(inst, magic_var(sprg1), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SPRG2):
    kvm_patch_ins_std(inst, magic_var(sprg2), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SPRG3):
    kvm_patch_ins_std(inst, magic_var(sprg3), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SRR0):
    kvm_patch_ins_std(inst, magic_var(srr0), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SRR1):
    kvm_patch_ins_std(inst, magic_var(srr1), inst_rt);
    break;

    case KVM_INST_MTSPR(SPRN_DEAR):

    case KVM_INST_MTSPR(SPRN_DAR):

    kvm_patch_ins_std(inst, magic_var(dar), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_DSISR):
    kvm_patch_ins_stw(inst, magic_var(dsisr), inst_rt);
    break;

    case KVM_INST_MTSPR(SPRN_MAS0):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_stw(inst, magic_var(mas0), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_MAS1):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_stw(inst, magic_var(mas1), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_MAS2):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_std(inst, magic_var(mas2), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_MAS3):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_stw(inst, magic_var(mas7_3) + 4, inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_MAS4):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_stw(inst, magic_var(mas4), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_MAS6):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_stw(inst, magic_var(mas6), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_MAS7):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_stw(inst, magic_var(mas7_3), inst_rt);
    break;

    case KVM_INST_MTSPR(SPRN_SPRG4):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_std(inst, magic_var(sprg4), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SPRG5):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_std(inst, magic_var(sprg5), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SPRG6):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_std(inst, magic_var(sprg6), inst_rt);
    break;
    case KVM_INST_MTSPR(SPRN_SPRG7):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_std(inst, magic_var(sprg7), inst_rt);
    break;

    case KVM_INST_MTSPR(SPRN_ESR):
    if (features & KVM_MAGIC_FEAT_MAS0_TO_SPRG7)
    kvm_patch_ins_stw(inst, magic_var(esr), inst_rt);
    break;

// Nops
    case KVM_INST_TLBSYNC:
    kvm_patch_ins_nop(inst);
    break;
// Rewrites
    case KVM_INST_MTMSRD_L1:
    kvm_patch_ins_mtmsrd(inst, inst_rt);
    break;
    case KVM_INST_MTMSR:
    case KVM_INST_MTMSRD_L0:
    kvm_patch_ins_mtmsr(inst, inst_rt);
    break;

    case KVM_INST_WRTEE:
    kvm_patch_ins_wrtee(inst, inst_rt, 0);
    break;

    }

    switch (inst_no_rt & ~KVM_MASK_RB) {
    case KVM_INST_MTSRIN:
    if (features & KVM_MAGIC_FEAT_SR) {
    let mut inst_rb: u32 = _inst & KVM_MASK_RB;
    kvm_patch_ins_mtsrin(inst, inst_rt, inst_rb);
    }
    break;
    }

    switch (_inst) {
    case KVM_INST_WRTEEI_0:
    kvm_patch_ins_wrteei_0(inst);
    break;
    case KVM_INST_WRTEEI_1:
    kvm_patch_ins_wrtee(inst, 0, 1);
    break;
    }

    }
    extern u32 kvm_template_start[];
    extern u32 kvm_template_end[];
#[no_mangle]
unsafe extern "C" fn kvm_use_magic_page() -> void __init {
    static void __init kvm_use_magic_page(void)
    {
    u32 *p;
    u32 *start, *end;
    u32 features;
// Tell the host to map the magic page to -4096 on all CPUs
    on_each_cpu(kvm_map_magic_page, &features, 1);
// Quick self-test to see if the mapping works
    if (fault_in_readable((const char __user *)KVM_MAGIC_PAGE,
    sizeof(u32))) {
    kvm_patching_worked = false;
    return;
    }
// Now loop through all code and find instructions
    start = (void*)_stext;
    end = (void*)_etext;
//
// Being interrupted in the middle of patching would
// be bad for SPRG4-7, which KVM can't keep in sync
// with emulated accesses because reads don't trap.
//
    local_irq_disable();
    for (p = start; p < end; p++) {
// Avoid patching the template code
    if (p >= kvm_template_start && p < kvm_template_end) {
    p = kvm_template_end - 1;
    continue;
    }
    kvm_check_ins(p, features);
    }
    local_irq_enable();
    printk(KERN_INFO "KVM: Live patching for a fast VM %s\n",
    kvm_patching_worked ? "worked" : "failed");
    }
#[no_mangle]
unsafe extern "C" fn kvm_guest_init() -> int __init {
    static int __init kvm_guest_init(void)
    {
    if (!kvm_para_available())
    return 0;
    if (!epapr_paravirt_enabled)
    return 0;
    if (kvm_para_has_feature(KVM_FEATURE_MAGIC_PAGE))
    kvm_use_magic_page();

// Enable napping
    powersave_nap = 1;

    return 0;
    }
    postcore_initcall(kvm_guest_init);
