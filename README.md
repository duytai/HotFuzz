# HotFuzz

Install dependencies and download pre-built **rustc** compiler

```bash
git clone --recursive git@github.com:duytai/HotFuzz.git
cd HotFuzz
./boostrap.sh
```

Instrument the `HelloWorld` project

```bash
cd HotFuzz/helloworld/
RUSTFLAGS="-C passes=HotFuzz -l afl-llvm-rt" RUSTC=../stage2/bin/rustc cargo build --release
```

Fuzzing `helloworld` binary file

```bash
python3 nnf/nnf.py helloworld.ini
```

Fuzzing's output is saved to `programs/helloworld`

To fuzz new rust project
+ Instrumentation
+ Creating an `.ini` file which is similar to `helloword.ini`
+ Fuzzing: `python3 nnf/nnf.py [your ini file]`

**Note**: 

Input files must have following format: id:000000,orig:<file_name> (e.g., id:000000,orig:jpeg-quality-10.jpg)
