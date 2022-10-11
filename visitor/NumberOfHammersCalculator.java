package visitor;

import products.impl.Hammer;

public class NumberOfHammersCalculator extends ItemVisitor {

    @Override
    public void visit(Hammer hammer) {
        number++;
    }

    public long getNumber() {
        return number;
    }

    private long number = 0;
}
