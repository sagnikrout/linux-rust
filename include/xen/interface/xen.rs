//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/xen.h
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


// SPDX-License-Identifier: MIT
//
// xen.h
//
// Guest OS interface to Xen.
//
// Copyright (c) 2004, K A Fraser
//

//
// XEN "SYSTEM CALLS" (a.k.a. HYPERCALLS).
//
// x86_32: EAX = vector; EBX, ECX, EDX, ESI, EDI = args 1, 2, 3, 4, 5.
// EAX = return value
// (argument registers may be clobbered on return)
// x86_64: RAX = vector; RDI, RSI, RDX, R10, R8, R9 = args 1, 2, 3, 4, 5, 6.
// RAX = return value
// (argument registers not clobbered on return; RCX, R11 are)
//
pub const __HYPERVISOR_set_trap_table: c_int = 0;
pub const __HYPERVISOR_mmu_update: c_int = 1;
pub const __HYPERVISOR_set_gdt: c_int = 2;
pub const __HYPERVISOR_stack_switch: c_int = 3;
pub const __HYPERVISOR_set_callbacks: c_int = 4;
pub const __HYPERVISOR_fpu_taskswitch: c_int = 5;
pub const __HYPERVISOR_sched_op_compat: c_int = 6;
pub const __HYPERVISOR_platform_op: c_int = 7;
pub const __HYPERVISOR_set_debugreg: c_int = 8;
pub const __HYPERVISOR_get_debugreg: c_int = 9;
pub const __HYPERVISOR_update_descriptor: c_int = 10;
pub const __HYPERVISOR_memory_op: c_int = 12;
pub const __HYPERVISOR_multicall: c_int = 13;
pub const __HYPERVISOR_update_va_mapping: c_int = 14;
pub const __HYPERVISOR_set_timer_op: c_int = 15;
pub const __HYPERVISOR_event_channel_op_compat: c_int = 16;
pub const __HYPERVISOR_xen_version: c_int = 17;
pub const __HYPERVISOR_console_io: c_int = 18;
pub const __HYPERVISOR_physdev_op_compat: c_int = 19;
pub const __HYPERVISOR_grant_table_op: c_int = 20;
pub const __HYPERVISOR_vm_assist: c_int = 21;
pub const __HYPERVISOR_update_va_mapping_otherdomain: c_int = 22;

pub const __HYPERVISOR_vcpu_op: c_int = 24;

pub const __HYPERVISOR_mmuext_op: c_int = 26;
pub const __HYPERVISOR_xsm_op: c_int = 27;
pub const __HYPERVISOR_nmi_op: c_int = 28;
pub const __HYPERVISOR_sched_op: c_int = 29;
pub const __HYPERVISOR_callback_op: c_int = 30;
pub const __HYPERVISOR_xenoprof_op: c_int = 31;
pub const __HYPERVISOR_event_channel_op: c_int = 32;
pub const __HYPERVISOR_physdev_op: c_int = 33;
pub const __HYPERVISOR_hvm_op: c_int = 34;
pub const __HYPERVISOR_sysctl: c_int = 35;
pub const __HYPERVISOR_domctl: c_int = 36;
pub const __HYPERVISOR_kexec_op: c_int = 37;
pub const __HYPERVISOR_tmem_op: c_int = 38;

pub const __HYPERVISOR_xenpmu_op: c_int = 40;
pub const __HYPERVISOR_dm_op: c_int = 41;
// Architecture-specific hypercall definitions.
pub const __HYPERVISOR_arch_0: c_int = 48;
pub const __HYPERVISOR_arch_1: c_int = 49;
pub const __HYPERVISOR_arch_2: c_int = 50;
pub const __HYPERVISOR_arch_3: c_int = 51;
pub const __HYPERVISOR_arch_4: c_int = 52;
pub const __HYPERVISOR_arch_5: c_int = 53;
pub const __HYPERVISOR_arch_6: c_int = 54;
pub const __HYPERVISOR_arch_7: c_int = 55;
//
// VIRTUAL INTERRUPTS
//
// Virtual interrupts that a guest OS may receive from Xen.
// In the side comments, 'V.' denotes a per-VCPU VIRQ while 'G.' denotes a
// global VIRQ. The former can be bound once per VCPU and cannot be re-bound.
// The latter can be allocated only once per guest: they must initially be
// allocated to VCPU0 but can subsequently be re-bound.
//

