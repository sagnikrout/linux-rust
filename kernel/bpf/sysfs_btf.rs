//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/sysfs_btf.c
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
// Provide kernel BTF information for introspection and use by eBPF tools.
//

// See scripts/link-vmlinux.sh, gen_btf() func for details
    extern char __start_BTF[];
    extern char __stop_BTF[];
    static int btf_sysfs_vmlinux_mmap(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr,
    struct vm_area_struct *vma)
    {
    let mut pages: c_ulong = PAGE_ALIGN(attr.size) >> PAGE_SHIFT;
    let mut vm_size: usize = vma.vm_end - vma.vm_start;
    let mut addr: phys_addr_t = __pa_symbol(__start_BTF);
    let mut pfn: c_ulong = addr >> PAGE_SHIFT;
    if (attr.private != __start_BTF || !PAGE_ALIGNED(addr))
    return -EINVAL;
    if (vma.vm_pgoff)
    return -EINVAL;
    if (vma.vm_flags & (VM_WRITE | VM_EXEC | VM_MAYSHARE))
    return -EACCES;
    if (pfn + pages < pfn)
    return -EINVAL;
    if ((vm_size >> PAGE_SHIFT) > pages)
    return -EINVAL;
    vm_flags_mod(vma, VM_DONTDUMP, VM_MAYEXEC | VM_MAYWRITE);
    return remap_pfn_range(vma, vma.vm_start, pfn, vm_size, vma.vm_page_prot);
    }
    static struct bin_attribute bin_attr_btf_vmlinux __ro_after_init = {
    .attr = { .name = "vmlinux", .mode = 0444, },
    .read = sysfs_bin_attr_simple_read,
    .mmap = btf_sysfs_vmlinux_mmap,
    };
    struct kobject *btf_kobj;
#[no_mangle]
unsafe extern "C" fn btf_vmlinux_init() -> int __init {
    static int __init btf_vmlinux_init(void)
    {
    bin_attr_btf_vmlinux.private = __start_BTF;
    bin_attr_btf_vmlinux.size = __stop_BTF - __start_BTF;
    if (bin_attr_btf_vmlinux.size == 0)
    return 0;
    btf_kobj = kobject_create_and_add("btf", kernel_kobj);
    if (!btf_kobj)
    return -ENOMEM;
    return sysfs_create_bin_file(btf_kobj, &bin_attr_btf_vmlinux);
    }
    subsys_initcall(btf_vmlinux_init);
