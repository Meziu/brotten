#!/bin/bash

gnuplot -p -e "set terminal wxt 0;\
	set view map;\
	set palette defined ( 0 'white', 0.07 'red', 0.3 'blue', 1 'black' );\
    set cbrange[0:255];\
	plot [-2:1][-1:1]'/home/andreaciliberti/Desktop/Programming/Rust/brotten/mandel-ncore.out' using 1:2:3 with image;\
	pause mouse close"