//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mm_types.h
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

pub const AT_VECTOR_SIZE_ARCH: c_int = 0;

//
// Each physical page in the system has a struct page associated with
// it to keep track of whatever it is we are using the page for at the
// moment. Note that we have no way to track which tasks are using
// a page, though if it is a pagecache page, rmap structures can tell us
// who is mapping it.
//
// If you allocate the page using alloc_pages(), you can use some of the
// space in struct page for your own purposes.  The five words in the main
// union are available, except for bit 0 of the first word which must be
// kept clear.  Many users use this word to store a pointer to an object
// which is guaranteed to be aligned.  If you use the same storage as
// page->mapping, you must restore it to NULL before freeing the page.
//
// The mapcount field must not be used for own purposes.
//
// If you want to use the refcount field, it must be used in such a way
// that other CPUs temporarily incrementing and then decrementing the
// refcount does not cause problems.  On receiving the page from
// alloc_pages(), the refcount will be positive.
//
// If you allocate pages of order > 0, you can use some of the fields
// in each subpage, but you may need to restore some of their values
// afterwards.
//
// SLUB uses cmpxchg_double() to atomically update its freelist and counters.
// That requires that freelist & counters in struct slab be adjacent and
// double-word aligned. Because struct slab currently just reinterprets the
// bits of struct page, we align all struct pages to double-word boundaries,
// and ensure that 'freelist' is aligned within struct slab.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page {
    pub possibly: *mut *mut memdesc_flags_t flags; / Atomic flags, some,
// updated asynchronously
//
// Five words (20/40 bytes) are available in this union.
// WARNING: bit 0 of the first word is used for PageTail(). That
// means the other users of this union MUST NOT use the bit to
// avoid collision and false-positive PageTail().
//
// @lru: Pageout list, eg. active_list protected by
// lruvec->lru_lock.  Sometimes used as a generic list
// by the page owner.
//
    pub lru: list_head,
// Or, free page
    pub buddy_list: list_head,
    pub pcp_list: list_head,
    pub pcp_llist: llist_node,
}

//
// @private: Mapping-private opaque data.
// Usually used for buffer_heads if PagePrivate.
// Used for swp_entry_t if swapcache flag set.
// Indicates order in the buddy system if PageBuddy
// or on pcp_llist.
//
// @pp_magic: magic value to avoid recycling non
// page_pool allocated pages.
//
// The first word is used for compound_info or folio
// pgmap
//
// ZONE_DEVICE private pages are counted as being
// mapped so the next 3 words hold the mapping, index,
// and private fields from the source anonymous or
// page cache page while the page is migrated to device
// private memory.
// ZONE_DEVICE MEMORY_DEVICE_FS_DAX pages also
// use the mapping, index, and private fields when
// pmem backed DAX files are mapped.
//
// @rcu_head: You can use this to free a page by RCU.
//
// For head pages of typed folios, the value stored here
// allows for determining what this page is used for. The
// tail pages of typed folios will not store a type
// (page_type == _mapcount == -1).
//
// See page-flags.h for a list of page types which are currently
// stored here.
//
// Owners of typed folios may reuse the lower 16 bit of the
// head page page_type field after setting the page type,
// but must reset these 16 bit to -1 before clearing the
// page type.
//
// For pages that are part of non-typed folios for which mappings
// are tracked via the RMAP, encodes the number of times this page
// is directly referenced by a page table.
//
// Note that the mapcount is always initialized to -1, so that
// transitions both from it and to it can be tracked, using
// atomic_inc_and_test() and atomic_add_negative(-1).
//
// Usage count. *DO NOT USE DIRECTLY*. See page_ref.h

//
// On machines where all RAM is mapped into kernel address space,
// we can simply calculate the virtual address. On machines with
// highmem some memory is mapped into kernel virtual memory
// dynamically, so we need a place to store that address.
// Note that this field could be 16 bits on x86 ... ;)
//
// Architectures with slow multiplication can define
// WANT_PAGE_VIRTUAL in asm/page.h
//

//
// KMSAN metadata for this page:
// - shadow page: every bit indicates whether the corresponding
// bit of the original page is initialized (0) or not (1);
// - origin page: every 4 bytes contain an id of the stack trace
// where the uninitialized value was created.
//

//
// struct encoded_page - a nonexistent type marking this pointer
//
// An 'encoded_page' pointer is a pointer to a regular 'struct page', but
// with the low bits of the pointer indicating extra context-dependent
// information. Only used in mmu_gather handling, and this acts as a type
// system check on that use.
//
// We only really have two guaranteed bits in general, although you could
// play with 'struct page' alignment (see CONFIG_HAVE_ALIGNED_STRUCT_PAGE)
// for more.
//
// Use the supplied helper functions to endcode/decode the pointer and bits.
//

// Perform rmap removal after we have flushed the TLB.

//
// The next item in an encoded_page array is the "nr_pages" argument, specifying
// the number of consecutive pages starting from this page, that all belong to
// the same folio. For example, "nr_pages" corresponds to the number of folio
// references that must be dropped. If this bit is not set, "nr_pages" is
// implicitly 1.
//

