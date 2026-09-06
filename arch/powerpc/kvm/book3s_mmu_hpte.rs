//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_mmu_hpte.c
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

pub const PTE_SIZE: c_int = 12;
    static struct kmem_cache *hpte_cache;
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hash_pte(eaddr: u64) -> u64 {
    static inline u64 kvmppc_mmu_hash_pte(u64 eaddr)
    {
    return hash_64(eaddr >> PTE_SIZE, HPTEG_HASH_BITS_PTE);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hash_pte_long(eaddr: u64) -> u64 {
    static inline u64 kvmppc_mmu_hash_pte_long(u64 eaddr)
    {
    return hash_64((eaddr & 0x0ffff000) >> PTE_SIZE,
    HPTEG_HASH_BITS_PTE_LONG);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hash_vpte(vpage: u64) -> u64 {
    static inline u64 kvmppc_mmu_hash_vpte(u64 vpage)
    {
    return hash_64(vpage & 0xfffffffffULL, HPTEG_HASH_BITS_VPTE);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hash_vpte_long(vpage: u64) -> u64 {
    static inline u64 kvmppc_mmu_hash_vpte_long(u64 vpage)
    {
    return hash_64((vpage & 0xffffff000ULL) >> 12,
    HPTEG_HASH_BITS_VPTE_LONG);
    }

#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hash_vpte_64k(vpage: u64) -> u64 {
    static inline u64 kvmppc_mmu_hash_vpte_64k(u64 vpage)
    {
    return hash_64((vpage & 0xffffffff0ULL) >> 4,
    HPTEG_HASH_BITS_VPTE_64K);
    }

#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hpte_cache_map(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    void kvmppc_mmu_hpte_cache_map(struct kvm_vcpu *vcpu, struct hpte_cache *pte)
    {
    u64 index;
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    trace_kvm_book3s_mmu_map(pte);
    spin_lock(&vcpu3s.mmu_lock);
// Add to ePTE list
    index = kvmppc_mmu_hash_pte(pte.pte.eaddr);
    hlist_add_head_rcu(&pte.list_pte, &vcpu3s.hpte_hash_pte[index]);
// Add to ePTE_long list
    index = kvmppc_mmu_hash_pte_long(pte.pte.eaddr);
    hlist_add_head_rcu(&pte.list_pte_long,
    &vcpu3s.hpte_hash_pte_long[index]);
// Add to vPTE list
    index = kvmppc_mmu_hash_vpte(pte.pte.vpage);
    hlist_add_head_rcu(&pte.list_vpte, &vcpu3s.hpte_hash_vpte[index]);
// Add to vPTE_long list
    index = kvmppc_mmu_hash_vpte_long(pte.pte.vpage);
    hlist_add_head_rcu(&pte.list_vpte_long,
    &vcpu3s.hpte_hash_vpte_long[index]);

// Add to vPTE_64k list
    index = kvmppc_mmu_hash_vpte_64k(pte.pte.vpage);
    hlist_add_head_rcu(&pte.list_vpte_64k,
    &vcpu3s.hpte_hash_vpte_64k[index]);

    vcpu3s.hpte_cache_count++;
    spin_unlock(&vcpu3s.mmu_lock);
    }
#[no_mangle]
unsafe extern "C" fn invalidate_pte(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    static void invalidate_pte(struct kvm_vcpu *vcpu, struct hpte_cache *pte)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    trace_kvm_book3s_mmu_invalidate(pte);
// Different for 32 and 64 bit
    kvmppc_mmu_invalidate_pte(vcpu, pte);
    spin_lock(&vcpu3s.mmu_lock);
// pte already invalidated in between?
    if (hlist_unhashed(&pte.list_pte)) {
    spin_unlock(&vcpu3s.mmu_lock);
    return;
    }
    hlist_del_init_rcu(&pte.list_pte);
    hlist_del_init_rcu(&pte.list_pte_long);
    hlist_del_init_rcu(&pte.list_vpte);
    hlist_del_init_rcu(&pte.list_vpte_long);

    hlist_del_init_rcu(&pte.list_vpte_64k);

    vcpu3s.hpte_cache_count--;
    spin_unlock(&vcpu3s.mmu_lock);
    kfree_rcu(pte, rcu_head);
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_pte_flush_all(vcpu: *mut kvm_vcpu) {
    static void kvmppc_mmu_pte_flush_all(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hpte_cache *pte;
    int i;
    rcu_read_lock();
    for (i = 0; i < HPTEG_HASH_NUM_VPTE_LONG; i++) {
    struct hlist_head *list = &vcpu3s.hpte_hash_vpte_long[i];
    hlist_for_each_entry_rcu(pte, list, list_vpte_long)
    invalidate_pte(vcpu, pte);
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_pte_flush_page(vcpu: *mut kvm_vcpu, guest_ea: c_ulong) {
    static void kvmppc_mmu_pte_flush_page(struct kvm_vcpu *vcpu, ulong guest_ea)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hlist_head *list;
    struct hpte_cache *pte;
// Find the list of entries in the map
    list = &vcpu3s.hpte_hash_pte[kvmppc_mmu_hash_pte(guest_ea)];
    rcu_read_lock();
// Check the list for matching entries and invalidate
    hlist_for_each_entry_rcu(pte, list, list_pte)
    if ((pte.pte.eaddr & ~0xfffUL) == guest_ea)
    invalidate_pte(vcpu, pte);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_pte_flush_long(vcpu: *mut kvm_vcpu, guest_ea: c_ulong) {
    static void kvmppc_mmu_pte_flush_long(struct kvm_vcpu *vcpu, ulong guest_ea)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hlist_head *list;
    struct hpte_cache *pte;
// Find the list of entries in the map
    list = &vcpu3s.hpte_hash_pte_long[
    kvmppc_mmu_hash_pte_long(guest_ea)];
    rcu_read_lock();
// Check the list for matching entries and invalidate
    hlist_for_each_entry_rcu(pte, list, list_pte_long)
    if ((pte.pte.eaddr & 0x0ffff000UL) == guest_ea)
    invalidate_pte(vcpu, pte);
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_pte_flush(vcpu: *mut kvm_vcpu, guest_ea: c_ulong, ea_mask: c_ulong) {
    void kvmppc_mmu_pte_flush(struct kvm_vcpu *vcpu, ulong guest_ea, ulong ea_mask)
    {
    trace_kvm_book3s_mmu_flush("", vcpu, guest_ea, ea_mask);
    guest_ea &= ea_mask;
    switch (ea_mask) {
    case ~0xfffUL:
    kvmppc_mmu_pte_flush_page(vcpu, guest_ea);
    break;
    case 0x0ffff000:
    kvmppc_mmu_pte_flush_long(vcpu, guest_ea);
    break;
    case 0:
// Doing a complete flush -> start from scratch
    kvmppc_mmu_pte_flush_all(vcpu);
    break;
    default:
    WARN_ON(1);
    break;
    }
    }
// Flush with mask 0xfffffffff
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_pte_vflush_short(vcpu: *mut kvm_vcpu, guest_vp: u64) {
    static void kvmppc_mmu_pte_vflush_short(struct kvm_vcpu *vcpu, u64 guest_vp)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hlist_head *list;
    struct hpte_cache *pte;
    let mut vp_mask: u64 = 0xfffffffffULL;
    list = &vcpu3s.hpte_hash_vpte[kvmppc_mmu_hash_vpte(guest_vp)];
    rcu_read_lock();
// Check the list for matching entries and invalidate
    hlist_for_each_entry_rcu(pte, list, list_vpte)
    if ((pte.pte.vpage & vp_mask) == guest_vp)
    invalidate_pte(vcpu, pte);
    rcu_read_unlock();
    }

// Flush with mask 0xffffffff0
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_pte_vflush_64k(vcpu: *mut kvm_vcpu, guest_vp: u64) {
    static void kvmppc_mmu_pte_vflush_64k(struct kvm_vcpu *vcpu, u64 guest_vp)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hlist_head *list;
    struct hpte_cache *pte;
    let mut vp_mask: u64 = 0xffffffff0ULL;
    list = &vcpu3s.hpte_hash_vpte_64k[
    kvmppc_mmu_hash_vpte_64k(guest_vp)];
    rcu_read_lock();
// Check the list for matching entries and invalidate
    hlist_for_each_entry_rcu(pte, list, list_vpte_64k)
    if ((pte.pte.vpage & vp_mask) == guest_vp)
    invalidate_pte(vcpu, pte);
    rcu_read_unlock();
    }

// Flush with mask 0xffffff000
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_pte_vflush_long(vcpu: *mut kvm_vcpu, guest_vp: u64) {
    static void kvmppc_mmu_pte_vflush_long(struct kvm_vcpu *vcpu, u64 guest_vp)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hlist_head *list;
    struct hpte_cache *pte;
    let mut vp_mask: u64 = 0xffffff000ULL;
    list = &vcpu3s.hpte_hash_vpte_long[
    kvmppc_mmu_hash_vpte_long(guest_vp)];
    rcu_read_lock();
// Check the list for matching entries and invalidate
    hlist_for_each_entry_rcu(pte, list, list_vpte_long)
    if ((pte.pte.vpage & vp_mask) == guest_vp)
    invalidate_pte(vcpu, pte);
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_pte_vflush(vcpu: *mut kvm_vcpu, guest_vp: u64, vp_mask: u64) {
    void kvmppc_mmu_pte_vflush(struct kvm_vcpu *vcpu, u64 guest_vp, u64 vp_mask)
    {
    trace_kvm_book3s_mmu_flush("v", vcpu, guest_vp, vp_mask);
    guest_vp &= vp_mask;
    switch(vp_mask) {
    case 0xfffffffffULL:
    kvmppc_mmu_pte_vflush_short(vcpu, guest_vp);
    break;

    case 0xffffffff0ULL:
    kvmppc_mmu_pte_vflush_64k(vcpu, guest_vp);
    break;

    case 0xffffff000ULL:
    kvmppc_mmu_pte_vflush_long(vcpu, guest_vp);
    break;
    default:
    WARN_ON(1);
    return;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_pte_pflush(vcpu: *mut kvm_vcpu, pa_start: c_ulong, pa_end: c_ulong) {
    void kvmppc_mmu_pte_pflush(struct kvm_vcpu *vcpu, ulong pa_start, ulong pa_end)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hpte_cache *pte;
    int i;
    trace_kvm_book3s_mmu_flush("p", vcpu, pa_start, pa_end);
    rcu_read_lock();
    for (i = 0; i < HPTEG_HASH_NUM_VPTE_LONG; i++) {
    struct hlist_head *list = &vcpu3s.hpte_hash_vpte_long[i];
    hlist_for_each_entry_rcu(pte, list, list_vpte_long)
    if ((pte.pte.raddr >= pa_start) &&
    (pte.pte.raddr < pa_end))
    invalidate_pte(vcpu, pte);
    }
    rcu_read_unlock();
    }
    struct hpte_cache *kvmppc_mmu_hpte_cache_next(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
    struct hpte_cache *pte;
    if (vcpu3s.hpte_cache_count == HPTEG_CACHE_NUM)
    kvmppc_mmu_pte_flush_all(vcpu);
    pte = kmem_cache_zalloc(hpte_cache, GFP_KERNEL);
    return pte;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hpte_cache_free(pte: *mut hpte_cache) {
    void kvmppc_mmu_hpte_cache_free(struct hpte_cache *pte)
    {
    kmem_cache_free(hpte_cache, pte);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hpte_destroy(vcpu: *mut kvm_vcpu) {
    void kvmppc_mmu_hpte_destroy(struct kvm_vcpu *vcpu)
    {
    kvmppc_mmu_pte_flush(vcpu, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_hpte_init_hash(hash_list: *mut hlist_head, len: c_int) {
    static void kvmppc_mmu_hpte_init_hash(struct hlist_head *hash_list, int len)
    {
    int i;
    for (i = 0; i < len; i++)
    INIT_HLIST_HEAD(&hash_list[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hpte_init(vcpu: *mut kvm_vcpu) -> c_int {
    int kvmppc_mmu_hpte_init(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_vcpu_book3s *vcpu3s = to_book3s(vcpu);
// init hpte lookup hashes
    kvmppc_mmu_hpte_init_hash(vcpu3s.hpte_hash_pte,
    ARRAY_SIZE(vcpu3s.hpte_hash_pte));
    kvmppc_mmu_hpte_init_hash(vcpu3s.hpte_hash_pte_long,
    ARRAY_SIZE(vcpu3s.hpte_hash_pte_long));
    kvmppc_mmu_hpte_init_hash(vcpu3s.hpte_hash_vpte,
    ARRAY_SIZE(vcpu3s.hpte_hash_vpte));
    kvmppc_mmu_hpte_init_hash(vcpu3s.hpte_hash_vpte_long,
    ARRAY_SIZE(vcpu3s.hpte_hash_vpte_long));

    kvmppc_mmu_hpte_init_hash(vcpu3s.hpte_hash_vpte_64k,
    ARRAY_SIZE(vcpu3s.hpte_hash_vpte_64k));

    spin_lock_init(&vcpu3s.mmu_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hpte_sysinit() -> c_int {
    int kvmppc_mmu_hpte_sysinit(void)
    {
// init hpte slab cache
    hpte_cache = kmem_cache_create("kvm-spt", sizeof(struct hpte_cache),
    sizeof(struct hpte_cache), 0, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_hpte_sysexit() {
    void kvmppc_mmu_hpte_sysexit(void)
    {
    kmem_cache_destroy(hpte_cache);
    }
