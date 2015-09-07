Array.prototype.pack=function() {
    var arr = this;
    if (arr.length == 0) {
        return [];
    }
    var newArr = [];
    var tempArr = [arr[0]];
    var last = arr[0];
    for (var i = 1; i < arr.length; i++) {
        if (arr[i] == last) {
            tempArr.push(last);
        } else {
            newArr.push(tempArr);
            tempArr = [arr[i]];
            last = arr[i];
        }
    }
    newArr.push(tempArr);
    return newArr;
};

var r = parseInt(readline());
var l = parseInt(readline());

var seqence = new Array();
seqence.push(r);

for (var i=0; i<l-1; i++){
    var temp = seqence.pack();
    seqence = [];
    temp.forEach(function(elem, ind, arr) {
        var t = [];
        seqence.push(elem.length);
        seqence.push(elem[0]);
    });
}

print(seqence.join(' '));
