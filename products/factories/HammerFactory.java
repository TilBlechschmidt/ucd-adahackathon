package products.factories;

import products.Item;
import products.impl.Hammer;
import products.materials.Titanium;
import products.materials.Steel;

public class HammerFactory {


    public static Item createLightSteelSledgeHammer() {
        return new Steel(
                new Hammer("sledge hammer", 25, 7));
    }

    public static Item createHeavySteelSledgeHammer() {
        return new Steel(
                new  Hammer("sledge hammer", 40, 14));
    }

    public static Item createLightTitaniumFramingHammer() {
        return new Titanium(
                new Hammer("framing hammer", 60, 1));
    }

    public static Item createLightSteelFramingHammer() {
        return new Steel(
                new Hammer("framing hammer", 30, 3));
    }
}
