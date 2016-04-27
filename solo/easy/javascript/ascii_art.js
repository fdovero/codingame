var L = parseInt(readline());
var H = parseInt(readline());
var T = readline().toLowerCase();
var letter = new Array();
letter['a'] = new Array();
letter['b'] = new Array();
letter['c'] = new Array();
letter['d'] = new Array();
letter['e'] = new Array();
letter['f'] = new Array();
letter['g'] = new Array();
letter['h'] = new Array();
letter['i'] = new Array();
letter['j'] = new Array();
letter['k'] = new Array();
letter['l'] = new Array();
letter['m'] = new Array();
letter['n'] = new Array();
letter['o'] = new Array();
letter['p'] = new Array();
letter['q'] = new Array();
letter['r'] = new Array();
letter['s'] = new Array();
letter['t'] = new Array();
letter['u'] = new Array();
letter['v'] = new Array();
letter['w'] = new Array();
letter['x'] = new Array();
letter['y'] = new Array();
letter['z'] = new Array();
letter['?'] = new Array();
var word = new Array();
for (var i = 0; i < H; i++) {
    var ROW = readline();
    letter['a'][i] = ROW.substring(0, L);
    letter['b'][i] = ROW.substring(L, 2 * L);
    letter['c'][i] = ROW.substring(2 * L, 3 * L);
    letter['d'][i] = ROW.substring(3 * L, 4 * L);
    letter['e'][i] = ROW.substring(4 * L, 5 * L);
    letter['f'][i] = ROW.substring(5 * L, 6 * L);
    letter['g'][i] = ROW.substring(6 * L, 7 * L);
    letter['h'][i] = ROW.substring(7 * L, 8 * L);
    letter['i'][i] = ROW.substring(8 * L, 9 * L);
    letter['j'][i] = ROW.substring(9 * L, 10 * L);
    letter['k'][i] = ROW.substring(10 * L, 11 * L);
    letter['l'][i] = ROW.substring(11 * L, 12 * L);
    letter['m'][i] = ROW.substring(12 * L, 13 * L);
    letter['n'][i] = ROW.substring(13 * L, 14 * L);
    letter['o'][i] = ROW.substring(14 * L, 15 * L);
    letter['p'][i] = ROW.substring(15 * L, 16 * L);
    letter['q'][i] = ROW.substring(16 * L, 17 * L);
    letter['r'][i] = ROW.substring(17 * L, 18 * L);
    letter['s'][i] = ROW.substring(18 * L, 19 * L);
    letter['t'][i] = ROW.substring(19 * L, 20 * L);
    letter['u'][i] = ROW.substring(20 * L, 21 * L);
    letter['v'][i] = ROW.substring(21 * L, 22 * L);
    letter['w'][i] = ROW.substring(22 * L, 23 * L);
    letter['x'][i] = ROW.substring(23 * L, 24 * L);
    letter['y'][i] = ROW.substring(24 * L, 25 * L);
    letter['z'][i] = ROW.substring(25 * L, 26 * L);
    letter['?'][i] = ROW.substring(26 * L, 27 * L);

    word[i] = '';
    for (j = 0; j < T.length; j++) {
        if (letter[T[j]]) {
            word[i] += letter[T[j]][i];
        } else {
            word[i] += letter['?'][i];
        }
    }

    print(word[i]);
}
