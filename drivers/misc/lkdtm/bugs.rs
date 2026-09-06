//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lkdtm/bugs.c
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
// This is for all the tests related to logic bugs (e.g. bad dereferences,
// bad alignment, bad loops, bad locking, bad scheduling, deep stacks, and
// lockups) along with other things that don't fit well into existing LKDTM
// test source files.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lkdtm_list {
    pub node: list_head,
}

//
// Make sure our attempts to over run the kernel stack doesn't trigger
// a compiler warning when CONFIG_FRAME_WARN is set. Then make sure we
// recurse past the end of THREAD_SIZE by default.
//

    let mut recur_count: static int = REC_NUM_DEFAULT;
    static DEFINE_SPINLOCK(lock_me_up);
//
// Make sure compiler does not optimize this function or stack frame away:
// - function marked noinline
// - stack variables are marked volatile
// - stack variables are written (memset()) and read (buf[..] passed as arg)
// - function may have external effects (memzero_explicit())
// - no tail recursion possible
//
#[no_mangle]
unsafe extern "C" fn recursive_loop(remaining: c_int) -> int noinline {
    static int noinline recursive_loop(int remaining)
    {
    volatile char buf[REC_STACK_SIZE];
    volatile int ret;
    memset((void *)buf, remaining & 0xFF, sizeof(buf));
    if (!remaining)
    ret = 0;
    else
    ret = recursive_loop((int)buf[remaining % sizeof(buf)] - 1);
    memzero_explicit((void *)buf, sizeof(buf));
    return ret;
    }
// If the depth is negative, use the default, otherwise keep parameter.
#[no_mangle]
pub unsafe extern "C" fn lkdtm_bugs_init(recur_param: *mut c_int) -> void __init {
    void __init lkdtm_bugs_init(int *recur_param)
    {
    if (*recur_param < 0)
// recur_param = recur_count;
    else
    recur_count = *recur_param;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_PANIC() {
    static void lkdtm_PANIC(void)
    {
    panic("dumptest");
    }
#[no_mangle]
unsafe extern "C" fn panic_stop_irqoff_fn(arg: *mut c_void) -> c_int {
    static int panic_stop_irqoff_fn(void *arg)
    {
    atomic_t *v = arg;
//
// As stop_machine() disables interrupts, all CPUs within this function
// have interrupts disabled and cannot take a regular IPI.
//
// The last CPU which enters here will trigger a panic, and as all CPUs
// cannot take a regular IPI, we'll only be able to stop secondaries if
// smp_send_stop() or crash_smp_send_stop() uses an NMI.
//
    if (atomic_inc_return(v) == num_online_cpus())
    panic("panic stop irqoff test");
    for (;;)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_PANIC_STOP_IRQOFF() {
    static void lkdtm_PANIC_STOP_IRQOFF(void)
    {
    let mut v: core::sync::atomic::AtomicI32 = ATOMIC_INIT(0);
    stop_machine(panic_stop_irqoff_fn, &v, cpu_online_mask);
    }
    static bool wait_for_panic;
#[no_mangle]
unsafe extern "C" fn panic_in_hardirq(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart panic_in_hardirq(struct hrtimer *timer)
    {
    panic("from hard IRQ context");
    wait_for_panic = false;
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_PANIC_IN_HARDIRQ() {
    static void lkdtm_PANIC_IN_HARDIRQ(void)
    {
    struct hrtimer timer;
    wait_for_panic = true;
    hrtimer_setup_on_stack(&timer, panic_in_hardirq,
    CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    hrtimer_start(&timer, us_to_ktime(100), HRTIMER_MODE_REL_HARD);
    while (READ_ONCE(wait_for_panic))
    cpu_relax();
    hrtimer_cancel(&timer);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_BUG() {
    static void lkdtm_BUG(void)
    {
    BUG();
    }
    static bool wait_for_bug;
#[no_mangle]
unsafe extern "C" fn bug_in_hardirq(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart bug_in_hardirq(struct hrtimer *timer)
    {
    BUG();
    wait_for_bug = false;
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_BUG_IN_HARDIRQ() {
    static void lkdtm_BUG_IN_HARDIRQ(void)
    {
    struct hrtimer timer;
    wait_for_bug = true;
    hrtimer_setup_on_stack(&timer, bug_in_hardirq,
    CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    hrtimer_start(&timer, us_to_ktime(100), HRTIMER_MODE_REL_HARD);
    while (READ_ONCE(wait_for_bug))
    cpu_relax();
    hrtimer_cancel(&timer);
    }
    static int warn_counter;
#[no_mangle]
unsafe extern "C" fn lkdtm_WARNING() {
    static void lkdtm_WARNING(void)
    {
    WARN_ON(++warn_counter);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_WARNING_MESSAGE() {
    static void lkdtm_WARNING_MESSAGE(void)
    {
    WARN(1, "Warning message trigger count: %d\n", ++warn_counter);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_EXCEPTION() {
    static void lkdtm_EXCEPTION(void)
    {
// ((volatile int *) 0) = 0;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_LOOP() {
    static void lkdtm_LOOP(void)
    {
    for (;;)
    ;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_EXHAUST_STACK() {
    static void lkdtm_EXHAUST_STACK(void)
    {
    pr_info("Calling function with %lu frame size to depth %d ...\n",
    REC_STACK_SIZE, recur_count);
    recursive_loop(recur_count);
    pr_info("FAIL: survived without exhausting stack?!\n");
    }
#[no_mangle]
unsafe extern "C" fn __lkdtm_CORRUPT_STACK(stack: *mut c_void) -> noinline void {
    static noinline void __lkdtm_CORRUPT_STACK(void *stack)
    {
    memset(stack, '\xff', 64);
    }
// This should trip the stack canary, not corrupt the return address.
#[no_mangle]
unsafe extern "C" fn lkdtm_CORRUPT_STACK() -> noinline void {
    static noinline void lkdtm_CORRUPT_STACK(void)
    {
// Use default char array length that triggers stack protection.
    char data[8] __aligned(sizeof(void *));
    pr_info("Corrupting stack containing char array ...\n");
    __lkdtm_CORRUPT_STACK((void *)&data);
    }
// Same as above but will only get a canary with -fstack-protector-strong
#[no_mangle]
unsafe extern "C" fn lkdtm_CORRUPT_STACK_STRONG() -> noinline void {
    static noinline void lkdtm_CORRUPT_STACK_STRONG(void)
    {
    union {
    unsigned short shorts[4];
    unsigned long *ptr;
    } data __aligned(sizeof(void *));
    pr_info("Corrupting stack containing union ...\n");
    __lkdtm_CORRUPT_STACK((void *)&data);
    }
    static pid_t stack_pid;
    static unsigned long stack_addr;
#[no_mangle]
unsafe extern "C" fn lkdtm_REPORT_STACK() {
    static void lkdtm_REPORT_STACK(void)
    {
    volatile uintptr_t magic;
    let mut pid: pid_t = task_pid_nr(current);
    if (pid != stack_pid) {
    pr_info("Starting stack offset tracking for pid %d\n", pid);
    stack_pid = pid;
    stack_addr = (uintptr_t)&magic;
    }
    pr_info("Stack offset: %d\n", (int)(stack_addr - (uintptr_t)&magic));
    }
    static pid_t stack_canary_pid;
    static unsigned long stack_canary;
    static unsigned long stack_canary_offset;
#[no_mangle]
unsafe extern "C" fn __lkdtm_REPORT_STACK_CANARY(stack: *mut c_void) -> noinline void {
    static noinline void __lkdtm_REPORT_STACK_CANARY(void *stack)
    {
    let mut i: c_int = 0;
    let mut pid: pid_t = task_pid_nr(current);
    unsigned long *canary = (unsigned long *)stack;
    let mut current_offset: c_ulong = 0, init_offset = 0;
// Do our best to find the canary in a 16 word window ...
    for (i = 1; i < 16; i++) {
    canary = (unsigned long *)stack + i;

    if (*canary == current.stack_canary)
    current_offset = i;
    if (*canary == init_task.stack_canary)
    init_offset = i;

    }
    if (current_offset == 0) {
//
// If the canary doesn't match what's in the task_struct,
// we're either using a global canary or the stack frame
// layout changed.
//
    if (init_offset != 0) {
    pr_err("FAIL: global stack canary found at offset %ld (canary for pid %d matches init_task's)!\n",
    init_offset, pid);
    } else {
    pr_warn("FAIL: did not correctly locate stack canary :(\n");
    pr_expected_config(CONFIG_STACKPROTECTOR);
    }
    return;
    } else if (init_offset != 0) {
    pr_warn("WARNING: found both current and init_task canaries nearby?!\n");
    }
    canary = (unsigned long *)stack + current_offset;
    if (stack_canary_pid == 0) {
    stack_canary = *canary;
    stack_canary_pid = pid;
    stack_canary_offset = current_offset;
    pr_info("Recorded stack canary for pid %d at offset %ld\n",
    stack_canary_pid, stack_canary_offset);
    } else if (pid == stack_canary_pid) {
    pr_warn("ERROR: saw pid %d again -- please use a new pid\n", pid);
    } else {
    if (current_offset != stack_canary_offset) {
    pr_warn("ERROR: canary offset changed from %ld to %ld!?\n",
    stack_canary_offset, current_offset);
    return;
    }
    if (*canary == stack_canary) {
    pr_warn("FAIL: canary identical for pid %d and pid %d at offset %ld!\n",
    stack_canary_pid, pid, current_offset);
    } else {
    pr_info("ok: stack canaries differ between pid %d and pid %d at offset %ld.\n",
    stack_canary_pid, pid, current_offset);
// Reset the test.
    stack_canary_pid = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_REPORT_STACK_CANARY() {
    static void lkdtm_REPORT_STACK_CANARY(void)
    {
// Use default char array length that triggers stack protection.
    char data[8] __aligned(sizeof(void *)) = { };
    __lkdtm_REPORT_STACK_CANARY((void *)&data);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_UNALIGNED_LOAD_STORE_WRITE() {
    static void lkdtm_UNALIGNED_LOAD_STORE_WRITE(void)
    {
    static u8 data[5] __attribute__((aligned(4))) = {1, 2, 3, 4, 5};
    u32 *p;
    let mut val: u32 = 0x12345678;
    p = (u32 *)(data + 1);
    if (*p == 0)
    val = 0x87654321;
// p = val;
    if (IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))
    pr_err("XFAIL: arch has CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS\n");
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_SOFTLOCKUP() {
    static void lkdtm_SOFTLOCKUP(void)
    {
    preempt_disable();
    for (;;)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_HARDLOCKUP() {
    static void lkdtm_HARDLOCKUP(void)
    {
    local_irq_disable();
    for (;;)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn __lkdtm_SMP_CALL_LOCKUP(unused: *mut c_void) {
    static void __lkdtm_SMP_CALL_LOCKUP(void *unused)
    {
    for (;;)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_SMP_CALL_LOCKUP() {
    static void lkdtm_SMP_CALL_LOCKUP(void)
    {
    unsigned int cpu, target;
    cpus_read_lock();
    cpu = get_cpu();
    target = cpumask_any_but(cpu_online_mask, cpu);
    if (target >= nr_cpu_ids) {
    pr_err("FAIL: no other online CPUs\n");
    goto out_put_cpus;
    }
    smp_call_function_single(target, __lkdtm_SMP_CALL_LOCKUP, core::ptr::null_mut(), 1);
    pr_err("FAIL: did not hang\n");
    out_put_cpus:
    put_cpu();
    cpus_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_SPINLOCKUP() {
    static void lkdtm_SPINLOCKUP(void)
    {
// Must be called twice to trigger.
    spin_lock(&lock_me_up);
// Let sparse know we intended to exit holding the lock.
    __release(&lock_me_up);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_HUNG_TASK() -> void __noreturn {
    static void __noreturn lkdtm_HUNG_TASK(void)
    {
    set_current_state(TASK_UNINTERRUPTIBLE);
    schedule();
    BUG();
    }
    let mut huge: static volatile unsigned int = INT_MAX - 2;
    static volatile unsigned int ignored;
#[no_mangle]
unsafe extern "C" fn lkdtm_OVERFLOW_SIGNED() {
    static void lkdtm_OVERFLOW_SIGNED(void)
    {
    int value;
    value = huge;
    pr_info("Normal signed addition ...\n");
    value += 1;
    ignored = value;
    pr_info("Overflowing signed addition ...\n");
    value += 4;
    ignored = value;
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_OVERFLOW_UNSIGNED() {
    static void lkdtm_OVERFLOW_UNSIGNED(void)
    {
    unsigned int value;
    value = huge;
    pr_info("Normal unsigned addition ...\n");
    value += 1;
    ignored = value;
    pr_info("Overflowing unsigned addition ...\n");
    value += 4;
    ignored = value;
    }
// Intentionally using unannotated flex array definition.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_bounds_flex_array {
    pub one: c_int,
    pub two: c_int,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_bounds {
    pub one: c_int,
    pub two: c_int,
    pub data: [c_char; 8],
    pub three: c_int,
}

#[no_mangle]
unsafe extern "C" fn lkdtm_ARRAY_BOUNDS() {
    static void lkdtm_ARRAY_BOUNDS(void)
    {
    struct array_bounds_flex_array *not_checked;
    struct array_bounds *checked;
    volatile int i;
    not_checked = kmalloc(sizeof(*not_checked) * 2, GFP_KERNEL);
    checked = kmalloc(sizeof(*checked) * 2, GFP_KERNEL);
    if (!not_checked || !checked) {
    kfree(not_checked);
    kfree(checked);
    return;
    }
    pr_info("Array access within bounds ...\n");
// For both, touch all bytes in the actual member size.
    for (i = 0; i < sizeof(checked.data); i++)
    checked.data[i] = 'A';
//
// For the uninstrumented flex array member, also touch 1 byte
// beyond to verify it is correctly uninstrumented.
//
    for (i = 0; i < 2; i++)
    not_checked.data[i] = 'A';
    pr_info("Array access beyond bounds ...\n");
    for (i = 0; i < sizeof(checked.data) + 1; i++)
    checked.data[i] = 'B';
    kfree(not_checked);
    kfree(checked);
    pr_err("FAIL: survived array bounds overflow!\n");
    if (IS_ENABLED(CONFIG_UBSAN_BOUNDS))
    pr_expected_config(CONFIG_UBSAN_TRAP);
    else
    pr_expected_config(CONFIG_UBSAN_BOUNDS);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lkdtm_cb_fam {
    pub flags: c_ulong,
    pub count: c_int,
    pub __counted_by(count): int array[],
}

    let mut element_count: static volatile int = 4;
#[no_mangle]
unsafe extern "C" fn lkdtm_FAM_BOUNDS() {
    static void lkdtm_FAM_BOUNDS(void)
    {
    struct lkdtm_cb_fam *inst;
    inst = kzalloc_flex(*inst, array, element_count + 1);
    if (!inst) {
    pr_err("FAIL: could not allocate test struct!\n");
    return;
    }
    inst.count = element_count;
    pr_info("Array access within bounds ...\n");
    inst.array[1] = element_count;
    ignored = inst.array[1];
    pr_info("Array access beyond bounds ...\n");
    inst.array[element_count] = element_count;
    ignored = inst.array[element_count];
    kfree(inst);
    pr_err("FAIL: survived access of invalid flexible array member index!\n");
    if (!IS_ENABLED(CONFIG_CC_HAS_COUNTED_BY))
    pr_warn("This is expected since this %s was built with a compiler that does not support __counted_by\n",
    lkdtm_kernel_info);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ENABLED(CONFIG_UBSAN_BOUNDS)) -> else {
    else if (IS_ENABLED(CONFIG_UBSAN_BOUNDS))
    pr_expected_config(CONFIG_UBSAN_TRAP);
    else
    pr_expected_config(CONFIG_UBSAN_BOUNDS);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lkdtm_extra {
    pub b: short a,,
    pub sixteen: u16,
    pub bigger: u32,
    pub biggest: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lkdtm_cb_ptr {
    pub c: int a, b,,
    pub nr_extra: c_int,
    pub __counted_by_ptr(len): *mut *mut char buf,
    pub len: usize,
    pub __counted_by_ptr(nr_extra): *mut *mut lkdtm_extra extra,
}

#[no_mangle]
unsafe extern "C" fn check_ptr_len(p: *mut lkdtm_cb_ptr, len: usize) -> noinline void {
    static noinline void check_ptr_len(struct lkdtm_cb_ptr *p, size_t len)
    {
    if (__member_size(p.buf) != len)
    pr_err("FAIL: could not determine size of inst.buf: %zu\n",
    __member_size(p.buf));
    else
    pr_info("good: inst.buf length is %zu\n", len);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_PTR_BOUNDS() {
    static void lkdtm_PTR_BOUNDS(void)
    {
    struct lkdtm_cb_ptr *inst;
    inst = kzalloc_obj(*inst);
    if (!inst) {
    pr_err("FAIL: could not allocate struct lkdtm_cb_ptr!\n");
    return;
    }
    inst.buf = kzalloc(element_count, GFP_KERNEL);
    if (!inst.buf) {
    pr_err("FAIL: could not allocate inst.buf!\n");
    return;
    }
    inst.len = element_count;
// Double element_count
    inst.extra = kzalloc_objs(*inst.extra, element_count * 2);
    inst.nr_extra = element_count * 2;
    pr_info("Pointer access within bounds ...\n");
    check_ptr_len(inst, 4);
// All 4 bytes
    inst.buf[0] = 'A';
    inst.buf[1] = 'B';
    inst.buf[2] = 'C';
    inst.buf[3] = 'D';
// Halfway into the array
    inst.extra[element_count].biggest = 0x1000;
    pr_info("Pointer access beyond bounds ...\n");
    ignored = inst.extra[inst.nr_extra].b;
    kfree(inst.extra);
    kfree(inst.buf);
    kfree(inst);
    pr_err("FAIL: survived access of invalid pointer member offset!\n");
    if (!IS_ENABLED(CONFIG_CC_HAS_COUNTED_BY_PTR))
    pr_warn("This is expected since this %s was built with a compiler that does not support __counted_by_ptr\n",
    lkdtm_kernel_info);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ENABLED(CONFIG_UBSAN_BOUNDS)) -> else {
    else if (IS_ENABLED(CONFIG_UBSAN_BOUNDS))
    pr_expected_config(CONFIG_UBSAN_TRAP);
    else
    pr_expected_config(CONFIG_UBSAN_BOUNDS);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_CORRUPT_LIST_ADD() {
    static void lkdtm_CORRUPT_LIST_ADD(void)
    {
//
// Initially, an empty list via LIST_HEAD:
// test_head.next = &test_head
// test_head.prev = &test_head
//
    LIST_HEAD(test_head);
    struct lkdtm_list good, bad;
    void *target[2] = { };
    void *redirection = &target;
    pr_info("attempting good list addition\n");
//
// Adding to the list performs these actions:
// test_head.next->prev = &good.node
// good.node.next = test_head.next
// good.node.prev = test_head
// test_head.next = good.node
//
    list_add(&good.node, &test_head);
    pr_info("attempting corrupted list addition\n");
//
// In simulating this "write what where" primitive, the "what" is
// the address of &bad.node, and the "where" is the address held
// by "redirection".
//
    test_head.next = redirection;
    list_add(&bad.node, &test_head);
    if (target[0] == core::ptr::null_mut() && target[1] == core::ptr::null_mut())
    pr_err("Overwrite did not happen, but no BUG?!\n");
    else {
    pr_err("list_add() corruption not detected!\n");
    pr_expected_config(CONFIG_LIST_HARDENED);
    }
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_CORRUPT_LIST_DEL() {
    static void lkdtm_CORRUPT_LIST_DEL(void)
    {
    LIST_HEAD(test_head);
    struct lkdtm_list item;
    void *target[2] = { };
    void *redirection = &target;
    list_add(&item.node, &test_head);
    pr_info("attempting good list removal\n");
    list_del(&item.node);
    pr_info("attempting corrupted list removal\n");
    list_add(&item.node, &test_head);
// As with the list_add() test above, this corrupts "next".
    item.node.next = redirection;
    list_del(&item.node);
    if (target[0] == core::ptr::null_mut() && target[1] == core::ptr::null_mut())
    pr_err("Overwrite did not happen, but no BUG?!\n");
    else {
    pr_err("list_del() corruption not detected!\n");
    pr_expected_config(CONFIG_LIST_HARDENED);
    }
    }
// Test that VMAP_STACK is actually allocating with a leading guard page
#[no_mangle]
unsafe extern "C" fn lkdtm_STACK_GUARD_PAGE_LEADING() {
    static void lkdtm_STACK_GUARD_PAGE_LEADING(void)
    {
    const unsigned char *stack = task_stack_page(current);
    const unsigned char *ptr = stack - 1;
    volatile unsigned char byte;
    pr_info("attempting bad read from page below current stack\n");
    byte = *ptr;
    pr_err("FAIL: accessed page before stack! (byte: %x)\n", byte);
    }
// Test that VMAP_STACK is actually allocating with a trailing guard page
#[no_mangle]
unsafe extern "C" fn lkdtm_STACK_GUARD_PAGE_TRAILING() {
    static void lkdtm_STACK_GUARD_PAGE_TRAILING(void)
    {
    const unsigned char *stack = task_stack_page(current);
    const unsigned char *ptr = stack + THREAD_SIZE;
    volatile unsigned char byte;
    pr_info("attempting bad read from page above current stack\n");
    byte = *ptr;
    pr_err("FAIL: accessed page after stack! (byte: %x)\n", byte);
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_UNSET_SMEP() {
    static void lkdtm_UNSET_SMEP(void)
    {

pub const MOV_CR4_DEPTH: c_int = 64;
    void (*direct_write_cr4)(unsigned long val);
    unsigned char *insn;
    unsigned long cr4;
    int i;
    cr4 = native_read_cr4();
    if ((cr4 & X86_CR4_SMEP) != X86_CR4_SMEP) {
    pr_err("FAIL: SMEP not in use\n");
    return;
    }
    cr4 &= ~(X86_CR4_SMEP);
    pr_info("trying to clear SMEP normally\n");
    native_write_cr4(cr4);
    if (cr4 == native_read_cr4()) {
    pr_err("FAIL: pinning SMEP failed!\n");
    cr4 |= X86_CR4_SMEP;
    pr_info("restoring SMEP\n");
    native_write_cr4(cr4);
    return;
    }
    pr_info("ok: SMEP did not get cleared\n");
//
// To test the post-write pinning verification we need to call
// directly into the middle of native_write_cr4() where the
// cr4 write happens, skipping any pinning. This searches for
// the cr4 writing instruction.
//
    insn = (unsigned char *)native_write_cr4;
    OPTIMIZER_HIDE_VAR(insn);
    for (i = 0; i < MOV_CR4_DEPTH; i++) {
// mov %rdi, %cr4
    if (insn[i] == 0x0f && insn[i+1] == 0x22 && insn[i+2] == 0xe7)
    break;
// mov %rdi,%rax; mov %rax, %cr4
    if (insn[i]   == 0x48 && insn[i+1] == 0x89 &&
    insn[i+2] == 0xf8 && insn[i+3] == 0x0f &&
    insn[i+4] == 0x22 && insn[i+5] == 0xe0)
    break;
    }
    if (i >= MOV_CR4_DEPTH) {
    pr_info("ok: cannot locate cr4 writing call gadget\n");
    return;
    }
    direct_write_cr4 = (void *)(insn + i);
    pr_info("trying to clear SMEP with call gadget\n");
    direct_write_cr4(cr4);
    if (native_read_cr4() & X86_CR4_SMEP) {
    pr_info("ok: SMEP removal was reverted\n");
    } else {
    pr_err("FAIL: cleared SMEP not detected!\n");
    cr4 |= X86_CR4_SMEP;
    pr_info("restoring SMEP\n");
    native_write_cr4(cr4);
    }

    pr_err("XFAIL: this test is x86_64-only\n");

    }
#[no_mangle]
unsafe extern "C" fn lkdtm_DOUBLE_FAULT() {
    static void lkdtm_DOUBLE_FAULT(void)
    {

//
// Trigger #DF by setting the stack limit to zero.  This clobbers
// a GDT TLS slot, which is okay because the current task will die
// anyway due to the double fault.
//
    struct desc_struct d = {
    .type = 3,	/* expand-up, writable, accessed data */
    .p = 1,		/* present */
    .d = 1,		/* 32-bit */
    .g = 0,		/* limit in bytes */
    .s = 1,		/* not system */
    };
    local_irq_disable();
    write_gdt_entry(get_cpu_gdt_rw(smp_processor_id()),
    GDT_ENTRY_TLS_MIN, &d, DESCTYPE_S);
//
// Put our zero-limit segment in SS and then trigger a fault.  The
// 4-byte access to (%esp) will fault with #SS, and the attempt to
// deliver the fault will recursively cause #SS and result in #DF.
// This whole process happens while NMIs and MCEs are blocked by the
// MOV SS window.  This is nice because an NMI with an invalid SS
// would also double-fault, resulting in the NMI or MCE being lost.
//
    asm volatile ("movw %0, %%ss; addl $0, (%%esp)" ::
    "r" ((unsigned short)(GDT_ENTRY_TLS_MIN << 3)));
    pr_err("FAIL: tried to double fault but didn't die\n");

    pr_err("XFAIL: this test is ia32-only\n");

    }

#[no_mangle]
unsafe extern "C" fn change_pac_parameters() -> noinline void {
    static noinline void change_pac_parameters(void)
    {
    if (IS_ENABLED(CONFIG_ARM64_PTR_AUTH_KERNEL)) {
// Reset the keys of current task
    ptrauth_thread_init_kernel(current);
    ptrauth_thread_switch_kernel(current);
    }
    }

#[no_mangle]
unsafe extern "C" fn lkdtm_CORRUPT_PAC() -> noinline void {
    static noinline void lkdtm_CORRUPT_PAC(void)
    {

pub const CORRUPT_PAC_ITERATE: c_int = 10;
    int i;
    if (!IS_ENABLED(CONFIG_ARM64_PTR_AUTH_KERNEL))
    pr_err("FAIL: kernel not built with CONFIG_ARM64_PTR_AUTH_KERNEL\n");
    if (!system_supports_address_auth()) {
    pr_err("FAIL: CPU lacks pointer authentication feature\n");
    return;
    }
    pr_info("changing PAC parameters to force function return failure...\n");
//
// PAC is a hash value computed from input keys, return address and
// stack pointer. As pac has fewer bits so there is a chance of
// collision, so iterate few times to reduce the collision probability.
//
    for (i = 0; i < CORRUPT_PAC_ITERATE; i++)
    change_pac_parameters();
    pr_err("FAIL: survived PAC changes! Kernel may be unstable from here\n");

    pr_err("XFAIL: this test is arm64-only\n");

    }
#[no_mangle]
unsafe extern "C" fn lkdtm_EFI_RUNTIME_CRASH() -> void __maybe_unused {
    static void __maybe_unused lkdtm_EFI_RUNTIME_CRASH(void)
    {
    let mut __ro_after_init: static unsigned long size = sizeof(efi_char16_t);
    efi_status_t status;
    if (!efi.get_next_variable ||
    !efi_enabled(EFI_RUNTIME_SERVICES) ||
    !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_NEXT_VARIABLE_NAME)) {
    pr_err("FAIL: EFI GetNextVariableName() is not available\n");
    return;
    }
//
// Provoke a fault by asking the firmware to write to a read-only
// variable.
//
    status = efi.get_next_variable(&size, L"", &(efi_guid_t){});
    if (status != EFI_ABORTED || efi_enabled(EFI_RUNTIME_SERVICES))
    pr_err("FAIL: EFI GetNextVariable() did not abort (%#lx)\n",
    status);
    }
    static struct crashtype crashtypes[] = {
    CRASHTYPE(PANIC),
    CRASHTYPE(PANIC_STOP_IRQOFF),
    CRASHTYPE(PANIC_IN_HARDIRQ),
    CRASHTYPE(BUG),
    CRASHTYPE(BUG_IN_HARDIRQ),
    CRASHTYPE(WARNING),
    CRASHTYPE(WARNING_MESSAGE),
    CRASHTYPE(EXCEPTION),
    CRASHTYPE(LOOP),
    CRASHTYPE(EXHAUST_STACK),
    CRASHTYPE(CORRUPT_STACK),
    CRASHTYPE(CORRUPT_STACK_STRONG),
    CRASHTYPE(REPORT_STACK),
    CRASHTYPE(REPORT_STACK_CANARY),
    CRASHTYPE(UNALIGNED_LOAD_STORE_WRITE),
    CRASHTYPE(SOFTLOCKUP),
    CRASHTYPE(HARDLOCKUP),
    CRASHTYPE(SMP_CALL_LOCKUP),
    CRASHTYPE(SPINLOCKUP),
    CRASHTYPE(HUNG_TASK),
    CRASHTYPE(OVERFLOW_SIGNED),
    CRASHTYPE(OVERFLOW_UNSIGNED),
    CRASHTYPE(ARRAY_BOUNDS),
    CRASHTYPE(FAM_BOUNDS),
    CRASHTYPE(PTR_BOUNDS),
    CRASHTYPE(CORRUPT_LIST_ADD),
    CRASHTYPE(CORRUPT_LIST_DEL),
    CRASHTYPE(STACK_GUARD_PAGE_LEADING),
    CRASHTYPE(STACK_GUARD_PAGE_TRAILING),
    CRASHTYPE(UNSET_SMEP),
    CRASHTYPE(DOUBLE_FAULT),
    CRASHTYPE(CORRUPT_PAC),

    CRASHTYPE(EFI_RUNTIME_CRASH),

    };
    struct crashtype_category bugs_crashtypes = {
    .crashtypes = crashtypes,
    .len	    = ARRAY_SIZE(crashtypes),
    };
