//! Automatically rewritten from C to Rust
//! Source: init/main.c
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
//
// linux/init/main.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// GK 2/5/95  -  Changed to support mounting root fs via NFS
// Added initrd & change_root: Werner Almesberger & Hans Lermen, Feb '96
// Moan early if gcc is old, avoiding bogus kernels - Paul Gortmaker, May '96
// Simplified starting of init:  Michael A. Griffith <grif@acm.org>
//

// Macro flag: #define CREATE_TRACE_POINTS

    static int kernel_init(void *);
//
// Debug helper: via this flag we know that we are in 'early bootup code'
// where only the boot processor is running with IRQ disabled.  This means
// two things - IRQ must not be enabled before the flag is cleared and some
// operations which are not allowed with IRQ disabled are allowed while the
// flag is set.
//
    bool early_boot_irqs_disabled __read_mostly;
    enum system_states system_state __read_mostly;
    EXPORT_SYMBOL(system_state);
//
// Boot command-line arguments
//

// Default late time init is NULL. archs can override this later.
    void (*__initdata late_time_init)(void);
// Untouched command line saved by arch-specific code.
    char __initdata boot_command_line[COMMAND_LINE_SIZE];
// Untouched saved command line (eg. for /proc)
    char *saved_command_line __ro_after_init;
    unsigned int saved_command_line_len __ro_after_init;
// Command line for parameter parsing
    static char *static_command_line;
// Untouched extra command line
    static char *extra_command_line;
// Extra init arguments
    static char *extra_init_args;

// Is bootconfig on command line?
    static bool bootconfig_found;
    static size_t initargs_offs;

    static char *execute_command;
    static char *ramdisk_execute_command = "/init";
    static bool __initdata ramdisk_execute_command_set;
//
// Used to generate warnings if static_key manipulation functions are used
// before jump_label_init is called.
//
    bool static_key_initialized __read_mostly;
    EXPORT_SYMBOL_GPL(static_key_initialized);
//
// If set, this is an indication to the drivers that reset the underlying
// device before going ahead with the initialization otherwise driver might
// rely on the BIOS and skip the reset operation.
//
// This is useful if kernel is booting in an unreliable environment.
// For ex. kdump situation where previous kernel has crashed, BIOS has been
// skipped and devices will be in unknown state.
//
    unsigned int reset_devices;
    EXPORT_SYMBOL(reset_devices);
