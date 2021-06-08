# gdb

## how to build
``` bash
g++ -g3 -std=c++11 ./main.cpp -o main
g++ -g3 -std=c++11 -shared -fPIC hook.cpp -o libhook.so
g++ -g3 -std=c++11 ./main_hook.cpp -o main_hook
```

## how to run
``` bash
# terminal A
LD_PRELOAD=./libstop_the_world.so:./libhook.so ./main

# teminal B
sudo gdb

attach $PID

source ./gdb-trace.py
trace-functions .*_hook

# terminal A
fg

# terminal B
c
c
```

* `gdb`を利用する場合には`rdynamic`関係なしに、共有ライブラリから実行ファイルの関数を呼び出すことが可能
  * gdbが実行ファイルからシンボルテーブルを作成しているので，共有ライブラリがそれを見つけることができるということ?
* `main_hook`の場合、実行ファイルにhookするコマンドのシンボルを実行前に探すことができるので、下記の手順のようにわざわざattachする必要なしにhook可能
* 実行ファイルをリンクするときに`-rdynamic`を付加とすると、共有ライブラリから実行ファイルのシンボル情報を動的に取得できるようになる
  * [gccの\-rdynamic option 調査メモ \- Qiita]( https://qiita.com/takeoverjp/items/14fdf7ab0d0a76d83d30 )
* ちなみに，仮に，`main.cpp`で`addXXX_hook`を呼び出すようにして、`-L. -lhook`を下記のコマンドに付加した際に、`-rdynamic`が無いと共有ライブラリ側から`addXXX`が見つからずにエラーとなる

[rust\-examples/stop\-the\-world at master · umaumax/rust\-examples]( https://github.com/umaumax/rust-examples/tree/master/stop-the-world )

## finish

`finish`直後は
`p` or `p $0`
で返り値がわかる

アーキテクチャ依存で下記でもよい
`x86`: `print $eax`
`x86_64`: `print $rax`

frameからリターンしている状態、つまり、関数を抜けていることに注意(`info args`の表示内容が異なる)

## return

現在のframe(関数)の返り値を上書きできる

つまり、代替関数を呼んでreturnすると関数の置き換えができる

## info args
* info argsは引数となっている変数の現在の値を出力する
  * frame内で処理を進めていき，変数の中身が変化するとそれに対応して変化する

## 関数から戻る直前にbreakpointを設定したい
* [c\+\+ \- How to set a breakpoint in GDB where the function returns? \- Stack Overflow]( https://stackoverflow.com/questions/3649468/how-to-set-a-breakpoint-in-gdb-where-the-function-returns )
  * returnする直前にbreakpointを貼るのは難しいので、一旦finishしてからreverseする方法が無難

## rustとc++が混在してる実行ファイルをデバッグしたとき
明示的に下記のような言語指定をしないと、gdb.executeなどでコマンドを実行する際に、余計な文字列も出力されてしまう

`set language c++`

## memo
* [Executing commands at breakpoint in gdb via python interface \- Stack Overflow]( https://stackoverflow.com/questions/31380754/executing-commands-at-breakpoint-in-gdb-via-python-interface )

----

## FYI: gdbで関数のトレースをするpython script
[Trace all function calls using gdb]( https://gist.github.com/Houdini/6a688fe06cc12b84fb61 )
