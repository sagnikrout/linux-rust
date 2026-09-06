//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page-flags.h
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
// Macros for manipulating and testing page->flags
//

//
// Various page->flags bits:
//
// PG_reserved is set for special pages. The "struct page" of such a page
// should in general not be touched (e.g. set dirty) except by its owner.
// Pages marked as PG_reserved include:
// - Pages part of the kernel image (including vDSO) and similar (e.g. BIOS,
// initrd, HW tables)
// - Pages reserved or allocated early during boot (before the page allocator
// was initialized). This includes (depending on the architecture) the
// initial vmemmap, initial page tables, crashkernel, elfcorehdr, and much
// much more. Once (if ever) freed, PG_reserved is cleared and they will
// be given to the page allocator.
// - Pages falling into physical memory gaps - not IORESOURCE_SYSRAM. Trying
// to read/write these pages might end badly. Don't touch!
// - The zero page(s)
// - Pages allocated in the context of kexec/kdump (loaded kernel image,
// control pages, vmcoreinfo)
// - MMIO/DMA pages. Some architectures don't allow to ioremap pages that are
// not marked PG_reserved (as they might be in use by somebody else who does
// not respect the caching strategy).
// - MCA pages on ia64
// - Pages holding CPU notes for POWER Firmware Assisted Dump
// - Device memory (e.g. PMEM, DAX, HMM)
// Some PG_reserved pages will be excluded from the hibernation image.
// PG_reserved does in general not hinder anybody from dumping or swapping
// and is no longer required for remap_pfn_range(). ioremap might require it.
// Consequently, PG_reserved for a page mapped into user space can indicate
// the zero page, the vDSO, MMIO pages or device memory.
//
// The PG_private bitflag is set on pagecache pages if they contain filesystem
// specific data (which is normally at page->private). It can be used by
// private allocations for its own usage.
//
// During initiation of disk I/O, PG_locked is set. This bit is set before I/O
// and cleared when writeback _starts_ or when read _completes_. PG_writeback
// is set before writeback starts and cleared when it finishes.
//
// PG_locked also pins a page in pagecache, and blocks truncation of the file
// while it is held.
//
// page_waitqueue(page) is a wait queue of all tasks waiting for the page
// to become unlocked.
//
// PG_swapbacked is set when a page uses swap as a backing storage.  This are
// usually PageAnon or shmem pages but please note that even anonymous pages
// might lose their PG_swapbacked flag when they simply can be dropped (e.g. as
// a result of MADV_FREE).
//
// PG_referenced, PG_reclaim are used for page reclaim for anonymous and
// file-backed pagecache (see mm/vmscan.c).
//
// PG_arch_1 is an architecture specific page state bit.  The generic code
// guarantees that this bit is cleared for a page when it first is entered into
// the page cache.
//
// PG_hwpoison indicates that a page got corrupted in hardware and contains
// data with incorrect ECC bits that triggered a machine check. Accessing is
// not safe since it may cause another machine check. Don't touch!
//
// Don't use the pageflags directly.  Use the PageFoo macros.
//
// The page flags field is split into two parts, the main flags area
// which extends from the low bits upwards, and the fields area which
// extends from the high bits downwards.
//
// | FIELD | ... | FLAGS |
// N-1           ^       0
// (NR_PAGEFLAGS)
//
// The fields area is reserved for fields mapping zone, node (for NUMA) and
// SPARSEMEM section (for variants of SPARSEMEM that require section ids like
// SPARSEMEM_EXTREME with !SPARSEMEM_VMEMMAP).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pageflags {
    PG_locked,		/* Page is locked. Don't touch. */
    PG_writeback,		/* Page is under writeback */
    PG_referenced,
    PG_uptodate,
    PG_dirty,
    PG_lru,
    PG_head,		/* Must be in bit 6 */
    PG_waiters,		/* Page has waiters, check its waitqueue. Must be bit #7 and in the same byte as "PG_locked" */
    PG_active,
    PG_workingset,
    PG_owner_priv_1,	/* Owner use. If pagecache, fs may use */
    PG_owner_2,		/* Owner use. If pagecache, fs may use */
    PG_arch_1,
    PG_reserved,
    PG_private,		/* If pagecache, has fs-private data */
    PG_private_2,		/* If pagecache, has fs aux data */
    PG_reclaim,		/* To be reclaimed asap */
    PG_swapbacked,		/* Page is backed by RAM/swap */
    PG_unevictable,		/* Page is "unevictable"  */
    PG_dropbehind,		/* drop pages on IO completion */

    PG_mlocked,		/* Page is vma mlocked */

    PG_hwpoison,		/* hardware poisoned page. Don't touch */

    PG_young,
    PG_idle,

    PG_arch_2,

    PG_arch_3,

    __NR_PAGEFLAGS,

    PG_readahead = PG_reclaim,

