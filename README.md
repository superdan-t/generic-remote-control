# Generic Remote Control
This is a new project to create a middle layer to use in remote vehicle operation stacks and allow vehicle platforms to interop with frontend controller applications through a generic interface.

The idea is like a greatly simplified Robot Operating System (ROS) and is also comparable to the LLVM compiler toolkit: Frontends, which are applications for users to operate vehicles, will use this interface to control generic vehicles, and backends, which are control systems for specific vehicle platforms, will enable generic control by implementing this interface.

In the past, I've built multiple vehicle types ranging from "hacking" toy RC cars to unmanned underwater vehicles (UUVs). As I continue to experiment with new platforms, I hope to make a control stack with lots of reusable pieces, especially a fun and functional graphic controller application that can control all these platforms, and this library is the most important part.