//
// A swap entry has to fit into a "unsigned long", as the entry is hidden
// in the "index" field of the swapper address space.
//
// typedef softleaf_t - Describes a page table software leaf entry, abstracted
// from its architecture-specific encoding.
//
// Page table leaf entries are those which do not reference any descendent page
// tables but rather either reference a data page, are an empty (or 'none'
// entry), or contain a non-present entry.
//
// If referencing another page table or a data page then the page table entry is
// pertinent to hardware - that is it tells the hardware how to decode the page
// table entry.
//
// Otherwise it is a software-defined leaf page table entry, which this type
// describes. See leafops.h and specifically @softleaf_type for a list of all
// possible kinds of software leaf entry.
//
// A softleaf_t entry is abstracted from the hardware page table entry, so is
// not architecture-specific.
//
// NOTE: While we transition from the confusing swp_entry_t type used for this
// purpose, we simply alias this type. This will be removed once the
// transition is complete.
//
pub type softleaf_t = swp_entry_t;

// We have some extra room after the refcount in tail pages.
// Macro flag: #define NR_PAGES_IN_LARGE_FOLIO

//
// On 32bit, we can cut the required metadata in half, because:
// (a) PID_MAX_LIMIT implicitly limits the number of MMs we could ever have,
// so we can limit MM IDs to 15 bit (32767).
// (b) We don't expect folios where even a single complete PTE mapping by
// one MM would exceed 15 bits (order-15).
//

pub type mm_id_mapcount_t = c_int;

pub type mm_id_t = c_uint;

pub type mm_id_mapcount_t = c_short;

pub type mm_id_t = c_ushort;

// We implicitly use the dummy ID for init-mm etc. where we never rmap pages.
pub const MM_ID_DUMMY: c_int = 0;

//
// We leave the highest bit of each MM id unused, so we can store a flag
// in the highest bit of each folio->_mm_id[].
//

//
// In order to use bit_spin_lock(), which requires an unsigned long, we
// operate on folio->_mm_ids when working on flags.
//

//
// struct folio - Represents a contiguous set of bytes.
// @flags: Identical to the page flags.
// @lru: Least Recently Used list; tracks how recently this folio was used.
// @mlock_count: Number of times this folio has been pinned by mlock().
// @mapping: The file this page belongs to, or refers to the anon_vma for
// anonymous memory.
// @index: Offset within the file, in units of pages.  For anonymous memory,
// this is the index from the beginning of the mmap.
// @share: number of DAX mappings that reference this folio. See
// dax_associate_entry.
// @private: Filesystem per-folio data (see folio_attach_private()).
// @swap: Used for swp_entry_t if folio_test_swapcache().
// @migrate_info: Stores migration state (anon_vma pointer and
// FOLIO_WAS_* markers).
// @_mapcount: Do not access this member directly.  Use folio_mapcount() to
// find out how many times this folio is mapped by userspace.
// @_refcount: Do not access this member directly.  Use folio_ref_count()
// to find how many references there are to this folio.
// @memcg_data: Memory Control Group data.
// @pgmap: Metadata for ZONE_DEVICE mappings
// @virtual: Virtual address in the kernel direct map.
// @_last_cpupid: IDs of last CPU and last process that accessed the folio.
// @_entire_mapcount: Do not use directly, call folio_entire_mapcount().
// @_large_mapcount: Do not use directly, call folio_mapcount().
// @_nr_pages_mapped: Do not use outside of rmap and debug code.
// @_pincount: Do not use directly, call folio_maybe_dma_pinned().
// @_nr_pages: Do not use directly, call folio_nr_pages().
// @_mm_id: Do not use outside of rmap code.
// @_mm_ids: Do not use outside of rmap code.
// @_mm_id_mapcount: Do not use outside of rmap code.
// @_hugetlb_subpool: Do not use directly, use accessor in hugetlb.h.
// @_hugetlb_cgroup: Do not use directly, use accessor in hugetlb_cgroup.h.
// @_hugetlb_cgroup_rsvd: Do not use directly, use accessor in hugetlb_cgroup.h.
// @_hugetlb_hwpoison: Do not use directly, call raw_hwp_list_head().
// @_deferred_list: Folios to be split under memory pressure.
// @_unused_slab_obj_exts: Placeholder to match obj_exts in struct slab.
//
// A folio is a physically, virtually and logically contiguous set
// of bytes.  It is a power-of-two in size, and it is aligned to that
// same power-of-two.  It is at least as large as %PAGE_SIZE.  If it is
// in the page cache, it is at a file offset which is a multiple of that
// power-of-two.  It may be mapped into userspace at an address which is
// at an arbitrary page offset, but its kernel virtual address is aligned
// to its size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio {
// private: don't document the anon union
// public:
    pub flags: memdesc_flags_t,
    pub lru: list_head,
// private: avoid cluttering the output
// For the Unevictable "LRU list" slot
// Avoid compound_info
    pub __filler: *mut c_void,
// public:
    pub mlock_count: c_uint,
// private:
}

// public:

// private: the union with struct page is transitional
// public:

// private: the union with struct page is transitional
// public:

// private: the union with struct page is transitional
// public:

// private: the union with struct page is transitional
// public:
// private: the union with struct page is transitional