// Architecture-specific VIRQ definitions.
pub const VIRQ_ARCH_0: c_int = 16;
pub const VIRQ_ARCH_1: c_int = 17;
pub const VIRQ_ARCH_2: c_int = 18;
pub const VIRQ_ARCH_3: c_int = 19;
pub const VIRQ_ARCH_4: c_int = 20;
pub const VIRQ_ARCH_5: c_int = 21;
pub const VIRQ_ARCH_6: c_int = 22;
pub const VIRQ_ARCH_7: c_int = 23;
pub const NR_VIRQS: c_int = 24;
//
// enum neg_errnoval HYPERVISOR_mmu_update(const struct mmu_update reqs[],
// unsigned count, unsigned *done_out,
// unsigned foreigndom)
// @reqs is an array of mmu_update_t structures ((ptr, val) pairs).
// @count is the length of the above array.
// @pdone is an output parameter indicating number of completed operations
// @foreigndom[15:0]: FD, the expected owner of data pages referenced in this
// hypercall invocation. Can be DOMID_SELF.
// @foreigndom[31:16]: PFD, the expected owner of pagetable pages referenced
// in this hypercall invocation. The value of this field
// (x) encodes the PFD as follows:
// x == 0 => PFD == DOMID_SELF
// x != 0 => PFD == x - 1
//
// Sub-commands: ptr[1:0] specifies the appropriate MMU_* command.
// -------------
// ptr[1:0] == MMU_NORMAL_PT_UPDATE:
// Updates an entry in a page table belonging to PFD. If updating an L1 table,
// and the new table entry is valid/present, the mapped frame must belong to
// FD. If attempting to map an I/O page then the caller assumes the privilege
// of the FD.
// FD == DOMID_IO: Permit /only/ I/O mappings, at the priv level of the caller.
// FD == DOMID_XEN: Map restricted areas of Xen's heap space.
// ptr[:2]  -- Machine address of the page-table entry to modify.
// val      -- Value to write.
//
// There also certain implicit requirements when using this hypercall. The
// pages that make up a pagetable must be mapped read-only in the guest.
// This prevents uncontrolled guest updates to the pagetable. Xen strictly
// enforces this, and will disallow any pagetable update which will end up
// mapping pagetable page RW, and will disallow using any writable page as a
// pagetable. In practice it means that when constructing a page table for a
// process, thread, etc, we MUST be very dilligient in following these rules:
// 1). Start with top-level page (PGD or in Xen language: L4). Fill out
// the entries.
// 2). Keep on going, filling out the upper (PUD or L3), and middle (PMD
// or L2).
// 3). Start filling out the PTE table (L1) with the PTE entries. Once
// done, make sure to set each of those entries to RO (so writeable bit
// is unset). Once that has been completed, set the PMD (L2) for this
// PTE table as RO.
// 4). When completed with all of the PMD (L2) entries, and all of them have
// been set to RO, make sure to set RO the PUD (L3). Do the same
// operation on PGD (L4) pagetable entries that have a PUD (L3) entry.
// 5). Now before you can use those pages (so setting the cr3), you MUST also
// pin them so that the hypervisor can verify the entries. This is done
// via the HYPERVISOR_mmuext_op(MMUEXT_PIN_L4_TABLE, guest physical frame
// number of the PGD (L4)). And this point the HYPERVISOR_mmuext_op(
// MMUEXT_NEW_BASEPTR, guest physical frame number of the PGD (L4)) can be
// issued.
// For 32-bit guests, the L4 is not used (as there is less pagetables), so
// instead use L3.
// At this point the pagetables can be modified using the MMU_NORMAL_PT_UPDATE
// hypercall. Also if so desired the OS can also try to write to the PTE
// and be trapped by the hypervisor (as the PTE entry is RO).
//
// To deallocate the pages, the operations are the reverse of the steps
// mentioned above. The argument is MMUEXT_UNPIN_TABLE for all levels and the
// pagetable MUST not be in use (meaning that the cr3 is not set to it).
//
// ptr[1:0] == MMU_MACHPHYS_UPDATE:
// Updates an entry in the machine->pseudo-physical mapping table.
// ptr[:2]  -- Machine address within the frame whose mapping to modify.
// The frame must belong to the FD, if one is specified.
// val      -- Value to write into the mapping entry.
//
// ptr[1:0] == MMU_PT_UPDATE_PRESERVE_AD:
// As MMU_NORMAL_PT_UPDATE above, but A/D bits currently in the PTE are ORed
// with those in @val.
//
// @val is usually the machine frame number along with some attributes.
// The attributes by default follow the architecture defined bits. Meaning that
// if this is a X86_64 machine and four page table layout is used, the layout
// of val is:
// - 63 if set means No execute (NX)
// - 46-13 the machine frame number
// - 12 available for guest
// - 11 available for guest
// - 10 available for guest
// - 9 available for guest
// - 8 global
// - 7 PAT (PSE is disabled, must use hypercall to make 4MB or 2MB pages)
// - 6 dirty
// - 5 accessed
// - 4 page cached disabled
// - 3 page write through
// - 2 userspace accessible
// - 1 writeable
// - 0 present
//
// The one bits that does not fit with the default layout is the PAGE_PSE
// also called PAGE_PAT). The MMUEXT_[UN]MARK_SUPER arguments to the
// HYPERVISOR_mmuext_op serve as mechanism to set a pagetable to be 4MB
// (or 2MB) instead of using the PAGE_PSE bit.
//
// The reason that the PAGE_PSE (bit 7) is not being utilized is due to Xen
// using it as the Page Attribute Table (PAT) bit - for details on it please
// refer to Intel SDM 10.12. The PAT allows to set the caching attributes of
// pages instead of using MTRRs.
//
// The PAT MSR is as follows (it is a 64-bit value, each entry is 8 bits):
// PAT4                 PAT0
// +-----+-----+----+----+----+-----+----+----+
// | UC  | UC- | WC | WB | UC | UC- | WC | WB |  <= Linux
// +-----+-----+----+----+----+-----+----+----+
// | UC  | UC- | WT | WB | UC | UC- | WT | WB |  <= BIOS (default when machine boots)
// +-----+-----+----+----+----+-----+----+----+
// | rsv | rsv | WP | WC | UC | UC- | WT | WB |  <= Xen
// +-----+-----+----+----+----+-----+----+----+
//
// The lookup of this index table translates to looking up
// Bit 7, Bit 4, and Bit 3 of val entry:
//
// PAT/PSE (bit 7) ... PCD (bit 4) .. PWT (bit 3).
//
// If all bits are off, then we are using PAT0. If bit 3 turned on,
// then we are using PAT1, if bit 3 and bit 4, then PAT2..
//
// As you can see, the Linux PAT1 translates to PAT4 under Xen. Which means
// that if a guest that follows Linux's PAT setup and would like to set Write
// Combined on pages it MUST use PAT4 entry. Meaning that Bit 7 (PAGE_PAT) is
// set. For example, under Linux it only uses PAT0, PAT1, and PAT2 for the
// caching as:
//
// WB = none (so PAT0)
// WC = PWT (bit 3 on)
// UC = PWT | PCD (bit 3 and 4 are on).
//
// To make it work with Xen, it needs to translate the WC bit as so:
//
// PWT (so bit 3 on) --> PAT (so bit 7 is on) and clear bit 3
//
// And to translate back it would:
//
// PAT (bit 7 on) --> PWT (bit 3 on) and clear bit 7.
//

