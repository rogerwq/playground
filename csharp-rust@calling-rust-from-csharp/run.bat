echo "Build Rust Dynamic Lib......"
cd cs_call_rst
cargo build
cd ..
cp cs_call_rst/target/debug/our_rust.dll rust_dll_poc

echo "Build & Run CSharp exe..."
cd rust_dll_poc
csc hello.cs
hello.exe
cd ..

