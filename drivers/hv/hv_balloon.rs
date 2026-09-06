//! Automatically rewritten from C to Rust
//! Source: drivers/hv/hv_balloon.c
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
// Copyright (c) 2012, Microsoft Corporation.
//
// Author:
// K. Y. Srinivasan <kys@microsoft.com>
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// We begin with definitions supporting the Dynamic Memory protocol
// with the host.
//
// Begin protocol definitions.
//
// Protocol versions. The low word is the minor version, the high word the major
// version.
//
// History:
// Initial version 1.0
// Changed to 0.1 on 2009/03/25
// Changes to 0.2 on 2009/05/14
// Changes to 0.3 on 2009/12/03
// Changed to 1.0 on 2011/04/05
//

//
// VERSION_1 and VERSION_2 are retained for the historical record,
// but are no longer supported in Linux guests.
//
    enum {
    DYNMEM_PROTOCOL_VERSION_1 = DYNMEM_MAKE_VERSION(0, 3),
    DYNMEM_PROTOCOL_VERSION_2 = DYNMEM_MAKE_VERSION(1, 0),
    DYNMEM_PROTOCOL_VERSION_3 = DYNMEM_MAKE_VERSION(2, 0),
    DYNMEM_PROTOCOL_VERSION_WIN7 = DYNMEM_PROTOCOL_VERSION_1,
    DYNMEM_PROTOCOL_VERSION_WIN8 = DYNMEM_PROTOCOL_VERSION_2,
    DYNMEM_PROTOCOL_VERSION_WIN10 = DYNMEM_PROTOCOL_VERSION_3
    };
//
// Message Types
//
    enum dm_message_type {
//
// Version 0.3
//
    DM_ERROR			= 0,
    DM_VERSION_REQUEST		= 1,
    DM_VERSION_RESPONSE		= 2,
    DM_CAPABILITIES_REPORT		= 3,
    DM_CAPABILITIES_RESPONSE	= 4,
    DM_STATUS_REPORT		= 5,
    DM_BALLOON_REQUEST		= 6,
    DM_BALLOON_RESPONSE		= 7,
    DM_UNBALLOON_REQUEST		= 8,
    DM_UNBALLOON_RESPONSE		= 9,
    DM_MEM_HOT_ADD_REQUEST		= 10,
    DM_MEM_HOT_ADD_RESPONSE		= 11,
    DM_VERSION_03_MAX		= 11,
//
// Version 1.0.
//
    DM_INFO_MESSAGE			= 12,
    DM_VERSION_1_MAX		= 12
    };
//
// Structures defining the dynamic memory management
// protocol.
//
    union dm_version {
    struct {
    __u16 minor_version;
    __u16 major_version;
    };
    __u32 version;
    } __packed;
    union dm_caps {
    struct {
    __u64 balloon:1;
    __u64 hot_add:1;
//
// To support guests that may have alignment
// limitations on hot-add, the guest can specify
// its alignment requirements; a value of n
// represents an alignment of 2^n in mega bytes.
//
    __u64 hot_add_alignment:4;
    __u64 reservedz:58;
    } cap_bits;
    __u64 caps;
    } __packed;
    union dm_mem_page_range {
    struct  {
//
// The PFN number of the first page in the range.
// 40 bits is the architectural limit of a PFN
// number for AMD64.
//
    __u64 start_page:40;
//
// The number of pages in the range.
//
    __u64 page_cnt:24;
    } finfo;
    __u64  page_range;
    } __packed;
//
// The header for all dynamic memory messages:
//
// type: Type of the message.
// size: Size of the message in bytes; including the header.
// trans_id: The guest is responsible for manufacturing this ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_header {
    pub type: __u16,
    pub size: __u16,
    pub trans_id: __u32,
    pub __packed: },
//
// A generic message format for dynamic memory.
// Specific message formats are defined later in the file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_message {
    pub hdr: dm_header,
    pub /: *mut *mut __u8 data[]; / enclosed message,
    pub __packed: },
//
// Specific message types supporting the dynamic memory protocol.
//
// Version negotiation message. Sent from the guest to the host.
// The guest is free to try different versions until the host
// accepts the version.
//
// dm_version: The protocol version requested.
// is_last_attempt: If TRUE, this is the last version guest will request.
// reservedz: Reserved field, set to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_version_request {
    pub hdr: dm_header,
    pub version: union dm_version,
    pub is_last_attempt:1: __u32,
    pub reservedz:31: __u32,
    pub __packed: },
//
// Version response message; Host to Guest and indicates
// if the host has accepted the version sent by the guest.
//
// is_accepted: If TRUE, host has accepted the version and the guest
// should proceed to the next stage of the protocol. FALSE indicates that
// guest should re-try with a different version.
//
// reservedz: Reserved field, set to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_version_response {
    pub hdr: dm_header,
    pub is_accepted:1: __u64,
    pub reservedz:63: __u64,
    pub __packed: },
//
// Message reporting capabilities. This is sent from the guest to the
// host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_capabilities {
    pub hdr: dm_header,
    pub caps: union dm_caps,
    pub min_page_cnt: __u64,
    pub max_page_number: __u64,
    pub __packed: },
//
// Response to the capabilities message. This is sent from the host to the
// guest. This message notifies if the host has accepted the guest's
// capabilities. If the host has not accepted, the guest must shutdown
// the service.
//
// is_accepted: Indicates if the host has accepted guest's capabilities.
// reservedz: Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_capabilities_resp_msg {
    pub hdr: dm_header,
    pub is_accepted:1: __u64,
    pub reservedz:63: __u64,
    pub __packed: },
//
// This message is used to report memory pressure from the guest.
// This message is not part of any transaction and there is no
// response to this message.
//
// num_avail: Available memory in pages.
// num_committed: Committed memory in pages.
// page_file_size: The accumulated size of all page files
// in the system in pages.
// zero_free: The number of zero and free pages.
// page_file_writes: The writes to the page file in pages.
// io_diff: An indicator of file cache efficiency or page file activity,
// calculated as File Cache Page Fault Count - Page Read Count.
// This value is in pages.
//
// Some of these metrics are Windows specific and fortunately
// the algorithm on the host side that computes the guest memory
// pressure only uses num_committed value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_status {
    pub hdr: dm_header,
    pub num_avail: __u64,
    pub num_committed: __u64,
    pub page_file_size: __u64,
    pub zero_free: __u64,
    pub page_file_writes: __u32,
    pub io_diff: __u32,
    pub __packed: },
//
// Message to ask the guest to allocate memory - balloon up message.
// This message is sent from the host to the guest. The guest may not be
// able to allocate as much memory as requested.
//
// num_pages: number of pages to allocate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_balloon {
    pub hdr: dm_header,
    pub num_pages: __u32,
    pub reservedz: __u32,
    pub __packed: },
//
// Balloon response message; this message is sent from the guest
// to the host in response to the balloon message.
//
// reservedz: Reserved; must be set to zero.
// more_pages: If FALSE, this is the last message of the transaction.
// if TRUE there will be at least one more message from the guest.
//
// range_count: The number of ranges in the range array.
//
// range_array: An array of page ranges returned to the host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_balloon_response {
    pub hdr: dm_header,
    pub reservedz: __u32,
    pub more_pages:1: __u32,
    pub range_count:31: __u32,
    pub range_array: [union dm_mem_page_range; ],
    pub __packed: },
//
// Un-balloon message; this message is sent from the host
// to the guest to give guest more memory.
//
// more_pages: If FALSE, this is the last message of the transaction.
// if TRUE there will be at least one more message from the guest.
//
// reservedz: Reserved; must be set to zero.
//
// range_count: The number of ranges in the range array.
//
// range_array: An array of page ranges returned to the host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_unballoon_request {
    pub hdr: dm_header,
    pub more_pages:1: __u32,
    pub reservedz:31: __u32,
    pub range_count: __u32,
    pub range_array: [union dm_mem_page_range; ],
    pub __packed: },
//
// Un-balloon response message; this message is sent from the guest
// to the host in response to an unballoon request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_unballoon_response {
    pub hdr: dm_header,
    pub __packed: },
//
// Hot add request message. Message sent from the host to the guest.
//
// mem_range: Memory range to hot add.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_hot_add {
    pub hdr: dm_header,
    pub range: union dm_mem_page_range,
    pub __packed: },
