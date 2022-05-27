# Build RUST Library
```
cd rust
cargo build --release
```

# Run Python code

simply run the main.py
```
python main.py
```

# Run Java Code

Compile java
```
javac Point.java Main.java
```
Now run java (and add path to rust lib)
```
java -cp . -Djava.library.path=rust/target/release Main
```

