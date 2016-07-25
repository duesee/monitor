#Usage

```
cargo build --release
./target/release/monitor monitor.txt
```

#Output

```
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
