import invoice.Invoice;
import invoice.InvoiceFormatter;
import invoice.SimpleFormatter;
import products.Bundle;
import products.factories.HammerFactory;
import products.factories.NailFactory;
import products.factories.SawFactory;
import visitor.NumberOfHammersCalculator;
import visitor.NumberOfNailsCalculator;
import visitor.NumberOfUnitsOfSteelCalculator;
import visitor.PriceCalculator;

import java.text.SimpleDateFormat;


/**
 * App to create simple invoices.
 */

public class InvoiceApplication {
  
  public static void main(String[] args) {
    SimpleDateFormat simpleDateFormat = new SimpleDateFormat("HH:mm:ss.SSSSSS");

    // Measure of how much work to do
    long noOfBundles = Long.parseLong(args[0]);
    // long noOfBundles = 5000;

    // **************START THE CLOCK!**************
    long startTime = System.currentTimeMillis();
    final Invoice invoice = new Invoice();
    final InvoiceFormatter formatter = new SimpleFormatter();

    // Create given number of bundles and add them to invoice
    for(int i = 0; i < noOfBundles; i++) {
      Bundle starterBundle = new Bundle();
      starterBundle.add(HammerFactory.createLightSteelFramingHammer());
      starterBundle.add(NailFactory.createLargeSteelFramingNails());
      starterBundle.add(NailFactory.createMediumBrassRoofingNails());
      starterBundle.add(SawFactory.createSteelJigsaw());
      starterBundle.add(HammerFactory.createHeavySteelSledgeHammer());
      invoice.addItem(starterBundle);
    }

    // Format invoice for the customer
    String formattedInvoice = invoice.format(formatter);

    // Computer data for the Company
    NumberOfHammersCalculator numHammersCalculator = new NumberOfHammersCalculator();
    invoice.accept(numHammersCalculator);
    long numberOfHammers = numHammersCalculator.getNumber();

    NumberOfUnitsOfSteelCalculator steelUnitsCalculator = new NumberOfUnitsOfSteelCalculator();
    invoice.accept(steelUnitsCalculator);
    long numberOfUnitsOfSteelRequired =  steelUnitsCalculator.getNumber();

    NumberOfNailsCalculator numNailsCalculator = new NumberOfNailsCalculator();
    invoice.accept(numNailsCalculator);
    long numberOfNailsDispensed = numNailsCalculator.getNumber();

    PriceCalculator calculator = new PriceCalculator();
    invoice.accept(calculator);
    double totalPrice = calculator.getTotalPrice();

    // **************STOP THE CLOCK!**************
    long endTime = System.currentTimeMillis(); 
    int duration = (int)(endTime - startTime); 

    //System.out.println(formattedInvoice); // print customer invoice

    // Print data for company
    System.out.println("\nData For Company Use");
    System.out.println("=====================");
    System.out.println("Number of hammers sold: " + numberOfHammers);
    System.out.println("Number of units of steel required: " + numberOfUnitsOfSteelRequired);
    System.out.println("Number of nails dispensed: " + numberOfNailsDispensed);
    System.out.println(String.format("Total price: €%,.2f\n", totalPrice));
    System.out.println("\n");

    // Print runtime stats
    System.out.println("START TIME: " + simpleDateFormat.format(startTime));
    System.out.println("END TIME:   " + simpleDateFormat.format(endTime));
    System.out.println("DURATION:   " + duration + " milliseconds");
  }
}