//
// MMU EXTENDED OPERATIONS
//
// enum neg_errnoval HYPERVISOR_mmuext_op(mmuext_op_t uops[],
// unsigned int count,
// unsigned int *pdone,
// unsigned int foreigndom)
//
// HYPERVISOR_mmuext_op() accepts a list of mmuext_op structures.
// A foreigndom (FD) can be specified (or DOMID_SELF for none).
// Where the FD has some effect, it is described below.
//
// cmd: MMUEXT_(UN)PIN_*_TABLE
// mfn: Machine frame number to be (un)pinned as a p.t. page.
// The frame must belong to the FD, if one is specified.
//
// cmd: MMUEXT_NEW_BASEPTR
// mfn: Machine frame number of new page-table base to install in MMU.
//
// cmd: MMUEXT_NEW_USER_BASEPTR [x86/64 only]
// mfn: Machine frame number of new page-table base to install in MMU
// when in user space.
//
// cmd: MMUEXT_TLB_FLUSH_LOCAL
// No additional arguments. Flushes local TLB.
//
// cmd: MMUEXT_INVLPG_LOCAL
// linear_addr: Linear address to be flushed from the local TLB.
//
// cmd: MMUEXT_TLB_FLUSH_MULTI
// vcpumask: Pointer to bitmap of VCPUs to be flushed.
//
// cmd: MMUEXT_INVLPG_MULTI
// linear_addr: Linear address to be flushed.
// vcpumask: Pointer to bitmap of VCPUs to be flushed.
//
// cmd: MMUEXT_TLB_FLUSH_ALL
// No additional arguments. Flushes all VCPUs' TLBs.
//
// cmd: MMUEXT_INVLPG_ALL
// linear_addr: Linear address to be flushed from all VCPUs' TLBs.
//
// cmd: MMUEXT_FLUSH_CACHE
// No additional arguments. Writes back and flushes cache contents.
//
// cmd: MMUEXT_FLUSH_CACHE_GLOBAL
// No additional arguments. Writes back and flushes cache contents
// on all CPUs in the system.
//
// cmd: MMUEXT_SET_LDT
// linear_addr: Linear address of LDT base (NB. must be page-aligned).
// nr_ents: Number of entries in LDT.
//
// cmd: MMUEXT_CLEAR_PAGE
// mfn: Machine frame number to be cleared.
//
// cmd: MMUEXT_COPY_PAGE
// mfn: Machine frame number of the destination page.
// src_mfn: Machine frame number of the source page.
//
// cmd: MMUEXT_[UN]MARK_SUPER
// mfn: Machine frame number of head of superpage to be [un]marked.
//
pub const MMUEXT_PIN_L1_TABLE: c_int = 0;
pub const MMUEXT_PIN_L2_TABLE: c_int = 1;
pub const MMUEXT_PIN_L3_TABLE: c_int = 2;
pub const MMUEXT_PIN_L4_TABLE: c_int = 3;
pub const MMUEXT_UNPIN_TABLE: c_int = 4;
pub const MMUEXT_NEW_BASEPTR: c_int = 5;
pub const MMUEXT_TLB_FLUSH_LOCAL: c_int = 6;
pub const MMUEXT_INVLPG_LOCAL: c_int = 7;
pub const MMUEXT_TLB_FLUSH_MULTI: c_int = 8;
pub const MMUEXT_INVLPG_MULTI: c_int = 9;
pub const MMUEXT_TLB_FLUSH_ALL: c_int = 10;
pub const MMUEXT_INVLPG_ALL: c_int = 11;
pub const MMUEXT_FLUSH_CACHE: c_int = 12;
pub const MMUEXT_SET_LDT: c_int = 13;
pub const MMUEXT_NEW_USER_BASEPTR: c_int = 15;
pub const MMUEXT_CLEAR_PAGE: c_int = 16;
pub const MMUEXT_COPY_PAGE: c_int = 17;
pub const MMUEXT_FLUSH_CACHE_GLOBAL: c_int = 18;
pub const MMUEXT_MARK_SUPER: c_int = 19;
pub const MMUEXT_UNMARK_SUPER: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmuext_op {
    pub cmd: c_uint,
// [UN]PIN_TABLE, NEW_BASEPTR, NEW_USER_BASEPTR
// CLEAR_PAGE, COPY_PAGE, [UN]MARK_SUPER
    pub mfn: xen_pfn_t,
// INVLPG_LOCAL, INVLPG_ALL, SET_LDT
    pub linear_addr: c_ulong,
    pub arg1: },
