//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_debugfs.c
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
// Copyright(c) 2013 - 2018 Intel Corporation.

    static struct dentry *dbg_root;
// Descriptor Seq Functions
    static void *fm10k_dbg_desc_seq_start(struct seq_file *s, loff_t *pos)
    {
    struct fm10k_ring *ring = s.private;
    return (*pos < ring.count) ? pos : core::ptr::null_mut();
    }
    static void *fm10k_dbg_desc_seq_next(struct seq_file *s,
    void __always_unused *v,
    loff_t *pos)
    {
    struct fm10k_ring *ring = s.private;
    return (++(*pos) < ring.count) ? pos : core::ptr::null_mut();
    }
    static void fm10k_dbg_desc_seq_stop(struct seq_file __always_unused *s,
    void __always_unused *v)
    {
// Do nothing.
    }
#[no_mangle]
unsafe extern "C" fn fm10k_dbg_desc_break(s: *mut seq_file, i: c_int) {
    static void fm10k_dbg_desc_break(struct seq_file *s, int i)
    {
    while (i--)
    seq_putc(s, '-');
    seq_putc(s, '\n');
    }
#[no_mangle]
unsafe extern "C" fn fm10k_dbg_tx_desc_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int fm10k_dbg_tx_desc_seq_show(struct seq_file *s, void *v)
    {
    struct fm10k_ring *ring = s.private;
    let mut i: c_int = *(loff_t *)v;
    static const char tx_desc_hdr[] =
    "DES BUFFER_ADDRESS     LENGTH VLAN   MSS    HDRLEN FLAGS\n";
// Generate header
    if (!i) {
    seq_printf(s, tx_desc_hdr);
    fm10k_dbg_desc_break(s, sizeof(tx_desc_hdr) - 1);
    }
// Validate descriptor allocation
    if (!ring.desc) {
    seq_printf(s, "%03X Descriptor ring not allocated.\n", i);
    } else {
    struct fm10k_tx_desc *txd = FM10K_TX_DESC(ring, i);
    seq_printf(s, "%03X %#018llx %#06x %#06x %#06x %#06x %#04x\n",
    i, txd.buffer_addr, txd.buflen, txd.vlan,
    txd.mss, txd.hdrlen, txd.flags);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fm10k_dbg_rx_desc_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int fm10k_dbg_rx_desc_seq_show(struct seq_file *s, void *v)
    {
    struct fm10k_ring *ring = s.private;
    let mut i: c_int = *(loff_t *)v;
    static const char rx_desc_hdr[] =
    "DES DATA       RSS        STATERR    LENGTH VLAN   DGLORT SGLORT TIMESTAMP\n";
// Generate header
    if (!i) {
    seq_printf(s, rx_desc_hdr);
    fm10k_dbg_desc_break(s, sizeof(rx_desc_hdr) - 1);
    }
// Validate descriptor allocation
    if (!ring.desc) {
    seq_printf(s, "%03X Descriptor ring not allocated.\n", i);
    } else {
    union fm10k_rx_desc *rxd = FM10K_RX_DESC(ring, i);
    seq_printf(s,
    "%03X %#010x %#010x %#010x %#06x %#06x %#06x %#06x %#018llx\n",
    i, rxd.d.data, rxd.d.rss, rxd.d.staterr,
    rxd.w.length, rxd.w.vlan, rxd.w.dglort,
    rxd.w.sglort, rxd.q.timestamp);
    }
    return 0;
    }
    static const struct seq_operations fm10k_dbg_tx_desc_seq_ops = {
    .start = fm10k_dbg_desc_seq_start,
    .next  = fm10k_dbg_desc_seq_next,
    .stop  = fm10k_dbg_desc_seq_stop,
    .show  = fm10k_dbg_tx_desc_seq_show,
    };
    static const struct seq_operations fm10k_dbg_rx_desc_seq_ops = {
    .start = fm10k_dbg_desc_seq_start,
    .next  = fm10k_dbg_desc_seq_next,
    .stop  = fm10k_dbg_desc_seq_stop,
    .show  = fm10k_dbg_rx_desc_seq_show,
    };
#[no_mangle]
unsafe extern "C" fn fm10k_dbg_desc_open(inode: *mut inode, filep: *mut file) -> c_int {
    static int fm10k_dbg_desc_open(struct inode *inode, struct file *filep)
    {
    struct fm10k_ring *ring = inode.i_private;
    struct fm10k_q_vector *q_vector = ring.q_vector;
    const struct seq_operations *desc_seq_ops;
    int err;
    if (ring < q_vector.rx.ring)
    desc_seq_ops = &fm10k_dbg_tx_desc_seq_ops;
    else
    desc_seq_ops = &fm10k_dbg_rx_desc_seq_ops;
    err = seq_open(filep, desc_seq_ops);
    if (err)
    return err;
    ((struct seq_file *)filep.private_data).private = ring;
    return 0;
    }
    static const struct file_operations fm10k_dbg_desc_fops = {
    .owner   = THIS_MODULE,
    .open    = fm10k_dbg_desc_open,
    .read    = seq_read,
    .llseek  = seq_lseek,
    .release = seq_release,
    };
//
// fm10k_dbg_q_vector_init - setup debugfs for the q_vectors
// @q_vector: q_vector to allocate directories for
//
// A folder is created for each q_vector found. In each q_vector
// folder, a debugfs file is created for each tx and rx ring
// allocated to the q_vector.
//
#[no_mangle]
pub unsafe extern "C" fn fm10k_dbg_q_vector_init(q_vector: *mut fm10k_q_vector) {
    void fm10k_dbg_q_vector_init(struct fm10k_q_vector *q_vector)
    {
    struct fm10k_intfc *interface = q_vector.interface;
    char name[16];
    int i;
    if (!interface.dbg_intfc)
    return;
// Generate a folder for each q_vector
    snprintf(name, sizeof(name), "q_vector.%03d", q_vector.v_idx);
    q_vector.dbg_q_vector = debugfs_create_dir(name, interface.dbg_intfc);
// Generate a file for each rx ring in the q_vector
    for (i = 0; i < q_vector.tx.count; i++) {
    struct fm10k_ring *ring = &q_vector.tx.ring[i];
    snprintf(name, sizeof(name), "tx_ring.%03d", ring.queue_index);
    debugfs_create_file(name, 0600,
    q_vector.dbg_q_vector, ring,
    &fm10k_dbg_desc_fops);
    }
// Generate a file for each rx ring in the q_vector
    for (i = 0; i < q_vector.rx.count; i++) {
    struct fm10k_ring *ring = &q_vector.rx.ring[i];
    snprintf(name, sizeof(name), "rx_ring.%03d", ring.queue_index);
    debugfs_create_file(name, 0600,
    q_vector.dbg_q_vector, ring,
    &fm10k_dbg_desc_fops);
    }
    }
//
// fm10k_dbg_q_vector_exit - setup debugfs for the q_vectors
// @q_vector: q_vector to allocate directories for
//
#[no_mangle]
pub unsafe extern "C" fn fm10k_dbg_q_vector_exit(q_vector: *mut fm10k_q_vector) {
    void fm10k_dbg_q_vector_exit(struct fm10k_q_vector *q_vector)
    {
    struct fm10k_intfc *interface = q_vector.interface;
    if (interface.dbg_intfc)
    debugfs_remove_recursive(q_vector.dbg_q_vector);
    q_vector.dbg_q_vector = core::ptr::null_mut();
    }
//
// fm10k_dbg_intfc_init - setup the debugfs directory for the intferface
// @interface: the interface that is starting up
//
#[no_mangle]
pub unsafe extern "C" fn fm10k_dbg_intfc_init(interface: *mut fm10k_intfc) {
    void fm10k_dbg_intfc_init(struct fm10k_intfc *interface)
    {
    const char *name = pci_name(interface.pdev);
    if (dbg_root)
    interface.dbg_intfc = debugfs_create_dir(name, dbg_root);
    }
//
// fm10k_dbg_intfc_exit - clean out the interface's debugfs entries
// @interface: the interface that is stopping
//
#[no_mangle]
pub unsafe extern "C" fn fm10k_dbg_intfc_exit(interface: *mut fm10k_intfc) {
    void fm10k_dbg_intfc_exit(struct fm10k_intfc *interface)
    {
    if (dbg_root)
    debugfs_remove_recursive(interface.dbg_intfc);
    interface.dbg_intfc = core::ptr::null_mut();
    }
//
// fm10k_dbg_init - start up debugfs for the driver
//
#[no_mangle]
pub unsafe extern "C" fn fm10k_dbg_init() {
    void fm10k_dbg_init(void)
    {
    dbg_root = debugfs_create_dir(fm10k_driver_name, core::ptr::null_mut());
    }
//
// fm10k_dbg_exit - clean out the driver's debugfs entries
//
#[no_mangle]
pub unsafe extern "C" fn fm10k_dbg_exit() {
    void fm10k_dbg_exit(void)
    {
    debugfs_remove_recursive(dbg_root);
    dbg_root = core::ptr::null_mut();
    }
