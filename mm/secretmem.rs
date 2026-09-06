//! Automatically rewritten from C to Rust
//! Source: mm/secretmem.c
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
// Copyright IBM Corporation, 2021
//
// Author: Mike Rapoport <rppt@linux.ibm.com>
//

//
// Define mode and flag masks to allow validation of the system call
// parameters.
//

    let mut __ro_after_init: static bool secretmem_enable = 1;
    module_param_named(enable, secretmem_enable, bool, 0400);
    MODULE_PARM_DESC(secretmem_enable,
    "Enable secretmem and memfd_secret(2) system call");
    static atomic_t secretmem_users;
#[no_mangle]
pub unsafe extern "C" fn secretmem_active() -> bool {
    bool secretmem_active(void)
    {
    return !!atomic_read(&secretmem_users);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secretmem_inode_state {
    pub user: *mut user_struct,
    pub nr_pages_accounted: atomic_long_t,
}

    static bool __secretmem_account_pages(struct user_struct *user,
    unsigned long nr_pages)
    {
    unsigned long page_limit, cur_pages, new_pages;
    if (!nr_pages)
    return true;
    page_limit = rlimit(RLIMIT_MEMLOCK) >> PAGE_SHIFT;
    cur_pages = atomic_long_read(&user.locked_vm);
    do {
    new_pages = cur_pages + nr_pages;
    if (new_pages > page_limit)
    return false;
    } while (!atomic_long_try_cmpxchg(&user.locked_vm,
    &cur_pages, new_pages));
    return true;
    }
    static bool secretmem_account_folio(struct secretmem_inode_state *state,
    const struct folio *folio)
    {
    let mut nr_pages: c_ulong = folio_nr_pages(folio);
    if (!__secretmem_account_pages(state.user, nr_pages))
    return false;
    atomic_long_add(nr_pages, &state.nr_pages_accounted);
    return true;
    }
    static void __secretmem_unaccount_pages(struct secretmem_inode_state *state,
    unsigned long nr_pages)
    {
    atomic_long_sub(nr_pages, &state.user.locked_vm);
    atomic_long_sub(nr_pages, &state.nr_pages_accounted);
    }
    static void secretmem_unaccount_folio(struct secretmem_inode_state *state,
    struct folio *folio)
    {
    __secretmem_unaccount_pages(state, folio_nr_pages(folio));
    }
#[no_mangle]
unsafe extern "C" fn secretmem_unaccount_all_folios(state: *mut secretmem_inode_state) {
    static void secretmem_unaccount_all_folios(struct secretmem_inode_state *state)
    {
    const unsigned long nr_pages_accounted =
    atomic_long_read(&state.nr_pages_accounted);
    __secretmem_unaccount_pages(state, nr_pages_accounted);
    }
#[no_mangle]
unsafe extern "C" fn secretmem_fault(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t secretmem_fault(struct vm_fault *vmf)
    {
    struct address_space *mapping = vmf.vma.vm_file.f_mapping;
    struct inode *inode = file_inode(vmf.vma.vm_file);
    struct secretmem_inode_state *state = inode.i_private;
    let mut offset: pgoff_t = vmf.pgoff;
    let mut gfp: gfp_t = vmf.gfp_mask;
    unsigned long addr;
    struct folio *folio;
    vm_fault_t ret;
    int err;
    if (((loff_t)vmf.pgoff << PAGE_SHIFT) >= i_size_read(inode))
    return vmf_error(-EINVAL);
    filemap_invalidate_lock_shared(mapping);
    retry:
    folio = filemap_lock_folio(mapping, offset);
    if (IS_ERR(folio)) {
    folio = folio_alloc(gfp | __GFP_ZERO, 0);
    if (!folio) {
    ret = VM_FAULT_OOM;
    goto out;
    }
    if (!secretmem_account_folio(state, folio)) {
    folio_put(folio);
    ret = VM_FAULT_SIGBUS;
    goto out;
    }
    err = set_direct_map_invalid_noflush(folio_page(folio, 0));
    if (err) {
    secretmem_unaccount_folio(state, folio);
    folio_put(folio);
    ret = vmf_error(err);
    goto out;
    }
    __folio_mark_uptodate(folio);
    err = filemap_add_folio(mapping, folio, offset, gfp);
    if (unlikely(err)) {
    secretmem_unaccount_folio(state, folio);
//
// If a split of large page was required, it
// already happened when we marked the page invalid
// which guarantees that this call won't fail
//
    set_direct_map_default_noflush(folio_page(folio, 0));
    folio_put(folio);
    if (err == -EEXIST)
    goto retry;
    ret = vmf_error(err);
    goto out;
    }
    addr = (unsigned long)folio_address(folio);
    flush_tlb_kernel_range(addr, addr + PAGE_SIZE);
    }
    vmf.page = folio_file_page(folio, vmf.pgoff);
    ret = VM_FAULT_LOCKED;
    out:
    filemap_invalidate_unlock_shared(mapping);
    return ret;
    }
    static const struct vm_operations_struct secretmem_vm_ops = {
    .fault = secretmem_fault,
    };
#[no_mangle]
unsafe extern "C" fn secretmem_destroy_inode_priv(inode: *mut inode) {
    static void secretmem_destroy_inode_priv(struct inode *inode)
    {
    struct secretmem_inode_state *state = inode.i_private;
    secretmem_unaccount_all_folios(state);
    free_uid(state.user);
    kfree(state);
    inode.i_private = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn secretmem_release(inode: *mut inode, file: *mut file) -> c_int {
    static int secretmem_release(struct inode *inode, struct file *file)
    {
    atomic_dec(&secretmem_users);
    secretmem_destroy_inode_priv(inode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn secretmem_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    static int secretmem_mmap_prepare(struct vm_area_desc *desc)
    {
    if (!vma_desc_test_any(desc, VMA_SHARED_BIT, VMA_MAYSHARE_BIT))
    return -EINVAL;
    vma_desc_set_flags(desc, VMA_DONTDUMP_BIT);
    desc.vm_ops = &secretmem_vm_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_is_secretmem(vma: *mut vm_area_struct) -> bool {
    bool vma_is_secretmem(struct vm_area_struct *vma)
    {
    return vma.vm_ops == &secretmem_vm_ops;
    }
    static const struct file_operations secretmem_fops = {
    .release	= secretmem_release,
    .mmap_prepare	= secretmem_mmap_prepare,
    };
    static int secretmem_migrate_folio(struct address_space *mapping,
    struct folio *dst, struct folio *src, enum migrate_mode mode)
    {
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn secretmem_free_folio(folio: *mut folio) {
    static void secretmem_free_folio(struct folio *folio)
    {
    set_direct_map_default_noflush(folio_page(folio, 0));
    folio_zero_segment(folio, 0, folio_size(folio));
    }
    const struct address_space_operations secretmem_aops = {
    .dirty_folio	= noop_dirty_folio,
    .free_folio	= secretmem_free_folio,
    .migrate_folio	= secretmem_migrate_folio,
    };
    static int secretmem_setattr(struct mnt_idmap *idmap,
    struct dentry *dentry, struct iattr *iattr)
    {
    struct inode *inode = d_inode(dentry);
    struct address_space *mapping = inode.i_mapping;
    let mut ia_valid: c_uint = iattr.ia_valid;
    int ret;
    filemap_invalidate_lock(mapping);
    if ((ia_valid & ATTR_SIZE) && inode.i_size)
    ret = -EINVAL;
    else
    ret = simple_setattr(idmap, dentry, iattr);
    filemap_invalidate_unlock(mapping);
    return ret;
    }
    static const struct inode_operations secretmem_iops = {
    .setattr = secretmem_setattr,
    };
    static struct vfsmount *secretmem_mnt;
#[no_mangle]
unsafe extern "C" fn secretmem_init_inode_priv(inode: *mut inode) -> c_int {
    static int secretmem_init_inode_priv(struct inode *inode)
    {
    struct secretmem_inode_state *state;
    state = kzalloc_obj(*state);
    if (!state)
    return -ENOMEM;
    state.user = get_uid(current_user());
    inode.i_private = state;
    return 0;
    }
    static struct file *secretmem_file_create(unsigned long flags)
    {
    struct file *file;
    struct inode *inode;
    const char *anon_name = "[secretmem]";
    int err;
    inode = anon_inode_make_secure_inode(secretmem_mnt.mnt_sb, anon_name, core::ptr::null_mut());
    if (IS_ERR(inode))
    return ERR_CAST(inode);
    err = secretmem_init_inode_priv(inode);
    if (err)
    goto err_free_inode;
    file = alloc_file_pseudo(inode, secretmem_mnt, "secretmem",
    O_RDWR | O_LARGEFILE, &secretmem_fops);
    if (IS_ERR(file)) {
    err = PTR_ERR(file);
    goto err_free_priv;
    }
    mapping_set_gfp_mask(inode.i_mapping, GFP_USER);
    mapping_set_unevictable(inode.i_mapping);
    inode.i_op = &secretmem_iops;
    inode.i_mapping.a_ops = &secretmem_aops;
// pretend we are a normal file with zero size
    inode.i_mode |= S_IFREG;
    inode.i_size = 0;
    atomic_inc(&secretmem_users);
    return file;
    err_free_priv:
    secretmem_destroy_inode_priv(inode);
    err_free_inode:
    iput(inode);
    return ERR_PTR(err);
    }
    SYSCALL_DEFINE1(memfd_secret, unsigned int, flags)
    {
// make sure local flags do not conflict with global fcntl.h
    BUILD_BUG_ON(SECRETMEM_FLAGS_MASK & O_CLOEXEC);
    if (!secretmem_enable || !can_set_direct_map())
    return -ENOSYS;
    if (flags & ~(SECRETMEM_FLAGS_MASK | O_CLOEXEC))
    return -EINVAL;
    if (atomic_read(&secretmem_users) < 0)
    return -ENFILE;
    return FD_ADD(flags & O_CLOEXEC, secretmem_file_create(flags));
    }
#[no_mangle]
unsafe extern "C" fn secretmem_init_fs_context(fc: *mut fs_context) -> c_int {
    static int secretmem_init_fs_context(struct fs_context *fc)
    {
    struct pseudo_fs_context *ctx;
    ctx = init_pseudo(fc, SECRETMEM_MAGIC);
    if (!ctx)
    return -ENOMEM;
    return 0;
    }
    static struct file_system_type secretmem_fs = {
    .name		= "secretmem",
    .init_fs_context = secretmem_init_fs_context,
    .kill_sb	= kill_anon_super,
    };
#[no_mangle]
unsafe extern "C" fn secretmem_init() -> int __init {
    static int __init secretmem_init(void)
    {
    if (!secretmem_enable || !can_set_direct_map())
    return 0;
    secretmem_mnt = kern_mount(&secretmem_fs);
    if (IS_ERR(secretmem_mnt))
    return PTR_ERR(secretmem_mnt);
    return 0;
    }
    fs_initcall(secretmem_init);