// SET_LDT
    pub nr_ents: c_uint,
// TLB_FLUSH_MULTI, INVLPG_MULTI
    pub vcpumask: *mut c_void,
// COPY_PAGE
    pub src_mfn: xen_pfn_t,
    pub arg2: },
}

// These are passed as 'flags' to update_va_mapping. They can be ORed.
// When specifying UVMF_MULTI, also OR in a pointer to a CPU bitmap.
// UVMF_LOCAL is merely UVMF_MULTI with a NULL bitmap pointer.

//
// Commands to HYPERVISOR_console_io().
//
pub const CONSOLEIO_write: c_int = 0;
pub const CONSOLEIO_read: c_int = 1;
//
// Commands to HYPERVISOR_vm_assist().
//
pub const VMASST_CMD_enable: c_int = 0;
pub const VMASST_CMD_disable: c_int = 1;
// x86/32 guests: simulate full 4GB segment limits.
pub const VMASST_TYPE_4gb_segments: c_int = 0;
// x86/32 guests: trap (vector 15) whenever above vmassist is used.
pub const VMASST_TYPE_4gb_segments_notify: c_int = 1;
//
// x86 guests: support writes to bottom-level PTEs.
// NB1. Page-directory entries cannot be written.
// NB2. Guest must continue to remove all writable mappings of PTEs.
//
pub const VMASST_TYPE_writable_pagetables: c_int = 2;
// x86/PAE guests: support PDPTs above 4GB.
pub const VMASST_TYPE_pae_extended_cr3: c_int = 3;
//
// x86 guests: Sane behaviour for virtual iopl
// - virtual iopl updated from do_iret() hypercalls.
// - virtual iopl reported in bounce frames.
// - guest kernels assumed to be level 0 for the purpose of iopl checks.
//
pub const VMASST_TYPE_architectural_iopl: c_int = 4;
//
// All guests: activate update indicator in vcpu_runstate_info
// Enable setting the XEN_RUNSTATE_UPDATE flag in guest memory mapped
// vcpu_runstate_info during updates of the runstate information.
//
pub const VMASST_TYPE_runstate_update_flag: c_int = 5;
pub const MAX_VMASST_TYPE: c_int = 5;
pub type domid_t = u16;
// Domain ids >= DOMID_FIRST_RESERVED cannot be used for ordinary domains.

