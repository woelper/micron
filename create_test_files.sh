for size in 1 2 4 8 10 100 1000;
do
    yes this is a $size MB test file | head -c ${size}MB > tmptest${size}MB.txt
    nl tmptest${size}MB.txt > test${size}MB.txt
    rm tmptest${size}MB.txt
    echo made a $size MB file
done