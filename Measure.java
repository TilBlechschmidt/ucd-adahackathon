
public class Measure {
    String label;
    long start;

    public Measure(String label) {
        System.out.println("START\t" + label);
        this.label = label;
        this.start = System.currentTimeMillis();
    }

    public void finish() {
        long end = System.currentTimeMillis();
        System.out.println("END\t" + label + " = " + (end - start) + "ms");
    }
}