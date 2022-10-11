package visitor;

import products.materials.Steel;

public class NumberOfUnitsOfSteelCalculator extends ItemVisitor {

    @Override
    public void visit(Steel steel) {
        number++;
    }

    public long getNumber() {
        return number;
    }

    private long number = 0;
}