//
// struct ptdesc -    Memory descriptor for page tables.
// @pt_flags: enum pt_flags plus zone/node/section.
// @pt_rcu_head:      For freeing page table pages.
// @pt_list:          List of used page tables. Used for s390 gmap shadow pages
// (which are not linked into the user page tables) and x86
// pgds.
// @_pt_pad_1:        Padding that aliases with page's compound head.
// @pmd_huge_pte:     Protected by ptdesc->ptl, used for THPs.
// @__page_mapping:   Aliases with page->mapping. Unused for page tables.
// @pt_index:         Used for s390 gmap.
// @pt_mm:            Used for x86 pgds.
// @pt_frag_refcount: For fragmented page table tracking. Powerpc only.
// @pt_share_count:   Used for HugeTLB PMD page table share count.
// @_pt_pad_2:        Padding to ensure proper alignment.
// @ptl:              Lock for the page table.
// @__page_type:      Same as page->page_type. Unused for page tables.
// @__page_refcount:  Same as page refcount.
// @pt_memcg_data:    Memcg data. Tracked for page tables here.
//
// This struct overlays struct page for now. Do not modify without a good
// understanding of the issues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdesc {
    pub pt_flags: memdesc_flags_t,
    pub pt_rcu_head: rcu_head,
    pub pt_list: list_head,
    pub _pt_pad_1: c_ulong,
    pub pmd_huge_pte: pgtable_t,
}

extern "C" {
    pub fn atomic_read(_arg: &ptdesc->pt_share_count) -> return;
}

//
// Used for sizing the vmemmap region on some architectures
//

//
// page_private can be used on tail pages.  However, PagePrivate is only
// checked by the VM on the head page.  So page_private on the tail pages
// should be used for data that's ancillary to the head page (eg attaching
// buffer heads to tail pages after attaching buffer heads to the head page)
//

pub type vm_flags_t = c_ulong;
//
// freeptr_t represents a SLUB freelist pointer, which might be encoded
// and not dereferenceable if CONFIG_SLAB_FREELIST_HARDENED is enabled.
//
// A region containing a mapping of a non-memory backed file under NOMMU
// conditions.  These are held in a global tree and are pinned by the VMAs that
// map parts of them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_region {
    pub /: *mut *mut rb_node vm_rb; / link in global region tree,
    pub /: *mut *mut vm_flags_t vm_flags; / VMA vm_flags,
    pub /: *mut *mut unsigned long vm_start; / start address of region,
    pub /: *mut *mut unsigned long vm_end; / region initialised to here,
    pub /: *mut *mut unsigned long vm_top; / region allocated to here,
    pub /: *mut *mut unsigned long vm_pgoff; / the offset in vm_file corresponding to vm_start,
    pub /: *mut *mut *mut file vm_file; / the backing file or NULL,
    pub /: *mut *mut int vm_usage; / region usage count (access under nommu_region_sem),
    pub for: *mut *mut bool vm_icache_flushed : 1; / true if the icache has been flushed,
// this region
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_userfaultfd_ctx {
    pub ctx: *mut userfaultfd_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_userfaultfd_ctx {

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_vma_name {
    pub kref: kref,
// The name needs to be at the end because it is dynamically sized.
    pub name: [c_char; ],
}

//
// mmap_lock should be read-locked when calling anon_vma_name(). Caller should
// either keep holding the lock while using the returned pointer or it should
// raise anon_vma_name refcount before releasing the lock.
//
extern "C" {
    pub fn anon_vma_name_free(kref: *mut kref);
}

//
// While __vma_enter_locked() is working to ensure are no read-locks held on a
// VMA (either while acquiring a VMA write lock or marking a VMA detached) we
// set the VM_REFCNT_EXCLUDE_READERS_FLAG in vma->vm_refcnt to indiciate to
// vma_start_read() that the reference count should be left alone.
//
// See the comment describing vm_refcnt in vm_area_struct for details as to
// which values the VMA reference count can be.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_numab_state {
//
// Initialised as time in 'jiffies' after which VMA
// should be scanned.  Delays first scan of new VMA by at
// least sysctl_numa_balancing_scan_delay:
//
    pub next_scan: c_ulong,
//
// Time in jiffies when pids_active[] is reset to
// detect phase change behaviour:
//
    pub pids_active_reset: c_ulong,
//
// Approximate tracking of PIDs that trapped a NUMA hinting
// fault. May produce false positives due to hash collisions.
//
// [0] Previous PID tracking
// [1] Current PID tracking
//
// Window moves after next_pid_reset has expired approximately
// every VMA_PID_RESET_PERIOD jiffies:
//
    pub pids_active: [c_ulong; 2],
// MM scan sequence ID when scan first started after VMA creation
    pub start_scan_seq: c_int,
//
// MM scan sequence ID when the VMA was last completely scanned.
// A VMA is not eligible for scanning if prev_scan_seq == numa_scan_seq
//
    pub prev_scan_seq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfnmap_track_ctx {
    pub kref: kref,
    pub pfn: c_ulong,
    pub /: *mut *mut unsigned long size; / in bytes,
}

// What action should be taken after an .mmap_prepare call is complete?
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmap_action_type {
    MMAP_NOTHING,		/* Mapping is complete, no further action. */
    MMAP_REMAP_PFN,		/* Remap PFN range. */
    MMAP_IO_REMAP_PFN,	/* I/O remap PFN range. */
    MMAP_SIMPLE_IO_REMAP,	/* I/O remap with guardrails. */
    MMAP_MAP_KERNEL_PAGES,	/* Map kernel page range from array. */
}

//
// Describes an action an mmap_prepare hook can instruct to be taken to complete
// the mapping of a VMA. Specified in vm_area_desc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_action {
    pub start: c_ulong,
    pub start_pfn: c_ulong,
    pub size: c_ulong,
    pub pgprot: pgprot_t,
    pub remap: },
    pub start_phys_addr: phys_addr_t,
    pub size: c_ulong,
    pub simple_ioremap: },
    pub start: c_ulong,
    pub pages: *mut page,
    pub nr_pages: c_ulong,
    pub pgoff: pgoff_t,
    pub map_kernel: },
}

