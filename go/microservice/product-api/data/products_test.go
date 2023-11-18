package data

import "testing"

func TestChecksValidation(t *testing.T) {
	p := &Product{
    Name: "test drink",
    Description: "Serves no purpost but to test",
    Price: 1,
    SKU: "ri",
  }

	err := p.Validate()
	if err != nil {
		t.Fatal(err)
	}
}
