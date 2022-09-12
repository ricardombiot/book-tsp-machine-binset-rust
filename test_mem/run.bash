#!bin/bash
./../target/release/tsp_machine_binset bench:quick $1 1 ./reports_times/times_n$1.part_a.csv &
_pid=$! 
sh ./py_study_mem/run_listen.sh $_pid
wait "$_pid"
sh ./py_study_mem/stop_listen.sh $_pid mem_report_$1
