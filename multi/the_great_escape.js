function goal () {
  var result = 'DEFAULT'
  switch (this.id) {
    case 0: result = 'RIGHT'
      break
    case 1: result = 'LEFT'
      break
    case 2: result = 'DOWN'
      break
    case 3: result = 'UP'
      break
  }
  return result
}

function move (bot, walls) {
  printErr(bot.x, bot.y)
}

var board = {
  w: 0,
  h: 0,
  playerCount: 0,
  wallCount: 0
}

var inputs = readline().split(' ')
board.w = parseInt(inputs[0]) // width of the board
board.h = parseInt(inputs[1]) // height of the board
board.playerCount = parseInt(inputs[2]); // number of players (2,3, or 4)

var eugene = {}

eugene.id = parseInt(inputs[3]); // id of my player (0 = 1st player, 1 = 2nd player, ...)

// game loop
while (true) {
  for (var i = 0; i < board.playerCount; i++) {
    var inputs = readline().split(' ')
    if (i == eugene.id) {
      eugene.x = parseInt(inputs[0]); // x-coordinate of the player
      eugene.y = parseInt(inputs[1]); // y-coordinate of the player
      eugene.wallsLeft = parseInt(inputs[2]) // number of walls available for the player
    }
  }

  board.wallCount = parseInt(readline()) // number of walls on the board

  var walls = [ ]
  for (var i = 0; i < board.wallCount; i++) {
    var inputs = readline().split(' ')
    walls.push({x: parseInt(inputs[0]), y: parseInt(inputs[1]), orientation: inputs[2]}); // x-coordinate of the wall

    printErr(walls[i].x, walls[i].y, walls[i].orientation)
  }

  move(eugene, walls)

  print(goal.call(eugene)); // action: LEFT, RIGHT, UP, DOWN or "putX putY putOrientation" to place a wall
}
