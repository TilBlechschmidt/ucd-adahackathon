package products;

import visitor.ItemVisitor;

/**
 * An item in an invoice.
 */

public interface Item {
  double getPrice();
  String toString();
  void accept(ItemVisitor visitor);
}