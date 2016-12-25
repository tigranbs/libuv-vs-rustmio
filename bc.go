package main

import (
	"io/ioutil"
	"os"
	"net"
	"fmt"
	"strconv"
	"time"
)

func main() {
	// reading file and keeping it in memory for sending it
	// as a batch of data
	buffer, err := ioutil.ReadFile(os.Args[3])
	if err != nil {
		fmt.Println("Unable to read given file", err.Error())
		return
	}

	conn_count, err := strconv.Atoi(os.Args[2])
	if err != nil {
		fmt.Println("Connections count should be valid number", err.Error())
		return
	}

	for i := 0; i < conn_count; i++ {
		go run_connection(os.Args[1], buffer)
	}

	// just waiting until exit
	for {
		time.Sleep(time.Second * 100)
	}
}

func run_connection(address string, buffer []byte) {
	addr, err := net.ResolveTCPAddr("tcp", address)
	if err != nil {
		fmt.Println("Unable to resolve address", err.Error())
		return
	}

	conn, err := net.DialTCP("tcp", nil, addr)
	if err != nil {
		fmt.Println("Unable to connect", err.Error())
		return
	}

	// just to free up socket buffer sent by echo server
	readable_buffer := make([]byte, 64000)

	go func()  {
		for {
			_, err = conn.Read(readable_buffer)
			if err != nil {
				conn.Close()
				return
			}
		}
	}()

	for {
		conn.Write(buffer)
		time.Sleep(time.Millisecond * 100)
	}
}
