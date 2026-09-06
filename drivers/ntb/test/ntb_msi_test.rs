//! Automatically rewritten from C to Rust
//! Source: drivers/ntb/test/ntb_msi_test.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)

    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_VERSION("0.1");
    MODULE_AUTHOR("Logan Gunthorpe <logang@deltatee.com>");
    MODULE_DESCRIPTION("Test for sending MSI interrupts over an NTB memory window");
    let mut num_irqs: static int = 4;
    module_param(num_irqs, int, 0644);
    MODULE_PARM_DESC(num_irqs, "number of irqs to use");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_msit_ctx {
    pub ntb: *mut ntb_dev,
    pub dbgfs_dir: *mut dentry,
    pub setup_work: work_struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_msit_isr_ctx {
    pub irq_idx: c_int,
    pub irq_num: c_int,
    pub occurrences: c_int,
    pub nm: *mut ntb_msit_ctx,
    pub desc: ntb_msi_desc,
    pub isr_ctx: *mut },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_msit_peer {
    pub nm: *mut ntb_msit_ctx,
    pub pidx: c_int,
    pub num_irqs: c_int,
    pub init_comp: completion,
    pub msi_desc: *mut ntb_msi_desc,
    pub peers: [}; ],
}

    static struct dentry *ntb_msit_dbgfs_topdir;