//
// If non-zero, replace errors that arise from mmap actions with this
// value instead. Only valid error codes may be specified.
//
// This should be set in rare instances where the operation required
// that the rmap should not be able to access the VMA until
// completely set up.
//
// Opaque type representing current VMA (vm_area_struct) flag state. Must be
// accessed via vma_flags_xxx() helper functions.
//

// Are no flags set in the specified VMA flags?
extern "C" {
    pub fn bitmap_empty(_arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}
//
// Describes a VMA that is about to be mmap()'ed. Drivers may choose to
// manipulate mutable fields which will cause those fields to be updated in the
// resultant VMA.
//
// Helper functions are not required for manipulating any field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_area_desc {
// Immutable state.
    pub mm: *mut mm_struct,
    pub /: *mut *mut *mut file file; / May vary from vm_file in stacked callers.,
    pub start: c_ulong,
    pub end: c_ulong,
// Mutable fields. Populated with initial state.
    pub pgoff: pgoff_t,
    pub vm_file: *mut file,
    pub vma_flags: vma_flags_t,
    pub page_prot: pgprot_t,
// Write-only fields.
    pub vm_ops: *const vm_operations_struct,
    pub private_data: *mut c_void,
// Take further action?
    pub action: mmap_action,
}

//
// This struct describes a virtual memory area. There is one of these
// per VM-area/task. A VM area is any part of the process virtual memory
// space that has a special rule for the page-fault handlers (ie a shared
// library, the executable area etc).
//
// Only explicitly marked struct members may be accessed by RCU readers before
// getting a stable reference.
//
// WARNING: when adding new members, please update vm_area_init_from() to copy
// them during vm_area_struct content duplication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_area_struct {
// The first cache line has the info for VMA tree walking.
// VMA covers [vm_start; vm_end) addresses within mm
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
}

//
// The address space we belong to.
// Unstable RCU readers are allowed to read this.
//
// Flags, see mm.h.
// To modify use vm_flags_{init|reset|set|clear|mod} functions.
// Preferably, use vma_flags_xxx() functions.
//
// Temporary while VMA flags are being converted.

//
// Can only be written (using WRITE_ONCE()) while holding both:
// - mmap_lock (in write mode)
// - vm_refcnt bit at VM_REFCNT_EXCLUDE_READERS_FLAG is set
// Can be read reliably while holding one of:
// - mmap_lock (in read or write mode)
// - vm_refcnt bit at VM_REFCNT_EXCLUDE_READERS_BIT is set or vm_refcnt > 1
// Can be read unreliably (using READ_ONCE()) for pessimistic bailout
// while holding nothing (except RCU to keep the VMA struct allocated).
//
// This sequence counter is explicitly allowed to overflow; sequence
// counter reuse can only lead to occasional unnecessary use of the
// slowpath.
//

//
// Low 32-bits of anonymous page offset.
// See vma_start_anon_pgoff() comment for details.
//
// A file's MAP_PRIVATE vma can be in both i_mmap tree and anon_vma
// list, after a COW of one of the file pages.	A MAP_SHARED vma
// can only be in the i_mmap tree.  An anonymous MAP_PRIVATE, stack
// or brk vma (with NULL file) can only be in an anon_vma list.
//
// page_table_lock
// Function pointers to deal with this struct.
// Information about our backing store:

//
// Used to keep track of firstly, whether the VMA is attached, secondly,
// if attached, how many read locks are taken, and thirdly, if the
// VM_REFCNT_EXCLUDE_READERS_FLAG is set, whether any read locks held
// are currently in the process of being excluded.
//
// This value can be equal to:
//
// 0 - Detached. IMPORTANT: when the refcnt is zero, readers cannot
// increment it.
//
// 1 - Attached and either unlocked or write-locked. Write locks are
// identified via __is_vma_write_locked() which checks for equality of
// vma->vm_lock_seq and mm->mm_lock_seq.
//
// >1, < VM_REFCNT_EXCLUDE_READERS_FLAG - Read-locked or (unlikely)
// write-locked with other threads having temporarily incremented the
// reference count prior to determining it is write-locked and
// decrementing it again.
//
// VM_REFCNT_EXCLUDE_READERS_FLAG - Detached, pending
// __vma_end_exclude_readers() completion which will decrement the
// reference count to zero. IMPORTANT - at this stage no further readers
// can increment the reference count. It can only be reduced.
//
// VM_REFCNT_EXCLUDE_READERS_FLAG + 1 - A thread is either write-locking
// an attached VMA and has yet to invoke __vma_end_exclude_readers(),
// OR a thread is detaching a VMA and is waiting on a single spurious
// reader in order to decrement the reference count. IMPORTANT - as
// above, no further readers can increment the reference count.
//
// > VM_REFCNT_EXCLUDE_READERS_FLAG + 1 - A thread is either
// write-locking or detaching a VMA is waiting on readers to
// exit. IMPORTANT - as above, no further readers can increment the
// reference count.
//
// NOTE: Unstable RCU readers are allowed to read this.
//

//
// High 32-bits of anonymous page offset.
// See vma_start_anon_pgoff() comment for details.
//

//
// For areas with an address space and backing store,
// linkage into the address_space->i_mmap interval tree.
//

