//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/cpupower.c
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
// (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
//
// Ideas taken over from the perf userspace tool (included in the Linus
// kernel git repo): subcommand builtins and param parsing.
//

    static int cmd_help(int argc, const char **argv);
// Global cpu_info object available for all binaries
// Info only retrieved from CPU 0
//
// Values will be zero/unknown on non X86 archs
//
    struct cpupower_cpu_info cpupower_cpu_info;
    int run_as_root;
    int base_cpu;
// Affected cpus chosen by -c/--cpu param
    struct bitmask *cpus_chosen;
    struct bitmask *online_cpus;
    struct bitmask *offline_cpus;

    int be_verbose;

    static void print_help(void);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_struct {
    pub cmd: *const c_char,
    pub ): *const *const int (main)(int, char,
    pub needs_root: c_int,
}

    static struct cmd_struct commands[] = {
    { "frequency-info",	cmd_freq_info,	0	},
    { "frequency-set",	cmd_freq_set,	1	},
    { "idle-info",		cmd_idle_info,	0	},
    { "idle-set",		cmd_idle_set,	1	},
    { "powercap-info",	cmd_cap_info,	0	},
    { "set",		cmd_set,	1	},
    { "info",		cmd_info,	0	},
    { "monitor",		cmd_monitor,	0	},
    { "help",		cmd_help,	0	},
// { "bench",	cmd_bench,	1	},
    };
#[no_mangle]
unsafe extern "C" fn print_help() {
    static void print_help(void)
    {
    unsigned int i;

    printf(_("Usage:\tcpupower [-d|--debug] [-c|--cpu cpulist ] <command> [<args>]\n"));

    printf(_("Usage:\tcpupower [-c|--cpu cpulist ] <command> [<args>]\n"));

    printf(_("Supported commands are:\n"));
    for (i = 0; i < ARRAY_SIZE(commands); i++)
    printf("\t%s\n", commands[i].cmd);
    printf(_("\nNot all commands can make use of the -c cpulist option.\n"));
    printf(_("\nUse 'cpupower help <command>' for getting help for above commands.\n"));
    }
#[no_mangle]
unsafe extern "C" fn print_man_page(subpage: *const c_char) -> c_int {
    static int print_man_page(const char *subpage)
    {
    int len;
    char *page;
    len = 10; /* enough for "cpupower-" */
    if (subpage != core::ptr::null_mut())
    len += strlen(subpage);
    page = malloc(len);
    if (!page)
    return -ENOMEM;
    sprintf(page, "cpupower");
    if ((subpage != core::ptr::null_mut()) && strcmp(subpage, "help")) {
    strcat(page, "-");
    strcat(page, subpage);
    }
    execlp("man", "man", page, core::ptr::null_mut());
// should not be reached
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn cmd_help(argc: c_int, argv: *const c_char) -> c_int {
    static int cmd_help(int argc, const char **argv)
    {
    if (argc > 1) {
    print_man_page(argv[1]); /* exits within execlp() */
    return EXIT_FAILURE;
    }
    print_help();
    return EXIT_SUCCESS;
    }
#[no_mangle]
unsafe extern "C" fn print_version() {
    static void print_version(void)
    {
    printf(PACKAGE " " VERSION "\n");
    printf(_("Report errors and bugs to %s, please.\n"), PACKAGE_BUGREPORT);
    }
#[no_mangle]
unsafe extern "C" fn handle_options(argc: *mut c_int, argv: *const c_char) {
    static void handle_options(int *argc, const char ***argv)
    {
    int ret, x, new_argc = 0;
    if (*argc < 1)
    return;
    for (x = 0;  x < *argc && ((*argv)[x])[0] == '-'; x++) {
    const char *param = (*argv)[x];
    if (!strcmp(param, "-h") || !strcmp(param, "--help")) {
    print_help();
    exit(EXIT_SUCCESS);
    } else if (!strcmp(param, "-c") || !strcmp(param, "--cpu")) {
    if (*argc < 2) {
    print_help();
    exit(EXIT_FAILURE);
    }
    if (!strcmp((*argv)[x+1], "all"))
    bitmask_setall(cpus_chosen);
    else {
    ret = bitmask_parselist(
    (*argv)[x+1], cpus_chosen);
    if (ret < 0) {
    fprintf(stderr, _("Error parsing cpu "
    "list\n"));
    exit(EXIT_FAILURE);
    }
    }
    x += 1;
// Cut out param: cpupower -c 1 info -> cpupower info
    new_argc += 2;
    continue;
    } else if (!strcmp(param, "-v") ||
    !strcmp(param, "--version")) {
    print_version();
    exit(EXIT_SUCCESS);

    } else if (!strcmp(param, "-d") || !strcmp(param, "--debug")) {
    be_verbose = 1;
    new_argc++;
    continue;

    } else {
    fprintf(stderr, "Unknown option: %s\n", param);
    print_help();
    exit(EXIT_FAILURE);
    }
    }
// argc -= new_argc;
// argv += new_argc;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const c_char) -> c_int {
    int main(int argc, const char *argv[])
    {
    const char *cmd;
    unsigned int i, ret;
    struct stat statbuf;
    struct utsname uts;
    char pathname[32];
    cpus_chosen = bitmask_alloc(sysconf(_SC_NPROCESSORS_CONF));
    online_cpus = bitmask_alloc(sysconf(_SC_NPROCESSORS_CONF));
    offline_cpus = bitmask_alloc(sysconf(_SC_NPROCESSORS_CONF));
    argc--;
    argv += 1;
    handle_options(&argc, &argv);
    cmd = argv[0];
    if (argc < 1) {
    print_help();
    return EXIT_FAILURE;
    }
    setlocale(LC_ALL, "");
    textdomain(PACKAGE);
// Turn "perf cmd --help" into "perf help cmd"
    if (argc > 1 && !strcmp(argv[1], "--help")) {
    argv[1] = argv[0];
    argv[0] = cmd = "help";
    }
    base_cpu = sched_getcpu();
    if (base_cpu < 0) {
    fprintf(stderr, _("No valid cpus found.\n"));
    return EXIT_FAILURE;
    }
    get_cpu_info(&cpupower_cpu_info);
    run_as_root = !geteuid();
    if (run_as_root) {
    ret = uname(&uts);
    sprintf(pathname, "/dev/cpu/%d/msr", base_cpu);
    if (!ret && !strcmp(uts.machine, "x86_64") &&
    stat(pathname, &statbuf) != 0) {
    if (system("modprobe msr") == -1)
    fprintf(stderr, _("MSR access not available.\n"));
    }
    }
    for (i = 0; i < ARRAY_SIZE(commands); i++) {
    struct cmd_struct *p = commands + i;
    if (strcmp(p.cmd, cmd))
    continue;
    if (!run_as_root && p.needs_root) {
    fprintf(stderr, _("Subcommand %s needs root "
    "privileges\n"), cmd);
    return EXIT_FAILURE;
    }
    ret = p.main(argc, argv);
    if (cpus_chosen)
    bitmask_free(cpus_chosen);
    if (online_cpus)
    bitmask_free(online_cpus);
    if (offline_cpus)
    bitmask_free(offline_cpus);
    return ret;
    }
    print_help();
    return EXIT_FAILURE;
    }
