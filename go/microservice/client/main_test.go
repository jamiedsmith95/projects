package main

import (
	"fmt"
	"testing"

	"my.org/micro/client/client"
	"my.org/micro/client/client/products"
)


func TestOurClient(t *testing.T) {
  cfg := client.DefaultTransportConfig().WithHost("localhost:9090")
  c := client.NewHTTPClientWithConfig(nil,cfg)

  params := products.NewListProductsParams()
  prod, err := c.Products.ListProducts(params)

  if err != nil {
    t.Fatal(err)
  }
  fmt.Printf("prod: %v\n", prod)


  // c.Products.ListProducts(&params)
  
  // prod, _ := c.Products.ListProducts(params)
  // fmt.Printf("prod: %v\n", prod)

}
