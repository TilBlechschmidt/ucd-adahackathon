package products.impl;

import products.Product;
import visitor.ItemVisitor;

public class Nails extends Product {

    public Nails(String description, double price, int number) {
        super(description,price);
        this.number = number;
    }

    @Override
    public String toString() {
        return super.toString() + " (x" + number + ")";
    }

    @Override
    public void accept(ItemVisitor visitor) {
        visitor.visit(this);
    }

    public int getNumber() {
        return this.number;
    }

    private int number;
}
