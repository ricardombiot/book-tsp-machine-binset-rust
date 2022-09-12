import os

class MemoryReader():
    def __init__(self, listen_pid):
        self.txt = ""
        self.mem = ""
        self.max_mem = ""
        self.valid = True
        self.listen_pid = listen_pid

    def run(self):
        # For MacOs
        cmd = f"top -pid {self.listen_pid} -l 1 -stats pid,mem | tail -n 1"
        #For LInux
        #cmd = f"python3 ./simula_top.py {self.listen_pid}"
        stream = os.popen(cmd)
        output = stream.read()

        result = output.replace("\n","").split("  ")

        if len(result) == 2:
            self.mem = result[1]
            self.save_max_mem()
            result.append(self.max_mem)
            print(result)
            self.valid = True
        else:
            self.valid = False

    def get_max_mem(self):
        return self.max_mem

    def save_max_mem(self):
        is_max_in_Kb = "K" in self.max_mem
        is_mem_in_Kb = "K" in self.mem

        is_max_in_Mb = "M" in self.max_mem
        is_mem_in_Mb = "M" in self.mem

        if self.max_mem == "" or (is_max_in_Kb and is_mem_in_Mb):
            self.max_mem = self.mem
        elif is_max_in_Mb and is_mem_in_Mb:
            max_mem_num = float(self.max_mem.replace("M",""))
            mem_num = float(self.mem.replace("M",""))

            if mem_num > max_mem_num:
                self.max_mem = self.mem

    def is_valid(self):
        return self.valid