//
// For private and shared anonymous mappings, a pointer to a null
// terminated string containing the name given to the vma, or NULL if
// unnamed. Serialized by mmap_lock. Use anon_vma_name to access.
//

// Clears all bits in the VMA flags bitmap, non-atomically.
//
// Helper function which converts a vma_flags_t value to a legacy vm_flags_t
// value. This is only valid if the input flags value can be expressed in a
// system word.
//
// Will be removed once the conversion to VMA flags is complete.
//
// Copy value to the first system word of VMA flags, non-atomically.
//
// IMPORTANT: This does not overwrite bytes past the first system word. The
// caller must account for this.
//
// Helper function which converts a legacy vm_flags_t value to a vma_flags_t
// value.
//
// Will be removed once the conversion to VMA flags is complete.
//
// Copy value to the first system word of VMA flags ONCE, non-atomically.
//
// IMPORTANT: This does not overwrite bytes past the first system word. The
// caller must account for this.
//
// Update the first system word of VMA flags setting bits, non-atomically.
// bitmap |= value;
// Update the first system word of VMA flags clearing bits, non-atomically.
// bitmap &= ~value;

//
// Opaque type representing current mm_struct flag state. Must be accessed via
// mm_flags_xxx() helper functions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_struct {
//
// Fields which are often written to are placed in a separate
// cache line.
//
// @mm_count: The number of references to &struct
// mm_struct (@mm_users count as 1).
//
// Use mmgrab()/mmdrop() to modify. When this drops to
// 0, the &struct mm_struct is freed.
//
    pub mm_count: core::sync::atomic::AtomicI32,
    pub ____cacheline_aligned_in_smp: },
    pub mm_mt: maple_tree,
    pub /: *mut *mut unsigned long mmap_base; / base of mmap area,
    pub /: *mut *mut unsigned long mmap_legacy_base; / base of mmap area in bottom-up allocations,

// Base addresses for compatible mmap()
    pub mmap_compat_base: c_ulong,
    pub mmap_compat_legacy_base: c_ulong,

    pub /: *mut *mut unsigned long task_size; / size of task vm space,
    pub pgd: *mut *mut pgd_t,

//
// @membarrier_state: Flags controlling membarrier behavior.
//
// This field is close to @pgd to hopefully fit in the same
// cache-line, which needs to be touched by switch_mm().
//
    pub membarrier_state: core::sync::atomic::AtomicI32,

//
// @mm_users: The number of users including userspace.
//
// Use mmget()/mmget_not_zero()/mmput() to modify. When this
// drops to 0 (i.e. when the task exits and there are no other
// temporary reference holders), we also release a reference on
// @mm_count (which may then free the &struct mm_struct if
// @mm_count also drops to 0).
//
    pub mm_users: core::sync::atomic::AtomicI32,
// MM CID related storage
    pub mm_cid: mm_mm_cid,
// sched_cache related statistics
    pub sc_stat: sched_cache_stat,

    pub /: *mut *mut atomic_long_t pgtables_bytes; / size of all page tables,

    pub /: *mut *mut int map_count; / number of VMAs,
    pub some: *mut *mut spinlock_t page_table_lock; / Protects page tables and,
// counters
//
// Typically the current mmap_lock's offset is 56 bytes from
// the last cacheline boundary, which is very optimal, as
// its two hot fields 'count' and 'owner' sit in 2 different
// cachelines, and when mmap_lock is highly contended, both
// of the 2 fields will be accessed frequently, current layout
// will help to reduce cache bouncing.
//
// So please be careful with adding new fields before
// mmap_lock, which can easily push the 2 fields into one
// cacheline.
//
    pub mmap_lock: rw_semaphore,
    pub These: *mut *mut list_head mmlist; / List of maybe swapped mm's.,
// are globally strung together off
// init_mm.mmlist, and are protected
// by mmlist_lock
//

    pub vma_writer_wait: rcuwait,
//
// This field has lock-like semantics, meaning it is sometimes
// accessed with ACQUIRE/RELEASE semantics.
// Roughly speaking, incrementing the sequence number is
// equivalent to releasing locks on VMAs; reading the sequence
// number can be part of taking a read lock on a VMA.
// Incremented every time mmap_lock is write-locked/unlocked.
// Initialized to 0, therefore odd values indicate mmap_lock
// is write-locked and even values that it's released.
//
// Can be modified under write mmap_lock using RELEASE
// semantics.
// Can be read with no other protection when holding write
// mmap_lock.
// Can be read with ACQUIRE semantics if not holding write
// mmap_lock.
//
    pub mm_lock_seq: seqcount_t,

    pub futex: futex_mm_data,
    pub /: *mut *mut unsigned long hiwater_rss; / High-watermark of RSS usage,
    pub /: *mut *mut unsigned long hiwater_vm; / High-water virtual memory usage,
    pub /: *mut *mut unsigned long total_vm; / Total pages mapped,
    pub /: *mut *mut unsigned long locked_vm; / Pages that have PG_mlocked set,
    pub /: *mut *mut atomic64_t pinned_vm; / Refcount permanently increased,
    pub /: *mut *mut unsigned long data_vm; / VM_WRITE & ~VM_SHARED & ~VM_STACK,
    pub /: *mut *mut unsigned long exec_vm; / VM_EXEC & ~VM_WRITE & ~VM_STACK,
    pub /: *mut *mut unsigned long stack_vm; / VM_STACK,
