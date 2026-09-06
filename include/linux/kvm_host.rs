//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kvm_host.h
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
// The bit 16 ~ bit 31 of kvm_userspace_memory_region::flags are internally
// used in kvm, other bits are visible for userspace which are defined in
// include/uapi/linux/kvm.h.
//

//
// Bit 63 of the memslot generation number is an "update in-progress flag",
// e.g. is temporarily set for the duration of kvm_swap_active_memslots().
// This flag effectively creates a unique generation number that is used to
// mark cached memslot data, e.g. MMIO accesses, as potentially being stale,
// i.e. may (or may not) have come from the previous memslots generation.
//
// This is necessary because the actual memslots update is not atomic with
// respect to the generation number update.  Updating the generation number
// first would allow a vCPU to cache a spte from the old memslots using the
// new generation number, and updating the generation number after switching
// to the new memslots would allow cache hits using the old generation number
// to reference the defunct memslots.
//
// This mechanism is used to prevent getting hits in KVM's caches while a
// memslot update is in-progress, and to prevent cache hits *after* updating
// the actual generation number against accesses that were inserted into the
// cache *before* the memslots were updated.
//

// Two fragments for cross MMIO pages.
pub const KVM_MAX_MMIO_FRAGMENTS: c_int = 2;

pub const KVM_MAX_NR_ADDRESS_SPACES: c_int = 1;

//
// For the normal pfn, the highest 12 bits should be zero,
// so we can mask bit 62 ~ bit 52  to indicate the error pfn,
// mask bit 63 to indicate the noslot pfn.
//

//
// error pfns indicate that the gfn is in slot but faild to
// translate it to pfn on host.
//
// KVM_PFN_ERR_SIGPENDING indicates that fetching the PFN was interrupted
// by a pending signal.  Note, the signal may or may not be fatal.
//
// error_noslot pfns indicate that the gfn can not be
// translated to pfn - it is not in slot or failed to
// translate it to pfn.
//
// noslot pfn indicates that the gfn is not in slot.
//
// architectures with KVM_HVA_ERR_BAD other than PAGE_OFFSET (e.g. s390)
// provide own defines and kvm_is_error_hva
//

//
// Architecture-independent vcpu->requests bit members
// Bits 3-7 are reserved for more arch-independent bits.
//

pub const KVM_REQ_UNBLOCK: c_int = 2;
pub const KVM_REQ_DIRTY_RING_SOFT_FULL: c_int = 3;
pub const KVM_REQUEST_ARCH_BASE: c_int = 8;
//
// KVM_REQ_OUTSIDE_GUEST_MODE exists is purely as way to force the vCPU to
// OUTSIDE_GUEST_MODE.  KVM_REQ_OUTSIDE_GUEST_MODE differs from a vCPU "kick"
// in that it ensures the vCPU has reached OUTSIDE_GUEST_MODE before continuing
// on.  A kick only guarantees that the vCPU is on its way out, e.g. a previous
// kick may have set vcpu->mode to EXITING_GUEST_MODE, and so there's no
// guarantee the vCPU received an IPI and has actually exited guest mode.
//

extern "C" {
    pub fn kvm_make_all_cpus_request(kvm: *mut kvm, req: c_uint) -> bool;
}
pub const KVM_USERSPACE_IRQ_SOURCE_ID: c_int = 0;
pub const KVM_IRQFD_RESAMPLE_IRQ_SOURCE_ID: c_int = 1;
pub const KVM_PIT_IRQ_SOURCE_ID: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_io_range {
    pub addr: gpa_t,
    pub len: c_int,
    pub dev: *mut kvm_io_device,
}

