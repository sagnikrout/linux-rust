//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lkdtm/usercopy.c
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
// This is for all the tests related to copy_to_user() and copy_from_user()
// hardening.
//

//
// Many of the tests here end up using const sizes, but those would
// normally be ignored by hardened usercopy, so force the compiler
// into choosing the non-const path to make sure we trigger the
// hardened usercopy checks by added "unconst" to all the const copies,
// and making sure "cache_size" isn't optimized into a const.
//
    static volatile size_t unconst;
    let mut cache_size: static volatile size_t = 1024;
    static struct kmem_cache *whitelist_cache;
    static const unsigned char test_text[] = "This is a test.\n";
//
// Instead of adding -Wno-return-local-addr, just pass the stack address
// through a function to obfuscate it from the compiler.
//
    static noinline unsigned char *trick_compiler(unsigned char *stack)
    {
    return stack + unconst;
    }
    static noinline unsigned char *do_usercopy_stack_callee(int value)
    {
    unsigned char buf[128];
    int i;
// Exercise stack to avoid everything living in registers.
    for (i = 0; i < sizeof(buf); i++) {
    buf[i] = value & 0xff;
    }
//
// Put the target buffer in the middle of stack allocation
// so that we don't step on future stack users regardless
// of stack growth direction.
//
    return trick_compiler(&buf[(128/2)-32]);
    }