// Temporary while VMA flags are being converted.
    pub def_flags: vm_flags_t,
    pub def_vma_flags: vma_flags_t,
}

//
// @write_protect_seq: Locked when any thread is write
// protecting pages mapped by this mm to enforce a later COW,
// for instance during page table copying for fork().
//

// the ABI-related flags from the ELF header. Used for core dump

// Architecture-specific MM context

//
// "owner" points to a task that is regarded as the canonical
// user/owner of this mm. All of the following must be true in
// order for it to be changed:
//
// current == mm->owner
// current->mm != mm
// new_owner->mm == mm
// new_owner->alloc_lock is held
//

// store ref to file /proc/<pid>/exe symlink points to

//
// numa_next_scan is the next time that PTEs will be remapped
// PROT_NONE to trigger NUMA hinting faults; such faults gather
// statistics and migrate pages to new nodes if necessary.
//
// Restart point for scanning and remapping PTEs.
// numa_scan_seq prevents two threads remapping PTEs.

//
// An operation with batched TLB flushing is going on. Anything
// that can move process memory needs to flush the TLB when
// moving a PROT_NONE mapped page.
//

// See flush_tlb_batched_pending()

//
// Represent how many pages of this process are involved in KSM
// merging (not including ksm_zero_pages).
//
// Represent how many pages are checked for ksm merging
// including merged and not merged.
//
// Represent how many empty pages are merged with kernel zero
// pages when enabling KSM use_zero_pages.
//

// this mm_struct is on lru_gen_mm_list
//
// Set when switching to this mm_struct, as a hint of
// whether it has been used since the last time per-node
// page table walkers cleared the corresponding bits.
//

// points to the memcg of "owner" above

//
// The mm_cpumask needs to be at the end of mm_struct, because it
// is dynamically sized based on nr_cpu_ids.
//
// Copy value to the first system word of mm flags, non-atomically.
// ACCESS_PRIVATE(&mm->flags, __mm_flags) = value;
// Obtain a read-only view of the mm flags bitmap.
// Read the first system word of mm flags, non-atomically.
//
// Update the first system word of mm flags ONLY, applying the specified mask to
// it, then setting all flags specified by bits.
//

// Pointer magic because the dynamic array size confuses some compilers.
// Future-safe accessor for struct mm_struct's cpu_vm_mask.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_gen_mm_list {
// mm_struct list for page table walkers
    pub fifo: list_head,
// protects the list above
    pub lock: spinlock_t,
}

extern "C" {
    pub fn lru_gen_add_mm(mm: *mut mm_struct);
}
extern "C" {
    pub fn lru_gen_del_mm(mm: *mut mm_struct);
}
extern "C" {
    pub fn lru_gen_migrate_mm(mm: *mut mm_struct);
}

//
// When the bitmap is set, page reclaim knows this mm_struct has been
// used since the last time it cleared the bitmap. So it might be worth
// walking the page tables of this mm_struct to clear the accessed bit.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_iterator {
    pub mas: ma_state,
}

//
// mm_cpus_allowed: Union of all mm's threads allowed CPUs.
//
// Skip cpu_bitmap
// Accessor for struct mm_struct's cidmask.
// Skip mm_cpus_allowed
extern "C" {
    pub fn mm_init_cid(mm: *mut mm_struct, p: *mut task_struct);
}

// mm_cpus_allowed(), mm_cidmask().
extern "C" {
    pub fn cpumask_size(bitmap_size(num_possible_cpus(): ) +) -> return;
}
// Use 2 * NR_CPUS as worse case for static allocation.

extern "C" {
    pub fn tlb_gather_mmu(tlb: *mut mmu_gather, mm: *mut mm_struct);
}
extern "C" {
    pub fn tlb_gather_mmu_fullmm(tlb: *mut mmu_gather, mm: *mut mm_struct);
}
extern "C" {
    pub fn tlb_gather_mmu_vma(tlb: *mut mmu_gather, vma: *mut vm_area_struct);
}
extern "C" {
    pub fn tlb_finish_mmu(tlb: *mut mmu_gather);
}
//
// typedef vm_fault_t - Return type for page fault handlers.
//
// Page fault handlers return a bitmask of %VM_FAULT values.
//
pub type vm_fault_t =  unsigned int;
//
// enum vm_fault_reason - Page fault handlers return a bitmask of
// these values to tell the core VM what happened when handling the
// fault. Used to decide whether a process gets delivered SIGBUS or
// just gets major/minor fault counters bumped up.
//
// @VM_FAULT_OOM:		Out Of Memory
// @VM_FAULT_SIGBUS:		Bad access
// @VM_FAULT_MAJOR:		Page read from storage
// @VM_FAULT_HWPOISON:		Hit poisoned small page
// @VM_FAULT_HWPOISON_LARGE:	Hit poisoned large page. Index encoded
// in upper bits
// @VM_FAULT_SIGSEGV:		segmentation fault
// @VM_FAULT_NOPAGE:		->fault installed the pte, not return page
// @VM_FAULT_LOCKED:		->fault locked the returned page
// @VM_FAULT_RETRY:		->fault blocked, must retry
// @VM_FAULT_FALLBACK:		huge page fault failed, fall back to small
// @VM_FAULT_DONE_COW:		->fault has fully handled COW
// @VM_FAULT_NEEDDSYNC:		->fault did not modify page tables and needs
// fsync() to complete (for synchronous page faults
// in DAX)
// @VM_FAULT_COMPLETED:		->fault completed, meanwhile mmap lock released
// @VM_FAULT_HINDEX_MASK:	mask HINDEX value
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vm_fault_reason {
    VM_FAULT_OOM            = ( vm_fault_t)0x000001,
    VM_FAULT_SIGBUS         = ( vm_fault_t)0x000002,
    VM_FAULT_MAJOR          = ( vm_fault_t)0x000004,
    VM_FAULT_HWPOISON       = ( vm_fault_t)0x000010,
    VM_FAULT_HWPOISON_LARGE = ( vm_fault_t)0x000020,
    VM_FAULT_SIGSEGV        = ( vm_fault_t)0x000040,
    VM_FAULT_NOPAGE         = ( vm_fault_t)0x000100,
    VM_FAULT_LOCKED         = ( vm_fault_t)0x000200,
    VM_FAULT_RETRY          = ( vm_fault_t)0x000400,
    VM_FAULT_FALLBACK       = ( vm_fault_t)0x000800,
    VM_FAULT_DONE_COW       = ( vm_fault_t)0x001000,
    VM_FAULT_NEEDDSYNC      = ( vm_fault_t)0x002000,
    VM_FAULT_COMPLETED      = ( vm_fault_t)0x004000,
    VM_FAULT_HINDEX_MASK    = ( vm_fault_t)0x0f0000,
}

