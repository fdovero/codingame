var width = parseInt(readline()); // the number of cells on the X axis
var height = parseInt(readline()); // the number of cells on the Y axis
var line = new Array();
var col = new Array();
for (var i = 0; i < height; i++) {
    line[i] = readline().replace(/0/g,'*'); // width characters, each either 0 or .
}

for (var j = 0; j < width; j++) {
    col[j]=''
    for (var i = 0; i < height; i++) {
        col[j] += line[i][j]
    }
}

for (var i = 0; i < height; i++) {
    for (var j = 0; j < width; j++ ) {
        var x1 = j;
        var y1 = i;
        var x2 = "-1"
        var y2 = "-1"
        var x3 = "-1"
        var y3 = "-1"

        if(line[i][j] == '.') {

        }
        else {
            if(j+1 < width) {
                sx = line[i].indexOf('*',j+1)
                if(sx > -1) {
                    x2 = sx
                    y2 = i
                }
            }

            if(i+1 < height) {
                sy = col[j].indexOf('*',i+1)
                if(sy > -1) {
                    x3 = j
                    y3 = sy
                }
            }
            print(x1,y1,x2,y2,x3,y3)
        }
    }
}
