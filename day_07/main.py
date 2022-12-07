class Dir:
    def __init__(self, name, child_dir=None, child_file=None, parent=None) -> None:
        self.name = name
        self.child_dir = [] if child_dir is None else child_dir
        self.child_file = [] if child_file is None else child_file
        self.parent = self if parent is None else parent

    def add_child(self, type, child):
        if type == "file":
            self.child_file.append(child)
        else:
            self.child_dir.append(child)

    def get_size(self):
        child_dir_sizes = sum(x.get_size() for x in self.child_dir)
        child_file_sizes = sum(x.size for x in self.child_file)

        return child_dir_sizes + child_file_sizes

    def get_child_size(self, out, x=100000):
        s = 0
        for f in self.child_dir:
            s += f.get_child_size(out, x=x)
        
        s += sum(x.size for x in self.child_file)

        if s < x:
         print(f"{self.name}: {s}")
         out.append(s)

        return s
    
    def get_child_size_f(self, out, x=100000):
        s = 0
        for f in self.child_dir:
            s += f.get_child_size_f(out, x=x)
        
        s += sum(x.size for x in self.child_file)

        if s > x:
         print(f"{self.name}: {s}")
         out.append(s)

        return s

    def cd(self, to:str):
        if to == "..":
            return self.parent
        
        for x in self.child_dir:
            if x.name == to:
                return x
        
        print(f"cant find {to}")
        return None


class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size

def main(file):
    root_dir = None
    active_dir = None

    with open(file, "r") as f:
        for line in f:
            line = line.replace("\n", "")
            if line.count("/") > 0:
                root_dir = Dir("/")
                active_dir = root_dir
            elif line.startswith("$ ls"):
                continue
            elif line.startswith("$ cd"):
                xzxzxzx = line.replace("$ cd ", "")
                print(active_dir.name)
                active_dir = active_dir.cd(xzxzxzx)

            elif line.startswith("dir"):
                active_dir.add_child(
                    "dir",
                    Dir(line.replace("dir ", ""), parent=active_dir)
                )
            else:
                stuff = line.split(" ")
                active_dir.add_child(
                    "file",
                    File(
                        stuff[1],
                        int(stuff[0])
                    )
                )

    out = []
    root_dir.get_child_size(out)
    print(sum(out))

    available_space = 70000000
    root_used = root_dir.get_size()
    print(f"root_used: {root_used}")
    rec_unused = 30000000
    rec_to_free = (root_used+rec_unused) - available_space
    print(root_used + rec_unused > available_space)
    print(f"to_free: {rec_to_free}")

    xx = []
    root_dir.get_child_size_f(xx, x = rec_to_free)
    print(min(xx))

if __name__=="__main__":
    main("input")