package handlers

import (
	"encoding/json"
	"log"
	"net/http"

	"my.org/micro/product-api/data"
)

type Products struct {
	l *log.Logger
}

func NewProduct(l *log.Logger) *Products {
	return &Products{l}
}

func (p *Products) ServeHTTP(rw http.ResponseWriter, h *http.Request) {
  lp := data.GetProducts()
  d, err := json.Marshal(lp)
  if err != nil {
    http.Error(rw, "unable to marshal json",http.StatusInternalServerError)
  }
  rw.Write(d)

}
