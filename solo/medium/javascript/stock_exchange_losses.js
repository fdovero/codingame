Array.prototype.max = function() {
    return Math.max.apply(Math, this);
};

Array.prototype.min = function() {
    return Math.min.apply(Math, this);
};

var n = parseInt(readline());
var vs = readline().split(' ').map(parseFloat);
var answer = 0;

/********* Execution too long for the 9999 datas ***************
var tab = new Array();

while(vs.length > 1) {
    var el_or = vs.shift();
    tab.push(vs.map(function(el){
       dif = el_or - parseInt(el);
       if (dif < 0) {
           res = 0;
       } else {
           res = dif;
       }
       return res;
    }).max());
    printErr(tab.length);
}
var answer = tab.max()>0?'-'+tab.max():0;
*******************************************************************/


/***** tricky O(n) solution *****/
var min = vs.min();
var minIndex = vs.lastIndexOf(min);
var max = vs.slice(0, minIndex).max();

answer = (max - min) > 0 ? '-'.concat(max - min) : 0;
print(answer);
