//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/machdep.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
#[derive(Copy, Clone)]
pub struct machdep_calls {
    pub name: *const c_char,
    pub compatible: *const c_char,
    pub compatibles: *const *const c_char,

    pub (*iommu_restore)(void): *mut c_void,

    pub (*memory_block_size)(void): *mut c_ulong,

    pub dma_mask): *mut *mut *mut void (dma_set_mask)(struct device dev, u64,
    pub (*probe)(void): *mut c_int,
    pub /: *mut *mut *mut void (setup_arch)(void); / Optional, may be NULL,
// Optional, may be NULL.
    pub m): *mut *mut void (show_cpuinfo)(struct seq_file,
// Returns the current operating frequency of "cpu" in Hz
    pub cpu): *mut *mut unsigned long (get_proc_freq)(unsigned int,
    pub (*init_IRQ)(void): *mut c_void,
// Return an irq, or 0 to indicate there are none pending.
    pub (*get_irq)(void): *mut c_uint,
// PCI stuff
// Called after allocating resources
    pub (*pcibios_fixup)(void): *mut c_void,
    pub dev): *mut *mut void (pci_irq_fixup)(struct pci_dev,
// bridge);
// finds all the pci_controllers present at boot
    pub (*discover_phbs)(void): *mut c_void,
// To setup PHBs when using automatic OF platform driver for PCI
    pub host): *mut *mut int (pci_setup_phb)(struct pci_controller,
    pub cmd): *mut *mut void __noreturn (restart)(char,
    pub (*halt)(void): *mut void __noreturn,
    pub str): *mut *mut void (panic)(char,
    pub /: *mut *mut *mut long (time_init)(void); / Optional, may be NULL,
    pub ): *mut *mut int (set_rtc_time)(struct rtc_time,
    pub ): *mut *mut void (get_rtc_time)(struct rtc_time,
    pub (*get_boot_time)(void): *mut time64_t,
    pub (*calibrate_decr)(void): *mut c_void,
    pub short): *mut *mut *mut void (progress)(char , unsigned,
// Interface for platform error logging
    pub fatal): *mut *mut *mut void (log_error)(char buf, unsigned int err_type, int,
    pub addr): *mut *mut unsigned char (nvram_read_val)(int,
    pub val): *mut *mut void (nvram_write_val)(int addr, unsigned char,
    pub index): *mut *mut *mut ssize_t (nvram_write)(char buf, size_t count, loff_t,
    pub index): *mut *mut *mut ssize_t (nvram_read)(char buf, size_t count, loff_t,
    pub (*nvram_size)(void): *mut isize,
    pub (*nvram_sync)(void): *mut c_void,
// Exception handlers
    pub regs): *mut *mut int (system_reset_exception)(struct pt_regs,
    pub regs): *mut *mut int (machine_check_exception)(struct pt_regs,
    pub regs): *mut *mut int (handle_hmi_exception)(struct pt_regs,
// Early exception handlers called in realmode
    pub regs): *mut *mut int (hmi_exception_early)(struct pt_regs,
    pub regs): *mut *mut long (machine_check_early)(struct pt_regs,
// Called during machine check exception to retrive fixup address.
    pub regs): *mut *mut bool (mce_check_early_recovery)(struct pt_regs,
    pub (*machine_check_log_err)(void): *mut c_void,
// Motherboard/chipset features. This is a kind of general purpose
// hook used to control some machine specific features (like reset
// lines, chip power control, etc...).
//
    pub ...): *mut *mut long (feature_call)(unsigned int feature,,
// Get legacy PCI/IDE interrupt mapping
    pub channel): *mut *mut *mut int (pci_get_legacy_ide_irq)(struct pci_dev dev, int,
// Get access protection for /dev/mem
    pub vma_prot): pgprot_t,
//
// Function for waiting for work with reduced power in idle loop;
// called with interrupts disabled.
//
    pub (*power_save)(void): *mut c_void,
// Function to enable performance monitor counters for this
    pub (*enable_pmcs)(void): *mut c_void,
// Set DABR for this platform, leave empty for default implementation
    pub dabrx): c_ulong,
// Set DAWR for this platform, leave empty for default implementation
    pub dawrx): c_ulong,

// A general init function, called by ppc_init in init/main.c.
    pub (*init)(void): *mut c_void,
//
// optional PCI "hooks"
//
// Called at then very end of pcibios_init()
    pub (*pcibios_after_init)(void): *mut c_void,

// Called in indirect_* to avoid touching devices
    pub char): *mut *mut *mut int (pci_exclude_device)(struct pci_controller , unsigned char, unsigned,
// Called after PPC generic resource fixup to perform
    pub ): *mut *mut void (pcibios_fixup_resources)(struct pci_dev,
// Called for each PCI bus in the system when it's probed
    pub ): *mut *mut void (pcibios_fixup_bus)(struct pci_bus,
// Called after scan and before resource survey
    pub hose): *mut *mut void (pcibios_fixup_phb)(struct pci_controller,
//
// Called after device has been added to bus and
// before sysfs has been created.
//
    pub pdev): *mut *mut void (pcibios_bus_add_device)(struct pci_dev,
    pub (*pcibios_default_alignment)(void): *mut resource_size_t,

    pub pdev): *mut *mut void (pcibios_fixup_sriov)(struct pci_dev,
    pub resno): *const *const *const resource_size_t (pcibios_iov_resource_alignment)(struct pci_dev , int,
    pub num_vfs): *mut *mut *mut int (pcibios_sriov_enable)(struct pci_dev pdev, u16,
    pub pdev): *mut *mut int (pcibios_sriov_disable)(struct pci_dev,

// Called to shutdown machine specific hardware not already controlled
// by other drivers.
//
    pub (*machine_shutdown)(void): *mut c_void,

    pub secondary): *mut *mut void (kexec_cpu_down)(int crash_shutdown, int,
// Called to perform the _real_ kexec.
// Do NOT allocate memory or fail here. We are past the point of
// no return.
//
    pub image): *mut *mut void (machine_kexec)(struct kimage,

// These are called to disable and enable, respectively, IRQs when
// entering a suspend state.  If NULL, then the generic versions
// will be called.  The generic versions disable/enable the
// decrementer along with interrupts.
//
    pub (*suspend_disable_irqs)(void): *mut c_void,
    pub (*suspend_enable_irqs)(void): *mut c_void,

    pub size_t): *const *const *const ssize_t (cpu_probe)(char ,,
    pub size_t): *const *const *const ssize_t (cpu_release)(char ,,

    pub v): *mut *mut int (get_random_seed)(unsigned long,
}

extern "C" {
    pub fn e500_idle();
}
extern "C" {
    pub fn power4_idle();
}
extern "C" {
    pub fn ppc6xx_idle();
}
//
// ppc_md contains a copy of the machine description structure for the
// current platform. machine_id contains the initial address where the
// description was found during boot.
//

