# 功能简介

在`TaskControlBlock`中添加了`syscall_times`用来记录系统调用的次数，添加了`begin_time`用来记录开始的时间

在`TaskManager`中实现了`increase_syscall_times`、`get_syscall_times`、`get_begin_time`来分别增加系统调用，获取系统调用次数以及获取开始时间

在syscall分发之前，通过调用`increase_syscall_times`根据syscall_id来增加系统调用次数就可以。

最后运行的时间就用当前的时间减去开始时间即可

# 简答题

## 第一题

sbi版本：RustSBI version 0.3.0-alpha.4

三个测例的报错信息：

```
[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.
[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.
```

分别对应的错误原因是：

在访问非法地址`0x0`时，出现访存错误，`trap`的原因在进行特权级切换时，被`trap_handler`捕获。

在U态使用S态特权指令`sret`时出现，错误，批处理系统已经杀死对应任务

在U态访问S态寄存器`sstatus`出现错误，批处理系统杀死对应任务。

## 第二题

1. 刚进入`__restore`时，`a0`代表了内核栈的栈顶指针。

- `__restore`可以用于在Trap结束时，恢复`Trap`的上下文。

- 在任务切换后，需要从内核态进入用户态，通过`__restore`恢复用户态的执行环境。

2. 汇编代码特殊处理了sstatus、sepc、sscratch寄存器。通过这些寄存器让程序从内核态进入用户态。`sstatus`存放了用户态的状态， `sepc`存放切换回用户态之后应该执行的指令地址。而`sscrath`保存了用户栈的栈顶指针，最后通过交换`sp`和`sscrath`的值，让程序正确恢复到用户态。
3. x2就是存放的sp寄存器，存放栈顶指针，可以先跳过。x4是因为用不到
4. 该指令之后，交换了sp和sscratch的值，现在sp指向用户栈栈顶，sscratch指向内核栈栈顶
5. `sret`，这条指定发生之后，cpu指向`sepc`，就切换到了用户态
6. sp指向内核栈栈顶，sscratch指向用户栈栈顶
7.   在`trap.S`中，`__alltraps`实际上已经在内核态了，在`call trap_handler`之后，进入到内核态处理trap

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

