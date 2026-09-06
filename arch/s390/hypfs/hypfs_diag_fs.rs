//! Automatically rewritten from C to Rust
//! Source: arch/s390/hypfs/hypfs_diag_fs.c
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
// Hypervisor filesystem for Linux on s390. Diag 204 and 224
// implementation.
//
// Copyright IBM Corp. 2006, 2008
// Author(s): Michael Holzheu <holzheu@de.ibm.com>
//

    static char *diag224_cpu_names;			/* diag 224 name table */
    static int diag224_idx2name(int index, char *name);
//
// DIAG 204 member access functions.
//
// Since we have two different diag 204 data formats for old and new s390
// machines, we do not access the structs directly, but use getter functions for
// each struct member instead. This should make the code more readable.
//
// Time information block
#[no_mangle]
pub unsafe extern "C" fn info_blk_hdr__size(type: enum diag204_format) -> c_int {
    static inline int info_blk_hdr__size(enum diag204_format type)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return sizeof(struct diag204_info_blk_hdr);
    else /* DIAG204_INFO_EXT */
    return sizeof(struct diag204_x_info_blk_hdr);
    }
#[no_mangle]
pub unsafe extern "C" fn info_blk_hdr__npar(type: enum diag204_format, hdr: *mut c_void) -> __u8 {
    static inline __u8 info_blk_hdr__npar(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_info_blk_hdr *)hdr).npar;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_info_blk_hdr *)hdr).npar;
    }
#[no_mangle]
pub unsafe extern "C" fn info_blk_hdr__flags(type: enum diag204_format, hdr: *mut c_void) -> __u8 {
    static inline __u8 info_blk_hdr__flags(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_info_blk_hdr *)hdr).flags;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_info_blk_hdr *)hdr).flags;
    }
// Partition header
#[no_mangle]
pub unsafe extern "C" fn part_hdr__size(type: enum diag204_format) -> c_int {
    static inline int part_hdr__size(enum diag204_format type)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return sizeof(struct diag204_part_hdr);
    else /* DIAG204_INFO_EXT */
    return sizeof(struct diag204_x_part_hdr);
    }
#[no_mangle]
pub unsafe extern "C" fn part_hdr__rcpus(type: enum diag204_format, hdr: *mut c_void) -> __u8 {
    static inline __u8 part_hdr__rcpus(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_part_hdr *)hdr).cpus;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_part_hdr *)hdr).rcpus;
    }
    static inline void part_hdr__part_name(enum diag204_format type, void *hdr,
    char *name)
    {
    if (type == DIAG204_INFO_SIMPLE)
    memcpy(name, ((struct diag204_part_hdr *)hdr).part_name,
    DIAG204_LPAR_NAME_LEN);
    else /* DIAG204_INFO_EXT */
    memcpy(name, ((struct diag204_x_part_hdr *)hdr).part_name,
    DIAG204_LPAR_NAME_LEN);
    EBCASC(name, DIAG204_LPAR_NAME_LEN);
    name[DIAG204_LPAR_NAME_LEN] = 0;
    strim(name);
    }
// CPU info block
#[no_mangle]
pub unsafe extern "C" fn cpu_info__size(type: enum diag204_format) -> c_int {
    static inline int cpu_info__size(enum diag204_format type)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return sizeof(struct diag204_cpu_info);
    else /* DIAG204_INFO_EXT */
    return sizeof(struct diag204_x_cpu_info);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_info__ctidx(type: enum diag204_format, hdr: *mut c_void) -> __u8 {
    static inline __u8 cpu_info__ctidx(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_cpu_info *)hdr).ctidx;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_cpu_info *)hdr).ctidx;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_info__cpu_addr(type: enum diag204_format, hdr: *mut c_void) -> __u16 {
    static inline __u16 cpu_info__cpu_addr(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_cpu_info *)hdr).cpu_addr;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_cpu_info *)hdr).cpu_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_info__acc_time(type: enum diag204_format, hdr: *mut c_void) -> __u64 {
    static inline __u64 cpu_info__acc_time(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_cpu_info *)hdr).acc_time;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_cpu_info *)hdr).acc_time;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_info__lp_time(type: enum diag204_format, hdr: *mut c_void) -> __u64 {
    static inline __u64 cpu_info__lp_time(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_cpu_info *)hdr).lp_time;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_cpu_info *)hdr).lp_time;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_info__online_time(type: enum diag204_format, hdr: *mut c_void) -> __u64 {
    static inline __u64 cpu_info__online_time(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return 0;	/* online_time not available in simple info */
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_cpu_info *)hdr).online_time;
    }
// Physical header
#[no_mangle]
pub unsafe extern "C" fn phys_hdr__size(type: enum diag204_format) -> c_int {
    static inline int phys_hdr__size(enum diag204_format type)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return sizeof(struct diag204_phys_hdr);
    else /* DIAG204_INFO_EXT */
    return sizeof(struct diag204_x_phys_hdr);
    }
