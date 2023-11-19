// Package classification of Product API
//
// # Documentation for Product API
//
// Schemes: http
// BasePath: /
// Version: 1.0.0
//
// Consumes:
// - application/json
//
// Produces:
// - application/json
// swagger:meta
package handlers

import (
	"context"
	"fmt"
	"log"
	"net/http"

	// "regexp"
	"strconv"

	"github.com/gorilla/mux"
	"my.org/micro/product-api/data"
)

// swagger:response noContent
type productsNoContent struct {

}

// A list of products returned in the response
// swagger:response productsResponse
type productsResponseWrapper struct {
	// All products in the system
  // in: body
	Body []data.Product
}

// swagger:parameters deleteProduct modifyProduct replaceProduct
type productIDParameterWrapper struct {
  // The id of the product to perform operation on
  // in: path
  // required: true
  ID int `json:"id"`
}

type Products struct {
	l *log.Logger
}

func NewProduct(l *log.Logger) *Products {
	return &Products{l}
}

// swagger:route DELETE /products/{id} products deleteProduct
// Removes the information from the specified product
// responses:
// 201: noContent

// RemoveProduct deletes a product from the database
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

// swagger:route PATCH /products/{id} products modifyProduct
// Modifies the specified product, altering the specified fields
// responses:
// 201: noContent
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

// swagger:route POST /products products addProduct
// Adds a new product onto the existing list of products
// responses:
// 201: noContent
func (p *Products) AddProducts(rw http.ResponseWriter, r *http.Request) {
	prod := r.Context().Value(KeyProduct{}).(data.Product)
	data.AddProduct(&prod)
	p.l.Printf("Prod added %#v", prod)
}

// swagger:route PUT /products/{id} products replaceProduct
// Replaces a product with a new product
// responses:
// 201: noContent
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

// swagger:route GET /products products listProducts
// Returns a list of products
// Responses:
//  200: productsResponse

// GetProducts returns the products from the datastore
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
		prod := data.Product{}
		err := prod.FromJSON(r.Body)
		if err != nil {
			http.Error(rw, "unable to unmarshal json", http.StatusBadRequest)
			return
		}
		// validate the product
		err = prod.Validate()
		if err != nil {
			p.l.Println("[ERROR] validating product", err)
			http.Error(rw, "Error validating product", http.StatusBadRequest)
			return
		}
		ctx := context.WithValue(r.Context(), KeyProduct{}, prod)
		r = r.WithContext(ctx)

		next.ServeHTTP(rw, r)
	})
}
