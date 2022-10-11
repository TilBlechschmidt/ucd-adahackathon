package products.factories;

import products.Item;
import products.impl.Nails;
import products.materials.Brass;
import products.materials.Steel;

public class NailFactory {

    //Brass nails for a fancy brass roof
    public static Item createSmallBrassRoofingNails(){
        return new Brass(
                new Nails("roofing nails", 10.0, 20));
    }

    public static Item createMediumBrassRoofingNails(){
        return new Brass(
                new Nails("roofing nails", 20.0, 50));
    }

    public static Item createLargeBrassRoofingNails(){
        return new Brass(
                new Nails("roofing nails", 35.0, 100));
    }

    //Steel nails for framing
    public static Item createSmallSteelFramingNails() {
        return new Steel(
                new Nails("framing nails", 15.0,50));
    }

    public static Item createMediumSteelFramingNails() {
        return new Steel(
                new Nails("framing nails",25.0,100));
    }

    public static Item createLargeSteelFramingNails() {
        return new Steel(
                new Nails("framing nails",45,200));
    }

}
