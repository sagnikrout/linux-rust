//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/machine_kexec.c
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
// Copyright (C) 2019 FORTH-ICS/CARV
// Nick Kossifidis <mick@ics.forth.gr>
//

//
// machine_kexec_prepare - Initialize kexec
//
// This function is called from do_kexec_load, when the user has
// provided us with an image to be loaded. Its goal is to validate
// the image and prepare the control code buffer as needed.
// Note that kimage_alloc_init has already been called and the
// control buffer has already been allocated.
//
    int
    machine_kexec_prepare(struct kimage *image)
    {
    struct kimage_arch *internal = &image.arch;
    let mut fdt: fdt_header = {0};
    void *control_code_buffer = core::ptr::null_mut();
    let mut control_code_buffer_sz: c_uint = 0;
    let mut i: c_int = 0;
// Find the Flattened Device Tree and save its physical address
    for (i = 0; i < image.nr_segments; i++) {
    if (image.segment[i].memsz <= sizeof(fdt))
    continue;
    if (!image.segment[i].buf)
    continue;
    if (image.file_mode)
    memcpy(&fdt, image.segment[i].buf, sizeof(fdt));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: copy_from_user(&fdt, _arg: image->segment[i].buf, _arg: sizeof(fdt))) -> else {
    else if (copy_from_user(&fdt, image.segment[i].buf, sizeof(fdt)))
    continue;
    if (fdt_check_header(&fdt))
    continue;
    internal.fdt_addr = (unsigned long) image.segment[i].mem;
    break;
    }
    if (!internal.fdt_addr) {
    pr_err("Device tree not included in the provided image\n");
    return -EINVAL;
    }
// Copy the assembler code for relocation to the control page
    if (image.type != KEXEC_TYPE_CRASH) {
    control_code_buffer = page_address(image.control_code_page);
    control_code_buffer_sz = page_size(image.control_code_page);
    if (unlikely(riscv_kexec_relocate_size > control_code_buffer_sz)) {
    pr_err("Relocation code doesn't fit within a control page\n");
    return -EINVAL;
    }
    memcpy(control_code_buffer, riscv_kexec_relocate,
    riscv_kexec_relocate_size);
// Mark the control page executable
    set_memory_x((unsigned long) control_code_buffer, 1);
    }
    return 0;
    }
//
// machine_kexec_cleanup - Cleanup any leftovers from
// machine_kexec_prepare
//
// This function is called by kimage_free to handle any arch-specific
// allocations done on machine_kexec_prepare. Since we didn't do any
// allocations there, this is just an empty function. Note that the
// control buffer is freed by kimage_free.
//
    void
    machine_kexec_cleanup(struct kimage *image)
    {
    }
//
// machine_shutdown - Prepare for a kexec reboot
//
// This function is called by kernel_kexec just before machine_kexec
// below. Its goal is to prepare the rest of the system (the other
// harts and possibly devices etc) for a kexec reboot.
//
#[no_mangle]
pub unsafe extern "C" fn machine_shutdown() {
    void machine_shutdown(void)
    {
//
// No more interrupts on this hart
// until we are back up.
//
    local_irq_disable();

    smp_shutdown_nonboot_cpus(smp_processor_id());

    }
//
// machine_crash_shutdown - Prepare to kexec after a kernel crash
//
// This function is called by crash_kexec just before machine_kexec
// and its goal is to shutdown non-crashing cpus and save registers.
//
    void
    machine_crash_shutdown(struct pt_regs *regs)
    {
    local_irq_disable();
// shutdown non-crashing cpus
    crash_smp_send_stop();
    crash_save_cpu(regs, smp_processor_id());
    machine_kexec_mask_interrupts();
    pr_info("Starting crashdump kernel...\n");
    }
//
// machine_kexec - Jump to the loaded kimage
//
// This function is called by kernel_kexec which is called by the
// reboot system call when the reboot cmd is LINUX_REBOOT_CMD_KEXEC,
// or by crash_kernel which is called by the kernel's arch-specific
// trap handler in case of a kernel panic. It's the final stage of
// the kexec process where the pre-loaded kimage is ready to be
// executed. We assume at this point that all other harts are
// suspended and this hart will be the new boot hart.
//
    void __noreturn
    machine_kexec(struct kimage *image)
    {
    struct kimage_arch *internal = &image.arch;
    let mut jump_addr: c_ulong = (unsigned long) image.start;
    let mut first_ind_entry: c_ulong = (unsigned long) &image.head;
    let mut this_cpu_id: c_ulong = __smp_processor_id();
    let mut this_hart_id: c_ulong = cpuid_to_hartid_map(this_cpu_id);
    let mut fdt_addr: c_ulong = internal.fdt_addr;
    void *control_code_buffer = page_address(image.control_code_page);
    let mut kexec_method: riscv_kexec_method = core::ptr::null_mut();

    WARN(smp_crash_stop_failed(),
    "Some CPUs may be stale, kdump will be unreliable.\n");

    if (image.type != KEXEC_TYPE_CRASH)
    kexec_method = control_code_buffer;
    else
    kexec_method = (riscv_kexec_method) &riscv_kexec_norelocate;
    pr_notice("Will call new kernel at %08lx from hart id %lx\n",
    jump_addr, this_hart_id);
    pr_notice("FDT image at %08lx\n", fdt_addr);
// Make sure the relocation code is visible to the hart
    local_flush_icache_all();
// Jump to the relocation code
    pr_notice("Bye...\n");
    kexec_method(first_ind_entry, jump_addr, fdt_addr,
    this_hart_id, kernel_map.va_pa_offset);
    unreachable();
    }
