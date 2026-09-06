//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_32_mmu_host.c
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
//
// Authors:
// Alexander Graf <agraf@suse.de>
//

// #define DEBUG_MMU
// #define DEBUG_SR

    static ulong htab;
    static u32 htabmask;
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_invalidate_pte(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    void kvmppc_mmu_invalidate_pte(struct kvm_vcpu *vcpu, struct hpte_cache *pte)
    {
    volatile u32 *pteg;
// Remove from host HTAB
    pteg = (u32*)pte.slot;
    pteg[0] = 0;
// And make sure it's gone from the TLB too
    asm volatile ("sync");
    asm volatile ("tlbie %0" : : "r" (pte.pte.eaddr) : "memory");
    asm volatile ("sync");
    asm volatile ("tlbsync");
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
    if (map.guest_vsid == gvsid) {
    dprintk_sr("SR: Searching 0x%llx . 0x%llx\n",
    gvsid, map.host_vsid);
    return map;
    }
    map = &to_book3s(vcpu).sid_map[SID_MAP_MASK - sid_map_mask];
    if (map.guest_vsid == gvsid) {
    dprintk_sr("SR: Searching 0x%llx . 0x%llx\n",
    gvsid, map.host_vsid);
    return map;
    }
    dprintk_sr("SR: Searching 0x%llx . not found\n", gvsid);
    return core::ptr::null_mut();
    }
    static u32 *kvmppc_mmu_get_pteg(struct kvm_vcpu *vcpu, u32 vsid, u32 eaddr,
    bool primary)
    {
    u32 page, hash;
    let mut pteg: c_ulong = htab;
    page = (eaddr & ~ESID_MASK) >> 12;
    hash = ((vsid ^ page) << 6);
    if (!primary)
    hash = ~hash;
    hash &= htabmask;
    pteg |= hash;
    dprintk_mmu("htab: %lx | hash: %x | htabmask: %x | pteg: %lx\n",
    htab, hash, htabmask, pteg);
    return (u32*)pteg;
    }
    int kvmppc_mmu_map_page(struct kvm_vcpu *vcpu, struct kvmppc_pte *orig_pte,
    bool iswrite)
    {
    struct page *page;
    kvm_pfn_t hpaddr;
    u64 vpn;
    u64 vsid;
    struct kvmppc_sid_map *map;
    volatile u32 *pteg;
    let mut eaddr: u32 = orig_pte.eaddr;
    u32 pteg0, pteg1;
    let mut rr: register int = 0;
    let mut primary: bool = false;
    let mut evict: bool = false;
    struct hpte_cache *pte;
    let mut r: c_int = 0;
    bool writable;
// Get host physical address for gpa
    hpaddr = kvmppc_gpa_to_pfn(vcpu, orig_pte.raddr, iswrite, &writable, &page);
    if (is_error_noslot_pfn(hpaddr)) {
    printk(KERN_INFO "Couldn't get guest page for gpa %lx!\n",
    orig_pte.raddr);
    r = -EINVAL;
    goto out;
    }
    hpaddr <<= PAGE_SHIFT;
// and write the mapping ea -> hpa into the pt
    vcpu.arch.mmu.esid_to_vsid(vcpu, orig_pte.eaddr >> SID_SHIFT, &vsid);
    map = find_sid_vsid(vcpu, vsid);
    if (!map) {
    kvmppc_mmu_map_segment(vcpu, eaddr);
    map = find_sid_vsid(vcpu, vsid);
    }
    BUG_ON(!map);
    vsid = map.host_vsid;
    vpn = (vsid << (SID_SHIFT - VPN_SHIFT)) |
    ((eaddr & ~ESID_MASK) >> VPN_SHIFT);
    next_pteg:
    if (rr == 16) {
    primary = !primary;
    evict = true;
    rr = 0;
    }
    pteg = kvmppc_mmu_get_pteg(vcpu, vsid, eaddr, primary);
// not evicting yet
    if (!evict && (pteg[rr] & PTE_V)) {
    rr += 2;
    goto next_pteg;
    }
    dprintk_mmu("KVM: old PTEG: %p (%d)\n", pteg, rr);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[0], pteg[1]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[2], pteg[3]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[4], pteg[5]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[6], pteg[7]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[8], pteg[9]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[10], pteg[11]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[12], pteg[13]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[14], pteg[15]);
    pteg0 = ((eaddr & 0x0fffffff) >> 22) | (vsid << 7) | PTE_V |
    (primary ? 0 : PTE_SEC);
    pteg1 = hpaddr | PTE_M | PTE_R | PTE_C;
    if (orig_pte.may_write && writable) {
    pteg1 |= PP_RWRW;
    mark_page_dirty(vcpu.kvm, orig_pte.raddr >> PAGE_SHIFT);
    } else {
    pteg1 |= PP_RWRX;
    }
    if (orig_pte.may_execute)
    kvmppc_mmu_flush_icache(hpaddr >> PAGE_SHIFT);
    local_irq_disable();
    if (pteg[rr]) {
    pteg[rr] = 0;
    asm volatile ("sync");
    }
    pteg[rr + 1] = pteg1;
    pteg[rr] = pteg0;
    asm volatile ("sync");
    local_irq_enable();
    dprintk_mmu("KVM: new PTEG: %p\n", pteg);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[0], pteg[1]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[2], pteg[3]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[4], pteg[5]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[6], pteg[7]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[8], pteg[9]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[10], pteg[11]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[12], pteg[13]);
    dprintk_mmu("KVM:   %08x - %08x\n", pteg[14], pteg[15]);
