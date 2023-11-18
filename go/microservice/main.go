package main

import (
	// "fmt"
	// "io"
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"time"

	"github.com/gorilla/mux"

	handlers "my.org/micro/handlers"
)

func main() {
	l := log.New(os.Stdout, "product-api", log.LstdFlags)
	// hh := handlers.NewHello(l)
	// gh := handlers.NewGoodbye(l)
	ph := handlers.NewProduct(l)

	sm := mux.NewRouter()
  getRouter := sm.Methods("GET").Subrouter()
  getRouter.HandleFunc("/", ph.GetProducts)

  putRouter := sm.Methods("PUT").Subrouter()
  putRouter.HandleFunc("/{id:[0-9]+}", ph.UpdateProducts)
  putRouter.Use(ph.MiddlewareProductValidation)

  postRouter := sm.Methods("POST").Subrouter()
  postRouter.HandleFunc("/", ph.AddProducts)
  postRouter.Use(ph.MiddlewareProductValidation)


  patchRouter := sm.Methods("PATCH").Subrouter()
  patchRouter.HandleFunc("/{id:[0-9]+}", ph.ChangeProduct)
  patchRouter.Use(ph.MiddlewareProductValidation)

  deleteRouter := sm.Methods("DELETE").Subrouter()
  deleteRouter.HandleFunc("/{id:[0-9]+}", ph.RemoveProduct )


	s := &http.Server{
		Addr:         ":9090",
		Handler:      sm,
		IdleTimeout:  120 * time.Second,
		ReadTimeout:  1 * time.Second,
		WriteTimeout: 1 * time.Second,
	}
	go func() {
		err := s.ListenAndServe()
		if err != nil {
			l.Fatal(err)
		}
	}()

	sigChan := make(chan os.Signal)
	signal.Notify(sigChan, os.Interrupt)
	signal.Notify(sigChan, os.Kill)

	sig := <-sigChan
	l.Println("Received terminate, graceful shutdown", sig)

	timeoutContext, _ := context.WithTimeout(context.Background(), 30*time.Second)
	s.Shutdown(timeoutContext)
}
