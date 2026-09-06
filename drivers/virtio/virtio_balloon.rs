//! Automatically rewritten from C to Rust
//! Source: drivers/virtio/virtio_balloon.c
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
//
// Virtio balloon implementation, inspired by Dor Laor and Marcelo
// Tosatti's implementations.
//
// Copyright 2008 Rusty Russell IBM Corporation
//

//
// Balloon device works in 4K page units.  So each page is pointed to by
// multiple balloon pages.  All memory counters in this driver are in balloon
// page units.
//

pub const VIRTIO_BALLOON_ARRAY_PFNS_MAX: c_int = 256;
// Maximum number of (4k) pages to deflate on OOM notifications.
pub const VIRTIO_BALLOON_OOM_NR_PAGES: c_int = 256;
pub const VIRTIO_BALLOON_OOM_NOTIFY_PRIORITY: c_int = 80;

    __GFP_NOMEMALLOC)
// The order of free page blocks to report to host

// The size of a free page block in bytes

    (1 << (VIRTIO_BALLOON_HINT_BLOCK_ORDER + PAGE_SHIFT))

    enum virtio_balloon_vq {
    VIRTIO_BALLOON_VQ_INFLATE,
    VIRTIO_BALLOON_VQ_DEFLATE,
    VIRTIO_BALLOON_VQ_STATS,
    VIRTIO_BALLOON_VQ_FREE_PAGE,
    VIRTIO_BALLOON_VQ_REPORTING,
    VIRTIO_BALLOON_VQ_MAX
    };
    enum virtio_balloon_config_read {
    VIRTIO_BALLOON_CONFIG_READ_CMD_ID = 0,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_balloon {
    pub vdev: *mut virtio_device,
    pub free_page_vq: *mut *mut *mut *mut virtqueue inflate_vq, deflate_vq, stats_vq,,
// Balloon's own wq for cpu-intensive work items
    pub balloon_wq: *mut workqueue_struct,
// The free page reporting work item submitted to the balloon wq
    pub report_free_page_work: work_struct,
// The balloon servicing is delegated to a freezable workqueue.
    pub update_balloon_stats_work: work_struct,
    pub update_balloon_size_work: work_struct,
// Prevent updating balloon when it is being canceled.
    pub stop_update_lock: spinlock_t,
    pub stop_update: bool,
// Bitmap to indicate if reading the related config fields are needed
    pub config_read_bitmap: c_ulong,
// The list of allocated free pages, waiting to be given back to mm
    pub free_page_list: list_head,
    pub free_page_list_lock: spinlock_t,
// The number of free page blocks on the above list
    pub num_free_page_blocks: c_ulong,
//
// The cmd id received from host.
// Read it via virtio_balloon_cmd_id_received to get the latest value
// sent from host.
//
    pub cmd_id_received_cache: u32,
// The cmd id that is actively in use
    pub cmd_id_active: __virtio32,
// Buffer to store the stop sign
    pub cmd_id_stop: __virtio32,
// Waiting for host to ack the pages we released.
    pub acked: wait_queue_head_t,
// Number of balloon pages we've told the Host we're not using.
    pub num_pages: c_uint,
//
// The pages we've told the Host we're not using are enqueued
// at vb_dev_info->pages list.
// Each page on this list adds VIRTIO_BALLOON_PAGES_PER_PAGE
// to num_pages above.
//
    pub vb_dev_info: balloon_dev_info,
// Synchronize access/update to this struct virtio_balloon elements
    pub balloon_lock: mutex,
// The array of pfns we tell the Host about.
    pub num_pfns: c_uint,
    pub pfns: [__virtio32; VIRTIO_BALLOON_ARRAY_PFNS_MAX],
// Memory statistics
    pub stats: [virtio_balloon_stat; VIRTIO_BALLOON_S_NR],
// Shrinker to return free pages - VIRTIO_BALLOON_F_FREE_PAGE_HINT
    pub shrinker: *mut shrinker,
// OOM notifier to deflate on OOM - VIRTIO_BALLOON_F_DEFLATE_ON_OOM
    pub oom_nb: notifier_block,
// Free page reporting device
    pub reporting_vq: *mut virtqueue,
    pub pr_dev_info: page_reporting_dev_info,
// State for keeping the wakeup_source active while adjusting the balloon
    pub wakeup_lock: spinlock_t,
    pub processing_wakeup_event: bool,
    pub wakeup_signal_mask: u32,
}

    static const struct virtio_device_id id_table[] = {
    { VIRTIO_ID_BALLOON, VIRTIO_DEV_ANY_ID },
    { 0 },
    };
#[no_mangle]
unsafe extern "C" fn page_to_balloon_pfn(page: *mut page) -> u32 {
    static u32 page_to_balloon_pfn(struct page *page)
    {
    let mut pfn: c_ulong = page_to_pfn(page);
    BUILD_BUG_ON(PAGE_SHIFT < VIRTIO_BALLOON_PFN_SHIFT);
// Convert pfn from Linux page size to balloon page size.
    return pfn * VIRTIO_BALLOON_PAGES_PER_PAGE;
    }
#[no_mangle]
unsafe extern "C" fn start_wakeup_event(vb: *mut virtio_balloon, mask: u32) {
    static void start_wakeup_event(struct virtio_balloon *vb, u32 mask)
    {
    unsigned long flags;
    spin_lock_irqsave(&vb.wakeup_lock, flags);
    vb.wakeup_signal_mask |= mask;
    if (!vb.processing_wakeup_event) {
    vb.processing_wakeup_event = true;
    pm_stay_awake(&vb.vdev.dev);
    }
    spin_unlock_irqrestore(&vb.wakeup_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn process_wakeup_event(vb: *mut virtio_balloon, mask: u32) {
    static void process_wakeup_event(struct virtio_balloon *vb, u32 mask)
    {
    spin_lock_irq(&vb.wakeup_lock);
    vb.wakeup_signal_mask &= ~mask;
    spin_unlock_irq(&vb.wakeup_lock);
    }
#[no_mangle]
unsafe extern "C" fn finish_wakeup_event(vb: *mut virtio_balloon) {
    static void finish_wakeup_event(struct virtio_balloon *vb)
    {
    spin_lock_irq(&vb.wakeup_lock);
    if (!vb.wakeup_signal_mask && vb.processing_wakeup_event) {
    vb.processing_wakeup_event = false;
    pm_relax(&vb.vdev.dev);
    }
    spin_unlock_irq(&vb.wakeup_lock);
    }
#[no_mangle]
unsafe extern "C" fn balloon_ack(vq: *mut virtqueue) {
    static void balloon_ack(struct virtqueue *vq)
    {
    struct virtio_balloon *vb = vq.vdev.priv;
    wake_up(&vb.acked);
    }
#[no_mangle]
unsafe extern "C" fn tell_host(vb: *mut virtio_balloon, vq: *mut virtqueue) {
    static void tell_host(struct virtio_balloon *vb, struct virtqueue *vq)
    {
    struct scatterlist sg;
    unsigned int len;
    int err;
    sg_init_one(&sg, vb.pfns, sizeof(vb.pfns[0]) * vb.num_pfns);
// We should always be able to add one buffer to an empty queue.
    err = virtqueue_add_outbuf(vq, &sg, 1, vb, GFP_KERNEL);
    if (WARN_ON_ONCE(err))
    return;
    virtqueue_kick(vq);
// When host has read buffer, this completes via balloon_ack
    wait_event(vb.acked, virtqueue_get_buf(vq, &len));
    }
    static int virtballoon_free_page_report(struct page_reporting_dev_info *pr_dev_info,
    struct scatterlist *sg, unsigned int nents)
    {
    struct virtio_balloon *vb =
    container_of(pr_dev_info, struct virtio_balloon, pr_dev_info);
    struct virtqueue *vq = vb.reporting_vq;
    unsigned int unused, err;
// We should always be able to add these buffers to an empty queue.
    err = virtqueue_add_inbuf(vq, sg, nents, vb, GFP_NOWAIT);
//
// In the extremely unlikely case that something has occurred and we
// are able to trigger an error we will simply display a warning
// and exit without actually processing the pages.
//
    if (WARN_ON_ONCE(err))
    return err;
    virtqueue_kick(vq);
// When host has read buffer, this completes via balloon_ack
    wait_event(vb.acked, virtqueue_get_buf(vq, &unused));
    return 0;
    }
    static void set_page_pfns(struct virtio_balloon *vb,
    __virtio32 pfns[], struct page *page)
    {
    unsigned int i;
    BUILD_BUG_ON(VIRTIO_BALLOON_PAGES_PER_PAGE > VIRTIO_BALLOON_ARRAY_PFNS_MAX);
//
// Set balloon pfns pointing at this page.
// Note that the first pfn points at start of the page.
//
    for (i = 0; i < VIRTIO_BALLOON_PAGES_PER_PAGE; i++)
    pfns[i] = cpu_to_virtio32(vb.vdev,
    page_to_balloon_pfn(page) + i);
    }
#[no_mangle]
unsafe extern "C" fn fill_balloon(vb: *mut virtio_balloon, num: usize) -> c_uint {
    static unsigned int fill_balloon(struct virtio_balloon *vb, size_t num)
    {
    unsigned int num_allocated_pages;
    struct page *page, *next;
    unsigned int num_pfns;
    LIST_HEAD(pages);
// We can only do one array worth at a time.
    num = min(num, ARRAY_SIZE(vb.pfns));
    for (num_pfns = 0; num_pfns < num;
    num_pfns += VIRTIO_BALLOON_PAGES_PER_PAGE) {
    page = balloon_page_alloc();
    if (!page) {
    dev_info_ratelimited(&vb.vdev.dev,
    "Out of puff! Can't get %u pages\n",
    VIRTIO_BALLOON_PAGES_PER_PAGE);
// Sleep for at least 1/5 of a second before retry.
    msleep(200);
    break;
    }
    list_add(&page.lru, &pages);
    }
    mutex_lock(&vb.balloon_lock);
    vb.num_pfns = 0;
    list_for_each_entry_safe(page, next, &pages, lru) {
    list_del(&page.lru);
    balloon_page_enqueue(&vb.vb_dev_info, page);
    set_page_pfns(vb, vb.pfns + vb.num_pfns, page);
    vb.num_pages += VIRTIO_BALLOON_PAGES_PER_PAGE;
    vb.num_pfns += VIRTIO_BALLOON_PAGES_PER_PAGE;
    }
    num_allocated_pages = vb.num_pfns;
// Did we get any?
    if (vb.num_pfns != 0)
    tell_host(vb, vb.inflate_vq);
    mutex_unlock(&vb.balloon_lock);
    return num_allocated_pages;
    }
    static void release_pages_balloon(struct virtio_balloon *vb,
    struct list_head *pages)
    {
    struct page *page, *next;
    list_for_each_entry_safe(page, next, pages, lru) {
    list_del(&page.lru);
    put_page(page); /* balloon reference */
    }
    }
#[no_mangle]
unsafe extern "C" fn leak_balloon(vb: *mut virtio_balloon, num: usize) -> c_uint {
    static unsigned int leak_balloon(struct virtio_balloon *vb, size_t num)
    {
    unsigned int num_freed_pages;
    struct page *page;
    struct balloon_dev_info *vb_dev_info = &vb.vb_dev_info;
    LIST_HEAD(pages);
// We can only do one array worth at a time.
    num = min(num, ARRAY_SIZE(vb.pfns));
    mutex_lock(&vb.balloon_lock);
// We can't release more pages than taken
    num = min(num, (size_t)vb.num_pages);
    for (vb.num_pfns = 0; vb.num_pfns < num;
    vb.num_pfns += VIRTIO_BALLOON_PAGES_PER_PAGE) {
    page = balloon_page_dequeue(vb_dev_info);
    if (!page)
    break;
    set_page_pfns(vb, vb.pfns + vb.num_pfns, page);
    list_add(&page.lru, &pages);
    vb.num_pages -= VIRTIO_BALLOON_PAGES_PER_PAGE;
    }
    num_freed_pages = vb.num_pfns;
//
// Note that if
// virtio_has_feature(vdev, VIRTIO_BALLOON_F_MUST_TELL_HOST);
// is true, we *have* to do it in this order
//
    if (vb.num_pfns != 0)
    tell_host(vb, vb.deflate_vq);
    release_pages_balloon(vb, &pages);
    mutex_unlock(&vb.balloon_lock);
    return num_freed_pages;
    }
    static inline void update_stat(struct virtio_balloon *vb, int idx,
    u16 tag, u64 val)
    {
    BUG_ON(idx >= VIRTIO_BALLOON_S_NR);
    vb.stats[idx].tag = cpu_to_virtio16(vb.vdev, tag);
    vb.stats[idx].val = cpu_to_virtio64(vb.vdev, val);
    }

// Return the number of entries filled by vm events
#[no_mangle]
pub unsafe extern "C" fn update_balloon_vm_stats(vb: *mut virtio_balloon) -> c_uint {
    static inline unsigned int update_balloon_vm_stats(struct virtio_balloon *vb)
    {
    unsigned long events[NR_VM_EVENT_ITEMS];
    let mut idx: c_uint = 0;
    unsigned int zid;
    let mut stall: c_ulong = 0;
    all_vm_events(events);
    update_stat(vb, idx++, VIRTIO_BALLOON_S_SWAP_IN,
    pages_to_bytes(events[PSWPIN]));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_SWAP_OUT,
    pages_to_bytes(events[PSWPOUT]));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_MAJFLT, events[PGMAJFAULT]);
    update_stat(vb, idx++, VIRTIO_BALLOON_S_MINFLT, events[PGFAULT]);
    update_stat(vb, idx++, VIRTIO_BALLOON_S_OOM_KILL, events[OOM_KILL]);
// sum all the stall events
    for (zid = 0; zid < MAX_NR_ZONES; zid++)
    stall += events[ALLOCSTALL_NORMAL - ZONE_NORMAL + zid];
    update_stat(vb, idx++, VIRTIO_BALLOON_S_ALLOC_STALL, stall);
    update_stat(vb, idx++, VIRTIO_BALLOON_S_ASYNC_SCAN,
    pages_to_bytes(global_node_page_state(PGSCAN_KSWAPD)));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_DIRECT_SCAN,
    pages_to_bytes(global_node_page_state(PGSCAN_DIRECT)));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_ASYNC_RECLAIM,
    pages_to_bytes(global_node_page_state(PGSTEAL_KSWAPD)));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_DIRECT_RECLAIM,
    pages_to_bytes(global_node_page_state(PGSTEAL_DIRECT)));

    update_stat(vb, idx++, VIRTIO_BALLOON_S_HTLB_PGALLOC,
    events[HTLB_BUDDY_PGALLOC]);
    update_stat(vb, idx++, VIRTIO_BALLOON_S_HTLB_PGFAIL,
    events[HTLB_BUDDY_PGALLOC_FAIL]);

    return idx;
    }

#[no_mangle]
pub unsafe extern "C" fn update_balloon_vm_stats(vb: *mut virtio_balloon) -> c_uint {
    static inline unsigned int update_balloon_vm_stats(struct virtio_balloon *vb)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn update_balloon_stats(vb: *mut virtio_balloon) -> c_uint {
    static unsigned int update_balloon_stats(struct virtio_balloon *vb)
    {
    struct sysinfo i;
    unsigned int idx;
    long available;
    unsigned long caches;
    idx = update_balloon_vm_stats(vb);
    si_meminfo(&i);
    available = si_mem_available();
    caches = global_node_page_state(NR_FILE_PAGES);
    update_stat(vb, idx++, VIRTIO_BALLOON_S_MEMFREE,
    pages_to_bytes(i.freeram));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_MEMTOT,
    pages_to_bytes(i.totalram));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_AVAIL,
    pages_to_bytes(available));
    update_stat(vb, idx++, VIRTIO_BALLOON_S_CACHES,
    pages_to_bytes(caches));
    return idx;
    }
//
// While most virtqueues communicate guest-initiated requests to the hypervisor,
// the stats queue operates in reverse.  The driver initializes the virtqueue
// with a single buffer.  From that point forward, all conversations consist of
// a hypervisor request (a call to this function) which directs us to refill
// the virtqueue with a fresh stats buffer.  Since stats collection can sleep,
// we delegate the job to a freezable workqueue that will do the actual work via
// stats_handle_request().
//
#[no_mangle]
unsafe extern "C" fn stats_request(vq: *mut virtqueue) {
    static void stats_request(struct virtqueue *vq)
    {
    struct virtio_balloon *vb = vq.vdev.priv;
    spin_lock(&vb.stop_update_lock);
    if (!vb.stop_update) {
    start_wakeup_event(vb, VIRTIO_BALLOON_WAKEUP_SIGNAL_STATS);
    queue_work(system_freezable_wq, &vb.update_balloon_stats_work);
    }
    spin_unlock(&vb.stop_update_lock);
    }
#[no_mangle]
unsafe extern "C" fn stats_handle_request(vb: *mut virtio_balloon) {
    static void stats_handle_request(struct virtio_balloon *vb)
    {
    struct virtqueue *vq;
    struct scatterlist sg;
    unsigned int len, num_stats;
    int err;
    num_stats = update_balloon_stats(vb);
    vq = vb.stats_vq;
    if (!virtqueue_get_buf(vq, &len))
    return;
    sg_init_one(&sg, vb.stats, sizeof(vb.stats[0]) * num_stats);
    err = virtqueue_add_outbuf(vq, &sg, 1, vb, GFP_KERNEL);
    if (WARN_ON_ONCE(err))
    return;
    virtqueue_kick(vq);
    }
#[no_mangle]
pub unsafe extern "C" fn towards_target(vb: *mut virtio_balloon) -> i64 {
    static inline s64 towards_target(struct virtio_balloon *vb)
    {
    s64 target;
    u32 num_pages;
// Legacy balloon config space is LE, unlike all other devices.
    virtio_cread_le(vb.vdev, struct virtio_balloon_config, num_pages,
    &num_pages);
//
// Aligned up to guest page size to avoid inflating and deflating
// balloon endlessly.
//
    target = ALIGN(num_pages, VIRTIO_BALLOON_PAGES_PER_PAGE);
    return target - vb.num_pages;
    }
// Gives back @num_to_return blocks of free pages to mm.
    static unsigned long return_free_pages_to_mm(struct virtio_balloon *vb,
    unsigned long num_to_return)
    {
    let mut num_returned: c_ulong = 0;
    struct page *page, *next;
    if (unlikely(!num_to_return))
    return 0;
    spin_lock_irq(&vb.free_page_list_lock);
    list_for_each_entry_safe(page, next, &vb.free_page_list, lru) {
    list_del(&page.lru);
    __free_pages(page, VIRTIO_BALLOON_HINT_BLOCK_ORDER);
    if (++num_returned == num_to_return)
    break;
    }
    vb.num_free_page_blocks -= num_returned;
    spin_unlock_irq(&vb.free_page_list_lock);
    return num_returned;
    }
#[no_mangle]
unsafe extern "C" fn virtio_balloon_queue_free_page_work(vb: *mut virtio_balloon) {
    static void virtio_balloon_queue_free_page_work(struct virtio_balloon *vb)
    {
    if (!virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    return;
// No need to queue the work if the bit was already set.
    if (test_and_set_bit(VIRTIO_BALLOON_CONFIG_READ_CMD_ID,
    &vb.config_read_bitmap))
    return;
    queue_work(vb.balloon_wq, &vb.report_free_page_work);
    }
#[no_mangle]
unsafe extern "C" fn start_update_balloon_size(vb: *mut virtio_balloon) {
    static void start_update_balloon_size(struct virtio_balloon *vb)
    {
    start_wakeup_event(vb, VIRTIO_BALLOON_WAKEUP_SIGNAL_ADJUST);
    queue_work(system_freezable_wq, &vb.update_balloon_size_work);
    }
#[no_mangle]
unsafe extern "C" fn virtballoon_changed(vdev: *mut virtio_device) {
    static void virtballoon_changed(struct virtio_device *vdev)
    {
    struct virtio_balloon *vb = vdev.priv;
    unsigned long flags;
    spin_lock_irqsave(&vb.stop_update_lock, flags);
    if (!vb.stop_update) {
    start_update_balloon_size(vb);
    virtio_balloon_queue_free_page_work(vb);
    }
    spin_unlock_irqrestore(&vb.stop_update_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn update_balloon_size(vb: *mut virtio_balloon) {
    static void update_balloon_size(struct virtio_balloon *vb)
    {
    let mut actual: u32 = vb.num_pages;
// Legacy balloon config space is LE, unlike all other devices.
    virtio_cwrite_le(vb.vdev, struct virtio_balloon_config, actual,
    &actual);
    }
#[no_mangle]
unsafe extern "C" fn update_balloon_stats_func(work: *mut work_struct) {
    static void update_balloon_stats_func(struct work_struct *work)
    {
    struct virtio_balloon *vb;
    vb = container_of(work, struct virtio_balloon,
    update_balloon_stats_work);
    process_wakeup_event(vb, VIRTIO_BALLOON_WAKEUP_SIGNAL_STATS);
    stats_handle_request(vb);
    finish_wakeup_event(vb);
    }
#[no_mangle]
unsafe extern "C" fn update_balloon_size_func(work: *mut work_struct) {
    static void update_balloon_size_func(struct work_struct *work)
    {
    struct virtio_balloon *vb;
    s64 diff;
    vb = container_of(work, struct virtio_balloon,
    update_balloon_size_work);
    process_wakeup_event(vb, VIRTIO_BALLOON_WAKEUP_SIGNAL_ADJUST);
    diff = towards_target(vb);
    if (diff) {
    if (diff > 0)
    diff -= fill_balloon(vb, diff);
    else
    diff += leak_balloon(vb, -diff);
    update_balloon_size(vb);
    }
    if (diff)
    queue_work(system_freezable_wq, work);
    else
    finish_wakeup_event(vb);
    }
#[no_mangle]
unsafe extern "C" fn init_vqs(vb: *mut virtio_balloon) -> c_int {
    static int init_vqs(struct virtio_balloon *vb)
    {
    struct virtqueue_info vqs_info[VIRTIO_BALLOON_VQ_MAX] = {};
    struct virtqueue *vqs[VIRTIO_BALLOON_VQ_MAX];
    int err;
//
// Inflateq and deflateq are used unconditionally. The names[]
// will be NULL if the related feature is not enabled, which will
// cause no allocation for the corresponding virtqueue in find_vqs.
//
    vqs_info[VIRTIO_BALLOON_VQ_INFLATE].callback = balloon_ack;
    vqs_info[VIRTIO_BALLOON_VQ_INFLATE].name = "inflate";
    vqs_info[VIRTIO_BALLOON_VQ_DEFLATE].callback = balloon_ack;
    vqs_info[VIRTIO_BALLOON_VQ_DEFLATE].name = "deflate";
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_STATS_VQ)) {
    vqs_info[VIRTIO_BALLOON_VQ_STATS].name = "stats";
    vqs_info[VIRTIO_BALLOON_VQ_STATS].callback = stats_request;
    }
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    vqs_info[VIRTIO_BALLOON_VQ_FREE_PAGE].name = "free_page_vq";
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_REPORTING)) {
    vqs_info[VIRTIO_BALLOON_VQ_REPORTING].name = "reporting_vq";
    vqs_info[VIRTIO_BALLOON_VQ_REPORTING].callback = balloon_ack;
    }
    err = virtio_find_vqs(vb.vdev, VIRTIO_BALLOON_VQ_MAX, vqs,
    vqs_info, core::ptr::null_mut());
    if (err)
    return err;
    vb.inflate_vq = vqs[VIRTIO_BALLOON_VQ_INFLATE];
    vb.deflate_vq = vqs[VIRTIO_BALLOON_VQ_DEFLATE];
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_STATS_VQ)) {
    struct scatterlist sg;
    unsigned int num_stats;
    vb.stats_vq = vqs[VIRTIO_BALLOON_VQ_STATS];
//
// Prime this virtqueue with one buffer so the hypervisor can
// use it to signal us later (it can't be broken yet!).
//
    num_stats = update_balloon_stats(vb);
    sg_init_one(&sg, vb.stats, sizeof(vb.stats[0]) * num_stats);
    err = virtqueue_add_outbuf(vb.stats_vq, &sg, 1, vb,
    GFP_KERNEL);
    if (err) {
    dev_warn(&vb.vdev.dev, "%s: add stat_vq failed\n",
    __func__);
    return err;
    }
    virtqueue_kick(vb.stats_vq);
    }
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    vb.free_page_vq = vqs[VIRTIO_BALLOON_VQ_FREE_PAGE];
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_REPORTING))
    vb.reporting_vq = vqs[VIRTIO_BALLOON_VQ_REPORTING];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtio_balloon_cmd_id_received(vb: *mut virtio_balloon) -> u32 {
    static u32 virtio_balloon_cmd_id_received(struct virtio_balloon *vb)
    {
    if (test_and_clear_bit(VIRTIO_BALLOON_CONFIG_READ_CMD_ID,
    &vb.config_read_bitmap)) {
// Legacy balloon config space is LE, unlike all other devices.
    virtio_cread_le(vb.vdev, struct virtio_balloon_config,
    free_page_hint_cmd_id,
    &vb.cmd_id_received_cache);
    }
    return vb.cmd_id_received_cache;
    }
#[no_mangle]
unsafe extern "C" fn send_cmd_id_start(vb: *mut virtio_balloon) -> c_int {
    static int send_cmd_id_start(struct virtio_balloon *vb)
    {
    struct scatterlist sg;
    struct virtqueue *vq = vb.free_page_vq;
    int err, unused;
// Detach all the used buffers from the vq
    while (virtqueue_get_buf(vq, &unused))
    ;
    vb.cmd_id_active = cpu_to_virtio32(vb.vdev,
    virtio_balloon_cmd_id_received(vb));
    sg_init_one(&sg, &vb.cmd_id_active, sizeof(vb.cmd_id_active));
    err = virtqueue_add_outbuf(vq, &sg, 1, &vb.cmd_id_active, GFP_KERNEL);
    if (!err)
    virtqueue_kick(vq);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn send_cmd_id_stop(vb: *mut virtio_balloon) -> c_int {
    static int send_cmd_id_stop(struct virtio_balloon *vb)
    {
    struct scatterlist sg;
    struct virtqueue *vq = vb.free_page_vq;
    int err, unused;
// Detach all the used buffers from the vq
    while (virtqueue_get_buf(vq, &unused))
    ;
    sg_init_one(&sg, &vb.cmd_id_stop, sizeof(vb.cmd_id_stop));
    err = virtqueue_add_outbuf(vq, &sg, 1, &vb.cmd_id_stop, GFP_KERNEL);
    if (!err)
    virtqueue_kick(vq);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn get_free_page_and_send(vb: *mut virtio_balloon) -> c_int {
    static int get_free_page_and_send(struct virtio_balloon *vb)
    {
    struct virtqueue *vq = vb.free_page_vq;
    struct page *page;
    struct scatterlist sg;
    int err, unused;
    void *p;
// Detach all the used buffers from the vq
    while (virtqueue_get_buf(vq, &unused))
    ;
    page = alloc_pages(VIRTIO_BALLOON_FREE_PAGE_ALLOC_FLAG,
    VIRTIO_BALLOON_HINT_BLOCK_ORDER);
//
// When the allocation returns NULL, it indicates that we have got all
// the possible free pages, so return -EINTR to stop.
//
    if (!page)
    return -EINTR;
    p = page_address(page);
    sg_init_one(&sg, p, VIRTIO_BALLOON_HINT_BLOCK_BYTES);
// There is always 1 entry reserved for the cmd id to use.
    if (vq.num_free > 1) {
    err = virtqueue_add_inbuf(vq, &sg, 1, p, GFP_KERNEL);
    if (unlikely(err)) {
    __free_pages(page, VIRTIO_BALLOON_HINT_BLOCK_ORDER);
    return err;
    }
    virtqueue_kick(vq);
    spin_lock_irq(&vb.free_page_list_lock);
    list_add(&page.lru, &vb.free_page_list);
    vb.num_free_page_blocks++;
    spin_unlock_irq(&vb.free_page_list_lock);
    } else {
//
// The vq has no available entry to add this page block, so
// just free it.
//
    __free_pages(page, VIRTIO_BALLOON_HINT_BLOCK_ORDER);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn send_free_pages(vb: *mut virtio_balloon) -> c_int {
    static int send_free_pages(struct virtio_balloon *vb)
    {
    int err;
    u32 cmd_id_active;
    while (1) {
//
// If a stop id or a new cmd id was just received from host,
// stop the reporting.
//
    cmd_id_active = virtio32_to_cpu(vb.vdev, vb.cmd_id_active);
    if (unlikely(cmd_id_active !=
    virtio_balloon_cmd_id_received(vb)))
    break;
//
// The free page blocks are allocated and sent to host one by
// one.
//
    err = get_free_page_and_send(vb);
    if (err == -EINTR)
    break;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: unlikely(err)) -> else {
    else if (unlikely(err))
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtio_balloon_report_free_page(vb: *mut virtio_balloon) {
    static void virtio_balloon_report_free_page(struct virtio_balloon *vb)
    {
    int err;
    struct device *dev = &vb.vdev.dev;
// Start by sending the received cmd id to host with an outbuf.
    err = send_cmd_id_start(vb);
    if (unlikely(err))
    dev_err(dev, "Failed to send a start id, err = %d\n", err);
    err = send_free_pages(vb);
    if (unlikely(err))
    dev_err(dev, "Failed to send a free page, err = %d\n", err);
// End by sending a stop id to host with an outbuf.
    err = send_cmd_id_stop(vb);
    if (unlikely(err))
    dev_err(dev, "Failed to send a stop id, err = %d\n", err);
    }
#[no_mangle]
unsafe extern "C" fn report_free_page_func(work: *mut work_struct) {
    static void report_free_page_func(struct work_struct *work)
    {
    struct virtio_balloon *vb = container_of(work, struct virtio_balloon,
    report_free_page_work);
    u32 cmd_id_received;
    cmd_id_received = virtio_balloon_cmd_id_received(vb);
    if (cmd_id_received == VIRTIO_BALLOON_CMD_ID_DONE) {
// Pass ULONG_MAX to give back all the free pages
    return_free_pages_to_mm(vb, ULONG_MAX);
    } else if (cmd_id_received != VIRTIO_BALLOON_CMD_ID_STOP &&
    cmd_id_received !=
    virtio32_to_cpu(vb.vdev, vb.cmd_id_active)) {
    virtio_balloon_report_free_page(vb);
    }
    }

//
// virtballoon_migratepage - perform the balloon page migration on behalf of
// a compaction thread.     (called under page lock)
// @vb_dev_info: the balloon device
// @newpage: page that will replace the isolated page after migration finishes.
// @page   : the isolated (old) page that is about to be migrated to newpage.
// @mode   : compaction mode -- not used for balloon page migration.
//
// After a ballooned page gets isolated by compaction procedures, this is the
// function that performs the page migration on behalf of a compaction thread
// The page migration for virtio balloon is done in a simple swap fashion which
// follows these two macro steps:
// 1) insert newpage into vb->pages list and update the host about it;
// 2) update the host about the old page removed from vb->pages list;
//
// This function preforms the balloon page migration task.
// Called through movable_operations->migrate_page
//
    static int virtballoon_migratepage(struct balloon_dev_info *vb_dev_info,
    struct page *newpage, struct page *page, enum migrate_mode mode)
    {
    struct virtio_balloon *vb = container_of(vb_dev_info,
    struct virtio_balloon, vb_dev_info);
//
// In order to avoid lock contention while migrating pages concurrently
// to leak_balloon() or fill_balloon() we just give up the balloon_lock
// this turn, as it is easier to retry the page migration later.
// This also prevents fill_balloon() getting stuck into a mutex
// recursion in the case it ends up triggering memory compaction
// while it is attempting to inflate the ballon.
//
    if (!mutex_trylock(&vb.balloon_lock))
    return -EAGAIN;
// balloon's page migration 1st step  -- inflate "newpage"
    vb.num_pfns = VIRTIO_BALLOON_PAGES_PER_PAGE;
    set_page_pfns(vb, vb.pfns, newpage);
    tell_host(vb, vb.inflate_vq);
// balloon's page migration 2nd step -- deflate "page"
    vb.num_pfns = VIRTIO_BALLOON_PAGES_PER_PAGE;
    set_page_pfns(vb, vb.pfns, page);
    tell_host(vb, vb.deflate_vq);
    mutex_unlock(&vb.balloon_lock);
    return 0;
    }

    static unsigned long shrink_free_pages(struct virtio_balloon *vb,
    unsigned long pages_to_free)
    {
    unsigned long blocks_to_free, blocks_freed;
    pages_to_free = round_up(pages_to_free,
    VIRTIO_BALLOON_HINT_BLOCK_PAGES);
    blocks_to_free = pages_to_free / VIRTIO_BALLOON_HINT_BLOCK_PAGES;
    blocks_freed = return_free_pages_to_mm(vb, blocks_to_free);
    return blocks_freed * VIRTIO_BALLOON_HINT_BLOCK_PAGES;
    }
    static unsigned long virtio_balloon_shrinker_scan(struct shrinker *shrinker,
    struct shrink_control *sc)
    {
    struct virtio_balloon *vb = shrinker.private_data;
    return shrink_free_pages(vb, sc.nr_to_scan);
    }
    static unsigned long virtio_balloon_shrinker_count(struct shrinker *shrinker,
    struct shrink_control *sc)
    {
    struct virtio_balloon *vb = shrinker.private_data;
    return vb.num_free_page_blocks * VIRTIO_BALLOON_HINT_BLOCK_PAGES;
    }
    static int virtio_balloon_oom_notify(struct notifier_block *nb,
    unsigned long dummy, void *parm)
    {
    struct virtio_balloon *vb = container_of(nb,
    struct virtio_balloon, oom_nb);
    unsigned long *freed = parm;
// freed += leak_balloon(vb, VIRTIO_BALLOON_OOM_NR_PAGES)
    VIRTIO_BALLOON_PAGES_PER_PAGE;
    update_balloon_size(vb);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn virtio_balloon_unregister_shrinker(vb: *mut virtio_balloon) {
    static void virtio_balloon_unregister_shrinker(struct virtio_balloon *vb)
    {
    shrinker_free(vb.shrinker);
    }
#[no_mangle]
unsafe extern "C" fn virtio_balloon_register_shrinker(vb: *mut virtio_balloon) -> c_int {
    static int virtio_balloon_register_shrinker(struct virtio_balloon *vb)
    {
    vb.shrinker = shrinker_alloc(0, "virtio-balloon");
    if (!vb.shrinker)
    return -ENOMEM;
    vb.shrinker.scan_objects = virtio_balloon_shrinker_scan;
    vb.shrinker.count_objects = virtio_balloon_shrinker_count;
    vb.shrinker.private_data = vb;
    shrinker_register(vb.shrinker);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtballoon_probe(vdev: *mut virtio_device) -> c_int {
    static int virtballoon_probe(struct virtio_device *vdev)
    {
    struct virtio_balloon *vb;
    int err;
    if (!vdev.config.get) {
    dev_err(&vdev.dev, "%s failure: config access disabled\n",
    __func__);
    return -EINVAL;
    }
    vdev.priv = vb = kzalloc_obj(*vb);
    if (!vb) {
    err = -ENOMEM;
    goto out;
    }
    INIT_WORK(&vb.update_balloon_stats_work, update_balloon_stats_func);
    INIT_WORK(&vb.update_balloon_size_work, update_balloon_size_func);
    spin_lock_init(&vb.stop_update_lock);
    mutex_init(&vb.balloon_lock);
    init_waitqueue_head(&vb.acked);
    vb.vdev = vdev;
    balloon_devinfo_init(&vb.vb_dev_info);
    err = init_vqs(vb);
    if (err)
    goto out_free_vb;
    if (!virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_DEFLATE_ON_OOM))
    vb.vb_dev_info.adjust_managed_page_count = true;

    vb.vb_dev_info.migratepage = virtballoon_migratepage;

    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT)) {
//
// There is always one entry reserved for cmd id, so the ring
// size needs to be at least two to report free page hints.
//
    if (virtqueue_get_vring_size(vb.free_page_vq) < 2) {
    err = -ENOSPC;
    goto out_del_vqs;
    }
    vb.balloon_wq = alloc_workqueue("balloon-wq",
    WQ_FREEZABLE | WQ_CPU_INTENSIVE | WQ_PERCPU,
    0);
    if (!vb.balloon_wq) {
    err = -ENOMEM;
    goto out_del_vqs;
    }
    INIT_WORK(&vb.report_free_page_work, report_free_page_func);
    vb.cmd_id_received_cache = VIRTIO_BALLOON_CMD_ID_STOP;
    vb.cmd_id_active = cpu_to_virtio32(vb.vdev,
    VIRTIO_BALLOON_CMD_ID_STOP);
    vb.cmd_id_stop = cpu_to_virtio32(vb.vdev,
    VIRTIO_BALLOON_CMD_ID_STOP);
    spin_lock_init(&vb.free_page_list_lock);
    INIT_LIST_HEAD(&vb.free_page_list);
//
// We're allowed to reuse any free pages, even if they are
// still to be processed by the host.
//
    err = virtio_balloon_register_shrinker(vb);
    if (err)
    goto out_del_balloon_wq;
    }
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_DEFLATE_ON_OOM)) {
    vb.oom_nb.notifier_call = virtio_balloon_oom_notify;
    vb.oom_nb.priority = VIRTIO_BALLOON_OOM_NOTIFY_PRIORITY;
    err = register_oom_notifier(&vb.oom_nb);
    if (err < 0)
    goto out_unregister_shrinker;
    }
    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_PAGE_POISON)) {
// Start with poison val of 0 representing general init
    let mut poison_val: __u32 = 0;
//
// Let the hypervisor know that we are expecting a
// specific value to be written back in balloon pages.
//
// If the PAGE_POISON value was larger than a byte we would
// need to byte swap poison_val here to guarantee it is
// little-endian. However for now it is a single byte so we
// can pass it as-is.
//
    if (!want_init_on_free())
    memset(&poison_val, PAGE_POISON, sizeof(poison_val));
    virtio_cwrite_le(vb.vdev, struct virtio_balloon_config,
    poison_val, &poison_val);
    }
    vb.pr_dev_info.report = virtballoon_free_page_report;
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_REPORTING)) {
    unsigned int capacity;
    capacity = virtqueue_get_vring_size(vb.reporting_vq);
    vb.pr_dev_info.order = PAGE_REPORTING_ORDER_UNSPECIFIED;
//
// The default page reporting order is @pageblock_order, which
// corresponds to 512MB in size on ARM64 when 64KB base page
// size is used. The page reporting won't be triggered if the
// freeing page can't come up with a free area like that huge.
// So we specify the page reporting order to 5, corresponding
// to 2MB. It helps to avoid THP splitting if 4KB base page
// size is used by host.
//
// Ideally, the page reporting order is selected based on the
// host's base page size. However, it needs more work to report
// that value. The hard-coded order would be fine currently.
//

    vb.pr_dev_info.order = 5;

    vb.pr_dev_info.capacity = capacity;
    err = page_reporting_register(&vb.pr_dev_info);
    if (err)
    goto out_unregister_oom;
    }
    spin_lock_init(&vb.wakeup_lock);
//
// The virtio balloon itself can't wake up the device, but it is
// responsible for processing wakeup events passed up from the transport
// layer. Wakeup sources don't support nesting/chaining calls, so we use
// our own wakeup source to ensure wakeup events are properly handled
// without trampling on the transport layer's wakeup source.
//
    device_set_wakeup_capable(&vb.vdev.dev, true);
    virtio_device_ready(vdev);
    if (towards_target(vb))
    virtballoon_changed(vdev);
    return 0;
    out_unregister_oom:
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_DEFLATE_ON_OOM))
    unregister_oom_notifier(&vb.oom_nb);
    out_unregister_shrinker:
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    virtio_balloon_unregister_shrinker(vb);
    out_del_balloon_wq:
    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    destroy_workqueue(vb.balloon_wq);
    out_del_vqs:
    vdev.config.del_vqs(vdev);
    out_free_vb:
    mutex_destroy(&vb.balloon_lock);
    kfree(vb);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn remove_common(vb: *mut virtio_balloon) {
    static void remove_common(struct virtio_balloon *vb)
    {
// There might be pages left in the balloon: free them.
    while (vb.num_pages)
    leak_balloon(vb, vb.num_pages);
    update_balloon_size(vb);
// There might be free pages that are being reported: release them.
    if (virtio_has_feature(vb.vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    return_free_pages_to_mm(vb, ULONG_MAX);
// Now we reset the device so we can clean up the queues.
    virtio_reset_device(vb.vdev);
    vb.vdev.config.del_vqs(vb.vdev);
    }
//
// Stop all asynchronous balloon work. The device must still be alive so that
// in-flight requests can drain via the host before it is reset or freed.
//
#[no_mangle]
unsafe extern "C" fn virtballoon_quiesce(vb: *mut virtio_balloon) {
    static void virtballoon_quiesce(struct virtio_balloon *vb)
    {
    struct virtio_device *vdev = vb.vdev;
    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_REPORTING))
    page_reporting_unregister(&vb.pr_dev_info);
    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_DEFLATE_ON_OOM))
    unregister_oom_notifier(&vb.oom_nb);
    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    virtio_balloon_unregister_shrinker(vb);
    spin_lock_irq(&vb.stop_update_lock);
    vb.stop_update = true;
    spin_unlock_irq(&vb.stop_update_lock);
    cancel_work_sync(&vb.update_balloon_size_work);
    cancel_work_sync(&vb.update_balloon_stats_work);
    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    cancel_work_sync(&vb.report_free_page_work);
    }
#[no_mangle]
unsafe extern "C" fn virtballoon_remove(vdev: *mut virtio_device) {
    static void virtballoon_remove(struct virtio_device *vdev)
    {
    struct virtio_balloon *vb = vdev.priv;
    virtballoon_quiesce(vb);
    if (virtio_has_feature(vdev, VIRTIO_BALLOON_F_FREE_PAGE_HINT))
    destroy_workqueue(vb.balloon_wq);
    remove_common(vb);
    mutex_destroy(&vb.balloon_lock);
    kfree(vb);
    }
#[no_mangle]
unsafe extern "C" fn virtballoon_shutdown(vdev: *mut virtio_device) {
    static void virtballoon_shutdown(struct virtio_device *vdev)
    {
    virtballoon_quiesce(vdev.priv);
    virtio_device_shutdown(vdev);
    }

#[no_mangle]
unsafe extern "C" fn virtballoon_freeze(vdev: *mut virtio_device) -> c_int {
    static int virtballoon_freeze(struct virtio_device *vdev)
    {
    struct virtio_balloon *vb = vdev.priv;
//
// The workqueue is already frozen by the PM core before this
// function is called.
//
    remove_common(vb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtballoon_restore(vdev: *mut virtio_device) -> c_int {
    static int virtballoon_restore(struct virtio_device *vdev)
    {
    struct virtio_balloon *vb = vdev.priv;
    int ret;
    ret = init_vqs(vdev.priv);
    if (ret)
    return ret;
    virtio_device_ready(vdev);
    if (towards_target(vb))
    virtballoon_changed(vdev);
    update_balloon_size(vb);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn virtballoon_validate(vdev: *mut virtio_device) -> c_int {
    static int virtballoon_validate(struct virtio_device *vdev)
    {
//
// Inform the hypervisor that our pages are poisoned or
// initialized. If we cannot do that then we should disable
// page reporting as it could potentially change the contents
// of our free pages.
//
    if (!want_init_on_free() && !page_poisoning_enabled_static())
    __virtio_clear_bit(vdev, VIRTIO_BALLOON_F_PAGE_POISON);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !virtio_has_feature(vdev, _arg: VIRTIO_BALLOON_F_PAGE_POISON)) -> else {
    else if (!virtio_has_feature(vdev, VIRTIO_BALLOON_F_PAGE_POISON))
    __virtio_clear_bit(vdev, VIRTIO_BALLOON_F_REPORTING);
//
// Disable indirect descriptors to avoid memory allocation in
// virtqueue_add during page reporting.
//
    __virtio_clear_bit(vdev, VIRTIO_RING_F_INDIRECT_DESC);
    __virtio_clear_bit(vdev, VIRTIO_F_ACCESS_PLATFORM);
    return 0;
    }
    static unsigned int features[] = {
    VIRTIO_BALLOON_F_MUST_TELL_HOST,
    VIRTIO_BALLOON_F_STATS_VQ,
    VIRTIO_BALLOON_F_DEFLATE_ON_OOM,
    VIRTIO_BALLOON_F_FREE_PAGE_HINT,
    VIRTIO_BALLOON_F_PAGE_POISON,
    VIRTIO_BALLOON_F_REPORTING,
    };
    static struct virtio_driver virtio_balloon_driver = {
    .feature_table = features,
    .feature_table_size = ARRAY_SIZE(features),
    .driver.name =	KBUILD_MODNAME,
    .id_table =	id_table,
    .validate =	virtballoon_validate,
    .probe =	virtballoon_probe,
    .remove =	virtballoon_remove,
    .shutdown =	virtballoon_shutdown,
    .config_changed = virtballoon_changed,

    .freeze	=	virtballoon_freeze,
    .restore =	virtballoon_restore,

    };
    module_virtio_driver(virtio_balloon_driver);
    MODULE_DEVICE_TABLE(virtio, id_table);
    MODULE_DESCRIPTION("Virtio balloon driver");
    MODULE_LICENSE("GPL");
