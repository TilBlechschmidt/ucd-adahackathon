package visitor;

import products.impl.Hammer;
import products.impl.Nails;
import products.impl.Saw;
import products.materials.Brass;
import products.materials.Steel;
import products.materials.Titanium;

public abstract class ItemVisitor {
    public void visit(Hammer hammer) {
    }
    
    public void visit(Nails nails) {
    }
    
    public void visit(Saw saw) {
    }
    
    public void visit(Brass brass) {
    }

    public void visit(Steel steel) {
    }
    
    public void visit(Titanium titanium) {
    }
}
