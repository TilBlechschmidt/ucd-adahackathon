package products.impl;

import products.Product;
import visitor.ItemVisitor;

public class Saw extends Product {


    public Saw(String description, double price, String madeToCut) {
        super(description, price);
        this.madeToCut = madeToCut;
    }

    public String getMaterialSawCuts() {
        return madeToCut;
    }

    @Override
    public void accept(ItemVisitor visitor) {
        visitor.visit(this);
    }

    private String madeToCut;
}
