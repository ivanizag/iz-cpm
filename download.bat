mkdir software

:: Standard CP/M v2.2 with BIOS and CBIOS sources included.
powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/os/STDCPM22.ZIP -OutFile STDCPM22.ZIP"
powershell -Command "Expand-Archive -DestinationPath software\cpm22 -LiteralPath STDCPM22.ZIP"
del STDCPM22.ZIP
:: WordStar 3.3 for CP/M-80/Kaypro
powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/text/Wskpro33.zip -OutFile Wskpro33.zip"
powershell -Command "Expand-Archive -DestinationPath software/wordstar33 -LiteralPath Wskpro33.zip"
del Wskpro33.zip

:: Two versions of Lisp
powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/lang/lisp80.zip -OutFile lisp80.zip"
powershell -Command "Expand-Archive -DestinationPath software/lisp80 -LiteralPath lisp80.zip"
del lisp80.zip
:: Turbo Pascal v3.01a
powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/lang/TP_301A.ZIP -OutFile TP_301A.ZIP"
powershell -Command "Expand-Archive -DestinationPath software/turbopascal30 -LiteralPath TP_301A.ZIP"
del TP_301A.ZIP

:: Microsoft BASIC v4.51
del software/OBASIC.COM
powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/lang/OBASIC.COM -OutFile OBASIC.COM"
move OBASIC.COM software

:: Microsoft MBASIC v5.29
::powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/lang/mbasic.zip -OutFile mbasic.zip"
::powershell -Command "Expand-Archive -DestinationPath software -LiteralPath mbasic.zip"
::del mbasic.zip

:: Game of Ladder for CP/M
powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/misc/LADDER.ZIP  -OutFile LADDER.ZIP"
powershell -Command "Expand-Archive -DestinationPath software/ladder -LiteralPath LADDER.ZIP"
del LADDER.ZIP

:: Zork I, II & III for CP/M-80
::powershell -Command "Invoke-WebRequest http://www.retroarchive.org/cpm/games/zork123_80.zip -OutFile zork123_80.zip"
::powershell -Command "Expand-Archive -DestinationPath software/zork -LiteralPath zork123_80.zip"
::del zork123_80.zip
