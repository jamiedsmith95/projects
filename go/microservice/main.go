package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

func main() {
  fmt.Println("program runs")
  // greedy matching anything other than requested will return the "/" function.
  http.HandleFunc("/", func(rw http.ResponseWriter, r*http.Request) {
    log.Println("This works")
    d, _ := ioutil.ReadAll(r.Body)
    fmt.Fprintf(rw, "Hello %s\n", d)
    fmt.Println("reached here")

  }) 

  http.HandleFunc("/goodbye", func(http.ResponseWriter, *http.Request) {
    log.Println("goodbye World")
     
  }) 

  http.ListenAndServe(":9090", nil)

}