// Encode hstate index for a hwpoisoned large page

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_special_mapping {
    pub /: *const *const *const char name; / The name, e.g. "[vdso]".,
//
// If .fault is not provided, this points to a
// NULL-terminated array of pages that back the special mapping.
//
// This must not be NULL unless .fault is provided.
//
    pub pages: *mut page,
//
// If non-NULL, then this is called to resolve page faults
// on the special mapping.  If used, .pages is not checked.
//
    pub vmf): *mut vm_fault,
    pub new_vma): *mut vm_area_struct,
    pub vma): *mut vm_area_struct,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tlb_flush_reason {
    TLB_FLUSH_ON_TASK_SWITCH,
    TLB_REMOTE_SHOOTDOWN,
    TLB_LOCAL_SHOOTDOWN,
    TLB_LOCAL_MM_SHOOTDOWN,
    TLB_REMOTE_SEND_IPI,
    TLB_REMOTE_WRONG_CPU,
}

//
// enum fault_flag - Fault flag definitions.
// @FAULT_FLAG_WRITE: Fault was a write fault.
// @FAULT_FLAG_MKWRITE: Fault was mkwrite of existing PTE.
// @FAULT_FLAG_ALLOW_RETRY: Allow to retry the fault if blocked.
// @FAULT_FLAG_RETRY_NOWAIT: Don't drop mmap_lock and wait when retrying.
// @FAULT_FLAG_KILLABLE: The fault task is in SIGKILL killable region.
// @FAULT_FLAG_TRIED: The fault has been tried once.
// @FAULT_FLAG_USER: The fault originated in userspace.
// @FAULT_FLAG_REMOTE: The fault is not for current task/mm.
// @FAULT_FLAG_INSTRUCTION: The fault was during an instruction fetch.
// @FAULT_FLAG_INTERRUPTIBLE: The fault can be interrupted by non-fatal signals.
// @FAULT_FLAG_UNSHARE: The fault is an unsharing request to break COW in a
// COW mapping, making sure that an exclusive anon page is
// mapped after the fault.
// @FAULT_FLAG_ORIG_PTE_VALID: whether the fault has vmf->orig_pte cached.
// We should only access orig_pte if this flag set.
// @FAULT_FLAG_VMA_LOCK: The fault is handled under VMA lock.
//
// About @FAULT_FLAG_ALLOW_RETRY and @FAULT_FLAG_TRIED: we can specify
// whether we would allow page faults to retry by specifying these two
// fault flags correctly.  Currently there can be three legal combinations:
//
// (a) ALLOW_RETRY and !TRIED:  this means the page fault allows retry, and
// this is the first try
//
// (b) ALLOW_RETRY and TRIED:   this means the page fault allows retry, and
// we've already tried at least once
//
// (c) !ALLOW_RETRY and !TRIED: this means the page fault does not allow retry
//
// The unlisted combination (!ALLOW_RETRY && TRIED) is illegal and should never
// be used.  Note that page faults can be allowed to retry for multiple times,
// in which case we'll have an initial fault with flags (a) then later on
// continuous faults with flags (b).  We should always try to detect pending
// signals before a retry to make sure the continuous page faults can still be
// interrupted if necessary.
//
// The combination FAULT_FLAG_WRITE|FAULT_FLAG_UNSHARE is illegal.
// FAULT_FLAG_UNSHARE is ignored and treated like an ordinary read fault when
// applied to mappings that are not COW mappings.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fault_flag {
    FAULT_FLAG_WRITE =		1 << 0,
    FAULT_FLAG_MKWRITE =		1 << 1,
    FAULT_FLAG_ALLOW_RETRY =	1 << 2,
    FAULT_FLAG_RETRY_NOWAIT = 	1 << 3,
    FAULT_FLAG_KILLABLE =		1 << 4,
    FAULT_FLAG_TRIED = 		1 << 5,
    FAULT_FLAG_USER =		1 << 6,
    FAULT_FLAG_REMOTE =		1 << 7,
    FAULT_FLAG_INSTRUCTION =	1 << 8,
    FAULT_FLAG_INTERRUPTIBLE =	1 << 9,
    FAULT_FLAG_UNSHARE =		1 << 10,
    FAULT_FLAG_ORIG_PTE_VALID =	1 << 11,
    FAULT_FLAG_VMA_LOCK =		1 << 12,
}

