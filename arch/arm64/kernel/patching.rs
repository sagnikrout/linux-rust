//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/patching.c
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

    static DEFINE_RAW_SPINLOCK(patch_lock);
#[no_mangle]
unsafe extern "C" fn is_exit_text(addr: c_ulong) -> bool {
    static bool is_exit_text(unsigned long addr)
    {
// discarded with init text/data
    return system_state < SYSTEM_RUNNING &&
    addr >= (unsigned long)__exittext_begin &&
    addr < (unsigned long)__exittext_end;
    }
#[no_mangle]
unsafe extern "C" fn is_image_text(addr: c_ulong) -> bool {
    static bool is_image_text(unsigned long addr)
    {
    return core_kernel_text(addr) || is_exit_text(addr);
    }
    static void __kprobes *patch_map(void *addr, int fixmap)
    {
    phys_addr_t phys;
    if (is_image_text((unsigned long)addr)) {
    phys = __pa_symbol(addr);
    } else {
    struct page *page = vmalloc_to_page(addr);
    BUG_ON(!page);
    phys = page_to_phys(page) + offset_in_page(addr);
    }
    return (void *)set_fixmap_offset(fixmap, phys);
    }
#[no_mangle]
unsafe extern "C" fn patch_unmap(fixmap: c_int) -> void __kprobes {
    static void __kprobes patch_unmap(int fixmap)
    {
    clear_fixmap(fixmap);
    }
//
// In ARMv8-A, A64 instructions have a fixed length of 32 bits and are always
// little-endian.
//
#[no_mangle]
pub unsafe extern "C" fn aarch64_insn_read(addr: *mut c_void, insnp: *mut u32) -> int __kprobes {
    int __kprobes aarch64_insn_read(void *addr, u32 *insnp)
    {
    int ret;
    __le32 val;
    ret = copy_from_kernel_nofault(&val, addr, AARCH64_INSN_SIZE);
    if (!ret)
// insnp = le32_to_cpu(val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __aarch64_insn_write(addr: *mut c_void, insn: __le32) -> int __kprobes {
    static int __kprobes __aarch64_insn_write(void *addr, __le32 insn)
    {
    void *waddr = addr;
    let mut flags: c_ulong = 0;
    int ret;
    raw_spin_lock_irqsave(&patch_lock, flags);
    waddr = patch_map(addr, FIX_TEXT_POKE0);
    ret = copy_to_kernel_nofault(waddr, &insn, AARCH64_INSN_SIZE);
    patch_unmap(FIX_TEXT_POKE0);
    raw_spin_unlock_irqrestore(&patch_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn aarch64_insn_write(addr: *mut c_void, insn: u32) -> int __kprobes {
    int __kprobes aarch64_insn_write(void *addr, u32 insn)
    {
    return __aarch64_insn_write(addr, cpu_to_le32(insn));
    }
#[no_mangle]
pub unsafe extern "C" fn aarch64_insn_write_literal_u64(addr: *mut c_void, val: u64) -> noinstr int {
    noinstr int aarch64_insn_write_literal_u64(void *addr, u64 val)
    {
    u64 *waddr;
    unsigned long flags;
    int ret;
    raw_spin_lock_irqsave(&patch_lock, flags);
    waddr = patch_map(addr, FIX_TEXT_POKE0);
    ret = copy_to_kernel_nofault(waddr, &val, sizeof(val));
    patch_unmap(FIX_TEXT_POKE0);
    raw_spin_unlock_irqrestore(&patch_lock, flags);
    return ret;
    }
    typedef void text_poke_f(void *dst, void *src, size_t patched, size_t len);
    static void *__text_poke(text_poke_f func, void *addr, void *src, size_t len)
    {
    unsigned long flags;
    let mut patched: usize = 0;
    size_t size;
    void *waddr;
    void *ptr;
    raw_spin_lock_irqsave(&patch_lock, flags);
    while (patched < len) {
    ptr = addr + patched;
    size = min(PAGE_SIZE - offset_in_page(ptr), len - patched);
    waddr = patch_map(ptr, FIX_TEXT_POKE0);
    func(waddr, src, patched, size);
    patch_unmap(FIX_TEXT_POKE0);
    patched += size;
    }
    raw_spin_unlock_irqrestore(&patch_lock, flags);
    flush_icache_range((uintptr_t)addr, (uintptr_t)addr + len);
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn text_poke_memcpy(dst: *mut c_void, src: *mut c_void, patched: usize, len: usize) {
    static void text_poke_memcpy(void *dst, void *src, size_t patched, size_t len)
    {
    copy_to_kernel_nofault(dst, src + patched, len);
    }
#[no_mangle]
unsafe extern "C" fn text_poke_memset(dst: *mut c_void, src: *mut c_void, patched: usize, len: usize) {
    static void text_poke_memset(void *dst, void *src, size_t patched, size_t len)
    {
    let mut c: u32 = *(u32 *)src;
    memset32(dst, c, len / 4);
    }
//
// aarch64_insn_copy - Copy instructions into (an unused part of) RX memory
// @dst: address to modify
// @src: source of the copy
// @len: length to copy
//
// Useful for JITs to dump new code blocks into unused regions of RX memory.
//
    noinstr void *aarch64_insn_copy(void *dst, void *src, size_t len)
    {
// A64 instructions must be word aligned
    if ((uintptr_t)dst & 0x3)
    return core::ptr::null_mut();
    return __text_poke(text_poke_memcpy, dst, src, len);
    }
//
// aarch64_insn_set - memset for RX memory regions.
// @dst: address to modify
// @insn: value to set
// @len: length of memory region.
//
// Useful for JITs to fill regions of RX memory with illegal instructions.
//
    noinstr void *aarch64_insn_set(void *dst, u32 insn, size_t len)
    {
    if ((uintptr_t)dst & 0x3)
    return core::ptr::null_mut();
    return __text_poke(text_poke_memset, dst, &insn, len);
    }
#[no_mangle]
pub unsafe extern "C" fn aarch64_insn_patch_text_nosync(addr: *mut c_void, insn: u32) -> int __kprobes {
    int __kprobes aarch64_insn_patch_text_nosync(void *addr, u32 insn)
    {
    u32 *tp = addr;
    int ret;
// A64 instructions must be word aligned
    if ((uintptr_t)tp & 0x3)
    return -EINVAL;
    ret = aarch64_insn_write(tp, insn);
    if (ret == 0)
    caches_clean_inval_pou((uintptr_t)tp,
    (uintptr_t)tp + AARCH64_INSN_SIZE);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aarch64_insn_patch {
    pub text_addrs: *mut c_void,
    pub new_insns: *mut u32,
    pub insn_cnt: c_int,
    pub cpu_count: core::sync::atomic::AtomicI32,
}

#[no_mangle]
unsafe extern "C" fn aarch64_insn_patch_text_cb(arg: *mut c_void) -> int __kprobes {
    static int __kprobes aarch64_insn_patch_text_cb(void *arg)
    {
    int i, ret = 0;
    struct aarch64_insn_patch *pp = arg;
// The last CPU becomes master
    if (atomic_inc_return(&pp.cpu_count) == num_online_cpus()) {
    for (i = 0; ret == 0 && i < pp.insn_cnt; i++)
    ret = aarch64_insn_patch_text_nosync(pp.text_addrs[i],
    pp.new_insns[i]);
// Notify other processors with an additional increment.
    atomic_inc(&pp.cpu_count);
    } else {
    while (atomic_read(&pp.cpu_count) <= num_online_cpus())
    cpu_relax();
    isb();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn aarch64_insn_patch_text(addrs[]: *mut c_void, insns[]: u32, cnt: c_int) -> int __kprobes {
    int __kprobes aarch64_insn_patch_text(void *addrs[], u32 insns[], int cnt)
    {
    struct aarch64_insn_patch patch = {
    .text_addrs = addrs,
    .new_insns = insns,
    .insn_cnt = cnt,
    .cpu_count = ATOMIC_INIT(0),
    };
    if (cnt <= 0)
    return -EINVAL;
    return stop_machine_cpuslocked(aarch64_insn_patch_text_cb, &patch,
    cpu_online_mask);
    }
