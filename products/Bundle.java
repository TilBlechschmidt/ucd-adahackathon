package products;


import visitor.ItemVisitor;

import java.util.*;

/**
 *  A bundle of items
 */

public class Bundle implements Item {
  
  public Bundle() {
    items = new ArrayList<Item>();
  }

  public void add(Item item) {
    items.add(item);
  }

  @Override
  public double getPrice() {
    double price = 0;
    for (Item item : items)
      price += item.getPrice();
    return price;
  }

  @Override
  public String toString() {
    String description = "Bundle: ";
    for (int i = 0; i < items.size(); i++) {
      if (i > 0)
        description += ", ";
      description += items.get(i).toString();
    }
    return description;
  }

  public void accept(ItemVisitor visitor) {
    for (Item item : items) {
      item.accept(visitor);
    }
  }

  private ArrayList<Item> items;
}