package visitor;

import products.impl.Hammer;
import products.impl.Nails;
import products.impl.Saw;
import products.materials.Brass;
import products.materials.Steel;
import products.materials.Titanium;

public class PriceCalculator extends ItemVisitor {

    @Override
    public void visit(Hammer hammer) {
        total += hammer.getPrice();
    }

    @Override
    public void visit(Nails nails) {
        total += nails.getPrice();
    }

    @Override
    public void visit(Saw saw) {
        total += saw.getPrice();
    }

    @Override
    public void visit(Brass brass) {
        total += brass.getBrassPremiumPrice();
    }

    @Override
    public void visit(Steel steel) {
        total += steel.getSteelPremiumPrice();
    }

    @Override
    public void visit(Titanium titanium) {
        total += titanium.getTitaniumPremiumPrice();
    }

    public double getTotalPrice() {
        return total;
    }

    private double total = 0;
}
