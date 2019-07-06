fn main() {
    {
        // 堆和栈
        // 进程在执行的时候，占用内存的虚拟地址空间被分成几个“段”
        // 代码段；bss 段，未初始化的全局变量和静态变量；数据段，存放初始化的全局变量和静态变量；
        // 函数调用栈：存放函数参数、局部变量、以及其他函数调用相关信息的区域；
        // 堆：存放动态分配内存的区域；
    }
    {
        // 段错误
    }
    {
        // 内存安全
        // rust 设计上试图避免的内存不安全问题：
        // 解引用空指针
        // 使用未初始化的指针 —— 野指针
        // 悬空指针
        // 使用未初始化内存
        // 非法释放
        // 缓冲区溢出
        // 执行非法函数指针
        // 数据竞争
    }
}