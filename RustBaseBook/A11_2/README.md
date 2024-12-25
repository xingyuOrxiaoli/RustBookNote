```
    可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件
    cargo test -- --test-threads=1
    展示 测试中打印的值
    cargo test -- --show-output
    通过指定名字来运行部分测试
    cargo test this_test_will_pass
    只运行被忽略的测试
    cargo test -- --ignored
    不管是否忽略都要运行全部测试可以运行
    cargo test -- --include-ignored
```