// Anonymous memory (and shmem)
    PG_swapcache = PG_owner_priv_1, /* Swap page: swp_entry_t in private */
// Some filesystems
    PG_checked = PG_owner_priv_1,

//
// Depending on the way an anonymous folio can be mapped into a page
// table (e.g., single PMD/PUD/CONT of the head page vs. PTE-mapped
// THP), PG_anon_exclusive may be set only for the head page or for
// tail pages of an anonymous folio. For now, we only expect it to be
// set on tail pages for PTE-mapped THP.
//
    PG_anon_exclusive = PG_owner_2,

//
// Set if all buffer heads in the folio are mapped.
// Filesystems which do not use BHs can use it for their own purpose.
//
    PG_mappedtodisk = PG_owner_2,

// Two page bits are conscripted by FS-Cache to maintain local caching
// state.  These bits are set on pages belonging to the netfs's inodes
// when those inodes are being locally cached.
//
    PG_fscache = PG_private_2,	/* page backed by cache */

// XEN
// Pinned in Xen as a read-only pagetable page.
    PG_pinned = PG_owner_priv_1,
// Pinned as part of domain save (see xen_mm_pin_all()).
    PG_savepinned = PG_dirty,
// Has a grant mapping of another (foreign) domain's page.
    PG_foreign = PG_owner_priv_1,
// Remapped by swiotlb-xen.
    PG_xen_remapped = PG_owner_priv_1,

// movable_ops page that is isolated for migration
    PG_movable_ops_isolated = PG_reclaim,
// this is a movable_ops page (for selected typed pages only)
    PG_movable_ops = PG_uptodate,

// Only valid for buddy pages. Used to track pages that are reported
    PG_reported = PG_uptodate,

// For self-hosted memmap pages
    PG_vmemmap_self_hosted = PG_owner_priv_1,

//
// Flags only valid for compound pages.  Stored in first tail page's
// flags word.  Cannot use the first 8 flags or any flag marked as
// PF_ANY.
//

// At least one page in this folio has the hwpoison flag set
    PG_has_hwpoisoned = PG_active,
    PG_large_rmappable = PG_workingset, /* anon or file-backed */
    PG_partially_mapped = PG_reclaim, /* was identified to be partially mapped */
}

//
// For tail pages, if the size of struct page is power-of-2 ->compound_info
// encodes the mask that converts the address of the tail page address to
// the head page address.
//
// Otherwise, ->compound_info has direct pointer to head pages.
//
// Limit mask usage to HugeTLB vmemmap optimization (HVO) where it
// makes a difference.
//
// The approach with mask would work in the wider set of conditions,
// but it requires validating that struct pages are naturally aligned
// for all orders up to the MAX_FOLIO_ORDER, which can be tricky.
//
extern "C" {
    pub fn is_power_of_2(page): sizeof(struct) -> return;
}
// Bit 0 encodes PageTail()
//
// If compound_info_has_mask() is true the rest of the info encodes
// the mask that converts the address of the tail page to the head page.
//
// No need to clear bit 0 in the mask as 'page' always has it clear.
//
// Let's do it in a branchless manner.
//
// Non-tail: -1UL, Tail: 0
// Non-tail: -1UL, Tail: info

//
// If the size of struct page is power-of-2, bits [shift:0] of the
// virtual address of compound head are zero.
//
// Calculate mask that can be applied to the virtual address of
// the tail page to get address of the head page.
//
// Bit 0 encodes PageTail()
//
// page_folio - Converts from page to folio.
// @p: The page.
//
// Every page is part of a folio.  This function cannot be called on a
// NULL pointer.
//
// Context: No reference, nor lock is required on @page.  If the caller
// does not hold a reference, this call may race with a folio split, so
// it should re-check the folio still contains this page after gaining
// a reference on the folio.
// Return: The folio which contains this page.
//

