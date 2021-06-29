package main

// #cgo LDFLAGS: -lhello -L${SRCDIR}/libs
// #include "./libs/hello.h"
import "C"

import (
	"fmt"
)

func main() {
	hello := C.hello()
	fmt.Printf("%T %v %v\n", hello, C.GoString(hello))
}
