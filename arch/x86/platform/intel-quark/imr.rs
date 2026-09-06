//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/intel-quark/imr.c
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
// imr.c -- Intel Isolated Memory Region driver
//
// Copyright(c) 2013 Intel Corporation.
// Copyright(c) 2015 Bryan O'Donoghue <pure.logic@nexus-software.ie>
//
// IMR registers define an isolated region of memory that can
// be masked to prohibit certain system agents from accessing memory.
// When a device behind a masked port performs an access - snooped or
// not, an IMR may optionally prevent that transaction from changing
// the state of memory or from getting correct data in response to the
// operation.
//
// Write data will be dropped and reads will return 0xFFFFFFFF, the
// system will reset and system BIOS will print out an error message to
// inform the user that an IMR has been violated.
//
// This code is based on the Linux MTRR code and reference code from
// Intel's Quark BSP EFI, Linux and grub code.
//
// See quark-x1000-datasheet.pdf for register definitions.
// http://www.intel.com/content/dam/www/public/us/en/documents/datasheets/quark-x1000-datasheet.pdf
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imr_device {
    pub init: bool,
    pub lock: mutex,
    pub max_imr: c_int,
    pub reg_base: c_int,
}

    static struct imr_device imr_dev;
//
// IMR read/write mask control registers.
// See quark-x1000-datasheet.pdf sections 12.7.4.5 and 12.7.4.6 for
// bit definitions.
//
// addr_hi
// 31		Lock bit
// 30:24	Reserved
// 23:2		1 KiB aligned lo address
// 1:0		Reserved
//
// addr_hi
// 31:24	Reserved
// 23:2		1 KiB aligned hi address
// 1:0		Reserved
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imr_regs {
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub rmask: u32,
    pub wmask: u32,
}

pub const IMR_SHIFT: c_int = 8;

//
// imr_is_enabled - true if an IMR is enabled false otherwise.
//
// Determines if an IMR is enabled based on address range and read/write
// mask. An IMR set with an address range set to zero and a read/write
// access mask set to all is considered to be disabled. An IMR in any
// other state - for example set to zero but without read/write access
// all is considered to be enabled. This definition of disabled is how
// firmware switches off an IMR and is maintained in kernel for
// consistency.
//
// @imr:	pointer to IMR descriptor.
// @return:	true if IMR enabled false if disabled.
//
#[no_mangle]
pub unsafe extern "C" fn imr_is_enabled(imr: *mut imr_regs) -> c_int {
    static inline int imr_is_enabled(struct imr_regs *imr)
    {
    return !(imr.rmask == IMR_READ_ACCESS_ALL &&
    imr.wmask == IMR_WRITE_ACCESS_ALL &&
    imr_to_phys(imr.addr_lo) == 0 &&
    imr_to_phys(imr.addr_hi) == 0);
    }
//
// imr_read - read an IMR at a given index.
//
// Requires caller to hold imr mutex.
//
// @idev:	pointer to imr_device structure.
// @imr_id:	IMR entry to read.
// @imr:	IMR structure representing address and access masks.
// @return:	0 on success or error code passed from mbi_iosf on failure.
//
#[no_mangle]
unsafe extern "C" fn imr_read(idev: *mut imr_device, imr_id: u32, imr: *mut imr_regs) -> c_int {
    static int imr_read(struct imr_device *idev, u32 imr_id, struct imr_regs *imr)
    {
    let mut reg: u32 = imr_id * IMR_NUM_REGS + idev.reg_base;
    int ret;
    ret = iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg++, &imr.addr_lo);
    if (ret)
    return ret;
    ret = iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg++, &imr.addr_hi);
    if (ret)
    return ret;
    ret = iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg++, &imr.rmask);
    if (ret)
    return ret;
    return iosf_mbi_read(QRK_MBI_UNIT_MM, MBI_REG_READ, reg++, &imr.wmask);
    }
