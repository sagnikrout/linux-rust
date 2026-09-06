//! Automatically rewritten from C to Rust
//! Source: mm/alloc_tag.c
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

    let mut mem_profiling_support: static bool = true;

    static bool mem_profiling_support;

//
// Memory allocation profiling is permanently disabled and cannot be enabled.
// Must be called after setup_early_mem_profiling().
//
#[no_mangle]
pub unsafe extern "C" fn mem_alloc_profiling_permanently_disabled() -> bool {
    bool mem_alloc_profiling_permanently_disabled(void)
    {
    return !mem_profiling_support;
    }
    static struct codetag_type *alloc_tag_cttype;

    DEFINE_PER_CPU(struct alloc_tag_counters, _shared_alloc_tag);
    EXPORT_SYMBOL(_shared_alloc_tag);

    DEFINE_STATIC_KEY_MAYBE(CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT,
    mem_alloc_profiling_key);
    EXPORT_SYMBOL(mem_alloc_profiling_key);
    DEFINE_STATIC_KEY_FALSE(mem_profiling_compressed);
    let mut kernel_tags: alloc_tag_kernel_section = { core::ptr::null_mut(), 0 };
    unsigned long alloc_tag_ref_mask;
    int alloc_tag_ref_offs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_private {
    pub iter: codetag_iterator,
    pub reported_iter: codetag_iterator,
    pub print_header: bool,
    pub filter: allocinfo_filter,
// ioctl uses a separate iterator not to interfere with reads
    pub ioctl_iter: codetag_iterator,
    pub /: *mut *mut bool positioned; / seq_open_private() sets to 0,
    pub ioctl_lock: mutex,
}

    static void *allocinfo_start(struct seq_file *m, loff_t *pos)
    {
    struct allocinfo_private *priv;
    let mut node: loff_t = *pos;
    priv = (struct allocinfo_private *)m.private;
    codetag_lock_module_list(alloc_tag_cttype);
    if (node == 0) {
    priv.print_header = true;
    priv.iter = codetag_get_ct_iter(alloc_tag_cttype);
    } else {
    priv.iter = priv.reported_iter;
    }
    codetag_next_ct(&priv.iter);
    return priv.iter.ct ? priv : core::ptr::null_mut();
    }
    static void *allocinfo_next(struct seq_file *m, void *arg, loff_t *pos)
    {
    struct allocinfo_private *priv = (struct allocinfo_private *)arg;
    struct codetag *ct;
    priv.reported_iter = priv.iter;
    ct = codetag_next_ct(&priv.iter);
    (*pos)++;
    if (!ct)
    return core::ptr::null_mut();
    return priv;
    }
#[no_mangle]
unsafe extern "C" fn allocinfo_stop(m: *mut seq_file, arg: *mut c_void) {
    static void allocinfo_stop(struct seq_file *m, void *arg)
    {
    codetag_unlock_module_list(alloc_tag_cttype);
    }
#[no_mangle]
unsafe extern "C" fn print_allocinfo_header(buf: *mut seq_buf) {
    static void print_allocinfo_header(struct seq_buf *buf)
    {
// Output format version, so we can change it.
    seq_buf_printf(buf, "allocinfo - version: 2.0\n");
    seq_buf_printf(buf, "#     <size>  <calls> <tag info>\n");
    }
#[no_mangle]
unsafe extern "C" fn alloc_tag_to_text(out: *mut seq_buf, ct: *mut codetag) {
    static void alloc_tag_to_text(struct seq_buf *out, struct codetag *ct)
    {
    struct alloc_tag *tag = ct_to_alloc_tag(ct);
    let mut counter: alloc_tag_counters = alloc_tag_read(tag);
    let mut bytes: i64 = counter.bytes;
    seq_buf_printf(out, "%12lli %8llu ", bytes, counter.calls);
    codetag_to_text(out, ct);
    if (unlikely(alloc_tag_is_inaccurate(tag)))
    seq_buf_printf(out, " accurate:no");
    seq_buf_putc(out, ' ');
    seq_buf_putc(out, '\n');
    }
#[no_mangle]
unsafe extern "C" fn allocinfo_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    static int allocinfo_show(struct seq_file *m, void *arg)
    {
    struct allocinfo_private *priv = (struct allocinfo_private *)arg;
    char *bufp;
    let mut n: usize = seq_get_buf(m, &bufp);
    struct seq_buf buf;
    seq_buf_init(&buf, bufp, n);
    if (priv.print_header) {
    print_allocinfo_header(&buf);
    priv.print_header = false;
    }
    alloc_tag_to_text(&buf, priv.iter.ct);
    seq_commit(m, seq_buf_used(&buf));
    return 0;
    }
    static const struct seq_operations allocinfo_seq_op = {
    .start	= allocinfo_start,
    .next	= allocinfo_next,
    .stop	= allocinfo_stop,
    .show	= allocinfo_show,
    };
//
// Initializes seq_file operations and allocates private state when opening
// the /proc/allocinfo procfs entry.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_open(inode: *mut inode, file: *mut file) -> c_int {
    static int allocinfo_open(struct inode *inode, struct file *file)
    {
    int ret;
    ret = seq_open_private(file, &allocinfo_seq_op,
    sizeof(struct allocinfo_private));
    if (!ret) {
    struct seq_file *m = file.private_data;
    struct allocinfo_private *priv = m.private;
    mutex_init(&priv.ioctl_lock);
    }
    return ret;
    }
//
// Cleans up the seq_file state and frees up the private state allocated in
// allocinfo_open() when closing the /proc/allocinfo file descriptor.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_release(inode: *mut inode, file: *mut file) -> c_int {
    static int allocinfo_release(struct inode *inode, struct file *file)
    {
    struct seq_file *m = file.private_data;
    struct allocinfo_private *priv = m.private;
    mutex_destroy(&priv.ioctl_lock);
    return seq_release_private(inode, file);
    }
//
// Returns a pointer to the suffix of a string so that its length fits within
// ALLOCINFO_STR_SIZE, preserving the trailing characters.
// Function, file and module names often have the same prefixes, therefore
// when filtering by these criteria, we compare the last 64 characters to
// minimize the chances of name collisions
//
    static const char *allocinfo_str(const char *str)
    {
    let mut len: usize = strlen(str);
// Keep an extra space for the trailing NULL.
    if (len >= ALLOCINFO_STR_SIZE)
    str += (len - ALLOCINFO_STR_SIZE) + 1;
    return str;
    }
// Copy a string and trim from the beginning if it's too long
#[no_mangle]
unsafe extern "C" fn allocinfo_copy_str(dest: *mut c_char, src: *const c_char) {
    static void allocinfo_copy_str(char *dest, const char *src)
    {
    strscpy_pad(dest, allocinfo_str(src), ALLOCINFO_STR_SIZE);
    }
// Compare two strings and only consider the trimmed suffix if s1 is too long
#[no_mangle]
unsafe extern "C" fn allocinfo_cmp_str(str: *const c_char, template: *const c_char) -> c_int {
    static int allocinfo_cmp_str(const char *str, const char *template)
    {
    return strncmp(allocinfo_str(str), template, ALLOCINFO_STR_SIZE);
    }
// Fetch the per-CPU counters
#[no_mangle]
pub unsafe extern "C" fn allocinfo_prefetch_counters(ct: *mut codetag) -> alloc_tag_counters {
    static inline struct alloc_tag_counters allocinfo_prefetch_counters(struct codetag *ct)
    {
    return alloc_tag_read(ct_to_alloc_tag(ct));
    }
//
// Populates the UAPI allocinfo_tag_data structure with active runtime
// profiling counters extracted from the given kernel codetag.
//
    static void allocinfo_to_params(struct codetag *ct,
    struct allocinfo_tag_data *data,
    struct alloc_tag_counters *counters)
    {
    if (ct.modname)
    allocinfo_copy_str(data.tag.modname, ct.modname);
    else
    data.tag.modname[0] = '\0';
    allocinfo_copy_str(data.tag.function, ct.function);
    allocinfo_copy_str(data.tag.filename, ct.filename);
    data.tag.lineno = ct.lineno;
    data.counter.bytes = counters.bytes;
    data.counter.calls = counters.calls;
    data.counter.accurate = !alloc_tag_is_inaccurate(ct_to_alloc_tag(ct));
    }
//
// Retrieves the unique content ID representing the current allocation tag module
// layout, allowing userspace to detect if modules were loaded / unloaded.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_ioctl_get_content_id(m: *mut seq_file, arg: *mut void __user) -> c_int {
    static int allocinfo_ioctl_get_content_id(struct seq_file *m, void __user *arg)
    {
    struct allocinfo_content_id params;
    codetag_lock_module_list(alloc_tag_cttype);
    params.id = codetag_get_content_id(alloc_tag_cttype);
    codetag_unlock_module_list(alloc_tag_cttype);
    if (copy_to_user(arg, &params, sizeof(params)))
    return -EFAULT;
    return 0;
    }
//
// Verifies whether a given codetag satisfies the active filtering criteria by
// matching its characteristics against the specified filter.
//
    static bool matches_filter(struct codetag *ct, struct allocinfo_filter *filter,
    struct alloc_tag_counters *counters,
    bool *fetched_counters)
    {
    bool inaccurate;
    if (!filter || !filter.mask)
    return true;
    if (filter.mask & ALLOCINFO_FILTER_MASK_MODNAME) {
// user wants to filter by modname but ct->modname is NULL
    if (!ct.modname) {
// validate if user was attempting to filter for built-in allocations
    if (filter.fields.modname[0] != '\0')
    return false;
    } else if (allocinfo_cmp_str(ct.modname, filter.fields.modname))
    return false;
    }
    if ((filter.mask & ALLOCINFO_FILTER_MASK_FUNCTION) &&
    ct.function && allocinfo_cmp_str(ct.function, filter.fields.function))
    return false;
    if ((filter.mask & ALLOCINFO_FILTER_MASK_FILENAME) &&
    ct.filename && allocinfo_cmp_str(ct.filename, filter.fields.filename))
    return false;
    if ((filter.mask & ALLOCINFO_FILTER_MASK_LINENO) &&
    ct.lineno != filter.fields.lineno)
    return false;
    if (filter.mask & ALLOCINFO_FILTER_MASK_INACCURATE) {
    inaccurate = !!(ct.flags & CODETAG_FLAG_INACCURATE);
    if (inaccurate != !!(filter.inaccurate))
    return false;
    }
    if (filter.mask & (ALLOCINFO_FILTER_MASK_MIN_SIZE | ALLOCINFO_FILTER_MASK_MAX_SIZE)) {
    if (!*fetched_counters) {
// counters = allocinfo_prefetch_counters(ct);
// fetched_counters = true;
    }
    if ((filter.mask & ALLOCINFO_FILTER_MASK_MIN_SIZE) &&
    counters.bytes < filter.min_size)
    return false;
    if ((filter.mask & ALLOCINFO_FILTER_MASK_MAX_SIZE) &&
    counters.bytes > filter.max_size)
    return false;
    }
    return true;
    }
//
// Seeks the ioctl iterator to the specified 0-indexed tag position, reads its
// profiling data and returns it to userspace.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_ioctl_get_at(m: *mut seq_file, arg: *mut void __user) -> c_int {
    static int allocinfo_ioctl_get_at(struct seq_file *m, void __user *arg)
    {
    struct allocinfo_private *priv;
    struct codetag *ct;
    let mut params: allocinfo_get_at = {0};
    __u64 skip_count;
    struct alloc_tag_counters counters;
    bool fetched_counters;
    if (copy_from_user(&params, arg, sizeof(params)))
    return -EFAULT;
    if (params.filter.mask & ~ALLOCINFO_FILTER_MASKS)
    return -EINVAL;
    if ((params.filter.mask & ALLOCINFO_FILTER_MASK_MIN_SIZE) &&
    (params.filter.mask & ALLOCINFO_FILTER_MASK_MAX_SIZE) &&
    params.filter.min_size > params.filter.max_size)
    return -EINVAL;
    priv = m.private;
    mutex_lock(&priv.ioctl_lock);
    codetag_lock_module_list(alloc_tag_cttype);
    if (params.pos >= codetag_get_count(alloc_tag_cttype)) {
    codetag_unlock_module_list(alloc_tag_cttype);
    mutex_unlock(&priv.ioctl_lock);
    return -ENOENT;
    }
    skip_count = params.pos;
    if (params.filter.mask)
    priv.filter = params.filter;
    else
    priv.filter.mask = 0;
// Find the codetag
    priv.ioctl_iter = codetag_get_ct_iter(alloc_tag_cttype);
    ct = codetag_next_ct(&priv.ioctl_iter);
    while (ct) {
    fetched_counters = false;
    if (matches_filter(ct, &priv.filter, &counters, &fetched_counters)) {
    if (skip_count == 0)
    break;
    skip_count--;
    }
    ct = codetag_next_ct(&priv.ioctl_iter);
    }
    if (ct) {
    if (!fetched_counters)
    counters = allocinfo_prefetch_counters(ct);
    allocinfo_to_params(ct, &params.data, &counters);
    priv.positioned = true;
    }
    codetag_unlock_module_list(alloc_tag_cttype);
    mutex_unlock(&priv.ioctl_lock);
    if (!ct)
    return -ENOENT;
    if (copy_to_user(arg, &params, sizeof(params)))
    return -EFAULT;
    return 0;
    }
//
// Advances the ioctl iterator to the next allocation tag in the sequence and
// returns its profiling data to userspace.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_ioctl_get_next(m: *mut seq_file, arg: *mut void __user) -> c_int {
    static int allocinfo_ioctl_get_next(struct seq_file *m, void __user *arg)
    {
    struct allocinfo_private *priv;
    struct codetag *ct;
    struct allocinfo_tag_data params;
    let mut ret: c_int = 0;
    struct alloc_tag_counters counters;
    bool fetched_counters;
    memset(&params, 0, sizeof(params));
    priv = m.private;
    mutex_lock(&priv.ioctl_lock);
    codetag_lock_module_list(alloc_tag_cttype);
    if (!priv.positioned) {
    priv.ioctl_iter = codetag_get_ct_iter(alloc_tag_cttype);
    priv.positioned = true;
    }
    ct = codetag_next_ct(&priv.ioctl_iter);
    while (ct) {
    fetched_counters = false;
    if (matches_filter(ct, &priv.filter, &counters, &fetched_counters))
    break;
    ct = codetag_next_ct(&priv.ioctl_iter);
    }
    if (ct) {
    if (!fetched_counters)
    counters = allocinfo_prefetch_counters(ct);
    allocinfo_to_params(ct, &params, &counters);
    }
    if (!ct) {
    priv.positioned = false;
    ret = -ENOENT;
    }
    codetag_unlock_module_list(alloc_tag_cttype);
    mutex_unlock(&priv.ioctl_lock);
    if (ret == 0) {
    if (copy_to_user(arg, &params, sizeof(params)))
    return -EFAULT;
    }
    return ret;
    }
//
// Entry point ioctl function for /proc/allocinfo routing requests to fetch the
// layout content ID, seek to a specific tag, or read sequential tags.
//
    static long allocinfo_ioctl(struct file *file, unsigned int cmd,
    unsigned long __arg)
    {
    void __user *arg = (void __user *)__arg;
    int ret;
    switch (cmd) {
    case ALLOCINFO_IOC_CONTENT_ID:
    ret = allocinfo_ioctl_get_content_id(file.private_data, arg);
    break;
    case ALLOCINFO_IOC_GET_AT:
    ret = allocinfo_ioctl_get_at(file.private_data, arg);
    break;
    case ALLOCINFO_IOC_GET_NEXT:
    ret = allocinfo_ioctl_get_next(file.private_data, arg);
    break;
    default:
    ret = -ENOIOCTLCMD;
    break;
    }
    return ret;
    }

    static long allocinfo_compat_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    return allocinfo_ioctl(file, cmd, (unsigned long)compat_ptr(arg));
    }

    static const struct proc_ops allocinfo_proc_ops = {
    .proc_open		= allocinfo_open,
    .proc_read_iter		= seq_read_iter,
    .proc_lseek		= seq_lseek,
    .proc_release		= allocinfo_release,
    .proc_ioctl		= allocinfo_ioctl,

    .proc_compat_ioctl	= allocinfo_compat_ioctl,

    };
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_top_users(tags: *mut codetag_bytes, count: usize, can_sleep: bool) -> usize {
    size_t alloc_tag_top_users(struct codetag_bytes *tags, size_t count, bool can_sleep)
    {
    struct codetag_iterator iter;
    struct codetag *ct;
    struct codetag_bytes n;
    unsigned int i, nr = 0;
    if (IS_ERR_OR_NULL(alloc_tag_cttype))
    return 0;
    if (can_sleep)
    codetag_lock_module_list(alloc_tag_cttype);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !codetag_trylock_module_list(alloc_tag_cttype)) -> else {
    else if (!codetag_trylock_module_list(alloc_tag_cttype))
    return 0;
    iter = codetag_get_ct_iter(alloc_tag_cttype);
    while ((ct = codetag_next_ct(&iter))) {
    let mut counter: alloc_tag_counters = alloc_tag_read(ct_to_alloc_tag(ct));
    n.ct	= ct;
    n.bytes = counter.bytes;
    for (i = 0; i < nr; i++)
    if (n.bytes > tags[i].bytes)
    break;
    if (i < count) {
    nr -= nr == count;
    memmove(&tags[i + 1],
    &tags[i],
    sizeof(tags[0]) * (nr - i));
    nr++;
    tags[i] = n;
    }
    }
    codetag_unlock_module_list(alloc_tag_cttype);
    return nr;
    }
#[no_mangle]
pub unsafe extern "C" fn pgalloc_tag_split(folio: *mut folio, old_order: c_int, new_order: c_int) {
    void pgalloc_tag_split(struct folio *folio, int old_order, int new_order)
    {
    int i;
    struct alloc_tag *tag;
    let mut nr_pages: c_uint = 1 << new_order;
    if (!mem_alloc_profiling_enabled())
    return;
    tag = __pgalloc_tag_get(&folio.page);
    if (!tag)
    return;
    for (i = nr_pages; i < (1 << old_order); i += nr_pages) {
    union pgtag_ref_handle handle;
    union codetag_ref ref;
    if (get_page_tag_ref(folio_page(folio, i), &ref, &handle)) {
// Set new reference to point to the original tag
    alloc_tag_ref_set(&ref, tag);
    update_page_tag_ref(handle, &ref);
    put_page_tag_ref(handle);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pgalloc_tag_swap(new: *mut folio, old: *mut folio) {
    void pgalloc_tag_swap(struct folio *new, struct folio *old)
    {
    union pgtag_ref_handle handle_old, handle_new;
    union codetag_ref ref_old, ref_new;
    struct alloc_tag *tag_old, *tag_new;
    if (!mem_alloc_profiling_enabled())
    return;
    tag_old = __pgalloc_tag_get(&old.page);
    if (!tag_old)
    return;
    tag_new = __pgalloc_tag_get(&new.page);
    if (!tag_new)
    return;
    if (!get_page_tag_ref(&old.page, &ref_old, &handle_old))
    return;
    if (!get_page_tag_ref(&new.page, &ref_new, &handle_new)) {
    put_page_tag_ref(handle_old);
    return;
    }
//
// Clear tag references to avoid debug warning when using
// __alloc_tag_ref_set() with non-empty reference.
//
    set_codetag_empty(&ref_old);
    set_codetag_empty(&ref_new);
// swap tags
    __alloc_tag_ref_set(&ref_old, tag_new);
    update_page_tag_ref(handle_old, &ref_old);
    __alloc_tag_ref_set(&ref_new, tag_old);
    update_page_tag_ref(handle_new, &ref_new);
    put_page_tag_ref(handle_old);
    put_page_tag_ref(handle_new);
    }
#[no_mangle]
unsafe extern "C" fn shutdown_mem_profiling(remove_file: bool) {
    static void shutdown_mem_profiling(bool remove_file)
    {
    if (mem_alloc_profiling_enabled())
    static_branch_disable(&mem_alloc_profiling_key);
    if (!mem_profiling_support)
    return;
    if (remove_file)
    remove_proc_entry(ALLOCINFO_FILE_NAME, core::ptr::null_mut());
    mem_profiling_support = false;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_sec_init() -> void __init {
    void __init alloc_tag_sec_init(void)
    {
    struct alloc_tag *last_codetag;
    if (!mem_profiling_support)
    return;
    if (!static_key_enabled(&mem_profiling_compressed))
    return;
    kernel_tags.first_tag = (struct alloc_tag *)kallsyms_lookup_name(
    SECTION_START(ALLOC_TAG_SECTION_NAME));
    last_codetag = (struct alloc_tag *)kallsyms_lookup_name(
    SECTION_STOP(ALLOC_TAG_SECTION_NAME));
    kernel_tags.count = last_codetag - kernel_tags.first_tag;
// Check if kernel tags fit into page flags
    if (kernel_tags.count > (1UL << NR_UNUSED_PAGEFLAG_BITS)) {
    shutdown_mem_profiling(false); /* allocinfo file does not exist yet */
    pr_err("%lu allocation tags cannot be references using %d available page flag bits. Memory allocation profiling is disabled!\n",
    kernel_tags.count, NR_UNUSED_PAGEFLAG_BITS);
    return;
    }
    alloc_tag_ref_offs = (LRU_REFS_PGOFF - NR_UNUSED_PAGEFLAG_BITS);
    alloc_tag_ref_mask = ((1UL << NR_UNUSED_PAGEFLAG_BITS) - 1);
    pr_debug("Memory allocation profiling compression is using %d page flag bits!\n",
    NR_UNUSED_PAGEFLAG_BITS);
    }

    let mut mod_area_mt: static struct maple_tree = MTREE_INIT(mod_area_mt, MT_FLAGS_ALLOC_RANGE);
    static struct vm_struct *vm_module_tags;
// A dummy object used to indicate an unloaded module
    static struct module unloaded_mod;
// A dummy object used to indicate a module prepended area
    static struct module prepend_mod;
    struct alloc_tag_module_section module_tags;
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_align(val: c_ulong) -> c_ulong {
    static inline unsigned long alloc_tag_align(unsigned long val)
    {
    if (!static_key_enabled(&mem_profiling_compressed)) {
// No alignment requirements when we are not indexing the tags
    return val;
    }
    if (val % sizeof(struct alloc_tag) == 0)
    return val;
    return ((val / sizeof(struct alloc_tag)) + 1) * sizeof(struct alloc_tag);
    }
#[no_mangle]
unsafe extern "C" fn ensure_alignment(align: c_ulong, prepend: *mut c_uint) -> bool {
    static bool ensure_alignment(unsigned long align, unsigned int *prepend)
    {
    if (!static_key_enabled(&mem_profiling_compressed)) {
// No alignment requirements when we are not indexing the tags
    return true;
    }
//
// If alloc_tag size is not a multiple of required alignment, tag
// indexing does not work.
//
    if (!IS_ALIGNED(sizeof(struct alloc_tag), align))
    return false;
// Ensure prepend consumes multiple of alloc_tag-sized blocks
    if (*prepend)
// prepend = alloc_tag_align(*prepend);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn tags_addressable() -> bool {
    static inline bool tags_addressable(void)
    {
    unsigned long tag_idx_count;
    if (!static_key_enabled(&mem_profiling_compressed))
    return true; /* with page_ext tags are always addressable */
    tag_idx_count = CODETAG_ID_FIRST + kernel_tags.count +
    module_tags.size / sizeof(struct alloc_tag);
    return tag_idx_count < (1UL << NR_UNUSED_PAGEFLAG_BITS);
    }
#[no_mangle]
unsafe extern "C" fn needs_section_mem(mod: *mut module, size: c_ulong) -> bool {
    static bool needs_section_mem(struct module *mod, unsigned long size)
    {
    if (!mem_profiling_support)
    return false;
    return size >= sizeof(struct alloc_tag);
    }
    static bool clean_unused_counters(struct alloc_tag *start_tag,
    struct alloc_tag *end_tag)
    {
    struct alloc_tag *tag;
    let mut ret: bool = true;
    for (tag = start_tag; tag <= end_tag; tag++) {
    struct alloc_tag_counters counter;
    if (!tag.counters)
    continue;
    counter = alloc_tag_read(tag);
    if (!counter.bytes) {
    free_percpu(tag.counters);
    tag.counters = core::ptr::null_mut();
    } else {
    ret = false;
    }
    }
    return ret;
    }
// Called with mod_area_mt locked
#[no_mangle]
unsafe extern "C" fn clean_unused_module_areas_locked() {
    static void clean_unused_module_areas_locked(void)
    {
    MA_STATE(mas, &mod_area_mt, 0, module_tags.size);
    struct module *val;
    mas_for_each(&mas, val, module_tags.size) {
    struct alloc_tag *start_tag;
    struct alloc_tag *end_tag;
    if (val != &unloaded_mod)
    continue;
// Release area if all tags are unused
    start_tag = (struct alloc_tag *)(module_tags.start_addr + mas.index);
    end_tag = (struct alloc_tag *)(module_tags.start_addr + mas.last);
    if (clean_unused_counters(start_tag, end_tag))
    mas_erase(&mas);
    }
    }
// Called with mod_area_mt locked
    static bool find_aligned_area(struct ma_state *mas, unsigned long section_size,
    unsigned long size, unsigned int prepend, unsigned long align)
    {
    let mut cleanup_done: bool = false;
    repeat:
// Try finding exact size and hope the start is aligned
    if (!mas_empty_area(mas, 0, section_size - 1, prepend + size)) {
    if (IS_ALIGNED(mas.index + prepend, align))
    return true;
// Try finding larger area to align later
    mas_reset(mas);
    if (!mas_empty_area(mas, 0, section_size - 1,
    size + prepend + align - 1))
    return true;
    }
// No free area, try cleanup stale data and repeat the search once
    if (!cleanup_done) {
    clean_unused_module_areas_locked();
    cleanup_done = true;
    mas_reset(mas);
    goto repeat;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn vm_module_tags_populate() -> c_int {
    static int vm_module_tags_populate(void)
    {
    unsigned long phys_end = ALIGN_DOWN(module_tags.start_addr, PAGE_SIZE) +
    (vm_module_tags.nr_pages << PAGE_SHIFT);
    let mut new_end: c_ulong = module_tags.start_addr + module_tags.size;
    if (phys_end < new_end) {
    struct page **next_page = vm_module_tags.pages + vm_module_tags.nr_pages;
    let mut old_shadow_end: c_ulong = ALIGN(phys_end, MODULE_ALIGN);
    let mut new_shadow_end: c_ulong = ALIGN(new_end, MODULE_ALIGN);
    unsigned long more_pages;
    let mut nr: c_ulong = 0;
    more_pages = ALIGN(new_end - phys_end, PAGE_SIZE) >> PAGE_SHIFT;
    while (nr < more_pages) {
    unsigned long allocated;
    allocated = alloc_pages_bulk_node(GFP_KERNEL | __GFP_NOWARN,
    NUMA_NO_NODE, more_pages - nr, next_page + nr);
    if (!allocated)
    break;
    nr += allocated;
    }
    if (nr < more_pages ||
    vmap_pages_range(phys_end, phys_end + (nr << PAGE_SHIFT), PAGE_KERNEL,
    next_page, PAGE_SHIFT) < 0) {
    let mut arg: release_pages_arg = { .pages = next_page };
// Clean up and error out
    release_pages(arg, nr);
    return -ENOMEM;
    }
    vm_module_tags.nr_pages += nr;
//
// Kasan allocates 1 byte of shadow for every 8 bytes of data.
// When kasan_alloc_module_shadow allocates shadow memory,
// its unit of allocation is a page.
// Therefore, here we need to align to MODULE_ALIGN.
//
    if (old_shadow_end < new_shadow_end)
    kasan_alloc_module_shadow((void *)old_shadow_end,
    new_shadow_end - old_shadow_end,
    GFP_KERNEL);
    }
//
// Mark the pages as accessible, now that they are mapped.
// With hardware tag-based KASAN, marking is skipped for
// non-VM_ALLOC mappings, see __kasan_unpoison_vmalloc().
//
    kasan_unpoison_vmalloc((void *)module_tags.start_addr,
    new_end - module_tags.start_addr,
    KASAN_VMALLOC_PROT_NORMAL);
    return 0;
    }
    static void *reserve_module_tags(struct module *mod, unsigned long size,
    unsigned int prepend, unsigned long align)
    {
    let mut section_size: c_ulong = module_tags.end_addr - module_tags.start_addr;
    MA_STATE(mas, &mod_area_mt, 0, section_size - 1);
    unsigned long offset;
    void *ret = core::ptr::null_mut();
// If no tags return error
    if (size < sizeof(struct alloc_tag))
    return ERR_PTR(-EINVAL);
//
// align is always power of 2, so we can use IS_ALIGNED and ALIGN.
// align 0 or 1 means no alignment, to simplify set to 1.
//
    if (!align)
    align = 1;
    if (!ensure_alignment(align, &prepend)) {
    shutdown_mem_profiling(true);
    pr_err("%s: alignment %lu is incompatible with allocation tag indexing. Memory allocation profiling is disabled!\n",
    mod.name, align);
    return ERR_PTR(-EINVAL);
    }
    mas_lock(&mas);
    if (!find_aligned_area(&mas, section_size, size, prepend, align)) {
    ret = ERR_PTR(-ENOMEM);
    goto unlock;
    }
// Mark found area as reserved
    offset = mas.index;
    offset += prepend;
    offset = ALIGN(offset, align);
    if (offset != mas.index) {
    let mut pad_start: c_ulong = mas.index;
    mas.last = offset - 1;
    mas_store(&mas, &prepend_mod);
    if (mas_is_err(&mas)) {
    ret = ERR_PTR(xa_err(mas.node));
    goto unlock;
    }
    mas.index = offset;
    mas.last = offset + size - 1;
    mas_store(&mas, mod);
    if (mas_is_err(&mas)) {
    mas.index = pad_start;
    mas_erase(&mas);
    ret = ERR_PTR(xa_err(mas.node));
    }
    } else {
    mas.last = offset + size - 1;
    mas_store(&mas, mod);
    if (mas_is_err(&mas))
    ret = ERR_PTR(xa_err(mas.node));
    }
    unlock:
    mas_unlock(&mas);
    if (IS_ERR(ret))
    return ret;
    if (module_tags.size < offset + size) {
    int grow_res;
    module_tags.size = offset + size;
    if (mem_alloc_profiling_enabled() && !tags_addressable()) {
    shutdown_mem_profiling(true);
    pr_warn("With module %s there are too many tags to fit in %d page flag bits. Memory allocation profiling is disabled!\n",
    mod.name, NR_UNUSED_PAGEFLAG_BITS);
    }
    grow_res = vm_module_tags_populate();
    if (grow_res) {
    shutdown_mem_profiling(true);
    pr_err("Failed to allocate memory for allocation tags in the module %s. Memory allocation profiling is disabled!\n",
    mod.name);
    return ERR_PTR(grow_res);
    }
    }
    return (struct alloc_tag *)(module_tags.start_addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn release_module_tags(mod: *mut module, used: bool) {
    static void release_module_tags(struct module *mod, bool used)
    {
    MA_STATE(mas, &mod_area_mt, module_tags.size, module_tags.size);
    struct alloc_tag *start_tag;
    struct alloc_tag *end_tag;
    struct module *val;
    mas_lock(&mas);
    mas_for_each_rev(&mas, val, 0)
    if (val == mod)
    break;
    if (!val) /* module not found */
    goto out;
    if (!used)
    goto release_area;
    start_tag = (struct alloc_tag *)(module_tags.start_addr + mas.index);
    end_tag = (struct alloc_tag *)(module_tags.start_addr + mas.last);
    if (!clean_unused_counters(start_tag, end_tag)) {
    struct alloc_tag *tag;
    for (tag = start_tag; tag <= end_tag; tag++) {
    struct alloc_tag_counters counter;
    if (!tag.counters)
    continue;
    counter = alloc_tag_read(tag);
    pr_info("%s:%u module %s func:%s has %llu allocated at module unload\n",
    tag.ct.filename, tag.ct.lineno, tag.ct.modname,
    tag.ct.function, counter.bytes);
    }
    } else {
    used = false;
    }
    release_area:
    mas_store(&mas, used ? &unloaded_mod : core::ptr::null_mut());
    val = mas_prev_range(&mas, 0);
    if (val == &prepend_mod)
    mas_store(&mas, core::ptr::null_mut());
    out:
    mas_unlock(&mas);
    }
#[no_mangle]
unsafe extern "C" fn load_module(mod: *mut module, start: *mut codetag, stop: *mut codetag) -> c_int {
    static int load_module(struct module *mod, struct codetag *start, struct codetag *stop)
    {
// Allocate module alloc_tag percpu counters
    struct alloc_tag *start_tag;
    struct alloc_tag *stop_tag;
    struct alloc_tag *tag;
// percpu counters for core allocations are already statically allocated
    if (!mod)
    return 0;
    start_tag = ct_to_alloc_tag(start);
    stop_tag = ct_to_alloc_tag(stop);
    for (tag = start_tag; tag < stop_tag; tag++) {
    WARN_ON(tag.counters);
    tag.counters = alloc_percpu(struct alloc_tag_counters);
    if (!tag.counters) {
    while (--tag >= start_tag) {
    free_percpu(tag.counters);
    tag.counters = core::ptr::null_mut();
    }
    pr_err("Failed to allocate memory for allocation tag percpu counters in the module %s\n",
    mod.name);
    return -ENOMEM;
    }
//
// Avoid a kmemleak false positive. The pointer to the counters is stored
// in the alloc_tag section of the module and cannot be directly accessed.
//
    kmemleak_ignore_percpu(tag.counters);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn replace_module(mod: *mut module, new_mod: *mut module) {
    static void replace_module(struct module *mod, struct module *new_mod)
    {
    MA_STATE(mas, &mod_area_mt, 0, module_tags.size);
    struct module *val;
    mas_lock(&mas);
    mas_for_each(&mas, val, module_tags.size) {
    if (val != mod)
    continue;
    mas_store_gfp(&mas, new_mod, GFP_KERNEL);
    break;
    }
    mas_unlock(&mas);
    }
#[no_mangle]
unsafe extern "C" fn alloc_mod_tags_mem() -> int __init {
    static int __init alloc_mod_tags_mem(void)
    {
// Map space to copy allocation tags
    vm_module_tags = execmem_vmap(MODULE_ALLOC_TAG_VMAP_SIZE);
    if (!vm_module_tags) {
    pr_err("Failed to map %lu bytes for module allocation tags\n",
    MODULE_ALLOC_TAG_VMAP_SIZE);
    module_tags.start_addr = 0;
    return -ENOMEM;
    }
    vm_module_tags.pages = kmalloc_objs(struct page *,
    get_vm_area_size(vm_module_tags) >> PAGE_SHIFT,
    GFP_KERNEL | __GFP_ZERO);
    if (!vm_module_tags.pages) {
    free_vm_area(vm_module_tags);
    return -ENOMEM;
    }
    module_tags.start_addr = (unsigned long)vm_module_tags.addr;
    module_tags.end_addr = module_tags.start_addr + MODULE_ALLOC_TAG_VMAP_SIZE;
// Ensure the base is alloc_tag aligned when required for indexing
    module_tags.start_addr = alloc_tag_align(module_tags.start_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_mod_tags_mem() -> void __init {
    static void __init free_mod_tags_mem(void)
    {
    let mut arg: release_pages_arg = { .pages = vm_module_tags.pages };
    module_tags.start_addr = 0;
    release_pages(arg, vm_module_tags.nr_pages);
    kfree(vm_module_tags.pages);
    free_vm_area(vm_module_tags);
    }

    static inline int alloc_mod_tags_mem(void) { return 0; }
    static inline void free_mod_tags_mem(void) {}

// See: Documentation/mm/allocation-profiling.rst
#[no_mangle]
unsafe extern "C" fn setup_early_mem_profiling(str: *mut c_char) -> int __init {
    static int __init setup_early_mem_profiling(char *str)
    {
    let mut compressed: bool = false;
    bool enable;
    if (!str || !str[0])
    return -EINVAL;
    if (!strncmp(str, "never", 5)) {
    enable = false;
    mem_profiling_support = false;
    pr_info("Memory allocation profiling is disabled!\n");
    } else {
    char *token = strsep(&str, ",");
    if (kstrtobool(token, &enable))
    return -EINVAL;
    if (str) {
    if (strcmp(str, "compressed"))
    return -EINVAL;
    compressed = true;
    }
    mem_profiling_support = true;
    pr_info("Memory allocation profiling is enabled %s compression and is turned %s!\n",
    compressed ? "with" : "without", str_on_off(enable));
    }
    if (enable != mem_alloc_profiling_enabled()) {
    if (enable)
    static_branch_enable(&mem_alloc_profiling_key);
    else
    static_branch_disable(&mem_alloc_profiling_key);
    }
    if (compressed != static_key_enabled(&mem_profiling_compressed)) {
    if (compressed)
    static_branch_enable(&mem_profiling_compressed);
    else
    static_branch_disable(&mem_profiling_compressed);
    }
    return 0;
    }
    early_param("sysctl.vm.mem_profiling", setup_early_mem_profiling);
#[no_mangle]
unsafe extern "C" fn need_page_alloc_tagging() -> __init bool {
    static __init bool need_page_alloc_tagging(void)
    {
    if (static_key_enabled(&mem_profiling_compressed))
    return false;
    return mem_profiling_support;
    }

//
// Track page allocations before page_ext is initialized.
// Some pages are allocated before page_ext becomes available, leaving
// their codetag uninitialized. Track these early PFNs so we can clear
// their codetag refs later to avoid warnings when they are freed.
//
// Each page is cast to a pfn_pool: the first few bytes hold metadata
// (next pointer and slot count), the remainder stores PFNs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfn_pool {
    pub next: *mut pfn_pool,
    pub count: core::sync::atomic::AtomicI32,
    pub pfns: [c_ulong; ],
}

    sizeof(unsigned long))
    static struct pfn_pool *current_pfn_pool __initdata;
#[no_mangle]
unsafe extern "C" fn __alloc_tag_add_early_pfn(pfn: c_ulong) -> void __init {
    static void __init __alloc_tag_add_early_pfn(unsigned long pfn)
    {
    struct pfn_pool *pool;
    int idx;
    do {
    pool = READ_ONCE(current_pfn_pool);
    if (!pool || atomic_read(&pool.count) >= PFN_POOL_SIZE) {
    struct page *new_page = __alloc_pages(__GFP_HIGH, 0, numa_mem_id(),
    core::ptr::null_mut(), ALLOC_NO_CODETAG);
    struct pfn_pool *new;
    if (!new_page) {
    pr_warn_once("early PFN tracking page allocation failed\n");
    return;
    }
    new = page_address(new_page);
    new.next = pool;
    atomic_set(&new.count, 0);
    if (cmpxchg(&current_pfn_pool, pool, new) != pool) {
    clear_page_tag_ref(new_page);
    __free_page(new_page);
    continue;
    }
    pool = new;
    }
    idx = atomic_read(&pool.count);
    if (idx >= PFN_POOL_SIZE)
    continue;
    if (atomic_cmpxchg(&pool.count, idx, idx + 1) == idx)
    break;
    } while (1);
    pool.pfns[idx] = pfn;
    }
    typedef void alloc_tag_add_func(unsigned long pfn);
    static alloc_tag_add_func __rcu *alloc_tag_add_early_pfn_ptr __refdata =
    RCU_INITIALIZER(__alloc_tag_add_early_pfn);
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_add_early_pfn(pfn: c_ulong, alloc_flags: c_uint) {
    void alloc_tag_add_early_pfn(unsigned long pfn, unsigned int alloc_flags)
    {
    alloc_tag_add_func *alloc_tag_add;
    if (static_key_enabled(&mem_profiling_compressed))
    return;
// Skip allocations for the tracking list itself to avoid recursion.
    if (alloc_flags & ALLOC_NO_CODETAG)
    return;
    rcu_read_lock();
    alloc_tag_add = rcu_dereference(alloc_tag_add_early_pfn_ptr);
    if (alloc_tag_add)
    alloc_tag_add(pfn);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn clear_early_alloc_pfn_tag_refs() -> void __init {
    static void __init clear_early_alloc_pfn_tag_refs(void)
    {
    struct pfn_pool *pool, *next;
    struct page *page;
    int i;
    if (static_key_enabled(&mem_profiling_compressed))
    return;
    rcu_assign_pointer(alloc_tag_add_early_pfn_ptr, core::ptr::null_mut());
// Make sure we are not racing with __alloc_tag_add_early_pfn()
    synchronize_rcu();
    for (pool = current_pfn_pool; pool; pool = next) {
    let mut nr_pfns: c_int = atomic_read(&pool.count);
    for (i = 0; i < nr_pfns; i++) {
    let mut pfn: c_ulong = pool.pfns[i];
    if (pfn_valid(pfn)) {
    union pgtag_ref_handle handle;
    union codetag_ref ref;
    if (get_page_tag_ref(pfn_to_page(pfn), &ref, &handle)) {
//
// An early-allocated page could be freed and reallocated
// after its page_ext is initialized but before we clear it.
// In that case, it already has a valid tag set.
// We should not overwrite that valid tag
// with CODETAG_EMPTY.
//
// Note: there is still a small race window between checking
// ref.ct and calling set_codetag_empty(). We accept this
// race as it's unlikely and the extra complexity of atomic
// cmpxchg is not worth it for this debug-only code path.
//
    if (ref.ct) {
    put_page_tag_ref(handle);
    continue;
    }
    set_codetag_empty(&ref);
    update_page_tag_ref(handle, &ref);
    put_page_tag_ref(handle);
    }
    }
    }
    next = pool.next;
    page = virt_to_page(pool);
    clear_page_tag_ref(page);
    __free_page(page);
    }
    }

    static inline void __init clear_early_alloc_pfn_tag_refs(void) {}

#[no_mangle]
unsafe extern "C" fn init_page_alloc_tagging() -> __init void {
    static __init void init_page_alloc_tagging(void)
    {
    clear_early_alloc_pfn_tag_refs();
    }
    struct page_ext_operations page_alloc_tagging_ops = {
    .size = sizeof(union codetag_ref),
    .need = need_page_alloc_tagging,
    .init = init_page_alloc_tagging,
    };
    EXPORT_SYMBOL(page_alloc_tagging_ops);

//
// Not using proc_do_static_key() directly to prevent enabling profiling
// after it was shut down.
//
    static int proc_mem_profiling_handler(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    if (write) {
//
// Call from do_sysctl_args() which is a no-op since the same
// value was already set by setup_early_mem_profiling.
// Return success to avoid warnings from do_sysctl_args().
//
    if (!current.mm)
    return 0;

// User can't toggle profiling while debugging
    return -EACCES;

    if (!mem_profiling_support)
    return -EINVAL;
    }
    return proc_do_static_key(table, write, buffer, lenp, ppos);
    }
    static const struct ctl_table memory_allocation_profiling_sysctls[] = {
    {
    .procname	= "mem_profiling",
    .data		= &mem_alloc_profiling_key,
    .mode		= 0644,
    .proc_handler	= proc_mem_profiling_handler,
    },
    {
    .procname	= "mem_profiling_compressed",
    .data		= &mem_profiling_compressed,
    .mode		= 0444,
    .proc_handler	= proc_do_static_key,
    },
    };
#[no_mangle]
unsafe extern "C" fn sysctl_init() -> void __init {
    static void __init sysctl_init(void)
    {
    register_sysctl_init("vm", memory_allocation_profiling_sysctls);
    }

    static inline void sysctl_init(void) {}

#[no_mangle]
unsafe extern "C" fn alloc_tag_init() -> int __init {
    static int __init alloc_tag_init(void)
    {
    const struct codetag_type_desc desc = {
    .section		= ALLOC_TAG_SECTION_NAME,
    .tag_size		= sizeof(struct alloc_tag),

    .needs_section_mem	= needs_section_mem,
    .alloc_section_mem	= reserve_module_tags,
    .free_section_mem	= release_module_tags,
    .module_load		= load_module,
    .module_replaced	= replace_module,

    };
    int res;
    sysctl_init();
    if (!mem_profiling_support) {
    pr_info("Memory allocation profiling is not supported!\n");
    return 0;
    }
    if (!proc_create(ALLOCINFO_FILE_NAME, 0400, core::ptr::null_mut(), &allocinfo_proc_ops)) {
    pr_err("Failed to create %s file\n", ALLOCINFO_FILE_NAME);
    shutdown_mem_profiling(false);
    return -ENOMEM;
    }
    res = alloc_mod_tags_mem();
    if (res) {
    pr_err("Failed to reserve address space for module tags, errno = %d\n", res);
    shutdown_mem_profiling(true);
    return res;
    }
    alloc_tag_cttype = codetag_register_type(&desc);
    if (IS_ERR(alloc_tag_cttype)) {
    pr_err("Allocation tags registration failed, errno = %pe\n", alloc_tag_cttype);
    free_mod_tags_mem();
    shutdown_mem_profiling(true);
    return PTR_ERR(alloc_tag_cttype);
    }
    return 0;
    }
    module_init(alloc_tag_init);
