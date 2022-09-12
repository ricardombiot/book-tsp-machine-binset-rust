#!/usr/bin/python

from ast import Try
import sys 

import os


def read_total_mem():
    stream = os.popen("cat /proc/meminfo | grep -i 'memtotal' | grep -o '[[:digit:]]*'")
    output = stream.read()
    return float(output)/1024

def read_proc_mem_process(pid):
    cmd=f"ps -p {pid} -o %mem | tail -n 1"
    stream = os.popen(cmd)
    output = stream.read()
    if output == "%MEM\n" :
        return (False,0)
    else:
        return (True,float(output)/100.0)

def main():
    args = sys.argv[1:]

    pid = args[0]
    total_mem = read_total_mem()
    is_valid, proc_mem = read_proc_mem_process(pid)
    
    if is_valid :
        abs_mem = int(total_mem*proc_mem)
        print(f"{pid}  {abs_mem}M")
    else:
        print(f"")

main()