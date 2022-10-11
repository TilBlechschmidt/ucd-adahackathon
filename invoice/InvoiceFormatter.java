package invoice;

import products.Item;

/**
 * Interface for an invoice formatter.
 */

public interface InvoiceFormatter {

  // Format the header of the invoice.
  String formatHeader();

  // Format a line item of the invoice.
  String formatItem(Item item);

 // Format the footer of the invoice.
  String formatFooter();

 // set the total invoice price
  public void setTotalPrice(double totalPrice);

}