//
// Hot add response message.
// This message is sent by the guest to report the status of a hot add request.
// If page_count is less than the requested page count, then the host should
// assume all further hot add requests will fail, since this indicates that
// the guest has hit an upper physical memory barrier.
//
// Hot adds may also fail due to low resources; in this case, the guest must
// not complete this message until the hot add can succeed, and the host must
// not send a new hot add request until the response is sent.
// If VSC fails to hot add memory DYNMEM_NUMBER_OF_UNSUCCESSFUL_HOTADD_ATTEMPTS
// times it fails the request.
//
// page_count: number of pages that were successfully hot added.
//
// result: result of the operation 1: success, 0: failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_hot_add_response {
    pub hdr: dm_header,
    pub page_count: __u32,
    pub result: __u32,
    pub __packed: },
//
// Types of information sent from host to the guest.
//
    enum dm_info_type {
    INFO_TYPE_MAX_PAGE_CNT = 0,
    MAX_INFO_TYPE
}

//
// Header for the information message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_info_header {
    pub type: enum dm_info_type,
    pub data_size: __u32,
    pub __packed: },
//
// This message is sent from the host to the guest to pass
// some relevant information (win8 addition).
//
// reserved: no used.
// info_size: size of the information blob.
// info: information blob.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_info_msg {
    pub hdr: dm_header,
    pub reserved: __u32,
    pub info_size: __u32,
    pub info: [__u8; ],
}

//
// End protocol definitions.
//
// State to manage hot adding memory into the guest.
// The range start_pfn : end_pfn specifies the range
// that the host has asked us to hot add. The range
// start_pfn : ha_end_pfn specifies the range that we have
// currently hot added. We hot add in chunks equal to the
// memory block size; it is possible that we may not be able
// to bring online all the pages in the region. The range
// covered_start_pfn:covered_end_pfn defines the pages that can
// be brought online.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_hotadd_state {
    pub list: list_head,
    pub start_pfn: c_ulong,
    pub covered_start_pfn: c_ulong,
    pub covered_end_pfn: c_ulong,
    pub ha_end_pfn: c_ulong,
    pub end_pfn: c_ulong,
