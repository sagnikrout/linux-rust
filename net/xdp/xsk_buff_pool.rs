//! Automatically rewritten from C to Rust
//! Source: net/xdp/xsk_buff_pool.c
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

#[no_mangle]
pub unsafe extern "C" fn xp_add_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock) {
    void xp_add_xsk(struct xsk_buff_pool *pool, struct xdp_sock *xs)
    {
    if (!xs.tx)
    return;
    spin_lock(&pool.xsk_tx_list_lock);
    list_add_rcu(&xs.tx_list, &pool.xsk_tx_list);
    spin_unlock(&pool.xsk_tx_list_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn xp_del_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock) {
    void xp_del_xsk(struct xsk_buff_pool *pool, struct xdp_sock *xs)
    {
    if (!xs.tx)
    return;
    spin_lock(&pool.xsk_tx_list_lock);
    list_del_rcu(&xs.tx_list);
    spin_unlock(&pool.xsk_tx_list_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn xp_destroy(pool: *mut xsk_buff_pool) {
    void xp_destroy(struct xsk_buff_pool *pool)
    {
    if (!pool)
    return;
    kvfree(pool.tx_descs);
    kvfree(pool.heads);
    kvfree(pool);
    }
    int xp_alloc_tx_descs(struct xsk_buff_pool *pool, struct xdp_sock *xs,
    u32 max_segs)
    {
    let mut nentries: u32 = max(xs.tx.nentries, max_segs);
    pool.tx_descs = kvzalloc_objs(*pool.tx_descs, nentries);
    if (!pool.tx_descs)
    return -ENOMEM;
    pool.tx_descs_nentries = nentries;
    return 0;
    }
    struct xsk_buff_pool *xp_create_and_assign_umem(struct xdp_sock *xs,
    struct xdp_umem *umem,
    u32 max_segs)
    {
    let mut unaligned: bool = umem.flags & XDP_UMEM_UNALIGNED_CHUNK_FLAG;
    struct xsk_buff_pool *pool;
    struct xdp_buff_xsk *xskb;
    u32 i, entries;
    entries = unaligned ? umem.chunks : 0;
    pool = kvzalloc_flex(*pool, free_heads, entries);
    if (!pool)
    goto out;
    pool.heads = kvzalloc_objs(*pool.heads, umem.chunks);
    if (!pool.heads)
    goto out;
    if (xs.tx)
    if (xp_alloc_tx_descs(pool, xs, max_segs))
    goto out;
    pool.chunk_mask = ~((u64)umem.chunk_size - 1);
    pool.addrs_cnt = umem.size;
    pool.heads_cnt = umem.chunks;
    pool.free_heads_cnt = umem.chunks;
    pool.headroom = umem.headroom;
    pool.chunk_size = umem.chunk_size;
    pool.chunk_shift = ffs(umem.chunk_size) - 1;
    pool.unaligned = unaligned;
    pool.frame_len = umem.chunk_size - umem.headroom -
    XDP_PACKET_HEADROOM;
    pool.umem = umem;
    pool.addrs = umem.addrs;
    pool.tx_metadata_len = umem.tx_metadata_len;
    pool.tx_sw_csum = umem.flags & XDP_UMEM_TX_SW_CSUM;
    spin_lock_init(&pool.rx_lock);
    INIT_LIST_HEAD(&pool.free_list);
    INIT_LIST_HEAD(&pool.xskb_list);
    INIT_LIST_HEAD(&pool.xsk_tx_list);
    spin_lock_init(&pool.xsk_tx_list_lock);
    spin_lock_init(&pool.cq_prod_lock);
    spin_lock_init(&xs.cq_tmp.cq_cached_prod_lock);
    refcount_set(&pool.users, 1);
    pool.fq = xs.fq_tmp;
    pool.cq = xs.cq_tmp;
    for (i = 0; i < pool.free_heads_cnt; i++) {
    xskb = &pool.heads[i];
    xskb.pool = pool;
    xskb.xdp.frame_sz = umem.chunk_size - umem.headroom;
    INIT_LIST_HEAD(&xskb.list_node);
    if (pool.unaligned)
    pool.free_heads[i] = xskb;
    else
    xp_init_xskb_addr(xskb, pool, (u64)i * pool.chunk_size);
    }
    return pool;
    out:
    xp_destroy(pool);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn xp_set_rxq_info(pool: *mut xsk_buff_pool, rxq: *mut xdp_rxq_info) {
    void xp_set_rxq_info(struct xsk_buff_pool *pool, struct xdp_rxq_info *rxq)
    {
    u32 i;
    for (i = 0; i < pool.heads_cnt; i++)
    pool.heads[i].xdp.rxq = rxq;
    }
    EXPORT_SYMBOL(xp_set_rxq_info);
#[no_mangle]
pub unsafe extern "C" fn xp_fill_cb(pool: *mut xsk_buff_pool, desc: *mut xsk_cb_desc) {
    void xp_fill_cb(struct xsk_buff_pool *pool, struct xsk_cb_desc *desc)
    {
    u32 i;
    for (i = 0; i < pool.heads_cnt; i++) {
    struct xdp_buff_xsk *xskb = &pool.heads[i];
    memcpy(xskb.cb + desc.off, desc.src, desc.bytes);
    }
    }
    EXPORT_SYMBOL(xp_fill_cb);
#[no_mangle]
unsafe extern "C" fn xp_disable_drv_zc(pool: *mut xsk_buff_pool) {
    static void xp_disable_drv_zc(struct xsk_buff_pool *pool)
    {
    struct netdev_bpf bpf;
    int err;
    ASSERT_RTNL();
    if (pool.umem.zc) {
    bpf.command = XDP_SETUP_XSK_POOL;
    bpf.xsk.pool = core::ptr::null_mut();
    bpf.xsk.queue_id = pool.queue_id;
    err = pool.netdev.netdev_ops.ndo_bpf(pool.netdev, &bpf);
    if (err)
    WARN(1, "Failed to disable zero-copy!\n");
    }
    }
    int xp_assign_dev(struct xsk_buff_pool *pool,
    struct net_device *netdev, u16 queue_id, u16 flags)
    {
    let mut needed: u32 = netdev.mtu + ETH_PAD_LEN;
    let mut segs: u32 = netdev.xdp_zc_max_segs;
    let mut mbuf: bool = flags & XDP_USE_SG;
    bool force_zc, force_copy;
    struct netdev_bpf bpf;
    u32 frame_size;
    let mut err: c_int = 0;
    ASSERT_RTNL();
    force_zc = flags & XDP_ZEROCOPY;
    force_copy = flags & XDP_COPY;
    if (force_zc && force_copy)
    return -EINVAL;
    if (pool.tx_sw_csum && (netdev.priv_flags & IFF_TX_SKB_NO_LINEAR))
    return -EOPNOTSUPP;
    if (xsk_get_pool_from_qid(netdev, queue_id))
    return -EBUSY;
    pool.netdev = netdev;
    pool.queue_id = queue_id;
    err = xsk_reg_pool_at_qid(netdev, pool, queue_id);
    if (err)
    return err;
    if (mbuf)
    pool.umem.flags |= XDP_UMEM_SG_FLAG;
    if (flags & XDP_USE_NEED_WAKEUP)
    pool.uses_need_wakeup = true;
// Tx needs to be explicitly woken up the first time.  Also
// for supporting drivers that do not implement this
// feature. They will always have to call sendto() or poll().
//
    pool.cached_need_wakeup = XDP_WAKEUP_TX;
    dev_hold(netdev);
    if (force_copy)
// For copy-mode, we are done.
    return 0;
    if ((netdev.xdp_features & NETDEV_XDP_ACT_XSK) != NETDEV_XDP_ACT_XSK) {
    err = -EOPNOTSUPP;
    goto err_unreg_pool;
    }
    if (mbuf) {
    if (segs == 1) {
    err = -EOPNOTSUPP;
    goto err_unreg_pool;
    }
    } else {
    segs = 1;
    }
// open-code xsk_pool_get_rx_frame_size() as pool->dev is not
// set yet at this point; we are before getting down to driver
//
    frame_size = __xsk_pool_get_rx_frame_size(pool) -
    xsk_pool_get_tailroom(mbuf);
    frame_size = ALIGN_DOWN(frame_size, 128);
    if (needed > frame_size * segs) {
    err = -EINVAL;
    goto err_unreg_pool;
    }
    if (dev_get_min_mp_channel_count(netdev)) {
    err = -EBUSY;
    goto err_unreg_pool;
    }
    bpf.command = XDP_SETUP_XSK_POOL;
    bpf.xsk.pool = pool;
    bpf.xsk.queue_id = queue_id;
    netdev_assert_locked_ops_compat(netdev);
    err = netdev.netdev_ops.ndo_bpf(netdev, &bpf);
    if (err)
    goto err_unreg_pool;
    if (!pool.dma_pages) {
    WARN(1, "Driver did not DMA map zero-copy buffers");
    err = -EINVAL;
    goto err_unreg_xsk;
    }
    pool.umem.zc = true;
    pool.xdp_zc_max_segs = netdev.xdp_zc_max_segs;
    return 0;
    err_unreg_xsk:
    xp_disable_drv_zc(pool);
    err_unreg_pool:
    if (!force_zc)
    err = 0; /* fallback to copy mode */
    if (err) {
    xsk_clear_pool_at_qid(netdev, queue_id);
    dev_put(netdev);
    }
    return err;
    }
    int xp_assign_dev_shared(struct xsk_buff_pool *pool, struct xdp_sock *umem_xs,
    struct net_device *dev, u16 queue_id)
    {
    u16 flags;
    struct xdp_umem *umem = umem_xs.umem;
    flags = umem.zc ? XDP_ZEROCOPY : XDP_COPY;
    if (umem.flags & XDP_UMEM_SG_FLAG)
    flags |= XDP_USE_SG;
    if (umem_xs.pool.uses_need_wakeup)
    flags |= XDP_USE_NEED_WAKEUP;
    return xp_assign_dev(pool, dev, queue_id, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn xp_clear_dev(pool: *mut xsk_buff_pool) {
    void xp_clear_dev(struct xsk_buff_pool *pool)
    {
    struct net_device *netdev = pool.netdev;
    if (!pool.netdev)
    return;
    netdev_lock_ops(netdev);
    xp_disable_drv_zc(pool);
    xsk_clear_pool_at_qid(pool.netdev, pool.queue_id);
    pool.netdev = core::ptr::null_mut();
    netdev_unlock_ops(netdev);
    dev_put(netdev);
    }
#[no_mangle]
unsafe extern "C" fn xp_release_deferred(work: *mut work_struct) {
    static void xp_release_deferred(struct work_struct *work)
    {
    struct xsk_buff_pool *pool = container_of(work, struct xsk_buff_pool,
    work);
    rtnl_lock();
    xp_clear_dev(pool);
    rtnl_unlock();
    if (pool.fq) {
    xskq_destroy(pool.fq);
    pool.fq = core::ptr::null_mut();
    }
    if (pool.cq) {
    xskq_destroy(pool.cq);
    pool.cq = core::ptr::null_mut();
    }
    xdp_put_umem(pool.umem, false);
    xp_destroy(pool);
    }
#[no_mangle]
pub unsafe extern "C" fn xp_get_pool(pool: *mut xsk_buff_pool) {
    void xp_get_pool(struct xsk_buff_pool *pool)
    {
    refcount_inc(&pool.users);
    }
#[no_mangle]
pub unsafe extern "C" fn xp_put_pool(pool: *mut xsk_buff_pool) -> bool {
    bool xp_put_pool(struct xsk_buff_pool *pool)
    {
    if (!pool)
    return false;
    if (refcount_dec_and_test(&pool.users)) {
    INIT_WORK(&pool.work, xp_release_deferred);
    schedule_work(&pool.work);
    return true;
    }
    return false;
    }
    static struct xsk_dma_map *xp_find_dma_map(struct xsk_buff_pool *pool)
    {
    struct xsk_dma_map *dma_map;
    list_for_each_entry(dma_map, &pool.umem.xsk_dma_list, list) {
    if (dma_map.netdev == pool.netdev)
    return dma_map;
    }
    return core::ptr::null_mut();
    }
    static struct xsk_dma_map *xp_create_dma_map(struct device *dev, struct net_device *netdev,
    u32 nr_pages, struct xdp_umem *umem)
    {
    struct xsk_dma_map *dma_map;
    dma_map = kzalloc_obj(*dma_map);
    if (!dma_map)
    return core::ptr::null_mut();
    dma_map.dma_pages = kvzalloc_objs(*dma_map.dma_pages, nr_pages);
    if (!dma_map.dma_pages) {
    kfree(dma_map);
    return core::ptr::null_mut();
    }
    dma_map.netdev = netdev;
    dma_map.dev = dev;
    dma_map.dma_pages_cnt = nr_pages;
    refcount_set(&dma_map.users, 1);
    list_add(&dma_map.list, &umem.xsk_dma_list);
    return dma_map;
    }
#[no_mangle]
unsafe extern "C" fn xp_destroy_dma_map(dma_map: *mut xsk_dma_map) {
    static void xp_destroy_dma_map(struct xsk_dma_map *dma_map)
    {
    list_del(&dma_map.list);
    kvfree(dma_map.dma_pages);
    kfree(dma_map);
    }
#[no_mangle]
unsafe extern "C" fn __xp_dma_unmap(dma_map: *mut xsk_dma_map, attrs: c_ulong) {
    static void __xp_dma_unmap(struct xsk_dma_map *dma_map, unsigned long attrs)
    {
    dma_addr_t *dma;
    u32 i;
    for (i = 0; i < dma_map.dma_pages_cnt; i++) {
    dma = &dma_map.dma_pages[i];
    if (*dma) {
// dma &= ~XSK_NEXT_PG_CONTIG_MASK;
    dma_unmap_page_attrs(dma_map.dev, *dma, PAGE_SIZE,
    DMA_BIDIRECTIONAL, attrs);
// dma = 0;
    }
    }
    xp_destroy_dma_map(dma_map);
    }
#[no_mangle]
pub unsafe extern "C" fn xp_dma_unmap(pool: *mut xsk_buff_pool, attrs: c_ulong) {
    void xp_dma_unmap(struct xsk_buff_pool *pool, unsigned long attrs)
    {
    struct xsk_dma_map *dma_map;
    if (!pool.dma_pages)
    return;
    dma_map = xp_find_dma_map(pool);
    if (!dma_map) {
    WARN(1, "Could not find dma_map for device");
    return;
    }
    if (refcount_dec_and_test(&dma_map.users))
    __xp_dma_unmap(dma_map, attrs);
    kvfree(pool.dma_pages);
    pool.dma_pages = core::ptr::null_mut();
    pool.dma_pages_cnt = 0;
    pool.dev = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(xp_dma_unmap);
#[no_mangle]
unsafe extern "C" fn xp_check_dma_contiguity(dma_map: *mut xsk_dma_map) {
    static void xp_check_dma_contiguity(struct xsk_dma_map *dma_map)
    {
    u32 i;
    for (i = 0; i < dma_map.dma_pages_cnt - 1; i++) {
    if (dma_map.dma_pages[i] + PAGE_SIZE == dma_map.dma_pages[i + 1])
    dma_map.dma_pages[i] |= XSK_NEXT_PG_CONTIG_MASK;
    else
    dma_map.dma_pages[i] &= ~XSK_NEXT_PG_CONTIG_MASK;
    }
    }
#[no_mangle]
unsafe extern "C" fn xp_init_dma_info(pool: *mut xsk_buff_pool, dma_map: *mut xsk_dma_map) -> c_int {
    static int xp_init_dma_info(struct xsk_buff_pool *pool, struct xsk_dma_map *dma_map)
    {
    if (!pool.unaligned) {
    u32 i;
    for (i = 0; i < pool.heads_cnt; i++) {
    struct xdp_buff_xsk *xskb = &pool.heads[i];
    u64 orig_addr;
    orig_addr = xskb.xdp.data_hard_start - pool.addrs - pool.headroom;
    xp_init_xskb_dma(xskb, pool, dma_map.dma_pages, orig_addr);
    }
    }
    pool.dma_pages = kvzalloc_objs(*pool.dma_pages,
    dma_map.dma_pages_cnt);
    if (!pool.dma_pages)
    return -ENOMEM;
    pool.dev = dma_map.dev;
    pool.dma_pages_cnt = dma_map.dma_pages_cnt;
    memcpy(pool.dma_pages, dma_map.dma_pages,
    pool.dma_pages_cnt * sizeof(*pool.dma_pages));
    return 0;
    }
    int xp_dma_map(struct xsk_buff_pool *pool, struct device *dev,
    unsigned long attrs, struct page **pages, u32 nr_pages)
    {
    struct xsk_dma_map *dma_map;
    dma_addr_t dma;
    int err;
    u32 i;
    dma_map = xp_find_dma_map(pool);
    if (dma_map) {
    err = xp_init_dma_info(pool, dma_map);
    if (err)
    return err;
    refcount_inc(&dma_map.users);
    return 0;
    }
    dma_map = xp_create_dma_map(dev, pool.netdev, nr_pages, pool.umem);
    if (!dma_map)
    return -ENOMEM;
    for (i = 0; i < dma_map.dma_pages_cnt; i++) {
    dma = dma_map_page_attrs(dev, pages[i], 0, PAGE_SIZE,
    DMA_BIDIRECTIONAL, attrs);
    if (dma_mapping_error(dev, dma)) {
    __xp_dma_unmap(dma_map, attrs);
    return -ENOMEM;
    }
    dma_map.dma_pages[i] = dma;
    }
    if (pool.unaligned)
    xp_check_dma_contiguity(dma_map);
    err = xp_init_dma_info(pool, dma_map);
    if (err) {
    __xp_dma_unmap(dma_map, attrs);
    return err;
    }
    return 0;
    }
    EXPORT_SYMBOL(xp_dma_map);
    static bool xp_addr_crosses_non_contig_pg(struct xsk_buff_pool *pool,
    u64 addr)
    {
    return xp_desc_crosses_non_contig_pg(pool, addr, pool.chunk_size);
    }
#[no_mangle]
unsafe extern "C" fn xp_check_unaligned(pool: *mut xsk_buff_pool, addr: *mut u64) -> bool {
    static bool xp_check_unaligned(struct xsk_buff_pool *pool, u64 *addr)
    {
// addr = xp_unaligned_extract_addr(*addr);
    if (*addr >= pool.addrs_cnt ||
// addr + pool->chunk_size > pool->addrs_cnt ||
    xp_addr_crosses_non_contig_pg(pool, *addr))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn xp_check_aligned(pool: *mut xsk_buff_pool, addr: *mut u64) -> bool {
    static bool xp_check_aligned(struct xsk_buff_pool *pool, u64 *addr)
    {
// addr = xp_aligned_extract_addr(pool, *addr);
    return *addr < pool.addrs_cnt;
    }
    static struct xdp_buff_xsk *xp_get_xskb(struct xsk_buff_pool *pool, u64 addr)
    {
    struct xdp_buff_xsk *xskb;
    if (pool.unaligned) {
    xskb = pool.free_heads[--pool.free_heads_cnt];
    xp_init_xskb_addr(xskb, pool, addr);
    if (pool.dma_pages)
    xp_init_xskb_dma(xskb, pool, pool.dma_pages, addr);
    } else {
    xskb = &pool.heads[xp_aligned_extract_idx(pool, addr)];
    }
    return xskb;
    }
    static struct xdp_buff_xsk *__xp_alloc(struct xsk_buff_pool *pool)
    {
    struct xdp_buff_xsk *xskb;
    u64 addr;
    bool ok;
    if (pool.free_heads_cnt == 0)
    return core::ptr::null_mut();
    for (;;) {
    if (!xskq_cons_peek_addr_unchecked(pool.fq, &addr)) {
    pool.fq.queue_empty_descs++;
    return core::ptr::null_mut();
    }
    ok = pool.unaligned ? xp_check_unaligned(pool, &addr) :
    xp_check_aligned(pool, &addr);
    if (!ok) {
    pool.fq.invalid_descs++;
    xskq_cons_release(pool.fq);
    continue;
    }
    break;
    }
    xskb = xp_get_xskb(pool, addr);
    xskq_cons_release(pool.fq);
    return xskb;
    }
    struct xdp_buff *xp_alloc(struct xsk_buff_pool *pool)
    {
    struct xdp_buff_xsk *xskb;
    if (!pool.free_list_cnt) {
    xskb = __xp_alloc(pool);
    if (!xskb)
    return core::ptr::null_mut();
    } else {
    pool.free_list_cnt--;
    xskb = list_first_entry(&pool.free_list, struct xdp_buff_xsk,
    list_node);
    list_del_init(&xskb.list_node);
    }
    xskb.xdp.data = xskb.xdp.data_hard_start + XDP_PACKET_HEADROOM;
    xskb.xdp.data_meta = xskb.xdp.data;
    xskb.xdp.flags = 0;
    if (pool.dev)
    xp_dma_sync_for_device(pool, xskb.dma, pool.frame_len);
    return &xskb.xdp;
    }
    EXPORT_SYMBOL(xp_alloc);
#[no_mangle]
unsafe extern "C" fn xp_alloc_new_from_fq(pool: *mut xsk_buff_pool, xdp: *mut xdp_buff, max: u32) -> u32 {
    static u32 xp_alloc_new_from_fq(struct xsk_buff_pool *pool, struct xdp_buff **xdp, u32 max)
    {
    u32 i, cached_cons, nb_entries;
    if (max > pool.free_heads_cnt)
    max = pool.free_heads_cnt;
    max = xskq_cons_nb_entries(pool.fq, max);
    cached_cons = pool.fq.cached_cons;
    nb_entries = max;
    i = max;
    while (i--) {
    struct xdp_buff_xsk *xskb;
    u64 addr;
    bool ok;
    __xskq_cons_read_addr_unchecked(pool.fq, cached_cons++, &addr);
    ok = pool.unaligned ? xp_check_unaligned(pool, &addr) :
    xp_check_aligned(pool, &addr);
    if (unlikely(!ok)) {
    pool.fq.invalid_descs++;
    nb_entries--;
    continue;
    }
    xskb = xp_get_xskb(pool, addr);
// xdp = &xskb->xdp;
    xdp++;
    }
    xskq_cons_release_n(pool.fq, max);
    return nb_entries;
    }
#[no_mangle]
unsafe extern "C" fn xp_alloc_reused(pool: *mut xsk_buff_pool, xdp: *mut xdp_buff, nb_entries: u32) -> u32 {
    static u32 xp_alloc_reused(struct xsk_buff_pool *pool, struct xdp_buff **xdp, u32 nb_entries)
    {
    struct xdp_buff_xsk *xskb;
    u32 i;
    nb_entries = min_t(u32, nb_entries, pool.free_list_cnt);
    i = nb_entries;
    while (i--) {
    xskb = list_first_entry(&pool.free_list, struct xdp_buff_xsk, list_node);
    list_del_init(&xskb.list_node);
// xdp = &xskb->xdp;
    xdp++;
    }
    pool.free_list_cnt -= nb_entries;
    return nb_entries;
    }
    static u32 xp_alloc_slow(struct xsk_buff_pool *pool, struct xdp_buff **xdp,
    u32 max)
    {
    int i;
    for (i = 0; i < max; i++) {
    struct xdp_buff *buff;
    buff = xp_alloc(pool);
    if (unlikely(!buff))
    return i;
// xdp = buff;
    xdp++;
    }
    return max;
    }
#[no_mangle]
pub unsafe extern "C" fn xp_alloc_batch(pool: *mut xsk_buff_pool, xdp: *mut xdp_buff, max: u32) -> u32 {
    u32 xp_alloc_batch(struct xsk_buff_pool *pool, struct xdp_buff **xdp, u32 max)
    {
    let mut nb_entries1: u32 = 0, nb_entries2;
    if (unlikely(pool.dev && dma_dev_need_sync(pool.dev)))
    return xp_alloc_slow(pool, xdp, max);
    if (unlikely(pool.free_list_cnt)) {
    nb_entries1 = xp_alloc_reused(pool, xdp, max);
    if (nb_entries1 == max)
    return nb_entries1;
    max -= nb_entries1;
    xdp += nb_entries1;
    }
    nb_entries2 = xp_alloc_new_from_fq(pool, xdp, max);
    if (!nb_entries2)
    pool.fq.queue_empty_descs++;
    return nb_entries1 + nb_entries2;
    }
    EXPORT_SYMBOL(xp_alloc_batch);
#[no_mangle]
pub unsafe extern "C" fn xp_can_alloc(pool: *mut xsk_buff_pool, count: u32) -> bool {
    bool xp_can_alloc(struct xsk_buff_pool *pool, u32 count)
    {
    u32 req_count, avail_count;
    if (pool.free_list_cnt >= count)
    return true;
    req_count = count - pool.free_list_cnt;
    avail_count = xskq_cons_nb_entries(pool.fq, req_count);
    if (!avail_count)
    pool.fq.queue_empty_descs++;
    return avail_count >= req_count;
    }
    EXPORT_SYMBOL(xp_can_alloc);
#[no_mangle]
pub unsafe extern "C" fn xp_free(xskb: *mut xdp_buff_xsk) {
    void xp_free(struct xdp_buff_xsk *xskb)
    {
    if (!list_empty(&xskb.list_node))
    return;
    xskb.pool.free_list_cnt++;
    list_add(&xskb.list_node, &xskb.pool.free_list);
    }
    EXPORT_SYMBOL(xp_free);
#[no_mangle]
unsafe extern "C" fn __xp_raw_get_addr(pool: *const xsk_buff_pool, addr: u64) -> u64 {
    static u64 __xp_raw_get_addr(const struct xsk_buff_pool *pool, u64 addr)
    {
    return pool.unaligned ? xp_unaligned_add_offset_to_addr(addr) : addr;
    }
    static void *__xp_raw_get_data(const struct xsk_buff_pool *pool, u64 addr)
    {
    return pool.addrs + addr;
    }
    void *xp_raw_get_data(struct xsk_buff_pool *pool, u64 addr)
    {
    return __xp_raw_get_data(pool, __xp_raw_get_addr(pool, addr));
    }
    EXPORT_SYMBOL(xp_raw_get_data);
#[no_mangle]
unsafe extern "C" fn __xp_raw_get_dma(pool: *const xsk_buff_pool, addr: u64) -> dma_addr_t {
    static dma_addr_t __xp_raw_get_dma(const struct xsk_buff_pool *pool, u64 addr)
    {
    return (pool.dma_pages[addr >> PAGE_SHIFT] &
    ~XSK_NEXT_PG_CONTIG_MASK) +
    (addr & ~PAGE_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn xp_raw_get_dma(pool: *mut xsk_buff_pool, addr: u64) -> dma_addr_t {
    dma_addr_t xp_raw_get_dma(struct xsk_buff_pool *pool, u64 addr)
    {
    return __xp_raw_get_dma(pool, __xp_raw_get_addr(pool, addr));
    }
    EXPORT_SYMBOL(xp_raw_get_dma);
//
// xp_raw_get_ctx - get &xdp_desc context
// @pool: XSk buff pool desc address belongs to
// @addr: desc address (from userspace)
// @options: desc options (from userspace)
//
// Helper for getting desc's DMA address and metadata pointer, if present.
// Saves one call on hotpath and double calculation of the actual address.
// Metadata is validated later by xsk_tx_metadata_request().
//
// Return: new &xdp_desc_ctx struct containing desc's DMA address and metadata
// pointer, if it is present (initialized to %NULL otherwise).
//
    struct xdp_desc_ctx xp_raw_get_ctx(const struct xsk_buff_pool *pool, u64 addr,
    u32 options)
    {
    struct xdp_desc_ctx ret;
    addr = __xp_raw_get_addr(pool, addr);
    ret.dma = __xp_raw_get_dma(pool, addr);
    ret.meta = __xsk_buff_get_metadata(pool, __xp_raw_get_data(pool, addr),
    options);
    return ret;
    }
    EXPORT_SYMBOL(xp_raw_get_ctx);
