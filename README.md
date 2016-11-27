# Build

```
$ cargo build --release
```

# Usage

```
$ watch -n1 "./target/release/monitor assets/monitor_1.txt"

/etc/shadow                                                       = <Permission denied (os error 13)>
/proc/sys/vm/hugepages_treat_as_movable                           = 0
/proc/sys/vm/hugetlb_shm_group                                    = 0
/proc/sys/vm/nr_hugepages                                         = 267
/proc/sys/vm/nr_hugepages_mempolicy                               = 267
/proc/sys/vm/nr_overcommit_hugepages                              = 0
/sys/kernel/mm/hugepages/hugepages-2048kB                         = <Is a directory (os error 21)>
/sys/kernel/mm/hugepages/hugepages-2048kB/free_hugepages          = 267
/sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages            = 267
/sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages_mempolicy  = 267
/sys/kernel/mm/hugepages/hugepages-2048kB/nr_overcommit_hugepages = 0
/sys/kernel/mm/hugepages/hugepages-2048kB/resv_hugepages          = 0
/sys/kernel/mm/hugepages/hugepages-2048kB/surplus_hugepages       = 0
/sys/kernel/mm/ksm/full_scans                                     = 0
/sys/kernel/mm/ksm/merge_across_nodes                             = 1
/sys/kernel/mm/ksm/pages_shared                                   = 0
/sys/kernel/mm/ksm/pages_sharing                                  = 0
/sys/kernel/mm/ksm/pages_to_scan                                  = 1000
/sys/kernel/mm/ksm/pages_unshared                                 = 0
/sys/kernel/mm/ksm/pages_volatile                                 = 0
/sys/kernel/mm/ksm/run                                            = 1
/sys/kernel/mm/ksm/sleep_millisecs                                = 2
```

```
$ find /proc -name "*_va_*" 2>/dev/null | ./target/release/monitor

/proc/sys/kernel/randomize_va_space = 2
/proc/sys/vm/legacy_va_layout       = 0
```

```
$ watch -n0.1 "./target/debug/monitor assets/monitor_2.txt -o graph.csv"

[...]

$ cat graph.csv 
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
0,0,0,0,0,0,0,1,0,0,100,0,0,0,20
```
