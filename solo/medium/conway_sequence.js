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

var sequence = new Array();
sequence.push(r);

for (var i=0; i<l-1; i++){
    var temp = sequence.pack();
    sequence = [];
    temp.forEach(function(elem, ind, arr) {
        sequence.push(elem.length);
        sequence.push(elem[0]);
    });
}

print(sequence.join(' '));