pub type zap_flags_t = u32;
// Flags for clear_young_dirty_ptes().
pub type cydp_t = int ;
// Clear the access bit

// Clear the dirty bit

//
// FOLL_PIN and FOLL_LONGTERM may be used in various combinations with each
// other. Here is what they mean, and how to use them:
//
// FIXME: For pages which are part of a filesystem, mappings are subject to the
// lifetime enforced by the filesystem and we need guarantees that longterm
// users like RDMA and V4L2 only establish mappings which coordinate usage with
// the filesystem.  Ideas for this coordination include revoking the longterm
// pin, delaying writeback, bounce buffer page writeback, etc.  As FS DAX was
// added after the problem with filesystems was found FS DAX VMAs are
// specifically failed.  Filesystem pages are still subject to bugs and use of
// FOLL_LONGTERM should be avoided on those pages.
//
// In the CMA case: long term pins in a CMA region would unnecessarily fragment
// that region.  And so, CMA attempts to migrate the page before pinning, when
// FOLL_LONGTERM is specified.
//
// FOLL_PIN indicates that a special kind of tracking (not just page->_refcount,
// but an additional pin counting system) will be invoked. This is intended for
// anything that gets a page reference and then touches page data (for example,
// Direct IO). This lets the filesystem know that some non-file-system entity is
// potentially changing the pages' data. In contrast to FOLL_GET (whose pages
// are released via put_page()), FOLL_PIN pages must be released, ultimately, by
// a call to unpin_user_page().
//
// FOLL_PIN is similar to FOLL_GET: both of these pin pages. They use different
// and separate refcounting mechanisms, however, and that means that each has
// its own acquire and release mechanisms:
//
// FOLL_GET: get_user_pages*() to acquire, and put_page() to release.
//
// FOLL_PIN: pin_user_pages*() to acquire, and unpin_user_pages to release.
//
// FOLL_PIN and FOLL_GET are mutually exclusive for a given function call.
// (The underlying pages may experience both FOLL_GET-based and FOLL_PIN-based
// calls applied to them, and that's perfectly OK. This is a constraint on the
// callers, not on the pages.)
//
// FOLL_PIN should be set internally by the pin_user_pages*() APIs, never
// directly by the caller. That's in order to help avoid mismatches when
// releasing pages: get_user_pages*() pages must be released via put_page(),
// while pin_user_pages*() pages must be released via unpin_user_page().
//
// Please see Documentation/core-api/pin_user_pages.rst for more information.
//
// check pte is writable
// do get_page on page
// give error on hole if it would be zero
// get_user_pages read/write w/o permission
//
// if a disk transfer is needed, start the IO and return without waiting
// upon it
//
// do not fault in pages
// check page is hwpoisoned
// don't do file mappings
//
// FOLL_LONGTERM indicates that the page will be held for an indefinite
// time period _often_ under userspace control.  This is in contrast to
// iov_iter_get_pages(), whose usages are transient.
//
// split huge pmd before returning
// allow returning PCI P2PDMA pages
// allow interrupts from generic signals
//
// Always honor (trigger) NUMA hinting faults.
//
// FOLL_WRITE implicitly honors NUMA hinting faults because a
// PROT_NONE-mapped page is not writable (exceptions with FOLL_FORCE
// apply). get_user_pages_fast_only() always implicitly honors NUMA
// hinting faults.
//
// See also internal only FOLL flags in mm/internal.h
// mm flags
//
// Bits 0 and 1 were dumpability; that moved to task->exec_state.  Reserve
// the bits so MMF_DUMP_FILTER_* positions stay stable for the
// /proc/<pid>/coredump_filter ABI.
//
pub const MMF_DUMPABLE_BITS: c_int = 2;
// coredump filter bits
pub const MMF_DUMP_ANON_PRIVATE: c_int = 2;
pub const MMF_DUMP_ANON_SHARED: c_int = 3;
pub const MMF_DUMP_MAPPED_PRIVATE: c_int = 4;
pub const MMF_DUMP_MAPPED_SHARED: c_int = 5;
pub const MMF_DUMP_ELF_HEADERS: c_int = 6;
pub const MMF_DUMP_HUGETLB_PRIVATE: c_int = 7;
pub const MMF_DUMP_HUGETLB_SHARED: c_int = 8;
pub const MMF_DUMP_DAX_PRIVATE: c_int = 9;
pub const MMF_DUMP_DAX_SHARED: c_int = 10;

pub const MMF_DUMP_FILTER_BITS: c_int = 9;

// leave room for more dump flags

//
// MMF_HAS_PINNED: Whether this mm has pinned any pages.  This can be either
// replaced in the future by mm.pinned_vm when it becomes stable, or grow into
// a counter on its own. We're aggresive on this bit for now: even if the
// pinned pages were unpinned later on, we'll still keep this bit set for the
// lifecycle of this mm, just for simplicity.
//

pub const MMF_HAS_MDWE: c_int = 28;

pub const MMF_HAS_MDWE_NO_INHERIT: c_int = 29;
pub const MMF_VM_MERGE_ANY: c_int = 30;

// Legacy flags must fit within 32 bits.
//
// Initialise legacy flags according to masks, propagating selected flags on
// fork. Further flag manipulation can be performed by the caller.
//
