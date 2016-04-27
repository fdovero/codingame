var N = parseInt(readline()); // Number of elements which make up the association table.
var Q = parseInt(readline()); // Number Q of file names to be analyzed.
var EXT = new Array();
var MT = new Array();

for (var i = 0; i < N; i++) {
    var inputs = readline().split(' ');
    EXT.push(inputs[0].toLowerCase()); // file extension
    MT.push(inputs[1]); // MIME type.
}
for (var i = 0; i < Q; i++) {
    var FNAME = readline(); // One file name per line.
    var spl = FNAME.split('.');
    var id = EXT.indexOf(spl[spl.length - 1].toLowerCase());

    if ((id != -1) && (spl.length > 1)) {
        print(MT[id]);
    } else {
        print('UNKNOWN');
    }
}
