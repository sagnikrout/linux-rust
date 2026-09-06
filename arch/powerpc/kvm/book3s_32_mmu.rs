//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_32_mmu.c
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
// Copyright SUSE Linux Products GmbH 2009
//
// Authors: Alexander Graf <agraf@suse.de>
//

// #define DEBUG_MMU
// #define DEBUG_MMU_PTE
// #define DEBUG_MMU_PTE_IP 0xfff14c40

pub const PTEG_FLAG_ACCESSED: c_uint = 0x00000100;
pub const PTEG_FLAG_DIRTY: c_uint = 0x00000080;

pub const SID_SHIFT: c_int = 28;

#[no_mangle]
pub unsafe extern "C" fn check_debug_ip(vcpu: *mut kvm_vcpu) -> bool {
    static inline bool check_debug_ip(struct kvm_vcpu *vcpu)
    {

    return vcpu.arch.regs.nip == DEBUG_MMU_PTE_IP;

    return true;

    }
#[no_mangle]
pub unsafe extern "C" fn sr_vsid(sr_raw: u32) -> u32 {
    static inline u32 sr_vsid(u32 sr_raw)
    {
    return sr_raw & 0x0fffffff;
    }
#[no_mangle]
pub unsafe extern "C" fn sr_valid(sr_raw: u32) -> bool {
    static inline bool sr_valid(u32 sr_raw)
    {
    return (sr_raw & 0x80000000) ? false : true;
    }
#[no_mangle]
pub unsafe extern "C" fn sr_ks(sr_raw: u32) -> bool {
    static inline bool sr_ks(u32 sr_raw)
    {
    return (sr_raw & 0x40000000) ? true: false;
    }
#[no_mangle]
pub unsafe extern "C" fn sr_kp(sr_raw: u32) -> bool {
    static inline bool sr_kp(u32 sr_raw)
    {
    return (sr_raw & 0x20000000) ? true: false;
    }
    static int kvmppc_mmu_book3s_32_xlate_bat(struct kvm_vcpu *vcpu, gva_t eaddr,
    struct kvmppc_pte *pte, bool data,
    bool iswrite);
    static int kvmppc_mmu_book3s_32_esid_to_vsid(struct kvm_vcpu *vcpu, ulong esid,
    u64 *vsid);
#[no_mangle]
unsafe extern "C" fn find_sr(vcpu: *mut kvm_vcpu, eaddr: gva_t) -> u32 {
    static u32 find_sr(struct kvm_vcpu *vcpu, gva_t eaddr)
    {
    return kvmppc_get_sr(vcpu, (eaddr >> 28) & 0xf);
    }
    static u64 kvmppc_mmu_book3s_32_ea_to_vp(struct kvm_vcpu *vcpu, gva_t eaddr,
    bool data)
    {
    u64 vsid;
    struct kvmppc_pte pte;
    if (!kvmppc_mmu_book3s_32_xlate_bat(vcpu, eaddr, &pte, data, false))
    return pte.vpage;
    kvmppc_mmu_book3s_32_esid_to_vsid(vcpu, eaddr >> SID_SHIFT, &vsid);
    return (((u64)eaddr >> 12) & 0xffff) | (vsid << 16);
    }
    static hva_t kvmppc_mmu_book3s_32_get_pteg(struct kvm_vcpu *vcpu,
    u32 sre, gva_t eaddr,
    bool primary)
    {
    struct kvmppc_vcpu_book3s *vcpu_book3s = to_book3s(vcpu);
    u32 page, hash, pteg, htabmask;
    hva_t r;
    page = (eaddr & 0x0FFFFFFF) >> 12;
    htabmask = ((vcpu_book3s.sdr1 & 0x1FF) << 16) | 0xFFC0;
    hash = ((sr_vsid(sre) ^ page) << 6);
    if (!primary)
    hash = ~hash;
    hash &= htabmask;
    pteg = (vcpu_book3s.sdr1 & 0xffff0000) | hash;
    dprintk("MMU: pc=0x%lx eaddr=0x%lx sdr1=0x%llx pteg=0x%x vsid=0x%x\n",
    kvmppc_get_pc(vcpu), eaddr, vcpu_book3s.sdr1, pteg,
    sr_vsid(sre));
    r = gfn_to_hva(vcpu.kvm, pteg >> PAGE_SHIFT);
    if (kvm_is_error_hva(r))
    return r;
    return r | (pteg & ~PAGE_MASK);
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_book3s_32_get_ptem(sre: u32, eaddr: gva_t, primary: bool) -> u32 {
    static u32 kvmppc_mmu_book3s_32_get_ptem(u32 sre, gva_t eaddr, bool primary)
    {
    return ((eaddr & 0x0fffffff) >> 22) | (sr_vsid(sre) << 7) |
    (primary ? 0 : 0x40) | 0x80000000;
    }
    static int kvmppc_mmu_book3s_32_xlate_bat(struct kvm_vcpu *vcpu, gva_t eaddr,
    struct kvmppc_pte *pte, bool data,
    bool iswrite)
    {
    struct kvmppc_vcpu_book3s *vcpu_book3s = to_book3s(vcpu);
    struct kvmppc_bat *bat;
    int i;
    for (i = 0; i < 8; i++) {
    if (data)
    bat = &vcpu_book3s.dbat[i];
    else
    bat = &vcpu_book3s.ibat[i];
    if (kvmppc_get_msr(vcpu) & MSR_PR) {
    if (!bat.vp)
    continue;
    } else {
    if (!bat.vs)
    continue;
    }
    if (check_debug_ip(vcpu))
    {
    dprintk_pte("%cBAT %02d: 0x%lx - 0x%x (0x%x)\n",
    data ? 'd' : 'i', i, eaddr, bat.bepi,
    bat.bepi_mask);
    }
    if ((eaddr & bat.bepi_mask) == bat.bepi) {
    u64 vsid;
    kvmppc_mmu_book3s_32_esid_to_vsid(vcpu,
    eaddr >> SID_SHIFT, &vsid);
    vsid <<= 16;
    pte.vpage = (((u64)eaddr >> 12) & 0xffff) | vsid;
    pte.raddr = bat.brpn | (eaddr & ~bat.bepi_mask);
    pte.may_read = bat.pp;
    pte.may_write = bat.pp > 1;
    pte.may_execute = true;
    if (!pte.may_read) {
    printk(KERN_INFO "BAT is not readable!\n");
    continue;
    }
    if (iswrite && !pte.may_write) {
    dprintk_pte("BAT is read-only!\n");
    continue;
    }
    return 0;
    }
    }
    return -ENOENT;
    }
    static int kvmppc_mmu_book3s_32_xlate_pte(struct kvm_vcpu *vcpu, gva_t eaddr,
    struct kvmppc_pte *pte, bool data,
    bool iswrite, bool primary)
    {
    u32 sre;
    hva_t ptegp;
    u32 pteg[16];
    u32 pte0, pte1;
    let mut ptem: u32 = 0;
    int i;
    let mut found: c_int = 0;
    sre = find_sr(vcpu, eaddr);
    dprintk_pte("SR 0x%lx: vsid=0x%x, raw=0x%x\n", eaddr >> 28,
    sr_vsid(sre), sre);
    pte.vpage = kvmppc_mmu_book3s_32_ea_to_vp(vcpu, eaddr, data);
    ptegp = kvmppc_mmu_book3s_32_get_pteg(vcpu, sre, eaddr, primary);
    if (kvm_is_error_hva(ptegp)) {
    printk(KERN_INFO "KVM: Invalid PTEG!\n");
    goto no_page_found;
    }
    ptem = kvmppc_mmu_book3s_32_get_ptem(sre, eaddr, primary);
    if(copy_from_user(pteg, (void __user *)ptegp, sizeof(pteg))) {
    printk_ratelimited(KERN_ERR
    "KVM: Can't copy data from 0x%lx!\n", ptegp);
    goto no_page_found;
    }
    for (i=0; i<16; i+=2) {
    pte0 = be32_to_cpu(pteg[i]);
    pte1 = be32_to_cpu(pteg[i + 1]);
    if (ptem == pte0) {
    u8 pp;
    pte.raddr = (pte1 & ~(0xFFFULL)) | (eaddr & 0xFFF);
    pp = pte1 & 3;
    if ((sr_kp(sre) &&  (kvmppc_get_msr(vcpu) & MSR_PR)) ||
    (sr_ks(sre) && !(kvmppc_get_msr(vcpu) & MSR_PR)))
    pp |= 4;
    pte.may_write = false;
    pte.may_read = false;
    pte.may_execute = true;
    switch (pp) {
    case 0:
    case 1:
    case 2:
    case 6:
    pte.may_write = true;
    fallthrough;
    case 3:
    case 5:
    case 7:
    pte.may_read = true;
    break;
    }
    dprintk_pte("MMU: Found PTE . %x %x - %x\n",
    pte0, pte1, pp);
    found = 1;
    break;
    }
    }
// Update PTE C and A bits, so the guest's swapper knows we used the
    page */
    if (found) {
    let mut pte_r: u32 = pte1;
    char __user *addr = (char __user *) (ptegp + (i+1) * sizeof(u32));
//
// Use single-byte writes to update the HPTE, to
// conform to what real hardware does.
//
    if (pte.may_read && !(pte_r & PTEG_FLAG_ACCESSED)) {
    pte_r |= PTEG_FLAG_ACCESSED;
    put_user(pte_r >> 8, addr + 2);
    }
    if (iswrite && pte.may_write && !(pte_r & PTEG_FLAG_DIRTY)) {
    pte_r |= PTEG_FLAG_DIRTY;
    put_user(pte_r, addr + 3);
    }
    if (!pte.may_read || (iswrite && !pte.may_write))
    return -EPERM;
    return 0;
    }
    no_page_found:
    if (check_debug_ip(vcpu)) {
    dprintk_pte("KVM MMU: No PTE found (sdr1=0x%llx ptegp=0x%lx)\n",
    to_book3s(vcpu).sdr1, ptegp);
    for (i=0; i<16; i+=2) {
    dprintk_pte("   %02d: 0x%x - 0x%x (0x%x)\n",
    i, be32_to_cpu(pteg[i]),
    be32_to_cpu(pteg[i+1]), ptem);
    }
    }
    return -ENOENT;
    }
    static int kvmppc_mmu_book3s_32_xlate(struct kvm_vcpu *vcpu, gva_t eaddr,
    struct kvmppc_pte *pte, bool data,
    bool iswrite)
    {
    int r;
    let mut mp_ea: c_ulong = vcpu.arch.magic_page_ea;
    pte.eaddr = eaddr;
    pte.page_size = MMU_PAGE_4K;
// Magic page override
    if (unlikely(mp_ea) &&
    unlikely((eaddr & ~0xfffULL) == (mp_ea & ~0xfffULL)) &&
    !(kvmppc_get_msr(vcpu) & MSR_PR)) {
    pte.vpage = kvmppc_mmu_book3s_32_ea_to_vp(vcpu, eaddr, data);
    pte.raddr = vcpu.arch.magic_page_pa | (pte.raddr & 0xfff);
    pte.raddr &= KVM_PAM;
    pte.may_execute = true;
    pte.may_read = true;
    pte.may_write = true;
    return 0;
    }
    r = kvmppc_mmu_book3s_32_xlate_bat(vcpu, eaddr, pte, data, iswrite);
    if (r < 0)
    r = kvmppc_mmu_book3s_32_xlate_pte(vcpu, eaddr, pte,
    data, iswrite, true);
    if (r == -ENOENT)
    r = kvmppc_mmu_book3s_32_xlate_pte(vcpu, eaddr, pte,
    data, iswrite, false);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_book3s_32_mfsrin(vcpu: *mut kvm_vcpu, srnum: u32) -> u32 {
    static u32 kvmppc_mmu_book3s_32_mfsrin(struct kvm_vcpu *vcpu, u32 srnum)
    {
    return kvmppc_get_sr(vcpu, srnum);
    }
    static void kvmppc_mmu_book3s_32_mtsrin(struct kvm_vcpu *vcpu, u32 srnum,
    ulong value)
    {
    kvmppc_set_sr(vcpu, srnum, value);
    kvmppc_mmu_map_segment(vcpu, srnum << SID_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_book3s_32_tlbie(vcpu: *mut kvm_vcpu, ea: c_ulong, large: bool) {
    static void kvmppc_mmu_book3s_32_tlbie(struct kvm_vcpu *vcpu, ulong ea, bool large)
    {
    unsigned long i;
    struct kvm_vcpu *v;
// flush this VA on all cpus
    kvm_for_each_vcpu(i, v, vcpu.kvm)
    kvmppc_mmu_pte_flush(v, ea, 0x0FFFF000);
    }
    static int kvmppc_mmu_book3s_32_esid_to_vsid(struct kvm_vcpu *vcpu, ulong esid,
    u64 *vsid)
    {
    let mut ea: c_ulong = esid << SID_SHIFT;
    u32 sr;
    let mut gvsid: u64 = esid;
    let mut msr: u64 = kvmppc_get_msr(vcpu);
    if (msr & (MSR_DR|MSR_IR)) {
    sr = find_sr(vcpu, ea);
    if (sr_valid(sr))
    gvsid = sr_vsid(sr);
    }
// In case we only have one of MSR_IR or MSR_DR set, let's put
    that in the real-mode context (and hope RM doesn't access
    high memory) */
    switch (msr & (MSR_DR|MSR_IR)) {
    case 0:
// vsid = VSID_REAL | esid;
    break;
    case MSR_IR:
// vsid = VSID_REAL_IR | gvsid;
    break;
    case MSR_DR:
// vsid = VSID_REAL_DR | gvsid;
    break;
    case MSR_DR|MSR_IR:
    if (sr_valid(sr))
// vsid = sr_vsid(sr);
    else
// vsid = VSID_BAT | gvsid;
    break;
    default:
    BUG();
    }
    if (msr & MSR_PR)
// vsid |= VSID_PR;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_mmu_book3s_32_is_dcbz32(vcpu: *mut kvm_vcpu) -> bool {
    static bool kvmppc_mmu_book3s_32_is_dcbz32(struct kvm_vcpu *vcpu)
    {
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mmu_book3s_32_init(vcpu: *mut kvm_vcpu) {
    void kvmppc_mmu_book3s_32_init(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_mmu *mmu = &vcpu.arch.mmu;
    mmu.mtsrin = kvmppc_mmu_book3s_32_mtsrin;
    mmu.mfsrin = kvmppc_mmu_book3s_32_mfsrin;
    mmu.xlate = kvmppc_mmu_book3s_32_xlate;
    mmu.tlbie = kvmppc_mmu_book3s_32_tlbie;
    mmu.esid_to_vsid = kvmppc_mmu_book3s_32_esid_to_vsid;
    mmu.ea_to_vp = kvmppc_mmu_book3s_32_ea_to_vp;
    mmu.is_dcbz32 = kvmppc_mmu_book3s_32_is_dcbz32;
    mmu.slbmte = core::ptr::null_mut();
    mmu.slbmfee = core::ptr::null_mut();
    mmu.slbmfev = core::ptr::null_mut();
    mmu.slbfee = core::ptr::null_mut();
    mmu.slbie = core::ptr::null_mut();
    mmu.slbia = core::ptr::null_mut();
    }