// Now tell our Shadow PTE code about the new page
    pte = kvmppc_mmu_hpte_cache_next(vcpu);
    if (!pte) {
    kvm_release_page_unused(page);
    r = -EAGAIN;
    goto out;
    }
    dprintk_mmu("KVM: %c%c Map 0x%llx: [%lx] 0x%llx (0x%llx) . %lx\n",
    orig_pte.may_write ? 'w' : '-',
    orig_pte.may_execute ? 'x' : '-',
    orig_pte.eaddr, (ulong)pteg, vpn,
    orig_pte.vpage, hpaddr);
    pte.slot = (ulong)&pteg[rr];
    pte.host_vpn = vpn;
    pte.pte = *orig_pte;
    pte.pfn = hpaddr >> PAGE_SHIFT;
    kvmppc_mmu_hpte_cache_map(vcpu, pte);
    kvm_release_page_clean(page);
    out:
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_unmap_page(vcpu: *mut kvm_vcpu, pte: *mut kvmppc_pte) {
    void kvmppc_mmu_unmap_page(struct kvm_vcpu *vcpu, struct kvmppc_pte *pte)
    {
    kvmppc_mmu_pte_vflush(vcpu, pte.vpage, 0xfffffffffULL);
    }
    static struct kvmppc_sid_map *create_sid_map(struct kvm_vcpu *vcpu, u64 gvsid)
    {
    struct kvmppc_sid_map *map;
    struct kvmppc_vcpu_book3s *vcpu_book3s = to_book3s(vcpu);
    u16 sid_map_mask;
    let mut backwards_map: static int = 0;
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
    if (vcpu_book3s.vsid_next >= VSID_POOL_SIZE) {
    vcpu_book3s.vsid_next = 0;
    memset(vcpu_book3s.sid_map, 0,
    sizeof(struct kvmppc_sid_map) * SID_MAP_NUM);
    kvmppc_mmu_pte_flush(vcpu, 0, 0);
    kvmppc_mmu_flush_segments(vcpu);
    }
    map.host_vsid = vcpu_book3s.vsid_pool[vcpu_book3s.vsid_next];
    vcpu_book3s.vsid_next++;
    map.guest_vsid = gvsid;
    map.valid = true;
    return map;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_map_segment(vcpu: *mut kvm_vcpu, eaddr: c_ulong) -> c_int {
    int kvmppc_mmu_map_segment(struct kvm_vcpu *vcpu, ulong eaddr)
    {
    let mut esid: u32 = eaddr >> SID_SHIFT;
    u64 gvsid;
    u32 sr;
    struct kvmppc_sid_map *map;
    struct kvmppc_book3s_shadow_vcpu *svcpu = svcpu_get(vcpu);
    let mut r: c_int = 0;
    if (vcpu.arch.mmu.esid_to_vsid(vcpu, esid, &gvsid)) {
// Invalidate an entry
    svcpu.sr[esid] = SR_INVALID;
    r = -ENOENT;
    goto out;
    }
    map = find_sid_vsid(vcpu, gvsid);
    if (!map)
    map = create_sid_map(vcpu, gvsid);
    map.guest_esid = esid;
    sr = map.host_vsid | SR_KP;
    svcpu.sr[esid] = sr;
    dprintk_sr("MMU: mtsr %d, 0x%x\n", esid, sr);
    out:
    svcpu_put(svcpu);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_flush_segments(vcpu: *mut kvm_vcpu) {
    void kvmppc_mmu_flush_segments(struct kvm_vcpu *vcpu)
    {
    int i;
    struct kvmppc_book3s_shadow_vcpu *svcpu = svcpu_get(vcpu);
    dprintk_sr("MMU: flushing all segments (%d)\n", ARRAY_SIZE(svcpu.sr));
    for (i = 0; i < ARRAY_SIZE(svcpu.sr); i++)
    svcpu.sr[i] = SR_INVALID;
    svcpu_put(svcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_destroy_pr(vcpu: *mut kvm_vcpu) {
    void kvmppc_mmu_destroy_pr(struct kvm_vcpu *vcpu)
    {
    int i;
    kvmppc_mmu_hpte_destroy(vcpu);
    preempt_disable();
    for (i = 0; i < SID_CONTEXTS; i++)
    __destroy_context(to_book3s(vcpu).context_id[i]);
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_init_pr(vcpu: *mut kvm_vcpu) -> c_int {
    int kvmppc_mmu_init_pr(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    int err;
    ulong sdr1;
    int i;
    int j;
    for (i = 0; i < SID_CONTEXTS; i++) {
    err = __init_new_context();
    if (err < 0)
    goto init_fail;
    vcpu3s.context_id[i] = err;
// Remember context id for this combination
    for (j = 0; j < 16; j++)
    vcpu3s.vsid_pool[(i * 16) + j] = CTX_TO_VSID(err, j);
    }
    vcpu3s.vsid_next = 0;
// Remember where the HTAB is
    asm ( "mfsdr1 %0" : "=r"(sdr1) );
    htabmask = ((sdr1 & 0x1FF) << 16) | 0xFFC0;
    htab = (ulong)__va(sdr1 & 0xffff0000);
    kvmppc_mmu_hpte_init(vcpu);
    return 0;
    init_fail:
    for (j = 0; j < i; j++) {
    if (!vcpu3s.context_id[j])
    continue;
    __destroy_context(to_book3s(vcpu).context_id[j]);
    }
    return -1;
    }
