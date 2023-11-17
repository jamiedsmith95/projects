package handlers

import (
	"fmt"
	"log"
	"net/http"

	// "regexp"
	"strconv"

	"github.com/gorilla/mux"
	"my.org/micro/product-api/data"
)

type Products struct {
	l *log.Logger
}

func NewProduct(l *log.Logger) *Products {
	return &Products{l}
}

func (p *Products) RemoveProduct(rw http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id, err := strconv.Atoi(vars["id"])
	p.l.Printf("Attempting to delete product id: %d", id)
	if err != nil {
		http.Error(rw, "Unable to remove product", http.StatusBadRequest)
		fmt.Println(err)
		return
	}
	data.RemoveProduct(id)
}

func (p *Products) ChangeProduct(rw http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id, err := strconv.Atoi(vars["id"])

	if err != nil {
		http.Error(rw, "Unable to convert id", http.StatusBadRequest)
		fmt.Println(err)
		return
	}
	prod := r.Context().Value(KeyProduct{}).(data.Product)

	p.l.Printf("Modified product %#v", prod)
	err = data.ChangeProduct(id, &prod)
	if err != nil {
		http.Error(rw, "unable to change product", http.StatusBadRequest)
		return
	}
}

func (p *Products) AddProducts(rw http.ResponseWriter, r *http.Request) {
  prod := r.Context().Value(KeyProduct{}).(data.Product)
	data.AddProduct(&prod)
	p.l.Printf("Prod added %#v", prod)
}

func (p *Products) UpdateProducts(rw http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id, err := strconv.Atoi(vars["id"])
	if err != nil {
		http.Error(rw, "Unable to convert id", http.StatusBadRequest)
		return
	}

	p.l.Println("Handle PUT id: ", id)
  prod := r.Context().Value(KeyProduct{}).(data.Product)
	err = data.UpdateProduct(id, &prod)

	if err == data.ErrProductNotFound {
		http.Error(rw, "Product not found", http.StatusNotFound)
		return
	}
	}

func (p *Products) GetProducts(rw http.ResponseWriter, r *http.Request) {
	lp := data.GetProducts()
	err := lp.ToJSON(rw)
	if err != nil {
		http.Error(rw, "unable to marshal json", http.StatusInternalServerError)
	}
}

type KeyProduct struct{}

func (p Products) MiddlewareProductValidation(next http.Handler) http.Handler {
	return http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {
		prod := &data.Product{}
		err := prod.FromJSON(r.Body)
		if err != nil {
			http.Error(rw, "unable to unmarshal json", http.StatusBadRequest)
			return
		}
		ctx := r.Context().WithValue(KeyProduct{}, prod)
		req := r.WithContext(ctx)

		next.ServeHTTP(rw, req)
	})
}
