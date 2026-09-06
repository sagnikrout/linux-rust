//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/mmu.c
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

#[no_mangle]
pub unsafe extern "C" fn arbitrary_virt_to_mfn(vaddr: *mut c_void) -> c_ulong {
    unsigned long arbitrary_virt_to_mfn(void *vaddr)
    {
    let mut maddr: xmaddr_t = arbitrary_virt_to_machine(vaddr);
    return PFN_DOWN(maddr.maddr);
    }
#[no_mangle]
pub unsafe extern "C" fn arbitrary_virt_to_machine(vaddr: *mut c_void) -> xmaddr_t {
    xmaddr_t arbitrary_virt_to_machine(void *vaddr)
    {
    let mut address: c_ulong = (unsigned long)vaddr;
    unsigned int level;
    pte_t *pte;
    unsigned offset;
//
// if the PFN is in the linear mapped vaddr range, we can just use
// the (quick) virt_to_machine() p2m lookup
//
    if (virt_addr_valid(vaddr))
    return virt_to_machine(vaddr);
// otherwise we have to do a (slower) full page-table walk
    pte = lookup_address(address, &level);
    BUG_ON(pte == core::ptr::null_mut());
    offset = address & ~PAGE_MASK;
    return XMADDR(((phys_addr_t)pte_mfn(*pte) << PAGE_SHIFT) + offset);
    }
    EXPORT_SYMBOL_GPL(arbitrary_virt_to_machine);
// Returns: 0 success
    int xen_unmap_domain_gfn_range(struct vm_area_struct *vma,
    int nr, struct page **pages)
    {
    if (!xen_pv_domain())
    return xen_xlate_unmap_gfn_range(vma, nr, pages);
    if (!pages)
    return 0;
    return -EINVAL;
    }
    EXPORT_SYMBOL_GPL(xen_unmap_domain_gfn_range);