// DOMID_SELF is used in certain contexts to refer to oneself.

//
// DOMID_IO is used to restrict page-table updates to mapping I/O memory.
// Although no Foreign Domain need be specified to map I/O pages, DOMID_IO
// is useful to ensure that no mappings to the OS's own heap are accidentally
// installed. (e.g., in Linux this could cause havoc as reference counts
// aren't adjusted on the I/O-mapping code path).
// This only makes sense in MMUEXT_SET_FOREIGNDOM, but in that context can
// be specified by any calling domain.
//

//
// DOMID_XEN is used to allow privileged domains to map restricted parts of
// Xen's heap space (e.g., the machine_to_phys table).
// This only makes sense in MMUEXT_SET_FOREIGNDOM, and is only permitted if
// the caller is privileged.
//

// DOMID_COW is used as the owner of sharable pages

// DOMID_INVALID is used to identify pages with unknown owner.

// Idle domain.

//
// Send an array of these to HYPERVISOR_mmu_update().
// NB. The fields are natural pointer/address size for this architecture.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_update {
    pub /: *mut *mut uint64_t ptr; / Machine address of PTE.,
    pub /: *mut *mut uint64_t val; / New contents of PTE.,
}

//
// Send an array of these to HYPERVISOR_multicall().
// NB. The fields are logically the natural register size for this
// architecture. In cases where xen_ulong_t is larger than this then
// any unused bits in the upper portion must be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct multicall_entry {
    pub op: xen_ulong_t,
    pub result: xen_long_t,
    pub args: [xen_ulong_t; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_time_info {
//
// Updates to the following values are preceded and followed
// by an increment of 'version'. The guest can therefore
// detect updates by looking for changes to 'version'. If the
// least-significant bit of the version number is set then an
// update is in progress and the guest must wait to read a
// consistent set of values.  The correct way to interact with
// the version number is similar to Linux's seqlock: see the
// implementations of read_seqbegin/read_seqretry.
//
    pub version: u32,
    pub pad0: u32,
    pub /: *mut *mut uint64_t tsc_timestamp; / TSC at last update of time vals.,
    pub /: *mut *mut uint64_t system_time; / Time, in nanosecs, since boot.,
//
// Current system time:
// system_time + ((tsc - tsc_timestamp) << tsc_shift) * tsc_to_system_mul
// CPU frequency (Hz):
// ((10^9 << 32) / tsc_to_system_mul) >> tsc_shift
//
    pub tsc_to_system_mul: u32,
    pub tsc_shift: i8,
    pub pad1: [i8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_info {
//
// 'evtchn_upcall_pending' is written non-zero by Xen to indicate
// a pending notification for a particular VCPU. It is then cleared
// by the guest OS /before/ checking for pending work, thus avoiding
// a set-and-check race. Note that the mask is only accessed by Xen
// on the CPU that is currently hosting the VCPU. This means that the
// pending and mask flags can be updated by the guest without special
// synchronisation (i.e., no need for the x86 LOCK prefix).
// This may seem suboptimal because if the pending flag is set by
// a different CPU then an IPI may be scheduled even when the mask
// is set. However, note:
// 1. The task of 'interrupt holdoff' is covered by the per-event-
// channel mask bits. A 'noisy' event that is continually being
// triggered can be masked at source at this very precise
// granularity.
// 2. The main purpose of the per-VCPU mask is therefore to restrict
// reentrant execution: whether for concurrency control, or to
// prevent unbounded stack usage. Whatever the purpose, we expect
// that the mask will be asserted only for short periods at a time,
// and so the likelihood of a 'spurious' IPI is suitably small.
// The mask is read before making an event upcall to the guest: a
// non-zero mask therefore guarantees that the VCPU will not receive
// an upcall activation. The mask is cleared when the VCPU requests
// to block: this avoids wakeup-waiting races.
//
    pub evtchn_upcall_pending: u8,
    pub evtchn_upcall_mask: u8,
    pub evtchn_pending_sel: xen_ulong_t,
    pub arch: arch_vcpu_info,
    pub time: pvclock_vcpu_time_info,
}

//
// Xen/kernel shared data -- pointer provided in start_info.
// NB. We expect that this struct is smaller than a page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_info {
    pub vcpu_info: [vcpu_info; MAX_VIRT_CPUS],
//
// A domain can create "event channels" on which it can send and receive
// asynchronous event notifications. There are three classes of event that
// are delivered by this mechanism:
// 1. Bi-directional inter- and intra-domain connections. Domains must
// arrange out-of-band to set up a connection (usually by allocating
// an unbound 'listener' port and avertising that via a storage service
// such as xenstore).
// 2. Physical interrupts. A domain with suitable hardware-access
// privileges can bind an event-channel port to a physical interrupt
// source.
// 3. Virtual interrupts ('events'). A domain can bind an event-channel
// port to a virtual interrupt source, such as the virtual-timer
// device or the emergency console.
//
// Event channels are addressed by a "port index". Each channel is
// associated with two bits of information:
// 1. PENDING -- notifies the domain that there is a pending notification
// to be processed. This bit is cleared by the guest.
// 2. MASK -- if this bit is clear then a 0->1 transition of PENDING
// will cause an asynchronous upcall to be scheduled. This bit is only
// updated by the guest. It is read-only within Xen. If a channel
// becomes pending while the channel is masked then the 'edge' is lost
// (i.e., when the channel is unmasked, the guest must manually handle
// pending notifications as no upcall will be scheduled by Xen).
//
// To expedite scanning of pending notifications, any 0->1 pending
// transition on an unmasked channel causes a corresponding bit in a
// per-vcpu selector word to be set. Each bit in the selector covers a
// 'C long' in the PENDING bitfield array.
//
    pub 8]: *mut *mut xen_ulong_t evtchn_pending[sizeof(xen_ulong_t),
    pub 8]: *mut *mut xen_ulong_t evtchn_mask[sizeof(xen_ulong_t),
//
// Wallclock time: updated only by control software. Guests should base
// their gettimeofday() syscall on this wallclock-base value.
//
    pub wc: pvclock_wall_clock,

    pub wc_sec_hi: u32,

    pub arch: arch_shared_info,
}

//
// Start-of-day memory layout
//
// 1. The domain is started within contiguous virtual-memory region.
// 2. The contiguous region begins and ends on an aligned 4MB boundary.
// 3. This the order of bootstrap elements in the initial virtual region:
// a. relocated kernel image
// b. initial ram disk              [mod_start, mod_len]
// (may be omitted)
// c. list of allocated page frames [mfn_list, nr_pages]
// (unless relocated due to XEN_ELFNOTE_INIT_P2M)
// d. start_info_t structure        [register ESI (x86)]
// in case of dom0 this page contains the console info, too
// e. unless dom0: xenstore ring page
// f. unless dom0: console ring page
// g. bootstrap page tables         [pt_base, CR3 (x86)]
// h. bootstrap stack               [register ESP (x86)]
// 4. Bootstrap elements are packed together, but each is 4kB-aligned.
// 5. The list of page frames forms a contiguous 'pseudo-physical' memory
// layout for the domain. In particular, the bootstrap virtual-memory
// region is a 1:1 mapping to the first section of the pseudo-physical map.
// 6. All bootstrap elements are mapped read-writable for the guest OS. The
// only exception is the bootstrap page table, which is mapped read-only.
// 7. There is guaranteed to be at least 512kB padding after the final
// bootstrap element. If necessary, the bootstrap virtual region is
// extended by an extra 4MB to ensure this.
//
pub const MAX_GUEST_CMDLINE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_info {
// THE FOLLOWING ARE FILLED IN BOTH ON INITIAL BOOT AND ON RESUME.
    pub /: *mut *mut char magic[32]; / "xen-<version>-<platform>".,
    pub /: *mut *mut unsigned long nr_pages; / Total pages allocated to this domain.,
    pub /: *mut *mut unsigned long shared_info; / MACHINE address of shared info struct.,
    pub /: *mut *mut uint32_t flags; / SIF_xxx flags.,
    pub /: *mut *mut xen_pfn_t store_mfn; / MACHINE page number of shared page.,
    pub /: *mut *mut uint32_t store_evtchn; / Event channel for store communication.,
    pub /: *mut *mut xen_pfn_t mfn; / MACHINE page number of console page.,
    pub /: *mut *mut uint32_t evtchn; / Event channel for console page.,
    pub domU: },
    pub /: *mut *mut uint32_t info_off; / Offset of console_info struct.,
    pub start.*/: *mut *mut uint32_t info_size; / Size of console_info struct from,
    pub dom0: },
    pub console: },