//
// folio_page - Return a page from a folio.
// @folio: The folio.
// @n: The page number to return.
//
// @n is relative to the start of the folio.  This function does not
// check that the page number lies within @folio; the caller is presumed
// to have a reference to the page.
//

extern "C" {
    pub fn page_init_poison(page: *mut page, size: usize);
}

//
// Page flags policies wrt compound pages
//
// PF_POISONED_CHECK
// check if this struct page poisoned/uninitialized
//
// PF_ANY:
// the page flag is relevant for small, head and tail pages.
//
// PF_HEAD:
// for compound page all operations related to the page flag applied to
// head page.
//
// PF_NO_TAIL:
// modifications of the page flag must be done on small or head pages,
// checks can be done on tail pages too.
//
// PF_NO_COMPOUND:
// the page flag is not relevant for compound pages.
//
// PF_SECOND:
// the page flag is stored in the first tail page.
//

// Which page is the flag stored in
pub const FOLIO_PF_ANY: c_int = 0;
pub const FOLIO_PF_HEAD: c_int = 0;
pub const FOLIO_PF_NO_TAIL: c_int = 0;
pub const FOLIO_PF_NO_COMPOUND: c_int = 0;
pub const FOLIO_PF_SECOND: c_int = 1;
pub const FOLIO_HEAD_PAGE: c_int = 0;
pub const FOLIO_SECOND_PAGE: c_int = 1;
//
// Macros to create function definitions for page flags
//

// Xen
//
// Private page markings that may be used by the filesystem that owns the page
// for its own purposes.
// - PG_private and PG_private_2 cause release_folio() and co to be invoked
//
// owner_2 can be set on tail pages for anon memory
//
// Only test-and-set exist for PG_writeback.  The unconditional operators are
// risky: they bypass page accounting.
//
// PG_readahead is only used for reads; PG_reclaim is only for writes

//
// Must use a macro here due to header dependency issues. page_zone() is not
// available at this point.
//

// Does kmap_local_folio() only allow access to one page of the folio?

pub const __PG_HWPOISON: c_int = 0;

// See page_idle.h for !64BIT workaround

//
// PageReported() is used to track reported free pages within the Buddy
// allocator. We can use the non-atomic version of the test and set
// operations as both should be shielded with the zone lock to prevent
// any possible races on the setting or clearing of the bit.
//

//
// On an anonymous folio mapped into a user virtual memory area,
// folio->mapping points to its anon_vma, not to a struct address_space;
// with the FOLIO_MAPPING_ANON bit set to distinguish it.  See rmap.h.
//
// On an anonymous folio in a VM_MERGEABLE area, if CONFIG_KSM is enabled,
// the FOLIO_MAPPING_ANON_KSM bit may be set along with the FOLIO_MAPPING_ANON
// bit; and then folio->mapping points, not to an anon_vma, but to a private
// structure which KSM associates with that merged folio.  See ksm.h.
//
// Please note that, confusingly, "folio_mapping" refers to the inode
// address_space which maps the folio from disk; whereas "folio_mapped"
// refers to user virtual address space into which the folio is mapped.
//
// For slab pages, since slab reuses the bits in struct page to store its
// internal states, the folio->mapping does not exist as such, nor do
// these flags below.  So in order to avoid testing non-existent bits,
// please make sure that folio_test_slab(folio) actually evaluates to
// false before calling the following functions (e.g., folio_test_anon).
// See mm/slab.h.
//
pub const FOLIO_MAPPING_ANON: c_uint = 0x1;
pub const FOLIO_MAPPING_ANON_KSM: c_uint = 0x2;

extern "C" {
    pub fn folio_test_anon(!folio_test_swapbacked(folio: folio) &&) -> return;
}
extern "C" {
    pub fn folio_test_anon(_arg: page_folio(page)) -> return;
}

