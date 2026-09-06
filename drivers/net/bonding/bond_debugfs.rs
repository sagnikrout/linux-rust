//! Automatically rewritten from C to Rust
//! Source: drivers/net/bonding/bond_debugfs.c
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

    static struct dentry *bonding_debug_root;
// Show RLB hash table
#[no_mangle]
unsafe extern "C" fn bond_debug_rlb_hash_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int bond_debug_rlb_hash_show(struct seq_file *m, void *v)
    {
    struct bonding *bond = m.private;
    struct alb_bond_info *bond_info = &(BOND_ALB_INFO(bond));
    struct rlb_client_info *client_info;
    u32 hash_index;
    if (BOND_MODE(bond) != BOND_MODE_ALB)
    return 0;
    seq_printf(m, "SourceIP        DestinationIP   "
    "Destination MAC   DEV\n");
    spin_lock_bh(&bond.mode_lock);
    hash_index = bond_info.rx_hashtbl_used_head;
    for (; hash_index != RLB_NULL_INDEX;
    hash_index = client_info.used_next) {
    client_info = &(bond_info.rx_hashtbl[hash_index]);
    if (client_info.slave)
    seq_printf(m, "%-15pI4 %-15pI4 %-17pM %s\n",
    &client_info.ip_src,
    &client_info.ip_dst,
    &client_info.mac_dst,
    client_info.slave.dev.name);
    else
    seq_printf(m, "%-15pI4 %-15pI4 %-17pM (none)\n",
    &client_info.ip_src,
    &client_info.ip_dst,
    &client_info.mac_dst);
    }
    spin_unlock_bh(&bond.mode_lock);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(bond_debug_rlb_hash);
#[no_mangle]
pub unsafe extern "C" fn bond_debug_register(bond: *mut bonding) {
    void bond_debug_register(struct bonding *bond)
    {
    bond.debug_dir =
    debugfs_create_dir(bond.dev.name, bonding_debug_root);
    debugfs_create_file("rlb_hash_table", 0400, bond.debug_dir,
    bond, &bond_debug_rlb_hash_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn bond_debug_unregister(bond: *mut bonding) {
    void bond_debug_unregister(struct bonding *bond)
    {
    debugfs_remove_recursive(bond.debug_dir);
    }
#[no_mangle]
pub unsafe extern "C" fn bond_debug_reregister(bond: *mut bonding) {
    void bond_debug_reregister(struct bonding *bond)
    {
    let mut err: c_int = debugfs_change_name(bond.debug_dir, "%s", bond.dev.name);
    if (err) {
    netdev_warn(bond.dev, "failed to reregister, so just unregister old one\n");
    bond_debug_unregister(bond);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bond_create_debugfs() -> void __init {
    void __init bond_create_debugfs(void)
    {
    bonding_debug_root = debugfs_create_dir("bonding", core::ptr::null_mut());
    if (IS_ERR(bonding_debug_root))
    pr_warn("Warning: Cannot create bonding directory in debugfs\n");
    }
#[no_mangle]
pub unsafe extern "C" fn bond_destroy_debugfs() {
    void bond_destroy_debugfs(void)
    {
    debugfs_remove_recursive(bonding_debug_root);
    bonding_debug_root = core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn bond_debug_register(bond: *mut bonding) {
    void bond_debug_register(struct bonding *bond)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bond_debug_unregister(bond: *mut bonding) {
    void bond_debug_unregister(struct bonding *bond)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bond_debug_reregister(bond: *mut bonding) {
    void bond_debug_reregister(struct bonding *bond)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bond_create_debugfs() -> void __init {
    void __init bond_create_debugfs(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bond_destroy_debugfs() {
    void bond_destroy_debugfs(void)
    {
    }
