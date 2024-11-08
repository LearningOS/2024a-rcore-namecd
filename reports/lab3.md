# 功能

首先通过`git cherry-pick`合并了之前`ch4`实现的系统调用，把之前在`TASKMANAGER`中的关于任务的一些方法整改到了`process`中（因为这些方法都是和某一个具体的进程相关的）。

然后实现了`sys_spawn()`系统调用，其实就是`fork + exec`，但是把这两个的部分做了一下整合。直接通过elf文件获取`memoryset`，不用像`fork`一样从父进程的地址空间复制。

最后实现了`stride`算法。在调度时`run_task()`中，让`stride`增加。同时更改了选取进程的算法`fetch_task`，使其通过遍历`ready_queue`来获取`stride`最小的进程

# 简答题

- 实际情况不是P1，仍然是p2。因为8bit的无符号整数溢出，导致p2的stride更小

- 因为在不考虑溢出的情况下， *STRIDE_MAX – STRIDE_MIN* 是一个步长pass的最大值。但是因为`pass = BigStride / _pri`，`_pri >= 2`，所以`pass <= BigStride / 2`。所以 `STRIDE_MAX – STRIDE_MIN <= BigStride / 2 `

- 代码如下：
```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // ...
        let diff = self.0.abs_diff(other.0);
        if diff <= BigStride / 2 {
            if self.0 > other.0{
                Some(Ordering::Greater);
            }
            else{
                Some(Ordering::Less);
            }
        }
        else{
            if self.0 < other.0{
                Some(Ordering::Greater);
            }
            else{
                Some(Ordering::Less);
        }
        
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

```

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 无


2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   >  无


3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。