#[no_mangle]
unsafe extern "C" fn set_reset_devices(str: *mut c_char) -> int __init {
    static int __init set_reset_devices(char *str)
    {
    reset_devices = 1;
    return 1;
    }
    __setup("reset_devices", set_reset_devices);
    static const char *argv_init[MAX_INIT_ARGS+2] = { "init", core::ptr::null_mut(), };
    const char *envp_init[MAX_INIT_ENVS+2] = { "HOME=/", "TERM=linux", core::ptr::null_mut(), };
    static const char *panic_later, *panic_param;
#[no_mangle]
unsafe extern "C" fn obsolete_checksetup(line: *mut c_char) -> bool __init {
    static bool __init obsolete_checksetup(char *line)
    {
    const struct obs_kernel_param *p;
    let mut had_early_param: bool = false;
    p = __setup_start;
    do {
    let mut n: c_int = strlen(p.str);
    if (parameqn(line, p.str, n)) {
    if (p.early) {
// Already done in parse_early_param?
// (Needs exact match on param part).
// Keep iterating, as we can have early
// params and __setups of same names 8(
    if (line[n] == '\0' || line[n] == '=')
    had_early_param = true;
    } else if (!p.setup_func) {
    pr_warn("Parameter %s is obsolete, ignored\n",
    p.str);
    return true;
    } else if (p.setup_func(line + n))
    return true;
    }
    p++;
    } while (p < __setup_end);
    return had_early_param;
    }
//
// This should be approx 2 Bo*oMips to start (note initial shift), and will
// still work even if initially too large, it will just take slightly longer
//
    let mut loops_per_jiffy: c_ulong = (1<<12);
    EXPORT_SYMBOL(loops_per_jiffy);
#[no_mangle]
unsafe extern "C" fn debug_kernel(str: *mut c_char) -> int __init {
    static int __init debug_kernel(char *str)
    {
    console_loglevel = CONSOLE_LOGLEVEL_DEBUG;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn quiet_kernel(str: *mut c_char) -> int __init {
    static int __init quiet_kernel(char *str)
    {
    console_loglevel = CONSOLE_LOGLEVEL_QUIET;
    return 0;
    }
    early_param("debug", debug_kernel);
    early_param("quiet", quiet_kernel);
#[no_mangle]
unsafe extern "C" fn loglevel(str: *mut c_char) -> int __init {
    static int __init loglevel(char *str)
    {
    int newlevel;
//
// Only update loglevel value when a correct setting was passed,
// to prevent blind crashes (when loglevel being set to 0) that
// are quite hard to debug
//
    if (get_option(&str, &newlevel)) {
    console_loglevel = newlevel;
    return 0;
    }
    return -EINVAL;
    }
    early_param("loglevel", loglevel);

#[no_mangle]
unsafe extern "C" fn get_boot_config_from_initrd(_size: *mut usize) -> *mut void  __init {
    static void * __init get_boot_config_from_initrd(size_t *_size)
    {
    u32 size, csum;
    char *data;
    u8 *hdr;
    int i;
    if (!initrd_end)
    return core::ptr::null_mut();
    data = (char *)initrd_end - BOOTCONFIG_MAGIC_LEN;
//
// Since Grub may align the size of initrd to 4, we must
// check the preceding 3 bytes as well.
//
    for (i = 0; i < 4; i++) {
    if (!memcmp(data, BOOTCONFIG_MAGIC, BOOTCONFIG_MAGIC_LEN))
    goto found;
    data--;
    }
    return core::ptr::null_mut();
    found:
    hdr = (u8 *)(data - 8);
    size = get_unaligned_le32(hdr);
    csum = get_unaligned_le32(hdr + 4);
    data = ((void *)hdr) - size;
    if ((unsigned long)data < initrd_start) {
    pr_err("bootconfig size %d is greater than initrd size %ld\n",
    size, initrd_end - initrd_start);
    return core::ptr::null_mut();
    }
    if (xbc_calc_checksum(data, size) != csum) {
    pr_err("bootconfig checksum failed\n");
    return core::ptr::null_mut();
    }
// Remove bootconfig from initramfs/initrd
    initrd_end = (unsigned long)data;
    if (_size)
// _size = size;
    return data;
    }

#[no_mangle]
unsafe extern "C" fn get_boot_config_from_initrd(_size: *mut usize) -> *mut void  __init {
    static void * __init get_boot_config_from_initrd(size_t *_size)
    {
    return core::ptr::null_mut();
    }

// Make an extra command line under given key word
#[no_mangle]
unsafe extern "C" fn xbc_make_cmdline(key: *const c_char) -> *mut char  __init {
    static char * __init xbc_make_cmdline(const char *key)
    {
    struct xbc_node *root;
    char *new_cmdline;
    int ret, len = 0;
    root = xbc_find_node(key);
    if (!root)
    return core::ptr::null_mut();
// Count required buffer size
    len = xbc_snprint_cmdline(core::ptr::null_mut(), 0, root);
    if (len <= 0)
    return core::ptr::null_mut();
    new_cmdline = memblock_alloc(len + 1, SMP_CACHE_BYTES);
    if (!new_cmdline) {
    pr_err("Failed to allocate memory for extra kernel cmdline.\n");
    return core::ptr::null_mut();
    }
    ret = xbc_snprint_cmdline(new_cmdline, len + 1, root);
    if (ret < 0 || ret > len) {
    pr_err("Failed to print extra kernel cmdline.\n");
    memblock_free(new_cmdline, len + 1);
    return core::ptr::null_mut();
    }
    return new_cmdline;
    }
#[no_mangle]
unsafe extern "C" fn warn_bootconfig(str: *mut c_char) -> int __init {
    static int __init warn_bootconfig(char *str)
    {
// The 'bootconfig' option is handled by setup_boot_config().
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_boot_config() -> void __init {
    static void __init setup_boot_config(void)
    {
    const char *msg, *data;
    int pos, ret, offs;
    size_t size;
    let mut from_embedded: bool = false;
// Cut out the bootconfig data even if we have no bootconfig option
    data = get_boot_config_from_initrd(&size);
// If there is no bootconfig in initrd, try embedded one.
    if (!data) {
    data = xbc_get_embedded_bootconfig(&size);
    from_embedded = true;
    }
    bootconfig_found = bootconfig_cmdline_requested(boot_command_line, &offs);
    if (!(bootconfig_found || IS_ENABLED(CONFIG_BOOT_CONFIG_FORCE)))
    return;
// Offset of the init arguments after a "--", located by the helper.
    initargs_offs = offs;
    if (!data) {
// If user intended to use bootconfig, show an error level message
    if (bootconfig_found)
    pr_err("'bootconfig' found on command line, but no bootconfig found\n");
    else
    pr_info("No bootconfig data provided, so skipping bootconfig");
    return;
    }
    if (size >= XBC_DATA_MAX) {
    pr_err("bootconfig size %ld greater than max size %d\n",
    (long)size, XBC_DATA_MAX);
    return;
    }
    ret = xbc_init(data, size, &msg, &pos);
    if (ret < 0) {
    if (pos < 0)
    pr_err("Failed to init bootconfig: %s.\n", msg);
    else
    pr_err("Failed to parse bootconfig: %s at %d.\n",
    msg, pos);
    } else {
    xbc_get_info(&ret, core::ptr::null_mut());
    pr_info("Load bootconfig: %ld bytes %d nodes\n", (long)size, ret);
//
// keys starting with "kernel." are passed via cmdline. When
// this bootconfig came from the embedded source and
// setup_arch() already prepended the rendered "kernel" subtree
// to boot_command_line, rendering again here would duplicate
// the keys in saved_command_line and make accumulating handlers
// (console=, earlycon=, ...) re-register the same value. Skip
// only when the prepend really happened.
//
// On arches that do not select ARCH_SUPPORTS_CMDLINE_FROM_BOOTCONFIG,
// CONFIG_CMDLINE_FROM_BOOTCONFIG is unselectable and
// xbc_embedded_cmdline_applied() collapses to a stub returning
// false, so this path still runs and the embedded "kernel"
// keys reach the cmdline via the runtime parser exactly as
// before this series.
//
    if (!from_embedded || !xbc_embedded_cmdline_applied())
    extra_command_line = xbc_make_cmdline("kernel");
// Also, "init." keys are init arguments
    extra_init_args = xbc_make_cmdline("init");
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn exit_boot_config() -> void __init {
    static void __init exit_boot_config(void)
    {
    xbc_exit();
    }

#[no_mangle]
unsafe extern "C" fn setup_boot_config() -> void __init {
    static void __init setup_boot_config(void)
    {
// Remove bootconfig data from initrd
    get_boot_config_from_initrd(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn warn_bootconfig(str: *mut c_char) -> int __init {
    static int __init warn_bootconfig(char *str)
    {
    pr_warn("WARNING: 'bootconfig' found on the kernel command line but CONFIG_BOOT_CONFIG is not set.\n");
    return 0;
    }

    early_param("bootconfig", warn_bootconfig);
#[no_mangle]
pub unsafe extern "C" fn cmdline_has_extra_options() -> bool __init {
    bool __init cmdline_has_extra_options(void)
    {
    return extra_command_line || extra_init_args;
    }
// Change NUL term back to "=", to make "param" the whole string.
#[no_mangle]
unsafe extern "C" fn repair_env_string(param: *mut c_char, val: *mut c_char) -> void __init {
    static void __init repair_env_string(char *param, char *val)
    {
    if (val) {
// param=val or param="val"?
    if (val == param+strlen(param)+1)
    val[-1] = '=';
#[no_mangle]
pub unsafe extern "C" fn if(param+strlen(param)+2: val ==) -> else {
    val[-2] = '=';
    memmove(val-1, val, strlen(val)+1);
    } else
    BUG();
    }
    }
// Anything after -- gets handed straight to init.
    static int __init set_init_arg(char *param, char *val,
    const char *unused, void *arg)
    {
    unsigned int i;
    if (panic_later)
    return 0;
    repair_env_string(param, val);
    for (i = 0; argv_init[i]; i++) {
    if (i == MAX_INIT_ARGS) {
    panic_later = "init";
    panic_param = param;
    return 0;
    }
    }
    argv_init[i] = param;
    return 0;
    }
//
// Unknown boot options get handed to init, unless they look like
// unused parameters (modprobe will find them in /proc/cmdline).
//
    static int __init unknown_bootoption(char *param, char *val,
    const char *unused, void *arg)
    {
    let mut len: usize = strlen(param);
//
// Well-known bootloader identifiers:
// 1. LILO/Grub pass "BOOT_IMAGE=...";
// 2. kexec/kdump (kexec-tools) pass "kexec".
//
    const char *bootloader[] = { "BOOT_IMAGE=", "kexec", core::ptr::null_mut() };
// Handle params aliased to sysctls
    if (sysctl_is_alias(param))
    return 0;
    repair_env_string(param, val);
// Handle bootloader identifier
    for (int i = 0; bootloader[i]; i++) {
    if (strstarts(param, bootloader[i]))
    return 0;
    }
// Handle obsolete-style parameters
    if (obsolete_checksetup(param))
    return 0;
// Unused module parameter.
    if (strnchr(param, len, '.'))
    return 0;
    if (panic_later)
    return 0;
    if (val) {
// Environment option
    unsigned int i;
    for (i = 0; envp_init[i]; i++) {
    if (i == MAX_INIT_ENVS) {
    panic_later = "env";
    panic_param = param;
    }
    if (!strncmp(param, envp_init[i], len+1))
    break;
    }
    envp_init[i] = param;
    } else {
// Command line option
    unsigned int i;
    for (i = 0; argv_init[i]; i++) {
    if (i == MAX_INIT_ARGS) {
    panic_later = "init";
    panic_param = param;
    }
    }
    argv_init[i] = param;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_setup(str: *mut c_char) -> int __init {
    static int __init init_setup(char *str)
    {
    unsigned int i;
    execute_command = str;
//
// In case LILO is going to boot us with default command line,
// it prepends "auto" before the whole cmdline which makes
// the shell think it should execute a script with such name.
// So we ignore all arguments entered _before_ init=... [MJ]
//
    for (i = 1; i < MAX_INIT_ARGS; i++)
    argv_init[i] = core::ptr::null_mut();
    return 1;
    }
    __setup("init=", init_setup);
#[no_mangle]
unsafe extern "C" fn rdinit_setup(str: *mut c_char) -> int __init {
    static int __init rdinit_setup(char *str)
    {
    unsigned int i;
    ramdisk_execute_command = str;
    ramdisk_execute_command_set = true;
// See "auto" comment in init_setup
    for (i = 1; i < MAX_INIT_ARGS; i++)
    argv_init[i] = core::ptr::null_mut();
    return 1;
    }
    __setup("rdinit=", rdinit_setup);

    static inline void setup_nr_cpu_ids(void) { }
    static inline void smp_prepare_cpus(unsigned int maxcpus) { }

//
// We need to store the untouched command line for future reference.
// We also need to store the touched command line since the parameter
// parsing is performed in place, and we should allow a component to
// store reference of name/value for future reference.
//
#[no_mangle]
unsafe extern "C" fn setup_command_line(command_line: *mut c_char) -> void __init {
    static void __init setup_command_line(char *command_line)
    {
    size_t len, xlen = 0, ilen = 0;
    if (extra_command_line)
    xlen = strlen(extra_command_line);
    if (extra_init_args) {
    extra_init_args = strim(extra_init_args); /* remove trailing space */
    ilen = strlen(extra_init_args) + 4; /* for " -- " */
    }
    len = xlen + strlen(boot_command_line) + ilen + 1;
    saved_command_line = memblock_alloc_or_panic(len, SMP_CACHE_BYTES);
    len = xlen + strlen(command_line) + 1;
    static_command_line = memblock_alloc_or_panic(len, SMP_CACHE_BYTES);
    if (xlen) {
//
// We have to put extra_command_line before boot command
// lines because there could be dashes (separator of init
// command line) in the command lines.
//
    strcpy(saved_command_line, extra_command_line);
    strcpy(static_command_line, extra_command_line);
    }
    strcpy(saved_command_line + xlen, boot_command_line);
    strcpy(static_command_line + xlen, command_line);
    if (ilen) {
//
// Append supplemental init boot args to saved_command_line
// so that user can check what command line options passed
// to init.
// The order should always be
// " -- "[bootconfig init-param][cmdline init-param]
//
    if (initargs_offs) {
    len = xlen + initargs_offs;
    strcpy(saved_command_line + len, extra_init_args);
    len += ilen - 4;	/* strlen(extra_init_args) */
    strcpy(saved_command_line + len,
    boot_command_line + initargs_offs - 1);
    } else {
    len = strlen(saved_command_line);
    strcpy(saved_command_line + len, " -- ");
    len += 4;
    strcpy(saved_command_line + len, extra_init_args);
    }
    }
    saved_command_line_len = strlen(saved_command_line);
    }
//
// We need to finalize in a non-__init function or else race conditions
// between the root thread and the init thread may cause start_kernel to
// be reaped by free_initmem before the root thread has proceeded to
// cpu_idle.
//
// gcc-3.4 accidentally inlines this function, so use noinline.
//
    static __initdata DECLARE_COMPLETION(kthreadd_done);
#[no_mangle]
unsafe extern "C" fn rest_init() -> noinline void __ref __noreturn {
    static noinline void __ref __noreturn rest_init(void)
    {
    struct kernel_clone_args init_args = {
    .flags		= (CLONE_VM | CLONE_UNTRACED),
    .fn		= kernel_init,
    .fn_arg		= core::ptr::null_mut(),
    };
    struct task_struct *tsk;
    int pid;
    rcu_scheduler_starting();
//
// We need to spawn init first so that it obtains pid 1, however
// the init task will end up wanting to create kthreads, which, if
// we schedule it before we create kthreadd, will OOPS.
//
    pid = kernel_clone(&init_args);
//
// Pin init on the boot CPU. Task migration is not properly working
// until sched_init_smp() has been run. It will set the allowed
// CPUs for init to the non isolated CPUs.
//
    rcu_read_lock();
    tsk = find_task_by_pid_ns(pid, &init_pid_ns);
    tsk.flags |= PF_NO_SETAFFINITY;
    set_cpus_allowed_ptr(tsk, cpumask_of(smp_processor_id()));
    rcu_read_unlock();
    numa_default_policy();
    pid = kernel_thread(kthreadd, core::ptr::null_mut(), core::ptr::null_mut(), CLONE_FS | CLONE_FILES);
    rcu_read_lock();
    kthreadd_task = find_task_by_pid_ns(pid, &init_pid_ns);
    rcu_read_unlock();
//
// Enable might_sleep() and smp_processor_id() checks.
// They cannot be enabled earlier because with CONFIG_PREEMPTION=y
// kernel_thread() would trigger might_sleep() splats. With
// CONFIG_PREEMPT_VOLUNTARY=y the init task might have scheduled
// already, but it's stuck on the kthreadd_done completion.
//
    system_state = SYSTEM_SCHEDULING;
    complete(&kthreadd_done);
//
// The boot idle thread must execute schedule()
// at least once to get things moving:
//
    schedule_preempt_disabled();
// Call into cpu_idle with preempt disabled
    cpu_startup_entry(CPUHP_ONLINE);
    }
// Check for early params.
    static int __init do_early_param(char *param, char *val,
    const char *unused, void *arg)
    {
    const struct obs_kernel_param *p;
    for (p = __setup_start; p < __setup_end; p++) {
    if (p.early && parameq(param, p.str)) {
    if (p.setup_func(val) != 0)
    pr_warn("Malformed early option '%s'\n", param);
    }
    }
// We accept everything at this stage.
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_early_options(cmdline: *mut c_char) -> void __init {
    void __init parse_early_options(char *cmdline)
    {
    parse_args("early options", cmdline, core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(),
    do_early_param);
    }
// Arch code calls this early on, or if not, just before other parsing.
#[no_mangle]
pub unsafe extern "C" fn parse_early_param() -> void __init {
    void __init parse_early_param(void)
    {
    static int done __initdata;
    static char tmp_cmdline[COMMAND_LINE_SIZE] __initdata;
    if (done)
    return;
// All fall through to do_early_param.
    strscpy(tmp_cmdline, boot_command_line, COMMAND_LINE_SIZE);
    parse_early_options(tmp_cmdline);
    done = 1;
    }
    void __init __weak arch_post_acpi_subsys_init(void) { }
#[no_mangle]
pub unsafe extern "C" fn smp_setup_processor_id() -> void __init __weak {
    void __init __weak smp_setup_processor_id(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn smp_prepare_boot_cpu() -> void __init __weak {
    void __init __weak smp_prepare_boot_cpu(void)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn thread_stack_cache_init() -> void __init __weak {
    void __init __weak thread_stack_cache_init(void)
    {
    }

    void __init __weak poking_init(void) { }
    void __init __weak pgtable_cache_init(void) { }
    void __init __weak trap_init(void) { }
    bool initcall_debug;
    core_param(initcall_debug, initcall_debug, bool, 0644);

    static void __init initcall_debug_enable(void);

#[no_mangle]
pub unsafe extern "C" fn initcall_debug_enable() {
    static inline void initcall_debug_enable(void)
    {
    }

    DEFINE_STATIC_KEY_MAYBE_RO(CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT,
    randomize_kstack_offset);
    DEFINE_PER_CPU(struct rnd_state, kstack_rnd_state);
#[no_mangle]
unsafe extern "C" fn random_kstack_init() -> int __init {
    static int __init random_kstack_init(void)
    {
    prandom_seed_full_state(&kstack_rnd_state);
    return 0;
    }
    late_initcall(random_kstack_init);
#[no_mangle]
unsafe extern "C" fn early_randomize_kstack_offset(buf: *mut c_char) -> int __init {
    static int __init early_randomize_kstack_offset(char *buf)
    {
    int ret;
    bool bool_result;
    ret = kstrtobool(buf, &bool_result);
    if (ret)
    return ret;
    if (bool_result)
    static_branch_enable(&randomize_kstack_offset);
    else
    static_branch_disable(&randomize_kstack_offset);
    return 0;
    }
    early_param("randomize_kstack_offset", early_randomize_kstack_offset);

#[no_mangle]
unsafe extern "C" fn print_unknown_bootoptions() -> void __init {
    static void __init print_unknown_bootoptions(void)
    {
    char *unknown_options;
    char *end;
    const char *const *p;
    size_t len;
    if (panic_later || (!argv_init[1] && !envp_init[2]))
    return;
//
// Determine how many options we have to print out, plus a space
// before each
//
    len = 1; /* null terminator */
    for (p = &argv_init[1]; *p; p++) {
    len++;
    len += strlen(*p);
    }
    for (p = &envp_init[2]; *p; p++) {
    len++;
    len += strlen(*p);
    }
    unknown_options = memblock_alloc(len, SMP_CACHE_BYTES);
    if (!unknown_options) {
    pr_err("%s: Failed to allocate %zu bytes\n",
    __func__, len);
    return;
    }
    end = unknown_options;
    for (p = &argv_init[1]; *p; p++)
    end += sprintf(end, " %s", *p);
    for (p = &envp_init[2]; *p; p++)
    end += sprintf(end, " %s", *p);
// Start at unknown_options[1] to skip the initial space
    pr_notice("Unknown kernel command line parameters \"%s\", will be passed to user space.\n",
    &unknown_options[1]);
    memblock_free(unknown_options, len);
    }
#[no_mangle]
unsafe extern "C" fn early_numa_node_init() -> void __init {
    static void __init early_numa_node_init(void)
    {

    int cpu;
// The early_cpu_to_node() should be ready here.
    for_each_possible_cpu(cpu)
    set_cpu_numa_node(cpu, early_cpu_to_node(cpu));

    }

    KERNEL_CMDLINE_CONTINUATION_LEN)

    MIN_CMDLINE_LOG_WRAP_IDEAL_LEN ? \
    CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN : \
    MIN_CMDLINE_LOG_WRAP_IDEAL_LEN)

//
// print_kernel_cmdline() - Print the kernel cmdline with wrapping.
// @cmdline: The cmdline to print.
//
// Print the kernel command line, trying to wrap based on the Kconfig knob
// CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN.
//
// Wrapping is based on spaces, ignoring quotes. All lines are prefixed
// with "Kernel command line: " and lines that are not the last line have
// a " \" suffix added to them. The prefix and suffix count towards the
// line length for wrapping purposes. The ideal length will be exceeded
// if no appropriate place to wrap is found.
//
// Example output if CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN is 40:
// Kernel command line: loglevel=7 \
// Kernel command line: init=/sbin/init \
// Kernel command line: root=PARTUUID=8c3efc1a-768b-6642-8d0c-89eb782f19f0/PARTNROFF=1 \
// Kernel command line: rootwait ro \
// Kernel command line: my_quoted_arg="The \
// Kernel command line: quick brown fox \
// Kernel command line: jumps over the \
// Kernel command line: lazy dog."
//
#[no_mangle]
unsafe extern "C" fn print_kernel_cmdline(cmdline: *const c_char) -> void __init {
    static void __init print_kernel_cmdline(const char *cmdline)
    {
    size_t len;
// Config option of 0 or anything longer than the max disables wrapping
    if (CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN == 0 ||
    IDEAL_CMDLINE_LEN >= COMMAND_LINE_SIZE - 1) {
    pr_notice("%s%s\n", KERNEL_CMDLINE_PREFIX, cmdline);
    return;
    }
    len = strlen(cmdline);
    while (len > IDEAL_CMDLINE_LEN) {
    const char *first_space;
    const char *prev_cutoff;
    const char *cutoff;
    int to_print;
    size_t used;
// Find the last ' ' that wouldn't make the line too long
    prev_cutoff = core::ptr::null_mut();
    cutoff = cmdline;
    while (true) {
    cutoff = strchr(cutoff + 1, ' ');
    if (!cutoff || cutoff - cmdline > IDEAL_CMDLINE_SPLIT_LEN)
    break;
    prev_cutoff = cutoff;
    }
    if (prev_cutoff)
    cutoff = prev_cutoff;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !cutoff) -> else {
    else if (!cutoff)
    break;
// Find the beginning and end of the string of spaces
    first_space = cutoff;
    while (first_space > cmdline && first_space[-1] == ' ')
    first_space--;
    to_print = first_space - cmdline;
    while (*cutoff == ' ')
    cutoff++;
    used = cutoff - cmdline;
// If the whole string is used, break and do the final printout
    if (len == used)
    break;
    if (to_print)
    pr_notice("%s%.*s%s\n", KERNEL_CMDLINE_PREFIX,
    to_print, cmdline, KERNEL_CMDLINE_CONTINUATION);
    len -= used;
    cmdline += used;
    }
    if (len)
    pr_notice("%s%s\n", KERNEL_CMDLINE_PREFIX, cmdline);
    }
    asmlinkage __visible __init __no_sanitize_address __noreturn __no_stack_protector
#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    void start_kernel(void)
    {
    char *command_line;
    char *after_dashes;
    set_task_stack_end_magic(&init_task);
    smp_setup_processor_id();
    debug_objects_early_init();
    init_vmlinux_build_id();
    cgroup_init_early();
    local_irq_disable();
    early_boot_irqs_disabled = true;
//
// Interrupts are still disabled. Do necessary setups, then
// enable them.
//
    boot_cpu_init();
    page_address_init();
    pr_notice("%s", linux_banner);
    setup_arch(&command_line);
    mm_core_init_early();
// Static keys and static calls are needed by LSMs
    jump_label_init();
    static_call_init();
    early_security_init();
    setup_boot_config();
    setup_command_line(command_line);
    setup_nr_cpu_ids();
    setup_per_cpu_areas();
    smp_prepare_boot_cpu();	/* arch-specific boot-cpu hooks */
    early_numa_node_init();
    boot_cpu_hotplug_init();
    print_kernel_cmdline(saved_command_line);
// parameters may set static keys
    parse_early_param();
    after_dashes = parse_args("Booting kernel",
    static_command_line, __start___param,
    __stop___param - __start___param,
    -1, -1, core::ptr::null_mut(), &unknown_bootoption);
    print_unknown_bootoptions();
    if (!IS_ERR_OR_NULL(after_dashes))
    parse_args("Setting init args", after_dashes, core::ptr::null_mut(), 0, -1, -1,
    core::ptr::null_mut(), set_init_arg);
    if (extra_init_args)
    parse_args("Setting extra init args", extra_init_args,
    core::ptr::null_mut(), 0, -1, -1, core::ptr::null_mut(), set_init_arg);
// Architectural and non-timekeeping rng init, before allocator init
    random_init_early(command_line);
//
// These use large bootmem allocations and must precede
// initalization of page allocator
//
    setup_log_buf(0);
    vfs_caches_init_early();
    sort_main_extable();
    trap_init();
    mm_core_init();
    maple_tree_init();
    poking_init();
    ftrace_init();
// trace_printk can be enabled here
    early_trace_init();
//
// Set up the scheduler prior starting any interrupts (such as the
// timer interrupt). Full topology setup happens at smp_init()
// time - but meanwhile we still have a functioning scheduler.
//
    sched_init();
    if (WARN(!irqs_disabled(),
    "Interrupts were enabled *very* early, fixing it\n"))
    local_irq_disable();
    radix_tree_init();
//
// Set up housekeeping before setting up workqueues to allow the unbound
// workqueue to take non-housekeeping into account.
//
    housekeeping_init();
//
// Allow workqueue creation and work item queueing/cancelling
// early.  Work item execution depends on kthreads and starts after
// workqueue_init().
//
    workqueue_init_early();
    rcu_init();
    kvfree_rcu_init();
// Trace events are available after this
    trace_init();
    if (initcall_debug)
    initcall_debug_enable();
    context_tracking_init();
// init some links before init_ISA_irqs()
    early_irq_init();
    init_IRQ();
    tick_init();
    rcu_init_nohz();
    timers_init();
    srcu_init();
    hrtimers_init();
    softirq_init();
    vdso_setup_data_pages();
    timekeeping_init();
    time_init();
// This must be after timekeeping is initialized
    random_init();
// These make use of the fully initialized rng
    kfence_init();
    boot_init_stack_canary();
    perf_event_init();
    profile_init();
    call_function_init();
    WARN(!irqs_disabled(), "Interrupts were enabled early\n");
    early_boot_irqs_disabled = false;
    local_irq_enable();
    kmem_cache_init_late();
//
// HACK ALERT! This is early. We're enabling the console before
// we've done PCI setups etc, and console_init() must be aware of
// this. But we do want output early, in case something goes wrong.
//
    console_init();
    if (panic_later)
    panic("Too many boot %s vars at `%s'", panic_later,
    panic_param);
    lockdep_init();
//
// Need to run this when irqs are enabled, because it wants
// to self-test [hard/soft]-irqs on/off lock inversion bugs
// too:
//
    locking_selftest();

    if (initrd_start && !initrd_below_start_ok &&
    page_to_pfn(virt_to_page((void *)initrd_start)) < min_low_pfn) {
    pr_crit("initrd overwritten (0x%08lx < 0x%08lx) - disabling it.\n",
    page_to_pfn(virt_to_page((void *)initrd_start)),
    min_low_pfn);
    initrd_start = 0;
    }

    setup_per_cpu_pageset();
    numa_policy_init();
    acpi_early_init();
    if (late_time_init)
    late_time_init();
    sched_clock_init();
    calibrate_delay();
    arch_cpu_finalize_init();
    pid_idr_init();
    anon_vma_init();
    thread_stack_cache_init();
    cred_init();
    fork_init();
    proc_caches_init();
    uts_ns_init();
    time_ns_init();
    key_init();
    security_init();
    dbg_late_init();
    net_ns_init();
    vfs_caches_init();
    pagecache_init();
    signals_init();
    seq_file_init();
    proc_root_init();
    nsfs_init();
    pidfs_init();
    cpuset_init();
    mem_cgroup_init();
    cgroup_init();
    taskstats_init_early();
    delayacct_init();
    acpi_subsystem_init();
    arch_post_acpi_subsys_init();
    kcsan_init();
// Do the rest non-__init'ed, we're now alive
    rest_init();
//
// Avoid stack canaries in callers of boot_init_stack_canary for gcc-10
// and older.
//

    prevent_tail_call_optimization();

    }
// Call all constructor functions linked into the kernel.
#[no_mangle]
unsafe extern "C" fn do_ctors() -> void __init {
    static void __init do_ctors(void)
    {
//
// For UML, the constructors have already been called by the
// normal setup code as it's just a normal ELF binary, so we
// cannot do it again - but we do need CONFIG_CONSTRUCTORS
// even on UML for modules.
//

    ctor_fn_t *fn = (ctor_fn_t *) __ctors_start;
    for (; fn < (ctor_fn_t *) __ctors_end; fn++)
    (*fn)();

    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blacklist_entry {
    pub next: list_head,
    pub buf: *mut c_char,
}

    static __initdata_or_module LIST_HEAD(blacklisted_initcalls);
#[no_mangle]
unsafe extern "C" fn initcall_blacklist(str: *mut c_char) -> int __init {
    static int __init initcall_blacklist(char *str)
    {
    char *str_entry;
    struct blacklist_entry *entry;
// str argument is a comma-separated list of functions
    do {
    str_entry = strsep(&str, ",");
    if (str_entry) {
    pr_debug("blacklisting initcall %s\n", str_entry);
    entry = memblock_alloc_or_panic(sizeof(*entry),
    SMP_CACHE_BYTES);
    entry.buf = memblock_alloc_or_panic(strlen(str_entry) + 1,
    SMP_CACHE_BYTES);
    strcpy(entry.buf, str_entry);
    list_add(&entry.next, &blacklisted_initcalls);
    }
    } while (str_entry);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn initcall_blacklisted(fn: initcall_t) -> bool __init_or_module {
    static bool __init_or_module initcall_blacklisted(initcall_t fn)
    {
    struct blacklist_entry *entry;
    char fn_name[KSYM_SYMBOL_LEN];
    unsigned long addr;
    if (list_empty(&blacklisted_initcalls))
    return false;
    addr = (unsigned long) dereference_function_descriptor(fn);
    sprint_symbol_no_offset(fn_name, addr);
//
// fn will be "function_name [module_name]" where [module_name] is not
// displayed for built-in init functions.  Strip off the [module_name].
//
    strreplace(fn_name, ' ', '\0');
    list_for_each_entry(entry, &blacklisted_initcalls, next) {
    if (!strcmp(fn_name, entry.buf)) {
    pr_debug("initcall %s blacklisted\n", fn_name);
    return true;
    }
    }
    return false;
    }

#[no_mangle]
unsafe extern "C" fn initcall_blacklist(str: *mut c_char) -> int __init {
    static int __init initcall_blacklist(char *str)
    {
    pr_warn("initcall_blacklist requires CONFIG_KALLSYMS\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn initcall_blacklisted(fn: initcall_t) -> bool __init_or_module {
    static bool __init_or_module initcall_blacklisted(initcall_t fn)
    {
    return false;
    }

    __setup("initcall_blacklist=", initcall_blacklist);
    static __init_or_module void
    trace_initcall_start_cb(void *data, initcall_t fn)
    {
    ktime_t *calltime = data;
    printk(KERN_DEBUG "calling  %pS @ %i\n", fn, task_pid_nr(current));
// calltime = ktime_get();
    }
    static __init_or_module void
    trace_initcall_finish_cb(void *data, initcall_t fn, int ret)
    {
    ktime_t rettime, *calltime = data;
    rettime = ktime_get();
    printk(KERN_DEBUG "initcall %pS returned %d after %lld usecs\n",
    fn, ret, (unsigned long long)ktime_us_delta(rettime, *calltime));
    }
    static __init_or_module void
    trace_initcall_level_cb(void *data, const char *level)
    {
    printk(KERN_DEBUG "entering initcall level: %s\n", level);
    }
    static ktime_t initcall_calltime;

#[no_mangle]
unsafe extern "C" fn initcall_debug_enable() -> void __init {
    static void __init initcall_debug_enable(void)
    {
    int ret;
    ret = register_trace_initcall_start(trace_initcall_start_cb,
    &initcall_calltime);
    ret |= register_trace_initcall_finish(trace_initcall_finish_cb,
    &initcall_calltime);
    ret |= register_trace_initcall_level(trace_initcall_level_cb, core::ptr::null_mut());
    WARN(ret, "Failed to register initcall tracepoints\n");
    }

#[no_mangle]
pub unsafe extern "C" fn do_trace_initcall_start(fn: initcall_t) {
    static inline void do_trace_initcall_start(initcall_t fn)
    {
    if (!initcall_debug)
    return;
    trace_initcall_start_cb(&initcall_calltime, fn);
    }
#[no_mangle]
pub unsafe extern "C" fn do_trace_initcall_finish(fn: initcall_t, ret: c_int) {
    static inline void do_trace_initcall_finish(initcall_t fn, int ret)
    {
    if (!initcall_debug)
    return;
    trace_initcall_finish_cb(&initcall_calltime, fn, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn do_trace_initcall_level(level: *const c_char) {
    static inline void do_trace_initcall_level(const char *level)
    {
    if (!initcall_debug)
    return;
    trace_initcall_level_cb(core::ptr::null_mut(), level);
    }

#[no_mangle]
pub unsafe extern "C" fn do_one_initcall(fn: initcall_t) -> int __init_or_module {
    int __init_or_module do_one_initcall(initcall_t fn)
    {
    let mut count: c_int = preempt_count();
    char msgbuf[64];
    int ret;
    if (initcall_blacklisted(fn))
    return -EPERM;
    do_trace_initcall_start(fn);
    ret = fn();
    do_trace_initcall_finish(fn, ret);
    msgbuf[0] = 0;
    if (preempt_count() != count) {
    sprintf(msgbuf, "preemption imbalance ");
    preempt_count_set(count);
    }
    if (irqs_disabled()) {
    strlcat(msgbuf, "disabled interrupts ", sizeof(msgbuf));
    local_irq_enable();
    }
    WARN(msgbuf[0], "initcall %pS returned with %s\n", fn, msgbuf);
    add_latent_entropy();
    return ret;
    }
    static initcall_entry_t *initcall_levels[] __initdata = {
    __initcall0_start,
    __initcall1_start,
    __initcall2_start,
    __initcall3_start,
    __initcall4_start,
    __initcall5_start,
    __initcall6_start,
    __initcall7_start,
    __initcall_end,
    };
// Keep these in sync with initcalls in include/linux/init.h
    static const char *initcall_level_names[] __initdata = {
    "pure",
    "core",
    "postcore",
    "arch",
    "subsys",
    "fs",
    "device",
    "late",
    };
    static int __init ignore_unknown_bootoption(char *param, char *val,
    const char *unused, void *arg)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_initcall_level(level: c_int, command_line: *mut c_char) -> void __init {
    static void __init do_initcall_level(int level, char *command_line)
    {
    initcall_entry_t *fn;
    parse_args(initcall_level_names[level],
    command_line, __start___param,
    __stop___param - __start___param,
    level, level,
    core::ptr::null_mut(), ignore_unknown_bootoption);
    do_trace_initcall_level(initcall_level_names[level]);
    for (fn = initcall_levels[level]; fn < initcall_levels[level+1]; fn++)
    do_one_initcall(initcall_from_entry(fn));
    }
#[no_mangle]
unsafe extern "C" fn do_initcalls() -> void __init {
    static void __init do_initcalls(void)
    {
    int level;
    let mut len: usize = saved_command_line_len + 1;
    char *command_line;
    command_line = kzalloc(len, GFP_KERNEL);
    if (!command_line)
    panic("%s: Failed to allocate %zu bytes\n", __func__, len);
    for (level = 0; level < ARRAY_SIZE(initcall_levels) - 1; level++) {
// Parser modifies command_line, restore it each time
    strcpy(command_line, saved_command_line);
    do_initcall_level(level, command_line);
    }
    kfree(command_line);
    }
//
// Ok, the machine is now initialized. None of the devices
// have been touched yet, but the CPU subsystem is up and
// running, and memory and process management works.
//
// Now we can finally start doing some real work..
//
#[no_mangle]
unsafe extern "C" fn do_basic_setup() -> void __init {
    static void __init do_basic_setup(void)
    {
    cpuset_init_smp();
    ksysfs_init();
    driver_init();
    init_irq_proc();
    do_ctors();
    do_initcalls();
    }
#[no_mangle]
unsafe extern "C" fn do_pre_smp_initcalls() -> void __init {
    static void __init do_pre_smp_initcalls(void)
    {
    initcall_entry_t *fn;
    do_trace_initcall_level("early");
    for (fn = __initcall_start; fn < __initcall0_start; fn++)
    do_one_initcall(initcall_from_entry(fn));
    }
#[no_mangle]
unsafe extern "C" fn run_init_process(init_filename: *const c_char) -> c_int {
    static int run_init_process(const char *init_filename)
    {
    const char *const *p;
    argv_init[0] = init_filename;
    pr_info("Run %s as init process\n", init_filename);
    pr_debug("  with arguments:\n");
    for (p = argv_init; *p; p++)
    pr_debug("    %s\n", *p);
    pr_debug("  with environment:\n");
    for (p = envp_init; *p; p++)
    pr_debug("    %s\n", *p);
    return kernel_execve(init_filename, argv_init, envp_init);
    }
#[no_mangle]
unsafe extern "C" fn try_to_run_init_process(init_filename: *const c_char) -> c_int {
    static int try_to_run_init_process(const char *init_filename)
    {
    int ret;
    ret = run_init_process(init_filename);
    if (ret && ret != -ENOENT) {
    pr_err("Starting init: %s exists but couldn't execute it (error %d)\n",
    init_filename, ret);
    }
    return ret;
    }
    static noinline void __init kernel_init_freeable(void);

    let mut __ro_after_init: bool rodata_enabled = true;

    static inline bool arch_parse_debug_rodata(char *str) { return false; }

#[no_mangle]
unsafe extern "C" fn set_debug_rodata(str: *mut c_char) -> int __init {
    static int __init set_debug_rodata(char *str)
    {
    if (arch_parse_debug_rodata(str))
    return 0;
    if (str && !strcmp(str, "on"))
    rodata_enabled = true;
#[no_mangle]
pub unsafe extern "C" fn if(!strcmp(str: str &&, _arg: "off")) -> else {
    else if (str && !strcmp(str, "off"))
    rodata_enabled = false;
    else
    pr_warn("Invalid option string for rodata: '%s'\n", str);
    return 0;
    }
    early_param("rodata", set_debug_rodata);

#[no_mangle]
unsafe extern "C" fn mark_readonly() {
    static void mark_readonly(void)
    {
    if (IS_ENABLED(CONFIG_STRICT_KERNEL_RWX) && rodata_enabled) {
//
// load_module() results in W+X mappings, which are cleaned
// up with init_free_wq. Let's make sure that queued work is
// flushed so that we don't hit false positives looking for
// insecure pages which are W+X.
//
    flush_module_init_free_work();
    jump_label_init_ro();
    mark_rodata_ro();
    debug_checkwx();
    rodata_test();
    } else if (IS_ENABLED(CONFIG_STRICT_KERNEL_RWX)) {
    pr_info("Kernel memory protection disabled.\n");
    } else if (IS_ENABLED(CONFIG_ARCH_HAS_STRICT_KERNEL_RWX)) {
    pr_warn("Kernel memory protection not selected by kernel config.\n");
    } else {
    pr_warn("This architecture does not have kernel memory protection.\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn free_initmem() -> void __weak {
    void __weak free_initmem(void)
    {
    free_initmem_default(POISON_FREE_INITMEM);
    }
#[no_mangle]
unsafe extern "C" fn kernel_init(unused: *mut c_void) -> int __ref {
    static int __ref kernel_init(void *unused)
    {
    int ret;
    init_userspace_fs();
//
// Wait until kthreadd is all set-up.
//
    wait_for_completion(&kthreadd_done);
    kernel_init_freeable();
// need to finish all async __init code before freeing the memory
    async_synchronize_full();
    system_state = SYSTEM_FREEING_INITMEM;
    kprobe_free_init_mem();
    ftrace_free_init_mem();
    kgdb_free_init_mem();
    exit_boot_config();
    free_initmem();
    mark_readonly();
//
// Kernel mappings are now finalized - update the userspace page-table
// to finalize PTI.
//
    pti_finalize();
    system_state = SYSTEM_RUNNING;
    numa_default_policy();
    rcu_end_inkernel_boot();
    do_sysctl_args();
    if (ramdisk_execute_command) {
    ret = run_init_process(ramdisk_execute_command);
    if (!ret)
    return 0;
    pr_err("Failed to execute %s (error %d)\n",
    ramdisk_execute_command, ret);
    }
//
// We try each of these until one succeeds.
//
// The Bourne shell can be used instead of init if we are
// trying to recover a really broken machine.
//
    if (execute_command) {
    ret = run_init_process(execute_command);
    if (!ret)
    return 0;
    panic("Requested init %s failed (error %d).",
    execute_command, ret);
    }
    if (CONFIG_DEFAULT_INIT[0] != '\0') {
    ret = run_init_process(CONFIG_DEFAULT_INIT);
    if (ret)
    pr_err("Default init %s failed (error %d)\n",
    CONFIG_DEFAULT_INIT, ret);
    else
    return 0;
    }
    if (!try_to_run_init_process("/sbin/init") ||
    !try_to_run_init_process("/etc/init") ||
    !try_to_run_init_process("/bin/init") ||
    !try_to_run_init_process("/bin/sh"))
    return 0;
    panic("No working init found.  Try passing init= option to kernel. "
    "See Linux Documentation/admin-guide/init.rst for guidance.");
    }
// Open /dev/console, for stdin/stdout/stderr, this should never fail
#[no_mangle]
pub unsafe extern "C" fn console_on_rootfs() -> void __init {
    void __init console_on_rootfs(void)
    {
    struct file *file = filp_open("/dev/console", O_RDWR, 0);
    if (IS_ERR(file)) {
    pr_err("Warning: unable to open an initial console.\n");
    return;
    }
    init_dup(file);
    init_dup(file);
    init_dup(file);
    fput(file);
    }
#[no_mangle]
unsafe extern "C" fn kernel_init_freeable() -> noinline void __init {
    static noinline void __init kernel_init_freeable(void)
    {
// Now the scheduler is fully set up and can do blocking allocations
    gfp_allowed_mask = __GFP_BITS_MASK;
//
// init can allocate pages on any node
//
    set_mems_allowed(node_states[N_MEMORY]);
    cad_pid = get_pid(task_pid(current));
    smp_prepare_cpus(setup_max_cpus);
    workqueue_init();
    init_mm_internals();
    do_pre_smp_initcalls();
    lockup_detector_init();
    smp_init();
    sched_init_smp();
    workqueue_init_topology();
    async_init();
    padata_init();
    page_alloc_init_late();
    do_basic_setup();
    kunit_run_all_tests();
    wait_for_initramfs();
    console_on_rootfs();
//
// check if there is an early userspace init.  If yes, let it do all
// the work
//
    int ramdisk_command_access;
    ramdisk_command_access = init_eaccess(ramdisk_execute_command);
    if (ramdisk_command_access != 0) {
    if (ramdisk_execute_command_set)
    pr_warn("check access for rdinit=%s failed: %i, ignoring\n",
    ramdisk_execute_command, ramdisk_command_access);
    ramdisk_execute_command = core::ptr::null_mut();
    prepare_namespace();
    }
//
// Ok, we have completed the initial bootup, and
// we're essentially up and running. Get rid of the
// initmem segments and start the user-mode stuff..
//
// rootfs is available now, try loading the public keys
// and default modules
//
    integrity_load_keys();
    }
