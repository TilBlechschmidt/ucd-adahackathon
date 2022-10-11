#!/bin/sh
javac *.java
java -Djava.compiler=NONE InvoiceApplication 1000
find . -name "*.class" -exec rm {} \;