//
// A list of gaps.
//
    pub gap_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_hotadd_gap {
    pub list: list_head,
    pub start_pfn: c_ulong,
    pub end_pfn: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct balloon_state {
    pub num_pages: __u32,
    pub wrk: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hot_add_wrk {
    pub ha_page_range: union dm_mem_page_range,
    pub ha_region_range: union dm_mem_page_range,
    pub wrk: work_struct,
}

    static bool allow_hibernation;
    let mut hot_add: static bool = true;
    static bool do_hot_add;
//
// Delay reporting memory pressure by
// the specified number of seconds.
//
    let mut pressure_report_delay: static uint = 45;
    extern unsigned int page_reporting_order;
pub const HV_MAX_FAILURES: c_int = 2;
//
// The last time we posted a pressure report to host.
//
    static unsigned long last_post_time;
    static int hv_hypercall_multi_failure;
    module_param(hot_add, bool, 0644);
    MODULE_PARM_DESC(hot_add, "If set attempt memory hot_add");
    module_param(pressure_report_delay, uint, 0644);
    MODULE_PARM_DESC(pressure_report_delay, "Delay in secs in reporting pressure");
    let mut trans_id: static atomic_t = ATOMIC_INIT(0);
    let mut dm_ring_size: static int = VMBUS_RING_SIZE(16 * 1024);
//
// Driver specific state.
//
    enum hv_dm_state {
    DM_INITIALIZING = 0,
    DM_INITIALIZED,
    DM_BALLOON_UP,
    DM_BALLOON_DOWN,
    DM_HOT_ADD,
    DM_INIT_ERROR
    };
    static __u8 recv_buffer[HV_HYP_PAGE_SIZE];
    static __u8 balloon_up_send_buffer[HV_HYP_PAGE_SIZE];
    static unsigned long ha_pages_in_chunk;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_dynmem_device {
    pub dev: *mut hv_device,
    pub state: enum hv_dm_state,
    pub host_event: completion,
    pub config_event: completion,
//
// Number of pages we have currently ballooned out.
//
    pub num_pages_ballooned: c_uint,
    pub num_pages_onlined: c_uint,
    pub num_pages_added: c_uint,
//
// State to manage the ballooning (up) operation.
//
    pub balloon_wrk: balloon_state,
//
// State to execute the "hot-add" operation.
//
    pub ha_wrk: hot_add_wrk,
//
// This state tracks if the host has specified a hot-add
// region.
//
    pub host_specified_ha_region: bool,
//
// State to synchronize hot-add.
//
    pub ol_waitevent: completion,
//
// This thread handles hot-add
// requests from the host as well as notifying
// the host with regards to memory pressure in
// the guest.
//
    pub thread: *mut task_struct,
//
// Protects ha_region_list, num_pages_onlined counter and individual
// regions from ha_region_list.
//
    pub ha_lock: spinlock_t,
//
// A list of hot-add regions.
//
    pub ha_region_list: list_head,
//
// We start with the highest version we can support
// and downgrade based on the host; we save here the
// next version to try.
//
    pub next_version: __u32,
//
// The negotiated version agreed by host.
//
    pub version: __u32,
    pub pr_dev_info: page_reporting_dev_info,
//
// Maximum number of pages that can be hot_add-ed
//
    pub max_dynamic_page_count: __u64,
}

    static struct hv_dynmem_device dm_device;
    static void post_status(struct hv_dynmem_device *dm);
    static void enable_page_reporting(void);
    static void disable_page_reporting(void);

    static inline bool has_pfn_is_backed(struct hv_hotadd_state *has,
    unsigned long pfn)
    {
    struct hv_hotadd_gap *gap;
// The page is not backed.
    if (pfn < has.covered_start_pfn || pfn >= has.covered_end_pfn)
    return false;
// Check for gaps.
    list_for_each_entry(gap, &has.gap_list, list) {
    if (pfn >= gap.start_pfn && pfn < gap.end_pfn)
    return false;
    }
    return true;
    }
    static unsigned long hv_page_offline_check(unsigned long start_pfn,
    unsigned long nr_pages)
    {
    let mut pfn: c_ulong = start_pfn, count = 0;
    struct hv_hotadd_state *has;
    bool found;
    while (pfn < start_pfn + nr_pages) {
//
// Search for HAS which covers the pfn and when we find one
// count how many consequitive PFNs are covered.
//
    found = false;
    list_for_each_entry(has, &dm_device.ha_region_list, list) {
    while ((pfn >= has.start_pfn) &&
    (pfn < has.end_pfn) &&
    (pfn < start_pfn + nr_pages)) {
    found = true;
    if (has_pfn_is_backed(has, pfn))
    count++;
    pfn++;
    }
    }
//
// This PFN is not in any HAS (e.g. we're offlining a region
// which was present at boot), no need to account for it. Go
// to the next one.
//
    if (!found)
    pfn++;
    }
    return count;
    }
    static int hv_memory_notifier(struct notifier_block *nb, unsigned long val,
    void *v)
    {
    struct memory_notify *mem = (struct memory_notify *)v;
    unsigned long pfn_count;
    switch (val) {
    case MEM_ONLINE:
    case MEM_CANCEL_ONLINE:
    complete(&dm_device.ol_waitevent);
    break;
    case MEM_OFFLINE:
    scoped_guard(spinlock_irqsave, &dm_device.ha_lock) {
    pfn_count = hv_page_offline_check(mem.start_pfn,
    mem.nr_pages);
    if (pfn_count <= dm_device.num_pages_onlined) {
    dm_device.num_pages_onlined -= pfn_count;
    } else {
//
// We're offlining more pages than we
// managed to online. This is
// unexpected. In any case don't let
// num_pages_onlined wrap around zero.
//
    WARN_ON_ONCE(1);
    dm_device.num_pages_onlined = 0;
    }
    }
    break;
    case MEM_GOING_ONLINE:
    case MEM_GOING_OFFLINE:
    case MEM_CANCEL_OFFLINE:
    break;
    }
    return NOTIFY_OK;
    }
    static struct notifier_block hv_memory_nb = {
    .notifier_call = hv_memory_notifier,
    .priority = 0
    };
// Check if the particular page is backed and can be onlined and online it.
#[no_mangle]
unsafe extern "C" fn hv_page_online_one(has: *mut hv_hotadd_state, pg: *mut page) {
    static void hv_page_online_one(struct hv_hotadd_state *has, struct page *pg)
    {
    if (!has_pfn_is_backed(has, page_to_pfn(pg))) {
    if (!PageOffline(pg))
    __SetPageOffline(pg);
    return;
    } else if (!PageOffline(pg))
    return;
// This frame is currently backed; online the page.
    generic_online_page(pg, 0);
    lockdep_assert_held(&dm_device.ha_lock);
    dm_device.num_pages_onlined++;
    }
    static void hv_bring_pgs_online(struct hv_hotadd_state *has,
    unsigned long start_pfn, unsigned long size)
    {
    int i;
    pr_debug("Online %lu pages starting at pfn 0x%lx\n", size, start_pfn);
    for (i = 0; i < size; i++)
    hv_page_online_one(has, pfn_to_page(start_pfn + i));
    }
    static void hv_mem_hot_add(unsigned long start, unsigned long size,
    unsigned long pfn_count,
    struct hv_hotadd_state *has)
    {
    let mut ret: c_int = 0;
    int i, nid;
    unsigned long start_pfn;
    unsigned long processed_pfn;
    let mut total_pfn: c_ulong = pfn_count;
    for (i = 0; i < (size/ha_pages_in_chunk); i++) {
    start_pfn = start + (i * ha_pages_in_chunk);
    scoped_guard(spinlock_irqsave, &dm_device.ha_lock) {
    has.ha_end_pfn += ha_pages_in_chunk;
    processed_pfn = umin(total_pfn, ha_pages_in_chunk);
    total_pfn -= processed_pfn;
    has.covered_end_pfn += processed_pfn;
    }
    reinit_completion(&dm_device.ol_waitevent);
    nid = memory_add_physaddr_to_nid(PFN_PHYS(start_pfn));
    ret = add_memory(nid, PFN_PHYS((start_pfn)),
    HA_BYTES_IN_CHUNK, MHP_MERGE_RESOURCE);
    if (ret) {
    pr_err("hot_add memory failed error is %d\n", ret);
    if (ret == -EEXIST) {
//
// This error indicates that the error
// is not a transient failure. This is the
// case where the guest's physical address map
// precludes hot adding memory. Stop all further
// memory hot-add.
//
    do_hot_add = false;
    }
    scoped_guard(spinlock_irqsave, &dm_device.ha_lock) {
    has.ha_end_pfn -= ha_pages_in_chunk;
    has.covered_end_pfn -=  processed_pfn;
    }
    break;
    }
//
// Wait for memory to get onlined. If the kernel onlined the
// memory when adding it, this will return directly. Otherwise,
// it will wait for user space to online the memory. This helps
// to avoid adding memory faster than it is getting onlined. As
// adding succeeded, it is ok to proceed even if the memory was
// not onlined in time.
//
    wait_for_completion_timeout(&dm_device.ol_waitevent, secs_to_jiffies(5));
    post_status(&dm_device);
    }
    }
#[no_mangle]
unsafe extern "C" fn hv_online_page(pg: *mut page, order: c_uint) {
    static void hv_online_page(struct page *pg, unsigned int order)
    {
    struct hv_hotadd_state *has;
    let mut pfn: c_ulong = page_to_pfn(pg);
    scoped_guard(spinlock_irqsave, &dm_device.ha_lock) {
    list_for_each_entry(has, &dm_device.ha_region_list, list) {
// The page belongs to a different HAS.
    if (pfn < has.start_pfn ||
    (pfn + (1UL << order) > has.end_pfn))
    continue;
    hv_bring_pgs_online(has, pfn, 1UL << order);
    return;
    }
    }
    generic_online_page(pg, order);
    }
#[no_mangle]
unsafe extern "C" fn pfn_covered(start_pfn: c_ulong, pfn_cnt: c_ulong) -> c_int {
    static int pfn_covered(unsigned long start_pfn, unsigned long pfn_cnt)
    {
    struct hv_hotadd_state *has;
    struct hv_hotadd_gap *gap;
    unsigned long residual;
    let mut ret: c_int = 0;
    guard(spinlock_irqsave)(&dm_device.ha_lock);
    list_for_each_entry(has, &dm_device.ha_region_list, list) {
//
// If the pfn range we are dealing with is not in the current
// "hot add block", move on.
//
    if (start_pfn < has.start_pfn || start_pfn >= has.end_pfn)
    continue;
//
// If the current start pfn is not where the covered_end
// is, create a gap and update covered_end_pfn.
//
    if (has.covered_end_pfn != start_pfn) {
    gap = kzalloc_obj(struct hv_hotadd_gap, GFP_ATOMIC);
    if (!gap) {
    ret = -ENOMEM;
    break;
    }
    INIT_LIST_HEAD(&gap.list);
    gap.start_pfn = has.covered_end_pfn;
    gap.end_pfn = start_pfn;
    list_add_tail(&gap.list, &has.gap_list);
    has.covered_end_pfn = start_pfn;
    }
//
// If the current hot add-request extends beyond
// our current limit; extend it.
//
    if ((start_pfn + pfn_cnt) > has.end_pfn) {
// Extend the region by multiples of ha_pages_in_chunk
    residual = (start_pfn + pfn_cnt - has.end_pfn);
    has.end_pfn += ALIGN(residual, ha_pages_in_chunk);
    }
    ret = 1;
    break;
    }
    return ret;
    }
    static unsigned long handle_pg_range(unsigned long pg_start,
    unsigned long pg_count)
    {
    let mut start_pfn: c_ulong = pg_start;
    let mut pfn_cnt: c_ulong = pg_count;
    unsigned long size;
    struct hv_hotadd_state *has;
    let mut pgs_ol: c_ulong = 0;
    unsigned long old_covered_state;
    let mut res: c_ulong = 0, flags;
    pr_debug("Hot adding %lu pages starting at pfn 0x%lx.\n", pg_count,
    pg_start);
    spin_lock_irqsave(&dm_device.ha_lock, flags);
    list_for_each_entry(has, &dm_device.ha_region_list, list) {
//
// If the pfn range we are dealing with is not in the current
// "hot add block", move on.
//
    if (start_pfn < has.start_pfn || start_pfn >= has.end_pfn)
    continue;
    old_covered_state = has.covered_end_pfn;
    if (start_pfn < has.ha_end_pfn) {
//
// This is the case where we are backing pages
// in an already hot added region. Bring
// these pages online first.
//
    pgs_ol = has.ha_end_pfn - start_pfn;
    if (pgs_ol > pfn_cnt)
    pgs_ol = pfn_cnt;
    has.covered_end_pfn +=  pgs_ol;
    pfn_cnt -= pgs_ol;
//
// Check if the corresponding memory block is already
// online. It is possible to observe struct pages still
// being uninitialized here so check section instead.
// In case the section is online we need to bring the
// rest of pfns (which were not backed previously)
// online too.
//
    if (start_pfn > has.start_pfn &&
    online_section_nr(pfn_to_section_nr(start_pfn)))
    hv_bring_pgs_online(has, start_pfn, pgs_ol);
    }
    if (has.ha_end_pfn < has.end_pfn && pfn_cnt > 0) {
//
// We have some residual hot add range
// that needs to be hot added; hot add
// it now. Hot add a multiple of
// ha_pages_in_chunk that fully covers the pages
// we have.
//
    size = (has.end_pfn - has.ha_end_pfn);
    if (pfn_cnt <= size) {
    size = ALIGN(pfn_cnt, ha_pages_in_chunk);
    } else {
    pfn_cnt = size;
    }
    spin_unlock_irqrestore(&dm_device.ha_lock, flags);
    hv_mem_hot_add(has.ha_end_pfn, size, pfn_cnt, has);
    spin_lock_irqsave(&dm_device.ha_lock, flags);
    }
//
// If we managed to online any pages that were given to us,
// we declare success.
//
    res = has.covered_end_pfn - old_covered_state;
    break;
    }
    spin_unlock_irqrestore(&dm_device.ha_lock, flags);
    return res;
    }
    static unsigned long process_hot_add(unsigned long pg_start,
    unsigned long pfn_cnt,
    unsigned long rg_start,
    unsigned long rg_size)
    {
    struct hv_hotadd_state *ha_region = core::ptr::null_mut();
    int covered;
    if (pfn_cnt == 0)
    return 0;
    if (!dm_device.host_specified_ha_region) {
    covered = pfn_covered(pg_start, pfn_cnt);
    if (covered < 0)
    return 0;
    if (covered)
    goto do_pg_range;
    }
//
// If the host has specified a hot-add range; deal with it first.
//
    if (rg_size != 0) {
    ha_region = kzalloc_obj(struct hv_hotadd_state);
    if (!ha_region)
    return 0;
    INIT_LIST_HEAD(&ha_region.list);
    INIT_LIST_HEAD(&ha_region.gap_list);
    ha_region.start_pfn = rg_start;
    ha_region.ha_end_pfn = rg_start;
    ha_region.covered_start_pfn = pg_start;
    ha_region.covered_end_pfn = pg_start;
    ha_region.end_pfn = rg_start + rg_size;
    scoped_guard(spinlock_irqsave, &dm_device.ha_lock) {
    list_add_tail(&ha_region.list, &dm_device.ha_region_list);
    }
    }
    do_pg_range:
//
// Process the page range specified; bringing them
// online if possible.
//
    return handle_pg_range(pg_start, pfn_cnt);
    }

#[no_mangle]
unsafe extern "C" fn hot_add_req(dummy: *mut work_struct) {
    static void hot_add_req(struct work_struct *dummy)
    {
    struct dm_hot_add_response resp;

    unsigned long pg_start, pfn_cnt;
    unsigned long rg_start, rg_sz;

    struct hv_dynmem_device *dm = &dm_device;
    memset(&resp, 0, sizeof(struct dm_hot_add_response));
    resp.hdr.type = DM_MEM_HOT_ADD_RESPONSE;
    resp.hdr.size = sizeof(struct dm_hot_add_response);

    pg_start = dm.ha_wrk.ha_page_range.finfo.start_page;
    pfn_cnt = dm.ha_wrk.ha_page_range.finfo.page_cnt;
    rg_start = dm.ha_wrk.ha_region_range.finfo.start_page;
    rg_sz = dm.ha_wrk.ha_region_range.finfo.page_cnt;
    if (rg_start == 0 && !dm.host_specified_ha_region) {
//
// The host has not specified the hot-add region.
// Based on the hot-add page range being specified,
// compute a hot-add region that can cover the pages
// that need to be hot-added while ensuring the alignment
// and size requirements of Linux as it relates to hot-add.
//
    rg_start = ALIGN_DOWN(pg_start, ha_pages_in_chunk);
    rg_sz = ALIGN(pfn_cnt, ha_pages_in_chunk);
    }
    if (do_hot_add)
    resp.page_count = process_hot_add(pg_start, pfn_cnt,
    rg_start, rg_sz);
    dm.num_pages_added += resp.page_count;

//
// The result field of the response structure has the
// following semantics:
//
// 1. If all or some pages hot-added: Guest should return success.
//
// 2. If no pages could be hot-added:
//
// If the guest returns success, then the host
// will not attempt any further hot-add operations. This
// signifies a permanent failure.
//
// If the guest returns failure, then this failure will be
// treated as a transient failure and the host may retry the
// hot-add operation after some delay.
//
    if (resp.page_count > 0)
    resp.result = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !do_hot_add) -> else {
    else if (!do_hot_add)
    resp.result = 1;
    else
    resp.result = 0;
    if (!do_hot_add || resp.page_count == 0) {
    if (!allow_hibernation)
    pr_err("Memory hot add failed\n");
    else
    pr_info("Ignore hot-add request!\n");
    }
    dm.state = DM_INITIALIZED;
    resp.hdr.trans_id = atomic_inc_return(&trans_id);
    vmbus_sendpacket(dm.dev.channel, &resp,
    sizeof(struct dm_hot_add_response),
    (unsigned long)core::ptr::null_mut(),
    VM_PKT_DATA_INBAND, 0);
    }
#[no_mangle]
unsafe extern "C" fn process_info(dm: *mut hv_dynmem_device, msg: *mut dm_info_msg) {
    static void process_info(struct hv_dynmem_device *dm, struct dm_info_msg *msg)
    {
    struct dm_info_header *info_hdr;
    info_hdr = (struct dm_info_header *)msg.info;
    switch (info_hdr.type) {
    case INFO_TYPE_MAX_PAGE_CNT:
    if (info_hdr.data_size == sizeof(__u64)) {
    __u64 *max_page_count = (__u64 *)&info_hdr[1];
    pr_info("Max. dynamic memory size: %llu MB\n",
    (*max_page_count) >> (20 - HV_HYP_PAGE_SHIFT));
    dm.max_dynamic_page_count = *max_page_count;
    }
    break;
    default:
    pr_warn("Received Unknown type: %d\n", info_hdr.type);
    }
    }
#[no_mangle]
unsafe extern "C" fn compute_balloon_floor() -> c_ulong {
    static unsigned long compute_balloon_floor(void)
    {
    unsigned long min_pages;
    let mut nr_pages: c_ulong = totalram_pages();

// Simple continuous piecewiese linear function:
// max MiB -> min MiB  gradient
// 0         0
// 16        16
// 32        24
// 128        72    (1/2)
// 512       168    (1/4)
// 2048       360    (1/8)
// 8192       744    (1/16)
// 32768      1512	(1/32)
//
    if (nr_pages < MB2PAGES(128))
    min_pages = MB2PAGES(8) + (nr_pages >> 1);
#[no_mangle]
pub unsafe extern "C" fn if(MB2PAGES(512): nr_pages <) -> else {
    else if (nr_pages < MB2PAGES(512))
    min_pages = MB2PAGES(40) + (nr_pages >> 2);
#[no_mangle]
pub unsafe extern "C" fn if(MB2PAGES(2048): nr_pages <) -> else {
    else if (nr_pages < MB2PAGES(2048))
    min_pages = MB2PAGES(104) + (nr_pages >> 3);
#[no_mangle]
pub unsafe extern "C" fn if(MB2PAGES(8192): nr_pages <) -> else {
    else if (nr_pages < MB2PAGES(8192))
    min_pages = MB2PAGES(232) + (nr_pages >> 4);
    else
    min_pages = MB2PAGES(488) + (nr_pages >> 5);

    return min_pages;
    }
//
// Compute total committed memory pages
//
#[no_mangle]
unsafe extern "C" fn get_pages_committed(dm: *mut hv_dynmem_device) -> c_ulong {
    static unsigned long get_pages_committed(struct hv_dynmem_device *dm)
    {
    return vm_memory_committed() +
    dm.num_pages_ballooned +
    (dm.num_pages_added > dm.num_pages_onlined ?
    dm.num_pages_added - dm.num_pages_onlined : 0) +
    compute_balloon_floor();
    }
//
// Post our status as it relates memory pressure to the
// host. Host expects the guests to post this status
// periodically at 1 second intervals.
//
// The metrics specified in this protocol are very Windows
// specific and so we cook up numbers here to convey our memory
// pressure.
//
#[no_mangle]
unsafe extern "C" fn post_status(dm: *mut hv_dynmem_device) {
    static void post_status(struct hv_dynmem_device *dm)
    {
    struct dm_status status;
    let mut now: c_ulong = jiffies;
    let mut last_post: c_ulong = last_post_time;
    unsigned long num_pages_avail, num_pages_committed;
    if (pressure_report_delay > 0) {
    --pressure_report_delay;
    return;
    }
    if (!time_after(now, (last_post_time + HZ)))
    return;
    memset(&status, 0, sizeof(struct dm_status));
    status.hdr.type = DM_STATUS_REPORT;
    status.hdr.size = sizeof(struct dm_status);
    status.hdr.trans_id = atomic_inc_return(&trans_id);
//
// The host expects the guest to report free and committed memory.
// Furthermore, the host expects the pressure information to include
// the ballooned out pages. For a given amount of memory that we are
// managing we need to compute a floor below which we should not
// balloon. Compute this and add it to the pressure report.
// We also need to report all offline pages (num_pages_added -
// num_pages_onlined) as committed to the host, otherwise it can try
// asking us to balloon them out.
//
    num_pages_avail = si_mem_available();
    num_pages_committed = get_pages_committed(dm);
    trace_balloon_status(num_pages_avail, num_pages_committed,
    vm_memory_committed(), dm.num_pages_ballooned,
    dm.num_pages_added, dm.num_pages_onlined);
// Convert numbers of pages into numbers of HV_HYP_PAGEs.
    status.num_avail = num_pages_avail * NR_HV_HYP_PAGES_IN_PAGE;
    status.num_committed = num_pages_committed * NR_HV_HYP_PAGES_IN_PAGE;
//
// If our transaction ID is no longer current, just don't
// send the status. This can happen if we were interrupted
// after we picked our transaction ID.
//
    if (status.hdr.trans_id != atomic_read(&trans_id))
    return;
//
// If the last post time that we sampled has changed,
// we have raced, don't post the status.
//
    if (last_post != last_post_time)
    return;
    last_post_time = jiffies;
    vmbus_sendpacket(dm.dev.channel, &status,
    sizeof(struct dm_status),
    (unsigned long)core::ptr::null_mut(),
    VM_PKT_DATA_INBAND, 0);
    }
    static void free_balloon_pages(struct hv_dynmem_device *dm,
    union dm_mem_page_range *range_array)
    {
    let mut num_pages: c_int = range_array.finfo.page_cnt;
    let mut start_frame: __u64 = range_array.finfo.start_page;
    struct page *pg;
    int i;
    for (i = 0; i < num_pages; i++) {
    pg = pfn_to_page(i + start_frame);
    __ClearPageOffline(pg);
    __free_page(pg);
    dm.num_pages_ballooned--;
    mod_node_page_state(page_pgdat(pg), NR_BALLOON_PAGES, -1);
    adjust_managed_page_count(pg, 1);
    }
    }
    static unsigned int alloc_balloon_pages(struct hv_dynmem_device *dm,
    unsigned int num_pages,
    struct dm_balloon_response *bl_resp,
    int alloc_unit)
    {
    unsigned int i, j;
    struct page *pg;
    for (i = 0; i < num_pages / alloc_unit; i++) {
    if (bl_resp.hdr.size + sizeof(union dm_mem_page_range) >
    HV_HYP_PAGE_SIZE)
    return i * alloc_unit;
//
// We execute this code in a thread context. Furthermore,
// we don't want the kernel to try too hard.
//
    pg = alloc_pages(GFP_HIGHUSER | __GFP_NORETRY |
    __GFP_NOMEMALLOC | __GFP_NOWARN,
    get_order(alloc_unit << PAGE_SHIFT));
    if (!pg)
    return i * alloc_unit;
    dm.num_pages_ballooned += alloc_unit;
    mod_node_page_state(page_pgdat(pg), NR_BALLOON_PAGES, alloc_unit);
//
// If we allocatted 2M pages; split them so we
// can free them in any order we get.
//
    if (alloc_unit != 1)
    split_page(pg, get_order(alloc_unit << PAGE_SHIFT));
// mark all pages offline
    for (j = 0; j < alloc_unit; j++) {
    __SetPageOffline(pg + j);
    adjust_managed_page_count(pg + j, -1);
    }
    bl_resp.range_count++;
    bl_resp.range_array[i].finfo.start_page =
    page_to_pfn(pg);
    bl_resp.range_array[i].finfo.page_cnt = alloc_unit;
    bl_resp.hdr.size += sizeof(union dm_mem_page_range);
    }
    return i * alloc_unit;
    }
#[no_mangle]
unsafe extern "C" fn balloon_up(dummy: *mut work_struct) {
    static void balloon_up(struct work_struct *dummy)
    {
    let mut num_pages: c_uint = dm_device.balloon_wrk.num_pages;
    let mut num_ballooned: c_uint = 0;
    struct dm_balloon_response *bl_resp;
    int alloc_unit;
    int ret;
    let mut done: bool = false;
    int i;
    long avail_pages;
    unsigned long floor;
//
// We will attempt 2M allocations. However, if we fail to
// allocate 2M chunks, we will go back to PAGE_SIZE allocations.
//
    alloc_unit = PAGES_IN_2M;
    avail_pages = si_mem_available();
    floor = compute_balloon_floor();
// Refuse to balloon below the floor.
    if (avail_pages < num_pages || avail_pages - num_pages < floor) {
    pr_info("Balloon request will be partially fulfilled. %s\n",
    avail_pages < num_pages ? "Not enough memory." :
    "Balloon floor reached.");
    num_pages = avail_pages > floor ? (avail_pages - floor) : 0;
    }
    while (!done) {
    memset(balloon_up_send_buffer, 0, HV_HYP_PAGE_SIZE);
    bl_resp = (struct dm_balloon_response *)balloon_up_send_buffer;
    bl_resp.hdr.type = DM_BALLOON_RESPONSE;
    bl_resp.hdr.size = sizeof(struct dm_balloon_response);
    bl_resp.more_pages = 1;
    num_pages -= num_ballooned;
    num_ballooned = alloc_balloon_pages(&dm_device, num_pages,
    bl_resp, alloc_unit);
    if (alloc_unit != 1 && num_ballooned == 0) {
    alloc_unit = 1;
    continue;
    }
    if (num_ballooned == 0 || num_ballooned == num_pages) {
    pr_debug("Ballooned %u out of %u requested pages.\n",
    num_pages, dm_device.balloon_wrk.num_pages);
    bl_resp.more_pages = 0;
    done = true;
    dm_device.state = DM_INITIALIZED;
    }
//
// We are pushing a lot of data through the channel;
// deal with transient failures caused because of the
// lack of space in the ring buffer.
//
    do {
    bl_resp.hdr.trans_id = atomic_inc_return(&trans_id);
    ret = vmbus_sendpacket(dm_device.dev.channel,
    bl_resp,
    bl_resp.hdr.size,
    (unsigned long)core::ptr::null_mut(),
    VM_PKT_DATA_INBAND, 0);
    if (ret == -EAGAIN)
    msleep(20);
    post_status(&dm_device);
    } while (ret == -EAGAIN);
    if (ret) {
//
// Free up the memory we allocatted.
//
    pr_err("Balloon response failed\n");
    for (i = 0; i < bl_resp.range_count; i++)
    free_balloon_pages(&dm_device,
    &bl_resp.range_array[i]);
    done = true;
    }
    }
    }
    static void balloon_down(struct hv_dynmem_device *dm,
    struct dm_unballoon_request *req)
    {
    union dm_mem_page_range *range_array = req.range_array;
    let mut range_count: c_int = req.range_count;
    struct dm_unballoon_response resp;
    int i;
    let mut prev_pages_ballooned: c_uint = dm.num_pages_ballooned;
    for (i = 0; i < range_count; i++) {
    free_balloon_pages(dm, &range_array[i]);
    complete(&dm_device.config_event);
    }
    pr_debug("Freed %u ballooned pages.\n",
    prev_pages_ballooned - dm.num_pages_ballooned);
    if (req.more_pages == 1)
    return;
    memset(&resp, 0, sizeof(struct dm_unballoon_response));
    resp.hdr.type = DM_UNBALLOON_RESPONSE;
    resp.hdr.trans_id = atomic_inc_return(&trans_id);
    resp.hdr.size = sizeof(struct dm_unballoon_response);
    vmbus_sendpacket(dm_device.dev.channel, &resp,
    sizeof(struct dm_unballoon_response),
    (unsigned long)core::ptr::null_mut(),
    VM_PKT_DATA_INBAND, 0);
    dm.state = DM_INITIALIZED;
    }
    static void balloon_onchannelcallback(void *context);
#[no_mangle]
unsafe extern "C" fn dm_thread_func(dm_dev: *mut c_void) -> c_int {
    static int dm_thread_func(void *dm_dev)
    {
    struct hv_dynmem_device *dm = dm_dev;
    while (!kthread_should_stop()) {
    wait_for_completion_interruptible_timeout(&dm_device.config_event,
    secs_to_jiffies(1));
//
// The host expects us to post information on the memory
// pressure every second.
//
    reinit_completion(&dm_device.config_event);
    post_status(dm);
//
// disable free page reporting if multiple hypercall
// failure flag set. It is not done in the page_reporting
// callback context as that causes a deadlock between
// page_reporting_process() and page_reporting_unregister()
//
    if (hv_hypercall_multi_failure >= HV_MAX_FAILURES) {
    pr_err("Multiple failures in cold memory discard hypercall, disabling page reporting\n");
    disable_page_reporting();
// Reset the flag after disabling reporting
    hv_hypercall_multi_failure = 0;
    }
    }
    return 0;
    }
    static void version_resp(struct hv_dynmem_device *dm,
    struct dm_version_response *vresp)
    {
    struct dm_version_request version_req;
    int ret;
    if (vresp.is_accepted) {
//
// We are done; wakeup the
// context waiting for version
// negotiation.
//
    complete(&dm.host_event);
    return;
    }
//
// If there are more versions to try, continue
// with negotiations; if not
// shutdown the service since we are not able
// to negotiate a suitable version number
// with the host.
//
    if (dm.next_version == 0)
    goto version_error;
    memset(&version_req, 0, sizeof(struct dm_version_request));
    version_req.hdr.type = DM_VERSION_REQUEST;
    version_req.hdr.size = sizeof(struct dm_version_request);
    version_req.hdr.trans_id = atomic_inc_return(&trans_id);
    version_req.version.version = dm.next_version;
    dm.version = version_req.version.version;
// Set the next version to try in case current version fails.
    dm.next_version = 0;
    version_req.is_last_attempt = 1;
    ret = vmbus_sendpacket(dm.dev.channel, &version_req,
    sizeof(struct dm_version_request),
    (unsigned long)core::ptr::null_mut(),
    VM_PKT_DATA_INBAND, 0);
    if (ret)
    goto version_error;
    return;
    version_error:
    dm.state = DM_INIT_ERROR;
    complete(&dm.host_event);
    }
    static void cap_resp(struct hv_dynmem_device *dm,
    struct dm_capabilities_resp_msg *cap_resp)
    {
    if (!cap_resp.is_accepted) {
    pr_err("Capabilities not accepted by host\n");
    dm.state = DM_INIT_ERROR;
    }
    complete(&dm.host_event);
    }
#[no_mangle]
unsafe extern "C" fn balloon_onchannelcallback(context: *mut c_void) {
    static void balloon_onchannelcallback(void *context)
    {
    struct hv_device *dev = context;
    u32 recvlen;
    u64 requestid;
    struct dm_message *dm_msg;
    struct dm_header *dm_hdr;
    struct hv_dynmem_device *dm = hv_get_drvdata(dev);
    struct dm_balloon *bal_msg;
    struct dm_hot_add *ha_msg;
    union dm_mem_page_range *ha_pg_range;
    union dm_mem_page_range *ha_region;
    memset(recv_buffer, 0, sizeof(recv_buffer));
    vmbus_recvpacket(dev.channel, recv_buffer,
    HV_HYP_PAGE_SIZE, &recvlen, &requestid);
    if (recvlen > 0) {
    dm_msg = (struct dm_message *)recv_buffer;
    dm_hdr = &dm_msg.hdr;
    switch (dm_hdr.type) {
    case DM_VERSION_RESPONSE:
    version_resp(dm,
    (struct dm_version_response *)dm_msg);
    break;
    case DM_CAPABILITIES_RESPONSE:
    cap_resp(dm,
    (struct dm_capabilities_resp_msg *)dm_msg);
    break;
    case DM_BALLOON_REQUEST:
    if (allow_hibernation) {
    pr_info("Ignore balloon-up request!\n");
    break;
    }
    if (dm.state == DM_BALLOON_UP)
    pr_warn("Currently ballooning\n");
    bal_msg = (struct dm_balloon *)recv_buffer;
    dm.state = DM_BALLOON_UP;
    dm_device.balloon_wrk.num_pages = bal_msg.num_pages;
    schedule_work(&dm_device.balloon_wrk.wrk);
    break;
    case DM_UNBALLOON_REQUEST:
    if (allow_hibernation) {
    pr_info("Ignore balloon-down request!\n");
    break;
    }
    dm.state = DM_BALLOON_DOWN;
    balloon_down(dm,
    (struct dm_unballoon_request *)recv_buffer);
    break;
    case DM_MEM_HOT_ADD_REQUEST:
    if (dm.state == DM_HOT_ADD)
    pr_warn("Currently hot-adding\n");
    dm.state = DM_HOT_ADD;
    ha_msg = (struct dm_hot_add *)recv_buffer;
    if (ha_msg.hdr.size == sizeof(struct dm_hot_add)) {
//
// This is a normal hot-add request specifying
// hot-add memory.
//
    dm.host_specified_ha_region = false;
    ha_pg_range = &ha_msg.range;
    dm.ha_wrk.ha_page_range = *ha_pg_range;
    dm.ha_wrk.ha_region_range.page_range = 0;
    } else {
//
// Host is specifying that we first hot-add
// a region and then partially populate this
// region.
//
    dm.host_specified_ha_region = true;
    ha_pg_range = &ha_msg.range;
    ha_region = &ha_pg_range[1];
    dm.ha_wrk.ha_page_range = *ha_pg_range;
    dm.ha_wrk.ha_region_range = *ha_region;
    }
    schedule_work(&dm_device.ha_wrk.wrk);
    break;
    case DM_INFO_MESSAGE:
    process_info(dm, (struct dm_info_msg *)dm_msg);
    break;
    default:
    pr_warn_ratelimited("Unhandled message: type: %d\n", dm_hdr.type);
    }
    }
    }
pub const HV_LARGE_REPORTING_ORDER: c_int = 9;

    HV_LARGE_REPORTING_ORDER)
    static int hv_free_page_report(struct page_reporting_dev_info *pr_dev_info,
    struct scatterlist *sgl, unsigned int nents)
    {
    unsigned long flags;
    struct hv_memory_hint *hint;
    int i, order;
    u64 status;
    struct scatterlist *sg;
    WARN_ON_ONCE(nents > HV_MEMORY_HINT_MAX_GPA_PAGE_RANGES);
    WARN_ON_ONCE(sgl.length < (HV_HYP_PAGE_SIZE << page_reporting_order));
    local_irq_save(flags);
    hint = *this_cpu_ptr(hyperv_pcpu_input_arg);
    if (!hint) {
    local_irq_restore(flags);
    return -ENOSPC;
    }
    hint.heat_type = HV_EXTMEM_HEAT_HINT_COLD_DISCARD;
    hint.reserved = 0;
    for_each_sg(sgl, sg, nents, i) {
    union hv_gpa_page_range *range;
    range = &hint.ranges[i];
    range.address_space = 0;
    order = get_order(sg.length);
//
// Hyper-V expects the additional_pages field in the units
// of one of these 3 sizes, 4Kbytes, 2Mbytes or 1Gbytes.
// This is dictated by the values of the fields page.largesize
// and page_size.
// This code however, only uses 4Kbytes and 2Mbytes units
// and not 1Gbytes unit.
//
// page reporting for pages 2MB or higher
    if (order >= HV_LARGE_REPORTING_ORDER) {
    range.page.largepage = 1;
    range.page_size = HV_GPA_PAGE_RANGE_PAGE_SIZE_2MB;
    range.base_large_pfn = page_to_hvpfn(
    sg_page(sg)) >> HV_LARGE_REPORTING_ORDER;
    range.page.additional_pages =
    (sg.length / HV_LARGE_REPORTING_LEN) - 1;
    } else {
// Page reporting for pages below 2MB
    range.page.basepfn = page_to_hvpfn(sg_page(sg));
    range.page.largepage = false;
    range.page.additional_pages =
    (sg.length / HV_HYP_PAGE_SIZE) - 1;
    }
    }
    status = hv_do_rep_hypercall(HV_EXT_CALL_MEMORY_HEAT_HINT, nents, 0,
    hint, core::ptr::null_mut());
    local_irq_restore(flags);
    if (!hv_result_success(status)) {
    pr_err("Cold memory discard hypercall failed with status %llx\n",
    status);
    if (hv_hypercall_multi_failure > 0)
    hv_hypercall_multi_failure++;
    if (hv_result(status) == HV_STATUS_INVALID_PARAMETER) {
    pr_err("Underlying Hyper-V does not support order less than 9. Hypercall failed\n");
    pr_err("Defaulting to page_reporting_order %d\n",
    pageblock_order);
    page_reporting_order = pageblock_order;
    hv_hypercall_multi_failure++;
    return -EINVAL;
    }
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_page_reporting() {
    static void enable_page_reporting(void)
    {
    int ret;
    if (!hv_query_ext_cap(HV_EXT_CAPABILITY_MEMORY_COLD_DISCARD_HINT)) {
    pr_debug("Cold memory discard hint not supported by Hyper-V\n");
    return;
    }
    BUILD_BUG_ON(PAGE_REPORTING_CAPACITY > HV_MEMORY_HINT_MAX_GPA_PAGE_RANGES);
    dm_device.pr_dev_info.report = hv_free_page_report;
//
// We let the page_reporting_order parameter decide the order
// in the page_reporting code
//
    dm_device.pr_dev_info.order = PAGE_REPORTING_ORDER_UNSPECIFIED;
    ret = page_reporting_register(&dm_device.pr_dev_info);
    if (ret < 0) {
    dm_device.pr_dev_info.report = core::ptr::null_mut();
    pr_err("Failed to enable cold memory discard: %d\n", ret);
    } else {
    pr_info("Cold memory discard hint enabled with order %d\n",
    page_reporting_order);
    }
    }
#[no_mangle]
unsafe extern "C" fn disable_page_reporting() {
    static void disable_page_reporting(void)
    {
    if (dm_device.pr_dev_info.report) {
    page_reporting_unregister(&dm_device.pr_dev_info);
    dm_device.pr_dev_info.report = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn ballooning_enabled() -> c_int {
    static int ballooning_enabled(void)
    {
//
// Disable ballooning if the page size is not 4k (HV_HYP_PAGE_SIZE),
// since currently it's unclear to us whether an unballoon request can
// make sure all page ranges are guest page size aligned.
//
    if (PAGE_SIZE != HV_HYP_PAGE_SIZE) {
    pr_info("Ballooning disabled because page size is not 4096 bytes\n");
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hot_add_enabled() -> c_int {
    static int hot_add_enabled(void)
    {
//
// Disable hot add on ARM64, because we currently rely on
// memory_add_physaddr_to_nid() to get a node id of a hot add range,
// however ARM64's memory_add_physaddr_to_nid() always return 0 and
// DM_MEM_HOT_ADD_REQUEST doesn't have the NUMA node information for
// add_memory().
//
    if (IS_ENABLED(CONFIG_ARM64)) {
    pr_info("Memory hot add disabled on ARM64\n");
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn balloon_connect_vsp(dev: *mut hv_device) -> c_int {
    static int balloon_connect_vsp(struct hv_device *dev)
    {
    struct dm_version_request version_req;
    struct dm_capabilities cap_msg;
    unsigned long t;
    int ret;
//
// max_pkt_size should be large enough for one vmbus packet header plus
// our receive buffer size. Hyper-V sends messages up to
// HV_HYP_PAGE_SIZE bytes long on balloon channel.
//
    dev.channel.max_pkt_size = HV_HYP_PAGE_SIZE * 2;
    ret = vmbus_open(dev.channel, dm_ring_size, dm_ring_size, core::ptr::null_mut(), 0,
    balloon_onchannelcallback, dev);
    if (ret)
    return ret;
//
// Initiate the hand shake with the host and negotiate
// a version that the host can support. The mechanism is in place
// to start with the highest version number and go down if the host
// cannot support it. But currently we only try the WIN10 version
// since support for older Hyper-V versions has been removed from
// Linux.
//
    memset(&version_req, 0, sizeof(struct dm_version_request));
    version_req.hdr.type = DM_VERSION_REQUEST;
    version_req.hdr.size = sizeof(struct dm_version_request);
    version_req.hdr.trans_id = atomic_inc_return(&trans_id);
    version_req.version.version = DYNMEM_PROTOCOL_VERSION_WIN10;
    version_req.is_last_attempt = 1;
    dm_device.version = version_req.version.version;
    ret = vmbus_sendpacket(dev.channel, &version_req,
    sizeof(struct dm_version_request),
    (unsigned long)core::ptr::null_mut(), VM_PKT_DATA_INBAND, 0);
    if (ret)
    goto out;
    t = wait_for_completion_timeout(&dm_device.host_event, secs_to_jiffies(5));
    if (t == 0) {
    ret = -ETIMEDOUT;
    goto out;
    }
//
// If we could not negotiate a compatible version with the host
// fail the probe function.
//
    if (dm_device.state == DM_INIT_ERROR) {
    ret = -EPROTO;
    goto out;
    }
    pr_info("Using Dynamic Memory protocol version %u.%u\n",
    DYNMEM_MAJOR_VERSION(dm_device.version),
    DYNMEM_MINOR_VERSION(dm_device.version));
//
// Now submit our capabilities to the host.
//
    memset(&cap_msg, 0, sizeof(struct dm_capabilities));
    cap_msg.hdr.type = DM_CAPABILITIES_REPORT;
    cap_msg.hdr.size = sizeof(struct dm_capabilities);
    cap_msg.hdr.trans_id = atomic_inc_return(&trans_id);
//
// When hibernation (i.e. virtual ACPI S4 state) is enabled, the host
// currently still requires the bits to be set, so we have to add code
// to fail the host's hot-add and balloon up/down requests, if any.
//
    cap_msg.caps.cap_bits.balloon = ballooning_enabled();
    cap_msg.caps.cap_bits.hot_add = hot_add_enabled();
//
// Specify our alignment requirements for memory hot-add. The value is
// the log base 2 of the number of megabytes in a chunk. For example,
// with 256 MiB chunks, the value is 8. The number of MiB in a chunk
// must be a power of 2.
//
    cap_msg.caps.cap_bits.hot_add_alignment =
    ilog2(HA_BYTES_IN_CHUNK / SZ_1M);
//
// Currently the host does not use these
// values and we set them to what is done in the
// Windows driver.
//
    cap_msg.min_page_cnt = 0;
    cap_msg.max_page_number = -1;
    ret = vmbus_sendpacket(dev.channel, &cap_msg,
    sizeof(struct dm_capabilities),
    (unsigned long)core::ptr::null_mut(), VM_PKT_DATA_INBAND, 0);
    if (ret)
    goto out;
    t = wait_for_completion_timeout(&dm_device.host_event, secs_to_jiffies(5));
    if (t == 0) {
    ret = -ETIMEDOUT;
    goto out;
    }
//
// If the host does not like our capabilities,
// fail the probe function.
//
    if (dm_device.state == DM_INIT_ERROR) {
    ret = -EPROTO;
    goto out;
    }
    return 0;
    out:
    vmbus_close(dev.channel);
    return ret;
    }
//
// DEBUGFS Interface
//

//
// hv_balloon_debug_show - shows statistics of balloon operations.
// @f: pointer to the &struct seq_file.
// @offset: ignored.
//
// Provides the statistics that can be accessed in hv-balloon in the debugfs.
//
// Return: zero on success or an error code.
//
#[no_mangle]
unsafe extern "C" fn hv_balloon_debug_show(f: *mut seq_file, offset: *mut c_void) -> c_int {
    static int hv_balloon_debug_show(struct seq_file *f, void *offset)
    {
    struct hv_dynmem_device *dm = f.private;
    char *sname;
    seq_printf(f, "%-22s: %u.%u\n", "host_version",
    DYNMEM_MAJOR_VERSION(dm.version),
    DYNMEM_MINOR_VERSION(dm.version));
    seq_printf(f, "%-22s:", "capabilities");
    if (ballooning_enabled())
    seq_puts(f, " enabled");
    if (hot_add_enabled())
    seq_puts(f, " hot_add");
    seq_printf(f, "\n%-22s: %u", "state", dm.state);
    switch (dm.state) {
    case DM_INITIALIZING:
    sname = "Initializing";
    break;
    case DM_INITIALIZED:
    sname = "Initialized";
    break;
    case DM_BALLOON_UP:
    sname = "Balloon Up";
    break;
    case DM_BALLOON_DOWN:
    sname = "Balloon Down";
    break;
    case DM_HOT_ADD:
    sname = "Hot Add";
    break;
    case DM_INIT_ERROR:
    sname = "Error";
    break;
    default:
    sname = "Unknown";
    }
    seq_printf(f, " (%s)\n", sname);
// HV Page Size
    seq_printf(f, "%-22s: %ld\n", "page_size", HV_HYP_PAGE_SIZE);
// Pages added with hot_add
    seq_printf(f, "%-22s: %u\n", "pages_added", dm.num_pages_added);
// pages that are "onlined"/used from pages_added
    seq_printf(f, "%-22s: %u\n", "pages_onlined", dm.num_pages_onlined);
// pages we have given back to host
    seq_printf(f, "%-22s: %u\n", "pages_ballooned", dm.num_pages_ballooned);
    seq_printf(f, "%-22s: %lu\n", "total_pages_committed",
    get_pages_committed(dm));
    seq_printf(f, "%-22s: %llu\n", "max_dynamic_page_count",
    dm.max_dynamic_page_count);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(hv_balloon_debug);
#[no_mangle]
unsafe extern "C" fn hv_balloon_debugfs_init(b: *mut hv_dynmem_device) {
    static void  hv_balloon_debugfs_init(struct hv_dynmem_device *b)
    {
    debugfs_create_file("hv-balloon", 0444, core::ptr::null_mut(), b,
    &hv_balloon_debug_fops);
    }
#[no_mangle]
unsafe extern "C" fn hv_balloon_debugfs_exit(b: *mut hv_dynmem_device) {
    static void  hv_balloon_debugfs_exit(struct hv_dynmem_device *b)
    {
    debugfs_lookup_and_remove("hv-balloon", core::ptr::null_mut());
    }

#[no_mangle]
pub unsafe extern "C" fn hv_balloon_debugfs_init(b: *mut hv_dynmem_device) {
    static inline void hv_balloon_debugfs_init(struct hv_dynmem_device  *b)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn hv_balloon_debugfs_exit(b: *mut hv_dynmem_device) {
    static inline void hv_balloon_debugfs_exit(struct hv_dynmem_device *b)
    {
    }

    static int balloon_probe(struct hv_device *dev,
    const struct hv_vmbus_device_id *dev_id)
    {
    int ret;
    allow_hibernation = hv_is_hibernation_supported();
    if (allow_hibernation)
    hot_add = false;

//
// Hot-add must operate in chunks that are of size equal to the
// memory block size because that's what the core add_memory()
// interface requires. The Hyper-V interface requires that the memory
// block size be a power of 2, which is guaranteed by the check in
// memory_dev_init().
//
    ha_pages_in_chunk = memory_block_size_bytes() / PAGE_SIZE;
    do_hot_add = hot_add;

//
// Without MEMORY_HOTPLUG, the guest returns a failure status for all
// hot add requests from Hyper-V, and the chunk size is used only to
// specify alignment to Hyper-V as required by the host/guest protocol.
// Somewhat arbitrarily, use 128 MiB.
//
    ha_pages_in_chunk = SZ_128M / PAGE_SIZE;
    do_hot_add = false;

    dm_device.dev = dev;
    dm_device.state = DM_INITIALIZING;
    dm_device.next_version = 0;
    init_completion(&dm_device.host_event);
    init_completion(&dm_device.config_event);
    INIT_LIST_HEAD(&dm_device.ha_region_list);
    spin_lock_init(&dm_device.ha_lock);
    INIT_WORK(&dm_device.balloon_wrk.wrk, balloon_up);
    INIT_WORK(&dm_device.ha_wrk.wrk, hot_add_req);
    dm_device.host_specified_ha_region = false;

    set_online_page_callback(&hv_online_page);
    init_completion(&dm_device.ol_waitevent);
    register_memory_notifier(&hv_memory_nb);

    hv_set_drvdata(dev, &dm_device);
    ret = balloon_connect_vsp(dev);
    if (ret != 0)
    goto connect_error;
    enable_page_reporting();
    dm_device.state = DM_INITIALIZED;
    dm_device.thread =
    kthread_run(dm_thread_func, &dm_device, "hv_balloon");
    if (IS_ERR(dm_device.thread)) {
    ret = PTR_ERR(dm_device.thread);
    goto probe_error;
    }
    hv_balloon_debugfs_init(&dm_device);
    return 0;
    probe_error:
    dm_device.state = DM_INIT_ERROR;
    dm_device.thread  = core::ptr::null_mut();
    disable_page_reporting();
    vmbus_close(dev.channel);
    connect_error:

    unregister_memory_notifier(&hv_memory_nb);
    restore_online_page_callback(&hv_online_page);

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn balloon_remove(dev: *mut hv_device) {
    static void balloon_remove(struct hv_device *dev)
    {
    struct hv_dynmem_device *dm = hv_get_drvdata(dev);
    struct hv_hotadd_state *has, *tmp;
    struct hv_hotadd_gap *gap, *tmp_gap;
    if (dm.num_pages_ballooned != 0)
    pr_warn("Ballooned pages: %d\n", dm.num_pages_ballooned);
    hv_balloon_debugfs_exit(dm);
    cancel_work_sync(&dm.balloon_wrk.wrk);
    cancel_work_sync(&dm.ha_wrk.wrk);
    kthread_stop(dm.thread);
//
// This is to handle the case when balloon_resume()
// call has failed and some cleanup has been done as
// a part of the error handling.
//
    if (dm_device.state != DM_INIT_ERROR) {
    disable_page_reporting();
    vmbus_close(dev.channel);

    unregister_memory_notifier(&hv_memory_nb);
    restore_online_page_callback(&hv_online_page);

    }
    guard(spinlock_irqsave)(&dm_device.ha_lock);
    list_for_each_entry_safe(has, tmp, &dm.ha_region_list, list) {
    list_for_each_entry_safe(gap, tmp_gap, &has.gap_list, list) {
    list_del(&gap.list);
    kfree(gap);
    }
    list_del(&has.list);
    kfree(has);
    }
    }
#[no_mangle]
unsafe extern "C" fn balloon_suspend(hv_dev: *mut hv_device) -> c_int {
    static int balloon_suspend(struct hv_device *hv_dev)
    {
    struct hv_dynmem_device *dm = hv_get_drvdata(hv_dev);
    tasklet_disable(&hv_dev.channel.callback_event);
    cancel_work_sync(&dm.balloon_wrk.wrk);
    cancel_work_sync(&dm.ha_wrk.wrk);
    if (dm.thread) {
    kthread_stop(dm.thread);
    dm.thread = core::ptr::null_mut();
    vmbus_close(hv_dev.channel);
    }
    tasklet_enable(&hv_dev.channel.callback_event);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn balloon_resume(dev: *mut hv_device) -> c_int {
    static int balloon_resume(struct hv_device *dev)
    {
    int ret;
    dm_device.state = DM_INITIALIZING;
    ret = balloon_connect_vsp(dev);
    if (ret != 0)
    goto out;
    dm_device.thread =
    kthread_run(dm_thread_func, &dm_device, "hv_balloon");
    if (IS_ERR(dm_device.thread)) {
    ret = PTR_ERR(dm_device.thread);
    dm_device.thread = core::ptr::null_mut();
    goto close_channel;
    }
    dm_device.state = DM_INITIALIZED;
    return 0;
    close_channel:
    vmbus_close(dev.channel);
    out:
    dm_device.state = DM_INIT_ERROR;
    disable_page_reporting();

    unregister_memory_notifier(&hv_memory_nb);
    restore_online_page_callback(&hv_online_page);

    return ret;
    }
    static const struct hv_vmbus_device_id id_table[] = {
// Dynamic Memory Class ID
// 525074DC-8985-46e2-8057-A307DC18A502
    { HV_DM_GUID, },
    { },
    };
    MODULE_DEVICE_TABLE(vmbus, id_table);
    static  struct hv_driver balloon_drv = {
    .name = "hv_balloon",
    .id_table = id_table,
    .probe =  balloon_probe,
    .remove =  balloon_remove,
    .suspend = balloon_suspend,
    .resume = balloon_resume,
    .driver = {
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn init_balloon_drv() -> int __init {
    static int __init init_balloon_drv(void)
    {
    return vmbus_driver_register(&balloon_drv);
    }
    module_init(init_balloon_drv);
    MODULE_DESCRIPTION("Hyper-V Balloon");
    MODULE_LICENSE("GPL");