#[no_mangle]
unsafe extern "C" fn ntb_msit_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t ntb_msit_isr(int irq, void *dev)
    {
    struct ntb_msit_isr_ctx *isr_ctx = dev;
    struct ntb_msit_ctx *nm = isr_ctx.nm;
    dev_dbg(&nm.ntb.dev, "Interrupt Occurred: %d",
    isr_ctx.irq_idx);
    isr_ctx.occurrences++;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_setup_work(work: *mut work_struct) {
    static void ntb_msit_setup_work(struct work_struct *work)
    {
    struct ntb_msit_ctx *nm = container_of(work, struct ntb_msit_ctx,
    setup_work);
    let mut irq_count: c_int = 0;
    int irq;
    int ret;
    uintptr_t i;
    ret = ntb_msi_setup_mws(nm.ntb);
    if (ret) {
    dev_err(&nm.ntb.dev, "Unable to setup MSI windows: %d\n",
    ret);
    return;
    }
    for (i = 0; i < num_irqs; i++) {
    nm.isr_ctx[i].irq_idx = i;
    nm.isr_ctx[i].nm = nm;
    if (!nm.isr_ctx[i].irq_num) {
    irq = ntbm_msi_request_irq(nm.ntb, ntb_msit_isr,
    KBUILD_MODNAME,
    &nm.isr_ctx[i],
    &nm.isr_ctx[i].desc);
    if (irq < 0)
    break;
    nm.isr_ctx[i].irq_num = irq;
    }
    ret = ntb_spad_write(nm.ntb, 2 * i + 1,
    nm.isr_ctx[i].desc.addr_offset);
    if (ret)
    break;
    ret = ntb_spad_write(nm.ntb, 2 * i + 2,
    nm.isr_ctx[i].desc.data);
    if (ret)
    break;
    irq_count++;
    }
    ntb_spad_write(nm.ntb, 0, irq_count);
    ntb_peer_db_set(nm.ntb, BIT(ntb_port_number(nm.ntb)));
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_desc_changed(ctx: *mut c_void) {
    static void ntb_msit_desc_changed(void *ctx)
    {
    struct ntb_msit_ctx *nm = ctx;
    int i;
    dev_dbg(&nm.ntb.dev, "MSI Descriptors Changed\n");
    for (i = 0; i < num_irqs; i++) {
    ntb_spad_write(nm.ntb, 2 * i + 1,
    nm.isr_ctx[i].desc.addr_offset);
    ntb_spad_write(nm.ntb, 2 * i + 2,
    nm.isr_ctx[i].desc.data);
    }
    ntb_peer_db_set(nm.ntb, BIT(ntb_port_number(nm.ntb)));
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_link_event(ctx: *mut c_void) {
    static void ntb_msit_link_event(void *ctx)
    {
    struct ntb_msit_ctx *nm = ctx;
    if (!ntb_link_is_up(nm.ntb, core::ptr::null_mut(), core::ptr::null_mut()))
    return;
    schedule_work(&nm.setup_work);
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_copy_peer_desc(nm: *mut ntb_msit_ctx, peer: c_int) {
    static void ntb_msit_copy_peer_desc(struct ntb_msit_ctx *nm, int peer)
    {
    int i;
    struct ntb_msi_desc *desc = nm.peers[peer].msi_desc;
    let mut irq_count: c_int = nm.peers[peer].num_irqs;
    for (i = 0; i < irq_count; i++) {
    desc[i].addr_offset = ntb_peer_spad_read(nm.ntb, peer,
    2 * i + 1);
    desc[i].data = ntb_peer_spad_read(nm.ntb, peer, 2 * i + 2);
    }
    dev_info(&nm.ntb.dev, "Found %d interrupts on peer %d\n",
    irq_count, peer);
    complete_all(&nm.peers[peer].init_comp);
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_db_event(ctx: *mut c_void, vec: c_int) {
    static void ntb_msit_db_event(void *ctx, int vec)
    {
    struct ntb_msit_ctx *nm = ctx;
    struct ntb_msi_desc *desc;
    let mut peer_mask: u64 = ntb_db_read(nm.ntb);
    u32 irq_count;
    int peer;
    ntb_db_clear(nm.ntb, peer_mask);
    for (peer = 0; peer < sizeof(peer_mask) * 8; peer++) {
    if (!(peer_mask & BIT(peer)))
    continue;
    irq_count = ntb_peer_spad_read(nm.ntb, peer, 0);
    if (irq_count == -1)
    continue;
    desc = kzalloc_objs(*desc, irq_count, GFP_ATOMIC);
    if (!desc)
    continue;
    kfree(nm.peers[peer].msi_desc);
    nm.peers[peer].msi_desc = desc;
    nm.peers[peer].num_irqs = irq_count;
    ntb_msit_copy_peer_desc(nm, peer);
    }
    }
    static const struct ntb_ctx_ops ntb_msit_ops = {
    .link_event = ntb_msit_link_event,
    .db_event = ntb_msit_db_event,
    };
#[no_mangle]
unsafe extern "C" fn ntb_msit_dbgfs_trigger(data: *mut c_void, idx: u64) -> c_int {
    static int ntb_msit_dbgfs_trigger(void *data, u64 idx)
    {
    struct ntb_msit_peer *peer = data;
    if (idx >= peer.num_irqs)
    return -EINVAL;
    dev_dbg(&peer.nm.ntb.dev, "trigger irq %llu on peer %u\n",
    idx, peer.pidx);
    return ntb_msi_peer_trigger(peer.nm.ntb, peer.pidx,
    &peer.msi_desc[idx]);
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ntb_msit_trigger_fops, core::ptr::null_mut(),
    ntb_msit_dbgfs_trigger, "%llu\n");
#[no_mangle]
unsafe extern "C" fn ntb_msit_dbgfs_port_get(data: *mut c_void, port: *mut u64) -> c_int {
    static int ntb_msit_dbgfs_port_get(void *data, u64 *port)
    {
    struct ntb_msit_peer *peer = data;
// port = ntb_peer_port_number(peer->nm->ntb, peer->pidx);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ntb_msit_port_fops, ntb_msit_dbgfs_port_get,
    core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn ntb_msit_dbgfs_count_get(data: *mut c_void, count: *mut u64) -> c_int {
    static int ntb_msit_dbgfs_count_get(void *data, u64 *count)
    {
    struct ntb_msit_peer *peer = data;
// count = peer->num_irqs;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ntb_msit_count_fops, ntb_msit_dbgfs_count_get,
    core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn ntb_msit_dbgfs_ready_get(data: *mut c_void, ready: *mut u64) -> c_int {
    static int ntb_msit_dbgfs_ready_get(void *data, u64 *ready)
    {
    struct ntb_msit_peer *peer = data;
// ready = try_wait_for_completion(&peer->init_comp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_dbgfs_ready_set(data: *mut c_void, ready: u64) -> c_int {
    static int ntb_msit_dbgfs_ready_set(void *data, u64 ready)
    {
    struct ntb_msit_peer *peer = data;
    return wait_for_completion_interruptible(&peer.init_comp);
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ntb_msit_ready_fops, ntb_msit_dbgfs_ready_get,
    ntb_msit_dbgfs_ready_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn ntb_msit_dbgfs_occurrences_get(data: *mut c_void, occurrences: *mut u64) -> c_int {
    static int ntb_msit_dbgfs_occurrences_get(void *data, u64 *occurrences)
    {
    struct ntb_msit_isr_ctx *isr_ctx = data;
// occurrences = isr_ctx->occurrences;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ntb_msit_occurrences_fops,
    ntb_msit_dbgfs_occurrences_get,
    core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn ntb_msit_dbgfs_local_port_get(data: *mut c_void, port: *mut u64) -> c_int {
    static int ntb_msit_dbgfs_local_port_get(void *data, u64 *port)
    {
    struct ntb_msit_ctx *nm = data;
// port = ntb_port_number(nm->ntb);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ntb_msit_local_port_fops,
    ntb_msit_dbgfs_local_port_get,
    core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn ntb_msit_create_dbgfs(nm: *mut ntb_msit_ctx) {
    static void ntb_msit_create_dbgfs(struct ntb_msit_ctx *nm)
    {
    struct pci_dev *pdev = nm.ntb.pdev;
    char buf[32];
    int i;
    struct dentry *peer_dir;
    nm.dbgfs_dir = debugfs_create_dir(pci_name(pdev),
    ntb_msit_dbgfs_topdir);
    debugfs_create_file("port", 0400, nm.dbgfs_dir, nm,
    &ntb_msit_local_port_fops);
    for (i = 0; i < ntb_peer_port_count(nm.ntb); i++) {
    nm.peers[i].pidx = i;
    nm.peers[i].nm = nm;
    init_completion(&nm.peers[i].init_comp);
    snprintf(buf, sizeof(buf), "peer%d", i);
    peer_dir = debugfs_create_dir(buf, nm.dbgfs_dir);
    debugfs_create_file_unsafe("trigger", 0200, peer_dir,
    &nm.peers[i],
    &ntb_msit_trigger_fops);
    debugfs_create_file_unsafe("port", 0400, peer_dir,
    &nm.peers[i], &ntb_msit_port_fops);
    debugfs_create_file_unsafe("count", 0400, peer_dir,
    &nm.peers[i],
    &ntb_msit_count_fops);
    debugfs_create_file_unsafe("ready", 0600, peer_dir,
    &nm.peers[i],
    &ntb_msit_ready_fops);
    }
    for (i = 0; i < num_irqs; i++) {
    snprintf(buf, sizeof(buf), "irq%d_occurrences", i);
    debugfs_create_file_unsafe(buf, 0400, nm.dbgfs_dir,
    &nm.isr_ctx[i],
    &ntb_msit_occurrences_fops);
    }
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_remove_dbgfs(nm: *mut ntb_msit_ctx) {
    static void ntb_msit_remove_dbgfs(struct ntb_msit_ctx *nm)
    {
    debugfs_remove_recursive(nm.dbgfs_dir);
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_probe(client: *mut ntb_client, ntb: *mut ntb_dev) -> c_int {
    static int ntb_msit_probe(struct ntb_client *client, struct ntb_dev *ntb)
    {
    struct ntb_msit_ctx *nm;
    int peers;
    int ret;
    peers = ntb_peer_port_count(ntb);
    if (peers <= 0)
    return -EINVAL;
    if (ntb_spad_is_unsafe(ntb) || ntb_spad_count(ntb) < 2 * num_irqs + 1) {
    dev_err(&ntb.dev, "NTB MSI test requires at least %d spads for %d irqs\n",
    2 * num_irqs + 1, num_irqs);
    return -EFAULT;
    }
    ret = ntb_spad_write(ntb, 0, -1);
    if (ret) {
    dev_err(&ntb.dev, "Unable to write spads: %d\n", ret);
    return ret;
    }
    ret = ntb_db_clear_mask(ntb, GENMASK(peers - 1, 0));
    if (ret) {
    dev_err(&ntb.dev, "Unable to clear doorbell mask: %d\n", ret);
    return ret;
    }
    ret = ntb_msi_init(ntb, ntb_msit_desc_changed);
    if (ret) {
    dev_err(&ntb.dev, "Unable to initialize MSI library: %d\n",
    ret);
    return ret;
    }
    nm = devm_kzalloc(&ntb.dev, struct_size(nm, peers, peers), GFP_KERNEL);
    if (!nm)
    return -ENOMEM;
    nm.isr_ctx = devm_kcalloc(&ntb.dev, num_irqs, sizeof(*nm.isr_ctx),
    GFP_KERNEL);
    if (!nm.isr_ctx)
    return -ENOMEM;
    INIT_WORK(&nm.setup_work, ntb_msit_setup_work);
    nm.ntb = ntb;
    ntb_msit_create_dbgfs(nm);
    ret = ntb_set_ctx(ntb, nm, &ntb_msit_ops);
    if (ret)
    goto remove_dbgfs;
    if (!nm.isr_ctx) {
    ret = -ENOMEM;
    goto remove_dbgfs;
    }
    ntb_link_enable(ntb, NTB_SPEED_AUTO, NTB_WIDTH_AUTO);
    return 0;
    remove_dbgfs:
    ntb_msit_remove_dbgfs(nm);
    devm_kfree(&ntb.dev, nm.isr_ctx);
    devm_kfree(&ntb.dev, nm);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntb_msit_remove(client: *mut ntb_client, ntb: *mut ntb_dev) {
    static void ntb_msit_remove(struct ntb_client *client, struct ntb_dev *ntb)
    {
    struct ntb_msit_ctx *nm = ntb.ctx;
    int i;
    ntb_link_disable(ntb);
    ntb_db_set_mask(ntb, ntb_db_valid_mask(ntb));
    ntb_msi_clear_mws(ntb);
    for (i = 0; i < ntb_peer_port_count(ntb); i++)
    kfree(nm.peers[i].msi_desc);
    ntb_clear_ctx(ntb);
    ntb_msit_remove_dbgfs(nm);
    }
    static struct ntb_client ntb_msit_client = {
    .ops = {
    .probe = ntb_msit_probe,
    .remove = ntb_msit_remove
    }
    };
#[no_mangle]
unsafe extern "C" fn ntb_msit_init() -> int __init {
    static int __init ntb_msit_init(void)
    {
    int ret;
    if (debugfs_initialized())
    ntb_msit_dbgfs_topdir = debugfs_create_dir(KBUILD_MODNAME,
    core::ptr::null_mut());
    ret = ntb_register_client(&ntb_msit_client);
    if (ret)
    debugfs_remove_recursive(ntb_msit_dbgfs_topdir);
    return ret;
    }
    module_init(ntb_msit_init);
#[no_mangle]
unsafe extern "C" fn ntb_msit_exit() -> void __exit {
    static void __exit ntb_msit_exit(void)
    {
    ntb_unregister_client(&ntb_msit_client);
    debugfs_remove_recursive(ntb_msit_dbgfs_topdir);
    }
    module_exit(ntb_msit_exit);