//
// imr_write - write an IMR at a given index.
//
// Requires caller to hold imr mutex.
// Note lock bits need to be written independently of address bits.
//
// @idev:	pointer to imr_device structure.
// @imr_id:	IMR entry to write.
// @imr:	IMR structure representing address and access masks.
// @return:	0 on success or error code passed from mbi_iosf on failure.
//
#[no_mangle]
unsafe extern "C" fn imr_write(idev: *mut imr_device, imr_id: u32, imr: *mut imr_regs) -> c_int {
    static int imr_write(struct imr_device *idev, u32 imr_id, struct imr_regs *imr)
    {
    unsigned long flags;
    let mut reg: u32 = imr_id * IMR_NUM_REGS + idev.reg_base;
    int ret;
    local_irq_save(flags);
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg++, imr.addr_lo);
    if (ret)
    goto failed;
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg++, imr.addr_hi);
    if (ret)
    goto failed;
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg++, imr.rmask);
    if (ret)
    goto failed;
    ret = iosf_mbi_write(QRK_MBI_UNIT_MM, MBI_REG_WRITE, reg++, imr.wmask);
    if (ret)
    goto failed;
    local_irq_restore(flags);
    return 0;
    failed:
//
// If writing to the IOSF failed then we're in an unknown state,
// likely a very bad state. An IMR in an invalid state will almost
// certainly lead to a memory access violation.
//
    local_irq_restore(flags);
    WARN(ret, "IOSF-MBI write fail range 0x%08x-0x%08x unreliable\n",
    imr_to_phys(imr.addr_lo), imr_to_phys(imr.addr_hi) + IMR_MASK);
    return ret;
    }
//
// imr_dbgfs_state_show - print state of IMR registers.
//
// @s:		pointer to seq_file for output.
// @unused:	unused parameter.
// @return:	0 on success or error code passed from mbi_iosf on failure.
//
#[no_mangle]
unsafe extern "C" fn imr_dbgfs_state_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int imr_dbgfs_state_show(struct seq_file *s, void *unused)
    {
    phys_addr_t base;
    phys_addr_t end;
    int i;
    struct imr_device *idev = s.private;
    struct imr_regs imr;
    size_t size;
    let mut ret: c_int = -ENODEV;
    mutex_lock(&idev.lock);
    for (i = 0; i < idev.max_imr; i++) {
    ret = imr_read(idev, i, &imr);
    if (ret)
    break;
//
// Remember to add IMR_ALIGN bytes to size to indicate the
// inherent IMR_ALIGN size bytes contained in the masked away
// lower ten bits.
//
    if (imr_is_enabled(&imr)) {
    base = imr_to_phys(imr.addr_lo);
    end = imr_to_phys(imr.addr_hi) + IMR_MASK;
    size = end - base + 1;
    } else {
    base = 0;
    end = 0;
    size = 0;
    }
    seq_printf(s, "imr%02i: base=%pa, end=%pa, size=0x%08zx "
    "rmask=0x%08x, wmask=0x%08x, %s, %s\n", i,
    &base, &end, size, imr.rmask, imr.wmask,
    imr_is_enabled(&imr) ? "enabled " : "disabled",
    imr.addr_lo & IMR_LOCK ? "locked" : "unlocked");
    }
    mutex_unlock(&idev.lock);
    return ret;
    }
    DEFINE_SHOW_ATTRIBUTE(imr_dbgfs_state);
