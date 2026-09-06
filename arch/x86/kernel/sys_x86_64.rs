//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/sys_x86_64.c
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
// Align a virtual address to avoid aliasing in the I$ on AMD F15h.
//
#[no_mangle]
unsafe extern "C" fn get_align_mask(filp: *mut file) -> c_ulong {
    static unsigned long get_align_mask(struct file *filp)
    {
    if (filp && is_file_hugepages(filp))
    return huge_page_mask_align(filp);
// handle 32- and 64-bit case with a single conditional
    if (va_align.flags < 0 || !(va_align.flags & (2 - mmap_is_ia32())))
    return 0;
    if (!(current.flags & PF_RANDOMIZE))
    return 0;
    return va_align.mask;
    }
//
// To avoid aliasing in the I$ on AMD F15h, the bits defined by the
// va_align.bits, [12:upper_bit), are set to a random value instead of
// zeroing them. This random value is computed once per boot. This form
// of ASLR is known as "per-boot ASLR".
//
// To achieve this, the random value is added to the info.align_offset
// value before calling vm_unmapped_area() or ORed directly to the
// address.
//
#[no_mangle]
unsafe extern "C" fn get_align_bits() -> c_ulong {
    static unsigned long get_align_bits(void)
    {
    return va_align.bits & get_align_mask(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn control_va_addr_alignment(str: *mut c_char) -> int __init {
    static int __init control_va_addr_alignment(char *str)
    {
// guard against enabling this on other CPU families
    if (va_align.flags < 0)
    return 1;
    if (*str == 0)
    return 1;
    if (!strcmp(str, "32"))
    va_align.flags = ALIGN_VA_32;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "64")) -> else {
    else if (!strcmp(str, "64"))
    va_align.flags = ALIGN_VA_64;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "off")) -> else {
    else if (!strcmp(str, "off"))
    va_align.flags = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "on")) -> else {
    else if (!strcmp(str, "on"))
    va_align.flags = ALIGN_VA_32 | ALIGN_VA_64;
    else
    pr_warn("invalid option value: 'align_va_addr=%s'\n", str);
    return 1;
    }
    __setup("align_va_addr=", control_va_addr_alignment);
    SYSCALL_DEFINE6(mmap, unsigned long, addr, unsigned long, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, unsigned long, off)
    {
    if (off & ~PAGE_MASK)
    return -EINVAL;
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> PAGE_SHIFT);
    }
    static void find_start_end(unsigned long addr, unsigned long flags,
    unsigned long *begin, unsigned long *end)
    {
    if (!in_32bit_syscall() && (flags & MAP_32BIT)) {
// This is usually used needed to map code in small
    model, so it needs to be in the first 31bit. Limit
    it to that.  This means we need to move the
    unmapped base down for this case. This can give
    conflicts with the heap, but we assume that glibc
    malloc knows how to fall back to mmap. Give it 1GB
    of playground for now. -AK */
// begin = 0x40000000;
// end = 0x80000000;
    if (current.flags & PF_RANDOMIZE) {
// begin = randomize_page(*begin, 0x02000000);
    }
    return;
    }
// begin	= get_mmap_base(1);
    if (in_32bit_syscall())
// end = task_size_32bit();
    else
// end = task_size_64bit(addr > DEFAULT_MAP_WINDOW);
    }
#[no_mangle]
pub unsafe extern "C" fn stack_guard_placement(vm_flags: vm_flags_t) -> c_ulong {
    static inline unsigned long stack_guard_placement(vm_flags_t vm_flags)
    {
    if (vm_flags & VM_SHADOW_STACK)
    return PAGE_SIZE;
    return 0;
    }
    unsigned long
    arch_get_unmapped_area(struct file *filp, unsigned long addr, unsigned long len,
    unsigned long pgoff, unsigned long flags, vm_flags_t vm_flags)
    {
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma;
    let mut info: vm_unmapped_area_info = {};
    unsigned long begin, end;
    if (flags & MAP_FIXED)
    return addr;
    find_start_end(addr, flags, &begin, &end);
    if (len > end)
    return -ENOMEM;
    if (addr) {
    addr = PAGE_ALIGN(addr);
    vma = find_vma(mm, addr);
    if (end - len >= addr &&
    (!vma || addr + len <= vm_start_gap(vma)))
    return addr;
    }
    info.length = len;
    info.low_limit = begin;
    info.high_limit = end;
    if (!(filp && is_file_hugepages(filp))) {
    info.align_offset = pgoff << PAGE_SHIFT;
    info.start_gap = stack_guard_placement(vm_flags);
    }
    if (filp) {
    info.align_mask = get_align_mask(filp);
    info.align_offset += get_align_bits();
    }
    return vm_unmapped_area(&info);
    }
    unsigned long
    arch_get_unmapped_area_topdown(struct file *filp, unsigned long addr0,
    unsigned long len, unsigned long pgoff,
    unsigned long flags, vm_flags_t vm_flags)
    {
    struct vm_area_struct *vma;
    struct mm_struct *mm = current.mm;
    let mut addr: c_ulong = addr0;
    let mut info: vm_unmapped_area_info = {};
// requested length too big for entire address space
    if (len > TASK_SIZE)
    return -ENOMEM;
// No address checking. See comment at mmap_address_hint_valid()
    if (flags & MAP_FIXED)
    return addr;
// for MAP_32BIT mappings we force the legacy mmap base
    if (!in_32bit_syscall() && (flags & MAP_32BIT))
    goto bottomup;
// requesting a specific address
    if (addr) {
    addr &= PAGE_MASK;
    if (!mmap_address_hint_valid(addr, len))
    goto get_unmapped_area;
    vma = find_vma(mm, addr);
    if (!vma || addr + len <= vm_start_gap(vma))
    return addr;
    }
    get_unmapped_area:
    info.flags = VM_UNMAPPED_AREA_TOPDOWN;
    info.length = len;
    if (!in_32bit_syscall() && (flags & MAP_ABOVE4G))
    info.low_limit = SZ_4G;
    else
    info.low_limit = PAGE_SIZE;
    info.high_limit = get_mmap_base(0);
    if (!(filp && is_file_hugepages(filp))) {
    info.start_gap = stack_guard_placement(vm_flags);
    info.align_offset = pgoff << PAGE_SHIFT;
    }
//
// If hint address is above DEFAULT_MAP_WINDOW, look for unmapped area
// in the full address space.
//
// !in_32bit_syscall() check to avoid high addresses for x32
// (and make it no op on native i386).
//
    if (addr > DEFAULT_MAP_WINDOW && !in_32bit_syscall())
    info.high_limit += TASK_SIZE_MAX - DEFAULT_MAP_WINDOW;
    if (filp) {
    info.align_mask = get_align_mask(filp);
    info.align_offset += get_align_bits();
    }
    addr = vm_unmapped_area(&info);
    if (!(addr & ~PAGE_MASK))
    return addr;
    VM_BUG_ON(addr != -ENOMEM);
    bottomup:
//
// A failed mmap() very likely causes application failure,
// so fall back to the bottom-up function here. This scenario
// can happen with large stack limits and large mmap()
// allocations.
//
    return arch_get_unmapped_area(filp, addr0, len, pgoff, flags, 0);
    }
