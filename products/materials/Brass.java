package products.materials;

import products.Item;
import visitor.ItemVisitor;

public class Brass extends ItemDecorator {

    final double BRASS_PREMIUM = 20;
    final String BRASS_DESCRIPTOR = "brass";

    public Brass(Item item) {
        super(item);
    }

    public double getPrice() {
        return BRASS_PREMIUM + item.getPrice();
    }

    public double getBrassPremiumPrice() {
        return BRASS_PREMIUM;
    }
    @Override
    public String toString() {
        return BRASS_DESCRIPTOR + " " + item.toString();
    }

    @Override
    public void accept(ItemVisitor visitor) {
        visitor.visit(this);
        item.accept(visitor);
    }
}
