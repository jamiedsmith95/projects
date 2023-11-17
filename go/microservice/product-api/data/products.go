package data

import (
	"encoding/json"
	"fmt"
	"io"
	"time"
)
type Updater interface {
  setVal() error
}

type Product struct {
	ID          int     `json:"id"`
	Name        string  `json:"name"`
	Description string  `json:"description"`
	Price       float32 `json:"price"`
	SKU         string  `json:"sku"`
	CreatedOn   string  `json:"-"`
	UpdatedOn   string  `json:"-"`
	DeletedOn   string  `json:"-"`
}

type Products []*Product

func (p *Products) ToJSON(w io.Writer) error {
	e := json.NewEncoder(w)
	return e.Encode(p)
}

func (p *Product) FromJSON(r io.Reader) error {
	d := json.NewDecoder(r)
	return d.Decode(p)
}
func RemoveProduct(id int) error {
  _,pos, err := findProduct(id)
  if err != nil {
    return err
  }
  p := productList[pos]
  p.DeletedOn = time.Now().UTC().String()
  prod := &Product {
    ID: p.ID,
    DeletedOn: p.DeletedOn,
  }
  productList[pos] = prod
  return nil
}

func UpdateProduct(id int, p *Product) error {
	_, pos, err := findProduct(id)
	if err != nil {
		return err
	}
	p.ID = id
	productList[pos] = p
	return nil
}

var ErrProductNotFound = fmt.Errorf("Product not found")
  

func (p *Product) setVal(np *Product) error {
  if len(np.Name) != 0 {
  p.Name = np.Name
  }
  if len(np.Description) != 0 {
  p.Description = np.Description
  }
  if np.Price > 0 {
  p.Price = np.Price
  }
  if len(np.SKU) != 0 {
  p.SKU = np.SKU
  }
  p.UpdatedOn = time.Now().UTC().String()
  return nil
}


func ChangeProduct(id int, np *Product) error {
  _, pos, err := findProduct(id)
	if err != nil {
		return err
	}
  p := productList[pos]
  p.setVal(np)
  return nil
   
    
}

func findProduct(id int) (*Product, int, error) {
	for i, p := range productList {
		if p.ID == id {
			return p, i, nil
		}
	}
	return nil, 0, ErrProductNotFound
}

func GetProducts() Products {
	return productList
}

func AddProduct(p *Product) {
	p.ID = getNextId()
	productList = append(productList, p)
}

func getNextId() int {
	lp := productList[len(productList)-1]
	return lp.ID + 1
}


var productList = []*Product{
	{
		ID:          1,
		Name:        "Latte",
		Description: "Frothy coffee",
		Price:       2.45,
		SKU:         "abc323",
		CreatedOn:   time.Now().UTC().String(),
		UpdatedOn:   time.Now().UTC().String(),
	},
	{
		ID:          2,
		Name:        "Esspresso",
		Description: "Short and strong coffee",
		Price:       1.99,
		SKU:         "fjd34",
		CreatedOn:   time.Now().UTC().String(),
		UpdatedOn:   time.Now().UTC().String(),
	},
}