#[no_mangle]
unsafe extern "C" fn do_usercopy_stack(to_user: bool, bad_frame: bool) -> noinline void {
    static noinline void do_usercopy_stack(bool to_user, bool bad_frame)
    {
    unsigned long user_addr;
    unsigned char good_stack[32];
    unsigned char *bad_stack;
    int i;
// Exercise stack to avoid everything living in registers.
    for (i = 0; i < sizeof(good_stack); i++)
    good_stack[i] = test_text[i % sizeof(test_text)];
// This is a pointer to outside our current stack frame.
    if (bad_frame) {
    bad_stack = do_usercopy_stack_callee((uintptr_t)&bad_stack);
    } else {
// Put start address just inside stack.
    bad_stack = task_stack_page(current) + THREAD_SIZE;
    bad_stack -= sizeof(unsigned long);
    }

    pr_info("stack     : %px\n", (void *)current_stack_pointer);

    pr_info("good_stack: %px-%px\n", good_stack, good_stack + sizeof(good_stack));
    pr_info("bad_stack : %px-%px\n", bad_stack, bad_stack + sizeof(good_stack));
    user_addr = vm_mmap(core::ptr::null_mut(), 0, PAGE_SIZE,
    PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_ANONYMOUS | MAP_PRIVATE, 0);
    if (user_addr >= TASK_SIZE) {
    pr_warn("Failed to allocate user memory\n");
    return;
    }
    if (to_user) {
    pr_info("attempting good copy_to_user of local stack\n");
    if (copy_to_user((void __user *)user_addr, good_stack,
    unconst + sizeof(good_stack))) {
    pr_warn("copy_to_user failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_to_user of distant stack\n");
    if (copy_to_user((void __user *)user_addr, bad_stack,
    unconst + sizeof(good_stack))) {
    pr_warn("copy_to_user failed, but lacked Oops\n");
    goto free_user;
    }
    } else {
//
// There isn't a safe way to not be protected by usercopy
// if we're going to write to another thread's stack.
//
    if (!bad_frame)
    goto free_user;
    pr_info("attempting good copy_from_user of local stack\n");
    if (copy_from_user(good_stack, (void __user *)user_addr,
    unconst + sizeof(good_stack))) {
    pr_warn("copy_from_user failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_from_user of distant stack\n");
    if (copy_from_user(bad_stack, (void __user *)user_addr,
    unconst + sizeof(good_stack))) {
    pr_warn("copy_from_user failed, but lacked Oops\n");
    goto free_user;
    }
    }
    free_user:
    vm_munmap(user_addr, PAGE_SIZE);
    }
//
// This checks for whole-object size validation with hardened usercopy,
// with or without usercopy whitelisting.
//
#[no_mangle]
unsafe extern "C" fn do_usercopy_slab_size(to_user: bool) {
    static void do_usercopy_slab_size(bool to_user)
    {
    unsigned long user_addr;
    unsigned char *one, *two;
    void __user *test_user_addr;
    void *test_kern_addr;
    let mut size: usize = unconst + 1024;
    one = kmalloc(size, GFP_KERNEL);
    two = kmalloc(size, GFP_KERNEL);
    if (!one || !two) {
    pr_warn("Failed to allocate kernel memory\n");
    goto free_kernel;
    }
    user_addr = vm_mmap(core::ptr::null_mut(), 0, PAGE_SIZE,
    PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_ANONYMOUS | MAP_PRIVATE, 0);
    if (user_addr >= TASK_SIZE) {
    pr_warn("Failed to allocate user memory\n");
    goto free_kernel;
    }
    memset(one, 'A', size);
    memset(two, 'B', size);
    test_user_addr = (void __user *)(user_addr + 16);
    test_kern_addr = one + 16;
    if (to_user) {
    pr_info("attempting good copy_to_user of correct size\n");
    if (copy_to_user(test_user_addr, test_kern_addr, size / 2)) {
    pr_warn("copy_to_user failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_to_user of too large size\n");
    if (copy_to_user(test_user_addr, test_kern_addr, size)) {
    pr_warn("copy_to_user failed, but lacked Oops\n");
    goto free_user;
    }
    } else {
    pr_info("attempting good copy_from_user of correct size\n");
    if (copy_from_user(test_kern_addr, test_user_addr, size / 2)) {
    pr_warn("copy_from_user failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_from_user of too large size\n");
    if (copy_from_user(test_kern_addr, test_user_addr, size)) {
    pr_warn("copy_from_user failed, but lacked Oops\n");
    goto free_user;
    }
    }
    pr_err("FAIL: bad usercopy not detected!\n");
    pr_expected_config_param(CONFIG_HARDENED_USERCOPY, "hardened_usercopy");
    free_user:
    vm_munmap(user_addr, PAGE_SIZE);
    free_kernel:
    kfree(one);
    kfree(two);
    }
//
// This checks for the specific whitelist window within an object. If this
// test passes, then do_usercopy_slab_size() tests will pass too.
//
#[no_mangle]
unsafe extern "C" fn do_usercopy_slab_whitelist(to_user: bool) {
    static void do_usercopy_slab_whitelist(bool to_user)
    {
    unsigned long user_alloc;
    unsigned char *buf = core::ptr::null_mut();
    unsigned char __user *user_addr;
    size_t offset, size;
// Make sure cache was prepared.
    if (!whitelist_cache) {
    pr_warn("Failed to allocate kernel cache\n");
    return;
    }
//
// Allocate a buffer with a whitelisted window in the buffer.
//
    buf = kmem_cache_alloc(whitelist_cache, GFP_KERNEL);
    if (!buf) {
    pr_warn("Failed to allocate buffer from whitelist cache\n");
    goto free_alloc;
    }
// Allocate user memory we'll poke at.
    user_alloc = vm_mmap(core::ptr::null_mut(), 0, PAGE_SIZE,
    PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_ANONYMOUS | MAP_PRIVATE, 0);
    if (user_alloc >= TASK_SIZE) {
    pr_warn("Failed to allocate user memory\n");
    goto free_alloc;
    }
    user_addr = (void __user *)user_alloc;
    memset(buf, 'B', cache_size);
// Whitelisted window in buffer, from kmem_cache_create_usercopy.
    offset = (cache_size / 4) + unconst;
    size = (cache_size / 16) + unconst;
    if (to_user) {
    pr_info("attempting good copy_to_user inside whitelist\n");
    if (copy_to_user(user_addr, buf + offset, size)) {
    pr_warn("copy_to_user failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_to_user outside whitelist\n");
    if (copy_to_user(user_addr, buf + offset - 1, size)) {
    pr_warn("copy_to_user failed, but lacked Oops\n");
    goto free_user;
    }
    } else {
    pr_info("attempting good copy_from_user inside whitelist\n");
    if (copy_from_user(buf + offset, user_addr, size)) {
    pr_warn("copy_from_user failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_from_user outside whitelist\n");
    if (copy_from_user(buf + offset - 1, user_addr, size)) {
    pr_warn("copy_from_user failed, but lacked Oops\n");
    goto free_user;
    }
    }
    pr_err("FAIL: bad usercopy not detected!\n");
    pr_expected_config_param(CONFIG_HARDENED_USERCOPY, "hardened_usercopy");
    free_user:
    vm_munmap(user_alloc, PAGE_SIZE);
    free_alloc:
    if (buf)
    kmem_cache_free(whitelist_cache, buf);
    }
// Callable tests.
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_SLAB_SIZE_TO() {
    static void lkdtm_USERCOPY_SLAB_SIZE_TO(void)
    {
    do_usercopy_slab_size(true);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_SLAB_SIZE_FROM() {
    static void lkdtm_USERCOPY_SLAB_SIZE_FROM(void)
    {
    do_usercopy_slab_size(false);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_SLAB_WHITELIST_TO() {
    static void lkdtm_USERCOPY_SLAB_WHITELIST_TO(void)
    {
    do_usercopy_slab_whitelist(true);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_SLAB_WHITELIST_FROM() {
    static void lkdtm_USERCOPY_SLAB_WHITELIST_FROM(void)
    {
    do_usercopy_slab_whitelist(false);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_STACK_FRAME_TO() {
    static void lkdtm_USERCOPY_STACK_FRAME_TO(void)
    {
    do_usercopy_stack(true, true);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_STACK_FRAME_FROM() {
    static void lkdtm_USERCOPY_STACK_FRAME_FROM(void)
    {
    do_usercopy_stack(false, true);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_STACK_BEYOND() {
    static void lkdtm_USERCOPY_STACK_BEYOND(void)
    {
    do_usercopy_stack(true, false);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_KERNEL() {
    static void lkdtm_USERCOPY_KERNEL(void)
    {
    unsigned long user_addr;
    user_addr = vm_mmap(core::ptr::null_mut(), 0, PAGE_SIZE,
    PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_ANONYMOUS | MAP_PRIVATE, 0);
    if (user_addr >= TASK_SIZE) {
    pr_warn("Failed to allocate user memory\n");
    return;
    }
    pr_info("attempting good copy_to_user from kernel rodata: %px\n",
    test_text);
    if (copy_to_user((void __user *)user_addr, test_text,
    unconst + sizeof(test_text))) {
    pr_warn("copy_to_user failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_to_user from kernel text: %px\n",
    vm_mmap);
    if (copy_to_user((void __user *)user_addr, vm_mmap,
    unconst + PAGE_SIZE)) {
    pr_warn("copy_to_user failed, but lacked Oops\n");
    goto free_user;
    }
    pr_err("FAIL: bad copy_to_user() not detected!\n");
    pr_expected_config_param(CONFIG_HARDENED_USERCOPY, "hardened_usercopy");
    free_user:
    vm_munmap(user_addr, PAGE_SIZE);
    }
//
// This expects "kaddr" to point to a PAGE_SIZE allocation, which means
// a more complete test that would include copy_from_user() would risk
// memory corruption. Just test copy_to_user() here, as that exercises
// almost exactly the same code paths.
//
#[no_mangle]
unsafe extern "C" fn do_usercopy_page_span(name: *const c_char, kaddr: *mut c_void) {
    static void do_usercopy_page_span(const char *name, void *kaddr)
    {
    unsigned long uaddr;
    uaddr = vm_mmap(core::ptr::null_mut(), 0, PAGE_SIZE, PROT_READ | PROT_WRITE,
    MAP_ANONYMOUS | MAP_PRIVATE, 0);
    if (uaddr >= TASK_SIZE) {
    pr_warn("Failed to allocate user memory\n");
    return;
    }
// Initialize contents.
    memset(kaddr, 0xAA, PAGE_SIZE);
// Bump the kaddr forward to detect a page-spanning overflow.
    kaddr += PAGE_SIZE / 2;
    pr_info("attempting good copy_to_user() from kernel %s: %px\n",
    name, kaddr);
    if (copy_to_user((void __user *)uaddr, kaddr,
    unconst + (PAGE_SIZE / 2))) {
    pr_err("copy_to_user() failed unexpectedly?!\n");
    goto free_user;
    }
    pr_info("attempting bad copy_to_user() from kernel %s: %px\n",
    name, kaddr);
    if (copy_to_user((void __user *)uaddr, kaddr, unconst + PAGE_SIZE)) {
    pr_warn("Good, copy_to_user() failed, but lacked Oops(?!)\n");
    goto free_user;
    }
    pr_err("FAIL: bad copy_to_user() not detected!\n");
    pr_expected_config_param(CONFIG_HARDENED_USERCOPY, "hardened_usercopy");
    free_user:
    vm_munmap(uaddr, PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_VMALLOC() {
    static void lkdtm_USERCOPY_VMALLOC(void)
    {
    void *addr;
    addr = vmalloc(PAGE_SIZE);
    if (!addr) {
    pr_err("vmalloc() failed!?\n");
    return;
    }
    do_usercopy_page_span("vmalloc", addr);
    vfree(addr);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_USERCOPY_FOLIO() {
    static void lkdtm_USERCOPY_FOLIO(void)
    {
    struct folio *folio;
    void *addr;
//
// FIXME: Folio checking currently misses 0-order allocations, so
// allocate and bump forward to the last page.
//
    folio = folio_alloc(GFP_KERNEL | __GFP_ZERO, 1);
    if (!folio) {
    pr_err("folio_alloc() failed!?\n");
    return;
    }
    addr = folio_address(folio);
    if (addr)
    do_usercopy_page_span("folio", addr + PAGE_SIZE);
    else
    pr_err("folio_address() failed?!\n");
    folio_put(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn lkdtm_usercopy_init() -> void __init {
    void __init lkdtm_usercopy_init(void)
    {
// Prepare cache that lacks SLAB_USERCOPY flag.
    whitelist_cache =
    kmem_cache_create_usercopy("lkdtm-usercopy", cache_size,
    0, 0,
    cache_size / 4,
    cache_size / 16,
    core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn lkdtm_usercopy_exit() -> void __exit {
    void __exit lkdtm_usercopy_exit(void)
    {
    kmem_cache_destroy(whitelist_cache);
    }
    static struct crashtype crashtypes[] = {
    CRASHTYPE(USERCOPY_SLAB_SIZE_TO),
    CRASHTYPE(USERCOPY_SLAB_SIZE_FROM),
    CRASHTYPE(USERCOPY_SLAB_WHITELIST_TO),
    CRASHTYPE(USERCOPY_SLAB_WHITELIST_FROM),
    CRASHTYPE(USERCOPY_STACK_FRAME_TO),
    CRASHTYPE(USERCOPY_STACK_FRAME_FROM),
    CRASHTYPE(USERCOPY_STACK_BEYOND),
    CRASHTYPE(USERCOPY_VMALLOC),
    CRASHTYPE(USERCOPY_FOLIO),
    CRASHTYPE(USERCOPY_KERNEL),
    };
    struct crashtype_category usercopy_crashtypes = {
    .crashtypes = crashtypes,
    .len	    = ARRAY_SIZE(crashtypes),
    };