//
// A KSM page is one of those write-protected "shared pages" or "merged pages"
// which KSM maps into multiple mms, wherever identical anonymous page content
// is found in VM_MERGEABLE vmas.  It's a PageAnon page, pointing not to any
// anon_vma, but to that page's node of the stable tree.
//

extern "C" {
    pub fn stable_page_flags(page: *const page) -> u64;
}
//
// folio_xor_flags_has_waiters - Change some folio flags.
// @folio: The folio.
// @mask: Bits set in this word will be changed.
//
// This must only be used for flags which are changed with the folio
// lock held.  For example, it is unsafe to use for PG_dirty as that
// can be set without the folio lock held.  It can also only be used
// on flags which are in the range 0-6 as some of the implementations
// only affect those bits.
//
// Return: Whether there are tasks waiting on the folio.
//
extern "C" {
    pub fn xor_unlock_is_negative_byte(_arg: mask, _arg: folio_flags(folio, _arg: 0)) -> return;
}
//
// folio_test_uptodate - Is this folio up to date?
// @folio: The folio.
//
// The uptodate flag is set on a folio when every byte in the folio is
// at least as new as the corresponding bytes on storage.  Anonymous
// and CoW folios are always uptodate.  If the folio is not uptodate,
// some of the bytes in it may be; see the is_partially_uptodate()
// address_space operation.
//
// Must ensure that the data we read out of the folio is loaded
// _after_ we've loaded folio->flags to check the uptodate bit.
// We can skip the barrier if the folio is not uptodate, because
// we wouldn't be reading anything from it.
//
// See folio_mark_uptodate() for the other side of the story.
//
extern "C" {
    pub fn folio_test_uptodate(_arg: page_folio(page)) -> return;
}
//
// Memory barrier must be issued before setting the PG_uptodate bit,
// so that all previous stores issued in order to bring the folio
// uptodate are actually visible before folio_test_uptodate becomes true.
//
extern "C" {
    pub fn __folio_start_writeback(folio: *mut folio, keep_write: bool);
}
extern "C" {
    pub fn set_page_writeback(page: *mut page);
}

extern "C" {
    pub fn test_bit(_arg: PG_head, _arg: const_folio_flags(folio, _arg: FOLIO_PF_ANY)) -> return;
}
extern "C" {
    pub fn test_bit(_arg: PG_head, _arg: &page->flags.f) -> return;
}
//
// folio_test_large() - Does this folio contain more than one page?
// @folio: The folio to test.
//
// Return: True if the folio is larger than one page.
//
extern "C" {
    pub fn folio_test_head(_arg: folio) -> return;
}

//
// PageHasHWPoisoned indicates that at least one subpage is hwpoisoned in the
// compound page.
//
// This flag is set by hwpoison handler.  Cleared by THP split or free page.
//

//
// For pages that do not use mapcount, page_type may be used.
// The low 24 bits of pagetype may be used for your own purposes, as long
// as you are careful to not affect the top 8 bits.  The low bits of
// pagetype will be overwritten when you clear the page_type from the page.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pagetype {
// 0x00-0x7f are positive numbers, ie mapcount
// Reserve 0x80-0xef for mapcount overflow.
    PGTY_buddy		= 0xf0,
    PGTY_offline		= 0xf1,
    PGTY_table		= 0xf2,
    PGTY_guard		= 0xf3,
    PGTY_hugetlb		= 0xf4,
    PGTY_slab		= 0xf5,
    PGTY_zsmalloc		= 0xf6,
    PGTY_unaccepted		= 0xf7,
    PGTY_large_kmalloc	= 0xf8,

    PGTY_mapcount_underflow = 0xff
}

// This takes a mapcount which is one more than page->_mapcount
extern "C" {
    pub fn page_type_has_type(1: mapcount -) -> return;
}
extern "C" {
    pub fn page_type_has_type(_arg: data_race(page->page_type)) -> return;
}

