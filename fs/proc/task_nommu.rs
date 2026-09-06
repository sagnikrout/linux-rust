//! Automatically rewritten from C to Rust
//! Source: fs/proc/task_nommu.c
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
// Logic: we've got two memory sums for each process, "shared", and
// "non-shared". Shared memory may get counted more than once, for
// each process that owns it. Non-shared memory is counted
// accurately.
//
#[no_mangle]
pub unsafe extern "C" fn task_mem(m: *mut seq_file, mm: *mut mm_struct) {
    void task_mem(struct seq_file *m, struct mm_struct *mm)
    {
    VMA_ITERATOR(vmi, mm, 0);
    struct vm_area_struct *vma;
    struct vm_region *region;
    let mut bytes: c_ulong = 0, sbytes = 0, slack = 0, size;
    mmap_read_lock(mm);
    for_each_vma(vmi, vma) {
    bytes += kobjsize(vma);
    region = vma.vm_region;
    if (region) {
    size = kobjsize(region);
    size += region.vm_end - region.vm_start;
    } else {
    size = vma.vm_end - vma.vm_start;
    }
    if (atomic_read(&mm.mm_count) > 1 ||
    is_nommu_shared_mapping(vma.vm_flags)) {
    sbytes += size;
    } else {
    bytes += size;
    if (region)
    slack = region.vm_end - vma.vm_end;
    }
    }
    if (atomic_read(&mm.mm_count) > 1)
    sbytes += kobjsize(mm);
    else
    bytes += kobjsize(mm);
    if (current.fs && current.fs.users > 1)
    sbytes += kobjsize(current.fs);
    else
    bytes += kobjsize(current.fs);
    if (current.files && atomic_read(&current.files.count) > 1)
    sbytes += kobjsize(current.files);
    else
    bytes += kobjsize(current.files);
    if (current.sighand && refcount_read(&current.sighand.count) > 1)
    sbytes += kobjsize(current.sighand);
    else
    bytes += kobjsize(current.sighand);
    bytes += kobjsize(current); /* includes kernel stack */
    mmap_read_unlock(mm);
    seq_printf(m,
    "Mem:\t%8lu bytes\n"
    "Slack:\t%8lu bytes\n"
    "Shared:\t%8lu bytes\n",
    bytes, slack, sbytes);
    }
#[no_mangle]
pub unsafe extern "C" fn task_vsize(mm: *mut mm_struct) -> c_ulong {
    unsigned long task_vsize(struct mm_struct *mm)
    {
    VMA_ITERATOR(vmi, mm, 0);
    struct vm_area_struct *vma;
    let mut vsize: c_ulong = 0;
    mmap_read_lock(mm);
    for_each_vma(vmi, vma)
    vsize += vma.vm_end - vma.vm_start;
    mmap_read_unlock(mm);
    return vsize;
    }
    unsigned long task_statm(struct mm_struct *mm,
    unsigned long *shared, unsigned long *text,
    unsigned long *data, unsigned long *resident)
    {
    VMA_ITERATOR(vmi, mm, 0);
    struct vm_area_struct *vma;
    struct vm_region *region;
    let mut size: c_ulong = kobjsize(mm);
    mmap_read_lock(mm);
    for_each_vma(vmi, vma) {
    size += kobjsize(vma);
    region = vma.vm_region;
    if (region) {
    size += kobjsize(region);
    size += region.vm_end - region.vm_start;
    }
    }
// text = (PAGE_ALIGN(mm->end_code) - (mm->start_code & PAGE_MASK))
    >> PAGE_SHIFT;
// data = (PAGE_ALIGN(mm->start_stack) - (mm->start_data & PAGE_MASK))
    >> PAGE_SHIFT;
    mmap_read_unlock(mm);
    size >>= PAGE_SHIFT;
    size += *text + *data;
// resident = size;
    return size;
    }
//
// display a single VMA to a sequenced file
//
#[no_mangle]
unsafe extern "C" fn nommu_vma_show(m: *mut seq_file, vma: *mut vm_area_struct) -> c_int {
    static int nommu_vma_show(struct seq_file *m, struct vm_area_struct *vma)
    {
    struct mm_struct *mm = vma.vm_mm;
    let mut ino: c_ulong = 0;
    struct file *file;
    let mut dev: dev_t = 0;
    int flags;
    let mut pgoff: c_ulonglong = 0;
    flags = vma.vm_flags;
    file = vma.vm_file;
    if (file) {
    struct inode *inode = file_inode(vma.vm_file);
    dev = inode.i_sb.s_dev;
    ino = inode.i_ino;
    pgoff = (loff_t)vma.vm_pgoff << PAGE_SHIFT;
    }
    seq_setwidth(m, 25 + sizeof(void *) * 6 - 1);
    seq_printf(m,
    "%08lx-%08lx %c%c%c%c %08llx %02x:%02x %lu ",
    vma.vm_start,
    vma.vm_end,
    flags & VM_READ ? 'r' : '-',
    flags & VM_WRITE ? 'w' : '-',
    flags & VM_EXEC ? 'x' : '-',
    flags & VM_MAYSHARE ? flags & VM_SHARED ? 'S' : 's' : 'p',
    pgoff,
    MAJOR(dev), MINOR(dev), ino);
    if (file) {
    seq_pad(m, ' ');
    seq_path(m, file_user_path(file), "");
    } else if (mm && vma_is_initial_stack(vma)) {
    seq_pad(m, ' ');
    seq_puts(m, "[stack]");
    }
    seq_putc(m, '\n');
    return 0;
    }
//
// display mapping lines for a particular process's /proc/pid/maps
//
#[no_mangle]
unsafe extern "C" fn show_map(m: *mut seq_file, _p: *mut c_void) -> c_int {
    static int show_map(struct seq_file *m, void *_p)
    {
    return nommu_vma_show(m, _p);
    }
    static struct vm_area_struct *proc_get_vma(struct proc_maps_private *priv,
    loff_t *ppos)
    {
    struct vm_area_struct *vma = vma_next(&priv.iter);
    if (vma) {
// ppos = vma->vm_start;
    } else {
// ppos = -1UL;
    }
    return vma;
    }
    static void *m_start(struct seq_file *m, loff_t *ppos)
    {
    struct proc_maps_private *priv = m.private;
    let mut last_addr: c_ulong = *ppos;
    struct mm_struct *mm;
// See proc_get_vma(). Zero at the start or after lseek.
    if (last_addr == -1UL)
    return core::ptr::null_mut();
// pin the task and mm whilst we play with them
    priv.task = get_proc_task(priv.inode);
    if (!priv.task)
    return ERR_PTR(-ESRCH);
    mm = priv.lock_ctx.mm;
    if (!mm || !mmget_not_zero(mm)) {
    put_task_struct(priv.task);
    priv.task = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    if (mmap_read_lock_killable(mm)) {
    mmput(mm);
    put_task_struct(priv.task);
    priv.task = core::ptr::null_mut();
    return ERR_PTR(-EINTR);
    }
    vma_iter_init(&priv.iter, mm, last_addr);
    return proc_get_vma(priv, ppos);
    }
#[no_mangle]
unsafe extern "C" fn m_stop(m: *mut seq_file, v: *mut c_void) {
    static void m_stop(struct seq_file *m, void *v)
    {
    struct proc_maps_private *priv = m.private;
    struct mm_struct *mm = priv.lock_ctx.mm;
    if (!priv.task)
    return;
    mmap_read_unlock(mm);
    mmput(mm);
    put_task_struct(priv.task);
    priv.task = core::ptr::null_mut();
    }
    static void *m_next(struct seq_file *m, void *_p, loff_t *ppos)
    {
    return proc_get_vma(m.private, ppos);
    }
    static const struct seq_operations proc_pid_maps_ops = {
    .start	= m_start,
    .next	= m_next,
    .stop	= m_stop,
    .show	= show_map
    };
    static int maps_open(struct inode *inode, struct file *file,
    const struct seq_operations *ops)
    {
    struct proc_maps_private *priv;
    priv = __seq_open_private(file, ops, sizeof(*priv));
    if (!priv)
    return -ENOMEM;
    priv.inode = inode;
    priv.lock_ctx.mm = proc_mem_open(inode, PTRACE_MODE_READ);
    if (IS_ERR_OR_NULL(priv.lock_ctx.mm)) {
    let mut err: c_int = priv.lock_ctx.mm ? PTR_ERR(priv.lock_ctx.mm) : -ESRCH;
    seq_release_private(inode, file);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_release(inode: *mut inode, file: *mut file) -> c_int {
    static int map_release(struct inode *inode, struct file *file)
    {
    struct seq_file *seq = file.private_data;
    struct proc_maps_private *priv = seq.private;
    if (priv.lock_ctx.mm)
    mmdrop(priv.lock_ctx.mm);
    return seq_release_private(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn pid_maps_open(inode: *mut inode, file: *mut file) -> c_int {
    static int pid_maps_open(struct inode *inode, struct file *file)
    {
    return maps_open(inode, file, &proc_pid_maps_ops);
    }
    const struct file_operations proc_pid_maps_operations = {
    .open		= pid_maps_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .release	= map_release,
    };
