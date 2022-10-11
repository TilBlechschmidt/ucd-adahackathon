package products;


import visitor.ItemVisitor;

/**
 * A product with a price and description.
 */

public abstract class Product implements Item {
  
  public Product(String description, double price) {
    this.description = description;
    this.price = price;
  }

  public double getPrice() {
    return price;
  }

  public String toString() {
    return description;
  }


  private String description;
  private double price;
}