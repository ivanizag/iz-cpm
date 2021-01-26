cd ld80
make
cd ..

cd zmac
make
cd ..

cd zcpr1
../zmac/zmac --rel7 --zmac -8 zcpr.asm -o zcpr.rel -o zcpr.lst
../ld80/ld80 -O bin -o zcpr.mem -Pf000 zcpr.rel
tail -c+61441 zcpr.mem > zcpr.bin
cd ..

mkdir -p bin
cp zcpr1/zcpr.bin bin
cp zcpr1/zcpr.lst bin
