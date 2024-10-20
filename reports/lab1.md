# ch3 lab1

## 实现思路
只需要维护一下taskinfo信息即可，taskinfo有几个字段，首先状态不再需要维护，syscall_time在每次的进行syscall调用的时候进行维护即可，time字段在run_first_task和切换task记录下start_time即可

## 简答问题
1. 
~~~shell
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
~~~

2. a0为指向当前的内核栈sp，进入该行有两种情况，一种是trap，一种是中断进入。

3. sstatus存放用户态的执行环境。sepc用来指示trap发生时程序在用户态中的执行位置。当通过sret指令返回用户态时，处理器会跳转到sepc保存的地址，恢复用户态程序的执行。sscratch的值为用户态的栈指针sp

4. x2是sp，开头就被修改，x4是tp，本次实验没什么用

5. 恢复用户态，准备进入U态，执行用户程序

6. sret，该指令执行后，cpu会根据sstatus和sepc的值，切换到用户态，并跳转到sepc指定的位置，继续执行用户态程序。

7. 使得sp指向内核栈,sscratch指向用户栈，进入内核态，开始处理trap

8. ecall，ecall指令是用户态到内核态的调用指令，它会触发一个异常，进入内核态，

## 荣誉规则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 与同组队员交流过sys_get_time()函数计算问题

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> 参考实验指定教材

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。