// THE FOLLOWING ARE ONLY FILLED IN ON INITIAL BOOT (NOT RESUME).
    pub /: *mut *mut unsigned long pt_base; / VIRTUAL address of page directory.,
    pub /: *mut *mut unsigned long nr_pt_frames; / Number of bootstrap p.t. frames.,
    pub /: *mut *mut unsigned long mfn_list; / VIRTUAL address of page-frame list.,
    pub /: *mut *mut unsigned long mod_start; / VIRTUAL address of pre-loaded module.,
    pub /: *mut *mut unsigned long mod_len; / Size (bytes) of pre-loaded module.,
    pub cmd_line: [i8; MAX_GUEST_CMDLINE],
// The pfn range here covers both page table and p->m table frames.
    pub /: *mut *mut unsigned long first_p2m_pfn;/ 1st pfn forming initial P->M table.,
    pub /: *mut *mut unsigned long nr_p2m_frames;/ # of pfns forming initial P->M table.,
}

// These flags are passed in the 'flags' field of start_info_t.

// P->M making the 3 level tree obsolete?

//
// A multiboot module is a package containing modules very similar to a
// multiboot module array. The only differences are:
// - the array of module descriptors is by convention simply at the beginning
// of the multiboot module,
// - addresses in the module descriptors are based on the beginning of the
// multiboot module,
// - the number of modules is determined by a termination descriptor that has
// mod_start == 0.
//
// This permits to both build it statically and reference it in a configuration
// file, and let the PV guest easily rebase the addresses to virtual addresses
// and at the same time count the number of modules.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_multiboot_mod_list {
// Address of first byte of the module
    pub mod_start: u32,
