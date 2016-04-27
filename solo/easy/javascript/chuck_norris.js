var MESSAGE = readline();
var BinTmp = new Array();
// Write an action using print()
// To debug: printErr('Debug messages...');

for (i = 0; i < MESSAGE.length; i++) {
    BinTmp[i] = parseInt(MESSAGE.charCodeAt(i)).toString(2);
    if (BinTmp[i].length < 7) {
        BinTmp[i] = '0000000' + BinTmp[i];
        BinTmp[i] = BinTmp[i].slice(BinTmp[i].length - 7);
    }
}

var BinMessage = BinTmp.join('');
//var Bin1 = BinMessage.split('0');
//var Bin0 = BinMessage.split('1');

var ind = 0;
var bit = 0;
var prevind = 0;
var Unaire = '';

while (ind < BinMessage.length) {
    ind = BinMessage.indexOf(bit, ind);
    if (ind == 0) {
        bit = 1;
        ind = BinMessage.indexOf(bit, ind);
    }

    switch (bit) {
        case 0:
            Unaire += ' 0 ';
            bit = 1;
            break;
        case 1:
            Unaire += ' 00 ';
            bit = 0;
            break;
    }

    if (ind != -1) {
        l = ind - prevind;
    } else {
        l = BinMessage.length - prevind;
        ind = BinMessage.length;
    }

    for (i = 0; i < l; i++) {
        Unaire += '0';
    }

    prevind = ind;
}

print(Unaire.trim());