//
// PageBuddy() indicates that the page is free and in the buddy system
// (see mm/page_alloc.c).
//
// PageOffline() indicates that the page is logically offline although the
// containing section is online. (e.g. inflated in a balloon driver or
// not onlined when onlining the section).
// The content of these pages is effectively stale. Such pages should not
// be touched (read/write/dump/save) except by their owner.
//
// When a memory block gets onlined, all pages are initialized with a
// refcount of 1 and PageOffline(). generic_online_page() will
// take care of clearing PageOffline().
//
// If a driver wants to allow to offline unmovable PageOffline() pages without
// putting them back to the buddy, it can do so via the memory notifier by
// decrementing the reference count in MEM_GOING_OFFLINE and incrementing the
// reference count in MEM_CANCEL_OFFLINE. When offlining, the PageOffline()
// pages (now with a reference count of zero) are treated like free (unmanaged)
// pages, allowing the containing memory block to get offlined. A driver that
// relies on this feature is aware that re-onlining the memory block will
// require not giving them to the buddy via generic_online_page().
//
// Memory offlining code will not adjust the managed page count for any
// PageOffline() pages, treating them like they were never exposed to the
// buddy using generic_online_page().
//
// There are drivers that mark a page PageOffline() and expect there won't be
// any further access to page content. PFN walkers that read content of random
// pages should check PageOffline() and synchronize with such drivers using
// page_offline_freeze()/page_offline_thaw().
//
extern "C" {
    pub fn page_offline_freeze();
}
extern "C" {
    pub fn page_offline_thaw();
}
extern "C" {
    pub fn page_offline_begin();
}
extern "C" {
    pub fn page_offline_end();
}
//
// Marks pages in use as page tables.
//
// Marks guardpages used with debug_pagealloc.
//

//
// Mark pages that has to be accepted before touched for the first time.
//
// Serialized with zone lock.
//
// PageHuge - Determine if the page belongs to hugetlbfs
// @page: The page to test.
//
// Context: Any context.
// Return: True for hugetlbfs pages, false for anon pages or pages
// belonging to other filesystems.
//
extern "C" {
    pub fn folio_test_hugetlb(_arg: page_folio(page)) -> return;
}
//
// Check if a page is currently marked HWPoisoned. Note that this check is
// best effort only and inherently racy: there is no way to synchronize with
// failing hardware.
//
extern "C" {
    pub fn folio_test_hugetlb(PageHWPoison(&folio->page: folio) &&) -> return;
}
extern "C" {
    pub fn is_free_buddy_page(page: *const page) -> bool;
}

//
// This page is migratable through movable_ops (for selected typed pages
// only).
//
// Page migration of such pages might fail, for example, if the page is
// already isolated by somebody else, or if the page is about to get freed.
//
// While a subsystem might set selected typed pages that support page migration
// as being movable through movable_ops, it must never clear this flag.
//
// This flag is only cleared when the page is freed back to the buddy.
//
// Only selected page types support this flag (see page_movable_ops()) and
// the flag might be used in other context for other pages. Always use
// page_has_movable_ops() instead.
//
// A movable_ops page has this flag set while it is isolated for migration.
// This flag primarily protects against concurrent migration attempts.
//
// Once migration ended (success or failure), the flag is cleared. The
// flag is managed by the migration core.
//

//
// page_has_movable_ops - test for a movable_ops page
// @page: The page to test.
//
// Test whether this is a movable_ops page. Such pages will stay that
// way until freed.
//
// Returns true if this is a movable_ops page, otherwise false.
//
// HugeTLB stores this information on the head page; THP keeps it per
// page
//
extern "C" {
    pub fn test_bit(_arg: PG_anon_exclusive, _arg: &PF_ANY(page, _arg: 1)->flags.f) -> return;
}

pub const __PG_MLOCKED: c_int = 0;

//
// Flags checked when a page is freed.  Pages being freed should not have
// these flags set.  If they are, there is a problem.
//

//
// Flags checked when a page is prepped for return by the page allocator.
// Pages being prepped should not have these flags set.  If they are set,
// there has been a kernel bug or struct page corruption.
//
// __PG_HWPOISON is exceptional because it needs to be kept beyond page's
// alloc-free cycle to prevent from reusing the page.
//

//
// Flags stored in the second page of a compound page.  They may overlap
// the CHECK_AT_FREE flags above, so need to be cleared.
//

//
// folio_has_private - Determine if folio has private stuff
// @folio: The folio to be checked
//
// Determine if a folio has private stuff, indicating that release routines
// should be invoked upon it.
//