// Address of last byte of the module (inclusive)
    pub mod_end: u32,
// Address of zero-terminated command line
    pub cmdline: u32,
// Unused, must be zero
    pub pad: u32,
}

//
// The console structure in start_info.console.dom0
//
// This structure includes a variety of information required to
// have a working VGA/VESA console.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dom0_vga_console_info {
    pub video_type: u8,
pub const XEN_VGATYPE_TEXT_MODE_3: c_uint = 0x03;
pub const XEN_VGATYPE_VESA_LFB: c_uint = 0x23;
pub const XEN_VGATYPE_EFI_LFB: c_uint = 0x70;
// Font height, in pixels.
    pub font_height: u16,
// Cursor location (column, row).
    pub cursor_y: uint16_t cursor_x,,
// Number of rows and columns (dimensions in characters).
    pub columns: uint16_t rows,,
    pub text_mode_3: },
// Width and height, in pixels.
    pub height: uint16_t width,,
// Bytes per scan line.
    pub bytes_per_line: u16,
// Bits per pixel.
    pub bits_per_pixel: u16,
// LFB physical address, and size (in units of 64kB).
    pub lfb_base: u32,
    pub lfb_size: u32,
// RGB mask offsets and sizes, as defined by VBE 1.2+
    pub red_size: uint8_t red_pos,,
    pub green_size: uint8_t green_pos,,
    pub blue_size: uint8_t blue_pos,,
    pub rsvd_size: uint8_t rsvd_pos,,
// VESA capabilities (offset 0xa, VESA command 0x4f00).
    pub gbl_caps: u32,
// Mode attributes (offset 0x0, VESA command 0x4f01).
    pub mode_attrs: u16,
    pub pad: u16,
// high 32 bits of lfb_base
    pub ext_lfb_base: u32,
    pub vesa_lfb: },
    pub u: },
}

pub type cpumap_t = u64;
// Turn a plain number into a C unsigned long constant.

pub const TMEM_SPEC_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmem_op {
    pub cmd: u32,
    pub pool_id: i32,
    pub uuid: [u64; 2],
    pub flags: u32,
    pub new: },
    pub oid: [u64; 3],
    pub index: u32,
    pub tmem_offset: u32,
    pub pfn_offset: u32,
    pub len: u32,
    pub /: *mut *mut GUEST_HANDLE(void) gmfn; / guest machine page frame,
    pub gen: },
    pub u: },
}

// In assembly code we cannot use C numeric constant suffixes.

