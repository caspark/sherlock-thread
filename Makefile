CFLAGS = -g -Wall -std=c11

build: build-c build-hack-c build-java build-hack-rust

build-c-target:
	mkdir -p target/c

build-c: build-c-target c/hello.c
	gcc $(CFLAGS) -o target/c/hello c/hello.c

build-java-target:
	mkdir -p target/java

build-java: build-java-target
	javac java/HelloWorld.java -d target/java

build-hack-c: build-c-target c/hack.c
	gcc $(CFLAGS) -shared -fPIC -o target/c/hack-c.so c/hack.c

build-hack-rust:
	cargo build

run-c: build-c
	target/c/hello

run-c-hack-c: build-c build-hack-c
	LD_PRELOAD=target/c/hack-c.so target/c/hello

run-c-hack-rust: build-c build-hack-rust
	LD_PRELOAD=target/debug/libsherlockthread.so target/c/hello

run-javav:
	java -version

run-javav-hack-rust: build-hack-rust
	LD_PRELOAD=target/debug/libsherlockthread.so java -version

run-java: build-java
	java -cp target/java HelloWorld

run-java-hack-rust: build-java build-hack-rust
	LD_PRELOAD=target/debug/libsherlockthread.so java -cp target/java HelloWorld

run-mvn-hack-rust: build-hack-rust
	LD_PRELOAD=target/debug/libsherlockthread.so mvn

clean:
	rm -r target
