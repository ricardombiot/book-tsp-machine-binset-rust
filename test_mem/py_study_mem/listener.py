import threading
import time
import memory_reader
import datetime

class ServiceLintener (threading.Thread):
    def __init__(self, pid):
        threading.Thread.__init__(self)
        self.listen_pid = pid
        self.name = f"Service({pid})"
        self.reader = memory_reader.MemoryReader(pid)
        self.stop = False
        self.start_time = datetime.datetime.now()

    def run(self):
        delay = 0.01
        print("Starting " + self.name)
        while not self.stop:
            print_time(self.name)
            self.read_stats()
            time.sleep(delay)
        print("Exiting " + self.name)

    def exit(self):
        self.stop = True

    def read_stats(self):
        self.reader.run()

        if not self.reader.is_valid():
            print(f"Cannot read stats pid: {self.listen_pid} then will stop listing process...")
            self.exit()

    def get_report(self):
        map = {}
        map["max_mem"] = self.get_max_mem()
        map["time"] = {}
        map["time"]["start_listing"] = self.get_start_time()
        map["time"]["report"] = datetime.datetime.now()

        return map

    def get_max_mem(self):
        return self.reader.get_max_mem()

    def get_start_time(self):
        return self.start_time

def print_time(threadName):
    print("%s : %s" % (threadName, time.ctime(time.time())))

def start(pid):
    thread1 = ServiceLintener(pid)
    thread1.start()
    return thread1

def stop(thread1):
    thread1.exit()
