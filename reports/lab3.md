# lab3 report

## 实验思路

1. 补全调用，抽象进程概念后，get_time是不受影响的，taskinfo维护的变量加入到TCBinner中，变量维护的位置也发生了小小的变化。mmap和munmap总体变化不大，调用TCBinner中的函数即可，实现上基本与ch4无差别

2. 新增的系统调用:spwan，实现思路,实际上，框架的代码把最难的部分做了，即是TCB中的new函数，我们只需要调用这个函数，然后修改父进程和子进程之间的关系即可。对于set_prio调用，实际上直接在TCB中添加stride这个字段值即可，然后添加修改优先权的实现，好像测例对这部分并没有太多的内容？

## 简答
1. 不是，还是p2，p2执行完后，修改stride会发生溢出，变的更小。
2. 这是个很显然的事情，pass<= BIG_STRIDE/2,取pass最大的情况就是BIG_STRIDE/2,也就是说每个任务执行完后都会加pass，每次挑选最小的执行，最大的和最小的插值最多也就是一个pass，即pass <= BIG_STRIDE/2。
3. 根据提示，stride之间差值不会超过BIG_STRIDE/2,如果超过了，就说明出现了溢出，实现如下：

~~~rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 计算 self 和 other 之间的差值
        let diff = self.0.wrapping_sub(other.0);
        
        // 如果 diff 小于等于 u64::MAX / 2，说明 self 比 other 小，否则反之
        if diff <= u64::MAX / 2 {
            Some(Ordering::Greater) 
        } else {
            Some(Ordering::Less)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

~~~

## 荣誉规则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 实验指导书(简洁版本),使用gpt等AI工具辅助理解代码

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

