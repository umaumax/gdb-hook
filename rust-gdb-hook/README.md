# rust func hook

## how to build
```
RUSTFLAGS="-C link-args=-rdynamic" cargo build

# 本体とは個別にビルドする必要がある
rustc --crate-type="dylib" -C link-dead-code -g src/hook.rs -o libadd_hook.so
# or
g++ -g3 -std=c++11 -shared -fPIC src/hook.cpp -o libadd_hook.so
```

同一のバイナリにhook関数を仕込む場合(`sub`,`sub_hook`)
```
RUSTFLAGS="-C link-dead-code" cargo build
```

## how to run
```
# terminal A
LD_PRELOAD=./libstop_the_world.so:./libadd_hook.so ./target/debug/rust-gdb-hook

# same as c++ version
```

## `-rdyamic`なしでgdbでのhookを実現したい場合の方法

case: C++

```cpp
#include <functional>

// [c\+\+ \- Storing function pointer in std::function \- Stack Overflow]( https://stackoverflow.com/questions/4770968/storing-function-pointer-in-stdfunction )
template <typename Signature>
std::function<Signature> cast(void* f) {
  return reinterpret_cast<Signature*>(f);
}
int add_hookX(void* f, int a, int b) {
  printf("hook function called!\n");
  return cast<int(int, int)>(f)(a, b) + 123;
}
```

case: rust

`WIP`

----

## memo
* 共有ライブラリ(C++)からrustの関数を呼び出す場合はrustのELFが共有ライブラリとして利用できる状態である必要がある(`-rdynamic`付きでリンクされていること)
  * なお、共有ライブラリ(C++)からC++の関数を呼び出す場合に、gdbを経由する場合は、`-rdynamic`は必要ない
* 共有ライブラリ(Rust)からrust/C++の関数を呼び出す場合はrustのELFが共有ライブラリとして利用できる状態である必要がある
  * その理由は、`LD_PRELOAD`で指定したrustの共有ライブラリをロードした時点(関数呼び出し時ではない)で未解決のシンボルの解決を行う挙動のため、最初のロード時に情報が取得できる状態である必要があるためである
  * `readelf -d ./libadd_hook.so`とすると、`(BIND_NOW)`,`(FLAGS_1) Flags: NOW`となっていることから、`LD_BIND_NOW=yes`相当の処理となっていることがわかる
  * `rustc`のログを見ると、`cc`に`-Wl,-zrelro -Wl,-znow`を渡している
    * [ld \-z relro で GOT overwrite attack から身を守る \- memologue]( https://yupo5656.hatenadiary.org/entry/20060618/p1 )
    * `RELRO(RELocation Read-Only)`で調べると良い
    * [c \- How to disable relro to overwrite fini\_array or got\.plt element \- Stack Overflow]( https://stackoverflow.com/questions/60493027/how-to-disable-relro-to-overwrite-fini-array-or-got-plt-element )
      * disableする方法は再ビルド以外には無理やりバイナリ書き換え
* ⚠ 関数のシンボル名とファイル名(拡張子を除く)が同名だとgdbで`call`する際に、あいまいであるというエラーとなるため注意
* 今回の例ではhook先の`add`関数は`i32(int)`のみを利用しているので、rust側で`extern "C"`を宣言する必要なく、c++とrustの両方からhookすることができる
* 今回の例では`#[no_mangle]`を付加しているが、関数名を探しやすくする目的で利用しており、必ずしも必要ではない

### extern
* `extern`を付加すると基本的にC ABIを用いることになるので、呼び出し時には`unsafe`が必要になる

#### `extern`ブロックで指定可能なABI
[External blocks \- The Rust Reference]( https://doc.rust-lang.org/reference/items/external-blocks.html#abi )

----

## links
* [Expose symbols to dynamic linker when linking with native library in Rust \- Stack Overflow]( https://stackoverflow.com/questions/34082636/expose-symbols-to-dynamic-linker-when-linking-with-native-library-in-rust )
