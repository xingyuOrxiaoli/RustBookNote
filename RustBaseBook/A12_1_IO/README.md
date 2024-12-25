```angular2html
     cargo run --  the A12_1_IO.txt
     IGNORE_CASE=1 cargo run --  the A12_1_IO.txt
用 PowerShell:
     $Env:IGNORE_CASE=1; cargo run --  the A12_1_IO.txt
可以通过 Remove-Item 命令来取消设置
     Remove-Item Env:IGNORE_CASE
```
