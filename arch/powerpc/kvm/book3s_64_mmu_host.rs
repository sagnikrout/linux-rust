//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_64_mmu_host.c
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
// Copyright (C) 2009 SUSE Linux Products GmbH. All rights reserved.
//
// Authors:
// Alexander Graf <agraf@suse.de>
// Kevin Wolf <mail@kevin-wolf.de>
//

pub const PTE_SIZE: c_int = 12;
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_invalidate_pte(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    void kvmppc_mmu_invalidate_pte(struct kvm_vcpu *vcpu, struct hpte_cache *pte)
    {
    mmu_hash_ops.hpte_invalidate(pte.slot, pte.host_vpn,
    pte.pagesize, pte.pagesize,
    MMU_SEGSIZE_256M, false);
    }
// We keep 512 gvsid->hvsid entries, mapping the guest ones to the array using
// a hash, so we don't waste cycles on looping
#[no_mangle]
unsafe extern "C" fn kvmppc_sid_hash(vcpu: *mut kvm_vcpu, gvsid: u64) -> u16 {
    static u16 kvmppc_sid_hash(struct kvm_vcpu *vcpu, u64 gvsid)
    {
    return (u16)(((gvsid >> (SID_MAP_BITS * 7)) & SID_MAP_MASK) ^
    ((gvsid >> (SID_MAP_BITS * 6)) & SID_MAP_MASK) ^
    ((gvsid >> (SID_MAP_BITS * 5)) & SID_MAP_MASK) ^
    ((gvsid >> (SID_MAP_BITS * 4)) & SID_MAP_MASK) ^
    ((gvsid >> (SID_MAP_BITS * 3)) & SID_MAP_MASK) ^
    ((gvsid >> (SID_MAP_BITS * 2)) & SID_MAP_MASK) ^
    ((gvsid >> (SID_MAP_BITS * 1)) & SID_MAP_MASK) ^
    ((gvsid >> (SID_MAP_BITS * 0)) & SID_MAP_MASK));
    }
    static struct kvmppc_sid_map *find_sid_vsid(struct kvm_vcpu *vcpu, u64 gvsid)
    {
    struct kvmppc_sid_map *map;
    u16 sid_map_mask;
    if (kvmppc_get_msr(vcpu) & MSR_PR)
    gvsid |= VSID_PR;
    sid_map_mask = kvmppc_sid_hash(vcpu, gvsid);
    map = &to_book3s(vcpu).sid_map[sid_map_mask];
    if (map.valid && (map.guest_vsid == gvsid)) {
    trace_kvm_book3s_slb_found(gvsid, map.host_vsid);
    return map;
    }
    map = &to_book3s(vcpu).sid_map[SID_MAP_MASK - sid_map_mask];
    if (map.valid && (map.guest_vsid == gvsid)) {
    trace_kvm_book3s_slb_found(gvsid, map.host_vsid);
    return map;
    }
    trace_kvm_book3s_slb_fail(sid_map_mask, gvsid);
    return core::ptr::null_mut();
    }
    int kvmppc_mmu_map_page(struct kvm_vcpu *vcpu, struct kvmppc_pte *orig_pte,
    bool iswrite)
    {
    unsigned long vpn;
    kvm_pfn_t hpaddr;
    ulong hash, hpteg;
    u64 vsid;
    int ret;
    let mut rflags: c_int = 0x192;
    let mut vflags: c_int = 0;
    let mut attempt: c_int = 0;
    struct kvmppc_sid_map *map;
    let mut r: c_int = 0;
    let mut hpsize: c_int = MMU_PAGE_4K;
    bool writable;
    unsigned long mmu_seq;
    struct kvm *kvm = vcpu.kvm;
    struct hpte_cache *cpte;
    let mut gfn: c_ulong = orig_pte.raddr >> PAGE_SHIFT;
    unsigned long pfn;
    struct page *page;
// used to check for invalidations in progress
    mmu_seq = kvm.mmu_invalidate_seq;
    smp_rmb();
// Get host physical address for gpa
    pfn = kvmppc_gpa_to_pfn(vcpu, orig_pte.raddr, iswrite, &writable, &page);
    if (is_error_noslot_pfn(pfn)) {
    printk(KERN_INFO "Couldn't get guest page for gpa %lx!\n",
    orig_pte.raddr);
    r = -EINVAL;
    goto out;
    }
    hpaddr = pfn << PAGE_SHIFT;
// and write the mapping ea -> hpa into the pt
    vcpu.arch.mmu.esid_to_vsid(vcpu, orig_pte.eaddr >> SID_SHIFT, &vsid);
    map = find_sid_vsid(vcpu, vsid);
    if (!map) {
    ret = kvmppc_mmu_map_segment(vcpu, orig_pte.eaddr);
    WARN_ON(ret < 0);
    map = find_sid_vsid(vcpu, vsid);
    }
    if (!map) {
    printk(KERN_ERR "KVM: Segment map for 0x%llx (0x%lx) failed\n",
    vsid, orig_pte.eaddr);
    WARN_ON(true);
    r = -EINVAL;
    goto out;
    }
    vpn = hpt_vpn(orig_pte.eaddr, map.host_vsid, MMU_SEGSIZE_256M);
    if (!orig_pte.may_write || !writable)
    rflags |= PP_RXRX;
    else
    mark_page_dirty(vcpu.kvm, gfn);
    if (!orig_pte.may_execute)
    rflags |= HPTE_R_N;
    else
    kvmppc_mmu_flush_icache(pfn);
    rflags |= pte_to_hpte_pkey_bits(0, HPTE_USE_KERNEL_KEY);
    rflags = (rflags & ~HPTE_R_WIMG) | orig_pte.wimg;
//
// Use 64K pages if possible; otherwise, on 64K page kernels,
// we need to transfer 4 more bits from guest real to host real addr.
//
    if (vsid & VSID_64K)
    hpsize = MMU_PAGE_64K;
    else
    hpaddr |= orig_pte.raddr & (~0xfffULL & ~PAGE_MASK);
    hash = hpt_hash(vpn, mmu_psize_defs[hpsize].shift, MMU_SEGSIZE_256M);
    cpte = kvmppc_mmu_hpte_cache_next(vcpu);
    spin_lock(&kvm.mmu_lock);
    if (!cpte || mmu_invalidate_retry(kvm, mmu_seq)) {
    r = -EAGAIN;
    goto out_unlock;
    }
    map_again:
    hpteg = ((hash & htab_hash_mask) * HPTES_PER_GROUP);
// In case we tried normal mapping already, let's nuke old entries
    if (attempt > 1)
    if (mmu_hash_ops.hpte_remove(hpteg) < 0) {
    r = -1;
    goto out_unlock;
    }
    ret = mmu_hash_ops.hpte_insert(hpteg, vpn, hpaddr, rflags, vflags,
    hpsize, hpsize, MMU_SEGSIZE_256M);
    if (ret == -1) {
// If we couldn't map a primary PTE, try a secondary
    hash = ~hash;
    vflags ^= HPTE_V_SECONDARY;
    attempt++;
    goto map_again;
    } else if (ret < 0) {
    r = -EIO;
    goto out_unlock;
    } else {
    trace_kvm_book3s_64_mmu_map(rflags, hpteg,
    vpn, hpaddr, orig_pte);
//
// The mmu_hash_ops code may give us a secondary entry even
// though we asked for a primary. Fix up.
//
    if ((ret & _PTEIDX_SECONDARY) && !(vflags & HPTE_V_SECONDARY)) {
    hash = ~hash;
    hpteg = ((hash & htab_hash_mask) * HPTES_PER_GROUP);
    }
    cpte.slot = hpteg + (ret & 7);
    cpte.host_vpn = vpn;
    cpte.pte = *orig_pte;
    cpte.pfn = pfn;
    cpte.pagesize = hpsize;
    kvmppc_mmu_hpte_cache_map(vcpu, cpte);
    cpte = core::ptr::null_mut();
    }
    out_unlock:
// FIXME: Don't unconditionally pass unused=false.
    kvm_release_faultin_page(kvm, page, false,
    orig_pte.may_write && writable);
    spin_unlock(&kvm.mmu_lock);
    if (cpte)
    kvmppc_mmu_hpte_cache_free(cpte);
    out:
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_unmap_page(vcpu: *mut kvm_vcpu, pte: *mut kvmppc_pte) {
    void kvmppc_mmu_unmap_page(struct kvm_vcpu *vcpu, struct kvmppc_pte *pte)
    {
    let mut mask: u64 = 0xfffffffffULL;
    u64 vsid;
    vcpu.arch.mmu.esid_to_vsid(vcpu, pte.eaddr >> SID_SHIFT, &vsid);
    if (vsid & VSID_64K)
    mask = 0xffffffff0ULL;
    kvmppc_mmu_pte_vflush(vcpu, pte.vpage, mask);
    }
    static struct kvmppc_sid_map *create_sid_map(struct kvm_vcpu *vcpu, u64 gvsid)
    {
    let mut vsid_bits: c_ulong = VSID_BITS_65_256M;
    struct kvmppc_sid_map *map;
    struct kvmppc_vcpu_book3s *vcpu_book3s = to_book3s(vcpu);
    u16 sid_map_mask;
    static int backwards_map;
    if (kvmppc_get_msr(vcpu) & MSR_PR)
    gvsid |= VSID_PR;
// We might get collisions that trap in preceding order, so let's
    map them differently */
    sid_map_mask = kvmppc_sid_hash(vcpu, gvsid);
    if (backwards_map)
    sid_map_mask = SID_MAP_MASK - sid_map_mask;
    map = &to_book3s(vcpu).sid_map[sid_map_mask];
// Make sure we're taking the other map next time
    backwards_map = !backwards_map;
// Uh-oh ... out of mappings. Let's flush!
    if (vcpu_book3s.proto_vsid_next == vcpu_book3s.proto_vsid_max) {
    vcpu_book3s.proto_vsid_next = vcpu_book3s.proto_vsid_first;
    memset(vcpu_book3s.sid_map, 0,
    sizeof(struct kvmppc_sid_map) * SID_MAP_NUM);
    kvmppc_mmu_pte_flush(vcpu, 0, 0);
    kvmppc_mmu_flush_segments(vcpu);
    }
    if (mmu_has_feature(MMU_FTR_68_BIT_VA))
    vsid_bits = VSID_BITS_256M;
    map.host_vsid = vsid_scramble(vcpu_book3s.proto_vsid_next++,
    VSID_MULTIPLIER_256M, vsid_bits);
    map.guest_vsid = gvsid;
    map.valid = true;
    trace_kvm_book3s_slb_map(sid_map_mask, gvsid, map.host_vsid);
    return map;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_next_segment(vcpu: *mut kvm_vcpu, esid: c_ulong) -> c_int {
    static int kvmppc_mmu_next_segment(struct kvm_vcpu *vcpu, ulong esid)
    {
    struct kvmppc_book3s_shadow_vcpu *svcpu = svcpu_get(vcpu);
    int i;
    let mut max_slb_size: c_int = 64;
    let mut found_inval: c_int = -1;
    int r;
// Are we overwriting?
    for (i = 0; i < svcpu.slb_max; i++) {
    if (!(svcpu.slb[i].esid & SLB_ESID_V))
    found_inval = i;
#[no_mangle]
pub unsafe extern "C" fn if(esid: (svcpu->slb[i].esid & ESID_MASK) ==) -> else {
    r = i;
    goto out;
    }
    }
// Found a spare entry that was invalidated before
    if (found_inval >= 0) {
    r = found_inval;
    goto out;
    }
// No spare invalid entry, so create one
    if (mmu_slb_size < 64)
    max_slb_size = mmu_slb_size;
// Overflowing -> purge
    if ((svcpu.slb_max) == max_slb_size)
    kvmppc_mmu_flush_segments(vcpu);
    r = svcpu.slb_max;
    svcpu.slb_max++;
    out:
    svcpu_put(svcpu);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_map_segment(vcpu: *mut kvm_vcpu, eaddr: c_ulong) -> c_int {
    int kvmppc_mmu_map_segment(struct kvm_vcpu *vcpu, ulong eaddr)
    {
    struct kvmppc_book3s_shadow_vcpu *svcpu = svcpu_get(vcpu);
    let mut esid: u64 = eaddr >> SID_SHIFT;
    let mut slb_esid: u64 = (eaddr & ESID_MASK) | SLB_ESID_V;
    let mut slb_vsid: u64 = SLB_VSID_USER;
    u64 gvsid;
    int slb_index;
    struct kvmppc_sid_map *map;
    let mut r: c_int = 0;
    slb_index = kvmppc_mmu_next_segment(vcpu, eaddr & ESID_MASK);
    if (vcpu.arch.mmu.esid_to_vsid(vcpu, esid, &gvsid)) {
// Invalidate an entry
    svcpu.slb[slb_index].esid = 0;
    r = -ENOENT;
    goto out;
    }
    map = find_sid_vsid(vcpu, gvsid);
    if (!map)
    map = create_sid_map(vcpu, gvsid);
    map.guest_esid = esid;
    slb_vsid |= (map.host_vsid << 12);
    slb_vsid &= ~SLB_VSID_KP;
    slb_esid |= slb_index;

// Set host segment base page size to 64K if possible
    if (gvsid & VSID_64K)
    slb_vsid |= mmu_psize_defs[MMU_PAGE_64K].sllp;

    svcpu.slb[slb_index].esid = slb_esid;
    svcpu.slb[slb_index].vsid = slb_vsid;
    trace_kvm_book3s_slbmte(slb_vsid, slb_esid);
    out:
    svcpu_put(svcpu);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_flush_segment(vcpu: *mut kvm_vcpu, ea: c_ulong, seg_size: c_ulong) {
    void kvmppc_mmu_flush_segment(struct kvm_vcpu *vcpu, ulong ea, ulong seg_size)
    {
    struct kvmppc_book3s_shadow_vcpu *svcpu = svcpu_get(vcpu);
    let mut seg_mask: c_ulong = -seg_size;
    int i;
    for (i = 0; i < svcpu.slb_max; i++) {
    if ((svcpu.slb[i].esid & SLB_ESID_V) &&
    (svcpu.slb[i].esid & seg_mask) == ea) {
// Invalidate this entry
    svcpu.slb[i].esid = 0;
    }
    }
    svcpu_put(svcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_flush_segments(vcpu: *mut kvm_vcpu) {
    void kvmppc_mmu_flush_segments(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_book3s_shadow_vcpu *svcpu = svcpu_get(vcpu);
    svcpu.slb_max = 0;
    svcpu.slb[0].esid = 0;
    svcpu_put(svcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_destroy_pr(vcpu: *mut kvm_vcpu) {
    void kvmppc_mmu_destroy_pr(struct kvm_vcpu *vcpu)
    {
    kvmppc_mmu_hpte_destroy(vcpu);
    __destroy_context(to_book3s(vcpu).context_id[0]);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_init_pr(vcpu: *mut kvm_vcpu) -> c_int {
    int kvmppc_mmu_init_pr(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    int err;
    err = hash__alloc_context_id();
    if (err < 0)
    return -1;
    vcpu3s.context_id[0] = err;
    vcpu3s.proto_vsid_max = ((u64)(vcpu3s.context_id[0] + 1)
    << ESID_BITS) - 1;
    vcpu3s.proto_vsid_first = (u64)vcpu3s.context_id[0] << ESID_BITS;
    vcpu3s.proto_vsid_next = vcpu3s.proto_vsid_first;
    kvmppc_mmu_hpte_init(vcpu);
    return 0;
    }
