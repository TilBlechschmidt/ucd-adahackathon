#!/bin/bash

# Builds the Invoice application, runs it 10 times with input 1000
# and outputs the average evergy consumed.

numberOfRuns=10
declare -a array

javac *.java

for ((i=0;i<$numberOfRuns;i++)); do
    array[$i]=$(java -Djava.compiler=NONE InvoiceApplication 1000 | tail -1 | cut -d ' ' -f4)
done

sum=0
for ((i=0;i<$numberOfRuns;i++)); do
    sum=$((sum+${array[$i]}))
done

((avg=sum/numberOfRuns))
echo "Average run time over ${numberOfRuns} runs:"
echo "$avg milliseconds"
