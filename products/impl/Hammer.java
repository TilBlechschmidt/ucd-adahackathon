package products.impl;

import products.Product;
import visitor.ItemVisitor;

public class Hammer extends Product {

    public Hammer(String description, double price, int weight) {
        super(description,price);
        this.weight = weight;
    }

    @Override
    public void accept(ItemVisitor visitor) {
        visitor.visit(this);
    }

    public int getWeight() {
        return weight;
    }

    private int weight;
}
