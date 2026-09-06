//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/adf_transport_debug.c
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

    static DEFINE_MUTEX(ring_read_lock);
    static DEFINE_MUTEX(bank_read_lock);

    (ADF_SIZE_TO_RING_SIZE_IN_BYTES(ring.ring_size) /	\
    ADF_MSG_SIZE_TO_BYTES(ring.msg_size))
    static void *adf_ring_start(struct seq_file *sfile, loff_t *pos)
    {
    struct adf_etr_ring_data *ring = sfile.private;
    let mut num_msg: c_uint = ADF_RING_NUM_MSGS(ring);
    let mut val: loff_t = *pos;
    mutex_lock(&ring_read_lock);
    if (val == 0)
    return SEQ_START_TOKEN;
    if (val >= num_msg)
    return core::ptr::null_mut();
    return ring.base_addr +
    (ADF_MSG_SIZE_TO_BYTES(ring.msg_size) * (*pos)++);
    }
    static void *adf_ring_next(struct seq_file *sfile, void *v, loff_t *pos)
    {
    struct adf_etr_ring_data *ring = sfile.private;
    let mut num_msg: c_uint = ADF_RING_NUM_MSGS(ring);
    let mut val: loff_t = *pos;
    (*pos)++;
    if (val >= num_msg)
    return core::ptr::null_mut();
    return ring.base_addr + (ADF_MSG_SIZE_TO_BYTES(ring.msg_size) * val);
    }