pub const NR_IOBUS_DEVS: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_io_bus {
    pub dev_count: c_int,
    pub ioeventfd_count: c_int,
    pub rcu: rcu_head,
    pub range: [kvm_io_range; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_bus {
    KVM_MMIO_BUS,
    KVM_PIO_BUS,
    KVM_VIRTIO_CCW_NOTIFY_BUS,
    KVM_FAST_MMIO_BUS,
    KVM_IOCSR_BUS,
    KVM_NR_BUSES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_async_pf {
    pub work: work_struct,
    pub link: list_head,
    pub queue: list_head,
    pub vcpu: *mut kvm_vcpu,
    pub cr2_or_gpa: gpa_t,
    pub addr: c_ulong,
    pub arch: kvm_arch_async_pf,
    pub wakeup_all: bool,
    pub notpresent_injected: bool,
}

extern "C" {
    pub fn kvm_clear_async_pf_completion_queue(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_check_async_pf_completion(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_async_pf_wakeup_all(vcpu: *mut kvm_vcpu) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_mmu_notifier_arg {
    pub attributes: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_gfn_range_filter {
    KVM_FILTER_SHARED		= BIT(0),
    KVM_FILTER_PRIVATE		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_gfn_range {
    pub slot: *mut kvm_memory_slot,
    pub start: gfn_t,
    pub end: gfn_t,
    pub arg: kvm_mmu_notifier_arg,
    pub attr_filter: kvm_gfn_range_filter,
    pub may_block: bool,
    pub lockless: bool,
}

extern "C" {
    pub fn kvm_unmap_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
extern "C" {
    pub fn kvm_age_gfn(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
extern "C" {
    pub fn kvm_test_age_gfn(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_host_map {
//
// Only valid if the 'pfn' is managed by the host kernel (i.e. There is
// a 'struct page' for it. When using mem= kernel parameter some memory
// can be used as guest memory but they are not managed by host
// kernel).
//
    pub pinned_page: *mut page,
    pub page: *mut page,
    pub hva: *mut c_void,
    pub pfn: kvm_pfn_t,
    pub gfn: kvm_pfn_t,
    pub writable: bool,
}

//
// Used to check if the mapping is valid or not. Never use 'kvm_host_map'
// directly to check for that.
//
extern "C" {
    pub fn single_task_running(ktime_before(cur: ) && !need_resched() &&, _arg: stop) -> return;
}
//
// Sometimes a large or cross-page mmio needs to be broken up into separate
// exits for userspace servicing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmio_fragment {
    pub gpa: gpa_t,
    pub data: *mut c_void,
    pub val: u64,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu {
    pub kvm: *mut kvm,

    pub preempt_notifier: preempt_notifier,

    pub cpu: c_int,
    pub /: *mut *mut int vcpu_id; / id given by userspace at creation,
    pub /: *mut *mut int vcpu_idx; / index into kvm->vcpu_array,
    pub /: *mut *mut int ____srcu_idx; / Don't use this directly. You've been warned.,

    pub srcu_depth: c_int,

    pub mode: c_int,
    pub requests: u64,
    pub guest_debug: c_ulong,
    pub mutex: mutex,
    pub run: *mut kvm_run,
    pub wait: rcuwait,

    pub pid: *mut pid,
    pub pid_lock: rwlock_t,
    pub sigset_active: c_int,
    pub sigset: sigset_t,
    pub halt_poll_ns: c_uint,
    pub valid_wakeup: bool,

    pub mmio_needed: c_int,
    pub mmio_read_completed: c_int,
    pub mmio_is_write: c_int,
    pub mmio_cur_fragment: c_int,
    pub mmio_nr_fragments: c_int,
    pub mmio_fragments: [kvm_mmio_fragment; KVM_MAX_MMIO_FRAGMENTS],
    pub queued: u32,
    pub queue: list_head,
    pub done: list_head,
    pub lock: spinlock_t,
    pub async_pf: },

//
// Cpu relax intercept or pause loop exit optimization
// in_spin_loop: set when a vcpu does a pause loop exit
// or cpu relax intercepted.
// dy_eligible: indicates whether vcpu is eligible for directed yield.
//
    pub in_spin_loop: bool,
    pub dy_eligible: bool,
    pub spin_loop: },

    pub wants_to_run: bool,
    pub preempted: bool,
    pub ready: bool,
    pub scheduled_out: bool,
    pub arch: kvm_vcpu_arch,
    pub stat: kvm_vcpu_stat,
    pub stats_id: [c_char; KVM_STATS_NAME_SIZE],
    pub dirty_ring: kvm_dirty_ring,
//
// The most recently used memslot by this vCPU and the slots generation
// for which it is valid.
// No wraparound protection is needed since generations won't overflow in
// thousands of years, even assuming 1M memslot operations per second.
//
    pub last_used_slot: *mut kvm_memory_slot,
    pub last_used_slot_gen: u64,
}

//
// Start accounting time towards a guest.
// Must be called before entering guest context.
//
// This is running in ioctl context so its safe to assume that it's the
// stime pending cputime to flush.
//
// Enter guest context and enter an RCU extended quiescent state.
//
// Between guest_context_enter_irqoff() and guest_context_exit_irqoff() it is
// unsafe to use any code which may directly or indirectly use RCU, tracing
// (including IRQ flag tracing), or lockdep. All code in this period must be
// non-instrumentable.
//
// KVM does not hold any references to rcu protected data when it
// switches CPU into a guest mode. In fact switching to a guest mode
// is very similar to exiting to userspace from rcu point of view. In
// addition CPU may stay in a guest mode for quite a long time (up to
// one time slice). Lets treat guest mode as quiescent state, just like
// we do with user-mode execution.
//
// Deprecated. Architectures should move to guest_timing_enter_irqoff() and
// guest_state_enter_irqoff().
//
// guest_state_enter_irqoff - Fixup state when entering a guest
//
// Entry to a guest will enable interrupts, but the kernel state is interrupts
// disabled when this is invoked. Also tell RCU about it.
//
// 1) Trace interrupts on state
// 2) Invoke context tracking if enabled to adjust RCU state
// 3) Tell lockdep that interrupts are enabled
//
// Invoked from architecture specific code before entering a guest.
// Must be called with interrupts disabled and the caller must be
// non-instrumentable.
// The caller has to invoke guest_timing_enter_irqoff() before this.
//
// Note: this is analogous to exit_to_user_mode().
//
// Exit guest context and exit an RCU extended quiescent state.
//
// Between guest_context_enter_irqoff() and guest_context_exit_irqoff() it is
// unsafe to use any code which may directly or indirectly use RCU, tracing
// (including IRQ flag tracing), or lockdep. All code in this period must be
// non-instrumentable.
//
// Guest mode is treated as a quiescent state, see
// guest_context_enter_irqoff() for more details.
//
// Stop accounting time towards a guest.
// Must be called after exiting guest context.
//
// Flush the guest cputime we spent on the guest
//
// Deprecated. Architectures should move to guest_state_exit_irqoff() and
// guest_timing_exit_irqoff().
//
// guest_state_exit_irqoff - Establish state when returning from guest mode
//
// Entry from a guest disables interrupts, but guest mode is traced as
// interrupts enabled. Also with NO_HZ_FULL RCU might be idle.
//
// 1) Tell lockdep that interrupts are disabled
// 2) Invoke context tracking if enabled to reactivate RCU
// 3) Trace interrupts off state
//
// Invoked from architecture specific code after exiting a guest.
// Must be invoked with interrupts disabled and the caller must be
// non-instrumentable.
// The caller has to invoke guest_timing_exit_irqoff() after this.
//
// Note: this is analogous to enter_from_user_mode().
//
// The memory barrier ensures a previous write to vcpu->requests cannot
// be reordered with the read of vcpu->mode.  It pairs with the general
// memory barrier following the write of vcpu->mode in VCPU RUN.
//
extern "C" {
    pub fn cmpxchg(_arg: &vcpu->mode, _arg: IN_GUEST_MODE, _arg: EXITING_GUEST_MODE) -> return;
}
//
// Some of the bitops functions do not support too long bitmaps.
// This number must be determined not to exceed such limits.
//

//
// Since at idle each memslot belongs to two memslot sets it has to contain
// two embedded nodes for each data structure that it forms a part of.
//
// Two memslot sets (one active and one inactive) are necessary so the VM
// continues to run on one memslot set while the other is being modified.
//
// These two memslot sets normally point to the same set of memslots.
// They can, however, be desynchronized when performing a memslot management
// operation by replacing the memslot to be modified by its copy.
// After the operation is complete, both memslot sets once again point to
// the same, common set of memslot data.
//
// The memslots themselves are independent of each other so they can be
// individually added or deleted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_memory_slot {
    pub id_node: [hlist_node; 2],
    pub hva_node: [interval_tree_node; 2],
    pub gfn_node: [rb_node; 2],
    pub base_gfn: gfn_t,
    pub npages: c_ulong,
    pub dirty_bitmap: *mut c_ulong,
    pub arch: kvm_arch_memory_slot,
    pub userspace_addr: c_ulong,
    pub flags: u32,
    pub id: c_short,
    pub as_id: u16,

//
// Writes protected by kvm->slots_lock.  Acquiring a
// reference via kvm_gmem_get_file() is protected by
// either kvm->slots_lock or kvm->srcu.
//
    pub file: *mut file,
    pub pgoff: pgoff_t,
    pub gmem: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_adapter_int {
    pub ind_addr: u64,
    pub ind_gaddr: u64,
    pub summary_addr: u64,
    pub summary_gaddr: u64,
    pub ind_offset: u64,
    pub summary_offset: u32,
    pub adapter_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hv_sint {
    pub vcpu: u32,
    pub sint: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xen_evtchn {
    pub port: u32,
    pub vcpu_id: u32,
    pub vcpu_idx: c_int,
    pub priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kernel_irq_routing_entry {
    pub gsi: u32,
    pub type: u32,
    pub line_status): bool,
    pub irqchip: unsigned,
    pub pin: unsigned,
    pub irqchip: },
    pub address_lo: u32,
    pub address_hi: u32,
    pub data: u32,
    pub flags: u32,
    pub devid: u32,
    pub msi: },
    pub adapter: kvm_s390_adapter_int,
    pub hv_sint: kvm_hv_sint,
    pub xen_evtchn: kvm_xen_evtchn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_table {
    pub chip: [c_int; KVM_NR_IRQCHIPS][KVM_IRQCHIP_NUM_PINS],
    pub nr_rt_entries: u32,
//
// Array indexed by gsi. Each entry contains list of irq chips
// the gsi is connected to.
//
    pub __counted_by(nr_rt_entries): hlist_head map[],
}

extern "C" {
    pub fn kvm_arch_irqchip_in_kernel(kvm: *mut kvm) -> bool;
}

pub const KVM_INTERNAL_MEM_SLOTS: c_int = 0;

extern "C" {
    pub fn kvm_arch_supports_gmem_init_shared(kvm: *mut kvm) -> bool;
}

extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_HAVE_KVM_READONLY_MEM) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_memslots {
    pub generation: u64,
    pub last_used_slot: atomic_long_t,
    pub hva_tree: rb_root_cached,
    pub gfn_tree: rb_root,
//
// The mapping table from slot id to memslot.
//
// 7-bit bucket count matches the size of the old id to index array for
// 512 slots, while giving good performance with this slot count.
// Higher bucket counts bring only small performance improvements but
// always result in higher memory usage (even for lower memslot counts).
//
    pub 7): DECLARE_HASHTABLE(id_hash,,
    pub node_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm {

    pub mmu_lock: rwlock_t,

    pub mmu_lock: spinlock_t,

    pub slots_lock: mutex,
//
// Protects the arch-specific fields of struct kvm_memory_slots in
// use by the VM. To be used under the slots_lock (above) or in a
// kvm->srcu critical section where acquiring the slots_lock would
// lead to deadlock with the synchronize_srcu in
// kvm_swap_active_memslots().
//
    pub slots_arch_lock: mutex,
    pub /: *mut *mut *mut mm_mm; / userspace tied to this vm,
    pub nr_memslot_pages: c_ulong,
// The two memslot sets - active and inactive (per address space)
    pub __memslots: [kvm_memslots; KVM_MAX_NR_ADDRESS_SPACES][2],
// The current active memslot set for each address space
    pub memslots: [*mut kvm_memslots __rcu; KVM_MAX_NR_ADDRESS_SPACES],
    pub vcpu_array: xarray,
    pub KVM_MAX_VCPU_IDS): DECLARE_BITMAP(vcpu_ids,,
//
// Protected by slots_lock, but can be read outside if an
// incorrect answer is acceptable.
//
    pub nr_memslots_dirty_logging: core::sync::atomic::AtomicI32,
// Used to wait for completion of MMU notifiers.
    pub mn_invalidate_lock: spinlock_t,
    pub mn_active_invalidate_count: c_ulong,
    pub mn_memslots_update_rcuwait: rcuwait,
// For management / invalidation of gfn_to_pfn_caches
    pub gpc_lock: spinlock_t,
    pub gpc_list: list_head,
//
// created_vcpus is protected by kvm->lock, and is incremented
// at the beginning of KVM_CREATE_VCPU.  online_vcpus is only
// incremented after storing the kvm_vcpu pointer in vcpus,
// and is accessed atomically.
//
    pub online_vcpus: core::sync::atomic::AtomicI32,
    pub max_vcpus: c_int,
    pub created_vcpus: c_int,
    pub last_boosted_vcpu: c_int,
    pub vm_list: list_head,
    pub lock: mutex,
    pub buses: [*mut kvm_io_bus __rcu; KVM_NR_BUSES],
    pub lock: spinlock_t,
    pub items: list_head,
// resampler_list update side is protected by resampler_lock.
    pub resampler_list: list_head,
    pub resampler_lock: mutex,
    pub irqfds: },

    pub ioeventfds: list_head,
    pub stat: kvm_vm_stat,
    pub arch: kvm_arch,
    pub users_count: refcount_t,

    pub coalesced_mmio_ring: *mut kvm_coalesced_mmio_ring,
    pub ring_lock: spinlock_t,
    pub coalesced_zones: list_head,

    pub irq_lock: mutex,

//
// Update side is protected by irq_lock.
//
    pub irq_routing: *mut kvm_irq_routing_table __rcu,
    pub irq_ack_notifier_list: hlist_head,

    pub mmu_notifier: mmu_notifier,
    pub mmu_invalidate_seq: c_ulong,
    pub mmu_invalidate_in_progress: c_long,
    pub mmu_invalidate_range_start: gfn_t,
    pub mmu_invalidate_range_end: gfn_t,
    pub devices: list_head,
    pub manual_dirty_log_protect: u64,
    pub debugfs_dentry: *mut dentry,
    pub debugfs_stat_data: *mut kvm_stat_data,
    pub srcu: srcu_struct,
    pub irq_srcu: srcu_struct,
    pub userspace_pid: pid_t,
    pub override_halt_poll_ns: bool,
    pub max_halt_poll_ns: c_uint,
    pub dirty_ring_size: u32,
    pub dirty_ring_with_bitmap: bool,
    pub vm_bugged: bool,
    pub vm_dead: bool,

    pub pm_notifier: notifier_block,

// Protected by slots_lock (for writes) and RCU (for reads)
    pub mem_attr_array: xarray,
    pub stats_id: [c_char; KVM_STATS_NAME_SIZE],
}

// The guest did something we don't support.

//
// Note, "data corruption" refers to corruption of host kernel data structures,
// not guest data.  Guest data corruption, suspected or confirmed, that is tied
// and contained to a single VM should *never* BUG() and potentially panic the
// host, i.e. use this variant of KVM_BUG() if and only if a KVM data structure
// is corrupted and that corruption can have a cascading effect to other parts
// of the hosts and/or to other VMs.
//

//
// Get a bus reference under the update-side lock. No long-term SRCU reader
// references are permitted, to avoid stale reads vs concurrent IO
// registrations.
//
// Explicitly verify the target vCPU is online, as the anti-speculation
// logic only limits the CPU's ability to speculate, e.g. given a "bad"
// index, clamping the index to 0 would return vCPU0, not NULL.
//
// Pairs with smp_wmb() in kvm_vm_ioctl_create_vcpu.
extern "C" {
    pub fn xa_load(_arg: &kvm->vcpu_array, _arg: i) -> return;
}

extern "C" {
    pub fn kvm_destroy_vcpus(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_trylock_all_vcpus(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_lock_all_vcpus(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_unlock_all_vcpus(kvm: *mut kvm);
}
extern "C" {
    pub fn vcpu_load(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vcpu_put(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvm_arch_post_irq_ack_notifier_list_update(kvm: *mut kvm);
}

extern "C" {
    pub fn kvm_irqfd_init() -> c_int;
}
extern "C" {
    pub fn kvm_irqfd_exit();
}

extern "C" {
    pub fn kvm_init(vcpu_size: unsigned, vcpu_align: unsigned, module: *mut module) -> c_int;
}
extern "C" {
    pub fn kvm_exit();
}
extern "C" {
    pub fn kvm_get_kvm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_get_kvm_safe(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_put_kvm(kvm: *mut kvm);
}
extern "C" {
    pub fn file_is_kvm(file: *mut file) -> bool;
}
extern "C" {
    pub fn kvm_put_kvm_no_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn __kvm_memslots(_arg: kvm, _arg: 0) -> return;
}
extern "C" {
    pub fn __kvm_memslots(_arg: vcpu->kvm, _arg: as_id) -> return;
}
extern "C" {
    pub fn RB_EMPTY_ROOT(_arg: &slots->gfn_tree) -> return;
}
extern "C" {
    pub fn kvm_are_all_memslots_empty(kvm: *mut kvm) -> bool;
}

// Iterator used for walking memslots that overlap a gfn range.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_memslot_iter {
    pub slots: *mut kvm_memslots,
    pub node: *mut rb_node,
    pub slot: *mut kvm_memory_slot,
}

//
// Find the so called "upper bound" of a key - the first node that has
// its key strictly greater than the searched one (the start gfn in our case).
//
// Find the slot with the lowest gfn that can possibly intersect with
// the range, so we'll ideally have slot start <= range start
//
// A NULL previous node means that the very first slot
// already has a higher start gfn.
// In this case slot start > range start.
//
// a NULL node below means no slots
//
// It is possible in the slot start < range start case that the
// found slot ends before or at range start (slot end <= range start)
// and so it does not overlap the requested range.
//
// In such non-overlapping case the next slot (if it exists) will
// already have slot start > range start, otherwise the logic above
// would have found it instead of the current slot.
//
// If this slot starts beyond or at the end of the range so does
// every next one
//
// Iterate over each memslot at least partially intersecting [start, end) range

//
// KVM_SET_USER_MEMORY_REGION ioctl allows the following operations:
// - create a new memory slot
// - delete an existing memory slot
// - modify an existing memory slot
// -- move it in the guest physical memory space
// -- just change its flags
//
// Since flags can be changed by some of these operations, the following
// differentiation is the best we can do for kvm_set_memory_region():
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_mr_change {
    KVM_MR_CREATE,
    KVM_MR_DELETE,
    KVM_MR_MOVE,
    KVM_MR_FLAGS_ONLY,
}

extern "C" {
    pub fn kvm_arch_free_memslot(kvm: *mut kvm, slot: *mut kvm_memory_slot);
}
extern "C" {
    pub fn kvm_arch_memslots_updated(kvm: *mut kvm, gen: u64);
}
// flush all memory translations
extern "C" {
    pub fn kvm_arch_flush_shadow_all(kvm: *mut kvm);
}
// flush memory translations pointing to 'slot'
extern "C" {
    pub fn __gfn_to_page(_arg: kvm, _arg: gfn, _arg: true) -> return;
}
extern "C" {
    pub fn gfn_to_hva(kvm: *mut kvm, gfn: gfn_t) -> c_ulong;
}
extern "C" {
    pub fn gfn_to_hva_prot(kvm: *mut kvm, gfn: gfn_t, writable: *mut bool) -> c_ulong;
}
extern "C" {
    pub fn gfn_to_hva_memslot(slot: *mut kvm_memory_slot, gfn: gfn_t) -> c_ulong;
}
extern "C" {
    pub fn kvm_release_page_clean(page: *mut page);
}
extern "C" {
    pub fn kvm_release_page_dirty(page: *mut page);
}
//
// If the page that KVM got from the *primary MMU* is writable, and KVM
// installed or reused a SPTE, mark the page/folio dirty.  Note, this
// may mark a folio dirty even if KVM created a read-only SPTE, e.g. if
// the GFN is write-protected.  Folios can't be safely marked dirty
// outside of mmu_lock as doing so could race with writeback on the
// folio.  As a result, KVM can't mark folios dirty in the fast page
// fault handler, and so KVM must (somewhat) speculatively mark the
// folio dirty if KVM could locklessly make the SPTE writable.
//
extern "C" {
    pub fn kvm_read_guest(kvm: *mut kvm, gpa: gpa_t, data: *mut c_void, len: c_ulong) -> c_int;
}

extern "C" {
    pub fn kvm_clear_guest(kvm: *mut kvm, gpa: gpa_t, len: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_is_visible_gfn(kvm: *mut kvm, gfn: gfn_t) -> bool;
}
extern "C" {
    pub fn kvm_vcpu_is_visible_gfn(vcpu: *mut kvm_vcpu, gfn: gfn_t) -> bool;
}
extern "C" {
    pub fn kvm_host_page_size(vcpu: *mut kvm_vcpu, gfn: gfn_t) -> c_ulong;
}
extern "C" {
    pub fn mark_page_dirty_in_slot(kvm: *mut kvm, memslot: *const kvm_memory_slot, gfn: gfn_t);
}
extern "C" {
    pub fn mark_page_dirty(kvm: *mut kvm, gfn: gfn_t);
}
extern "C" {
    pub fn kvm_vcpu_mark_page_dirty(vcpu: *mut kvm_vcpu, gfn: gfn_t);
}
extern "C" {
    pub fn kvm_vcpu_unmap(vcpu: *mut kvm_vcpu, map: *mut kvm_host_map);
}
extern "C" {
    pub fn __kvm_vcpu_map(_arg: vcpu, _arg: gfn, _arg: map, _arg: true) -> return;
}
extern "C" {
    pub fn __kvm_vcpu_map(_arg: vcpu, _arg: gfn, _arg: map, _arg: false) -> return;
}

extern "C" {
    pub fn kvm_vcpu_gfn_to_hva(vcpu: *mut kvm_vcpu, gfn: gfn_t) -> c_ulong;
}
extern "C" {
    pub fn kvm_vcpu_gfn_to_hva_prot(vcpu: *mut kvm_vcpu, gfn: gfn_t, writable: *mut bool) -> c_ulong;
}
//
// kvm_gpc_init - initialize gfn_to_pfn_cache.
//
// @gpc:	   struct gfn_to_pfn_cache object.
// @kvm:	   pointer to kvm instance.
//
// This sets up a gfn_to_pfn_cache by initializing locks and assigning the
// immutable attributes.  Note, the cache must be zero-allocated (or zeroed by
// the caller before init).
//
extern "C" {
    pub fn kvm_gpc_init(gpc: *mut gfn_to_pfn_cache, kvm: *mut kvm);
}
//
// kvm_gpc_activate - prepare a cached kernel mapping and HPA for a given guest
// physical address.
//
// @gpc:	   struct gfn_to_pfn_cache object.
// @gpa:	   guest physical address to map.
// @len:	   sanity check; the range being access must fit a single page.
//
// @return:	   0 for success.
// -EINVAL for a mapping which would cross a page boundary.
// -EFAULT for an untranslatable guest physical address.
//
// This primes a gfn_to_pfn_cache and links it into the @gpc->kvm's list for
// invalidations to be processed.  Callers are required to use kvm_gpc_check()
// to ensure that the cache is valid before accessing the target page.
//
extern "C" {
    pub fn kvm_gpc_activate(gpc: *mut gfn_to_pfn_cache, gpa: gpa_t, len: c_ulong) -> c_int;
}
//
// kvm_gpc_activate_hva - prepare a cached kernel mapping and HPA for a given HVA.
//
// @gpc:          struct gfn_to_pfn_cache object.
// @hva:          userspace virtual address to map.
// @len:          sanity check; the range being access must fit a single page.
//
// @return:       0 for success.
// -EINVAL for a mapping which would cross a page boundary.
// -EFAULT for an untranslatable guest physical address.
//
// The semantics of this function are the same as those of kvm_gpc_activate(). It
// merely bypasses a layer of address translation.
//
extern "C" {
    pub fn kvm_gpc_activate_hva(gpc: *mut gfn_to_pfn_cache, hva: c_ulong, len: c_ulong) -> c_int;
}
//
// kvm_gpc_check - check validity of a gfn_to_pfn_cache.
//
// @gpc:	   struct gfn_to_pfn_cache object.
// @len:	   sanity check; the range being access must fit a single page.
//
// @return:	   %true if the cache is still valid and the address matches.
// %false if the cache is not valid.
//
// Callers outside IN_GUEST_MODE context should hold a read lock on @gpc->lock
// while calling this function, and then continue to hold the lock until the
// access is complete.
//
// Callers in IN_GUEST_MODE may do so without locking, although they should
// still hold a read lock on kvm->scru for the memslot checks.
//
extern "C" {
    pub fn kvm_gpc_check(gpc: *mut gfn_to_pfn_cache, len: c_ulong) -> bool;
}
//
// kvm_gpc_refresh - update a previously initialized cache.
//
// @gpc:	   struct gfn_to_pfn_cache object.
// @len:	   sanity check; the range being access must fit a single page.
//
// @return:	   0 for success.
// -EINVAL for a mapping which would cross a page boundary.
// -EFAULT for an untranslatable guest physical address.
//
// This will attempt to refresh a gfn_to_pfn_cache. Note that a successful
// return from this function does not mean the page can be immediately
// accessed because it may have raced with an invalidation. Callers must
// still lock and check the cache status, as this function does not return
// with the lock still held to permit access.
//
extern "C" {
    pub fn kvm_gpc_refresh(gpc: *mut gfn_to_pfn_cache, len: c_ulong) -> c_int;
}
//
// kvm_gpc_deactivate - deactivate and unlink a gfn_to_pfn_cache.
//
// @gpc:	   struct gfn_to_pfn_cache object.
//
// This removes a cache from the VM's list to be processed on MMU notifier
// invocation.
//
extern "C" {
    pub fn kvm_gpc_deactivate(gpc: *mut gfn_to_pfn_cache);
}
extern "C" {
    pub fn kvm_sigset_activate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_sigset_deactivate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_halt(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_block(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_arch_vcpu_blocking(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arch_vcpu_unblocking(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_wake_up(vcpu: *mut kvm_vcpu) -> bool;
}

extern "C" {
    pub fn __kvm_vcpu_kick(vcpu: *mut kvm_vcpu, wait: bool);
}

extern "C" {
    pub fn kvm_vcpu_yield_to(target: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_on_spin(vcpu: *mut kvm_vcpu, yield_to_kernel_mode: bool);
}
extern "C" {
    pub fn kvm_flush_remote_tlbs(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_flush_remote_tlbs_range(kvm: *mut kvm, gfn: gfn_t, nr_pages: u64);
}

extern "C" {
    pub fn kvm_mmu_topup_memory_cache(mc: *mut kvm_mmu_memory_cache, min: c_int) -> c_int;
}
extern "C" {
    pub fn __kvm_mmu_topup_memory_cache(mc: *mut kvm_mmu_memory_cache, capacity: c_int, min: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_mmu_memory_cache_nr_free_objects(mc: *mut kvm_mmu_memory_cache) -> c_int;
}
extern "C" {
    pub fn kvm_mmu_free_memory_cache(mc: *mut kvm_mmu_memory_cache);
}

extern "C" {
    pub fn kvm_mmu_invalidate_start(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_mmu_invalidate_range_add(kvm: *mut kvm, start: gfn_t, end: gfn_t);
}
extern "C" {
    pub fn kvm_mmu_invalidate_end(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_mmu_unmap_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
extern "C" {
    pub fn kvm_arch_vcpu_fault(vcpu: *mut kvm_vcpu, vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn kvm_vm_ioctl_check_extension(kvm: *mut kvm, ext: c_long) -> c_int;
}
extern "C" {
    pub fn kvm_arch_sync_dirty_log(kvm: *mut kvm, memslot: *mut kvm_memory_slot);
}

extern "C" {
    pub fn kvm_vm_ioctl_get_dirty_log(kvm: *mut kvm, log: *mut kvm_dirty_log) -> c_int;
}

extern "C" {
    pub fn kvm_arch_vm_ioctl(filp: *mut file, ioctl: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_ioctl_get_fpu(vcpu: *mut kvm_vcpu, fpu: *mut kvm_fpu) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_ioctl_set_fpu(vcpu: *mut kvm_vcpu, fpu: *mut kvm_fpu) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_ioctl_get_regs(vcpu: *mut kvm_vcpu, regs: *mut kvm_regs) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_ioctl_set_regs(vcpu: *mut kvm_vcpu, regs: *mut kvm_regs) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_ioctl_run(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn kvm_arch_vcpu_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arch_vcpu_precreate(kvm: *mut kvm, id: c_uint) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_create(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_postcreate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arch_vcpu_destroy(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvm_arch_pm_notifier(kvm: *mut kvm, state: c_ulong) -> c_int;
}

extern "C" {
    pub fn kvm_arch_create_vcpu_debugfs(vcpu: *mut kvm_vcpu, debugfs_dentry: *mut dentry);
}

//
// kvm_arch_shutdown() is invoked immediately prior to forcefully disabling
// hardware virtualization on all CPUs via IPI function calls (in preparation
// for shutdown or reboot), e.g. to allow arch code to prepare for disabling
// virtualization while KVM may be actively running vCPUs.
//
extern "C" {
    pub fn kvm_arch_shutdown();
}
//
// kvm_arch_{enable,disable}_virtualization() are called on one CPU, under
// kvm_usage_lock, immediately after/before 0=>1 and 1=>0 transitions of
// kvm_usage_count, i.e. at the beginning of the generic hardware enabling
// sequence, and at the end of the generic hardware disabling sequence.
//
extern "C" {
    pub fn kvm_arch_enable_virtualization();
}
extern "C" {
    pub fn kvm_arch_disable_virtualization();
}
//
// kvm_arch_{enable,disable}_virtualization_cpu() are called on "every" CPU to
// do the actual twiddling of hardware bits.  The hooks are called on all
// online CPUs when KVM enables/disabled virtualization, and on a single CPU
// when that CPU is onlined/offlined (including for Resume/Suspend).
//
extern "C" {
    pub fn kvm_arch_enable_virtualization_cpu() -> c_int;
}
extern "C" {
    pub fn kvm_arch_disable_virtualization_cpu();
}

extern "C" {
    pub fn kvm_vcpu_has_events(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_arch_vcpu_runnable(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_arch_vcpu_in_kernel(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_arch_vcpu_should_kick(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_arch_dy_runnable(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_arch_dy_has_pending_interrupt(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_arch_vcpu_preempted_in_kernel(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_arch_pre_destroy_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_arch_create_vm_debugfs(kvm: *mut kvm);
}
//
// All architectures that want to use vzalloc currently also
// need their own kvm_arch_alloc_vm implementation.
//
extern "C" {
    pub fn kzalloc_obj(kvm: struct, _arg: GFP_KERNEL_ACCOUNT) -> return;
}

extern "C" {
    pub fn kvm_arch_flush_remote_tlbs(kvm: *mut kvm) -> c_int;
}

extern "C" {
    pub fn kvm_arch_flush_remote_tlbs_range(kvm: *mut kvm, gfn: gfn_t, nr_pages: u64) -> c_int;
}

extern "C" {
    pub fn kvm_arch_register_noncoherent_dma(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_arch_unregister_noncoherent_dma(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_arch_has_noncoherent_dma(kvm: *mut kvm) -> bool;
}

//
// Wake a vCPU if necessary, but don't do any stats/metadata updates.  Returns
// true if the vCPU was blocking and was awakened, false otherwise.
//
extern "C" {
    pub fn rcuwait_active(_arg: kvm_arch_vcpu_get_wait(vcpu)) -> return;
}

//
// returns true if the virtual interrupt controller is initialized and
// ready to accept virtual IRQ. On some architectures the virtual interrupt
// controller is dynamically instantiated and this is not always true.
//
extern "C" {
    pub fn kvm_arch_intc_initialized(kvm: *mut kvm) -> bool;
}

extern "C" {
    pub fn kvm_arch_vcpu_get_ip(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn kvm_unregister_perf_callbacks();
}

extern "C" {
    pub fn kvm_arch_init_vm(kvm: *mut kvm, type: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_arch_destroy_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_cpu_has_pending_timer(vcpu: *mut kvm_vcpu) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_ack_notifier {
    pub link: hlist_node,
    pub gsi: unsigned,
    pub kian): *mut *mut void (irq_acked)(struct kvm_irq_ack_notifier,
}

extern "C" {
    pub fn kvm_irq_map_chip_pin(kvm: *mut kvm, irqchip: unsigned, pin: unsigned) -> c_int;
}
extern "C" {
    pub fn kvm_irq_has_notifier(kvm: *mut kvm, irqchip: unsigned, pin: unsigned) -> bool;
}
extern "C" {
    pub fn kvm_notify_acked_gsi(kvm: *mut kvm, gsi: c_int);
}
extern "C" {
    pub fn kvm_notify_acked_irq(kvm: *mut kvm, irqchip: unsigned, pin: unsigned);
}
extern "C" {
    pub fn kvm_arch_irqfd_allowed(kvm: *mut kvm, args: *mut kvm_irqfd) -> bool;
}
//
// Returns a pointer to the memslot if it contains gfn.
// Otherwise returns NULL.
//
// Returns a pointer to the memslot that contains gfn. Otherwise returns NULL.
//
// With "approx" set returns the memslot also when the address falls
// in a hole. In that case one of the memslots bordering the hole is
// returned.
//
// __gfn_to_memslot() and its descendants are here to allow arch code to inline
// the lookups in hot paths.  gfn_to_memslot() itself isn't here as an inline
// because that would bloat other code too much.
//
extern "C" {
    pub fn ____gfn_to_memslot(_arg: slots, _arg: gfn, _arg: false) -> return;
}
//
// The index was checked originally in search_memslots.  To avoid
// that a malicious guest builds a Spectre gadget out of e.g. page
// table walks, do not let the processor speculate loads outside
// the guest's registered memslots.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_stat_kind {
    KVM_STAT_VM,
    KVM_STAT_VCPU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_stat_data {
    pub kvm: *mut kvm,
    pub desc: *const kvm_stats_desc,
    pub kind: kvm_stat_kind,
}

// SCOPE: VM, VM_GENERIC, VCPU, VCPU_GENERIC

// Cumulative counter, read/write

// Instantaneous counter, read only

// Peak counter, read/write

// Instantaneous boolean value, read only

// Peak (sticky) boolean value, read/write

// Cumulative time in nanosecond

// Linear histogram for time in nanosecond

// Logarithmic histogram for time in nanosecond

//
// kvm_stats_linear_hist_update() - Update bucket value for linear histogram
// statistics data.
//
// @data: start address of the stats data
// @size: the number of bucket of the stats data
// @value: the new value used to update the linear histogram's bucket
// @bucket_size: the size (width) of a bucket
//
// kvm_stats_log_hist_update() - Update bucket value for logarithmic histogram
// statistics data.
//
// @data: start address of the stats data
// @size: the number of bucket of the stats data
// @value: the new value used to update the logarithmic histogram's bucket
//

//
// Ensure the read of mmu_invalidate_in_progress happens before
// the read of mmu_invalidate_seq.  This interacts with the
// smp_wmb() in mmu_notifier_invalidate_range_end to make sure
// that the caller either sees the old (non-zero) value of
// mmu_invalidate_in_progress or the new (incremented) value of
// mmu_invalidate_seq.
//
// PowerPC Book3s HV KVM calls this under a per-page lock rather
// than under kvm->mmu_lock, for scalability, so can't rely on
// kvm->mmu_lock to keep things ordered.
//
// If mmu_invalidate_in_progress is non-zero, then the range maintained
// by kvm_mmu_notifier_invalidate_range_start contains all addresses
// that might be being invalidated. Note that it may include some false
// positives, due to shortcuts when handing concurrent invalidations.
//
// Dropping mmu_lock after bumping mmu_invalidate_in_progress
// but before updating the range is a KVM bug.
//
// This lockless version of the range-based retry check *must* be paired with a
// call to the locked version after acquiring mmu_lock, i.e. this is safe to
// use only as a pre-check to avoid contending mmu_lock.  This version *will
// get false negatives and false positives.
//
// Use READ_ONCE() to ensure the in-progress flag and sequence counter
// are always read from memory, e.g. so that checking for retry in a
// loop won't result in an infinite retry loop.  Don't force loads for
// start+end, as the key to avoiding infinite retry loops is observing
// the 1=>0 transition of in-progress, i.e. getting false negatives
// due to stale start+end values is acceptable.
//

extern "C" {
    pub fn kvm_arch_can_set_irq_routing(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_init_irq_routing(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_free_irq_routing(kvm: *mut kvm);
}

extern "C" {
    pub fn kvm_send_userspace_msi(kvm: *mut kvm, msi: *mut kvm_msi) -> c_int;
}
extern "C" {
    pub fn kvm_eventfd_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_ioeventfd(kvm: *mut kvm, args: *mut kvm_ioeventfd) -> c_int;
}

extern "C" {
    pub fn kvm_irqfd(kvm: *mut kvm, args: *mut kvm_irqfd) -> c_int;
}
extern "C" {
    pub fn kvm_irqfd_release(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_irq_routing_update(: *mut kvm);
}

extern "C" {
    pub fn kvm_arch_irq_routing_update(kvm: *mut kvm);
}
//
// Ensure the rest of the request is published to kvm_check_request's
// caller.  Paired with the smp_mb__after_atomic in kvm_check_request.
//
// Request that don't require vCPU action should never be logged in
// vcpu->requests.  The vCPU won't clear the request, so it will stay
// logged indefinitely and prevent the vCPU from entering the guest.
//

extern "C" {
    pub fn READ_ONCE(_arg: vcpu->requests) -> return;
}
extern "C" {
    pub fn test_bit(KVM_REQUEST_MASK: req &, )&vcpu->requests: *mut (void) -> return;
}
//
// Ensure the rest of the request is visible to kvm_check_request's
// caller.  Paired with the smp_wmb in kvm_make_request.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_device {
    pub ops: *const kvm_device_ops,
    pub kvm: *mut kvm,
    pub private: *mut c_void,
    pub vm_node: list_head,
}

// create, destroy, and name are mandatory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_device_ops {
    pub name: *const c_char,
//
// create is called holding kvm->lock and any operations not suitable
// to do while holding the lock should be deferred to init (see
// below).
//
    pub type): *mut *mut *mut int (create)(struct kvm_device dev, u32,
//
// init is called after create if create is successful and is called
// outside of holding kvm->lock.
//
    pub dev): *mut *mut void (init)(struct kvm_device,
//
// Destroy is responsible for freeing dev.
//
// Destroy may be called before or after destructors are called
// on emulated I/O regions, depending on whether a reference is
// held by a vcpu or other kvm component that gets destroyed
// after the emulated I/O.
//
    pub dev): *mut *mut void (destroy)(struct kvm_device,
//
// Release is an alternative method to free the device. It is
// called when the device file descriptor is closed. Once
// release is called, the destroy method will not be called
// anymore as the device is removed from the device list of
// the VM. kvm->lock is held.
//
    pub dev): *mut *mut void (release)(struct kvm_device,
    pub attr): *mut *mut *mut int (set_attr)(struct kvm_device dev, struct kvm_device_attr,
    pub attr): *mut *mut *mut int (get_attr)(struct kvm_device dev, struct kvm_device_attr,
    pub attr): *mut *mut *mut int (has_attr)(struct kvm_device dev, struct kvm_device_attr,
    pub arg): c_ulong,
    pub vma): *mut *mut *mut int (mmap)(struct kvm_device dev, struct vm_area_struct,
}

extern "C" {
    pub fn kvm_register_device_ops(ops: *const kvm_device_ops, type: u32) -> c_int;
}
extern "C" {
    pub fn kvm_unregister_device_ops(type: u32);
}

extern "C" {
    pub fn kvm_arch_has_irq_bypass() -> bool;
}
extern "C" {
    pub fn kvm_arch_irq_bypass_stop(: *mut irq_bypass_consumer);
}
extern "C" {
    pub fn kvm_arch_irq_bypass_start(: *mut irq_bypass_consumer);
}

// If we wakeup during the poll time, was it a sucessful poll?

// Callback that tells if we must not poll
extern "C" {
    pub fn kvm_arch_no_poll(vcpu: *mut kvm_vcpu) -> bool;
}

extern "C" {
    pub fn kvm_arch_guest_memory_reclaimed(kvm: *mut kvm);
}

extern "C" {
    pub fn kvm_arch_vcpu_run_pid_change(vcpu: *mut kvm_vcpu) -> c_int;
}

//
// If more than one page is being (un)accounted, @virt must be the address of
// the first page of a block of pages what were allocated together (i.e
// accounted together).
//
// kvm_account_pgtable_pages() is thread-safe because mod_lruvec_page_state()
// is thread-safe.
//
// This defines how many reserved entries we want to keep before we
// kick the vcpu to the userspace to avoid dirty ring full.  This
// value can be tuned to higher if e.g. PML is enabled on the host.
//
pub const KVM_DIRTY_RING_RSVD_ENTRIES: c_int = 64;
// Max number of entries allowed for each kvm dirty ring
pub const KVM_DIRTY_RING_MAX_ENTRIES: c_int = 65536;
// RWX flags are not (yet) defined or communicated to userspace.

extern "C" {
    pub fn xa_to_value(_arg: xa_load(&kvm->mem_attr_array, _arg: gfn)) -> return;
}

//
// kvm_gmem_populate() - Populate/prepare a GPA range with guest data
//
// @kvm: KVM instance
// @gfn: starting GFN to be populated
// @src: userspace-provided buffer containing data to copy into GFN range
// (passed to @post_populate, and incremented on each iteration
// if not NULL). Must be page-aligned.
// @npages: number of pages to copy from userspace-buffer
// @post_populate: callback to issue for each gmem page that backs the GPA
// range
// @opaque: opaque data to pass to @post_populate callback
//
// This is primarily intended for cases where a gmem-backed GPA range needs
// to be initialized with userspace-provided data prior to being mapped into
// the guest as a private page. This should be called with the slots->lock
// held so that caller-enforced invariants regarding the expected memory
// attributes of the GPA range do not race with KVM_SET_MEMORY_ATTRIBUTES.
//
// Returns the number of pages that were populated.
//

extern "C" {
    pub fn kvm_arch_gmem_reclaim(pfn: kvm_pfn_t, nr_pages: kvm_pfn_t);
}

extern "C" {
    pub fn kvm_arch_gmem_invalidate_range(kvm: *mut kvm, range: *mut kvm_gfn_range);
}

