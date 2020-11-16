mkdir software

# Standard CP/M v2.2 with BIOS and CBIOS sources included.
curl http://www.retroarchive.org/cpm/os/STDCPM22.ZIP -o STDCPM22.ZIP
unzip -u -d software/cpm22 STDCPM22.ZIP
rm STDCPM22.ZIP

# WordStar 3.3 for CP/M-80/Kaypro
curl http://www.retroarchive.org/cpm/text/Wskpro33.zip -o Wskpro33.zip
unzip -u -d software/wordstar33 Wskpro33.zip
rm Wskpro33.zip

# Two versions of Lisp
curl http://www.retroarchive.org/cpm/lang/lisp80.zip -o lisp80.zip
unzip -u -d software/lisp80 lisp80.zip
rm lisp80.zip

# Turbo Pascal v3.01a
curl http://www.retroarchive.org/cpm/lang/TP_301A.ZIP -o TP_301A.ZIP
unzip -u -d software/turbopascal30 TP_301A.ZIP
rm TP_301A.ZIP

# Microsoft BASIC v4.51
curl http://www.retroarchive.org/cpm/lang/OBASIC.COM -o OBASIC.COM
mv OBASIC.COM software

# Microsoft MBASIC v5.29
curl http://www.retroarchive.org/cpm/lang/mbasic.zip -o mbasic.zip
unzip -u -d software mbasic.zip
rm mbasic.zip

# Game of Ladder for CP/M
curl http://www.retroarchive.org/cpm/misc/LADDER.ZIP -o LADDER.ZIP
unzip -u -d software/ladder LADDER.ZIP
rm LADDER.ZIP

# Zork I, II & III for CP/M-80
curl http://www.retroarchive.org/cpm/games/zork123_80.zip -o zork123_80.zip
unzip -u -d software/zork zork123_80.zip
rm zork123_80.zip

# BBC Basic for the Z-80
curl  http://www.bbcbasic.co.uk/bbcbasic/bbccpm.zip -o bbccpm.zip
unzip -u -d software/bbcbasic bbccpm.zip
rm bbccpm.zip

## Gorilla using Turbo Modula-2
wget https://github.com/sblendorio/gorilla-cpm/raw/master/binary/gorilla.com
mv gorilla.com software