#[no_mangle]
unsafe extern "C" fn adf_ring_show(sfile: *mut seq_file, v: *mut c_void) -> c_int {
    static int adf_ring_show(struct seq_file *sfile, void *v)
    {
    struct adf_etr_ring_data *ring = sfile.private;
    struct adf_etr_bank_data *bank = ring.bank;
    struct adf_hw_csr_ops *csr_ops = GET_CSR_OPS(bank.accel_dev);
    void __iomem *csr = ring.bank.csr_addr;
    if (v == SEQ_START_TOKEN) {
    int head, tail, empty;
    head = csr_ops.read_csr_ring_head(csr, bank.bank_number,
    ring.ring_number);
    tail = csr_ops.read_csr_ring_tail(csr, bank.bank_number,
    ring.ring_number);
    empty = csr_ops.read_csr_e_stat(csr, bank.bank_number);
    seq_puts(sfile, "------- Ring configuration -------\n");
    seq_printf(sfile, "ring name: %s\n",
    ring.ring_debug.ring_name);
    seq_printf(sfile, "ring num %d, bank num %d\n",
    ring.ring_number, ring.bank.bank_number);
    seq_printf(sfile, "head %x, tail %x, empty: %d\n",
    head, tail, (empty & 1 << ring.ring_number)
    >> ring.ring_number);
    seq_printf(sfile, "ring size %lld, msg size %d\n",
    (long long)ADF_SIZE_TO_RING_SIZE_IN_BYTES(ring.ring_size),
    ADF_MSG_SIZE_TO_BYTES(ring.msg_size));
    seq_puts(sfile, "----------- Ring data ------------\n");
    return 0;
    }
    seq_hex_dump(sfile, "", DUMP_PREFIX_ADDRESS, 32, 4,
    v, ADF_MSG_SIZE_TO_BYTES(ring.msg_size), false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_ring_stop(sfile: *mut seq_file, v: *mut c_void) {
    static void adf_ring_stop(struct seq_file *sfile, void *v)
    {
    mutex_unlock(&ring_read_lock);
    }
    static const struct seq_operations adf_ring_debug_sops = {
    .start = adf_ring_start,
    .next = adf_ring_next,
    .stop = adf_ring_stop,
    .show = adf_ring_show
    };
    DEFINE_SEQ_ATTRIBUTE(adf_ring_debug);
#[no_mangle]
pub unsafe extern "C" fn adf_ring_debugfs_add(ring: *mut adf_etr_ring_data, name: *const c_char) -> c_int {
    int adf_ring_debugfs_add(struct adf_etr_ring_data *ring, const char *name)
    {
    struct adf_etr_ring_debug_entry *ring_debug;
    char entry_name[16];
    ring_debug = kzalloc_obj(*ring_debug);
    if (!ring_debug)
    return -ENOMEM;
    strscpy(ring_debug.ring_name, name);
    snprintf(entry_name, sizeof(entry_name), "ring_%02d",
    ring.ring_number);
    ring_debug.debug = debugfs_create_file(entry_name, S_IRUSR,
    ring.bank.bank_debug_dir,
    ring, &adf_ring_debug_fops);
    ring.ring_debug = ring_debug;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_ring_debugfs_rm(ring: *mut adf_etr_ring_data) {
    void adf_ring_debugfs_rm(struct adf_etr_ring_data *ring)
    {
    if (ring.ring_debug) {
    debugfs_remove(ring.ring_debug.debug);
    kfree(ring.ring_debug);
    ring.ring_debug = core::ptr::null_mut();
    }
    }
    static void *adf_bank_start(struct seq_file *sfile, loff_t *pos)
    {
    struct adf_etr_bank_data *bank = sfile.private;
    let mut num_rings_per_bank: u8 = GET_NUM_RINGS_PER_BANK(bank.accel_dev);
    mutex_lock(&bank_read_lock);
    if (*pos == 0)
    return SEQ_START_TOKEN;
    if (*pos >= num_rings_per_bank)
    return core::ptr::null_mut();
    return pos;
    }
    static void *adf_bank_next(struct seq_file *sfile, void *v, loff_t *pos)
    {
    struct adf_etr_bank_data *bank = sfile.private;
    let mut num_rings_per_bank: u8 = GET_NUM_RINGS_PER_BANK(bank.accel_dev);
    if (++(*pos) >= num_rings_per_bank)
    return core::ptr::null_mut();
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn adf_bank_show(sfile: *mut seq_file, v: *mut c_void) -> c_int {
    static int adf_bank_show(struct seq_file *sfile, void *v)
    {
    struct adf_etr_bank_data *bank = sfile.private;
    struct adf_hw_csr_ops *csr_ops = GET_CSR_OPS(bank.accel_dev);
    if (v == SEQ_START_TOKEN) {
    seq_printf(sfile, "------- Bank %d configuration -------\n",
    bank.bank_number);
    } else {
    let mut ring_id: c_int = *((int *)v) - 1;
    struct adf_etr_ring_data *ring = &bank.rings[ring_id];
    void __iomem *csr = bank.csr_addr;
    int head, tail, empty;
    if (!(bank.ring_mask & 1 << ring_id))
    return 0;
    head = csr_ops.read_csr_ring_head(csr, bank.bank_number,
    ring.ring_number);
    tail = csr_ops.read_csr_ring_tail(csr, bank.bank_number,
    ring.ring_number);
    empty = csr_ops.read_csr_e_stat(csr, bank.bank_number);
    seq_printf(sfile,
    "ring num %02d, head %04x, tail %04x, empty: %d\n",
    ring.ring_number, head, tail,
    (empty & 1 << ring.ring_number) >>
    ring.ring_number);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adf_bank_stop(sfile: *mut seq_file, v: *mut c_void) {
    static void adf_bank_stop(struct seq_file *sfile, void *v)
    {
    mutex_unlock(&bank_read_lock);
    }
    static const struct seq_operations adf_bank_debug_sops = {
    .start = adf_bank_start,
    .next = adf_bank_next,
    .stop = adf_bank_stop,
    .show = adf_bank_show
    };
    DEFINE_SEQ_ATTRIBUTE(adf_bank_debug);
#[no_mangle]
pub unsafe extern "C" fn adf_bank_debugfs_add(bank: *mut adf_etr_bank_data) -> c_int {
    int adf_bank_debugfs_add(struct adf_etr_bank_data *bank)
    {
    struct adf_accel_dev *accel_dev = bank.accel_dev;
    struct dentry *parent = accel_dev.transport.debug;
    char name[16];
    snprintf(name, sizeof(name), "bank_%02d", bank.bank_number);
    bank.bank_debug_dir = debugfs_create_dir(name, parent);
    bank.bank_debug_cfg = debugfs_create_file("config", S_IRUSR,
    bank.bank_debug_dir, bank,
    &adf_bank_debug_fops);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_bank_debugfs_rm(bank: *mut adf_etr_bank_data) {
    void adf_bank_debugfs_rm(struct adf_etr_bank_data *bank)
    {
    debugfs_remove(bank.bank_debug_cfg);
    debugfs_remove(bank.bank_debug_dir);
    }