//
// imr_debugfs_register - register debugfs hooks.
//
// @idev:	pointer to imr_device structure.
//
#[no_mangle]
unsafe extern "C" fn imr_debugfs_register(idev: *mut imr_device) {
    static void imr_debugfs_register(struct imr_device *idev)
    {
    debugfs_create_file("imr_state", 0444, core::ptr::null_mut(), idev,
    &imr_dbgfs_state_fops);
    }
//
// imr_check_params - check passed address range IMR alignment and non-zero size
//
// @base:	base address of intended IMR.
// @size:	size of intended IMR.
// @return:	zero on valid range -EINVAL on unaligned base/size.
//
#[no_mangle]
unsafe extern "C" fn imr_check_params(base: phys_addr_t, size: usize) -> c_int {
    static int imr_check_params(phys_addr_t base, size_t size)
    {
    if ((base & IMR_MASK) || (size & IMR_MASK)) {
    pr_err("base %pa size 0x%08zx must align to 1KiB\n",
    &base, size);
    return -EINVAL;
    }
    if (size == 0)
    return -EINVAL;
    return 0;
    }
//
// imr_raw_size - account for the IMR_ALIGN bytes that addr_hi appends.
//
// IMR addr_hi has a built in offset of plus IMR_ALIGN (0x400) bytes from the
// value in the register. We need to subtract IMR_ALIGN bytes from input sizes
// as a result.
//
// @size:	input size bytes.
// @return:	reduced size.
//
#[no_mangle]
pub unsafe extern "C" fn imr_raw_size(size: usize) -> usize {
    static inline size_t imr_raw_size(size_t size)
    {
    return size - IMR_ALIGN;
    }
//
// imr_address_overlap - detects an address overlap.
//
// @addr:	address to check against an existing IMR.
// @imr:	imr being checked.
// @return:	true for overlap false for no overlap.
//
#[no_mangle]
pub unsafe extern "C" fn imr_address_overlap(addr: phys_addr_t, imr: *mut imr_regs) -> c_int {
    static inline int imr_address_overlap(phys_addr_t addr, struct imr_regs *imr)
    {
    return addr >= imr_to_phys(imr.addr_lo) && addr <= imr_to_phys(imr.addr_hi);
    }
//
// imr_add_range - add an Isolated Memory Region.
//
// @base:	physical base address of region aligned to 1KiB.
// @size:	physical size of region in bytes must be aligned to 1KiB.
// @rmask:	read access mask.
// @wmask:	write access mask.
// @return:	zero on success or negative value indicating error.
//
    int imr_add_range(phys_addr_t base, size_t size,
    unsigned int rmask, unsigned int wmask)
    {
    phys_addr_t end;
    unsigned int i;
    struct imr_device *idev = &imr_dev;
    struct imr_regs imr;
    size_t raw_size;
    int reg;
    int ret;
    if (WARN_ONCE(idev.init == false, "driver not initialized"))
    return -ENODEV;
    ret = imr_check_params(base, size);
    if (ret)
    return ret;
// Tweak the size value.
    raw_size = imr_raw_size(size);
    end = base + raw_size;
//
// Check for reserved IMR value common to firmware, kernel and grub
// indicating a disabled IMR.
//
    imr.addr_lo = phys_to_imr(base);
    imr.addr_hi = phys_to_imr(end);
    imr.rmask = rmask;
    imr.wmask = wmask;
    if (!imr_is_enabled(&imr))
    return -ENOTSUPP;
    mutex_lock(&idev.lock);
//
// Find a free IMR while checking for an existing overlapping range.
// Note there's no restriction in silicon to prevent IMR overlaps.
// For the sake of simplicity and ease in defining/debugging an IMR
// memory map we exclude IMR overlaps.
//
    reg = -1;
    for (i = 0; i < idev.max_imr; i++) {
    ret = imr_read(idev, i, &imr);
    if (ret)
    goto failed;
// Find overlap @ base or end of requested range.
    ret = -EINVAL;
    if (imr_is_enabled(&imr)) {
    if (imr_address_overlap(base, &imr))
    goto failed;
    if (imr_address_overlap(end, &imr))
    goto failed;
    } else {
    reg = i;
    }
    }
// Error out if we have no free IMR entries.
    if (reg == -1) {
    ret = -ENOMEM;
    goto failed;
    }
    pr_debug("add %d phys %pa-%pa size %zx mask 0x%08x wmask 0x%08x\n",
    reg, &base, &end, raw_size, rmask, wmask);
// Enable IMR at specified range and access mask.
    imr.addr_lo = phys_to_imr(base);
    imr.addr_hi = phys_to_imr(end);
    imr.rmask = rmask;
    imr.wmask = wmask;
    ret = imr_write(idev, reg, &imr);
    if (ret < 0) {
//
// In the highly unlikely event iosf_mbi_write failed
// attempt to rollback the IMR setup skipping the trapping
// of further IOSF write failures.
//
    imr.addr_lo = 0;
    imr.addr_hi = 0;
    imr.rmask = IMR_READ_ACCESS_ALL;
    imr.wmask = IMR_WRITE_ACCESS_ALL;
    imr_write(idev, reg, &imr);
    }
    failed:
    mutex_unlock(&idev.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(imr_add_range);
//
// __imr_remove_range - delete an Isolated Memory Region.
//
// This function allows you to delete an IMR by its index specified by reg or
// by address range specified by base and size respectively. If you specify an
// index on its own the base and size parameters are ignored.
// imr_remove_range(0, base, size); delete IMR at index 0 base/size ignored.
// imr_remove_range(-1, base, size); delete IMR from base to base+size.
//
// @reg:	imr index to remove.
// @base:	physical base address of region aligned to 1 KiB.
// @size:	physical size of region in bytes aligned to 1 KiB.
// @return:	-EINVAL on invalid range or out or range id
// -ENODEV if reg is valid but no IMR exists or is locked
// 0 on success.
//
#[no_mangle]
unsafe extern "C" fn __imr_remove_range(reg: c_int, base: phys_addr_t, size: usize) -> c_int {
    static int __imr_remove_range(int reg, phys_addr_t base, size_t size)
    {
    phys_addr_t end;
    let mut found: bool = false;
    unsigned int i;
    struct imr_device *idev = &imr_dev;
    struct imr_regs imr;
    size_t raw_size;
    let mut ret: c_int = 0;
    if (WARN_ONCE(idev.init == false, "driver not initialized"))
    return -ENODEV;
//
// Validate address range if deleting by address, else we are
// deleting by index where base and size will be ignored.
//
    if (reg == -1) {
    ret = imr_check_params(base, size);
    if (ret)
    return ret;
    }
// Tweak the size value.
    raw_size = imr_raw_size(size);
    end = base + raw_size;
    mutex_lock(&idev.lock);
    if (reg >= 0) {
// If a specific IMR is given try to use it.
    ret = imr_read(idev, reg, &imr);
    if (ret)
    goto failed;
    if (!imr_is_enabled(&imr) || imr.addr_lo & IMR_LOCK) {
    ret = -ENODEV;
    goto failed;
    }
    found = true;
    } else {
// Search for match based on address range.
    for (i = 0; i < idev.max_imr; i++) {
    ret = imr_read(idev, i, &imr);
    if (ret)
    goto failed;
    if (!imr_is_enabled(&imr) || imr.addr_lo & IMR_LOCK)
    continue;
    if ((imr_to_phys(imr.addr_lo) == base) &&
    (imr_to_phys(imr.addr_hi) == end)) {
    found = true;
    reg = i;
    break;
    }
    }
    }
    if (!found) {
    ret = -ENODEV;
    goto failed;
    }
    pr_debug("remove %d phys %pa-%pa size %zx\n", reg, &base, &end, raw_size);
// Tear down the IMR.
    imr.addr_lo = 0;
    imr.addr_hi = 0;
    imr.rmask = IMR_READ_ACCESS_ALL;
    imr.wmask = IMR_WRITE_ACCESS_ALL;
    ret = imr_write(idev, reg, &imr);
    failed:
    mutex_unlock(&idev.lock);
    return ret;
    }
//
// imr_remove_range - delete an Isolated Memory Region by address
//
// This function allows you to delete an IMR by an address range specified
// by base and size respectively.
// imr_remove_range(base, size); delete IMR from base to base+size.
//
// @base:	physical base address of region aligned to 1 KiB.
// @size:	physical size of region in bytes aligned to 1 KiB.
// @return:	-EINVAL on invalid range or out or range id
// -ENODEV if reg is valid but no IMR exists or is locked
// 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn imr_remove_range(base: phys_addr_t, size: usize) -> c_int {
    int imr_remove_range(phys_addr_t base, size_t size)
    {
    return __imr_remove_range(-1, base, size);
    }
    EXPORT_SYMBOL_GPL(imr_remove_range);
//
// imr_clear - delete an Isolated Memory Region by index
//
// This function allows you to delete an IMR by an address range specified
// by the index of the IMR. Useful for initial sanitization of the IMR
// address map.
// imr_ge(base, size); delete IMR from base to base+size.
//
// @reg:	imr index to remove.
// @return:	-EINVAL on invalid range or out or range id
// -ENODEV if reg is valid but no IMR exists or is locked
// 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn imr_clear(reg: c_int) -> c_int {
    static inline int imr_clear(int reg)
    {
    return __imr_remove_range(reg, 0, 0);
    }
//
// imr_fixup_memmap - Tear down IMRs used during bootup.
//
// BIOS and Grub both setup IMRs around compressed kernel, initrd memory
// that need to be removed before the kernel hands out one of the IMR
// encased addresses to a downstream DMA agent such as the SD or Ethernet.
// IMRs on Galileo are setup to immediately reset the system on violation.
// As a result if you're running a root filesystem from SD - you'll need
// the boot-time IMRs torn down or you'll find seemingly random resets when
// using your filesystem.
//
// @idev:	pointer to imr_device structure.
// @return:
//
#[no_mangle]
unsafe extern "C" fn imr_fixup_memmap(idev: *mut imr_device) -> void __init {
    static void __init imr_fixup_memmap(struct imr_device *idev)
    {
    let mut base: phys_addr_t = virt_to_phys(&_text);
    let mut size: usize = virt_to_phys(&__end_rodata) - base;
    unsigned long start, end;
    int i;
    int ret;
// Tear down all existing unlocked IMRs.
    for (i = 0; i < idev.max_imr; i++)
    imr_clear(i);
    start = (unsigned long)_text;
    end = (unsigned long)__end_rodata - 1;
//
// Setup an unlocked IMR around the physical extent of the kernel
// from the beginning of the .text section to the end of the
// .rodata section as one physically contiguous block.
//
// We don't round up @size since it is already PAGE_SIZE aligned.
// See vmlinux.lds.S for details.
//
    ret = imr_add_range(base, size, IMR_CPU, IMR_CPU);
    if (ret < 0) {
    pr_err("unable to setup IMR for kernel: %zu KiB (%lx - %lx)\n",
    size / 1024, start, end);
    } else {
    pr_info("protecting kernel .text - .rodata: %zu KiB (%lx - %lx)\n",
    size / 1024, start, end);
    }
    }
    static const struct x86_cpu_id imr_ids[] __initconst = {
    X86_MATCH_VFM(INTEL_QUARK_X1000, core::ptr::null_mut()),
    {}
    };
//
// imr_init - entry point for IMR driver.
//
// return: -ENODEV for no IMR support 0 if good to go.
//
#[no_mangle]
unsafe extern "C" fn imr_init() -> int __init {
    static int __init imr_init(void)
    {
    struct imr_device *idev = &imr_dev;
    if (!x86_match_cpu(imr_ids) || !iosf_mbi_available())
    return -ENODEV;
    idev.max_imr = QUARK_X1000_IMR_MAX;
    idev.reg_base = QUARK_X1000_IMR_REGBASE;
    idev.init = true;
    mutex_init(&idev.lock);
    imr_debugfs_register(idev);
    imr_fixup_memmap(idev);
    return 0;
    }
    device_initcall(imr_init);
