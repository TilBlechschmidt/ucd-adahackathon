package visitor;

import products.impl.Nails;

public class NumberOfNailsCalculator extends ItemVisitor{

    @Override
    public void visit(Nails nails) {
        number += nails.getNumber();
    }

    public long getNumber() {
        return number;
    }

    private long number = 0;
}