#[no_mangle]
pub unsafe extern "C" fn phys_hdr__cpus(type: enum diag204_format, hdr: *mut c_void) -> __u8 {
    static inline __u8 phys_hdr__cpus(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_phys_hdr *)hdr).cpus;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_phys_hdr *)hdr).cpus;
    }
// Physical CPU info block
#[no_mangle]
pub unsafe extern "C" fn phys_cpu__size(type: enum diag204_format) -> c_int {
    static inline int phys_cpu__size(enum diag204_format type)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return sizeof(struct diag204_phys_cpu);
    else /* DIAG204_INFO_EXT */
    return sizeof(struct diag204_x_phys_cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn phys_cpu__cpu_addr(type: enum diag204_format, hdr: *mut c_void) -> __u16 {
    static inline __u16 phys_cpu__cpu_addr(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_phys_cpu *)hdr).cpu_addr;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_phys_cpu *)hdr).cpu_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn phys_cpu__mgm_time(type: enum diag204_format, hdr: *mut c_void) -> __u64 {
    static inline __u64 phys_cpu__mgm_time(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_phys_cpu *)hdr).mgm_time;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_phys_cpu *)hdr).mgm_time;
    }
#[no_mangle]
pub unsafe extern "C" fn phys_cpu__ctidx(type: enum diag204_format, hdr: *mut c_void) -> __u64 {
    static inline __u64 phys_cpu__ctidx(enum diag204_format type, void *hdr)
    {
    if (type == DIAG204_INFO_SIMPLE)
    return ((struct diag204_phys_cpu *)hdr).ctidx;
    else /* DIAG204_INFO_EXT */
    return ((struct diag204_x_phys_cpu *)hdr).ctidx;
    }
//
// Functions to create the directory structure
//
#[no_mangle]
unsafe extern "C" fn hypfs_create_cpu_files(cpus_dir: *mut dentry, cpu_info: *mut c_void) -> c_int {
    static int hypfs_create_cpu_files(struct dentry *cpus_dir, void *cpu_info)
    {
    struct dentry *cpu_dir;
    char buffer[TMP_SIZE];
    int rc;
    snprintf(buffer, TMP_SIZE, "%d", cpu_info__cpu_addr(diag204_get_info_type(),
    cpu_info));
    cpu_dir = hypfs_mkdir(cpus_dir, buffer);
    if (IS_ERR(cpu_dir))
    return PTR_ERR(cpu_dir);
    rc = hypfs_create_u64(cpu_dir, "mgmtime",
    cpu_info__acc_time(diag204_get_info_type(), cpu_info) -
    cpu_info__lp_time(diag204_get_info_type(), cpu_info));
    if (rc)
    return rc;
    rc = hypfs_create_u64(cpu_dir, "cputime",
    cpu_info__lp_time(diag204_get_info_type(), cpu_info));
    if (rc)
    return rc;
    if (diag204_get_info_type() == DIAG204_INFO_EXT) {
    rc = hypfs_create_u64(cpu_dir, "onlinetime",
    cpu_info__online_time(diag204_get_info_type(),
    cpu_info));
    if (rc)
    return rc;
    }
    diag224_idx2name(cpu_info__ctidx(diag204_get_info_type(), cpu_info), buffer);
    return hypfs_create_str(cpu_dir, "type", buffer);
    }
    static void *hypfs_create_lpar_files(struct dentry *systems_dir, void *part_hdr)
    {
    struct dentry *cpus_dir;
    struct dentry *lpar_dir;
    char lpar_name[DIAG204_LPAR_NAME_LEN + 1];
    void *cpu_info;
    int i;
    part_hdr__part_name(diag204_get_info_type(), part_hdr, lpar_name);
    lpar_name[DIAG204_LPAR_NAME_LEN] = 0;
    lpar_dir = hypfs_mkdir(systems_dir, lpar_name);
    if (IS_ERR(lpar_dir))
    return lpar_dir;
    cpus_dir = hypfs_mkdir(lpar_dir, "cpus");
    if (IS_ERR(cpus_dir))
    return cpus_dir;
    cpu_info = part_hdr + part_hdr__size(diag204_get_info_type());
    for (i = 0; i < part_hdr__rcpus(diag204_get_info_type(), part_hdr); i++) {
    int rc;
    rc = hypfs_create_cpu_files(cpus_dir, cpu_info);
    if (rc)
    return ERR_PTR(rc);
    cpu_info += cpu_info__size(diag204_get_info_type());
    }
    return cpu_info;
    }
#[no_mangle]
unsafe extern "C" fn hypfs_create_phys_cpu_files(cpus_dir: *mut dentry, cpu_info: *mut c_void) -> c_int {
    static int hypfs_create_phys_cpu_files(struct dentry *cpus_dir, void *cpu_info)
    {
    struct dentry *cpu_dir;
    char buffer[TMP_SIZE];
    int rc;
    snprintf(buffer, TMP_SIZE, "%i", phys_cpu__cpu_addr(diag204_get_info_type(),
    cpu_info));
    cpu_dir = hypfs_mkdir(cpus_dir, buffer);
    if (IS_ERR(cpu_dir))
    return PTR_ERR(cpu_dir);
    rc = hypfs_create_u64(cpu_dir, "mgmtime",
    phys_cpu__mgm_time(diag204_get_info_type(), cpu_info));
    if (rc)
    return rc;
    diag224_idx2name(phys_cpu__ctidx(diag204_get_info_type(), cpu_info), buffer);
    return hypfs_create_str(cpu_dir, "type", buffer);
    }
    static void *hypfs_create_phys_files(struct dentry *parent_dir, void *phys_hdr)
    {
    int i;
    void *cpu_info;
    struct dentry *cpus_dir;
    cpus_dir = hypfs_mkdir(parent_dir, "cpus");
    if (IS_ERR(cpus_dir))
    return cpus_dir;
    cpu_info = phys_hdr + phys_hdr__size(diag204_get_info_type());
    for (i = 0; i < phys_hdr__cpus(diag204_get_info_type(), phys_hdr); i++) {
    int rc;
    rc = hypfs_create_phys_cpu_files(cpus_dir, cpu_info);
    if (rc)
    return ERR_PTR(rc);
    cpu_info += phys_cpu__size(diag204_get_info_type());
    }
    return cpu_info;
    }
#[no_mangle]
pub unsafe extern "C" fn hypfs_diag_create_files(root: *mut dentry) -> c_int {
    int hypfs_diag_create_files(struct dentry *root)
    {
    struct dentry *systems_dir, *hyp_dir;
    void *time_hdr, *part_hdr;
    void *buffer, *ptr;
    int i, rc, pages;
    buffer = diag204_get_buffer(diag204_get_info_type(), &pages);
    if (IS_ERR(buffer))
    return PTR_ERR(buffer);
    rc = diag204_store(buffer, pages);
    if (rc)
    return rc;
    systems_dir = hypfs_mkdir(root, "systems");
    if (IS_ERR(systems_dir))
    return PTR_ERR(systems_dir);
    time_hdr = (struct x_info_blk_hdr *)buffer;
    part_hdr = time_hdr + info_blk_hdr__size(diag204_get_info_type());
    for (i = 0; i < info_blk_hdr__npar(diag204_get_info_type(), time_hdr); i++) {
    part_hdr = hypfs_create_lpar_files(systems_dir, part_hdr);
    if (IS_ERR(part_hdr))
    return PTR_ERR(part_hdr);
    }
    if (info_blk_hdr__flags(diag204_get_info_type(), time_hdr) &
    DIAG204_LPAR_PHYS_FLG) {
    ptr = hypfs_create_phys_files(root, part_hdr);
    if (IS_ERR(ptr))
    return PTR_ERR(ptr);
    }
    hyp_dir = hypfs_mkdir(root, "hyp");
    if (IS_ERR(hyp_dir))
    return PTR_ERR(hyp_dir);
    return hypfs_create_str(hyp_dir, "type", "LPAR Hypervisor");
    }
// Diagnose 224 functions
#[no_mangle]
unsafe extern "C" fn diag224_idx2name(index: c_int, name: *mut c_char) -> c_int {
    static int diag224_idx2name(int index, char *name)
    {
    memcpy(name, diag224_cpu_names + ((index + 1) * DIAG204_CPU_NAME_LEN),
    DIAG204_CPU_NAME_LEN);
    name[DIAG204_CPU_NAME_LEN] = 0;
    strim(name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn diag224_get_name_table() -> c_int {
    static int diag224_get_name_table(void)
    {
// memory must be below 2GB
    diag224_cpu_names = (char *)__get_free_page(GFP_KERNEL | GFP_DMA);
    if (!diag224_cpu_names)
    return -ENOMEM;
    if (diag224(diag224_cpu_names)) {
    free_page((unsigned long)diag224_cpu_names);
    return -EOPNOTSUPP;
    }
    EBCASC(diag224_cpu_names + 16, (*diag224_cpu_names + 1) * 16);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn diag224_delete_name_table() {
    static void diag224_delete_name_table(void)
    {
    free_page((unsigned long)diag224_cpu_names);
    }
#[no_mangle]
pub unsafe extern "C" fn __hypfs_diag_fs_init() -> int __init {
    int __init __hypfs_diag_fs_init(void)
    {
    if (machine_is_lpar())
    return diag224_get_name_table();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __hypfs_diag_fs_exit() {
    void __hypfs_diag_fs_exit(void)
    {
    diag224_delete_name_table();
    }
