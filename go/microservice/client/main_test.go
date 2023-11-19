package main

import (
	"context"
	"testing"

	"github.com/go-openapi/strfmt"
	"my.org/micro/client/client"
	"my.org/micro/client/client/products"
)


func TestOurClient(t *testing.T) {
  cfg := client.DefaultTransportConfig().WithHost("localhost:9090")
  c := client.NewHTTPClientWithConfig(strfmt.Default,cfg)
  ctx := context.Background()
  c.Products.ListProducts(products.NewListProductsParamsWithContext(ctx))

  // c.Products.ListProducts(&params)
  
  // prod, _ := c.Products.ListProducts(params)
  // fmt.Printf("prod: %v\n", prod)

}
